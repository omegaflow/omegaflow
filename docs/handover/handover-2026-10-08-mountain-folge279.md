<!--
  title: Handover — Mountain-Folge 279 (2026-10-08)
  session: Mountain-Folge 279
  class: handover
  date: 2026-10-08
  sha256: bf760dab8d9c80a7d2f8c441f592724a23ed4b9ac713f74ddb7e7ff68af917f9
  status: live
-->
# Handover — Mountain-Folge 279 (2026-10-08)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
gemessen 2026-10-08T14:3xZ, Mycelium-269). Diese Session konsumierte
`handover-2026-10-08-mountain-folge278.md` (→ `archiv/`). Kein pro/max.

## Burn: open 0.0000 · close 0.098 · cap 0.50 — Grund: line $0.0535 + grind-flash band_id-Feld $0.0443; kein pro/max. (gemessen `session_burn`, 2026-10-08)

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–279)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–279)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„ja bitte" — Index-Riss als Mountain-Verdikt `quantity` setzen + die `sources.φ`-Zeilen bauen | 2026-10-08 | Operator (Session, Mountain 276)
„ich glaube du musst nochmal breiter fragen" — Science-Layer + starke Frontier-Seats für die Route-Admission | 2026-10-08 | Operator (Session, Mountain 276)

## Offen (aufgeschlüsselt)

### Route-Admissionen — Liveness gemessen; Wind-SWE-Riss
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium (Manifestation)
- **Trigger:** je Route der Arm/`refused`-Befund; DLR/Wind-Klasse neu
- **Lage:** (gemessen 2026-10-08 279, gefaltete Blöcke future-202 + mycelium-269) **200:** CDAWeb HAPI `WI_H0_SWE` · `THG_L2_MAG_ABK` (`themis_mag` registriert) · DLR-IMPC TEC-Nowcast/ROTI/`_D.json` · Zenodo `15316905` (Kuprat LSCO INS) · Zenodo `4444068` (Kellerman) · FMI MIRACLE ASC-keograms · `prop.kc2g.com/api/stations.json` · `superdarn.usask.ca/convection-maps` · `vizier.cds.unistra.fr/…/J/A+A/633/A99/members` · `datalab.noirlab.edu/tap/sync`. **206:** EMTF DOI `10.17611/DP/EMTF/USARRAY/TA` · `vires.services/…SW_FAST_MAGA_LR_1B` · `cedar.openmadrigal.org` · `ds.iris.edu/ds/products/emtf/` · `ssusi.jhuapl.edu/` · LEOS `leos.ac.cn`. **400 (Wayback 503):** `geomag.usgs.gov/ws/data/`. **Riss Wind SWE:** Route lebt (200), `info` trägt `startDate 1994-12-29 · stopDate 2001-05-31` — `{now}` liest 0; eine SWE-Zeile braucht einen historischen Fenster-Mechanismus. Neue Route-Pools (future-202): DLR-IMPC sechs Produkte (ROTI/TEC-Nowcast/TEC-Forecast/MUF/Slab-Thickness/CALLISTO FITS), SuperDARN VT-Portal (`POST /api/session/data_download`, Session-Cookie, 15/Tag), ONCat (ORNL SNS BL-15), SuperMAG, Open-Sources-Audit (HI4PI/BK18/CERN Open Data u. a.).
- **Blockade:** per-Route-Verdikt (`sources.φ`-Zeile/Arm oder `refused`/`pending` mit Trigger) fehlt; EMTF-Route trägt bereits eine Zeile.
- **Braucht:** je Route `archive_search --verdict`/`--sniff` gegen `phi/sources.φ` prüfen, Verdikt/`ttl`/Arm setzen oder `refused`/`pending` mit Trigger registrieren; Wind SWE als `pending` mit Trigger „historischer Fenster-Mechanismus"; Mycelium manifestiert nach Zulassung.

### GIC-Faden §A–G — neue Arme registriert; Zeilen-Bau offen
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium
- **Trigger:** `sources.φ`-Zeilen je entschiedenem Arm gebaut
- **Lage:** (gemessen 2026-10-08 279, gefaltet mycelium-269) SSUSI/CPCP/EMTF registriert (273). INTERMAGNET-HAPI-Form im Compiler (275). **cors entschieden** (`cors_compiler` disponiert, `ledger.φ:89-92`). **Wind SWE — Riss** (s. Route-Admissionen). Offen: OMTI/Abisko, Substorm-Onsets (`account`), DLR ROTI, Kellerman/Zenodo 4444068, USGS E-Feld.
- **Blockade:** Zeilen-Bau für die entschiedenen Arme; Receiver-Feld-Reader (river-138, s. GIC-Stufe-2).
- **Braucht:** übrige Arme nach dem Rat-Verdikt bauen oder `refused`/`quantity` registrieren.

### GIC-Stufe-2 — Receiver/force ENTSCHIEDEN; Receiver-Reader steht (river-138)
- **Status:** eigen | **Bindung:** eigen (Register) · river (`compute_max_t`)
- **Trigger:** AE/AL/AU · SME/SML/SMU als `sources.φ`-Zeilen gebaut
- **Lage:** (gemessen 2026-10-08 279) `quantity`/`index nt` (Operator-Wort „ja bitte"); `QuantityKind::Index` gebaut; die 6 `wdc_ae`-Zeilen (`sources.φ:4087-4092`) umgestellt. AE/AL/AU stehen; SME/SMU/SML: SuperMAG `blocked account` (`blocked_sources.φ:210-212`). `dB/dt-Bestand 2/154` (ABK `:2150-2168`, SOD `:2170-2178`). **Neu (river-138):** der deklarierte Receiver-Body mit `#aperture=` ist gebaut (`DeclaredBody` `types.rs:124`, Parse `parse.rs:1207-1271`, `main_flow.rs:364/843`). **GIC-Stufe-2 Member-Pool (mycelium-269):** als Register-Klassenträger zulässig, als ein Wire-Deskriptor verboten; River verdrahtet die drei Deskriptoren.
- **Blockade:** ob der GIC-Force-Deskriptor den Receiver-Reader (`DeclaredBody`) nutzt, ist ungemessen.
- **Braucht:** SME/SMU/SML über `blocked account` (SuperMAG-Login, Operator-Hand); den `DeclaredBody`-Reader am GIC-Deskriptor messen (river) — nutzt er ihn, ist die Receiver-Blockade gelöst.

### Flyby-Kette — Doppler `pending`
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium
- **Trigger:** DSN/ESTRACK-Residualroute admittiert oder Descope-Befund
- **Lage:** (gemessen 2026-10-08 279) `SW_FAST_MAGA_LR_1B` registriert (`sources.φ:8153`). RTSW/ACE/Kp/OMNI2/Swarm/DSN-Power/JUICE registriert. Nicht registriert: Doppler (`src/archivar/doppler.rs` absent — `open_points_check` ABSENT), σ_recon absent. Neuer Kanal: BepiColombo-Zenodo `17813314` (60-s, CC-BY-4.0).
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
- **Lage:** (gemessen 2026-10-08 279, gefaltet mycelium-269) (1) **umziehen** — Klasse a (11 `pending`: Arm+WF stehen) → `ledger.φ` `ausstehend`; 3 redundante `descoped` (nssdc.ac.cn, titanNotebook, ioc-v1) → `declined_sources.φ` `decline superseded-by-integrated`. (2) **umtaggen** — Klasse b (9) → `blocked account`/`key`; Klasse c (9) → `parser-def` + korrektes `gap`. (3) **korrigieren** — Madrigal-`gap` `html-parser-arm` → OpenMadrigal-API-Arm; GHSL (`covariate-carrier`) bleibt. (4) **re-messen** — Klasse d (10 Stale); `leos.ac.cn` (206 user-gated) + Gaia `cluster_ka` (VizieR members) re-registrieren; TUH/NSRR re-messen. (5) **entfernen nur mit Befund** — die 20 Legenden-`note`s mit stehendem Arm + Zeile 2. (6) **Stale-Schutz** — Messstelle/Stempel/Trigger je Eintrag + `descoped-check` auf `blocked parser-def` ausdehnen. **NICHTS entfernen, was lebt/registriert ist** (Rat, einhellig). **Riss:** per-Eintrag-Klassenlisten liegen in keinem Baum (nur Aggregat a/b/c/d = 11/9/9/10).
- **Blockade:** Klassen-Mitgliedschaft nicht im Baum.
- **Braucht:** Diver-Tabelle (mycelium) ins Handover oder als Datei; dann Klasse a→`ledger.φ` `ausstehend`, b→`blocked account`/`key`, c→`parser-def`+`gap`, d-Stale-Schutz.
- **Träger (Klassen-Punkt):** `phi/blocked_sources.φ::gap:openmadrigal-api ×1`.

### LEOS-Riss — `blocked_sources.φ` vs. Open-Sources-Delta
- **Status:** eigen | **Bindung:** eigen (Disposition)
- **Trigger:** Re-Messung der LEOS-Route
- **Lage:** (gemessen 2026-10-08 279) `phi/blocked_sources.φ` LEOS-Eintrag trägt `pending` (Zeile driftet: folge278 nannte `:100-102`, mycelium-269 `:104-106`); `survey-2026-10-08-open-sources-delta.md:75/133` misst `206` (user-gated).
- **Blockade:** Zeilen-Drift + zwei Fassungen.
- **Braucht:** LEOS-Zeile am Baum festnageln, `206 user-gated` als `pending`/Auth-Route registrieren (nie `declined`, Auth kein Ausschluss).

## An river

Origin: mountain-folge279.

- **`main_flow.rs` sauber:** der Fremd-Commit (river-138) ist gelandet; der EMTF-Format-Arm ist jetzt verdrahtet (s. u.). Der eigene Format-Listen-Hunk (`superdarn_cpcp`/`ssusi_aurora`/`wdc_ae`/`bpa_gic`) steht (277).
- **Receiver:** river-138 (`DeclaredBody` + `#aperture=`) — ob der GIC-Force-Deskriptor den Reader nutzt, ist die nächste Messung (GIC-Stufe-2).

## An mycelium

Origin: mountain-folge279.

- **`emtf-cdn.yml` neu dispatchen** — der EMTF-Format-Arm ist verdrahtet (`main_flow.rs` `emtf_impedance`), das Bin-Format ist 16-B-Header (Epoch) — das alte CDN-Asset ist alt-Format.
- **KC2G-Parser geheilt** (`1ed930b82`, mountain-274): `kc2g_stations.rs` liest `station.latitude`/`longitude` (Strings), Fixture spiegelt die Live-Form. Der CDN-`failure`-Lauf ist damit obsolet — neu dispatchen.
- **`ci-check` verdrängt jeden Lauf:** `.github/workflows/ci-check.yml:18-20` `cancel-in-progress: false`; Fix nötig (IGRF-Witness).
- **`hadisst-cdn.yml`** dispatchen.
- **Route-Admissionen** manifestieren, sobald Mountain die Zeilen/Arme baut; Diver-Klassentabelle (a/b/c/d) für `blocked_sources.φ`-Aufräumen in den Baum legen.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push (stehendes Wort 2026-10-07, Mountain 264).

Eigene Pfade: `src/archivar/main_flow.rs` (EMTF-Arm), `src/archivar/types.rs` ·
`src/archivar/parse.rs` · `src/archivar/extract.rs` · `src/archivar/channels.rs` ·
`src/archivar/relay.rs` · `src/archivar/fetch.rs` · `src/archivar/tests.rs` ·
`src/mathematikerin/machines/tests.rs` und die 28 Einleser-Module (band_id-Feld),
`docs/handover/handover-2026-10-08-mountain-folge279.md`,
`docs/handover/archiv/handover-2026-10-08-mountain-folge278.md` (Move).
