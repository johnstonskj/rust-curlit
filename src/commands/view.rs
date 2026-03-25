use crate::cache::ResourceCache;
use crate::config::{ConfigEntry, resolve_config};
use crate::error::{CurlitError, Result};
use crate::tabular::Table;
use std::path::Path;
use std::process::ExitCode;
use tracing::{error, info, trace};

const COLUMN_NAMES: [&str; 5] = ["Name", "URL", "Shell", "Type", "Cached File"];
const NON_TABLE_HEADER_WIDTH: usize = 20;

pub fn run(file: Option<&Path>, name: Option<&str>, as_table: bool) -> Result<ExitCode> {
    trace!("run(file: {file:?}, name: {name:?}, as_table: {as_table})");
    let (path, config, _explicit) = resolve_config(file)?;
    info!("loaded configuration from file {path:?}");

    println!("Loaded from: {path:?}\n");
    println!();

    let entries: Vec<&ConfigEntry> = if let Some(name) = name {
        let entry = config.get(name).ok_or_else(|| {
            error!("entry {name} not found in configuration file");
            CurlitError::EntryNotFound {
                name: name.to_string(),
            }
        })?;
        vec![entry]
    } else {
        config.entries.iter().map(|(_, entry)| entry).collect()
    };

    if entries.is_empty() {
        println!("No entries.");
    } else if as_table {
        let mut table = Table::new(&COLUMN_NAMES);
        let mut rows: Vec<Vec<String>> = Default::default();

        for (name, entry) in &config.entries {
            rows.push(vec![
                name.clone(),
                entry.url.clone(),
                entry
                    .shell
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or("-".to_string()),
                entry
                    .entry_type
                    .as_ref()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "-".to_string()),
                cached_file_name(name, entry),
            ]);
        }
        table.update_widths(&rows);
        table.print(&rows);
    } else {
        for (n, entry) in &config.entries {
            print_entry(n, entry);
        }
        println!();
    }

    Ok(ExitCode::SUCCESS)
}

fn cached_file_name(name: &str, entry: &ConfigEntry) -> String {
    let cache = if let Some(cache_dir) = &entry.cache_dir {
        ResourceCache::open(cache_dir.clone())
    } else {
        ResourceCache::open_default()
    }
    .expect("could not open cache");

    let content_path = cache.entry_content_path(name);

    if content_path.is_file() {
        content_path.display().to_string()
    } else {
        "-".to_string()
    }
}

fn print_entry(name: &str, entry: &ConfigEntry) {
    let cached_file = entry.cache_dir.as_ref().and_then(|dir| {
        let filename = entry
            .url
            .rsplit('/')
            .find(|s| !s.is_empty())
            .unwrap_or("script.sh");
        let path = dir.join(filename);
        if path.exists() {
            Some(path.display().to_string())
        } else {
            None
        }
    });
    println!("[{name}]");
    print_entry_row("url", &entry.url);
    if let Some(s) = &entry.command_name {
        print_entry_row("command-name", s);
    }
    if let Some(s) = &entry.shell {
        print_entry_row("shell", s);
    }
    if let Some(t) = &entry.entry_type {
        print_entry_row("type", t.as_str());
    }
    if let Some(dir) = &entry.cache_dir {
        print_entry_row("cache-dir", dir.display().to_string().as_str());
    }
    if let Some(f) = cached_file {
        print_entry_row("cached-file", &f);
    }
    println!();
}

#[inline(always)]
fn print_entry_row(header: &str, value: &str) {
    println!("  {header:<0$} = {value}", NON_TABLE_HEADER_WIDTH);
}
