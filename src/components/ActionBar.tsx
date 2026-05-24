import { invoke } from "@tauri-apps/api/core";
import { save } from "@tauri-apps/plugin-dialog";
import { writeTextFile } from "@tauri-apps/plugin-fs";
import { useAppStore } from "../store/useAppStore";
import type { SetupOptions } from "../types/events";

export function ActionBar() {
  const store = useAppStore();

  function buildOptions(): SetupOptions {
    return {
      username: store.username.trim(),
      password: store.password,
      add_to_admins: store.add_to_admins,
      install_office: store.install_office,
      install_teams: store.install_teams,
      install_onenote: store.install_onenote,
      include_access: store.include_access,
      include_publisher: store.include_publisher,
      include_skype_for_business: store.include_skype_for_business,
      office_language: store.office_language,
      office_channel: store.office_channel,
      office_architecture: store.office_architecture,
      office_auto_updates: store.office_auto_updates,
      office_shared_computer_activation: store.office_shared_computer_activation,
      uninstall_office: store.uninstall_office,
      quick_repair: store.quick_repair,
      online_repair: store.online_repair,
      license_reset: store.license_reset,
      clear_office_cache: store.clear_office_cache,
      clear_onenote_cache: store.clear_onenote_cache,
      clear_teams_cache: store.clear_teams_cache,
      uninstall_onedrive: store.uninstall_onedrive,
      install_onedrive: store.install_onedrive,
      winget_packages: store.selectedWingetPackages,
      dry_run: store.dry_run,
    };
  }

  async function startSetup() {
    const options = buildOptions();

    if (options.username && options.password !== store.confirmPassword) {
      alert("Passwords do not match!");
      return;
    }

    try {
      await invoke("validate_setup_options", { options });
    } catch (e) {
      alert(`Validation error: ${e}`);
      return;
    }

    store.resetRunState();
    store.set("isRunning", true);

    try {
      await invoke("run_setup", { options });
    } catch (e) {
      store.addLog({ level: "error", message: String(e), timestamp: new Date().toLocaleTimeString() });
    } finally {
      store.set("isRunning", false);
    }
  }

  async function cancelSetup() {
    await invoke("cancel_setup");
  }

  async function exportLog() {
    const lines = store.logs.map((l) => `[${l.timestamp}] [${l.level.toUpperCase()}] ${l.message}`).join("\n");
    const path = await save({
      defaultPath: `winmedic-log-${new Date().toISOString().slice(0, 10)}.txt`,
      filters: [{ name: "Text file", extensions: ["txt"] }],
    });
    if (path) {
      await writeTextFile(path, lines);
    }
  }

  return (
    <div className="action-bar">
      {!store.isRunning ? (
        <button className="btn-primary" onClick={startSetup}>
          ▶ Start Setup
        </button>
      ) : (
        <button className="btn-danger" onClick={cancelSetup}>
          ⊘ Cancel
        </button>
      )}
      <button
        className="btn-secondary"
        onClick={exportLog}
        disabled={store.logs.length === 0}
      >
        ↓ Export Log
      </button>
    </div>
  );
}
