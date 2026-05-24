import { useAppStore } from "../store/useAppStore";

export function SoftwareSection() {
  const store = useAppStore();

  return (
    <section className="card">
      <h2 className="card-title">Install Software</h2>

      <label
        className="checkbox-row"
        title="Installs Office 365 via ODT. Existing Click-to-Run Office is upgraded in-place; old MSI versions (2013/2016) are removed automatically."
      >
        <input
          type="checkbox"
          checked={store.install_office}
          onChange={(e) => store.set("install_office", e.target.checked)}
        />
        <span>Microsoft Office 365 (ODT)</span>
      </label>

      {store.install_office && (
        <div className="sub-options">
          <span className="sub-options-label">Include Office components:</span>
          <label className="checkbox-row">
            <input
              type="checkbox"
              checked={store.include_access}
              onChange={(e) => store.set("include_access", e.target.checked)}
            />
            <div>
              <span>Microsoft Access</span>
              <span className="hint">Default: not installed</span>
            </div>
          </label>
          <label className="checkbox-row">
            <input
              type="checkbox"
              checked={store.include_publisher}
              onChange={(e) => store.set("include_publisher", e.target.checked)}
            />
            <div>
              <span>Microsoft Publisher</span>
              <span className="hint">Default: not installed</span>
            </div>
          </label>
          <label className="checkbox-row">
            <input
              type="checkbox"
              checked={store.include_skype_for_business}
              onChange={(e) => store.set("include_skype_for_business", e.target.checked)}
            />
            <div>
              <span>Skype for Business</span>
              <span className="hint">Default: not installed</span>
            </div>
          </label>
        </div>
      )}

      <label className="checkbox-row">
        <input
          type="checkbox"
          checked={store.install_teams}
          onChange={(e) => store.set("install_teams", e.target.checked)}
        />
        <span>Microsoft Teams (New)</span>
      </label>

      <label className="checkbox-row">
        <input
          type="checkbox"
          checked={store.install_onenote}
          onChange={(e) => store.set("install_onenote", e.target.checked)}
        />
        <span>OneNote Backup Exporter</span>
      </label>
    </section>
  );
}
