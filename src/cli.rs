use crate::config::EntryType;
use clap::{Parser, Subcommand};
use clap_complete::Shell;
use clap_verbosity_flag::{ErrorLevel, Verbosity};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "curlit", version, about = "Automate `curl | shell` installs")]
pub struct Cli {
    #[command(flatten)]
    pub verbosity: Verbosity<ErrorLevel>,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize curlit config and cache directories and create a default configuration file.
    Init,

    /// Generate shell completions, writing to stdout.
    Completions {
        /// Shell to generate completions for (defaults to $SHELL).
        shell: Option<Shell>,
    },

    /// Fetch and immediately execute a script, do not use the configuration file.
    ///
    /// This is a thin wrapper around the `curl | bash` pattern, especially when the `cache-dir`
    /// argument is not used. In this case the following two lines are equivalent.
    ///
    /// ❯ curlit now --url 'https://example.com/install/run.sh' --shell bash
    ///
    /// ❯ curl -sSL 'https://example.com/install/run.sh' >./run.sh | bash
    ///
    /// However, when the `cache-dir` option is used the downloaded file is stored locally before
    /// execution, allowing it to be inspected if execution fails.
    ///
    /// ❯ curlit now --url 'https://example.com/install/run.sh' --shell bash --cache-dir .
    ///
    /// ❯ curl -sSL 'https://example.com/install/run.sh' >./run.sh && bash ./run.sh
    ///
    Now {
        /// The URL of the script to fetch and then execute.
        #[arg(short = 'u', long)]
        url: String,

        /// The shell to use to execute the downloaded script.
        ///
        /// If not specified the value of the environment variable "${SHELL}" is used, and if that
        /// does not exist the 'bash' shell is used.
        #[arg(short = 's', long)]
        shell: Option<String>,

        /// Directory to cache the downloaded script before execution.
        ///
        /// The default value is "${XDG_CACHE_DIR}/curlit".
        #[arg(short = 'c', long)]
        cache_dir: Option<PathBuf>,
    },

    /// Install all entries, or a named entry, from the configuration file.
    Install {
        /// Path to a configuration file, otherwise known locations are checked.
        ///
        /// If no file path is specified, the following directories are checked in order:
        /// 1. `${XDG_CONFIG_HOME}/curlit/config.toml`,
        /// 2. `${XDG_DATA_HOME}/curlit/config.toml`,
        /// 3. `${HOME}/curlit.toml`,
        /// 4. `${PWD}/curit.toml`. If no file is found in any of these locations an error is returned.
        #[arg(short = 'f', long)]
        file: Option<PathBuf>,

        /// Name of an entry in the configuration file to install; installs all entries if omitted.
        #[arg(short = 'n', long)]
        name: Option<String>,
    },

    /// Add a new entry to the configuration file.
    Add {
        /// Path to a configuration file, otherwise known locations are checked.
        ///
        /// If no file path is specified, the following directories are checked in order:
        /// 1. `${XDG_CONFIG_HOME}/curlit/config.toml`,
        /// 2. `${XDG_DATA_HOME}/curlit/config.toml`,
        /// 3. `${HOME}/curlit.toml`,
        /// 4. `${PWD}/curit.toml`. If no file is found in any of these locations an error is returned.
        #[arg(short = 'f', long)]
        file: Option<PathBuf>,

        /// URL of the script to fetch and execute.
        #[arg(short = 'u', long)]
        url: String,

        /// Name for this entry, which becomes the configuration file key.
        #[arg(short = 'n', long)]
        name: String,

        /// Command name for this entry, used to determine whether a command is already installed.
        ///
        /// If not specified, the name of the entry, or `name` argument, is used as the command name.
        #[arg(short = 'N', long)]
        command_name: Option<String>,

        /// The shell to use to execute downloaded scripts.
        ///
        /// If not specified the value of the environment variable "${SHELL}" is used, and if that
        /// does not exist the 'bash' shell is used.
        #[arg(short = 's', long)]
        shell: Option<String>,

        /// Override the standard directory to cache downloaded scripts.
        ///
        /// The default value is "${XDG_CACHE_DIR}/curlit".
        #[arg(short = 'c', long)]
        cache_dir: Option<PathBuf>,

        /// The type of the entry, currently only `cli` is supported.
        #[arg(short = 't', long = "type")]
        entry_type: Option<EntryType>,

        /// Overwrite the entry if it already exists in the configuration file.
        #[arg(long)]
        force: bool,
    },

    /// Delete an entry from the configuration file.
    Delete {
        /// Path to a configuration file, otherwise known locations are checked.
        ///
        /// If no file path is specified, the following directories are checked in order:
        /// 1. `${XDG_CONFIG_HOME}/curlit/config.toml`,
        /// 2. `${XDG_DATA_HOME}/curlit/config.toml`,
        /// 3. `${HOME}/curlit.toml`,
        /// 4. `${PWD}/curit.toml`. If no file is found in any of these locations an error is returned.
        #[arg(short = 'f', long)]
        file: Option<PathBuf>,

        /// Name of entry to delete from the configuration file.
        #[arg(short = 'n', long)]
        name: String,
    },

    /// View configuration file entries
    View {
        /// Path to a configuration file, otherwise known locations are checked.
        ///
        /// If no file path is specified, the following directories are checked in order:
        /// 1. `${XDG_CONFIG_HOME}/curlit/config.toml`,
        /// 2. `${XDG_DATA_HOME}/curlit/config.toml`,
        /// 3. `${HOME}/curlit.toml`,
        /// 4. `${PWD}/curit.toml`. If no file is found in any of these locations an error is returned.
        #[arg(short = 'f', long)]
        file: Option<PathBuf>,

        /// Name of an entry in the configuration file to view; shows all if omitted.
        #[arg(short = 'n', long)]
        name: Option<String>,

        /// Display results as a Markdown table.
        #[arg(long)]
        as_table: bool,
    },

    /// View the current cache directory contents.
    CacheView {
        /// Override the standard directory to cache downloaded scripts.
        ///
        /// The default value is "${XDG_CACHE_DIR}/curlit".
        #[arg(short = 'c', long)]
        cache_dir: Option<PathBuf>,

        /// Display results as a Markdown table
        #[arg(long)]
        as_table: bool,
    },

    /// Refresh items in the cache
    CacheRefresh {
        /// Override the standard directory to cache downloaded scripts.
        ///
        /// The default value is "${XDG_CACHE_DIR}/curlit".
        #[arg(short = 'c', long)]
        cache_dir: Option<PathBuf>,

        /// Name of entry to view; shows all if omitted.
        #[arg(short = 'n', long)]
        name: Option<String>,
    },

    /// Clear the cache
    CacheClear {
        /// Override the standard directory to cache downloaded scripts.
        ///
        /// The default value is "${XDG_CACHE_DIR}/curlit".
        #[arg(short = 'c', long)]
        cache_dir: Option<PathBuf>,

        /// Name of entry to view; shows all if omitted.
        #[arg(short = 'n', long)]
        name: Option<String>,
    },
}
