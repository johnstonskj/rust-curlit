pub mod add;
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
            let _span = info_span!("now").entered();
            now::run(&url, shell.as_deref(), cache_dir.as_ref())?;
        }

        Commands::Install { file, name } => {
            let _span = info_span!("install").entered();
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
            let _span = info_span!("add").entered();
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
            let _span = info_span!("delete").entered();
            delete::run(file.as_deref(), &name)?;
        }

        Commands::View {
            file,
            name,
            as_table,
        } => {
            let _span = info_span!("view").entered();
            view::run(file.as_deref(), name.as_deref(), as_table)?;
        }
    }

    Ok(ExitCode::SUCCESS)
}
