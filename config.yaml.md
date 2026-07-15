# =============================================================================
# Continue.dev Configuration
# =============================================================================
name: omegaflow
version: 1.0.0
schema: v1
# -----------------------------------------------------------------------------
# UI Settings
# -----------------------------------------------------------------------------
ui:
codeBlockWrap: true
showSessionTabs: true
showChatScrollbar: true
generateSessionTitles: true
markdown: true
fontSize: 14
multilineAutocomplete: auto
disableAutocompleteInFiles:
- "*.is"
- "*.dat"
- "*.idx"
- "*.φ"
- immunity.is
- epigenome.is
- dormant
- index.html
- scan.html
- main.rs
- sources.φ
- constants.js
- definitions.js
addCurrentFileToContext: true
useToolsForCodebaseRetrieval: true
streamAfterToolRejection: true
experimental:
allTools: false
systemMessageToolsOnly: false
modelContextProtocol: true
# =============================================================================
# MODELS
# =============================================================================
models:
# ===========================================================================
# 1. LOCAL MODELS (Ollama)
# ===========================================================================
# HARDWARE-HINWEIS (Dell XPS 13 9350, 2016, i5, 8GB RAM, Intel HD 520):
# Kein dediziertes VRAM -> Ollama läuft komplett auf CPU + Shared RAM.
# 8GB RAM ist der limitierende Faktor: OS + Editor + Browser + Continue.dev
# belegen realistisch schon 4-5GB, bleiben ~3-4GB für ein Modell.
#   - qwen2.5-coder:1.5b (Q4, ~1GB) -> unproblematisch, guter Autocomplete
#   - qwen2.5-coder:7b (Q4, ~4.5GB) -> SEHR eng bis nicht tragbar, wird
#     spürbar swappen und Autocomplete/System verlangsamen
#   -> ersetzt durch qwen2.5-coder:3b (Q4, ~2GB): deutlich sicherer, noch
#      brauchbare Qualität für einfaches Chat/Edit lokal.
# Codebase-Indexing (Embeddings) ist ebenfalls CPU-lastig: bei großen
# Repos (viele tausend Dateien) kann der initiale Index-Lauf mehrere
# Minuten bis Stunden dauern. Empfehlung: .continueignore konsequent
# nutzen (build/, target/, node_modules/, große Assets ausschließen),
# sonst wird sowohl RAM als auch Zeit unnötig verbraucht.
- name: 🆓💻 Qwen2.5-Coder 1.5B │ ⚡ Tab-Autocomplete
provider: ollama
model: qwen2.5-coder:1.5b
roles:
- autocomplete
autocompleteOptions:
debounceDelay: 250
maxPromptTokens: 512
prefixPercentage: 0.5
maxSuffixPercentage: 0.3
modelTimeout: 400
# debounceDelay erhöht, maxPromptTokens gesenkt: auf schwacher CPU
# reduziert das spürbar "Ruckeln" beim Tippen.
- name: 🆓💻 nomic-embed-text │ 🔍 Codebase Embeddings
provider: ollama
model: nomic-embed-text
roles:
- embed
# ===========================================================================
# 2. Z.AI — GLM MODELLE (optimiertes Routing nach omegaflow-Konventionen)
#    Alle Preise pro 1M Tokens, Input / Output. "Coding Plan" = Flatrate
#    ohne Pay-per-Token über apiBase .../api/coding/paas/v4.
# ===========================================================================
# --- 2.0 FREE TIER (Standard für Routineaufgaben) -------------------------
- name: 🆓☁️ GLM-4.7 Flash │ ⚡🔧 Free & Fast [128K]
provider: openai
apiBase: https://api.z.ai/api/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- edit
- apply
model: glm-4.7-flash
defaultCompletionOptions:
contextLength: 128000
maxTokens: 8192
- name: 🆓☁️ GLM-4.6V Flash │ 📸 Free Vision [128K]
provider: openai
apiBase: https://api.z.ai/api/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- summarize
model: glm-4.6v-flash
capabilities:
- tool_use
- image_input
defaultCompletionOptions:
contextLength: 128000
maxTokens: 8192
# --- 2.1 MITTELKLASSE (für Vision-Analysen) -------------------------------
- name: 🆓☁️ GLM-4.5V │ 📸🔧 Mittelklasse Vision [128K]
provider: openai
apiBase: https://api.z.ai/api/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- summarize
model: glm-4.5v
capabilities:
- tool_use
- image_input
defaultCompletionOptions:
contextLength: 128000
maxTokens: 8192
# --- 2.2 ULTRA-BUDGET (~$0.03–0.1 / $0.1–0.4 pro 1M) ----------------------
- name: €☁️ GLM-4-32B-0414 │ ⚡ Ultra-Budget [128K]
provider: openai
apiBase: https://api.z.ai/api/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- edit
- apply
model: glm-4-32b-0414-128k
defaultCompletionOptions:
contextLength: 128000
maxTokens: 8192
- name: €☁️ GLM-OCR │ 📄 Dokument-/Texterkennung [Budget]
provider: openai
apiBase: https://api.z.ai/api/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- summarize
model: glm-ocr
capabilities:
- image_input
defaultCompletionOptions:
contextLength: 32000
maxTokens: 4096
- name: €☁️ GLM-4.7 FlashX │ ⚡🔧 Budget Coding [128K]
provider: openai
apiBase: https://api.z.ai/api/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- edit
- apply
model: glm-4.7-flashx
defaultCompletionOptions:
contextLength: 128000
maxTokens: 8192
- name: €☁️ GLM-4.6V FlashX │ 📸 Budget Vision [128K]
provider: openai
apiBase: https://api.z.ai/api/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- summarize
model: glm-4.6v-flashx
capabilities:
- tool_use
- image_input
defaultCompletionOptions:
contextLength: 128000
maxTokens: 8192
# --- 2.3 MITTELKLASSE (~$0.2–0.6 / $1.1–2.2 pro 1M) -----------------------
- name: €€☁️ GLM-4.5-Air │ 🔧 Mittelklasse Coding [128K]
provider: openai
apiBase: https://api.z.ai/api/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- edit
- apply
model: glm-4.5-air
defaultCompletionOptions:
contextLength: 128000
maxTokens: 8192
- name: €€☁️ GLM-4.5 │ 🧠🔧 Mittelklasse Reasoning [128K]
provider: openai
apiBase: https://api.z.ai/api/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- edit
- apply
- summarize
model: glm-4.5
defaultCompletionOptions:
contextLength: 128000
maxTokens: 8192
- name: €€☁️ GLM-4.6 │ 🧠🔧 Mittelklasse Reasoning (neuer) [200K]
provider: openai
apiBase: https://api.z.ai/api/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- edit
- apply
- summarize
model: glm-4.6
defaultCompletionOptions:
contextLength: 200000
maxTokens: 8192
- name: €€☁️ GLM-4.7 │ 🧠🔧 Mittelklasse (aktuellster 4.x) [200K]
provider: openai
apiBase: https://api.z.ai/api/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- edit
- apply
- summarize
model: glm-4.7
capabilities:
- tool_use
defaultCompletionOptions:
contextLength: 200000
maxTokens: 8192
# --- 2.4 OBERKLASSE (~$1.0–1.4 / $3.2–4.4 pro 1M) -------------------------
- name: €€€☁️ GLM-5 │ 🧠🔧 Oberklasse Coding [200K]
provider: openai
apiBase: https://api.z.ai/api/coding/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- edit
- apply
- summarize
model: glm-5
capabilities:
- tool_use
defaultCompletionOptions:
contextLength: 200000
maxTokens: 8192
requestOptions:
extraBodyProperties:
thinking:
type: enabled
reasoning_effort: high
- name: €€€☁️ GLM-5 Turbo │ ⚡🔧 Oberklasse, schnell [262K]
provider: openai
apiBase: https://api.z.ai/api/coding/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- edit
- apply
- summarize
model: glm-5-turbo
capabilities:
- tool_use
defaultCompletionOptions:
contextLength: 262144
maxTokens: 8192
requestOptions:
extraBodyProperties:
thinking:
type: enabled
reasoning_effort: high
- name: €€€☁️ GLM-5V Turbo │ 📸🔧 Oberklasse Vision-Agent [203K]
provider: openai
apiBase: https://api.z.ai/api/coding/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- edit
- apply
- summarize
model: glm-5v-turbo
capabilities:
- tool_use
- image_input
defaultCompletionOptions:
contextLength: 203000
maxTokens: 8192
requestOptions:
extraBodyProperties:
thinking:
type: enabled
reasoning_effort: high
- name: €€€☁️ GLM-5.1 │ 🧠🔧 Oberklasse Long-Horizon [200K]
provider: openai
apiBase: https://api.z.ai/api/coding/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- edit
- apply
- summarize
model: glm-5.1
capabilities:
- tool_use
- image_input
defaultCompletionOptions:
contextLength: 200000
maxTokens: 8192
requestOptions:
extraBodyProperties:
thinking:
type: enabled
reasoning_effort: high
# --- 2.5 FLAGSHIP GLM-5.2 (für komplexe Rust/Binärprotokoll-Arbeit) ------
- name: €€€☁️ GLM-5.2 │ ⚡🔧 Fast (No Reasoning) [200K]
provider: openai
apiBase: https://api.z.ai/api/coding/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- edit
- apply
model: glm-5.2
defaultCompletionOptions:
contextLength: 200000
maxTokens: 8192
requestOptions:
extraBodyProperties:
thinking:
type: disabled
reasoning_effort: none
- name: €€€☁️ GLM-5.2 │ 🧠🔧 Balanced (High Reasoning) [200K]
provider: openai
apiBase: https://api.z.ai/api/coding/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- edit
- apply
- summarize
model: glm-5.2
capabilities:
- tool_use
- image_input
defaultCompletionOptions:
contextLength: 200000
maxTokens: 8192
requestOptions:
extraBodyProperties:
thinking:
type: enabled
reasoning_effort: high
- name: €€€€☁️ GLM-5.2 │ 🧠🔧📦 Deep (Max Reasoning) [1M]
provider: openai
apiBase: https://api.z.ai/api/coding/paas/v4
apiKey: ${{ secrets.ZAI_API_KEY }}
roles:
- chat
- edit
- apply
- summarize
model: glm-5.2
capabilities:
- tool_use
- image_input
defaultCompletionOptions:
contextLength: 1000000
maxTokens: 16384
requestOptions:
extraBodyProperties:
thinking:
type: enabled
reasoning_effort: max
# =============================================================================
# CONTEXT PROVIDERS
# =============================================================================
context:
- provider: web
params:
apiBase: https://api.tavily.com
apiKey: ${{ secrets.TAVILY_API_KEY }}
model: tavily-searchcontext
- provider: currentFile
- provider: code
params:
languages:
- rust
- html
- javascript
- toml
- provider: file
- provider: open
params:
onlyPinned: true
- provider: diff
- provider: terminal
- provider: problems
- provider: os
- provider: docs
params:
search: true
# =============================================================================
# DOCS
# =============================================================================
docs:
- name: Rust std
startUrl: https://doc.rust-lang.org/std/
favicon: https://doc.rust-lang.org/favicon.ico
- name: MDN Web APIs
startUrl: https://developer.mozilla.org/en-US/docs/Web/API
favicon: https://developer.mozilla.org/favicon.ico
- name: WebGPU & WGSL
startUrl: https://developer.mozilla.org/en-US/docs/Web/API/WebGPU_API
favicon: https://developer.mozilla.org/favicon.ico
- name: Continue.dev
startUrl: https://docs.continue.dev/intro
favicon: https://docs.continue.dev/favicon.ico
# =============================================================================
# MCP SERVERS
# =============================================================================
mcpServers:
# ---------------------------------------------------------------------------
# 1. Vision MCP (lokal, stdio) — Bild-/Video-Analyse
# ---------------------------------------------------------------------------
- name: zai-vision
command: /usr/bin/npx
args:
- -y
- "@z_ai/mcp-server@latest"
env:
Z_AI_API_KEY: ${{ secrets.ZAI_API_KEY }}
Z_AI_MODE: ZAI
alwaysAllow:
- ui_to_artifact
- extract_text_from_screenshot
- diagnose_error_screenshot
- understand_technical_diagram
- analyze_data_visualization
- ui_diff_check
- image_analysis
- video_analysis
# ---------------------------------------------------------------------------
# 2. Web Search MCP (remote, HTTP) — Live-Websuche
# ---------------------------------------------------------------------------
- name: zai-web-search
type: streamable-http
url: https://api.z.ai/api/mcp/web_search_prime/mcp
requestOptions:
headers:
Authorization: Bearer ${{ secrets.ZAI_API_KEY }}
alwaysAllow:
- webSearchPrime
# ---------------------------------------------------------------------------
# 3. Web Reader MCP (remote, HTTP) — Volltext-Extraktion einer URL
# ---------------------------------------------------------------------------
- name: zai-web-reader
type: streamable-http
url: https://api.z.ai/api/mcp/web_reader/mcp
requestOptions:
headers:
Authorization: Bearer ${{ secrets.ZAI_API_KEY }}
alwaysAllow:
- webReader
# ---------------------------------------------------------------------------
# 4. Zread MCP (remote, HTTP) — Public-GitHub-Repo-Analyse
#    Hinweis: benötigt GLM Coding Plan
# ---------------------------------------------------------------------------
- name: zai-zread
type: streamable-http
url: https://api.z.ai/api/mcp/zread/mcp
requestOptions:
headers:
Authorization: Bearer ${{ secrets.ZAI_API_KEY }}
alwaysAllow:
- search_doc
- read_file
- get_repo_structure
# ---------------------------------------------------------------------------
# 5. GitHub MCP (lokal, stdio) — Issues, PRs, Commits, Code-Suche
#    Braucht ein Personal Access Token mit repo-Scope (github.com/settings/tokens)
# ---------------------------------------------------------------------------
- name: github
command: npx
args:
- -y
- "@modelcontextprotocol/server-github"
env:
GITHUB_PERSONAL_ACCESS_TOKEN: ${{ secrets.GITHUB_PERSONAL_ACCESS_TOKEN }}
# ---------------------------------------------------------------------------
# 6. Playwright MCP (lokal, stdio) — Browser-Automation & Screenshots
#    Sinnvoll kombiniert mit zai-vision: Seite rendern -> screenshotten ->
#    von zai-vision analysieren lassen (z.B. WebGPU/WGSL-Rendering pruefen).
# ---------------------------------------------------------------------------
- name: playwright
command: npx
args:
- "@playwright/mcp@latest"
# ---------------------------------------------------------------------------
# 7. Context7 MCP (remote, HTTP) — aktuelle, versionsgenaue Library-Doku
#    Funktioniert ohne API-Key (Basis-Rate-Limit); Key optional fuer mehr
#    Requests/Tag, kostenlos unter context7.com/dashboard.
# ---------------------------------------------------------------------------
- name: context7
type: streamable-http
url: https://mcp.context7.com/mcp
requestOptions:
headers:
CONTEXT7_API_KEY: ${{ secrets.CONTEXT7_API_KEY }}
# Falls kein Key vorhanden: die Zeile "CONTEXT7_API_KEY: ..." oben loeschen
# oder CONTEXT7_API_KEY einfach leer in der .env lassen - Server laeuft
# dann im kostenlosen Basis-Rate-Limit weiter.
# =============================================================================
# NOTE
# =============================================================================
# Die "omegaflow identity" (README/φ-Notation) und das heuristische Gremium
# (Mountain / River / Mycelium / Sensory / Future) leben NICHT hier, sondern
# in: .continue/rules/omegaflow-gremium.md
# Grund: eigenes YAML-Frontmatter-Schema (description-only), Fließtext mit
# Sonderzeichen (φ, ω, Δt) und Blockskalaren — das würde die strikte
# Modell-/Provider-Validierung dieser Datei unnötig gefährden.
# Continue.dev lädt Rules-Dateien automatisch als zusätzlichen Kontext für
# den Chat, unabhängig von dieser config.yaml.
#
# Z.AI-Modelle: vollständige, nach Kosten gestaffelte Liste
# (Preise laut docs.z.ai/guides/overview/pricing, $/1M Tokens Input→Output):
#   🆓 Free      GLM-4.7-Flash, GLM-4.5-Flash, GLM-4.6V-Flash        $0 → $0
#   €  Budget    GLM-4-32B-0414 ($0.10→$0.10), GLM-OCR ($0.03→$0.03),
#                GLM-4.7-FlashX ($0.07→$0.40), GLM-4.6V-FlashX ($0.04→$0.40)
#   €€ Mittel    GLM-4.5-Air ($0.20→$1.10), GLM-4.5V ($0.60→$1.80),
#                GLM-4.5 ($0.60→$2.20), GLM-4.6 ($0.60→$2.20),
#                GLM-4.7 ($0.60→$2.20)
#   €€€ Ober     GLM-5 ($1.00→$3.20), GLM-5-Turbo ($1.20→$4.00),
#                GLM-5V-Turbo ($1.20→$4.00), GLM-5.1 ($1.40→$4.40)
#   €€€/€€€€ Flagship  GLM-5.2 in 3 Reasoning-Stufen ($1.40→$4.40, gestaffelt
#                nach reasoning_effort: none/high/max)
#
# Hinweis: apiBase .../api/coding/paas/v4 erwartet i.d.R. einen aktiven
# GLM Coding Plan (Flatrate); .../api/paas/v4 ist der reguläre Pay-per-Token-
# Endpoint. Modellnamen (glm-5, glm-5-turbo, glm-5.1, glm-4.6, etc.) vor
# Produktivbetrieb gegen die aktuelle Z.AI-Doku prüfen, da neue Versionen
# schnell erscheinen.
#
# =============================================================================
# OMEGAFLOW MODEL ROUTING (optimiert nach omegaflow-project-conventions.md)
# =============================================================================
# AUTOCOMPLETE (Tab) -> 🆓💻 Qwen2.5-Coder 1.5B (lokal, kein Kontingent)
# 
# CHAT & EINFACHE EDITS (Routineaufgaben, Doku, Boilerplate) ->
# 🆓☁️ GLM-4.7 Flash (free, 128K)
# 
# KOMPLEXE RUST/BINÄRPROTOKOLL-ARBEIT, REFACTORING ->
# €€€☁️ GLM-5.2 Balanced (high reasoning, 200K)
# 
# VISION-ANALYSEN (Screenshots, Diagramme, UI) ->
# 🆓☁️ GLM-4.6V Flash (free, 128K) oder
# €€☁️ GLM-4.5V (mittelklasse, 128K)
# 
# DOKUMENTATION & ZUSAMMENFASSUNGEN ->
# 🆓☁️ GLM-4.7 Flash (free, 128K)
# 
# EMBEDDINGS ->
# 🆓💻 nomic-embed-text (lokal, kostenlos)
#
# KOSTENOPTIMIERUNG:
#   - ~70% der Aufgaben mit kostenlosen GLM-Flash-Modellen erledigen
#   - Nur komplexe Refactors und Rust-Spezialaufgaben mit GLM-5.2
#   - Lokale Modelle (Ollama) für Autocomplete/Embeddings
#
# =============================================================================
# STRATEGIE: NUR GLM (Coding Plan + Free-Tier) + lokale Ollama-Modelle.
# Umgestellt am 02.07.2026 — Vorher zusätzlich Gemini/Cloudflare/OpenRouter
# als Fallbacks, jetzt bewusst komplett auf GLM konsolidiert (Ethos/Stil
# passt für diesen Nutzer besser, weniger Provider-Wechsel im Alltag).
# =============================================================================
# Enthalten sind ausschließlich:
#   - Alle GLM-Modelle über den Coding Plan bzw. Pay-per-Token-Endpoint
#     (zählt vollständig gegen dein Z.AI-Kontingent — kein Zusatz-Invoice)
#   - Lokale Ollama-Modelle (Qwen2.5-Coder 1.5B, nomic-embed) — 0 Kosten,
#     laufen komplett auf deiner eigenen Hardware. Bewusst NICHT auf GLM
#     umgestellt:
#     * Autocomplete: Cloud-Latenz bei jedem Tastendruck wäre spürbar UND
#       würde das GLM-Kontingent im Sekundentakt verbrauchen.
#     * Embed: aktuell eh irrelevant (disableIndexing: true in
#       .continuerc.json), und Embedding-Läufe über große Repos sind viele
#       Tokens — lokal kostenlos, cloud würde nur Kontingent fressen.
#
# Entfernt (Stand 02.07.2026 bewusst rausgenommen, kein Fallback mehr):
#   - Google Gemini 2.5 Flash, Cloudflare Workers AI, beide OpenRouter-
#     Modelle. Kein technischer Grund (liefen nach den Fixes einwandfrei),
#     reine Konsolidierung auf einen Anbieter.
#   - Achtung: das bedeutet KEIN Fallback mehr, falls Z.AI down ist oder
#     das Coding-Plan-Kontingent für den Monat aufgebraucht ist. Falls das
#     mal zum Problem wird: irgendein Free-Tier-Modell (z.B. Gemini 2.5
#     Flash oder OpenRouter „openrouter/free") lässt sich jederzeit wieder
#     als Notfall-Fallback ergänzen.
#
# Gegen dein Coding-Plan-Limit hilft am meisten:
#   1. Reasoning-Stufe bewusst wählen — "Fast/None" statt "Balanced/Deep"
#      für einfache Edits spart einen Großteil des Kontingents.
#   2. Für Routineaufgaben (Boilerplate, einfache Refactors, Doku) auf
#      GLM-Flash/FlashX (free/billig) statt der teuren GLM-5.x-Stufen
#      ausweichen — siehe Modell-Routing-Tabelle in
#      omegaflow-project-conventions.md.
#   3. Autocomplete läuft ohnehin lokal über Ollama — kostet nie etwas
#      vom Plan.
#   4. In den letzten 1-2 Tagen vor Reset: konsequent auf GLM-Flash (free)
#      umschalten und teurere Stufen nur für Aufgaben reservieren, die
#      Flash nachweislich nicht gut löst.
# =============================================================================