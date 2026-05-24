import { useAppStore } from "../store/useAppStore";

export function SystemInfo() {
  const info = useAppStore((s) => s.systemInfo);

  if (!info) return <footer className="sysinfo">Loading system information…</footer>;

  return (
    <footer className="sysinfo">
      <span>🖥 {info.hostname}</span>
      <span>|</span>
      <span>{info.os_name} {info.os_version}</span>
      <span>|</span>
      <span>RAM {info.used_memory_gb.toFixed(1)} / {info.total_memory_gb.toFixed(1)} GB</span>
      <span>|</span>
      <span>Disk C: {info.disk_free_gb.toFixed(1)} GB free</span>
      <span>|</span>
      <span>🌐 {info.ip_address}</span>
    </footer>
  );
}
