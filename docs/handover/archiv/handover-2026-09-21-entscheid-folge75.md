<!--
  title: Handover — Entscheid-Folge 75 (gemessener Katalog) (Stand 2026-09-21)
  session: Entscheid-Folge 75
  class: handover
  date: 2026-09-21
  sha256: 3b7fa149caa7e56566408eb18b7473d3e14c703146441c75eaf1376fa6001d6f
  status: live
-->
# Handover — Entscheid-Folge 75 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge — die offenen Punkte werden parallel von Agenten
abgearbeitet. Jeder offene Punkt wird **aufgeschlüsselt** geführt: **Lage**
(Zustand, gemessen) / **Blockade** (woran es hängt, oder „keine") / **Braucht**
(was es löst: Werkzeug, Datei, URL, Anfrage, Operator-Wort). Jeder Punkt trägt
seinen Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-21, Entscheid-Folge 75)

- **HEAD** `d30f5bd1` (entscheid folge74) — `origin/main` == HEAD. Arbeitsbaum
  trägt nur fremde, uncommittete Arbeit (`ci-check.yml`,
  `register_lookup.rs`, `dropped-baseline.md`, `radnet_compiler.rs`) — **nicht
  eigene**; nicht angefasst.
- **Postfach** — `state/mail/mail_ledger.φ`: kein neuer Eingang; neuester
  `external-state.md` Postfach-Zeile nachgezogen.
- **CI** — `ci_manage view` 2026-09-21: `free-model-bench` `35567266519` +
  `free-model-agent-bench` `35567268692` weiter **in_progress** (`updated_at`
  06:09Z, seither kein Update). Watchdog-Snapshot 09:07:02: `ps1-cdn`
  `35569486280`, `ci-check` `35569256029`, `te-gate` `35567711055`,
  `demeter-cdn` `35567568429` (queued), `health-check` `35556807317`
  in_progress; failed: `hyperscanning-te` `35570480672`/`35567708611`,
  `rpw-cdn` `35569266300`, `ci-check` `35566258372`. Kein Poll.
  `external-state.md` CI-Zeile auf HEAD `d30f5bd1` nachgezogen.
- **`register_lookup --open`** — 3 `[entscheid]`-Registerpunkte
  `phi/blocked_sources.φ:21/60/65` (SuperDARN / solar-system-open-data /
  Amentum) → Operator-Queue; 2 fremde OPEN
  (`handover-2026-09-20-operator-entscheidungen.md`).
- **`git_safety --snapshot`** — `refs/safety/1789975789`.

## Messung dieses Atoms (kein offener Punkt)

- **`open_points_check` Parser-Rest-Gap geschlossen** (eigene Linie, folge74-Datei):
  ein backtick-quotierter Pfad mit unmittelbar folgendem `:` (`` `path`: prose ``)
  wurde mit dem `:` als Pfad getestet → falsches `ABSENT`. Fix: `:` in die
  `trim_end_matches`-Menge (`normalize`); zwei Testfälle ergänzt. `cargo check
  -p omegaflow-register` grün, null Warnungen. **Die laufende Binärdatei ist
  weiterhin der alte Release-Stand** (folge74-Fix + dieser Fix noch nicht in
  `tools-latest`) — daher meldet `open_points_check` 5 `ABSENT` in diesem
  Handover, alle **falsch-positiv** (Pfade existieren auf dem Baum: die drei
  `state/mail/*`, `phi/blocked_sources.φ`, `docs/zustand/dropped-baseline.md`).

## Offen (aufgeschlüsselt)

### `open_points_check`-Fix im Release (Register-Duty)
- **Status:** `wartend` | **Bindung:** `eigen`
- **Lage:** Der Parser-Fix (folge74: Trailing-Backtick/Klammer; folge75: `:`)
  liegt im Quellbaum, aber `tools-latest` trägt noch die alte Binärdatei —
  `open_points_check` meldet weiter falsche `ABSENT`.
- **Blockade:** Trigger `tools-build`-Lauf → neues `tools-latest`.
- **Braucht:** nach dem Push `gh workflow run tools-build.yml`; danach
  `bin/.tools_ensure open_points_check` (sha256-Abgleich). Kein Poll.

### Free-Model-Bench (P13 + P2–P4) — die 105 Modelle
- **Status:** `wartend` | **Bindung:** `eigen`
- **Lage:** Input = `tools/measure/free_models.tsv` (**exakt 105 Zeilen**, 10
  Provider: zai 3, tokenrouter 1, kenari 15, kilo 18, openrouter 19, opencode 7,
  google 12, groq 4, nvidia 15, cloudflare-workers-ai 11). Läufe `35567266519`
  (free-model-bench, 7 Tasks × 105 Modelle) + `35567268692`
  (free-model-agent-bench) seit 06:09Z `in_progress`, `updated_at` 06:09:56
  eingefroren, **kein Job-Log** (`ci_manage log` → 404, Job 106231471964) —
  Runner-Queue statt Fehler. Kein Artefakt. Kein früherer erfolgreicher
  free-model-Lauf in den letzten 100 Läufen → **keine Median-Basis** (Watchdog
  cancelt nicht).
- **Blockade:** Trigger „Run-Abschluss" nicht gefeuert; Laufzeit bis 360 min
  erlaubt (Bench: curl `-m 60`, max 3×429-Retry ≤120 s).
- **Braucht:** Run-Abschluss abwarten, dann Artefakte **einmal** lesen
  (`ci_manage view 35567266519` / `35567268692`; `gh run download <id> -n
  free-model-bench`) und das Ranking oder `pending` als Zeile tragen. Kein Poll.

### PINE64-Dokumentationspflicht (Ox64 / Mantis-Shrimp)
- **Status:** `blockiert` | **Bindung:** `linie:bau`
- **Lage:** Ox64 von Pine64 zugesagt (Hardware beidseitig geschlossen); die
  Presence-Hardware „Mantis-Shrimp" ist ungebaut.
- **Blockade:** Hardware existiert nicht; Bau gehört zur **bau-Linie**.
- **Braucht:** bau baut minimalen Mantis-Shrimp (Spec
  `docs/specs/mantis-shrimp-bom.md` + BOM liegen) und dokumentiert danach den
  Ox64. Nachricht liegt in `docs/handover/post.md` (`An bau:`).

## Operator-Queue (Stand folge75; einfache Sprache, je Eintrag Lage/Blockade/Braucht, mit Alter)

4. **ISH Chat (GitHub-Dritt-OAuth-App, Scopes `read:user`/`user:email`)** —
   **Lage:** Sicherheitsereignis, App hat Kontozugriff. **Blockade:** offener
   Zugriff. **Braucht:** App unter
   `github.com/settings/connections/applications` widerrufen? (Alter: seit 2026-09-20)
5. **SuperDARN** (`blocked account`, `phi/blocked_sources.φ:21`) — **Lage:**
   HF-Radar-Ionosphären-Konvektion, Route über Globus + PI-Vereinbarung.
   **Blockade:** kein Konto, keine PI-Vereinbarung. **Braucht:** Konto +
   PI-Vereinbarung eingehen? (Alter: seit 2026-09-16)
6. **solar-system-open-data REST** (`blocked key`, `phi/blocked_sources.φ:60`) —
   **Lage:** HTTP 401 (Bearer-Token). **Blockade:** kein Token. **Braucht:**
   Konto/Token anlegen? (Alter: seit 2026-09-20)
7. **Amentum Developer** (`blocked account`, `phi/blocked_sources.φ:65`) —
   **Lage:** geomagnetisch/aviation-radiation/gravity (trial). **Blockade:**
   keine Registrierung. **Braucht:** `developer.amentum.io/register`? (Alter:
   seit 2026-09-20)
8. **Split-Routing-Verifikation** — **Lage:** 8 `000`-Hosts ungemessen.
   **Blockade:** braucht sudo + Netz. **Braucht:** Operator-Wort/Route für
   `./bin/proton-exit.sh ca` + direct↔tunnel-Nachmessung. (Alter: seit Ernte
   folge12–17)
9. **Cookie-Transfer** — **Lage:** `operator-gebunden`, Auslöser „Bedarf".
   **Blockade:** kein Bedarf. **Braucht:** nichts — wartend. (Alter: seit 2026-09-16)

## Benchmark

- **Delegation (Entscheid-Folge 75):** 1 × `research-max` (pro/max) für das
  Netz-Recherche über 16 Quellen, Klasse „harte Recherche" (kein Doppel-Lauf
  gegen flash, Klasse ist als hart geführt). Ergebnis: 12 Programme gemessen,

## Geteilter Baum — eigener Pfad-Satz

- `state/mail/emergent-ventures-application.md` (neu, gitignored)
- `tools/register/src/bin/open_points_check.rs` (Parser-Fix `:` + 2 Testfälle)
- `docs/zustand/external-state.md` (Postfach- + CI-Zeile)
- `docs/handover/handover-2026-09-21-entscheid-folge75.md` (neu)
- Move `handover-2026-09-21-entscheid-folge74.md` → `archiv/` (eigene Linie, atomar)
- **nicht** angefasst: fremde `phi/*`, `tools/harvest`, fremde CI-/Zustand-Zeilen,
  die fremde uncommittete Änderung `tools/register/src/bin/register_lookup.rs`,
  `.github/workflows/ci-check.yml`, `docs/zustand/dropped-baseline.md`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Nach dem Push
`tools-build.yml` dispatchten (Register-Crate geändert → neues `tools-latest`,
das den `open_points_check`-Fix trägt) und `ci-check.yml`; kein Poll — Ergebnis
einmalig lesen (`ci_manage view <id>`).
