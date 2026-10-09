<!--
  title: Handover — Mycelium-Folge 283 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. Der Rust-Refresh-Arm ist am ersten Lauf verifiziert (sources-refresh 37970858064 success, data-Snapshot 415a14b; sources-README auf den Rust-Arm gezogen). PETREL19-CDN success (37966130284), der pending-Eintrag geschlossen. Mountains dropped-gate-Verdikt gelandet (canonical_point_key = kurzer Namenskopf, Baseline neu gezogen); die zwei tools-build-Fehler der 282 geheilt.
  class: handover
  date: 2026-10-09
  sha256: fe32a8d2eb669a8b42bb976404abf6ee39dd74c79a0b66bf2b812fbf54e130ee
  status: live
-->
# Handover — Mycelium-Folge 283 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge282.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`.

## Burn: open 0.0000 · close 0.0315 · cap 0.5 — Grund: der Rust-Refresh-Arm ist am ersten `sources-refresh`-Lauf verifiziert (`37970858064` success, `data`-Snapshot `415a14b`) und das sources-README auf den Rust-Arm gezogen; PETREL19-CDN success (`37966130284`), der `pending`-Eintrag `blocked_sources.φ` geschlossen; Mountains dropped-gate-Verdikt (287) gefaltet, die zwei tools-build-Fehler geheilt (Lage steigt); `register_lookup --fired/--stale`/`open_points_check` = 0 · deepseek-flash, kein pro/max · Session-Kosten per-session via `session_burn` (Session „Mycelium-Linie starten und Stehenden Pass sch…").

## Operator-Wort-Register

- „das müsst ihr doch unter euch klären" | 2026-10-09 | Quelle: mycelium-282. **Konsequenz:** die Linien-Zuordnung eines Artefakts (wer das `sources_refresh`-Bin gebaut hat) wird unter den Linien geklärt (Commit-/Register-Spur), nie dem Operator vorgelegt. Kein Consent-Stopp für Bekanntes.
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-09-mycelium-folge281.md` §Operator-Wort-Register (und folge280) — gefaltet, nicht kopiert | 2026-10-09 | Quelle: mycelium-283.

## Offen — eigen

### CI — Tip-`subset`, `hips-png-cdn` und tools-build
- **Status:** wartend | **Bindung:** eigen (CI-Infra)
- **Trigger:** `state/zustand/ci-gate.φ` wird am Tip rot; `hips-png-cdn` `37932098229` endet
- **Lage:** (gemessen 2026-10-09T18:35Z via `ci_manage`) `hips-png-cdn` `37932098229` **in_progress seit 12:45Z** (~5,8 h, SHA `eaf1337fd`). Die zwei tools-build-Fehler der 282 sind **geheilt**: `channels: Vec::new()` steht (`tools/utils/src/bin/volume_builder.rs:367`), `tools-build 37972383719` = success an `588735e39`. Am Tip `81b6db7cc`: `tools-build 37972593811` in_progress, `register-dropped 37972593839` success, `register-coverage 37972593906` success, `ci-gate 37972593903` queued.
- **Blockade:** Single-Runner-Stau.
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
- **Lage:** (gemessen 2026-10-09) `.github/workflows/sources-repo-licence.yml` ruft `sources_repo_license`; Parser konsistent (`sources_repo_license.rs:115`). **Riss:** `license_census` no-terms 827 vs. Generator no-terms 1359 — verschiedene Block-Basen. Der Release-Body-Arm steht; `LICENSE` wird nur nach `/tmp/licence` erzeugt, nicht ins Repo geschrieben.
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

### USGS-geomag E-Feld — Reader-Arm fehlt (`blocked_sources.φ:100`)
- **Status:** wartend | **Bindung:** eigen (Erhebung) · mountain (Kernkontrakt/Rat-Linse)
- **Trigger:** Rat-Linsen-Verdikt über den neuen `Extract`-Zweig; danach Arm
- **Lage:** (gemessen 2026-10-09, mountain-285) `https://geomag.usgs.gov/ws/data/?id=BOU&elements=E-E,E-N&format=json` HTTP 206; zwei parallele Top-Level-Arrays `times[]` + `values[].values[]`; kein Extract-Zweig zippt sie.
- **Blockade:** neuer `Extract`-Variant + Direktive + Consumer (`src/archivar/parse.rs`) — Kernkontrakt, Rat-Linse.
- **Braucht:** Rat-Verdikt → Arm (Mountain); danach `sources.φ`-Zeile/Workflow (Mycelium).

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
