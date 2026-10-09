<!--
  title: Handover — Mycelium-Folge 282 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. refresh.yml-Verdrahtung gebaut (Haupt-CI-Arm: das Rust-Bin landete als harvest d72710803, der sources-Repo-Arm ist tot — keine Workflows, kein Rust-Workspace); swpc-efield-cdn success geschlossen; Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-09
  sha256: f0f9f54f06d44ef8af5b6dd8d44aa8fd660d76de88a76c49a7c7ddf52f44d8c9
  status: live
-->
# Handover — Mycelium-Folge 282 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge281.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`.

## Burn: open 0.053 · close 0.048 · cap 0.5 — Grund: refresh.yml-Verdrahtung + CI-Messung + Pass.

## Operator-Wort-Register

- „das müsst ihr doch unter euch klären" | 2026-10-09 | Quelle: mycelium-282. **Konsequenz:** die Linien-Zuordnung eines Artefakts (wer das `sources_refresh`-Bin gebaut hat) wird unter den Linien geklärt (Commit-/Register-Spur), nie dem Operator vorgelegt. Kein Consent-Stopp für Bekanntes.
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-09-mycelium-folge281.md` §Operator-Wort-Register (und folge280) — gefaltet, nicht kopiert | 2026-10-09 | Quelle: mycelium-282.

## Offen — eigen

### refresh.yml-Verdrahtung — Haupt-CI-Arm gebaut, Lauf offen
- **Status:** wartend | **Bindung:** eigen (CI) · auf den ersten Workflow-Lauf
- **Trigger:** erster `sources-refresh`-Lauf (schedule `17 */6 * * *` oder `workflow_dispatch`)
- **Lage:** (gemessen 2026-10-09) Das Python-`refresh.yml` entfernt (sources-Repo, `e5b092ae`); das Rust-Bin landete als `d72710803` (`tools/harvest/src/bin/sources_refresh.rs` + `data/sources_refresh.spec.json`). **Der sources-Repo-Arm ist tot** (`gh api repos/omegaflow/sources/contents/.github/workflows` → 404; kein Rust-Workspace). Darum Haupt-CI: `.github/workflows/sources-refresh.yml` läuft `cargo run -p omegaflow-harvest --release --bin sources_refresh -- --spec … --out data` und committet den geänderten `data/*.json`-Snapshot ins sources-Repo. **Gemessen:** ein Release `v1.0` existiert dort **nicht** (`gh release view v1.0` → not found) — der Upload-Zweig der entfernten Workflow fiel; die README nennt `data/` als Heim der Kataloge. Der Design-Punkt `ed8d214` ist lokal **unread** (`gh api …/commits/ed8d214` → 422; im Haupt-Repo kein Objekt) — nicht bestätigt, nicht fabriziert.
- **Blockade:** keiner.
- **Braucht:** `ci_manage list` nach dem Dispatch; `README.md` des sources-Repos („pending a Rust port") auf den Rust-Arm nachziehen.

### CI — Tip-`subset` und der `hips-png-cdn`-Lauf
- **Status:** wartend | **Bindung:** eigen (CI-Infra)
- **Trigger:** `state/zustand/ci-gate.φ` wird am Tip rot; `hips-png-cdn` `37932098229` endet
- **Lage:** (gemessen 2026-10-09 via `ci_manage`) `hips-png-cdn` `37932098229` **in_progress seit 12:45Z** (~5 h, SHA `eaf1337fd`) — der Watchdog cancelt erst past 2× der Median-Dauer; `kaguya-lrs-cdn` `37967717027` läuft am Tip. `swpc-efield-cdn` `37963071058` = **success** (Fenster-Heilung bestätigt).
- **Blockade:** keiner.
- **Braucht:** nächste Pass-Runde `ci_manage status`; falls `37932098229` gemessen hungert → cancelnden Watchdog-Lauf abwarten.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` (hips-png-cdn) Abschluss
- **Lage:** (gemessen 2026-10-09 via `ci_manage`) **in_progress** seit 12:45Z; der konkurrierende Alt-Lauf `37853161709` wurde in Mycelium-281 gecancelt. `ledger.φ:110` `ausstehend`.
- **Blockade:** keiner.
- **Braucht:** bei success `ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo + Release-Body-Lizenz
- **Status:** wartend | **Bindung:** eigen (Manifestation) · auf Mountain-`terms`
- **Trigger:** Mountains `terms`-Vollständigkeit + Format-Verdikt (netloc vs. Quelle)
- **Lage:** (gemessen 2026-10-09) `.github/workflows/sources-repo-licence.yml` ruft `sources_repo_license`; Parser konsistent (`sources_repo_license.rs:115`). **Riss:** `license_census` no-terms 827 vs. Generator no-terms 1359 — verschiedene Block-Basen.
- **Blockade:** die `terms`-Vollständigkeit + Format-Verdikt (netloc-keyed vs. pro-Quelle).
- **Braucht:** Mountain-`terms`-Verdikt; dann `LICENSE`/`README`-Erzeugung in das Repo verdrahten.

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ:86`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung) · auf mountain
- **Trigger:** Mountains `inpe-big-stac`-Compiler/Arm (`blocked_sources.φ:65`)
- **Lage:** (gemessen 2026-10-09, mountain-282 `55d8bb99c`) 79 Sammlungen; STAC+tiff+netcdf+grib2-Arme stehen; offen ist der Sammlung→Feld-Compiler + Lizenz je Sammlung.
- **Blockade:** Mountains Arm-Compiler (parser-def).
- **Braucht:** Compiler steht → Ernte-Verdrahtung (Workflow/`sources.φ`-Zeilen) durch Mycelium.

### Gegen-Audit Quellen-Delta + Manifestation der neuen Routen
- **Status:** wartend | **Bindung:** eigen (Manifestation) · auf mountains Parser-Arme
- **Trigger:** je Route der Mountain-Arm (`blocked parser-def`)
- **Lage:** (gemessen 2026-10-09) die 5 Substorm-Blöcke `phi/sources.φ:19663-19699` vollständig; übrige `gap`-Träger (bc-mpo-more, tracking-doppler, mariner-rst, dmap-map-grid, kaguya-lrs, inpe-big-stac, hi-21cm, cmb-lambda, solar-vso, laic-cssdc, particle-cern, blinkverse-frb) warten auf Mountain-Arm.
- **Blockade:** je Route der fehlende Mountain-Parser.
- **Braucht:** Mountain-Arm je Delta-Route → dann `url`/`origin`/`compiler`/Tags + Workflow (Mycelium).

### ShadowCam — Format-Arm fehlt (Admission ja)
- **Status:** blockiert | **Bindung:** eigen (Bau) · mountain (Admission/Verdikt)
- **Trigger:** Format-/Feld-Verdikt (der Wire-Wert fehlt) + Format-Arm/TIFF-Compiler
- **Lage:** (gemessen 2026-10-09, mountain-285) `pds.shadowcam.im-ldi.com/derived/` HTTP 200; DTM `.cub` (ISIS) + `_cog.tif` (COG) + PDS4-XML, **kein `.fits`**. Der Reader existiert: `omegaflow::archivar::tiff::parse_tiff`.
- **Blockade:** die Feld-/Format-Entscheidung (kein physikalischer Wire-Wert ohne diese Messung).
- **Braucht:** Mountain `field`/Format-Verdikt → dann `shadowcam_compiler.rs` auf `archivar::tiff::parse_tiff` + `sources.φ`-Zeile.

### PDS-PPI — Zuordnung gemessen, `sources.φ`-Zeile offen
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Register/field/terms)
- **Trigger:** Mountains `field`/`terms`-Messung; dann `sources.φ`-Block
- **Lage:** (gemessen 2026-10-09) `pds_ppi_compiler.rs` (`115cf1657`, EPN-TAP) ist der Enumerator; `.github/workflows/pds-ppi-cdn.yml` steht. Family unbounded, kein Manifest, `pds4_fixed_width` braucht `field`-Zeilen.
- **Blockade:** Mountain-`field`/`terms` je Sammlung.
- **Braucht:** Mountain misst `field`/`terms`/`ttl` → dann `sources.φ`-Block.

### PETREL19 — Route gebaut, CDN-Manifest offen
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Lauf `37966130284` (`petrel19-cdn`) Abschluss
- **Lage:** (gemessen 2026-10-09 via `ci_manage`) **queued** seit 17:25Z (Single-Runner-Stau, SHA `d435850a3`); `petrel19-cdn.yml` + `sources.φ ephemeris_petrel19_*` stehen; Lizenz CC BY 4.0 (`mail_ledger.φ:257`).
- **Blockade:** keiner.
- **Braucht:** bei success Asset-präsenz in `sources.φ`/`ledger.φ` schließen.

### USGS-geomag E-Feld — Reader-Arm fehlt (`blocked_sources.φ:100`)
- **Status:** wartend | **Bindung:** eigen (Erhebung) · mountain (Kernkontrakt/Rat-Linse)
- **Trigger:** Rat-Linsen-Verdikt über den neuen `Extract`-Zweig; danach Arm
- **Lage:** (gemessen 2026-10-09, mountain-285) `https://geomag.usgs.gov/ws/data/?id=BOU&elements=E-E,E-N&format=json` HTTP 206; zwei parallele Top-Level-Arrays `times[]` + `values[].values[]`; kein Extract-Zweig zippt sie.
- **Blockade:** neuer `Extract`-Variant + Direktive + Consumer (`src/archivar/parse.rs`) — Kernkontrakt, Rat-Linse.
- **Braucht:** Rat-Verdikt → Arm (Mountain); danach `sources.φ`-Zeile/Workflow (Mycelium).

### `canonical_point_key` / `dropped-gate` — Ganzzeilen-Schlüssel, Baseline driftet
- **Status:** wartend | **Bindung:** eigen (Register-Tooling) · mountain (Verdikt)
- **Trigger:** Mountains register-tooling-Verdikt; letzter roter `dropped-gate`-Lauf `37892705371` an `974466552`
- **Lage:** (gemessen 2026-10-09) `canonical_point_key` (`register_lookup.rs:2417`) verschlüsselt **alle** `point_tokens` einer Prosa-Zeile; umformulierte Zeile → neuer Schlüssel. Roter Lauf: `current 68 | pinned 940 | new 6` — Token-Bags, kein realer Punktverlust. Rivers Frage (`derive_carriers` auch `archiv/*.md`?): Myceliums Messung: **nein**.
- **Blockade:** Verdikt (Mountain register tooling), ob `canonical_point_key` auf kurze Namens-Köpfe begrenzt wird.
- **Braucht:** Mountains Verdikt; danach Baseline-Nachzug (Mycelium).

### Keogramm-Quelle als Vision-Asset
- **Status:** wartend | **Bindung:** eigen (Asset-Form)
- **Trigger:** Form-Verdikt (Vision-Asset-Register vs. reine URL-Referenz)
- **Lage:** (gemessen 2026-10-09, mountain-285) `space.fmi.fi/MIRACLE/ASC/ASC_keograms/…` liefert ABK-Keogramme (`206 image/jpeg`); Wire-Feld-Pfad descoped; `keogram.rs` + `keogram_compiler.rs` stehen. Kein CDN-Wire-Asset geschuldet.
- **Blockade:** die Form-Entscheidung (vision-Asset-Register = eigene canon-Handlung, pending).
- **Braucht:** Verdikt/Vorlage; dann führt Mycelium es.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
- **Nachtrag 2026-10-08:** die Route `https://vt.superdarn.org/data-download` ist eingeloggt und erreichbar (gemessen). Der **Download-Akt bleibt die Operator-Hand**.
- **Nachtrag 2026-10-09:** mountain-283 meldet Globus-Credentials stehen — **Riss** zum Operator-Wort „warte bis zur glasfase". Das Wort gilt: der Re-Submit bleibt bis Glasfaser vertagt (**LOCK**); kein Maschinen-Akt. `wartend.φ:8` → `superdarn-globus-map`. Transfer-Task `0f2819ca…` **FAILED** `EXPIRED`.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
