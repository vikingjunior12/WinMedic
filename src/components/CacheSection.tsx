import { useAppStore } from "../store/useAppStore";

export function CacheSection() {
  const { clear_office_cache, clear_onenote_cache, clear_teams_cache, set } = useAppStore();

  return (
    <section className="card">
      <h2 className="card-title">Clear Cache</h2>
      <label className="checkbox-row">
        <input
          type="checkbox"
          checked={clear_office_cache}
          onChange={(e) => set("clear_office_cache", e.target.checked)}
        />
        <span>Office Cache</span>
      </label>
      <label className="checkbox-row">
        <input
          type="checkbox"
          checked={clear_onenote_cache}
          onChange={(e) => set("clear_onenote_cache", e.target.checked)}
        />
        <span>OneNote Cache</span>
      </label>
      <label className="checkbox-row">
        <input
          type="checkbox"
          checked={clear_teams_cache}
          onChange={(e) => set("clear_teams_cache", e.target.checked)}
        />
        <span>Teams Cache (Classic + New)</span>
      </label>
    </section>
  );
}
