<!--
  title: Handover — Mycelium-Folge 181 (2026-09-27)
  session: Mycelium-Folge 181
  class: handover
  date: 2026-09-27
  sha256: a123e03793b1bf3cb36fc9c7e709e088f588d7cfcd2902ef5a0328ae207242f0
  status: live
-->
# Handover — Mycelium-Folge 181 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `wartend` | `operator-gebunden` |
`blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-27-mycelium-folge180.md`.

## Operator-Wort-Register

- Wort | 2026-09-27 | „nein bitte so festschreiben" — Rundenordnung (Mycelium→Sensory→Mountain→River→Future) + Spirale in `docs/concepts/kybernaut-native-methodology.md`, mechanik-only.
- Wort | 2026-09-27 | „bitte ausführen" — Ownership-Audit (Aufenthalt = Eigentum): planeto-epncore-Direktiven + CDN-Reconcile-Dispatch → Mycelium, clippy-Owner je Linie, Beschaffungs-LOCKs aus Mountain/River entfernt (kanonisch Future).
- Wort | 2026-09-27 | Secrets-Inventar-Träger prüfen: direkter Edit in die Owner-Übergabe (Messung ergab keinen Register-Akt) — erledigt (Verdict, Survey geschlossen).
- Wort | 2026-09-27 | **Myceliums eigener Job umfasst CI-Workflows, die `sources.φ`-Manifestations-Direktiven und das CDN-/Release-Management** — matrix-rotor-Instrumentierung, planeto-Direktiven und die modis-Asset-Brücke sind NICHT an Sensory/Mountain/Operator abzugeben; der „Träger" im Pass meint die Sache, nicht das Handwerk.

## Offen (aufgeschlüsselt)

### dcom5-cdn — Workflow ohne Checkout (fix im Baum)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `dcom5-cdn`-Lauf am neuen HEAD grün.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36316331564`) failure: `error: could not find Cargo.toml in /home/runner/work/omegaflow/omegaflow` + `bash: .github/workflows/scripts/gh_issue_once.sh: No such file or directory` (exit 127) — `dcom5-cdn.yml` hatte keinen `actions/checkout`-Step. Fix im Arbeitsbaum: `actions/checkout@v7` + `actions-rust-lang/setup-rust-toolchain@v1` ergänzt.
- **Blockade:** keine.
- **Braucht:** `/commit` + Push, dann `gh workflow run dcom5-cdn.yml`; `ci_manage view <id>`.

### gosat-cdn — 4096-Granule-Bound + 2024-Leerlauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `gosat-cdn`-Lauf am neuen HEAD.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36309708111`) `compile (2026)` failure: `GWT3F_L1B 2026-01-01..2026-12-31 carries 8512 granules — over the 4096 granule bound; narrow the window`; zweite Failure `compile (2024)`: `no granules for GWT3F_L1B 2024 … nothing fabricated` (Instrumentenstart Okt 2024; ehrlicher 0). Fix im Arbeitsbaum (grind-pro, gemessen): `MAX_SEARCH_RESULTS` `1<<12`→`1<<14` (willkürlicher Power-of-2-Cap, keine Speicherkonstante — `MAX_RECORDS=1<<20`, `MAX_VAL_BYTES=1<<28`; der 2025-Shard trägt 8 099 233 Records); Default `start_year` „2024"→„2025".
- **Blockade:** keine.
- **Braucht:** `/commit` + Push, dann `gh workflow run gosat-cdn.yml`; `ci_manage view <id>`; Manifest `gosat_tanso3.manifest` + sha256 prüfen.

### clippy extract.rs — CI-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` am neuen HEAD lesbar.
- **Lage:** (gemessen 2026-09-27 via `ci_manage list`) Lauf `36320549136` @`edfcedad0` pending, kein Log; der Fix liegt in `266a85144` (11 extract.rs-Stellen).
- **Blockade:** keiner mehr — nur die CI-Verifikation fehlt.
- **Braucht:** eigener `/commit` + Push → neuer `ci-check`; `ci_manage log <id>`.

### CDN-Reconcile — 3/4 dispatcht, Orphan gepruned
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `dcom5-cdn` grün + `cdn-reconcile`-Re-Lauf.
- **Lage:** (gemessen 2026-09-27 via `ci_manage list` + `sgrep`) `cdn-reconcile 36316474032` success; `icecat-cdn 36316333118` / `weberin-verdicts-cdn 36316334808` / `ned-byparams-cdn 36318854380` success; `dcom5-cdn 36316331564` failure (eigener Punkt oben). Orphan-Referenz `phi/pipeline/frame_registry.φ:508` (coastal-lidar, Mountain disponiert) entfernt (einzige `sgrep`-Referenz im Baum).
- **Blockade:** keine.
- **Braucht:** nach dem dcom5-Fix `gh workflow run cdn-reconcile.yml`; prüfen, dass die 4 aus `unmanifested_source_netlocs`/`missing_assets` fallen.

### modis-cdn — Release-Cap (1000 Assets)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `modis-asset-bridge` `36320551698` und `modis-cdn` `36320557426` abgeschlossen.
- **Lage:** (gemessen 2026-09-27 via `ci_manage list`) beide in_progress, kein Log; die Brücke braucht Download→Re-Upload (GitHub kein Server-Copy), die `delete`-Stufe ist destruktiv.
- **Blockade:** destruktive `delete`-Stufe = Operator-Hand.
- **Braucht:** `ci_manage view 36320551698` / `36320557426`; bei success `CDN_TAG`-Wechsel + `modis-cdn.yml` + `phi/sources.φ`.

### matrix-rotor — SIGTERM-Quelle runner-seitig (Trap greift nicht)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rerun (`attempt 2`) des instrumentierten Laufs `36320306259`.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36320306259` + `/tmp/opencode/ci_watchdog.log`) failure: `##[error]The runner has received a shutdown signal…` + `The operation was canceled`; die Trap-Zeile (`matrix-rotor.yml:62`) feuerte nicht — die Ganz-Job-Stornierung tötet vor Trap/Artefakt (Trap-Zeile im Step-Script vorhanden, `timeout --verbose … 18000`). Kein Cancel durch den `ci_watchdog` (keine cancel-Zeile; klassifiziert `assertion-red`), `bin/matrix_watchdog.sh` trägt keinen cancel-Aufruf.
- **Blockade:** die Stornierung kommt runner-/framework-seitig, außerhalb des Job-Logs.
- **Braucht:** `ci_manage rerun 36320306259`; wiederholt sich der Shutdown, ist die Instrumentierung das falsche Werkzeug → `descoped` mit diesem Befund (die State-Resumierbarkeit trägt den Rotor, nicht der Trap).

### Voyager 1/2 closed-loop Doppler — keine Antwort
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort von `gsfc-dl-nssdca-request@mail.nasa.gov` (PSNO-00007).
- **Lage:** (gemessen 2026-09-27 via `mail_ledger.φ`) kein Eingang; Anfrage 2026-09-16 13:47 UTC.
- **Blockade:** keine.
- **Braucht:** Trigger in `state/zustand/wartend.φ` (voyager-nssdca).

### termin-Punkte — re-verdict
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27) `pithia.cbk.waw.pl` backend-tot (Mountain-Folge 180: PADC VESPA portiert); `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
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
