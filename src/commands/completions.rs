use crate::{cli::Cli, error::Result};
use clap::CommandFactory;
use clap_complete::{Shell, generate};
use std::{io, process::ExitCode};
use tracing::{info, trace};

pub fn run(shell: Option<Shell>) -> Result<ExitCode> {
    trace!("run(shell: {shell:?})");
    let shell = shell.unwrap_or_else(|| {
        // Try to detect from $SHELL
        std::env::var("SHELL")
            .ok()
            .and_then(|s| {
                let basename = std::path::Path::new(&s)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();
                basename.parse::<Shell>().ok()
            })
            .unwrap_or(Shell::Bash)
    });
    info!("creating completions for shell `{shell:?}`");

    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "curlit", &mut io::stdout());
    Ok(ExitCode::SUCCESS)
}
