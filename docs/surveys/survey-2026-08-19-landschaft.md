<!--
  title: Landschafts-Vermessung 2026-08-19
  class: survey
  date: 2026-08-19
  sha256: 7f1b32358c8402a7164aca48c63fd0bffdf7cd5e802c158242c1f8d109eda894
-->
# Landschafts-Vermessung 2026-08-19

Das Repo wurde gegen den Code und gegen Git vermessen: Leichen, Redundanzen,
Drift, unklare Rollen. Befund + vollzogene Reparaturen + offene Urteile.
Stand: 1485 Commits, 341 getrackte Dateien, Branch main.

## Landkarte

- `src/` — 17 lib-Module (mat, force, inflate, fk, fits, lsk, netcdf, pck,
  cdn, kepler, dastcom, fit, json, bpc, sexagesimal + lib.rs + main.rs) und
  23 Bins. Alle 16 Funktions-Module sind verdrahtet (geprüft per
  `omegaflow::<modul>::`-Sweep) — kein totes lib-Modul.
- `static/` — index.html (Relay-Sensor), constants.js (Protokoll-Parser),
  landing.html (Pages-Landing, omegaflow.space, CNAME + pages.yml).
- `docs/` — concepts/ (32, mit Verdict-Index in TODO), reference/ (19),
  surveys/, plans/ (6), SOURCE_PORT.md (der eine Pfad), DENKRAUM.md
  (bindende Lektüre des Rats), source_curation.md (abgelöst, Verweis).
- `phi/` — sources.φ, dead_sources.φ, blocked_sources.φ,
  interesting_domains.φ, sources_index.φ; pipeline/ (ledger.φ, index.φ,
  prompt.φ, frame_registry.φ, library.φ + katalog/, queue/, stage/,
  research/).
- `.github/workflows/` — ci.yml, healthcheck.yml (3-h-Cron,
  `cargo run -- --verify phi` + Anomalie-Issues), kernel_flatten.yml
  (monatlich, ~35 Kompilate), probe_sweep.yml (wöchentlich), pages.yml
  (Landing + Binaries).
- `scripts/` — bootstrap_ephemerides.sh + ARCHIVED/ (18 Python der
  Vor-CDN-Ära).
- `.opencode/` + `.continue/` — zwei Agent-Harnesse nebeneinander.

## Leichen

- `FOUNDATION.txt` (198 KB, getrackt): null Referenzen im Repo —
  veraltete TODO-Kopie + Counter-Slope-Material. Bestattungs-Kandidat
  (Urteil des Operators steht aus).
- `docs/AUFTRAG-crossmatch.md`: Auftrag ohne Referenzen; `--crossmatch`
  lebt in kernel_flatten.yml (LMXB u. a.). Vermutlich vollzogen —
  prüfen, dann tilgen.
- `docs/plans/` K03_runtime_auftrag, K05_mond_bpc_wiedereinbau,
  K05_mond_bpc_uebergabe, dual_mode_architecture,
  agnostic_membrane_manifestation: Pläne der Vor-CDN-Ära, Themen sind
  gebaut. Nur ref-auth-apis.md ist LIVE (TODO I03 referenziert §E).
- `docs/reference/EXTRACT_TYPES.md`: SUPERSEDED (gebannert) — nur
  Navigationswert.
- `scripts/ARCHIVED/` (18 Python): deklariert archiviert; die
  .gitignore-Ausnahmen `!scripts/verify_sources.py` und
  `!scripts/generate_ephemerides.py` zeigten auf Dateien, die nur noch
  in ARCHIVED liegen — tote Zeilen, am 2026-08-19 getilgt.

## Redundanzen

- Drei φ-Quell-Register: dead_sources.φ + blocked_sources.φ +
  interesting_domains.φ — letzteres ist laut Header ein abgeleiteter
  Index aus den beiden anderen (Stand 2026-08-17), seit den
  Grind-Wellen nicht erkennbar gepflegt.
- `pangaea_catalog_full.φ` + `pangaea_catalog.φ` — Voll- und
  Arbeitskopie nebeneinander (nur erstere LFS, .gitattributes).

## Drift — Doku gegen Code (reparariert 2026-08-19)

- `README.md:9` — trug „Protocol v6: 168 bytes, 21 × f64, 0x06";
  der Code ist v7 (176 B, 22 × f64, 0x07). Berichtigt.
- `docs/reference/BINARY_PROTOCOL.md` — trug v6 (Header, 168-B-Record,
  21 Slots, Version-Checks 6, props-Pack ohne color_index). Auf v7
  berichtigt (Record-Slot 21 = color_index, meta-Pack trägt ihn,
  props[j*3+2] = vec4f(j4, r_eq, color_index, 0), Checks 7).
- `docs/reference/README.md:9` — v6-Zeile auf v7 berichtigt.
- `opencode.json` — instructions trugen `docs/ALIGNMENT_PROTOCOL.md`;
  die Datei existiert nicht. Geisterreferenz getilgt.
- `TODO.md` CI-Zeile — nannte `refresh-protected-data.yml (Python
  inline)`; diese Workflow-Datei existiert nicht mehr, die Rolle trägt
  healthcheck.yml. Zeile auf die Wahrheit berichtigt (I02-Rest =
  sources-Repo-Python-Abschaltung).
- `TODO.md` Vorräte — verwies auf docs/source_curation.md (abgelöst);
  Verweis auf docs/SOURCE_PORT.md berichtigt.
- `AGENTS.md` — „CI Archivar runs every 5 minutes"; in diesem Repo
  taktet healthcheck 3-stündlich. Der 5-min-Takt müsste im
  sources-Repo leben — unverifiziert, AGENTS unangetastet.

## Git-Befund

- Zum Messzeitpunkt trugen `src/main.rs`, `docs/SOURCE_PORT.md`,
  `phi/pipeline/ledger.φ`, `phi/pipeline/queue/grind_astro_tap_2026-08-19.φ`
  uncommittete Änderungen einer Parallel-Session — Haupt-Ader in Arbeit,
  nicht anfassen.
- 7 lokale Branches: backup-7d0ac9f, of_backup_localmain,
  parallel-welle-2026-08-16 (+remote), session-2026-08-16 (+remote),
  wgpu-mono-hud (nur lokal), main-browser (+remote). Rollen unklar;
  main-browser ist seit dem browser_relay-Feature vermutlich überflüssig
  — Aufräum-Urteil des Operators steht aus.
- Große getrackte Blobs (~14 MB): phi/pipeline/weights_*.txt
  (7,5 + 2,7 + 1,6 + 1,1 + 0,3 + 0,24 + 0,21 MB, TF-IDF-Gewichte),
  master_urls.txt (1,1 MB), probe_url_candidates.txt (1,6 MB),
  staging_void_ledger.txt (405 KB), 12_intro_to_kernels.pdf (438 KB) —
  abgeleitete Arbeitsdaten und Referenz; LFS trägt nur
  pangaea_catalog_full.φ.
- Ignorierte lokale Pfade: kernels/ (Ephemeriden), reference/ (Rolle
  unklar), .opencode/node_modules, .secrets.local (korrekt ignoriert).

## Offene Urteile (der Operator entscheidet)

1. FOUNDATION.txt — vollzogen 2026-08-19: entfernt (Git trägt die Historie).
2. docs/AUFTRAG-crossmatch.md — vollzogen 2026-08-19: die Crossmatch-Welle
   war längst geschlossen (git), Datei ins Archiv (docs_erledigt_2026-08-19).
3. docs/plans/ (5 alte Pläne) — vollzogen 2026-08-19: K03/K05×2/dual_mode/
   agnostic_membrane ins Archiv; ref-auth-apis.md bleibt live.
4. interesting_domains.φ — vollzogen 2026-08-19: eingefroren, nach
   phi/pipeline/ verschoben (kein Code-Leser).
5. Branches: 7 lokale Branches auf Rollen prüfen, wgpu-mono-hud und
   main-browser vermutlich schließen.
6. Große Blobs: die weights_*.txt sind lebende Probe-Daten (source_scanner
   → --draft-context, main.rs:11146) und bleiben getrackt — kein
   Aufräum-Kandidat; LFS ist nur eine optionale Größen-Frage
   (weights_*, 12_intro_to_kernels.pdf).
7. .continue/ — vollzogen 2026-08-19: nach /home/johannes/projects/archive/
   verschoben (continue_2026-08-19) + gitignored.
