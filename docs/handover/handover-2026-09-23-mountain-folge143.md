<!--
  title: Handover — Mountain-Folge 143 (Stand 2026-09-23)
  session: Mountain-Folge 143
  class: handover
  date: 2026-09-23
  sha256: bdfa53b92e8b0490befd89bce42dfdc16835fe49a50996b2a1e249a3c14f1b2a
  status: live
-->
# Handover — Mountain-Folge 143 (2026-09-23)

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
Punkt wird **aufgeschlüsselt** geführt — kein Register-Kürzel: **Trigger** (das
Ereignis/Datum/Wort/der Lauf, dessen Eintreffen den Punkt kippt — Status =
f(Trigger)) / **Lage** (der Zustand, gemessen, mit Messstempel) / **Blockade**
( woran es hängt, oder „keine") / **Braucht** (was es löst: der wörtliche,
kopierbare Schritt — Werkzeug/Datei/URL/Befehl/Operator-Wort). `operator-gebunden`,
`blockiert` und `wartend` werden benannt, nie dispatcht. Jeder Punkt trägt seinen
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`open_points_check`/`sgrep`/`git log`/`sread`) — das Register ist die Frage, der
Baum die Messung; `open_points_check` prüft billig jeden in den offenen Punkten
genannten Pfad gegen den Arbeitsbaum (absent = stale Punkt); eine Session, die nur
dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-23 ~15:05Z)

- **HEAD** `2af5b5b51` (`main`) == `origin/main`. Arbeitsbaum trägt **fremde**
  Arbeit (river folge15/16: `AGENTS.md`, Move `handover-…-river-folge15` → `archiv/`,
  `handover-…-river-folge16.md`; sensory 153: `src/archivar/main_flow.rs` + `src/archivar/mod.rs`,
  `fit.rs`, `post.md`-Hunks, `handover-…-sensory-folge153.md`;
  `docs/auftrag/auftrag-flyby2-kette.md`; staged sensory-152-Rename) — unberührt.
- **CI** (gemessen via `ci_manage`): `te-gate 35875025486` @`122d36ef2` — `flare`-Job
  grün; die Power-Probe misst **n=400 → 0,933 (28/30), n=600 → 1,000, n=1000 →
  1,000** (`ci_manage log 35875025486 --all`); `view` liest den Lauf noch
  `in_progress` (Restjobs). Kein Mountain-Red.
- **Postfach** (gemessen via `state/mail/mail_ledger.φ`) — keine Mountain-Zeile;
  jüngste: `1790171985` (STScI-News, Maschinendigest), `1790155037`/`1790154958`/
  `1790154897`/`1790154842` (SSDC-Passwort-Recovery-Serie `omegaflow`, Konto — nicht
  mountain-eigen), `1790116573` (Rubin-Forum-Summary, Maschine).
- **register_lookup --open** — keine mountain-eigenen `DISPOSITION`-Zeilen;
  `open_points_check` folge142: 5 Pfad-Refs, 0 absent.

## Offen (aufgeschlüsselt)

### 1. flare-Re-Insert green-confirm — nach dem Push
- **Status:** termin | **Bindung:** eigen
- **Trigger:** der neue `te-gate`-Lauf nach dem Push (Workflow geändert) — der Lauf
  selbst ist die Messung
- **Lage:** `te.rs` `flare_envelope_conditional_keeps_true_coupling` n=240→**400**,
  ignore-Note korrigiert; `te-gate.yml` `flare`-Job trägt den assert-step **wieder**
  (vor der Probe) (gemessen 2026-09-23, `cargo check -p omegaflow --lib` 0/0);
  `grind-flash`-Delegation. Rat-Verdikt **B′**: der assert kehrt zurück, **n=400**
  (kleinstes n über dem 50 %-Boden), Floor 0.5 bleibt — die festen Seeds
  reproduzieren deterministisch 28/30; Operator bestätigte n=400 (2026-09-23).
- **Blockade:** keine
- **Braucht:** nach dem Push einmalig `ci_manage view <neuer te-gate-run>` → `flare`-Job
  grün (assert + `flare power probe:`-Zeilen).

### 2. te-gate `issue`-Job-Text — nennt nur den n=1000-FPR-Gate
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächster Mountain-Pass
- **Lage:** `te-gate.yml:156–160` Titel/Body nennen „the n=1000 FPR gate" auch dann,
  wenn allein der `flare`-assert rot ist (Rat benannt 2026-09-23, sensory-Nennung).
- **Blockade:** keine
- **Braucht:** Issue-Titel/Body gate-neutral fassen (`te-gate.yml:154–160`).

### 3. Ox64-Zweitknoten
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Geräteankunft (Tracking `LZ473049629CN`)
- **Lage:** PINE64 hat zwei Ox64 SBCs versandt, nicht angekommen (gemessen 2026-09-23
  via `state/mail/mail_ledger.φ` `1790046330`).
- **Blockade:** physische Ankunft.
- **Braucht:** nach Ankunft Bring-up + Kopplung messen.

## Benchmark

- **Rat** (pro/max) — Abschluss-/Architektur-Entscheid flare-Re-Insert: Verdikt
  **B′** (assert zurück in den `flare`-Job, **n=400**, Floor 0.5, Owner Mountain;
  die stale `An research:`-Route aufgelöst; `issue`-Text als Risiko benannt). Kein
  flash-Doppellauf für die mechanische Edit (`grind-flash`, `cargo check` 0/0) —
  Routine-Klasse seit 2026-09-16 geschlossen.

## Geteilter Baum — eigener Pfad-Satz

- `src/mathematikerin/te.rs` — 1 eigener Hunk (n=240→400, ignore-Note)
- `.github/workflows/te-gate.yml` — 1 eigener Hunk (assert-step im `flare`-Job)
- `docs/handover/handover-2026-09-23-mountain-folge143.md` (neu)
- Move `handover-2026-09-23-mountain-folge142.md` → `archiv/` (eigene Linie, atomar)
- `docs/handover/post.md` — eigene Hunks: stale `An research:`-Zeile **entfernt**,
  neue `An future:`-Zeile (Browser-Kaltstart) ergänzt; trägt **fremde** uncommittete
  sensory-Hunks → **nicht** von Mountain committet (Write-Boundary).
- **Fremd/unberührt:** river folge15/16 (`AGENTS.md`, river-Handover-Move + folge16),
  sensory 153 (`src/archivar/main_flow.rs` + `src/archivar/mod.rs`, `fit.rs`, `post.md`-Hunks,
  `handover-…-sensory-folge153.md`), `docs/auftrag/auftrag-flyby2-kette.md`,
  staged sensory-152-Rename.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
