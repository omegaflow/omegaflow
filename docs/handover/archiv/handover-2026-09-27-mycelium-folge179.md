<!--
  title: Handover — Mycelium-Folge 179 (2026-09-27)
  session: Mycelium-Folge 179
  class: handover
  date: 2026-09-27
  sha256: 1f96a4d42fde819a4f3652a199f2e29431ae900f370fe07a505c859a05621953
  status: live
-->
# Handover — Mycelium-Folge 179 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `wartend` | `operator-gebunden` |
`blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-27-mycelium-folge178.md`.

## Operator-Wort-Register

- Wort | 2026-09-27 | „Du kannst" (`/consent`/`mycelium_go`) — session-weiter Delegations-Consent; nicht das Commit-Wort.
- Wort | 2026-09-27 | Secrets-Inventar-Träger prüfen: direkter Edit in die Owner-Übergabe statt Operator-Queue (Aufenthalt = Eigentum) — Messung ergab keinen Register-Akt.
- Wort | 2026-09-27 | „nein bitte so festschreiben" — Rundenordnung (Mycelium→Sensory→Mountain→River→Future) + Spirale in `docs/concepts/kybernaut-native-methodology.md` festschreiben, **mechanik-only** (keine Spiritualität ins öffentliche Repo).
- Wort | 2026-09-27 | „bitte ausführen" — Ownership-Audit-Kanten ziehen (Aufenthalt = Eigentum): planeto-epncore-Direktiven + CDN-Reconcile-Dispatch → Mycelium, clippy-Owner je Linie, Beschaffungs-LOCKs aus Mountain/River entfernt (kanonisch Future).

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### Flyby-Path-2 (revised) — DE441/DE442-Kernel registrieren (getragen von River 2026-09-27)
- **Status:** termin | **Bindung:** eigen
- **Trigger:** vor der Benotung des Flyby-Path-2 (nach dem Perigäum 2026-09-28).
- **Lage:** (gemessen 2026-09-27 via Rat + `sgrep`) die revidierte Flyby-Präregistrierung `docs/paper/flyby-path-2-preregistration-revised.md` setzt als Fehlschlag-Schwelle δ = max|x_DE441(t) − x_DE442(t)| am Perigäum-Fenster; die zwei JPL-Editionen sind in `phi/sources.φ` **nicht** registriert.
- **Blockade:** keine (die Quellen-Registrierung ist Myceliums exklusives Recht).
- **Braucht:** zwei `url`-Zeilen (DE441/DE442, NAIF-Kernel) in `phi/sources.φ` + `register_sort`; dann kann ein Gate-Bin δ messen.

#### GOSAT-GW GWT3F_L1B — 4096-Granule-Gate (Jahres-Sharding gebaut)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `gosat-cdn`-Lauf grün.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view 36309708111`) Jahres-Sharding gebaut; Re-Dispatch-Lauf `36309708111` am HEAD `7dae786b0` `in_progress` (09:32:36Z); Vorgänger `36302098032` (07:06) failure überholt.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36309708111`; bei success Manifest/sha256 verifizieren (URL `phi/sources.φ:8300` auf `gosat_tanso3.manifest`).

#### auto-dispatch — Workflow-Dispatch (Fix gebaut, Lauf offen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Push, der `tools/harvest/src/bin/**` berührt → neuer `auto-dispatch`-Lauf.
- **Lage:** (gemessen 2026-09-27 via `ci_manage list --limit 60`) letzter Lauf `36302092969` (07:06) **failure**; `36283521902` (00:46) success; `.github/workflows/auto-dispatch.yml` feuert nur auf `push` der Harvest-Bins (kein `workflow_dispatch`), der Fix (`--include` vor `--`, Inputs mit `default`) ist seither nicht ausgelöst.
- **Blockade:** keine.
- **Braucht:** nächsten Harvest-Bin-Push abwarten; `ci_manage list` auf `auto-dispatch` grün.

#### modis-cdn — Release-Cap (1000 Assets), Asset-Brücke vorbereitet
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Asset-Brücke gebaut + `modis-cdn`-Re-Dispatch.
- **Lage:** (gemessen 2026-09-27 via Job-Logs + grind-flash) CDN-Release `data.lpdaac.earthdatacloud.nasa.gov` hat 1000 Assets → `HTTP 422` bei Upload; der Compiler nutzt die Netloc direkt als Release-Tag (`tools/harvest/src/bin/modis_lst_cmg_compiler.rs:15`), kein `CDN_TAG`; `modis_lst_cmg_8day`/`_monthly.manifest` fehlen. Brücke braucht Download→Re-Upload (GitHub bietet keinen Server-Copy); die `delete`-Stufe ist destruktiv, die Schleife (`while`) einer Session strukturell verwehrt.
- **Blockade:** Brücken-Akt + destruktive Freigabe = Operator-Hand/CI-Job.
- **Braucht:** Einmal-Job: (1) `gh release view data.lpdaac.earthdatacloud.nasa.gov --repo omegaflow/sources --json assets` → modis-Shards listen; (2) `gh release download … --pattern 'modis_lst_cmg_*'` + sha256; (3) Familien-Release `data.lpdaac.earthdatacloud.nasa.gov-modis_lst_cmg` anlegen; (4) Shards hochladen + sha256 prüfen; (5) bare-Slots erst nach Verifikation freigeben; dann `CDN_TAG`-Wechsel + `modis-cdn.yml` + `phi/sources.φ` — erst die Brücke, dann anwenden (verbotener Re-Harvest vermeiden).

#### secrets-inventar — 6 Lebendquellen bereits disponiert (Riss zum Vorgänger-Handover)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein Register-Akt — die Messung korrigiert die Vorgänger-These.
- **Lage:** (gemessen 2026-09-27 via `sgrep`) die 6 Quellen sind disponiert: GOSAT-GW `phi/sources.φ:8300` (`gosat_tanso3`), IGETS `phi/sources.φ:8349` (`igets`), Babamul `phi/sources.φ:778` — registriert; GFW `phi/declined_sources.φ:5198` (derived-satellite-product), Movebank `phi/declined_sources.φ:4594` (position-only) — declined; Rubin/LSST `phi/blocked_sources.φ:334` — descoped. Die folge178-These „GFW/Rubin/Movebank neu als source" widerspricht dem Register — kein neuer `source`-Eintrag nötig.
- **Blockade:** keine.
- **Braucht:** Verdict über die 6 ungenutzten Secret-Namen (`GFW_PASS`, `GOSAT_GW_MAIL/PASS`, `IGETS2_USER/PASS`, `RUBIN_PASS`, `BABAMUL_*`, `MOVEBANK_*`): behalten als Vorrat mit Träger oder entfernen; dann `survey-2026-09-26-secrets-inventar.md` §Offen schließen.

#### Voyager 1/2 closed-loop Doppler — Anfrage läuft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort von `gsfc-dl-nssdca-request@mail.nasa.gov` (PSNO-00007).
- **Lage:** (gemessen 2026-09-27 via `state/zustand/wartend.φ:7`) Anfrage 2026-09-16 13:47 UTC gesandt; kein Eingang im Mail-Ledger (169 Zeilen).
- **Blockade:** keine.
- **Braucht:** Trigger in `state/zustand/wartend.φ` (voyager-nssdca).

#### planeto-epncore — Manifestations-Direktiven (getragen von Mountain 2026-09-27)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains Verdikt (accept/decline) zum `voparis-tap-planeto.obspm.fr`-Block.
- **Lage:** (gemessen 2026-09-27 via `sread`/`sgrep`) der `sources.φ`-Block (Arm `epncore`) trägt `url` = Live-TAP, `format tap`, keine `origin`/`compiler`, kein `sha256`; `phi/declined_sources.φ` declinet denselben PADC-Host.
- **Blockade:** Mountains Verdikt steht aus (dessen Übergabe).
- **Braucht:** bei `accept` die Manifestations-Direktiven schreiben (`url` = Asset, `origin` = Live-TAP, `compiler`, `sha256`); bei `decline` keine.

#### CDN-Reconcile — Manifestations-Dispatch (getragen von Mountain 2026-09-27)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains Verdikt über die fehlenden Releases (`handover-2026-09-27-mountain-folge181.md`, Punkt CDN-Reconcile).
- **Lage:** (gemessen 2026-09-27 via Mountain-Übergabe `handover-2026-09-27-mountain-folge181.md`) `cdn_reconcile.rs` meldet fehlende Releases (`ssd.jpl.nasa.gov-{dcom5,icecat,weberin}`, `ned.ipac.caltech.edu-byparams`, `noaa-nos-coastal-lidar-pds.s3.amazonaws.com`).
- **Blockade:** Mountains Verdikt über Manifestation vs. Disposal.
- **Braucht:** je `accept` den `*-cdn.yml`-Lauf dispatchen (`gh workflow run <wf>`); die Residue-Tags bleiben Mountains Verdikt.

#### clippy `-D warnings` — extract.rs (eigener Datei-Owner)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** `ci-check`-Lauf am HEAD lesbar.
- **Lage:** (gemessen 2026-09-25 via Stehender Pass, external-state.md:23) `src/archivar/extract.rs` gehört Mycelium; der clippy-Lauf (`mountain-folge181`) nennt extract.rs in der Wand.
- **Blockade:** Log noch pending.
- **Braucht:** `ci_manage log <id>` lesen; die extract.rs-Warnungen heilen (kein lokales clippy).

#### IGETS — sha256 in den igets-Block (getragen von Sensory 2026-09-27)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-27 via GH-API `releases/tags/igetsftp.gfz.de`) `igets.bin`
  Digest `sha256:852ec68215eb7e8e385f3ce537c93e634e231b6ff7ff9c87bbacbe6eeccdc096`
  (755 430 788 B); Block `phi/sources.φ:8349-8355` trägt keine `sha256`-Zeile.
- **Blockade:** keine.
- **Braucht:** `sha256 852ec68215eb7e8e385f3ce537c93e634e231b6ff7ff9c87bbacbe6eeccdc096`
  in den igets-Block, dann `register_sort`.

#### Blatt-1-Bojen-Matrix — matrix-rotor 143 gemessen (getragen von Sensory 2026-09-27)
- **Status:** eigen (CI) | **Bindung:** eigen
- **Trigger:** sofort — kein Rerun vor der Ursachenmessung.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log`) 3× exit 143 = SIGTERM von außen
  (Runner-Shutdown), Step „Den verborgenen Rotor für einen begrenzten Slice fahren";
  Slice-Timeout 18000 s (Lauf 43 s/52 s) und Job-Timeout 350 min exkulpiert, der
  `timeout`-Prozess war beim Signal noch aktiv; Akteur `unread` (nicht im Job-Log).
  Runs `36282378750` (att.2), `36298095390`.
- **Blockade:** der auslösende Canceller liegt außerhalb des Job-Logs.
- **Braucht:** Cancel-Akteur am Run messen (`gh api …/actions/runs/<id>` + Audit-Log
  `action:cancel_workflow_run`); `matrix-rotor.yml:66` `2>&1 | tee matrix-rotor.txt`
  instrumentieren — VOR jedem Rerun.

#### ci_watchdog — Matcher greift nicht (Nebenbefund, getragen von Sensory 2026-09-27)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-27 via `sread`) `bin/ci_watchdog.sh:97,103` liest mit
  `gh run view --log-failed` (das CI-Organ ist `ci_manage`) und führt „runner has
  received a shutdown signal" in der Transient-Liste, meldete aber `cause not measured
  transient` für `matrix-rotor` (kein `cancel`/`rerun`).
- **Blockade:** keine.
- **Braucht:** `:97` auf `ci_manage log "$id"` umstellen, Muster gegen das reale Log
  verifizieren.

### Termin

#### termin-Punkte — re-verdict
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27) `pithia.cbk.waw.pl/tap` 200 (von Mountain aufgelöst, external-state.md:45); `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine (Wiedervorlage).
- **Braucht:** `archive_search --verdict <url>`; bei Erholung `*-cdn.yml` dispatchen.

#### BepiColombo bc_mpo_more — Termin 2027-04-01
- **Status:** termin | **Bindung:** termin:2027-04-01
- **Trigger:** 2027-04-01 (Science-Phase-Beginn).
- **Lage:** (gemessen 2026-09-27 via `state/zustand/wartend.φ:12`) MORE-Cruise nicht öffentlich, Freigabe April 2027; Ticket `YYM-342-97327`; Freigabe-Anfrage `phi/blocked_sources.φ:53`.
- **Blockade:** Freigabe (dritter).
- **Braucht:** Wiedervorlage 04/2027.

## Träger (Prosadokumente)

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | `mariner-occlt`-CDN-Dispatch geschlossen; offen: native ODF-Serien-Arm + vier request-only-Routen | nächster Schritt: `archive_search --verdict` beim Trigger.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | SAMPLE_CONTACT (MPI-FKF/LAB_A) sagte zu, danach kein Eingang | wartend auf Mail-Eingang (kein Nachfassen).
- `docs/surveys/survey-2026-09-26-secrets-inventar.md` | Quellen disponiert; offen: 6 ungenutzte Secret-Namen | Träger ist DIESE Linie (eigener Punkt oben).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | nur `Wiedervorlage 2026-12-02` bindet | nächster Schritt: 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending weiter tot | nächster Schritt: `--verdict` je Host beim Trigger.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offen: Step 5 (CDN-kanonisch, destruktiv → Operator-Wort) | nächster Schritt: Klassen-Zensus messen.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Migrationsplan-Vorlage steht; stoppt am Operator-Wort | nächster Schritt: Operator-Wort zum Layout `knowledge/`+`backups/`.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen nur §7 Roh-Korpora/Scratch-Disposition | nächster Schritt: Operator-Wort.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
