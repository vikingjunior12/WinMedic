<div align="center">
  <img src="winmedic-icon.svg" width="120" alt="WinMedic Logo" />
  <h1>WinMedic</h1>
  <p>A portable Windows desktop tool for anyone dealing with common Windows problems —<br/>whether you're a home user, a student, or someone who just wants things to work again.</p>

  ![Version](https://img.shields.io/badge/version-0.5.0-cba6f7?style=flat-square)
  ![Platform](https://img.shields.io/badge/platform-Windows%2010%2F11-89b4fa?style=flat-square)
  ![License](https://img.shields.io/badge/license-MIT-a6e3a1?style=flat-square)
  ![Built with Tauri](https://img.shields.io/badge/built%20with-Tauri%202.0-94e2d5?style=flat-square)
</div>

---

WinMedic brings together the most common fixes in one place: reinstall or repair Office, clean up caches, fix OneDrive, remove stuck accounts, update software, and more. No technical knowledge required. New features are added as new problem areas come up.

Ships as a single `.exe` — no installer, no runtime dependencies.

---

## What it can fix

| Module | What it does |
|---|---|
| **Office Setup** | Silent install of Microsoft 365 via ODT (language, channel, architecture selectable) |
| **Office Repair** | Quick repair and full online repair of broken Office installations |
| **Office Uninstall** | Fully removes Office |
| **Teams** | Install, uninstall, or clear Teams cache (Classic & New Teams 2.x) |
| **OneDrive** | Install or uninstall OneDrive (supports classic and MSIX installs) |
| **Account Cleanup** | Remove stuck Office/Teams work or school accounts, clear token caches, remove Workplace Join |
| **Cache** | Clear Office, OneNote, and Teams cache |
| **OneNote** | Reset and repair OneNote data |
| **Account** | Create local user accounts with optional admin rights |
| **Winget** | List and apply pending software updates |
| **Password** | Generate cryptographically secure passwords |
| **System Info** | Shows OS, hardware, RAM, disk, and network info |

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
| Icons | Lucide React |
| Theme | Catppuccin Mocha |
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
└── winmedic-icon.svg       # App icon source
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
