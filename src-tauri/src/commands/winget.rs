use crate::models::update_entry::WingetEntry;
use crate::services::process;
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
    process::run("winget", &["--version"]).await.is_ok_and(|r| r.exit_code == 0)
}

#[tauri::command]
pub async fn get_winget_updates() -> Result<Vec<WingetEntry>, String> {
    let result = process::run(
        "winget",
        &[
            "upgrade",
            "--output",
            "json",
            "--accept-source-agreements",
        ],
    )
    .await?;

    // Winget gibt auch bei vorhandenen Updates Exit 0 zurück
    let output: WingetOutput = serde_json::from_str(&result.stdout).unwrap_or_default();

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

#[tauri::command]
pub async fn upgrade_winget_package(package_id: String) -> Result<String, String> {
    let result = process::run(
        "winget",
        &[
            "upgrade",
            "--id",
            &package_id,
            "--silent",
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
