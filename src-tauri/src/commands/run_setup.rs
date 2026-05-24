use crate::commands::{account, cache, office, onedrive, onenote, teams, winget};
use crate::models::setup_options::SetupOptions;
use crate::models::step_result::{LogEntry, StepResult};
use chrono::Local;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter};

static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

fn emit_log(app: &AppHandle, level: &str, message: &str) {
    let entry = LogEntry {
        level: level.to_string(),
        message: message.to_string(),
        timestamp: Local::now().format("%H:%M:%S").to_string(),
    };
    app.emit("setup-log", &entry).ok();
}

fn emit_progress(app: &AppHandle, done: usize, total: usize) {
    if total == 0 {
        return;
    }
    let progress = done as f64 / total as f64;
    app.emit("setup-progress", progress).ok();
}


#[tauri::command]
pub fn cancel_setup() {
    CANCEL_FLAG.store(true, Ordering::SeqCst);
}

#[tauri::command]
pub fn validate_setup_options(options: SetupOptions) -> Result<(), String> {
    if options.install_office
        || options.install_teams
        || options.install_onenote
        || options.uninstall_office
        || options.quick_repair
        || options.online_repair
        || options.license_reset
        || options.clear_office_cache
        || options.clear_onenote_cache
        || options.clear_teams_cache
        || !options.winget_packages.is_empty()
        || options.uninstall_onedrive
        || options.install_onedrive
    {
        // Mindestens eine Aktion ausgewählt
    } else if options.username.is_empty() {
        return Err("At least one action or a username must be provided".to_string());
    }

    if !options.username.is_empty() {
        if options.password().is_empty() {
            return Err("Password must not be empty".to_string());
        }
        if options.password().len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }
    }

    Ok(())
}

macro_rules! step {
    ($app:expr, $steps:expr, $done:expr, $total:expr, $name:expr, $dry_run:expr, $action:expr) => {{
        if CANCEL_FLAG.load(Ordering::SeqCst) {
            $steps.push(StepResult::cancelled($name));
            $app.emit("setup-cancelled", ()).ok();
            return Ok($steps);
        }
        emit_log(&$app, "info", &format!("Starting: {}", $name));
        if $dry_run {
            emit_log(&$app, "debug", &format!("[Dry Run] {}", $name));
            $steps.push(StepResult::success($name, "Dry Run"));
        } else {
            match $action.await {
                Ok(msg) => {
                    emit_log(&$app, "success", &format!("{}: {}", $name, msg));
                    $steps.push(StepResult::success($name, &msg));
                }
                Err(e) => {
                    emit_log(&$app, "error", &format!("{}: {}", $name, e));
                    $steps.push(StepResult::failed($name, &e));
                }
            }
        }
        $done += 1;
        emit_progress(&$app, $done, $total);
    }};
}

#[tauri::command]
pub async fn run_setup(app: AppHandle, options: SetupOptions) -> Result<Vec<StepResult>, String> {
    CANCEL_FLAG.store(false, Ordering::SeqCst);
    validate_setup_options_inner(&options)?;

    let dry_run = options.dry_run;

    // Gesamtzahl der aktiven Schritte berechnen
    let total = {
        let mut n = 0usize;
        if !options.username.is_empty() { n += 1; }
        if options.install_office { n += 1; }
        if options.install_teams { n += 1; }
        if options.install_onenote { n += 1; }
        if options.uninstall_office { n += 1; }
        if options.quick_repair { n += 1; }
        if options.online_repair { n += 1; }
        if options.license_reset { n += 1; }
        if options.clear_office_cache { n += 1; }
        if options.clear_onenote_cache { n += 1; }
        if options.clear_teams_cache { n += 1; }
        if options.uninstall_onedrive { n += 1; }
        if options.install_onedrive { n += 1; }
        n += options.winget_packages.len();
        n
    };

    let mut steps: Vec<StepResult> = Vec::new();
    let mut done = 0usize;

    emit_log(&app, "info", &format!("Setup started ({total} steps)"));
    emit_progress(&app, 0, total);

    // 1. Benutzerkonto
    if !options.username.is_empty() {
        let username = options.username.clone();
        let password = options.password().to_string();
        let add_to_admins = options.add_to_admins;
        step!(app, steps, done, total, "Create user account", dry_run, async {
            account::create_local_user(username, password, add_to_admins).await
        });
    }

    // 2. Office installieren
    if options.install_office {
        let office_opts = options.office_install_options();
        step!(app, steps, done, total, "Install Office", dry_run, async {
            office::install_office(office_opts).await
        });
    }

    // 3. Teams installieren
    if options.install_teams {
        step!(app, steps, done, total, "Install Teams", dry_run, async {
            teams::install_teams().await
        });
    }

    // 4. OneNote installieren
    if options.install_onenote {
        step!(app, steps, done, total, "OneNote Backup Exporter", dry_run, async {
            onenote::install_onenote_backup_exporter().await
        });
    }

    // 5a. Office deinstallieren
    if options.uninstall_office {
        step!(app, steps, done, total, "Uninstall Office", dry_run, async {
            office::uninstall_office().await
        });
    }

    // 5. Quick Repair
    if options.quick_repair {
        step!(app, steps, done, total, "Office Quick Repair", dry_run, async {
            office::repair_office_quick().await
        });
    }

    // 6. Online Repair
    if options.online_repair {
        let office_opts = options.office_install_options();
        step!(app, steps, done, total, "Office Online Repair", dry_run, async {
            office::repair_office_online(office_opts).await
        });
    }

    // 7. Lizenz-Reset
    if options.license_reset {
        step!(app, steps, done, total, "Office License Reset", dry_run, async {
            office::reset_office_license().await
        });
    }

    // 8. Office-Cache
    if options.clear_office_cache {
        step!(app, steps, done, total, "Clear Office Cache", dry_run, async {
            cache::clear_office_cache().await
        });
    }

    // 9. OneNote-Cache
    if options.clear_onenote_cache {
        step!(app, steps, done, total, "Clear OneNote Cache", dry_run, async {
            onenote::clear_onenote_cache().await
        });
    }

    // 10. Teams-Cache
    if options.clear_teams_cache {
        step!(app, steps, done, total, "Clear Teams Cache", dry_run, async {
            teams::clear_teams_cache().await
        });
    }

    // 11. OneDrive deinstallieren
    if options.uninstall_onedrive {
        step!(app, steps, done, total, "Uninstall OneDrive", dry_run, async {
            onedrive::uninstall_onedrive().await
        });
    }

    // 12. OneDrive installieren
    if options.install_onedrive {
        step!(app, steps, done, total, "Install OneDrive", dry_run, async {
            onedrive::install_onedrive().await
        });
    }

    // 13. Winget-Updates
    for pkg_id in &options.winget_packages {
        let pkg = pkg_id.clone();
        let name = format!("Update: {pkg}");
        step!(app, steps, done, total, &name, dry_run, async {
            winget::upgrade_winget_package(pkg).await
        });
    }

    emit_log(&app, "success", "Setup complete");
    app.emit("setup-complete", &steps).ok();

    Ok(steps)
}

fn validate_setup_options_inner(options: &SetupOptions) -> Result<(), String> {
    if !options.username.is_empty() {
        if options.password().is_empty() {
            return Err("Password must not be empty".to_string());
        }
        if options.password().len() < 8 {
            return Err("Password must be at least 8 characters long".to_string());
        }
    }
    Ok(())
}
