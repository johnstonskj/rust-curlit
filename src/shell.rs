use crate::error::{CurlitError, Result};
use std::{
    path::Path,
    process::{Command, ExitCode},
};
use tracing::{error, trace};

/// Resolve which shell to use: explicit > $SHELL > "bash"
pub fn resolve_shell(explicit: Option<&str>) -> String {
    trace!("resolve_shell(explicit: {explicit:?})");
    if let Some(s) = explicit {
        return s.to_string();
    } else if let Ok(shell_env) = std::env::var("SHELL")
        && !shell_env.is_empty()
    {
        return shell_env;
    } else {
        "bash".to_string()
    }
}

/// Execute a script file with the given shell. Returns ShellFailed on non-zero exit.
pub fn execute_with_shell(shell: &str, script_path: &Path, name: &str) -> Result<ExitCode> {
    trace!("execute_with_shell(shell: {shell:?}, script_path: {script_path:?}, name: {name:?})");
    let status = Command::new(shell).arg(script_path).status()?;

    if !status.success() {
        error!("execute_with_shell error; status: {status}");
        let code = status.code().unwrap_or(-1);
        return Err(CurlitError::ShellFailed {
            name: name.to_string(),
            code,
        });
    }
    Ok(ExitCode::SUCCESS)
}
