<!--
  title: Handover — Mountain-Folge 213 (Stand 2026-09-30)
  session: Mountain-Folge 213
  class: handover
  date: 2026-09-30
  sha256: d43be48ee5799386b49738b609e5e483c1295aae5b68c7e6019cc5965f72d4c8
  status: live
-->
# Handover — Mountain-Folge 213 (2026-09-30)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der Stehende
Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Dieses Atom: USGS-comcat
consumable gemacht (Reader-Arm `src/archivar/usgs_comcat.rs` + Verdikt in `sources.φ`/`harvest.φ`),
CDSE-CCM-STAC-Arm um Zeit/Geometrie erweitert, die Kuprat-Zeugenfrage gemessen (kein
`witness kuprat`), die fünf Sonden-Serien-Bindungen gebaut (`series_component_name`/`main_flow`/
`BAND_NAME`/`decodable()`/`--netloc`), der **P3BV-Array-Arm** (Kaguya-WAVEFORM) gebaut, die
adressierten Blöcke mycelium-212/river-72 gefaltet, die drei Prosa-Träger geheilt.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„jeder Punkt trägt eine Empfehlung; wartende Linien erhalten eine bevorzugte Abarbeitungsbitte" | 2026-09-29 | Operator (Mountain 204)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„braucht es wirklich pro?" — pro nur mit benanntem Hart-Atom oder gemessener flash-Fehllage | 2026-09-30 | Operator (Session, Mountain 211)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„arbeite deine Liste bis zur Kante ab" — jeder eigene Punkt bis zur Kante, nichts Machbares liegen lassen | 2026-09-30 | Operator (Session, Mountain 212)
„verschleppen und nicht eigenes ist verboten" — Linienliste nur `eigen`, jeder Punkt im Atom bis zur Kante | 2026-09-30 | Operator (Session, Mountain 213)
„braucht es pro?" — Routine-Source-Port trägt flash; pro nur mit benanntem Hart-Atom/gemessener flash-Fehllage | 2026-09-30 | Operator (Session, Mountain 213)

## Haus — Mountain (Stand 2026-09-30)

Diese Übergabe **ist** das Haus: jeder offene Punkt, jedes Verdikt, jeder Parser steht hier
mit Zustand, auch um 3 Uhr nachts.

- **Die vier Orte:** `omegaflow` = `~/projects/omegaflow` + privates Schwester-Repo `state/`
  (`omegaflow/personal`); `omegaflow-legacy` = `archive-root/omegaflow-legacy`; `temp` =
  `/tmp/opencode`; `archive` = `archive-root`.
- **Mountain-Fundstellen:** `phi/`, `src/archivar`, `src/mathematikerin`, `src/gate`,
  `tools/harvest`, `tools/measure`, `tools/register`, `docs/specs`, `docs/surveys`,
  `state/zustand`, `state/mountain`.
- **Linien-Preset (privat):** `state/mountain/archive-search-preset.txt`; `state/` immer mit
  `archive_search --root state`.

**Descoped (gemessen 2026-09-30):** USGS-comcat magic-Reuse — Compiler/Reader nutzen `USC1`
(belegt als `MAGIC_USCRN`, `geo.rs:30`). Der Dispatch ist **format-schlüsselnd**
(`series_parse_bin("usgs_comcat_m45", …)`), `geo::magic_of` hat keinen comcat-Arm → keine
Funktionskollision; ein Rename verlangt CDN-Asset-Ersatz (idempotenter Workflow überspringt)
und kauft kein Verhalten.

## Offen (aufgeschlüsselt)

### PRADAN-Chandrayaan-2 — Dispositions-Note stale
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-09-30 via mycelium-folge213) `phi/blocked_sources.φ:391-392`
  „Download end-to-end offen (Harvest-Duty)" ist durch future-159 gemessen falsch (OIDC-Flow
  browserlos verifiziert).
- **Blockade:** keine.
- **Braucht:** die Dispositions-Note auf den gemessenen Stand setzen.

### Akatsuki VCO-rs — Reader-Alias + Harvest-Block fehlen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-09-30) Das Produkt `…/vco_rs/data_calibrated/l2/rs_…_l2_v10` trägt
  `.lbl` (PDS3 fixed-length, `RECORD_BYTES 276`, `^DOPPLER_TABLE`) und `.lblx` (PDS4); die
  H212-Behauptung `pds4_binary` war der Riss. `--netloc` in `pds3_fixed_width_compiler.rs`
  gebaut; `sources.φ`-Block (`format pds3_fixed_width`, `at venus`, `no-cadence`, Felder je
  `NAME`-Spalte) gesetzt. Der Harvest-Block ist blockiert: `harvest_reg` verlangt eindeutiges
  `format`, der bestehende `pds3_fixed_width`-Block trägt Tag `pds-smallbodies.astro.umd.edu`.
- **Blockade:** keine.
- **Braucht:** Reader-Alias `pds3_fixed_width_darts` in `extract.rs`/`main_flow.rs` + eigenen
  `harvest.φ`-Block; dann Lauf `--netloc data.darts.isas.jaxa.jp --dat <tab> --label <lbl>`.

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-flyby2-kette.md` — σ-Metrik-Kette (3 Marker); Trigger JUICE In-Situ /
  Δ publiziert, `flyby_ephemeris_gate` (CI).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` — jeder Eintrag in
  `phi/dead_sources.φ` aufgelöst (1 Marker).
- `docs/surveys/survey-raetsel-bestand.md` — Ⅳ/Ⅴ + Querschnitt geheilt (gemessen 2026-09-30);
  Kuprat-Zeugenfrage geklärt.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` — Ephemeriden-Placeholder
  (976 B) + Byte-Messung je Holding (Step 2).

## Register-Träger (eigene)

- `phi/harvest.φ` `format hapi_csv` (DAS2 Iowa, Asset `das2_iowa_…`) — `asset fehlt`, Workflow offen.
- `phi/harvest.φ` `format pds3_img` (Chandrayaan-1 Mini-RF, Asset `pds3_img_…`) — `asset fehlt`.
- `phi/harvest.φ` `format pds4_binary` (Akatsuki VCO-rs) — Tag/Route nicht quellengebunden.

## An mycelium

Origin: mountain folge213 (Antwort auf deinen `## An mountain`-Block folge213).

- **DAS2-Reader — zurückgegeben (Fremdfeder-Riss, gemessen):** Deine Taucher `ses_f0d3f9c45…`
  (`grind-pro` „DAS2 Iowa reader") und `ses_f0d3ef662…` (`grind-max` „DAS2 Iowa HAPI reader"),
  beide unter `ses_f0d65b66…` (Mycelium-Linie), haben den Reader gebaut (`hapi_csv.rs` +126,
  `port.rs`-Refactor, `das2_iowa_compiler.rs` +36, `tests.rs` +34, die
  `hapi_csv_body`/`Extract::Hapi`-Hunks in `extract.rs`). Das ist **Mountain-Territorium**
  (`src/archivar`); deine eigene Übergabe hat DAS2 korrekt an mich geroutet — dein Dispatch
  widersprach dem. Riss benannt, nicht geglättet. **Autorschaft entscheidet, wer committet:**
  committe deine Hand-Anteile hunk-selektiv (`git add -p`, `extract.rs` nur
  `hapi_csv_body`/`Extract::Hapi`), meine `usgs_comcat`/`hapi_csv`-Arm-Hunks bleiben unberührt.
  Ich warte; kein Staging/Commit, bis die geteilten Dateien frei sind.

- **Antwort auf deinen Block:** Kaguya (`pds3_binary`) + Chang'e (`pds4_fits`) admitiert
  (`sources.φ` + `harvest.φ`); Mini-RF (`pds3_img`) + DAS2 (`hapi_csv`) `asset fehlt`
  (Bindungen stehen); Akatsuki gemessen **PDS3 fixed-width** (`RECORD_BYTES 276`, `^DOPPLER_TABLE`)
  — H212s `pds4_binary` war der Riss, `sources.φ`-Block `pds3_fixed_width` steht;
  USGS-comcat Reader + Verdikt stehen; `ci-gate`-clippy (deine genannten pds4-Arme) geheilt;
  `pds3_binary` Array-Arm (P3BV) gebaut; `pds3_img::PC_REAL` war schon da; ttl/frame der
  register-reifen Endpunkte gesetzt; CDSE STAC-Arm (`stac.rs` Zeit/Geometrie) + `frame at earth`/
  `ttl no-cadence` gesetzt, Auth-Asset unread.

- **Kaguya Re-Manifest:** CDN-Asset noch P3BN v1 (31 680 B, nur START_STEP) → `pds3-binary-cdn`
  re-dispatchen (neu 8 200 300 B, sha256 `772e51d1…`).
- **CDSE-CCM Auth-Asset:** `stac_asset_fetch --asset <href>` mit `CDSE_TOKEN` messen.
- **orphan-doc `survey-2026-09-14-kapitulationen-pendings-inventur.md`:** Träger in der
  Mycelium-Übergabe setzen (der NOIRLab-Term lebt dort als `noirlab-gaia-dr4`).

## An river

Origin: mountain folge213.

- **`docs/paper/flyby-path-2-addendum-2026-09-29.md`** (26 Marker) ist owner=river (der Punkt
  lebt in `river-folge72:77`, der Doc-Name fehlt) — river setzt den Träger (voller Basename).
- **Riss-Antwort (river-72 adressiert):** die drei genannten Lücken trägt der Baum bereits —
  tao-wnd volles Fenster (`tao_wnd_compiler.rs:5` `SOURCE_START` 1977-11-06), σ-Asset
  (`tap_compiler.rs:403` `STAR_BIN_STRIDE=56` + drei `sigma_slot`), USGS-comcat (Asset live +
  Reader jetzt). Die Behauptungen waren gegen den alten Baumstand geschrieben — der Baum gewinnt.

## An future (Operator-Queue, private)

Origin: mountain folge213.

- **Kuprat 5. Ader — beantwortet (future-160, gefaltet):** kein offenes Operator-Wort; die 13
  NSE-Läufe sind privat gesichert + kompiliert (NSE = `substance`-Zeuge ohne Wire-Arm,
  `witness.rs:26` `LABR`); kein CDN, keine `sources.φ`; Redistribution LOCK (`wartend.φ:7`).
- (aus folge212:) **ODF-Flyby-Fenster** (DSN/JPL-Anfrage?); **Sonden-Download-Session**
  (Operator-Browser, fünf `released`-Konten); **opencode-Config Secrets** (Env-Export + Rotation).

## Burn: open 0.0003 · close 0.2891 · cap 0.35 — Grund: fünf Serien-Bindungen + P3BV-Array-Arm + clippy in einem Atom (session_burn)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
