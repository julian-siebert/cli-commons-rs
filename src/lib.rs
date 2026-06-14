use std::path::{Path, PathBuf};

#[cfg(feature = "author")]
pub mod author;
#[cfg(feature = "license")]
pub mod license;
#[cfg(feature = "toml")]
pub mod toml;

pub mod gitignore;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "miette", derive(miette::Diagnostic))]
pub enum Error {
    #[cfg(feature = "author")]
    #[error("Author error: {0}")]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Author(#[from] crate::author::Error),

    #[cfg(feature = "license")]
    #[error("License error: {0}")]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    License(#[from] crate::license::Error),

    #[cfg(feature = "toml")]
    #[error("TOML error: {0}")]
    #[cfg_attr(feature = "miette", diagnostic(transparent))]
    Toml(#[from] crate::toml::Error),

    #[cfg(feature = "which")]
    #[error("Path error: {0}")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(code(which), help("Is the program installed in PATH?"))
    )]
    Which(#[from] which::Error),

    #[error("File not found: {path}")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(fs::not_found),
            help("Make sure the file exists and the path is correct.")
        )
    )]
    NotFound {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Permission denied for {path}")]
    #[cfg_attr(feature = "miette", diagnostic(
            code(fs::permission_denied),
            help("Check the file permissions with `ls -l {}`.", path.display())
        ))]
    PermissionDenied {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("{path} already exists")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(fs::already_exists),
            help("Remove it first or pick a different path.")
        )
    )]
    AlreadyExists {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Expected {path} to be a directory")]
    #[cfg_attr(feature = "miette", diagnostic(
            code(fs::not_a_directory),
            help("Something at {} is not a directory.", path.display())
        ))]
    NotADirectorySource {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Expected {path} to be a directory")]
    #[cfg_attr(feature = "miette", diagnostic(
            code(fs::not_a_directory),
            help("Something at {} is not a directory.", path.display())
        ))]
    NotADirectory { path: PathBuf },

    #[error("{path} is a directory, expected a file")]
    #[cfg_attr(feature = "miette", diagnostic(
            code(fs::is_a_directory),
            help("Point at a file, not the directory {}.", path.display())
        ))]
    IsADirectory {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Directory {path} is not empty")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(fs::directory_not_empty),
            help("Empty the directory or use a recursive removal.")
        )
    )]
    DirectoryNotEmpty {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Filesystem at {path} is read-only")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(fs::read_only),
            help("Remount the filesystem read-write or choose another location.")
        )
    )]
    ReadOnlyFilesystem {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("No space left while accessing {path}")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(code(fs::storage_full), help("Free up disk space and try again."))
    )]
    StorageFull {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Quota exceeded while accessing {path}")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(fs::quota_exceeded),
            help("You have hit a disk quota; free space or raise the quota.")
        )
    )]
    QuotaExceeded {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid filename: {path}")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(
            code(fs::invalid_filename),
            help("The path contains characters the OS rejects.")
        )
    )]
    InvalidFilename {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    // Generischer Fall: trägt den ErrorKind im help mit, ohne 30 Varianten zu brauchen.
    #[error("I/O error at {path}: {kind}")]
    #[cfg_attr(
        feature = "miette",
        diagnostic(code(fs::io), help("Underlying I/O error kind: {kind}."))
    )]
    Io {
        path: PathBuf,
        kind: std::io::ErrorKind,
        #[source]
        source: std::io::Error,
    },
}

impl Error {
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        use std::io::ErrorKind as K;
        let path = path.into();
        let kind = source.kind();
        match source.kind() {
            K::NotFound => Error::NotFound { path, source },
            K::PermissionDenied => Error::PermissionDenied { path, source },
            K::AlreadyExists => Error::AlreadyExists { path, source },
            K::NotADirectory => Error::NotADirectorySource { path, source },
            K::IsADirectory => Error::IsADirectory { path, source },
            K::DirectoryNotEmpty => Error::DirectoryNotEmpty { path, source },
            K::ReadOnlyFilesystem => Error::ReadOnlyFilesystem { path, source },
            K::StorageFull => Error::StorageFull { path, source },
            K::QuotaExceeded => Error::QuotaExceeded { path, source },
            K::InvalidFilename => Error::InvalidFilename { path, source },
            _ => Error::Io { path, kind, source },
        }
    }
}

pub trait IoResultExt<T> {
    fn path_ctx(self, path: impl Into<PathBuf>) -> Result<T>;
}

impl<T> IoResultExt<T> for std::result::Result<T, std::io::Error> {
    fn path_ctx(self, path: impl Into<PathBuf>) -> Result<T> {
        self.map_err(|e| Error::io(path, e))
    }
}

pub fn create_dir_all(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    std::fs::create_dir_all(path).path_ctx(path)
}

pub fn remove_dir_all(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    std::fs::remove_dir_all(path).path_ctx(path)
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let path = path.as_ref();
    std::fs::read(path).path_ctx(path)
}

pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    std::fs::read_to_string(path).path_ctx(path)
}

pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<()> {
    let path = path.as_ref();
    std::fs::write(path, contents).path_ctx(path)
}

pub fn exists<P: AsRef<Path>>(path: P) -> Result<bool> {
    let path = path.as_ref();

    std::fs::exists(path).path_ctx(path)
}

pub fn ensure_dir(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    if path.is_dir() {
        return Ok(());
    }
    Err(Error::NotADirectory { path: path.into() })
}

pub fn read_dir(path: impl AsRef<Path>) -> Result<ReadDir> {
    let path = path.as_ref();
    let inner = std::fs::read_dir(path).path_ctx(path)?;
    Ok(ReadDir {
        inner,
        path: path.to_owned(),
    })
}

pub struct ReadDir {
    inner: std::fs::ReadDir,
    path: PathBuf,
}

impl Iterator for ReadDir {
    type Item = Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|res| res.map(|entry| entry.path()).path_ctx(&self.path))
    }
}
