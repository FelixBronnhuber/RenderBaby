use anyhow::Result;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::write::FileOptions;
use log::{info, debug};

use std::sync::atomic::{AtomicUsize, Ordering};

pub struct FileManager;

static IMPORT_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl FileManager {
    /// Unzips a .rscn file to a temporary directory and returns the path to the root of the extracted scene.
    pub fn unzip_scene(rscn_path: &Path) -> Result<PathBuf> {
        let file = fs::File::open(rscn_path)?;

        let mut archive = zip::ZipArchive::new(file)?;

        // Create a unique temp directory

        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        let count = IMPORT_COUNTER.fetch_add(1, Ordering::SeqCst);

        let temp_dir = std::env::temp_dir()
            .join("renderbaby")
            .join(format!("import_{}_{}", nanos, count));

        // Clean up if exists (unlikely with timestamp)
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }
        fs::create_dir_all(&temp_dir)?;

        info!(
            "FileManager: Unzipping scene from {:?} to {:?}",
            rscn_path, temp_dir
        );

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => temp_dir.join(path),
                None => continue,
            };

            if (*file.name()).ends_with('/') {
                debug!("FileManager: Creating directory {:?}", outpath);
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent()
                    && !p.exists()
                {
                    fs::create_dir_all(p)?;
                }
                debug!(
                    "FileManager: Extracting file {:?} to {:?}",
                    file.name(),
                    outpath
                );
                let mut outfile = fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }

        Ok(temp_dir)
    }

    /// Recursively finds the first scene.json in the directory.
    /// If not found, returns the first .json file found.
    pub fn find_scene_json(root: &Path) -> Result<PathBuf> {
        let mut queue = vec![root.to_path_buf()];
        let mut first_json_fallback: Option<PathBuf> = None;

        while let Some(dir) = queue.pop() {
            if dir.is_dir() {
                for entry in fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        queue.push(path);
                    } else if let Some(name) = path.file_name() {
                        if name == "scene.json" {
                            debug!("FileManager: Found scene.json at {:?}", path);
                            return Ok(path);
                        } else if path.extension().map(|e| e == "json").unwrap_or(false)
                            && first_json_fallback.is_none()
                        {
                            first_json_fallback = Some(path);
                        }
                    }
                }
            }
        }

        if let Some(path) = first_json_fallback {
            debug!("FileManager: Falling back to found json at {:?}", path);
            Ok(path)
        } else {
            Err(anyhow::anyhow!("No scene json file found in archive"))
        }
    }

    /// Zips a staging directory to the output path.
    /// The content of `staging_root` becomes the content of the zip file.
    pub fn zip_scene(staging_root: &Path, output_path: &Path) -> Result<()> {
        info!(
            "FileManager: Zipping scene from {:?} to {:?}",
            staging_root, output_path
        );
        let file = fs::File::create(output_path)?;
        let mut zip = zip::ZipWriter::new(file);
        // Use Store for speed and compatibility, or Deflated if available
        let options = FileOptions::<()>::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);

        let mut buffer = Vec::new();

        let mut paths = Vec::new();
        let mut q = vec![staging_root.to_path_buf()];
        while let Some(dir) = q.pop() {
            for entry in fs::read_dir(&dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    q.push(path.clone());
                }
                paths.push(path);
            }
        }
        // sort paths to ensure directories come before files if needed?
        // Standard zip doesn't strictly require it but it's nice.
        paths.sort();

        for path in paths {
            let name = path
                .strip_prefix(staging_root)?
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid path encoding"))?;

            // Windows path separator handling?
            // zip expects forward slashes.
            let name = name.replace('\\', "/");

            if path.is_dir() {
                debug!("FileManager: Adding directory to zip: {}", name);
                zip.add_directory(name, options)?;
            } else {
                debug!(
                    "FileManager: Adding file to zip: {} (from {:?})",
                    name, path
                );
                zip.start_file(name, options)?;
                let mut f = fs::File::open(&path)?;
                f.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;
                buffer.clear();
            }
        }

        zip.finish()?;
        info!("FileManager: Zip archive creation complete.");
        Ok(())
    }
}
