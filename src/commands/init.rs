use crate::{cache::ResourceCache, config::config_search_paths, error::Result};
use std::{fs, process::ExitCode};
use tracing::{info, trace, warn};

pub fn run() -> Result<ExitCode> {
    trace!("run()");
    // Use the first search path (XDG_CONFIG_HOME/curlit/config.toml) as canonical
    let paths = config_search_paths();
    let config_path = paths.into_iter().next().expect("at least one search path");

    // Create parent directories
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
        info!("created configuration directory: {parent:?}");
    }

    // Write empty config if not present
    if !config_path.exists() {
        fs::write(&config_path, "")?;
        info!("created config file: {}", config_path.display());
    } else {
        warn!("config file already exists: {}", config_path.display());
    }

    let cache = ResourceCache::create_default()?;
    info!("created cache directory: {}", cache.path().display());

    info!("curlit initialized; config file: {}", config_path.display());
    Ok(ExitCode::SUCCESS)
}
