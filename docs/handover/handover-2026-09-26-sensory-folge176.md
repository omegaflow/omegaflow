<!--
  title: Handover — Sensory-Folge 176 (Stand 2026-09-26)
  session: Sensory-Folge 176
  class: handover
  date: 2026-09-26
  sha256: f28ddd730ed65dcebc897039e4594662533d51b50f4688c4c247aa10748ed49f
  status: live
-->
# Handover — Sensory-Folge 176 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes ist gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Nur eigene Arbeit: bei geteilten Dateien nur
die eigenen Hunks — committet wird pfad-begrenzt, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main` Vorfahr
von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage** (mit
Messstempel) / **Blockade** / **Braucht**. Sortierung: erst logisch nach Akteur
(Linie | Rat | Operator | Dritter), dann chronologisch (Messdatum). Operator-gebundene
Punkte sind in Vorbereitung (autonom) und Akt (operator-gebunden) getrennt.

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

### Letzte Messung (Folge 176, 2026-09-26)

- **Postfach:** kein Sensory-Treffer; CSES-Limadou (Sotgiu, „wait a few weeks" →
  Wiedervorlage), DSN-Briefe im Ledger, Forum/Alerts. Ledger
  `state/mail/mail_ledger.φ`.
- **CI am HEAD `85fcd3cf2`:** `zigbee-host` 36235538625 success (sizeof leer →
  Fix, s. ZNSP); `meteo-cdn` alle vier success (tibet/aaretal/japan/bordeaux,
  s. Kreuz-Screening); `te-gate` 36228804363 in_progress (`fpr-ksg-arx`, 2 h+).
- **Arbeitsbaum:** fremde uncommittete Arbeit (Mycelium/Mountain an
  `phi/sources.φ`, `phi/blocked_sources.φ`, `src/archivar/skydirection.rs`,
  `kbo_residue_probe.rs`, `rixs_cuprate_probe.rs`, `suprastrom_form_probe.rs`,
  River-Handover, `gll-rss-rsr-cdn.yml`, `survey-2026-09-26-membran-ladearchitektur.md`)
  — **nicht berührt, nicht committet**.

## Träger-Zeilen (Orphan-Faltung)

Die folgenden Prosa-Dokumente tragen offene Marker und werden hiermit an ihre
Trägerpunkte gebunden (der Scanner `register_lookup --orphan-docs` erkennt den
Träger nur, wenn eine lebende Übergabe den Dateinamen nennt):

- `survey-2026-09-07-weberin-sonnensystem-kette.md`, `survey-2026-09-07-weberin-thread-matrix.md`,
  `survey-2026-09-13-weberin-quellen.md`, `survey-2026-09-13-weberin-quellen-folge.md`,
  `survey-2026-09-13-weberin-quellen-treffer.md`, `survey-2026-09-14-weberin-quellen-rerun.md`
  → Träger `docs/concepts/die-weberin.md` (Weberin-Punkte unten).

### Rest-Träger-Zeilen (Stand 2026-09-26)

Je Zeile ein zuletzt trägerloses Dokument: `Pfad` (offene Marker) → Trägerpunkt
oder descoped-Befund. Der Dateiname in dieser Übergabe ist der Träger
(`register_lookup --orphan-docs`).

- `docs/blatt/blatt-der-grat.md` (2) → ENSO-Pfeil messen: `cross_te_screen`.
- `docs/blatt/blatt-kreuz-screening-gyirong.md` (3) → Träger Punkt „Kreuz-Screening auf weitere Ereignisse" (unten).
- `docs/blatt/blatt-solar-seconds-matrix.md` (1) → `corona_conditional_probe` auf das Paar 211A→193A.
- `docs/blatt/blatt-thuan-fragesteller.md` (4) → `termin:2026-12-02` (Gaia DR4), dann `docs/auftrag/archiv/auftrag-gaia-dr4-iapetus.md`.
- `docs/concepts/arxiv-api.md` (2) → Träger Punkt „arxiv HTTP 406" (Mountain-Linie).
- `docs/concepts/blatt-papier-resultat.md` (1) → Blatt-1-Bojen-Matrix-Rotor laufen lassen, Matrix-Zeile Σ p̂·M nachtragen (Z.63–71).
- `docs/concepts/das-eine-instrument.md` (2) → Träger Punkt „Das eine Instrument — Anomalie offen" (unten).
- `docs/concepts/der-paradigmenwechsel.md` (9) → Träger Punkt „JUICE-Erdpassage 28./29.09.2026" (unten).
- `docs/concepts/die-akteure-im-boden-und-wasser.md` (7) → Träger Punkt „Seismik-Flotte" (unten).
- `docs/concepts/exzellenz-konzept.md` (3) → descoped (gemessen 2026-09-26: Marker sind die Definition von pending §2/§2.5; Body final → `docs/concepts/exzellenz-konzept.md:91`).
- `docs/concepts/fuenf-funken-anomalie-suche.md` (4) → Funke 3 via `broker_difference_probe`, Funke 5 (TDB-Koinzidenz-Fenster) bauen.
- `docs/concepts/kybernetische-astrophysik.md` (10) → descoped (gemessen 2026-09-26: Marker sind Status-Vokabular „offen"/„pending" im Essay-Body → `docs/concepts/kybernetische-astrophysik.md:45`).
- `docs/concepts/recherche-galileo-kadenz-reconciliation.md` (1) → `archive_search --ntrs 19930010224` bzw. DSMS Services Catalog v7.5 §"Doppler count interval".
- `docs/concepts/the-seven-spheres.md` (2) → Träger Punkt „Sieben Sphären" (unten): die zwei `pending`-Marker (Stern-Winkeldurchmesser-Feld; gemessenes Δz je Okkultation).
- `docs/concepts/zeugnis.md` (5) → descoped (gemessen 2026-09-26: Marker sind die Disziplin-Prosa des Handover-Abschnitts F → `docs/concepts/zeugnis.md:370`).
- `docs/paper/asmar-2005-spacecraft-doppler-tracking-noise-budget.md` (2) → descoped (gemessen 2026-09-26: Treffer sind der Substring „depending" Z.29/35, kein offener Punkt).
- `docs/paper/causal-arrow-preregistration.md` (1) → `te_pair_probe` (Lag-Sweep {1,3,6,12,24,48}) gegen Rasuwa-Regen; DAHITI Koshi via `sfetch` (api_key).
- `docs/paper/corona-heating-ladder.md` (2) → Träger Punkt „Korona-Heizung" (unten): `aia_ladder_probe` über den vollen 613-Event-Korpus.
- `docs/paper/cross-screening-tibet.md` (1) → Träger Punkt „Kreuz-Screening auf weitere Ereignisse" (unten).
- `docs/paper/depth-phase-echo-fleet.md` (5) → Träger Punkt „Seismik-Flotte" (unten).
- `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4) → Träger Punkt „JUICE-Erdpassage 28./29.09.2026" (unten).
- `docs/paper/flyby-path-2-preregistration.md` (1) → Träger Punkt „JUICE-Erdpassage 28./29.09.2026" (unten).
- `docs/paper/galileo-rotor-spin-era-floor.md` (1) → CK-Kerne jenseits `ck90341a`–`ck90344b` (Frame −77000) von `naif.jpl.nasa.gov` harvesten.
- `docs/paper/jwst-disequilibrium-survey.md` (7) → Träger Punkt „JWST Biosignatur-Kanäle pending" (unten).
- `docs/paper/laic-arrow-direction.md` (3) → `archive_search --playwright https://leos.ac.cn` (CSES SPA).
- `docs/paper/nadel-v-fresh-area-dip-scan.md` (1) → Träger Punkt „Nadel V" (unten): `lsst_anomaly_probe` auf dem positiven Kontrollkegel.
- `docs/paper/planet-nine-kbo-residue.md` (1) → descoped (gemessen 2026-09-26: §5(iv) auf „implementiert + getestet" nachgezogen, `spk_type1_check.rs`; der Reader ist gebaut).
- `docs/paper/probe-front-dark-matter.md` (2) → Träger Punkt „Pioneer/Dark-Matter" (unten): Syntonisation-1983/GPS-1982–87 via `--ads`/`--ntrs`.
- `docs/paper/solar-seconds-matrix.md` (3) → `corona_conditional_probe` auf das Paar 211A→193A.
- `docs/paper/sturzflut-tibet-pfeil.md` (23) → Träger Punkt „Trishuli" (unten).
- `docs/paper/tonga-lamb-crosscheck.md` (3) → Träger Punkt „BGR-Matched-Filter-Ankunft (Tonga)" (unten).
- `docs/surveys/axiom-gate-broken-null-control.md` (1) → Träger Punkt „Broken-Null-Control" (unten).
- `docs/surveys/axiom-gate-depth-phase-echo-fleet.md` (1) → Träger Punkt „Seismik-Flotte" (unten).
- `docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md` (17) → Träger Punkte „Pioneer/Dark-Matter" + „DSN-Briefe in Flug" (unten): vier Werkzeuglücken.
- `docs/surveys/survey-2026-09-16-sonden-flotte.md` (4) → Träger Punkt „DSN-Briefe in Flug" (unten).
- `docs/surveys/survey-ein-blatt-korona-heizung.md` (1) → Träger Punkt „Korona-Heizung" (unten).
- `docs/surveys/survey-messpunkt-verteilung.md` (6) → descoped (gemessen 2026-09-26: Kandidaten 1–8 verdiktet; Kandidat 9/§6 Konsultations-Einladung).
- `docs/concepts/ein-blatt-papier.md` (2) → Lag-Sweep-Träger Punkt „causal-arrow-preregistration" (`te_pair_probe`); KDE-Bandbreiten-Gate descoped.
- `docs/concepts/blatt-papier-beweis.md` (3) → descoped (Membran-Bindung gebaut `src/mathematikerin/omega.rs:349`, Commit `356fa616`; pending-Kanalzellen = Quellen-Port-Stand).
- `docs/paper/terminologie-der-gegenstroemung.md` (1) → descoped (Marker „pending" im Definitions-Kopf → `:22`).
- `docs/surveys/survey-2026-09-06-codestruktur.md` (8) → Träger Stehender Pass „CI-Status am HEAD" (oben).
- `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md` (2) → descoped (Substring „wartet" in „erwartete"; Silence-Map-Probe gebaut).
- `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` (9) → Träger Punkt „HRV/Puls→Strahlung" (unten, wartend).
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` (2) → Träger River-/Browser-Linie (Kaltstart-Entschärfung, Versionslücke).
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` (7) → Träger „FIT-Verifikation FR945" (operator) + „BLE-HR-Live-Messung FR945" (unten).
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` (1) → Träger Punkt „Postfach" (unten): nächster Schritt MPI-FKF/TRISP-Antwort via `smail`; die CSES-Limadou-Antwort (Sotgiu 2026-09-16, „wait a few weeks") ist Wiedervorlage.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md`, `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md`, `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md`, `docs/concepts/mirror-research.md` (je 1) → Bestand der Mycelium-Linie (kein eigener Sensory-Schritt).

- **Inline-Code-Klasse — geschlossen:** `strip_inline_code` (`tools/register/src/bin/register_lookup.rs:92`) trägt keine offenen Marker mehr in `die-weberin.md`/`docs-naming.md`/`kybernaut-native-methodology.md`/`the-counter-slope.md`.

- **Alt-Orphans (2026-09-26 gemessen, trägerlos):** `docs/concepts/pfeiler-der-architektur.md` (2), `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` (3), `docs/surveys/survey-fortschritt.md` (1) — Prosadokumente, Marker = Status-Vokabular; kein eigener Sensory-Schritt. Schritt: beim nächsten Pass je Datei die Marker lesen → descope oder Träger (Aufenthalt Mycelium/Mountain).

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### Commit ohne Operator-Wort — `c6edfbe45` (Riss, 2026-09-26)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) ein Sub-Agent hat Workflow-Änderungen (`laic-verdict-cdn.yml` `--kde-scale`, `te-gate.yml` fpr-ksg split) ohne `/commit`-Wort committet und gepusht (`c6edfbe45`). Session-Consent ist Delegation, nicht das Commit-Wort (AGENTS.md).
- **Blockade:** keine (der Commit steht; kein Rückbau erlaubt).
- **Braucht:** im Abschluss-Check sichtbar tragen.

#### Kreuz-Screening — 4/4 gelesen, Screen misst den Diurnal-Zyklus
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) alle vier `meteo-cdn`-Läufe success (tibet `36234656173`, aaretal `36234657686`, japan `36234658873`, bordeaux `36234660015`); `cross_te_screen`-Logs gelesen. **Befund:** die stärksten Paare zielen durchweg auf `is_day` — tibet `rasuwa_cloud_cover_low`→`is_day` 0.0184; aaretal `beatenberg_leaf_wetness`→`is_day` 0.0093; japan `kumamoto_pressure_msl`→`is_day` 0.0090; bordeaux `landiras_leaf_wetness`→`is_day` 0.0082 — plus gleichvariable Raumkopplung (`weather_code`→`weather_code`). Kein ereignisspezifisches Signal.
- **Blockade:** keine.
- **Braucht:** Re-Run mit ereignisspezifischer Zielgröße ohne `is_day`.

#### Seismik-Flotte — 3D-Modell-Kandidat + Stationsterm gemessen, Registrierung offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `research-max`) W-Phase-M9 gebaut/verdrahtet (`w_phase_bandpass`/`w_phase_energy_ratio`/`w_phase_discriminate`, `tools/measure/src/depthphase.rs`); Flotte unverzerrt (+1,7 km, se 4,7 km), 19/36 km dominieren das ±10-km-Gate; ak135 1D (`positive-maske.md:64`). 3D-Kandidat **LLNL-G3D-JPS** (gemessen 2026-09-26 via `general`: Downloads `gs.llnl.gov/sites/gs/files/2021-09/llnl_g3d_jps.interpolated.zip` 200, 47,17 MB, sha `3bb04377…`; `LLNL-G3D-JPS.e3d.binary` 200, 146,8 MB; `llnl-g3d-jps_tomofilt_1.zip` 200, 166,1 MB; `LLNL-Earth3D.5.4.3.jar` 200, 76,8 MB; **Lizenz gemessen absent**). Stationsterm **ISC-EHB**: Bulk ist **RES/HDF-gz**, nicht CSV — `http://download.isc.ac.uk/isc-ehb/` dir-listing, `1964.res.gz` 200, 10,07 MB; CSV nur per Query (`isc.ac.uk/isc-ehb/search/arrivals/csvoutput/` 200, PHP-Form); Lizenz nur „cite", Nachbar ISC-GEM CC-BY-SA 3.0. IRIS/EarthScope-EMC gemessen: `data.earthscope.org/archive/seismology/products/emc/netcdf/` dir-listing 200 (117652 B), `GLAD-M35.r0.1-n4c.nc` 200, 343,76 MB (`\x89HDF`); EMC-Lizenz measured absent; die frühere 61297-B-Template-Angabe **nicht reproduziert** (`ds.earthscope.org` 000/404) → ungemessen.
- **Blockade:** `phi/sources.φ` fremd-dirty → keine Registrierung in diesem Atom.
- **Braucht:** LLNL-G3D-JPS + ISC-EHB als `sources.φ`-Ernte-Kandidaten registrieren (wenn `sources.φ` frei); pP-Residuum sonst `pending` halten.

#### Positive Maske — ODF-Regel + Slab2-Ersatzroute gemessen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `research-max`) Picker kanonisiert (`picker.rs`); Slab2 ScienceBase 403 direct+Proton `blocked`, **Wayback-Route trivial**: `web.archive.org/web/20250309001257if_/…sciencebase.gov/catalog/file/get/5aa1b00ee4b0b1c392e86467?f=__disk__…` (gemessen). Galileo-ODF-Trenner geklärt: **820-013/209G** (nicht 810-005), Separator = **Format-ID Bits 129–131**; 1988er-SIS text-bestätigt (`pds-ppi.igpp.ucla.edu/annex/GO-J-RSS-1-ODF-V1.0/DOCUMENT/TRK_2_18.TXT`, 60720 B), Format-2 golden-verifiziert (`odf.rs:52–100`), rev-G-Scan ohne Text-Layer (OCR offen).
- **Blockade:** `sources.φ`/`blocked_sources.φ` fremd-dirty.
- **Braucht:** Slab2-Ersatzroute + LLNL-Tomografie registrieren; ODF rev-G OCR (vision) oder die zwei Beine als abgeschlossen tragen.

#### Sieben Sphären — Feld + Aggregation gebaut, zwei Live-Quellen pending
- **Status:** offen | **Bindung:** eigen
- **Trigger:** eine Live-Quelle (Stern-Winkeldurchmesser / gemessenes Δz je Okkultation)
- **Lage:** (gemessen 2026-09-26 via `grind-max`) Sphäre I gebaut: `src/archivar/fresnel.rs` (`fresnel_line`, FRS1; θ=2a/D, F=a²/(λD), √(λD/2)); Gaia-Farbe→λ über `spectral.rs::bp_rp_to_lambda_nm` (Planck-SED, photon-gewichtet durch den eingebetteten Passband); Sphäre VII in `src/mathematikerin/doppler.rs` (`doppler_dz`=GM/(c²b), Kreuzprodukt-Impactparameter, `prediction_stat`/`residual_stat`, DGZ1). `cargo check`/`--tests` 0 Warnungen; 19 neue Tests (CI-only). Carrier `the-seven-spheres.md` trägt jetzt genau 2 `pending`.
- **Blockade:** kein Katalog trägt das Stern-Winkeldurchmesser-Feld; kein gemessenes Δz je Okkultation.
- **Braucht:** Okkultations-Lichtkurven-Katalog / gemessenes Δz je Okkultation als Quelle; dann die 2 pending-Marker füllen.

#### Korona-Heizung — Feldmap fertig, Port durch fremde Dirty-Datei blockiert
- **Status:** offen | **Bindung:** eigen
- **Trigger:** `sources.φ` frei
- **Lage:** (gemessen 2026-09-26 via `grind-pro`) Route lebt (`services.swpc.noaa.gov/json/solar_regions.json`, 139854 B heute, sha `008d3294…`); Feldmap-Block fertig in `/tmp/opencode/korona-register-block.φ` (units: area=millionths, extent=deg, `lon carrington_longitude`; `at sun` + `map .` → Surface→ICRS über WGCCRE; kein Compiler nötig — `extract.rs:3228–3357` löst das). Riss: `blocked_sources.φ` trägt den Eintrag heute als `descoped` (Z.244, uncommitted Batch-Bereinigung).
- **Blockade:** `phi/sources.φ`/`blocked_sources.φ`/`src/archivar/units.rs` fremd-dirty.
- **Braucht:** den Feldmap-Block nach `sources.φ` portieren; `millionths`→SI-Arm in `units.rs` (×2πR☉²·1e-6 ≈ 3,04e12 m²); `aia_compiler --harvest` (CI-only).

#### Trishuli — Bahrabise-Route entblockt, Ernte + S1 offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** Ernte-Lauf / Post-Sentinel-1-Szene
- **Lage:** (gemessen 2026-09-26 via `grind-pro`) Pegel-Route live: DHM Nepal `river-watch` (200, 4,9 MB) trägt Station **113 „Bhote Koshi at Bahrabise"** (27.7868/85.8993, series_id 1640); Flood-Fenster nur im **Wayback-Snapshot `20260901142220`** (2026-08-25…09-01, 1007 pts). Regen: Open-Meteo archive-api (200, keyless). `trishuli_gauge_probe.rs` parameterisiert (`--station-id/--station-name/--precip-name/--precip-coords`); `phi/meteo/tibet-flut-2026.json` trägt `bahrabise`; Doc-sha `03280d51…`.
- **Blockade:** S1-Post-Szene nicht archiviert.
- **Braucht:** Bahrabise-Ernte fahren (`livefeed_gate --dhm 113 …` → `trishuli_gauge_probe`/`te_pair_probe`); S1-Footprint nach Archivierung.

#### Weberin — cometels/mpcorb gebaut, Datenpfad + Consumer offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `grind-pro`) `src/weberin.rs` +284: `CometelsLine`/`CometelsRec::state_at` + `Weberin::weave_cometels` (8 Tests); TNO-Zweitlinie über `small_body_number`→`weberin_mpc_spk_verdict` (TNOs lesen `absent spk` bis SPK registriert). Routen: `cometels.json.gz` 200 (52939 B), `mpcorb_extended.json.gz` 200 (74,2 MB), CDN `cometels_flat.json` 404 (nicht kompiliert). Register-Block in `/tmp/opencode/weberin-register-block.φ`. Carrier `die-weberin.md`-sha `a5ca7c8f…`.
- **Blockade:** `cometels_compiler` `--catalog`-Modus + `catalog_cometels`-Parser-Arm fehlen; `sources.φ` fremd-dirty.
- **Braucht:** cometels-Katalog-Arm bauen; `weberin_body_verdict` auf `weave_cometels` verdrahten; Register-Block anwenden; TNO `ephemeris_compiler`-Ernte.

#### Weberin Faden-Matrix — Broker-Feldmaps gemessen, Compiler-Bau offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `research-max`) ANTARES `v1/loci` 200, `meta.count`=10000, `page[limit]`/`page[offset]`, Felder `attributes.ra/dec/htm16/properties.ztf_object_id`; kein Server-Filter (clientseitig über htm16). Lasair-ZTF `api/query/` keyed (401 anonym; `Authorization: Token`), POST-Body `selected`/`tables`(nicht-leer)/`conditions`/`limit`/`offset`. Fink `api.ztf.fink-portal.org/api/v1/conesearch` 200 (default-Spalten gemessen); ALeRCE `api.alerce.online/alerts/v1/objects` 200. Seismische Weltlinien: IRIS/FDSN-Katalog als Kandidat.
- **Blockade:** keine (Bau).
- **Braucht:** `antares_loci_compiler.rs` + `lasair_ztf_compiler.rs` bauen; seismische Stations-Weltlinien registrieren.

#### Weberin-Quellen — HAWC-Bundle reproduziert + Reader gebaut, CI-Secret beim Operator
- **Status:** offen | **Bindung:** eigen
- **Trigger:** `OMEGAFLOW_CA_BUNDLE` gesetzt (Operator/CI-Secret)
- **Lage:** (gemessen 2026-09-26 via `research-max`) 4-Zert-Bundle (`yr1 + rootyr_x1 + rootyr_x1b + isrgrootx1`) reproduziert unter `/tmp/opencode/hawc-ca-bundle.pem` (sha `498f9281…`); HAWC `--verdict` 200 (11161 B), `--sniff 2HWC.yaml` 200 (18588 B, sha `1c9566d4…`). Fink/ALeRCE-Reader gebaut: `tools/measure/src/weberin/fink_alerce.rs` + `bin/weberin_fink_alerce_probe.rs` (6 Fixture-Tests, `cargo check` clean). CDN-Assets sha-verifiziert.
- **Blockade:** das YR1-PEM als `OMEGAFLOW_CA_BUNDLE` (CI-Repo-Secret in `hawc-cdn.yml:19,34-37`) — Operator/CI setzt.
- **Braucht:** Operator setzt das Secret auf das Bundle; dann HAWC fetchen.

#### Broken-Null-Control — Spec nachgezogen, te-gate-Lesung offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** der `te-gate`-Lauf 36228804363 (in_progress @ `c6edfbe45`)
- **Lage:** (gemessen 2026-09-26) `docs/specs/broken-null-control.md` §6 auf den gemessenen Stand nachgezogen (Header-sha `63b7e8b7…`); `te-gate` läuft noch.
- **Blockade:** keine.
- **Braucht:** den `te-gate`-Lauf lesen (`ci_manage view`/`log 36228804363`).

#### ZNSP FORMNETWORK — sizeof-Probe-Pfad gefixt, Zahl beim Lauf `36235778378`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `zigbee-host`-Lauf `36235778378`
- **Lage:** (gemessen 2026-09-26) Lauf `36235538625` @ `75bf3131f` success, aber `SIZEOF_ESP_ZB_CFG_T=` **leer**: `size.sh` las `build/main/libmain.a`, das unter ESP-IDF 5.3 `build/esp-idf/main/libmain.a` heisst → `nm` fand `zb_cfg_size` nicht (stille Leerzahl, kein Fehler). Fix `85fcd3cf2`: `find build -name libmain.a`, `test -n "$SIZE_HEX"` (lautes Scheitern statt stillem Leerwert). Neu dispatcht `36235778378`. `form_network_payload_pending()` liefert weiter `None`.
- **Blockade:** keine.
- **Braucht:** Lauf `36235778378` lesen (`SIZEOF_ESP_ZB_CFG_T`), dann den FORMNETWORK-Encoder mit der gemessenen Größe setzen.

#### HRV/Puls→Strahlung — Leser gebaut, Live-Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der BLE-Live-Lauf (Operator) hat `rr` geliefert
- **Lage:** (gemessen 2026-09-26) perm-Log trägt 10 Spalten (`omega.rs:1663-1676`); neuer Leser `tools/measure/src/bin/perm_tone_probe.rs` liest Spalten 7–10 (`tone_code,tone_scale,aperture,field_permeability`), Histogramm + Perzentile, `absent` statt 0.0 (`cargo build -p omegaflow-measure --bin perm_tone_probe` clean). Hinweis: liest den perm-Log direkt (kein `sensors=`-Header-Gate).
- **Blockade:** hängt am BLE-Live-Fluss (Operator).
- **Braucht:** nach dem Live-Lauf `perm_tone_probe <log>` lesen.

#### M2c — Rust-ZNSP-Host auf dem BL808 gegen das H2
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ox64 angekommen (`LZ473049629CN`)
- **Lage:** (gemessen 2026-09-25) Ox64 wird Host-CPU des Coordinators (ZNSP über UART), H2 das Funkmodul; UART0 GPIO14/15, UART1 GPIO16/17. Buildroot-Bring-up ungemessen.
- **Blockade:** Ox64 liegt beim Carrier.
- **Braucht:** Buildroot-Bring-up, dann der Rust-ZNSP-Host auf dem BL808 gegen das H2.

#### BL808-eigenes 802.15.4-Radio — `pending` erneut gemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** öffentlicher 802.15.4-Treiber-/Stack-Fund für den BL808 (`openbouffalo/bouffalo_sdk_bl808/branches`, `zephyr` PRs)
- **Lage:** (gemessen 2026-09-26 via `general`) Zephyr PR #112921 (offen, `will-tm:feature/bflb-ieee802154`) deckt **BL61x/BL70x/BL70xL**, kein BL808; kein Follow-up-PR. `openbouffalo/bouffalo_sdk_bl808`: kein `lmac154`-Zweig (404), nur `master` + `bl808-support-only`. Verdikt: gemessenes `pending`.
- **Blockade:** kein Treiber.
- **Braucht:** PR-Suche `is:pr author:will-tm` / `is:pr BL808 802.15.4` und `bouffalo_sdk_bl808/branches` beobachten.

#### Galileo Borduhr-Sprung A/B — keine Absolutfrequenz-Reihe gefunden
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Fund einer öffentlichen Absolutfrequenz-Reduktion über 1995-11-30/12-01
- **Lage:** (gemessen 2026-09-26 via `general`) gemessenes `absent`: Asmar-1997-USO-Survey ist eine Parametertabelle, keine Zeitreihe; Morabito endet 1993; DESCANSO-Begleitdokumente ohne Reihe; PDS-Galileo-RSS ohne Jupiter-Phasen-Datenkopf; einziger die Grenze kreuzender Katalog ist der SCLK-Kernel (`mk00062a.tsc` 200, 10127 B — Korrelation, keine USO-Reihe).
- **Blockade:** keine Quelle trägt eine Reihe.
- **Braucht:** eine andere Absolut-Frequenzreduktion finden; notfalls als absent halten.

#### JWST Biosignatur-Kanäle pending — 2025–2026 neu gemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine JWST-Detektion + Spektrum eines dieser Kanäle (Referenzstand `10.1073/pnas.2416188122`)
- **Lage:** (gemessen 2026-09-26 via `general`) gemessenes `pending` bestätigt und um 2025–2026-Modi erweitert: nur Modelle/Feasibility (TRAPPIST-1e-O₃-Simulation, Red-Edge-Analog, Saisonal-Klimamodell); K2-18b DMS/DMDS umstritten, nicht O₂/O₃. Kein JWST-Signal mit Spektrum.
- **Blockade:** keine Quelle trägt eine Detektion.
- **Braucht:** bei Fund die Detektion + ihr Spektrum ins Register.

#### Das eine Instrument — Anomalie offen (zweite Augenklasse fehlt)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** ein zweiter Messkanal (VLBI-Beacon auf einer interstellaren Sonde) existiert
- **Lage:** (gemessen 2026-09-25) die Pionier-Anomalie ist unter dem einen Instrument nicht entscheidbar; eine zweite Augenklasse fehlt.
- **Blockade:** kein Instrument misst den vollen Phasenraum der Pioniere.
- **Braucht:** VLBI-Beacon auf der nächsten interstellaren Sonde, von Tag eins zweikanalig.

#### Nadel V — Positive-Control-Konus gemessen, IR-Routen-Registrierung offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `research-max`, aus folge174 getragen) AllWISE-Job `23542882` **COMPLETED**; genau 1 Zeile `J122906.70+020308.6` (w1 8.369 / w2 7.407 / w3 5.147 / w4 2.944, W1−W2 = +0.962). IRAS-PSC `iraspsc` live registriert (`sources.φ:12335-12345`). **AllWISE bleibt `pending`** (`blocked_sources.φ:232-234`) — async-UWS-Arm fehlt; Rat: UWS ist Transport, kein Parser-Gap → **Mountain-Arbeit** (`tap_body_to_json` parst TAP-JSON bereits).
- **Blockade:** AllWISE sync-Stall → async-UWS-Arm nötig (Querlinie Mountain).
- **Braucht:** AllWISE async-UWS-Arm (POST→Poll phase→GET Result); dann `lsst_anomaly_probe` auf dem positiven Kontrollkegel.

#### Pioneer/Dark-Matter — Sweep nachgetragen, Syntonisation-Routen offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) Rampen-Sweep in `probe-front-dark-matter.md` §5.6 nachgetragen (p10 2399 / p11 1518 Pässe; Median-LS-Steigung 2,257e-1/1,383e-1 Hz/s; Resid-RMS 8,845e2/7,958e3 Hz); ASC byte-exakt verdrahtet (`pioneer_doppler_compiler.rs:7-8,70`); Voyager-2-Route registriert (`blocked_sources.φ:51`). Syntonisation-Routen gemessen (2026-09-26 via `research-max`): **NTRS 19830011507** „A two-year history of atomic frequency standards syntonization in the DSN" (1983, trägt eine 2-Jahres-Serie, kein NTRS-Volltext-Download); **NTRS 19820012645** (NBS/GPS-Empfänger 1982, Vergleichswerte <10 ns / ≤1e-14), **NTRS 19840011567** (1984), **DOI 10.1109/freq.1982.200599**; arXiv-Route HTTP 406 `pending`.
- **Blockade:** kein NTRS-Volltext für 19830011507.
- **Braucht:** die Collected Work 19830011495 / den Volltext via `archive_search --ads`/`--playwright` beschaffen (Träger `survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md`).

#### Register-Port-Stau — `phi/sources.φ`/`blocked_sources.φ`/`units.rs` fremd-dirty
- **Status:** offen | **Bindung:** eigen
- **Trigger:** die fremde Linie committet (Dateien sauber)
- **Lage:** (gemessen 2026-09-26) drei fertige Register-Blöcke warten im `/tmp/opencode/`: `korona-register-block.φ`, `weberin-register-block.φ`; Slab2-Ersatzroute + LLNL-G3D-JPS + ISC-EHB sind jetzt gemessen (s. Seismik-Flotte/Positive Maske) und nur noch als Block zu schreiben. `sources.φ`, `blocked_sources.φ`, `units.rs` tragen fremde uncommittete Änderungen (RAVE-DR4/RCSED-ADQL-Kuration).
- **Blockade:** fremde uncommittete Arbeit (nicht überschreiben).
- **Braucht:** nach dem fremden Commit die Blöcke portieren.

### Operator handelt

#### JUICE-Erdpassage 28./29.09.2026 — Kanal fertig, Siegel-Wort fehlt (termin-kritisch)
- **Status:** termin:2026-09-29 | **Bindung:** termin:2026-09-29 (Operator für das Siegel-Wort)
- **Trigger:** 2026-09-28/2026-09-29
- **Lage:** (gemessen 2026-09-26 via `grind-pro`) Prädiktionskanal nachgezogen (`flyby-path-2-preregistration.md` sha `502e06c3…`, addendum sha `b54d5298…`): RTSW mag/wind, Swarm, OMNI2 live gemessen; alle Zellen korrekt `pending` (Feldzustand füllt erst ~1 h vor dem Perigäum). **Riss:** die Kp-Route (`noaa-planetary-k-index.json`) wurde in `61e272ab0` aus `sources.φ` entfernt → Kp-Zelle kann nicht füllen, bis re-registriert (Mycelium-Akt).
- **Blockade:** der Operator muss das **Siegel-Wort vor dem 28.09.** setzen (in 2 Tagen).
- **Braucht:** Siegel-Wort setzen; nach dem Flyby die In-situ-Messung gegen den präregistrierten Feldzustand (σ-Metrik gegen fam).

#### BLE-HR-Live-Messung FR945
- **Status (Vorbereitung):** eigen | **Bindung:** eigen — Kantenzeile liegt bereit.
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
- **Status (Vorbereitung):** eigen | **Bindung:** eigen — Leser `tools/harvest/src/bin/regards_order_read.rs` gebaut (`cargo check` clean): order → Dateiliste → Download-URL; `--live` meldet den gemessenen 403-WAF. REGARDS voll WAF-blockiert (403 direct+Proton, kein Wayback).
- **Status (Akt):** operator-gebunden (Zugang) | **Bindung:** operator
- **Trigger:** Operator-Browser (passiert die WAF) exportiert die Order-/Datei-JSON
- **Braucht:** `regards_order_read <order.json>`; ein Neu-/Nach-Order bleibt konsenspflichtiger Dritt-Akt (Operator-Wort). Order-Ablauf 2026-09-28.

#### Onboard-/CIQ-Bedarf benennen
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort
- **Lage:** (gemessen 2026-09-24) Host-Reader `parse_fit` verdrahtet; CIQ ohne Reader. Ein-Quellen-Regel.
- **Braucht:** Bedarf Ja/Nein — Nein → released, `FIT_DIR` bleibt der FIT-Kanal.

#### Beat-Arbitrierung — verdeckter Lauf mit zwei Beat-Quellen
- **Status (Vorbereitung):** eigen — Spawn-Arbitrierung steht (`main_flow.rs:504`), Command bereit.
- **Status (Akt):** operator-gebunden (hidden) | **Bindung:** operator
- **Trigger:** Operator startet einen verdeckten Lauf mit zwei Beat-Quellen
- **Braucht:** `OMEGAFLOW_HIDDEN=1`-Lauf mit zwei Quellen → genau eine `beat source:`-Zeile.

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

#### Weberin-Quellen — account-/key-Kanäle: 3 von 4 per Credential gelöst, vDEC offen
- **Status (Vorbereitung):** eigen — Entwurf `state/mail/weberin-quellen-konten-2026-09-26.md` (privat, gitignored).
- **Status (Akt):** operator-gebunden (Zugang) | **Bindung:** operator
- **Trigger:** Operator-Wort je Konto
- **Lage:** (gemessen 2026-09-26 via `grind-pro`) **IGETS** (`IGETS_USER/PASS` gesetzt, `isdc.gfz.de/igets-data-base/data-access` 200), **ONC** (`OCEANNETWORKS_TOKEN` verifiziert, `data.oceannetworks.ca/api/devices` 200), **TNS** (`TNS_API_KEY`/`TNS_UA` gesetzt) → nur Harvest-Duty, kein Kontakt. **vDEC** offen: braucht Vertrag, `ctbto.org/.../vdec` 403 direct+Proton, Wayback 200; Draft + QUELLEN send-ready.
- **Braucht:** vDEC-Antrag (Operator) — Webform + Projekttext; Daten bleiben **lokal** (`data/`, keine CDN-Redistribution laut Vertrag).

### Extern handelt (Dritte)

#### Ox64-Lieferung
- **Status:** wartend | **Bindung:** termin (Carrier)
- **Trigger:** Ankunft (`LZ473049629CN`)
- **Lage:** (gemessen 2026-09-25) PINE64 versandte zwei Ox64 — Ankunft offen.
- **Braucht:** Ankunft quittieren; dann M2c.

#### Postfach
- **Status:** wartend | **Bindung:** extern (Mail)
- **Trigger:** neuer Eingang (`state/mail/mail_ledger.φ`)
- **Lage:** (gemessen 2026-09-26) kein Sensory-Treffer; CSES-Limadou-Antwort („wait a few weeks" → Wiedervorlage), DSN-Briefe in Flug, Forum/Alerts.
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

### Benannt — ungemessen (offene Messungen aus diesem Atom)

In diesem Atom benannt, aber nicht gemessen — sie gehören gemessen, nicht geglaubt
(kein Punkt wird als Wahrheit getragen, was keine Messung hat):

- **Korona:** SWPC-Update-Kadenz ungemessen → `ttl 3600`/`τ 86400` sind Schätzungen;
  `millionths` fehlt in `convert_to_si` (`src/archivar/units.rs`).
- **Weberin-Quellen:** ALeRCE `/alerts/v1/detections/?oid=` 404 (Routenform nicht
  exponiert) — `pending`; Fink `getSchema` (159 Felder) nur teilweise ausgewertet.
- **Weberin Faden-Matrix:** die seismischen Stations-Weltlinien-Endpunkte
  (IRIS/FDSN-Katalog) ungemessen.
- **Gekürzte Detailreports:** drei `research-max`-Reports kamen nur gekürzt an —
  Seismik-Flotte (`tool_0dd067430001ov7ekuasCowyjH`), Positive Maske
  (`tool_0dd137a02001vzDSlrnh9T4ga6`), Faden-Matrix
  (`tool_0dd1f38a7001hnGZS1Vqbx5I2W`); die nicht gelesenen Ränder sind ungemessen
  und über `explore` aus den Temp-Dateien nachlesbar.
- **Galileo Borduhr:** DESCANSO Monograph Vol. 14 (Radio Science, 52 MB) nicht
  OCR'd; 209G rev-G ist ein Scan ohne Text-Layer (OCR offen).
- **JUICE:** OMNI2-Fenster 19.09. liefert leeres HAPI-Payload (mehrtägiger Lag) →
  Verifikationskanal, nicht Füllkanal.
- **Weberin cometels:** `mpcorb_extended.json.gz` sha nur partiell (74,2 MB);
  `cometels_flat.json` CDN-Asset 404 (nicht kompiliert).
- **Trishuli:** S1-Post-Szene nicht archiviert (CEMS nur Grading, optisch
  wolkenverdeckt) → räumlicher Footprint bleibt `pending`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.

Eigene Pfade dieses Atoms (pfad-begrenzt committen):
`docs/handover/handover-2026-09-26-sensory-folge176.md`,
`docs/handover/archiv/handover-2026-09-26-sensory-folge175.md`,
`.github/workflows/zigbee-host.yml`.
Nicht committen (fremd): `phi/sources.φ`, `phi/blocked_sources.φ`,
`src/archivar/skydirection.rs`, `tools/measure/src/bin/kbo_residue_probe.rs`,
`tools/measure/src/bin/rixs_cuprate_probe.rs`,
`tools/measure/src/bin/suprastrom_form_probe.rs`, River-Handover,
`gll-rss-rsr-cdn.yml`, `survey-2026-09-26-membran-ladearchitektur.md`.
