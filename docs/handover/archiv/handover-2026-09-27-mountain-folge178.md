<!--
  title: Handover — Mountain-Folge 178 (Stand 2026-09-27)
  session: Mountain-Folge 178
  class: handover
  date: 2026-09-27
  sha256: d9e36a42370e25a6ecfb855b3b98fd233633d29aae0c1bf0813a4902a272eb63
  status: live
-->
# Handover — Mountain-Folge 178 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge — die offenen Punkte werden parallel
abgearbeitet; nur der Akt am Gegenüber bleibt benannt. Jeder Punkt
aufgeschlüsselt: Status | Bindung / Trigger / Lage / Blockade / Braucht.

## Offen (aufgeschlüsselt)

### P2 EHT uvfits — Lauf noch offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `eht-uvfits-cdn` am HEAD success.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view`) 36280390959 `in_progress`, 36280394440 `pending`, tools-build 36280394283 `in_progress` — kein success, kein Asset, kein Lauf-Log. Der `url`-Block `phi/sources.φ:7952` trägt **keine** `sha256`-Zeile (gemessen `sread`); der Compiler-`--run`-Pfad loggt `df` nicht (nur `--inspect`).
- **Blockade:** keine (Lauf offen).
- **Braucht:** nach success `ci_manage log <id> [--all]` → Asset-`sha256` (alternativ `curl -sL <asset-url> | sha256sum`); `sha256 <hex>`-Zeile bei ~:7952 ergänzen; `df` via `eht_uvfits_compiler --inspect` aufs tgz separat messen.

### www→bare Netloc — 3 Releases kept
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** bare-Twin trägt das Daten-Asset.
- **Lage:** (gemessen 2026-09-27 via `gh release view`/`gh api`) `www.atnf.csiro.au`/bare je 1 Asset `psr.json.sha256` — Daten-Asset `psr.json` fehlt in beiden; `www.ncdc.noaa.gov`/bare je `noaa_cdo_ghcnd_tmax.bin` (bare trägt Daten-Asset, Operator-Hold); `www.gmrt.org` kein bare-Tag. Keine Löschung. Anomalie: bare `isc.ac.uk` doppelt (id 367048555 3 Assets / 367048556 0).
- **Blockade:** keine.
- **Braucht:** `psr.json`-Daten-Asset für atnf beschaffen; ncdc-Operator-Wort; gmrt-bare-Lauf abwarten.

### CDN-Reconcile-Drift — Wurzel gemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `cdn_reconcile` am HEAD.
- **Lage:** (gemessen 2026-09-27 via `cargo run -p omegaflow-register --bin cdn_reconcile`) orphan 171 (dataset_host 6, repo_tag 14, stale_pending 151), unmanifest 23, divergence 2948, missing 1495, dupegroups 10, www 3, duplicate_netloc_tags 1. Residue = **66** (65 bare + 1 www `www.atnf.csiro.au`), nicht 9; davon 46 in dead/declined/blocked/library registriert, 20 nirgends (`ps1-dr2-{560…2400}` 11, `ssd.jpl.nasa.gov-{…}` 7, `rave-survey.org`, `srdata.nist.gov`). Wurzel: `cdn_reconcile` baut netloc nur aus `s.url`, liest `origin` nie (`cdn_reconcile.rs:210-218`) → 85 Schein-Orphans. Neu-Orphan `product.gosat-gw.nies.go.jp` ist Schein (`phi/sources.φ:8295/:8297`).
- **Blockade:** keine.
- **Braucht:** `docs/specs/cdn_reconciliation.json` (in diesem Atom neu geschrieben) gegen `phi/sources.φ` halten; die 20 nirgends-registrierten Residue-Tags entscheiden; `cdn_reconcile` um `origin` erweitern (Wurzel-Fix).

### P5 CI-Verify @ HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner `ci-check` am HEAD `2aec28f2`.
- **Lage:** (gemessen 2026-09-27 via `ci_manage list`) `ci-check` 36280407391 `pending`, `register-coverage` 36280407422 `in_progress`; `register-coverage`/`paper-check` wiederholt failure (fremde Klasse).
- **Blockade:** fremder `register-coverage`/`paper-check`-Rot.
- **Braucht:** `ci_manage view <id>` nach Abschluss (nicht pollen).

### NED ByParams — Workflow gebaut, Token + Compiler-Anbindung offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** NED-Timeout-Token-Mail.
- **Lage:** (gemessen 2026-09-27) `.github/workflows/ned-byparams-cdn.yml` neu: `gate`-Job (Secret `NED_BYPARAMS_TIMEOUT_TOKEN` → `ready`) → `sweep`-Job (180 Bänder, cron stündlich + `workflow_dispatch` mit `dec_step`). Der Compiler `ned_byparams_compiler.rs` liest den Token noch nicht; der Token ist nicht eingetroffen (`state/mail/mail_ledger.φ`).
- **Blockade:** Token fehlt; Compiler-Token-Anbindung `pending`; per-act Consent für den NED-POST (Maschinen-Gegenüber) aus.
- **Braucht:** Compiler-Token-Anbindung bauen (Env/Formfeld/Header); Token-Mail abwarten; Operator-Wort fürs POST.

### Orphan-Fakt 2 — R_struct/Multipol-Fixture
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `docs/surveys/survey-messpunkt-verteilung.md:88/:100`.
- **Lage:** (gemessen 2026-09-27) zwei `grind-pro`-Dispatches kamen **leer** zurück, kein Baum-Edit; `docs/surveys/survey-messpunkt-verteilung.md:88/:100` unverändert. Fakt 1 (`docs/concepts/recherche-galileo-kadenz-reconciliation.md:100`, 60-s-Track-Parameter) ist in diesem Atom aufgelöst und eingetragen.
- **Blockade:** Sub-Agent lieferte kein Ergebnis (Grund ungemessen) — Retry mit engerem Zuschnitt nötig.
- **Braucht:** `sgrep kernel src/` → 7 Formen; `R_struct = F32_EPS·|Ω(d)|/|Ω′(d)|` je Form als `#[cfg(test)]`-Fixture; Multipol-Fehler-Fixture (Cluster vs. Barycenter-Kernel).

### epncore parser-def — orphaned, Träger gesetzt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `phi/blocked_sources.φ:344`.
- **Lage:** (gemessen 2026-09-27) `phi/blocked_sources.φ:344 parser-def epncore`, `:345 gap epncore-spatial`, url `http://pithia.cbk.waw.pl/tap`; `register_lookup --orphans` = ORPHAN_COMMITTED (kein Handover trug ihn). Mit diesem Eintrag steht der Träger.
- **Blockade:** keine.
- **Braucht:** Ersatzquelle für die EPN-core Region c1/c2/c3 (min/max/resol + `s_region` STC-S) suchen — `voparis-tap-maser` declined (`declined_sources.φ`); `archive_search --all "EPN-core"`.

### P6 Sony RX100 V Luminanz
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Förderung.
- **Lage:** (gemessen 2026-09-27) `phi/harvest.φ:233 asset fehlt` bleibt wahr.
- **Blockade:** keine (bis Förderung).
- **Braucht:** kein Schritt (LOCK bis Förderung); danach K-Beschaffung + Messung gegen kalibriertes Luminanzmeter.
- **Wort:** Sony RX100 V (P6) auf `LOCK` | 2026-09-27 | Operator (Session)

### UI-Chat-Stimmen zum `epochrange`-Befund
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Operator-Wort.
- **Lage:** (gemessen 2026-09-26 via `chrome-devtools`) `chat.z.ai` descoped, übrige an Login-Wänden.
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
