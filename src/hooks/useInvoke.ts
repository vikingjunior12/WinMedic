import { invoke } from "@tauri-apps/api/core";
import { useState, useCallback } from "react";

export function useInvoke<T>(cmd: string) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const call = useCallback(
    async (args?: Record<string, unknown>): Promise<T | null> => {
      setLoading(true);
      setError(null);
      try {
        const result = await invoke<T>(cmd, args);
        return result;
      } catch (e) {
        setError(String(e));
        return null;
      } finally {
        setLoading(false);
      }
    },
    [cmd]
  );

  return { call, loading, error };
}
