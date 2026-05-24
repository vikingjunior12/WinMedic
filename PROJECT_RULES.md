# O365Utility — Projekt-Regeln für Claude Code

> **Lies diese Datei VOR jedem Coding-Task.** Sie definiert Rollen, Workflow und Constraints.

---

## Rollen

```
Jo (Product Owner)
  │  Ideen, Anforderungen, Feedback (per Voice/Telegram)
  ▼
Hermes (Planer/Manager)
  │  Schreibt .md-Pläne in Nextcloud2/rust/O365Utility/
  │  Reviewt Code, schlägt Verbesserungen vor
  │  Ändert NIEMALS selbst Code
  │  Führt KEINE git- oder Build-Befehle aus
  ▼
Claude Code (Implementer)
  │  Liest .md-Pläne aus Nextcloud2/rust/O365Utility/
  │  Implementiert auf der Windows-VM
  │  Debuggt, testet, committed
  ▼
Windows-VM (Laufzeitumgebung)
   └── Das Tool wird HIER entwickelt, gebaut und getestet.
       Nur auf Windows 10/11 sinnvoll testbar (PowerShell, WMI, Office).
```

## Workflow

1. **Jo** schickt Idee/Anforderung per Voice an Hermes (Telegram)
2. **Hermes** schreibt/aktualisiert einen Plan als `.md`-Datei im Ordner `Nextcloud2/rust/O365Utility/`
3. **Sync** (Nextcloud) → Datei erscheint auf der Windows-VM
4. **Claude Code** (auf Windows-VM): Jo sagt "Lies `Nextcloud2/rust/O365Utility/<plan>.md` und implementiere"
5. **Claude Code** implementiert → testet auf Windows-VM → committed
6. Bei Fragen/Problemen: Jo → Hermes → neuer/aktualisierter Plan

## Wichtige Regeln

- **Hermes reviewed Code, schreibt aber keinen.**
- **Hermes führt keine Builds aus.** Das passiert auf der Windows-VM.
- **Hermes committed nicht.** Das macht Claude Code auf der VM.
- **Pläne sind .md-Dateien** im `Nextcloud2/rust/O365Utility/`-Ordner.
- **Pläne vor Implementation von Hermes reviewen lassen** (via Telegram).

## Constraints (niemals verletzen)

1. **Single Binary** — eine .exe, sonst nichts. Läuft von USB-Stick, aus Downloads-Ordner, überall.
   USB-Stick-fähig bedeutet: Doppelklick → läuft. Kein Installer. Kein Runtime-Download.
2. **Windows-only** — Win10/11. Kein macOS, kein Linux.
3. **~8-12 MB Zielgröße** — setup.exe (6.9 MB) ist eingebettet, das ist der größte Brocken.
4. **UAC-Manifest** — requireAdministrator, kein nachträglicher Banner.
5. **PowerShell immer über `-ArgumentList` Array** — nie String-Interpolation mit User-Input.
6. **OsRng für Passwörter** — nie ThreadRng.
7. **winget `--accept-source-agreements`** — Pflicht, sonst hängt der Prozess.
8. **Teams 2.0 Support** — Classic und New Teams haben verschiedene Cache-Pfade.
9. **Keine Passwörter in Logs/Errors** — SetupOptions.password ist ein privates Feld.
