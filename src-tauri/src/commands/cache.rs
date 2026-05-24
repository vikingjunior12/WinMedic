use crate::services::powershell;
use crate::services::process;

// Exakt dieselben Pfade wie CareumSchulstartTool (OfficeCacheClearService.cs)
const OFFICE_CACHE_PATHS: &[&str] = &[
    r"%LOCALAPPDATA%\Microsoft\Office\16.0\OfficeFileCache",
    r"%LOCALAPPDATA%\Microsoft\Office\16.0\Wef",
    r"%LOCALAPPDATA%\Packages\Microsoft.Win32WebViewHost_cw5n1h2txyewy\AC\#!123\INetCache",
    r"%LOCALAPPDATA%\Microsoft\Outlook\HubAppFileCache",
];

const OFFICE_PROCESSES: &[&str] = &["EXCEL", "WINWORD", "POWERPNT", "OUTLOOK"];

#[tauri::command]
pub async fn clear_office_cache() -> Result<String, String> {
    // Office-Prozesse beenden
    for proc in OFFICE_PROCESSES {
        let _ = process::run("taskkill", &["/F", "/IM", &format!("{proc}.EXE")]).await;
    }

    let paths_ps = OFFICE_CACHE_PATHS
        .iter()
        .map(|p| format!("    '{p}'"))
        .collect::<Vec<_>>()
        .join(",\n");

    let ps = format!(
        r#"
$paths = @(
{paths_ps}
)
foreach ($p in $paths) {{
    $exp = [System.Environment]::ExpandEnvironmentVariables($p)
    if (Test-Path $exp) {{
        Get-ChildItem $exp -Recurse -Force -ErrorAction SilentlyContinue |
            Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
    }}
}}"#
    );

    powershell::run(&ps).await?;
    Ok("Office cache cleared".to_string())
}
