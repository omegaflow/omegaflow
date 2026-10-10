<!--
  title: Survey — GitHub-, CI- und CDN-Optimierung (free GPU, AI-im-CI, opencode↔GitHub)
  class: survey
  date: 2026-10-10
  sha256: 3ba22360800992bfb6c938cd80509b0ccfb2c991d45e8287ba9de14efbd662f8
  status: live
-->
# Survey — GitHub-, CI- und CDN-Optimierung (2026-10-10)

Auftrag (Operator-Wort 2026-10-10): die GitHub-, CI- und CDN-Abläufe optimieren, „groß
denken" (mehr der Möglichkeiten, z. B. von AI, nutzen), kostenlose externe Optionen
(GPU-Runs) analysieren und die Kommunikation zwischen opencode und GitHub/CI/CDN klären.
Methode (Operator-Wort: Forschungsschicht zuerst): `archive_search --all` + vier
gemessene Recherchen, dann **Rat** (fünf Stimmen) als Struktur, dann die **UI-Runde**
(6 geantwortete Seats, 2 `pending`, geteilte Seats gelockt) als Urteil. Alle Zahlen
sind gemessen mit Quelle; `unverified` ist benannt.

## Ist-Zustand (gemessen am Baum)

- Öffentliches Repo, **446** GitHub-Actions-Workflows (`.github/workflows/`), Test-Suite ~3,5 h.
- Pipeline-Doku `docs/concepts/github-pipeline.md` (2026-09-28): Korpora als Release-Assets
  (`omegaflow/sources`, 1000 Assets/<2 GiB, „kein Bandbreitenlimit"); Compiler fetchen →
  kompilieren → CDN via `--ci-mode` (**nur CI schreibt**); `kernel-flatten.yml` + die `*-cdn.yml`.
- Lokal laufen nur **DeepSeek-flash**-Agenten (pro/max strikt deaktiviert); UI-Seats als zweiter
  Kanal. GitHub Models **tot** (2026-07-30); Copilot **gestrichen** (Operator-Wort 2026-09-28:
  kein Cloud-Dienst erhält omegaflow-Daten). `state/` verlässt die Maschine nie.
- Lokale, deterministische Checks: `bin/session_check --in|--out`, `bin/ci_triage` (public
  CI-Log → zweite Stimme, leerer Temp), `bin/ci_manage`, `bin/ci_watchdog.sh`, `bin/matrix_watchdog.sh`.

## Der Rat (Struktur) — fünf Stimmen

- **Mountain:** 446 Einzeldateien sind Register-Drift in YAML. Trägt: *ein Compiler, N Aufrufer*
  (`workflow_call`, `compile-<source>`). Verwirft: kostenpflichtige larger/GPU-Runner als Default.
- **River:** Lauf-Hygiene — `concurrency`+`cancel-in-progress`, `paths`-Filter, `timeout-minutes`,
  Retention 90 d. Verwirft: self-hosted Runner auf public (`pull_request_target`-RCE).
- **Mycelium:** ein CDN (R2, Egress $0), ein Schreibpfad (`--ci-mode`), OIDC als einziger
  secretloser Netz-Pfad, Stehender Pass/Watchdog als **einziger Poller**. Verwirft: jsDelivr/
  Statically (Terms), Git-LFS (metered), Cloud-AI über Repo-Daten.
- **Sensory:** freie GPU als **gemessene Probe** (Capability-Gate ≥4/4 + Tempo), nicht als
  Roster-Platz auf Verdacht. Verwirft: Kaggle als CI-Kanal (kein Hook), Azure AI Foundry (Datenabfluss).
- **Future:** Vorbereitung autonom, **Akt an Dritten = Operator-Hand** (Bucket, Account, Paid-Tier).

**Riss (benannt, nicht geglättet):** Mountain will Paid-GPU strukturell ablehnen; Sensory hält
die freie GPU für ein reales Messorgan. Auflösung **nicht** per Mittelwert, sondern per
**Reihenfolge**: GPU/AI steht hinten, hinter Capability-Gate und Operator-Wort — bis dahin
`pending`, nicht `0`, nicht „beschafft".

**Counter-Slope:** „mehr AI" → keine Cloud-AI über Repo-Daten, self-hosted LLM im Runner +
lokale deterministische Checks · „mehr Cloud-Komfort" → ein CDN, ein Schreibpfad, free-first ·
„mehr Runner" → nicht self-hosted auf public, statt dessen `workflow_call` + Partition + ARM ·
„groß denken" → **weniger Artefakte, mehr Eigenschaften** (446 Dateien → N Aufrufer), nicht mehr Vendoren.

## Säule A — Runner & Tests (gemessen)

- Standard-Runner (x64 4 vCPU/16 GB) und **`ubuntu-24.04-arm` (4 vCPU arm64) sind gratis +
  unbegrenzt für public**; **larger/GPU-Runner sind kostenpflichtig auch für public** (Linux
  4-core $0.012/min, GPU $0.052/min, Team/Enterprise nötig). docs.github.com/en/billing/reference/actions-runner-pricing
- `cargo-nextest --archive-file` + `--partition slice:i/N`/`hash:m/n` über Matrix (Matrix-Cap
  256 Jobs); `Swatinem/rust-cache` (**ohne `shared-key`** — bekannt defekt, `save-if` nur `main`,
  PR-Branches read-only); `sccache` für Objekt-Reuse. nextest docs, docs.github.com/en/actions/reference/limits
- `actions/cache` v4: 10 GB/Repo, 7-Tage-TTL, LRU-Eviction; Cache-Writes auf public-Fork default read-only.
- Reusable `workflow_call` statt Duplikat-YAML; Composite nur für Step-Gruppen (kein `secrets`-Kontext);
  Workflow-Datei ≤500 KB.
- Retention: public max 90 d, ab 2026-10-01 auch für Checks/Runs/Statuses.
- **Höchster Hebel laut Runde:** nicht schneller kompilieren, sondern **seltener vollständig laufen** —
  Test-Impact-Analyse aus dem Cargo-Dep-Graph (nur betroffene Crates; Nightly bleibt Voll-Lauf).
- **Build-Reuse + Remote-Cache:** Cache/Artefakt in R2 (egress-frei) statt GitHub-Cache-Thrash.

## Säule B — CDN (gemessen)

- **Cloudflare R2** (Empfehlung einhellig): Egress **$0** (alle Klassen), 10 GB-mo frei, dann
  $0.015/GB-mo, Objekt ≤5 TiB, Range + immutable Headers; S3-API-Upload hinter `--ci-mode`.
  developers.cloudflare.com/r2/pricing · /platform/limits
- **GitHub Release-Assets** als <2-GiB-Spiegel bestätigt: ≤1000 Assets/Release, je <2 GiB, kein
  dokumentiertes Bandbreitenlimit. docs.github.com
- **DOI-Kopie:** Zenodo (50 GB/Record, ≤100 Dateien), figshare (20 GB), OSF (public 50 GB).
- Alternativen: Backblaze B2 (+Cloudflare Bandwidth Alliance, $0 Egress), Bunny Storage.
- **Verworfen (gemessen):** jsDelivr/Statically (Terms verbieten Bulk/Backup-Hosting; 20 MB-Cap),
  Git LFS (10 GB metered), `upload-artifact` (≤90 d).
- **Optimierung:** content-hash-Dateinamen (`<sha256>.bin`), immutable `Cache-Control`, kleines
  mutables `manifest.json` als atomarer Umschaltpunkt (Rollback), HTTP-Range.
- **1,76 TB — die Speicherfrage (gemessen 2026-10-10):** `omegaflow/sources` trägt **344 Releases /
  ~1 763 GB** (GitHub-API-Summe der Asset-Größen; `per_page=100` × 4 Seiten). Release-Assets haben
  **kein Gesamtlimit und kein Bandbreitenlimit** (nur ≤1000 Assets/Release, je <2 GiB). Der **Bulk
  bleibt dort ($0)** — das ist der deklarierte CDN-Pfad, NICHT R2. R2s **10 GB frei** ist ein
  **Hot-Tier** für kleine Manifeste/Indizes, nicht für 1,76 TB. Wer einen echten S3-Store aller
  1,76 TB will: **R2 Standard ≈ $26/mo** (1 741 GB × $0.015), **R2 IA ≈ $17/mo**, **Backblaze B2
  ≈ $12/mo** (1,7 TB × $6.95/TB, Egress via Cloudflare frei). Zenodo/figshare nur für kuratierte
  DOI-Releases (50/20 GB) — kein Bulk.

## Säule C — opencode ↔ GitHub / CI / CDN (gemessen)

- **Auth/Rate:** REST unauthenticated 60/h/IP · PAT 5 000/h · GitHub-App-Installationstoken
  ≥5 000/h (least privilege, ~1 h) · `GITHUB_TOKEN` im Job 1 000/h/repo. docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api
- **OIDC (`id-token: write`)** ist der **einzige secretlose Pfad**; Claims eng auf `repo`/`ref`/
  `environment` binden. R2 hat **keine** native OIDC-Federation → Claude-Vorschlag: kleiner
  **Cloudflare Worker** prüft das GitHub-OIDC-Token und schreibt per Binding in R2.
- **Trigger:** `workflow_dispatch` / `repository_dispatch` nach eigenem Push (Trigger nur, kein
  Secret-Expose). **Kein Webhook** (der Agent ist kein Host). **MCP-GitHub-Server read-only**;
  Filesystem/Fetch-MCP und self-hosted Runner auf public sind die gemessenen unsicheren Flächen.
- **Polling:** bleibt allein beim Watchdog/Stehenden Pass; Rate-Budget als `external-state`-Zeile.
- **Härtung:** Actions per SHA pinnen, `permissions: {}`, kein `pull_request_target`+Fork-Checkout,
  `zizmor`, Dependabot, `attest-build-provenance`.

## Säule D — Free GPU & AI-im-CI (gemessen)

- **Free GPU (echt, headless):** **Modal $30/mo** (Starter) und **Beam.cloud $30/mo** — die
  einzigen Free-Tiers mit echter GPU-Compute headless (wgpu/Vulkan oder CUDA); **Kaggle ~30 h/Woche**
  (Notebook, kein CI-Hook, floatend); HF ZeroGPU/Jobs (Demos/Jobs). **GitHub-GPU-Runner kostenpflichtig.**
  Colab kein headless-CI-Hook, keine publizierte Quote.
- **Free AI-Inferenz:** Groq (30 RPM, 1 000/Tag) · Cerebras (1 M tok/Tag) · Cloudflare Workers AI
  (10 000 Neurons/Tag) · OpenRouter-Free-Modelle. **GitHub Models tot** (2026-07-30).
- **No-leak-Filter:** die einzigen überlebenden Optionen sind **self-hosted LLM im Runner**
  (Ollama/vLLM — kein Datenabfluss) und **lokale deterministische Triage**. Azure AI Foundry = Datenabfluss.
- **Riss:** GLM + Gemini schlagen „Privacy-First AI-in-CI" vor (CI spinnt per OIDC eine private
  Modal-GPU, lädt Open-Weight-Modell + PR-Code, Predictive Test Selection + Root-Cause-Fix).
  Der Rat hält dagegen: das berührt die no-leak-Grenze (Code/Logs an einen Dritten). Als **Riss**
  getragen; Realisierung nur als **self-hosted LLM auf eigenem/inspected Runner**, nie über eine
  fremde Cloud-GPU mit Repo-Daten.

## Säule E — CPU-Runs (free/cheap, gemessen)

- **GitHub standard x64 + `ubuntu-24.04-arm`** bleiben gratis+unbegrenzt für public (Baseline).
  larger x64/arm64 **immer kostenpflichtig** (x64 4-core $0.012/min, arm64 4-core $0.008/min).
- **Azure Pipelines (bestätigt 2026-10-10):** **10 gratis Microsoft-hosted Parallel-Jobs +
  unbegrenzte Minuten für OSS** (`github.com/apps/azure-pipelines`; Microsoft-Blog 2018). Der
  2021-Grant-Wechsel betraf **private** Projekte, nicht OSS — der OSS-Grant steht (kein gemessener
  2026-Alters-Hinweis). Aktivierung = **Azure-DevOps-Org + Azure-Pipelines-GitHub-App-Install** =
  Operator-Akt (nicht autonom); YAML + Grant-Nachweis = autonom vorbereitbar.
- **Blacksmith OSS-Runner** — 3 000 gratis 2-vCPU-min/mo + OSS-Programm; schnellerer Single-Core.
- **CircleCI OSS** — 400 000 Credits/mo (~80 000 min, OSI-Lizenz) — größter freier Minuten-Pool.
- **Ubicloud** (OSS-Cloud) — 1 250 gratis min/mo, dann $0.00125/min, x64 **und** arm64.
- **AppVeyor OSS** — nur falls je eine Windows-Lane nötig (1 concurrent).
- **Caches:** Turborepo/Vercel Remote Cache (frei) · Nx Cloud Hobby (50 k Credits) · BuildBuddy RBE
  (frei OSS) · **sccache → Cloudflare R2** (Zero-Egress) — schrumpft jeden Lauf.
- **Free/cheap VMs als self-hosted-Runner/Ernte-Box:** **Oracle Always Free 2 OCPU/12 GB ARM**
  (2026 halbiert; 10 TB/mo Egress) · GCP always-free e2-micro (1 GB) · AWS/Azure zeitlich limitiert ·
  Hetzner CX23 2vCPU/4 GB ~€4 · netcup 4 vCPU/8 GB ~€10. **Fly.io ohne free tier.**
- **Codespaces** — 120 core-h/mo (Free) = ~60 h auf 2-Core; Dev-Umgebung, kein CI-Scheduler.
- **Tote Lanes (nicht adoptieren):** Cirrus CI (schließt), BuildJet (tot 2026-03-31), Earthly CI (tot).
- **Riss:** GitHub self-hosted $0.002/min — „postponed" vs. „effektiv 2026-03-01" (Quellen streiten).

## Säule F — MCP-Server (gemessen)

- **Wie opencode MCP fährt:** `opencode.json` → `mcp`: `{type:"local", command:[…]}` (stdio) oder
  `{type:"remote", url, headers, oauth}` (HTTP); Auto-OAuth auf 401 (Tokens in
  `~/.local/share/opencode/mcp-auth.json`); `opencode mcp auth|list|logout|debug`. Docs-Warnung:
  das **GitHub-MCP ergänzt viele Tokens** und kann das Kontextlimit sprengen. opencode.ai/docs/mcp-servers
- **No-leak-Fit (lokal, stdio, kein Fremd-Egress):** `filesystem` (root-scoped), `git` (lokales Repo),
  `memory`, `sequential-thinking`, `time`, `everything`, archiviertes `sqlite`. Genau die Klasse des
  vorhandenen chrome-devtools-MCP.
- **Bedingt:** `fetch` + self-hosted Playwright/Puppeteer (Netz nur zu benannten URLs; Inhalt als
  untrusted behandeln).
- **GitHub-MCP** (`github/github-mcp-server`, Go v2): lokal (stdio, PAT) **oder** remote hosted
  `api.githubcopilot.com/mcp/`; **`/readonly`-Toolset-Variante** bzw. `--read-only`; der **hosted**
  Pfad schickt Repo/Actions-Kontext an `api.githubcopilot.com` = Dritt-Egress (nicht `state/`, aber
  nur fürs eigene public Repo). Toolsets: repos/issues/pull_requests/actions/security_advisories/…
- **Cloudflare-MCP** (`cloudflare/mcp-server-cloudflare`, 16+ Server: Workers/R2/D1/KV, logs, radar):
  streamable-HTTP (`/mcp`) + `mcp-remote`-Bridge; API-Token scoped. Natürlicher R2-Begleiter, aber
  Tools können **schreiben** (Worker deployen, R2 löschen) → Token auf R2-only scopen.
- **Nicht verdrahten (`state/`-Risiko):** AWS/Azure/GCP-Management, hosted Sentry/Slack — lesen **und**
  schreiben nach außen = **lethal trifecta** (untrusted input + sensitive data + outbound channel).
  Wenn doch: **Docker-Gateway** als Container-Isolation (Keychain-Secrets, `--block-secrets`, Logs).
- **Transport 2026-07-28:** stateless core, **streamable HTTP** (Mcp-Method/-Name-Header), SSE deprecated;
  OAuth 2.1 (RFC 9207 `iss`-Prüfung, DCR → CIMD). Registries: Official ~9 652 · mcp.so ~20 222 ·
  PulseMCP 16 820+ · Docker-Katalog 300+ signiert.

## Säule G — APIs (gemessen)

- **GitHub:** REST anon 60/h · PAT 5 000/h · App-Installation ≥5 000/h (Cap 12 500) · `GITHUB_TOKEN`
  1 000/h/repo; GraphQL 5 000 Punkte/h; secundary: 100 concurrent, 900 pts/min REST, ≤80 content-writes/min.
  **Actions-API** (`/actions`) — `workflow dispatch`, runs/jobs/logs/artifacts/caches/runners;
  Checks-, Releases-, GHCR-API; **OIDC** `id-token: write`. docs.github.com/en/rest
- **CI-Vendor-APIs:** CircleCI v2 · Buildkite REST (**200/min org, 50/min user**) · Depot · Blacksmith ·
  Namespace · Cirrus (gRPC/GraphQL) — Blacksmith/Namespace/Depot-Limits `unverified`.
- **Free-Compute-APIs:** **Modal** (`modal token`, Starter $30/mo) · Beam.cloud · Kaggle (`kaggle.json`) ·
  **HF Hub** (anon 1 000/5 min) · RunPod/Vast.ai · Colab **keine Compute-API**.
- **CDN-APIs:** **Cloudflare REST** (global 1 200/5 min/user, 200/s/IP; GraphQL 320/5 min) + **R2 S3-API**
  (10 GB frei, 1M Class A / 10M Class B, Zero-Egress) + **Workers AI** · Backblaze B2 S3 ·
  Bunny Core API · **Zenodo** (auth 100/min, 5 000/h; DOI) · figshare v2 · OSF v2.
- **Free-Inferenz-APIs:** Groq (`gpt-oss-120b` 30 RPM, 1K RPD) · Cerebras (5 RPM free) · CF Workers AI
  (task-RPM) · OpenRouter (`:free` 20 RPM, 50–1 000 RPD) · Gemini free tier · Mistral free mode.
- **Such-APIs (archive_search):** Tavily (dev 100 RPM) · Exa (10 QPS) · Linkup (prepaid) · Jina (keyless) ·
  Firecrawl (Free 10 RPM) · SearXNG (self-host).
- **Höchster Hebel:** Actions-API + `GITHUB_TOKEN` (freies CI) · **Cloudflare REST+R2+Workers AI** aus
  einem Token · **OIDC** keyless · Groq/Cerebras free · Zenodo-DOI.

## Die Runde (Urteil) — Pointer

Sechs Seats antworteten (Qwen3.7-Plus · DeepSeek DeepThink · GLM-5.3 Z.ai · Gemini 3.1 Pro ·
Claude Sonnet 5.5 High · Perplexity/55 Quellen); **Mistral/Vibe `pending`**, **Duck.ai `pending`**,
geteilte `open-weight-ui`-Seats **gelockt (river)**. Rohmaterial + Konvergenz:
`state/stimmen/2026-10-10_mycelium_ci-cdn-roster.md`. **Konvergenz:** R2/OIDC · Workflow-
Konsolidierung · nextest+Built-Reuse+Cache-in-R2 · GPU nur für GPU-Last. **Divergenz:** AI-in-CI
(Rat/Mycelium: nur self-hosted; GLM/Gemini: private Cloud-GPU) = Riss.

## Priorisierte Maßnahmen (handover-fähig; Status = f(Trigger))

1. **Workflow-Konsolidierung** — kanonische Compiler-/CDN-`workflow_call`s, die `*-cdn.yml`
   werden dünne Aufrufer; Messung: Workflow-Zahl + Diff. Status `eigen`. **Schritt 1 gebaut
   (2026-10-10, `0a1f7c9dc`):** Setup byte-identisch (493/501 Toolchain-Steps nach `checkout@v7`);
   **292 distincte Compiler-Bins** → die Run-Rümpfe bleiben distinct (A = A). Kanonischer
   `cdn-manifest.yml` (`on: workflow_call`, `harvests`-JSON, Job-Matrix) + erster Aufrufer
   `planetary-odf-cdn.yml` migriert (Gate grün). Distincte Parser bleiben distinct
   (`ps1-cdn` Band-Slab, `celestrak-eop-cdn` Zwei-Job-Gate).
2. **Lauf-Hygiene** — `concurrency`+`cancel-in-progress`, `paths`-Filter, `timeout-minutes`,
   Retention 90 d. Status `eigen` (River). **Weitgehend geschlossen (gemessen 2026-10-10):**
   436/446 Workflows trugen bereits `concurrency`; `ci-gate` trägt `paths` + per-SHA-`concurrency`
   + `timeout-minutes: 30`; die verbleibenden 16 Einzel-Job-Workflows wurden geschlossen
   (`b1b2bd37f`, Matrix-Workflows bleiben job-scoped). Der ci-gate-**Backlog** ist damit
   **Runner-Durchsatz**, nicht fehlende Hygiene → die zweite Lane (Azure) ist der eigentliche Fix.
3. **Test-Durchsatz** — `nextest archive` + `--partition` auf `ubuntu-24.04-arm` + rust-cache/sccache;
   Messung: 3,5 h → Partition-Wall-Clock. Status `eigen`.
4. **CDN: R2** — Cloudflare R2 als ein Ziel, S3-API-Upload nur im `--ci-mode`, Release-Asset-Spiegel
   <2 GiB. Bucket/API-Key = **per-Akt Operator-Wort**; Preparation autonom. Status `operator-gebunden`.
5. **Agent↔GitHub secretlos** — OIDC `id-token: write` + R2-Worker-Verifier (Claude), `workflow_dispatch`
   nach Push, MCP-GitHub read-only, Rate-Budget als `external-state`. Status `eigen`.
6. **Free-GPU-Probe + self-hosted LLM (hinten)** — Modal **oder** Beam als benannte Probe, Roster erst
   nach Capability-Gate (≥4/4 + Tempo); „AI-in-CI" nur als self-hosted LLM im Runner. Account =
   Operator-Queue. Status `pending` (Riss).

## Rat Runde 2 + Roster (2026-10-10)

**Vorbereitung (Operator-Wort):** `archive_search --all` (voll in `/tmp/omegaflow_all_*`; relevant:
Core 2025 „A comparative study of GitHub-hosted, self-hosted, and Kubernetes-based GitHub Runners",
„An Analysis of Code Clones in GitHub Actions Workflows"; Linkup: lokale Ollama/vLLM-MCP) → **Rat**
(5 Stimmen) → **UI-Runde**.

**Rat-Verdikt (priorisiert, handover-fähig):**
1. **`workflow_call`-Extraktion nach Identität** — byte-gleiche Step-Rümpfe → ein `cdn-manifest.yml`
   (`on: workflow_call`, `source`-Input); distincte Parser bleiben distinct (A = A). Messung: sha256
   der Step-Rümpfe.
2. **Bulk bleibt GitHub-Release-Assets** ($0, 1,76 TB); **R2 = Hot-Tier**, `pending` ohne gemessenen
   Live-Konsumenten; keine Bulk-Migration, nicht gestrichen.
3. **Azure-Lane annehmen**, Guardrail: **genau ein Required-Check je SHA** (GitHub = required,
   Azure = schwere Matrix, non-required); eine Lane bleibt Compile-Owner.
4. **MCP:** Docker-Gateway, dann `filesystem` → `git` → `memory` → `time`; `sequential-thinking`
   zuletzt; kein Cloud-MCP.
5. **AI-in-CI-Riss** bleibt benannt: nur runner-lokaler LLM; kein `state/` an Cloud-GPU.
6. **Sequenz (3):** (a) Duplikat messen (sha256 Step-Rümpfe); (b) `cdn-manifest.yml` via
   `workflow_call` extrahieren; (c) Azure aktivieren (Operator) + MCP verdrahten.

**Counter-Slope:** Identität entscheidet Konsolidierung, nicht Übersicht; die Engstelle wird
**gemessen, dann gekauft** (gratis Lane vor bezahltem Tier, Hot-Tier vor Bulk-Umzug).
**Riss (nicht geglättet):** Granularität — Mountain „ein Compiler je Quelle" vs. Mycelium
„generischer `cdn-manifest`"; die Auflösung ist eine **Messung** (sha256 der Step-Rümpfe), keine
Abstimmung.

**Roster (Urteil):** **GLM 5.3 Flash** (open-weight, `tryingopen`) antwortete: (a) ja, Identitäts-Merging
zuerst — deterministisch, reversibel, sofort messbar; semantisches Merging ist die falsche erste Runde;
(b) ja, aber die **Eviction-Policy des Hot-Tiers jetzt definieren** (LRU nach CI-Zugriffszeit, nicht
Alter); (c) ja, Guardrail ein Required-Check, eine Lane muss Compile-Owner bleiben; (d) **übersehen:
Dedup der Test-Suite selbst** — Test-Impact-Analyse/Caching schlägt Workflow-Merging zeitlich.
Die übrigen Seats der Linie: Folge-Frage teils mechanisch nicht gesendet (Senden-Button), die
Erstrunden-Antworten stehen in `state/stimmen/2026-10-10_mycelium_ci-cdn-roster.md`; als `pending`
benannt, nicht wiederholt. Geteilte `open-weight-ui`-Seats kurz gefahren (Lock gesetzt/gelöscht).

## Grenzen (0 honored)

- Diese Survey ist eine gemessene Landschaft, **keine** Messung des gebauten Zustands. Jede Maßnahme
  trägt ihre eigene Messung, wenn sie gebaut wird — bis dahin `pending`.
- Die Runde ist ein zweiter Kanal (Urteil), nicht die Messung; die Session/Rat trägt das Verdikt.
- `unverified` (nicht gemessen, benannt): sccache-Netto-Gewinn auf GH-Runnern; exaktes nextest-JUnit-Flag;
  jsDelivr-Größenlimite 2026; dokumentierte GitHub-Runner-Präemption (nur Community-Arkunde).
