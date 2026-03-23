use crate::error::CurlitError;
use std::path::PathBuf;

pub fn cache_directory() -> Result<PathBuf, CurlitError> {
    let cache_dir =
        xdirs::cache_dir_for("curlit").unwrap_or_else(|| std::env::temp_dir().join("curlit"));
    std::fs::create_dir_all(&cache_dir)?;
    Ok(cache_dir)
}

pub fn cached_file_name(entry_name: &str, file_name: &str) -> Result<PathBuf, CurlitError> {
    Ok(cache_directory()?.join(entry_name).join(file_name))
}
