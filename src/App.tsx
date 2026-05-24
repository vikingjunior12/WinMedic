import { useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useAppStore } from "./store/useAppStore";
import { useEvent } from "./hooks/useEvent";
import { Header } from "./components/Header";
import { ProfileSelector } from "./components/ProfileSelector";
import { AccountSection } from "./components/AccountSection";
import { SoftwareSection } from "./components/SoftwareSection";
import { RepairSection } from "./components/RepairSection";
import { CacheSection } from "./components/CacheSection";
import { WingetSection } from "./components/WingetSection";
import { StatusPanel } from "./components/StatusPanel";
import { LogViewer } from "./components/LogViewer";
import { ActionBar } from "./components/ActionBar";
import { SystemInfo } from "./components/SystemInfo";
import type { LogEntry, StepResult, SystemInfo as SysInfoType } from "./types/events";
import "./App.css";

export default function App() {
  const store = useAppStore();

  // Initialisierung beim Start
  useEffect(() => {
    async function init() {
      try {
        const isAdmin = await invoke<boolean>("check_is_admin");
        store.set("isAdmin", isAdmin);
        const info = await invoke<SysInfoType>("get_system_info");
        store.set("systemInfo", info);
        const wingetOk = await invoke<boolean>("check_winget_available");
        store.set("wingetAvailable", wingetOk);
      } catch {
        // Fehler ignorieren, UI bleibt funktionsfähig
      }
    }
    init();
  }, []);

  const handleLog = useCallback((entry: LogEntry) => {
    store.addLog(entry);
    if (entry.level === "info" && entry.message.startsWith("Starte:")) {
      store.set("currentStep", entry.message.replace("Starte: ", ""));
    }
    if (entry.level === "success" || entry.level === "error") {
      store.set("currentStep", null);
    }
  }, []);

  const handleProgress = useCallback(
    (progress: number) => store.set("progress", progress),
    []
  );

  const handleComplete = useCallback((_steps: StepResult[]) => {
    store.set("isRunning", false);
    store.set("currentStep", null);
  }, []);

  const handleCancelled = useCallback(() => {
    store.set("isRunning", false);
    store.set("currentStep", null);
    store.addLog({
      level: "warning",
      message: "Setup abgebrochen",
      timestamp: new Date().toLocaleTimeString(),
    });
  }, []);

  useEvent<LogEntry>("setup-log", handleLog);
  useEvent<number>("setup-progress", handleProgress);
  useEvent<StepResult[]>("setup-complete", handleComplete);
  useEvent<void>("setup-cancelled", handleCancelled);

  return (
    <div className="app">
      <Header />
      <main className="main-layout">
        <div className="left-col">
          <ProfileSelector />
          <AccountSection />
          <SoftwareSection />
          <RepairSection />
          <CacheSection />
          {store.wingetAvailable && <WingetSection />}
        </div>
        <div className="right-col">
          <StatusPanel />
          <LogViewer />
          <ActionBar />
        </div>
      </main>
      <SystemInfo />
    </div>
  );
}
