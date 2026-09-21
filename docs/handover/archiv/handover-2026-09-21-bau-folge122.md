<!--
  title: Handover — Bau-Folge 122 (Stand 2026-09-21)
  session: Bau-Folge 122
  class: handover
  date: 2026-09-21
  sha256: 0700315ff04a4c02d97f84caf569dcf24e8bbf9471da849c1fe03900e4b2b18b
  status: live
-->
# Handover — Bau-Folge 122 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der
eigene Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden **parallel** von Agenten
abgearbeitet (Operator-Wort 2026-09-21). Jeder offene Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Session-Beginn)

- **HEAD** `0c0b30fb` == `origin/main` (Vorfahr: ja). Der Baum trägt fremde
  uncommittete Arbeit (`.github/workflows/{babamul,ned,tess}-cdn.yml`,
  `docs/handover/post.md`, `phi/pipeline/ledger.φ`, `phi/sources.φ`, gestaged
  `forschung-folge134` + `external-state.md` + `KERNEL_INDEX.md`) — nicht angefasst.
- **Postfach** — `state/mail/mail_ledger.φ` leer; kein neuer Eingang.
- **CI** — Watchdog-Snapshot 10:14–10:15Z: Masse `*-cdn`-Läufe `failed` (ernte-
  Domäne); `ps1/psr/denis/gaia` in_progress, `tevcat/rave/frbcat` queued. Für bau:
  `glm_l2-cdn` Lauf `35579950220` failure (offener Punkt 1). Kein Poll.
- **Safety-Net** `refs/safety/1789989004`.

## Messung dieses Atoms (kein offener Punkt)

- **SearXNG descoped + gelöscht** (Operator-Wort 2026-09-21, „searxng löschen").
  Der `--searxng`-Modus (`archive_search.rs` Flag/Hilfe, `net.rs`
  `searxng_lines`/`searxng_results`, `QUERY_MODES`/Dispatch/2 Tests,
  `tools-map.md`-Eintrag) ist entfernt. `cargo check -p omegaflow-utils
  --all-targets` 0 Fehler/0 Warnungen.
- **Die Prämisse „self-host fällt das IP-Problem weg" ist gemessen falsch.** Der
  direkte Anschluss `185.51.184.2` (`firewall.rst.de`, AS6730 Sunrise, Albbruck
  DE — nicht der Proton-Exit) gated die Engine-Abfragen ebenso: DDG
  `html`/`lite` → **202** (JS-Landeseite, keine Treffer im HTML), Mojeek → 200
  **Captcha**, Google → 200 **JS-Shell** (`url?q=` = 0), Brave → **429**. Die
  Blockade ist Anti-Automation/JS-Erkennung, **nicht** (nur) Datacenter-IP-
  Reputation — die Diagnose `121:80–84` war unvollständig. Ein self-hosted
  SearXNG fan-outet dieselben Endpunkte und träfe dieselben Gates; der
  Playwright-Pfad (echter Browser) löst JS-Challenges, SearXNG nicht.
  `--mwmbl` bleibt der gemessene, gate-freie Web-Suchpfad.

## Offen (aufgeschlüsselt)

### 1. glm_l2.bin CDN-Manifestation — Fix gebaut, Lauf misst das Gate
- **Status:** `blockiert` | **Bindung:** `operator`
- **Lage:** Lauf `35579950220` failure (0 flashes je Granule, `glm_l2_compiler.rs:434`).
  Gemessen am echten Granule `…s20260010000000_e20260010000200…nc` (Byte-Ebene):
  5 Datasets, alle dims `[12]` — `flash_lat`/`flash_lon` **float32**,
  `flash_energy`/`flash_time_offset_of_first_event`/`flash_quality_flag`
  **int16** (LE, signed), Fill `-1`, chunked `[256]`, units
  `seconds since YYYY-MM-DD 00:00:00.000` (parst). Kein
  Early-Return-Log → alle Datasets/Attrs laden; 0–5 qf-degraded je Granule;
  der Rest stirbt an einem der Record-Gates (energy/time/lat/lon/tdb).
  Fix gebaut (`glm_l2_compiler.rs`): Klassen-bewusster Decode
  (`decode_num_at`/`NumGate`/`gated_value` — int wie float, float
  NaN-gated, `_Unsigned`-bewusst), per-Gate-Skip-Zähler (qf/energy/time/
  lat/lon/tdb) in der Granule-Zeile, Kurz-Lese-Diagnose, 4 neue Tests.
  `cargo check -p omegaflow-harvest --all-targets` 0 Fehler/0 Warnungen.
- **Blockade:** Commit-Wort + Push (der Lauf checkt main aus).
- **Braucht:** `/commit` → push → `gh workflow run glm-l2-cdn` (der Lauf
  druckt N flashes + Gate-Zähler je Granule = definitive Messung; danach
  sha256 in `phi/sources.φ`, wenn das Asset steht).

### 2. PINE64 / Mantis-Shrimp (Ox64-Dokumentation)
- **Status:** `blockiert` | **Bindung:** `linie:entscheid`
- **Lage:** Ox64 zugesagt (Hardware beidseitig geschlossen); Presence-Hardware
  ungebaut; Spec `docs/specs/mantis-shrimp-bom.md` + BOM liegen.
  `entscheid-folge76:97` trägt die Doku-Pflicht als `blockiert | linie:bau`.
- **Blockade:** die Hardware existiert nicht; kein Förderweg.
- **Braucht:** Operator-Wort via entscheid — Prototyp bauen oder descopen; bis
  dahin kein Schritt.

### 3. `text_review` erreicht `tools-latest` nicht (Post an bau, gefaltet)
- **Status:** `eigen` | **Bindung:** `eigen`
- **Lage:** `tools/measure/src/bin/text_review.rs` committet `197bccf1`;
  `tools-build.yml` baut/publiziert nur register/utils/gate/omegaflow
  (`tools-build.yml:19–52`), keinen measure-Bin → `bin/.tools_ensure text_review`
  liest `pending — absent from the tools-latest manifest`.
- **Blockade:** `tools-build.yml` hat keinen measure-Build-Step.
- **Braucht:** Workflow-Hunk — `cargo build -p omegaflow-measure --bin text_review`,
  `tools.manifest`-Zeile, `gh release upload`; `grind-flash`. Die post.md-Zeile
  wird beim Stand des Schritts gelöscht (post.md trägt derzeit fremde Hunk).

## Benchmark

- **Bau-Folge 122:** keine Delegation — die Löschung (3 Dateien) + die
  Endpunkt-Messung im `build`-Kontext. Flash-first.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `tools/utils/src/bin/archive_search.rs` (Flag + Hilfe),
  `tools/utils/src/bin/archive_search/net.rs` (Funktionen, `QUERY_MODES`,
  Dispatch, Tests), `docs/concepts/tools-map.md` (`--searxng` entfernt), neues
  `docs/handover/handover-2026-09-21-bau-folge122.md`, Move
  `handover-2026-09-21-bau-folge121.md` → `archiv/`.
- **Diese Session (GLM-L2-Fix):** `tools/harvest/src/bin/glm_l2_compiler.rs`
  (klassen-bewusster Decode + Gate-Zähler + Diagnose + Tests), Hunk in dieser
  Übergabe (Punkt 1). `phi/sources.φ` und `.github/workflows/tools-build.yml`
  bleiben fremd (uncommitted) — nicht angefasst.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`).
