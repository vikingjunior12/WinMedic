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
          <span className="sub-options-label">Language:</span>
          <select
            className="select-row"
            value={store.office_language}
            onChange={(e) => store.set("office_language", e.target.value)}
          >
            <option value="de-de">Deutsch (de-de)</option>
            <option value="en-us">English US (en-us)</option>
            <option value="en-gb">English UK (en-gb)</option>
            <option value="fr-fr">Français (fr-fr)</option>
            <option value="it-it">Italiano (it-it)</option>
            <option value="es-es">Español (es-es)</option>
            <option value="nl-nl">Nederlands (nl-nl)</option>
            <option value="pl-pl">Polski (pl-pl)</option>
            <option value="pt-pt">Português (pt-pt)</option>
            <option value="tr-tr">Türkçe (tr-tr)</option>
          </select>

          <span className="sub-options-label">Update Channel:</span>
          <select
            className="select-row"
            value={store.office_channel}
            onChange={(e) => store.set("office_channel", e.target.value)}
          >
            <option value="Current">Current – monthly, latest features</option>
            <option value="MonthlyEnterprise">Monthly Enterprise – monthly, 1 month behind</option>
            <option value="SemiAnnualEnterprise">Semi-Annual Enterprise – twice a year, most stable</option>
            <option value="SemiAnnualEnterprisePreview">Semi-Annual Enterprise Preview</option>
            <option value="BetaChannel">Beta Channel – insider builds</option>
          </select>

          <span className="sub-options-label">Architecture:</span>
          <select
            className="select-row"
            value={store.office_architecture}
            onChange={(e) => store.set("office_architecture", e.target.value)}
          >
            <option value="64">64-bit (recommended)</option>
            <option value="32">32-bit (legacy add-ins only)</option>
          </select>

          <label className="checkbox-row" title="Disable if updates are managed centrally (WSUS / Intune)">
            <input
              type="checkbox"
              checked={store.office_auto_updates}
              onChange={(e) => store.set("office_auto_updates", e.target.checked)}
            />
            <div>
              <span>Auto-Updates</span>
              <span className="hint">Disable for centrally managed environments</span>
            </div>
          </label>

          <label className="checkbox-row" title="Required for RDS / Terminal Server (shared sessions)">
            <input
              type="checkbox"
              checked={store.office_shared_computer_activation}
              onChange={(e) => store.set("office_shared_computer_activation", e.target.checked)}
            />
            <div>
              <span>Shared Computer Activation</span>
              <span className="hint">For RDS / Terminal Server</span>
            </div>
          </label>

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
