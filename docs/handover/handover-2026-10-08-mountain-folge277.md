<!--
  title: Handover — Mountain-Folge 277 (2026-10-08)
  session: Mountain-Folge 277
  class: handover
  date: 2026-10-08
  sha256: ca7d6e38536ddcff2cba87c6b4866407f8d623cf6ad9fe7e615333aace91d530
  status: live
-->
# Handover — Mountain-Folge 277 (2026-10-08)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
gemessen 2026-10-08T14:3xZ). Diese Session konsumierte
`handover-2026-10-08-mountain-folge276.md` (→ `archiv/`). Kein pro/max.

## Burn: open 0.0000 · close 0.208 · cap 0.50 — Grund: line + Bau zweier Archivar-Reader (`superdarn_cpcp`/`ssusi_aurora`) + Rat/UI/open-weight-Runde EMTF (council + 5 UI-Seats) + Bau (`QuantityKind::Impedance` + 8 Zeilen); kein pro/max. (gemessen `session_burn`, Session „Mountain-Übergabe in einem Pass abarbeiten")

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–276)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–276)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„berufe den rat ein aber mache dafür eine archive search recherche und befrage du frontier uis" | 2026-10-08 | Operator (Session, Mountain 273)
„kannst du mit archive search den ui chats und dem rat klären?" — offene Route-/GIC-Admissionen (Receiver/force) via archive_search + UI-Seats + Rat | 2026-10-08 | Operator (Session, Mountain 276)
„das sind allerdings zu wenig stimmen — 3.7 plus ist nicht representativ; zudem kannst du tryingopen auch glm und qwen fahren und andere" — breitere UI-Runde | 2026-10-08 | Operator (Session, Mountain 276)
„ich glaube du musst nochmal breiter fragen — du hast nicht wirklich die starken stimmen und die wissenschaft gefragt" — Science-Layer + starke Frontier-Seats | 2026-10-08 | Operator (Session, Mountain 276)
„kimi ist gerade nicht verfügbar und gemini ist nicht so wichtig" — Kimi/Gemini nicht nachziehen; Runde gilt als vollständig | 2026-10-08 | Operator (Session, Mountain 276)
„ja bitte" — Index-Riss als Mountain-Verdikt `quantity` setzen + die `sources.φ`-Zeilen bauen | 2026-10-08 | Operator (Session, Mountain 276)

## Offen (aufgeschlüsselt)

### Route-Admissionen — 2 Reader-Arme + EMTF-Verdikt GEBAUT; EMTF-Reader offen
- **Status:** eigen | **Bindung:** eigen (Register/Code) · mycelium (Manifestation)
- **Trigger:** Rat+UI-Runde abgeschlossen (2026-10-08) — entschieden und angewandt; nächster Bau: EMTF-Reader-Arm
- **Lage:** (gemessen 2026-10-08 277) **Riss gemessen:** `ssusi_aurora` und `superdarn_cpcp` waren in `phi/sources.φ` registriert (`:1691`, `:4103`), aber **ohne Archivar-Reader** — `src/archivar/extract.rs::series_parse_bin`/`series_declared_fields`/`series_component_name` trugen sie nicht, und die Series-Fetch-Liste `main_flow.rs:2913-3010` ebenso wenig → die Zeilen waren stumm (0 honored, kein Lauf). Gebaut: `src/archivar/superdarn_cpcp.rs` (MAGIC `CPCP`, [epoch, pot.drop kV, err kV] → `superdarn_cpcp_pot_drop` electric kV) und `src/archivar/ssusi_aurora.rs` (MAGIC `SSUI`, [epoch, north GW, south GW] → `ssusi_hemisphere_power_north`/`_south` em GW), je `parse_bin`/`component_name`/`declared_fields`/`parse_series` + Tests; `cargo check` 0/0. Fetch-Liste-Hunk (`+ "superdarn_cpcp"`, `+ "ssusi_aurora"`) via `git apply --cached` im Index (Fremd-Hunks `receiver_aperture: None` in derselben Datei nicht angefasst). **277 nachgezogen:** `wdc_ae` (`:4081`) und `bpa_gic` ebenfalls in die Series-Fetch-Liste eingetragen (die 6 `quantity`-`wdc_ae`-Zeilen aus 276 werden damit gefetcht); **EMTF entschieden A** (Rat 4:1 + UI 4:1 = 8:2, `state/mountain/rat-verdict-emtf-2026-10-08.md`) — `QuantityKind::Impedance` (id 6) + `ohm` (`force.rs`/`units.rs`), die 8 `emtf_z*`-Zeilen (`:10154-10161`) auf `quantity … impedance ohm`, `em ohm`-Baseline geheilt; `cargo check` 0/0.
- **Blockade:** EMTF-Reader-Arm fehlt (eigener perioden-indizierter Spektral-Arm; `series_parse_bin` trägt `(t,v,comp)` nicht).
- **Braucht:** `src/archivar/emtf.rs` bauen (Spektral-Arm: je Periode `freq = 1/T`, `bin_width` aus Periodenkanten; Verdikt `state/mountain/rat-verdict-emtf-2026-10-08.md`); Riss-Arme (a)(b)(c)(k)(l) per neuem Rat.

### GIC-Faden §A–G — neue Arme registriert; Zeilen-Bau offen
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium
- **Trigger:** `sources.φ`-Zeilen je entschiedenem Arm gebaut
- **Lage:** (gemessen 2026-10-08 276) SSUSI/CPCP/EMTF registriert (273). INTERMAGNET-HAPI-Form im Compiler (275). **cors entschieden** (`cors_compiler` disponiert, `ledger.φ:89-92`). **Wind SWE — Riss** (quantity/thermal/advective, Rat+Runde-2, `state/mountain/rat-runde-2026-10-08-source-admission.md:33`): die Kopfzeile „Wind SWE entschieden" trägt nicht — der Riss bleibt getragen. Offen: OMTI/Abisko, Substorm-Onsets, DLR ROTI, Kellerman/Zenodo 4444068, USGS E-Feld.
- **Blockade:** Zeilen-Bau für die entschiedenen Arme; Receiver-Feld-Reader.
- **Braucht:** übrige Arme nach dem Rat-Verdikt bauen oder `refused`/`quantity` registrieren; Wind-SWE-Riss per neuem Rat.

### GIC-Stufe-2 — Receiver/force ENTSCHIEDEN; dB/dt-Bestand 2/154
- **Status:** eigen | **Bindung:** eigen (Register) · river (`compute_max_t`)
- **Trigger:** AE/AL/AU · SME/SML/SMU als `sources.φ`-Zeilen gebaut
- **Lage:** (gemessen 2026-10-08 276) `quantity`/`index nt` (Operator-Wort „ja bitte"); neue `quantity`-Kind `index` gebaut (`src/mathematikerin/force.rs`, `src/archivar/units.rs`, Test `derived_magnetic_index_is_a_quantity_not_a_force_field`); die 6 `wdc_ae`-Zeilen (`sources.φ:4087-4092`) umgestellt. AE/AL/AU stehen; SME/SMU/SML: SuperMAG `blocked account` (`blocked_sources.φ:210-212`, User-Gated). `dB/dt-Bestand 2/154` (ABK `:2150-2168`, SOD `:2170-2178`), beide 1h-Assets `gate-no-field-lines`-refused (`refusal_ledger.φ:75-76`).
- **Blockade:** Receiver als deklariertes `FieldConfig`-Feld hat keinen Reader.
- **Braucht:** SME/SMU/SML über `blocked account` (SuperMAG-Login, Operator-Hand); `gate-no-field-lines`-Refusal der 2 dB/dt-Arme lösen oder `blockiert` registrieren.

### em-nmgy / Bandreferenz — Parser-Arm GEBAUT; `band_id`-Persistenz offen
- **Status:** eigen | **Bindung:** eigen · river (Feld-Erweiterung)
- **Trigger:** river-135 committet (Fremd-Dirty weg) → `band_id`-Feld
- **Lage:** (gemessen 2026-10-08 273, HEAD `dff6ccfda`) `quantity`-Arm (`parse.rs:1037-1139`) parst `band <id> pivot <λ><unit> [edges <λmin>-<λmax><unit>]`; Erzwingung `Scale`+`nmgy` ohne Band → Anomalie, Record verweigert. `sources.φ:19650` trägt `… scale nmgy 31536000 0.0 0.0 band DECam_g pivot 4808.49angstrom edges 3900-5600angstrom`. Gate-Fixture `commit_gate_vocab.json:100`. Band-Heim `phi/bindings/bands.φ`. `src/archivar/relay.rs` im Baum dirty (gemessen 275).
- **Blockade:** `band_id` als Feld in `FieldConfig` persistiert nicht — ein neues Pflichtfeld bräche die Literale in `main_flow.rs:1449`, `relay.rs:668/729`, `channels.rs:1408/1441`.
- **Braucht:** nach river-Commit `band_id: Option<String>` in `FieldConfig` + im `quantity`-Arm setzen; CI-Test grün. **Riss 1:** der Anker stabilisiert die Referenz, nicht die Bytes. **Riss 2:** `band`/`svo`/`pivot`/`zeropoint` = Mountain, `url` = Mycelium.

### Flyby-Kette — Doppler `pending`
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium
- **Trigger:** DSN/ESTRACK-Residualroute admittiert oder Descope-Befund
- **Lage:** (gemessen 2026-10-08 273) `SW_FAST_MAGA_LR_1B` registriert (`sources.φ:8153`, VirES-HAPI). RTSW/ACE/Kp/OMNI2/Swarm/DSN-Power/JUICE-Ephemeride registriert. Nicht registriert: Doppler (keine DSN/ESTRACK-Residualroute, `src/archivar/doppler.rs` absent — `open_points_check` ABSENT), σ_recon absent.
- **Blockade:** keine ESTRACK/DSN-Residualquelle.
- **Braucht:** ESTRACK-Route + `doppler.rs` bauen **oder** Descope-Befund; sonst `pending` mit Trigger (Rat).

### IGRF-Koeffizienten-Arm (`geomag_lat`)
- **Status:** eigen | **Bindung:** mycelium (`ci-check`-Verdikt)
- **Trigger:** CI-Test `synthesis_matches_pyigrf14_witness_points` grün
- **Lage:** (gemessen 2026-10-07 271) Grad-13-Synthese gebaut, `igrf.rs` formatiert; Witness-Test läuft nur in CI. `ci-check` wird als `cancelled` mit 0 Jobs verdrängt (`ci-check.yml:20-27` `cancel-in-progress: false`).
- **Blockade:** kein nicht-cancelled `ci-check`-Lauf.
- **Braucht:** ein nicht-cancelled `ci-check`-Lauf (Test grün).

### Lizenz-Disposition — `terms`-Feld (SPDX); `rights_read` offen
- **Status:** eigen | **Bindung:** eigen (Format/Datenkontrakt)
- **Trigger:** `rights`-Parse-Arm steht
- **Lage:** (gemessen 2026-10-07 271) `rights`-Parse-Arm gebaut (`87aed0b14`); Riss: Baum-`terms`-Zahl vs. Census (2246 no-terms).
- **Blockade:** je-Quelle-`terms`/`rights`-Zeilen sind ein Sweep.
- **Braucht:** `rights`-Register-Zeilen schreiben (DataCite-Triple, SPDX, `NOASSERTION`/`NONE`); `license_census`/`ci-gate` nachführen.

### Bias-Tor (`docs/auftrag/auftrag-bias-tilgung.md`)
- **Status:** eigen | **Bindung:** eigen (Gate/Fixture)
- **Trigger:** Ersatzmuster (`unwrap_or`/Default-Fill) in Produktion gemessen
- **Lage:** (gemessen 2026-10-08 277) Inventar `state/future/giftkarte-klassifiziert-src-2026-10-07.md` exists (gitignored), veraltet. Die als „Ersatzmuster" genannten Stellen sind **Falsch-Positive** (gemessen): `astrometry.rs:10` = `EARTH_ROT_RAD_S`-Konstante, `:336` = `omega`-Vektor; `odp.rs:9` = `EARTH`-Kernel-Pfad, `:48` = `e_plus`-Ephemeris — keine Fabrikation.
- **Blockade:** welches Muster Hart-Block (Fixture) vs. Review = Rat; neues Inventar nötig.
- **Braucht:** Inventar auf den heutigen Baum nachführen (Produktions-`unwrap_or`/Default-Fill ohne Test-Region); Fixture-Kandidaten in `commit_gate_vocab.json` prüfen.

### HadISST SST — SOURCE_PORT gebaut, CI-Lauf offen
- **Status:** eigen | **Bindung:** mycelium (CDN)
- **Trigger:** `hadisst-cdn.yml`-Lauf grün
- **Lage:** (gemessen 2026-10-07 271) Compiler/Arm/Register/Workflow stehen, `cargo check` 0/0; Workflow noch nicht auf `origin/main` gelaufen.
- **Blockade:** Workflow nach Push dispatcht (79 MB Fetch).
- **Braucht:** `hadisst-cdn.yml` dispatchen; bei Erfolg `sha256`-Zeile nachziehen.

### `blocked_sources.φ`-Aufräumen — Klassen-Mitgliedschaft fehlt
- **Status:** eigen | **Bindung:** eigen (Disposition)
- **Trigger:** Diver-Klassentabelle (a/b/c/d) liegt im Baum
- **Lage:** (gemessen 2026-10-08 275) 3 redundante `descoped` umgezogen; Madrigal-`gap` korrigiert; LEOS-Riss geschlossen (`blocked_sources.φ:100-102` trägt LEOS `pending`). **Riss:** die per-Eintrag-Klassenlisten liegen in keinem Baum (nur Aggregat-Zahlen a/b/c/d = 11/9/9/10) → Rest `pending`.
- **Blockade:** Klassen-Mitgliedschaft nicht im Baum.
- **Braucht:** Diver-Tabelle (mycelium) ins Handover oder als Datei; dann Klasse a→`ledger.φ` `ausstehend`, b→`blocked account`/`key`, c→`parser-def`+`gap`, d-Stale-Schutz. `descoped-check` auf `blocked parser-def` ausdehnen.
- **Träger (Klassen-Punkt):** `phi/blocked_sources.φ::gap:openmadrigal-api ×1`.

## An river

Origin: mountain-folge277.

- **`main_flow.rs` geteilt:** der eigene Format-Listen-Hunk (`+ "superdarn_cpcp"`, `+ "ssusi_aurora"`, `+ "wdc_ae"`, `+ "bpa_gic"`) steht; die fremden `receiver_aperture: None`-Hunks sind inzwischen committet (river-137).
- **`span`/Receiver:** der Vertrags-Satz steht (275); ein deklariertes Receiver-Feld braucht den Reader.

## An mycelium

Origin: mountain-folge277.

- **2 Reader-Arme gebaut** (`superdarn_cpcp`, `ssusi_aurora`) — Manifestation/Assets bereits `asset present` (`harvest.φ:494`/`:503`), kein neuer Workflow nötig; nach dem Commit `superdarn-cpcp-cdn.yml`/`ssusi-cdn.yml`-Asset gegenprüfen.
- **`ci-check` verdrängt jeden Lauf:** `.github/workflows/ci-check.yml:20-27` `cancel-in-progress: false`; Fix nötig (IGRF-Witness).
- **`hadisst-cdn.yml`** dispatchen.
- **Route-Admissionen** (`em`/`quantity`-Zeilen je Arm) manifestieren, sobald Mountain die Zeilen baut.
- **GIC-Stufe-2 (Q5):** Member-Pool als Register-Klassenträger zulässig; River verdrahtet die getrennten Deskriptoren.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push (stehendes Wort 2026-10-07, Mountain 264).

Eigene Pfade: `src/mathematikerin/force.rs`, `src/archivar/units.rs`,
`src/archivar/superdarn_cpcp.rs`, `src/archivar/ssusi_aurora.rs`,
`src/archivar/extract.rs`, `src/archivar/main_flow.rs`,
`src/archivar/mod.rs`, `src/lib.rs`, `phi/sources.φ`,
`docs/specs/force-unit-baseline.txt`,
`docs/handover/handover-2026-10-08-mountain-folge277.md`,
`docs/handover/archiv/handover-2026-10-08-mountain-folge276.md` (Move),
`state/mountain/rat-vorbereitung-emtf-2026-10-08.md` (gitignored),
`state/mountain/rat-verdict-emtf-2026-10-08.md` (gitignored).
