<!--
  title: Zustand — geteilter externer Zustand
  class: zustand
  date: 2026-09-17
  sha256: 2309c1dc7b5030d1efc629722f08426dc776eb6a3ee35dd0f4d2e84dfb546a90
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
| Postfach (extern) | letzter Ledger-Eingang `1789670592` (2026-09-17, Rubin-Forum-Migrations-Thread — Publika-Kommentar, **kein Agenten-Eingang**); davor `1789662457` (GitHub-Support #4761801 GC-Thread-Update, kein Agenten-Reply), `1789650050` (Keller/TRISP-MLZ: sendet die NSE I(q,t)-Rohdaten in einigen Tagen), `1789650823` (Rubin-Summary); die fünf Sonden-Anfragen 13:47–13:55Z gesendet, **keine Antwort**; offen: GitHub-GC #4761801, Privacy-Antwort, Rubin-Review, NSE/Haug (Daten zugesagt), die fünf Sonden-Antworten | 2026-09-18 (Session-Read, Entscheid-Folge 41 — unverändert seit Folge 40) | neuer Ledger-Eingang oder 2⁶ min | `state/mail/mail_ledger.φ` (`smail` + `cloudflared`) |
| GitHub PII-Exposition | 45 (Datei, Ref)-Kombinationen aus zehn PII-tragenden Dateien, 15/15 Pre-Rewrite-Commits erreichbar; PII retrievable: yes; GC offen (Ticket #4761801) | pii-exposure run 35273689697 @ ab45b3f3 (completed 2026-09-17 20:59Z, exit 2 = Exposition bleibt: `commits reachable 15/15`, `PII retrievable: yes`, `exposed combinations 45` — identisch zu 45 @ bc9d6a0b / 35101796537 @ 8d0c9090); Neumessung @da2cf8f9 dispatched: pii-exposure 35287195140 (2026-09-18, pending) | HEAD-Wechsel oder GitHub-GC-Antwort | `ci_manage view 35287195140` (Wert @da2cf8f9; exit 2 ist das Gate „Exposition bleibt", kein Messdefekt) |
| CI-Status | HEAD `da2cf8f9` (== `origin/main`) — gemessen 2026-09-18 (`ci_manage list` + Watchdog-Snapshot 00:34): `ci-check` `35284285968` pending @da2cf8f9, `ci-check` `35278522861` in_progress @ff212c28; `harvest` `35278345279` in_progress; `allwise-cdn` `35281913809` in_progress; `health-check` `35286550387` pending; success: `quake-feeds-cdn` `35284468943`, `swpc-mirror-cdn` `35283920499`, `harvest` `35278476391`; failure (fremde Linien): `harvest` `35278536469`, `pii-exposure` `35273689697` exit 2; cancelled: `harvest` `35279473017` | da2cf8f9 (2026-09-18) | HEAD-Wechsel (nächster Atom-Push) | Check-Runs des gepushten SHA (GH-Check-Runs / `ci_manage view 35284285968`) |
| TE-Gate n=1000 FPR-Boden | rot (alte Nulls): residual/ksg 19.51 % @ a=0.9 (Boden 8 %), phase/ksg 8.52 %, restricted/binned 10.71 %; block/binned rise 2.47pp, block/ksg 2.75pp (Grenze 2pp); shift_sweep grün (binned/ksg ≤ 6.87 %); Arx-Switch committet (`1337c6c1`), Verifikation abgeschlossen (2026-09-18): te-gate 35129638318 @ 1337c6c1 **success**, 35129679496 cancelled, 35139361939 @ 43af521f **failure**; konditionales FP/FN-Gate gebaut (`gate_conditional_arx_fpr_fn_n1000`, `gate_conditional_arx_2_fpr_n1000`, `conditional_arx_fit_resolves_at_small_n`); `arx_restricted_surrogate_conditional`/`_2` geben jetzt `Option` (None = Verweigerung, kein Shuffle); `TeNull::Residual` gestrichen (2026-09-16, pcmci `--null residual` = exit 1); konditionales FP/FN-Gate dispatcht; 35139346318 cancelled; 35139361939 (headSha 43af521f — nicht e28abbdf, Register-Korrektur) **failure** (konditionales FP/FN-n=1000-Gate rot; der Lauf mit shift_sweep + FPR-Gate + block_sweep war 35129638318 @1337c6c1, success — Block-Null hält) | te-gate 35129638318 success + 35139361939 failure @43af521f (2026-09-18) | Run-Abschluss (neuer Dispatch) | `ci_manage log 35139361939` — der Assert nennt die rote Zelle; dann fixen und `gh workflow run te-gate.yml` |
| dr3_stars.bin (CDN) | 75 001 828 B = 44-B-Stride (`rv`), sha256 fb9a14089ef8348e12961caf8253c8e886b4a16862a1a71394b96465d75bcfbb; sniff 2026-09-16: HTTP 200, present | 2026-09-16 (sniff) | HEAD-Wechsel / gaia-cdn Re-Dispatch | `gh workflow run gaia-cdn.yml` |
| voyager_odr Shards (CDN) | 14 Shards `voyager_odr_s0..s13.bin` present (run success; 452 MB–1.06 GB je Shard); in `phi/sources.φ` als 14 Blöcke registriert, die Einzeldatei `voyager_odr.bin` (Teilmenge, `C0XR13AA` im INDEX) ersetzt | run 35143340703 (2026-09-16, success) | HEAD-Wechsel / Re-Dispatch | `gh run view 35143340703` (idempotent — skip bei vorhandenen Shards) |
