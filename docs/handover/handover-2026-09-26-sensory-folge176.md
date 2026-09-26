<!--
  title: Handover — Sensory-Folge 176 (Stand 2026-09-26)
  session: Sensory-Folge 176
  class: handover
  date: 2026-09-26
  sha256: 97eea119085221bfff57b414c8647ee30d12d41cb7fbfd57d0b216f48a6ba7d9
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
- **CI am HEAD `3b659fbe4`:** `zigbee-host` 36235778378 success
  (`SIZEOF_ESP_ZB_CFG_T=16`, s. ZNSP); `meteo-cdn` alle vier success
  (s. Kreuz-Screening); `cometels-cdn` 36238538942 dispatched (s. Weberin);
  `te-gate` 36228804363 in_progress (`fpr-ksg-arx`).
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

#### clippy-Lints in eigenen Dateien — gefixt; `skydirection.rs` fremd
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-26 am HEAD `3eaa60de7` via `ci_manage log 36244460936`, von Mountain getragen) `src/archivar/cometels.rs:125` (`match`→`if let`) und `src/archivar/llnl_g3d.rs:431` (`E0716`) **gefixt**; `src/mathematikerin/te.rs:2971` (`too_many_arguments 8/7`) → `BlattPairSpec`-Struct + zwei Aufrufer, `te.rs:3046` (`neg_cmp_op_on_partial_ord`) **gefixt**; `cargo check -p omegaflow` + `-p omegaflow-measure --bin bz_blatt_probe --bin frb_blatt_probe` 0 Warnungen. `skydirection.rs:200` (rustfmt) ist fremd-dirty (skydirection-cdn-Linie), nicht angefasst.
- **Blockade:** keine (die eigenen Stellen sind gefixt).
- **Braucht:** `gh workflow run ci-check.yml`; die fremden roten Dateien (`extract.rs`, `main_flow.rs`, `port.rs`, `spatial.rs`, `uws.rs`, `skydirection.rs`) bleiben fremde Linien.

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

#### Seismik-Flotte — Compiler-Arme gebaut, CDN-Läufe + 3D-Katalog-Arm offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** die CDN-Läufe
- **Lage:** (gemessen 2026-09-26) **LLNL-G3D-JPS-Arm gebaut** (`src/archivar/llnl_g3d.rs`, `tools/harvest/src/bin/llnl_g3d_jps_compiler.rs`, magic `G3D1`; **3,855,119 Records**, 93,046,104 B, sha `3a1b4c63…`; `volume-cdn.yml`), **ISC-EHB-Arm gebaut** (`isc_ehb_compiler.rs`, `EHB1`; **153,959 arrivals**, 12,316,733 B), **EMC-Arm gebaut** (`src/archivar/emc.rs`, `emc_compiler.rs`, `emc_radial.bin`; der netCDF-4/HDF5-Reader `hdf5.rs` existiert bereits). Quellen in `sources.φ` registriert (LLNL/ISC-EHB/Slab2 live; EMC radial). Der frühere ISC-EHB-Bulk ist RES (Text), die `.grd`/`.nc` sind netCDF-4 (`\x89HDF`), nicht classic.
- **Blockade:** keine.
- **Braucht:** CDN-Läufe dispatcht/lesen (`volume-cdn.yml`, `isc-ehb-cdn.yml`, `emc-cdn.yml`). Der EMC-3D-netCDF-**Katalog**-Arm ist **`descoped`** (gemessen 2026-09-26: der volume-Arm `tools/utils/src/bin/volume_builder.rs` + `volume-cdn.yml` + `src/archivar/hdf5.rs`/`nc4.rs` + 8 `*.volume.bin`-Assets tragen die 3D-Modelle bereits — ein separater netcdf-Katalog-Arm ist nicht nötig).

#### Positive Maske — ODF-Regel + Slab2-Ersatzroute gemessen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) Picker kanonisiert (`picker.rs`); Slab2 ScienceBase 403 direct+Proton `blocked`, **Wayback-Route live** und **Slab2-Arm gebaut** (`slab2_compiler.rs`, `.grd` netCDF-4 → **1,021,034 Records**; `sources.φ`-origin auf die Wayback-Route aktualisiert). Galileo-ODF-Trenner geklärt: **820-013/209G** (nicht 810-005), Separator = **Format-ID Bits 129–131**; 1988er-SIS text-bestätigt (`pds-ppi.igpp.ucla.edu/annex/GO-J-RSS-1-ODF-V1.0/DOCUMENT/TRK_2_18.TXT`, 60720 B), Format-2 golden-verifiziert (`odf.rs:52–100`). **OCR (2026-09-26, WUSTL-Mirror `dsn_trk-2-18.1988-10-15.pdf`, da der JPL-Host 429te):** Table 3b bestätigt — Word 5, Bits **129–131**, 3 b, `Format ID`; Format 1 → `= 1`, Format 2 → `= 2` (1996er Rev); Field-Maps unverändert.
- **Blockade:** keine.
- **Braucht:** `slab2-cdn.yml` lesen; ODF rev-G OCR (vision) oder die zwei Beine als abgeschlossen tragen.

#### Sieben Sphären — Feld + Aggregation gebaut, zwei Live-Quellen pending
- **Status:** offen | **Bindung:** eigen
- **Trigger:** eine Live-Quelle (Stern-Winkeldurchmesser / gemessenes Δz je Okkultation)
- **Lage:** (gemessen 2026-09-26 via `grind-max`) Sphäre I gebaut: `src/archivar/fresnel.rs` (`fresnel_line`, FRS1; θ=2a/D, F=a²/(λD), √(λD/2)); Gaia-Farbe→λ über `spectral.rs::bp_rp_to_lambda_nm` (Planck-SED, photon-gewichtet durch den eingebetteten Passband); Sphäre VII in `src/mathematikerin/doppler.rs` (`doppler_dz`=GM/(c²b), Kreuzprodukt-Impactparameter, `prediction_stat`/`residual_stat`, DGZ1). `cargo check`/`--tests` 0 Warnungen; 19 neue Tests (CI-only). Carrier `the-seven-spheres.md` trägt jetzt genau 2 `pending`.
- **Blockade:** kein Katalog trägt das Stern-Winkeldurchmesser-Feld; kein gemessenes Δz je Okkultation.
- **Braucht:** JSDC II/346 registriert (`02e09dbbc`: `jsdc_ldd_mas`, `asu-tsv`, `at sun`; `convert_to_si mas` vorhanden) — **noch kein `pending` gefüllt**: der Marker nennt ein *okkultations-abgeleitetes* Winkeldurchmesser-Feld, nicht den JSDC-Interferometrie-Katalog, und dem JSDC fehlt die Distanzspalte (Cross-Match nötig). Schritt: Gaia-Parallaxe cross-mappen ODER die Okkultations-Lichtkurven beschaffen. Δz-Okkultation: kein Live-Katalog (Gaia-Archiv-TAP `gea.esac.esa.int` 401 anonym) → beide pendings bleiben.

#### Korona-Heizung — Feldmap + `millionths`-Arm live registriert
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) Feldmap in `sources.φ` registriert (`at sun`, `map .` → Surface→ICRS, area=„millionths", extent=deg, `lon carrington_longitude`); `millionths`→SI-Arm in `src/archivar/units.rs` (×2πR☉²·1e-6 ≈ 3,04e12 m²), `cargo check -p omegaflow` 0 Warnungen. **Kadenz gemessen:** rollierendes 31-Tage-Tagesfenster (`observed_date` 2026-08-27…09-26), Last-Modified 2026-09-26T14:04:31Z; CDX-Proof intraday-Rewrite (2 Captures 4 h 22 m auseinander, 2026-01-02) → `ttl 3600` OK, `τ 86400` gilt auf Tagesebene.
- **Blockade:** keine.
- **Braucht:** `aia_compiler --harvest` (CI-only).

#### Trishuli — Bahrabise-Ernte + TE-Sweep gemessen, S1-Footprint offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** S1-Post-Szene
- **Lage:** (gemessen 2026-09-26 via `grind-flash`) DHM-113-Ernte gefahren: **1007 Punkte**, 10-min, 2026-08-25T14:25…2026-09-01T14:15 UTC, Level 1.115–2.763 m (Snapshot `20260901142220`, sha `17b34f91…`); Regen Open-Meteo 312 Zeilen; Alignment n=169. TE-Sweep: **precip→stage signifikant bei lag 3 (TE 0.2275 > threshold 0.1725) und lag 6 (0.2532 > 0.1687)**, lag 1/12/24 kein Fund, stage→precip in keinem lag. `te_pair_probe` bewusst nicht gelaufen (keine Zeit-Alignment → bogus n=min).
- **Blockade:** S1-Post-Szene nicht archiviert.
- **Braucht:** S1-Footprint nach Archivierung.

#### Weberin — cometels-Katalog-Arm + Consumer gebaut, CDN + TNO offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `grind-max`) `src/archivar/cometels.rs` neu (`catalog_cometels`-Parser-Arm, magic `CTL1`, 5 Tests); `CometelsRec` aus `weberin.rs` in den Archivar verschoben, `weave_cometels` über `desig_of`; `cometels_compiler --catalog` (CI-Upload) und `weberin_body_verdict --cometels` (Consumer) verdrahtet. `cargo check -p omegaflow` 0 Warnungen; Probe: **837 Element-Records, 122 skipped** (e≥1/void). Routen: `cometels.json.gz` 200 (52939 B); `cometels_flat.json` CDN 404. Der CDN-Lauf `36238538942` **failure** — der Workflow hatte **keinen Checkout/Toolchain** (Job lief ohne Repo, `could not find Cargo.toml`, 4 s); Fix `1a4925344`, neu dispatcht `36252200125`.
- **Blockade:** keine.
- **Braucht:** Lauf `36252200125` lesen (Manifestation); TNO `ephemeris_compiler`-Ernte (SPK-Registrierung, wenn `sources.φ` frei).

#### Weberin Faden-Matrix — Broker-Compiler gebaut, seismische Endpunkte offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `grind-flash`) `antares_loci_compiler.rs` + `lasair_ztf_compiler.rs` gebaut (`cargo build -p omegaflow-harvest --bin …` 0 Warnungen). **Korrigierte Route:** ANTARES-JSON-API = `https://api.antares.noirlab.edu/v1/loci` (die Vorhandover-URL `antares.noirlab.edu/api/v1/loci` ist die HTML-Frontend, 974 B); `meta.count` 10000, Pagination `page[limit]/page[offset]`, Felder `attributes.ra/dec/htm16`, `properties.ztf_object_id`. Lasair `api.lasair.lsst.ac.uk/api/query/` 401 anonym (Key `LASAIR_LSST_TOKEN`), POST `selected/tables/conditions/limit/offset`. **ALeRCE korrigiert:** Basis `api.alerce.online/ztf/v1/` (`/objects/{id}/detections` 200, `swagger.json` 19653 B; `/alerts/v1/…` 404 bestätigt). **Fink:** `/api/v1/schema` 200 (4 Gruppen, 169 Tokens; Felder u. a. `magpsf/sigmapsf/magdiff/rb/drb/roid/…`). **Seismik-Weltlinien gemessen:** EarthScope-FDSN `service.earthscope.org/fdsnws/station/1/query?level=station&format=text&nodata=404` (Spalten `Network|Station|Latitude|Longitude|Elevation|SiteName|StartTime|EndTime`; v1.1.57); ISC `station.zip` 2.144.159 B.
- **Blockade:** keine.
- **Braucht:** den FDSN-Station-Endpunkt in `sources.φ` registrieren.

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

#### ZNSP FORMNETWORK — sizeof gemessen (16), Encoder-Site nicht im Baum
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) sizeof-Probe-Pfad gefixt (`85fcd3cf2`); Lauf `36235778378` @ `85fcd3cf2` **success**, **`SIZEOF_ESP_ZB_CFG_T=16`** gemessen. Aber `sgrep --all -i "form_network|formnetwork|znsp|zigbee" .` findet **keine Encoder-Site** im Baum — die im Vorhandover genannte `form_network_payload_pending()` ist nicht auffindbar; einzig `.github/workflows/zigbee-host.yml` existiert.
- **Blockade:** die Encoder-Site fehlt (Handover-Claim ohne Read-Site).
- **Braucht:** Encoder-Site lokalisieren, sonst den Claim streichen und `16` als Register-Wert führen (`phi/…`).

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
- **Lage:** (gemessen 2026-09-26 via `general`) gemessenes `absent`: Asmar-1997-USO-Survey ist eine Parametertabelle, keine Zeitreihe; Morabito endet 1993; PDS-Galileo-RSS ohne Jupiter-Phasen-Datenkopf; einziger die Grenze kreuzender Katalog ist der SCLK-Kernel (`mk00062a.tsc` 200, 10127 B — Korrelation, keine USO-Reihe). **DESCANSO Monograph Vol. 14** (`descanso.jpl.nasa.gov/monograph/series14/Radio-Science.pdf`, 200, 52.985.643 B, 458 S., Text-Layer) gemessen: Lehrbuch, keine Reihe; Table 5.1 (PDF p.277) trägt nur USO-Charakteristika (Galileo-Probe: FEI 1975, SC-Cut, 23.117 MHz nominal, Allan ≈5e-10 @1000 s; Orbiter-USO Serial #4 aus der FEI-1975-Charge, 19.1 MHz) — kein 1995-11-30/12-01-Bezug.
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

#### Nadel V — AllWISE-Arm gebaut (Mycelium), Probe offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** der `allwise-cdn`-Lauf `36249453619`
- **Lage:** (gemessen 2026-09-26) AllWISE-Job `23542882` COMPLETED (genau 1 Zeile `J122906.70+020308.6`, w1 8.369 / w2 7.407 / w3 5.147 / w4 2.944, W1−W2 = +0.962); IRAS-PSC `iraspsc` live (`sources.φ:12335-12345`). **Entblockt:** Mycelium hat den async-UWS-Arm gebaut (`c8e87ab2e`: `src/archivar/uws.rs` + `src/archivar/allwise.rs` + `tools/harvest/src/bin/allwise_tap_compiler.rs`) und die Quelle registriert (`sources.φ:9697` `catalog_allwise_psd`, Felder `w1mpro..w4mpro`/`w3snr`/`w4snr`); `allwise-cdn` `36249453619` in_progress auf `0e09a3e81`.
- **Blockade:** keine.
- **Braucht:** Lauf `36249453619` lesen (Manifestation `allwise_psd.bin`); dann `lsst_anomaly_probe` auf dem positiven Kontrollkegel.

#### Pioneer/Dark-Matter — Sweep nachgetragen, Syntonisation-Routen offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) Rampen-Sweep in `probe-front-dark-matter.md` §5.6 nachgetragen (p10 2399 / p11 1518 Pässe; Median-LS-Steigung 2,257e-1/1,383e-1 Hz/s; Resid-RMS 8,845e2/7,958e3 Hz); ASC byte-exakt verdrahtet (`pioneer_doppler_compiler.rs:7-8,70`); Voyager-2-Route registriert (`blocked_sources.φ:51`). Syntonisation-Routen gemessen (2026-09-26 via `research-max`): **NTRS 19830011507** „A two-year history of atomic frequency standards syntonization in the DSN" (1983, trägt eine 2-Jahres-Serie, kein NTRS-Volltext-Download); **NTRS 19820012645** (NBS/GPS-Empfänger 1982, Vergleichswerte <10 ns / ≤1e-14), **NTRS 19840011567** (1984), **DOI 10.1109/freq.1982.200599**; arXiv-Route HTTP 406 `pending`.
- **Blockade:** kein NTRS-Volltext für 19830011507.
- **Braucht:** NTRS-19830011507-Volltext ist **measured absent**; ADS hat den Artikel (`1982TDAPR..72..118W` / `1983tdar.nasa..118W`) — Volltext-/Bezugsroute via `archive_search --playwright` auf ADS prüfen (Träger `survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md`).

### Operator handelt

#### JUICE-Erdpassage 28./29.09.2026 — Kanal fertig, Siegel-Wort fehlt (termin-kritisch)
- **Status:** termin:2026-09-29 | **Bindung:** termin:2026-09-29 (Operator für das Siegel-Wort)
- **Trigger:** 2026-09-28/2026-09-29
- **Lage:** (gemessen 2026-09-26 via `grind-pro`) Prädiktionskanal nachgezogen (`flyby-path-2-preregistration.md` sha `502e06c3…`, addendum sha `b54d5298…`): RTSW mag/wind, Swarm, OMNI2 live gemessen; alle Zellen korrekt `pending` (Feldzustand füllt erst ~1 h vor dem Perigäum). **Riss:** die Kp-Route (`noaa-planetary-k-index.json`) wurde in `61e272ab0` aus `sources.φ` entfernt → Kp-Zelle kann nicht füllen, bis re-registriert (Mycelium-Akt). **OMNI2:** das Fenster 19.09. liefert leeres HAPI-Payload (mehrtägiger Lag) → Verifikationskanal, kein Füllkanal.
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

Keine offen — alle zuvor hier benannten Messungen sind in diesem Atom gemessen und
in ihre Punkte gefaltet (Korona-Kadenz, Weberin ALeRCE/Fink + Seismik-Weltlinien,
Galileo/TRK-2-18 + DESCANSO, cometels-CDN, JUICE-OMNI2, Trishuli-S1).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.

Eigene Pfade dieses Atoms (pfad-begrenzt committen):
`docs/handover/handover-2026-09-26-sensory-folge176.md`,
`docs/handover/archiv/handover-2026-09-26-sensory-folge175.md`,
`.github/workflows/zigbee-host.yml`,
`src/archivar/cometels.rs`, `src/archivar/mod.rs`, `src/lib.rs`, `src/weberin.rs`,
`tools/harvest/src/bin/cometels_compiler.rs`,
`tools/measure/src/bin/weberin_body_verdict.rs`,
`.github/workflows/cometels-cdn.yml`,
`tools/harvest/src/bin/antares_loci_compiler.rs`,
`tools/harvest/src/bin/lasair_ztf_compiler.rs`,
`src/archivar/llnl_g3d.rs`, `src/archivar/emc.rs`, `src/archivar/zeuge.rs`,
`src/archivar/units.rs`,
`tools/harvest/src/bin/llnl_g3d_jps_compiler.rs`,
`tools/harvest/src/bin/isc_ehb_compiler.rs`,
`tools/harvest/src/bin/slab2_compiler.rs`,
`tools/harvest/src/bin/emc_compiler.rs`,
`.github/workflows/volume-cdn.yml`, `.github/workflows/emc-cdn.yml`,
`.github/workflows/isc-ehb-cdn.yml`, `phi/sources.φ`.
Nicht committen (fremde Hunks in geteilten Dateien): `phi/blocked_sources.φ`
(Planck-SZ-Eintrag), `phi/declined_sources.φ` (Kp),
`src/archivar/skydirection.rs`, `tools/measure/src/bin/kbo_residue_probe.rs`,
`tools/measure/src/bin/rixs_cuprate_probe.rs`,
`tools/measure/src/bin/suprastrom_form_probe.rs`, River-Handover,
`gll-rss-rsr-cdn.yml`, `survey-2026-09-26-membran-ladearchitektur.md`.
