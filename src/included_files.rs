use std::fs::File;
use std::io::{Cursor, Read, Seek};
use std::path::{Display, Path, PathBuf};
use include_dir::{include_dir, Dir, File as IncludeFile};

/* Functions to include files from the binary in the executable - instead of relying on the path.
This makes the executable more reliable. Disagree?
 */

static INCLUDED_FILES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/included");
const INCLUDED_PREFIX: &str = "$INCLUDED/";

pub trait ReadSeek: Read + Seek {}
impl<T: Read + Seek> ReadSeek for T {}

#[derive(Clone, Debug)]
pub enum AutoPath<'a> {
    Included(
        &'a Path,
        Option<&'static Dir<'static>>,
        Option<&'static IncludeFile<'static>>,
    ),
    External(PathBuf),
}

impl<'a> AutoPath<'a> {
    fn get_included_files_with_extensions(
        dir: &'static Dir<'static>,
        extensions: &[&str],
    ) -> Vec<&'static IncludeFile<'static>> {
        dir.files()
            .filter(|f| {
                f.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .is_some_and(|ext| extensions.contains(&ext))
            })
            .collect()
    }
}

impl<'a> AutoPath<'a> {
    pub fn all_from_extensions(&self, extensions: &[&str]) -> Vec<AutoPath<'static>> {
        match self {
            AutoPath::Included(_, d, _) => {
                if let Some(d) = d {
                    AutoPath::get_included_files_with_extensions(d, extensions)
                        .into_iter()
                        .map(AutoPath::from)
                        .collect::<Vec<AutoPath>>()
                } else {
                    Vec::new()
                }
            }
            AutoPath::External(path) => {
                let mut files = Vec::new();
                if path.is_dir()
                    && let Ok(entries) = std::fs::read_dir(path)
                {
                    for entry in entries.flatten() {
                        let path_inner = entry.path();
                        if path_inner.is_file()
                            && extensions
                                .contains(&path_inner.extension().unwrap().to_str().unwrap())
                        {
                            files.push(AutoPath::External(path_inner));
                        }
                    }
                }
                files
            }
        }
    }

    pub fn reader(&self) -> anyhow::Result<Box<dyn ReadSeek + 'static>> {
        match self {
            AutoPath::Included(_, _, f) => {
                if let Some(f) = f {
                    Ok(Box::new(Cursor::new(f.contents())))
                } else {
                    Err(anyhow::Error::msg(format!(
                        "Not a file: {}!",
                        self.display()
                    )))
                }
            }
            AutoPath::External(path) => Ok(Box::new(File::open(path)?)),
        }
    }

    pub fn display(&self) -> Display<'_> {
        self.path().display()
    }

    pub fn contents(&self) -> anyhow::Result<String> {
        match self {
            AutoPath::Included(_, _, f) => match f {
                Some(f) => Ok(String::from_utf8(f.contents().to_vec())?),
                None => Err(anyhow::Error::msg(format!(
                    "Not a file: {}!",
                    self.display()
                ))),
            },
            AutoPath::External(path) => Ok(std::fs::read_to_string(path)?),
        }
    }

    pub fn path(&self) -> &Path {
        match self {
            AutoPath::Included(path, _, _) => path,
            AutoPath::External(path) => path,
        }
    }

    pub fn path_buf(&self) -> PathBuf {
        match self {
            AutoPath::Included(path, _, _) => path.to_path_buf(),
            AutoPath::External(path) => path.clone(),
        }
    }

    pub fn get_popped(&self) -> Option<AutoPath<'a>> {
        match self {
            AutoPath::Included(path, _, _) => path
                .parent()
                .map(|p| AutoPath::try_from(p.to_path_buf()).unwrap()),
            AutoPath::External(path) => {
                let mut new_path = path.clone();
                if new_path.pop() {
                    Some(AutoPath::External(new_path))
                } else {
                    None
                }
            }
        }
    }

    pub fn get_joined(&self, other: &str) -> Option<AutoPath<'a>> {
        let joined = self.path().join(other);
        AutoPath::try_from(joined).ok()
    }
}

impl<'a> From<&'a Path> for AutoPath<'a> {
    fn from(path: &'a Path) -> Self {
        if is_included_and_exists(path) {
            let file = get_included_file(path);
            let dir = get_included_subdir(path);
            AutoPath::Included(path, dir, file)
        } else {
            AutoPath::External(path.to_path_buf())
        }
    }
}

impl<'a> From<&'static IncludeFile<'static>> for AutoPath<'a> {
    fn from(file: &'static IncludeFile<'static>) -> Self {
        AutoPath::Included(file.path(), None, Some(file))
    }
}

impl<'a> From<&'static Dir<'static>> for AutoPath<'a> {
    fn from(dir: &'static Dir<'static>) -> Self {
        AutoPath::Included(dir.path(), Some(dir), None)
    }
}

impl<'a> TryFrom<PathBuf> for AutoPath<'a> {
    type Error = anyhow::Error;
    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        if is_include_path(&path) {
            let file = get_included_file(&path);
            let dir = get_included_subdir(&path);
            let path = if let Some(file) = file {
                file.path()
            } else if let Some(dir) = dir {
                dir.path()
            } else {
                return Err(anyhow::Error::msg(format!(
                    "Path {} is neither included nor exists!",
                    path.display()
                )));
            };
            Ok(AutoPath::Included(path, dir, file))
        } else if path.exists() {
            Ok(AutoPath::External(path.to_path_buf()))
        } else {
            Err(anyhow::Error::msg(format!(
                "Path {} is neither included nor exists!",
                path.display()
            )))
        }
    }
}

impl TryFrom<String> for AutoPath<'static> {
    type Error = anyhow::Error;
    fn try_from(path: String) -> Result<Self, Self::Error> {
        AutoPath::try_from(PathBuf::from(path))
    }
}

impl TryFrom<&str> for AutoPath<'static> {
    type Error = anyhow::Error;
    fn try_from(path: &str) -> Result<Self, Self::Error> {
        AutoPath::try_from(PathBuf::from(path))
    }
}

fn is_include_path(path: &Path) -> bool {
    path.display()
        .to_string()
        .trim()
        .starts_with(INCLUDED_PREFIX)
}

fn is_included_and_exists(path: &Path) -> bool {
    is_include_path(path) && get_included_file(path).is_some()
}

fn strip_include_path(path: &Path) -> &Path {
    path.strip_prefix("$INCLUDED/").unwrap()
}

fn get_included_file(path: &Path) -> Option<&'static IncludeFile<'static>> {
    if is_include_path(path) {
        INCLUDED_FILES.get_file(strip_include_path(path))
    } else {
        None
    }
}

fn get_included_subdir(dir: &Path) -> Option<&'static Dir<'static>> {
    if is_include_path(dir) {
        INCLUDED_FILES.get_dir(strip_include_path(dir))
    } else {
        None
    }
}

// AUTO FUNCTIONS:
