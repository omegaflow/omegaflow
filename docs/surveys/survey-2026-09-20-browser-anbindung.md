<!--
  title: Survey — Browser-Anbindung: Extensions, Captcha, Verdikte (Stand 2026-09-20)
  class: survey
  date: 2026-09-20
  sha256: c964a19b5294858098b564ecb07bb2fc6eb7d382ad494957c0916e906031383c
  status: live
  see-also: docs/concepts/tools-map.md
-->
# Survey — Browser-Anbindung: Extensions, Captcha, Verdikte (Stand 2026-09-20)

Anlass: die Browser-Anbindung soll für drei Ziele sauber und stabil tragen —
(i) Debuggen der Membran (`cargo run` → `127.0.0.1:1618`, WebGPU-Seite, `φ window:`
stderr-Zeile), (ii) Recherche (JS-Seiten, Cloudflare), (iii) Registrierungen bei
Dritt-Quellen. Der Operator hat Captcha-Option (a) gesetzt (Operator löst selbst im
sichtbaren Browser), Option (b) (lokale Erweiterung) zur Prüfung benannt, Option (c)
(bezahlte Solver) ausgeschlossen. Messung + Urteil, kein Wunsch.

## Messbasis

`archive_search` (`--github` ×13, `--brave` ×6, `--crates` ×3, `--verdict` ×5, `--all`
×1 → `/tmp/omegaflow_all_1789916874_chrome_extension_browser_automation_MCP.txt`, 427
Zeilen, `--playwright` ×1), GitHub-REST-API (20 Repos), npm-Registry, Raw-READMEs,
`vision`-Lesung der Buster-GitHub-Seite. Alle `pushed_at`/`stargazers_count`/`license`-
Werte API-gemessen 2026-09-20; URLs direkt verifiziert (HTTP 200, `--verdict` stage 1),
sofern nicht anders benannt.

## Der Ist-Zustand — drei Pfade, drei Naturen (A = A)

| Pfad | Was | Wofür gemessen genutzt |
|---|---|---|
| 1 `@vymalo/opencode-browser` 0.16.1 | CDP-Bridge, lokaler Server Port 4517, Extension am Operator-Profil | Gruppen `taprecon`, `register` |
| 2 `@playwright/mcp` | globaler MCP, headless, eigener Kontext/Profil | Cross-Browser/Tests |
| 3 `archive_search --playwright` | eigenes Rust→node-Tool, persistentes Profil `~/.cache/omegaflow/playwright-profile`, SOCKS/proton, Cloudflare-Interstitial-Erkennung | Kopflos-Fetch/Recherche |

## Befund A — Kandidaten je Klasse

**K1 CDP-/MCP-Bridges.** **Chrome DevTools MCP** (ChromeDevTools/chrome-devtools-mcp,
Apache-2.0, push 2026-09-20, npm `chrome-devtools-mcp@1.9.0`, 52 355★): steuert live
Chrome via CDP (puppeteer), Performance-Traces/Network/Console/Screenshots; Chrome-only;
Google-Telemetrie default an (Flags `--no-usage-statistics`, `--no-performance-crux`
nötig). **Playwright MCP + Browser Extension** (microsoft/playwright-mcp, Apache-2.0,
npm `@playwright/mcp@0.0.82`, 2026-09-18, 37 387★): Extension dockt an bestehende
Operator-Tabs an (Session/Cookies/Extensions). **@vymalo/opencode-browser** npm **0.17.0**
(Projekt nutzt 0.16.1 → Versionslücke gemessen), MIT, Repo-Heimat jetzt
ADORSYS-GIS/lightbridge-opencode-toolbeit. **hangwin/mcp-chrome** (MIT, push 2026-01-06,
~8,5 Monate still, 12 437★). **BrowserMCP** (browsermcp/mcp, **Apache-2.0 gemessen**;
läuft lokal — README: „your browser activity stays on your device and isn't sent to
remote servers"). **real-browser-mcp** (51★, Nische). **kapture** (license null → aus).
**native-devtools-mcp** (vectora-foundry, **Rust** gemessen, MIT, 0.10.1; **Linux nicht
unterstützt** — README „Linux is not supported yet", badge macOS|Windows|Android). Gesichtet, nicht im Detail: different-ai/opencode-browser
(580★, license null), michaljach/opencode-browser (100★, MIT), stowns/opencode-browser-
extension (2★, license null), Smotree/BrowserMCP, cmsflash/agent-browser-mcp.

**K2 DOM→Text/Markdown.** **MarkDownload** (deathau/markdownload, Apache-2.0, push
2025-06-11, 4023★; Readability.js + Turndown, lokal, kein Server).

**K3 Cookie/Profil/Challenge.** **Cookie-Editor** (Moustachauve/cookie-editor, GPL-3.0,
push 2026-09-03, 1706★; Export/Import). **Privacy Pass** (cloudflare/pp-browser-
extension, BSD-3-Clause, push 2026-08-05, 416★): **kein Captcha-Solver** — anonyme
Tokens, reduzieren Challenge-Häufigkeit, von Cloudflare selbst für End-Nutzer
empfohlen; Alt-Repo privacypass/challenge-bypass-extension **DEPRECATED** (push
2025-02-17). Kein seriöser Cloudflare-„Bypass"-Store-Kandidat gemessen; Treffer sind
Python-Tools (FlareSolverr) oder Blogs.

**K4 Tab/Session.** **Tab Session Manager** (sienori/Tab-Session-Manager, MPL-2.0, push
2026-08-17, 2456★).

**K5 Screenshot/Vision.** über Chrome DevTools MCP abgedeckt; kein eigener Kandidat
gemessen → `pending`.

**K6 Session Replay.** nur SaaS gemessen (Amplitude/Browsee/Sentry); `puppeteer/replay`
ist Library, keine Extension. DevTools Recorder eingebaut → als Extension-Suche
`pending`.

## Befund B — Buster (Captcha-Option b), mit `vision`

- **Repo** https://github.com/dessant/buster — GPL-3.0 („Copyright (c) 2018-2026 Armin
  Sebastian"), JavaScript, 9301★, push 2026-06-27, Release **v3.4.0** (2026-06-20).
- **Store** Chrome: `chromewebstore.google.com/detail/buster-captcha-solver-for/
  mpbjkejclgfgadiemmefgebjfooflfhl` — HTTP 200 direkt; README nennt zusätzlich Firefox,
  Edge, Opera.
- **Prinzip (README, `vision`-gelesen):** „helps you to solve difficult CAPTCHAs by
  completing **reCAPTCHA audio challenges using speech recognition**… solved by
  clicking on the extension button at the bottom of the reCAPTCHA widget" —
  semi-automatisch (Mensch klickt). Optionale Client-App (dessant/buster-client)
  verbessert die Erfolgsrate. Screenshots zeigen reCAPTCHA-Bild- und -Audio-Challenge.
- **Lokal vs. Dritter:** README nennt nur „speech recognition"; sichtbare
  Commit-Messages nennen **„feat: solve challenges with a local Whisper model"** →
  lokale Spracherkennung existiert als Weg. Der Web-Speech-API-Weg (Chrome = Google-
  Server-STT) bleibt als Alternative ungemessen → `pending`.
- **Gemessen 2026-09-20 (Folge 110):** `manifest_version: 3`,
  `minimum_chrome_version: 123.0` (`src/assets/manifest/chrome.json`); **hCaptcha nicht
  unterstützt** (README nennt nur reCAPTCHA; offenes Issue #319); STT = **lokales
  Whisper** (`Xenova/whisper-tiny`, wasm/q8, via `@huggingface/transformers`), nicht die
  Web-Speech-API. Store-„Updated"-Datum **gemessen 2026-09-20** (Operator-Lesung im
  echten Browser): Version **3.4.0**, aktualisiert **20. Juni 2026**, Größe 6,57 MiB,
  Sprache English — die Store-`pending`-Lücke ist geschlossen.

## Befund C — Verdikte

- **bereichernd:** Chrome DevTools MCP (Ziel i; Telemetrie-Flags Bedingung),
  Playwright Browser Extension (Pfad 2 → echtes Profil; **befund-gated**, s. Rat),
  @vymalo 0.17.0 (Pfad-1-Update; **Drop-in gemessen** — 0.17.0-BREAKING betrifft nur
  `lightbridge`, Browser-Options unverändert, `{port, token}` gültig), BrowserMCP
  (Apache-2.0, lokal), Cookie-Editor (Ziel iii, Operator-Werkzeug), Privacy Pass (Ziel ii),
  MarkDownload (Nische), Tab Session Manager (niedrig).
- **abgelehnt:** kapture / different-ai-opencode-browser / stowns (license null),
  real-browser-mcp (Nische), native-devtools-mcp (Linux nicht unterstützt),
  nanobrowser (In-Browser-Agent-Paradigma, keine Bridge),
  browser-use + FlareSolverr (Python gemessen), alle bezahlten Solver/Cloud-Browser
  (Option c), Session-Replay-Extensions (nur SaaS).
- **Ersetzt wird nichts.** Pfad 1 bleibt (Operator-Profil-Bridge), Pfad 2 bleibt
  (Cross-Browser/Tests) und wird durch Chrome DevTools MCP ergänzt, Pfad 3 bleibt
  (eigener Kopflos-Fetcher). Store-Seiten brauchen in Pfad 3 vorab Consent-Cookies
  (gemessen: Google-Consent-Wand im headless Profil; `--playwright` traf
  `consent.google.com`).

## Befund D — Rat-Verdikt (Architektur)

Der Rat (2026-09-20) trägt einstimmig: **nicht konsolidieren** — drei Pfade sind drei
Identitäten, je mit gemessener Nutzung; Konsolidierung wäre ein Descope ohne Befund.

- **(i) Membran-Debug → Chrome DevTools MCP: ja.** Es ist der Debugger (Konsole/Netz/
  Performance via CDP), den heute kein Pfad trägt; Telemetrie-Flags sind Bedingung
  (Silence-Regel). **Operator-Wort nötig** (Debugger-Rechte am live Chrome).
- **(iii) Registrierung → Pfad 1 (Operator-Profil).** Dort leben Sessions und
  Sichtbarkeit; die Operatorin löst die Captcha selbst (Option a). Die Maschine füllt
  das Formular bis an die Kante, legt den Akt vor, Operator-Wort + Operator-Hand.
- **(ii) Recherche → Pfad 3 erster Zug**, Pfad 2 für mehrschrittige Flüsse. Die
  Playwright-Browser-Extension wird **befund-gated** erst eingebaut, wenn Pfad 1 an
  einer konkreten Seite gemessen scheitert — nie als vierte Dauerleitung.
- **Store-IDs (gemessen 2026-09-20, Folge 110):** „OpenCode Browser"
  `cabnfapnafjlijmbpmgjkgobhdkbmpci`; Playwright Extension
  `mmlmfjhmonkocbjadbfplnigmagldckm`; Buster `mpbjkejclgfgadiemmefgebjfooflfhl`;
  `chrome-devtools-mcp` npm **1.9.0** (Telemetrie-Flags `--no-usage-statistics`,
  `--no-performance-crux` bestätigt).
- **Buster (Option b) descoped mit Befund** — solange das Operator-Wort ausbleibt;
  Option (a) braucht die Operator-Präsenz ohnehin, die Captcha reitet denselben Akt.
  Bezahlte Solver bleiben ausgeschlossen.

## Bau-Schnitt (Reihenfolge, mit Operator-Wort-Grenzen)

1. Chrome DevTools MCP pinnen (npm 1.9.0) + Telemetrie-Flags; an den Pfad-1-Chrome;
   Verifikation: ein Membran-Lauf, Konsole/Netz gelesen. **Operator-Wort.**
2. `tools-map.md` um die Drei-Pfade-Rollentabelle erweitern (Name = Implementation).
3. Pfad-1-Version 0.16.1 → 0.17.0 schließen (oder als `pending` mit Grund registrieren).
4. Cookie-Editor als **manuelles** Operator-Werkzeug; Transfer nur per Akt mit
   Operator-Wort (Cookies = Zugangsdaten).
5. Playwright-Browser-Extension **nicht** einbauen — befund-gated im Handover.

## Messnachtrag 2026-09-23 — Pfad 4 registriert (`opencode-chromium`)

Operator-Wort „beide behalten und registrieren". Bau-Schnitt Punkt 5 ist damit überholt: neben
Pfad 1 wurde `opencode-chromium` (stärkster Kandidat aus Befund A, „bereichernd") installiert,
vermessen und als vierter Pfad registriert (`docs/concepts/tools-map.md`).

**Mechanik:** npm `opencode-chromium@1.7.2` (MIT, `Quindart-com/opencode-chromium`) + Store-Extension
`hdljmmpfnhojebplbbgdgejoobmjcbml` + Native-Messaging-Host `com.opencode.browser.plugin` (kein
localhost-Dauerport; CDP auf kontrollierten Tabs). Vier Tools: `browser_run` (Step-Kette ≤20),
`browser_observe`, `browser_session`, `browser_finalize`; `find` = semantische Suche (`query`).

**Messung (2026-09-23, gleiche Maschine, gleiches Chrome Profile 1, example.com):**
- Transport-Roundtrip: Pfad 1 `tabs` 12–20 ms; Pfad 4 echte Host-Roundtrips ~10–20 ms → gleichauf.
- Navigation: Pfad 1 ~180 ms (warm) / ~475 ms (cache-busted, wartet auf `load`); Pfad 4 default
  `dom-quiet`-Settle ~2,7 s (Timeout) → mit `waitUntil:load` + `settle:{exists,selector}` ~130 ms.
- Text/DOM: Pfad 1 `get_text` 9–20 ms; Pfad 4 `observe extract` 13–60 ms.
- Koexistenz beider Brücken am selben Profil: gemessen, kein Konflikt.

**Verdikt:** Transport identisch; der spürbare Unterschied ist die Warte-Politik (Pfad 4 default
langsamer, mit gezieltem Settle schneller; batcht bis 20 Steps/Roundtrip). Pfad 4 trägt den
OpenCode-1.18.x-Adapter **und** den V2-Vertrag → blockiert den opencode-2.0-Umstieg nicht.
