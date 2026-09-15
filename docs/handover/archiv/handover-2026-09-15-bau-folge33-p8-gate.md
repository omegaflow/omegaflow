<!--
  title: Handover — Bau & Code (Stand 2026-09-15, Bau33)
  session: Bau-Folge 33
  class: handover
  date: 2026-09-15
  sha256: bac2449132c3f149b937feddb5710b6260320f302e4031a68036408d03c2d4f0
  status: live
-->
# Handover — Bau & Code (2026-09-15, Bau33)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

## P8 — Gate: template_slang + SECURITY.md-Ausnahme

- `src/gate/commit_gate.rs` + `src/gate/commit_gate_vocab.json` sind uncommittet:
  der `template_slang`-Block (verbietet das Slang-Wort, das vorher der `--all`-Flag
  trug), die `SECURITY.md`-Root-Ausnahme (`canonical_root_doc`), Fixture + Test
  `fp_tool_template_slang_blocked`. `commit_gate.rs` mischt fremdes `assert!`-Reformat
  einer anderen Session mit den 6 eigenen Hunks.
  (Schritt: sobald der Baum ruhig ist, `git add -p src/gate/commit_gate.rs` — nur die
  eigenen Hunks (template_slang-Feld/load/Loop, SECURITY-Ausnahme, SECURITY-Test-Zeile,
  `fp_tool_template_slang_blocked`) — dann `git add src/gate/commit_gate_vocab.json`
  und committen; `cargo check --features commit_gate` + `cargo test -p omegaflow
  --features commit_gate commit_gate`.)

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
