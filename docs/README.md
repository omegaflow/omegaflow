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

## phi/ — Register & Arbeitsdaten

Code-gelesen (Archivar/Compiler lesen sie direkt):

| Datei | Rolle | Fundort |
|---|---|---|
| `sources.φ` | Live-Quell-Register | main.rs:5134 |
| `dead_sources.φ` | abgelehnte/erledigte Quellen (learn-gate negativ) | main.rs:11389 |
| `blocked_sources.φ` | gesperrte Quellen (key-needed u. a.) | main.rs:11401 |
| `sources_index.φ` | Kernel-Index (ephemeris_compiler --index) | ephemeris_compiler.rs |
| `pipeline/library.φ` | gewichtete Tags (--learn-gate) | main.rs:11430 |
| `pipeline/frame_learned.φ` | Route→Frame-Lernen | main.rs:11207 |
| `pipeline/weights_*.txt` | Probe-Kontext (source_scanner-Ausgaben) | main.rs:11146 |
| `pipeline/queue/master.φ` | Master-Source-Arbeitsliste (--port) | main.rs:14336 |
| `pipeline/stage/staging_verified.φ`, `staging_void_ledger.txt` | Staging-Verifikation | main.rs:14707 |
| `pipeline/katalog/*.φ` | Katalog-Korpora (source_scanner, --draft-context) | source_scanner.rs |

Code-geschrieben (transiente Arbeitsausgaben, teils gitignored): probe_survivors.φ,
probe_void.txt, probe_live.txt, probe_url_void.txt, probe_jina.txt, probe_drafts.φ,
probe_drafts_enriched.φ, frame_registry.φ, library_gate_delta.φ.

Arbeitsfläche des Source-Ports (Register/Agent, kein Code-Leser): ledger.φ
(Zustands-Register), index.φ, prompt.φ (Port-Vorlage), queue/grind_*.φ (Drafts),
park/ (geparkt), stage/*_converted.φ (Konvertierungs-Ausgänge).

Eingefroren: `interesting_domains.φ` — abgeleiteter Kandidaten-Pool (Stand
2026-08-17) aus blocked_sources.φ + dead_sources.φ; kein Code-Leser.
