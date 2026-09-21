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

### 1. glm_l2.bin CDN-Manifestation — Compiler liest 0 Flashes
- **Status:** `blockiert` | **Bindung:** `eigen`
- **Lage:** Lauf `35579950220` @grünem HEAD — **failure**. Der Compiler lief über
  die ~30 Granules von `GLM-L2-LCFA/2026/001/00/` und meldete je Granule
  `0 flashes` → `exit 1` via `tools/harvest/src/bin/glm_l2_compiler.rs:434`; kein
  Upload. Echte GLM-L2-LCFA-Granules tragen hunderte–tausende Flashes. Asset
  `--sniff` 404.
- **Blockade:** Parser-Defekt (Record-Zahl/Dimension/Qualitäts-Gate).
- **Braucht:** erste Messung — `dataset_load`/Dimension in `glm_l2_compiler.rs`
  gegen ein echtes Granule (Record-Zahl + `flash_quality_flag`-Verteilung);
  `grind-max`. Danach Re-Dispatch + sha256 in `phi/sources.φ`.

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
- **Fremd (nicht angefasst):** die `*-cdn.yml`, `post.md`, `phi/pipeline/ledger.φ`,
  `phi/sources.φ`, gestaged `forschung-folge134` + `external-state.md` +
  `KERNEL_INDEX.md`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`).
