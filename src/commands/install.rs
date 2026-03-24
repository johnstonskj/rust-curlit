use crate::{
    cache::cached_file_name,
    config::{EntryType, resolve_config},
    error::Result,
    fetch::{fetch_url, filename_from_url},
    shell::{execute_with_shell, resolve_shell},
};
use std::{io::Write, path::Path, process::ExitCode};
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
                && command_in_path(&entry_name)
            {
                info!(
                    "skipping install, command-name: `{command_name}` executable already in PATH"
                );
                continue;
            } else if command_in_path(&entry_name) {
                info!("skipping install, name: `{entry_name}` executable already in PATH");
                continue;
            }
        }

        info!("installing `{entry_name}` from {}", entry.url);
        let content = fetch_url(&entry.url)?;
        let file_name = filename_from_url(&entry.url);
        let shell = resolve_shell(entry.shell.as_deref());

        let script_path = if let Some(dir) = &entry.cache_dir {
            std::fs::create_dir_all(dir)?;
            dir.join(&file_name)
        } else {
            // Use default cache dir
            cached_file_name(&entry_name, &file_name)?
        };
        let mut f = std::fs::File::create(&script_path)?;
        f.write_all(content.as_bytes())?;

        execute_with_shell(&shell, &script_path, &entry_name)?;
        info!("installed `{entry_name}`");
    }

    Ok(ExitCode::SUCCESS)
}

fn command_in_path(name: &str) -> bool {
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_var) {
            if dir.join(name).is_file() {
                return true;
            }
        }
    }
    false
}
