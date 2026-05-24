export interface LogEntry {
  level: "info" | "success" | "error" | "debug" | "warning";
  message: string;
  timestamp: string;
}

export interface StepResult {
  name: string;
  status: "success" | "failed" | "skipped" | "cancelled";
  message: string;
}

export interface SystemInfo {
  hostname: string;
  os_name: string;
  os_version: string;
  total_memory_gb: number;
  used_memory_gb: number;
  disk_total_gb: number;
  disk_free_gb: number;
  ip_address: string;
}

export interface WingetEntry {
  id: string;
  name: string;
  current_version: string;
  available_version: string;
}

export interface SetupOptions {
  username: string;
  password: string;
  add_to_admins: boolean;
  install_office: boolean;
  install_teams: boolean;
  install_onenote: boolean;
  // Office-Komponenten (XML-Steuerung)
  include_access: boolean;
  include_publisher: boolean;
  include_skype_for_business: boolean;
  uninstall_office: boolean;
  quick_repair: boolean;
  online_repair: boolean;
  license_reset: boolean;
  clear_office_cache: boolean;
  clear_onenote_cache: boolean;
  clear_teams_cache: boolean;
  winget_packages: string[];
  dry_run: boolean;
}
