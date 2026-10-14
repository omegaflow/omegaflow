<!--
  title: Agent Permission Profiles — role = profile
  class: concept
  date: 2026-10-14
  sha256: 8149d73c83f338f8ce49411600c532d0685053ede059d560dcba9e2124d7aa42
  status: live
  see-also: AGENTS.md
-->
## Agent permission profiles — role = profile

Every agent maps to one profile; in a bash map the catch-all
`"*"` stands **first**, the specific rules after it (opencode evaluates by
pattern, **last matching rule wins**; a `"deny"` string on a tool key removes the
tool):

- **P1 primary** (`line`) — edit + full bash (global). Läuft auf `deepseek/deepseek-flash`.
- **P2 research** (`taucher`) — edit + full bash (global). Läuft auf `deepseek/deepseek-flash`.
- **P3 council** (`rat`) — no edit; bash = `register_lookup` + `git_safety`
  + `open_points_check` + git-read + `merge-base` + `sgrep` +
  `ci_manage status`/`list`/`view`/`jobs`/`log` + `du`/`find`/`awk`.
  Läuft auf `deepseek/deepseek-flash`. Der Rat (0,9) — die fünf Stimmen — hält
  Architektur-/Ethik-Fragen.
- **P4 future/PII** (`future`) — edit + full bash (global). Läuft auf `deepseek/deepseek-flash`.
  Nur für Future-Linie und PII-geschützte Arbeit (Mail, Secrets, Operator-Queue, Förderung).
  Keine öffentliche Arbeit — kein Bau, kein Port, kein öffentlicher Code.

The global Rust-tool table (`fd`/`bat`/`eza`/`sd`/`ouch`/`jaq`/`dust`/`difft`/
`btm`/`procs`) does not reach the read-only profiles — read-only bash is exactly
the named list above; a needed tool is added to the one map that needs it, at the
measurement.

Structurally denied in every profile (leading form): `grep`, `ls`, `cat`, `rg`,
`cd`, `python`, `python3`. Content search = `archive_search <kws> --root <dir>`
or `sgrep`; discovery = `glob`; reading = `sread` or the `read` tool. `cd` is
replaced by the bash tool's `workdir` parameter. A compound command
(`cmd; echo; cmd`) is evaluated **per segment** — every segment must match an
allow pattern, so chain only allowed commands with `&&`, or call them one by one
(a stray `echo` in the chain denies the whole command). Allowed in every bash
profile: bare `archive_search`, `sread`, and `time archive_search|sgrep|sfetch|
sread`. Read your exact allow-list before the first call — do not try a command
that is not on it; a denied call wastes a turn and the session's bash quota.
Run the canonical tools through their `bin/` wrappers — the wrapper is the
freshness contract (`bin/.tools_ensure`); `./target/release/*` serves the freshly
built own bin and the named pregates only. `gh run list`/`gh run view` are denied:
the CI reading is `ci_manage`. File edits run through the edit/write tools, never
`sed -i` — structurally denied in `opencode.json` (the lesson of 2026-09-27: two
sha-header updates went through `sed -i`; the rule is anchored in the config, not
in practice).

No agent has `webfetch`/`websearch` (global deny) — web runs through
`archive_search`. Every subagent has `task: deny` (no sub-subagents); only
`line` spawns. Never answer "always" to a bash `ask` outside the written maps —
`approved` is instance-shared and evaluates last, so it would cross profiles.

### The cost ladder — the cheapest agent that fits the job

Model tiers (cost/speed): `flash` (low) < `pro` (high) < `pro/max`. Dispatch the
cheapest profile whose tools and role fit; a `max` agent is for the hard atoms
only, never for routine work:

| Job | Agent | Provider |
|---|---|---|
| öffentliche Arbeit (Bau, Port, Code) | `line` · `taucher` | `deepseek/deepseek-flash` |
| Architektur / Ethik (Rat, fünf Stimmen) | `rat` | `deepseek/deepseek-flash` |
| Future-Linie + PII (Mail, Secrets, Förderung) | `future` | `deepseek/deepseek-flash` |

Measured (2026-09-15, `session_burn`/opencode.db): flash dispatches cost
~$0.001–0.011, pro/max ~$0.007–0.126 — 5–40× more. Head-to-head on a real repo
task (diagnose the red `number_audit` test): taucher $0.0024 / 7.8 s vs
elitetaucher $0.0104 / 19.7 s — **identical diagnosis**. Rule: **flash first** —
dispatch the cheapest profile, and escalate to pro/max only when flash returned
a wrong or incomplete answer, never by default.

### Benchmarking — every active task is a benchmark

The benchmark doubles a task only when the class has no recorded winner in the
register, or the flash answer was measured wrong or incomplete. A recorded
winner closes the class: the session cites it, never re-runs it; a
wrong/incomplete flash answer reopens it and the re-run records the reason. The
hard atoms (novel parser, TE-/null construction, multi-stage register routes, 4D
contract) are the benchmark classes; routine work runs flash-first — pro/max is
the escalation, not the mirror. A doubled run records the winner
(correctness/completeness) and the burn (`session_burn` / opencode.db: `cost`,
`tokens_input`, `tokens_cache_read`, duration) as a handover line. The measured
PII-exposure class is closed (flash 5.3x cheaper, identical result, 2026-09-16).
The measured routine-agent class is closed too (8 profiles, identical
four-command search task, 2026-09-16): flash $0.0008–0.0017 against pro/max
$0.0041–0.0090 — 2.4–11x for an identical result; the winner is `taucher`
($0.0008). Routine search/inspection therefore dispatches a flash profile;
pro/max stays for the named hard atoms only.

**Modell-Politik (Operator-Wort 2026-10-14).** Ein Anbieter: **DeepSeek (original) `deepseek/deepseek-flash`** — öffentliche Arbeit (Bau, Port, Code, Recherche, Architektur/Ethik) **und** Future-Linie/PII (Mail, Secrets, Operator-Queue, Förderung) — `line` · `taucher` · `rat` · `future`. Günstiger Direkt-API-Pfad, kein Reseller. Groq (Rate-Limit, englisch) und DeepInfra (unglaublich langsam, Cache teuer — GLM-5.3 ca. 2,6 ct/Nachricht gegen DeepSeek 0,16 ct) wurden verworfen.

Kein pro/max, kein Gremium, kein Elitetaucher. Thinking aus (flash/low). Der Burn wird gemessen (`session_burn`), nie geschätzt.

**UI-Chats (zweiter Kanal, unverändert).** Die Frontier-Chats bleiben als Denk-/Urteils-Kanal: Duck.ai (primärer OpenAI-Seat) · Claude · Qwen · GLM · Kimi · MiMo V2.6 Pro · Nemotron 3 Ultra · MiniMax M3 · Gemini 3.1 Pro · DeepSeek Chat · Mistral · Proton Lumo. Die tryingopen-Seats in der Gruppe `open-weight-ui` (Lock). Jede Linie fährt ihre UI-Stimmen in der eigenen Gruppe `<line>-ui`; Tabs JIT öffnen/schließen.

**Ein Dispatch = ein begrenzter Schritt (Operator-Wort, 2026-10-06).** A sub-agent plans inside its output-token budget — reasoning and output share one ceiling (~32 000 tokens across the DeepSeek tiers). An order large enough to need a multi-hundred-line plan is spent entirely on `reasoning` and the step is cut (`step-finish reason:"length"`, reasoning 32 000, output 0, no file touched): measured 2026-10-06, one whole-feature order to `taucher` burned ~$0.12 each and wrote nothing, while the same feature split into small steps ran at reasoning 1 608–2 390, output ~2 700, ~$0.006 per step, code written and `cargo check`-green. A feature is therefore built as a **sequence of bounded dispatches**, never one whole-feature order: the line session owns the sequence and hands each diver exactly one bounded step (one function, one flag, one file-section) with its `cargo check` gate, reading the result before the next. A diver that receives an over-large order stops and names it (`the order exceeds one bounded step`), instead of planning it out and starving the output.



