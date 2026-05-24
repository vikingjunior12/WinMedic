use serde::Serialize;
use sysinfo::{Disks, System};

#[derive(Serialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub total_memory_gb: f64,
    pub used_memory_gb: f64,
    pub disk_total_gb: f64,
    pub disk_free_gb: f64,
    pub ip_address: String,
}

#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    let mut sys = System::new();
    sys.refresh_memory();

    let total_memory_gb = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_memory_gb = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

    let disks = Disks::new_with_refreshed_list();
    let (disk_total_gb, disk_free_gb) = disks
        .list()
        .iter()
        .find(|d| d.mount_point().to_str().map_or(false, |s| s.starts_with('C')))
        .map(|d| {
            (
                d.total_space() as f64 / 1024.0 / 1024.0 / 1024.0,
                d.available_space() as f64 / 1024.0 / 1024.0 / 1024.0,
            )
        })
        .unwrap_or((0.0, 0.0));

    let ip_address = local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    SystemInfo {
        hostname: System::host_name().unwrap_or_else(|| "unknown".to_string()),
        os_name: System::name().unwrap_or_else(|| "Windows".to_string()),
        os_version: System::os_version().unwrap_or_else(|| "unknown".to_string()),
        total_memory_gb: (total_memory_gb * 10.0).round() / 10.0,
        used_memory_gb: (used_memory_gb * 10.0).round() / 10.0,
        disk_total_gb: (disk_total_gb * 10.0).round() / 10.0,
        disk_free_gb: (disk_free_gb * 10.0).round() / 10.0,
        ip_address,
    }
}

#[tauri::command]
pub fn check_is_admin() -> bool {
    // Auf Windows: Versuche einen Admin-only Schlüssel zu öffnen
    #[cfg(target_os = "windows")]
    {
        use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
        RegKey::predef(HKEY_LOCAL_MACHINE)
            .create_subkey(r"SOFTWARE\WinMedicAdminCheck")
            .map(|_| {
                let _ = RegKey::predef(HKEY_LOCAL_MACHINE)
                    .delete_subkey(r"SOFTWARE\WinMedicAdminCheck");
                true
            })
            .unwrap_or(false)
    }
    #[cfg(not(target_os = "windows"))]
    false
}
