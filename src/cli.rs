use crate::config::EntryType;
use clap::{Parser, Subcommand};
use clap_complete::Shell;
use clap_verbosity_flag::{ErrorLevel, Verbosity};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "curlit", version, about = "Automate curl | shell installs")]
pub struct Cli {
    #[command(flatten)]
    pub verbosity: Verbosity<ErrorLevel>,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize curlit config and cache directories
    Init,

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for (defaults to $SHELL)
        shell: Option<Shell>,
    },

    /// Fetch and immediately execute a script
    Now {
        /// URL of the script to fetch and run
        #[arg(short = 'u', long)]
        url: String,

        /// Shell to use (defaults to $SHELL or bash)
        #[arg(short = 's', long)]
        shell: Option<String>,

        /// Directory to cache the downloaded script
        #[arg(short = 'c', long)]
        cache_dir: Option<PathBuf>,
    },

    /// Install all (or a named) entry from the config
    Install {
        /// Path to config file
        #[arg(short = 'f', long)]
        file: Option<PathBuf>,

        /// Name of entry to install (installs all if omitted)
        #[arg(short = 'n', long)]
        name: Option<String>,
    },

    /// Add an entry to the config
    Add {
        /// Path to config file
        #[arg(short = 'f', long)]
        file: Option<PathBuf>,

        /// URL of the script
        #[arg(short = 'u', long)]
        url: String,

        /// Name for this entry
        #[arg(short = 'n', long)]
        name: String,

        /// Command name for this entry
        #[arg(short = 'N', long)]
        command_name: Option<String>,

        /// Shell to use when running this script
        #[arg(short = 's', long)]
        shell: Option<String>,

        /// Directory to cache the downloaded script
        #[arg(short = 'c', long)]
        cache_dir: Option<PathBuf>,

        /// Type of the entry
        #[arg(short = 't', long = "type")]
        entry_type: Option<EntryType>,

        /// Overwrite if entry already exists
        #[arg(long)]
        force: bool,
    },

    /// Delete an entry from the config
    Delete {
        /// Path to config file
        #[arg(short = 'f', long)]
        file: Option<PathBuf>,

        /// Name of entry to delete
        #[arg(short = 'n', long)]
        name: String,
    },

    /// View config entries
    View {
        /// Path to config file
        #[arg(short = 'f', long)]
        file: Option<PathBuf>,

        /// Name of entry to view (shows all if omitted)
        #[arg(short = 'n', long)]
        name: Option<String>,

        /// Display results as a Markdown table
        #[arg(long)]
        as_table: bool,
    },
}
