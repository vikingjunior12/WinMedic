import { Monitor, Globe, MemoryStick, HardDrive, Cpu } from "lucide-react";
import { useAppStore } from "../store/useAppStore";

const ICON_SIZE = 13;
const ICON_STYLE = { flexShrink: 0, opacity: 0.7 };

export function SystemInfo() {
  const info = useAppStore((s) => s.systemInfo);

  if (!info) return <footer className="sysinfo">Loading system information…</footer>;

  return (
    <footer className="sysinfo">
      <span className="sysinfo-item">
        <Monitor size={ICON_SIZE} style={ICON_STYLE} />
        {info.hostname}
      </span>
      <span className="sysinfo-sep" />
      <span className="sysinfo-item">
        <Cpu size={ICON_SIZE} style={ICON_STYLE} />
        {info.os_name} {info.os_version}
      </span>
      <span className="sysinfo-sep" />
      <span className="sysinfo-item">
        <MemoryStick size={ICON_SIZE} style={ICON_STYLE} />
        {info.used_memory_gb.toFixed(1)} / {info.total_memory_gb.toFixed(1)} GB
      </span>
      <span className="sysinfo-sep" />
      <span className="sysinfo-item">
        <HardDrive size={ICON_SIZE} style={ICON_STYLE} />
        {info.disk_free_gb.toFixed(1)} GB free
      </span>
      <span className="sysinfo-sep" />
      <span className="sysinfo-item">
        <Globe size={ICON_SIZE} style={ICON_STYLE} />
        {info.ip_address}
      </span>
    </footer>
  );
}
