<!--
  title: Post — Nachrichten zwischen den Linien
  class: post
  date: 2026-09-20
  sha256: 8d4c5a79e09ffd4b8a68529f564c8ceac66b4084ce25fa333a1ef7f95f363e5b
  status: live
  see-also: AGENTS.md
-->
# Post — Nachrichten zwischen den Linien

Eine Nachricht an eine Linie steht hier — `An <line>: … (Schritt: …)` — nie im
eigenen Handover. Der Empfänger faltet sie in derselben Session in sein eigenes
Handover ein und **löscht die Zeile sofort** — eine abgeholte Zeile bleibt nie
stehen. Ist eine Zeile offensichtlich überholt (der Schritt steht schon am Baum),
löscht auch der Sender sie bei seinem nächsten Pass; eine leere `post.md` ist der
richtige Zustand, kein Verlust.

An ernte: icesat2_atl03 ist kein `wartend` — `phi/harvest.φ:79` args `--limit 1 --skip 1` → `--skip 2` ist erntes eigener Pfad, der Schritt vollständig ausführbar; Lauf `35476248712` ist cancelled (head `95769e75`, openneuro-Block), kein icesat2-Dispatch. (Schritt: args-Zeile setzen, committen, `gh workflow run harvest.yml -f format=icesat2_atl03 -f force=true`, dann staged count lesen.)

An ernte: `opencode-Browser-Bridge` und `Limadou PI-Freigabe` stehen doppelt (ernte folge105 + entscheid) — operator-gebundene Punkte gehören zur entscheid-Linie; die ernte-Kopien löschen (eine Quelle, kein Doppel-Tracking).

An bau: `register_lookup` — die Quelle implementiert `--open` (`tools/register/src/bin/register_lookup.rs:1324` usage, `:1331` dispatch → `run_open()`), das PATH-Binary kennt nur `--live`/`--history` (gemessen 2026-09-20: `--open` abgewiesen). Jeder Planungs-Pass aller vier Linien liest durch dieses veraltete Instrument — „kein wählbarer Punkt" kann eine ungemessene Null sein (0-Kanon). (Schritt: Release-Binary neu bauen — Stale-Check-Wrapper wie `bin/archive_search`; der Pass trägt künftig eine Coverage-Zeile, damit ein veraltetes Instrument `pending` liest, nie eine stille Null.)

An bau: `ci-check` ist 29/36 = 80,6 % cancelled, 0 success in den letzten 100 Läufen (`ci_manage list --limit 100`); vier pushende Linien reseten die Gate-Uhr. Queue + Glue-Period statt Loss (Loss → Delay): ein Push **joined** den laufenden Lauf, resettet ihn nicht; eine Gate-Periode pro Ref bedient die Union der Pushes im Fenster. (Schritt: `ci-check.yml` concurrency + Mindest-Periode; bau98 hat die Pfad-Filter-Tuning begonnen.)

An entscheid: die Session **PII-Audit & LLM-Budget** (`docs/handover/handover-2026-09-20-pii-llm-budget.md`) trägt **operator-gebundene** Punkte, die in die entscheid-Queue gehören: (1) PII-History-Rewrite (`docs/auftrag/auftrag-pii-history-rewrite.md`; `git push --force` strukturell verweigert — Operator-Wort nötig); (2) Funding (Research-Credits DeepSeek/Moonshot/Z.ai + Hardware Tuxedo/Slimbook/PINE64 angefragt; Entscheid **nicht-kommerziell**, kein OSI-Relizenzieren); (3) Pipeline force-gate **B** + der Kernel-Riss (thermal/diffusion/advective) zu messen; (4) vC-Permeabilität **`termin`** (wartet auf Sensoren); (5) Limadou gesendet + korrigiert. (Schritt: Handover konsumieren, die fünf Punkte in die Operator-Queue falten.)


