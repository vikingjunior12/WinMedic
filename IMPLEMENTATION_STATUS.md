# O365Utility — Implementation Status & Offene Punkte

> **Stand:** 2026-05-23 · Phase 2+3 Grundgerüst ✅ · Offene Punkte folgen

---

## ✅ Bereits implementiert (Grundgerüst)

### Rust Backend (`src-tauri/`)
- [x] Projektstruktur (Cargo.toml, build.rs, tauri.conf.json)
- [x] UAC-Manifest (`app.manifest` — `requireAdministrator`)
- [x] `main.rs` / `lib.rs` mit CancelFlag State
- [x] **Commands:** account, cache, office, onenote, password, run_setup, system_info, teams, winget
- [x] **Models:** setup_options, step_result, update_entry
- [x] **Services:** powershell, process, registry, network
- [x] Build läuft (`cargo build`)

### React Frontend (`src/`)
- [x] Vite + React + TypeScript
- [x] **Components:** Header, ProfileSelector, AccountSection, SoftwareSection, RepairSection, CacheSection, WingetSection, StatusPanel, LogViewer, ActionBar, SystemInfo
- [x] **Hooks:** useEvent, useInvoke
- [x] **Store:** Zustand (`useAppStore.ts`)
- [x] **Types:** `events.ts` (SetupProgressPayload etc.)
- [x] `vite.config.ts` (base: "")
- [x] App-Initialisierung (check_is_admin, check_winget_available, get_system_info)

---

## ❌ Noch zu implementieren

### 🔴 Kritisch (vor erstem Release)

1. **PowerShell Injection-Schutz prüfen**
   - `account.rs`: Wird `create_local_user` mit `-ArgumentList` Array aufgerufen?
   - Keine String-Interpolation mit Passwort/Username.

2. **Passwort-Generator prüfen**
   - `password.rs`: Verwendet `OsRng`? Nicht `ThreadRng`?

3. **winget `--accept-source-agreements` prüfen**
   - `winget.rs`: Ist das Flag bei ALLEN Aufrufen (`get_updates`, `upgrade_package`)?

4. **Teams 2.0 Support**
   - `teams.rs` (`clear_teams_cache`): Werden BEIDE Teams-Varianten behandelt?
   - Classic: `%LocalAppData%\Packages\MSTeams_8wekyb3d8bbwe`, Prozess `MSTeams.exe`
   - New Teams: `%LocalAppData%\Microsoft\Teams`, Prozess `ms-teams.exe`

5. **repair_office_online Pre-Check**
   - `office.rs`: Wird VOR dem Entfernen Internet geprüft?

6. **window.emit() mit `.ok()` nicht `?`**
   - `run_setup.rs`: Alle emit()-Aufrufe mit `let _ = ...` oder `.ok()`?

### 🟡 Wichtig

7. **ospp.vbs 4-Pfad-Suche**
   - `office.rs` (`reset_office_license`): Werden alle 4 Pfade durchsucht?

8. **Office-ODT Extraktion**
   - `office.rs` (`install_office`): Wird `setup.exe` via `include_bytes!` eingebettet und nach `%TEMP%` extrahiert?
   - Wird die XML im selben Ordner abgelegt?

9. **Config-Pfad**
   - Wird `tauri-plugin-store` auf `{exe_dir}/config.json` konfiguriert (nicht %APPDATA%)?
   - Falls exe_dir read-only → Fallback %APPDATA%

10. **Fehlende Commands prüfen**
    - `check_winget_available` — existiert der Command?
    - `save_profile` / `delete_profile` — existieren?
    - `export_log` — existiert? Nutzt `tauri-plugin-dialog` + `tauri-plugin-fs`?

11. **Download Retry**
    - `teams.rs`, `onenote.rs`: Gibt's 2 Versuche bei Download-Fehlern?

12. **PowerShell: absoluter Pfad + UTF8**
    - `powershell.rs`: `C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe`
    - Wird `[Console]::OutputEncoding = [System.Text.Encoding]::UTF8` gesetzt?

### 🟢 Nice-to-have

13. **winget Version-Check** (>=1.2 für JSON)
14. **package_id Whitelist** (Regex vor winget-Aufruf)
15. **Dry-Run-Modus** (in `execute_step` implementiert?)
16. **Teams 2.0 Cache-Clear testen** (kein Code-Thema)

---

## Build & Test

```bash
# Auf Windows-VM:
cd C:\Users\admin\Nextcloud\rust\O365Utility
npx tauri build     # Finale .exe bauen (~8-12 MB)
```

---

## Nächste Schritte für Claude Code

1. **Diesen Status lesen** und die ❌-Punkte der Reihe nach abarbeiten
2. **Bei jedem Fix:** Commit mit aussagekräftiger Message
3. **Nach allen Fixes:** `cargo build` (muss grün sein)
4. **Test:** Auf Windows-VM ausführen, alle Funktionen durchgehen
