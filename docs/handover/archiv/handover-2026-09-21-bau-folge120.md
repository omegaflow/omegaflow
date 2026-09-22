<!--
  title: Handover — Bau-Folge 120 (Stand 2026-09-21)
  session: Bau-Folge 120
  class: handover
  date: 2026-09-21
  sha256: 29711ca2bf13b69fbfd6afef23be7bccf378118d4950a7b71cdbf1def25b890d
  status: live
-->
# Handover — Bau-Folge 120 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet (Operator-Wort 2026-09-21). Jeder offene Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Session-Beginn)

- **HEAD** `839746c2` == `origin/main`. Der Baum trägt fremde uncommittete Arbeit
  (HFRNet: `src/archivar/mod.rs` + `hfrnet_rtv.rs` + `hfrnet_compiler.rs` +
  `hfrnet-cdn.yml`; ferner te/hyperscanning/register) — nicht angefasst.
- **CI** — `radnet-cdn` `35573513812` @`ab8ad28c` in_progress (unverändert seit
  07:33:30Z); `te-gate` `35572569205` pending; `hyperscanning-te` failure
  (Forschung-Linie). Watchdog-Snapshot zitiert, kein Poll.

## Messung dieses Atoms (kein offener Punkt)

- **GLM-L2-LCFA-Compiler gebaut** (post.md-Zeile eingelöst): die token-freie
  S3-Route `noaa-goes16/17/18/19` trägt `GLM-L2-LCFA/YYYY/DDD/HH/` (netCDF-4/HDF5,
  flach, 54 Datasets; gemessen 2026-09-21, kein L1B im offenen Bucket). Neues
  `tools/harvest/src/bin/glm_l2_compiler.rs` (588 Z.): `flash_lat`/`flash_lon`
  float32, `flash_energy`/`flash_time_offset_of_first_event` int16 `_Unsigned`
  mit scale_factor/add_offset (Unsigned-Dekodierung wie `goes_abi.rs`, **nicht**
  der signierte L1B-Pfad), `_FillValue 65535` / valid_range `[0,65530]`, Epoch aus
  dem granule-relativen `units`-String („seconds since <granule start>"),
  Qualitäts-Gate `flash_quality_flag == 0` (degraded gezählt), Wert-Gate
  `is_finite && > 0`, lat/lon-Bounds. 3 Gate-Tests.
  `cargo check -p omegaflow-harvest` und `-p omegaflow --tests` 0 Fehler/0 Warnungen.
- **Core-Plumbing:** `geo.rs` (`MAGIC_GLML2` = GLM2, `COMP_GLML2_FLASH_ENERGY`),
  `extract.rs`, `main_flow.rs`, `tests.rs` (Roundtrip + Register-Feld-Test).
- **Register/CDN:** `phi/sources.φ` `glm_l2`-Block (`on earth`, em, J, origin S3);
  `.github/workflows/glm-l2-cdn.yml` (Tag `noaa-goes18`, Asset `glm_l2.bin`, eine
  Stunde Granules, `--ci-mode`).
- **post.md** — die überholte RadNet-Zeile (folge119) und die GLM-L2-Zeile
  (eingelöst) gefaltet und gelöscht; die PINE64-Zeile bleibt als Post bestehen.
- **Force-Gate:** kein neuer Kraft-Kanal; em-Flash, wie L1B akzeptiert.

## Offen (aufgeschlüsselt)

### 1. radnet.bin CDN-Manifestation
- **Status:** `wartend` | **Bindung:** `eigen`
- **Lage:** Workflow `radnet-cdn.yml` dispatcht als Lauf `35573513812` @`ab8ad28c`;
  das Asset ist noch nicht manifestiert.
- **Blockade:** Run-Abschluss (CI), unverändert in_progress.
- **Braucht:** `ci_manage view 35573513812` / Watchdog-Snapshot; danach
  `archive_search --sniff …/download/data.epa.gov/radnet.bin`.

### 2. glm_l2.bin CDN-Manifestation
- **Status:** `wartend` | **Bindung:** `eigen`
- **Lage:** Compiler + Workflow stehen; dispatcht als Lauf `35576757352` (queued);
  das Asset ist noch nicht manifestiert.
- **Blockade:** Run-Abschluss (CI).
- **Braucht:** `ci_manage view 35576757352` / Watchdog-Snapshot; Asset per
  `archive_search --sniff …/download/noaa-goes18/glm_l2.bin`; danach sha256 in
  `phi/sources.φ`.

### 3. PINE64 / Mantis-Shrimp (post.md:22)
- **Status:** `operator-gebunden` | **Bindung:** `operator`
- **Lage:** Ox64 zugesagt, Presence-Hardware ungebaut; Spec
  `docs/specs/mantis-shrimp-bom.md` + BOM liegen; entscheid-folge73: kein
- **Blockade:** physische Teile / Operator-Wort.
- **Braucht:** Operator-Entscheid (via entscheid-Linie).

## Benchmark

- **Bau-Folge 120**: `grind-flash` ×1 (GLM-L2-Schema-Messung, ~1 min),
  `grind-max` ×1 (netCDF-4-Parser). `cargo check` im `build`-Kontext. Flash-first;
  `grind-max` für den novel Parser.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `tools/harvest/src/bin/glm_l2_compiler.rs` (neu),
  `.github/workflows/glm-l2-cdn.yml` (neu), `src/archivar/geo.rs`,
  `src/archivar/extract.rs`, `src/archivar/main_flow.rs`, `src/archivar/tests.rs`,
  `phi/sources.φ` (glm_l2-Block), `docs/handover/post.md`, neues
  `docs/handover/handover-2026-09-21-bau-folge120.md`, Move
  `handover-2026-09-21-bau-folge119.md` → `archiv/`.
- **Fremd (nicht angefasst):** `src/archivar/mod.rs`,
  `src/archivar/hfrnet_rtv.rs`, `tools/harvest/src/bin/hfrnet_compiler.rs`,
  `.github/workflows/hfrnet-cdn.yml`, `.github/workflows/ci-check.yml`,
  `.github/workflows/hyperscanning-te.yml`, `src/mathematikerin/te.rs`,
  `tools/measure/src/bin/hyperscanning_group_te.rs`,
  `tools/register/src/bin/register_lookup.rs`, `docs/zustand/dropped-baseline.md`.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`).
