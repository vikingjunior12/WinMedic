# O365Utility

A portable Windows desktop tool for IT administrators to install, repair, and maintain Microsoft 365 on Windows 10/11 workstations.

Built with **Tauri 2.0** (Rust backend) + **React / TypeScript** frontend. Ships as a single `.exe` — no installer, no runtime dependencies.

---

## Features

| Module | Description |
|---|---|
| **Office Setup** | Silent install of Microsoft 365 via ODT (Office Deployment Tool) |
| **Office Repair** | Online and local repair of existing Office installations |
| **Teams** | Clear Teams cache (Classic & New Teams 2.x) |
| **Account** | Create local admin accounts for troubleshooting |
| **Cache** | Clear Windows credential cache, Office token cache |
| **OneNote** | Reset and repair OneNote data |
| **Winget** | List and apply pending software updates via winget |
| **System Info** | Display OS, hardware, domain, and Office install status |
| **Password** | Generate and set cryptographically secure passwords |

---

## Requirements

- Windows 10 / 11 (x64)
- Administrator privileges (UAC prompt on launch)
- Internet connection for Office online repair and winget updates
- [Office Deployment Tool](https://www.microsoft.com/en-us/download/details.aspx?id=49117) `setup.exe` placed in `assets/` (not included)

---

## Tech Stack

| Layer | Technology |
|---|---|
| Frontend | React 18, TypeScript, Vite |
| Backend | Rust, Tauri 2.0 |
| State | Zustand |
| Packaging | Tauri bundler (single portable `.exe`) |
| Privileges | UAC manifest (`requireAdministrator`) |

---

## Project Structure

```
O365Utility/
├── src/                    # React frontend
│   ├── components/         # UI sections (Office, Teams, Cache, ...)
│   ├── hooks/              # useEvent, useInvoke
│   ├── store/              # Zustand state
│   └── types/              # Tauri event types
├── src-tauri/
│   ├── src/
│   │   ├── commands/       # Tauri commands (account, office, teams, ...)
│   │   ├── models/         # Shared data types
│   │   └── services/       # PowerShell, process, registry, network
│   └── tauri.conf.json
└── assets/
    └── configuration-Office365-x64.xml   # ODT config (German, x64)
```

---

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) 18+
- [Tauri CLI prerequisites](https://tauri.app/start/prerequisites/) (WebView2 on Windows)

### Setup

```bash
npm install
npm run tauri dev
```

### Build (release)

```bash
npm run tauri build
```

The output `.exe` is in `src-tauri/target/release/`.

> **Note:** Place the Office Deployment Tool `setup.exe` in `assets/` and `src-tauri/resources/` before building — it is bundled into the final executable.

---

## Office Deployment Configuration

The included `assets/configuration-Office365-x64.xml` configures a silent Microsoft 365 Apps for Enterprise install:

- 64-bit, German (`de-de`)
- Current channel, auto-updates enabled
- OneDrive (Groove) excluded
- Silent install, EULA auto-accepted

Modify this file to match your organization's needs before building.

---

## License

MIT
