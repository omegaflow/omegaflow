# phi/ — Register & Arbeitsdaten

class: ref
status: live
date: 2026-09-01
see-also: AGENTS.md

Stehende Referenzliste der `phi/`-Dateien: was der Code liest (Compiler/
Archivar) und was er schreibt, plus die Arbeitsfläche des Source-Ports.
Heimat des Source-Port-Protokolls: `docs/SOURCE_PORT.md`.

## Code-gelesen (Archivar/Compiler lesen sie direkt)

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
| `pipeline/catalog/*.φ` | Katalog-Korpora (source_scanner, --draft-context) | source_scanner.rs |

## Code-geschrieben (transiente Arbeitsausgaben, teils gitignored)

probe_survivors.φ, probe_void.txt, probe_live.txt, probe_url_void.txt,
probe_jina.txt, probe_drafts.φ, probe_drafts_enriched.φ, frame_registry.φ,
library_gate_delta.φ.

## Arbeitsfläche des Source-Ports (Register/Agent, kein Code-Leser)

ledger.φ (Zustands-Register), index.φ, prompt.φ (Port-Vorlage),
queue/grind_*.φ (Drafts), park/ (geparkt), stage/*_converted.φ
(Konvertierungs-Ausgänge).

## Eingefroren

`pipeline/interesting_domains.φ` — abgeleiteter Kandidaten-Pool (Stand
2026-08-17) aus blocked_sources.φ + dead_sources.φ; kein Code-Leser.
