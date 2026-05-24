use crate::services::{network, powershell, process};
use serde::Deserialize;
use std::path::PathBuf;

// Eingebettete echte ODT setup.exe (6.8 MB, aus officedeploymenttool_*.exe /extract extrahiert)
const ODT_SETUP: &[u8] = include_bytes!("../../resources/setup.exe");

const OFFICE_REMOVE_XML: &str = r#"<Configuration>
  <Remove All="TRUE"/>
  <Display Level="None" AcceptEULA="TRUE"/>
</Configuration>"#;

const OSPP_PATHS: &[&str] = &[
    r"C:\Program Files\Microsoft Office\Office16\ospp.vbs",
    r"C:\Program Files (x86)\Microsoft Office\Office16\ospp.vbs",
    r"C:\Program Files\Microsoft Office\root\Office16\ospp.vbs",
    r"C:\Program Files (x86)\Microsoft Office\root\Office16\ospp.vbs",
];

#[derive(Debug, Deserialize)]
pub struct OfficeInstallOptions {
    pub include_access: bool,
    pub include_publisher: bool,
    pub include_skype_for_business: bool,
    pub language: String,
}

pub fn build_install_xml(opts: &OfficeInstallOptions) -> String {
    let lang = if opts.language.is_empty() { "de-de" } else { &opts.language };

    let mut excluded = vec!["Groove"];
    if !opts.include_access {
        excluded.push("Access");
    }
    if !opts.include_publisher {
        excluded.push("Publisher");
    }
    if !opts.include_skype_for_business {
        excluded.push("Lync");
    }

    let exclude_lines: String = excluded
        .iter()
        .map(|app| format!("      <ExcludeApp ID=\"{app}\" />\n"))
        .collect();

    format!(
        r#"<Configuration>
  <RemoveMSI All="True" />
  <Add OfficeClientEdition="64" Channel="Current">
    <Product ID="O365ProPlusRetail">
      <Language ID="{lang}" />
{exclude_lines}    </Product>
  </Add>
  <Updates Enabled="TRUE" Channel="Current" />
  <Display Level="None" AcceptEULA="TRUE" />
  <Property Name="AUTOACTIVATE" Value="1" />
  <Property Name="FORCEAPPSHUTDOWN" Value="TRUE" />
</Configuration>"#
    )
}

fn extract_odt() -> Result<PathBuf, String> {
    let path = std::env::temp_dir().join("winmedic_setup.exe");
    std::fs::write(&path, ODT_SETUP)
        .map_err(|e| format!("Failed to extract setup.exe: {e}"))?;
    Ok(path)
}

fn write_xml(filename: &str, content: &str) -> Result<PathBuf, String> {
    let path = std::env::temp_dir().join(filename);
    std::fs::write(&path, content)
        .map_err(|e| format!("Failed to write XML: {e}"))?;
    Ok(path)
}

fn find_c2r_client() -> Option<String> {
    let paths = [
        r"C:\Program Files\Common Files\Microsoft Shared\ClickToRun\OfficeC2RClient.exe",
        r"C:\Program Files (x86)\Common Files\Microsoft Shared\ClickToRun\OfficeC2RClient.exe",
    ];
    paths
        .iter()
        .find(|p| std::path::Path::new(p).exists())
        .map(|s| s.to_string())
}

#[tauri::command]
pub async fn install_office(options: OfficeInstallOptions) -> Result<String, String> {
    if !network::check_internet().await {
        return Err("No internet connection – Office installation not possible".to_string());
    }

    let odt = extract_odt()?;
    let xml = build_install_xml(&options);
    let xml_path = write_xml("winmedic_install.xml", &xml)?;

    let result = process::run(
        odt.to_str().unwrap(),
        &["/configure", xml_path.to_str().unwrap()],
    )
    .await?;

    if result.exit_code == 0 {
        Ok("Office installed successfully".to_string())
    } else {
        Err(format!(
            "Office installation failed (Exit {}): {}",
            result.exit_code, result.stderr
        ))
    }
}

#[tauri::command]
pub async fn repair_office_quick() -> Result<String, String> {
    let c2r = find_c2r_client()
        .ok_or_else(|| "OfficeC2RClient.exe not found – is Office installed?".to_string())?;

    let result = process::run(
        &c2r,
        &["/repair", "level=QuickRepair", "forceappshutdown=true", "displaylevel=true"],
    )
    .await?;

    if result.exit_code == 0 {
        Ok("Office Quick Repair completed".to_string())
    } else {
        Err(format!("Quick Repair failed (Exit {})", result.exit_code))
    }
}

#[tauri::command]
pub async fn repair_office_online(options: OfficeInstallOptions) -> Result<String, String> {
    if !network::check_internet().await {
        return Err("No internet – Online Repair requires a network connection".to_string());
    }

    let odt = extract_odt()?;

    // Schritt 1: Entfernen
    let remove_xml = write_xml("winmedic_remove.xml", OFFICE_REMOVE_XML)?;
    let remove = process::run(
        odt.to_str().unwrap(),
        &["/configure", remove_xml.to_str().unwrap()],
    )
    .await?;

    if remove.exit_code != 0 {
        return Err(format!(
            "Office uninstallation failed (Exit {})",
            remove.exit_code
        ));
    }

    // Nochmals Internet prüfen – kein Rollback nach Remove möglich
    if !network::check_internet().await {
        return Err(
            "No internet after uninstallation – reinstallation aborted"
                .to_string(),
        );
    }

    // Schritt 2: Neuinstallation
    let install_xml_path = write_xml("winmedic_install.xml", &build_install_xml(&options))?;
    let install = process::run(
        odt.to_str().unwrap(),
        &["/configure", install_xml_path.to_str().unwrap()],
    )
    .await?;

    if install.exit_code == 0 {
        Ok("Office Online Repair completed".to_string())
    } else {
        Err(format!(
            "Office reinstallation failed (Exit {})",
            install.exit_code
        ))
    }
}

#[tauri::command]
pub async fn uninstall_office() -> Result<String, String> {
    let odt = extract_odt()?;
    let remove_xml = write_xml("winmedic_remove.xml", OFFICE_REMOVE_XML)?;

    let result = process::run(
        odt.to_str().unwrap(),
        &["/configure", remove_xml.to_str().unwrap()],
    )
    .await?;

    if result.exit_code == 0 {
        Ok("Office uninstalled completely".to_string())
    } else {
        Err(format!(
            "Office uninstallation failed (Exit {}): {}",
            result.exit_code, result.stderr
        ))
    }
}

#[tauri::command]
pub async fn reset_office_license() -> Result<String, String> {
    let ospp = OSPP_PATHS
        .iter()
        .find(|&&p| std::path::Path::new(p).exists())
        .ok_or_else(|| "ospp.vbs not found – is Office installed?".to_string())?;

    // Installierte Keys auslesen
    let dstatus = powershell::run(&format!(r#"cscript.exe "{ospp}" /dstatus"#)).await?;

    // Letzte 5 Zeichen jedes Keys extrahieren und entfernen
    let keys: Vec<String> = dstatus
        .lines()
        .filter_map(|line| {
            if line.contains("Last 5 characters of installed product key:") {
                line.split_whitespace().last().map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect();

    if keys.is_empty() {
        return Ok("No installed product keys found".to_string());
    }

    for key in &keys {
        powershell::run(&format!(r#"cscript.exe "{ospp}" /unpkey:{key}"#)).await?;
    }

    Ok(format!("License reset complete – {} key(s) removed", keys.len()))
}
