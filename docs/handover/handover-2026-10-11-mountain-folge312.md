<!--
  title: Handover — Mountain-Folge 312 (2026-10-11)
  session: Mountain-Linie in einem Pass — Wartend-Register gegen den Baum gemessen und bereinigt (Operator-Wort), zwei rote measure-Bins geheilt, zwei Prosa-Docs auf den Baum gezogen
  class: handover
  date: 2026-10-11
  sha256: 59b69113a6fd9a3b062e253f4c23d5b8e565c44e5220643194994efe58fa5ed1
  status: live
-->
# Handover — Mountain-Folge 312 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge311.md` (→ `archiv/`).

## Burn: open 0.0 · close 0.24 · cap 0.25 — Grund: 1× line (flash only), kein pro/max; Taucher (2× grind-flash, 1× general) im selben Atom

## Wartend-Register (Operator-Wort 2026-10-11: „die wartend vornehmen … gegen den Baum gemessen")

`state/zustand/wartend.φ` gegen den Baum gemessen und bereinigt (71 → 53 Zeilen;
Archiv der Vor-Fassung: `state/zustand/wartend-archiv-2026-10-11.φ`). **Entfernt
(gemessen aufgelöst):** `ned-token` (`phi/sources.φ:11597` `ned_byparams_redshift.json`
steht), `laic-cses` (Operator-Wort „Nein", der Akt entfällt), `petrel19-license`
(`phi/sources.φ:2386` `terms CC-BY-4.0`), `solar-omega-g` (`docs/concepts/zeugnis.md:275`
— 1277 nHz quellenlos entfernt, Kanal bleibt abwesend = 0 honored), `gic-paper-groundtruth`
(Artefakt gelesen, Riss → te-ground-truth unten), die 13 `#`-Resolutionen.
**Zusammengeführt:** `demeter-18400` + `demeter-refresh` (ein Wait).
**Toten Register-Verweisen gezogen** (die alten Ledger-/Dispositions-Register
existieren nicht mehr — `phi/` trägt nur `sources.φ`).
Die 12 „Compiler pending"-Zeilen (copernicus-insitu ×8, noaa ×4) bleiben — sie sind
per `AGENTS.md` der Heimat-Ort der realen pending-Compiler.

## Offen (aufgeschlüsselt)

### particle-cern — Baskets + Sample-Verifikation
- **Status:** wartend | **Bindung:** eigen
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
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste abgeschlossene `ci-gate`-Lauf am HEAD
- **Lage:** (gemessen 2026-10-11 via `ci_manage status`) `ci-gate`-Läufe in Flug; die 18 Lints in `77a7f7e71` getilgt.
- **Blockade:** kein abgeschlossener Lauf am HEAD.
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

### MODVOLC · ITS_LIVE · MASIE — apathogene Zulassung (future-228)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** Regel-Zulassung `docs/SOURCE_PORT.md:76` (kein Operator-Wort) + Belege
- **Lage:** (gemessen 2026-10-11) `archive_search` über `phi` = 0 Treffer für `modvolc`/`its_live`/`masie`; Belege `state/future/revive-kandidaten-2026-10-11.md`. MODVOLC `http://modis.higp.hawaii.edu/data/<jahr>/` 200 · ITS_LIVE STAC `stac.itslive.cloud` 200 + S3 `its-live-data.s3.amazonaws.com/datacubes/catalog_v02.json` 200 · MASIE Auth-Route (`EARTHDATA_EDL_TOKEN` liegt in `.secrets.local`). Riss benannt: Future-Route (behalten) vs. Mountain-Akt (Register leer, keine Admission).
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** die Verdikt-Zeilen in `phi/sources.φ` setzen.

### SuperMAG-Ersatz — Substurm-Onset-Katalog (future-228)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** bleibt die SuperMAG-Redistribution ungeklärt
- **Lage:** (gemessen 2026-10-11) Zenodo **CC BY 4.0** McPherron `10.5281/zenodo.22711679` (314 560 Onsets, 1966–2025; Code `10.5281/zenodo.22693469`), Ohtani & Gjerloev `10.1029/2020JA027902`; Roh-Magnetometer `INTERMAGNET` (`phi/sources.φ:2422`); SuperMAG-`substorm_*`-Blöcke `phi/sources.φ:19935-19970` tragen `terms unknown`.
- **Blockade:** SuperMAG-Verdikt offen.
- **Braucht:** Klasse/Verdikt/`sources.φ`-Zeile.

### Vier Portal-Coverage-φ-Blöcke (mycelium-305)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** der `coverage-manifest`-Lauf je Portal
- **Lage:** (gemessen 2026-10-11) `.github/workflows/coverage-manifest.yml:24-51` fährt vier Bins mit Netloc; je Bin strahlt `{CDN_BASE}/{netloc}/{name}.manifest` + `format` (`src/archivar/cdn.rs:26`); Netlocs `ns_hwh.fundaments.nl` (`ahn4_coverage`) · `data.geo.admin.ch` (`swisstopo_coverage`) · `nz-elevation.s3-ap-southeast-2.amazonaws.com` (`linz_coverage`) · `data.geopf.fr` (`ign_lidarhd_coverage`); `archive_search` über `phi` = 0 Treffer. **Riss:** die `terms`-Lizenz ist für `ahn4`/`swisstopo` **nicht im Baum gemessen** (kein Lizenz-Token im Bin/page) → kein Verdikt per Annahme.
- **Blockade:** Lizenz-Token (ahn4/swisstopo) ungemessen.
- **Braucht:** je Portal zuerst `terms <SPDX-Token> <URL>` messen, dann `url`/`origin`/`compiler`/`format` + `at earth`/`ttl` + Verdikt in einem Block.

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
- **Lage:** (gemessen 2026-10-11) kein offener Datensatz trägt eine im Datensatz gemessene Koordinate.
- **Blockade:** der Repräsentativpunkt ist eine wissenschaftliche Annahme, keine Messung.
- **Braucht:** je Zeile den extern gedeckten Repräsentativpunkt → eine Zeile in `phi/sources.φ` → TE-Paar-CI-Feed.

### iEEG — registriertes Wort maßgeblich
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort über 2026-10-06 hinaus
- **Lage:** (gemessen 2026-10-11) iEEG = privates Experiment (`state/zustand/wartend.φ`, `# ieeg-openneuro-backend` archiviert), kein CDN.
- **Blockade:** keine.
- **Braucht:** nur ein neues Operator-Wort öffnet es.

### api.sensor.community — zulassen, Zielzeile unbenannt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das wörtliche Future-Wort (welcher Arm/welche Zeile)
- **Lage:** (gemessen 2026-10-11) HTTP 200, sha256 `ad345805…`; `sensor.community` bereits registriert.
- **Blockade:** „zulassen" nennt keinen Arm/keine Zeile.
- **Braucht:** das wörtliche Future-Wort.

## An future

Origin: mountain-312 (2026-10-11).

- **tuh-eeg — Residuum aus der Wartend-Bereinigung (Operator-gebunden).** Aus
  `state/zustand/wartend.φ` entfernte `#`-Resolutionen mit offenem Rest: (a)
  **tuh-eeg-eligibility** (`# … 2026-10-04`, NEDC-Antwort `mail_ledger.φ:233`):
  Weg (a) NEDC-PDF füllen/signieren/senden wieder offen = Operator-Hand; (b)
  **tuh-eeg-access** (`# … 2026-10-07`, `mail_ledger.φ:245`): SSH-Zugang live,
  Feld-Verdikt `decline no-physical-force`; Nutzung für einen Nicht-Feld-Zweck
  `operator-gebunden`. Beide gehören in die Future-Operator-Queue (das Archiv
  `state/zustand/wartend-archiv-2026-10-11.φ` trägt den Wortlaut).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push.
