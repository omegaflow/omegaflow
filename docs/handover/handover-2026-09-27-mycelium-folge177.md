<!--
  title: Handover — Mycelium-Folge 177 (2026-09-27)
  session: Mycelium-Folge 177
  class: handover
  date: 2026-09-27
  sha256: e170cc9ab6748bb3bc8934bbb5733a003bb33618c3872bf73d5a22e47564bea5
  status: live
-->
# Handover — Mycelium-Folge 177 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `wartend` | `operator-gebunden` |
`blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-27-mycelium-folge176.md`.

## Operator-Wort-Register

- Wort | 2026-09-26 | „all" — session-weiter Consent (`mycelium_go`), Delegation an alle Taucher.
- Wort | 2026-09-27 | „DEMETER ist Sensory" → DEMETER bleibt aus der Mycelium-Übergabe (Sensory führt ihn).
- Wort | 2026-09-27 | „du machst GOSAT" → GOSAT bleibt Mycelium-Punkt.
- Wort | 2026-09-27 | „monthly + 8-day als per-Granule-Serie bauen; daily descopen" → modis-cdn per-Granule + Manifest; daily descoped.
- Wort | 2026-09-27 | „volume" — gibt die φ-Direktive `volume` (3D-Gitter-Ingest) frei.
- Wort | 2026-09-27 | „Du kannst" (Phase-2-Ausführung bestätigt) — session-weiter Delegations-Consent; nicht das Commit-Wort.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### GOSAT-GW GWT3F_L1B — Void/Overflow-Fix gebaut, Re-Dispatch nach Push
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `gosat-cdn`-Lauf grün (nach Re-Dispatch am Fix).
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36283215548`) der Sharding-Fix wirkte (Split am 3000-Cap); der Lauf riss am neuen Fehler: die Zero-Result-Summary als `Overflow` klassifiziert → die leere Fensterhälfte bis auf einen Tag bisektiert → `Void => None` + `?` brach die Rekursion ab, die rechte Hälfte unbearbeitet. Fix gebaut: `search_parse` trennt `Empty` (null-echt) von `Overflow` (Cap) von `Void` (unbekannt); `search_granules` `Empty => Some(vec![])`; `cargo check -p omegaflow-harvest --bin gosat_tanso3_compiler` 0/0.
- **Blockade:** keine.
- **Braucht:** nach Commit+Push `gh workflow run gosat-cdn.yml -f product=GWT3F_L1B -f start=2024-01-01 -f end=2026-12-31`; dann `ci_manage view <id>`/`log <id>`; bei success sha256 in `phi/sources.φ` + `phi/blocked_sources.φ` → released.

#### volume_netcdf — Parity-Gate gebaut, CI-Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `volume-cdn.yml`-Lauf (gypsump-Job) grün.
- **Lage:** (gemessen 2026-09-27) `volume_builder --verify-live` gebaut — ruft `build_netcdf4_volume` auf denselben Bytes, gated dims/axes/data (tol `1e-6·max(|a|,|b|,1)`; `mask` bewusst nicht verglichen, der offline-Arm kennt `_FillValue` nicht); Schritt im `gypsump`-Job (`vp`, eindeutige Rang-3-Variable); `cargo check -p omegaflow-utils --bin volume_builder` 0/0. Der Arm war 0× im Register genutzt — der Lauf ist seine erste reale Übung.
- **Blockade:** keine.
- **Braucht:** `gh workflow run volume-cdn.yml`; `ci_manage log <id>` — Zeile `live parity holds` oder benannter mismatch.

#### modis-cdn — Serien-Manifest-Job, Lauf läuft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `modis-cdn`-Lauf `36283216818` abgeschlossen (extern).
- **Lage:** (gemessen 2026-09-27 via `ci_manage list`/`view`) `36283216818` (b8a9ae421) in_progress: raster ✓, release ✓, 8day 2000–2024 ✓, 2025/2026 + monthly 2006–2021 rot, monthly 2022–2026 in Queue; der `series-manifest`-Job (`modis-cdn.yml:105`, `needs:[raster,compile]`) noch nicht gestartet; `modis_lst_cmg_8day.manifest` + `_monthly.manifest` unter Tag `data.lpdaac.earthdatacloud.nasa.gov` absent (1001 Assets, nur Jahr-Manifeste).
- **Blockade:** keine (Lauf läuft; der CI-Watchdog pollt).
- **Braucht:** bei Abschluss `ci_manage view 36283216818`/`log`; dann `archive_search --sniff https://github.com/omegaflow/sources/releases/download/data.lpdaac.earthdatacloud.nasa.gov/modis_lst_cmg_8day.manifest` (+ `_monthly.manifest`).

#### secrets-inventar — Prosa-Träger, 6 Lebendquellen-Disposition
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `phi/sources.φ` bzw. `phi/declined_sources.φ` Disposition der 6 Lebendquellen gesetzt.
- **Lage:** (gemessen 2026-09-27 via `register_lookup --orphan-docs` + `state/funding/handover/handover-2026-09-27-future-folge137.md`) der Träger steht in der Future-Übergabe (`### Prosa-Träger — Secrets-Inventar`, Bindung `linie:mycelium`); der `§Offen`-Marker des Surveys ist offen.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict` je der 6 Quellen (GFW, GOSAT-GW, IGETS, Rubin, Babamul, Movebank); danach Disposition in `phi/sources.φ` bzw. `phi/declined_sources.φ`; dann `docs/surveys/survey-2026-09-26-secrets-inventar.md` §Offen schließen.

#### Voyager 1/2 closed-loop Doppler — Anfrage läuft, Register-Note korrigiert
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort von `gsfc-dl-nssdca-request@mail.nasa.gov` (PSNO-00007).
- **Lage:** (gemessen 2026-09-27 via `state/mail/mail_ledger.φ:74`) die Anfrage wurde 2026-09-16 13:47 UTC gesandt (die frühere `(:207)`-Referenz war stale); `blocked_sources.φ:54`-Note auf `→ Antwort offen` korrigiert (Tag `[wartend]`).
- **Blockade:** keine.
- **Braucht:** Postfach bei Trigger (`smail` + `state/mail/mail_ledger.φ`).

### Dritter

#### EPN-core `epncore-spatial` — Parser-Arm fehlt
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** `epncore-spatial`-Parser-Arm (mountain).
- **Lage:** (gemessen 2026-09-27 via `archive_search --verdict`) `http://pithia.cbk.waw.pl/tap` direct 200 (13 342 B); `/tap/tables` 500; Region c1/c2/c3 + `s_region` STC-S, kein lat/lon-Skalar; `blocked_sources.φ:344` `[mountain] parser-def`.
- **Blockade:** Parser-Arm fehlt (mountain).
- **Braucht:** mountain baut den Arm; danach Re-Check.

#### SSDC / Limadou — wartend auf CSES-02-Prozedur
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur `limadou.ssdc.asi.it`.
- **Lage:** (gemessen 2026-09-26 via `sfetch`) `query.php` → SSDC-CAS-Login; Prozedur nicht live; Sotgiu: „wait a few weeks"; `ledger.φ:14` `[mycelium] ausstehend`.
- **Blockade:** Portal-Umbau (dritter).
- **Braucht:** neue Prozedur abwarten; beim Trigger `query.php` re-messen.

### Termin

#### EMODNET HFRADAR NADR — Re-Messung
- **Status:** termin | **Bindung:** termin:2026-10-19
- **Trigger:** 2026-10-19.
- **Lage:** (gemessen 2026-09-24 via `external-state.md:44`) Asset registriert, Re-Messung offen.
- **Blockade:** keine (Wiedervorlage).
- **Braucht:** `archive_search --sniff https://github.com/omegaflow/sources/releases/download/erddap.emodnet-physics.eu/emodnet_hfr_nadr.bin` bei Fälligkeit.

#### termin-Punkte — re-verdict
- **Status:** termin | **Bindung:** termin:2026-09-28 / 2026-10-02 / 2026-12-02
- **Trigger:** 2026-09-28 (DEMETER) / 2026-10-02 (übrige) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27 via `archive_search --verdict`) `regards.cnes.fr/api/v1/rs-order` 403 direct+proton; `pithia.cbk.waw.pl/tap` 200; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine (Wiedervorlage).
- **Braucht:** `archive_search --verdict <url>`; bei Erholung `*-cdn.yml` dispatchen.

#### BepiColombo bc_mpo_more — Termin
- **Status:** termin | **Bindung:** termin:2027-04-01
- **Trigger:** 2027-04-01 (Science-Phase-Beginn).
- **Lage:** (gemessen 2026-09-26) MORE-Cruise nicht öffentlich, Freigabe April 2027; Ticket `YYM-342-97327`; `blocked_sources.φ:53` Freigabe-Anfrage an psahelp.
- **Blockade:** Freigabe (dritter).
- **Braucht:** Wiedervorlage 04/2027.

## Träger (Prosadokumente)

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | `mariner-occlt`-CDN-Dispatch geschlossen (gemessen); offen: native ODF-Serien-Arm + die vier request-only-Routen | nächster Schritt: `archive_search --verdict` beim Trigger.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | SAMPLE_CONTACT (MPI-FKF/LAB_A) sagte LAB_A-`I(q,t)`-Daten zu, danach kein Eingang | wartend auf Mail-Eingang (kein Nachfassen).
- `docs/surveys/survey-2026-09-26-secrets-inventar.md` | Namens-Disposition — Träger steht in `future-folge137`; Arbeit reist an mycelium | nächster Schritt: siehe eigener Punkt.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | nur `Wiedervorlage 2026-12-02` bindet; übrige Body-Marker sind Nachzug-Staleness | nächster Schritt: 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending weiter tot | nächster Schritt: `--verdict` je Host beim Trigger.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offen: Step 5 (CDN-kanonisch, destruktiv → Operator-Wort) | nächster Schritt: Klassen-Zensus messen.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Migrationsplan-Vorlage steht; stoppt am Operator-Wort | nächster Schritt: Operator-Wort zum Layout `knowledge/`+`backups/`.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen nur §7 Roh-Korpora/Scratch-Disposition | nächster Schritt: Operator-Wort.

## Abschluss

Commit-Wort (`/commit`) offen — Phase 1/2 lief unter `/consent`/„Du kannst" (Delegation).
