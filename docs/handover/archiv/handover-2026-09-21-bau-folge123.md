<!--
  title: Handover — Bau-Folge 123 (Stand 2026-09-21)
  session: Bau-Folge 123
  class: handover
  date: 2026-09-21
  sha256: 1956e0efbfa03441ea2767addfdd5d1b4e5fe03795c8aedb701134e5689b3797
  status: live
-->
# Handover — Bau-Folge 123 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet (Operator-Wort 2026-09-21). Jeder offene Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Session-Beginn)

- **HEAD** `6d256545` == `origin/main` (Vorfahr: ja); Baum sauber bei
  Session-Beginn. Die in folge122 genannten fremden Hunks sind committet
  (ernte-/forschung-Commits `cd764c0b`, `6d256545`).
- **Postfach** — `state/mail/mail_ledger.φ` trägt nur alte August-Einträge; kein
  neuer Eingang. `post.md` trug 1 Zeile an bau (Mantis-Shrimp, gefaltet) + 1 an
  entscheid (DEMETER, von entscheid selbst abgeholt).
- **CI** — Watchdog 13:23Z: Masse `*-cdn` in Arbeit/failed; bau-relevant
  `glm_l2-cdn` `35579950220` failure (Punkt 1). Kein Poll.
- **Safety-Net** — `git_safety --snapshot`: Baum == HEAD, nichts zu sichern.

## Messung dieses Atoms (kein offener Punkt)

- **`archive_search`-Hilfe selbsttragend gemacht.** Der Drift war gemessen: die
  Netzwerk-Zeile listet **42** Modi, `QUERY_MODES` (was `--all` fährt) trägt
  **35**, die `--all`-Hilfe behauptete „16 calls"; `AGENTS.md:288` sagte
  „sixteen sources", `:300` „(13)". Fix: `net::query_mode_count()`
  (`QUERY_MODES.len()`) speist die `--all`-Hilfe — die Zahl kann nicht mehr
  driften; `AGENTS.md` auf 42/35 korrigiert. `cargo check -p omegaflow-utils
  --all-targets` 0 Fehler/0 Warnungen. Der `--searxng`-Fund einer Fremd-Session
  ist **nicht** der Hilfe geschuldet — der Modus fehlt dort seit folge122; er
  stammt aus altem Kontext, kein Build-Defekt.

## Offen (aufgeschlüsselt)

### 1. glm_l2.bin CDN-Manifestation — Fix gebaut, Lauf misst das Gate
- **Status:** `operator-gebunden` | **Bindung:** `operator`
- **Lage:** Lauf `35579950220` failure (0 flashes je Granule,
  `glm_l2_compiler.rs:434`). Gemessen am echten Granule
  `…s20260010000000_e20260010000200…nc` (Byte-Ebene): 5 Datasets, alle dims
  `[12]` — `flash_lat`/`flash_lon` **float32**,
  `flash_energy`/`flash_time_offset_of_first_event`/`flash_quality_flag`
  **int16** (LE, signed), Fill `-1`, chunked `[256]`, units
  `seconds since YYYY-MM-DD 00:00:00.000` (parst). Kein Early-Return-Log → alle
  Datasets/Attrs laden; 0–5 qf-degraded je Granule; der Rest stirbt an einem der
  Record-Gates (energy/time/lat/lon/tdb). Fix gebaut (`glm_l2_compiler.rs`):
  Klassen-bewusster Decode (`decode_num_at`/`NumGate`/`gated_value` — int wie
  float, float NaN-gated, `_Unsigned`-bewusst), per-Gate-Skip-Zähler
  (qf/energy/time/lat/lon/tdb) in der Granule-Zeile, Kurz-Lese-Diagnose, 4 neue
  Tests. `cargo check -p omegaflow-harvest --all-targets` 0 Fehler/0 Warnungen.
- **Blockade:** Commit-Wort + Push (der Lauf checkt main aus).
- **Braucht:** Lauf `35599198872` dispatcht (nach Commit+Push); Log einmalig
  lesen (`ci_manage view 35599198872`) — druckt N flashes + Gate-Zähler je
  Granule = definitive Messung; danach sha256 in `phi/sources.φ`, wenn das Asset
  steht.

### 2. PINE64 / Mantis-Shrimp (Ox64-Dokumentation) — von entscheid gefaltet
- **Status:** `eigen` | **Bindung:** `eigen`
- **Lage:** PINE64 hat den Ox64 zugesagt (Mail 2026-09-21, Versanddaten erbeten);
  Presence-Hardware ungebaut; Spec `docs/specs/mantis-shrimp-bom.md` + BOM
  liegen. `entscheid` hat die Doku-Pflicht an bau gegeben (post.md, gefaltet).
- **Blockade:** keine für die Doku; die physische Hardware hängt am Ox64-Versand.
- **Braucht:** Spec lesen, BOM/Aufbau beginnen — `grind-flash`.

### 3. `text_review` erreicht `tools-latest` — Hunk gebaut, Lauf verifiziert
- **Status:** `eigen` | **Bindung:** `eigen`
- **Lage:** `tools-build.yml` um measure-Build-Step + `tools.manifest`-Zeile +
  `gh release upload` ergänzt (Z. 21/30/51) — `cargo build --release
  -p omegaflow-measure --bin text_review`, Manifest-Schleife, Upload-Liste.
- **Blockade:** Push (der Workflow triggert auf push `tools/**`).
- **Braucht:** `tools-build`-Lauf `35599194982` dispatcht; danach
  `bin/.tools_ensure text_review` prüft den sha gegen `tools.manifest`.

## Benchmark

- **Bau-Folge 123:** P1 (Parser-Defekt) `grind-max` — hard atom (Urteil UND
  Schreiben in einem Kontext); P3 (Workflow-Hunk) `grind-flash` — Mechanik.
  Tool-Hilfe-Fix im `build`-Kontext. Flash-first; kein pro/max-Doubling, da die
  flash-Antwort vollständig war.

## Geteilter Baum — eigener Pfad-Satz

- **Diese Session:** `tools/harvest/src/bin/glm_l2_compiler.rs`,
  `.github/workflows/tools-build.yml`, `tools/utils/src/bin/archive_search.rs`,
  `tools/utils/src/bin/archive_search/net.rs`, `AGENTS.md` (Kosten-Leiter-Zahlen
  42/35), `docs/handover/post.md` (Mantis-Shrimp-Zeile gelöscht — eigener Hunk),
  neues `docs/handover/handover-2026-09-21-bau-folge123.md`, Move
  `handover-2026-09-21-bau-folge122.md` → `archiv/`.
- **Fremd (nicht angefasst):** `phi/sources.φ`, `phi/pipeline/ledger.φ`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`).
