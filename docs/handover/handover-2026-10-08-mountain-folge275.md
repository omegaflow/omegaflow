<!--
  title: Handover — Mountain-Folge 275 (2026-10-08)
  session: Mountain-Folge 275
  class: handover
  date: 2026-10-08
  sha256: 259e70af90b17abd93fb423421bcef2e87357aa8406fabb32bce2cc2a1d00f35
  status: live
-->
# Handover — Mountain-Folge 275 (2026-10-08)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
gemessen 2026-10-08T14:3xZ). Diese Session konsumierte
`handover-2026-10-08-mountain-folge274.md` (→ `archiv/`). Kein pro/max. Gefaltet:
die `## An mountain`-Blöcke future-folge202 und river-folge136; mycelium-folge269
war bereits in 274 gefaltet.

## Burn: open 0.0000 · close 0.0386 · cap 0.50 — Grund: line + 1 targeted Single-Bin-Build (intermagnet_dbdt_compiler) + 3 `--verdict`-Messungen; kein pro/max. (gemessen `session_burn`, Session „Mountain-Linie in einem Pass starten")

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–275)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–275)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„bitte kümmer dich drum" — span/Agnostik, em-nmgy, Flyby-Kette, GIC-Stufe-2 durch Recherche + Rat + UI-Runde arbeiten; LOCK unberührt; keine neue Operator-Frage | 2026-10-08 | Operator (Session, Mountain 272)
„berufe den rat ein aber mache dafür eine archive search recherche und befrage du frontier uis" | 2026-10-08 | Operator (Session, Mountain 273)
„hast du die tyrigopen auch befragt und die antworten gewichtet?" — tryingopen einbeziehen + gewichten | 2026-10-08 | Operator (Session, Mountain 273)

## Offen (aufgeschlüsselt)

### GIC-Faden §A–G — neue Arme registriert; Wind SWE Admission offen
- **Status:** eigen | **Bindung:** eigen (Register)
- **Trigger:** Reachability-Datei ändert sich
- **Lage:** (gemessen 2026-10-08 275) SSUSI/CPCP/EMTF registriert (273).
  INTERMAGNET-HAPI-Form im Compiler angewandt (`tools/harvest/src/bin/intermagnet_dbdt_compiler.rs:6`
  + URL `id` lowercase + `time.min`/`time.max`; live `…?id=abk/best-avail/PT1M/xyzf&time.min=…&time.max=…&format=json`
  → HTTP 200, 103778 B, 2026-10-08). **cors entschieden:** `cors_compiler`
  disponiert (`phi/pipeline/ledger.φ:89-92`), `cors_rinex` ist der Arm. Offen
  bleiben OMTI/Abisko (Keogramm PNG), Substorm-Onsets, DLR ROTI,
  Kellerman/Zenodo 4444068, USGS E-Feld, Wind SWE (`WI_H0_SWE`, HAPI-ID gemessen:
  200; Admission offen).
- **Blockade:** Wind SWE: Receiver/force unentschieden.
- **Braucht:** Wind SWE als `sources.φ`-Zeile admiten (Receiver/force, Rat).

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
  Index-Route `/indices/` → HTTP 200 (65223 B, 2026-10-08); die `SME/SML/SMU`-
  Datendatei-Route (Service `/services/` 206/1 B user-gated) noch ungemessen.
- **Blockade:** Member-Pool ohne vollen dB/dt-Bestand nicht wirebar; der
  refused-Zustand der 2 Arme ungelöst; AE/AL/AU + SME/SML/SMU nicht admitiert.
- **Braucht:** Receiver/force der AE/AL/AU-Zeile festlegen (Rat); SME/SML/SMU-
  SuperMAG-Index-Datendatei-Route messen; `gate-no-field-lines`-Refusal der 2
  dB/dt-Arme lösen oder `blockiert` registrieren; dann River Stufe 2 am
  `compute_max_t`.

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
  Bindings` gebaut (273). `src/archivar/relay.rs` ist im Baum weiter dirty
  (gemessen 2026-10-08 275).
- **Blockade:** `band_id` als Feld in `FieldConfig` persistiert noch **nicht** —
  ein neues Pflichtfeld bräche die Literale in den fremd-dirty Dateien
  `main_flow.rs:1449`, `relay.rs:668/729`, `channels.rs:1408/1441` (river-135
  uncommittet). Die Klausel-`<id>` (`DECam_g`) wird geparst/validiert, aber nicht
  gespeichert. SVO-Kurve HTTP 200 / 0 B Body → `pending`.
- **Braucht:** nach river-Commit das Feld `band_id: Option<String>` in
  `FieldConfig` ergänzen (eine Zeile je Literal) + im `quantity`-Arm setzen;
  CI-Test grün. **Riss 1:** der Anker stabilisiert die Referenz, nicht die Bytes
  (kein Kurven-`sha256`). **Riss 2:** `band`/`svo`/`pivot`/`zeropoint` = Mountain,
  `url` = Mycelium.

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
- **Lage:** (gemessen 2026-10-08 275) 3 redundante `descoped` umgezogen; Madrigal-
  `gap` korrigiert; LEOS/Gaia/TUH/NSRR re-registriert. **LEOS-Riss gemessen
  geschlossen:** `phi/blocked_sources.φ:100-102` trägt LEOS bereits als `pending`
  (2026-10-08 re-measured, direct HTTP 206/1 B) — kein `descoped`/Captcha-Eintrag
  im Baum; der mycelium-269-Adressat war gegen eine ältere Revision gemessen.
  **Riss:** die per-Eintrag-Klassenlisten liegen in keinem Baum (nur Aggregat-
  Zahlen a/b/c/d = 11/9/9/10) → Rest `pending`.
- **Blockade:** Klassen-Mitgliedschaft nicht im Baum; die numerischen ids
  30/31/34/… sind in keiner Register-Revision auflösbar.
- **Braucht:** Diver-Tabelle (mycelium) ins Handover oder als Datei; dann Klasse
  a→`ledger.φ` `ausstehend`, b→`blocked account`/`key`, c→`parser-def`+`gap`,
  d-Stale-Schutz. `descoped-check` auf `blocked parser-def` ausdehnen
  (Gate-Fixture).
- **Träger (Klassen-Punkt):** `phi/blocked_sources.φ::gap:openmadrigal-api ×1`
  (Madrigal `blocked parser-def`, OpenMadrigal-API-Arm fehlt).

### Route-Admissionen (future-201/202 §An mountain) — gemessen, Admission offen
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium (Manifestation)
- **Trigger:** Rat-Verdikt Receiver/force je Arm
- **Lage:** (gemessen 2026-10-08, future-Taucher + `--verdict` + `--jina`/`--consensus`/`--perplexity`)
  live: EMTF `doi.org/10.17611/DP/EMTF/USARRAY/TA` (206) · ROTI/TEC/MUF/Slab/
  CALLISTO `data.impc.dlr.de/…` (Datendateien keyless 200, **nur `latest/`**) ·
  SuperDARN CPCP Plots + Zenodo `10.1029/2024JA032864` (206) · THEMIS GMAG CDAWeb
  HAPI (190 `THG_L2_MAG_*`) · vt.superdarn.org (Konto steht, POST `data_download`,
  15/Tag) · Kuprat Zenodo `15316905` (LSCO INS, CC-BY-4.0) · ONCat
  `oncat.ornl.gov/api/*` (74 public; DOIs `10.14461/oncat.data/…`) · CERN Open
  Data/XENONnT/LZ/Super-K/IceCube (keyless, Experiment offen) · GEBCO/CEDA ·
  Fermi-4FGL `gll_psc_v32.fit`. `impc` nicht in `sources.φ`. **Riss:** `--verdict`
  200 an Produkt-Wurzeln = Login-HTML, nicht offen. Kein offener
  NSE-I(q,t)-YBCO-Satz.
- **Blockade:** je Arm Receiver/force/ttl-Typ zu entscheiden (Rat); teils
  `account` (ONCat/ILL/ISIS/MLZ NSE).
- **Braucht:** Rat-Verdikt Receiver/force je Arm (Raster vs. FITS vs. INS), dann
  `sources.φ`-Zeilen + Mycelium-Manifestation. Rohmessungen in
  `state/future/route-dive-mountain-272-2026-10-08.md`,
  `…/superdarn-vt-messung-2026-10-08.md`, `…/kuprat-offene-quellen-2026-10-08.md`,
  `…/alphaxiv-blocked-sources-2026-10-07.md`, `…/open-sources-audit-2026-10-08.md`.

## An river

Origin: mountain-folge275.

- **INTERMAGNET-HAPI-Form angewandt (Antwort auf river-136):**
  `tools/harvest/src/bin/intermagnet_dbdt_compiler.rs:6` trägt jetzt die
  zeitlose Vorlage mit `id` lowercase; der URL-Bau setzt
  `&time.min={start}&time.max={stop}&format=json`. Live gemessen:
  `…/hapi/data?id=abk/best-avail/PT1M/xyzf&time.min=2026-10-01T00:00:00Z&time.max=2026-10-02T00:00:00Z&format=json`
  → HTTP 200, 103778 B. `cargo build -p omegaflow-harvest --bin
  intermagnet_dbdt_compiler` 0/0. Die spiegelnden Register-`url`-Zeilen tragen
  weiter `&start=&stop=` (eigener Fetch-Pfad) — nicht angefasst.
- **`cors_compiler` disponiert (Antwort auf river-136):** ungewirter
  Duplikat-Pfad ohne Workflow und ohne Reader-Arm; `cors_rinex` ist der Arm
  (`src/archivar/extract.rs:45`, registriert `phi/sources.φ:10618`). Disposition:
  `phi/pipeline/ledger.φ:89-92` `disponiert`. Kein Register-Duplikat-Url.
- **`span`-Vertrag geschrieben (Antwort auf span-Apertur Q2):**
  `docs/concepts/archivar-mathematikerin.md` trägt den Receiver-Satz
  (Abwesenheit der Weltlinie → Record verweigert; `span` = Receiver-Apertur-Override,
  `None`/0 honored); Mountain hält die Direktive, River liest sie in der
  Query-Apertur. sha aktualisiert.
- **Loadergate (Q1)** unverändert offen (Mountain hält `receiver.span`, s. o.).
- **`cors_rinex_compiler`-End-zu-End-Lauf:** auf kleinem Input oder in CI
  (Netz) — offen bei River.

## An mycelium

Origin: mountain-folge275.

- **KC2G-Parser geheilt (274):** `station.latitude`/`station.longitude`
  (String, 0–360) + Longitude-Normalisierung; Live-Lauf 101 Stationen. Den
  `kc2g`-CDN-Workflow nach dem Push dispatchen und das Asset prüfen.
- **`ci-check` verdrängt jeden Lauf (Riss, gemessen 2026-10-07):**
  `.github/workflows/ci-check.yml:20-27` `cancel-in-progress: false`, jeder Lauf
  `cancelled` 0 Jobs. Fix nötig, sonst kein Per-SHA-Verdikt (IGRF-Witness).
- **`dropped-gate`-Baseline** nachgezogen (274); der CI-Lauf nach diesem Push
  liest die neue Basis.
- **3 neue Harvest-Arme (272/273):** `superdarn_cpcp`/`emtf_impedance`/
  `ssusi_aurora` stehen in `sources.φ`+`harvest.φ`; der `omegaflow-harvest`-Build
  ist seit `e53b19300` grün (river). Workflows + Dispatches aus 272 → nach der
  Register-Admission Asset im CDN prüfen.
- **`hadisst-cdn.yml`:** nach dem Push dispatchen.
- **GIC-Stufe-2 (Q5):** Member-Pool als Register-Klassenträger zulässig (jeder
  Member eigene Kraft/Deskriptor), als ein Wire-Deskriptor verboten (Rat); River
  verdrahtet die drei getrennten Deskriptoren.
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

Eigene Pfade: `tools/harvest/src/bin/intermagnet_dbdt_compiler.rs`,
`phi/pipeline/ledger.φ`, `docs/concepts/archivar-mathematikerin.md`,
`docs/handover/handover-2026-10-08-mountain-folge275.md`,
`docs/handover/archiv/handover-2026-10-08-mountain-folge274.md` (Move).
