mod cache;
mod cli;
mod commands;
mod config;
mod error;
mod fetch;
mod shell;
mod tabular;

use clap::Parser;
use cli::Cli;
use commands::run;
use std::process::ExitCode;
use tracing::{error, info};
use tracing_subscriber::filter::EnvFilter;

pub const APP_NAME: &str = "curlit";

fn main() -> Result<ExitCode, error::CurlitError> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(cli.verbosity.tracing_level_filter().into())
                .from_env()?,
        )
        .init();

    match run(cli) {
        Err(e) => {
            error!("command failed: {e}");
            eprintln!("{e}");
            Ok(ExitCode::FAILURE)
        }
        Ok(code) => {
            info! {"command returned {code:?} and no error"}
            Ok(code)
        }
    }
}
