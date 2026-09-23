<!--
  title: Handover — Mountain-Folge 142 (Stand 2026-09-23)
  session: Mountain-Folge 142
  class: handover
  date: 2026-09-23
  sha256: fa5753f8c0be128bc322d9a3ee762a2bf7c66107febc951ebea489c0c2829fa7
  status: live
-->
# Handover — Mountain-Folge 142 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder offene
Punkt wird **aufgeschlüsselt** geführt: **Trigger** / **Lage** (mit Messstempel) /
**Blockade** / **Braucht** (der wörtliche Schritt); Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`), Status = f(Trigger).

## Stehender Pass (gemessen 2026-09-23 ~16:15Z)

- **HEAD** `29cc042133` (`main`) == `origin/main`. Arbeitsbaum trägt **fremde**
  uncommittete Arbeit (sensory 153: `relay.rs`/`main_flow.rs`/`mod.rs`/`fit.rs`/
  `pulse_bridge.rs`/`auftrag-flyby2`/`post.md` + staged sensory-152-Rename) —
  unberührt.
- **Postfach** — keine Mountain-Zeile; jüngster Ledger-Eingang `1790116573`
  (Rubin Observatory LSST Community forum — Summary, Maschinendigest, keine
  Aktion). `post.md` fremd-dirty (sensory 153, 2 eigene Zeilen).
- **CI** (gemessen via `ci_manage list`/`view`/`log`): `te-gate 35834155918`
  @`7ed618d5a` **completed/failure** 13:24Z — Jobs `flare` (assert `te.rs:6239:
  13/30 < 50 %`) und `issue` (`gh_issue_once.sh: No such file or directory`,
  exit 127, fehlender Checkout) rot; `ksg-k-gate` **grün** (Log: `sweep void →
  K bleibt 0`). `ci-check`-Reds geroutet und von mycelium 146/147 geheilt
  (HAPI-Test, `len_zero`, river's `beat_pair`) — kein Mountain-Red. Aktiv:
  `ci-check 35868973875` in_progress.
- **register_lookup --open** — keine mountain-eigenen `DISPOSITION`-Zeilen;
  `INDEX 11 offen`, `CANDIDATES 1 → mycelium`.
- **open_points_check** — folge141: 13 Pfad-Refs, 0 absent.

## Erledigt in diesem Atom (git trägt es)

- **P1 KSG K-Regel** geschlossen: `ksg-k-gate`-Log misst `formula=8 sweep=void →
  K bleibt 0`; kein Code-Eingriff, der Log ist der Befund (zustand-Zeile
  „TE-Gate n=1000 FPR-Boden" trägt die Messung).
- **P3 te-gate `issue`-Job** — `actions/checkout@v7` ergänzt (`te-gate.yml`).
- **P2 flare-Gate** — Rat-Verdikt **B**: der assert-step
  `flare_envelope_conditional_keeps_true_coupling` verlässt den `flare`-Job
  (nie grün bei n=240: eigene ignore-Note 43 % < 50 %); der print-only
  `flare_envelope_power_probe` bleibt. Re-Budget an research geroutet
  (`post.md` `An research:`).

## Offen (aufgeschlüsselt)

### 1. te-gate green-confirm (flare + issue) — nach dem Push
- **Status:** termin | **Bindung:** eigen
- **Trigger:** te-gate-Dispatch nach dem Push (Workflow geändert) — der Lauf
  selbst ist die Messung
- **Lage:** `te-gate.yml` geändert (issue-Job +Checkout; flare-Job: assert-step
  entfernt, probe bleibt), gemessen 2026-09-23; es lief noch **kein** Lauf über
  die Änderung.
- **Blockade:** keine
- **Braucht:** `gh workflow run te-gate.yml` → run-id registrieren; einmal
  `ci_manage view <id>`; prüfen: `flare`-Job grün (Log-Zeilen
  `flare power probe: n=… power=…`), `issue`-Job kein exit-127. Die probe-Kurve
  ist der Trigger des research-Re-Budgets.

### 2. opencode-Browser Pfad 1 — Kaltstart (gemeldet „verbindet sich nicht")
- **Status:** operator-gebunden | **Bindung:** dritter
- **Trigger:** Operator-Wort — Kaltstart akzeptieren **oder**
  Extension-Änderung anstoßen
- **Lage:** kein harter Ausfall (gemessen 2026-09-23 via Session-Start-
  `browser_targets` + Messnachtrag
  `docs/surveys/survey-2026-09-20-browser-anbindung.md`): die Store-Extension
  (`cabnfapnafjlijmbpmgjkgobhdkbmpci` 0.16.1) verbindet sich erst beim nächsten
  Service-Worker-Wachruf; Abstand Broker→Executor bimodal (4–8 s warm,
  67–2276 s kalt); Reconnect ist ein `setTimeout`-Backoff im MV3-Worker.
  Offene Versionslücke: Plugin 0.17.0 vs Extension 0.16.1 (drop-in, Protokoll v1).
- **Blockade:** Heilung braucht eine Extension-Änderung (dritter, Store) — nicht
  aus `bridge.json`/`opencode.jsonc` lösbar.
- **Braucht:** Operator-Wort auf die vorgelegte Frage im Survey-Nachtrag.

### 3. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Geräteankunft (Tracking `LZ473049629CN`)
- **Lage:** PINE64 hat zwei Ox64 SBCs versandt, physisch nicht angekommen
  (gemessen 2026-09-23 via `state/mail/mail_ledger.φ` `1790046330`).
- **Blockade:** physische Ankunft.
- **Braucht:** nach Ankunft Bring-up + Kopplung messen.

## Benchmark

- Rat (pro/max) — Architektur-Entscheid flare-Gate, Verdikt **B** (der
  assert-step verlässt den Job, die Power-Studie geht als `pending` an research).
  Kein flash/pro-Doppellauf: Architektur-Klasse, kein Routine-Messatom (die
  Routine-Klasse ist seit 2026-09-16 geschlossen). Werkzeug-Fix und Handover
  mechanisch in der Hauptsession.

## Geteilter Baum — eigener Pfad-Satz

- `.github/workflows/te-gate.yml` — 2 eigene Hunks (issue-Job `+actions/checkout@v7`;
  flare-Job assert-step entfernt)
- `docs/handover/handover-2026-09-23-mountain-folge142.md` (neu)
- Move `handover-2026-09-23-mountain-folge141.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` — eigene `An research:`-Zeile ergänzt, aber post.md
  trägt **fremde** uncommittete sensory-153-Hunks → **nicht** von Mountain
  committet (Write-Boundary); die Zeile landet mit sensory's post.md-Commit.
- **Fremd/unberührt:** sensory 153 (`relay.rs`, `main_flow.rs`, `mod.rs`, `fit.rs`,
  `pulse_bridge.rs`, `auftrag-flyby2-kette.md`, post.md-Hunks) + staged
  sensory-152-Rename.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
