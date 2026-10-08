<!--
  title: Handover — Mountain-Folge 274 (2026-10-08)
  session: Mountain-Folge 274
  class: handover
  date: 2026-10-08
  sha256: 4ac4048c0eaa3b1971a738f2b1dc7ae5a1d663ecaf068a416f4853e7ac851371
  status: live
-->
# Handover — Mountain-Folge 274 (2026-10-08)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
gemessen 2026-10-08T14:3xZ). Diese Session konsumierte
`handover-2026-10-08-mountain-folge273.md` (→ `archiv/`). Kein pro/max. Gefaltet:
die adressierten Blöcke `## An mountain` (future-folge201, mycelium-folge269,
river-folge135).

**274-Arbeit (dieses Atom):**

- **KC2G-Parser-Schema-Drift geheilt (mycelium-269 §An mountain, gemessen):**
  `tools/harvest/src/bin/kc2g_stations.rs:42-45` las `station.lat`/`station.lon`;
  die Live-API `prop.kc2g.com/api/stations.json` trägt `station.latitude`/
  `station.longitude` als **String**, Longitude east-positive 0–360 (z. B.
  `"30.4"`/`"262.3"`). Fix: die zwei Schlüssel + Longitude-Normalisierung
  `((lon+180).rem_euclid(360))-180`; Fixture + `#[cfg(test)]` an die reale Form
  angeglichen. Gemessener Lauf gegen die Live-JSON: **101 Stationen** (vorher 0),
  `target/debug/kc2g_stations` → `/tmp/opencode/kc2g_out.csv`. `time` ist
  ISO-8601 → `time_unix` absent (0 honored, nie fabriziert). Mycelium:
  CDN-Arm nach dispatch prüfen (Workflow `kc2g-*`).
- **INTERMAGNET-HAPI-Route korrigiert (river-135 §An mountain, gemessen):** die
  Route war `…/hapi/data?id=ABK/best-avail/PT1M/xyzf&format=json` → HTTP 400.
  Korrekte Form: id **lowercase**, `time.min`/`time.max` **Pflicht**:
  `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=<code-lower>/best-avail/PT1M/xyzf&time.min=<iso>&time.max=<iso>&format=json`
  → **200** (`abk`). `info?id=<code-lower>/best-avail/PT1M/xyzf` → 200;
  `catalog` → 200 (IDs `<code>/<best-avail|definitive|quasi-definitive|provisional>/<PT1M|PT1S>/<native|xyzf|hdzf|diff>`).
  Der 400 lag an **fehlenden Zeitgrenzen** (HAPI 3.1 `data` verlangt sie), nicht
  am Format. Der Parameter ist `Field_Vector` (Größe 3, nT, `fill` 99999.0).
- **`register`-Job `url-order` bereits kanonisch (Messung):**
  `cargo run -p omegaflow-utils --bin register_sort -- phi/sources.φ` →
  `is canonical (ttl asc, url asc, no exact duplicates) across 2691 blocks`. Der
  river-135-Block (`usgs_comcat` nach `hadisst`) war gegen die vor-273-
  Registerrevision gemessen; **273 hatte ihn schon geheilt**. Keine Aktion.
- **`dropped-gate` — Baseline nachgezogen (gemessen):**
  `register_lookup --dropped-keys` (61 Keys) gegen
  `docs/zustand/dropped-legacy-baseline.txt`; zwei neue Keys sind umbenannte,
  weitergetragene Punkte (kein echter Drop): `bandreferenz em-nmgy-riss reinem
  scale statt` und `body cache fetch gebaut jeden loader-gate q1 uncommittet wie
  zulassung über`. In die NAMENS-Basis übernommen (Präzedenz Mycelium-266); der
  Riss (Key-Drift bei umbenanntem Heading) bleibt benannt.

**GIC-Messungen (274, gemessen):**

- **Wind SWE `WI_H0_SWE`** lebt: `cdaweb.gsfc.nasa.gov/hapi/info?id=WI_H0_SWE` →
  200 (SWE-Elektronen-Parameter `Te`/`average_energy`/…). Admission
  (Receiver/force) offen.
- **AE/AL/AU** liegen in `OMNI2_H0_MRG1HR` als `AE1800`/`AL_INDEX1800`/
  `AU_INDEX1800` (Parameterliste gemessen, ebenso `KP1800`/`DST1800`); eigene
  `sources.φ`-Zeile braucht Receiver/force (Rat), nicht an den `at sun`-OMNI-Block
  hängen.
- **SuperMAG-Indizes** `supermag.jhuapl.edu/indices/` → 200 (65223 B);
  Service-Route `/services/` → 206/1 B (user-gated). Index-Route (`SME/SML/SMU`)
  noch nicht als Quelle gemessen.

## Burn: open 0.0000 · close 0.0545 · cap 0.50 — Grund: line + 2 targeted Single-Bin-Builds (kc2g_stations, register_sort) + 1 flash-Dispatch-Lauf; kein pro/max. (gemessen `session_burn`, Session „Mountain-Linie in einem Pass starten")

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–274)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–274)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„bitte kümmer dich drum" — span/Agnostik, em-nmgy, Flyby-Kette, GIC-Stufe-2 durch Recherche + Rat + UI-Runde arbeiten; LOCK unberührt; keine neue Operator-Frage | 2026-10-08 | Operator (Session, Mountain 272)
„berufe den rat ein aber mache dafür eine archive search recherche und befrage du frontier uis" | 2026-10-08 | Operator (Session, Mountain 273)
„hast du die tyrigopen auch befragt und die antworten gewichtet?" — tryingopen einbeziehen + gewichten | 2026-10-08 | Operator (Session, Mountain 273)

## Offen (aufgeschlüsselt)

### GIC-Faden §A–G — neue Arme registriert; cors-Riss offen
- **Status:** eigen | **Bindung:** eigen (Register) · river (cors)
- **Trigger:** Reachability-Datei ändert sich
- **Lage:** (gemessen 2026-10-08 274) SSUSI/CPCP/EMTF registriert (273).
  **NEU:** INTERMAGNET-HAPI-Form korrigiert + Wind-SWE-HAPI-ID gemessen (s. o.).
  Offen bleiben OMTI/Abisko (Keogramm PNG), Substorm-Onsets, DLR ROTI,
  Kellerman/Zenodo 4444068, USGS E-Feld, Wind SWE (`WI_H0_SWE`, ID jetzt
  gemessen: 200; Admission offen). cors-Riss benannt (river-134: `cors_compiler`
  ungewirert).
- **Blockade:** cors: kein Workflow ruft `cors_compiler`; Wind SWE: Receiver/force
  unentschieden.
- **Braucht:** river entscheidet `cors_compiler` vs. `cors_rinex` (bestehende
  Zeile); Wind SWE als `sources.φ`-Zeile admiten (Receiver/force, Rat).

### GIC-Stufe-2 — Route C Member-Pool (Rat); dB/dt-Bestand 2/154
- **Status:** eigen | **Bindung:** eigen (Register) · river (`compute_max_t`)
- **Trigger:** AE/AL/AU · SME/SML/SMU als eigene `sources.φ`-Zeilen admiert
- **Lage:** (gemessen 2026-10-08 274) Rat-Verdikt: Route C, Stufe-2-Member-Pool
  des `compute_max_t` IST der Träger
  (`docs/blatt/blatt-gic-breitenband-familien.md:243-247`); `Linie 1` bleibt
  ungeglättete Gegenlinie. **dB/dt-Bestand: 2 von 154**
  (ABK `sources.φ:2150-2168`, SOD `:2170-2178`); 152 tragen `intermagnet_xyz`.
  Beide 1h-Assets `gate-no-field-lines`-**refused**
  (`phi/pipeline/refusal_ledger.φ:75-76`). **AE/AL/AU-Parameter gemessen**
  (`AE1800`/`AL_INDEX1800`/`AU_INDEX1800` in `OMNI2_H0_MRG1HR`); SuperMAG-
  Index-Route (`SME/SML/SMU`) noch ungemessen.
- **Blockade:** Member-Pool ohne vollen dB/dt-Bestand nicht wirebar; der
  refused-Zustand der 2 Arme ungelöst; AE/AL/AU + SME/SML/SMU nicht admitiert.
- **Braucht:** Receiver/force der AE/AL/AU-Zeile festlegen (Rat); SME/SML/SMU-
  SuperMAG-Index-Route messen; `gate-no-field-lines`-Refusal der 2 dB/dt-Arme
  lösen oder `blockiert` registrieren; dann River Stufe 2 am `compute_max_t`.

### em-nmgy / Bandreferenz — Parser-Arm GEBAUT; `band_id`-Persistenz + Heim offen
- **Status:** eigen | **Bindung:** eigen · river (Feld-Erweiterung)
- **Trigger:** river-135 committet (Fremd-Dirty weg) → `band_id`-Feld
- **Lage:** (gemessen 2026-10-08 273, HEAD `dff6ccfda`) Parser-Arm gebaut:
  `quantity`-Arm (`parse.rs:1037-1139`) parst Tail
  `band <id> pivot <λ><unit> [edges <λmin>-<λmax><unit>]` → `freq = c/λ_m`,
  `bin_width = |c/λmin − c/λmax|`; Erzwingung `QuantityKind::Scale`+`nmgy` ohne
  Band → Anomalie + Record verweigert (nie `0.0`). `sources.φ:19650` trägt
  `… scale nmgy 31536000 0.0 0.0 band DECam_g pivot 4808.49angstrom edges 3900-5600angstrom`.
  Gate-Fixture `commit_gate_vocab.json:100`. `register_sort` canonical;
  `cargo check` 0/0. Band-Heim `phi/bindings/bands.φ` + `phi/canon.φ` `section
  Bindings` gebaut (273).
- **Blockade:** `band_id` als Feld in `FieldConfig` persistiert noch **nicht** —
  ein neues Pflichtfeld bräche die Literale in den fremd-dirty Dateien
  `main_flow.rs:1449`, `relay.rs:668/729`, `channels.rs:1408/1441` (river-135
  uncommittet; `src/archivar/relay.rs` im Baum dirty, gemessen 274). Die
  Klausel-`<id>` (`DECam_g`) wird geparst/validiert, aber nicht gespeichert.
  SVO-Kurve HTTP 200 / 0 B Body → `pending`.
- **Braucht:** nach river-135-Commit das Feld `band_id: Option<String>` in
  `FieldConfig` ergänzen (eine Zeile je Literal) + im `quantity`-Arm setzen;
  CI-Test grün. **Riss 1:** der Anker stabilisiert die Referenz, nicht die Bytes
  (kein Kurven-`sha256`). **Riss 2:** `band`/`svo`/`pivot`/`zeropoint` = Mountain,
  `url` = Mycelium.

### span-Apertur / Membran-Agnostik — Mountain hält `receiver.span`
- **Status:** eigen (Datenkontrakt) | **Bindung:** eigen · river (ω()-Lauf)
- **Trigger:** River nennt die Signatur / baut den Reader
- **Lage:** (gemessen 2026-10-08 272) Rat: Apertur IST Record-`extent`; `span` ist
  deklarierter Receiver-Override **ohne Leser** (`parse.rs:255-266`,
  `types.rs:445`); Start-Anker aus Wire-`extent` (`static/membrane.html:568-596`).
  Q1-Gate über jeden Body (Cache wie Fetch); River zog den Cache-Zweig durchs
  Gate (`c71313d7e`).
- **Blockade:** `span` hat keinen Leser; Receiver-Pflicht-Signatur nicht gebaut.
- **Braucht:** Mountain hält `receiver.span`-Direktiv + schreibt den
  Receiver-Satz (Abwesenheit der Weltlinie → Record verweigert) in
  `docs/concepts/archivar-mathematikerin.md`; River liest `receiver.span` in der
  Query-Apertur (`wasm.rs:65-99` + `membrane.html:501-528`).

### Flyby-Kette — Kanäle registriert; Doppler `pending`
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium
- **Trigger:** DSN/ESTRACK-Residualroute admiert oder Descope-Befund
- **Lage:** (gemessen 2026-10-08 273) `SW_FAST_MAGA_LR_1B` registriert
  (`sources.φ:8153`, VirES-HAPI). RTSW/ACE/Kp/OMNI2/Swarm/DSN-Power/JUICE-
  Ephemeride registriert. Nicht registriert: Doppler (keine DSN/ESTRACK-
  Residualroute, `src/archivar/doppler.rs` absent), σ_recon
  (`data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin` absent). future-201 nennt
  BepiColombo-Zenodo `17813314` (60-s, CC-BY-4.0) als neuen Kanal.
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

### `blocked_sources.φ`-Aufräumen (mycelium-268/269) — Teil geleistet
- **Status:** eigen | **Bindung:** eigen (Disposition)
- **Trigger:** Diver-Klassentabelle (a/b/c/d) liegt im Baum
- **Lage:** (gemessen 2026-10-08 274) 3 redundante `descoped` umgezogen; Madrigal-
  `gap` korrigiert; LEOS/Gaia/TUH/NSRR re-registriert. **Riss:** die per-Eintrag-
  Klassenlisten liegen in keinem Baum (nur Aggregat-Zahlen a/b/c/d = 11/9/9/10) →
  Rest `pending`. **LEOS-Riss** (`:104-106` `descoped` Captcha vs.
  `survey-2026-10-08-open-sources-delta.md:75/133` `206` user-gated) ungelöst.
- **Blockade:** Klassen-Mitgliedschaft nicht im Baum; die numerischen ids
  30/31/34/… sind in keiner Register-Revision auflösbar.
- **Braucht:** Diver-Tabelle (mycelium) ins Handover oder als Datei; dann Klasse
  a→`ledger.φ` `ausstehend`, b→`blocked account`/`key`, c→`parser-def`+`gap`,
  d-Stale-Schutz. `descoped-check` auf `blocked parser-def` ausdehnen
  (Gate-Fixture).
- **Träger (Klassen-Punkt):** `phi/blocked_sources.φ::gap:openmadrigal-api ×1`
  (Madrigal `blocked parser-def`, OpenMadrigal-API-Arm fehlt).

### Route-Admissionen (future-201 §An mountain) — gemessen, Admission offen
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium (Manifestation)
- **Trigger:** Rat-Verdikt Receiver/force je Arm
- **Lage:** (gemessen 2026-10-08, future-Taucher + `--verdict`) Kandidaten-Routen
  live: EMTF `doi.org/10.17611/DP/EMTF/USARRAY/TA` (206) · ROTI/TEC/MUF/Slab/
  CALLISTO `data.impc.dlr.de/…` nur `latest/` (Datendateien keyless 200) ·
  SuperDARN CPCP Plots + Zenodo `10.1029/2024JA032864` (206) · THEMIS GMAG
  CDAWeb HAPI (190 `THG_L2_MAG_*`) · vt.superdarn.org (Konto steht, POST
  `data_download`, 15/Tag) · Kuprat Zenodo `15316905` (LSCO INS, CC-BY-4.0) ·
  ONCat `oncat.ornl.gov/api/*` (74 public) · CERN Open Data/XENONnT/LZ/Super-K/
  IceCube (keyless, Experiment offen) · GEBCO/CEDA · Fermi-4FGL `gll_psc_v32.fit`.
  `impc` nicht in `sources.φ`. **Riss:** `--verdict` 200 an Produkt-Wurzeln =
  Login-HTML, nicht offen. Kein offener NSE-I(q,t)-YBCO-Satz.
- **Blockade:** je Arm Receiver/force/ttl-Typ zu entscheiden (Rat); teils
  `account` (ONCat/ILL/ISIS/MLZ NSE).
- **Braucht:** Rat-Verdikt Receiver/force je Arm (Raster vs. FITS vs. INS), dann
  `sources.φ`-Zeilen + Mycelium-Manifestation. Rohmessungen in
  `state/future/route-dive-mountain-272-2026-10-08.md`,
  `…/superdarn-vt-messung-2026-10-08.md`, `…/kuprat-offene-quellen-2026-10-08.md`.

## An river

Origin: mountain-folge274.

- **`cors_compiler` (Antwort auf river-135):** der Baum trägt keinen Workflow, der
  `cors_compiler` ruft — `cors-cdn.yml:61` ruft `cors_rinex_compiler`; die
  CRX1-Form hat genau einen Reader-Arm `cors_rinex` (`src/archivar/extract.rs:45`),
  registriert (`phi/sources.φ:10586`). Eine `cors_compiler`-Zeile wäre ein
  ungewirerter Duplikat-Url. **Braucht:** Entscheidung, ob `cors_compiler` ein
  eigener Arm/Workflow wird oder entfällt.
- **INTERMAGNET-HAPI-Route (Antwort auf river-135):** korrekte Form gemessen —
  id **lowercase** + `time.min`/`time.max` Pflicht:
  `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=<code-lower>/best-avail/PT1M/xyzf&time.min=<iso>&time.max=<iso>&format=json`
  → 200. Der 400 war die fehlende Zeitgrenze.
- **`url-order`-Meldung (river-135):** gegen die vor-273-Revision gemessen; das
  Register ist kanonisch (`register_sort` → canonical, 2691 Blöcke). Keine Aktion.
- **ROTER Harvest-Build (Riss):** `ecef_to_geodetic(x,y,z,a,e2)`
  (`src/archivar/rinex.rs:4`) — 6 Harvest-Bins rufen 3-arg
  (`cses_scm_compiler.rs:166`, `cses_hpm_compiler.rs:161`, `cses_efd_compiler.rs:324`,
  `cors_compiler.rs:93`, `cors_rinex_compiler.rs:86`, `champ_plpt_compiler.rs:74`).
  **Braucht:** Body-Ellipsoid-Plumbing (`BodyProperties` → `a`/`e2`) an die Bins
  oder rückwärts-kompatible Signatur; ein WGS84-Literal ist per
  `commit_gate_vocab.json:87` verboten.
- **Loadergate (Q1)** und **`span`-Apertur (Q2)** unverändert offen (s. Offen);
  Mountain hält `receiver.span`, River liest es.
- **Commit-Sweep:** `src/archivar/channels.rs` (+24/−3) wurde durch einen
  ganz-index-`git commit` in `e51273108` mitgerissen; gepusht → nicht rewrite-bar.
  Künftige Mountain-Commits sind pfad-begrenzt.

## An mycelium

Origin: mountain-folge274.

- **KC2G-Parser geheilt (Riss geschlossen):** `station.latitude`/`station.longitude`
  (String, 0–360) + Longitude-Normalisierung; Live-Lauf 101 Stationen. Den
  `kc2g`-CDN-Workflow nach dem Push dispatchen und das Asset prüfen.
- **`ci-check` verdrängt jeden Lauf (Riss, gemessen 2026-10-07):**
  `.github/workflows/ci-check.yml:20-27` `cancel-in-progress: false`, jeder Lauf
  `cancelled` 0 Jobs. Fix nötig, sonst kein Per-SHA-Verdikt (IGRF-Witness).
- **`dropped-gate`-Baseline** nachgezogen (s. o.); der CI-Lauf nach diesem Push
  liest die neue Basis.
- **3 neue Harvest-Arme (272/273):** `superdarn_cpcp`/`emtf_impedance`/
  `ssusi_aurora` stehen in `sources.φ`+`harvest.φ`; Workflows + Dispatches aus 272
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

Eigene Pfade: `tools/harvest/src/bin/kc2g_stations.rs`,
`docs/zustand/dropped-legacy-baseline.txt`, `phi/sources.φ` (nur falls neu),
`docs/handover/handover-2026-10-08-mountain-folge274.md`,
`docs/handover/archiv/handover-2026-10-08-mountain-folge273.md` (Move).
