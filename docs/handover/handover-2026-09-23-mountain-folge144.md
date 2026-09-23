<!--
  title: Handover — Mountain-Folge 144 (2026-09-23)
  session: Mountain-Folge 144
  class: handover
  date: 2026-09-23
  sha256: 9d18430eda0610ed34619ad9daf8f40d503f6da348106553015ecc0d15ff9175
  status: live
-->
# Handover — Mountain-Folge 144 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet (Operator-Wort 2026-09-21). Jeder offene
Punkt wird **aufgeschlüsselt** geführt — **Trigger** / **Lage** (gemessen, mit
Messstempel) / **Blockade** / **Braucht** (der wörtliche, kopierbare Schritt).
`operator-gebunden`, `blockiert` und `wartend` werden benannt, nie dispatcht. Jeder
Punkt trägt seinen Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-23 ~17:0xZ)

- **HEAD** `d754ecdf8` (`main`) == `origin/main`. Arbeitsbaum trägt **fremde** Arbeit
  (river: staged Rename `handover-…-river-folge16` → `archiv/`, untracked
  `handover-…-river-folge17.md`; sensory: `src/archivar/fit.rs`, `src/archivar/ingress.rs`,
  `docs/specs/mantis-shrimp-build.md`, `post.md`-Hunk `An sensory:`) — unberührt.
- **CI** (gemessen via `ci_manage`): `te-gate 35879893019` @`4c97c8a6f` **pending**;
  `free-model-bench 35893010538` + `free-model-agent-bench 35893014541` @`d754ecdf8`
  **pending** (neu dispatcht 17:03Z). `tools-build`-Reds (14:29/14:34) waren der
  `libdbus`/bluer-Pfad — river hat den BLE-Bridge zurückgezogen; kein `bluer`/`dbus`
  mehr in einem `Cargo.toml` (gemessen via `sgrep -i -g "*.toml"`).
- **Postfach** (gemessen via `state/mail/mail_ledger.φ`) — keine Mountain-Zeile.
- **register_lookup --open** — keine mountain-eigenen `DISPOSITION`-Zeilen.
- **Post.md:** die Mountain-Zeilen abgeholt — Rename-Brücke **gebaut** (dieses Atom),
  Free-Model-Bench in eine `An future:`-Zeile (Worker-AI-Permission) umgeschrieben.

## Offen (aufgeschlüsselt)

### 1. flare-Re-Insert green-confirm — nach dem Push
- **Status:** termin | **Bindung:** eigen
- **Trigger:** der `te-gate`-Lauf `35879893019` @`4c97c8a6f` (Workflow geändert) — der
  Lauf selbst ist die Messung
- **Lage:** `te.rs` n=400 + `te-gate.yml` `flare`-assert committet (`42c2bf060`); Lauf
  seit 2026-09-23T15:12Z **pending** (gemessen 2026-09-23 via `ci_manage view 35879893019`)
- **Blockade:** keine
- **Braucht:** einmalig `ci_manage view 35879893019` → `flare`-Job grün (assert +
  `flare power probe:`-Zeilen)

### 2. Free-Model-Bench Lauf-Ausgang — T4/T7-Verifikation
- **Status:** termin | **Bindung:** eigen
- **Trigger:** Abschluss von `35893010538` (fmb) + `35893014541` (agent-bench) @`d754ecdf8`
- **Lage:** T4/T7-Scoring reparert + committet (`d754ecdf8`): Score auf dekodiertem
  `message.content` statt Transport-Envelope (T4 suchte `\"name\"` im escapten Envelope →
  0/97; T7 scannte die `reasoning_content`-Spur → 1/97), `no_output` als benannter
  Zustand, `max_tokens` T4/T7 256→2048; beide Läufe 2026-09-23T17:03Z neu dispatcht,
  **pending** (gemessen via `ci_manage view`)
- **Blockade:** keine
- **Braucht:** `ci_manage log 35893010538 --all` → T4/T7 pass counts > 0; der
  CF-`http_401`-Anteil entscheidet sich im selben Lauf (operator-gebunden, `An future:`-Zeile liegt)

### 3. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Geräteankunft (Tracking `LZ473049629CN`)
- **Lage:** PINE64 hat zwei Ox64 SBCs versandt, nicht angekommen (gemessen 2026-09-23
  via `state/mail/mail_ledger.φ` `1790046330`)
- **Blockade:** physische Ankunft
- **Braucht:** nach Ankunft Bring-up + Kopplung messen

## Benchmark

- Alle drei Punkte flash-first dispatcht (Routine-Klasse geschlossen 2026-09-16):
  #2 `grind-flash` (YAML-Text), #5 `grind-flash` (Harness-Scoring Hunk + Re-Dispatch),
  #4 `grind-pro` (Parser-Semantik + benannter privater Nachfolger + Gate-Fixture/Tests).
  Kein flash-Doppellauf, kein `max`. `grind-pro` lieferte #4 in einem Kontext
  (Slug-Normalisierung + `BOUNDARY`-Zeile + 2 Tests, `cargo check` 0/0); der gefundene
  `--dropped <alias>`-Filter-Gap wurde im selben Atom geheilt.

## Geteilter Baum — eigener Pfad-Satz

- `.github/workflows/te-gate.yml` — 1 Hunk (Issue-Titel/Body gate-neutral, `issue`-Job
  Z.158–162; `flare`-assert ausdrücklich umfasst)
- `tools/register/src/bin/register_lookup.rs` — Alias-Konstante `LINE_ALIASES` +
  `canonical_line` + `PRIVATE_HANDOVER_DIR` + `private_successor_exists_in` +
  `BOUNDARY`-Zeile in `run_dropped` + Filter-Normalisierung + 2 Tests
- `tools/measure/src/bin/free_model_bench.rs` — committet `d754ecdf8` (Score auf
  dekodiertem `content`, `no_output`, T4/T7 `max_tokens` 2048)
- `docs/handover/handover-2026-09-23-mountain-folge144.md` (neu)
- Move `handover-2026-09-23-mountain-folge143.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` — eigene Hunks: die abgeholte Rename-Brücken-Zeile entfernt,
  Free-Model-Bench-Zeile → `An future:` umgeschrieben; trägt **fremde** uncommittete
  `An sensory:`-Hunk → **nicht** von Mountain committet (Write-Boundary).
- **Fremd/unberührt:** river (staged Rename folge16→archiv, untracked river-folge17),
  sensory (`src/archivar/fit.rs`, `src/archivar/ingress.rs`, `docs/specs/mantis-shrimp-build.md`,
  `post.md`-Hunk), staged river-16-Rename.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
