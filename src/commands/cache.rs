use crate::error::Result;
use std::{path::PathBuf, process::ExitCode};
use tracing::trace;

pub fn view(cache_dir: Option<PathBuf>, as_table: bool) -> Result<ExitCode> {
    trace!(
        "view(cache_dir: {:?}, as_table: {as_table})",
        cache_dir
            .map(|p| p.display().to_string())
            .unwrap_or_default()
    );

    Ok(ExitCode::SUCCESS)
}

pub fn clear(cache_dir: Option<PathBuf>, name: Option<String>) -> Result<ExitCode> {
    trace!(
        "clear(cache_dir: {:?}, name: {:?})",
        cache_dir
            .map(|p| p.display().to_string())
            .unwrap_or_default(),
        name.map(|n| n.to_string()).unwrap_or_default()
    );

    Ok(ExitCode::SUCCESS)
}

pub fn refresh(cache_dir: Option<PathBuf>, name: Option<String>) -> Result<ExitCode> {
    trace!(
        "refresh(cache_dir: {:?}, name: {:?})",
        cache_dir
            .map(|p| p.display().to_string())
            .unwrap_or_default(),
        name.map(|n| n.to_string()).unwrap_or_default()
    );

    Ok(ExitCode::SUCCESS)
}
