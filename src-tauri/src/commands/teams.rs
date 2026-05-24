use crate::services::process;

const TEAMS_INSTALLER_URL: &str =
    "https://statics.teams.cdn.office.net/production-windows-x86/lkg/MSTeamsSetup.exe";

const TEAMS_PROCESSES: &[&str] = &["ms-teams", "MSTeams", "Teams"];

// New Teams Cache (MSTeams_8wekyb3d8bbwe)
const TEAMS_CACHE_PATH: &str =
    r"%LOCALAPPDATA%\Packages\MSTeams_8wekyb3d8bbwe\LocalCache\Microsoft\MSTeams";

#[tauri::command]
pub async fn install_teams() -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    let bytes = client
        .get(TEAMS_INSTALLER_URL)
        .send()
        .await
        .map_err(|e| format!("Teams download failed: {e}"))?
        .bytes()
        .await
        .map_err(|e| format!("Teams download failed: {e}"))?;

    let installer = std::env::temp_dir().join("o365util_TeamsSetup.exe");
    std::fs::write(&installer, &bytes)
        .map_err(|e| format!("Failed to save Teams installer: {e}"))?;

    let result = process::run(installer.to_str().unwrap(), &["/silent"]).await?;

    if result.exit_code == 0 {
        Ok("Microsoft Teams installed successfully".to_string())
    } else {
        Err(format!(
            "Teams installation failed (Exit {}): {}",
            result.exit_code, result.stderr
        ))
    }
}

#[tauri::command]
pub async fn clear_teams_cache() -> Result<String, String> {
    // Prozesse beenden
    for proc in TEAMS_PROCESSES {
        let _ = process::run("taskkill", &["/F", "/IM", &format!("{proc}.exe")]).await;
    }

    let ps = format!(
        r#"
$p = [System.Environment]::ExpandEnvironmentVariables('{TEAMS_CACHE_PATH}')
if (Test-Path $p) {{
    Get-ChildItem $p -Recurse -Force -ErrorAction SilentlyContinue |
        Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
    Write-Output "OK"
}} else {{
    Write-Output "Path not found: $p"
}}"#
    );

    let result = crate::services::powershell::run(&ps).await?;
    Ok(format!("Teams cache cleared: {}", result.trim()))
}
