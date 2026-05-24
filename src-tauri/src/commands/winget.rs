use crate::models::update_entry::WingetEntry;
use crate::services::{powershell, process};
use serde::Deserialize;

#[derive(Deserialize, Default)]
struct WingetOutput {
    #[serde(rename = "Sources", default)]
    sources: Vec<WingetSource>,
}

#[derive(Deserialize, Default)]
struct WingetSource {
    #[serde(rename = "Packages", default)]
    packages: Vec<WingetPackage>,
}

#[derive(Deserialize, Default)]
struct WingetPackage {
    #[serde(rename = "PackageIdentifier", default)]
    id: String,
    #[serde(rename = "Name", default)]
    name: String,
    #[serde(rename = "Version", default)]
    current_version: String,
    #[serde(rename = "AvailableVersion", default)]
    available_version: String,
}

#[tauri::command]
pub async fn check_winget_available() -> bool {
    process::run("winget", &["--version"])
        .await
        .is_ok_and(|r| r.exit_code == 0)
}

#[tauri::command]
pub async fn get_winget_updates() -> Result<Vec<WingetEntry>, String> {
    // winget uses Windows Console APIs that don't work when piped directly
    // with CREATE_NO_WINDOW — running via PowerShell fixes subprocess output capture.
    let output = powershell::run(
        "winget upgrade --output json --accept-source-agreements --disable-interactivity 2>$null",
    )
    .await?;

    let json = extract_json(&output);
    let output: WingetOutput = serde_json::from_str(json).unwrap_or_default();

    let entries = output
        .sources
        .into_iter()
        .flat_map(|s| s.packages)
        .filter(|p| !p.id.is_empty() && p.current_version != p.available_version)
        .map(|p| WingetEntry {
            id: p.id,
            name: p.name,
            current_version: p.current_version,
            available_version: p.available_version,
        })
        .collect();

    Ok(entries)
}

/// Winget sometimes prepends a UTF-8 BOM or stray text before the JSON object.
fn extract_json(s: &str) -> &str {
    let s = s.trim_start_matches('\u{feff}');
    s.find('{').map(|i| &s[i..]).unwrap_or(s)
}

#[tauri::command]
pub async fn upgrade_winget_package(package_id: String) -> Result<String, String> {
    let result = process::run(
        "winget",
        &[
            "upgrade",
            "--id",
            &package_id,
            "--silent",
            "--disable-interactivity",
            "--accept-package-agreements",
            "--accept-source-agreements",
        ],
    )
    .await?;

    if result.exit_code == 0 {
        Ok(format!("{package_id} aktualisiert"))
    } else {
        Err(format!(
            "{package_id}: Update fehlgeschlagen (Exit {})",
            result.exit_code
        ))
    }
}
