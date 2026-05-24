# O365Utility — Architektur-Plan v2.0 (Reviewed)

> **Status:** ✅ Reviewed · Bereit für Implementation
>
> **Ziel:** Leichtgewichtiges Windows-Tool (Single Binary, USB-Stick-fähig) zur Einrichtung und
> Wartung von Office-365-Schulgeräten. Ersetzt das bisherige CareumSchulstartTool (C#/WPF, 140+ MB).
>
> **Stack:** Rust (Backend) + React/TypeScript (Frontend) + Tauri 2.0 (Shell)
> **Zielgröße:** ~5–8 MB Single .exe (kein eingebettetes setup.exe mehr)
> **Plattform:** Windows 10/11 only

---

## 1. Warum dieser Stack

| Kriterium | Rust + Tauri | C#/WPF (bisher) |
|---|---|---|
| Binary-Größe | ~5–8 MB | ~140 MB (self-contained) |
| Single File EXE | ✅ Tauri v2 nativ | ⚠️ NativeAOT experimentell |
| GUI-Technologie | HTML/CSS (modern, flexibel) | XAML (starr) |
| Build-Komplexität | `npx tauri build` (ein Befehl) | `dotnet publish` mit Runtime-Optionen |
| WebView2-Abhängigkeit | Auf Win10/11 vorinstalliert | — |
| USB-Stick-Fähig | ✅ Keine Installation, keine Runtime | ❌ .NET Runtime nötig |

**Warum nicht Go + Wails?** Tauri ist bereits bekannt (voice-app, hermes-remote), kein Lernaufwand.
**Warum nicht egui?** Sieht nie "professionell" genug aus für ein Support-Tool.

---

## 2. Funktionsumfang

Alle Funktionen des CareumSchulstartTool v0.8.0 werden übernommen:

### Kernfunktionen
- **Benutzerkonto anlegen** — New-LocalUser, Add-LocalGroupMember
- **Office installieren** — setup.exe mit XML-Konfiguration (braucht Internet, ODT lädt von CDN)
- **Teams installieren** — Download + Silent-Install (Classic + New Teams)
- **OneNote Backup Exporter** — Download + Silent-Install

### Office-Tools
- **Quick Repair** — OfficeC2RClient.exe /repair
- **Online Repair** — Remove + Neuinstallation (mit Netzwerk-Pre-Check!)
- **Lizenz-Reset** — ospp.vbs /dstatus + /unpkey (4-Pfad-Suche, kein stiller Fehler)

### Wartung
- **Office-Cache leeren** — 4 Cache-Ordner, Prozesse killen
- **OneNote-Cache leeren** — Prozesse killen, Cache rekursiv löschen
- **Teams-Cache leeren** — Classic + New Teams Pfade, Prozesse killen, Cache löschen, Teams neu starten

### Updates
- **Winget-Integration** — Verfügbare Updates anzeigen (`--output json`), einzeln aktualisieren

### Hilfsfunktionen
- **Passwort-Generator** — kryptographisch sicher (OsRng)
- **System-Info** — Gerätename, Windows-Version, RAM, Disk, IP (ohne PowerShell)
- **Profile** — 4 Built-in, über UI erweiterbar
- **Dry-Run-Modus** — Simuliert alle Schritte ohne System-Änderungen
- **Cancel** — Abbruch während des laufenden Setups
- **Log-Export** — Protokoll als .txt exportieren (Save-Dialog)

---

## 3. Projekt-Struktur

```
o365utility/
├── src-tauri/                    # Rust Backend
│   ├── Cargo.toml
│   ├── build.rs                  # + Windows Manifest (requireAdministrator)
│   ├── tauri.conf.json
│   ├── capabilities/
│   │   └── default.json          # core:default, store:default, fs:default, dialog:default
│   ├── icons/
│   │   └── icon.png
│   ├── src/
│   │   ├── app.manifest          # requireAdministrator
│   │   ├── main.rs
│   │   ├── lib.rs                # Tauri builder + CancelFlag state
│   │   ├── commands/
│   │   │   ├── mod.rs
│   │   │   ├── account.rs        # create_local_user, check_user_exists
│   │   │   ├── office.rs         # install_office, repair_office, reset_license
│   │   │   ├── teams.rs          # install_teams, clear_teams_cache
│   │   │   ├── onenote.rs        # install_backup_exporter, clear_onenote_cache
│   │   │   ├── cache.rs          # clear_office_cache
│   │   │   ├── winget.rs         # check_available, get_updates, upgrade_package
│   │   │   ├── password.rs       # generate_password
│   │   │   ├── system_info.rs    # get_system_info, check_is_admin
│   │   │   └── run_setup.rs      # Orchestrator + cancel_setup
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   ├── powershell.rs     # PowerShell via -ArgumentList Array + absoluter Pfad
│   │   │   ├── process.rs        # Prozess-Management (Timeouts, Kill, Track)
│   │   │   ├── registry.rs       # Windows Registry Zugriff
│   │   │   └── network.rs        # IP/Hostname-Utilities
│   │   └── models/
│   │       ├── mod.rs
│   │       ├── setup_options.rs  # Ausführungsoptionen (serde, private Felder für Passwort!)
│   │       ├── step_result.rs    # Ergebnis pro Schritt
│   │       └── update_entry.rs   # Winget-Update-Eintrag (-> DisplayText für Frontend)
│
├── src/                          # React Frontend
│   ├── index.html
│   ├── main.tsx
│   ├── App.tsx
│   ├── App.css
│   ├── types/
│   │   └── events.ts             # Typisierte Event-Payloads
│   ├── components/
│   │   ├── Header.tsx
│   │   ├── ProfileSelector.tsx
│   │   ├── AccountSection.tsx    # + PasswordGenerator inline
│   │   ├── SoftwareSection.tsx
│   │   ├── OfficeComponents.tsx
│   │   ├── RepairSection.tsx
│   │   ├── CacheSection.tsx
│   │   ├── WingetSection.tsx     # Nur sichtbar wenn wingetAvailable
│   │   ├── StatusPanel.tsx
│   │   ├── LogViewer.tsx
│   │   ├── ActionBar.tsx         # GO + Abbrechen + Export
│   │   └── SystemInfo.tsx
│   ├── hooks/
│   │   ├── useInvoke.ts
│   │   └── useEvent.ts
│   └── store/
│       └── useAppStore.ts        # Zustand
│
├── package.json
├── vite.config.ts                # base: ""
├── tsconfig.json
└── README.md
```

---

## 4. Backend-Design (Rust)

### 4.1 Architektur-Prinzipien

1. **Kein globaler State-Mutex.** Jedes Command bekommt, was es braucht, über Parameter.
2. **PowerShell/CMD-Aufrufe sind gekapselt.**
3. **Jeder Command returned `Result<T, String>`.**
4. **Progress-Events via Tauri-Emitter** an das Frontend (kein Polling).
5. **Serde für Config/Options** — Structured, type-safe.
6. **PowerShell per `-ArgumentList` Array** — nie String-Interpolation mit User-Input!
7. **PowerShell via absoluten Pfad** — `C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe`
8. **Passwörter sind `private` Felder in SetupOptions** — nie in Log-Events oder Errors.

### 4.2 Crate-Abhängigkeiten

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-store = "2"
tauri-plugin-fs = "2"
tauri-plugin-dialog = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["process", "sync", "rt-multi-thread", "fs"] }
reqwest = { version = "0.12", features = ["json", "native-tls"] }
rand = "0.9"                      # Nur OsRng verwenden, niemals ThreadRng; in 0.9 TryRngCore-Trait nutzen
winreg = "0.52"
sysinfo = "0.31"
local-ip-address = "0.6"
tempfile = "3"
chrono = "0.4"

[build-dependencies]
tauri-build = { version = "2", features = [] }
```

### 4.3 Commands

#### Account
```
create_local_user(username, password, add_to_admins) -> Result<StepResult, String>
check_user_exists(username) -> Result<bool, String>
```

Implementierung mit PowerShell — **KEINE String-Interpolation!** Passwort und Username
werden als separate Argumente an PowerShell übergeben:

```rust
Command::new("C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe")
    .args(["-NoProfile", "-Command",
           "$pw = ConvertTo-SecureString $args[0] -AsPlainText -Force; New-LocalUser -Name $args[1] -Password $pw -PasswordNeverExpires:$true",
           &password, &username])
```

#### Office
```
install_office(include_access, include_publisher, include_skype) -> Result<StepResult, String>
repair_office_quick() -> Result<StepResult, String>
repair_office_online(include_access, include_publisher, include_skype) -> Result<StepResult, String>
reset_office_license() -> Result<StepResult, String>
```

- `install_office`: ODT `setup.exe` via reqwest von Microsoft CDN laden (`https://aka.ms/ODTonline`) → `%TEMP%\odt_setup.exe` → `/configure <xml>`. Office-Install braucht ohnehin Internet, kein Vorteil durch Einbettung — und setup.exe (~7 MB) würde die Binary aufblähen. Die XML-Konfiguration wird als `include_str!()` eingebettet (< 1 KB).
- `repair_office_quick`: `OfficeC2RClient.exe /repair /level QuickRepair /quiet`
- `repair_office_online`: **Vor dem Entfernen Internet prüfen!** → `BuildRemoveConfig()` ausführen → erneut Internet prüfen → Neuinstallation. **Kein Rollback möglich:** Nach dem Remove-Schritt im Log explizit warnen ("Office wurde entfernt — bei Verbindungsabbruch Setup erneut starten"). Abbruch via Cancel wird NACH dem Remove-Schritt nicht mehr erlaubt.
- `reset_office_license`: ospp.vbs in 4 Pfaden suchen (`ProgramFiles`/`ProgramFilesX86`, mit/ohne `root/Office16`). `cscript`-Output parsen, nicht nur Exit-Code prüfen.

Office-ODT braucht Internet zum Download der eigentlichen Office-Dateien (~3 GB). Kein Offline-Install, aber sauberes Logging bei Netzwerk-Fehlern.

#### Teams
```
install_teams(download_url) -> Result<StepResult, String>
clear_teams_cache() -> Result<StepResult, String>
```

Download via reqwest → `%TEMP%\TeamsSetup.exe` → Silent-Install (`/silent`).
**Teams 2.0 Support:** New Teams hat andere Pfade und Prozessnamen als Classic Teams:

| Variante | Cache-Pfad | Prozess |
|---|---|---|
| Classic Teams | `%AppData%\Microsoft\Teams\` | `Teams.exe` |
| New Teams (2.0) | `%LocalAppData%\Packages\MSTeams_8wekyb3d8bbwe\LocalCache\Microsoft\MSTeams\` | `ms-teams.exe` |

→ Beide Varianten in `clear_teams_cache()` behandeln.

#### OneNote
```
install_onenote_backup_exporter(download_url) -> Result<StepResult, String>
clear_onenote_cache() -> Result<StepResult, String>
```

Download von GitHub-Release-URL → Silent-Install (`/S`).
Cache: Kill onenotem/onenote/msoia → `%LocalAppData%\Microsoft\OneNote\16.0\cache` rekursiv löschen.

#### Cache
```
clear_office_cache() -> Result<StepResult, String>
```

Kill EXCEL/WINWORD/POWERPNT/OUTLOOK → Lösche 4 Cache-Ordner → Wiederherstellen als leere Verzeichnisse.

#### Winget
```
check_winget_available() -> Result<bool, String>
get_winget_updates() -> Result<Vec<UpdateEntry>, String>
upgrade_winget_package(package_id) -> Result<StepResult, String>
```

- `check_winget_available`: `winget --version` aufrufen, Exit-Code prüfen, Version >= 1.2 für JSON
- `get_winget_updates`: `winget upgrade --output json --accept-source-agreements`
- `upgrade_winget_package`: `winget upgrade --id <id> --silent --accept-source-agreements --accept-package-agreements`

**`--accept-source-agreements` ist Pflicht** — ohne Flag blockiert winget interaktiv auf `[Y]`-Eingabe.

Winget-Output wird via `serde_json` direkt geparst, kein Regex.

#### Password
```
generate_password(length) -> Result<String, String>
```

**`rand::rngs::OsRng` verwenden.** `ThreadRng` ist nicht kryptographisch sicher.
In rand 0.9 implementiert `OsRng` den `TryRngCore`-Trait — `try_fill_bytes()` statt `fill_bytes()` nutzen.
Alphabet: 64 Zeichen, keine verwirrenden Zeichen (0/O, 1/l/I).

#### System Info
```
get_system_info() -> Result<SystemInfo, String>
check_is_admin() -> Result<bool, String>
```

`system_info`: `sysinfo` crate (RAM, Disk), `winreg` (Windows Version), `local-ip-address` (IP).
`check_is_admin`: WinAPI `CheckTokenMembership` oder `IsUserAnAdmin()`. Mit `requireAdministrator`-Manifest startet die App nie ohne Elevation — der Check ist immer `true`, aber dient als UI-Guard für den Fall, dass das Manifest beim Build fehlt.

#### Orchestrator
```
run_setup(options: SetupOptions) -> Result<Vec<StepResult>, String>
cancel_setup() -> ()
validate_setup_options(options: SetupOptions) -> Result<(), String>
```

### 4.4 UAC-Manifest

`requireAdministrator` im Windows-App-Manifest (`src/app.manifest`). Windows zeigt UAC-Dialog
vor dem Start der EXE — kein nachträglicher Banner nötig.

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>
      </requestedPrivileges>
    </security>
  </trustInfo>
</assembly>
```

In `build.rs`: `println!("cargo:rustc-link-arg=/MANIFEST:EMBED");`

### 4.5 Run-Setup-Orchestrator

```rust
#[tauri::command]
async fn run_setup(
    window: tauri::Window,
    options: SetupOptions,
    cancel: tauri::State<'_, CancelFlag>,
) -> Result<Vec<StepResult>, String> {
    // Validierung VOR dem ersten Schritt
    validate_setup_options(&options)?;

    let steps = build_step_list(&options);
    let total = steps.len();
    if total == 0 { return Ok(vec![]); }  // Divide-by-zero Guard

    cancel.0.store(false, Ordering::SeqCst);
    let mut results = Vec::new();

    for (idx, step) in steps.into_iter().enumerate() {
        if cancel.0.load(Ordering::SeqCst) {
            let _ = window.emit("setup-cancelled", ());
            break;
        }

        let progress = (idx + 1) as f64 / total as f64;
        let _ = window.emit("setup-progress", progress); // .ok(), nicht ?

        let result = execute_step(&step, &options).await;
        let log_safe = result.clone().redact_sensitive();
        let _ = window.emit("setup-log", &log_safe);

        let is_fatal = result.fatal && !result.success;
        results.push(result);
        if is_fatal { break; }
    }

    let _ = window.emit("setup-complete", &results);
    Ok(results)
}
```

**Wichtig:**
- `window.emit()` mit `.ok()` — Window kann während des Runs geschlossen werden
- `CancelFlag` = `AtomicBool` in Tauri-State, `cancel_setup()` setzt es auf `true`
- `validate_setup_options()` prüft: Benutzername-Zeichensatz, max. 20 Zeichen, Passwort != leer, URL nicht leer

### 4.6 Ausführungsreihenfolge

```
1.  Benutzerkonto erstellen     (CreateUser)
2.  Office installieren          (InstallOffice)
3.  Teams installieren           (InstallTeams)
4.  OneNote Backup Exporter      (InstallOneNoteBackupExporter)
5.  Office Quick Repair          (RepairOfficeQuick)
6.  Office Online Repair         (RepairOfficeOnline)
7.  Office Lizenz-Reset          (ResetOfficeLicense)
8.  Office-Cache leeren          (ClearOfficeCache)
9.  OneNote-Cache leeren         (ClearOneNoteCache)
10. Teams-Cache leeren           (ClearTeamsCache)
11. Winget-Upgrades              (ein Schritt pro Paket)
```

---

## 5. Frontend-Design (React/TypeScript)

### 5.1 State Management (Zustand)

```typescript
interface AppState {
  createUser: boolean;
  username: string;
  password: string;
  confirmPassword: string;
  addToAdmins: boolean;
  installOffice: boolean;
  includeAccess: boolean;
  includePublisher: boolean;
  includeSkype: boolean;
  installTeams: boolean;
  teamsDownloadUrl: string;
  installOneNoteExporter: boolean;
  repairOfficeQuick: boolean;
  repairOfficeOnline: boolean;
  resetLicense: boolean;
  clearOfficeCache: boolean;
  clearOneNoteCache: boolean;
  clearTeamsCache: boolean;
  runWinget: boolean;
  dryRun: boolean;

  // Profile
  selectedProfileIndex: number;
  profiles: Profile[];

  // System
  systemInfo: SystemInfo;
  isAdmin: boolean;
  wingetAvailable: boolean;

  // Run state
  isRunning: boolean;
  progress: number;
  steps: StepResult[];
  logs: LogEntry[];
  wingetUpdates: UpdateEntry[];
  validationMessage: string;
}
```

### 5.2 Component Tree

```
<App>
  <Header title="O365Utility" version="1.0.0" />
  <main-layout>
    <left-column>
      <ProfileSelector />
      <AccountSection />
      <SoftwareSection />
      <RepairSection />
      <CacheSection />
      <WingetSection />            ← nur sichtbar wenn wingetAvailable
    </left-column>
    <right-column>
      <StatusPanel />
      <LogViewer />
      <ActionBar />                ← GO + Abbrechen + Export
    </right-column>
  </main-layout>
  <footer>
    <SystemInfo />
  </footer>
</App>
```

### 5.3 Typisierte Events

```typescript
// src/types/events.ts
export type SetupProgressPayload = number;     // 0.0 – 1.0
export type SetupLogPayload = LogEntry;
export type SetupCompletePayload = StepResult[];
export type SetupCancelledPayload = void;
```

### 5.4 Event-Listener (Frontend)

Listener werden BEIM MOUNT registriert, nicht erst beim GO-Klick — sonst gehen erste Events verloren.

```typescript
useEffect(() => {
  const unlisteners = Promise.all([
    listen<SetupProgressPayload>("setup-progress", (evt) => setProgress(evt.payload)),
    listen<SetupLogPayload>("setup-log", (evt) => addLog(evt.payload)),
    listen<SetupCompletePayload>("setup-complete", (evt) => {
      setSteps(evt.payload);
      setIsRunning(false);
    }),
    listen<SetupCancelledPayload>("setup-cancelled", () => setIsRunning(false)),
  ]);
  return () => { unlisteners.then(fns => fns.forEach(f => f())); };
}, []);
```

GO-Button: `invoke('run_setup')` in try/catch — falls Validierungsfehler im Backend,
wird `setup-complete` nicht emittiert, also `setIsRunning(false)` im catch.
**Vor dem Invoke:** `setLogs([])` und `setSteps([])` aufrufen — sonst akkumulieren sich Logs bei Mehrfach-Run.

### 5.5 App-Initialisierung

```typescript
useEffect(() => {
  async function init() {
    const [admin, wingetOk, sysInfo] = await Promise.all([
      invoke<boolean>('check_is_admin'),
      invoke<boolean>('check_winget_available'),
      invoke<SystemInfo>('get_system_info'),
    ]);
    useAppStore.setState({ isAdmin: admin, wingetAvailable: wingetOk, systemInfo: sysInfo });
  }
  init().catch(console.error);
}, []);
```

### 5.6 UI-Konventionen

- **Farbpalette:** Neutrales Grau/Weiß mit Teal oder Indigo als Akzent
- **Schriftart:** Segoe UI
- **Mindestgröße:** 960×620px
- **CSS:** Tailwind oder Custom-CSS, kein Bootstrap/MUI
- **Kein Logo** (austauschbarer Platzhalter)

---

## 6. Was wird besser als im Original

### 6.1 Code-Qualität

| Original (C#) | Neu (Rust) |
|---|---|
| 19 Service-Klassen mit je 1–2 Methoden | ~10 Commands gruppiert nach Domain |
| ObservableObject/RelayCommand Boilerplate | Entfällt (Tauri invoke direkt) |
| ServiceResult-Wrapper | `Result<T, String>` Rust-Standard |
| SharedHttpClient Singleton | `reqwest::Client` |
| try/catch {} Leer-Fänger | Explizites Error-Handling |
| Kein Cancel-Mechanismus | AtomicBool + cancel_setup |
| Keine Prozess-Timeouts | Tokio Timeouts auf allen Aufrufen |

### 6.2 ~1000 Zeilen Boilerplate gespart

MainViewModel (600), ObservableObject/RelayCommand (110), XAML/Code-Behind (300),
SetupOptions-Klasse (15), InverseBooleanConverter (18) — alles nicht mehr nötig.

---

## 7. Build & Deployment

### 7.1 Build

```bash
cd ~/projects/o365utility
npx tauri build       # single .exe
```

Output: `src-tauri/target/release/o365utility.exe` (~5–8 MB)

### 7.2 USB-Stick

```
E:\
└── o365utility.exe           ← einzige Datei
```

Config-Speicherpfad:
1. `{exe_dir}/config.json` (wenn beschreibbar — ideal für USB-Stick)
2. `%APPDATA%\O365Utility\config.json` (Fallback, wenn exe_dir read-only)
3. Kein %TEMP% (ephemer, überlebt Neustart nicht)

**Achtung `tauri-plugin-store`:** Der Store-Plugin nutzt standardmäßig `%APPDATA%`. Den Pfad beim App-Start explizit auf `{exe_dir}/config.json` setzen und nur bei Schreibfehler auf `%APPDATA%` zurückfallen — das passiert nicht automatisch.

### 7.3 Auto-Update

**Nein.** USB-Stick-Tools haben keine Update-URL.

---

## 8. Migrations-Strategie

### Phase 1: Planung ✅
- Architektur finalisiert, alle Funktionen gelistet

### Phase 2: Backend-Kern (MVP)
1. Tauri-Projekt aufsetzen (manuell, UAC-Manifest)
2. PowerShell-Service mit `-ArgumentList` Array + Timeouts
3. Admin-Check via WinAPI
4. Account-Command (Injection-sicher)
5. Office-Command (embedded setup.exe → tempfile)
6. Teams-Command (Classic + New)
7. Orchestrator (run_setup mit Validation, Cancel, Fortschritt)

### Phase 3: Frontend
1. Vite + React + TypeScript
2. Layout (2-Column + Footer)
3. Alle Sections als Komponenten
4. Zustand State-Management
5. Typisierte Event-Listener
6. App-Initialisierung (Admin/Winget-Availability-Check)

### Phase 4: Restliche Funktionen
1. Cache-Commands (Office, OneNote, Teams beide Varianten)
2. Winget (Version-Check, JSON-Parsing, `--accept-source-agreements`)
3. Office-Repair + License-Reset (ospp.vbs 4-Pfad-Suche)
4. OneNote Backup Exporter
5. System-Info
6. Profile-Management

### Phase 5: Polish
1. CSS-Styling
2. Dry-Run-Modus
3. Log-Export (tauri-plugin-dialog + fs)
4. Tests auf Win11-VM
5. USB-Stick-Test (read-only simulieren)
6. Teams 2.0 Cache-Clear testen

---

## 9. Risiken

| Risiko | Eintritts-W. | Plan |
|---|---|---|
| WebView2 fehlt (Win10 <1809) | Niedrig | Evergreen-Installer-Link anzeigen |
| `winget` fehlt | Mittel | Winget-Section ausblenden |
| `winget` <1.2 (kein JSON) | Mittel | Version prüfen, Section ausblenden |
| Tauri v2 Bugs | Niedrig | voice-app/hermes-remote laufen stabil |
| PowerShell-Encoding (Umlaute) | Mittel | `[Console]::OutputEncoding = UTF8` |
| Teams 2.0 Cache-Pfade | Hoch | Beide Pfade/Prozesse behandeln |
| ospp.vbs nicht gefunden | Mittel | 4-Pfad-Suche, Fehler statt stiller Erfolg |
| Online Repair ohne Netz | Mittel | Pre-Check, kein Entfernen ohne Gewissheit |
| winget blockiert interaktiv | Hoch | `--accept-source-agreements` Pflicht |
| winget JSON-Format instabil | Hoch | Format ist undokumentiert und hat sich bereits geändert — beim Parsen defensiv vorgehen, fehlende Felder tolerieren, Version prüfen |
| PowerShell Injection | Hoch | `-ArgumentList` Array |
| Window geschlossen während Run | Niedrig | `.ok()` auf emit() |
| Divide-by-zero (0 Steps) | Niedrig | Guard |
| Config auf read-only USB | Niedrig | Fallback %APPDATA% |
