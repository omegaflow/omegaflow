---
description: omegaflow project conventions — stack, formats, model routing, hardware constraints
---

# Projekt-Konventionen: omegaflow

Diese Datei ergänzt `omegaflow-gremium.md` (Identität/Heuristik) um
technische Fakten, an denen sich Code-Vorschläge orientieren sollen.

## Stack

- **Sprache/Runtime:** Rust (Edition 2024), Cargo-Projekt (`Cargo.toml`,
  Paketname `omegaflow`, Lizenz CC-BY-NC-SA-4.0 — kein Standard-OSS-Lizenztext,
  bei Lizenzfragen im Code nicht MIT/Apache annehmen).
- **Frontend:** Vanilla JS (ES-Module, `export const` / `export function`),
  kein Framework, kein Bundler-Setup sichtbar — Vorschläge sollen ohne
  npm-Abhängigkeiten auskommen, sofern nicht explizit anders verlangt.
- **Protokoll:** binäres Custom-Format über WebSocket (`DataView`,
  Little-Endian, `Float64`/`Uint32`/`Uint8`-Felder). Magic Bytes `0xCF 0x86`
  + Versionsbyte tauchen im Deserializer auf — bei Änderungen am
  Binärformat IMMER Encoder (Rust-Seite) und Decoder (JS-Seite) synchron
  halten, sonst bricht die Byte-Offset-Arithmetik.
- **Konstanten:** goldener Schnitt (`Φ = 1.618...`), Lichtgeschwindigkeit,
  WGS84-Ellipsoid-Parameter, J2000-Epoche — das Projekt arbeitet mit
  physikalischen/geodätischen Größen. Bei numerischem Code auf Einheiten
  und Epochenbezug achten (z.B. `j2000()` erwartet Unix-Sekunden).

## Dateiformate & Namenskonventionen

- `*.φ`-Dateien (z.B. `sources.φ`) und `*.is`/`*.dat`/`*.idx`-Dateien sind
  **projektspezifische, nicht-standardisierte Formate**. Kein bekanntes
  Schema annehmen oder raten — bei Bedarf explizit nachfragen statt
  Strukturvorschläge zu erfinden.
- Griechische Bezeichner (`φ`, `ω`, `Φ`, `τ`, `Δt`) sind bewusst gewählte
  Variablennamen im Code (nicht nur Doku-Symbolik) — beim Refactoring
  nicht automatisch in lateinische Namen umbenennen, das entspricht der
  bestehenden Konvention.

## Modell-Routing (Empfehlung für Continue.dev)

Passend zur config.yaml — grobe Faustregel, welches Modell wofür:

| Aufgabe | Empfohlenes Modell |
|---|---|
| Autocomplete (Tab) | Qwen2.5-Coder 1.5B (lokal) |
| Kleine Edits, Boilerplate | Qwen2.5-Coder 3B (lokal) oder GLM-4.7-Flash (free) |
| Rust/Binärprotokoll-Arbeit, komplexe Refactors | GLM-5.x (Coding Plan) |
| Dokumentation, Zusammenfassungen | Gemini 2.5 Flash oder GLM Flash (free) |
| Wenn GLM-Kontingent knapp: Coding-Ersatz | OpenRouter `qwen/qwen3-coder:free` oder `z-ai/glm-4.5-air:free` |

## Hardware-Kontext (Dell XPS 13 9350, 2016, i5, 8GB RAM, kein dedizierter GPU)

- Lokale Modelle laufen komplett auf CPU. Bei Vorschlägen, die neue
  Tooling-Schritte einführen (Linter, Formatter, zusätzliche Build-Stufen),
  auf Leichtgewichtigkeit achten — die Maschine ist der Flaschenhals, nicht
  das Netzwerk.
- Große, rekursive Codebase-Scans (z.B. `@codebase`-Anfragen über das
  gesamte Repo) können auf dieser Hardware spürbar dauern. Wo möglich
  gezielt einzelne Dateien referenzieren statt breite Repo-weite Anfragen.

## Was diese Rules-Datei NICHT tut

- Sie ersetzt keine Sicherheits- oder Ethikprüfung von Anfragen — auch
  Code, der zum "omegaflow"-Framing passt, wird individuell bewertet.
- Sie ist kein Freibrief, unbekannte `.is`/`.dat`-Formate zu erfinden oder
  Verhalten von `dormant` zu spekulieren — bei Unklarheit nachfragen.
