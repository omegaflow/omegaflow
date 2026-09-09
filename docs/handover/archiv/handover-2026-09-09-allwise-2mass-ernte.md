<!--
  title: Übergabe — AllWISE/2MASS-Footprint-Ernte, geschlossen
  class: handover
  date: 2026-09-09
  sha256: 189f6b83d24f0512c602dbff4617224fbad4a7d7401d0138e35961d3fec78e17
  status: archived
  see-also: docs/handover/handover-2026-09-09-survey-footprint-asset.md phi/footprints.φ phi/blocked_sources.φ docs/TODO.md
-->
# Übergabe — AllWISE/2MASS-Footprint-Ernte, geschlossen

Eine Folge-Session erbt den Stand der Survey-Footprint-Arbeit nach der
AllWISE-/2MASS-Ernte-Sitzung. Gemessen, nichts geraten.

## 1. Was gebaut ist (alles committet)

- **2MASS binar** (`tools/harvest/src/bin/2mass_binary_compiler.rs`, Rezept A):
  `fp_scan_dat` (59.731 Scan-Kacheln, 4 Himmels-Ecken + Zentrum, 0 NULL, Join
  zum Coadd-Atlas deckungsgleich) → binäre Maske Nside 256 (order 8), J/H/Ks
  simultan (frac=1.0). Live verifiziert: MAXREC 60000 trägt alle 59.731 Kacheln,
  `--limit 100` rasterte 5031 Records, Route gemessen. Caveat wie PS1: geblankte
  Sub-Regionen werden überzeichnet, Polar-Keile sind Quadrat-Geometrie.
- **AllWISE fraktional** (`wise_coverage_compiler.rs`): cov-3.fits.gz je Kachel
  (4095² float32, RA---SIN) → HEALPix-Re-Grid (flächengewichtet), count →
  Fraction = effektive-Pixel/Nenntiefe. **Nside 1024** (order 10) — Operator-Wort:
  Ganzhimmel × 4 Bänder auf Nside 4096 wäre ~9,6 GB und über der 2-GB-CDN-Grenze;
  die Tiefe bleibt (W1/W2 ~323, W3/W4 ~201/200). `--coadd-lo/--coadd-hi`
  (deterministisch über sortierte coadd_ids). Parallelisiert (Commit 4f5dbd4):
  4 Worker, Work-Stealing je Kachel, je Worker eigene Accumulation, am Ende gemerged.
- **wise_coverage_combiner**: Chunk-Partials sind **additiv** (Nenntiefe =
  surveyweite TAP-MAX(maxcov)-Konstante, identisch je Chunk) → Summe + clamp,
  nicht max wie PS1. 7 Tests.
- **Workflows**: `allwise-cdn.yml` (Cron stündlich, chunk 60, Combiner am Ende),
  `2mass-binary-cdn.yml` (one-shot). Der 2mass-Job heißt `twomass-binary`
  (GitHub-Job-IDs dürfen nicht mit einer Ziffer beginnen).
- **Gate-Bindungen** (`tools/measure/src/bin/footprint_gate_probe.rs`): des-dr2,
  ps1 (ps1_dr2_binary.fp01), allwise (allwise_coverage.fp01), 2mass
  (2mass_binary.fp01); Tabellen II/349/ps1, II/328/allwise, II/365/catwise,
  II/281/2mass6x. Band-Register trägt W3/W4.
- **Register**: `phi/footprints.φ` (Kopf-Band-Register + W3/W4; allwise-/2mass-/
  ps1-Noten), TODO-Eintrag auf „fünf Surveys entschieden" gezogen,
  `phi/blocked_sources.φ` DES-Note auf GELOEST (liegt in 0c43422).

Commits: 14a4f5d (Bau), 8cae272 (Job-Id), 4f5dbd4 (Parallelisierung + chunk 60).

## 2. Stand der Assets (CDN, Release ssd.jpl.nasa.gov)

- `des_dr2_coverage.fp01` — vertrauenswürdig (123,7 MB).
- `ps1_dr2_binary.fp01` — 35,9 MB, liegt.
- `2mass_binary.fp01` — 27 MB (Run 34316774357 success). Gate `--survey 2mass` live.
- `allwise_coverage.fp01` — in der Ernte (Run 34343245265, ~13 Tage, chunk 60);
  Partials `allwise_part_<lo>_<hi>.fp01` landen je Lauf.
- `ps1_dr2_coverage.fp01` (fraktional) — läuft weiter (Autoresume, Operator-Wort).

## 3. Gemessene Befunde

- Regrid ~62 s/Ebene (CPU-gebunden); die 16×16-Subzellen-Quadratur an den
  HEALPix-Grenzen ist der dominante Anteil. Lokaler Download ~600 KB/s (lokal
  der Engpass, im CI nicht).
- chunk 100 wäre ~7 h pro Lauf gewesen → der 180-min-Timeout riss den ersten
  Lauf; chunk 60 + Parallelisierung passt (~4× auf 4 CI-Kernen, ~50 → ~13 Tage).
- 2MASS `fp_coadd_dat`-Ecken liegen im U-Scan-Pixel-Frame (kein Himmelswert) —
  ungenutzt, Fabrication vermieden.

## 4. Was die Folge-Session erbt

- AllWISE-Ernte abwarten (~13 Tage). Wenn `allwise_coverage.fp01` liegt:
  Roundtrip + Größe verifizieren; dann lebt `--survey allwise`.
- Den parallelisierten CI-Lauf beim ersten Partial prüfen (arbeitet er im CI,
  nicht nur lokal?).
- Regrid-Optimierung (Subzellen-Reduktion) als benannter, korrektheitskritischer
  Schritt (Integral-Erhaltung neu validieren) — nicht nebenbei.
- `stash@{0}` („parallel-session-live-work") liegt als Sicherheitsnetz im
  Git-Stash — fremde Arbeit, unangetastet lassen; die Parallel-Session löst es auf.

## 5. Entscheidungen (Operator-Wort)

- AllWISE fraktional Nside 1024 (Tiefe je Band erhalten).
- 2MASS Rezept A (binär statt absent).
- PS1 fraktional läuft weiter.
- Commit 0c43422 (Seismik/Tōhoku der Parallel-Session + DES-Note) bleibt in der
  History — Operator-Wort „lassen wie es ist".
- Ernte-Dispatch AllWISE + 2MASS = Operator-Wort (erteilt).

## 6. Shared-Tree-Schramme

Die Parallel-Session hat ihre History rebased, während diese Sitzung committete;
der eigene Push wurde zweimal per stash + `reset --hard origin/main` + cherry-pick
+ `stash pop` nachgezogen. Die fremde Arbeit blieb erhalten (stash@{0} als Netz);
ein `ned_cone_compiler.rs`-Konflikt war ein Neuerzeugung-Rennen — die neuere
Version der Parallel-Session liegt im Baum.
