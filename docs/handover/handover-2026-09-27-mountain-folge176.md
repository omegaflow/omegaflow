<!--
  title: Handover — Mountain-Folge 176 (Stand 2026-09-27)
  session: Mountain-Folge 176
  class: handover
  date: 2026-09-27
  sha256: 3981ad7f505d3b1c8627fdaa17f2efb04a4ec4b8268821751262a98be3e770d6
  status: live
-->
# Handover — Mountain-Folge 176 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge — die offenen Punkte werden parallel
abgearbeitet; nur der Akt am Gegenüber bleibt benannt. Jeder Punkt
aufgeschlüsselt: Status | Bindung / Trigger / Lage / Blockade / Braucht.

## Offen (aufgeschlüsselt)

### P2 EHT uvfits — sgra-Pass, Manifestation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `eht-uvfits-cdn` Lauf **36276702701** (queued/in_progress seit 2026-09-27); bei success die Archiv-sha256 und die echte AA–AP-`df` aus dem Vollauf lesen.
- **Lage:** (gemessen 2026-09-27 via `ci_manage`) Lauf 36276702701 pending; die `na`-Pässe tragen kein AA, die `sgra`-Pässe sind ALMA-inklusiv (`e17b06-7-hi-sgra-J1924-2914-fits.tgz`), `.github/workflows/eht-uvfits-cdn.yml:24-31` auf diese Quelle umgestellt, `--pair AA AP` unverändert.
- **Blockade:** keine.
- **Braucht:** bei success die Archiv-sha256 + echte AA–AP-`df` in `phi/sources.φ` (Block um `:7952`) nachtragen; `ci_manage view 36276702701` (nicht pollen).

### P5 CI-Verify @ HEAD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner `ci-check` am HEAD `ffb17c9c`.
- **Lage:** (gemessen 2026-09-27 via `ci_manage`) HEAD `ffb17c9c` == `origin/main`; `register-coverage` 36277799498 **failure**, `paper-check` 36277799438 **failure**, `ci-check` 36277799390 pending/cancelled. Fremd-Rot: `tools/measure/src/bin/membrane_hull_probe.rs:245` ruft `build_star_samples(&bytes)` mit 1 Arg, die Signatur (`src/archivar/spatial.rs:401`) verlangt 2 → `tools-build`/`membrane-hull-probe` rot; nicht Mountain.
- **Blockade:** fremder Baum-Rot (`membrane_hull_probe.rs`) + `register-coverage`/`paper-check` fremd.
- **Braucht:** `ci_manage view 36277799390` nach Abschluss; den Fremd-Bruch der Linie melden (membrane_hull_probe-Aufruf an die 2-Arg-Signatur angleichen).

### P6 Sony RX100 V Luminanz — K-Beschaffung auf LOCK
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Operator-Wort 2026-09-27 — K wird erst nach Förderung beschafft.
- **Lage:** (gemessen 2026-09-27 via `ci_manage`) `harvest` success; `phi/harvest.φ:233 asset fehlt` bleibt wahr.
- **Blockade:** keine (bis zur Förderung).
- **Braucht:** Förderung; danach K-Beschaffung + K-Messung gegen kalibriertes Luminanzmeter (river), dann K verdrahten (`freq`/`bin_width`, `river-folge38.md:109-114`).

### UI-Chat-Stimmen zum `epochrange`-Befund — LOCK
- **Status:** LOCK | **Bindung:** operator
- **Trigger:** Operator-Wort 2026-09-27 — derzeit nicht gebraucht.
- **Lage:** (gemessen 2026-09-26 via `chrome-devtools`) `claude.ai`/`chat.deepseek.com`/`kimi.ai`/`arena.ai` an Login-Wänden; `chat.z.ai` trägt `descoped` (`state/stimmen/2026-09-26_zai_ui_epochrange-p6.json`).
- **Blockade:** keine (derzeit nicht gebraucht).
- **Wort:** UI-Stimmen derzeit nicht gebraucht | 2026-09-27 | Operator (Session).
- **Braucht:** nichts bis zum Operator-Wort.

### www→bare Netloc — φ-Registry-Flip + CDN-Migration + Cleanup offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die bare-Releases sind manifestiert (die 10 `X-cdn`-Läufe dieser Session).
- **Lage:** (gemessen 2026-09-27 via `cdn_reconcile` + `sgrep`) der Code der Aufzeichnung emittiert jetzt bare Netloc — 17 harvest-Compiler (`const NETLOC`/`CDN_TAG`/`GEOAZUR_NETLOC` + `upload_release`), `neptune_apdb_manifestor`, 4 `measure`-Bins, 10 `X-cdn.yml`; die Herkunfts-URLs (`origin`, `https://…`) bleiben unverändert. `cdn_reconcile` meldet die Drift nun als `www_prefixed_release_tags` (12 live: atnf.csiro.au, crystallography.net, geoazur.fr, gmrt.org, hamqsl.com, isc.ac.uk, minorplanetcenter.net, ncdc.noaa.gov, ncei.noaa.gov, nohrsc.noaa.gov, ogimet.com, sciencebase.gov). Noch prefixiert: `phi/sources.φ` (33 `url`-Zeilen) + `phi/pipeline/frame_registry.φ` (33 Zeilen, generiert aus sources.φ). `phi/sources.φ` ist seit dem mycelium-Commit (gosat/arvo-`register_sort`) wieder sauber.
- **Blockade:** keine.
- **Braucht:** `phi/sources.φ` 33 `url`-Zeilen auf bare `releases/download/<host>/`; `frame_registry.φ` regenerieren (`cargo run -p omegaflow-utils --bin frame_registry`); nach bestätigter bare-Manifestation die 12 www-Releases entfernen (§4: nie letzte Kopie).

### CDN-Reconcile-Drift-Zahlen — offen, nachmessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `cdn_reconcile`-Lauf am HEAD nach dem source-of-truth-Fix.
- **Lage:** (gemessen 2026-09-27 via `cargo run -p omegaflow-register --bin cdn_reconcile`) `docs/specs/cdn_reconciliation.json` neu geschrieben: orphan 173, unmanifest 24, asset_name_divergence 2947, missing_assets 1494, dupegroups 10, www 12. Die Zahlen liegen deutlich über der Messung 2026-09-03 (156 Orphans) — Ursache offen (Compiler-`url` = CDN-github-Netloc vs. Origin-Netloc?).
- **Blockade:** keine.
- **Braucht:** `register_lookup --open` + die Orphan-Klassen in `docs/specs/cdn_reconciliation.json` gegen `phi/sources.φ` halten; erste Messung: die `repo_tag`/`dataset_host`-Klassen von den `stale_pending` trennen.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
Sony RX100 V (P6) auf `LOCK` — K-Beschaffung erst nach Förderung ("on hold") | 2026-09-27 | Operator (Session)
UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | 2026-09-27 | Operator (Session)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
