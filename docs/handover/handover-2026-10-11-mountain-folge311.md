<!--
  title: Handover — Mountain-Folge 311 (2026-10-11)
  session: Mountain-Linie in einem Pass — Gate-Release-Frische geheilt (river-Commit entblockt), adressierte Blöcke gefaltet, register_sort-Gap mit dem Register gefallen, Offen-Register fortgeschrieben
  class: handover
  date: 2026-10-11
  sha256: 4b28749767c97a079470786668e037387c616cae0232f4a5ea894bf12436ddf1
  status: live
-->
# Handover — Mountain-Folge 311 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge310.md` (→ `archiv/`).
flash only, kein pro/max. Operator-Trigger dieser Session: „kannst du das bitte
fixen" (river-Commit-Gate-Blockade).

## Burn: open 0.0 · close 0.02 · cap 0.15 — Grund: 1× line (flash only), kein pro/max; session_burn total 2.99→3.01, 37 Sessions

## Offen (aufgeschlüsselt)

### particle-cern — Baskets + Sample-Verifikation
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** der nächste `subset`-Lauf (CI führt die Tests aus)
- **Lage:** (gemessen 2026-10-11 via `src/archivar/root.rs`) Element-Tail + Class-Ref + Bootstrap (`TBranchElement`/Ref-Tags); **Basket-Arrays** (`fBasketBytes/fBasketEntry/fBasketSeek`) gebaut, Basis-Wrapper (`TBranchElement` v9 → TBranch v12) konsumiert; `cargo check --all-targets` 0/0.
- **Blockade:** `cargo test` läuft nur in CI; v9-Body ohne v12-Basis nur aus ROOT-Quelle + synthetisch geprüft.
- **Braucht:** `subset` grün; ggf. ein v9-only-Sample.

### Beobachtungsoperator + Fit — Schritt 2 (ref_hz vertakten)
- **Status:** eigen (Bau/Messung) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt
- **Lage:** (gemessen 2026-10-11, unverändert) `ReferenceStage`/`reference_stage`/`ref_uplink_hz` (`src/archivar/odf.rs:2218-2242`) stehen; der Konsument `tools/measure/src/bin/pioneer10_odf_residuum.rs:502,510` behandelt `ref_hz` noch als Skalar.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** die zwei `pub`-Funktionen im Konsumenten verdrahten; den TSF-vs-Uplink-Diskriminator (dt12/13) bestätigen; dann Troposphäre/Ionosphäre in die Kette.

### ci-gate — Trigger: nächster abgeschlossener Lauf
- **Status:** eigen (Messung) | **Bindung:** eigen
- **Trigger:** der nächste abgeschlossene `ci-gate`-Lauf am HEAD
- **Lage:** (gemessen 2026-10-11 via `ci_manage status`) `ci-gate`-Läufe in Flug (`38125644767`/`38125533625`); die 18 Lints in `77a7f7e71` getilgt.
- **Blockade:** kein abgeschlossener Lauf am HEAD `260f995c9`.
- **Braucht:** `ci_manage log <ci-gate-id>` auf dem nächsten abgeschlossenen Lauf → grün/rot; bei rot die verbliebenen Lints tilgen (kein `#[allow]`).

### PEP + Tudat — Schritt 3 (mehrere Bahnen)
- **Status:** eigen (Register + Bau) | **Bindung:** eigen
- **Trigger:** Schritt 3 — weitere Zeugen (Merkur/Venus Radar+VEX-Ranging) durch die Kette
- **Lage:** (gemessen 2026-10-11, folge306) vier Häuser als `ephemeris_*` registriert; Träger-Doks `docs/concepts/eigene-ephemeride.md`, `docs/surveys/survey-2026-10-10-ephemeris-quellen.md`.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** weitere Zeugen durch die Kette, Kette CI-nah verankern.

### Mycelium-φ-Blöcke — Serien-Manifest-Format
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** Entwurf des Serien-Manifest-Formats
- **Lage:** (gemessen 2026-10-11, unverändert) der ecad-Riss ist gelöst (netloc = Quellen-Identität); AODN-`ttl 1` = Datensatz-Kadenz, kein fester Refresh.
- **Blockade:** ohne Serien-Manifest trägt jeder Klima-Block nur einen Default-Tag.
- **Braucht:** Serien-Manifest-Format; der CI-Wait je Block steht in `state/zustand/wartend.φ`.

### — gefaltet: `future-folge228` → mountain

### MODVOLC · ITS_LIVE · MASIE — apathogene Zulassung (future-228)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** Regel-Zulassung `docs/SOURCE_PORT.md:76` (kein Operator-Wort) + Belege
- **Lage:** (gemessen 2026-10-11) `archive_search` über `phi` = 0 Treffer für `modvolc`/`its_live`/`masie`; Belege `state/future/revive-kandidaten-2026-10-11.md`. MODVOLC `http://modis.higp.hawaii.edu/data/<jahr>/` 200 · ITS_LIVE STAC `stac.itslive.cloud` 200 + S3 `its-live-data.s3.amazonaws.com/datacubes/catalog_v02.json` 200 · MASIE Auth-Route (`EARTHDATA_EDL_TOKEN` liegt in `.secrets.local`). Riss benannt: Future-Route (behalten) vs. Mountain-Akt (Register leer, keine Admission).
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** die Verdikt-Zeilen in `phi/sources.φ` setzen.

### SuperMAG-Ersatz — Substurm-Onset-Katalog (future-228)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** bleibt die SuperMAG-Redistribution ungeklärt
- **Lage:** (gemessen 2026-10-11) Zenodo **CC BY 4.0** McPherron `10.5281/zenodo.22711679` (314 560 Onsets, 1966–2025; Code `10.5281/zenodo.22693469`), Ohtani & Gjerloev `10.1029/2020JA027902`; Roh-Magnetometer `INTERMAGNET` (`phi/sources.φ:2422`).
- **Blockade:** SuperMAG-Verdikt offen.
- **Braucht:** Klasse/Verdikt/`sources.φ`-Zeile.

### — gefaltet: `mycelium-folge305` → mountain

### VNP46A3 — φ-Block-Verdikt
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** Mycelium-Direktive liegt vor
- **Lage:** (gemessen 2026-10-11) `url`/`origin`/`compiler`/`format`/`sha256` `d85ee99b…` gemessen (`mycelium-folge305`).
- **Blockade:** keine.
- **Braucht:** das Mountain-Verdikt (`terms`/`at`/`ttl`) in **denselben** Block.

### open-lidar-data — φ-Block-Verdikt
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** Mycelium-Direktive liegt vor
- **Lage:** (gemessen 2026-10-11) `url`/`origin`/`compiler`/`format`/`sha256` `e6560344…` gemessen; `register_sort` verlangt eine `ttl`-Zeile.
- **Blockade:** keine.
- **Braucht:** das Mountain-Verdikt (`terms`/`at`/`ttl`) in **denselben** Block.

### Vier Portal-Coverage-φ-Blöcke (mycelium-305)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der `coverage-manifest`-Lauf je Portal
- **Lage:** (gemessen 2026-10-11) Netlocs `ns_hwh.fundaments.nl` · `data.geo.admin.ch` · `nz-elevation.s3-ap-southeast-2.amazonaws.com` · `data.geopf.fr`.
- **Blockade:** keine.
- **Braucht:** je Portal `url`/`origin`/`compiler`/`format` + Mountain-Verdikt (`terms`/`at`/`ttl`) in einem Block.

### Sechs Klima-Quellen — `terms`/`ttl` gegenprüfen (mycelium-305)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der Serien-Manifest-Entwurf
- **Lage:** (gemessen 2026-10-11) die Compiler erzeugen je-Akt-Instanzen (Station/Tag/Variable), die Blöcke sind Workflow-Default-Repräsentanten.
- **Blockade:** ohne Serien-Manifest trägt jeder Block nur einen Default-Tag.
- **Braucht:** `terms`/`ttl` je Compiler gegenprüfen.

### te-ground-truth — Riss + GIC-Paper-Trigger (mycelium-305)
- **Status:** eigen (Messung) | **Bindung:** eigen
- **Trigger:** der GIC-Paper-Trigger (gefeuert) — §3.5/Abstract/§7 nachziehen
- **Lage:** (gemessen 2026-10-11) Lauf `te-bias-n 38038722712` (head `ea4025b28`) success; Produktionsarm (Takens-embedded KSG, dim 3, `omega.rs:489`) **NOT PASS** — Null-Control c=0 zeigt in beiden Richtungen einen Pfeil; der skalare Silverman-KDE-Referenzarm (`docs/paper/gic-causal-driver.md:655-660`) nullt den Rückkanal nicht. Riss getragen, nicht geglättet.
- **Blockade:** kein Code-Äquivalent beider Arme unter dem Null-Control.
- **Braucht:** den Riss in §3.5/Abstract/§7 des Papers ziehen (Mountain) oder den Null-Control-Arm angleichen.

### — gefaltet: `river-folge184` → mountain

### `omni_imf_bz_gsm_nt` — Dreifach-Registrierung entflechten
- **Status:** eigen (Register/Parser) | **Bindung:** eigen
- **Trigger:** der Rat-Befund (`state/stimmen/2026-10-11-river-panel-aufloesung-rat.md`) liegt vor
- **Lage:** (gemessen 2026-10-11) `phi/sources.φ:742` HAPI-Live (3600) · `:1935` Tages-Bin `omni2_serie.bin` (86400) · `:1950` 1h-Bin `omni2_serie_1h.bin` (3600); Überlappungs-Korrelation Tages- vs 1h-Bz = 0.6277 über 19935 gemeinsame 86400s-Perioden (Artefakt `38119733631/field-te-panel-vlies.txt`) — die zwei Registrierungen tragen nicht dieselbe Größe. Codeweg gebaut (`3093e9432`): `load_field_across_sources` liest `median_dt` je Kandidat, nennt den Zeugen; verschiedene Aggregation → Riss.
- **Blockade:** die drei Zeilen tragen denselben Namen über verschiedene Aggregation.
- **Braucht:** die drei Zeilen entflechten (getrennte Namen je Aggregation), den HAPI-Text-Arm um `omni_imf_bz_gsm_nt` ergänzen.

### omegaflow-measure — zwei Bins rot (river-184)
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt
- **Lage:** (gemessen 2026-10-11 am HEAD `260f995c9`) `tools/measure/src/bin/eclipse_shadow_probe.rs:274` ruft `orientation_angles_at(props, jd).2` auf einem `Option<(f64,f64,f64)>` (`src/archivar/motion.rs:59`); `tools/measure/src/bin/orientation_probe.rs:452-487` baut `BodyProperties`/`BodyEphemeris` mit entfernten Feldern.
- **Blockade:** die zwei Bins sind nicht an die `Option`/`medium`-Signaturen gezogen.
- **Braucht:** die zwei Bins anpassen (oder entfernen) → `cargo build -p omegaflow-measure --bin <name>` grün.

### FMHY-Routing — Riss 4 bot-gated
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Riss 4/5-Abschluss — Overpass bot-gated (gemessen 2026-10-10)
- **Lage:** (gemessen 2026-10-10, unverändert) Overpass — HTTP 406 direkt+Proton → bot-gated.
- **Blockade:** Overpass bot-gated (HTTP 406 direkt+Proton).
- **Braucht:** Riss 4-Abschluss oder Descope.

### Flyby-Kette — termin 2026-11-01
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-09) 157 ODF-Referenzen; `doppler.rs` absent; Wahrheit `state/zustand/wartend.φ`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### Exposom-Matrix → TE-Paar-Feed — Repräsentativpunkt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** extern gedeckter Repräsentativpunkt je Zeile (Operator-Wort 2026-10-10)
- **Lage:** (gemessen 2026-10-11, `--fired`) kein offener Datensatz trägt eine im Datensatz gemessene Koordinate.
- **Blockade:** der Repräsentativpunkt ist eine wissenschaftliche Annahme, keine Messung.
- **Braucht:** je Zeile den extern gedeckten Repräsentativpunkt → `phi/sources.φ`-Zeile → TE-Paar-CI-Feed.

### iEEG — registriertes Wort maßgeblich
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort über 2026-10-06 hinaus
- **Lage:** (gemessen 2026-10-11, `--fired`) iEEG = privates Experiment (`state/zustand/wartend.φ`), kein CDN.
- **Blockade:** keine.
- **Braucht:** nur ein neues Operator-Wort öffnet es.

### api.sensor.community — zulassen, Zielzeile unbenannt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das wörtliche Future-Wort (welcher Arm/welche Zeile)
- **Lage:** (gemessen 2026-10-11) HTTP 200, sha256 `ad345805…`; `sensor.community` bereits registriert.
- **Blockade:** „zulassen" nennt keinen Arm/keine Zeile.
- **Braucht:** das wörtliche Future-Wort.

### Asservatenkammer-Survey-Marker — Prosa-Reconciliation
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** die Marker-Reconciliation des Surveys
- **Lage:** (gemessen 2026-10-11) `docs/surveys/survey-2026-10-08-research-api-mcp.md`: Status-Absatz trägt den gemessenen Stand, die `Top-Schritte` (`:55-74`) noch die stale „Braucht: Key…"-Sprache.
- **Blockade:** keine.
- **Braucht:** die `Top-Schritte` auf den gemessenen Status ziehen (Header-sha neu).

### docs/concepts/bindings-dust-maske-gebco.md — Body stale
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** die Prosa-Reconciliation des Docs
- **Lage:** (gemessen 2026-10-11) see-also im Atom 310 gezogen; der Body (`:11`, `:95-99`) nennt noch `phi/bindings/*.φ`, und `:102` referenziert `phi/witnesses.φ` — seit 310 in `phi/sources.φ` gefaltet.
- **Blockade:** keine.
- **Braucht:** Body + `witnesses.φ`-Referenz (`:102`) auf das gemessene Heim `phi/sources.φ` ziehen.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern; Lauf lokal/silent,
  nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen:
  der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„mach das ab jetzt automatisch — committe und pushe selbst" | 2026-10-07 | Operator (Session, Mountain 264)
„kannst du das bitte fixen river schreibt: Der Commit ist bereit, aber der Gate blockiert" | 2026-10-11 | Operator (Session, Mountain 311)
„alle 5 an agenten und weitere agenten für 3" | 2026-10-11 | Operator (Session, Mountain 310)
„verstehst du nicht ich will die dead sources leer haben kein mensch interessiert was da drin steht ausser tote quellen die wir haben wollen" | 2026-10-11 | Operator (Session, Mountain 310)
„harvest ... sind arbeitsanweisungen die anderen drei sind daten ... und es sind nur 5 footprints müssen die nicht wirklich in die yamls" | 2026-10-11 | Operator (Session, Mountain 310)

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch",
2026-10-07) trägt Commit und Push. **Dieses Atom (Mountain 311):**
- **river-Gate-Blockade geheilt.** Gemessen: das rote `tools-build 38123001050` lief auf
  `e87398886` — **vor** dem Fix `94129c942` (der `band`/`footprint`-Initializer in
  `tools/utils/src/bin/volume_builder.rs:351-352`). Am HEAD `260f995c9` ist `volume_builder.rs`
  vollständig; `tools-build` ist grün (`38125533617`, `38124162720`). Die lokale
  `.tools_ensure`-Manifest-Kopie war auf `18c95157…` gepinnt (TTL); nach `rm
  target/release/.tools_manifest.checked` lädt sie die frische Release `853e6bc9…`
  (`commit_check`); `commit_check` + `concurrency_contract` über das gestagte Set → rc=0.
  **Kein `--no-verify`-Wort nötig** — river committet mit grünem Gate; kein fremdes
  gestagtes Stück angefasst.
- **Adressierte Blöcke gefaltet:** `future-folge228` (MODVOLC/ITS_LIVE/MASIE-Zulassung, SuperMAG-Ersatz),
  `mycelium-folge305` (VNP46A3, open-lidar-data, vier Portal-Coverage-Blöcke, sechs Klima-Quellen,
  te-ground-truth-Riss), `river-folge184` (omni_imf-Dreifach-Registrierung, omegaflow-measure rot).
- **register_sort-Gap mit dem Register gefallen:** `phi/blocked_sources.φ` existiert nicht mehr
  (`phi/` = `sources.φ`); `register_sort phi/sources.φ` = canonical (rc=0) — der `blocked`-Opener-Punkt entfällt.
- Stehender Pass zitiert, nicht kopiert. Geteilter Baum unangetastet:
  `docs/concepts/4d-membrane.md`, `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md`,
  `src/archivar/*`, `tools/utils/src/bin/archive_search.rs`, die river-/mycelium-handover-Stücke.
- Commit dieses Atoms: `Mountain 311` (dieses Handover + Archiv-Move).
