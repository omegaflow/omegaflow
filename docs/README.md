# docs/ — Benennung & Formate

Konvention (2026-08-19): Prosa-Dokumente tragen `.md`, Kleinbuchstaben,
kebab-case, ASCII (keine Umlaute, Leerzeichen, `&`).

- `concepts/` — Konzept-Dokumente; UPPER_SNAKE ist hier der Eigenname der
  Konzepte (verankert im TODO-Index und AGENTS) und bleibt. Historische
  Ausreißer wurden normalisiert: `WGSL_SHADER`,
  `INTUITIVE-TOUCHPAD-TOUCHSTEUERUNG`, `KERNEL-CURATION-CI-AUTOMATION-PLAN`.
- `surveys/` — Session-Messungen; kebab-case + Datum (handover-2026-08-19-*);
  Fremdmodell-Transkripte tragen den Modellnamen, ASCII.
- `reference/` — nachschlagen; Daten bleiben im nativen Format.

Daten-/Maschinen-Dateien (bewusst NICHT `.md`):

| Datei | Rolle |
|---|---|
| `council_voices.yaml` | opencode lädt sie (Council-Stimmen) |
| `omegaflow_sense_hardware.yaml` | ESP32-Sensor-Spec (35 Sensoren/Aktuatoren) |
| `reference/naif_body_ids.tsv` | eincompiliert in ephemeris_compiler (`include_str!`) |
| `reference/ucum-essence.xml` | UCUM-Einheiten-Registry |
| `reference/extractPC.for`, `getascomPC.for` | FORTRAN-Referenz (DASTCOM) |
| `reference/12_intro_to_kernels.pdf` | NAIF-Tutorial (PDF) |

Einstieg: `TODO.md` (Register der offenen Arbeit) + `AGENTS.md` (Regelwerk).
`docs/SOURCE_PORT.md` ist der eine Pfad für Source-Arbeit,
`docs/DENKRAUM.md` die bindende Lektüre des Rats.
