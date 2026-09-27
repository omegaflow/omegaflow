<!--
  title: Handover — Mountain-Folge 180 (Stand 2026-09-27)
  session: Mountain-Folge 180
  class: handover
  date: 2026-09-27
  sha256: c10429da51262bc6dadca7a650c71f77e70f0f1a13770a132dc091a481021409
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
- **Lage:** (gemessen 2026-09-27 via `cdn_reconcile` + `cargo check`) Wurzelfix
  gebaut in `tools/register/src/bin/cdn_reconcile.rs`: `cdn_tag_from_url` liest
  die Release-Tag-Netloc, `canonical_map` den Dateinamen (nicht den Slug),
  `shard_base` erkennt `X_<n>.<ext>`. orphan 97 → **84**, unmanifest 38 → **34**,
  missing 2419 → **715**, divergence 7067 → 7330. `docs/specs/cdn_reconciliation.json`
  neu geschrieben. 20 Residue-Tags bleiben: `ps1-dr2-{…}` 11, `ssd.jpl.nasa.gov-{…}` 7
  (`horizons/korpora/laic/ps1/signal-cone/sky-crossmatch/spk`), `rave-survey.org`,
  `srdata.nist.gov` — alle ohne Quellen-Eintrag → `pending`.
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

### NED ByParams — Token-Anbindung gebaut, Token-Mail + POST-Consent offen
- **Status:** wartend | **Bindung:** operator (POST) / eigen (Rest)
- **Trigger:** NED-Timeout-Token-Mail.
- **Lage:** (gemessen 2026-09-27 via `sread state/mail/mail_ledger.φ` + `sfetch`)
  keine Token-Mail eingetroffen; `.secrets.local` ohne `NED_BYPARAMS_TIMEOUT_TOKEN`.
  Code (`ned_byparams_compiler.rs`): Token via env → `.secrets.local`-Fallback,
  als Header `X-NED-Timeout-Token` **nur** am Form-POST (`:116-144`), nicht an der
  Ticket-Poll-GET (`:247-280`) und am Ergebnis-Fetch (`:325-346).
- **Blockade:** Token-Mail fehlt; damit ist der Kanal (Header/Form/Query) nicht
  messbar.
- **Braucht:** Token-Mail abwarten → Kanal verifizieren; prüfen, ob der Token auch
  an Poll/Result gehört; Operator-Wort fürs POST.

### P2 EHT uvfits — Lauf + sha256 offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `eht-uvfits-cdn` 36283852455 am HEAD success.
- **Lage:** (gemessen 2026-09-27 via `ci_manage list` + `gh release view`) Lauf
  36283852455 in_progress @ HEAD `84ea90763`; publiziertes Asset
  `almascience.org/eht_uvfits.bin` 98 B, sha256
  `04680693d3aa18c766d15cbe776485636488570b9359d4687b31cd36bfb20ac0`; Vorlauf
  36280394440 success. `phi/sources.φ:7957-7960` trägt keine `sha256`-Zeile.
- **Blockade:** keine (Lauf offen).
- **Braucht:** nach success `gh release view almascience.org --repo
  omegaflow/sources --json assets` → Digest bestätigen; `sha256 <hex>` zwischen
  `:7960` und `at earth` (`:7961`) ergänzen.

### CDN www-Twins — ncdc-Hold + gmrt-Lauf
- **Status:** wartend | **Bindung:** eigen (gmrt) / operator (ncdc)
- **Trigger:** `gmrt-cdn` 36286911638 success / ncdc-Operator-Wort.
- **Lage:** (gemessen 2026-09-27 via `gh release view` + `ci_manage view`) in
  diesem Atom gelöscht: `www.atnf.csiro.au` (bare-Twin asset-identisch) und die
  leere `isc.ac.uk`-Dublette (Release 367048556); `gmrt-cdn` 36286911638 dispatcht
  (bare `gmrt.org` fehlte). Offen: `www.ncdc.noaa.gov`/bare je
  `noaa_cdo_ghcnd_tmax.bin` — Größen **8392 vs 7912 B** differieren.
- **Blockade:** ncdc-Operator-Hold (Größen-Differenz).
- **Braucht:** gmrt-Lauf lesen (`ci_manage view 36286911638`, nicht pollen);
  ncdc Operator-Wort — welche Kopie gilt, dann Twin löschen.

### P5 CI-Verify @ HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner `ci-check` am HEAD.
- **Lage:** (gemessen 2026-09-27 via `ci_manage list`) `ci-check` 36283676049
  pending @ HEAD `84ea90763`; `register-coverage` 36283676048 success,
  `paper-check` 36283676047 failure (fremd).
- **Blockade:** keine (Lauf pending); der ttl-order-Punkt hält `ci-check` rot.
- **Braucht:** `ci_manage view 36283676049` nach Abschluss (nicht pollen).

### planeto-epncore — Manifestation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Push des Atoms → `planeto-epncore-cdn.yml`.
- **Lage:** (gemessen 2026-09-27) neue Quelle `phi/sources.φ:10212`
  (planet-o `mars_craters.epn_core`, Arm `epncore-spatial` in
  `src/archivar/extract.rs`); Workflow `.github/workflows/planeto-epncore-cdn.yml`
  (`cdn_mirror_compiler --ci-mode`) steht; `harvest-dispatch` kennt den
  Format-Token `tap` nicht → Manual-Dispatch ist die Route.
- **Blockade:** kein Push (Commit-Wort aus).
- **Braucht:** nach Push `gh workflow run planeto-epncore-cdn.yml`; Asset-`sha256`
  in `phi/sources.φ:10212` nachtragen.

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
