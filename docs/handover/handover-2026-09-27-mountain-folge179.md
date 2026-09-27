<!--
  title: Handover — Mountain-Folge 179 (Stand 2026-09-27)
  session: Mountain-Folge 179
  class: handover
  date: 2026-09-27
  sha256: 9272c353595b5e8f8c5492025edbd75d88bcf51fdf7b1740f9329e8f09faff8e
  status: live
-->
# Handover — Mountain-Folge 179 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge — die offenen Punkte werden parallel
abgearbeitet; nur der Akt am Gegenüber bleibt benannt. Jeder Punkt
aufgeschlüsselt: Status | Bindung / Trigger / Lage / Blockade / Braucht.

## Offen (aufgeschlüsselt)

### CDN-Reconcile-Drift — Wurzelfix gebaut, Shard-Divergenz offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `cdn_reconcile` am HEAD.
- **Lage:** (gemessen 2026-09-27) Wurzelfix gebaut: `SourceConfig` trägt `origin`
  (`src/archivar/types.rs`), `parse_sources` liest die `origin`-Direktive
  (`src/archivar/parse.rs`), `cdn_reconcile` extrahiert `origin_netlocs`
  (`tools/register/src/bin/cdn_reconcile.rs`). Vorher/Nachher: orphan 171 → **97**
  (−74 Schein-Orphans). `docs/specs/cdn_reconciliation.json` neu geschrieben:
  orphan 97, unmanifest 38, divergence 7067, missing 2419, dupegroups 15, www 3.
  Die zuvor maskierte Shard-Namens-Divergenz der großen Hosts
  (`ssd.jpl.nasa.gov`, `data.lpdaac.earthdatacloud.nasa.gov`,
  `noaa-*-pds.s3.amazonaws.com`) erscheint jetzt ungefedert (`shard_base`/
  `is_manifest` deckt sie nicht).
- **Blockade:** keine.
- **Braucht:** `shard_base`/`is_manifest` gegen die großen Shard-Hosts prüfen;
  die 20 nirgends-registrierten Residue-Tags entscheiden
  (`ps1-dr2-{560…2400}` 11, `ssd.jpl.nasa.gov-{…}` 7, `rave-survey.org`,
  `srdata.nist.gov`).

### NED ByParams — Token-Anbindung gebaut, Token + POST-Consent offen
- **Status:** wartend | **Bindung:** operator (POST) / eigen (Rest)
- **Trigger:** NED-Timeout-Token-Mail.
- **Lage:** (gemessen 2026-09-27) `ned_byparams_compiler.rs` liest den Token jetzt:
  env `NED_BYPARAMS_TIMEOUT_TOKEN` → `.secrets.local`-Fallback; sendet Header
  `X-NED-Timeout-Token`; absent → benannte Zeile + `exit(2)`, kein Send (kein
  `unwrap_or`, kein leerer Default). Workflow `ned-byparams-cdn` exportiert das
  Secret bereits auf Job-Ebene. `ned-byparams-cdn` 36281942979 success.
- **Blockade:** Token-Mail noch nicht eingetroffen (`state/mail/mail_ledger.φ`);
  das genaue Kanal-Feld des NED-ByParams (Header/Form/Query) ist aus dem Code
  nicht ableitbar; per-act Consent für den NED-POST aus.
- **Braucht:** Token-Mail abwarten → Kanal verifizieren; Operator-Wort fürs POST.

### epncore parser-def — Ersatzquelle gemessen, Port offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `phi/blocked_sources.φ:344`.
- **Lage:** (gemessen 2026-09-27 research) Ersatzquelle gefunden: PADC VESPA
  planeto `http://voparis-tap-planeto.obspm.fr/tap` (ivo://padc.obspm.planeto/tap)
  — anonyme ADQL, 40 epn_core-Tabellen, c1/c2/c3 min/max/resol + `s_region` STC-S
  (gmap 92 Zeilen, mars_craters 384344 Zeilen, volles epntap#table-2.0-Schema).
  Zweitkandidat FU Berlin HRSC `http://dachs.planet.fu-berlin.de/tap`
  (Mars-Footprints, Polygon STC-S). `voparis-tap-maser` bleibt declined;
  src.pas `pithia.cbk.waw.pl/tap` backend-tot (500, PG :5432 refused).
- **Blockade:** keine.
- **Braucht:** planeto epn_core als Quelle in `phi/sources.φ` portieren
  (`docs/SOURCE_PORT.md`); `phi/blocked_sources.φ:344`-Verdikt nachziehen.

### P2 EHT uvfits — Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `eht-uvfits-cdn` am HEAD success.
- **Lage:** (gemessen 2026-09-27 via `ci_manage list`) kein `eht-uvfits`-Lauf in
  der Liste; `phi/sources.φ:7957-7960` Block trägt **keine** `sha256`-Zeile,
  `origin https://almascience.org/almadata/ec/eht/2016.1.01404.V/`.
- **Blockade:** keine (Lauf offen).
- **Braucht:** nach success `ci_manage log <id> [--all]` → Asset-`sha256`;
  `sha256 <hex>` bei ~:7962 ergänzen; `df` via `eht_uvfits_compiler --inspect`.

### www→bare Netloc — 3 Releases kept
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** bare-Twin trägt das Daten-Asset.
- **Lage:** (gemessen 2026-09-27) `www.atnf.csiro.au`/bare je 1 Asset
  `psr.json.sha256` — Daten-Asset `psr.json` fehlt in beiden;
  `www.ncdc.noaa.gov`/bare je `noaa_cdo_ghcnd_tmax.bin` (bare trägt Daten-Asset,
  Operator-Hold); `www.gmrt.org` kein bare-Tag. Anomalie: bare `isc.ac.uk`
  doppelt (id 367048555 3 Assets / 367048556 0).
- **Blockade:** keine.
- **Braucht:** `psr.json`-Daten-Asset für atnf beschaffen; ncdc-Operator-Wort;
  gmrt-bare-Lauf abwarten.

### P5 CI-Verify @ HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner `ci-check` am HEAD.
- **Lage:** (gemessen 2026-09-27 via `ci_manage list`) HEAD `b8a9ae421`;
  `ci-check` 36283213410 `in_progress`, `register-coverage` 36283213413 success.
- **Blockade:** keine (fremder Rot zuletzt grün).
- **Braucht:** `ci_manage view <id>` nach Abschluss (nicht pollen).

### P6 Sony RX100 V Luminanz
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Förderung.
- **Lage:** (gemessen 2026-09-27) `phi/harvest.φ:233 asset fehlt` bleibt wahr.
- **Blockade:** keine (bis Förderung).
- **Braucht:** kein Schritt (LOCK bis Förderung); danach K-Beschaffung + Messung
  gegen kalibriertes Luminanzmeter.
- **Wort:** Sony RX100 V (P6) auf `LOCK` | 2026-09-27 | Operator (Session)

### UI-Chat-Stimmen zum `epochrange`-Befund
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Operator-Wort.
- **Lage:** (gemessen 2026-09-26 via `chrome-devtools`) `chat.z.ai` descoped,
  übrige an Login-Wänden.
- **Blockade:** keine.
- **Braucht:** kein Schritt (LOCK; derzeit nicht gebraucht).
- **Wort:** UI-Stimmen derzeit nicht gebraucht | 2026-09-27 | Operator (Session)

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
Sony RX100 V (P6) auf `LOCK` — K-Beschaffung erst nach Förderung | 2026-09-27 | Operator (Session)
UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | 2026-09-27 | Operator (Session)
D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | 2026-09-27 | Operator (Session)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
