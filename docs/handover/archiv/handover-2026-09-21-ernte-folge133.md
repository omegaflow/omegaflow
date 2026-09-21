<!--
  title: Handover — Ernte-Folge 133 (Stand 2026-09-21)
  session: Ernte-Folge 133
  class: handover
  date: 2026-09-21
  sha256: 6cf051bfe1e21a31bb4f9f62c3ccc7c70383d72589d81b2ea155f78943ba0b5e
  status: live
-->
# Handover — Ernte-Folge 133 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden parallel von Agenten
abgearbeitet. Jeder offene Punkt trägt seinen nächsten Schritt in derselben Zeile;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Folge 133)

- **HEAD** `ca3953f6` beim Start (Folge 132 stand auf `0c0b30fb`); Arbeitsbaum
  trägt Fremdarbeit (bau: `docs/specs/mantis-shrimp-bom.md` M,
  `mantis-shrimp-build.md` ??; entscheid folge83 committet) — nicht angetastet.
- **Postfach** — drei neue `An ernte` gefaltet und aus `post.md` gelöscht:
  SuperDARN-Route, DEMETER-ISL-Order 18387, glm-l2-cdn.
- **CI** — Watchdog 13:23Z + `ci_manage list`: `emodnet-hfr-cdn` `35598877845`
  **success**; `glm-l2-cdn` `35599198872`/`35599180155` **failure** (Wurzel unten);
  die Massen-`-cdn`-Welle hat sich selbst geheilt.
- **Zustand** — `external-state.md` fremd fortgeschrieben.

## Offen (aufgeschlüsselt)

### glm-l2-cdn — Parser-Fix, Verifikation ausstehend
- **Status:** wartend | **Bindung:** termin (Lauf)
- **Lage:** Wurzel gemessen: `flash_energy` ist int16 mit `_Unsigned = "true"` als
  **String**-Attribut; `attr_unsigned` las nur Integer → signed-Decode →
  `valid_range=(0,-1)` verwarf **jeden** Blitz → `no flashes harvested`. Fix in
  `tools/harvest/src/bin/glm_l2_compiler.rs` (`attr_unsigned` liest jetzt auch die
  String-Form; Test `unsigned_attr_reads_the_string_true_form`),
  `cargo check -p omegaflow-harvest` 0/0.
- **Blockade:** Commit+Push, dann Lauf.
- **Braucht:** nach Push `gh workflow run glm-l2-cdn`; bei success sha256 →
  `sources.φ` glm_l2-Block → `kompiliert`.

### DEMETER ISL — Harvest (CDPP/REGARDS Order 18387)
- **Status:** wartend | **Bindung:** termin (Order) + eigen
- **Lage:** Order läuft (97 078 Dateien / 34,71 GB, gültig bis 2026-09-28);
  Metallink `/tmp/opencode/demeter_isl.metalink` (69 MB, 97 078 `<file>`,
  DMT_N1_1143 Burst 39 318 + DMT_N1_1144 Survey 57 760, **keine Prüfsummen**,
  per-File-Token-URLs); `CDPP_USER`/`CDPP_PASS` in `.secrets.local`; Compiler
  `demeter_compiler.rs` + `src/archivar/demeter.rs` (289-B-Blöcke BE, Marker
  `TOULOUSE`+`ISL SURVEY`, 64-B-f64-Records `[vs_prev, unix, orbit, ne, ni, te, vf, vi0]`).
- **Blockade:** 34,71 GB Download; Riss: Parser verlangt Marker `ISL SURVEY`,
  `DMT_N1_1143` ist **Burst** — Marker ungemessen.
- **Braucht:** `aria2c -M /tmp/opencode/demeter_isl.metalink` →
  `data/cdpp-archive.cnes.fr/`; `demeter_compiler --aggregate <dir> --ci-mode`;
  Burst-Marker prüfen; dann `phi/blocked_sources.φ:71` → `phi/sources.φ` url-Zeile.

### SuperDARN — offene FITACF/Zenodo-Route
- **Status:** wartend | **Bindung:** eigen
- **Lage:** `blocked_sources.φ:21` auf `pending` korrigiert; anonyme Routen
  gemessen (FITACF-POST superdarn.ca → sdc-serv.usask.ca 200; FRDR 31 Datasets;
  Zenodo 822 Records cc-zero/cc-by-4.0 netCDF/FITACF); nur full-res wide-beam
  RAWACF bleibt Globus+PI.
- **Blockade:** Compiler fehlt.
- **Braucht:** Zenodo `10.5281/zenodo.12996103` sniffen; `superdarn_compiler` anlegen.

### Quaoar Sternbedeckung — Manifestation
- **Status:** wartend | **Bindung:** termin (Run)
- **Lage:** `sources.φ:8729` pending sha256; Compiler + Workflow gebaut.
- **Blockade:** Lauf.
- **Braucht:** `ci_manage view <run>`; bei success sha256 → `sources.φ:8729`/`ledger.φ:30`.

### PS1 Order-10-Final
- **Status:** wartend | **Bindung:** termin (Run 35595711594)
- **Lage:** `footprints.φ:19` Asset absent.
- **Blockade:** Lauf.
- **Braucht:** `ci_manage view 35595711594`.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin (Dienst)
- **Lage:** `ledger.φ:14`; `/tap/tables` HTTP 500 (Re-Messung 2026-09-21), Backend down.
- **Blockade:** Pithia-DB (dienstseitig).
- **Braucht:** Re-Messung bei 200.

### SSDC Limadou
- **Status:** wartend | **Bindung:** termin (PI)
- **Lage:** `ledger.φ:26`; Portal 200 (2026-09-21), keine neue Anleitung.
- **Blockade:** PI-Portal.
- **Braucht:** neue Anleitung auf dem Limadou-Portal.

### EPA RadNet Koordinaten
- **Status:** wartend | **Bindung:** termin (FRS)
- **Lage:** `blocked_sources.φ:74`; ERM_LOCATION 200 (nur loc/state/city), FRS 503.
- **Blockade:** FRS-Maintenance.
- **Braucht:** FRS-Retry.

### Candidates-Pools
- **Status:** wartend | **Bindung:** eigen
- **Lage:** Force-Gate-Review gelaufen; Register-Zählung (34 offen) und gemessene
  Kandidaten-Zeilen (~182 gesamt) weichen ab — Riss, nicht geglättet. Survivor
  u.a. IERS EOP (hpiers/maia/cddis), RAPID-AMOC, e-callisto, Coral Reef Watch,
  CNEOS Fireball/CAD.
- **Blockade:** Port-Entscheidung je Survivor.
- **Braucht:** Survivor → `sources.φ` portieren (nächster Atom).

### Wartend (kein Auswahlpunkt)
- Lasair-LSST (502), Sonden-Antworten, BepiColombo, NRS02-10/12/13 SHAPE,
  EMODnet HFRADAR NADR (Re-Messung fällig 2026-10-19).

## Benchmark

- Kein Doppel-Lauf: glm-l2-Wurzel lief `grind-pro` (Parser-Gap), Candidates
  `grind-pro` — keine flash/pro-Paarung auf derselben Aufgabe, kein neuer Sieger.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Commit:** `phi/sources.φ`, `phi/pipeline/ledger.φ`,
  `tools/harvest/src/bin/glm_l2_compiler.rs`, `docs/handover/post.md`, neues
  Handover `handover-2026-09-21-ernte-folge133.md`, Move
  `handover-2026-09-21-ernte-folge132.md` → `archiv/`.
- **Fremd (nicht angetastet):** bau (`docs/specs/mantis-shrimp-bom.md` M,
  `mantis-shrimp-build.md` ??).
- Nie ein nacktes `git commit`; committet wird pfad-begrenzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
