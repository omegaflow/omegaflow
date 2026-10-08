<!--
  title: Handover — Mountain-Folge 273 (2026-10-08)
  session: Mountain-Folge 273
  class: handover
  date: 2026-10-08
  sha256: 7d1fa4d6d4db43d7a7714b7a1724e289482bafca3928029479b03bd4bc71e88e
  status: live
-->
# Handover — Mountain-Folge 273 (2026-10-08)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-08-mountain-folge272.md` (→ `archiv/`).
Kein pro/max. Gefaltet: die adressierten Blöcke `## An mountain` (mycelium-268,
river-134).

**273-Arbeit (dieses Atom, Operator-Wort „bitte kümmer dich drum" aus 272):** die
GIC-Register-Admission gearbeitet.
- **3 GIC-Arme registriert** (hand-authored, gegen Bin+Workflow gemessen, nicht
  fabriziert): `ssusi_aurora` (`sources.φ` + `harvest.φ`), `superdarn_cpcp`
  (`zenodo.org/superdarn_cpcp.bin`, `pot.drop` kV, `electric`),
  `emtf_impedance` (`data.earthscope.org/emtf_usarray_cao01_2010.bin`, 8
  Z-Komponenten SI-Ω, `em`). `register_sort phi/sources.φ --write` → canonical
  (2691 Blöcke, zusätzlich eine vorbestehende `url-order`-Violation
  usgs_comcat/hadisst geheilt); `harvest_reg --check` grün; `cargo check` 0/0.
- **cors-Riss (gemessen):** `cors-cdn.yml:61` ruft `cors_rinex_compiler`, **nicht**
  `cors_compiler`; kein Workflow ruft `cors_compiler`; die CRX1-Form hat genau
  einen Reader-Arm `cors_rinex` (`extract.rs:45`), bereits registriert
  (`sources.φ:10586`). Eine `cors_compiler`-Zeile wäre ein ungewirerter
  Duplikat-Url → **nicht** geschrieben; Riss an river (unten).
- **river-134 `cgm_lat` CPL/TTB gegenstandslos (gemessen):** `cgm_lat 11.23`
  (`sources.φ:6328` CPL) + `cgm_lat -2.62` (`:7673` TTB) stehen; `cgm_source
  bgs-quasi-dipole` je daneben. Stale Behauptung.
- **`blocked_sources.φ`-Aufräumen (mycelium-268, Taucher):** 3 redundante
  `descoped` (nssdc.ac.cn, titanNotebook, ioc-v1) → `declined_sources.φ`
  `decline superseded-by-integrated`; Madrigal-`gap` `html-parser-arm` →
  `openmadrigal-api`; LEOS (`:100`) + Gaia `cluster_ka` (`:120`) von `descoped`
  auf `pending` re-registriert (Routen re-gemessen HTTP 206/200), TUH + NSRR auf
  `blocked account`. Klassen a/b/c/d-Mitgliederlisten liegen **nicht** im Baum →
  benannt `pending`, nicht rekonstruiert.
- **Riss (nicht geglättet):** die 272-Behauptung „Drafts aus 272: `format
  superdarn_cpcp`/`emtf_impedance`/`ssusi_aurora`" trägt nicht — CPCP/EMTF-Bins
  drucken **keine** Registerzeilen (nur SSUSI); die Blöcke wurden daher aus
  Bin-Layout + Workflow-Ausgaben hand-authored. Der `omegaflow-harvest`-Build ist
  **rot** (E0061 `ecef_to_geodetic`, 6 Bins — river).

## Burn: open 0.0015 · close 0.20 · cap 0.50 — Grund: line + 3 flash-Dispatches (Register-Arme, blocked_sources, CPCP/EMTF); kein pro/max.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–273)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–273)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„bitte kümmer dich drum" — span/Agnostik, em-nmgy, Flyby-Kette, GIC-Stufe-2 durch Recherche + Rat + UI-Runde arbeiten; LOCK unberührt; keine neue Operator-Frage | 2026-10-08 | Operator (Session, Mountain 272)
„berufe den rat ein aber mache dafür eine archive search recherche und befrage du frontier uis" | 2026-10-08 | Operator (Session, Mountain 273)
„hast du die tyrigopen auch befragt und die antworten gewichtet?" — tryingopen einbeziehen + gewichten | 2026-10-08 | Operator (Session, Mountain 273)

## Offen (aufgeschlüsselt)

### GIC-Faden §A–G — neue Arme registriert; cors-Riss offen
- **Status:** eigen | **Bindung:** eigen (Register) · river (cors)
- **Trigger:** Reachability-Datei ändert sich
- **Lage:** (gemessen 2026-10-08 273) SSUSI/CPCP/EMTF registriert (s. o.). Offen
  bleiben OMTI/Abisko (Keogramm PNG), Substorm-Onsets, DLR ROTI, Kellerman/Zenodo
  4444068, USGS E-Feld, Wind SWE (`WI_H0_SWE`, HAPI-ID ungemessen). cors-Riss
  benannt (river-134: cors_compiler ungewirert).
- **Blockade:** cors: kein Workflow ruft `cors_compiler`; Wind SWE: HAPI-ID
  ungemessen.
- **Braucht:** river entscheidet cors_compiler vs. cors_rinex (bestehende Zeile);
  Wind SWE HAPI-ID messen (`cdaweb.gsfc.nasa.gov/hapi/info?id=WI_H0_SWE`, Deckung
  endet 2001-05).

### GIC-Stufe-2 — Route C Member-Pool (Rat); dB/dt-Bestand 2/154
- **Status:** eigen | **Bindung:** eigen (Register) · river (`compute_max_t`)
- **Trigger:** AE/AL/AU · SME/SML/SMU als eigene `sources.φ`-Zeilen admiert
- **Lage:** (gemessen 2026-10-08 272) Rat-Verdikt: Route C, Stufe-2-Member-Pool des
  `compute_max_t` IST der Träger (`docs/blatt/blatt-gic-breitenband-familien.md:243-247`);
  `Linie 1` (drei getrennte Familien-Deskriptoren) bleibt ungeglättete Gegenlinie.
  **dB/dt-Bestand: 2 von 154** (ABK `sources.φ:2150-2168`, SOD `:2170-2178`);
  152 tragen `intermagnet_xyz`. Beide 1h-Assets `gate-no-field-lines`-**refused**
  (`phi/pipeline/refusal_ledger.φ:75-76`).
- **Blockade:** Member-Pool ohne vollen dB/dt-Bestand nicht wirebar; der
  refused-Zustand der 2 Arme ungelöst; AE/AL/AU + SME/SML/SMU nicht registriert.
- **Braucht:** AE/AL/AU (OMNI-HAPI) + SME/SML/SMU (SuperMAG-Index-Route) messen +
  als `sources.φ`-Zeilen admiten; `gate-no-field-lines`-Refusal der 2 dB/dt-Arme
  lösen oder `blockiert` registrieren; dann River Stufe 2 am `compute_max_t`.

### em-nmgy / Bandreferenz — Parser-Arm GEBAUT; `band_id`-Persistenz + Heim offen
- **Status:** eigen | **Bindung:** eigen · river (Feld-Erweiterung)
- **Trigger:** river-135 committet (Fremd-Dirty weg) → `band_id`-Feld
- **Lage:** (gemessen 2026-10-08 273, HEAD `dff6ccfda`) **Parser-Arm gebaut:**
  `quantity`-Arm (`parse.rs:1037-1139`) parst Tail
  `band <id> pivot <λ><unit> [edges <λmin>-<λmax><unit>]` → `freq = c/λ_m`,
  `bin_width = |c/λmin − c/λmax|`; Erzwingung `QuantityKind::Scale`+`nmgy` ohne Band
  → Anomalie + Record verweigert (nie `0.0`); Helfer + `#[cfg(test)]`-Test
  (`parse.rs:2017-`, `:2436-`). Register `sources.φ:19650` trägt jetzt
  `… scale nmgy 31536000 0.0 0.0 band DECam_g pivot 4808.49angstrom edges 3900-5600angstrom`
  (kind bleibt `scale` — kein `em`-Kind im Code, `force.rs:18-36`). Gate-Fixture
  `commit_gate_vocab.json:100`. `register_sort` canonical; `cargo check` 0/0.
  **Rat-Verdikt (2026-10-08, Synthese A+C; Recherche + 10 UI-Seats + tryingopen):**
  A Syntax (Werte an der Zeile), C Semantik; B/D verworfen; gewichtet Frontier 4/4 C,
  GLM 5.3 (753B) C, DeepSeek V4 Pro (1.7T) A (Gegenlinie). Rohantworten
  `state/stimmen/2026-10-08_ui-runde_bandreferenz.md`.
- **Blockade:** `band_id` als Feld in `FieldConfig` persistiert noch **nicht** — ein
  neues Pflichtfeld bräche die Literale in den fremd-dirty Dateien
  `main_flow.rs:1449`, `relay.rs:668/729`, `channels.rs:1408/1441` (river-135
  uncommittet). Die Klausel-`<id>` (`DECam_g`) wird geparst/validiert, aber nicht
  gespeichert. Der Test-Lauf ist CI-pending (lokal `cargo test` verweigert).
- **Braucht:** nach river-135-Commit das Feld `band_id: Option<String>` in
  `FieldConfig` ergänzen (eine Zeile je Literal) + im `quantity`-Arm setzen;
  CI-Test grün. **Riss:** Band-Heim offen (lokal `phi/bands.φ` vs. SVO FPS) — der
  `<id>`-Home ist damit noch nicht gebunden; SVO-FPS-id + pivot-λ am Harvest messen.

### span-Apertur / Membran-Agnostik — Mountain hält `receiver.span`
- **Status:** eigen (Datenkontrakt) | **Bindung:** eigen · river (ω()-Lauf)
- **Trigger:** River nennt die Signatur / baut den Reader
- **Lage:** (gemessen 2026-10-08 272) Rat: Apertur IST Record-`extent`; `span` ist
  deklarierter Receiver-Override **ohne Leser** (`parse.rs:255-266`, `types.rs:445`);
  Start-Anker aus Wire-`extent` (`static/membrane.html:568-596`). Q1-Gate über jeden
  Body (Cache wie Fetch); River zog den Cache-Zweig durchs Gate (`c71313d7e`).
- **Blockade:** `span` hat keinen Leser; Receiver-Pflicht-Signatur nicht gebaut.
- **Braucht:** Mountain hält `receiver.span`-Direktiv + schreibt den Receiver-Satz
  (Abwesenheit der Weltlinie → Record verweigert) in
  `docs/concepts/archivar-mathematikerin.md`; River liest `receiver.span` in der
  Query-Apertur (`wasm.rs:65-99` + `membrane.html:501-528`).

### Flyby-Kette — Kanäle registriert; Doppler `pending`
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium
- **Trigger:** DSN/ESTRACK-Residualroute admiert oder Descope-Befund
- **Lage:** (gemessen 2026-10-08 273) `SW_FAST_MAGA_LR_1B` registriert
  (`sources.φ:8153`, VirES-HAPI) — future-199 bestätigt. RTSW/ACE/Kp/OMNI2/Swarm/DSN-Power/
  JUICE-Ephemeride registriert. Nicht registriert: Doppler (keine DSN/ESTRACK-Residualroute,
  `src/archivar/doppler.rs` absent), σ_recon (`data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin` absent).
- **Blockade:** keine ESTRACK/DSN-Residualquelle.
- **Braucht:** ESTRACK-Route + `doppler.rs` bauen **oder** Descope-Befund; sonst
  `pending` mit Trigger (Rat).

### IGRF-Koeffizienten-Arm (`geomag_lat`)
- **Status:** eigen | **Bindung:** mycelium (`ci-check`-Verdikt)
- **Trigger:** CI-Test `synthesis_matches_pyigrf14_witness_points` grün
- **Lage:** (gemessen 2026-10-07 271) Grad-13-Synthese gebaut, `igrf.rs` formatiert;
  Witness-Test läuft nur in CI. `ci-check` wird als `cancelled` mit 0 Jobs verdrängt.
- **Blockade:** kein nicht-cancelled `ci-check`-Lauf.
- **Braucht:** ein nicht-cancelled `ci-check`-Lauf (Test grün).

### Lizenz-Disposition — `terms`-Feld (SPDX); `rights_read` offen
- **Status:** eigen | **Bindung:** eigen (Format/Datenkontrakt)
- **Trigger:** `rights`-Parse-Arm steht
- **Lage:** (gemessen 2026-10-07 271) `rights`-Parse-Arm gebaut (`87aed0b14`);
  Riss: Baum-`terms`-Zahl vs. Census (2246 no-terms).
- **Blockade:** je-Quelle-`terms`/`rights`-Zeilen sind ein Sweep.
- **Braucht:** `rights`-Register-Zeilen schreiben (DataCite-Triple, SPDX,
  `NOASSERTION`/`NONE`); `license_census`/`ci-gate` nachführen.

### Bias-Tor (`docs/auftrag/auftrag-bias-tilgung.md`)
- **Status:** eigen | **Bindung:** eigen (Gate/Fixture)
- **Trigger:** Ersatzmuster (`unwrap_or`/Default-Fill) in Produktion gemessen
- **Lage:** (gemessen 2026-10-07 270) Inventar
  `state/future/giftkarte-klassifiziert-src-2026-10-07.md` existiert (gitignored),
  veraltet; gemessene Ersatzmuster `astrometry.rs:10,336`, `odp.rs:9,48`.
- **Blockade:** welches Muster Hart-Block (Fixture) vs. Review = Rat.
- **Braucht:** Inventar auf den heutigen Baum nachführen; Fixture-Kandidaten in
  `commit_gate_vocab.json` prüfen.

### HadISST SST — SOURCE_PORT gebaut, CI-Lauf offen
- **Status:** eigen | **Bindung:** mycelium (CDN)
- **Trigger:** `hadisst-cdn.yml`-Lauf grün
- **Lage:** (gemessen 2026-10-07 271) Compiler/Arm/Register/Workflow stehen,
  `cargo check` 0/0; Workflow noch nicht auf `origin/main` gelaufen.
- **Blockade:** Workflow nach Push dispatcht (79 MB Fetch).
- **Braucht:** `hadisst-cdn.yml` dispatchen; bei Erfolg `sha256`-Zeile nachziehen.

### `blocked_sources.φ`-Aufräumen (mycelium-268) — Teil geleistet
- **Status:** eigen | **Bindung:** eigen (Disposition)
- **Trigger:** Diver-Klassentabelle (a/b/c/d) liegt im Baum
- **Lage:** (gemessen 2026-10-08 273) 3 redundante `descoped` umgezogen; Madrigal-`gap`
  korrigiert; LEOS/Gaia/TUH/NSRR re-registriert. **Riss:** die per-Eintrag-Klassenlisten
  liegen in keinem Baum (nur Aggregat-Zahlen a/b/c/d = 11/9/9/10) → Rest `pending`.
- **Blockade:** Klassen-Mitgliedschaft nicht im Baum; die numerischen ids 30/31/34/… 
  sind in keiner Register-Revision auflösbar.
- **Braucht:** Diver-Tabelle (mycelium) ins Handover oder als Datei; dann Klasse
  a→`ledger.φ` `ausstehend`, b→`blocked account`/`key`, c→`parser-def`+`gap`,
  d-Stale-Schutz. `descoped-check` auf `blocked parser-def` ausdehnen (Gate-Fixture).
- **Träger (Klassen-Punkt):** `phi/blocked_sources.φ::gap:openmadrigal-api ×1` (Madrigal
  `blocked parser-def`, OpenMadrigal-API-Arm fehlt).

## An river

Origin: mountain-folge273.

- **cors-Riss (gemessen 2026-10-08 273):** `river-134` bat um eine
  `cors_compiler`-Registerzeile. Der Baum trägt keinen Workflow, der
  `cors_compiler` ruft — `cors-cdn.yml:61` ruft `cors_rinex_compiler`; die
  CRX1-Form hat genau einen Reader-Arm `cors_rinex` (`src/archivar/extract.rs:45`),
  bereits registriert (`phi/sources.φ:10586`, compiler `cors_rinex_compiler`).
  Die `at earth`-Zeile für `cors_rinex` steht. **Braucht:** Entscheidung, ob
  `cors_compiler` ein eigener Arm/Workflow wird oder entfällt; sonst keine
  Registerzeile.
- **ROTER Harvest-Build (Riss, gemessen 2026-10-08):** `ecef_to_geodetic(x,y,z,a,e2)`
  (`src/archivar/rinex.rs:4`) — 6 Harvest-Bins rufen 3-arg
  (`cses_scm_compiler.rs:166`, `cses_hpm_compiler.rs:161`, `cses_efd_compiler.rs:324`,
  `cors_compiler.rs:93`, `cors_rinex_compiler.rs:86`, `champ_plpt_compiler.rs:74`);
  `cargo check -p omegaflow-harvest` → E0061. **Braucht:** Body-Ellipsoid-Plumbing
  (`BodyProperties` → `a`/`e2`) an die Bins oder rückwärts-kompatible Signatur; ein
  WGS84-Literal ist per `commit_gate_vocab.json:87` verboten.
- **Loadergate (Q1)** und **`span`-Apertur (Q2)** unverändert offen (s. Offen);
  Mountain hält `receiver.span`, River liest es.

## An mycelium

Origin: mountain-folge273.

- **`ci-check` verdrängt jeden Lauf (Riss, gemessen 2026-10-07):**
  `.github/workflows/ci-check.yml:20-27` `cancel-in-progress: false`, gemessen jeder
  Lauf `cancelled` 0 Jobs. Fix nötig, sonst kein Per-SHA-Verdikt (IGRF-Witness).
- **3 neue Harvest-Arme (272) registriert (273):** `superdarn_cpcp`/`emtf_impedance`/
  `ssusi_aurora` stehen in `sources.φ`+`harvest.φ`; Workflows + Dispatches aus 272
  (`superdarn-cpcp-cdn 37755349709` · `emtf-cdn 37755354486` · `ssusi-cdn 37755359472`)
  → nach der Register-Admission Asset im CDN prüfen. **Blockade:** der
  `omegaflow-harvest`-Build ist rot (river, s. `## An river`).
- **`hadisst-cdn.yml`:** nach dem Push dispatchen.
- **GIC-Stufe-2 (Q5):** Member-Pool als Register-Klassenträger zulässig, als ein
  Wire-Deskriptor verboten (Rat); River verdrahtet drei getrennte Deskriptoren.
- **Doppler-Kanal (Q4):** bleibt `pending` mit Trigger (`route erscheint`).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern
  (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI.
  Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün;
  offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und
Push (stehendes Wort 2026-10-07, Mountain 264).

Eigene Pfade: `phi/sources.φ`, `phi/harvest.φ`, `phi/blocked_sources.φ`,
`phi/declined_sources.φ`,
`docs/handover/handover-2026-10-08-mountain-folge273.md`,
`docs/handover/archiv/handover-2026-10-08-mountain-folge272.md` (Move).
