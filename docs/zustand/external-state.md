<!--
  title: Zustand — geteilter externer Zustand
  class: zustand
  date: 2026-09-16
  sha256: 347d30ee6005b64014f5e03b0c756910c5fe558da56a768c7ed8bcec32dc5e1a
  status: live
  see-also: AGENTS.md
-->
# Zustand — geteilter externer Zustand

Eine Zeile je Abhängigkeit: Wert | measured-at | fällig | Schritt. Eine Session
misst nur, wenn der Eintrag fällig ist oder sein Trigger gefeuert hat; sonst
zitiert sie den Eintrag. Kein Handover kopiert einen dieser Werte — es nennt den
Eintrag. Ein Wert-am-SHA ist sein eigenes Gate: dieselbe SHA erneut messen heißt
dieselbe Sache messen und eine andere Antwort erwarten (A = A). Ein verfallener
Eintrag ist nicht null — er ist `pending` mit Fälligkeit, eine Registraturpflicht.

| Abhängigkeit | Wert | measured-at | fällig | Schritt |
|---|---|---|---|---|
| Postfach (extern) | letzter Eingang 00:40 (Rubin-Freigabe); offen: GitHub-GC #4761801, Privacy-Antwort, Rubin-Review, NSE/Haug, CSES-Limadou | 2026-09-16 10:56 | neuer Ledger-Eingang oder 2⁶ min | `state/mail/mail_ledger.φ` (`smail_recv` + `cloudflared`) |
| GitHub PII-Exposition | 45 (Datei, Ref)-Kombinationen aus zehn PII-tragenden Dateien, 15/15 Pre-Rewrite-Commits erreichbar; GC offen | 9fb70eb4 | HEAD-Wechsel oder GitHub-GC-Antwort | `cargo run -p omegaflow-register --bin pii_exposure` |
| CI-Status | rot: clippy/format failure, build success, test in_progress (ci-check in_progress) | 9fb70eb4 | HEAD-Wechsel | Check-Runs des gepushten SHA (GH-API) |
