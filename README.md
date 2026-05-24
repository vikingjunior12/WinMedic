# WinMedic

A portable Windows desktop tool for anyone dealing with common Windows problems — whether you're a home user, a student, or someone who just wants things to work again.

WinMedic brings together the most common fixes in one place: reinstall or repair Office, clean up caches, fix OneDrive, update software, and more. No technical knowledge required. New features are added as new problem areas come up.

Built with **Tauri 2.0** (Rust backend) + **React / TypeScript** frontend. Ships as a single `.exe` — no installer, no runtime dependencies.

---

## What it can fix

| Module | What it does |
|---|---|
| **Office Setup** | Silent install of Microsoft 365 via ODT |
| **Office Repair** | Online and local repair of broken Office installations |
| **Office Uninstall** | Fully removes Office |
| **Teams** | Clear Teams cache (Classic & New Teams 2.x) |
| **OneDrive** | Install or uninstall OneDrive (supports classic and MSIX installs) |
| **Account** | Create local user accounts |
| **Cache** | Clear Office, OneNote, and Teams cache |
| **OneNote** | Reset and repair OneNote data |
| **Winget** | List and apply pending software updates |
| **System Info** | Shows OS, hardware, and network info |
| **Password** | Generate cryptographically secure passwords |

---

## Requirements

- Windows 10 / 11 (x64)
- Administrator privileges (UAC prompt on launch)
- Internet connection for online repair, installs, and winget updates

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
WinMedic/
├── src/                    # React frontend
│   ├── components/         # UI sections
│   ├── hooks/              # useEvent, useInvoke
│   ├── store/              # Zustand state
│   └── types/              # Tauri event types
├── src-tauri/
│   ├── src/
│   │   ├── commands/       # Tauri commands (office, teams, onedrive, ...)
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

---

## License

MIT
