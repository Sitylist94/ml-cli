use std::path::PathBuf;

use thiserror::Error;

/// Errors specific to project generation.
#[derive(Debug, Error)]
pub enum Error {
    #[error("template directory not found: {0}")]
    TemplateDirectoryNotFound(PathBuf),

    #[error("output directory already exists: {0}")]
    OutputDirectoryAlreadyExists(PathBuf),

    #[error("failed to read template file {path}: {source}")]
    ReadTemplate {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to write generated file {path}: {source}")]
    WriteGeneratedFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to render template {template}: {source}")]
    RenderTemplate {
        template: String,
        #[source]
        source: tera::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
