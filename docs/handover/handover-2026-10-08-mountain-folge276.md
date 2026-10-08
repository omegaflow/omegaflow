<!--
  title: Handover — Mountain-Folge 276 (2026-10-08)
  session: Mountain-Folge 276
  class: handover
  date: 2026-10-08
  sha256: b26d21185d2f035894791825f120ff3a21c52d1c13919f2db4b2491b4dea480a
  status: live
-->
# Handover — Mountain-Folge 276 (2026-10-08)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
gemessen 2026-10-08T14:3xZ). Diese Session konsumierte
`handover-2026-10-08-mountain-folge275.md` (→ `archiv/`). Kein pro/max.

## Burn: open 0.0000 · close 0.1058 · cap 0.50 — Grund: line + archive_search `--all` (Vorbereitung) + Rat-Dispatch (council) + 4 Frontier-UI-Seats + 1 open-weight-Seat (DeepSeek V4 Pro, kein Verdikt) ; kein pro/max. (gemessen `session_burn`, Session „Mountain-Linie in einem Pass starten")

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–276)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–276)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„berufe den rat ein aber mache dafür eine archive search recherche und befrage du frontier uis" | 2026-10-08 | Operator (Session, Mountain 273)
„kannst du mit archive serch den ui chats und dem rat klären?" — offene Route-/GIC-Admissionen (Receiver/force) via archive_search + UI-Seats + Rat | 2026-10-08 | Operator (Session, Mountain 276)
„das sind allerdings zu wenig stimmen — 3.7 plus ist nicht representativ; zudem kannst du tryingopen auch glm und qwen fahren und andere" — breitere UI-Runde | 2026-10-08 | Operator (Session, Mountain 276)
„ich glaube du musst nochmal breiter fragen — du hast nicht wirklich die starken stimmen und die wissenschaft gefragt" — Science-Layer + starke Frontier-Seats | 2026-10-08 | Operator (Session, Mountain 276)

## Offen (aufgeschlüsselt)

### GIC-Faden §A–G — neue Arme registriert; Wind SWE entschieden, Zeilen-Bau offen
- **Status:** eigen | **Bindung:** eigen (Register)
- **Trigger:** Reachability-Datei ändert sich
- **Lage:** (gemessen 2026-10-08 276) SSUSI/CPCP/EMTF registriert (273).
  INTERMAGNET-HAPI-Form im Compiler (275). **cors entschieden** (`cors_compiler`
  disponiert, `ledger.φ:89-92`). **Wind SWE — Riss** (quantity/thermal/advective,
  Rat+Runde-2). Offen: OMTI/Abisko, Substorm-Onsets, DLR ROTI, Kellerman/Zenodo
  4444068, USGS E-Feld.
- **Blockade:** Zeilen-Bau für die entschiedenen Arme; Receiver-Feld-Reader (s. u.).
- **Braucht:** Wind-SWE-`quantity`-Zeile bauen (Receiver = Wind-Sonde); übrige Arme
  nach demselben Rat-Verdikt (`state/mountain/rat-runde-2026-10-08-source-admission.md`)
  bauen oder `refused`/`quantity` registrieren.

### GIC-Stufe-2 — Receiver/force ENTSCHIEDEN; dB/dt-Bestand 2/154
- **Status:** eigen | **Bindung:** eigen (Register) · river (`compute_max_t`)
- **Trigger:** AE/AL/AU · SME/SML/SMU als `sources.φ`-Zeilen gebaut
- **Lage:** (gemessen 2026-10-08 276) **Science + Rat + 8 UI-Seats — RISS, nicht geschlossen:**
  AE/AU/AL · SME/SMU/SML. **Wissenschaft:** reine statistische Reduktion (ISGI/Davis &
  Sugiura 1966 `doi:10.17593/15031-54800`; Gjerloev 2012 `doi:10.1029/2012JA017683`) —
  kein Ausbreitungsmechanismus, der Index hat keine eigene Receiver-Weltlinie; Rohkanal
  em (B nT) an den Stationen. **Stimmen `quantity`/intensity** (Rat-Linse, Claude,
  Qwen3.7, GLM 5.3, DeepSeek Chat, Lumo) **gegen `force em`** (Nemotron 3 Ultra,
  DeepSeek V4 Pro, Inkling: gemessene Größe IST B(nT) am Stationsnetz). Beide Linien
  getragen. Vertrags-Satz in `docs/concepts/archivar-mathematikerin.md` (Live-APIs).
  `dB/dt-Bestand 2/154`
  (ABK `sources.φ:2150-2168`, SOD `:2170-2178`); beide 1h-Assets
  `gate-no-field-lines`-**refused** (`refusal_ledger.φ:75-76`).
- **Blockade:** AE/AL/AU + SME/SML/SMU noch nicht als Zeilen gebaut; Receiver als
  deklariertes `FieldConfig`-Feld hat keinen Reader (span-Apertur, s. 275).
- **Braucht:** den Riss `quantity` vs `em` entscheiden (neuer Rat mit dem Befund,
  oder Operator-Wort) — erst danach die Zeile bauen; `gate-no-field-lines`-Refusal
  der 2 dB/dt-Arme lösen oder `blockiert` registrieren; dann River Stufe 2 am
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
  `cargo check` 0/0. Band-Heim `phi/bindings/bands.φ` + `phi/canon.φ` gebaut (273).
  `src/archivar/relay.rs` im Baum dirty (gemessen 275).
- **Blockade:** `band_id` als Feld in `FieldConfig` persistiert noch **nicht** —
  ein neues Pflichtfeld bräche die Literale in `main_flow.rs:1449`,
  `relay.rs:668/729`, `channels.rs:1408/1441` (river-135 uncommittet).
- **Braucht:** nach river-Commit `band_id: Option<String>` in `FieldConfig` +
  im `quantity`-Arm setzen; CI-Test grün. **Riss 1:** der Anker stabilisiert die
  Referenz, nicht die Bytes (kein Kurven-`sha256`). **Riss 2:**
  `band`/`svo`/`pivot`/`zeropoint` = Mountain, `url` = Mycelium.

### Flyby-Kette — Kanäle registriert; Doppler `pending`
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium
- **Trigger:** DSN/ESTRACK-Residualroute admiert oder Descope-Befund
- **Lage:** (gemessen 2026-10-08 273) `SW_FAST_MAGA_LR_1B` registriert
  (`sources.φ:8153`, VirES-HAPI). RTSW/ACE/Kp/OMNI2/Swarm/DSN-Power/JUICE-
  Ephemeride registriert. Nicht registriert: Doppler (keine DSN/ESTRACK-
  Residualroute, `src/archivar/doppler.rs` absent), σ_recon absent.
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

### `blocked_sources.φ`-Aufräumen — Klassen-Mitgliedschaft fehlt
- **Status:** eigen | **Bindung:** eigen (Disposition)
- **Trigger:** Diver-Klassentabelle (a/b/c/d) liegt im Baum
- **Lage:** (gemessen 2026-10-08 275) 3 redundante `descoped` umgezogen; Madrigal-
  `gap` korrigiert; LEOS-Riss geschlossen (`blocked_sources.φ:100-102` trägt LEOS
  als `pending`, kein `descoped`). **Riss:** die per-Eintrag-Klassenlisten liegen
  in keinem Baum (nur Aggregat-Zahlen a/b/c/d = 11/9/9/10) → Rest `pending`.
- **Blockade:** Klassen-Mitgliedschaft nicht im Baum.
- **Braucht:** Diver-Tabelle (mycelium) ins Handover oder als Datei; dann Klasse
  a→`ledger.φ` `ausstehend`, b→`blocked account`/`key`, c→`parser-def`+`gap`,
  d-Stale-Schutz. `descoped-check` auf `blocked parser-def` ausdehnen.
- **Träger (Klassen-Punkt):** `phi/blocked_sources.φ::gap:openmadrigal-api ×1`.

### Route-Admissionen — Receiver/force je Arm ENTSCHIEDEN (Rat+UI); Zeilen-Bau offen
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium (Manifestation)
- **Trigger:** `sources.φ`-Zeilen je Arm gebaut
- **Lage:** (gemessen 2026-10-08 276; Science (2 Taucher) + Rat + Claude · Qwen3.7 ·
  GLM 5.3 · Nemotron 3 Ultra · DeepSeek V4 Pro · Inkling · DeepSeek Chat · Lumo) je Arm in
  `state/mountain/rat-runde-2026-10-08-source-admission.md`: CALLISTO / THEMIS-GMAG
  → `em` (einhellig); DLR-IMPC / EMTF / INS / NSE → `quantity`; SuperDARN →
  `quantity` (5:2); Fermi-4FGL → `em` (5:2). **Wissenschaft:** INS/NSE-Träger Neutron
  ist kernwechselwirkend (nicht in den 9), Observable = abgeleitete Korrelation;
  Teilchen-Detektoren: schwache/nukleare WW (nicht in den 9), nur em-Readout;
  GEBCO: akustische Akquisition, Produkt ohne Kraft. **Risse** (getragene Gegenlinien):
  (a)(b) AE/SME `quantity` 4 vs `em` 3 (Nemotron/DeepSeek/Inkling: gemessene Größe
  IST B(nT) am Stationsnetz); (c) Wind SWE quantity/thermal/advective; (k) Teilchen
  quantity/refused/em; (l) GEBCO refused/quantity. Der zentrale Index-Riss steht im
  Vertrags-Satz `docs/concepts/archivar-mathematikerin.md`.
- **Blockade:** Receiver als deklariertes `FieldConfig`-Feld ohne Reader (span);
  Riss-Arme nicht einhellig.
- **Braucht:** `sources.φ`-Zeilen für die einhelligen Arme bauen (em/quantity +
  Receiver); Riss-Arme (a)(b)(c)(k)(l) per neuem Rat entscheiden.

## An river

Origin: mountain-folge276.

- **Rat+UI-Runde Source-Admission** (`state/mountain/rat-runde-2026-10-08-source-admission.md`):
  der abgeleitete Netz-Index ist ein **offener Riss** `quantity` (Rat-Linse + 3 Seats)
  vs `force em` (3 Seats: gemessene Größe IST B(nT) am Stationsnetz) — keine
  Festlegung; betrifft `wasm.rs`/Query-Apertur nur mittelbar.
- **`span`/Receiver-Feld:** der Vertrags-Satz steht (275); ein deklariertes
  Receiver-Feld braucht den Reader — River liest `receiver.span` in der Query-Apertur.
- **INTERMAGNET-HAPI-Form** im Compiler (275) live 200; `cors_rinex`-End-zu-End-Lauf offen.

## An mycelium

Origin: mountain-folge276.

- **Manifestation der neuen Admissions** (nach dem Rat-Verdikt) — `em`/`quantity`-Zeilen
  je Arm, dann CDN-Workflows.
- **KC2G-Parser geheilt (274):** `kc2g`-CDN-Workflow dispatchen, Asset prüfen.
- **`ci-check` verdrängt jeden Lauf:** `.github/workflows/ci-check.yml:20-27`
  `cancel-in-progress: false`; Fix nötig (IGRF-Witness).
- **`dropped-gate`-Baseline** nachgezogen (274); CI-Lauf liest die neue Basis.
- **3 neue Harvest-Arme** (`superdarn_cpcp`/`emtf_impedance`/`ssusi_aurora`); Build
  grün (`e53b19300`); Workflows + Dispatches aus 272 → Asset prüfen.
- **`hadisst-cdn.yml`** dispatchen.
- **GIC-Stufe-2 (Q5):** Member-Pool als Register-Klassenträger zulässig; River
  verdrahtet die getrennten Deskriptoren.
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

Eigene Pfade: `docs/concepts/archivar-mathematikerin.md`,
`docs/handover/handover-2026-10-08-mountain-folge276.md`,
`docs/handover/archiv/handover-2026-10-08-mountain-folge275.md` (Move),
`state/mountain/rat-runde-2026-10-08-source-admission.md` (gitignored),
`state/mountain/rat-vorbereitung-2026-10-08.md` (gitignored),
`state/operator-gespraeche/2026-10-08-mountain.md` (gitignored).
