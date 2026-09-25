<!--
  title: Handover — Mycelium-Folge 165 (2026-09-25)
  session: Mycelium-Folge 165
  class: handover
  date: 2026-09-25
  sha256: bd00fefb1c8478d91a26f2ad4eea48c3d374242e4e8f20718c328407abdf5c53
  status: live
-->
# Handover — Mycelium-Folge 165 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge. Sortierung: erst Akteur (**Linie** | **Rat** | **Operator** |
**Dritter**), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
**Trigger** / **Lage** / **Blockade** / **Braucht**. Status-Tag: `autonom` |
`operator-gebunden` | `blockiert` | `wartend` | `termin` | `LOCK`.

Diese Session konsumierte `handover-2026-09-25-mycelium-folge164.md`.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### Voyager-Okkultation — T0-Anker + Narrowband-Rate (Riss)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt: entscheidende Messung).
- **Lage:** (gemessen 2026-09-25 via research-max + grind-flash, Benchmark flash/pro)
  Decode gelöst (Mediumband 704-B-Kopf + 512 BE-f32-(I,Q), Narrowband 120-B-Kopf +
  1024 Samples). Der `t0_ms`-Zähler ist **nicht** epochen-monoton (V1 1980 =
  1230962381 > V2 1981 = 1229277827) → `t0_ms/1000.0` (`src/archivar/voyager_occlt.rs:230`)
  ist eine unbelegte Zeit-Ableitung; `timetagdays` (Header 8..10) = DOY UTC ist belegt
  (317 = 1980-11-12). Narrowband-Rate = **Riss**: NSSDCA PSPA-00217 (V1) **625 Hz S /
  1875 Hz X** (nach Filter+Dezimation) vs VG2 OP2S Tab. 6-4 „Sample Rate: **1 kHz**"
  (Recovery-Konfiguration). Zweiter Riss: `_att` der NB-Dateien nennt Data General
  Eclipse (NSSD1395) vs Mediumband Univac 1108 (NSSD1394). Das NB-Asset liegt auf dem
  CDN (`voyager_occlt.bin`, 198391190 B, sha256 `860f6927…`).
- **Nachzug (2026-09-25 via grind-flash):** NB-Strom ist mit **IEEE-BE f32** konsistent
  (Range `bytes=1016-5235`, sha256 `d31e532b…`: Längenprefixe `00 78`/`10 00` BE;
  1024 Samples, min −19.7077 / max 22.9837, 0 NaN, kohärente I/Q-Hüllkurve → kein
  DG/Eclipse-Exponentenlayout). Die `_att`-Formatdeklaration liegt im Tar und bleibt
  `pending`. OP2S-Dokument gemessen (sha256 `e38b0481…`): „Sample Rate: 1 kHz" steht
  **nur** in Tab. 6-4 unter „Maneuver Anomaly Recovery Plan" — die nominalen Tabellen
  tragen keine NB-Rate → **Riss bleibt** (625/1875 Hz vs. Recovery-1 kHz).
- **Blockade:** kein erreichbares Format-Dokument NSSD1394/1395; die `_att`-Deklaration
  im Tar (`PSPA-00217_DD059825_13-NOV-80.tar`) ungemessen; die OP2SF-Figuren sind Scans
  ohne Textlayer.
- **Braucht:** die NB-f32-Repräsentation am echten Strom messen (DG-Eclipse vs IEEE-BE)
  und die nominale OP2S-Konfiguration lesen — `archive_search --playwright
  https://pds-ppi.igpp.ucla.edu/data/VG2-S-RSS-1-ROCC-V1.0/DOCUMENT/OP2S_TXT.ASC`; danach
  Anker + Rate setzen oder als Riss mit beiden Zeugen führen. Die `t0_ms/1000.0`-Zeile
  bis dahin als unbelegt führen, nie als Zeit.

#### gap-Orphans — 4× TAP HTTP-500
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Server-Erholung / Re-Messung.
- **Lage:** (gemessen 2026-09-25 via grind-flash) unverändert HTTP 500 auf direct **und**
  socks5h, kein Wayback-Snapshot: `gavo.aip.de…ravedr4`, `padc-tap-rcsed…rcsed_fibermags`,
  `voparis-tap-astro-m…hyperleda.galaxies`, `skvo.science.upjs.sk…ogle.lightcurves`.
- **Blockade:** Server.
- **Braucht:** `archive_search --verdict <url>` je Eintrag zu gegebener Zeit.

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

#### api.sensor.community — CI-Puls
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `source-census 36179928830` (in_progress seit 19:28Z).
- **Lage:** (gemessen 2026-09-25 via ci_manage) direct 403 / Proton 403 (ip-blocked);
  `phi/blocked_sources.φ` `blocked ip-blocked`; `source-census.yml` um `--blocked`
  erweitert.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view 36179928830`; Report `source_latency_census.φ`.

#### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check 36188536478` am HEAD.
- **Lage:** (gemessen 2026-09-25 via ci_manage list) `ci-check 36188536478` pending seit
  20:54Z; `36181028802` nicht mehr in der Liste (letzte ~20 Läufe, CDN-Batch
  3618842…/3618855…); `clippy`/`test`-Reds in `2fc953b4a` geheilt. `ci_manage view`
  antwortet 403 (API-Rate-Limit) — der Watchdog-Snapshot ist der stehende Lesepfad.
- **Blockade:** CI-Lauf.
- **Braucht:** Watchdog-Snapshot `/tmp/opencode/ci_status.md`; bei Rot `ci_manage log <id>`.

#### Manifestations-Hashes (TOAR · Zenodo)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Manifest-Lauf (kernel-flatten / CDN).
- **Lage:** (gemessen 2026-09-25 via grind-max) Zenodo `data.zip` 23 506 041 274 B ohne
  Stream-sha256 (nur md5 `86b37400…`); TOAR sha256 `8fd55224…` nur 5-Serien-Sample.
- **Blockade:** Manifest-Lauf.
- **Braucht:** CI-Full-Range-Manifest trägt die Hashes nach.

#### sources-Repo — 5-min-Takt/I02 (Messlage)
- **Status:** wartend | **Bindung:** dritter (privates `omegaflow/sources`)
- **Trigger:** das `omegaflow/sources`-Repo ist lokal geklont/zugänglich.
- **Lage:** (gemessen 2026-09-17 via `survey-2026-09-17-verlorene-diskussionen.md:128-132`)
  lokal nicht geklont; offen, ob der 5-min-Takt (`refresh.yml`/I02) dort lebt und ob
  die Python-I02-Behauptung noch stimmt. Betrifft die drei Doku-Stellen
  `archivar-mathematikerin.md:22`, `pfeiler-der-architektur.md:155-158` und
  `CI_REFRESH_S` (`src/archivar/fetch.rs:900`).
- **Blockade:** Zugang zum privaten `omegaflow/sources`-Repo.
- **Braucht:** `git clone github.com/omegaflow/sources` (shallow), dann im Klon die
  Workflow-Datei `refresh.yml` mit `sgrep "cron\|refresh\|5"` prüfen + I02-Python;
  den 5-min-Takt entweder bauen oder die drei Doku-Stellen + `CI_REFRESH_S` auf die
  gemessene Wahrheit korrigieren.

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

#### DEMETER Order 18387
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-09-28 (Order-Ablauf).
- **Lage:** (gemessen 2026-09-25) `regards.cnes.fr/api/v1/rs-order` direct 403 / proton
  403 (F5-ASM-WAF).
- **Blockade:** CNES/REGARDS F5-ASM-WAF.
- **Braucht:** `archive_search --verdict https://regards.cnes.fr/api/v1/rs-order`; bei
  Erholung `gh workflow run demeter-cdn.yml`.

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

## Träger (Prosadokumente) — orphan-docs-Fold (2026-09-25)

Je Zeile ein trägerloses Dokument der Linie: `Pfad | offene Marker: N | nächster Schritt: <tool/file/url>`.
Der Dateiname in dieser Übergabe ist der Träger (`register_lookup --orphan-docs`).
Kein `git mv`: `mirror-research.md` ist abgeschlossene Referenz-Doktrin (Marker = 3×
Statuswort „blocked" in der Ergebnistabelle); `fremde-parser-sammlungen.md` wird von
der lebenden `survey-2026-09-16-sonden-flotte.md` referenziert (see-also + `:106`).

- `docs/concepts/mirror-research.md` | offene Marker: 3 | nächster Schritt: keine — 3× Statuswort „blocked" in der Ergebnistabelle, abgeschlossene Messung.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | offene Marker: 5 | nächster Schritt: `phi/sources.φ` gegen `abk_dbdt_1h_*`/kegel/GIC/corona prüfen; fehlt → `pending` registrieren.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offene Marker: 10 | nächster Schritt: 55 undocumented `stale_pending` per `docs/SOURCE_PORT.md`   disponieren → Register-Disposition in `phi/sources.φ` oder `phi/dead_sources.φ`.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offene Marker: 8 |   nächster Schritt: NOAA-NODD NRS bioacoustic — Harvest-Compiler in `tools/harvest/` bauen/registrieren.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | offene Marker: 29 | nächster Schritt: `register_lookup --open` — Registerstand (Nachzug 2026-09-17) abgleichen, dann disponieren.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | offene Marker: 12 | nächster Schritt: `state/mail/mail_ledger.φ` auf MPI-FKF/TRISP-Antwort (`smail`).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | offene Marker: 3 | nächster Schritt: `archive_search --verdict` für dods.wh.gov · osdr.nasa.gov · pskreporter.info · reversebeacon.net.
- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | offene Marker: 4 | nächster Schritt: keine — Verdikt final, „Gegenprobe offen — nicht meßpflichtig"; live referenziert von `survey-2026-09-16-sonden-flotte.md`.
- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | offene Marker: 15 | nächster Schritt: `gh workflow run mariner-occlt-cdn.yml`; Pioneer-ATDF dtype-12 `atdf.rs:503/636`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
