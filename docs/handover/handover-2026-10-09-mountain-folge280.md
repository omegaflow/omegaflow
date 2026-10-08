<!--
  title: Handover — Mountain-Folge 280 (2026-10-09)
  session: Mountain-Folge 280
  class: handover
  date: 2026-10-09
  sha256: 92392c18eccbcf6adbeaee2ff729d8fc98a5ea4ba4d55cf99652dc1642e2761e
  status: live
-->
# Handover — Mountain-Folge 280 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
gemessen 2026-10-08T14:3xZ, Mycelium-269). Diese Session konsumierte
`handover-2026-10-08-mountain-folge279.md` (→ `archiv/`). Kein pro/max.

## Burn: open 0.0000 · close 0.130 · cap 0.50 — Grund: line $0.0767 + Sub-Agenten (ROTI $0.0183 · Kellerman $0.0155 · Rat $0.0121 · SuperDARN $0.0074); kein pro/max. (gemessen `session_burn`, 2026-10-09)

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–279)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–279)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„ja bitte" — Index-Riss als Mountain-Verdikt `quantity` setzen + die `sources.φ`-Zeilen bauen | 2026-10-08 | Operator (Session, Mountain 276)
„ich glaube du musst nochmal breiter fragen" — Science-Layer + starke Frontier-Seats für die Route-Admission | 2026-10-08 | Operator (Session, Mountain 276)
„bitte umsetzen Offen (im Report benannt): 2 blocked_sources-Risse (limadou/vco_rs Dubletten; cluster_ka-Zeile ohne gap), SuperDARN dritter Layout-Slot (kein pot.drop.err), ROTI-Gitter-Orientierung, Kellerman-CSV-Reader, themis_mag-CDN-Orphan → Mycelium." | 2026-10-09 | Operator (Session, Mountain 280)

## Offen (aufgeschlüsselt)

### Route-Admissionen — Liveness gemessen; Wind-SWE-Riss
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium (Manifestation)
- **Trigger:** je Route der Arm/`refused`-Befund; DLR/Wind-Klasse neu
- **Lage:** (gemessen 2026-10-08 279) **200:** CDAWeb HAPI `WI_H0_SWE` · `THG_L2_MAG_ABK` (`themis_mag` registriert) · DLR-IMPC TEC-Nowcast/ROTI/`_D.json` · Zenodo `15316905` (Kuprat LSCO INS) · Zenodo `4444068` (Kellerman) · FMI MIRACLE ASC-keograms · `prop.kc2g.com/api/stations.json` · `superdarn.usask.ca/convection-maps` · `vizier.cds.unistra.fr/…/J/A+A/633/A99/members` · `datalab.noirlab.edu/tap/sync`. **206:** EMTF DOI · `vires.services/…SW_FAST_MAGA_LR_1B` · `cedar.openmadrigal.org` · `ds.iris.edu/ds/products/emtf/` · `ssusi.jhuapl.edu/` · LEOS. **400 (Wayback 503):** `geomag.usgs.gov/ws/data/`. **Riss Wind SWE:** Route lebt, `info` trägt `startDate 1994-12-29 · stopDate 2001-05-31`; literale ISO-Grenzen liefern Datenzeilen (`{now}` liest 0) — Wind-SWE-Zeile bereits gebaut (Rat F1, s. Git). Neue Pools (future-202): DLR-IMPC sechs Produkte, SuperDARN VT-Portal, ONCat, SuperMAG, Open-Sources-Audit.
- **Blockade:** per-Route-Verdikt (`sources.φ`-Zeile/Arm oder `refused`/`pending` mit Trigger) fehlt für die verbleibenden Routen.
- **Braucht:** je Route `archive_search --verdict`/`--sniff` gegen `phi/sources.φ` prüfen, Verdikt/`ttl`/Arm setzen oder `refused`/`pending` mit Trigger registrieren; Mycelium manifestiert nach Zulassung.

### GIC-Faden §A–G — neue Arme registriert; Zeilen-Bau offen
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium
- **Trigger:** `sources.φ`-Zeilen je entschiedenem Arm gebaut
- **Lage:** (gemessen 2026-10-08 279) SSUSI/CPCP/EMTF registriert (273). INTERMAGNET-HAPI-Form im Compiler (275). **cors entschieden** (`cors_compiler` disponiert, `ledger.φ:89-92`). **Wind SWE — Riss** (s. Route-Admissionen). Offen: OMTI/Abisko, Substorm-Onsets (`account`), DLR ROTI, Kellerman/Zenodo 4444068, USGS E-Feld.
- **Blockade:** Zeilen-Bau für die entschiedenen Arme; Receiver-Feld-Reader (river-138, s. GIC-Stufe-2).
- **Braucht:** übrige Arme nach dem Rat-Verdikt bauen oder `refused`/`quantity` registrieren.

### GIC-Stufe-2 — Receiver/force ENTSCHIEDEN; Receiver-Reader steht (river-138)
- **Status:** eigen | **Bindung:** eigen (Register) · river (`compute_max_t`)
- **Trigger:** AE/AL/AU · SME/SML/SMU als `sources.φ`-Zeilen gebaut
- **Lage:** (gemessen 2026-10-08 279) `quantity`/`index nt` (Operator-Wort „ja bitte"); `QuantityKind::Index` gebaut; die 6 `wdc_ae`-Zeilen (`sources.φ:4087-4092`) umgestellt. AE/AL/AU stehen; SME/SMU/SML: SuperMAG `blocked account` (`blocked_sources.φ:210-212`). `dB/dt-Bestand 2/154` (ABK `:2150-2168`, SOD `:2170-2178`). **river-138:** Receiver-Body mit `#aperture=` gebaut. **GIC-Stufe-2 Member-Pool:** als Register-Klassenträger zulässig, als ein Wire-Deskriptor verboten; River verdrahtet die drei Deskriptoren.
- **Blockade:** ob der GIC-Force-Deskriptor den Receiver-Reader (`DeclaredBody`) nutzt, ist ungemessen.
- **Braucht:** SME/SMU/SML über `blocked account` (SuperMAG-Login, Operator-Hand); den `DeclaredBody`-Reader am GIC-Deskriptor messen (river).

### Flyby-Kette — Doppler `pending`
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium
- **Trigger:** DSN/ESTRACK-Residualroute admittiert oder Descope-Befund
- **Lage:** (gemessen 2026-10-08 279) `SW_FAST_MAGA_LR_1B` registriert (`sources.φ:8153`). RTSW/ACE/Kp/OMNI2/Swarm/DSN-Power/JUICE registriert. Nicht registriert: Doppler (`src/archivar/doppler.rs` absent), σ_recon absent. Neuer Kanal: BepiColombo-Zenodo `17813314` (60-s, CC-BY-4.0).
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
- **Lage:** (gemessen 2026-10-08 277) Inventar `state/future/giftkarte-klassifiziert-src-2026-10-07.md` exists (gitignored), veraltet; die genannten „Ersatzmuster" sind gemessene Falsch-Positive (`astrometry.rs:10/336`, `odp.rs:9/48`).
- **Blockade:** welches Muster Hart-Block (Fixture) vs. Review = Rat; neues Inventar nötig.
- **Braucht:** Inventar auf den heutigen Baum nachführen (Produktions-`unwrap_or`/Default-Fill ohne Test-Region); Fixture-Kandidaten in `commit_gate_vocab.json` prüfen.

### HadISST SST — SOURCE_PORT gebaut, CI-Lauf offen
- **Status:** eigen | **Bindung:** mycelium (CDN)
- **Trigger:** `hadisst-cdn.yml`-Lauf grün
- **Lage:** (gemessen 2026-10-07 271) Compiler/Arm/Register/Workflow stehen, `cargo check` 0/0; Workflow noch nicht auf `origin/main` gelaufen.
- **Blockade:** Workflow nach Push dispatcht (79 MB Fetch).
- **Braucht:** `hadisst-cdn.yml` dispatchen; bei Erfolg `sha256`-Zeile nachziehen.

### `blocked_sources.φ`-Aufräumen — Klassen-Träger (`gap`-Token)
- **Status:** eigen | **Bindung:** eigen (Disposition) · mycelium (Diver-Tabelle)
- **Trigger:** Bau je Klassen-Träger
- **Lage:** (gemessen 2026-10-09 280) Klassen-Vollstreckung ausgeführt (`bd0e34fcf`): 213→130 Z., 20 Moves nach `ledger.φ` `ausstehend`, 11 `parser-def`-Retags + `gap`. Die drei gemeldeten Risse sind geschlossen: `limadou` (Dublette wartend.φ:5) entfernt; `vco_rs` (in `sources.φ:27800+` integriert) entfernt; `cluster_ka` (in `sources.φ:19486` integriert, generischer `asu-tsv`-Arm) entfernt. **Offen: 11 Klassen-Träger** `phi/blocked_sources.φ::gap:bc-mpo-more · tracking-doppler · mariner-rst · viking-tracking · juno-efb · dmap-map-grid · kaguya-lrs · leos-cses · themis-tail · mms-magnetosheath · aurora-keogram ×1` — je Arm/Zeile zu bauen oder `refused`/`pending` mit Trigger.
- **Blockade:** je Träger der Bau (Arm/Workflow/Register-Zeile).
- **Braucht:** je Träger Arm/Workflow/`sources.φ`-Zeile bauen oder Disposition registrieren.

### LEOS-Riss — `blocked_sources.φ` vs. Open-Sources-Delta
- **Status:** eigen | **Bindung:** eigen (Disposition)
- **Trigger:** Re-Messung der LEOS-Route
- **Lage:** (gemessen 2026-10-08 279) `phi/blocked_sources.φ` LEOS-Eintrag trägt `pending` (Zeile driftet); `survey-2026-10-08-open-sources-delta.md:75/133` misst `206` (user-gated).
- **Blockade:** Zeilen-Drift + zwei Fassungen.
- **Braucht:** LEOS-Zeile am Baum festnageln, `206 user-gated` als `pending`/Auth-Route registrieren (nie `declined`, Auth kein Ausschluss).

## An river

Origin: mountain-folge280.

- **`main_flow.rs` sauber:** der Fremd-Commit (river-138) ist gelandet. Der eigene Format-Listen-Hunk (`superdarn_cpcp`/`ssusi_aurora`/`wdc_ae`/`bpa_gic`) steht (277).
- **Receiver:** river-138 (`DeclaredBody` + `#aperture=`) — ob der GIC-Force-Deskriptor den Reader nutzt, ist die nächste Messung (GIC-Stufe-2).

## An mycelium

Origin: mountain-folge280.

- **`themis_mag`-CDN-Orphan (neu).** Der THEMIS-Routenwechsel ist ausgeführt (`2328b58a2`): `themis_mag` liest jetzt live CDAWeb HAPI `THG_L2_MAG_ABK` (Felder A=A `themis_gmag_h/e/z_nt`). Folge: das alte `themis_mag.bin`-Release-Asset und `tools/harvest/src/bin/themis_mag_compiler.rs` sind verwaist → aus dem CDN-Release entfernen (bzw. `themis-mag-cdn.yml` auf den CDAWeb-HAPI-Pfad umstellen).
- **`emtf-cdn.yml` neu dispatchen** — der EMTF-Format-Arm ist verdrahtet (`main_flow.rs` `emtf_impedance`), das Bin-Format ist 16-B-Header (Epoch); das alte CDN-Asset ist alt-Format.
- **KC2G-Parser geheilt** (`1ed930b82`, mountain-274): `kc2g_stations.rs` liest `station.latitude`/`longitude` — CDN-`failure`-Lauf obsolet, neu dispatchen.
- **`ci-check` verdrängt jeden Lauf** (`.github/workflows/ci-check.yml:18-20` `cancel-in-progress: false`); Fix nötig (IGRF-Witness).
- **`hadisst-cdn.yml`** dispatchen.
- **Route-Admissionen** manifestieren, sobald Mountain die Zeilen/Arme baut.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push (stehendes Wort 2026-10-07, Mountain 264).

Eigene Pfade: `src/archivar/parse.rs` · `src/archivar/tests.rs` · `src/archivar/superdarn_cpcp.rs` · `tools/harvest/src/bin/impc_roti_compiler.rs` · `phi/blocked_sources.φ` · `docs/handover/handover-2026-10-09-mountain-folge280.md` · `docs/handover/archiv/handover-2026-10-08-mountain-folge279.md` (Move).
