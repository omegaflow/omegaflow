<!--
  title: Zustand — geteilter externer Zustand
  class: zustand
  date: 2026-09-17
  sha256: f7f95cee7fa4321c3d863d1252599dea78d66aff50a693e73f7119ea0a37a8ff
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
| Postfach (extern) | letzter Ledger-Eingang `1789670592` (2026-09-17, Rubin-Forum-Migrations-Thread — Publika-Kommentar, **kein Agenten-Eingang**); davor `1789662457` (GitHub-Support #4761801 GC-Thread-Update, kein Agenten-Reply), `1789650050` (Keller/TRISP-MLZ: sendet die NSE I(q,t)-Rohdaten in einigen Tagen), `1789650823` (Rubin-Summary); die fünf Sonden-Anfragen 13:47–13:55Z gesendet, **keine Antwort**; offen: GitHub-GC #4761801, Privacy-Antwort, Rubin-Review, NSE/Haug (Daten zugesagt), die fünf Sonden-Antworten | 2026-09-17 (Session-Read, Entscheid-Folge 40 — unverändert seit Folge 39) | neuer Ledger-Eingang oder 2⁶ min | `state/mail/mail_ledger.φ` (`smail` + `cloudflared`) |
| GitHub PII-Exposition | 45 (Datei, Ref)-Kombinationen aus zehn PII-tragenden Dateien, 15/15 Pre-Rewrite-Commits erreichbar; PII retrievable: yes; GC offen (Ticket #4761801) | pii-exposure run 35273689697 @ ab45b3f3 (completed 2026-09-17 20:59Z, exit 2 = Exposition bleibt: `commits reachable 15/15`, `PII retrievable: yes`, `exposed combinations 45` — identisch zu 45 @ bc9d6a0b / 35101796537 @ 8d0c9090) | HEAD-Wechsel oder GitHub-GC-Antwort | `gh run view 35273689697 --log-failed` (Wert 45 @ab45b3f3; exit 2 ist das Gate „Exposition bleibt", kein Messdefekt) |
| CI-Status | HEAD 964d18b9 — gemessen 2026-09-17 (`ci_manage list` + Watchdog-Snapshot 22:26); `origin/main` inzwischen `5a670e09` (cdn-reconcile-Bot, 21:00Z — eigene Check-Runde steht aus): `ci-check` 35273949693 pending @964d18b9; CDN-/Harvest-Runde @ab45b3f3 abgeschlossen (trmm-lis 35270822493, lis-otd 35270818836, kcdc 35270814640, iss-lis 35270812515, ionex 35270810486, igets 35270808236 success; auto-dispatch 35273075509 success; cdn-reconcile 35273461901 success); failure: pii-exposure 35273689697 (exit 2 = Exposition bleibt, kein CI-Defekt), harvest-dispatch 35273075476, harvest 35270996845; in_progress/queued: gaia-sso-cdn 35272160298, goes-cdn 35274196575, harvest 35270867738, pioneer-cell-census 35266366575, health-check 35245084696 | 964d18b9 (2026-09-17) | HEAD-Wechsel (nächster Atom-Push) | Check-Runs des gepushten SHA (GH-API) |
| TE-Gate n=1000 FPR-Boden | rot (alte Nulls): residual/ksg 19.51 % @ a=0.9 (Boden 8 %), phase/ksg 8.52 %, restricted/binned 10.71 %; block/binned rise 2.47pp, block/ksg 2.75pp (Grenze 2pp); shift_sweep grün (binned/ksg ≤ 6.87 %); Arx-Switch committet (`1337c6c1`), Verifikation läuft: te-gate run 35129638318 @ 1337c6c1 (in_progress) + 35129679496 (pending); konditionales FP/FN-Gate gebaut (`gate_conditional_arx_fpr_fn_n1000`, `gate_conditional_arx_2_fpr_n1000`, `conditional_arx_fit_resolves_at_small_n`); `arx_restricted_surrogate_conditional`/`_2` geben jetzt `Option` (None = Verweigerung, kein Shuffle); `TeNull::Residual` gestrichen (2026-09-16, pcmci `--null residual` = exit 1); konditionales FP/FN-Gate dispatcht; 35139346318 cancelled; 35139361939 (headSha 43af521f — nicht e28abbdf, Register-Korrektur) pending, concurrency-blockiert hinter 35129638318 @1337c6c1 (in_progress, gemessen 2026-09-16: shift_sweep + FPR-Gate + block_sweep success — Block-Null hält, KSG-Sweep läuft) | te-gate run 35129638318 @1337c6c1 (2026-09-16) | Run-Abschluss | `gh run view 35129638318` — grün: Gate zu; rot: Assert nennt die Zelle; Block-Null: hält eine Blocklänge rise ≤ 2pp, sonst `multi_force_te_probe.rs:74` auf Arx |
| dr3_stars.bin (CDN) | 75 001 828 B = 44-B-Stride (`rv`), sha256 fb9a14089ef8348e12961caf8253c8e886b4a16862a1a71394b96465d75bcfbb; sniff 2026-09-16: HTTP 200, present | 2026-09-16 (sniff) | HEAD-Wechsel / gaia-cdn Re-Dispatch | `gh workflow run gaia-cdn.yml` |
| voyager_odr Shards (CDN) | 14 Shards `voyager_odr_s0..s13.bin` present (run success; 452 MB–1.06 GB je Shard); in `phi/sources.φ` als 14 Blöcke registriert, die Einzeldatei `voyager_odr.bin` (Teilmenge, `C0XR13AA` im INDEX) ersetzt | run 35143340703 (2026-09-16, success) | HEAD-Wechsel / Re-Dispatch | `gh run view 35143340703` (idempotent — skip bei vorhandenen Shards) |
