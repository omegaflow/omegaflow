<!--
  title: Handover — Bau-Folge 116 (Stand 2026-09-21)
  session: Bau-Folge 116
  class: handover
  date: 2026-09-21
  sha256: 8b9396d472407bea3faf2ef277878eb2b7374197461a6d72024d5497d0f30612
  status: live
-->
# Handover — Bau-Folge 116 (2026-09-21)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage.
Der Planungs-Pass nennt die offenen Punkte als Tafel (Punkt | Status | Bindung |
Schritt); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-21, Session-Beginn)

- **HEAD** `f95e6d9a` == `origin/main` (Forschung-Folge 126); Arbeitsbaum trägt
  zwei **fremde** uncommittete Dateien (`phi/blocked_sources.φ`,
  `tools/harvest/src/bin/ps1_coverage_compiler.rs`) — nicht angefasst.
  `git_safety --snapshot`: `refs/safety/1789971364`.
- **Postfach** — kein `An bau` in `post.md`; `external-state.md`-Eingang
  unverändert (`1789970277`, Framework-decline, entscheid folge71). Keine neue
  eigene Post zu falten.
- **CI** — `ci_manage list`: `ci-check` `35566258372` **in_progress** (hängt seit
  05:53, kein Update), `ci-check` `35567752434` **pending** @06:16, vier
  `ci-check` **cancelled** (`35567704064`/`35567688093`/`35567290094`/`35567260076`
  = Concurrency-Kette); `hyperscanning-te` `35567708611` failure; `te-gate`
  `35567711055` in_progress; `tools-build` `35567704085` in_progress; `ps1-cdn`
  `35563001793` in_progress; `free-model-bench`/`free-model-agent-bench`
  in_progress; `demeter-cdn` `35567568429` queued; `health-check` `35556807317`
  pending.

## Offen

| Punkt | Status | Bindung | Schritt |
|---|---|---|---|
| „strukturierte Feld-Grammatik" (Kanon-Akt) | `operator-gebunden` | `operator` (via entscheid) | entscheid legt die Frage beim Operator-Rückkehr vor (Operator-Queue, entscheid folge71). |
| ci-check format grün (fremde Hunks) | `wartend` | `linie:ernte/forschung/entscheid` | `post.md`-Zeilen gesetzt; Trigger: deren fmt-Fixes + frischer `ci-check` (`ci_manage view <id>`). |

## Messung dieses Atoms (kein Punkt)

- **fmt-Rot der eigenen `src/archivar`-Hunks geheilt** (kein lokaler fmt — CI-only,
  Hand-Formatierung nach dem CI-Diff):
  - `src/archivar/relay.rs` (`relay_ws_config`): Presence-Tupel einzeilig
    `mpsc::channel::<(String, f64×10)>()`.
  - `src/archivar/tests.rs` (`field crosswind …`): `fields.contains("…")` einzeilig.
  - Gemessen im format-Job `35537130867` @`5894b345`; Zuschreibung per `git blame`:
    tests.rs-Hunk = bau (`68a8240f` folge109), relay.rs-Hunk = forschung
    (`f75e3245` folge121) — als Crate-Owner `src/archivar` mitgeheilt.
  - `cargo check --all-targets --features browser_relay`: 0 Fehler, 0 Warnungen.
- **Fremde fmt-Stellen gepostet** (format-Job `35537130867` @`5894b345`): ernte
  (`tools/harvest/bia_efield_compiler.rs:287`,
  `ps1_coverage_compiler.rs:605,644`), forschung
  (`tools/measure/hyperscanning_group_te.rs:583`,
  `tools/register/register_lookup.rs:1335,1513,1630,2294`), entscheid
  (`tools/measure/free_model_agent_bench.rs`, `free_model_bench.rs`). Der
  format-Job bleibt rot, bis diese Linien ihre Hunks heilen.
- **Postfach:** drei `An …`-Zeilen geschrieben (ernte/forschung/entscheid).

## Benchmark

- **Bau-Folge 116**: kein Sub-Agent — der Fix ist zwei Zeilen Hand-Formatierung,
  direkt im `build`-Kontext. Kein flash/pro-Vergleich nötig.

## Geteilter Baum — eigener Pfad-Satz

- **Dieser Session:** `src/archivar/relay.rs`, `src/archivar/tests.rs`,
  `docs/handover/post.md`, `docs/zustand/external-state.md`, neues
  `docs/handover/handover-2026-09-21-bau-folge116.md`, Move
  `handover-2026-09-21-bau-folge115.md` → `archiv/`.
- **Fremd (nicht anfassen):** `phi/blocked_sources.φ`,
  `tools/harvest/src/bin/ps1_coverage_compiler.rs` (uncommitted). Nie ein nacktes
  `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
