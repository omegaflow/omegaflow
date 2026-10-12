<!--
  title: Handover — Mountain-Folge 325 (2026-10-12)
  session: Mountain-Linie in einem Pass — Auftrag „Eigene Ephemeriden + Kulturerbe" angelegt und von Rat (5 Stimmen) + vollem UI-Roster gehalten; M10-EDR-Entscheidungsmessung (Riss); Register-Kanten (terms/format); Gate-Regel prior-work-unread; ui-seats Runden-Reihenfolge
  class: handover
  date: 2026-10-12
  sha256: 4685c4d9d1596382fb7d06f0d57ead2f6a9a911978bf3f29cebbbdaf31b92da5
  status: live
-->
# Handover — Mountain-Folge 325 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der Stehende
Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese Session
konsumierte `handover-2026-10-12-mountain-folge324.md` (→ `docs/handover/archiv/`).
**Jeder offene Punkt nennt ab jetzt seine Datei(en)/Pfade.**

## Burn: open 0.00 · close 0.30 · cap 0.50 · reason: lange Linien-Session (Mountain) + 6 flash-Taucher (M1-Bau, 2× Lizenz/Format, Gate-Regel, Rat, Bestands-Sichtung), dazu ein UI-Roster-Round; deepseek-flash, kein pro/max.

## In diesem Atom erledigt (git trägt es; nicht erneut vorlegen)

- **license-vocabulary-Rot geheilt:** `OGL-3.0`→`OGL-UK-3.0`, `open-by-attribution`→`free-open`,
  `etalab-2.0` in die Kanonliste — in `tools/register/src/bin/license_census.rs` + `phi/sources.φ`.
  (commits `a0d2c04e2` Vorgänger, `1bfffb010`).
- **MCON `declared_fields` gebaut:** `src/archivar/mcon_event.rs` (`COLUMNS`+`declared_fields`),
  `src/archivar/extract.rs` (2 Arme), `src/archivar/units.rs` (`"min" => *60`).
- **Gate-Regel `prior-work-unread`:** `src/gate/commit_gate.rs` + `src/gate/commit_gate_vocab.json`
  — handover-gescannt, flaggt einen Vorarbeits-Artefakt unter `state/` mit Byte-Größe ohne Re-Meßstempel.
- **M1-Proxy gebaut:** `tools/measure/src/bin/m10_expected_diameter.rs` (Winkeldurchmesser/FOV).
- **Auftrag angelegt + gehalten:** `docs/auftrag/auftrag-eigene-ephemeriden-und-kulturerbe.md`
  (sha `4a9dac1da4a9a9c95d39d6237af657792340076898592f1f16a02a3470d7f937`); Rat (5 Stimmen) +
  UI-Roster. commits `e797cd20e`, `2ce575935`, `402835ee3`.
- **UI-Roster-Stimmen registriert:** `state/stimmen/2026-10-12_ui_eigene-ephemeriden-verdikt.md`.
- **Bestands-Karte:** `state/mountain/mariner-ephemeris-bestand.md` (Mariner + Ephemeriden).
- **Runden-Reihenfolge kodifiziert:** `docs/concepts/ui-seats.md` (commit `fbea1747d`).
- **Mycelium 323** (`9e370d0e7`) hat die M10-Terms gesetzt (PSJ `CC-BY-4.0`, USGS `PD`,
  LPSC/camera_angles `unknown`, UCalgary keogram `free-open`) — mein R1/R3-Ask ist erledigt.

## Offen (aufgeschlüsselt)

### Auftrag „Eigene Ephemeriden + Kulturerbe" — Phase 0/1 (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Operator-Wort zum Start
- **Lage:** (gemessen 2026-10-12) `docs/auftrag/auftrag-eigene-ephemeriden-und-kulturerbe.md`
  (sha `4a9dac1d…`). Rat + UI (Qwen · Claude · DeepSeek Chat · DeepSeek V4 Pro · Qwen3.8 2.4T ·
  GLM Deep Think Max) **konvergent**: EIN Archivar, zwei Register (`obs`/`heritage`/`observable`/
  `asset`); `house_gate` → Konsistenz-Hülle, ehrliches Gate = **Hold-out**; minimaler Operator
  mit `pending` (nie 0.0); **Riss 1** M10-Bild = Erbe; **Riss 2** Anker (a-priori/Massen) reisen
  mit; `raw_level` 0/1/2 (Qwen3.8 2.4T); **Ordnung** Rahmenschicht → LLR (relativ) → ODF als
  Prüfling → Radar → ΔDOR/VLBI → Tiefraum; **Erbe→Brennstoff** beim Stand der inneren Region.
  Stimmen: `state/stimmen/2026-10-12_ui_eigene-ephemeriden-verdikt.md`.
- **Blockade:** CDDIS/Earthdata (`blocked account`, `state/zustand/wartend.φ`); LLR-Beschaffung
  = Politik (Operator/Future).
- **Braucht:** Phase 0 = `archive_search --all` je Beobachtungsklasse → Text; Phase 1 =
  ITRF2020 (`https://itrf.ign.fr`) + IERS-EOP + LLR (ILRS/CDDIS, historisch POLAC/Z&N) +
  Operator `canObserve`.

### M10-EDR Winkeldurchmesser — Riss: EDR-Korpus ↔ M10-Ephemeride disjunkt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eine M10-Ephemeride, die den Venus-Encounter trifft
- **Lage:** (gemessen via `tools/measure/src/bin/m10_expected_diameter.rs`) EDR-Asset
  `mariner_m10_edr.bin` (`phi/sources.φ:31385-31390`, `mvm_0001.tgz`, `ser.im-ldi.com`) = 651
  Frames, alle **Venus 1974-02-05/06**; `data/ssd.jpl.nasa.gov/ephemeris_mariner10.bin` deckt nur
  **Merkur-1 (JD 2442128–2442141)**. NAIF `…/naif/M10/kernels/spk/` nur `M10_archive_1.bsp`. 0 meßbar.
- **Blockade:** keine breite M10-Ephemeride / kein passender Korpus.
- **Braucht:** JPL Horizons `-76` via `tools/harvest/src/bin/horizons_compiler.rs` /
  `ephemeris_horizons_check.rs` (Mycelium `url`/`compiler`) **oder** Merkur-Tapes `mvm_0002+`
  (`ser.im-ldi.com/M10/MVM/`).

### M10-Kontrollnetz (ISIS) — Erbe (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Phase 4 des Auftrags
- **Lage:** (gemessen 2026-10-12) `state/research/m10-mdis-sonke-robinson/MANIFEST.md` (+ `psjae32f5t1..t4`,
  `sonke-robinson-2026-psj-fulltext.jina.md`); `state/mountain/mariner4_dsc0119_format.md`;
  `https://ser.im-ldi.com/M10/M10_ISIS/camera_angles.txt` (sha `0a65df59…`, 194 640 B, Pose je Bild);
  Table 1 σ0 = 0,570 px.
- **Blockade:** ISIS-Quelle unverifiziert (USGS/PD, **nicht** Zenodo/Unlicense); `SXSY SZ`-Einheit ungelegt.
- **Braucht:** `https://astrogeology.usgs.gov/software/ISIS` + `https://github.com/DOI-USGS/ISIS3`
  messen; `SXSY SZ`-Format pinnen.

### JADES `.tar` → Parser-Arm `tar_fits` (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau-Pass
- **Lage:** (gemessen 2026-10-12 via `sread phi/sources.φ`) `phi/sources.φ:32235-32237` (`jades_early_galaxies.tar`); `terms CC-BY-4.0` gesetzt;
  **kein** Plain-`.tar`→FITS-Arm (nur `tar_gz_yaml` gzip-only; `fits_to_json` nur BINTABLE).
- **Blockade:** neuer Parser-Arm.
- **Braucht:** `tar_fits` (Plain-Tar via `inflate::tar_members` + FITS-Image-Ingest), dann `format tar_fits`.

### BK18 `text` — Zeilen-Direktiven fehlen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau-Pass
- **Lage:** (gemessen 2026-10-12 via `sread phi/sources.φ`) `phi/sources.φ:31292-31294` (`bk18_bandpowers_20210607.txt`, `terms CC-BY-4.0`); Muster
  Venera-Block `phi/sources.φ:32300` (`format text`/`at`/`rows .`/`quantity`).
- **Blockade:** Spaltenlayout ungemessen.
- **Braucht:** BK18-Spaltenköpfe messen → `format text`/`at earth`/`rows .`/`quantity`.

### `mariner_rst` — Summary-Tapes + Observable-Skala (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11 via grind-flash) `tools/harvest/src/bin/mariner_rst_compiler.rs`; Parser auf M6 `DD005408`/M7 `DD005410`
  verifiziert; `DD005407`/`DD005409` sind strukturell andere Summary-Tapes; Skalierung (Frequenz roh
  1.08e12, /2⁹ ≈ 2114 MHz) gegenzuprüfen.
- **Blockade:** Zusatz-Tapes brauchen eigene Messung.
- **Braucht:** Summary-Tapes messen oder `descoped`; gegen DSC 0073/Spec prüfen.

### DSC 0119 / M4-Feldtabelle — Parser-Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `url`-Zeile (Mycelium)
- **Lage:** (gemessen 2026-10-12 via `sread`) `state/mountain/mariner4_dsc0119_format.md` (ODG/ODP-Record, 6-bit-Byte-Walk, Zeit dekodiert);
  **Riss:** Table 3 (Multi-Datentyp-Recordlänge) fehlt im 16-S.-Scan.
- **Blockade:** `url`/`origin` (Mycelium); Table 3.
- **Braucht:** DSC-0119-`url` (Mycelium) + Table 3 pinnen → `mariner_rst`-Arm.

### THEMIS-ASI Stationstabelle — river-196-Korrektur (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Bau-Pass
- **Lage:** (gemessen 2026-10-12 via river-196) river-196 (`docs/handover/handover-2026-10-12-river-folge196.md`) übergibt: Geodätische
  Höhe = ellipsoidale Höhe; 16/24 Stationen nominal 0.0; die Archivar-Tabelle trägt stadt-abgeleitetes
  `H` und braucht Korrekturen an **PGEO/EKAT/ATHA/MCGR**.
- **Blockade:** keine.
- **Braucht:** `src/archivar/themis_asi.rs::STATIONS` an PGEO/EKAT/ATHA/MCGR korrigieren;
  WHIT/GBAY `H` bleiben `pending` (keine Punktmessung).

### `solar_f107_flux_sfu` — Zeitbasis-Defekt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Bau-Pass
- **Lage:** (gemessen 2026-10-12 via `general`) `src/archivar/f107.rs:22` (rohe Unix-Sekunden) vs. `src/archivar/extract.rs:6506`
  (TDB/J2000); `series_epoch_of` (`extract.rs:6506-6539`) wählt den ersten `"date"`-Key.
- **Blockade:** beide Arme brauchen dieselbe Epoch-Basis (Signatur-/Aufrufer-Änderung).
- **Braucht:** beide auf TDB; `f107`-Join erneut messen.

### `MCON`-Spanne (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-12 via mycelium-318/river-194) `phi/sources.φ:21545-21552` (`mcpherron_onsets.bin`, `format mcon_event`); Zenodo nennt
  `1966–2025`, Parser endet 2024-05-29.
- **Blockade:** keine.
- **Braucht:** ersten/letzten Onset messen.

### `intermagnet_dbdt` — Resolver-Regel (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Freigabe von `tools/measure/src/bin/field_te_query.rs`
- **Lage:** (gemessen 2026-10-12 via Rat) Defekt `field_te_query.rs:1263-1341` (prüft `abk_1m` gegen `sod_1h`, läßt gröberen Zeugen fallen).
- **Blockade:** Datei hält parallele Session.
- **Braucht:** nach `station_code` gruppieren + gröberen Zeugen als Riss daneben tragen.

### Quellen-Terms — not-admit/declined-Disposition (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Rat/Operator-Wort
- **Lage:** (gemessen 2026-10-12 via `sread`) `phi/sources.φ:11887-11893` (`oe417`, kein `terms`); BGS-GIC/NERC-GMD ohne Block; es gibt
  **keine** Register-Direktive für ein not-admit-Verdikt (`src/archivar/parse.rs:265` = freier String).
- **Blockade:** fehlende Register-Form.
- **Braucht:** gemessene Register-Form **oder** `descoped`; OE-417 entscheiden.

### Orphan-Prosa-Träger (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-/Verdikt-Pass
- **Lage:** trägerlos (gemessen): `docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md` (51) ·
  `docs/concepts/kybernetische-astrophysik.md` (1) · `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (7) ·
  `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` (1) · `docs/surveys/survey-2026-10-08-open-sources-delta.md` (12) ·
  `docs/surveys/survey-2026-10-09-domaenen.md` (4) · `docs/surveys/survey-2026-10-10-ephemeris-quellen.md` (5) ·
  `docs/surveys/survey-raetsel-bestand.md` (5).
- **Blockade:** die Verdikte sind Mountain-Recht.
- **Braucht:** je Klasse eine Träger-/Verdikt-Zeile **oder** `descoped`.

### Re-Admission der lizenz-blockierten Kandidaten (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Verdikt-Pass
- **Lage:** (gemessen 2026-10-11, `survey-2026-10-09-redistribution-alternativen.md`) `docs/surveys/survey-2026-10-09-redistribution-alternativen.md` §Dritter Lauf: ALMA→ESO,
  NC-admissibel: INTERMAGNET (CC BY-NC 4.0), GIRO DIDBase (CC BY-NC-SA 4.0).
- **Blockade:** Admission-Verdikte je Kandidat fehlen.
- **Braucht:** Admission je Kandidat; danach Mycelium-CDN.

### `api.sensor.community` — Trigger leer (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste Register-Pass
- **Lage:** (gemessen 2026-10-11/12) `phi/sources.φ:338`; kein Future-Wort (Ursprung `mycelium-158…162` 403 + `mountain-306`).
- **Blockade:** keine (der Punkt hängt an der eigenen Namenswahl).
- **Braucht:** Arm/Zielzeile benennen **oder** `descoped`.

### Rätsel-Datenlücken (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `url`-Zeilen (Mycelium)
- **Lage:** (gemessen 2026-10-12 via `sread`) `phi/sources.φ:32226` (UNCOVER `14281664`, `format fits` + `CC-BY-4.0` gesetzt) ·
  `:32201` (JADES `7460116`) · `:31292` (BK18) · GIC-Mäntsälä (FMI NUR `space.fmi.fi`).
- **Blockade:** `url`/`origin` (Mycelium).
- **Braucht:** je Lücke die `url`-Zeile → `terms`/`format` → Parser.

### NIST NCNR NSE · Teil C A5 · Band-Paar-Atom · opentopography · SWPC/RTSW · Sechs Klima-Quellen · Eigene Ephemeride · Frame/Epoche-Sammel (eigen/wartend/blockiert)
- **Status:** eigen/wartend/blockiert | **Bindung:** eigen/termin:2026-11-01
- **Trigger:** je der benannte (CDN-/Manifest-Lauf, `url`-Zeile, Freigabe `src/mathematikerin`, Termin 2026-11-01)
- **Lage:** (gemessen 2026-10-11 via 319-Register) NIST NCNR NSE (`https://ncnr.nist.gov/pub/ncnrdata/nse/`, US-PD) braucht `url`; Teil C A5 `GO-SS-RSS-1-TDF-V1.0` (PDS PPI) braucht `url`; Band-Paar `pioneer-odf-cdn.yml:21-31` (Idempotenz-Sperre); opentopography_coverage braucht Manifest; SWPC/RTSW braucht Archivreihe-Verdikt+`ttl`; Sechs Klima-Quellen (`series-manifest`) brauchen den ersten Lauf; eigene Ephemeride blockiert an `src/mathematikerin`; Frame/Epoche CRS-VLR ungemessen.
- **Blockade:** `url`/Manifest (Mycelium); die Freigabe von `src/mathematikerin`.
- **Braucht:** je `url`-Zeile (Mycelium) → `terms`/`format` (Mountain); Läufe lesen (`ci_manage log <id>`); iEEG **nicht erneut vorlegen** (Operator-Wort 2026-10-06).

## An mycelium

Origin: mountain-folge325 (2026-10-12).

- **M10-EDR/M10-Ephemeride:** für den Venus-Encounter (M1-Riss) brauche ich eine **breite
  M10-Ephemeride** — JPL Horizons `-76` (`tools/harvest/src/bin/horizons_compiler.rs`,
  `ephemeris_horizons_check.rs`) oder die Merkur-Tapes `mvm_0002+` (`ser.im-ldi.com/M10/MVM/`).
  `url`/`origin`/`compiler` je Mycelium, dann `terms`/`format` Mountain.
- **DSC 0119:** `url`-Zeile für `spdf.gsfc.nasa.gov/pub/documents/old/documentation_from_nssdc/renamed_dataset_catalogs/mariner4_celestial-mechanics_64-077a-09a_DSC_0119.pdf`.
- **Rätsel/NSE-Identität:** JADES `format` offen (`tar_fits`-Arm fehlt), BK18 `format text`+rows offen.
- **Danke (323):** die M10-Terms (PSJ CC-BY-4.0, USGS PD, UCalgary free-open) sind gesetzt.

## Operator-Wort-Register

- **„bitte mache das richtige … WIR BAUEN UNSERE EIGENEN EPHEMERIDEN!!!! und wir retten KULTURERBE"** | 2026-10-12 | Operator-Session (mountain) — Auftrag angelegt; kein Quick-and-dirty.
- **„ja bitte lege ihn an und route ihn und wichtig den kompletten UI roster … openweight max und seek … nicht nur duck und claude"** | 2026-10-12 | Operator-Session (mountain) — Rat + voller UI-Roster.
- **„du kannst die parallel fahren und glm kannst du abholen"** | 2026-10-12 | Operator-Session (mountain) — Open-Weight parallel; GLM abholen (ich hatte es zu früh `pending` gesetzt).
- **„verstehst du jetz warum ich auf glm 5.3 bestehe?"** | 2026-10-12 | Operator-Session (mountain) — GLM Deep Think Max trägt den tiefsten Riss (Selbstbestätigung der Unabhängigkeit).
- **„ist denn geschwindigkeit wirklich ein problem … ihr müsst euch nur angewöhnen die ui chats richtig zu nutzen … glm start > pending schluss du hast gesehen dass es anders geht"** | 2026-10-12 | Operator-Session (mountain) — Runden-Reihenfolge (langsamstes zuerst, zuletzt abholen), `pending` erst nach gemessener Frist; in `docs/concepts/ui-seats.md` kodifiziert.
- **„warte ich habe kein glm guthaben soll ich welches kaufen … oder reicht unser kostenfreier weg?"** | 2026-10-12 | Operator-Session (mountain) — Antwort: der kostenfreie Weg reicht; Kauf kauft Tempo, nie Urteil.
- **„übergeben … komplett nicht nur oberflächlich … die pfade mit in eure übergabe schreiben"** | 2026-10-12 | Operator-Session (mountain) — diese Übergabe trägt Pfade/Dateinamen je Punkt.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch",
2026-10-07) trägt Commit und Push. In diesem Atom committet: `docs/auftrag/auftrag-eigene-ephemeriden-und-kulturerbe.md`,
`docs/concepts/ui-seats.md`, `docs/handover/handover-2026-10-12-mountain-folge325.md`,
`docs/handover/archiv/handover-2026-10-12-mountain-folge324.md`, `phi/sources.φ`,
`src/archivar/*`, `src/gate/*`, `tools/register/src/bin/license_census.rs`,
`tools/measure/src/bin/m10_expected_diameter.rs`.
