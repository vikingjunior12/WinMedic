use crate::services::{powershell, process};


const ONENOTE_CACHE_PATH: &str = r"%LOCALAPPDATA%\Microsoft\OneNote\16.0\cache";
const ONENOTE_PROCESSES: &[&str] = &["onenotem", "onenote", "msoia", "ONENOTE"];

#[tauri::command]
pub async fn install_onenote_backup_exporter() -> Result<String, String> {
    // OneNote für Windows 10 aus dem Store / Winget
    let result = process::run(
        "winget",
        &[
            "install",
            "--id",
            "Microsoft.OneNote",
            "--silent",
            "--accept-package-agreements",
            "--accept-source-agreements",
        ],
    )
    .await?;

    if result.exit_code == 0 {
        Ok("OneNote Backup Exporter installed successfully".to_string())
    } else {
        Err(format!(
            "OneNote installation failed (Exit {}): {}",
            result.exit_code, result.stderr
        ))
    }
}

#[tauri::command]
pub async fn clear_onenote_cache() -> Result<String, String> {
    // Prozesse beenden
    for proc in ONENOTE_PROCESSES {
        let _ = process::run("taskkill", &["/F", "/IM", &format!("{proc}.EXE")]).await;
        let _ = process::run("taskkill", &["/F", "/IM", &format!("{proc}.exe")]).await;
    }

    let ps = format!(
        r#"
$p = [System.Environment]::ExpandEnvironmentVariables('{ONENOTE_CACHE_PATH}')
if (Test-Path $p) {{
    Get-ChildItem $p -Recurse -Force -ErrorAction SilentlyContinue |
        ForEach-Object {{
            $_.Attributes = 'Normal'
            Remove-Item $_.FullName -Recurse -Force -ErrorAction SilentlyContinue
        }}
    Write-Output "OK"
}} else {{
    Write-Output "Path not found"
}}"#
    );

    let result = powershell::run(&ps).await?;
    Ok(format!("OneNote cache cleared: {}", result.trim()))
}
