<!--
  title: Handover — Ernte-Folge 87 (Stand 2026-09-18)
  session: Ernte-Folge 87
  class: handover
  date: 2026-09-18
  sha256: 8c61956687e6d04189c979f68304a0e8a77b2d5e6bc6dd49232b04abfbf29358
  status: live
-->
# Handover — Ernte-Folge 87 (2026-09-18)

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
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-18)

- **HEAD** `533c4b2a` == `origin/main` (Fast-Forward, Push steht). Safety-Net
  `refs/safety/1789738817`.
- **Postfach** — external-state CI-Ledger-Eintrag zitiert (letzter Eingang
  `1789737560`, eigene Antwort an PI Iess, aus Proton gesendet); kein eigener
  Eingang, Trigger (neuer Eingang) nicht gefeuert. Eigener Post gesetzt:
  `An bau:` `register_lookup --open` fehlt (siehe `post.md`).
- **CI** — Watchdog 15:12Z + `ci_manage list` 15:4xZ: gedi `35351460528` /
  atl03 `35351464250` in_progress, LRO `35351467845` / rosetta `35351411938`
  pending; `harvest-dispatch` `35351394369`/`35348641264`, `auto-dispatch`
  `35351394111`, `pages-deploy` `35349393629`, `swpc-mirror-cdn` `35347254759`
  success; die `ci-check`-Serie cancelled. `docs/zustand/external-state.md`
  CI-Status-Zeile aktualisiert.

## Kein abarbeitbarer undatierter Punkt

Die Register-Digest-Lücke ist gemessen und geschlossen: `register_lookup --open`
ist nicht gebaut (der Aufruf gibt die usage-Zeile `<term> | --live | --history`);
der Befund steht als `An bau:`-Zeile in `docs/handover/post.md`. Alle
verbleibenden Punkte sind `wartend` mit ungefeuertem Trigger (Run-Abschluss) —
keine Aktion in dieser Session, kein erfundener Schritt.

## gedi_l2a / icesat2_atl03 — Wurzel `dataset absent` (wartend)

Re-Dispatch nach dem hdf5-Continuation-Zyklus-Fix (`src/archivar/hdf5.rs`,
Commit `c4533fe9`; Test `v2_continuation_self_cycle_terminates`). Dispatcht
2026-09-18: gedi_l2a `35351460528`, icesat2_atl03 `35351464250` — beide
in_progress.
- **Schritt (Trigger Run-Abschluss):** einmalig `ci_manage view <id>`; bei
  success `asset present` + sha256 in `phi/sources.φ`. Nie pollen.

## LRO utF harvest — Timeout zu kurz (wartend)

Lauf `35339151004` durch `timeout 8` abgebrochen, der Compiler produktiv (Tag
267/365 in 7,3 min); `phi/harvest.φ` `timeout` auf 16 erhöht, dispatcht
`35351467845` — pending.
- **Schritt (Trigger Run-Abschluss):** einmalig `ci_manage view 35351467845`;
  bei success `shard` ins Register = gemessene Asset-Zahl.

## rosetta_odf — Shard-Manifestation (wartend)

Lauf `35313968728` HTTP 422 (`rosetta_odf.bin` 2415275024 B > 2³¹);
`rosetta_odf_compiler.rs` shardet (PODF_SHARD_BUDGET, 3 Shards),
`planetary-odf-cdn.yml:30` `sharded: true, prefix: rosetta_odf`. Dispatcht:
`planetary-odf-cdn` `35351411938` — pending; `harvest-dispatch` `35351394369`
routet rosetta zusätzlich via `harvest.yml` (Upload `--clobber`, idempotent).
- **Schritt (Trigger Run-Abschluss):** nach success die 3 Shard-`url`+`sha256`
  in `phi/sources.φ` (ersetzt die Einzeldatei-`url`), `phi/harvest.φ` rosetta
  `asset present`; die Shard-Namen stehen im Compile-stdout.

## HTTP-Reach-Zahl je Granule (Council, nicht blockierend)

Überlappende 64-B-then-Span-Fetches verfehlen den Cache per Containment —
langsamere Harvests, korrekte Records (atl03: 427 MB in 1h37m). Lauf-Zeit im
nächsten CI-Ergebnis nennen.

## Benchmark

Log-Messung + Regressions-Test `grind-flash`; hdf5-Diagnose/Fix + rosetta-Shard
`grind-pro` — Routine, kein Doppel-Lauf, kein harter Doppel-Benchmark.
Schiedsrichter ist der nächste CI-Lauf. Diese Session: kein Agent-Dispatch
(kein abarbeitbarer Punkt; flash-first wäre hier die Session selbst).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `docs/handover/handover-2026-09-18-ernte-folge87.md` (neu),
  `docs/handover/archiv/handover-2026-09-18-ernte-folge86.md` (verschoben),
  `docs/zustand/external-state.md` (CI-Status-Zeile),
  `docs/handover/post.md` (`An bau:`-Zeile).
- **Fremd (nicht anfassen):** `AGENTS.md`, `src/gate/commit_gate.rs`,
  `src/gate/commit_gate_vocab.json`,
  `tools/measure/src/bin/weberin_verdicts_compiler.rs`,
  `.github/workflows/weberin-verdicts-cdn.yml`, `phi/sources.φ` (fremd
  modifiziert), die `handover-2026-09-16-*`-Moves, die `forschung-folge83`-Dateien.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
