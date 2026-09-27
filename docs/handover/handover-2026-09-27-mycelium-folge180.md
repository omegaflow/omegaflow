<!--
  title: Handover — Mycelium-Folge 180 (2026-09-27)
  session: Mycelium-Folge 180
  class: handover
  date: 2026-09-27
  sha256: cf93430026ef7404ab95936838bbeda044337e0d3d97754fa3a23d970ae82198
  status: live
-->
# Handover — Mycelium-Folge 180 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `wartend` | `operator-gebunden` |
`blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-27-mycelium-folge179.md`.

## Operator-Wort-Register

- Wort | 2026-09-27 | „nein bitte so festschreiben" — Rundenordnung (Mycelium→Sensory→Mountain→River→Future) + Spirale in `docs/concepts/kybernaut-native-methodology.md`, mechanik-only.
- Wort | 2026-09-27 | „bitte ausführen" — Ownership-Audit (Aufenthalt = Eigentum): planeto-epncore-Direktiven + CDN-Reconcile-Dispatch → Mycelium, clippy-Owner je Linie, Beschaffungs-LOCKs aus Mountain/River entfernt (kanonisch Future).
- Wort | 2026-09-27 | Secrets-Inventar-Träger prüfen: direkter Edit in die Owner-Übergabe (Messung ergab keinen Register-Akt) — erledigt (Verdict, Survey geschlossen).

## Offen (aufgeschlüsselt)

### DE441 — JPL-Kernel registriert, de44-cdn-Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `de44-cdn`-Lauf grün.
- **Lage:** (gemessen 2026-09-27 via `sgrep` + `curl -sI`) die folge179-These „zwei JPL-Editionen nicht registriert" war falsch — DE442 war registriert (`phi/sources.φ` `ephemeris_de442_{earth,moon,sun}`, ttl 86400); es fehlte allein DE441. Jetzt registriert: 3 Blöcke `ephemeris_de441_{earth,moon,sun}` (origin = beide NAIF-Teile, `de_compiler.rs`), `de44-cdn.yml`-Step ergänzt; alle drei NAIF-URLs 200.
- **Blockade:** keine.
- **Braucht:** `ci_manage view <id>` des dispatchten `de44-cdn`; bei success sha256 in die de441-Blöcke + Gate-Bin für δ = max|x_DE441(t) − x_DE442(t)| (`docs/paper/flyby-path-2-preregistration-revised.md:65`).

### GOSAT-GW GWT3F_L1B — 4096-Granule-Gate
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `gosat-cdn` `36309708111` abgeschlossen.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view`) in_progress; Job `raster` success, Zweitjob ohne Log (`ci_manage log --all` → 404) → Lauf nicht abgeschlossen. Manifest `…/gosat_tanso3.manifest` 404 (noch nicht publiziert); `gosat_tanso3`-Block `phi/sources.φ:10781` trägt keine `sha256`-Zeile.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36309708111`; bei success Manifest/sha256 verifizieren, `sha256`-Zeile in den Block.

### clippy `-D warnings` — extract.rs
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`-Lauf am HEAD `ab0a1faa` (`36315178308`, pending) lesbar.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36310976945`, HEAD `452d406`) 11 extract.rs-Warnungen: `manual_is_multiple_of` ×2 (`:132`,`:1317`), `chunks_exact_to_as_chunks` ×2 (`:136`,`:2101`), `question_mark` ×5 (`:1356`,`:1990`,`:1994`,`:1997`,`:2262`), `manual_div_ceil` ×2 (`:2057`,`:2223`).
- **Blockade:** HEAD-Lauf pending (kein lokales clippy).
- **Braucht:** `ci_manage log 36315178308` → die genannten Zeilen heilen.

### SECRETS-CLEANUP — 7 ungenutzte Namen entfernen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-27 via `sgrep` + GH-API) GOSAT_GW_MAIL/PASS genutzt (behalten, `gosat-cdn.yml:29-30`); IGETS2_USER/PASS Vorrat (behalten, Quelle registriert); zu entfernen: `GFW_PASS`, `MOVEBANK_PASS`, `MOVEBANK_TOKEN`, `RUBIN_PASS` (declined/descoped) + `BABAMUL_KAFKA_PASSWORD`/`BABAMUL_KAFKA_USERNAME`/`BABAMUL_PASSWORD` (überholt durch `BABAMUL_API_TOKEN`).
- **Blockade:** keine.
- **Braucht:** die 7 Zeilen aus `.secrets.local` entfernen (Werte werden nie gelesen/gedruckt).

### Blatt-1-Bojen-Matrix — matrix-rotor 143 (getragen von Sensory)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `matrix-rotor`-Rerun — die Instrumentierung (`matrix-rotor.yml:63`) läuft zuvor.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view`/`log` + `gh api`) kein Cancel: `conclusion=failure` (nicht cancelled), `cancelled_at`/`stopped_at` = null; Actor `omegaflow` = Dispatcher. Kill-Quelle = Runner-Shutdown (Log: `The runner has received a shutdown signal`, `Terminate orphan process: pid (4158) (timeout)`), extern/GitHub.
- **Blockade:** die SIGTERM-Quelle liegt außerhalb des Job-Logs (Runner-Dienst).
- **Braucht:** `matrix-rotor.yml:63` instrumentieren (`trap … TERM` + `timeout --verbose`, sichtbar im Artifact) — CI-Workflow, dann Rerun.

### modis-cdn — Release-Cap (1000 Assets), Asset-Brücke vorbereitet
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Asset-Brücke gebaut + `modis-cdn`-Re-Dispatch.
- **Lage:** (gemessen 2026-09-27 via Job-Logs + grind-flash) CDN-Release `data.lpdaac.earthdatacloud.nasa.gov` hat 1000 Assets → `HTTP 422`; `modis_lst_cmg_8day`/`_monthly.manifest` fehlen; Brücke braucht Download→Re-Upload (GitHub kein Server-Copy), die `delete`-Stufe ist destruktiv.
- **Blockade:** Brücken-Akt + destruktive Freigabe = Operator-Hand/CI-Job.
- **Braucht:** Einmal-Job (5 Schritte, in folge179:55); dann `CDN_TAG`-Wechsel + `modis-cdn.yml` + `phi/sources.φ`.

### auto-dispatch — Workflow-Dispatch (Fix gebaut, Lauf offen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Push, der `tools/harvest/src/bin/**` berührt → neuer `auto-dispatch`-Lauf.
- **Lage:** (gemessen 2026-09-27 via `ci_manage list`) letzter Lauf `36302092969` (07:06) failure; `.github/workflows/auto-dispatch.yml` feuert nur auf `push` (kein `workflow_dispatch`), der Fix (`--include` vor `--`, Inputs mit `default`) ist seither nicht ausgelöst.
- **Blockade:** kein Trigger.
- **Braucht:** nächsten Harvest-Bin-Push abwarten; `ci_manage list` auf `auto-dispatch` grün.

### Voyager 1/2 closed-loop Doppler — Anfrage läuft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort von `gsfc-dl-nssdca-request@mail.nasa.gov` (PSNO-00007).
- **Lage:** (gemessen 2026-09-27 via `state/zustand/wartend.φ`) Anfrage 2026-09-16 13:47 UTC gesandt; kein Eingang im Mail-Ledger.
- **Blockade:** keine.
- **Braucht:** Trigger in `state/zustand/wartend.φ` (voyager-nssdca).

### CDN-Reconcile — Manifestations-Dispatch (getragen von Mountain)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `cdn-reconcile` `36311386072` success — Mountains Verdikt liegt vor (2026-09-27).
- **Lage:** (gemessen 2026-09-27 via `cdn_reconciliation.json` + Mountain-Verdikt) 4 akzeptiert: `ssd.jpl.nasa.gov-dcom5` → `dcom5-cdn.yml`, `ssd.jpl.nasa.gov-icecat` → `icecat-cdn.yml`, `ssd.jpl.nasa.gov-weberin` → `weberin-verdicts-cdn.yml`, `ned.ipac.caltech.edu-byparams` → `ned-byparams-cdn.yml`; `noaa-nos-coastal-lidar-pds.s3.amazonaws.com` disponiert (Mountain löschte die Quelle, `decline terrain`).
- **Blockade:** keine.
- **Braucht:** die 4 `*-cdn.yml` dispatchen (`gh workflow run <wf>`); danach `cdn-reconcile.yml` und prüfen, dass die 4 aus `unmanifested_source_netlocs`/`missing_assets` fallen. Die verwaisten coastal-lidar-Referenzen in `phi/pipeline/frame_registry.φ:508`/`noaa_nodd_disposition.φ:40` prunen.

### termin-Punkte — re-verdict
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27) `pithia.cbk.waw.pl/tap` 200 (external-state.md); `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine (Wiedervorlage).
- **Braucht:** `archive_search --verdict <url>`; bei Erholung `*-cdn.yml` dispatchen.

### BepiColombo bc_mpo_more — Termin 2027-04-01
- **Status:** termin | **Bindung:** termin:2027-04-01
- **Trigger:** 2027-04-01 (Science-Phase-Beginn).
- **Lage:** (gemessen 2026-09-27 via `state/zustand/wartend.φ`) MORE-Cruise nicht öffentlich, Freigabe April 2027; Ticket `YYM-342-97327`; Freigabe-Anfrage `phi/blocked_sources.φ:53`.
- **Blockade:** Freigabe (dritter).
- **Braucht:** Wiedervorlage 04/2027.

## Träger (Prosadokumente)

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | `mariner-occlt`-CDN-Dispatch geschlossen; offen: native ODF-Serien-Arm + vier request-only-Routen | nächster Schritt: `archive_search --verdict` beim Trigger.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | SAMPLE_CONTACT (MPI-FKF/LAB_A) sagte zu, danach kein Eingang | wartend auf Mail-Eingang (kein Nachfassen).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | nur `Wiedervorlage 2026-12-02` bindet | nächster Schritt: 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending weiter tot | nächster Schritt: `--verdict` je Host beim Trigger.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offen: Step 5 (CDN-kanonisch, destruktiv → Operator-Wort) | nächster Schritt: Klassen-Zensus messen.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Migrationsplan-Vorlage steht; stoppt am Operator-Wort | nächster Schritt: Operator-Wort zum Layout `knowledge/`+`backups/`.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen nur §7 Roh-Korpora/Scratch-Disposition | nächster Schritt: Operator-Wort.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
