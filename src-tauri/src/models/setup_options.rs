use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SetupOptions {
    pub username: String,
    password: String,
    pub add_to_admins: bool,
    // Software
    pub install_office: bool,
    pub install_teams: bool,
    pub install_onenote: bool,
    // Office-Komponenten (XML-Steuerung)
    pub include_access: bool,
    pub include_publisher: bool,
    pub include_skype_for_business: bool,
    // Repair / Deinstallation
    pub uninstall_office: bool,
    pub quick_repair: bool,
    pub online_repair: bool,
    pub license_reset: bool,
    // Cache
    pub clear_office_cache: bool,
    pub clear_onenote_cache: bool,
    pub clear_teams_cache: bool,
    // OneDrive
    pub uninstall_onedrive: bool,
    pub install_onedrive: bool,
    // Winget
    pub winget_packages: Vec<String>,
    pub dry_run: bool,
}

impl SetupOptions {
    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn office_install_options(&self) -> crate::commands::office::OfficeInstallOptions {
        crate::commands::office::OfficeInstallOptions {
            include_access: self.include_access,
            include_publisher: self.include_publisher,
            include_skype_for_business: self.include_skype_for_business,
        }
    }
}
