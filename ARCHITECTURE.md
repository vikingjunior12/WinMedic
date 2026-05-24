# O365Utility — Architektur-Plan v2.0

> **Status:** ✅ Reviewed · Implementing
> **Letztes Update:** 2026-05-23

## Zusammenfassung

| Eigenschaft | Wert |
|---|---|
| **Name** | O365Utility |
| **Ziel** | Windows-Tool für Office-365-Einrichtung und -Wartung (Single Binary) |
| **Stack** | Rust (Backend) + React/TypeScript (Frontend) + Tauri 2.0 |
| **Größe** | ~8–12 MB (.exe) |
| **Plattform** | Windows 10/11 only |
| **Portabilität** | Single .exe — läuft von überall: USB-Stick, Downloads, Desktop, Netzlaufwerk |
| **Admin** | UAC-Manifest (requireAdministrator) |

## USB-Stick / Portabilität

"USB-Stick-fähig" bedeutet: **Eine Datei, die überall läuft** — kein Installer, kein Setup, kein Runtime-Download. Die .exe kann:

- Vom USB-Stick gestartet werden (Portabilität zwischen Geräten)
- Aus dem Downloads-Ordner gestartet werden (Web-Download)
- Vom Desktop gestartet werden
- Von einem Netzlaufwerk gestartet werden

**Es bedeutet NICHT:**
- ❌ Dass die App NUR vom USB-Stick läuft
- ❌ Dass alle Daten auf dem USB-Stick gespeichert werden müssen
- ❌ Dass die App ohne Internet funktioniert (Office-Download braucht Internet)

Setup.exe und XML sind via `include_bytes!()` in der Binary eingebettet. Keine externen Dateien nötig.

## Eingebettete Assets

Die folgenden Dateien sind aus dem CareumSchulstartTool übernommen und werden in die Binary eingebettet:

| Datei | Pfad in diesem Repo | Größe |
|---|---|---|
| `setup.exe` | `assets/setup.exe` | 6.9 MB |
| `configuration-Office365-x64.xml` | `assets/configuration-Office365-x64.xml` | 424 B |

In Rust: `include_bytes!("../assets/setup.exe")` und `include_bytes!("../assets/configuration-Office365-x64.xml")`.
Zur Laufzeit werden sie via `tempfile` nach `%TEMP%\O365Utility\` extrahiert.

---

*Vollständiger Architektur-Plan unter [`2026-05-23-o365utility-architecture.md`](./2026-05-23-o365utility-architecture.md)*
