import { useAppStore } from "../store/useAppStore";

export function RepairSection() {
  const { uninstall_office, quick_repair, online_repair, license_reset, set } = useAppStore();

  return (
    <section className="card">
      <h2 className="card-title">Office Repair</h2>

      <label
        className="checkbox-row"
        title="Repairs Office files locally without downloading. Fixes crashes and corrupted installations in minutes."
      >
        <input
          type="checkbox"
          checked={quick_repair}
          onChange={(e) => set("quick_repair", e.target.checked)}
        />
        <div>
          <span>Quick Repair</span>
          <span className="hint">Local, no internet required</span>
        </div>
      </label>

      <label
        className="checkbox-row"
        title="Removes Office completely and reinstalls via ODT. Fixes deep corruption – takes longer and requires internet."
      >
        <input
          type="checkbox"
          checked={online_repair}
          onChange={(e) => set("online_repair", e.target.checked)}
        />
        <div>
          <span>Online Repair</span>
          <span className="hint">Uninstalls and reinstalls Office</span>
        </div>
      </label>

      <label
        className="checkbox-row"
        title="Removes all installed product keys via ospp.vbs. Useful for licensing issues or before switching accounts."
      >
        <input
          type="checkbox"
          checked={license_reset}
          onChange={(e) => set("license_reset", e.target.checked)}
        />
        <span>Reset License (ospp.vbs)</span>
      </label>

      <div className="divider" />

      <label
        className="checkbox-row checkbox-row--danger"
        title="Removes Office completely from the system. No rollback possible – Office must be reinstalled afterwards."
      >
        <input
          type="checkbox"
          checked={uninstall_office}
          onChange={(e) => set("uninstall_office", e.target.checked)}
        />
        <div>
          <span>Uninstall Office completely</span>
          <span className="hint hint--danger">Removes Office entirely – no rollback possible</span>
        </div>
      </label>
    </section>
  );
}
