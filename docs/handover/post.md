<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-16
  sha256: fb404eaab09c1929508afbb6b4dc7ee8c61a9a88157740eb436f494c3c900436
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und löscht die Zeile.

An bau: Gate-Fixtures für die Consent-Akt-Vokabeln (Mail senden, Konto/API-Key anlegen, Antrag/Auskunft, Einreichung an fremder Stelle, Vertragsannahme, Zahlung, fremdes Konto löschen) + Gate-Test im selben Atom — die strukturelle Sichtbarkeit für die curl-Schreibpfade (der `smail`-Pfad selbst ist Default-dry-run, `--send` explizit). (Schritt: `src/gate/commit_gate_vocab.json` + Test in `src/gate/commit_gate.rs`.)
