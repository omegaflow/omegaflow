<!--
  title: Zustand — geteilter externer Zustand
  class: zustand
  date: 2026-09-16
  sha256: c1a94d141fc1f7604e5462734ef70a44d5e15ae168e9deb88499d146fba571e7
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
| Postfach (extern) | letzter Eingang 2026-09-15 22:30Z (Rubin RSP: in Einzelprüfung); offen: GitHub-GC #4761801, Privacy-Antwort, Rubin-Review, NSE/Haug, CSES-Limadou | 2026-09-16T09:23Z | neuer Ledger-Eingang oder 2⁶ min | `state/mail/mail_ledger.φ` (`smail` + `cloudflared`) |
| GitHub PII-Exposition | 44 (Datei, Ref)-Kombinationen aus zehn PII-tragenden Dateien, 15/15 Pre-Rewrite-Commits erreichbar; GC offen | b66a9c94 | HEAD-Wechsel oder GitHub-GC-Antwort | `gh workflow run pii-exposure.yml` (oder `curl` GH-REST) |
| CI-Status | rot: clippy failure, format failure, build success, test in_progress (ci-check #411) | b66a9c94 | HEAD-Wechsel | Check-Runs des gepushten SHA (GH-API) |
