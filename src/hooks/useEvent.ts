import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";

export function useEvent<T>(
  event: string,
  handler: (payload: T) => void,
  enabled = true
) {
  useEffect(() => {
    if (!enabled) return;
    let unlisten: (() => void) | undefined;

    listen<T>(event, (e) => handler(e.payload)).then((fn) => {
      unlisten = fn;
    });

    return () => {
      unlisten?.();
    };
  }, [event, handler, enabled]);
}
