<!--
  title: Handover — Sensory-Folge 174 (Stand 2026-09-26)
  session: Sensory-Folge 174
  class: handover
  date: 2026-09-26
  sha256: 94b2e1ce54db86489b319e9b471bc19695e299534287f6c43902b58786a57357
  status: live
-->
# Handover — Sensory-Folge 174 (2026-09-26)

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
oder descoped-Befund. Die nächsten Schritte sind die in
`archiv/handover-2026-09-25-sensory-folge171.md` (Träger-Register) und
`archiv/handover-2026-09-25-sensory-folge172.md` gemessenen, am 2026-09-26 gegen
`register_lookup --orphan-docs` abgeglichen. Der Dateiname in dieser Übergabe ist
der Träger (`register_lookup --orphan-docs`).

- `docs/blatt/blatt-der-grat.md` (2) → ENSO-Pfeil messen: `cross_te_screen`.
- `docs/blatt/blatt-kreuz-screening-gyirong.md` (3) → Träger Punkt „Kreuz-Screening auf weitere Ereignisse" (unten): `cross_te_screen` auf `meteo_harvest/tibet-flut-2026`; Bordeaux/Aaretal/Japan ernten.
- `docs/blatt/blatt-solar-seconds-matrix.md` (1) → `corona_conditional_probe` auf das Paar 211A→193A.
- `docs/blatt/blatt-thuan-fragesteller.md` (4) → `termin:2026-12-02` (Gaia DR4), dann `docs/auftrag/archiv/auftrag-gaia-dr4-iapetus.md`.
- `docs/concepts/arxiv-api.md` (2) → Träger Punkt „arxiv HTTP 406" (Mountain-Linie, `handover-2026-09-26-mountain-folge166`); Marker Z.59/65.
- `docs/concepts/blatt-papier-resultat.md` (1) → Blatt-1-Bojen-Matrix-Rotor laufen lassen, Matrix-Zeile Σ p̂·M nachtragen (Z.63–71).
- `docs/concepts/das-eine-instrument.md` (2) → Träger Punkt „Das eine Instrument — Anomalie offen" (unten).
- `docs/concepts/der-paradigmenwechsel.md` (9) → Träger Punkt „JUICE-Erdpassage 28./29.09.2026" (unten): ECM-Magnetfeld + Sonnenwind/IMF-Bz/Kp am Perigäums-ICRS-Punkt.
- `docs/concepts/die-akteure-im-boden-und-wasser.md` (7) → Träger Punkt „Seismik-Flotte: Streuung senken + Stationsterm + W-Phase-M9" (unten).
- `docs/concepts/exzellenz-konzept.md` (3) → descoped (gemessen 2026-09-26: Marker sind die Definition von pending §2/§2.5; Body final → `docs/concepts/exzellenz-konzept.md:91`).
- `docs/concepts/fuenf-funken-anomalie-suche.md` (4) → Funke 3 via `broker_difference_probe`, Funke 5 (TDB-Koinzidenz-Fenster) bauen.
- `docs/concepts/kybernetische-astrophysik.md` (10) → descoped (gemessen 2026-09-26: die Marker sind Status-Vokabular „offen"/"pending" im Essay-Body → `docs/concepts/kybernetische-astrophysik.md:45`).
- `docs/concepts/recherche-galileo-kadenz-reconciliation.md` (1) → `archive_search --ntrs 19930010224` bzw. DSMS Services Catalog v7.5 §"Doppler count interval".
- `docs/concepts/the-seven-spheres.md` (2) → Träger Punkt „Sieben Sphären — Messvorschriften" (unten): Fresnel-Winkeldurchmesser-Feld (`src/archivar`).
- `docs/concepts/zeugnis.md` (5) → descoped (gemessen 2026-09-26: die Marker sind die Disziplin-Prosa des Handover-Abschnitts F → `docs/concepts/zeugnis.md:370`).
- `docs/paper/asmar-2005-spacecraft-doppler-tracking-noise-budget.md` (2) → descoped (gemessen 2026-09-26: Treffer sind der Substring „depending" Z.29/35, kein offener Punkt).
- `docs/paper/causal-arrow-preregistration.md` (1) → `te_pair_probe` (Lag-Sweep {1,3,6,12,24,48}) gegen Rasuwa-Regen; DAHITI Koshi via `sfetch` (api_key).
- `docs/paper/corona-heating-ladder.md` (2) → Träger Punkt „Korona-Heizung" (unten): `aia_ladder_probe` über den vollen 613-Event-Korpus.
- `docs/paper/cross-screening-tibet.md` (1) → Träger Punkt „Kreuz-Screening auf weitere Ereignisse" (unten).
- `docs/paper/depth-phase-echo-fleet.md` (5) → Träger Punkt „Seismik-Flotte" (unten).
- `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4) → Träger Punkt „JUICE-Erdpassage 28./29.09.2026" (unten): nach dem Flyby das In-situ-Feld + σ-Metrik gegen fam.
- `docs/paper/flyby-path-2-preregistration.md` (1) → Träger Punkt „JUICE-Erdpassage 28./29.09.2026" (unten): Feldzustand auf den Perigäum-Tube füllen.
- `docs/paper/galileo-rotor-spin-era-floor.md` (1) → CK-Kerne jenseits `ck90341a`–`ck90344b` (Frame −77000) von `naif.jpl.nasa.gov` harvesten.
- `docs/paper/jwst-disequilibrium-survey.md` (7) → Träger Punkt „JWST Biosignatur-Kanäle pending" (unten).
- `docs/paper/laic-arrow-direction.md` (3) → `archive_search --playwright https://leos.ac.cn` (CSES SPA).
- `docs/paper/nadel-v-fresh-area-dip-scan.md` (1) → Träger Punkt „Nadel V" (unten): `lsst_anomaly_probe` auf dem positiven Kontrollkegel.
- `docs/paper/planet-nine-kbo-residue.md` (1) → descoped (gemessen 2026-09-26: die Zählung ist exhaustiv (Z.66), Familiensumme 7180 = Katalogtotal; Arbeit liegt im Punkt „Planet-Nine/KBO").
- `docs/paper/probe-front-dark-matter.md` (2) → Träger Punkt „Pioneer/Dark-Matter" (unten); Syntonisation-1983/GPS-1982–87 via `--ads`/`--ntrs`.
- `docs/paper/solar-seconds-matrix.md` (3) → `corona_conditional_probe` auf das Paar 211A→193A (wie `blatt-solar-seconds-matrix`).
- `docs/paper/sturzflut-tibet-pfeil.md` (23) → Träger Punkt „Trishuli — Bahrabise-Richtungsverifikation + S1/SAR-Footprint" (unten).
- `docs/paper/tonga-lamb-crosscheck.md` (3) → Träger Punkt „BGR-Matched-Filter-Ankunft (Tonga) — account-blockiert" (unten).
- `docs/surveys/axiom-gate-broken-null-control.md` (1) → Träger Punkt „Broken-Null-Control" (unten).
- `docs/surveys/axiom-gate-depth-phase-echo-fleet.md` (1) → Träger Punkt „Seismik-Flotte" (unten).
- `docs/surveys/survey-2026-09-14-ehrlich-benannt-werkzeug-luecke.md` (17) → Träger Punkte „Pioneer/Dark-Matter" + „DSN-Briefe in Flug" (unten): vier Werkzeuglücken.
- `docs/surveys/survey-2026-09-16-sonden-flotte.md` (4) → Träger Punkt „DSN-Briefe in Flug" (unten).
- `docs/surveys/survey-ein-blatt-korona-heizung.md` (1) → Träger Punkt „Korona-Heizung" (unten).
- `docs/surveys/survey-fortschritt.md` (1) → descoped (gemessen 2026-09-26: Historien-Survey 2026-08-16, Abschnitt C „leben zusätzlich im TODO" → `docs/surveys/survey-messpunkt-verteilung.md`).
- `docs/surveys/survey-messpunkt-verteilung.md` (6) → descoped (gemessen 2026-09-26: Kandidaten 1–8 sind in `docs/surveys/survey-auswertung.md` verdiktet; Kandidat 9 und §6 sind Konsultations-Einladung, kein Bau-Punkt).
- `docs/concepts/ein-blatt-papier.md` (2) → Lag-Sweep-Träger Punkt „causal-arrow-preregistration" (`te_pair_probe` Lag-Sweep {1,3,6,12,24,48} gegen Rasuwa-Regen); KDE-Bandbreiten-Gate descoped (gemessen 2026-09-26: `.github/workflows/laic-verdict-cdn.yml:43-48` fährt `laic_probe --kde-scale 2.0`); die `pending`-Zellen der Kanal-Lage (Z.74) sind der Quellen-Port-Stand, kein Handlungspunkt dieses Konzepts.
- `docs/concepts/blatt-papier-beweis.md` (3) → descoped (gemessen 2026-09-26: die `Membran-Bindung` (Z.42) ist seit Atom 9 gebaut — `src/mathematikerin/omega.rs:349`, `src/mathematikerin/actuators.rs:29`, Test `src/mathematikerin/tests.rs:511`, Commit `356fa616`; die `pending`-Kanalzellen (Z.69/71: imos_argo_sst/ESA-CCI, TAO/ERA5, SOI, CSES) sind der Quellen-Port-Stand, kein offener Handlungspunkt dieses Konzepts).
- `docs/paper/terminologie-der-gegenstroemung.md` (1) → descoped (gemessen 2026-09-26: der Marker ist der Begriff „pending" im Definitions-Kopf `**pending.**` → `docs/paper/terminologie-der-gegenstroemung.md:22`; die Definitionen Ontologie-Motor/mycorrhizal_internet sind gesetzt → `docs/concepts/glossar.md:59`, `:61`).
- `docs/surveys/survey-2026-09-06-codestruktur.md` (8) → Träger Stehender Pass „CI-Status am HEAD" (oben): die `pending CI`-Zeilen (Z.79–85; Run-IDs 36171288869/36172029908/36172034181) sind mit `ci_manage view <run-id>` lesbar; die `Offen`-Köpfe (Z.13/68/70) sind die Struktur-Karte.
- `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md` (2) → descoped (gemessen 2026-09-26: die zwei Marker (Z.56/143) sind das Substring „wartet" in „erwartete" — Scanner-Artefakt; die Silence-Map-Probe ist gebaut → `tools/measure/src/bin/silence_map_probe.rs`, CI `.github/workflows/measure-gates.yml:27`).
- `docs/surveys/survey-2026-09-17-verlorene-diskussionen.md` (9) → Träger Punkt „HRV/Puls→Strahlung" (unten, wartend, Z.84); die sources-Repo-Messung (Z.97/128) ist privat/Mycelium-Linie; Z.83/86 sind Beleg-geführt (2026-09-23), Z.19/61/95/117 sind Prosa.
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` (2) → Träger River-/Browser-Linie: Kaltstart-Entschärfung der Store-Extension (Z.195–199, operator-gebunden) + Versionslücke 0.16.1→0.17.0 (Z.146); die Marker Z.93/164 sind Prosa (Upstream-Issue #319, Navigations-Wartezeit `load`).
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` (7) → Träger Punkte „FIT-Verifikation eigene FR945-Datei" (operator-gebunden, Z.332) + „BLE-HR-Live-Messung FR945" (unten); die Pixel-LAN-Gerätemessung (Z.325) ist River folge13/operator-gebunden; die Quest-`ungemessen`-Kanäle (Z.102/105/110/116) und „Offene Messpunkte" (Z.346) sind hardware-gated.

- **Inline-Code-Klasse — geschlossen:** descoped (gemessen 2026-09-26: mit `strip_inline_code` (`tools/register/src/bin/register_lookup.rs:92`) tragen `docs/concepts/die-weberin.md`, `docs/concepts/docs-naming.md`, `docs/concepts/kybernaut-native-methodology.md` und `docs/concepts/the-counter-slope.md` keine offenen Marker mehr; `register_lookup --orphan-docs` listet sie nicht).

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### Commit ohne Operator-Wort — `c6edfbe45` (Riss, 2026-09-26)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) ein Sub-Agent hat die Workflow-Änderungen (`laic-verdict-cdn.yml` `--kde-scale`, `te-gate.yml` fpr-ksg split) ohne das `/commit`-Wort committet und gepusht (`c6edfbe45`, `HEAD == origin/main`). Der session-weite Consent ist Delegation, nicht das Commit-Wort (AGENTS.md: „Commit and push carry the operator's consent word").
- **Blockade:** keine (der Commit steht; kein destruktiver Rückbau erlaubt).
- **Braucht:** im Handover sichtbar tragen; im nächsten Abschluss-Check benennen.

#### paper-check rot — `big-bang-echo-sheet-12` Header-sha stale (von River getragen, 2026-09-26)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `ci_manage log 36224203710` + `omega_sh sha`) `export_latex --check` (paper-check) ist rot und nennt neben dem bereits behobenen gic-Abstract das Blatt `docs/paper/big-bang-echo-sheet-12.md`: Header `sha256: e269ada0…` ≠ gemessener Body-sha `0a5f2c78…`; zusätzlich `title=64 (arxiv=66)`. Datei ist committed, letzter Touch `82178d0e8 sensory`.
- **Blockade:** keine.
- **Braucht:** Header-`sha256` auf `omega_sh sha docs/paper/big-bang-echo-sheet-12.md` setzen (bzw. Body/Header konsistent machen), committen; damit wird `paper-check` wieder grün.

#### Kreuz-Screening auf weitere Ereignisse — Konfigs gebaut, Läufe offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25 via `grind-pro`) die drei Ereignis-Konfigs stehen unter `phi/meteo/` im tibet-Schema: `aaretal-hochwasser-2026.json` (Fenster 27.05.–05.06.2026, Habkern/Beatenberg/Interlaken), `bordeaux-waldbrand-2026.json` (22.–31.07.2026, Landiras/La-Teste/Bordeaux), `japan-tsunami-2026.json` (25.07.–03.08.2026, Kumamoto/Yatsushiro/Kashima); `.gitignore` trägt `!phi/meteo/*.json`. Drei gemessene Rissen: „Bordeaux" ist ein **Waldbrand** (kein Hochwasser); „Japan" ist das **Kumamoto-Beben** (Mww 6.8 vs 7.1) mit aufgehobener Tsunami-Advisory; das Aaretal-2026-Ereignis ist die **Sturzflut Habkern/Beatenberg** (Aare-Hochwasser war 2025). `.github/workflows/meteo-cdn.yml` ist jetzt verdrahtet: neuer Build-Step (`meteo_harvest`, `cross_te_screen`), Step „Harvest series" (`meteo_harvest --event phi/meteo/<event> --out phi/pipeline/meteo_harvest/<id>`), Step „Cross-screen" (`cross_te_screen --dir … --lags 1,6,12,24 --surrogate 20 --min-n 100`), Concurrency per Event (`group: ${workflow}-${inputs.event}`, `:4`) — die drei parallelen Dispatches brechen einander nicht mehr ab. Die Änderung ist UNCOMMITTED (Dispatch gegen `--ref main` trug noch den alten Pfad).
- **Blockade:** keine — die Läufe sind offline/CI.
- **Braucht:** committen/pushen (mit `/commit`), dann die vier Events dispatchen. Run-ID tibet `36228880981` (alter Pfad), aaretal `36226620984`, japan `36226623897` (in_progress/pending), bordeaux `36226622412` cancelled (Concurrency) → nach Commit neu dispatchen.

#### Seismik-Flotte: Streuung senken + Stationsterm + W-Phase-M9
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `grind-pro`) der W-Phase-M9-Zweig ist gebaut: `w_phase_bandpass` (RC-Kaskadenfilter, Band 100–1000 s), `w_phase_energy_ratio` (RMS-Fenster `[t_s, t_s+600 s]` gegen `[t_p−200 s, t_p]`, `Option`-skip), `w_phase_discriminate` → `Option<WPhasePick>` mit Gate `energy_ratio >= 8.0` (4 Tests, `tools/measure/src/depthphase.rs`); pP-Zweig und `MIN_DIST_DEG = 30.0` unberührt. Flotte unverzerrt (+1,7 km, se 4,7 km), aber 19/36 km dominieren das ±10-km-Gate; ak135 ist 1D (`positive-maske.md:64`), kein 3D-Modell registriert. Der W-Phase-M9-Zweig ist jetzt in die Flotte verdrahtet (`depthphase.rs` `measure_station` trägt `w_phase: Option<WPhasePick>`, S-Anker = ak135 S−P-Lag auf dem gemessenen P-Onset, Band 100–1000 s, Gate energy_ratio ≥ 8.0; Flotte druckt je Ereignis `w_m9`/`w_below`/`w_pending`); `depth_phase_fleet_probe.rs` + die field/azimuth/stromboli-Bins auf `picker` verdrahtet. pP-Residuum bleibt `pending` (kein 3D-Modell).
- **Blockade:** kein 3D-Geschwindigkeitsmodell (pP-Residuum); Stationsterm-Quelle.
- **Braucht:** 3D-Modell registrieren (pP-Residuum) oder `pending` halten; Stationsterm II.KIV.

#### Positive Maske — Audit-Punkte + fehlende Treiber
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `grind-pro`) der M9.1-Picker ist kanonisiert: `tools/measure/src/picker.rs` (`p_onset`, `first_break_arrival`, `sta_lta_arrival`, `bandpass`, 6 Tests) + `pub mod picker` (`lib.rs`); `quake_location_probe.rs` nutzt das Modul (privates Duplikat entfernt). Die Flotte nutzt `p_onset` bereits über `depthphase::measure_station` (`depthphase.rs:857`); die `depthphase.rs`-Picker-Kopie ist entfernt, alle Konsumenten laufen über `tools/measure/src/picker.rs` (6 Tests); neue Tests für W-Phase (2). Echo-Tiefe σ 19/36 km, Stationsterm offen (II.KIV +5,69 s); **Slab2 registriert** (`phi/sources.φ:12169-12174`), ScienceBase-Route 403 direct+Proton `blocked`; `galileo_odf.bin` registriert (`sources.φ:8369-8375`).
- **Blockade:** Slab2 403.
- **Braucht:** Galileo-ODF-Formattrenn „1 vs 2" im 209G-Text; Tomografie als Ernte-Kandidat registrieren.

#### Sieben Sphären — Messvorschriften
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) Sphäre I (Fresnel-Winkeldurchmesser-Feld) ausstehend, VII (Dopplergeist) ohne statistische Aggregation; II–VI sind Konzept.
- **Blockade:** keine.
- **Braucht:** das Fresnel-Winkeldurchmesser-Feld (Asteroiden-Bahn ∩ Gaia-Farbe ∩ IR-Ø) und die Dopplergeist-Aggregation bauen.

#### Korona-Heizung — Aktive-Region-Route gemessen, Parser-Arm offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `research-max`) die lebende Route ist `https://services.swpc.noaa.gov/json/solar_regions.json` (HTTP 200, 138 703 B, sha256 `9a91cea7…`, `application/json`); Schema `observed_date, region, latitude, longitude, location, carrington_longitude, area, spot_class, extent, number_spots, mag_class, …` (kein ICRS, heliographisch). Der alte Register-Eintrag `/products/solar-regions.json` war 404-tot und ist im Dispositions-Register auf die lebende URL + gemessene Felder korrigiert. NGDC Sunspot-`table_international-sunspot-numbers_daily.txt` 404; lebende Alternative SILSO `SN_d_tot_V2.0.csv` (200, `text/csv`). `nobel_probe_corona.rs` v2 + `aia_compiler --region` gebaut. Die SWPC-Route ist als `parser-def json` + `gap unit-auto-detect` in `phi/blocked_sources.φ:286` registriert (note: live `/json/solar_regions.json` 200 138703 B sha9a91cea7; area=millionths, extent, spot_class=McIntosh, number_spots einheitenlos; heliographisch→ICRS). Der Rat hat entschieden: keine neue `phi/*.φ`-Datei, Port-Eintrag in bestehende Struktur; `lon_key` muss `carrington_longitude` tragen, nicht `longitude`.
- **Blockade:** kein `sources.φ`-Parser-Arm für die JSON-Route (unit-auto-detect gap); die Ernte ist CI-only.
- **Braucht:** Feldmap in `sources.φ` mit gelösten Einheiten (oder `unit-auto-detect` schließen); `aia_compiler --harvest`.

#### Nadel V — Positive-Control-Konus gemessen, IR-Routen-Registrierung offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `research-max`) AllWISE-Job `23542882` **COMPLETED**; Result 200 VOTable 1.3, Query `SELECT designation,ra,dec,w1mpro..w4mpro,w3snr,w4snr FROM allsky_4band_p3as_psd WHERE CONTAINS(POINT('ICRS',ra,dec),CIRCLE('ICRS',187.2779,2.0524,0.001667))=1` — **genau 1 Zeile** `J122906.70+020308.6`, w1 8.369 / w2 7.407 / w3 5.147 / w4 2.944, W1−W2 = +0.962. Routen gemessen: AllWISE/TAP `https://irsa.ipac.caltech.edu/TAP` (async-UWS der Weg; Sync-Stall bekannt), IRAS-PSC Tabelle `iraspsc` (Sync 200, 81 Spalten, fnu_12/25/60/100), Herschel HSA `archives.esac.esa.int/hsa/whsa-tap-server/tap/sync` (`hsa.pacs_point_source_100/160` 200, registriert `sources.φ:7452`). **IRAS-PSC `iraspsc` ist live in `phi/sources.φ:12335-12345` registriert** (`format tap`, Sync-TAP, `field fnu_12/25/60/100 iras_psc_fnu_*_jy inverse-square em Jy`); **AllWISE bleibt `pending` in `phi/blocked_sources.φ:232-234`** (async-UWS-Arm fehlt; der Rat: UWS ist Transport, kein Parser-Gap — `tap_body_to_json` parst das TAP-JSON schon; der async-Arm ist Mountain-Arbeit).
- **Blockade:** keine (Registrierung); AllWISE sync-Stall → async-UWS-Parser-Arm nötig.
- **Braucht:** AllWISE async-UWS-Arm bauen (POST→Poll phase→GET Result); dann `lsst_anomaly_probe`.

#### Pioneer/Dark-Matter — Routen gemessen, Rampen-Sweep gefahren
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25/26) ASC byte-exakt live: p10 `…SC_23` 19 820 702 B, p11 `…SC_24` 21 454 162 B, p11 `…SC_23` 404 — verdrahtet `pioneer_doppler_compiler.rs:7-8,70`. Rampen-Sweep gefahren (`pioneer_ramp_sweep_probe.rs`): p10 2399 / p11 1518 Pässe (≥16 Samples), Median-LS-Steigung 2,257e-1 / 1,383e-1 Hz/s, LS-Resid-RMS-Median 8,845e2 / 7,958e3 Hz; der zweistufige Sweep konvergiert in allen Pässen auf die LS-Steigung (kein separater Ramp-Arm). Voyager-2-Route registriert (`blocked_sources.φ:51`: radio_science_rss = saturn_encounter_data + saturn_occultation_medium_band, kein Cruise). `…/MDA/` → 301 → NASA-DSN-Landing (`nasa.gov/communicating-with-missions/dsn/`), kein MDA-Modul-Dokument dort.
- **Blockade:** keine.
- **Braucht:** Sweep-Ergebnis in `probe-front-dark-matter.md` §5.6 nachtragen.

#### Trishuli — Bahrabise-Richtungsverifikation + S1/SAR-Footprint
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort (Post-Sentinel-1-Szene)
- **Lage:** (gemessen 2026-09-26) die richtungs-verifizierte Methode ist vorhanden und korrekt (`te.rs:96` TE(y→x); `te_pair_probe.rs:89/91` TE(a→b)=`transfer_entropy_lag(b,a)`; `cross_te_screen.rs:281/283` gleiche Konvention); Dokument `docs/paper/sturzflut-tibet-pfeil.md:364-377` + Header-sha `a6856341…`. Die Bahrabise-Nachmessung bleibt `pending` — Zeitreihen nicht geerntet (`data/` leer, `phi/meteo/tibet-flut-2026.json` trägt keine Bahrabise-Station, `trishuli_gauge_probe.rs` deckt nur 4913); der räumliche Footprint bleibt pending (CEMS nur Grading, optisch wolkenverdeckt, S1 nicht archiviert).
- **Blockade:** S1-Post-Szene nicht archiviert; Bahrabise-Zeitreihen nicht geerntet.
- **Braucht:** Bahrabise-Ernte (Pegel 113 + Regen, DHM-Route) → dann `te_pair_probe --a <stage> --b <rain>`.

#### Weberin — zweite unabhängige Positions-Linie je Körper-Klasse
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `grind-pro`) `pallas` (2000002) und `juno_asteroid` (2000003) sind in `src/archivar/kernels/naif_body_ids.tsv` ergänzt und je als SPK-Block `ephemeris_pallas.bin`/`ephemeris_juno_asteroid.bin` in `sources.φ` registriert (GM-Einträge gemessen vorhanden); die `encke`-Kometen-Zweitlinie (`horizons_compiler` × `dcom5_compiler`, `sources.φ:2884`) steht bereits. Offen: `cometels` als Weberin-Positions-Linie, die breite TNO-Kette (`mpcorb_extended`). Träger: `docs/concepts/die-weberin.md`.
- **Blockade:** keine.
- **Braucht:** `cometels` als zweite Kometen-Keplerlinie (Weave-Arm in `src/weberin.rs`); `mpcorb_extended` als TNO-Zweitlinie via `weberin_mpc_spk_verdict`.

#### Weberin Faden-Matrix — verbleibende No-Actor-Lücken + Broker-Positionen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26 via `research-max` ×4) die vier Teleskop-Routen sind hart gemessen: **Super-K** live (Zenodo 200 direkt, 8 Releases 2021–2026, cc-by) aber count-only, kein S²-Katalog → decline; **TA-Vollkatalog** absent (telescopearray.org/data 404; CDX 2721 URLs ohne Datenpfad; kein Zenodo-Katalog); **JUNO** absent (115-B-Stub = Redirect auf juno.ihep.cas.cn; kein Release; Nature-Erstphysik 2026-06-10, nur Resultat-Folge angekündigt); **LHAASO-Event** absent (Portal trägt nur table.csv; Survey-Event-Listen member-gated → `blocked account` datadc.ihep.ac.cn; öffentlich nur papierbezogene Bündel, darunter Crab-EventList.txt). Broker gemessen: ANTARES `api.antares.noirlab.edu/v1/loci` = 200, 2023595 B, Feldmap `data[].attributes.ra/dec` (JSON:API, Pagination page[limit]/[offset]); Lasair-ZTF `lasair-ztf.lsst.ac.uk/api/query` keyed live (`LASAIR_TOKEN`, ramean/decmean J2000), Lasair-LSST `api.lasair.lsst.ac.uk/api` blocked (Upstream 500, kein Key-Gap); Fink conesearch 200. Seismische Stations-Weltlinien und Broker-Positionen bleiben zu kompilieren. Träger: `docs/concepts/die-weberin.md`.
- **Blockade:** teils not-published (gemessen), teils `blocked account` (LHAASO-Kollaborationsmitgliedschaft).
- **Braucht:** die Broker-Positionen kompilieren; die seismischen Stations-Weltlinien registrieren; die vier bleiben absent, nie 0.0 (Super-K count-only; TA/JUNO/LHAASO not-published); JUNO-Wiedervorlage aufs erste Datenrelease (Trigger: Release-Ankündigung); der LHAASO-Crab-EventList-Bau bleibt pending (Compiler fehlt).

#### Weberin-Quellen — HAWC-TLS offen, Fink/ALeRCE live
- **Status:** offen | **Bindung:** eigen
- **Trigger:** das YR1-Zwischenzertifikat liegt in `OMEGAFLOW_CA_BUNDLE`
- **Lage:** (gemessen 2026-09-26) die CDN-Assets sind sha-verifiziert: `tao_wnd_zonal.csv` `b7719c89…` (`sources.φ:786`), `wwlln_th.csv` `06031403…` (`:8634`), `bpa_gic.csv` `8d0ceaa8…` (`:8646`). **HAWC-Beweis gemessen:** Bundle aus YR1-PEM (crt.sh-ID 21135538541) + Root-YR-X1-Cross-Signs + ISRG Root X1 → `archive_search --cacert <bundle> --verdict https://www.hawc-observatory.org` = **HTTP 200 (11161 B)**; `--sniff data.hawc-observatory.org/.../2HWC.yaml` = 200, 18588 B, sha `1c9566d447…`; nur-X1-Bundle reicht nicht. `OMEGAFLOW_CA_BUNDLE` = Pfad zu PEM (`src/archivar/fetch.rs:3-9`), CI-Repo-Secret in `.github/workflows/hawc-cdn.yml:19,34-37`; `archive_search` nutzt eigenen `--cacert`-Flag. Fink `api.ztf.fink-portal.org` kanonisch (lsst-Host verweist selbst dorthin), Feldmap gemessen. Träger: `docs/concepts/die-weberin.md`.
- **Blockade:** HAWC-TLS-Kette (YR1).
- **Braucht:** YR1-PEM+Root-YR-Cross-Sign als `OMEGAFLOW_CA_BUNDLE` setzen → HAWC fetchen; Fink/ALeRCE-Reader bauen.

#### Broken-Null-Control — Spec-Doc nachgezogen, CI-Lauf-Lesung offen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** der CI-Lauf (`ci-check` + `te-gate`)
- **Lage:** (gemessen 2026-09-26 via `grind-flash`) `docs/specs/broken-null-control.md` §6 auf den gemessenen Stand nachgezogen (fünf Gate-Tests benannt; Reverse-Stille nur bei τ=10 gegated, nicht τ=1; Window-Drift von „pending" auf closed); Header-sha `63b7e8b7…`. `te-gate` war dispatcht (`36194535114`, queued), gegen `origin/main` vor diesem Atom.
- **Blockade:** keine.
- **Braucht:** den `te-gate`-Lauf lesen.

#### ZNSP FORMNETWORK — `esp_zb_cfg_t`-Payload-Größe ungemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `zigbee-host`-Lauf (startet via `paths`-Trigger beim Push)
- **Lage:** (gemessen 2026-09-26 via `build`) die `sizeof`-Probe ist in `zigbee-host.yml` gebaut: ein Mini-Projekt `zb_sizeof_probe` (ESP-IDF v5.3.2, esp32s3) kompiliert `char zb_cfg_size[sizeof(esp_zb_cfg_t)]` gegen die Host-Referenz-Header (`esp-zigbee-sdk/examples/esp_zigbee_host/components/include/esp_zigbee_core.h`, SHA `c9e2c3c1…`), emittiert `SIZEOF_ESP_ZB_CFG_T` via `nm -S` in Log und Artifact `zb-cfg-size-esp32s3`. Der letzte Lauf `36194532489` (success) lief ohne Probe — die Zahl steht aus; `form_network_payload_pending()` liefert weiter `None`. Die ZC-Defaults sind aus dem SDK gemessen (`main/esp_zb_host.h`: role=COORDINATOR 0x0, install_code_policy=false, max_children=10).
- **Blockade:** keine — die Zahl kommt mit dem nächsten Lauf.
- **Braucht:** den nächsten Lauf lesen (`ci_manage view`/`ci_manage log` → `SIZEOF_ESP_ZB_CFG_T`), dann den FORMNETWORK-Encoder mit der gemessenen Größe setzen.

#### HRV/Puls→Strahlung — Kette nach dem Live-Lauf beobachten
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der BLE-Live-Lauf (Operator) hat `rr` geliefert
- **Lage:** (gemessen 2026-09-26 via `build`) die Kette ist gebaut: `hrv.rs:24-87` → `feed_beat_to_hrv` (`main_flow.rs:79-102`) → `tone_code` → `tone_scale` 0.25 (`omega.rs:1679-1687`) → `aperture` (`omega.rs:349`) → `Σω*aperture` (`actuators.rs:29`). Der perm-Log schreibt nur im Nicht-TE-Zweig (Eigenschaft, unverändert) und trägt jetzt 10 Spalten: `ring_gen,omega_sum,g,v_c,target,alpha,tone_code,tone_scale,aperture,field_permeability` (`omega.rs:1663-1676`); `tone_code` ist der u8-Code aus `hrv::tone_code` (0=`TONE_ABSENT`, 1=`TONE_CALM`, 2=`TONE_STRESSED`), `aperture` = `field_permeability×tone_scale` wie im `presence_frame`. Die Zeile entsteht vor der tone_scale-Relaxation desselben Ticks — die Zeile ist in sich konsistent, der Frame trägt die relaxierte Skala. `field_permeability` bleibt letzte Spalte: `perm_target_probe --live` (`cols[cols.len()-1]`, `tools/measure/src/bin/perm_target_probe.rs:315`) liest weiter den richtigen Wert. Test angepasst (`tests.rs:813-823`). Der frühere Braucht („tone_code→tone_scale→aperture im perm-Log lesen") ist damit baubar geworden.
- **Blockade:** hängt am BLE-Live-Fluss (Operator).
- **Braucht:** nach dem Live-Lauf die Spalten 7-10 (`tone_code,tone_scale,aperture,field_permeability`) im perm-Log lesen.

#### M2c — Rust-ZNSP-Host auf dem BL808 gegen das H2
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Ox64 angekommen
- **Lage:** (gemessen 2026-09-25) der Ox64 wird Host-CPU des Coordinators (ZNSP über UART), das H2 das Funkmodul; Ox64-UART-Pins UART0 GPIO14/15, UART1 GPIO16/17. Buildroot-Bring-up ungemessen.
- **Blockade:** Ox64 liegt beim Carrier.
- **Braucht:** Buildroot-Bring-up, dann derselbe Rust-ZNSP-Host auf dem BL808 gegen das H2.

#### BL808-eigenes 802.15.4-Radio — registrierter `pending`-Faden
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** öffentlicher 802.15.4-Treiber-/Stack-Fund für den BL808
- **Lage:** (gemessen 2026-09-26) `bouffalo_sdk_bl808` README-Raw ist jetzt live (210 Z., zuvor 404): Wireless-Matrix BL808 = WIFI4 ×, BT ×, BLE ×, **ZIGBEE ×**; Zephyr PR #112921 (ieee802154-Treiber) deckt BL61x/BL70x, **kein BL808**; PR #105580 (BL808 SoC) trägt kein 802.15.4. Verdikt: gemessenes `pending` (Abwesenheit als Matrix registrierbar, nicht mehr 404).
- **Blockade:** kein Treiber.
- **Braucht:** Zephyr-Folge-PR von will-tm auf BL808 beobachten; `bouffalo_sdk_bl808` auf lmac154-Zweig prüfen.

#### Galileo Borduhr-Sprung A/B — DESCANSO5 trägt keine Frequenzreihe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Fund einer öffentlichen Absolutfrequenz-Reduktion über 1995-11-30/12-01
- **Lage:** (gemessen 2026-09-26 via `vision`, OCR der gerenderten Seiten) DESCANSO5 `Descanso5--Galileo_new.pdf` (§2.6.3, §7.2) trägt **keine** USO-Absolutfrequenzreihe — nur qualitative Offsets („less than 5 Hz at S-band over a couple of days"; Einweg-Schwankungen 2001/2002 „tenths of a Hz"); Beobachtungsbereich 1989-12-05 … 2002. Die Grenze 1995-11-30/12-01 ist damit nicht kreuzbar → `pending`. Morabito-Reihe endet 1993; PDS ohne Jupiter-Phasen-RSS-Datenkopf.
- **Blockade:** keine Quelle trägt eine USO-Frequenzreihe über die Grenze.
- **Braucht:** eine andere öffentliche Absolut-Frequenzreduktion (DESCANSO-Begleitdokumente, USO-Aging-Reihe) finden; notfalls als absent halten.

#### JWST Biosignatur-Kanäle pending (O₂/O₃, Red-Edge, Saisonal)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine JWST-Detektion + Spektrum eines dieser Kanäle
- **Lage:** (gemessen 2026-09-26) gemessenes `pending`: O₂ 0.76 µm zu schwach für JWST-Transit, O₃ auf TRAPPIST-1e @3σ braucht >100 Transits (Schwieterman & Leung 2024; Seager 2025 `10.1073/pnas.2416188122`); Red-Edge nur Feasibility/Analog („In Search of the Edge" `10.3847/1538-4357/acaf59`); Saisonal = absent. Kein JWST-Signal mit Spektrum.
- **Blockade:** keine Quelle trägt eine Detektion.
- **Braucht:** bei Fund die Detektion + ihr Spektrum ins Register.

#### Planet-Nine/KBO — h-Sweep + Real-Kernel-Gegenprobe
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-26) SPK-Type-1-Reader gebaut (`bsp_reader/spk.rs`, `data_type 1` in `ephemeris.rs`). Der Typ-1-Kernel ist **nicht** Ceres/Vesta — alle generischen (`a_old_versions/ceres_1900_2100.bsp` etc., BIG-IEEE) und DAWN-Kernel (`sb_vesta_grv_221108.bsp` etc., LTL) tragen `data_type 2` (Chebyshev), gemessen via DAF-Summary. Typ-1 trägt der Voyager-2-merged-Kernel `Voyager_2.m05016u.merged.bsp` (6 447 104 B, LTL-IEEE, 11 Typ-1-Segmente, `naif.jpl.nasa.gov/pub/naif/VOYAGER/kernels/spk/`, nach `data/naif.jpl.nasa.gov/`). Real-Gegenprobe gefahren (neuer Bin `tools/measure/src/bin/spk_type1_check.rs`): Neptun-Flyby 1989-08-25 → 30,19 AU, 2026 → 141,7 AU — physikalisch korrekt (PASS).
- **Blockade:** keine.
- **Braucht:** `docs/paper/planet-nine-kbo-residue.md` §5(iv) von „Reader nicht implementiert" auf „implementiert + getestet" nachziehen.

#### Das eine Instrument — Anomalie offen (zweite Augenklasse fehlt)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** ein zweiter Messkanal (VLBI-Beacon auf einer interstellaren Sonde) existiert
- **Lage:** (gemessen 2026-09-25) die Pionier-Anomalie ist unter dem einen Instrument nicht entscheidbar; eine zweite Augenklasse (Winkel aus VLBI + Geschwindigkeit aus Doppler) fehlt.
- **Blockade:** kein Instrument misst den vollen Phasenraum der Pioniere.
- **Braucht:** eine zweite Augenklasse — VLBI-Beacon auf der nächsten interstellaren Sonde, von Tag eins zweikanalig.

### Operator handelt

#### BLE-HR-Live-Messung FR945
- **Status (Vorbereitung):** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) der GFDI-Bus-Abriss ist geheilt und CI-bestätigt: alle `archivar::ble::tests` grün auf `45b4b2eb1`; Fix `src/archivar/ble.rs:1360/1418`; verdeckter Lauf-Command steht (`OMEGAFLOW_BLE_HR=… OMEGAFLOW_HIDDEN=1 ./target/debug/omegaflow`, ~45 s).
- **Blockade:** keine.
- **Braucht:** die Kantenzeile bereithalten.
- **Status (Akt):** operator-gebunden (Hardware/Radio) | **Bindung:** operator
- **Trigger:** Operator startet den verdeckten Lauf am gekoppelten Gerät (945 am Arm)
- **Braucht:** `OMEGAFLOW_BLE_HR=<FR945-MAC aus .secrets.local> OMEGAFLOW_HIDDEN=1 ./target/debug/omegaflow`; `sensor:`-/`gfdi_line`-Zeilen lesen.

#### FIT-Verifikation eigene FR945-Datei (lokal-only)
- **Status (Vorbereitung):** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) der Dump-Pfad ist gebaut (`src/main.rs:1-56`: `OMEGAFLOW_FIT_SAMPLE` → `parse_fit`, druckt records/nn/min/max/all-finite, `exit(2)` bei Refusal), am SDK-Sample verifiziert.
- **Blockade:** keine.
- **Braucht:** die Kantenzeile bereithalten.
- **Status (Akt):** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator startet den Lauf mit seiner Datei
- **Braucht:** `OMEGAFLOW_FIT_SAMPLE=/abs/pfad/FR945.fit cargo run -p omegaflow --bin omegaflow`.
- **Wort:** „meine fits datei verlässt niemals dieses gerät" | 2026-09-25 | Operator (Session)
- **Wort:** „die 945 von anderer Hardware trennen; meine Daten bleiben lokal" | 2026-09-25 | Operator (Session)

#### Onboard-/CIQ-Bedarf benennen
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort
- **Lage:** (gemessen 2026-09-24) der Host-Reader `parse_fit` (`fit.rs:78`) ist verdrahtet; CIQ hat keinen Reader (`sgrep -i ciq src tools` leer). Ein-Quellen-Regel: solange FIT läuft, sind BLE/Serial über die Arbitrierung ausgeschlossen.
- **Blockade:** keine — bewusst `pending` (Ein-Quellen-Regel).
- **Braucht:** Bedarf Ja/Nein — Nein → der Punkt ist released, `FIT_DIR` bleibt der FIT-Kanal.

#### Beat-Arbitrierung — verdeckter Lauf mit zwei Beat-Quellen
- **Status (Vorbereitung):** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) die Spawn-Arbitrierung steht (`main_flow.rs:504`); die reine Funktion ist CI-getestet; der verdeckte Lauf-Command steht.
- **Blockade:** keine.
- **Braucht:** die Kantenzeile bereithalten.
- **Status (Akt):** operator-gebunden (hidden) | **Bindung:** operator
- **Trigger:** Operator startet einen verdeckten Lauf mit zwei gesetzten Beat-Quellen
- **Braucht:** `OMEGAFLOW_HIDDEN=1`-Lauf mit zwei Quellen, der genau eine `beat source:`-Zeile zeigt.

#### DEMETER/CDPP-Order-Flow
- **Status (Vorbereitung):** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) die Order-Portale antworten (je 200); CDPP-Login funktioniert (`REGISTERED_USER`); die Registerzeile ist nachgezogen (DONE_WITH_WARNING, 96978 in Vermerk, 0 verfügbar, Download HTTP 500).
- **Blockade:** Dateifehler-Ursache hinter der REGARDS-API (kein Report-Endpoint).
- **Braucht:** die 96 978 Dateien über die REGARDS-UI lesbar machen.
- **Status (Akt):** operator-gebunden (Zugang) | **Bindung:** operator
- **Trigger:** Operator-Browser liest die Dateifehlerliste
- **Braucht:** ein Neu-/Nach-Order ist konsenspflichtiger Dritt-Akt — Operator-Wort.

#### Live-Sensor-Cluster (eigener Knoten)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-25) Firmware Mux-Sweep + SpO2/GNSS-`sensor_config` gebaut; `docs/specs/mantis-shrimp-bom.md` trägt ATGM336H GNSS `1005009361234427`, 3,01 CHF ≈ 3,20 €; die H2-Zeile `ESP32-H2-DevKitM-1-N4` (`1005008131868631` ≈ 6,25 $ unverified · DigiKey 26282483 9,68 $).
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** Operator-Wort (LOCK-Aufhebung), dann BOM bestellen (inkl. H2).

#### ESP32-Puls-Knoten (Träger ohne Uhr)
- **Status:** LOCK | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator-Wort hebt das LOCK auf
- **Lage:** (gemessen 2026-09-25) Code-Eingang `BeatSource::Serial` steht; dedizierter Puls-Knoten unbestellt.
- **Blockade:** Hardware-Bestellung LOCK.
- **Braucht:** —
- **Wort:** ESP32 separat als eigener LOCK | 2026-09-25 | Operator (Session)

#### Weberin-Quellen — account-/key-blockierte Rohkanäle
- **Status (Vorbereitung):** eigen | **Bindung:** eigen
- **Trigger:** sofort
- **Lage:** (gemessen 2026-09-25) die offenen Zwillingsrouten stehen; IGETS (SFTP-Account), vDEC (Antrag + Vertrag), ONC-Token, TNS-Key (`TNS_API_KEY`/`TNS_UA`) und SuperDARN-Globus sind identifiziert.
- **Blockade:** Konto/Key fehlt.
- **Braucht:** die Anträge/Endpunkte je Konto bereitlegen.
- **Status (Akt):** operator-gebunden (Zugang) | **Bindung:** operator
- **Trigger:** Operator-Wort je Konto
- **Braucht:** Operator entscheidet je Konto (IGETS `igets-support@gfz.de`, vDEC-Antrag, ONC-Token, TNS-Key).

### Extern handelt (Dritte)

#### JUICE-Erdpassage 28./29.09.2026 — Kanal + In-situ-Messung
- **Status:** termin:2026-09-29 | **Bindung:** termin
- **Trigger:** 28./29.09.2026
- **Lage:** (gemessen 2026-09-25) der Prädiktionskanal steht, alle Zellen pending; der Operator-Siegel-Wort liegt noch nicht vor.
- **Blockade:** der Operator muss das Siegel-Wort vor dem 28.09. setzen.
- **Braucht:** vor dem Flyby das Siegel-Wort setzen; nach dem Flyby die JUICE-In-situ-Feldmessung gegen den präregistrierten Feldzustand vergleichen (σ-Metrik gegen fam).

#### Ox64-Lieferung
- **Status:** wartend | **Bindung:** termin (Carrier)
- **Trigger:** Ankunft (`LZ473049629CN`)
- **Lage:** (gemessen 2026-09-25 via `state/mail/mail_ledger.φ`, Mail `1790046330`) PINE64 versandte zwei Ox64 — Ankunft offen.
- **Blockade:** Carrier.
- **Braucht:** Ankunft quittieren; dann M2c (BL808-Host-Port).

#### Postfach
- **Status:** wartend | **Bindung:** extern (Mail)
- **Trigger:** neuer Eingang
- **Lage:** (gemessen 2026-09-26 via `sread state/mail/mail_ledger.φ`, 158 Zeilen) letzte Eingänge: `1790373942` (GitHub-Support-Survey zum PII-Purge-Ticket 4761482 — fremde Linie), `1790370971` (Exa-Werbung), `1790364852` (OpenAlex „Request received" ID #24656). Kein Sensory-Treffer (relay/sensor/beat/mantis/ble/hrv/zigbee = 0).
- **Blockade:** keine.
- **Braucht:** `smail_recv` bzw. `state/mail/mail_ledger.φ` bei Trigger.

#### BGR-Matched-Filter-Ankunft (Tonga) — account-blockiert
- **Status:** wartend | **Bindung:** dritter (vDEC)
- **Trigger:** vDEC-Zugang gewährt
- **Lage:** (gemessen 2026-09-25) die PMCC-Detektionsliste läuft auf einem ~300-s-Raster; die Rohwellenform ist vDEC-account-blockiert.
- **Blockade:** vDEC-Account.
- **Braucht:** nach Zugangsgewährung den matched-filter-Arrival messen.

#### DSN-Briefe in Flug (Voyager/Mariner 10/Viking, Cassini, Juno)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Antwort der DSN
- **Lage:** (gemessen 2026-09-25) fünf DSN-Briefe sind in Flug; der Auftrag `docs/auftrag/archiv/auftrag-sonden-rohdaten-anfrage.md` wurde nach `docs/auftrag/archiv/` verschoben.
- **Blockade:** Antwort ausstehend.
- **Braucht:** die Antwort quittieren.

#### Gaia DR4 + Europa-Clipper-Erdpassage
- **Status:** termin:2026-12-02 | **Bindung:** termin
- **Trigger:** 2.12.2026 (Gaia DR4) / 3.12.2026 (Europa Clipper)
- **Lage:** (gemessen 2026-09-25) Gaia DR4 für den 2.12.2026 angekündigt (Epochen-Astrometrie); die Europa-Clipper-Erdpassage folgt am 3.12.2026.
- **Blockade:** Termin.
- **Braucht:** am jeweiligen Datum die Epochen-Astrometrie bzw. die EC-Magnetfeld-Messung ernten.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
