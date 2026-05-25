import { useAppStore } from "../store/useAppStore";

export function AccountCleanupSection() {
  const { clear_office_account_cache, remove_workplace_join, set } = useAppStore();

  return (
    <section className="card">
      <h2 className="card-title">Account Cleanup</h2>

      <label
        className="checkbox-row"
        title={
          "Removes stuck Office/Teams work or school accounts:\n" +
          "• Kills Office + Teams processes\n" +
          "• Deletes HKCU:\\...\\Office\\16.0\\Common\\Identity + Licensing\n" +
          "• Clears %LOCALAPPDATA%\\Microsoft\\OneAuth\n" +
          "• Clears %LOCALAPPDATA%\\Microsoft\\IdentityCache\n" +
          "• Clears %LOCALAPPDATA%\\Microsoft\\TokenBroker\n" +
          "• Removes Office/Teams entries from Windows Credential Manager"
        }
      >
        <input
          type="checkbox"
          checked={clear_office_account_cache}
          onChange={(e) => set("clear_office_account_cache", e.target.checked)}
        />
        <div>
          <span>Clear Office Account Cache</span>
          <span className="hint">Removes cached work/school account tokens and credentials</span>
        </div>
      </label>

      <div className="divider" />

      <label
        className="checkbox-row checkbox-row--danger"
        title={
          "Removes the Azure AD / Entra ID Workplace Join registration.\n" +
          "Only runs if dsregcmd /status shows WorkplaceJoined=YES.\n\n" +
          "WARNING: Disconnects SSO for all cloud apps in this session.\n" +
          "Do NOT use on primary managed work devices."
        }
      >
        <input
          type="checkbox"
          checked={remove_workplace_join}
          onChange={(e) => set("remove_workplace_join", e.target.checked)}
        />
        <div>
          <span>Remove Workplace Join</span>
          <span className="hint hint--danger">Only for stuck Entra ID / Azure AD registrations</span>
        </div>
      </label>
    </section>
  );
}
