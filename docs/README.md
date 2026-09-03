# docs/ — Karte

Die Ordnung folgt dem Zweck, nicht einer Klassen-Lehre. Ein Ordner trägt,
was sein Name sagt; Tiefe bleibt bei `docs/<ordner>/<datei>` — eine Ebene
mehr nur für genuine externe Bündel (`reference/pioneer-anomaly/`).

## Ordner

| Ordner | Inhalt | Benennung |
|---|---|---|
| `concepts/` | stehende Ideen, Philosophie, kybernetische Ethik | kebab-case; Eigenname in Prosa bleibt UPPER_SNAKE |
| `specs/` | feste Verträge + Register (binäre Protokolle, Force-System, CI-Pläne, lauf-log, ref-Listen) | kebab-case; Register/Listen `ref-<slug>.md` |
| `surveys/` | datierte Befunde, Snapshot-Surveys | `survey-YYYY-MM-DD-<slug>.md` bzw. `survey-<slug>.md` (stehend) |
| `handover/` | Übergaben + Session-Pläne | `handover-YYYY-MM-DD-<slug>.md`; unveränderlich, nach Einarbeitung + Commit → `/home/johannes/projects/archive/handover/` |
| `paper/` | publizierbare Messungen (Paper + Ein-Blatt-Verdikte) | kebab-case, `class: paper` |
| `auftrag/` | Untersuchungsaufträge eines Gates | `auftrag-<slug>.md` |
| `reference/` | externes Material, nativer Format (kein Header), Originalnamen | Provenienz unangetastet |

`reference/`-Rohtranskripte fremdmodell-Konsultationen (Arena) gehören nach
`/home/johannes/projects/archive/arena/`, nicht hierher.

## Lose Dateien in docs/

- `README.md` — diese Karte.
- `SOURCE_PORT.md` — der eine Pfad für Source-Arbeit (Kuratierung/Grind/Port).
- `council.yaml` — Council-Infrastruktur (Stimmen).
- `LICENSE` — Lizenz.

## Versionierung

Git-only (kein `vN`/`_ancestral` im Namen). Jedes Prosa-Doc trägt einen
Header-Block mit `title/class/date/version/sha256/status/see-also`; das
`sha256` deckt den Dateikörper **ohne** den Header
(`sed '/^<!--/,/^-->/d' <f> | sha256sum`), so dass zwei lokale Kopien in
einem Befehl verglichen werden. Daten-/Maschinen-Dateien im `reference/`
sind bewusst nicht `.md`.

Einstieg: `TODO.md` (Register der offenen Arbeit) + `AGENTS.md` (Regelwerk).

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
| `pipeline/catalog/*.φ` | Katalog-Korpora (source_scanner, --draft-context) | source_scanner.rs |

Code-geschrieben (transiente Arbeitsausgaben, teils gitignored): probe_survivors.φ,
probe_void.txt, probe_live.txt, probe_url_void.txt, probe_jina.txt, probe_drafts.φ,
probe_drafts_enriched.φ, frame_registry.φ, library_gate_delta.φ.

Arbeitsfläche des Source-Ports (Register/Agent, kein Code-Leser): ledger.φ
(Zustands-Register), index.φ, prompt.φ (Port-Vorlage), queue/grind_*.φ (Drafts),
park/ (geparkt), stage/*_converted.φ (Konvertierungs-Ausgänge).

Eingefroren: `pipeline/interesting_domains.φ` — abgeleiteter Kandidaten-Pool (Stand
2026-08-17) aus blocked_sources.φ + dead_sources.φ; kein Code-Leser.

Stehende Ref-Listen zur phi-Register-Landschaft: `specs/ref-phi-register.md`,
`specs/ref-auth-apis.md`.
