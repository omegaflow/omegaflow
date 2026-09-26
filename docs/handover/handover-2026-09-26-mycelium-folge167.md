<!--
  title: Handover — Mycelium-Folge 167 (2026-09-26)
  session: Mycelium-Folge 167
  class: handover
  date: 2026-09-26
  sha256: 9e7d85d44676a5c9c4264d2e41a84dfd1ed1f84a45694fe8d8c71d8ff57db981
  status: live
-->
# Handover — Mycelium-Folge 167 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge. Sortierung: erst Akteur (**Linie** | **Rat** | **Operator** |
**Dritter**), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
**Trigger** / **Lage** / **Blockade** / **Braucht**. Status-Tag: `autonom` |
`operator-gebunden` | `blockiert` | `wartend` | `termin` | `LOCK`.

Diese Session konsumierte `handover-2026-09-26-mycelium-folge166.md`.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### sources-Repo — 5-min-Takt/I02 (Messlage)
- **Status:** wartend | **Bindung:** dritter (privates `omegaflow/sources`)
- **Trigger:** das `omegaflow/sources`-Repo ist lokal geklont/zugänglich.
- **Lage:** (gemessen 2026-09-17 via `survey-2026-09-17-verlorene-diskussionen.md:128-132`)
  lokal nicht geklont; offen, ob der 5-min-Takt (`refresh.yml`/I02) dort lebt und ob
  die Python-I02-Behauptung noch stimmt. Betrifft `archivar-mathematikerin.md:22`,
  `pfeiler-der-architektur.md:155-158`, `CI_REFRESH_S` (`src/archivar/fetch.rs:900`).
- **Blockade:** Zugang zum privaten `omegaflow/sources`-Repo.
- **Braucht:** `git clone github.com/omegaflow/sources` (shallow), dann im Klon
  `refresh.yml` + I02 prüfen; Takt bauen oder die drei Doku-Stellen + `CI_REFRESH_S`
  auf die gemessene Wahrheit korrigieren.

#### gap-Orphans — 4× TAP HTTP-500
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Server-Erholung / Re-Messung.
- **Lage:** (gemessen 2026-09-25 via grind-flash) unverändert HTTP 500 auf direct **und**
  socks5h, kein Wayback-Snapshot. Register: `phi/blocked_sources.φ:146,150,154,158`
  (`pending`).
- **Blockade:** Server.
- **Braucht:** `archive_search --verdict` je Eintrag zu gegebener Zeit:
  `http://gavo.aip.de/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=json&QUERY=SELECT+TOP+5000+radeg,dedeg,hrv,teff,plx+FROM+ravedr4.rave_dr4+WHERE+plx+IS+NOT+NULL`
  · `http://padc-tap-rcsed.obspm.fr/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=json&QUERY=SELECT+TOP+5000+ra,dec,z,umag,gmag,rmag+FROM+specphot.rcsed_fibermags+WHERE+z+IS+NOT+NULL`
  · `http://voparis-tap-astro-m.obspm.fr/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=json&QUERY=SELECT+TOP+5000+al2000,de2000,v,bmag+FROM+hyperleda.galaxies+WHERE+v+IS+NOT+NULL`
  · `https://skvo.science.upjs.sk/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=json&QUERY=SELECT+TOP+5000+ra,dec,mag+FROM+ogle.lightcurves+WHERE+mag+IS+NOT+NULL`

#### gap-Orphans — NSSDC-Anfragen
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Anfrage-Antwort.
- **Lage:** (gemessen 2026-09-25 via grind-flash) Katalogseiten HTTP 200 (PSNO-00007,
  PSCM-00009, PSPG-00011, PSPA-00605); Anfragen vom 2026-09-16 ohne Antwort; PSPG-00011
  hat einen Wayback-Snapshot (2021-04-05).
- **Blockade:** Antwort des NSSDCA.
- **Braucht:** die vier Anfragen weiter beobachten; bei Antwort Route öffnen.

#### dataverse K88GFI — Größen-Verdikt
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Dataverse-Metadaten / Manifest-Lauf.
- **Lage:** (gemessen 2026-09-25 via grind-flash) `dataset.xhtml` HTTP 202, 0 B (JS-Gate);
  `--playwright` rendert die Datensatzseite; Größe 280 GB bleibt pending.
- **Blockade:** JS-Gate.
- **Braucht:** Größe aus dem Dataverse-API/Metadaten messen.

#### hamqsl · nohrsc · ogimet — sources.φ-Block offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Harvest-Schritt.
- **Lage:** (gemessen 2026-09-26 via grind-flash) drei physische Messwerte ohne
  Register-Heim, als `pending` in `phi/blocked_sources.φ:455-465` geschrieben:
  `hamqsl.com` (HF-Propagation, 200/50452 B), `nohrsc.noaa.gov` (Schnee/SWE, 200/18000 B),
  `ogimet.com` (SYNOP, 200/15196 B).
- **Blockade:** kein `sources.φ`-Block / Compiler.
- **Braucht:** je Quelle einen `sources.φ`-Block + Compiler bauen (Muster
  `tools/harvest/src/bin/*_compiler.rs`); Feld/Verdikt nach SOURCE_PORT §8.
  Träger-URLs: `https://hamqsl.com/` · `https://nohrsc.noaa.gov/` · `https://ogimet.com/`.

#### NOAA ERI imagery — Compiler pending
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Harvest-Schritt.
- **Lage:** (gemessen 2026-09-26 via grind-flash) in
  `phi/pipeline/catalog/noaa_nodd_disposition.φ:50` registriert; Compiler
  `noaa_eri_compiler` fehlt.
- **Blockade:** Compiler ungebaut.
- **Braucht:** `noaa_eri_compiler` in `tools/harvest/` bauen.

#### ceic.ac.cn — registriert, aktuell ohne Antwort
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Messung.
- **Lage:** (gemessen 2026-09-26 via grind-flash) in `phi/sources.φ` registriert, aber
  `archive_search --verdict` liefert no response.
- **Blockade:** Host antwortet nicht.
- **Braucht:** `archive_search --verdict https://www.ceic.ac.cn` erneut; bei Fortdauer
  Verdikt in `phi/dead_sources.φ` nachziehen.

#### kegel · corona — Register-Lücke
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Quellen-Bestimmung.
- **Lage:** (gemessen 2026-09-26 via grind-flash, `survey-2026-09-03-daten-holdings-inventur.md`)
  0 Treffer für `kegel`/`corona` in `phi/sources.φ`; `abk_dbdt_1h` (`:1567`) und `GIC`
  (`:8642`,`:8662`) sind registriert.
- **Blockade:** keine Messung der Quelle (was ist `kegel`/`corona`?).
- **Braucht:** erste Messung — die zwei Tokens im Survey-Kontext lesen, Quelle bestimmen,
  dann `sources.φ`-Block oder `declined_sources.φ`-Verdikt.

#### api.sensor.community — CI-Puls
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `source-census`-Lauf.
- **Lage:** (gemessen 2026-09-25 via ci_manage) direct 403 / Proton 403 (ip-blocked);
  `phi/blocked_sources.φ:451` `blocked ip-blocked`
  (`https://api.sensor.community/v1/data/measurements`); `source-census.yml` um
  `--blocked` erweitert.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view <id>`; Report `source_latency_census.φ`.

#### Manifestations-Hashes (TOAR · Zenodo)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Manifest-Lauf (kernel-flatten / CDN).
- **Lage:** (gemessen 2026-09-25 via grind-max) Zenodo `data.zip` 23 506 041 274 B ohne
  Stream-sha256 (nur md5 `86b37400…`); TOAR sha256 `8fd55224…` nur 5-Serien-Sample.
- **Blockade:** Manifest-Lauf.
- **Braucht:** CI-Full-Range-Manifest trägt die Hashes nach.

#### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check 36192645925` am HEAD `42471cde`.
- **Lage:** (gemessen 2026-09-26 00:12 via ci_manage list) `36192645925` **in_progress**;
  der gefeuerte Alt-Trigger `36188536478` ist überholt; `ci_manage list` antwortet 403
  (API-Rate-Limit) — HEAD steht jetzt auf `497aa4c8`.
- **Blockade:** CI-Lauf + GitHub-API-Rate-Limit.
- **Braucht:** Watchdog-Snapshot `/tmp/opencode/ci_status.md`; bei Rot `ci_manage log <id>`.

#### tools-latest stale / tools-build Dispatch
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** GitHub-API-Rate-Limit-Erholung / nächster Push.
- **Lage:** (gemessen 2026-09-26 via `sread target/release/.tools_manifest --limit 1`)
  `git_sha=9747008486e9925b6be282f0cb4ee5c35dc2084f` < HEAD `497aa4c8` → der gepullte
  Bin-Satz ist **stale**; `gh workflow run tools-build.yml` → HTTP 403 (API-Rate-Limit).
- **Blockade:** GitHub-API-Rate-Limit.
- **Braucht:** `gh workflow run tools-build.yml`; danach
  `sread target/release/.tools_manifest --limit 1` gegen `git rev-parse HEAD`.

### Rat

#### Voyager — timetagdays 1-Indizierung (Formel-Riss)
- **Status:** wartend | **Bindung:** Rat
- **Trigger:** Rat-Verdikt.
- **Lage:** (gemessen 2026-09-26 via grind-pro, `src/archivar/voyager_occlt.rs:246-261`)
  die Rat-Formel `sample_epoch = anchor + DOY·86400` ist exakt wie verfügt gebaut; die
  Beobachtung: `timetagdays` ist **1-indiziert** (317 = 12-NOV-80, Schaltjahr), die Formel
  liefert damit den Start des **Folgetags**. Der Beobachter änderte die Formel nicht —
  ein `−1` wäre eine Formeländerung, die der Rat nicht verfügt hat.
- **Blockade:** offene Rat-Entscheidung.
- **Braucht:** Rat-Verdikt `timetagdays − 1` oder Bestätigung der 1-Index-Formel.

### Operator

#### Fremdmodell-Benchmark — per-Akt-Consent
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (per Akt) + definierte Benchmark-Aufgabe.
- **Lage:** (gemessen 2026-09-25) Chrome-Target verbunden; Rekord
  `docs/surveys/survey-2026-09-24-fremdmodell-bedienung.md`; jeder Prompt ist ein
  Schreib-Akt bei Dritten (z.ai/claude.ai) → per-Akt-Consent.
- **Blockade:** fehlender per-Akt-Consent + kein definierter Benchmark-Auftrag.
- **Braucht:** Operator-Wort auf den präsentierten Akt.

### Dritter

#### DEMETER Order 18387
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-09-28 (Order-Ablauf).
- **Lage:** (gemessen 2026-09-25) `regards.cnes.fr/api/v1/rs-order` direct 403 / proton
  403 (F5-ASM-WAF).
- **Blockade:** CNES/REGARDS F5-ASM-WAF.
- **Braucht:** `archive_search --verdict https://regards.cnes.fr/api/v1/rs-order`; bei
  Erholung `gh workflow run demeter-cdn.yml`.

#### src.pas TAP
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02 (Re-Messung).
- **Lage:** (gemessen 2026-09-25) `/tap` 200; `/tap/tables` 500 (PostgreSQL refused).
- **Blockade:** Pithia-Backend.
- **Braucht:** `archive_search --verdict http://pithia.cbk.waw.pl/tap/tables`.

#### BepiColombo PSA
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02.
- **Lage:** (gemessen 2026-09-25) TAP 200; Produkt 403; `release_date 2099-01-01`.
- **Blockade:** ESA-Freigabe.
- **Braucht:** `archive_search --verdict https://psa.esa.int/psa-tap/tap/`.

#### SuperDARN MAP
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02.
- **Lage:** (gemessen 2026-09-25) `data-download` 200; Globus-Task `af68c4f1` ohne
  anonymen Statuskanal.
- **Blockade:** kein anonymer Statuskanal.
- **Braucht:** `archive_search --verdict https://superdarn.ca/data-download`.

#### ESA LPF Legacy (D3)
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02.
- **Lage:** (gemessen 2026-09-25) `/lpfsa-sl/data-action` direct 500 / proton 500.
- **Blockade:** ESA-Backend.
- **Braucht:** `archive_search --verdict https://lpf.esac.esa.int/lpfsa-sl/data-action`.

#### Lasair-LSST
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02.
- **Lage:** (gemessen 2026-09-25) `api.lasair.lsst.ac.uk/api/` keine Antwort / proton
  404; ZTF-Twin 401.
- **Blockade:** Broker-Backend.
- **Braucht:** `archive_search --verdict https://api.lasair.lsst.ac.uk/api/`.

#### SSDC Limadou (CSES-L2)
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-10-02 (Re-Messung; Sotgiu: neue CSES-02-Prozedur abwarten).
- **Lage:** (gemessen 2026-09-25) Portal direct 200 / proton 200; `query.php` CAS-gated;
  Mail Alessandro Sotgiu (2026-09-16, `state/mail/mail_ledger.φ`): Limadou-Website wird
  für CSES-02 umgebaut, Zugriffsprozedur ändert sich — „wait a few weeks".
- **Blockade:** PI-seitige Prozedur.
- **Braucht:** `archive_search --verdict https://limadou.ssdc.asi.it/`.

## Träger (Prosadokumente) — aus folge166 weitergereicht

Je Zeile ein trägerloses Dokument der Linie: `Pfad | offene Marker: N | nächster Schritt: <tool/file/url>`.
Der Dateiname in dieser Übergabe ist der Träger (`register_lookup --orphan-docs`).

- `docs/concepts/mirror-research.md` | offene Marker: 3 | nächster Schritt: keine — 3× Statuswort „blocked" in der Ergebnistabelle, abgeschlossene Messung.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | offene Marker: 5 | nächster Schritt: `kegel`/`corona` bestimmen (Punkt oben); `abk_dbdt_1h`/`GIC` gemessen registriert.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offene Marker: 10 | nächster Schritt: abgeschlossen — 55 disponiert (48 in Registern verifiziert, 7 neu: 3 `pending` `phi/blocked_sources.φ:455-465`, 4 live/gedeckt benannt).
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offene Marker: 8 | nächster Schritt: abgeschlossen — NOAA-NODD-Compiler `tools/harvest/src/bin/noaa_nodd_bucket_harvester.rs` gebaut, CDN `noaa-nrs-psd-cdn.yml`.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | offene Marker: 29 | nächster Schritt: abgeschlossen — Registerstand abgeglichen, Disposition `phi/pipeline/ledger.φ:26-28`; NOAA ERI Compiler pending (Punkt oben).
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | offene Marker: 12 | nächster Schritt: `state/mail/mail_ledger.φ` auf MPI-FKF/TRISP-Antwort (`smail`); Ledger absent → pending.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | offene Marker: 3 | nächster Schritt: abgeschlossen — 4 Hosts 2026-09-26 gemessen, Disposition in `phi/dead_sources.φ:356,716,732,948`.
- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | offene Marker: 4 | nächster Schritt: keine — Verdikt final, „Gegenprobe offen — nicht meßpflichtig".
- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | offene Marker: 15 | nächster Schritt: `gh workflow run mariner-occlt-cdn.yml`; Pioneer-ATDF dtype-12 `atdf.rs:503/636`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
