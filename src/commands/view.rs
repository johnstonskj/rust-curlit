use crate::config::{ConfigEntry, resolve_config};
use crate::error::{CurlitError, Result};
use std::path::Path;
use std::process::ExitCode;
use tracing::{error, info, trace};

const COLUMN_NAMES: [&str; 5] = ["Name", "URL", "Shell", "Type", "Cached File"];
const NON_TABLE_HEADER_WIDTH: usize = 20;

pub fn run(file: Option<&Path>, name: Option<&str>, as_table: bool) -> Result<ExitCode> {
    trace!("run(file: {file:?}, name: {name:?}, as_table: {as_table})");
    let (path, config, _explicit) = resolve_config(file)?;
    info!("loaded configuration from file {path:?}");

    let widths: [usize; 5] = if as_table {
        let mut widths: [usize; 5] = COLUMN_NAMES
            .iter()
            .map(|s| s.len())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();
        for (n, entry) in &config.entries {
            widths[0] = usize::max(widths[0], n.len());
            widths[1] = usize::max(widths[1], entry.url.len());
            widths[2] = usize::max(
                widths[2],
                entry
                    .shell
                    .as_ref()
                    .map(|v| v.to_string().len())
                    .unwrap_or_default(),
            );
            widths[3] = usize::max(
                widths[3],
                entry
                    .entry_type
                    .as_ref()
                    .map(|v| v.to_string().len())
                    .unwrap_or_default(),
            );
            widths[4] = usize::max(
                widths[4],
                entry
                    .cache_dir
                    .as_ref()
                    .map(|v| v.display().to_string().len())
                    .unwrap_or_default(),
            );
        }
        widths
    } else {
        [0_usize, 0_usize, 0_usize, 0_usize, 0_usize]
    };

    println!("Loaded from: {path:?}\n");

    if let Some(n) = name {
        let entry = config.get(n).ok_or_else(|| {
            error!("entry {n} not found in configuration file");
            CurlitError::EntryNotFound {
                name: n.to_string(),
            }
        })?;
        if as_table {
            print_table_header(&COLUMN_NAMES, &widths);
        }
        print_entry(n, entry, as_table, &widths);
    } else {
        if config.entries.is_empty() {
            info!("no entries to view");
            return Ok(ExitCode::SUCCESS);
        } else {
            if as_table {
                print_table_header(&COLUMN_NAMES, &widths);
            }
            for (n, entry) in &config.entries {
                print_entry(n, entry, as_table, &widths);
            }
            println!();
        }
    }

    Ok(ExitCode::SUCCESS)
}

fn print_table_header(headers: &[&str], widths: &[usize]) {
    println!(
        "| {} |",
        headers
            .iter()
            .enumerate()
            .map(|(i, v)| format!("{:<1$}", v, widths[i]))
            .collect::<Vec<_>>()
            .join(" | ")
    );
    println!(
        "| {} |",
        widths
            .iter()
            .map(|w| "-".repeat(*w))
            .collect::<Vec<_>>()
            .join(" | ")
    );
}

fn print_entry(name: &str, entry: &ConfigEntry, as_table: bool, widths: &[usize]) {
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

    if as_table {
        println!(
            "| {} |",
            [
                name,
                entry.url.as_str(),
                entry.shell.as_deref().unwrap_or("-"),
                entry
                    .entry_type
                    .as_ref()
                    .map(|t| t.as_str())
                    .unwrap_or_else(|| "-"),
                cached_file.as_deref().unwrap_or("-"),
            ]
            .iter()
            .enumerate()
            .map(|(i, v)| format!("{:<1$}", v, widths[i]))
            .collect::<Vec<_>>()
            .join(" | ")
        );
    } else {
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
}

#[inline(always)]
fn print_entry_row(header: &str, value: &str) {
    println!("  {header:<0$} = {value}", NON_TABLE_HEADER_WIDTH);
}
