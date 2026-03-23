use crate::{config::resolve_config, error::Result};
use std::{path::Path, process::ExitCode};
use tracing::{info, trace};

pub fn run(file: Option<&Path>, name: &str) -> Result<ExitCode> {
    trace!("run(file: {file:?}, name: {name:?})");
    let (path, mut config, _explicit) = resolve_config(file)?;
    info!("loaded configuration from file {path:?}");

    config.remove(name)?;
    config.path = Some(path.clone());
    config.save()?;

    info!("deleted entry `{name}` from file {path:?}");
    Ok(ExitCode::SUCCESS)
}
