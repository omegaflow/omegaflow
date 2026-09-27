<!--
  title: Handover — Mountain-Folge 181 (Stand 2026-09-27)
  session: Mountain-Folge 181
  class: handover
  date: 2026-09-27
  sha256: 5a7a31cdbbbdd4d6e269d979842ced23c08d4dca7a2ea9b872adc399f024c8ab
  status: live
-->
# Handover — Mountain-Folge 181 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge — die offenen Punkte werden parallel
abgearbeitet; nur der Akt am Gegenüber bleibt benannt. Jeder Punkt
aufgeschlüsselt: Status | Bindung / Trigger / Lage / Blockade / Braucht.

## Offen (aufgeschlüsselt)

### CDN-Reconcile — Shard-Fix gebaut, Divergenz-Rest offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `cdn-reconcile.yml`-Lauf `36311386072` (dispatcht 2026-09-27) schreibt `docs/specs/cdn_reconciliation.json` neu.
- **Lage:** (gemessen 2026-09-27 via `cargo check` + `gh api`) zweistufige
  MODIS-Shard-Hierarchie in `tools/register/src/bin/cdn_reconcile.rs`
  (`is_shard_of` + `sharded` über Manifest-Stem-Präfix) gebaut — Granule
  `modis_lst_cmg_8day_MOD11C2.A<…>.bin` / Jahres-`.manifest` werden dem
  Template `modis_lst_cmg_8day.manifest` zugeordnet; `cargo check -p
  omegaflow-register` 0/0. Releases live 281, orphan 83, unmanifested 35.
- **Blockade:** keine.
- **Braucht:** (a) den neuen Lauf lesen (`ci_manage list`/`view`,
  `docs/specs/cdn_reconciliation.json`) und prüfen, dass die MODIS-Granules aus
  `asset_name_divergence`/`missing_assets` fallen; (b) die 5 referenzierten
  fehlenden Releases entscheiden — `ssd.jpl.nasa.gov-dcom5` (dcom5-Blöcke),
  `ssd.jpl.nasa.gov-icecat`, `ssd.jpl.nasa.gov-weberin`,
  `ned.ipac.caltech.edu-byparams`, `noaa-nos-coastal-lidar-pds.s3.amazonaws.com`
  — Manifestation (CI-Lauf) oder Verdikt; (c) 20 Residue-Tags entscheiden:
  `ps1-dr2-{560…2400}` (11, in `phi/footprints.φ:18` benannt, keine
  Register-Zeile), `ssd.jpl.nasa.gov-{horizons,korpora,laic,ps1,signal-cone,
  sky-crossmatch,spk}` (7), `rave-survey.org`, `srdata.nist.gov`.

### NED ByParams — Token-Kanal (Code-Kanal gemessen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** NED-Cook-Token (`NED_BYPARAMS_TIMEOUT_TOKEN`) trifft per Mail ein.
- **Lage:** (gemessen 2026-09-27 via `sread`/`curl`) `X-NED-Timeout-Token` wird
  nur am Form-POST gesetzt (`ned_byparams_compiler.rs:122-123`); die
  Ticket-Poll-GET (`poll_ticket` → `http_get`, kein `-H`) und der Ergebnis-Fetch
  (`fetch_body`, kein Token **und** kein Cookie-Jar) tragen ihn nicht. Ob NED den
  Token als Session-Credential auf allen Requests prüft, ist aus Code und
  öffentlicher Doku nicht ableitbar (NED-ByParams-Landing-Page ohne `token`;
  Header-Test am Callback: HTTP 200 mit/ohne Header bytegleich bei ungültigem
  Ticket).
- **Blockade:** Token fehlt (Mail nicht eingetroffen); die entscheidende Messung ist der echte Job.
- **Braucht:** mit dem Token den echten ByParams-Job fahren und messen, ob
  Poll/Fetch ohne Header scheitern; falls ja, Token an `http_get`-Signatur und
  `fetch_body` (Header + Cookie-Jar) ergänzen.

### planeto-epncore — Manifestations-Riss (Block nicht im CDN-Muster)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Verdikt nach dem nächsten `cdn-reconcile.yml`-Lauf.
- **Lage:** (gemessen 2026-09-27 via `sread`/`sgrep`) der `sources.φ`-Block
  `voparis-tap-planeto.obspm.fr` (Arm `epncore`) trägt `url` = Live-TAP
  (`/tap/sync?…mars_craters.epn_core…`), `format tap`, keine `origin`/`compiler`,
  kein `sha256` — nicht im CDN-Muster (`url` = Asset / `origin` = Live /
  `compiler`). Der Lauf `planeto-epncore-cdn 36302017453` success spiegelt ihn
  via `cdn_mirror_compiler`; der gemessene Asset-`sha256`
  `9c368dbbe513d0e8c836247cb0f4677642843aee7b2be57ba1285349bf503cc6` gilt dem
  Spiegel, nicht dem Block-`url`. Riss: `phi/declined_sources.φ` declinet
  denselben PADC-Host (keine Weitergabe-Erlaubnis).
- **Blockade:** keine.
- **Braucht:** Verdikt — Block auf das CDN-Muster umbauen (`url` = Asset,
  `origin` = Live-TAP, `compiler`, `sha256`; Manifestations-Direktiven =
  Mycelium-Domäne) **oder** gemäß `declined_sources.φ` disposal; `format tap`
  konsumiert `sha256` heute nicht (port.rs prüft nur `format reference`).

### clippy `-D warnings`-Wand (ci-check am HEAD)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` am HEAD `7dae786b0` fertig (Lauf `36310976945`, pending).
- **Lage:** (gemessen 2026-09-27 via `ci_manage list`) Lauf pending; der Stehende
  Pass nennt Mountain als Träger der clippy-Wand (`aia.rs`, `ble.rs`, `eve.rs`,
  `hdf4.rs`, `channels.rs`, `extract.rs`).
- **Blockade:** Log noch nicht lesbar (Lauf pending).
- **Braucht:** `ci_manage log 36310976945` am HEAD lesen; je Datei-Owner heilen
  (kein lokales clippy).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
Sony RX100 V (P6) auf `LOCK` — K-Beschaffung erst nach Förderung | 2026-09-27 | Operator (Session)
UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | 2026-09-27 | Operator (Session)
D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | 2026-09-27 | Operator (Session)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
