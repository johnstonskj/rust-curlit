pub mod add;
pub mod cache;
pub mod completions;
pub mod delete;
pub mod init;
pub mod install;
pub mod now;
pub mod view;

use crate::{
    cli::{Cli, Commands},
    error::Result,
};
use std::process::ExitCode;
use tracing::{info_span, trace};

pub fn run(cli: Cli) -> Result<ExitCode> {
    trace!("commands::run ...");
    match cli.command {
        Commands::Init => {
            let _span = info_span!("init").entered();
            init::run()?;
        }

        Commands::Completions { shell } => {
            let _span = info_span!("run").entered();
            completions::run(shell)?;
        }

        Commands::Now {
            url,
            shell,
            cache_dir,
        } => {
            let _span = info_span!(
                "now",
                url = url,
                shell = shell,
                cache_dir = cache_dir.as_ref().map(|p| p.display().to_string()),
            )
            .entered();
            now::run(&url, shell.as_deref(), cache_dir.as_ref())?;
        }

        Commands::Install { file, name } => {
            let _span = info_span!(
                "install",
                file = file.as_ref().map(|p| p.display().to_string()),
                name = name,
            )
            .entered();
            install::run(file.as_deref(), name.as_deref())?;
        }

        Commands::Add {
            file,
            url,
            name,
            command_name,
            shell,
            cache_dir,
            entry_type,
            force,
        } => {
            let _span = info_span!(
                "add",
                file = file.as_ref().map(|p| p.display().to_string()),
                url = url,
                name = name,
                command_name = command_name,
                shell = shell,
                cache_dir = cache_dir.as_ref().map(|p| p.display().to_string()),
                entry_type = entry_type
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
                force = force,
            )
            .entered();
            add::run(
                file.as_deref(),
                url,
                name,
                command_name,
                shell,
                cache_dir,
                entry_type,
                force,
            )?;
        }

        Commands::Delete { file, name } => {
            let _span = info_span!(
                "delete",
                file = file.as_ref().map(|p| p.display().to_string()),
                name = name,
            )
            .entered();
            delete::run(file.as_deref(), &name)?;
        }

        Commands::View {
            file,
            name,
            as_table,
        } => {
            let _span = info_span!(
                "view",
                file = file.as_ref().map(|p| p.display().to_string()),
                name = name,
                as_table = as_table,
            )
            .entered();
            view::run(file.as_deref(), name.as_deref(), as_table)?;
        }

        Commands::CacheView {
            cache_dir,
            as_table,
        } => {
            let _span = info_span!(
                "cache-view",
                cache_dir = cache_dir.as_ref().map(|p| p.display().to_string()),
                as_table = as_table
            )
            .entered();
            cache::view(cache_dir, as_table)?;
        }

        Commands::CacheRefresh { cache_dir, name } => {
            let _span = info_span!(
                "cache-refresh",
                cache_dir = cache_dir.as_ref().map(|p| p.display().to_string()),
                name = name
            )
            .entered();
            cache::refresh(cache_dir, name)?;
        }

        Commands::CacheClear { cache_dir, name } => {
            let _span = info_span!(
                "cache-clear",
                cache_dir = cache_dir.as_ref().map(|p| p.display().to_string()),
                name = name
            )
            .entered();
            cache::clear(cache_dir, name)?;
        }
    }

    Ok(ExitCode::SUCCESS)
}
