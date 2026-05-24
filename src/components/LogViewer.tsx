import { useEffect, useRef } from "react";
import { useAppStore } from "../store/useAppStore";

const LEVEL_CLASS: Record<string, string> = {
  info: "log-info",
  success: "log-success",
  error: "log-error",
  warning: "log-warning",
  debug: "log-debug",
};

export function LogViewer() {
  const logs = useAppStore((s) => s.logs);
  const endRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    endRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [logs]);

  return (
    <div className="log-viewer">
      {logs.length === 0 && <span className="hint">No logs yet</span>}
      {logs.map((entry, i) => (
        <div key={i} className={`log-row ${LEVEL_CLASS[entry.level] ?? ""}`}>
          <span className="log-time">{entry.timestamp}</span>
          <span className="log-msg">{entry.message}</span>
        </div>
      ))}
      <div ref={endRef} />
    </div>
  );
}
