use super::process;

const PS_EXE: &str = r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe";

pub async fn run(command: &str) -> Result<String, String> {
    let result = process::run(
        PS_EXE,
        &["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", command],
    )
    .await?;

    if result.exit_code == 0 {
        Ok(result.stdout)
    } else {
        Err(format!(
            "PowerShell Exit {}: {}",
            result.exit_code,
            result.stderr.trim()
        ))
    }
}

pub async fn run_with_env(command: &str, env_vars: &[(&str, &str)]) -> Result<String, String> {
    let result = process::run_with_env(
        PS_EXE,
        &["-NoProfile", "-ExecutionPolicy", "Bypass", "-Command", command],
        env_vars,
    )
    .await?;

    if result.exit_code == 0 {
        Ok(result.stdout)
    } else {
        Err(format!(
            "PowerShell Exit {}: {}",
            result.exit_code,
            result.stderr.trim()
        ))
    }
}

/// Escapt einen String für PS-Einfachanführungszeichen (ersetzt ' durch '')
pub fn escape(s: &str) -> String {
    s.replace('\'', "''")
}
