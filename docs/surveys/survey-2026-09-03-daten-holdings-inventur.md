<!--
  title: Daten-Holdings-Inventur (Teil B) — was existiert, wo, was gehört wohin
  class: survey
  date: 2026-09-03
  sha256: 57ceab4ab048271ab194b553d7c795b1773d681a8a8dd0cdd15865018a596bcc
  status: live
  see-also: AGENTS.md (The Cache Ablage), docs/plans/ref-* 
-->

# Daten-Holdings-Inventur (Teil B)

Gemessen am 2026-09-03 auf dem Run-Host `johannes` (`/home/johannes`).
Zweck: vollständige Landschaft der großen Ablagen erfassen, jede nach Herkunft
und Disposition einordnen, bevor irgendetwas bewegt wird. Regel: nie blind
Gigabytes verschieben; Mess-/Sitzungs-/Backup-Daten erst nach Freigabe je
Holding.

## Run-Pfad (autoritativ)

- `OMEGAFLOW_STATE`-Default: `~/.local/state/omegaflow` (= `/home/johannes/.local/state/omegaflow`; systemd-Dienste setzen es explizit auf genau diesen Pfad).
- Cache-Root (`cache_root()`): `~/.local/state/omegaflow/archivar_cache`.
- Mess-/Probe-Datensätze gehören als **flache Dateien** direkt in diesen Cache-Root (siehe AGENTS „The Cache Ablage"). Füllung ist lazy: erst ein Lauf/Compiler oder manuelles Staging legt sie an.
- Dauerhafte Heimat jedes Datensatzes ist das CDN (`omegaflow/sources`, netloc-Releases). Abwesenheit lokal = `pending` (0 honored), nie „verloren".

## Gesamtland (gemessen, vor Aufräum)

| Ablage | Größe | Natur | Disposition |
|---|---|---|---|
| `projects/omegaflow-legacy` | 14 G | alter git-repo-Snapshot; 13 G `target/` | `target/` = regenerierbar → **entfernt 2026-09-03 (13 G)**; Code liegt in Git-Historie |
| `projects/omegaflow` | 4,2 G | Live-Repo; 3,8 G `target/` | `target/` regenerierbar → **entfernt 2026-09-03 (3,8 G)** |
| `knowledge` | 32 G | Mess-/Sitzungs-/Herkunfts-Sicherung | Archivgut |
| `knowledge/archive` | 16 G | davon `archive/data` 15 G (abk, opencode-tmp, jwst-harvest …) | Archivgut — weiter zu inventarisieren |
| `knowledge/sessions` | 5,1 G | LLM-Sitzungs-Logs | Archivgut |
| `knowledge/provenance` | 2,4 G | Herkunfts-/Nachweis-Daten | Archivgut |
| `knowledge/data` | 1,5 G | Ephemeriden (`ephemeris_{body}.bin`), `omni2_serie.bin` | Staging-Kandidaten → Cache-Root |
| `backups` | 23 G | Sicherungskopien | Dedup gegen CDN/Repo |
| `backups/omegaflow` | 16 G | Repo-Schnappschuss (`data/` = gitignored Messdaten: cmb/cosmicflows, pioneer, ephemeris) | Dedup |
| `backups/omegaflow-worktrees` | 6,5 G | Worktree-Schnappschüsse (gitignored cwd-Daten wie `omni2_serie_1h.bin`) | Dedup |
| `backups/state/omegaflow` | 194 M | Kopie des Live-OMEGAFLOW_STATE (gate/mail/reports/phi) | Dedup gegen Live-State |

## Konkrete Probe-Datensätze und ihr jetziger (Backup-)Ort

| Datensatz | aktueller Ort |
|---|---|
| `omni2_serie.bin` | `knowledge/data/omni2_serie.bin`, `backups/omegaflow/data/` |
| `omni2_serie_1h.bin` | `backups/omegaflow-worktrees/bz-messen/omni2_serie_1h.bin` |
| `abk_dbdt_daily.tsv` | `knowledge/archive/data/abk_dbdt_daily.tsv` |
| dark_flow (`cmb_planck_smica_n64.json`, `cosmicflows_cf4.json`) | `backups/omegaflow/data/` |
| Ephemeriden (`ephemeris_{body}.bin`) | `knowledge/data/` (12 aktive) + `backups/omegaflow/data/` |
| Pioneer (`pioneer10_*.bin` u. a.) | `backups/omegaflow/data/` |
| `abk_dbdt_1h_*`, kegel-, GIC-/Corona-Serien | **noch nicht lokalisiert** — pending |

## Aktive Ephemeriden (gehören als `omegaflow_eph_{body}.bin` in den Cache-Root)

earth, juno, jupiter, mars, mercury, neptune, new_horizons, saturn, uranus,
venus, voyager1, voyager2. (`new_horizons`/`voyager1`/`voyager2` sind 976-B-Placeholder, pending.)
Nicht aktive (nur Archiv): cassini, europa_clipper, galileo_e1/e2, juice,
messenger, near, rosetta.

## Erledigt (2026-09-03)

- `omegaflow-legacy/target` + `omegaflow/target` entfernt (~17 G freigegeben, regenerierbar).
- Staging in den Cache-Root (`~/.local/state/omegaflow/archivar_cache/`), als Kopien, Backups bleiben:
  `omni2_serie.bin`, `omni2_serie_1h.bin` (aus bz-messen), `abk_dbdt_daily.tsv`,
  `cmb_planck_smica_n64.json`, `cosmicflows_cf4.json`, und die 12 aktiven
  Ephemeriden als `omegaflow_eph_{body}.bin` (earth/juno/jupiter/mars/mercury/
  neptune/new_horizons/saturn/uranus/venus/voyager1/voyager2).

## Offen (Schritt-für-Schritt, je Freigabe)

1. Dedup `backups/*` + `knowledge/data` + `archive/data` gegen CDN/Repo (mehrfache Kopien derselben Datensätze sind nun im Live-Cache bestätigt).
2. Bewegung nach `projects/archive` nur nach Inventur je Holding.
3. Lokalisierung der noch nicht gefundenen Serien (`abk_dbdt_1h_*`, kegel, GIC/corona).
