<!--
  title: Handover — Mycelium-Folge 182 (2026-09-27)
  session: Mycelium-Folge 182
  class: handover
  date: 2026-09-27
  sha256: 971fbc2154c9d5a3329c54601642cad0675936f59ebd19c6a3fa77fc469d2e57
  status: live
-->
# Handover — Mycelium-Folge 182 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `wartend` | `blockiert` |
`termin`; Operator-Akte leben in Futures Operator-Queue, nie als Linien-Punkt.

Diese Session konsumierte `handover-2026-09-27-mycelium-folge181.md`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
HEAD `36b559852`, gemessen nach Folge 181) — zitiert, nie in dieses Register kopiert.

## Operator-Wort-Register

- Wort | 2026-09-27 | „kümmer dich drum" — der verwaiste rustfmt-Fix `tools/utils/src/bin/omega_sh.rs` fällt Mycelium zu (Aufenthalt = Eigentum); Commit trägt `/commit`.
- Wort | 2026-09-27 | „Du kannst. Führe den … Plan aus — als `line`-Agent" — session-weiter Consent (Delegation), **nicht** das Commit-Wort.
- Wort | 2026-09-27 | „die Kante bin ich" — jede Linie arbeitet bis zur Kante des Operators; Wert, Wort, Dritt-Akt und Send bleiben seine Hand | Operator (Future-Session).
- Wort | 2026-09-27 | ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben — Verbreitung im selben Atom | Operator (Future-Session).
- Wort | 2026-09-27 | RX100-Kalibrierer descoped — „über exif weg": Luminanz über den Kamera-EXIF-Weg (K=12.5) | Operator (Future-Session).
- Wort | 2026-09-27 | UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | Operator (Session, Mountain).
- Wort | 2026-09-27 | D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | Operator (Session, Mountain).

## Offen (aufgeschlüsselt)

### harvest rx100_luminance — Workflow-Absent-Klassifikation repariert (Verifikation)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `harvest.yml` mit `format=rx100_luminance` am neuen HEAD grün.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36322766207`) `rx100_compiler --ci-mode` ohne `--jpeg` → exit 2, `sony-camera-remote: --jpeg <path> names the capture — no record written without it`; der Workflow prüfte aber `rc≠0` **vor** der Absent-Erkennung und stufte die Kamera-Absenz als „returned void" ein (rote `harvest`-Läufe, Issue #69/#70 offen). Fix im Baum: `.github/workflows/harvest.yml` erkennt `no record written without it` **vor** dem Void-Gate → `absent=true`.
- **Blockade:** keine.
- **Braucht:** nach `/commit`+Push `gh workflow run harvest.yml -f format=rx100_luminance`; bei grün `gh issue close 69` + `gh issue close 70` (health-Issues obsolet).

### gosat-cdn — 4096-Granule-Bound + 2025-Default (Verifikation)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `gosat-cdn` `36322845122` / `36322848137` abgeschlossen.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view`) `36322845122` in_progress (3 Jobs grün), `36322848137` pending; Fix (`MAX_SEARCH_RESULTS` 1<<12→1<<14, `start_year` 2025) steht in `36b559852`.
- **Blockade:** keine (CI läuft).
- **Braucht:** `ci_manage view 36322845122`; Manifest `gosat_tanso3.manifest` + sha256 prüfen.

### ci-check — clippy extract.rs + format (Verifikation)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check 36322836780` beendet.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view`) pending, kein Log; Fix in `266a85144` (11 extract.rs-Stellen). Der rustfmt-Fix `omega_sh.rs` heilt zugleich den `format`-Job auf der committeten Fassung.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36322836780` (Jobs `format`/`clippy`).

### matrix-rotor — SIGTERM-Quelle runner-seitig
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `36320306259` `attempt 2` beendet.
- **Lage:** (gemessen 2026-09-27) Rerun angestoßen; `attempt 1` failure: `The runner has received a shutdown signal` + `The operation was canceled`, Trap `matrix-rotor.yml:62` feuerte nicht.
- **Blockade:** framework-/runner-seitig, außerhalb des Job-Logs.
- **Braucht:** `ci_manage view 36320306259`; wiederholt sich der Shutdown → `descoped` mit Befund.

### CDN-Reconcile — 4 Netlocs
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `cdn-reconcile 36323140642` beendet.
- **Lage:** (gemessen 2026-09-27 via `gh workflow run`) neu dispatcht, nachdem `dcom5-cdn 36322843156` success wurde; Orphan `phi/pipeline/frame_registry.φ:508` bereits gepruned.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36323140642`; prüfen, dass die 4 aus `unmanifested_source_netlocs`/`missing_assets` fallen.

### hinet-cdn — CONT-Readiness nie `Available`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `hinet-cdn 36323256126` beendet.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36309711883`) `hinet: cont status never read Available — the request stays unfetched`; Auth öffnete (`/auth/?LANG=en`, 200 Stationen selektiert), `attempt 0..7 stayed unready`. Re-Dispatch `36323256126` läuft.
- **Blockade:** quellenseitige Readiness (Hinet bereitet die CONT-Anfrage nicht bereit).
- **Braucht:** `ci_manage view 36323256126`; bleibt es rot, `wartend` auf Hinet-Readiness (Trigger: periodischer Re-Dispatch) setzen.

### modis-cdn / modis-asset-bridge — Release-Cap (Verifikation)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `36320551698` (bridge) und `36320557426` (cdn) abgeschlossen.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view` + `sgrep`) beide in_progress; bridge 1 Job, cdn 25 Jobs (`compile 2019–2026` offen). Die destruktive `delete`-Stufe ist **nicht** Teil dieser Läufe — sie lebt als eigener Operator-Akt in Future's Operator-Queue (Aufenthalt = Eigentum).
- **Blockade:** keine (CI läuft).
- **Braucht:** `ci_manage view 36320551698` / `36320557426`; bei success CDN_TAG-Wechsel + `modis-cdn.yml` + `phi/sources.φ`.

### Voyager 1/2 closed-loop Doppler — keine Antwort
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort von `gsfc-dl-nssdca-request@mail.nasa.gov` (PSNO-00007).
- **Lage:** (gemessen 2026-09-27 via `mail_ledger.φ`) kein Eingang; Anfrage 2026-09-16 13:47 UTC.
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

## Träger (Prosadokumente)

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | `mariner-occlt`-CDN-Dispatch geschlossen; offen: native ODF-Serien-Arm + vier request-only-Routen | nächster Schritt: `archive_search --verdict` beim Trigger.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | SAMPLE_CONTACT (MPI-FKF/LAB_A) sagte zu, danach kein Eingang (gemessen 2026-09-27 via mail_ledger) | wartend auf Mail-Eingang (kein Nachfassen).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | nur `Wiedervorlage 2026-12-02` bindet | nächster Schritt: 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending weiter tot | nächster Schritt: `--verdict` je Host beim Trigger.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offen: Step 5 (CDN-kanonisch, destruktiv → Operator-Wort) | nächster Schritt: Klassen-Zensus messen.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Migrationsplan-Vorlage steht; stoppt am Operator-Wort | nächster Schritt: Operator-Wort zum Layout `knowledge/`+`backups/`.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen nur §7 Roh-Korpora/Scratch-Disposition | nächster Schritt: Operator-Wort.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
