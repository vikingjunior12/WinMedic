#![allow(dead_code)]
use winreg::{enums::*, RegKey};

pub fn read_string(hive: winreg::HKEY, path: &str, name: &str) -> Option<String> {
    let key = RegKey::predef(hive).open_subkey(path).ok()?;
    key.get_value(name).ok()
}

pub fn key_exists(hive: winreg::HKEY, path: &str) -> bool {
    RegKey::predef(hive).open_subkey(path).is_ok()
}

/// Sucht den Office-Installationspfad aus der Registry.
pub fn find_office_install_path() -> Option<String> {
    let paths = [
        r"SOFTWARE\Microsoft\Office\ClickToRun\Configuration",
        r"SOFTWARE\WOW6432Node\Microsoft\Office\ClickToRun\Configuration",
    ];

    for path in &paths {
        if let Some(val) = read_string(HKEY_LOCAL_MACHINE, path, "InstallationPath") {
            if !val.is_empty() {
                return Some(val);
            }
        }
    }
    None
}
