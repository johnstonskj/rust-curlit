use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurlitError {
    #[error("config file not found; searched: {paths:?}")]
    ConfigNotFound { paths: Vec<PathBuf> },

    #[error("config file `{path}` is not writable")]
    ConfigNotWritable { path: PathBuf },

    #[error("cache directory `{path}` was not found, cannot open")]
    CacheNotFound { path: PathBuf },

    #[error("cache directory `{path}` already exists, cannot create")]
    CacheAlreadyExists { path: PathBuf },

    #[error(
        "entry named `{name}` already exists\n  Help: to overwrite the current entry, use the `--force` argument"
    )]
    EntryAlreadyExists { name: String },

    #[error("entry named `{name}` not found in config")]
    EntryNotFound { name: String },

    #[error("HTTP error fetching `{url}`: {message}")]
    FetchError { url: String, message: String },

    #[error("shell execution failed for `{name}`: exit code {code}")]
    ShellFailed { name: String, code: i32 },

    #[error("Tracing environment parser error: {0}")]
    TracingEnv(#[from] tracing_subscriber::filter::FromEnvError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("OsString conversion error; string: {:?}", bytes)]
    OsString { bytes: Vec<u8> },

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("Date/Time parse error: {0}")]
    TimeParse(#[from] chrono::ParseError),
}

pub type Result<T> = std::result::Result<T, CurlitError>;
