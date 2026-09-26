<!--
  title: Handover — Mountain-Folge 177 (Stand 2026-09-27)
  session: Mountain-Folge 177
  class: handover
  date: 2026-09-27
  sha256: 522f9cdcc00f8e122dd9196be51ddfd1366d0675983aa3df27b866e56719fb18
  status: live
-->
# Handover — Mountain-Folge 177 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge — die offenen Punkte werden parallel
abgearbeitet; nur der Akt am Gegenüber bleibt benannt. Jeder Punkt
aufgeschlüsselt: Status | Bindung / Trigger / Lage / Blockade / Braucht.

## Offen (aufgeschlüsselt)

### P2 EHT uvfits — reduzierter Writer gebaut, Lauf-Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `eht-uvfits-cdn`-Lauf am HEAD success (schreibt jetzt das reduzierte Bin).
- **Lage:** (gemessen 2026-09-27) der Writer ist gebaut: `write_beat_bin`/`parse_beat_bin` in `src/archivar/uvfits.rs` (MAGIC `CF 86 0B 00`, 45-B-Zeile, `beat_rows` erkennt das Bin), der Compiler schreibt das reduzierte 2-Zeilen-Bin statt der 4,26-GiB-Roh-FITS; `cargo check` 0/0 + `beat_bin_roundtrips_the_two_tones`-Test. Der sgra-Lauf 36276702701 failure ist damit an der Wurzel geheilt (4,26 GiB → 2 Zeilen ≪ 2 GiB).
- **Blockade:** keine.
- **Braucht:** `gh workflow run eht-uvfits-cdn.yml`; bei success die Archiv-sha256 (`ed95566f…`) + echte AA–AP-`df` in `phi/sources.φ` (~:7952) nachtragen.

### www→bare Netloc — 3 Releases offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die bare-Twins von `atnf.csiro.au` / `ncdc.noaa.gov` / `gmrt.org` tragen ihr Daten-Asset.
- **Lage:** (gemessen 2026-09-27 via `gh api repos/omegaflow/sources/releases`) 7 www-Releases gelöscht (crystallography.net, hamqsl.com, isc.ac.uk, minorplanetcenter.net, ncei.noaa.gov, ogimet.com, sciencebase.gov); 3 kept-pending: `www.atnf.csiro.au` (bare-Twin trägt nur `psr.json.sha256`, das Daten-Asset fehlt), `www.ncdc.noaa.gov` (Operator-Hold), `www.gmrt.org` (kein bare-Tag). `cdn_reconcile` www 12→3. Anomalie: bare `isc.ac.uk` existiert doppelt (id 367048555 3 Assets / id 367048556 0 Assets).
- **Blockade:** keine.
- **Braucht:** `gh release view <bare> --repo omegaflow/sources --json assets --jq '.assets|length'` gegen den www-Bestand; bei Daten-Asset `gh release delete <www> --repo omegaflow/sources --yes`; gmrt bare-Lauf abwarten (nie die letzte Kopie).

### CDN-Reconcile-Drift — Orphan-Wurzel messen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `cdn_reconcile`-Messung am HEAD.
- **Lage:** (gemessen 2026-09-27 via `cargo run -p omegaflow-register --bin cdn_reconcile`) nach Cleanup: orphan **170** (repo_tag 14, dataset_host 6, stale_pending 150), unmanifest 23, asset_name_divergence 2948, missing_assets 1494, dupegroups 10, www 3, duplicate_netloc_tags 1. Die `stale_pending`-Migrations-Residue (7 bare + 2 www) sind Release-Tags **ohne** passende `source_netlocs`-Zeile — `extract_netloc` und `release_tag_netloc` strippen **beide** `www.` (`naming.rs:8`/`cdn_reconcile.rs:12`), die www-Asymmetrie ist widerlegt.
- **Blockade:** keine.
- **Braucht:** erste Messung — je Residue-Tag die tragende `sources.φ`-`url`/`origin` prüfen (`sgrep "<tag>" phi/sources.φ`); dann `docs/specs/cdn_reconciliation.json` gegen `phi/sources.φ` halten.

### P5 CI-Verify @ HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner `ci-check` am HEAD `873a7aed2`.
- **Lage:** (gemessen 2026-09-27 via `ci_manage list`) `ci-check` 36280032365 **pending**, `register-coverage` 36280032339 **failure**, `paper-check` 36279231144 **failure** — fremde Klassen; der Fremd-Bruch `membrane_hull_probe` ist inzwischen geschlossen (`membrane-hull-probe` 36279727331 success, `star-dmax-probe` 36279729881 success).
- **Blockade:** fremder `register-coverage`/`paper-check`-Rot.
- **Braucht:** `ci_manage view 36280032365` nach Abschluss (nicht pollen).

### NED ByParams — Ernte gebaut + registriert, Token + CI-Sweep offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** NED-Token-Mail (Timeout-Verlängerung) bzw. CI-Sweep-Freigabe.
- **Lage:** (gemessen 2026-09-27) `tools/harvest/src/bin/ned_byparams_compiler.rs` **neu gebaut** (845 Z., `cargo check` 0/0) und in `phi/sources.φ` registriert (`url …/ned.ipac.caltech.edu-byparams/ned_byparams_redshift.json`, `compiler ned_byparams_compiler.rs`, `at sun`, `cmap .`, `field z ned_byparams_redshift_z`); `register_lookup --open` parst sources (0 open). ByParams = Drupal-Form-POST + CSRF + Rechen-CAPTCHA + Hintergrund-Ticket (`/ticket/refresh/callback`), Query deklinationsband-weise (`--dec-step`, default 1,0° → 180 Bänder), Ergebnis VOTable-TD. Token **noch nicht eingetroffen** (`state/mail/mail_ledger.φ`: letzte NED-Mail `1790410852` = unsere Annahme).
- **Blockade:** Timeout-Token für Bänder > 90 min.
- **Braucht:** den 180-Band-Sweep als CI-Job anlegen; Trigger = nächste NED-Mail (Token). Testticket `b08cfc32` bestätigt Spalten-/URL-Form am fertigen Ticket.

### Orphan-Docs — 2 echte offene Fakten
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die zwei Fakt-Artefakte `docs/concepts/recherche-galileo-kadenz-reconciliation.md` / `docs/surveys/survey-messpunkt-verteilung.md`.
- **Lage:** (gemessen 2026-09-27 via `register_lookup --orphan-docs` + `sgrep`) 22 Orphan-Docs; die Marker sind überwiegend **Prosa** (das Konzeptwort `pending`/`offen`), kein Arbeitsmarker. 1 stale Fakt direkt korrigiert: `docs/surveys/survey-2026-09-06-codestruktur.md` §3-Tabelle, die 5 `pending CI`-Zellen auf `cancelled`/`success` (gemessen 2026-09-27, Läufe 36171288869/36172029908/36172034181), sha256 neu. 5 `weberin-*`-Docs = fremde Linie (Aufenthalt dort).
- **Blockade:** keine.
- **Braucht:** die 2 echten offenen Fakten als eigene Punkte: `docs/concepts/recherche-galileo-kadenz-reconciliation.md:100` (60-s-Track-Parameter — DSMS Services Catalog v7.5 „Doppler count interval", NTRS 19930010224) und `docs/surveys/survey-messpunkt-verteilung.md:88/:100` (R_struct-Verlauf je Kernel-Form als Fixture + Multipol-Fehler-Fixture).

### P6 Sony RX100 V Luminanz — K-Beschaffung auf LOCK
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Förderung.
- **Lage:** (gemessen 2026-09-27) `phi/harvest.φ:233 asset fehlt` bleibt wahr.
- **Blockade:** keine (bis zur Förderung).
- **Braucht:** Förderung; danach K-Beschaffung + K-Messung gegen kalibriertes Luminanzmeter (river), dann K verdrahten (`freq`/`bin_width`).

### UI-Chat-Stimmen zum `epochrange`-Befund — LOCK
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Operator-Wort.
- **Lage:** (gemessen 2026-09-26 via `chrome-devtools`) `chat.z.ai` trägt `descoped`; die übrigen an Login-Wänden.
- **Blockade:** keine (derzeit nicht gebraucht).
- **Wort:** UI-Stimmen derzeit nicht gebraucht | 2026-09-27 | Operator (Session).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
Sony RX100 V (P6) auf `LOCK` — K-Beschaffung erst nach Förderung ("on hold") | 2026-09-27 | Operator (Session)
UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | 2026-09-27 | Operator (Session)
D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | 2026-09-27 | Operator (Session)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
