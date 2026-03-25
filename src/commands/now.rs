use crate::{
    error::Result,
    fetch::{fetch_url, filename_from_url},
    shell::{execute_with_shell, resolve_shell},
};
use std::{io::Write, path::PathBuf, process::ExitCode};
use tracing::info;

pub fn run(url: &str, shell: Option<&str>, cache_dir: Option<&PathBuf>) -> Result<ExitCode> {
    info!("fetching {url}");
    let resource = fetch_url(url)?;

    let filename = filename_from_url(url);
    let shell = resolve_shell(shell);

    if let Some(dir) = cache_dir {
        std::fs::create_dir_all(dir)?;
        let script_path = dir.join(&filename);
        let mut f = std::fs::File::create(&script_path)?;
        f.write_all(resource.content.as_bytes())?;
        info!("content cached to {}", script_path.display());
        execute_with_shell(&shell, &script_path, &filename)?;
    } else {
        // Write to temp file, then execute
        let tmp_dir = std::env::temp_dir();
        let script_path = tmp_dir.join(&filename);
        let mut f = std::fs::File::create(&script_path)?;
        f.write_all(resource.content.as_bytes())?;
        let result = execute_with_shell(&shell, &script_path, &filename);
        // Clean up temp file regardless
        let _ = std::fs::remove_file(&script_path);
        result?;
    }

    info!("executed script from {url} in shell {shell}");
    Ok(ExitCode::SUCCESS)
}
