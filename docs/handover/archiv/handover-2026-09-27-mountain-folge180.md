<!--
  title: Handover — Mountain-Folge 180 (Stand 2026-09-27)
  session: Mountain-Folge 180
  class: handover
  date: 2026-09-27
  sha256: 9ff40b134613f390aae40abd0d5a7952822138c4517d08f504632466b524831a
  status: live
-->
# Handover — Mountain-Folge 180 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge — die offenen Punkte werden parallel
abgearbeitet; nur der Akt am Gegenüber bleibt benannt. Jeder Punkt
aufgeschlüsselt: Status | Bindung / Trigger / Lage / Blockade / Braucht.

## Offen (aufgeschlüsselt)

### CDN-Reconcile — Shard-Fix gebaut, Rest-Divergenzen offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neuer `cdn_reconcile`-Lauf am HEAD.
- **Lage:** (gemessen 2026-09-27 via `ci_manage`) neuer Lauf `36302019654`
  **success** @`4a9d1fefe`; `docs/specs/cdn_reconciliation.json` am HEAD
  (sources_parsed 1534, `orphan_releases` enthält `ps1-dr2-*`/`ssd.jpl.nasa.gov-*`/
  `rave-survey.org`/`srdata.nist.gov`; `duplicate_netloc_tags`:
  `ncdc.noaa.gov`/`www.ncdc.noaa.gov`).
- **Blockade:** keine.
- **Braucht:** (a) zweistufige MODIS-Shard-Hierarchie
  (Template-`.manifest` → Jahres-`.manifest` → Granule-`.bin`) in
  `shard_base`/`is_manifest` abbilden, sonst bleiben die Granule-Bins Divergenz;
  (b) drei Quellen referenzieren fehlende Releases
  (`ssd.jpl.nasa.gov-{dcom5,icecat,weberin}`, `ned.ipac.caltech.edu-byparams`,
  `noaa-nos-coastal-lidar-pds.s3.amazonaws.com`) — Manifestation oder Verdikt;
  (c) die 20 Residue-Tags je entscheiden (Registraturpflicht).

### register_sort — 4 ttl-order violations (preexistenter ci-check-Rot)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` am HEAD rot via `register_sort`.
- **Lage:** (gemessen 2026-09-27 via `./target/release/register_sort`, +Gegenprobe
  `git show HEAD:phi/sources.φ`) `phi/sources.φ` hält **4 ttl-order violations**
  über 1535 Blöcke; am HEAD identisch 4/1534 — nicht aus diesem Atom.
- **Blockade:** keine.
- **Braucht:** `./target/release/register_sort phi/sources.φ` (die 4 benennen,
  Werkzeug itemisiert sie derzeit nicht) und heilen; die Reihenfolge berührt
  fremde Hunks in `phi/sources.φ` — nur eigene Regionen bewegen, `register_sort
  --write` vorher gegen die Fremd-Hunks prüfen.

### NED ByParams — Code-Kanal (Token-Kanal ausgelagert)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner — eigener Code-Schritt.
- **Lage:** (gemessen 2026-09-27 via `sgrep`, Pfad-only) `.secrets.local` ohne
  `NED_BYPARAMS_TIMEOUT_TOKEN`; die Token-Beschaffung liegt in Future's
  Operator-Queue, der Code-Kanal bleibt hier.
- **Blockade:** keine.
- **Braucht:** prüfen, ob der `X-NED-Timeout-Token`-Header auch an die
  Ticket-Poll-GET (`ned_byparams_compiler.rs:247-280`) und den Ergebnis-Fetch
  (`:325-346`) gehört — heute nur am Form-POST (`:116-144`).

### P2 EHT uvfits — Lauf + sha256 offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `eht-uvfits-cdn` 36283852455 success.
- **Lage:** (gemessen 2026-09-27 via `ci_manage` + `curl content-length`) Lauf
  `36283852455` **success** (00:53→01:16 UTC); Asset `eht_uvfits.bin` 98 B am CDN.
- **Blockade:** keine.
- **Braucht:** Digest bestätigen, dann `sha256 <hex>` zwischen `phi/sources.φ:7960`
  und `at earth` (`:7961`) ergänzen.

### planeto-epncore — Manifestation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Push des Atoms → `planeto-epncore-cdn.yml`.
- **Lage:** (gemessen 2026-09-27 via `ci_manage`) Lauf `36302017453` **success**
  (07:05→07:07); Quelle `phi/sources.φ:10212`, Arm `epncore-spatial`
  (`src/archivar/parse.rs:319` + 8 Tests).
- **Blockade:** keine.
- **Braucht:** Asset-`sha256` in `phi/sources.φ:10212` nachtragen.

### Archivar-Doku — `force_type`-Offset widersprüchlich (Riss, getragen von Sensory 2026-09-27)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner — Doku-Riss im eigenen Konzept.
- **Lage:** (gemessen 2026-09-27 via `sread`/`sgrep`) `docs/concepts/archivar-mathematikerin.md:74`
  verortet `force_type` im „JS `meta` packer at offset 2"; gemessen liegt es in
  `static/constants.js:126` (`field[f+6]`) = `field[id*3+1].z`
  (`src/mathematikerin/shaders.rs:182`); meta-Offset 2 ist `kernelId`
  (`static/constants.js:136`). `:74` widerspricht `:53`/`:60` desselben Dokuments.
- **Blockade:** keine.
- **Braucht:** `:74` auf den gemessenen Ort ziehen (`field`-Offset 6) oder als
  Verweis auf `:53`/`:60` formulieren.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
Sony RX100 V (P6) auf `LOCK` — K-Beschaffung erst nach Förderung | 2026-09-27 | Operator (Session)
UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | 2026-09-27 | Operator (Session)
D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | 2026-09-27 | Operator (Session)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
