<!--
  title: Zustand — geteilter externer Zustand
  class: zustand
  date: 2026-09-16
  sha256: 1c661df4791614e9830ac3f8449603440c0d540c3e120415f7e8d2892c8d18fb
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
| CI-Status | HEAD b85438ba — gemessen 2026-09-16: nexrad-cdn 35144618354 success, voyager-odr-series 35143340703 success (14 Shards), te-gate 35129638318 in_progress (KSG-Sweep; shift_sweep/FPR-Gate/block_sweep success @1337c6c1), te-gate 35139361939 pending (concurrency-blockiert, headSha 43af521f — nicht e28abbdf), gaia-cdn pending; format rot @ 35148755293 (14 fremde Dateien, post.md aktualisiert) | b85438ba (2026-09-16) | HEAD-Wechsel (nächster Atom-Push) | Check-Runs des gepushten SHA (GH-API) |
| TE-Gate n=1000 FPR-Boden | rot (alte Nulls): residual/ksg 19.51 % @ a=0.9 (Boden 8 %), phase/ksg 8.52 %, restricted/binned 10.71 %; block/binned rise 2.47pp, block/ksg 2.75pp (Grenze 2pp); shift_sweep grün (binned/ksg ≤ 6.87 %); Arx-Switch committet (`1337c6c1`), Verifikation läuft: te-gate run 35129638318 @ 1337c6c1 (in_progress) + 35129679496 (pending); konditionales FP/FN-Gate gebaut (`gate_conditional_arx_fpr_fn_n1000`, `gate_conditional_arx_2_fpr_n1000`, `conditional_arx_fit_resolves_at_small_n`); `arx_restricted_surrogate_conditional`/`_2` geben jetzt `Option` (None = Verweigerung, kein Shuffle); `TeNull::Residual` gestrichen (2026-09-16, pcmci `--null residual` = exit 1); konditionales FP/FN-Gate dispatcht; 35139346318 cancelled; 35139361939 (headSha 43af521f — nicht e28abbdf, Register-Korrektur) pending, concurrency-blockiert hinter 35129638318 @1337c6c1 (in_progress, gemessen 2026-09-16: shift_sweep + FPR-Gate + block_sweep success — Block-Null hält, KSG-Sweep läuft) | te-gate run 35129638318 @1337c6c1 (2026-09-16) | Run-Abschluss | `gh run view 35129638318` — grün: Gate zu; rot: Assert nennt die Zelle; Block-Null: hält eine Blocklänge rise ≤ 2pp, sonst `multi_force_te_probe.rs:74` auf Arx |
| dr3_stars.bin (CDN) | 75 001 828 B = 44-B-Stride (`rv`), sha256 fb9a14089ef8348e12961caf8253c8e886b4a16862a1a71394b96465d75bcfbb; sniff 2026-09-16: HTTP 200, present | 2026-09-16 (sniff) | HEAD-Wechsel / gaia-cdn Re-Dispatch | `gh workflow run gaia-cdn.yml` |
| voyager_odr Shards (CDN) | 14 Shards `voyager_odr_s0..s13.bin` present (run success; 452 MB–1.06 GB je Shard); in `phi/sources.φ` als 14 Blöcke registriert, die Einzeldatei `voyager_odr.bin` (Teilmenge, `C0XR13AA` im INDEX) ersetzt | run 35143340703 (2026-09-16, success) | HEAD-Wechsel / Re-Dispatch | `gh run view 35143340703` (idempotent — skip bei vorhandenen Shards) |
