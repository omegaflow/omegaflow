<!--
  title: Handover — Befund-Migration: die 111 Befunde gelesen, klassifiziert, gelöscht
  class: handover
  date: 2026-09-10
  sha256: 8035c41aa18c601e369b41c15cc0e4b0d7338fbebd4d011a0924b55bd1669f56
  status: live
  see-also: docs/handover/handover-2026-09-10-autonom.md docs/handover/handover-2026-09-10-nicht-autonom.md
-->
# Handover — Befund-Migration (2026-09-10)

## Angenommen

- Die Migrations-Zeile des Autonom-Handovers (Stand db6a662): 111 Befunde
  im Baum, 93 mit „pending" — jeder wird umgesetzt oder gelöscht, kein
  Regal, kein neuer Befund, das Handover wächst nur durch geleistete Arbeit.

## Geleistet (Commit 10c830e)

- Alle 111 Befunde gegen Code, git und die live Handover gelesen (sechs
  Taucher, dann Extraktion): 108 gelöscht, weil das Finding im Code lebt
  (Sonde/Compiler mit benanntem Commit — z. B. `depthphase.rs`, `ak135.rs`,
  die ~60 Galileo-Sonden) oder überholt ist; 0 neue Befunde.
- Messergebnisse als Handover-Zeilen getragen: Galileo-RSS-Bestand,
  Dispersions-Ortungstest, Pioneer-1978–82-Ära, Voyager-Roh-Doppler
  (Autonom-Handover, Migrations-Zeile).
- Klasse gestrichen: `docs-naming.md` (`docs/befund/` = Alt-Bestand, leer),
  `commit_gate.rs` (Scan-Root, Completion-Wort, Test-Fixture), AGENTS.md
  (Rat-Zeile). Sechs Sonden-Diagnosen von Befund-Namen auf Code-Pfade
  umgestellt; alle Live-Verweise umverdrahtet (Blatt, drei Papers, zwei
  Concepts, Spec, Survey mit Nachsatz, zwei Handover, eine Reference);
  sha256-Heads neu berechnet.
- Der Gate hielt zweimal an `te_pair_probe.rs` (`unwrap_or_default`,
  deutsches Diagnose-Wort): `read_series` → `Option`, Match-Arme statt
  `unwrap_or`, Diagnosen englisch.
- Rat gehalten: Verdict „fix-named-points" — alle sieben Punkte umgesetzt
  (u. a. TRK-2-34 „0 B" → „absent", `job_dashboard.html` aus dem
  Migrations-Commit gehalten).
- Verifikation: `cargo check` 0 Warnungen (core, measure, harvest);
  88 Gate-Tests grün (`--features commit_gate`).

## Offen

- Die sechs Galileo-Floor-Atome stehen im Autonom-Handover (Sektion
  „Forschung — Galileo-Floor") — dort gearbeitet, nicht hierher verschoben.
- Beobachtet, nicht committet, nicht diese Session:
  `tools/service/src/assets/job_dashboard.html` — 269-Zeilen-Diff, seit vor
  dieser Session uncommitted, in keinem Register geführt.
  `tools/vo-tap/src/{lib,main}.rs` — Operator-Arbeit (Markus-Übergabe,
  Nicht-Autonom-Handover); nicht angefasst.

## Archiv

- Keine Übergabe archiviert: das Autonom-Handover bleibt das stehende
  Tagesregister — nur die Migrations-Zeile wurde darin gearbeitet und durch
  das Ergebnis ersetzt; der verbrauchte Stand ist git db6a662.
