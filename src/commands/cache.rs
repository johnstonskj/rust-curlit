use crate::{cache::ResourceCache, error::Result, tabular::Table};
use chrono::{DateTime, Local};
use humansize::{DECIMAL, format_size};
use std::{fs, path::PathBuf, process::ExitCode};
use tracing::trace;

const COLUMN_NAMES: [&str; 7] = [
    "Name",
    "Cache Path",
    "Source URL",
    "Fetched",
    "Size",
    "Last-Modified",
    "Entity Tag",
];
const NON_TABLE_HEADER_WIDTH: usize = 20;

pub fn view(cache_dir: Option<PathBuf>, as_table: bool) -> Result<ExitCode> {
    let cache = if let Some(cache_dir) = cache_dir.as_ref() {
        ResourceCache::open(cache_dir.clone())
    } else {
        ResourceCache::open_default()
    }?;
    println!("Cache directory = {:?}", cache.path());
    println!();

    let cache_entries = cache.entries()?;

    if as_table {
        let mut table = Table::new(&COLUMN_NAMES);
        let mut rows: Vec<Vec<String>> = Default::default();

        for entry in cache_entries {
            let entry_metadata = cache.load_metadata(&entry.name).unwrap().unwrap();
            let fs_metadata = fs::metadata(&entry.content_path)?;
            let fs_last_modified: DateTime<Local> = DateTime::from(fs_metadata.modified()?);

            rows.push(vec![
                entry.name.clone(),
                entry.content_path.display().to_string(),
                entry_metadata.src_url,
                fs_last_modified.to_rfc2822(),
                format_size(fs_metadata.len(), DECIMAL),
                entry_metadata
                    .last_modified
                    .map(|d| d.to_rfc2822())
                    .unwrap_or("-".to_string()),
                entry_metadata.entity_tag.unwrap_or("-".to_string()),
            ]);
        }
        table.update_widths(&rows);
        table.print(&rows);
    } else {
        for entry in cache_entries {
            let entry_metadata = cache.load_metadata(&entry.name)?;
            let fs_metadata = fs::metadata(&entry.content_path)?;
            let modified: DateTime<Local> = DateTime::from(fs_metadata.modified()?);

            println!("[{}]", entry.name);
            print_entry_row("path", &entry.content_path.display().to_string());
            if let Some(metadata) = &entry_metadata {
                print_entry_row("src-url", &metadata.src_url);
            }

            // File system metadata for content file.
            print_entry_row("fetched", &modified.to_rfc2822());
            print_entry_row("size", &format_size(fs_metadata.len(), DECIMAL));

            // Optional entry metadata
            if let Some(metadata) = &entry_metadata {
                if let Some(last_modified) = &metadata.last_modified {
                    print_entry_row("last-modified", &last_modified.to_rfc2822());
                }
                if let Some(entity_tag) = &metadata.entity_tag {
                    print_entry_row("entity-tag", &format!("{entity_tag:?}"));
                }
            }

            println!();
        }
    }

    Ok(ExitCode::SUCCESS)
}

pub fn clear(cache_dir: Option<PathBuf>, name: Option<String>) -> Result<ExitCode> {
    let cache = if let Some(cache_dir) = cache_dir.as_ref() {
        ResourceCache::open(cache_dir.clone())
    } else {
        ResourceCache::open_default()
    }?;
    trace!("clear(cache: {cache:?}, name: {:?})", name);

    if let Some(name) = &name {
        cache.remove(name)?;
    } else {
        cache.remove_all()?;
    }

    Ok(ExitCode::SUCCESS)
}

pub fn refresh(cache_dir: Option<PathBuf>, name: Option<String>) -> Result<ExitCode> {
    let cache = if let Some(cache_dir) = cache_dir.as_ref() {
        ResourceCache::open(cache_dir.clone())
    } else {
        ResourceCache::open_default()
    }?;
    trace!(
        "refresh(cache: {cache:?}, name: {:?})",
        name.map(|n| n.to_string()).unwrap_or_default()
    );

    Ok(ExitCode::SUCCESS)
}

#[inline(always)]
fn print_entry_row(header: &str, value: &str) {
    println!("  {header:<0$} = {value}", NON_TABLE_HEADER_WIDTH);
}
