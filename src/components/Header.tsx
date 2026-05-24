import { useAppStore } from "../store/useAppStore";

export function Header() {
  const isAdmin = useAppStore((s) => s.isAdmin);

  return (
    <header className="header">
      <div className="header-left">
        <span className="header-logo">⚙</span>
        <h1>WinMedic</h1>
        <span className="header-version">v0.1.0</span>
      </div>
      <div className="header-right">
        <span className={`admin-badge ${isAdmin ? "admin-badge--ok" : "admin-badge--warn"}`}>
          {isAdmin ? "✓ Administrator" : "⚠ No Admin"}
        </span>
      </div>
    </header>
  );
}
