import { useAppStore } from "../store/useAppStore";

export function ProfileSelector() {
  const { profiles, activeProfileId, applyProfile, dry_run, set } = useAppStore();

  return (
    <section className="card">
      <h2 className="card-title">Profiles</h2>
      <div className="profile-grid">
        {profiles.map((p) => (
          <button
            key={p.id}
            className={`profile-btn ${activeProfileId === p.id ? "profile-btn--active" : ""}`}
            onClick={() => applyProfile(p.id)}
          >
            {p.name}
          </button>
        ))}
      </div>
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
