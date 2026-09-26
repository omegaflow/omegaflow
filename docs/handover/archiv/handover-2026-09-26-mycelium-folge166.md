<!--
  title: Handover — Mycelium-Folge 166 (2026-09-26)
  session: Mycelium-Folge 166
  class: handover
  date: 2026-09-26
  sha256: c19ba4a0923d22c54f969ebc87d7fca3df2132406cffe5e7d403da0bf5ed4f13
  status: live
-->
# Handover — Mycelium-Folge 166 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge. Sortierung: erst Akteur (**Linie** | **Rat** | **Operator** |
**Dritter**), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
**Trigger** / **Lage** / **Blockade** / **Braucht**. Status-Tag: `autonom` |
`operator-gebunden` | `blockiert` | `wartend` | `termin` | `LOCK`.

Diese Session konsumierte `handover-2026-09-25-mycelium-folge165.md`.

## Offen (aufgeschlüsselt)

### Linie (eigen)

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
- **Trigger:** `source-census`-Lauf.
- **Lage:** (gemessen 2026-09-25 via ci_manage) direct 403 / Proton 403 (ip-blocked);
  `phi/blocked_sources.φ:911` `blocked ip-blocked`; `source-census.yml` um `--blocked`
  erweitert.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view <id>`; Report `source_latency_census.φ`.

#### Manifestations-Hashes (TOAR · Zenodo)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Manifest-Lauf (kernel-flatten / CDN).
- **Lage:** (gemessen 2026-09-25 via grind-max) Zenodo `data.zip` 23 506 041 274 B ohne
  Stream-sha256 (nur md5 `86b37400…`); TOAR sha256 `8fd55224…` nur 5-Serien-Sample.
- **Blockade:** Manifest-Lauf.
- **Braucht:** CI-Full-Range-Manifest trägt die Hashes nach.

#### Voyager-Okkultation — Epochen-Anker bauen (Tar-Jahr × DOY)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-26 via research-max + Rat) Riss aufgelöst: NB-Rate **625 Hz S /
  1875 Hz X** (NSSDCA PSPA-00217); die frühere „1 kHz" ist die Recovery-Konfiguration
  Tab. 6-4 des VG2-Plans — andere Klasse, kein nominaler Gegenzeuge. OP2S nominal trägt
  keine NB-Rate (gemessen, die 3 „Sample Rate"-Treffer: Recovery + zwei leere Formularfelder).
  Format NB + Mediumband **IEEE-BE f32** (Messbeweis; `_att` deklariert nur den
  Maschinen-Namen ohne Encoding → kein Widerspruch). Der Rat-Verdikt (vierte Linie):
  `t0_ms` ist nicht epochen-monoton → nie als Zeit; `t0_ms/1000.0` entfernt,
  `epoch_anchor() -> None` + `sample_epoch(anchor, r) = anchor + DOY·86400` gesetzt,
  Mediumband-Zeilen bis dahin **pending** (absent+mandatory → record skipped; Amplitude/DOY
  bleiben gehalten). Gebaut: `src/archivar/voyager_occlt.rs`,
  `tools/harvest/src/bin/voyager_occlt_compiler.rs`; `cargo check` 0/0, Compiler-Bin gebaut.
- **Blockade:** der Record trägt kein Jahr (nur der Tar-Name `DD059817_12-NOV-80`); der
  absolute Epochen-Anker ist aus dem Record allein nicht bildbar.
- **Braucht:** das Tar-Jahr in Pack/Compiler tragen und `epoch_anchor()` speisen
  (Jahr aus Tar-Namen × DOY aus Header); das Formatdokument NSSD1394/1395 bleibt `pending`
  (`--playwright …/DOCUMENT/OP2S_TXT.ASC` liefert `about:blank`, `--verdict` 200).
- **Benchmark (Rate-Frage):** grind-flash reproduziert das research-max-Verdikt identisch
  (1 kHz nur in Recovery-Tab. 6-4; OP2S ohne nominale NB-Rate; NSSDCA „625 Hz S / 1875 Hz X",
  HTTP 200) — flash == max, flash günstiger; Sieger `grind-flash` für diese Messklasse.

#### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check 36192645925` am HEAD `42471cde`.
- **Lage:** (gemessen 2026-09-26 00:12 via ci_manage list) `36192645925` **in_progress**;
  der gefeuerte Alt-Trigger `36188536478` ist überholt; `ci_manage view` antwortet 403
  (API-Rate-Limit) — der Watchdog-Snapshot ist der stehende Lesepfad.
- **Blockade:** CI-Lauf + GitHub-API-Rate-Limit.
- **Braucht:** Watchdog-Snapshot `/tmp/opencode/ci_status.md`; bei Rot `ci_manage log <id>`.

#### tools-latest stale / tools-build Dispatch
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** GitHub-API-Rate-Limit-Erholung / nächster Push.
- **Lage:** (gemessen 2026-09-26 via `sread target/release/.tools_manifest --limit 1`)
  `git_sha=9747008486e9925b6be282f0cb4ee5c35dc2084f` < HEAD `42471cde…` → der gepullte
  Bin-Satz ist **stale**; `gh workflow run tools-build.yml` → HTTP 403 (API-Rate-Limit).
- **Blockade:** GitHub-API-Rate-Limit.
- **Braucht:** `gh workflow run tools-build.yml`; danach
  `sread target/release/.tools_manifest --limit 1` gegen `git rev-parse HEAD`.

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

## Träger (Prosadokumente) — aus folge165 weitergereicht

Je Zeile ein trägerloses Dokument der Linie: `Pfad | offene Marker: N | nächster Schritt: <tool/file/url>`.
Der Dateiname in dieser Übergabe ist der Träger (`register_lookup --orphan-docs`).

- `docs/concepts/mirror-research.md` | offene Marker: 3 | nächster Schritt: keine — 3× Statuswort „blocked" in der Ergebnistabelle, abgeschlossene Messung.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | offene Marker: 5 | nächster Schritt: `phi/sources.φ` gegen `abk_dbdt_1h_*`/kegel/GIC/corona prüfen; fehlt → `pending` registrieren.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offene Marker: 10 | nächster Schritt: 55 undocumented `stale_pending` per `docs/SOURCE_PORT.md` disponieren → Register-Disposition in `phi/sources.φ` oder `phi/dead_sources.φ`.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offene Marker: 8 | nächster Schritt: NOAA-NODD NRS bioacoustic — Harvest-Compiler in `tools/harvest/` bauen/registrieren.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | offene Marker: 29 | nächster Schritt: `register_lookup --open` — Registerstand (Nachzug 2026-09-17) abgleichen, dann disponieren.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | offene Marker: 12 | nächster Schritt: `state/mail/mail_ledger.φ` auf MPI-FKF/TRISP-Antwort (`smail`).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | offene Marker: 3 | nächster Schritt: `archive_search --verdict` für dods.wh.gov · osdr.nasa.gov · pskreporter.info · reversebeacon.net.
- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | offene Marker: 4 | nächster Schritt: keine — Verdikt final, „Gegenprobe offen — nicht meßpflichtig"; live referenziert von `survey-2026-09-16-sonden-flotte.md`.
- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | offene Marker: 15 | nächster Schritt: `gh workflow run mariner-occlt-cdn.yml`; Pioneer-ATDF dtype-12 `atdf.rs:503/636`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
