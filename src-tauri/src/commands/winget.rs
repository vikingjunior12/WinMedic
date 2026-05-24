use crate::models::update_entry::WingetEntry;
use crate::services::process;

#[tauri::command]
pub async fn check_winget_available() -> bool {
    process::run("winget", &["--version"])
        .await
        .is_ok_and(|r| r.exit_code == 0)
}

#[tauri::command]
pub async fn get_winget_updates() -> Result<Vec<WingetEntry>, String> {
    // Run winget directly (same as C# CareumSupportTool approach): CreateNoWindow +
    // redirected stdout. Avoid PowerShell wrapper — it swallows winget's output.
    // --include-unknown: also list packages where installed version is unknown ("< x.y.z").
    let result = process::run(
        "winget",
        &[
            "upgrade",
            "--include-unknown",
            "--accept-source-agreements",
            "--disable-interactivity",
        ],
    )
    .await?;

    Ok(parse_winget_table(&result.stdout))
}

/// Parses winget's fixed-width table output (works for DE/EN/FR locales).
///
/// Strategy: find the separator line (all dashes), use the header above it
/// to locate the "ID" column, then split each data row at that offset.
/// Everything after the ID column is space-delimited tokens: ID, version,
/// available — handling the "< X.Y.Z" unknown-version format.
fn parse_winget_table(output: &str) -> Vec<WingetEntry> {
    let lines: Vec<&str> = output
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .collect();

    // Separator line: long run of dashes (language-independent)
    let Some(sep_idx) = lines.iter().position(|l| {
        let t = l.trim();
        t.len() > 20 && t.bytes().all(|b| b == b'-')
    }) else {
        return vec![];
    };

    if sep_idx == 0 {
        return vec![];
    }

    // "ID" column header is always "ID" regardless of locale
    let header = lines[sep_idx - 1];
    let Some(id_col) = header.find("ID") else {
        return vec![];
    };

    lines[sep_idx + 1..]
        .iter()
        .take_while(|l| {
            let t = l.trim();
            // Stop at summary line ("6 Aktualisierungen verfügbar" / "6 upgrades available")
            !t.is_empty() && !t.starts_with(|c: char| c.is_ascii_digit())
        })
        .filter_map(|line| {
            if line.len() <= id_col {
                return None;
            }
            let name = line[..id_col].trim().to_string();
            let tokens: Vec<&str> = line[id_col..].split_whitespace().collect();

            // tokens: [ID, version_or_"<", ...]
            // "< X.Y.Z" means installed version is unknown
            let (id, current_version, available_version) = match tokens.as_slice() {
                [id, ver, avail, ..] if *ver != "<" => {
                    (id.to_string(), ver.to_string(), avail.to_string())
                }
                [id, _, ver, avail, ..] => {
                    (id.to_string(), format!("< {ver}"), avail.to_string())
                }
                _ => return None,
            };

            if id.is_empty() || available_version.is_empty() {
                return None;
            }

            Some(WingetEntry { id, name, current_version, available_version })
        })
        .collect()
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
