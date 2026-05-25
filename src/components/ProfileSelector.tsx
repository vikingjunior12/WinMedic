import { useAppStore } from "../store/useAppStore";

export function ProfileSelector() {
  const { profiles, activeProfileId, applyProfile, dry_run, set } = useAppStore();

  return (
    <section className="card">
      <h2 className="card-title">Profiles</h2>
      <select
        className="select-row"
        value={activeProfileId ?? ""}
        onChange={(e) => {
          if (e.target.value) applyProfile(e.target.value);
        }}
      >
        <option value="" disabled>Select a profile…</option>
        {profiles.map((p) => (
          <option key={p.id} value={p.id}>{p.name}</option>
        ))}
      </select>
      <label className="checkbox-row mt-sm">
        <input
          type="checkbox"
          checked={dry_run}
          onChange={(e) => set("dry_run", e.target.checked)}
        />
        <span>Dry Run (no real changes)</span>
      </label>
    </section>
  );
}
