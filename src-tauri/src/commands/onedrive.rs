use crate::services::process;

#[tauri::command]
pub async fn uninstall_onedrive() -> Result<String, String> {
    let _ = process::run("taskkill", &["/F", "/IM", "OneDrive.exe"]).await;

    // 1. Winget (MSIX + modern installer)
    if let Ok(r) = process::run(
        "winget",
        &[
            "uninstall",
            "--id", "Microsoft.OneDrive",
            "-e",
            "--silent",
            "--accept-source-agreements",
        ],
    )
    .await
    {
        if r.exit_code == 0 {
            return Ok("OneDrive uninstalled via winget".to_string());
        }
    }

    // 2. Fallback: OneDriveSetup.exe via Registry → feste Pfade → rekursive Suche
    let ps = r#"
$setup = $null

# Registry-Uninstall-String
$regPaths = @(
    'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\OneDriveSetup.exe',
    'HKLM:\Software\Microsoft\Windows\CurrentVersion\Uninstall\OneDriveSetup.exe',
    'HKLM:\Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\OneDriveSetup.exe'
)
foreach ($reg in $regPaths) {
    $item = Get-ItemProperty $reg -ErrorAction SilentlyContinue
    if ($item.UninstallString) {
        $candidate = ($item.UninstallString -split ' ')[0].Trim('"')
        if (Test-Path $candidate) { $setup = $candidate; break }
    }
}

# Feste Pfade
if (-not $setup) {
    $fixed = @(
        "$env:SystemRoot\SysWOW64\OneDriveSetup.exe",
        "$env:SystemRoot\System32\OneDriveSetup.exe",
        "$env:ProgramFiles\Microsoft OneDrive\OneDriveSetup.exe",
        "${env:ProgramFiles(x86)}\Microsoft OneDrive\OneDriveSetup.exe",
        "$env:LOCALAPPDATA\Microsoft\OneDrive\OneDriveSetup.exe"
    )
    $setup = $fixed | Where-Object { Test-Path $_ } | Select-Object -First 1
}

# Rekursive Suche in bekannten Verzeichnissen
if (-not $setup) {
    $searchRoots = @(
        "$env:LOCALAPPDATA\Microsoft\OneDrive",
        "$env:ProgramFiles\Microsoft OneDrive",
        "${env:ProgramFiles(x86)}\Microsoft OneDrive"
    )
    foreach ($root in $searchRoots) {
        if (Test-Path $root) {
            $found = Get-ChildItem -Path $root -Recurse -Filter 'OneDriveSetup.exe' -ErrorAction SilentlyContinue |
                Select-Object -First 1 -ExpandProperty FullName
            if ($found) { $setup = $found; break }
        }
    }
}

if (-not $setup) {
    Write-Error 'OneDriveSetup.exe not found and winget unavailable'
    exit 1
}

Write-Output "Using: $setup"
$proc = Start-Process -FilePath $setup -ArgumentList '/uninstall' -Wait -PassThru
exit $proc.ExitCode
"#;

    let result = crate::services::powershell::run(ps).await?;
    Ok(format!("OneDrive uninstalled via setup.exe: {}", result.trim()))
}

#[tauri::command]
pub async fn install_onedrive() -> Result<String, String> {
    // 1. Winget
    if let Ok(r) = process::run(
        "winget",
        &[
            "install",
            "--id", "Microsoft.OneDrive",
            "-e",
            "--silent",
            "--accept-package-agreements",
            "--accept-source-agreements",
        ],
    )
    .await
    {
        if r.exit_code == 0 {
            return Ok("OneDrive installed via winget".to_string());
        }
    }

    // 2. Fallback: direkter Download von Microsoft
    const ONEDRIVE_URL: &str =
        "https://go.microsoft.com/fwlink/?linkid=844652";

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    let bytes = client
        .get(ONEDRIVE_URL)
        .send()
        .await
        .map_err(|e| format!("OneDrive download failed: {e}"))?
        .bytes()
        .await
        .map_err(|e| format!("OneDrive download failed: {e}"))?;

    let installer = std::env::temp_dir().join("winmedic_OneDriveSetup.exe");
    std::fs::write(&installer, &bytes)
        .map_err(|e| format!("Failed to save OneDrive installer: {e}"))?;

    let result = process::run(installer.to_str().unwrap(), &["/silent", "/allusers"]).await?;

    if result.exit_code == 0 {
        Ok("OneDrive installed via direct download".to_string())
    } else {
        Err(format!(
            "OneDrive installation failed (Exit {}): {}",
            result.exit_code,
            result.stderr.trim()
        ))
    }
}
