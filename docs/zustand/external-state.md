<!--
  title: Zustand — geteilter externer Zustand
  class: zustand
  date: 2026-09-16
  sha256: 7a9e466e347a20916468a4eb9e7485670198b15140b5391a6ea867838a86ae5e
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
| Postfach (extern) | letzter Eingang 2026-09-16T13:28Z (STScI-Newsletter; davor Sotgiu-Antwort 13:22Z „Re: CSES-Limadou" → CSES-02-Umstellung, „wait a few weeks"; davor die fünf Sonden-Anfrage-Kopien 13:47–13:55Z); offen: GitHub-GC #4761801, Privacy-Antwort, Rubin-Review, NSE/Haug | 2026-09-16T13:56Z | neuer Ledger-Eingang oder 2⁶ min | `state/mail/mail_ledger.φ` (`smail` + `cloudflared`) |
| GitHub PII-Exposition | 45 (Datei, Ref)-Kombinationen aus zehn PII-tragenden Dateien, 15/15 Pre-Rewrite-Commits erreichbar; PII retrievable: yes; GC offen | pii-exposure run 35101796537 @ 8d0c9090 (2026-09-16; Vormessung 35081360127 @ 2187c30c: 45) | HEAD-Wechsel oder GitHub-GC-Antwort | `gh workflow run pii-exposure.yml` (oder `curl` GH-REST) |
| CI-Status | rot @ 625452e5: format (commit_gate.rs:540; harvest/src/bin/{gedi_l2a,icesat2_atl03,noaa_cdo,swot_l2_lr_ssh,vlass_tap}_compiler.rs; measure/src/bin/{aia_ladder,trishuli_gauge}_probe.rs; utils/src/bin/archive_search.rs), clippy grün, build grün, test cancelled/unvermessen (ci-check run 35091175017: format-Job 104777576508, test-Job 104777576226) | 625452e5 (2026-09-16) | HEAD-Wechsel | Check-Runs des gepushten SHA (GH-API) — test-Job braucht neuen, nicht abgebrochenen Lauf |
| TE-Gate n=1000 FPR-Boden | rot: residual/ksg 19.51 % @ a=0.9 (Boden 8 %), phase/ksg 8.52 %, restricted/binned 10.71 %; block/binned rise 2.47pp, block/ksg 2.75pp (Grenze 2pp); shift_sweep grün (binned/ksg ≤ 6.87 %); Issue „te-gate: the n=1000 FPR gate carries a finding" offen | te-gate run 35092997862 @ bf2423a5 (2026-09-16) | HEAD-Wechsel | Issue lesen; `src/mathematikerin/te.rs:3185` (8 %-Boden) / `:3206` (2pp-Anstieg) |
