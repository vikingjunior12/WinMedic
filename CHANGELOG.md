# Changelog – O365Utility

## v0.1.0 – 2026-05-23/24 (Initial Implementation)

### Stack
- Rust + Tauri 2.0 + React/TypeScript
- Single Binary (~10–12 MB), USB-Stick-fähig, Windows 10/11 only
- Ersetzt CareumSchulstartTool (C#/WPF, ~140 MB)

### Implementiert & funktionierend

#### Backend (Rust)
- **PowerShell-Service** – `-NoProfile -ExecutionPolicy Bypass`, PS-Escaping (`'` → `''`)
- **Process-Service** – `CREATE_NO_WINDOW`, async mit Tokio
- **Network-Service** – Internet-Check vor Downloads (reqwest)
- **Registry-Service** – winreg-Wrapper für Office-Pfadsuche

#### Benutzerkonto
- `create_local_user` – `New-LocalUser` + `Add-LocalGroupMember`
- `check_user_exists` – `Get-LocalUser`
- Optionale Aufnahme in Administratoren-Gruppe

#### Office 365
- `install_office` – Eingebettete `setup.exe` (6.8 MB ODT, `include_bytes!()`), Internet-Check, dynamisches XML
- `uninstall_office` – setup.exe mit Remove-XML, restlose Deinstallation
- `repair_office_quick` – `OfficeC2RClient.exe /repair level=QuickRepair forceappshutdown=true displaylevel=true`
- `repair_office_online` – Remove + Reinstall mit Internet-Check vor und nach Remove
- `reset_office_license` – `/dstatus` → alle Keys auslesen → `/unpkey:XXXXX` je Key

#### XML-Konfiguration (GUI-gesteuert)
- Channel: `Current` (fest)
- Sprache: `de-de` (fest)
- Product: `O365ProPlusRetail` (fest)
- ExcludeApp Groove: immer
- ExcludeApp Access: optional (Standard: ausgeschlossen)
- ExcludeApp Publisher: optional (Standard: ausgeschlossen)
- ExcludeApp Lync/Skype: optional (Standard: ausgeschlossen)
- `Display Level="None"`, `AUTOACTIVATE=1`, `FORCEAPPSHUTDOWN=TRUE`

#### Teams
- `install_teams` – Download von `https://statics.teams.cdn.office.net/production-windows-x86/lkg/MSTeamsSetup.exe` + `/silent`
- `clear_teams_cache` – `%LOCALAPPDATA%\Packages\MSTeams_8wekyb3d8bbwe\LocalCache\Microsoft\MSTeams`

#### OneNote
- `install_onenote_backup_exporter` – via winget
- `clear_onenote_cache` – `%LOCALAPPDATA%\Microsoft\OneNote\16.0\cache`, Attribute-Reset vor Löschung

#### Cache
- `clear_office_cache` – 4 Pfade wie Original (OfficeFileCache, Wef, WebViewHost INetCache, Outlook HubAppFileCache), beendet EXCEL/WINWORD/POWERPNT/OUTLOOK vorher

#### Winget
- `check_winget_available` – prüft ob winget verfügbar
- `get_winget_updates` – JSON-Output, defensiv geparst
- `upgrade_winget_package` – mit `--accept-source-agreements`

#### Sonstiges
- `generate_password` – OsRng, 64-Zeichen-Charset, kein Modulo-Bias
- `get_system_info` – Hostname, OS, RAM, Disk C:, IP
- `check_is_admin` – Registry-Schreibtest
- `run_setup` – Orchestrator mit AtomicBool Cancel, Progress-Events, Log-Events
- `validate_setup_options` – Vor-Start-Validierung

### Frontend (React/TypeScript)
- Zustand-Store (30+ Felder)
- 4 Built-in-Profile (Vollständiges Setup, Nur Reparatur, Cache leeren, Neuer Benutzer)
- Dry-Run-Modus
- Blinkende Fortschrittsanzeige für laufenden Schritt
- Log-Export als .txt
- System-Info-Footer
- Passwort-Generator mit Anzeige/Verbergen
- Office-Komponenten via Sub-Checkboxen steuerbar

### Bekannte Fixes während Entwicklung
- ODT-Wrapper vs. echte setup.exe: `officedeploymenttool_*.exe /extract` → setup.exe
- `ConvertTo-SecureString` Modul-Fehler: `-NonInteractive` → `-ExecutionPolicy Bypass`
- Teams: winget → direkter CDN-Download
- Doppelte Logs: React StrictMode entfernt
- Validierung: `uninstall_office` fehlte in Validation-Check
