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

    let installer = std::env::temp_dir().join("winmedic_TeamsSetup.exe");
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
pub async fn uninstall_teams() -> Result<String, String> {
    for proc in TEAMS_PROCESSES {
        let _ = process::run("taskkill", &["/F", "/IM", &format!("{proc}.exe")]).await;
    }

    // 1. Winget (New Teams)
    for id in &["Microsoft.Teams", "Microsoft.Teams.Free"] {
        if let Ok(r) = process::run(
            "winget",
            &["uninstall", "--id", id, "-e", "--silent", "--accept-source-agreements"],
        )
        .await
        {
            if r.exit_code == 0 {
                return Ok(format!("Teams uninstalled via winget ({id})"));
            }
        }
    }

    // 2. Fallback: AppX + Registry + klassischer Uninstaller
    let ps = r#"
$removed = @()

# MSIX / AppX
Get-AppxPackage -AllUsers | Where-Object { $_.Name -like '*Teams*' } | ForEach-Object {
    Remove-AppxPackage -Package $_.PackageFullName -AllUsers -ErrorAction SilentlyContinue
    $removed += $_.Name
}

# Registry-Uninstall-String (Classic Teams)
$regRoots = @(
    'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall',
    'HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall',
    'HKLM:\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall'
)
foreach ($root in $regRoots) {
    Get-ChildItem $root -ErrorAction SilentlyContinue | ForEach-Object {
        $item = Get-ItemProperty $_.PSPath -ErrorAction SilentlyContinue
        if ($item.DisplayName -like '*Teams*' -and $item.UninstallString) {
            $cmd = $item.UninstallString -replace ' /I.*$', '' -replace '"', ''
            if (Test-Path $cmd) {
                Start-Process -FilePath $cmd -ArgumentList '/uninstall /silent' -Wait -ErrorAction SilentlyContinue
                $removed += $item.DisplayName
            }
        }
    }
}

if ($removed.Count -gt 0) {
    Write-Output "Removed: $($removed -join ', ')"
} else {
    Write-Output "No Teams installation found"
}
"#;

    let result = crate::services::powershell::run(ps).await?;
    Ok(format!("Teams uninstalled: {}", result.trim()))
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
