<!--
  title: Handover — Ernte-Folge 86 (Stand 2026-09-18)
  session: Ernte-Folge 86
  class: handover
  date: 2026-09-18
  sha256: 5a01e49481aedaaeff129fe5041854e735449ba5d5b23720dbba96244427bdaf
  status: live
-->
# Handover — Ernte-Folge 86 (2026-09-18)

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

- **HEAD** `624a195f` == `origin/main` (Fast-Forward, Push steht). Safety-Net
  `refs/safety/1789737134`.
- **Postfach** — external-state CI-Ledger-Eintrag zitiert (letzter Eingang
  `1789737560`, eigene Antwort an PI Iess; gemessen Entscheid-Folge 51). Kein
  eigener Post; Trigger (neuer Eingang) in dieser Session nicht gefeuert.
- **CI** — Watchdog-Snapshot 12:38Z + `ci_manage list` 13:15Z: gedi/atl03
  cancelled (Compiler-Hang, Fix dieser Session), LRO cancelled (Timeout 8),
  rosetta failure (2³¹-Wand), bc_mpo_mag success. `docs/zustand/external-state.md`
  CI-Status-Zeile aktualisiert.

## gedi_l2a / icesat2_atl03 — Wurzel `dataset absent` (wartend)

Re-Dispatch nach dem hdf5-Continuation-Zyklus-Fix (`src/archivar/hdf5.rs`,
Commit dieser Session; Test `v2_continuation_self_cycle_terminates`). Dispatcht
2026-09-18: gedi_l2a `35351460528`, icesat2_atl03 `35351464250`.
- **Schritt (Trigger Run-Abschluss):** einmalig `ci_manage view <id>`; bei
  success `asset present` + sha256 in `phi/sources.φ`. Nie pollen.

## LRO utF harvest — Timeout zu kurz (wartend)

Lauf `35339151004` durch `timeout 8` abgebrochen, der Compiler produktiv (Tag
267/365 in 7,3 min); `phi/harvest.φ` `timeout` auf 16 erhöht, dispatcht
`35351467845`.
- **Schritt (Trigger Run-Abschluss):** einmalig `ci_manage view 35351467845`;
  bei success `shard` ins Register = gemessene Asset-Zahl.

## rosetta_odf — Shard-Manifestation (wartend)

Lauf `35313968728` HTTP 422 (`rosetta_odf.bin` 2415275024 B > 2³¹);
`rosetta_odf_compiler.rs` shardet (PODF_SHARD_BUDGET, 3 Shards),
`planetary-odf-cdn.yml:30` `sharded: true, prefix: rosetta_odf`. Dispatcht:
`planetary-odf-cdn` `35351411938`; `harvest-dispatch` `35351394369` routet
rosetta zusätzlich via `harvest.yml` (Upload `--clobber`, idempotent).
- **Schritt (Trigger Run-Abschluss):** nach success die 3 Shard-`url`+`sha256`
  in `phi/sources.φ` (ersetzt die Einzeldatei-`url`), `phi/harvest.φ` rosetta
  `asset present`; die Shard-Namen stehen im Compile-stdout.

## HTTP-Reach-Zahl je Granule (Council, nicht blockierend)

Überlappende 64-B-then-Span-Fetches verfehlen den Cache per Containment —
langsamere Harvests, korrekte Records (atl03: 427 MB in 1h37m). Lauf-Zeit im
nächsten CI-Ergebnis nennen.

## Register-Digest-Lücke (gemessen)

`register_lookup --live` (673 Zeilen) lieferte Doc-Index + Summary (111 Docs,
542 offen), aber keine owner-getaggten `phi/*.φ`-Zustandsregister-Einträge.
- **Schritt:** `register_lookup --live | sgrep -c 'blocked_sources\|ledger\|harvest'`;
  bei 0 ein Post an die `bau`-Linie.

## Benchmark

Log-Messung + Regressions-Test `grind-flash`; hdf5-Diagnose/Fix + rosetta-Shard
`grind-pro` — Routine, kein Doppel-Lauf, kein harter Doppel-Benchmark.
Schiedsrichter ist der nächste CI-Lauf.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `src/archivar/hdf5.rs`,
  `tools/harvest/src/bin/rosetta_odf_compiler.rs`, `phi/harvest.φ`,
  `.github/workflows/planetary-odf-cdn.yml`,
  `docs/zustand/external-state.md` (CI-Status-Zeile),
  `docs/handover/handover-2026-09-18-ernte-folge86.md` (+ archiviertes
  `handover-2026-09-18-ernte-folge85.md`).
- **Fremd (nicht anfassen):** `AGENTS.md`, `src/gate/commit_gate.rs`,
  `src/gate/commit_gate_vocab.json`, `src/mathematikerin/te.rs`,
  `phi/sources.φ` (fremd modifiziert), `tools/measure/src/bin/weberin_verdicts_compiler.rs`,
  `betti0_probe.rs`, `silence_map_probe.rs`, `.github/workflows/weberin-verdicts-cdn.yml`,
  die `handover-2026-09-16-*`-Moves, `handover-2026-09-18-forschung-folge82.md`,
  `handover-2026-09-18-bau-folge84.md`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
