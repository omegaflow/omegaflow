<!--
  title: Handover — Sensory-Folge 177 (Stand 2026-09-26)
  session: Sensory-Folge 177
  class: handover
  date: 2026-09-26
  sha256: 32994247dcdac9ae79e88c3249182354bd6a6687a6b56283dd8ab55955cfd356
  status: live
-->
# Handover — Sensory-Folge 177 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes ist gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Nur eigene Arbeit: bei geteilten Dateien nur
die eigenen Hunks — committet wird pfad-begrenzt, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr
von HEAD ist.

Es gibt keinen `härtesten Punkt` — die offenen Punkte werden **parallel** von Agenten
abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage** (mit Messstempel) / **Blockade** /
**Braucht**. Sortierung: **umsetzbar zuerst** — (1) sofort abarbeitbar (eigen,
dispatchbar), (2) operator-gebundene Vorbereitung (Kante fertig, nur das Wort fehlt),
(3) blockiert/wartend (mit Trigger), (4) extern (Dritter). Der Akteur steht pro Punkt
in `Bindung`, nicht in der Reihenfolge. Operator-gebundene Punkte sind in Vorbereitung
(autonom) und Akt (operator-gebunden) getrennt.

Die FR945 ist das persönliche Gerät des Operators; ihre Kennung (MAC) und ihre Daten
bleiben lokal (`.secrets.local`, `data/`), nie getrackt, nie am CDN.

## Stehender Pass (automatisch, keine Auswahl)

Die fälligen Einträge aus `docs/zustand/external-state.md` werden zu Session-Beginn
gemessen; ihr Ausgang steht im Zustand-Ledger, nicht als Kopie hier. Karte:
`docs/concepts/tools-map.md`.

- **Postfach** — `smail` + `state/mail/mail_ledger.φ` (fällig 2⁶ min).
- **CI-Status am HEAD** — Watchdog-Snapshot `/tmp/opencode/ci_status.md` (nur wenn
  jünger als der letzte HEAD-Wechsel), sonst `ci_manage list`/`view`; nie
  `gh run list`/`gh run view`.
- **Artefakt-Frische** — erzeugte Klassen (Tools, Core-Bin, CDN-Assets, WGSL,
  Firmware) hinken HEAD; nie einen aktuellen Satz über ein Artefakt hinter HEAD.

## Träger-Zeilen (Orphan-Faltung)

Die folgenden Prosa-Dokumente tragen offene Marker und werden an ihre Trägerpunkte
gebunden (der Scanner `register_lookup --orphan-docs` erkennt den Träger nur, wenn
eine lebende Übergabe den Dateinamen nennt):

- `survey-2026-09-07-weberin-sonnensystem-kette.md`, `survey-2026-09-07-weberin-thread-matrix.md`,
  `survey-2026-09-13-weberin-quellen.md`, `survey-2026-09-13-weberin-quellen-folge.md`,
  `survey-2026-09-13-weberin-quellen-treffer.md`, `survey-2026-09-14-weberin-quellen-rerun.md`
  → Träger `docs/concepts/die-weberin.md` (Weberin-Punkte unten).

### Rest-Träger-Zeilen (Stand 2026-09-26)

Je Zeile ein zuletzt trägerloses Dokument: `Pfad` (offene Marker) → Trägerpunkt
oder descoped-Befund. Der Dateiname in dieser Übergabe ist der Träger.

- `docs/blatt/blatt-der-grat.md` (2) → ENSO-Pfeil messen: `cross_te_screen`.
- `docs/blatt/blatt-kreuz-screening-gyirong.md` (3) → Träger Punkt „Kreuz-Screening" (unten).
- `docs/blatt/blatt-solar-seconds-matrix.md` (1) → `corona_conditional_probe` auf das Paar 211A→193A.
- `docs/blatt/blatt-thuan-fragesteller.md` (4) → `termin:2026-12-02` (Gaia DR4), dann `docs/auftrag/archiv/auftrag-gaia-dr4-iapetus.md`.
- `docs/concepts/arxiv-api.md` (2) → Träger Punkt „arxiv HTTP 406" (Mountain-Linie).
- `docs/concepts/blatt-papier-resultat.md` (1) → Blatt-1-Bojen-Matrix-Rotor laufen lassen, Matrix-Zeile Σ p̂·M nachtragen (Z.63–71).
- `docs/concepts/das-eine-instrument.md` (2) → Träger Punkt „Das eine Instrument — Anomalie offen" (unten).
- `docs/concepts/der-paradigmenwechsel.md` (9) → Träger Punkt „JUICE-Erdpassage 28./29.09.2026" (unten).
- `docs/concepts/die-akteure-im-boden-und-wasser.md` (7) → Träger Punkt „Seismik-Flotte" (unten).
- `docs/concepts/fuenf-funken-anomalie-suche.md` (4) → Funke 3 via `broker_difference_probe`, Funke 5 (TDB-Koinzidenz-Fenster) bauen.
- `docs/concepts/recherche-galileo-kadenz-reconciliation.md` (1) → `archive_search --ntrs 19930010224` bzw. DSMS Services Catalog v7.5 §"Doppler count interval".
- `docs/concepts/the-seven-spheres.md` (2) → Träger Punkt „Sieben Sphären" (unten): Stern-Winkeldurchmesser-Feld (CHARM2, füllbar); gemessenes Δz je Okkultation.
- `docs/paper/causal-arrow-preregistration.md` (1) → `te_pair_probe` (Lag-Sweep {1,3,6,12,24,48}) gegen Rasuwa-Regen; DAHITI Koshi via `sfetch` (api_key).
- `docs/paper/corona-heating-ladder.md` (2) → Träger Punkt „Korona-Heizung" (unten): `aia_ladder_probe` über den vollen 613-Event-Korpus.
- `docs/paper/cross-screening-tibet.md` (1) → Träger Punkt „Kreuz-Screening" (unten).
- `docs/paper/depth-phase-echo-fleet.md` (5) → Träger Punkt „Seismik-Flotte" (unten).
- `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4), `docs/paper/flyby-path-2-preregistration.md` (1) → Träger Punkt „JUICE-Erdpassage 28./29.09.2026" (unten).
- `docs/paper/galileo-rotor-spin-era-floor.md` (1) → CK-Kerne jenseits `ck90341a`–`ck90344b` (Frame −77000) von `naif.jpl.nasa.gov` harvesten.
- `docs/paper/jwst-disequilibrium-survey.md` (7) → Träger Punkt „JWST Biosignatur-Kanäle pending" (unten).
- `docs/paper/laic-arrow-direction.md` (3) → `archive_search --playwright https://leos.ac.cn` (CSES SPA).
- `docs/paper/nadel-v-fresh-area-dip-scan.md` (1) → Träger Punkt „Nadel V" (unten).
- `docs/paper/probe-front-dark-matter.md` (2) → Träger Punkt „Pioneer/Dark-Matter" (unten).
- `docs/paper/solar-seconds-matrix.md` (3) → `corona_conditional_probe` auf das Paar 211A→193A.
- `docs/paper/sturzflut-tibet-pfeil.md` (23) → Träger Punkt „Trishuli" (unten).
- `docs/paper/tonga-lamb-crosscheck.md` (3) → Träger Punkt „BGR-Matched-Filter-Ankunft (Tonga)" (unten).
- `docs/surveys/axiom-gate-broken-null-control.md` (1) → Träger Punkt „Broken-Null-Control" (geschlossen — te-gate 36228804363 success).
- `docs/surveys/axiom-gate-depth-phase-echo-fleet.md` (1) → Träger Punkt „Seismik-Flotte" (unten).
- `docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md` (17) → Träger Punkte „Pioneer/Dark-Matter" + „DSN-Briefe in Flug" (unten): vier Werkzeuglücken.
- `docs/surveys/survey-2026-09-16-sonden-flotte.md` (4) → Träger Punkt „DSN-Briefe in Flug" (unten).
- `docs/surveys/survey-ein-blatt-korona-heizung.md` (1) → Träger Punkt „Korona-Heizung" (unten).
- `docs/surveys/survey-2026-09-06-codestruktur.md` (8) → Träger Stehender Pass „CI-Status am HEAD" (oben).
- `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` (9) → Träger Punkt „HRV/Puls→Strahlung" (unten, wartend).
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` (2) → Träger River-/Browser-Linie (Kaltstart-Entschärfung, Versionslücke).
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` (7) → Träger „FIT-Verifikation FR945" (operator) + „BLE-HR-Live-Messung FR945" (unten).
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` (1) → Träger Punkt „Postfach" (unten): nächster Schritt MPI-FKF/TRISP-Antwort via `smail`; die CSES-Limadou-Antwort (Sotgiu 2026-09-16, „wait a few weeks") ist Wiedervorlage.
- `docs/concepts/ein-blatt-papier.md` (2) → Lag-Sweep-Träger Punkt „causal-arrow-preregistration" (`te_pair_probe`); KDE-Bandbreiten-Gate descoped.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md`, `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md`, `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md`, `docs/concepts/mirror-research.md` (je 1) → Bestand der Mycelium-Linie (kein eigener Sensory-Schritt).

- **Alt-Orphans (2026-09-26 gemessen, trägerlos):** `docs/concepts/pfeiler-der-architektur.md` (2), `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` (3), `docs/surveys/survey-fortschritt.md` (1) — Marker = Status-Vokabular; kein eigener Sensory-Schritt. Schritt: beim nächsten Pass je Datei die Marker lesen → descope oder Träger (Aufenthalt Mycelium/Mountain).

## Offen (umsetzbar zuerst)

Die Reihenfolge ist die Umsetzbarkeit; der Akteur steht pro Punkt in `Bindung`.

### 1. Sofort abarbeitbar (eigen, dispatchbar)

#### Kreuz-Screening — `is_day`-/Selbstpaar-Ausschluss gebaut, Re-Run nach Push
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Push (dann `meteo-cdn`)
- **Lage:** (gemessen 2026-09-26 via Sub-Agent) `tools/measure/src/bin/cross_te_screen.rs` schließt `is_day` ganz aus (Quelle wie Ziel) und verbietet gleichvariable Paare (`variable_of`/`pair_admitted`, `:71–86`, `:179–184`, `:260–268`); die Paarzahl im Header ist jetzt die ehrlich zugelassene. `cargo check -p omegaflow-measure --bin cross_te_screen` 0/0. Der dispatchte Lauf `36256288763` trug den Fix **nicht** (uncommitted → CI baute HEAD `f2a88fbf`).
- **Blockade:** keine (Fix steht im Baum).
- **Braucht:** nach Commit+Push `gh workflow run meteo-cdn.yml`; Lauf lesen (`ci_manage view/log`).

#### Positive Maske — Slab2-Idempotenz-Gate gefixt, Re-Manifestation dispatcht
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Lauf `36256966495`
- **Lage:** (gemessen 2026-09-26 via Sub-Agent) `slab2-cdn.yml:21–34` gate war **name-only** → das stale 61-Record-Asset (1472 B, `67fdd44c…`, 2026-09-13) übersprang jede Re-Manifestation; Gate auf Größe erweitert (`expected_size=24504824` = 1.021.034×24+8), stale Asset per `gh release delete-asset www.sciencebase.gov slab2_depth.bin` gelöscht, Lauf `36256966495` dispatcht. ODF rev-G OCR (Format-ID Bits 129–131) bereits gefaltet (`eecce0b98`). Galileo-ODF-Trenner 820-013/209G geklärt.
- **Blockade:** keine.
- **Braucht:** Lauf `36256966495` lesen (Manifestation `slab2_depth.bin`); ODF-Beine abschließend tragen.

#### Seismik-Flotte — Compiler-Arme gebaut, CDN-Läufe offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die CDN-Läufe
- **Lage:** (gemessen 2026-09-26) LLNL-G3D-JPS-Arm (`src/archivar/llnl_g3d.rs`, `G3D1`; 3.855.119 Records, 93.046.104 B), ISC-EHB-Arm (`EHB1`; 153.959 arrivals), EMC-Arm (`src/archivar/emc.rs`, `emc_radial.bin`), Slab2-Arm (`.grd` netCDF-4 → 1.021.034 Records, Wayback-Route) gebaut; Quellen in `sources.φ` registriert. Der EMC-3D-netCDF-**Katalog**-Arm ist **`descoped`** (der volume-Arm + `hdf5.rs`/`nc4.rs` + 8 `*.volume.bin` tragen die 3D-Modelle).
- **Blockade:** keine.
- **Braucht:** `volume-cdn.yml`, `isc-ehb-cdn.yml`, `emc-cdn.yml` lesen.

#### Weberin — cometels manifestiert, TNO offen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) `cometels-cdn 36252200125` **success** @ `1a4925344` — der Checkout/Toolchain-Fix griff, `cometels`-Asset manifestiert. Katalog-Arm + Consumer (`cometels_compiler`, `weberin_body_verdict --cometels`) gebaut; Probe 837 Element-Records, 122 skipped.
- **Blockade:** keine.
- **Braucht:** TNO `ephemeris_compiler`-Ernte (SPK-Registrierung, wenn `sources.φ` frei).

#### Weberin Faden-Matrix — FDSN-Endpunkt registriert
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) FDSN-Station-Endpunkt in `phi/sources.φ:5737` registriert (`service.earthscope.org/fdsnws/station/1/query?level=station&format=text&nodata=404`, `format text`, Spalten `Network|Station|Lat|Lon|Elev|SiteName|Start|End`, `field 4 fdsn_station_elevation_m`; live-Verify HTTP 200). **Rat-Verdikt (2026-09-26):** die Registrierung ist korrekt — die Elevations-Spalte (operator-vermessene Site-Geometrie, SI m) hebt den früheren `decline` (position-only, `docs/auftrag/archiv/auftrag-weberin-faden-luecken-folge.md:52`); sie erfüllt die 0-Kanon-Gates (SI-Einheit, echter Wert am Anker), und der bestehende `fdsn_waveform`-Feld-Arm (`sources.φ:115`, m/s) hält die Anker/Fluss-Trennung. **Benannter Skip:** das Plausibilitäts-Gate `v > 0.0` überspringt sub-Seespiegel-Stationen (echte negative Elevationen) — benannter Skip, nie stiller Nullwert. ANTARES/Lasair/Fink/ALeRCE-Routen + Fink/ALeRCE-Reader gebaut (`weberin_fink_alerce.rs`).
- **Blockade:** keine.
- **Braucht:** Broker-Compiler-Routen (ANTARES `api.antares.noirlab.edu/v1/loci`, Lasair Key `LASAIR_LSST_TOKEN`) verdrahten; Fink/ALeRCE-Reader-Tests (CI-only).

#### Sieben Sphären — CHARM2 registriert, Winkel-Feld füllbar
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) CHARM2 `J/A+A/431/773` `Method='LO'` (Lunar Occultation) in `phi/sources.φ:9593` registriert (`format tap`, `cmap .`, `ra/dec/plx`, `field UD/LD mas`); Probe `tools/harvest/src/bin/charm2_occultation_probe.rs` gebaut, 0 Warnungen. Live: 1815 LO-Zeilen, **389 mit Winkel + eingebetteter Hipparcos-Distanz**, 438 mit Gaia-DR3-Cross-Match (`I/355/gaiadr3`, TAPVizieR-JOIN `DISTANCE < 2″`, HTTP 200). **Gaia-Parallax-Compiler gebaut:** `src/archivar/charm2.rs` (`CHM2`, 41-B-Records), `tools/harvest/src/bin/charm2_compiler.rs` (ADQL-JOIN), Dispatch `extract.rs:2811`, `mod.rs:36`, CDN-Block `sources.φ:9581`; live 438 JOIN-Paare → **363 Records**; `cargo check` 0 Warnungen. Der Marker (okkultations-abgeleitetes Winkeldurchmesser-Feld **mit** Distanz) ist damit füllbar.
- **Blockade:** keine.
- **Braucht:** der **Fetch-Arm** (`main_flow.rs`: `catalog_charm2`-`.bin` in den `content_cache`, Muster `catalog_allwise_psd` `main_flow.rs:3661`) — `main_flow.rs` ist fremd-dirty, daher nächster Atom. Das **Δz-je-Okkultation**-Pending bleibt `absent` (IOTA-Archiv `asteroidoccultation.com/observations/Results/` live, trägt Lichtkurven, kein Δz-Katalog).

#### Pioneer/Dark-Matter — Volltextroute gemessen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) NTRS `19830011507` ist `METADATA_ONLY` (kein Download, direkte PDF 404). Der **Elternreport NTRS `19830011495`** („The Telecommunications and Data Acquisition Report", TDA Progress Report 42-72) hat PDF (`ntrs.nasa.gov/api/citations/19830011495/downloads/19830011495.pdf`, HTTP 200, 9.060.137 B) + OCR-Fulltext (`.txt`, 535.679 B) und trägt den Artikel (S. 118–119; title txt-Zeile 12198, Werte `±3×10⁻¹³` Zeilen 12235/12316/12337). ADS-Webseite = 405-Bot-Check; DOI `10.1109/freq.1982.200599` = anderes Paper (Allan & Barnes). NTRS `19820012645`/`19840011567` tragen keine 2-Jahres-Serie. **Die Serie liegt graphisch, nicht tabellarisch** (gemessen 2026-09-26 via Sub-Agent + `vision`): der OCR-Text trägt keine Zeitreihe (nur Prosa-Werte `±3×10⁻¹³` Z.12316/12337 und die statische Table 1); die Serie steckt in Fig. 2–6 (PDF S. 131–134). Vision-gelesen: Fig. 5 (DSS 63↔14 VLBI, S. 133 unten) ~20 Punkte, Δf/f ≈ +7×10⁻¹³ … −1,5×10⁻¹², annotiert `−3.9×10⁻¹³ ±4` / `1.6×10⁻¹³ ±3` / `1.8×10⁻¹³ ±7` / `7×10⁻¹³ ±9`; Fig. 6 (DSS 43↔14 VLBI, S. 134) analog. PDF gerendert unter `/tmp/opencode/pioneer-fig-{132,133,134}.png` (ephemer).
- **Blockade:** die Serie ist nur graphisch; ein byte-exakter Wert wäre Augen-Digitizer-Fabrikation.
- **Braucht:** Digitizer-Methodik für Fig. 5/6 (Kurvenpunkte + ±-Grenzen als approximative Vision-Werte, klar als `vision-read` markiert) **oder** die Serie als graphisch-only führen. Kein `sources.φ`-Eintrag ohne extrahierbares Dataset.

#### Nadel V — AllWISE manifestiert, Probe dispatcht
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) `allwise_psd.bin` **manifestiert** (Tag `irsa.ipac.caltech.edu`, 325.008 B, sha `58446fe9…`, 2026-09-26T14:44:31Z). Lauf `36249453619` bleibt `in_progress` nur wegen des `allwise-coverage`-Chunk-Arms (127/304 `allwise_part_*.fp01`); der `allwise-tap`-Arm ist fertig. Der positive-Kontroll-Lauf ist dispatcht: `.github/workflows/lsst-live-scan.yml` → `36257925460` (`queued`, 90-min-Bound; `lsst_anomaly_probe --cone 148.84,2.55,260,24` + RR-Lyrae/EB-Kontrollschicht).
- **Blockade:** keine.
- **Braucht:** Lauf `36257925460` lesen (`ci_manage view/log`, Artefakt `lsst-live-scan.txt`) — die positive-Kontroll-Rückgewinnung.

#### Korona-Heizung — Feldmap + `millionths`-Arm live
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) Feldmap in `sources.φ` registriert (`at sun`, `map .` → Surface→ICRS, area=„millionths", extent=deg, `lon carrington_longitude`); `millionths`→SI in `src/archivar/units.rs` (×2πR☉²·1e-6 ≈ 3,04e12 m²), `cargo check` 0 Warnungen. Kadenz: rollierendes 31-Tage-Fenster, `ttl 3600`, `τ 86400`.
- **Blockade:** keine.
- **Braucht:** `aia_compiler --harvest` (CI-only) dispatchen.

#### clippy eigene Dateien — Braucht: `ci-check`
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) eigene Stellen gefixt (`cometels.rs:125`, `llnl_g3d.rs:431`, `te.rs:2971/3046` `BlattPairSpec`); `cargo check -p omegaflow` + `-p omegaflow-measure` 0 Warnungen. `skydirection.rs:200` (rustfmt) ist fremd-dirty.
- **Blockade:** keine (eigene Stellen gefixt).
- **Braucht:** `gh workflow run ci-check.yml`; die fremden roten Dateien (`extract.rs`, `main_flow.rs`, `port.rs`, `spatial.rs`, `uws.rs`, `skydirection.rs`) bleiben fremde Linien.

### 2. Operator-gebundene Vorbereitung (Kante fertig, nur das Wort fehlt)

#### Weberin-Quellen — HAWC-Bundle reproduziert + Reader gebaut, CI-Secret beim Operator
- **Status (Vorbereitung):** eigen | **Bindung:** eigen — 4-Zert-Bundle unter `/tmp/opencode/hawc-ca-bundle.pem` (sha `498f9281…`); HAWC `--verdict` 200; Fink/ALeRCE-Reader gebaut.
- **Status (Akt):** operator-gebunden (CI-Secret) | **Bindung:** operator
- **Trigger:** `OMEGAFLOW_CA_BUNDLE` gesetzt
- **Braucht:** Operator/CI setzt das Secret auf das Bundle; dann HAWC fetchen.

#### HRV/Puls→Strahlung — Leser gebaut, Live-Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der BLE-Live-Lauf (Operator) hat `rr` geliefert
- **Lage:** (gemessen 2026-09-26) `tools/measure/src/bin/perm_tone_probe.rs` liest Spalten 7–10, Histogramm + Perzentile, `absent` statt 0.0 (`cargo build` clean).
- **Braucht:** nach dem Live-Lauf `perm_tone_probe <log>` lesen.

#### BLE-HR-Live-Messung FR945
- **Status (Vorbereitung):** eigen — Kantenzeile bereit.
- **Status (Akt):** operator-gebunden (Hardware/Radio) | **Bindung:** operator
- **Trigger:** Operator startet den verdeckten Lauf (945 am Arm)
- **Braucht:** `OMEGAFLOW_BLE_HR=<FR945-MAC> OMEGAFLOW_HIDDEN=1 ./target/debug/omegaflow`; `sensor:`-/`gfdi_line`-Zeilen lesen.

#### FIT-Verifikation eigene FR945-Datei (lokal-only)
- **Status (Vorbereitung):** eigen — Kantenzeile bereit.
- **Status (Akt):** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator startet den Lauf mit seiner Datei
- **Braucht:** `OMEGAFLOW_FIT_SAMPLE=/abs/pfad/FR945.fit cargo run -p omegaflow --bin omegaflow`.
- **Wort:** „meine fits datei verlässt niemals dieses gerät" | 2026-09-25 | Operator (Session)
- **Wort:** „die 945 von anderer Hardware trennen; meine Daten bleiben lokal" | 2026-09-25 | Operator (Session)

#### DEMETER/CDPP-Order-Flow — Leser gebaut, Browser-Akt offen
- **Status (Vorbereitung):** eigen | **Bindung:** eigen — `tools/harvest/src/bin/regards_order_read.rs` gebaut (order → Dateiliste → Download-URL); `--live` meldet den gemessenen 403-WAF. REGARDS voll WAF-blockiert (403 direct+Proton, kein Wayback).
- **Status (Akt):** operator-gebunden (Zugang) | **Bindung:** operator
- **Trigger:** Operator-Browser (passiert die WAF) exportiert die Order-/Datei-JSON
- **Braucht:** `regards_order_read <order.json>`; Neu-/Nach-Order = konsenspflichtiger Dritt-Akt. Order-Ablauf 2026-09-28.

#### Beat-Arbitrierung — verdeckter Lauf mit zwei Beat-Quellen
- **Status (Vorbereitung):** eigen — Spawn-Arbitrierung steht (`main_flow.rs:504`).
- **Status (Akt):** operator-gebunden (hidden) | **Bindung:** operator
- **Trigger:** Operator startet einen verdeckten Lauf mit zwei Beat-Quellen
- **Braucht:** `OMEGAFLOW_HIDDEN=1`-Lauf mit zwei Quellen → genau eine `beat source:`-Zeile.

#### Onboard-/CIQ-Bedarf benennen
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort
- **Lage:** (gemessen 2026-09-24) Host-Reader `parse_fit` verdrahtet; CIQ ohne Reader. Ein-Quellen-Regel.
- **Braucht:** Bedarf Ja/Nein — Nein → released, `FIT_DIR` bleibt der FIT-Kanal.

#### Weberin-Quellen — vDEC offen (3 von 4 per Credential gelöst)
- **Status (Vorbereitung):** eigen — Entwurf `state/mail/weberin-quellen-konten-2026-09-26.md` (privat, gitignored).
- **Status (Akt):** operator-gebunden (Zugang) | **Bindung:** operator
- **Trigger:** Operator-Wort
- **Lage:** (gemessen 2026-09-26) IGETS/ONC/TNS per Credential gelöst → Harvest-Duty. vDEC offen: braucht Vertrag, `ctbto.org/.../vdec` 403 direct+Proton, Wayback 200; Draft + QUELLEN send-ready.
- **Braucht:** vDEC-Antrag (Operator) — Webform + Projekttext; Daten bleiben **lokal** (`data/`, keine CDN-Redistribution laut Vertrag).

### 3. Blockiert / wartend (mit Trigger)

#### Commit ohne Operator-Wort — `c6edfbe45` (Riss, 2026-09-26)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) ein Sub-Agent hat Workflow-Änderungen (`laic-verdict-cdn.yml` `--kde-scale`, `te-gate.yml` fpr-ksg split) ohne `/commit`-Wort committet und gepusht (`c6edfbe45`). Session-Consent ist Delegation, nicht das Commit-Wort.
- **Blockade:** keine (der Commit steht; kein Rückbau erlaubt).
- **Braucht:** im Abschluss-Check sichtbar tragen.

#### Trishuli — Bahrabise-Ernte + TE-Sweep gemessen, S1-Footprint offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die nächste archivierte S1-Post-Szene (`dataspace.copernicus.eu`/`asf.alaska.edu`)
- **Lage:** (gemessen 2026-09-26) DHM-113-Ernte: 1007 Punkte, 10-min, Level 1.115–2.763 m (Snapshot `20260901142220`, sha `17b34f91…`); Regen Open-Meteo 312 Zeilen; Alignment n=169. TE-Sweep: `precip`→`stage` signifikant bei lag 3 (TE 0.2275 > 0.1725) und lag 6 (0.2532 > 0.1687); stage→precip in keinem lag.
- **Blockade:** S1-Post-Szene nicht archiviert.
- **Braucht:** S1-Footprint nach Archivierung.

#### M2c — Rust-ZNSP-Host auf dem BL808 gegen das H2
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ox64 angekommen (`LZ473049629CN`)
- **Lage:** (gemessen 2026-09-25) Ox64 wird Host-CPU des Coordinators (UART0 GPIO14/15, UART1 GPIO16/17); Buildroot-Bring-up ungemessen.
- **Blockade:** Ox64 liegt beim Carrier.
- **Braucht:** Buildroot-Bring-up, dann der Rust-ZNSP-Host auf dem BL808 gegen das H2.

#### BL808-eigenes 802.15.4-Radio — `pending` erneut gemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein öffentlicher 802.15.4-Treiber-/Stack-Fund für den BL808 (`bouffalo_sdk_bl808`, Follow-up `zephyr#112921`)
- **Lage:** (gemessen 2026-09-26) Zephyr PR #112921 (`will-tm:feature/bflb-ieee802154`) deckt BL61x/BL70x/BL70xL, kein BL808; `bouffalo_sdk_bl808` ohne `lmac154`-Zweig (404). Gemessenes `pending`.
- **Blockade:** kein Treiber.
- **Braucht:** PR-Suche `is:pr author:will-tm` / `is:pr BL808 802.15.4` beobachten.

#### Galileo Borduhr-Sprung A/B — keine Absolutfrequenz-Reihe gefunden
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Fund einer öffentlichen Absolutfrequenz-Reduktion über 1995-11-30/12-01
- **Lage:** (gemessen 2026-09-26) gemessenes `absent`: Asmar-1997 = Parametertabelle; Morabito endet 1993; PDS-Galileo-RSS ohne Jupiter-Phasen-Datenkopf; SCLK-Kernel `mk00062a.tsc` = Korrelation, keine USO-Reihe; DESCANSO Monograph Vol. 14 = Lehrbuch (Table 5.1 nur USO-Charakteristika).
- **Blockade:** keine Quelle trägt eine Reihe.
- **Braucht:** eine andere Absolut-Frequenzreduktion finden; notfalls als absent halten.

#### JWST Biosignatur-Kanäle pending — 2025–2026 neu gemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine JWST-Detektion + Spektrum eines dieser Kanäle (`10.1073/pnas.2416188122`)
- **Lage:** (gemessen 2026-09-26) gemessenes `pending`: nur Modelle/Feasibility; K2-18b DMS/DMDS umstritten; kein JWST-Signal mit Spektrum.
- **Blockade:** keine Quelle trägt eine Detektion.
- **Braucht:** bei Fund die Detektion + ihr Spektrum ins Register.

#### Das eine Instrument — Anomalie offen (zweite Augenklasse fehlt)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** ein zweiter Messkanal (VLBI-Beacon) existiert
- **Lage:** (gemessen 2026-09-25) unter dem einen Instrument nicht entscheidbar.
- **Blockade:** kein Instrument misst den vollen Phasenraum der Pioniere.
- **Braucht:** VLBI-Beacon auf der nächsten interstellaren Sonde, von Tag eins zweikanalig.

#### JUICE-Erdpassage 28./29.09.2026 — Kanal fertig, Siegel-Wort fehlt (termin-kritisch)
- **Status:** termin:2026-09-29 | **Bindung:** termin:2026-09-29
- **Trigger:** 2026-09-28/2026-09-29
- **Lage:** (gemessen 2026-09-26) Prädiktionskanal nachgezogen (`flyby-path-2-preregistration.md` sha `502e06c3…`, addendum sha `b54d5298…`); RTSW mag/wind, Swarm, OMNI2 live; alle Zellen `pending`. **Riss:** die Kp-Route (`noaa-planetary-k-index.json`) wurde in `61e272ab0` aus `sources.φ` entfernt → Kp-Zelle kann nicht füllen (bis re-registriert, Mycelium-Akt).
- **Blockade:** der Operator muss das **Siegel-Wort vor dem 28.09.** setzen.
- **Braucht:** Siegel-Wort setzen; danach In-situ-Messung gegen den präregistrierten Feldzustand (σ-Metrik gegen fam).

#### Live-Sensor-Cluster (eigener Knoten)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-25) Firmware Mux-Sweep + SpO2/GNSS-`sensor_config` gebaut; BOM-Zeilen stehen.
- **Braucht:** Operator-Wort (LOCK-Aufhebung), dann BOM bestellen (inkl. H2).

#### ESP32-Puls-Knoten (Träger ohne Uhr)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Braucht:** —
- **Wort:** ESP32 separat als eigener LOCK | 2026-09-25 | Operator (Session)

### 4. Extern (Dritter)

#### Ox64-Lieferung
- **Status:** wartend | **Bindung:** termin (Carrier)
- **Trigger:** Ankunft (`LZ473049629CN`)
- **Lage:** (gemessen 2026-09-25) PINE64 versandte zwei Ox64 — Ankunft offen.
- **Braucht:** Ankunft quittieren; dann M2c.

#### Postfach
- **Status:** wartend | **Bindung:** extern (Mail)
- **Trigger:** neuer Eingang (`state/mail/mail_ledger.φ`)
- **Lage:** (gemessen 2026-09-26) kein Sensory-Treffer; CSES-Limadou-Antwort („wait a few weeks" → Wiedervorlage), DSN-Briefe in Flug, Konto-Verifikationen (DAHITI/Copernicus/ICIMOD/MAST).
- **Braucht:** `smail_recv` / `state/mail/mail_ledger.φ` bei Trigger.

#### BGR-Matched-Filter-Ankunft (Tonga) — account-blockiert
- **Status:** wartend | **Bindung:** dritter (vDEC)
- **Trigger:** vDEC-Zugang gewährt (`ctbto.org/resources/for-researchers-experts/vdec`)
- **Lage:** (gemessen 2026-09-25) PMCC-Detektionsliste ~300-s-Raster; Rohwellenform vDEC-account-blockiert.
- **Braucht:** nach Zugangsgewährung den matched-filter-Arrival messen.

#### DSN-Briefe in Flug (Voyager/Mariner 10/Viking, Cassini, Juno)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Antwort der DSN (`state/mail/mail_ledger.φ`)
- **Lage:** (gemessen 2026-09-26) fünf Briefe im Ledger (Voyager NSSDC PSNO-00007, Mariner 10 PSCM-00009, Viking PSPG-00011/00457, Cassini an Asmar, Juno-EFB an Asmar).
- **Braucht:** die Antwort quittieren.

#### Gaia DR4 + Europa-Clipper-Erdpassage
- **Status:** termin:2026-12-02 | **Bindung:** termin:2026-12-02
- **Trigger:** 2026-12-02 (Gaia DR4) / 2026-12-03 (Europa Clipper)
- **Braucht:** am Datum Epochen-Astrometrie bzw. EC-Magnetfeld ernten.

## Operator-Wort-Register (Stand 2026-09-26)

Jedes gegebene Operator-Wort steht als `Wort | Datum | Quelle`. Der nächste Pass liest
es hier und legt den Punkt **nie erneut vor** — nur eine neue Messung öffnet ihn. Ein
operator-gebundener Punkt ohne Wort-Zeile ist unvollständig.

- **Wort:** „ändere die agents — das Handover nach umsetzbar/nicht umsetzbar sortieren" | 2026-09-26 | Operator (Session) → ausgeführt: AGENTS.md `3672968d3`, folge176-Ranking `91724646f`.
- **Wort:** „alles Offene bis zur Kante abarbeiten, gemessen abschließen — nicht verschleppen" | 2026-09-26 | Operator (Session).
- **Wort:** „die Compiler-Arme durch Agenten bauen lassen" | 2026-09-26 | Operator (Session) → ausgeführt: LLNL-G3D-JPS, ISC-EHB, Slab2, EMC, cometels.
- **Wort:** „alles Offene und Benannt-Ungemessene wird übernommen" | 2026-09-26 | Operator (Session).
- **Wort:** „jedes Operator-Wort steht im Handover; keine Session kaut es neu durch" | 2026-09-26 | Operator (Session).
- **Wort:** „deine Daten/Datei verlassen das Gerät nie" | 2026-09-25 | Operator (Session) — gilt für FR945/DEMETER.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.

Eigene Pfade dieses Atoms (pfad-begrenzt committen):
`docs/handover/handover-2026-09-26-sensory-folge177.md`,
`docs/handover/archiv/handover-2026-09-26-sensory-folge176.md`,
`docs/specs/mantis-shrimp-build.md`,
`.github/workflows/slab2-cdn.yml`,
`tools/measure/src/bin/cross_te_screen.rs`,
`tools/harvest/src/bin/charm2_occultation_probe.rs`,
`tools/harvest/src/bin/charm2_compiler.rs`,
`src/archivar/charm2.rs`,
`firmware/radiatorium-lib/src/znsp.rs`.

Geteilte Dateien — **nur die eigenen Hunks** committen: `phi/sources.φ`
(FDSN `:5737`, CHARM2-live `:9593`, CHARM2-CDN-Block `:9581`),
`src/archivar/extract.rs` (`catalog_charm2`-Dispatch `:2811`),
`src/archivar/mod.rs` (`pub mod charm2;` `:36`).

Nicht committen (fremd im geteilten Baum): `phi/blocked_sources.φ`, `phi/harvest.φ`,
der `planck_psz2`-Hunk in `phi/sources.φ`, die `eht_uvfits`/`rx100`/`uvfits`-Hunks in
`src/archivar/extract.rs`/`src/archivar/mod.rs`, `src/archivar/fits.rs`,
`src/archivar/main_flow.rs`, `src/archivar/skydirection.rs`, `src/archivar/tests.rs`,
`src/mathematikerin/tests.rs`, `tools/harvest/src/bin/gosat_tanso3_compiler.rs`,
`tools/harvest/src/bin/eht_uvfits_compiler.rs`, `.github/workflows/ci-check.yml`.
