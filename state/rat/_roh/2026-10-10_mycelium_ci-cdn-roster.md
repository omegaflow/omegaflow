# Stimmen-Runde — Mycelium — GitHub/CI/CDN-Optimierung (2026-10-10)

Akt: Operator-Wort 2026-10-10 „archive search all und roster recherche … GitHub, CI und CDN
optimieren … free GPU … Kommunikation opencode↔GitHub/CI/CDN". Gruppe `mycelium-ui`.
Frage (eine, terse): *Welche 3–5 Maßnahmen priorisierst du für GitHub/CI/CDN – und welche
große Möglichkeit übersehen wir? Je Maßnahme Hebel, Kosten, Risiko.* Read-only; Antworten
sind Rohmaterial, das Verdikt trägt Session + Rat.

## Qwen3.7-Plus (chat.qwen.ai)
1. CDN-Migration → Cloudflare R2 via OIDC (Egress immer frei).
2. Distributed Rust Caching: **sccache via R2** über alle Shards (nextest partitioniert nur Testausführung, nicht Kompilierung — 80 % der 3,5 h ist Compile).
3. Workflow-Konsolidierung: `workflow_call` + **Dynamic Matrices** (JSON-Matrix statt 446 YAML) → −80 % Dateien.
4. ARM64-First (`ubuntu-24.04-arm`) für reine Build/Script-Lasten.
**Übersehen:** Dynamic Matrices + sccache; **AI/GPU im CI sind für dieses Setup Sackgassen** (CI muss deterministisch sein), DeepSeek-flash lokal für YAML-Refactoring einsetzen.

## DeepSeek (chat.deepseek.com, DeepThink)
**Wichtigste übersehene Chance:** nicht Runner-Geschwindigkeit, sondern **Cache-Wiederverwendung über PR-Branches**.
1. **Rust-Cache-Hit-Rate reparieren**: `Swatinem/rust-cache` **ohne `shared-key`** (bekannt kaputt — restoreKey/key in falscher Reihenfolge), breitere `prefix-key`; `save-if` nur auf `main`, PR-Branches restoren nur. Hebel: 4 Shards × 22 min ≈ 88 min/PR.
2. nextest-Partitionierung behalten, aber **Build zentralisieren** (ein `target/`-Cache für alle Shards).
3. OIDC → R2 statt Release-Assets.
4. GPU-Runs nur für GPU-Tests, nie für Builds.
5. 446 Workflows **selektiv** konsolidieren (Composite Actions für gemeinsames Setup), nicht pauschal.
**Übersehen:** deterministische Cache-Invalidierung + Build-Sharing — die Konsolidierung ist Voraussetzung, damit ein Cache-Fix überhaupt greift.

## GLM-5.3 / Z.ai (chat.z.ai) — Deep Think
1. CDN → R2 via OIDC.
2. Radikale Workflow-Konsolidierung (`workflow_call` + Composite, Matrix) → <10 zentrale Workflows.
3. Test-Suite: ARM-Matrix + `cargo-nextest --partition` + **Rust-Cache in R2** (GitHub-Cache thrash bei großen Matrix-Runs) → 3,5 h auf <20 min.
4. GPU-Runs isoliert per Modal/Beam (nur Heavy-Compute-Suite).
**Übersehen:** „Privacy-First AI im CI" — CI spinnt per OIDC eine **private, temporäre Modal-GPU** hoch, lädt Open-Weight-Modell (DeepSeek) + PR-Code, macht **Predictive Test Selection** + Root-Cause/Fix-Kommentar, zerstört den Container. Repodaten bleiben in der kontrollierten Pipeline.

## Gemini 3.1 Pro (aistudio.google.com) — High thinking
1. CDN → R2 via OIDC (GitHub Releases kein echtes CDN: Rate Limits, langsame globale Verteilung).
2. Workflow-Konsolidierung (Reusable Workflows + Composite; Matrix OS/Arch/Features).
3. Test-Suite rasieren: ARM-Matrix + nextest + R2-Remote-Cache.
4. GPU isolieren (Modal/Beam).
**Übersehen:** dieselbe Privacy-First-AI-in-CI-Idee wie GLM (private Modal-GPU-Instanz, Predictive Test Selection, Root-Cause-Fix).

## Claude Sonnet 5.5 High (claude.ai) — größter Risiko-Fokus
1. **Publish-Pfad absichern**: statt statischer R2-Keys ein **Cloudflare Worker**, der das GitHub-OIDC-Token prüft (repo, ref, environment) und per Binding in R2 schreibt (R2 hat **keine** native OIDC-Federation); hash-adressierte Objekte, Manifest als atomarer Umschaltpunkt, `attest-build-provenance`.
2. **Workflow-Härtung per Lint-Gate**: Actions per SHA pinnen, `permissions: {}` Default, kein `pull_request_target`+Fork-Checkout, **zizmor** in CI, Dependabot für Actions.
3. Suite: einmal bauen, dann shardieren (`nextest archive` + `--partition count:i/N`, N≈8–16).
4. Workflows tieren: PR-Tier (Path-Filter + `cancel-in-progress`), Nightly-Tier (volle Suite), Required-Check über Aggregator-Job.
5. GPU/AI niedrig priorisieren; nur ohne Repodaten.
**Übersehen:** **Test-Impact-Analyse** (Cargo-Dep-Graph → nur betroffene Crates testen) + **Remote-Cache auf R2** als eigentlicher Hebel gegen Concurrency-Cap und 3,5 h.

## Perplexity (perplexity.ai) — mit 55 Quellen
1. 3,5-h-Suite in Shards (`slice:m/n`), pro Shard ein Job (Wall-Clock ≈ /N; Kompilierung + Ausreißer begrenzen).
2. Builds zwischen Shards wiederverwenden (`nextest archive`) + `sccache`; **Cache-Hit-Rate messen**, nicht blind `target/` cachen.
3. 446 Workflows inventarisieren/konsolidieren.
4. „Ernten" **einmal bauen**, dann unveränderlich deployen (Artefakt mit Commit-SHA/Manifest, attestieren, via OIDC nach R2).
5. Workflow-Sicherheit als Release-Gate (minimales `GITHUB_TOKEN`, OIDC eng auf repo/branch/env, Actions per SHA pinnen).
**Übersehen:** die 3,5 h **seltener vollständig** laufen lassen — PR-Check nur betroffene Pakete/Dep-Graph, volle Suite für Merge/Nightly (konservativ, Fallback „alles testen").

## Nicht geantwortet (gemessen)
- **Mistral / Vibe** (chat.mistral.ai): „Denke nach" ohne Antwortkörper zum Lesezeitpunkt → `pending` (kein Re-Ask).
- **Duck.ai**: Frage nicht gesendet/kein Verlaufseintrag zum Lesezeitpunkt (Tageslimit-Kandidat) → `pending`.
- **Geteilte `open-weight-ui`-Seats**: Lock `state/ui-open-weight.lock` von **river** gehalten (2026-10-10) → nicht gefahren (Lock-Disziplin, zwei Linien nie gleichzeitig).

## Konvergenz (alle geantworteten Seats)
1. **Cloudflare R2 via OIDC** als CDN-Ziel — einhellig (auch Claude: Worker-vermittelt, da R2 kein natives OIDC).
2. **Workflow-Konsolidierung** (`workflow_call`/Composite/Dynamic Matrices) — einhellig, höchster struktureller Hebel.
3. **nextest-Sharding + Build-Reuse + Rust-Cache in R2/sccache** — einhellig; DeepSeek/Perplexity betonen Cache-Hit-Rate über bloßes Parallelisieren.
4. **GPU nur für GPU-Last** (Modal/Beam), nie für Builds.
5. **Übersehen:** Test-Impact-Analyse (Dep-Graph) + Remote-Cache; und (GLM/Gemini) private AI-in-CI über Modal-GPU — die Session hält dagegen: das berührt die no-leak-Grenze und ist als **Riss** zu tragen, nicht als Konsens.
