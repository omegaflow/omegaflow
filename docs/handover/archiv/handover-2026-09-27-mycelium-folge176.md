<!--
  title: Handover — Mycelium-Folge 176 (2026-09-27)
  session: Mycelium-Folge 176
  class: handover
  date: 2026-09-27
  sha256: bfec91c590c28a8c26eaffada858c14cb31ad3fe86a57531e1306d3165d6fbcb
  status: live
-->
# Handover — Mycelium-Folge 176 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `wartend` | `operator-gebunden` |
`blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-27-mycelium-folge175.md`.

## Operator-Wort-Register

- Wort | 2026-09-26 | „all" — session-weiter Consent (`mycelium_go`), Delegation an alle Taucher.
- Wort | 2026-09-27 | „DEMETER ist Sensory" → DEMETER bleibt aus der Mycelium-Übergabe (Sensory führt ihn).
- Wort | 2026-09-27 | „du machst GOSAT" → GOSAT bleibt Mycelium-Punkt.
- Wort | 2026-09-27 | „monthly + 8-day als per-Granule-Serie bauen; daily descopen" → modis-cdn per-Granule + Manifest; daily descoped.
- Wort | 2026-09-27 | „volume" — gibt die φ-Direktive `volume` (3D-Gitter-Ingest) frei.
- Wort | 2026-09-27 | „/consent" (Phase-1-Plan bestätigt) — session-weiter Delegations-Consent; nicht das Commit-Wort.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### GOSAT-GW GWT3F_L1B — Sharding-Fix gebaut, Re-Dispatch offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `gosat-cdn`-Lauf grün (nach Re-Dispatch am Fix).
- **Lage:** (gemessen 2026-09-27 via `ci_manage view 36280012910`/`log`) Lauf `36280012910` failure: das Fenster `2024-01-01..2026-12-31` überschreitet den 3000-Treffer-Server-Cap → HTTP 200 `results:{}` + `messages.summary`; `search_parse` verschluckte die Summary via `?` (Diagnose unreachable). Fix gebaut: rekursives Fenster-Sharding + Summary-Diagnose in `gosat_tanso3_compiler.rs`, `cargo check` 0/0.
- **Blockade:** keine.
- **Braucht:** nach Push `gh workflow run gosat-cdn.yml -f product=GWT3F_L1B -f start=2024-01-01 -f end=2026-12-31`; dann `ci_manage view <id>` bei Abschluss; bei success `sha256` in `phi/sources.φ` + `phi/blocked_sources.φ` → released.

#### modis-cdn — Serien-Manifest-Job gebaut, Jahres-Manifeste offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `modis-cdn`-Läufe grün.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view`) `36278484232` in_progress, `36278795720` pending; Jahres-Manifeste `8day` 2000–2023 (24), `monthly` 2000–2003 (4) auf CDN; jahrloses Serien-Manifest fehlt. `series-manifest`-Job in `.github/workflows/modis-cdn.yml` gebaut (faltet die Shard-Assets in `modis_lst_cmg_8day.manifest`/`modis_lst_cmg_monthly.manifest`).
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36278484232` bei Abschluss; nach Push `gh workflow run modis-cdn.yml`; dann `archive_search --sniff` der beiden Serien-Manifeste.

#### IRIS/EarthScope EMC netCDF — Live-`volume_netcdf`-Arm offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `value_key`-Messung + CI-Manifestation.
- **Lage:** (gemessen 2026-09-27 via `archive_search`/`sfetch`) Earth Model Collaboration (nicht magnetotellurisch): dir + `.nc` HTTP 200, HDF5, anonym; Lizenz **CC BY 4.0**; die Non-Human-Visitor-Klausel ist keine Login-Wand. Roh-`.nc` als `format reference` registriert (`phi/sources.φ:1298+`, mit sha256), `volume.bin` auf CDN; Ingest-Arm gebaut (`88fb49209`). Offen: der Live-Arm `volume_netcdf`.
- **Blockade:** keine (ToS gedeckt; CDN-Write = eigene Domäne).
- **Braucht:** `value_key` (Rang-3-Datenvariable) je Modell aus dem netCDF-Header lesen; dann `volume_netcdf`-Block (`volume <name> <value_key>`/`lat`/`lon`/`depth`) + CI-Manifestation.

#### EPN-core `epncore-spatial` — Daten erreichbar, Parser-Arm fehlt
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** `epncore-spatial`-Parser-Arm (mountain).
- **Lage:** (gemessen 2026-09-27 via `archive_search --verdict`) `http://pithia.cbk.waw.pl/tap` direct 200 (13 342 B) / proton 200 — Daten erreichbar; `/tap/tables` 500 (PostgreSQL-Backend); Wayback-Snapshot 20250424.
- **Blockade:** Parser-Arm `epncore-spatial` fehlt (mountain); kein lat/lon-Skalar (Region c1/c2/c3 + s_region STC-S).
- **Braucht:** mountain baut den Arm; danach Re-Check.

#### EMODNET HFRADAR NADR — Termin-Re-Messung
- **Status:** termin | **Bindung:** termin:2026-10-19
- **Trigger:** 2026-10-19.
- **Lage:** (gemessen 2026-09-24 via `external-state.md:44`) Asset registriert; Re-Messung offen.
- **Braucht:** `archive_search --sniff https://github.com/omegaflow/sources/releases/download/erddap.emodnet-physics.eu/emodnet_hfr_nadr.bin` bei Fälligkeit.

#### termin-Punkte — re-verdict
- **Status:** termin | **Bindung:** termin:2026-09-28 / 2026-10-02 / 2026-12-02
- **Trigger:** 2026-09-28 (DEMETER) / 2026-10-02 (übrige) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27 via `archive_search --verdict`) `regards.cnes.fr/api/v1/rs-order` weiter 403 direct+proton (Host-Root 200); `pithia.cbk.waw.pl/tap` **200** (erholt); `api.lasair.lsst.ac.uk/api` direct absent / **proton 200** (erholt via Exit).
- **Braucht:** `archive_search --verdict <url>`; bei Erholung den `*-cdn.yml`-Lauf dispatchen (pithia: `epncore`; lasair: Proton-Route prüfen).

#### Secrets-Inventar — 6 Lebendquellen ohne `phi`-Disposition
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `phi/sources.φ` Eintrag je Quelle.
- **Lage:** (gemessen 2026-09-27 via `archive_search --verdict`) 6 in `.secrets.local` benannte, im getrackten Baum nirgends registrierte Lebendquellen: GFW `https://data-api.globalforestwatch.org` direct 200 · GOSAT-GW `https://gosat-gw.nies.go.jp/en/` direct 200 (Homepage gefunden, Survey-`pending` gelöst) · IGETS `http://igets.u-strasbg.fr` direct 200 (https pending — nur Port 80) · Rubin `https://rubinobservatory.org` direct 200 · Babamul `https://api.babamul.dev` pending (health/docs kein Response; Basis `babamul.dev` pending) · Movebank `https://www.movebank.org` direct 200. Survey: `docs/surveys/survey-2026-09-26-secrets-inventar.md`; Träger: Future `handover-2026-09-27-future-folge137.md`.
- **Blockade:** keine.
- **Braucht:** je Quelle eine `phi/sources.φ` Zeile (IGETS-URL auf `http` korrigieren) oder `declined_sources.φ`; Babamul `pending` belassen.

### Dritter

#### BepiColombo bc_mpo_more — Termin
- **Status:** termin | **Bindung:** termin:2027-04-01
- **Trigger:** 2027-04-01 (Science-Phase-Beginn).
- **Lage:** (gemessen 2026-09-26) MORE-Cruise nicht öffentlich, Freigabe April 2027; Ticket `YYM-342-97327`.
- **Braucht:** Wiedervorlage 04/2027, dann MORE-`data_raw`/`calibration_raw`.

#### SSDC / Limadou — wartend auf CSES-02-Prozedur
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur `limadou.ssdc.asi.it`.
- **Lage:** (gemessen 2026-09-26 via `sfetch`) `query.php` → SSDC-CAS-Login; neue Prozedur noch nicht live. Sotgiu (`mail_ledger.φ`): „wait a few weeks".
- **Blockade:** Portal-Umbau (dritter).
- **Braucht:** neue Prozedur abwarten; beim Trigger `query.php` re-messen.

## Träger (Prosadokumente)

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | `mariner-occlt` Asset liegt | nächster Schritt: verbleibende Survey-Marker + `mariner-occlt`-CDN-Dispatch.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | SAMPLE_CONTACT (MPI-FKF/LAB_A) sagte LAB_A-`I(q,t)`-Daten zu, danach kein Eingang | wartend auf Mail-Eingang (kein Nachfassen).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | nur `Wiedervorlage 2026-12-02` bindet; übrige Body-Marker sind Nachzug-Staleness | nächster Schritt: 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending weiter tot | nächster Schritt: `--verdict` je Host beim Trigger.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | Step 4 (CI-Dedupe) in `docs/auftrag/archiv/auftrag-saubere-datenbank.md`; offen: Step 5 (CDN-kanonisch, destruktiv → Operator-Wort) | nächster Schritt: Klassen-Zensus messen.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Migrationsplan-Vorlage steht; stoppt am Operator-Wort | nächster Schritt: Operator-Wort zum Layout `knowledge/`+`backups/`.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen nur §7 Roh-Korpora/Scratch-Disposition (NOAA-NRS entschieden) | nächster Schritt: Operator-Wort.

## Abschluss

Commit-Wort (`/commit`) offen — Phase 1/2 lief unter `/consent` (Delegation).
