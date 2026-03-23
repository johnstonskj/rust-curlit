use crate::{
    config::{ConfigEntry, EntryType, resolve_config},
    error::{CurlitError, Result},
};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};
use tracing::{error, info, trace};

pub fn run(
    file: Option<&Path>,
    url: String,
    name: String,
    command_name: Option<String>,
    shell: Option<String>,
    cache_dir: Option<PathBuf>,
    entry_type: Option<EntryType>,
    force: bool,
) -> Result<ExitCode> {
    trace!(
        "run(file: {file:?}, url: {url:?}, name: {name:?}, command_name: {command_name:?}, shell: {shell:?}, cache_dir: {cache_dir:?}, entry_type: {entry_type:?}, force: {force})"
    );
    let (path, mut config, _explicit) = resolve_config(file)?;
    info!("loaded configuration from file {path:?}");

    if !force && config.get(&name).is_some() {
        error!("entry `{name}` already exists in config");
        return Err(CurlitError::EntryAlreadyExists { name });
    } else if force && config.get(&name).is_some() {
        info!("overwriting entry `{name}` in config")
    }

    let entry = ConfigEntry {
        url,
        command_name,
        shell,
        cache_dir,
        entry_type,
    };

    config.upsert(name.clone(), entry);
    config.path = Some(path.clone());
    config.save()?;

    info!("added entry `{name}` to config");
    Ok(ExitCode::SUCCESS)
}
