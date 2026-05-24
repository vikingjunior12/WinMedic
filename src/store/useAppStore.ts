import { create } from "zustand";
import type { LogEntry, StepResult, SystemInfo, WingetEntry } from "../types/events";

export interface Profile {
  id: string;
  name: string;
  options: Partial<AppState>;
}

const BUILT_IN_PROFILES: Profile[] = [
  {
    id: "full-setup",
    name: "Full Setup",
    options: {
      install_office: true,
      install_teams: true,
      install_onenote: true,
    },
  },
  {
    id: "repair-only",
    name: "Repair Only",
    options: {
      quick_repair: true,
      clear_office_cache: true,
      clear_teams_cache: true,
    },
  },
  {
    id: "cache-clear",
    name: "Clear Cache",
    options: {
      clear_office_cache: true,
      clear_onenote_cache: true,
      clear_teams_cache: true,
    },
  },
  {
    id: "new-user",
    name: "New User",
    options: {
      install_office: true,
      install_teams: true,
    },
  },
];

interface AppState {
  // Benutzerkonto
  username: string;
  password: string;
  confirmPassword: string;
  add_to_admins: boolean;
  // Software
  install_office: boolean;
  install_teams: boolean;
  install_onenote: boolean;
  // Office-Komponenten
  include_access: boolean;
  include_publisher: boolean;
  include_skype_for_business: boolean;
  // Repair / Deinstallation
  uninstall_office: boolean;
  quick_repair: boolean;
  online_repair: boolean;
  license_reset: boolean;
  // Cache
  clear_office_cache: boolean;
  clear_onenote_cache: boolean;
  clear_teams_cache: boolean;
  // Winget
  wingetUpdates: WingetEntry[];
  selectedWingetPackages: string[];
  wingetAvailable: boolean;
  // Dry-Run
  dry_run: boolean;
  // Profile
  profiles: Profile[];
  activeProfileId: string | null;
  // System
  isAdmin: boolean;
  systemInfo: SystemInfo | null;
  // Run-State
  isRunning: boolean;
  progress: number;
  steps: StepResult[];
  logs: LogEntry[];
  currentStep: string | null;
}

interface AppActions {
  set: <K extends keyof AppState>(key: K, value: AppState[K]) => void;
  applyProfile: (id: string) => void;
  addLog: (entry: LogEntry) => void;
  addStep: (step: StepResult) => void;
  resetRunState: () => void;
  toggleWingetPackage: (id: string) => void;
}

const DEFAULT_STATE: AppState = {
  username: "",
  password: "",
  confirmPassword: "",
  add_to_admins: false,
  install_office: false,
  install_teams: false,
  install_onenote: false,
  include_access: false,
  include_publisher: false,
  include_skype_for_business: false,
  uninstall_office: false,
  quick_repair: false,
  online_repair: false,
  license_reset: false,
  clear_office_cache: false,
  clear_onenote_cache: false,
  clear_teams_cache: false,
  wingetUpdates: [],
  selectedWingetPackages: [],
  wingetAvailable: false,
  dry_run: false,
  profiles: BUILT_IN_PROFILES,
  activeProfileId: null,
  isAdmin: false,
  systemInfo: null,
  isRunning: false,
  progress: 0,
  steps: [],
  logs: [],
  currentStep: null,
};

export const useAppStore = create<AppState & AppActions>((set, get) => ({
  ...DEFAULT_STATE,

  set: (key, value) => set({ [key]: value } as Pick<AppState, typeof key>),

  applyProfile: (id) => {
    const profile = get().profiles.find((p) => p.id === id);
    if (!profile) return;
    const reset = {
      install_office: false,
      install_teams: false,
      install_onenote: false,
      quick_repair: false,
      online_repair: false,
      license_reset: false,
      clear_office_cache: false,
      clear_onenote_cache: false,
      clear_teams_cache: false,
    };
    set({ ...reset, ...profile.options, activeProfileId: id });
  },

  addLog: (entry) =>
    set((state) => ({ logs: [...state.logs.slice(-499), entry] })),

  addStep: (step) => set((state) => ({ steps: [...state.steps, step] })),

  resetRunState: () =>
    set({ isRunning: false, progress: 0, steps: [], logs: [], currentStep: null }),

  toggleWingetPackage: (id) =>
    set((state) => {
      const selected = state.selectedWingetPackages;
      return {
        selectedWingetPackages: selected.includes(id)
          ? selected.filter((p) => p !== id)
          : [...selected, id],
      };
    }),
}));
