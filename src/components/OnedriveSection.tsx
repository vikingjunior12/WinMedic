import { useAppStore } from "../store/useAppStore";

export function OnedriveSection() {
  const { uninstall_onedrive, install_onedrive, set } = useAppStore();

  return (
    <section className="card">
      <h2 className="card-title">OneDrive</h2>
      <label
        className="checkbox-row"
        title="Terminates OneDrive and runs OneDriveSetup.exe /uninstall"
      >
        <input
          type="checkbox"
          checked={uninstall_onedrive}
          onChange={(e) => set("uninstall_onedrive", e.target.checked)}
        />
        <span>Uninstall OneDrive</span>
      </label>
      <label
        className="checkbox-row"
        title="Installs OneDrive via winget (Microsoft.OneDrive)"
      >
        <input
          type="checkbox"
          checked={install_onedrive}
          onChange={(e) => set("install_onedrive", e.target.checked)}
        />
        <span>Install OneDrive</span>
      </label>
    </section>
  );
}
