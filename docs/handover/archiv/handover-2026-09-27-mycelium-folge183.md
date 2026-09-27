<!--
  title: Handover — Mycelium-Folge 183 (2026-09-27)
  session: Mycelium-Folge 183
  class: handover
  date: 2026-09-27
  sha256: 424a0ef7d45597a79a076479490ce90cab75b05eb30b05f2693fb62390f53e2d
  status: live
-->
# Handover — Mycelium-Folge 183 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` |
`termin`; Operator-Akte leben in Futures Operator-Queue, nie als Linien-Punkt.

Diese Session konsumierte `handover-2026-09-27-mycelium-folge182.md`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`)
— zitiert, nie in dieses Register kopiert.

## Operator-Wort-Register

- Wort | 2026-09-27 | „kümmer dich drum" — der verwaiste rustfmt-Fix `tools/utils/src/bin/omega_sh.rs` fällt Mycelium zu (Aufenthalt = Eigentum); Commit trägt `/commit`.
- Wort | 2026-09-27 | „Du kannst. Führe den … Plan aus — als `line`-Agent" — session-weiter Consent (Delegation), **nicht** das Commit-Wort.
- Wort | 2026-09-27 | „die Kante bin ich" — jede Linie arbeitet bis zur Kante des Operators; Wert, Wort, Dritt-Akt und Send bleiben seine Hand | Operator (Future-Session).
- Wort | 2026-09-27 | ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben — Verbreitung im selben Atom | Operator (Future-Session).
- Wort | 2026-09-27 | RX100-Kalibrierer descoped — „über exif weg": Luminanz über den Kamera-EXIF-Weg (K=12.5) | Operator (Future-Session).
- Wort | 2026-09-27 | Entscheidungen nie als Liste vorlegen — eine Liste ist keine Entscheidungshilfe; jede Entscheidung braucht eine aussagekräftige Erklärung | Operator (Future-Session).
- Wort | 2026-09-27 | UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | Operator (Session, Mountain).
- Wort | 2026-09-27 | D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | Operator (Session, Mountain).
- Wort | 2026-09-27 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent" — session-weiter Consent der Folge 183 (Delegation), nicht das Commit-Wort | Operator (Mycelium-Session 183).
- Wort | 2026-09-27 | RX100 war nur ein Gedanke — Quelle zurückgezogen, Workflow-Zweige entfernen | Operator (Session, Mountain).

## Offen (aufgeschlüsselt)

### gosat-cdn — 4096-Granule-Bound + 2025-Default (Verifikation)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `gosat-cdn` `36322845122` / `36322848137` abgeschlossen (am HEAD `36b559852`).
- **Lage:** (gemessen 2026-09-27 via `ci_manage view`) `36322845122` in_progress, `36322848137` pending, beide am `36b559852`; Fix (`MAX_SEARCH_RESULTS` 1<<12→1<<14, `start_year` 2025) steht in `36b559852`.
- **Blockade:** keine (CI läuft/queued auf altem HEAD).
- **Braucht:** `ci_manage view 36322845122`; Manifest `gosat_tanso3.manifest` + sha256 prüfen.

### ci-check — clippy extract.rs + format (Verifikation)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` am HEAD `e376f73b0` beendet.
- **Lage:** (gemessen 2026-09-27 via `ci_manage list`) die Verifikationsläufe `36322836780`/`36323627215` sind cancelled (durch Push überholt, kein Rot); jüngster `ci-check` am HEAD `e376f73b0` ist `36324738298` pending. Fix in `266a85144` (11 extract.rs-Stellen); rustfmt-Fix `omega_sh.rs` in `f92d4fa3a`.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36324738298` (Jobs `format`/`clippy`); bei Rot `ci_manage log 36324738298`.

### matrix-rotor — SIGTERM-Quelle runner-seitig
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** nächster `matrix-rotor`-Lauf oder ein Design-Wort (kürzerer Slice / stabiler Runner).
- **Lage:** (gemessen 2026-09-27 via `ci_manage view` + `ci_manage log 36320306259`) attempt 2 completed failure; Log: `The runner has received a shutdown signal` + `The operation was canceled` ~50 s nach Slice-Start (13:41:42), Trap `matrix-rotor.yml:62` feuerte nicht; attempt 1 dito. Der 18000-s-Slice auf GitHub-Hosted-Runnern wird zweimal preempted.
- **Blockade:** Runner-Infrastruktur (framework-/runner-seitig, außerhalb des Job-Logs).
- **Braucht:** Design-Entscheidung (Rat/Operator): kürzerer Slice oder stabiler Runner; dritter Shutdown → `descoped` mit Befund.

### hinet-cdn — CONT-Readiness nie `Available`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `hinet-cdn 36323256126` beendet.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view`) in_progress; Vorgänger-Log `36309711883`: `hinet: cont status never read Available — the request stays unfetched`; Auth öffnete (`/auth/?LANG=en`, 200), `attempt 0..7 stayed unready`.
- **Blockade:** quellenseitige Readiness (Hinet bereitet die CONT-Anfrage nicht bereit).
- **Braucht:** `ci_manage view 36323256126`; bleibt es rot → `wartend` auf Hinet-Readiness (Trigger: periodischer Re-Dispatch).

### modis-cdn / modis-asset-bridge — Release-Cap (Verifikation)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `36320551698` (bridge) und `36320557426` (cdn) abgeschlossen (am HEAD `edfcedad`).
- **Lage:** (gemessen 2026-09-27 via `ci_manage view` + `SGrep`) bridge in_progress, cdn queued, beide am `edfcedad`; die destruktive `delete`-Stufe ist **nicht** Teil dieser Läufe. **Operator-Wort 2026-09-27: „löschen"** (aus Future-Queue 18) — nach grüner Verifikation die bare Slots löschen (`modis-asset-bridge.yml:6`).
- **Blockade:** keine (CI läuft/queued).
- **Braucht:** `ci_manage view 36320551698` / `36320557426`; bei success CDN_TAG-Wechsel + `modis-cdn.yml` + `phi/sources.φ`.

### ncdc-CDN-`www.`-Orphan löschen (Operator-Wort 2026-09-27)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen Future 142, 2026-09-27) `…/www.ncdc.noaa.gov/ncdc.noaa.gov/noaa_cdo_ghcnd_tmax.bin` = 8392 B, sha `0071dcc0…c21e4a8c`, 262 Records, **kein Register-/Compiler-Eintrag** (Orphan; `docs/specs/cdn_reconciliation.json:9` führt beide als `duplicate_netloc_tags`). Bare `…/ncdc.noaa.gov/…` = 7912 B, sha `1f7b4dcb…c4f314f3`, 247 Records — kanonisch (`phi/sources.φ:2416`, Compiler `noaa_cdo_compiler.rs` schreibt nur das bare Tag). Operator-Wort: „löschen" (`www.` bleibt entfernt, bare kanonisch).
- **Blockade:** keine.
- **Braucht:** den `www.`-Zwilling im CDN entfernen (destruktive Stufe), danach `cdn-reconcile` messen.

### solar-system-open-data Key als GitHub-Secret (Operator-Wort 2026-09-27)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-27) lokaler Key gültig (HTTP 200); Operator-Wort „natürlich eintragen".
- **Blockade:** keine.
- **Braucht:** `SOLAR_SYSTEM_OPEN_DATA_KEY` als GitHub-Secret für `.github/workflows/solar-system-open-data-cdn.yml` setzen; den CDN-Auto-Abgleich (`cdn-reconcile`) nutzen.

### dropped-gate — 34 offene Punkte ohne auflösenden Commit
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` am HEAD `e376f73b0` beendet (`36324738298`).
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36324206113`) Lauf `36324206113` (auf `5f535b4c6`): Job `dropped-gate` failure — `baseline 989 | current 1023 | delta 34`; 34 offene Punkte fielen ohne auflösenden Commit; Issue „health: dropped-gate returned void" offen.
- **Blockade:** keine.
- **Braucht:** am HEAD `target/release/register_lookup --dropped --count` messen; Punkte per Aufenthalt ins Owner-Handover tragen oder die Baseline `docs/zustand/dropped-baseline.md` im annehmenden Commit bumpen.

### Voyager 1/2 closed-loop Doppler — keine Antwort
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort von `gsfc-dl-nssdca-request@mail.nasa.gov` (PSNO-00007).
- **Lage:** (gemessen 2026-09-27 via `state/zustand/wartend.φ`) kein Eingang; Anfrage 2026-09-16 13:47 UTC.
- **Blockade:** keine.
- **Braucht:** Trigger in `state/zustand/wartend.φ` (voyager-nssdca).

### termin-Punkte — re-verdict
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27) `pithia.cbk.waw.pl` backend-tot; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine (Wiedervorlage).
- **Braucht:** `archive_search --verdict <url>`; bei Erholung `*-cdn.yml` dispatchen.

### BepiColombo bc_mpo_more — Termin 2027-04-01
- **Status:** termin | **Bindung:** termin:2027-04-01
- **Trigger:** 2027-04-01 (Science-Phase-Beginn).
- **Lage:** (gemessen 2026-09-27 via `state/zustand/wartend.φ`) MORE-Cruise nicht öffentlich, Freigabe April 2027; Ticket `YYM-342-97327`; Freigabe-Anfrage `phi/blocked_sources.φ:53`.
- **Blockade:** Freigabe (dritter).
- **Braucht:** Wiedervorlage 04/2027.

### RX100-Workflow-Zweige entfernen (Quelle zurückgezogen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-27 via `sgrep rx100`) Operator-Wort: RX100 war nur ein Gedanke; die Quelle ist aus `phi/sources.φ`/`phi/harvest.φ` entfernt (`decline operator-withdrawn`), das CDN-Asset gelöscht. Die rx100-Zweige stehen noch in `.github/workflows/harvest.yml` (`FORMAT = rx100_luminance`-Branches) und `.github/workflows/harvest-long.yml`; ohne Register-Block liefert `harvest_reg --lookup rx100_luminance` jetzt void → ein Dispatch des Formats erzeugte ein falsches Health-Issue.
- **Blockade:** keine.
- **Braucht:** die rx100-Branches aus beiden Workflows entfernen; die übrigen Formate unberührt lassen.

## Träger (Prosadokumente)

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | `mariner-occlt`-CDN-Dispatch geschlossen; offen: native ODF-Serien-Arm + vier request-only-Routen | nächster Schritt: `archive_search --verdict` beim Trigger.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | SAMPLE_CONTACT (MPI-FKF/LAB_A) sagte zu, danach kein Eingang (gemessen 2026-09-27 via mail_ledger) | wartend auf Mail-Eingang (kein Nachfassen).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | nur `Wiedervorlage 2026-12-02` bindet | nächster Schritt: 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending weiter tot | nächster Schritt: `--verdict` je Host beim Trigger.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offen: Step 5 (CDN-kanonisch); **Operator-Wort 2026-09-27: „umschreiben"** (aus Future-Queue 19) | nächster Schritt (eigen): Klassen-Zensus messen, dann den kanonischen Schritt ausführen.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Migrationsplan-Vorlage steht; das Layout-Wort liegt in Future's Operator-Queue | nächster Schritt: Migration nach Wort.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen nur §7 Roh-Korpora-Disposition; der Akt liegt in Future's Operator-Queue | nächster Schritt: Disposition nach Wort.
- `docs/concepts/tools-map.md` | CI-Werkzeug-Detail (`ci_manage status`/`jobs`) ergänzt; übrige offene Marker unverändert | nächster Schritt: `register_lookup --orphan-docs` beim nächsten Pass; Owner-Klärung offen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
