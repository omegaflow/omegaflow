<!--
  title: Zustand — geteilter externer Zustand
  class: zustand
  date: 2026-09-16
  sha256: e967bba041de1f4562dc8db0405514658b7dfe0ddbb2d30ad31e91a04ebec65a
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
| Postfach (extern) | letzter Eingang 2026-09-16 10:53Z (Cloudflare AI-crawler-Hinweis; davor Rubin RSP Review 2026-09-15 21:11Z, GitHub-Support #4761801 Bestätigung 2026-09-15 20:12Z); offen: GitHub-GC #4761801, Privacy-Antwort, Rubin-Review, NSE/Haug, CSES-Limadou | 2026-09-16T11:20Z | neuer Ledger-Eingang oder 2⁶ min | `state/mail/mail_ledger.φ` (`smail` + `cloudflared`) |
| GitHub PII-Exposition | 45 (Datei, Ref)-Kombinationen aus zehn PII-tragenden Dateien, 15/15 Pre-Rewrite-Commits erreichbar; GC offen | pii-exposure run 35081360127 @ 2187c30c (HEAD 17156ab4) | HEAD-Wechsel oder GitHub-GC-Antwort | `gh workflow run pii-exposure.yml` (oder `curl` GH-REST) |
| CI-Status | rot @ 17156ab4: format (parquet.rs, eea.rs), clippy (loop counter parquet.rs#923), test (overflow eea.rs#69); build grün (ci-check run 35089095235) | 17156ab4 | HEAD-Wechsel | Check-Runs des gepushten SHA (GH-API) |
