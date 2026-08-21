# docs/ — Benennung & Versionierung

Bindende Regel: `AGENTS.md` § „Docs — Benennung & Versionierung". Kurzform:

- `handover/` — Übergaben + Session-Pläne, `handover-YYYY-MM-DD-<slug>.md`;
  unveränderlich, nach Einarbeitung + Commit → `/home/johannes/projects/archive/handover/`.
- `surveys/` — `survey-YYYY-MM-DD-<slug>.md` (datierter Befund) bzw.
  `survey-<slug>.md` (stehend). Rohtranskripte von Fremdmodell-Konsultationen
  (Arena) wandern nach `/home/johannes/projects/archive/arena/`.
- `plans/` — stehende Referenzlisten, `ref-<slug>.md`.
- `concepts/` — Konzept-Docs, kebab-case Dateiname; der Eigenname in Prosa
  bleibt UPPER_SNAKE (`sources-v2-spec.md` ↔ „SOURCES_V2_SPEC §1").
- `reference/` — Nachschlage im nativen Format (kein Header).

Versionierung ist Git-only (kein `vN`/`_ancestral` im Namen). Jedes
Prosa-Doc (handover/survey/ref/concept) trägt einen Header-Block mit
`title/class/date/version/sha256/status/see-also`; das `sha256` deckt den
Dateikörper **ohne** den Header (`sed '/^<!--/,/^-->/d' <f> | sha256sum`).

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

Eingefroren: `pipeline/interesting_domains.φ` — abgeleiteter Kandidaten-Pool (Stand
2026-08-17) aus blocked_sources.φ + dead_sources.φ; kein Code-Leser.
