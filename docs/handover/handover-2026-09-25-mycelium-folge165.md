<!--
  title: Handover — Mycelium-Folge 165 (2026-09-25)
  session: Mycelium-Folge 165
  class: handover
  date: 2026-09-25
  sha256: 288eeb2fbc4d59fbc9b670658436920dcb6dbf2a819131ca62bbda09a82c840e
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
- **Blockade:** kein erreichbares Format-Dokument NSSD1394/1395; nominale OP2S-Konfiguration
  (Tab. 3-5/A-6) fehlt; die OP2SF-Figuren sind Scans ohne Textlayer.
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
- **Trigger:** `ci-check 36181028802` am HEAD.
- **Lage:** (gemessen 2026-09-25 via ci_manage) `ci-check 36181028802` pending seit
  19:39Z; die Läufe 36179718053/36179707243/36179902137/36180724745 cancelled
  (`cancel-in-progress`); `clippy`/`test`-Reds in `2fc953b4a` geheilt.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view 36181028802`; bei Rot `ci_manage log <id>`.

#### Manifestations-Hashes (TOAR · Zenodo)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Manifest-Lauf (kernel-flatten / CDN).
- **Lage:** (gemessen 2026-09-25 via grind-max) Zenodo `data.zip` 23 506 041 274 B ohne
  Stream-sha256 (nur md5 `86b37400…`); TOAR sha256 `8fd55224…` nur 5-Serien-Sample.
- **Blockade:** Manifest-Lauf.
- **Braucht:** CI-Full-Range-Manifest trägt die Hashes nach.

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

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
