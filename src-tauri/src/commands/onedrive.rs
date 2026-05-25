use crate::services::process;

fn winget_says_not_installed(r: &process::ProcessResult) -> bool {
    let combined = format!("{} {}", r.stdout, r.stderr).to_lowercase();
    combined.contains("no installed package found")
        || combined.contains("no package found")
        || combined.contains("nicht gefunden")
}

#[tauri::command]
pub async fn uninstall_onedrive() -> Result<String, String> {
    let _ = process::run("taskkill", &["/F", "/IM", "OneDrive.exe"]).await;

    // 1. Winget (MSIX + modern installer)
    match process::run(
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
        Ok(r) if r.exit_code == 0 => return Ok("OneDrive uninstalled via winget".to_string()),
        Ok(r) if winget_says_not_installed(&r) => return Ok("OneDrive is not installed – nothing to do".to_string()),
        _ => {}
    }

    // 2. Fallback: OneDriveSetup.exe via Registry → feste Pfade → rekursive Suche
    let ps = r#"
$setup = $null

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
    Write-Output 'NOT_INSTALLED'
    exit 0
}

Write-Output "Using: $setup"
$proc = Start-Process -FilePath $setup -ArgumentList '/uninstall' -Wait -PassThru
exit $proc.ExitCode
"#;

    let result = crate::services::powershell::run(ps).await?;
    if result.trim() == "NOT_INSTALLED" {
        Ok("OneDrive is not installed – nothing to do".to_string())
    } else {
        Ok(format!("OneDrive uninstalled via setup.exe: {}", result.trim()))
    }
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
    const ONEDRIVE_URL: &str = "https://go.microsoft.com/fwlink/?linkid=844652";

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
