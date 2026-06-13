use miette::{NamedSource, SourceSpan};
use serde::de::DeserializeOwned;

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    #[error("TOML syntax error: {message}")]
    #[diagnostic(
        code(toml::syntax),
        help("Check for missing quotes, brackets, or invalid values.")
    )]
    Syntax {
        message: String,
        #[source_code]
        src: miette::NamedSource<String>,
        #[label("here")]
        span: SourceSpan,
    },

    #[error("TOML syntax error: {message}")]
    #[diagnostic(
        code("toml::syntax_no_span"),
        help("Check for missing quotes, brackets, or invalid values.")
    )]
    SyntaxNoSpan { message: String },

    #[error("TOML serialize error: {message}")]
    #[diagnostic(
        code("toml::serialize"),
        help("Make sure  all types in the struct are TOML-serializeable")
    )]
    Serialize { message: String },
}

pub fn from_str_named<T: DeserializeOwned>(content: &str, name: &str) -> Result<T, Error> {
    toml::from_str(content).map_err(|e| {
        let message = e.message().to_string();

        match e.span() {
            Some(span) => Error::Syntax {
                message,
                src: NamedSource::new(name, content.to_string()),
                span: SourceSpan::from((span.start, span.end - span.start)),
            },
            None => Error::SyntaxNoSpan { message },
        }
    })
}

pub fn to_string<T: serde::Serialize>(value: &T) -> Result<String, Error> {
    toml::to_string(value).map_err(|e| Error::Serialize {
        message: e.to_string(),
    })
}
