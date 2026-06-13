#[cfg(feature = "author")]
pub mod author;
#[cfg(feature = "license")]
pub mod license;
#[cfg(feature = "toml")]
pub mod toml;

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

    #[error("IO error: {0}")]
    #[cfg_attr(feature = "miette", diagnostic(code(std::io)))]
    IoArc(#[from] std::sync::Arc<std::io::Error>),

    #[error("IO error: {0}")]
    #[cfg_attr(feature = "miette", diagnostic(code(std::io)))]
    Io(#[from] std::io::Error),
}
