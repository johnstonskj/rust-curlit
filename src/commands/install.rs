use crate::{
    cache::ResourceCache,
    config::{EntryType, resolve_config},
    error::Result,
    fetch::fetch_url,
    shell::{execute_with_shell, resolve_shell},
};
use is_executable::IsExecutable;
use std::{path::Path, process::ExitCode};
use tracing::{error, info, trace};

pub fn run(file: Option<&Path>, name: Option<&str>) -> Result<ExitCode> {
    trace!("run(file: {file:?}, name: {name:?}) ");
    let (path, config, _explicit) = resolve_config(file)?;
    info!("loaded configuration from file {path:?}");

    let entries: Vec<_> = if let Some(n) = name {
        let entry = config.get(n).ok_or_else(|| {
            error!("entry named `{name:?}` not in configuration");
            crate::error::CurlitError::EntryNotFound {
                name: n.to_string(),
            }
        })?;
        vec![(n.to_string(), entry.clone())]
    } else {
        config
            .entries
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    };

    for (entry_name, entry) in entries {
        // Skip if type=cli and command already in PATH
        if entry.entry_type == Some(EntryType::Cli) {
            if let Some(command_name) = &entry.command_name
                && is_command_in_path(&entry_name)
            {
                info!(
                    "skipping install, command-name: `{command_name}` executable already in PATH"
                );
                continue;
            } else if is_command_in_path(&entry_name) {
                info!("skipping install, name: `{entry_name}` executable already in PATH");
                continue;
            }
        }

        let cache = if let Some(cache_dir) = entry.cache_dir.as_ref() {
            ResourceCache::open(cache_dir.clone())
        } else {
            ResourceCache::open_default()
        }?;
        let script_path = if cache.is_cached(&entry_name)? {
            // TODO: use metadata to call HEAD on resource?
            cache.entry_content_path(&entry_name)
        } else {
            let resource = fetch_url(&entry.url)?;
            let (script_path, _) = cache.save(&resource, &entry_name)?;
            script_path
        };

        info!("installing `{entry_name}` using script {script_path:?}");

        let shell = resolve_shell(entry.shell.as_deref());
        execute_with_shell(&shell, &script_path, &entry_name)?;
        info!("installed `{entry_name}`");
    }

    Ok(ExitCode::SUCCESS)
}

fn is_command_in_path(name: &str) -> bool {
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_var) {
            let maybe_path = dir.join(name);
            if maybe_path.is_file() && maybe_path.is_executable() {
                return true;
            }
        }
    }
    false
}
