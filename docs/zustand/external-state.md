<!--
  title: Zustand — geteilter externer Zustand
  class: zustand
  date: 2026-09-16
  sha256: a5a9fcadb51288f7530cc423b19e59095a91691a09180045468520f8467b321e
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
| Postfach (extern) | letzter Eingang 00:40 (Rubin-Freigabe); offen: GitHub-GC #4761801, Privacy-Antwort, Rubin-Review, NSE/Haug, CSES-Limadou | 2026-09-16 07:42 | neuer Ledger-Eingang oder 2⁶ min | `state/mail/mail_ledger.φ` (`smail_recv` + `cloudflared`) |
| GitHub PII-Exposition | 45 (Datei, Ref)-Kombinationen aus zehn PII-tragenden Dateien, 15 Pre-Rewrite-Refs; GC offen | ce367dd0 | HEAD-Wechsel oder GitHub-GC-Antwort | `cargo run -p omegaflow-register --bin pii_exposure` |
| CI-Status | rot: test/build/clippy/format failure, gate in_progress, allwise-coverage success | c179220d | HEAD-Wechsel | Check-Runs des gepushten SHA (GH-API) |
