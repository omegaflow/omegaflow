<!--
  title: Zustand — geteilter externer Zustand
  class: zustand
  date: 2026-09-16
  sha256: 5e1c2879a60599da20eb33a8dfa97aeea68abafc34175d9e153dc9bc616e6d57
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
| CI-Status | pending: ci-check run 35137777699 @ e7795aea in_progress; 35134701772 @ 4254770c failure (format rot: fremde Dateien src/archivar/*, s2.rs; Bau-Datei commit_gate.rs:540 in diesem Atom formatiert) | e7795aea (2026-09-16) | HEAD-Wechsel (dieser Atom-Push) | Check-Runs des gepushten SHA (GH-API) |
| TE-Gate n=1000 FPR-Boden | rot (alte Nulls): residual/ksg 19.51 % @ a=0.9 (Boden 8 %), phase/ksg 8.52 %, restricted/binned 10.71 %; block/binned rise 2.47pp, block/ksg 2.75pp (Grenze 2pp); shift_sweep grün (binned/ksg ≤ 6.87 %); Arx-Switch committet (`1337c6c1`), Verifikation läuft: te-gate run 35129638318 @ 1337c6c1 (in_progress) + 35129679496 (pending); konditionales FP/FN-Gate gebaut (`gate_conditional_arx_fpr_fn_n1000`, `gate_conditional_arx_2_fpr_n1000`, `conditional_arx_fit_resolves_at_small_n`); `arx_restricted_surrogate_conditional`/`_2` geben jetzt `Option` (None = Verweigerung, kein Shuffle) | te-gate run 35092997862 @ bf2423a5 (2026-09-16; alte Nulls) | HEAD-Wechsel / te-gate-Run-Abschluss | `gh run view 35129638318` (Arx n=1000 + block_sweep); Issue „te-gate: the n=1000 FPR gate carries a finding" lesen |
| dr3_stars.bin (CDN) | 75 001 828 B = 44-B-Stride (`rv`), sha256 fb9a14089ef8348e12961caf8253c8e886b4a16862a1a71394b96465d75bcfbb | 2026-09-16 (Asset-Stand) | HEAD-Wechsel / gaia-cdn Re-Dispatch | `gh workflow run gaia-cdn.yml`, dann `archive_search --sniff https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/dr3_stars.bin` |
