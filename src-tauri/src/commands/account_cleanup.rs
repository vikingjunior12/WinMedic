use crate::services::process;

#[tauri::command]
pub async fn clear_office_account_cache() -> Result<String, String> {
    // 1. Office- und Teams-Prozesse beenden
    let procs = &[
        "WINWORD", "EXCEL", "POWERPNT", "OUTLOOK", "ONENOTE",
        "MSACCESS", "MSPUB", "Teams", "ms-teams", "OneDrive",
    ];
    for p in procs {
        let _ = process::run("taskkill", &["/F", "/IM", &format!("{p}.exe")]).await;
    }

    let ps = r#"
$removed = [System.Collections.Generic.List[string]]::new()

# Registry: Office Identity + Licensing
$regKeys = @(
    'HKCU:\Software\Microsoft\Office\16.0\Common\Identity',
    'HKCU:\Software\Microsoft\Office\16.0\Common\Licensing'
)
foreach ($key in $regKeys) {
    if (Test-Path $key) {
        Remove-Item $key -Recurse -Force -ErrorAction SilentlyContinue
        $removed.Add("Registry: $key")
    }
}

# Token-Cache-Ordner
$folders = @(
    "$env:LOCALAPPDATA\Microsoft\OneAuth",
    "$env:LOCALAPPDATA\Microsoft\IdentityCache",
    "$env:LOCALAPPDATA\Microsoft\TokenBroker"
)
foreach ($f in $folders) {
    if (Test-Path $f) {
        Remove-Item $f -Recurse -Force -ErrorAction SilentlyContinue
        $removed.Add("Folder: $f")
    }
}

# Credential Manager: Office/Teams Eintraege entfernen
$credList = cmdkey /list 2>$null
$targets = $credList | Select-String 'Target:' | ForEach-Object {
    ($_ -replace '.*Target:\s*', '').Trim()
} | Where-Object {
    $_ -match 'MicrosoftOffice|MicrosoftTeams|ADAL|MSAL|OC1|Office16|LegacyGeneric:target=microsoft'
}
foreach ($t in $targets) {
    cmdkey /delete:$t 2>$null | Out-Null
    $removed.Add("Credential: $t")
}

if ($removed.Count -eq 0) {
    Write-Output "Nothing to clean – no cached Office accounts found"
} else {
    Write-Output "Cleaned $($removed.Count) item(s):`n$($removed -join "`n")"
}
"#;

    let result = crate::services::powershell::run(ps).await?;
    Ok(result.trim().to_string())
}

#[tauri::command]
pub async fn remove_workplace_join() -> Result<String, String> {
    // Status prüfen
    let status = process::run("dsregcmd", &["/status"]).await?;
    let output = format!("{} {}", status.stdout, status.stderr);

    let workplace_joined = output
        .lines()
        .any(|l| l.contains("WorkplaceJoined") && l.contains("YES"));

    if !workplace_joined {
        return Ok("Device is not Workplace Joined – nothing to do".to_string());
    }

    let result = process::run("dsregcmd", &["/leave"]).await?;
    if result.exit_code == 0 {
        Ok("Workplace Join removed successfully – restart recommended".to_string())
    } else {
        Err(format!(
            "dsregcmd /leave failed (Exit {}): {}",
            result.exit_code,
            result.stderr.trim()
        ))
    }
}
