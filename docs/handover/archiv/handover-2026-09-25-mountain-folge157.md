<!--
  title: Handover — Mountain-Folge 157 (2026-09-25)
  session: Mountain-Folge 157
  class: handover
  date: 2026-09-25
  sha256: 3db10f312a8274aec6c678c95525558d2918faa4f8ce733da0680f1267e4142e
  status: live
-->
# Handover — Mountain-Folge 157 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Geschichts-Abschnitte (Stehender-Pass-Ergebnis, geschlossen-Register,
Benchmark, Geteilter Baum): sie leben in git. Geteilter externer Zustand lebt in
`docs/zustand/external-state.md`, nie als Kopie hier. Keine Rangfolge — die offenen
Punkte werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Sortierung von Handlungsfähigkeit zu Nicht-Handlungsfähigkeit:
`autonom` → `operator-gebunden` → `blockiert` → `wartend` → `termin` → `LOCK`.

Diese Session konsumierte `docs/handover/archiv/handover-2026-09-25-mountain-folge156.md`
(zum Session-Beginn noch `docs/handover/`).

## Offen (aufgeschlüsselt)

#### Stufe 1 — autonom

### gap-Port der `parser-def`/`parser-gap`-Blöcke + Mountain-Klassen-Träger
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** nächster Dispatch — der Register-Baum ist jetzt sauber (die fremden
  Register-Commits in `phi/blocked_sources.φ`, `phi/declined_sources.φ` und
  `phi/pipeline/index.φ` sind gelandet, `git status` trägt nur noch diese Session).
- **Lage:** (gemessen 2026-09-25 @`da1aafc8` via `register_lookup --orphans`)
  197 Orphans — mountain 188, mycelium 6, future 3, alle `ORPHAN_COMMITTED` (in HEAD);
  0 `gap`-Direktiven im Bestand (`sgrep -c "gap "` = 0), daher trägt jeder
  Mountain-Eintrag den Marker `NO_GAP (gap directive absent)`. Scanner + Regel +
  Tests sind in diesem Atom gebaut.
- **Blockade:** keine.
- **Braucht:** `gap <token>` in jeden `parser-def`/`parser-gap`-Block setzen (Tokens:
  `unit-auto-detect`, `force-undetermined`, `votable-reader`, `html-parser-arm`,
  `konverter`; eine Klasse existiert nur, wo das Register sie erklärt — A = A;
  mechanisch, `grind-flash`), dann die Mountain-Klassen-Träger in **dieses** Handover
  in der festen Form `phi/blocked_sources.φ::gap:<token> ×N` (N = live count)
  eintragen, dann `register_lookup --orphans` als Nulllinie messen. Die 6 mycelium-
  und 3 future-Orphans trägt jede Linie selbst: der Scanner nennt sie owner-getaggt
  in ihrem nächsten Planungs-Pass (`register_lookup --open`, ORPHAN-Sektion).

#### Stufe 4 — wartend

### `register_lookup --orphans` auf PATH (Release-Artefakt hinkt HEAD)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `tools-build` success am HEAD nach dem Push dieser Session.
- **Lage:** (gemessen 2026-09-25 @`da1aafc8` via `register_lookup --help`) das
  PATH-Binär kennt `--orphans` nicht (exit 2, Usage listet nur `--open`/`--dropped`/
  `--history`); der lokale Debug-Build (`cargo run -p omegaflow-register --bin
  register_lookup -- --orphans`) kennt es und misst 197 Orphans.
- **Blockade:** das via `tools-latest` publizierte Session-Tool ist ein
  CI-/Deploy-Produkt und liegt hinter HEAD.
- **Braucht:** nach `/commit` `gh workflow run tools-build.yml`; danach
  `bin/.tools_ensure register_lookup` (sha256-Abgleich) und `register_lookup --help`
  auf `--orphans` prüfen.

#### Stufe 3 — blockiert

keiner. / Stufe 5 — termin: keiner. / Stufe 6 — LOCK: keiner.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort. Diese Session trägt:
`tools/register/src/bin/register_lookup.rs` (Scanner `--orphans` + ORPHAN-Sektion in
`--open` + Tests), `tools/register/src/bin/open_points_check.rs` (Globs und
`::gap:`-Klassenschlüssel sind keine Repo-Pfade), `AGENTS.md` (Regel „Aufenthalt =
Eigentum — die Registerseite"), `docs/concepts/tools-map.md` (`--orphans`), diese
Übergabe + der Move der konsumierten folge156 ins `archiv/`.
