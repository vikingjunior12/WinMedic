import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useAppStore } from "../store/useAppStore";
import type { WingetEntry } from "../types/events";

export function WingetSection() {
  const { wingetUpdates, selectedWingetPackages, toggleWingetPackage, set } = useAppStore();
  const [loading, setLoading] = useState(false);

  async function fetchUpdates() {
    setLoading(true);
    try {
      const updates = await invoke<WingetEntry[]>("get_winget_updates");
      set("wingetUpdates", updates);
      set("selectedWingetPackages", updates.map((u) => u.id));
    } catch {
      set("wingetUpdates", []);
    } finally {
      setLoading(false);
    }
  }

  return (
    <section className="card">
      <div className="card-title-row">
        <h2 className="card-title">Winget Updates</h2>
        <button className="btn-sm" onClick={fetchUpdates} disabled={loading}>
          {loading ? "Loading…" : "Refresh"}
        </button>
      </div>

      {wingetUpdates.length === 0 ? (
        <p className="hint">No updates found</p>
      ) : (
        <div className="winget-list">
          {wingetUpdates.map((pkg) => (
            <label key={pkg.id} className="checkbox-row winget-row">
              <input
                type="checkbox"
                checked={selectedWingetPackages.includes(pkg.id)}
                onChange={() => toggleWingetPackage(pkg.id)}
              />
              <div className="winget-info">
                <span className="winget-name">{pkg.name}</span>
                <span className="hint">
                  {pkg.current_version} → {pkg.available_version}
                </span>
              </div>
            </label>
          ))}
        </div>
      )}
    </section>
  );
}
