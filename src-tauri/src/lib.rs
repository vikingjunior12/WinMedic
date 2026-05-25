mod commands;
mod models;
mod services;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            account::check_user_exists,
            account::create_local_user,
            account_cleanup::clear_office_account_cache,
            account_cleanup::remove_workplace_join,
            office::install_office,
            office::uninstall_office,
            office::repair_office_quick,
            office::repair_office_online,
            office::reset_office_license,
            onedrive::uninstall_onedrive,
            onedrive::install_onedrive,
            teams::install_teams,
            teams::uninstall_teams,
            teams::clear_teams_cache,
            onenote::install_onenote_backup_exporter,
            onenote::clear_onenote_cache,
            cache::clear_office_cache,
            winget::check_winget_available,
            winget::get_winget_updates,
            winget::upgrade_winget_package,
            password::generate_password,
            system_info::get_system_info,
            system_info::check_is_admin,
            run_setup::run_setup,
            run_setup::cancel_setup,
            run_setup::validate_setup_options,
        ])
        .run(tauri::generate_context!())
        .expect("Fehler beim Starten der Anwendung");
}
