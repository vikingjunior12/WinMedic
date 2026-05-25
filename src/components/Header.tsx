import { useAppStore } from "../store/useAppStore";

function WinMedicLogo() {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="28"
      height="28"
      viewBox="0 0 28 28"
      fill="none"
      aria-hidden="true"
    >
      <defs>
        <linearGradient id="shield-grad" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" stopColor="#cba6f7" />
          <stop offset="100%" stopColor="#89b4fa" />
        </linearGradient>
        <linearGradient id="pulse-grad" x1="0%" y1="0%" x2="100%" y2="0%">
          <stop offset="0%" stopColor="#94e2d5" />
          <stop offset="100%" stopColor="#a6e3a1" />
        </linearGradient>
        <filter id="pulse-glow" x="-20%" y="-40%" width="140%" height="180%">
          <feGaussianBlur stdDeviation="0.8" result="blur" />
          <feMerge>
            <feMergeNode in="blur" />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>
      </defs>

      {/* Shield body */}
      <path
        d="M14 2.5L23.5 6.2V14C23.5 19.8 14 25.5 14 25.5C14 25.5 4.5 19.8 4.5 14V6.2L14 2.5Z"
        fill="#11111b"
        stroke="url(#shield-grad)"
        strokeWidth="1.5"
        strokeLinejoin="round"
      />

      {/* ECG / heartbeat pulse line */}
      <polyline
        points="6,14 8.5,14 10,11 12,17.5 14,10.5 16,15.5 17.5,14 22,14"
        stroke="url(#pulse-grad)"
        strokeWidth="1.6"
        strokeLinecap="round"
        strokeLinejoin="round"
        fill="none"
        filter="url(#pulse-glow)"
      />
    </svg>
  );
}

export function Header() {
  const isAdmin = useAppStore((s) => s.isAdmin);

  return (
    <header className="header">
      <div className="header-left">
        <WinMedicLogo />
        <h1>WinMedic</h1>
        <span className="header-version">v0.5.0</span>
      </div>
      <div className="header-right">
        <span className={`admin-badge ${isAdmin ? "admin-badge--ok" : "admin-badge--warn"}`}>
          {isAdmin ? "✓ Administrator" : "⚠ No Admin"}
        </span>
      </div>
    </header>
  );
}
