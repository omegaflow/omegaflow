<!--
  title: Handover — Mountain-Folge 278 (2026-10-08)
  session: Mountain-Folge 278
  class: handover
  date: 2026-10-08
  sha256: e35e789f5ee57fe021e05f116ba08bf435ab496aefceabae018e702fbbcf8901
  status: live
-->
# Handover — Mountain-Folge 278 (2026-10-08)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
gemessen 2026-10-08T14:3xZ, Mycelium-269). Diese Session konsumierte
`handover-2026-10-08-mountain-folge277.md` (→ `archiv/`). Kein pro/max.

## Burn: open 0.0000 · close 0.062 · cap 0.50 — Grund: line 0.0416 + grind-flash EMTF-Bau 0.0137 + general Route-Batch; kein pro/max. (gemessen `session_burn`, 2026-10-08)

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–278)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–278)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„ja bitte" — Index-Riss als Mountain-Verdikt `quantity` setzen + die `sources.φ`-Zeilen bauen | 2026-10-08 | Operator (Session, Mountain 276)
„ich glaube du musst nochmal breiter fragen" — Science-Layer + starke Frontier-Seats für die Route-Admission | 2026-10-08 | Operator (Session, Mountain 276)

## Offen (aufgeschlüsselt)

### EMTF-Reader-Arm — Compiler-Epoch + Reader-Modul GEBAUT; main_flow-Verdrahtung offen
- **Status:** eigen | **Bindung:** eigen (Code) · mycelium (CDN) · fremde main_flow-Hunks
- **Trigger:** `main_flow.rs` der Fremd-Linie committet (Baum sauber) → Arm verdrahten
- **Lage:** (gemessen 2026-10-08 278, `cargo check` 0/0, realer XML-Lauf) Der Compiler verlor die Mess-Epoche. Gebaut: `tools/harvest/src/bin/emtf_compiler.rs` schreibt jetzt MAGIC `EMTF` + u32 count + f64 `epoch_unix` + count×9 f64 (Header 16 B) und liest die Epoche aus dem ersten `<Start>` des XML (`parse_iso_seconds`); fehlt sie → `exit(1)`, Bin bleibt ungeschrieben. Neu: `src/archivar/emtf.rs` (`parse_bin`, `component_name`, `component_bins` je der 8 Komponenten `emtf_zxx_re…emtf_zyy_im`, Perioden→freq + Kantenbreite), `pub mod emtf;` in `src/archivar/mod.rs`. Realer Lauf `/tmp/opencode/emtf.xml`: 30 Perioden, epoch 1275444092 unix (2010-06-02T02:01:32Z), Roundtrip parst (2176 B).
- **Blockade:** `src/archivar/main_flow.rs` trägt fremde uncommittete Hunks → der eigene Arm-Hunk ist nicht pfad-begrenzt committbar.
- **Braucht:** nach dem Fremd-Commit einen `if format == "emtf_impedance"`-Arm bauen, der je Komponente einen `SpectralHash` sendet (Muster: `main_flow.rs:2469-2542` xp_spectra, `Motion` aus `src.frame`); dann Mycelium `emtf-cdn.yml` neu dispatchen (neues 16-B-Format).

### Route-Admissionen — Liveness gemessen; Wind-SWE-Riss
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium (Manifestation)
- **Trigger:** je Route der Arm/`refused`-Befund; DLR/Wind-Klasse neu
- **Lage:** (gemessen 2026-10-08 278, `archive_search --verdict`) **200:** CDAWeb HAPI `WI_H0_SWE` (2511 B) · `THG_L2_MAG_ABK` (987 B; `themis_mag` registriert) · DLR-IMPC TEC-Nowcast EUROPE `…_D.json` (28503 B) · Zenodo `15316905` (Kuprat LSCO INS, 138814 B, sha `04250d6d…`) · Zenodo `4444068` (Kellerman, 125839 B) · FMI `space.fmi.fi/MIRACLE/ASC/ASC_keograms/` (719242 B). **206:** EMTF DOI `10.17611/DP/EMTF/USARRAY/TA` · DLR-IMPC ROTI MAX GLOBAL `…_D.json` (200/sniff 1054243 B, sha `6380a3de…`). **absent:** `sdc-serv.usask.ca/convection_plots/2026/10/` (500, kein CDX) · `geomag.usgs.gov/ws/data/` (400, Wayback 503). **Riss Wind SWE:** Route lebt (200), aber `info` trägt `startDate 1994-12-29 · stopDate 2001-05-31` — ein `{now}`-Fenster liest 0; die bestehenden Wind-MFI-URLs nutzen `{now}` → eine SWE-Zeile braucht einen historischen Fenster-Mechanismus. **THEMIS GMAG** bereits als `themis_mag` registriert (CDAWeb-HAPI-Arm offen).
- **Blockade:** EMTF/ROTI/DLR-IMPC/Zenodo-Kuprat/Kellerman haben keinen Format-Compiler-Arm; Wind SWE hat kein historisches Fenster.
- **Braucht:** je Route Verdikt/`ttl` (Arm bauen oder `refused`/`pending` mit Trigger); Wind SWE als `pending` mit Trigger „historischer Fenster-Mechanismus" registrieren; Mycelium manifestiert nach Zulassung.

### GIC-Faden §A–G — neue Arme registriert; Zeilen-Bau offen
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium
- **Trigger:** `sources.φ`-Zeilen je entschiedenem Arm gebaut
- **Lage:** (gemessen 2026-10-08 276) SSUSI/CPCP/EMTF registriert (273). INTERMAGNET-HAPI-Form im Compiler (275). **cors entschieden** (`cors_compiler` disponiert, `ledger.φ:89-92`). **Wind SWE — Riss** (s. Route-Admissionen). Offen: OMTI/Abisko, Substorm-Onsets, DLR ROTI, Kellerman/Zenodo 4444068, USGS E-Feld.
- **Blockade:** Zeilen-Bau für die entschiedenen Arme; Receiver-Feld-Reader.
- **Braucht:** übrige Arme nach dem Rat-Verdikt bauen oder `refused`/`quantity` registrieren.

### GIC-Stufe-2 — Receiver/force ENTSCHIEDEN; dB/dt-Bestand 2/154
- **Status:** eigen | **Bindung:** eigen (Register) · river (`compute_max_t`)
- **Trigger:** AE/AL/AU · SME/SML/SMU als `sources.φ`-Zeilen gebaut
- **Lage:** (gemessen 2026-10-08 276) `quantity`/`index nt` (Operator-Wort „ja bitte"); `QuantityKind::Index` gebaut; die 6 `wdc_ae`-Zeilen (`sources.φ:4087-4092`) umgestellt. AE/AL/AU stehen; SME/SMU/SML: SuperMAG `blocked account` (`blocked_sources.φ:210-212`). `dB/dt-Bestand 2/154` (ABK `:2150-2168`, SOD `:2170-2178`), beide 1h-Assets `gate-no-field-lines`-refused (`refusal_ledger.φ:75-76`).
- **Blockade:** Receiver als deklariertes `FieldConfig`-Feld hat keinen Reader.
- **Braucht:** SME/SMU/SML über `blocked account` (SuperMAG-Login, Operator-Hand); `gate-no-field-lines`-Refusal der 2 dB/dt-Arme lösen oder `blockiert` registrieren.

### em-nmgy / Bandreferenz — Parser-Arm GEBAUT; `band_id`-Persistenz offen
- **Status:** eigen | **Bindung:** eigen · river (Feld-Erweiterung)
- **Trigger:** river-135 committet (Fremd-Dirty weg) → `band_id`-Feld
- **Lage:** (gemessen 2026-10-08 273, HEAD `dff6ccfda`) `quantity`-Arm (`parse.rs:1037-1139`) parst `band <id> pivot <λ><unit> [edges …]`; Erzwingung `Scale`+`nmgy` ohne Band → Record verweigert. `sources.φ:19650` trägt `… scale nmgy … band DECam_g pivot 4808.49angstrom edges 3900-5600angstrom`. Gate-Fixture `commit_gate_vocab.json:100`. Band-Heim `phi/bindings/bands.φ`.
- **Blockade:** `band_id` als Feld in `FieldConfig` persistiert nicht — ein neues Pflichtfeld bräche die Literale in `main_flow.rs:1449`, `relay.rs:668/729`, `channels.rs:1408/1441`.
- **Braucht:** nach river-Commit `band_id: Option<String>` in `FieldConfig` + im `quantity`-Arm setzen; CI-Test grün. **Riss:** `band`/`svo`/`pivot` = Mountain, `url` = Mycelium.

### Flyby-Kette — Doppler `pending`
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium
- **Trigger:** DSN/ESTRACK-Residualroute admittiert oder Descope-Befund
- **Lage:** (gemessen 2026-10-08 273) `SW_FAST_MAGA_LR_1B` registriert (`sources.φ:8153`). RTSW/ACE/Kp/OMNI2/Swarm/DSN-Power/JUICE registriert. Nicht registriert: Doppler (`src/archivar/doppler.rs` absent — `open_points_check` ABSENT), σ_recon absent. Neuer Kanal: BepiColombo-Zenodo `17813314` (60-s, CC-BY-4.0).
- **Blockade:** keine ESTRACK/DSN-Residualquelle.
- **Braucht:** ESTRACK-Route + `doppler.rs` bauen **oder** Descope-Befund; sonst `pending` mit Trigger (Rat).

### IGRF-Koeffizienten-Arm (`geomag_lat`)
- **Status:** eigen | **Bindung:** mycelium (`ci-check`-Verdikt)
- **Trigger:** CI-Test `synthesis_matches_pyigrf14_witness_points` grün
- **Lage:** (gemessen 2026-10-07 271) Grad-13-Synthese gebaut, `igrf.rs` formatiert; Witness-Test läuft nur in CI. `ci-check` wird als `cancelled` mit 0 Jobs verdrängt (`.github/workflows/ci-check.yml:18-20` `cancel-in-progress: false`).
- **Blockade:** kein nicht-cancelled `ci-check`-Lauf.
- **Braucht:** ein nicht-cancelled `ci-check`-Lauf (Test grün). Mycelium-Feder (`ci-check`-Workflow).

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

Origin: mountain-folge278.

- **`main_flow.rs` geteilt:** der eigene Format-Listen-Hunk (`superdarn_cpcp`/`ssusi_aurora`/`wdc_ae`/`bpa_gic`) steht (277). Die EMTF-Verdrahtung wartet auf den Fremd-Commit der aktuell uncommitteten `main_flow.rs`-Hunks.
- **`span`/Receiver:** der Vertrags-Satz steht (275); ein deklariertes Receiver-Feld braucht den Reader.

## An mycelium

Origin: mountain-folge278.

- **`emtf-cdn.yml` neu dispatchen** — das Bin-Format ist auf 16-B-Header (Epoch) geändert; das alte CDN-Asset ist alt-Format.
- **2 Reader-Arme** (`superdarn_cpcp`, `ssusi_aurora`) — Assets bereits `asset present` (`harvest.φ:494`/`:503`).
- **`ci-check` verdrängt jeden Lauf:** `.github/workflows/ci-check.yml:18-20` `cancel-in-progress: false`; Fix nötig (IGRF-Witness).
- **`hadisst-cdn.yml`** dispatchen.
- **Route-Admissionen** manifestieren, sobald Mountain die Zeilen/Arme baut.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push (stehendes Wort 2026-10-07, Mountain 264).

Eigene Pfade: `tools/harvest/src/bin/emtf_compiler.rs`, `src/archivar/emtf.rs`,
`src/archivar/mod.rs`, `docs/handover/handover-2026-10-08-mountain-folge278.md`,
`docs/handover/archiv/handover-2026-10-08-mountain-folge277.md` (Move).
