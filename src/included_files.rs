use std::ffi::OsStr;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Read, Seek};
use std::path::{Path, PathBuf};
use include_dir::{include_dir, Dir, File as IncludeFile};

static INCLUDED_FILES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/included");
const INCLUDED_PREFIX: &str = "$INCLUDED/";

pub trait ReadSeek: Read + Seek + BufRead {}
impl<T: Read + Seek + BufRead> ReadSeek for T {}

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
    #[inline]
    fn with_include_prefix(path: &Path) -> PathBuf {
        let mut pb = PathBuf::from(INCLUDED_PREFIX);
        pb.push(path);
        pb
    }
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

use std::hash::{Hash, Hasher};

impl<'a> PartialEq for AutoPath<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.path_buf() == other.path_buf()
    }
}

impl Eq for AutoPath<'_> {}

impl<'a> Hash for AutoPath<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path_buf().hash(state)
    }
}

impl<'a> AutoPath<'a> {
    pub fn all_from_extensions(&self, extensions: &[&str]) -> Vec<AutoPath<'a>> {
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

    pub fn get_absolute_or_join(path: &str, base: &AutoPath<'a>) -> anyhow::Result<AutoPath<'a>> {
        match base {
            AutoPath::Included(_, _, _) => {
                if is_include_path(path.as_ref()) {
                    return AutoPath::try_from(path);
                }
            }
            AutoPath::External(_) => {
                if Path::new(path).is_absolute() {
                    return AutoPath::try_from(path);
                }
            }
        }
        base.get_joined(path).ok_or_else(|| {
            anyhow::Error::msg(format!(
                "Could not join path {} with base path {}!",
                path, base
            ))
        })
    }

    pub fn reader(&self) -> anyhow::Result<Box<dyn ReadSeek + 'static>> {
        match self {
            AutoPath::Included(_, _, f) => {
                if let Some(f) = f {
                    Ok(Box::new(Cursor::new(f.contents())))
                } else {
                    Err(anyhow::Error::msg(format!("Not a file: {}!", self)))
                }
            }
            AutoPath::External(path) => Ok(Box::new(BufReader::new(File::open(path)?))),
        }
    }

    pub fn contents(&self) -> anyhow::Result<String> {
        match self {
            AutoPath::Included(_, _, f) => match f {
                Some(f) => Ok(String::from_utf8(f.contents().to_vec())?),
                None => Err(anyhow::Error::msg(format!("Not a file: {}!", self))),
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
            AutoPath::Included(path, _, _) => Self::with_include_prefix(path),
            AutoPath::External(path) => path.clone(),
        }
    }

    pub fn get_popped(&self) -> Option<AutoPath<'a>> {
        match self {
            AutoPath::Included(path, _, _) => path.parent().map(|p| {
                let prefixed = Self::with_include_prefix(p);
                AutoPath::try_from(prefixed).unwrap()
            }),
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
        match self {
            AutoPath::Included(path, _, _) => {
                let joined = Self::with_include_prefix(path).join(other);
                AutoPath::try_from(joined).ok()
            }
            AutoPath::External(_) => {
                let joined = self.path().join(other);
                AutoPath::try_from(joined).ok()
            }
        }
    }

    pub fn extension(&self) -> Option<&str> {
        self.extension_os().and_then(|f| f.to_str())
    }

    pub fn extension_os(&self) -> Option<&OsStr> {
        self.path().extension()
    }

    pub fn iter_dir(&self) -> anyhow::Result<impl Iterator<Item = AutoPath<'a>>> {
        match self {
            AutoPath::Included(_, d, _) => {
                if let Some(d) = d {
                    let paths = d
                        .dirs()
                        .map(|dir| AutoPath::Included(dir.path(), Some(dir), None))
                        .chain(
                            d.files()
                                .map(|file| AutoPath::Included(file.path(), None, Some(file))),
                        );
                    Ok(Box::new(paths) as Box<dyn Iterator<Item = AutoPath>>)
                } else {
                    Err(anyhow::Error::msg(format!("Not a directory: {}!", self)))
                }
            }
            AutoPath::External(path) => {
                if path.is_dir() {
                    let entries = std::fs::read_dir(path)?
                        .filter_map(|res| res.ok())
                        .map(|entry| AutoPath::External(entry.path()));
                    Ok(Box::new(entries) as Box<dyn Iterator<Item = AutoPath>>)
                } else {
                    Err(anyhow::Error::msg(format!("Not a directory: {}!", self)))
                }
            }
        }
    }

    pub fn is_dir(&self) -> bool {
        match self {
            AutoPath::Included(_, d, _) => d.is_some(),
            AutoPath::External(path) => path.is_dir(),
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            AutoPath::Included(_, _, f) => f.is_some(),
            AutoPath::External(path) => path.is_file(),
        }
    }

    fn os_to_string(os_str: Option<&OsStr>) -> Option<String> {
        os_str.and_then(|name| name.to_str().map(|s| s.to_string()))
    }

    pub fn file_name(&self) -> Option<String> {
        match self {
            AutoPath::Included(_, _, f) => f
                .as_ref()
                .and_then(|file| Self::os_to_string(file.path().file_name())),
            AutoPath::External(path) => Self::os_to_string(path.file_name()),
        }
    }
}

impl Display for AutoPath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path_buf().to_string_lossy())
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
        let path = strip_include_path(path);
        INCLUDED_FILES.get_file(path)
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
