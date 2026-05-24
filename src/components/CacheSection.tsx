import { useAppStore } from "../store/useAppStore";

export function CacheSection() {
  const { clear_office_cache, clear_onenote_cache, clear_teams_cache, set } = useAppStore();

  return (
    <section className="card">
      <h2 className="card-title">Clear Cache</h2>
      <label
        className="checkbox-row"
        title={
          "Clears the following paths:\n" +
          "• %LOCALAPPDATA%\\Microsoft\\Office\\16.0\\OfficeFileCache\n" +
          "• %LOCALAPPDATA%\\Microsoft\\Office\\16.0\\Wef\n" +
          "• %LOCALAPPDATA%\\Packages\\Microsoft.Win32WebViewHost_cw5n1h2txyewy\\AC\\#!123\\INetCache\n" +
          "• %LOCALAPPDATA%\\Microsoft\\Outlook\\HubAppFileCache"
        }
      >
        <input
          type="checkbox"
          checked={clear_office_cache}
          onChange={(e) => set("clear_office_cache", e.target.checked)}
        />
        <span>Office Cache</span>
      </label>
      <label
        className="checkbox-row"
        title={"Clears the following path:\n• %LOCALAPPDATA%\\Microsoft\\OneNote\\16.0\\cache"}
      >
        <input
          type="checkbox"
          checked={clear_onenote_cache}
          onChange={(e) => set("clear_onenote_cache", e.target.checked)}
        />
        <span>OneNote Cache</span>
      </label>
      <label
        className="checkbox-row"
        title={"Clears the following path:\n• %LOCALAPPDATA%\\Packages\\MSTeams_8wekyb3d8bbwe\\LocalCache\\Microsoft\\MSTeams"}
      >
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
