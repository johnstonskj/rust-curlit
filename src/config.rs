use crate::error::{CurlitError, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
use tracing::{error, info, trace};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryType {
    Cli,
}

pub const GLOBAL_CONFIG_FILE_NAME: &str = "config.toml";
pub const LOCAL_CONFIG_FILE_NAME: &str = "curlit.toml";

impl clap::ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Cli]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Cli => Some(clap::builder::PossibleValue::new("cli")),
        }
    }
}

impl std::fmt::Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cli => write!(f, "cli"),
        }
    }
}

impl EntryType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Cli => "cli",
        }
    }
}

/// Body of a TOML section. The section name (= entry name) is the BTreeMap key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigEntry {
    pub url: String,
    #[serde(rename = "command-name", skip_serializing_if = "Option::is_none")]
    pub command_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<String>,
    #[serde(rename = "cache-dir", skip_serializing_if = "Option::is_none")]
    pub cache_dir: Option<PathBuf>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub entry_type: Option<EntryType>,
}

/// Newtype wrapper around BTreeMap<name, entry>.
#[derive(Debug, Clone, Default)]
pub struct ConfigFile {
    pub entries: BTreeMap<String, ConfigEntry>,
    pub path: Option<PathBuf>,
}

impl ConfigFile {
    pub fn load(path: &Path) -> Result<Self> {
        trace!("ConfigFile::load(path: {path:?})");
        let content = std::fs::read_to_string(path)?;
        let entries: BTreeMap<String, ConfigEntry> = toml::from_str(&content)?;
        Ok(Self {
            entries,
            path: Some(path.to_path_buf()),
        })
    }

    pub fn save(&self) -> Result<()> {
        trace!("ConfigFile::save() / path: {:?}", self.path);
        let path = self.path.as_ref().expect("ConfigFile has no path set");

        // Check writable
        if path.exists() {
            let meta = std::fs::metadata(path)?;
            if meta.permissions().readonly() {
                error!("configuration file is not writable");
                return Err(CurlitError::ConfigNotWritable {
                    path: path.to_path_buf(),
                });
            }
        }

        let content = toml::to_string(&self.entries)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&ConfigEntry> {
        trace!("ConfigFile::get(name: {name:?}) / path: {:?}", self.path);
        self.entries.get(name)
    }

    pub fn upsert(&mut self, name: String, entry: ConfigEntry) {
        trace!("ConfigFile::upsert(name: {name:?}) / entry: {entry:?}");
        self.entries.insert(name, entry);
    }

    pub fn remove(&mut self, name: &str) -> Result<ConfigEntry> {
        trace!("ConfigFile::remove(name: {name:?}) / path: {:?}", self.path);
        self.entries.remove(name).ok_or_else(|| {
            error!("no entry named {name:?}");
            CurlitError::EntryNotFound {
                name: name.to_string(),
            }
        })
    }
}

pub fn config_search_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(config_home) = xdirs::config_dir_for("curlit") {
        paths.push(config_home.join(GLOBAL_CONFIG_FILE_NAME));
    }
    if let Some(data_home) = xdirs::data_local_dir_for("curlit") {
        paths.push(data_home.join(GLOBAL_CONFIG_FILE_NAME));
    }
    #[allow(deprecated)]
    if let Some(home) = std::env::home_dir() {
        paths.push(home.join(LOCAL_CONFIG_FILE_NAME));
    }
    if let Ok(cwd) = std::env::current_dir() {
        paths.push(cwd.join(LOCAL_CONFIG_FILE_NAME));
    }

    info!("config search paths: {paths:?}");

    paths
}

/// Returns (resolved_path, config_file, is_explicit).
pub fn resolve_config(file: Option<&Path>) -> Result<(PathBuf, ConfigFile, bool)> {
    trace!("resolve_config(file: {file:?})");
    if let Some(explicit) = file {
        let config = if explicit.exists() {
            ConfigFile::load(explicit)?
        } else {
            ConfigFile {
                entries: BTreeMap::new(),
                path: Some(explicit.to_path_buf()),
            }
        };
        return Ok((explicit.to_path_buf(), config, true));
    }

    let search = config_search_paths();
    for path in &search {
        if path.exists() {
            let config = ConfigFile::load(path)?;
            return Ok((path.clone(), config, false));
        }
    }

    error!("no configuration file found");
    Err(CurlitError::ConfigNotFound { paths: search })
}
