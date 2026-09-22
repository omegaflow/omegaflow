<!--
  title: Handover — Entscheid-Folge 62 (Free-Model-Benchmark) (Stand 2026-09-20)
  session: Entscheid-Folge 62
  class: handover
  date: 2026-09-20
  sha256: d37639335a5b9665cdbbd5c2f37f567e3eeb397bf6e0d6af9e6dfa27ab17ee09
  status: live
-->
# Handover — Entscheid-Folge 62 (2026-09-20)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird nur der eigene Teil,
fremde uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist (Fast-Forward).

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen nächsten
Schritt in derselben Zeile. Wartestellungen (`wartend`) sind kein Auswahlpunkt,
sondern nennen nur ihren Auslöser. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

## Stehender Pass (gemessen 2026-09-20, Entscheid-Folge 62)

- **HEAD** `aa120ac6` (== `origin/main`, „ernte: complete handover 107 archive move").
  Start war `e3c478af`; die Research-Linie committete während der Session (`0e037e1e`,
  Kernel-Riss aufgelöst). Der Arbeitsbaum war beim Start sauber, `git_safety --snapshot`
  meldete „tree equals HEAD — nothing to record".
- **CI** — Watchdog-Snapshot 2026-09-20T16:03: aktiv `harvest` `35515194248`, `te-gate`
  `35513982359`, `ci-check` `35513190719`, `health-check` `35505471538`; rot
  `release-build` `35513611936` + die ci-check-Kette. Kein Rot außerhalb ci-check.
- **Postfach** — `state/mail/mail_ledger.φ` (243 Zeilen): neuester Eingang `1789906306`
  (Sotgiu-Weiterleitung); kein neuer Eingang seit folge61.

## Free-Model-Benchmark (2026-09-20) — der harte undatierte Punkt dieser Session

Gemessen und abgeschlossen; die Ergebnis-Zeile für die Zukunft:

- **Models-Datei** `$HOME/.cache/opencode/models.json` (4,7 MB, models.dev,
  ~135 Provider). **62 Free-Modelle** (cost 0 oder `:free`): openrouter 21, kilo 19,
  unorouter 10, kenari 10, bothub 2. Registrierungsschlüssel: `OPENROUTER_API_KEY`,
  `KILO_API_KEY`, `UNOROUTER_API_KEY`, `KENARI_API_KEY`, `BOTHUB_API_KEY`.
- **Auth vorhanden** (`~/.local/share/opencode/auth.json`): `kilo`, `openrouter`,
  `tokenrouter`, `zai` (+ `deepseek`, `nvidia`). Die globale Whitelist (untracked
  `~/.config/opencode/opencode.jsonc`) wurde auf alle Free-Modelle der vier Provider
  erweitert.
- **Task A** (Routine-Tool-Use, read-only): Zeilennummern von `default_kernel_for` /
  `kernel_id_for_force` in `src/mathematikerin/force.rs`. Ground Truth am HEAD
  `0e037e1e`: **53 / 33**. 23 Modelle, ein Lauf je Modell (`opencode run --format json`,
  read-only Agent).
  - **13 korrekt mit Tool-Use, $0**: `cohere/north-mini-code`, `dots-studio/dots-3-note`,
    `inclusionai/ling-3.0-flash-fin`, `-sante`, `-vl`, `liquid/lfm-2.5-2.6b`,
    `nex-agi/nex-n2.5-mini`, `-pro`, `nvidia/nemotron-3-super-120b`, `-ultra-550b`,
    `qwen/qwen3.8-27b`, `thinkingmachines/inkling`, `-small`.
  - **5 unvollständig**: `poolside/laguna-s-2.1` (kein Zahlenwert), `laguna-xs-2.1`
    (keine Antwort), `nvidia/nemotron-3-nano-omni` (`</tool_call>`),
    `nvidia/nemotron-3.5-lightning` (keine Antwort), `zai/glm-4.7-flash` (keine Antwort).
  - **5 nicht nutzbar**: `google/gemma-4-26b`/`-31b` (429, Upstream-Pool),
    `nvidia/nemotron-3.5-content-safety` + `z-ai/glm-5.2` („No endpoints found that
    support tool use"), `tokenrouter/z-ai/glm-5.3-free` („no available channel").
  - **Referenz** `deepseek/deepseek-v4-flash`: korrekt, ~$0.0003.
- **Ergebnis**: Free-Modelle lösen Routine-Tool-Use korrekt zu $0; der harte Filter ist
  Tool-Use-Support, der reale Preis sind 429-Rate-Limits. Die Regel „flash first"
  (Zuverlässigkeit) steht; Free ist die Null-Kosten-Option für read-only Bulk-Arbeit.

## Free-Tier-Endpunkt-Audit (2026-09-20)

Alle Provider-`/models`-Endpunkte live gemessen (Bearer-Key aus `auth.json`):
openrouter 446 Modelle (22 free), kilo 380 (21 free), unorouter 261 (138 free), kenari 80
(15 free), tokenrouter 140 (1 free), zai 11, nvidia 82, mistral 46, inception 2.

- **Ursache vieler Abweichungen:** `~/.cache/opencode/models.json` (models.dev-Snapshot) war
  veraltet — NVIDIA-IDs teils EOL (`deepseek-ai/deepseek-v4-flash`, 410 Gone), unorouter/
  kenari-Kataloge viel größer als dort gelistet. opencode kennt nur die models.json-Modelle;
  live-only-IDs liefern `Unexpected server error`.
- **Auffrischung (2026-09-20 18:03, 4,71 MB):** Cache neu geladen (Backup
  `models.json.pre-refresh-20260920`). models.dev hinkt aber weiterhin — unorouter 23 statt
  live 261, kenari 59 statt 80. Deshalb die lohnendsten **live-only-Gratismodelle explizit in
  der globalen Config definiert** (`models`-Block) und getestet: unorouter
  `gemini-3.6-flash:free`, `glm-5.3:free`, `gpt-oss-20b:free` (✅), kenari
  `agnes-3-0-flash:free`, `nex-n2-5-pro:free` (✅).
- **Korrigiert (globale Whitelist):** `nvidia` auf die 4 live-funktionierenden
  (`nvidia/nemotron-3-super-120b-a12b`, `-ultra-550b-a55b`, `nemotron-3.5-lightning-30b-a3b`,
  `openai/gpt-oss-20b`); `zai` auf die **Gratis**-Modelle laut `docs.z.ai/guides/overview/
  pricing` (`glm-4.7-flash`, `glm-4.5-flash`, `glm-4.6v-flash` — alle „Free"); `tokenrouter` auf das einzige live-free
  (`nvidia/nemotron-3-nano-omni-30b-a3b-reasoning:free`); `unorouter`/`kenari` auf die
  live-gültige Schnittmenge gepruned.
- **Messstand:** inception ✅ (`mercury-2`); nvidia ✅ (4 Modelle); unorouter ✅
  (`deepseek-v4-flash:free`), weitere live-free noch nicht nutzbar; kenari ✅
  (`glm-4-7-flash:free`); zai ✅ (`glm-4.7-flash`, `glm-4.5-flash`, `glm-4.6v-flash` — laut
  `docs.z.ai` gratis, antworten); mistral ⚠️ „Rate limit exceeded" (Free-Tier streng /
  Pay-as-you-go nicht aktiviert); tokenrouter ⚠️ live-free-ID opencode-unbekannt → nicht nutzbar.
- **Neu registriert:** `mistral` (`MISTRAL_API_KEY`), `inception` (`INCEPTION_API_KEY`, Free
  Tier 100 Mio Token) — beide in `auth.json` + globaler Config. **ISH entfällt** (API nur mit
  bezahltem Guthaben, gemessen in `api.ish.chat/docs/opencode`).

## Offen

- **Weitere live-only-free-Modelle definieren** — models.dev hinkt (unorouter live 138 free,
  kenari 15); die lohnendsten sind explizit definiert, der Rest auf Bedarf. **`wartend`**.
- **Benchmark-Klassen B–D** (Quellen-Recherche, Register-Digest, Force-Gate-Diagnose)
  nicht gelaufen. **`wartend`** (Auslöser: Bedarf).
- **Free-Model-Benchmark auf UnoRouter/Kenari ausweiten** (je 10 Free-Modelle; die
  Schlüssel stehen). **`wartend`** (Auslöser: Bedarf). Bothub entfällt (Löschung).
- **Kernel-Riss** — von der Research-Linie aufgelöst (`0e037e1e`). **`wartend`**.
- **F2 — flare-Gate-Power** — print-only Probe n∈{400,600,1000}, dann n setzen oder
  Driver-Design ändern. An forschung gepostet. **`wartend`**.
- **09-16-Limbo** — forschung-Pass. **`wartend`**.
- `termin` — **vC-Permeabilität** (Smartwatch + Mantis-Shrimp-Sensoren),
  **Lasair-LSST** (API 502, Backend server-seitig), **BepiColombo MORE** (~April 2027).
- `blockiert` — TAP-Backends dachs/pithia (extern). `wartend` — adoption-Block,
  SuperDARN-Globus, GitHub-PII/GC (#4761801), Rubin-Review, Sonden-Antworten,
  `register_lookup`-Binary + `ci-check` (an bau gepostet).

## Operator-Queue (einmal vorlegen beim Operator-Rückkehr; einfache Sprache)

1. **PII-History-Rewrite** — *Lage:* private Adresse in 1.472 Commits, HEAD ist
   redigiert, die Historie nicht. *Frage:* Rewrite ausführen? *Ja:* der Auftrag
   `docs/auftrag/auftrag-pii-history-rewrite.md` läuft (destruktiv, Force-Push);
   *Nein:* bleibt offen.
4. **Mistral** — *Lage:* **Bothub** ist gelöscht und der **UnoRouter-Schlüssel auf
   antworten. **Mistral** meldet „Rate limit exceeded" (Free-Tier streng; Console zeigt
   „Pay-As-You-Go aktivieren"). *Frage:* Mistral Pay-as-you-go aktivieren — oder als
   nicht-nutzbar führen?

## Quer-Linien (Post)

- `An forschung` (Kernel-Riss) ist durch `0e037e1e` erledigt — keine offene Zeile.

## Geteilter Baum — eigener Pfad-Satz

- `docs/handover/handover-2026-09-20-entscheid-folge62.md` (neu)
- Move `handover-2026-09-20-entscheid-folge61.md` → `archiv/` (eigene Linie, atomar)

## Benchmark

- Task A: 23 Free-Modelle gegen `deepseek-v4-flash`; 13 korrekt zu $0, Referenz
  ~$0.0003. Kein pro/max-Doppellauf — die Klasse „Routine-Tool-Use" ist geschlossen,
  Free dient der Null-Kosten-Kalibrierung.
- Nachtrag: `unorouter/deepseek-v4-flash:free` und `kenari/glm-4-7-flash:free` antworten
  nach Key-Eintrag; `unorouter/gpt-5.5:free`, `kenari/deepseek-v4-flash:free` und
  `kenari/kimi-k2-7-code:free` lieferten keine Antwort (Rate-/Modell-Stand offen).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
