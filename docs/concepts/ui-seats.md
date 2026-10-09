<!--
  title: UI-Seats — Roster, Composer-Selektoren, Runden-Disziplin
  class: concept
  date: 2026-10-07
  sha256: 0e11b3793aface7ee76264089182b5a2f9704f80ad7e092a990e329be6981a34
  status: live
  see-also: AGENTS.md docs/concepts/tools-map.md state/stimmen/README.md
-->
# UI-Seats — Roster, Composer-Selektoren, Runden-Disziplin

Diese Karte ist der **Bedien-Katalog** für die UI-Stimmen: welche Seats es gibt,
wie jede geöffnet und gefüllt wird, und wie eine Stimmen-Runde läuft. Jede Linie
liest sie, bevor sie eine UI-Runde fährt. Der Roster selbst (welche Modelle, welche
Rolle) steht im AGENTS.md; diese Karte trägt das **Wie**.

## Runden-Disziplin (bindend)

- **Per-Akt:** eine Runde = ein benannter Akt (Operator-Wort); kein Daemon, kein Polling.
- **JIT:** Tabs nur für den Round öffnen (`browser_open … focus:false`), danach
  schließen (`browser_close <group>`); der Browser bekommt **nie** Fokus.
- **Eigene Gruppe:** jede Linie fährt nur ihre Gruppe (`<line>-ui`,
  Gruppenname in `state/<line>/ui-group.txt`); fremde Linien-Gruppen und die
  `1-ui`/`2-backup`-Gruppen des Operators bleiben unberührt.
- **Geteilte Tryingopen-Seats:** Gruppe `open-weight-ui`, Zugang **nur über das Lock**
  `state/zustand/ui-open-weight.lock` (Halter-Linie + Zeit setzen, nach der Runde
  löschen) — zwei Linien nie gleichzeitig. Ein Tab je Modell, kein Modellwechsel im Chat.
- **Eine Frage je Runde, terse** (Verdikt + Begründung); read-only, keine Baum-Hände;
  Antworten sind Rohmaterial, das Verdikt trägt die Session/Rat.
- **Nicht-Antwort ist `pending`:** Tageslimit / Login-Wall / Rate-Limit wird **gemessen**
  benannt, nie als stille Null gelesen.

## Feste UI-Seats

| Seat | URL | Composer | Senden | Gruppe |
|---|---|---|---|---|
| Duck.ai (GPT, primär OpenAI) | `https://duck.ai/` | `textarea[placeholder*="privat fragen"]` | Senden-Button / Enter | `<line>-ui` |
| Claude | `https://claude.ai/new` | `div[contenteditable="true"]` (ProseMirror) | Enter | `<line>-ui` |
| Qwen | `https://chat.qwen.ai/` | `textarea[placeholder*="Qwen fragen"]` | Klick `.message-input-right-button-send` (Enter füllt nur) | `<line>-ui` |
| GLM / Z.ai | `https://chat.z.ai/` | Textbox „How can I help you today?" | Senden-Button / Enter | `<line>-ui` |
| Kimi | `https://www.kimi.com/` | `div[contenteditable]` | Senden-Button | `<line>-ui` |
| MiMo V2.6 Pro (Xiaomi) | `https://agent.minimax.io/` | `div[contenteditable][placeholder*="Enter message"]` | Senden | `<line>-ui` |
| Nemotron 3 Ultra (NVIDIA) | über `open-weight-ui` (s. u.) oder NIM | — | — | geteilt |

## Neue UI-Seats (Gretchenfrage 2026-10-07)

| Seat | URL | Composer | Senden | Gruppe |
|---|---|---|---|---|
| MiniMax M3 | `https://agent.minimax.io/` | `div[contenteditable]` („Enter message…") | Senden-Button | `<line>-ui` |
| Gemini 3.1 Pro | `https://aistudio.google.com/app/prompts/new_chat` | `textarea[placeholder="Enter a prompt"]` | Run-Button (`aria-label` „Run …") | `<line>-ui` |
| DeepSeek Chat | `https://chat.deepseek.com/` | `textarea[placeholder*="Message DeepSeek"]` | Enter | `<line>-ui` |
| Mistral | `https://chat.mistral.ai/` | `div[contenteditable="true"]` | Enter | `<line>-ui` |
| Proton Lumo | `https://lumo.proton.me/` | `textarea[placeholder*="Frag alles"]` | Enter | `<line>-ui` |

## Auth-Route-Seats (Gretchenfrage gemessen 2026-10-08)

Über die **Auth-Route** (Operator-Profil-Browser bzw. Konto) erreichbar; Auth ist **kein**
Ausschlusskriterium (nur kommerziell/illegal). Probe = `state/mycelium/philosophy-probe-2026-10-05.txt`
(korrekt `PHIL: absent,null-echt,pending,nein,b,fabrication`).

| Seat | URL | Composer | Senden | Fähigkeit | Tempo |
|---|---|---|---|---|---|
| **Perplexity** | `https://www.perplexity.ai/` | `div[contenteditable]` (Cloudflare via Bridge) | Enter | **4/4** ✅ | **3 s** (Site „Recherchiert 3 Sek.") |
| **Sakana Chat** | `https://chat.sakana.ai/` | `textarea[placeholder*="Ask anything"]` | Enter | **4/4** ✅ | `pending` (kein Turn-Stempel) |
| SciSpace | `https://scispace.com/` | textarea („Research task input") | — | `pending` | **Login-Wall** (Sign-up-Dialog vor dem Chat) |
| Gemini Notebook | `https://notebooklm.google.com/` | — | — | `pending` | Dokument-Chatbot, kein freier Composer |

Perplexity und Sakana sind damit **4/4-Seats** (Perplexity Deep Research ist science-relevant);
SciSpace/Gemini Notebook bleiben Kandidaten (Auth-Route/Dokument-Bindung).

## Geteilte Open-Weight-Seats — `tryingopen.com` (`open-weight-ui`)

- URL `https://www.tryingopen.com/` · Composer `textarea[placeholder*="Ask any of these models"]`
  · Senden: Send-Button (`button[type="submit"]`) / Enter.
- **Modell-Wahl:** Button „Model: …" öffnet die Liste (22–23 offene Modelle); ein Tab je
  Modell. Gemessen 2026-10-07 vorhanden: DeepSeek V4 Pro (1.7T) · DeepSeek V4 Flash ·
  Qwen3.8 2.4T · Qwen3.8 27B · Nemotron 3 Ultra (550B) · GLM 5.3 · MiMo V2.6 Pro ·
  GPT-OSS 120B · Inkling (975B) · Mistral Large 4 … (Kimi K3 war 2026-10-07 **nicht** in
  der Liste).
- `queue`/`fill`: der Composer nimmt `browser_fill` (native value-setter) → Send-Button;
  in `contenteditable`-Seats per `browser_type` (CDP-Tippen) füllen.

## Gemessener Seat-Stand (2026-10-07, ci-check-Ratsfrage)

Erreichbar und antwortend: Qwen · GLM/Z.ai · DeepSeek V4 Pro · GPT-OSS 120B · MiMo V2.6 Pro ·
Qwen3.8 2.4T · Inkling · Nemotron 3 Ultra · Gemini 3.1 Pro · MiniMax M3 · DeepSeek Chat ·
Mistral · Proton Lumo.
Nicht erreichbar (gemessen): Duck.ai (Tageslimit) · Claude (5-h-Nachrichtenlimit) · Kimi
(`www.kimi.com` Login-Wall; K3 nicht in der tryingopen-Liste).

## Lead-Quellen für neue Seats

- **FMHY — Free Media Heck Yeah, `/ai`** (`https://fmhy.net/ai`; roher Text `https://raw.githubusercontent.com/fmhy/edit/main/docs/ai.md`) — kuratierte Riesenliste der AI-Chats/Tools/Benchmarks mit aktuellem Modell je Seat. **Lead-Index, keine Quelle** — jeder Seat wird am Baum gemessen (gemessen 2026-10-08: HTTP 200, 176673 B).
- **awesome-ai-web-search** (`https://github.com/felladrin/awesome-ai-web-search`) — kuratierte Timeline von AI-Web-Suchsoftware (OSS: Khoj · Perplexica · STORM · Open WebUI · Farfalle …; Agent-APIs: SearXNG · Tavily · Exa · Jina · Firecrawl · Linkup …). **Lead-Quelle für Suchwege**; FMHY listet sie als **ersten Eintrag** von `#specialized-chatbots`.
- **Kimi-Befund (Operator-Wort + gemessen 2026-10-08):** `https://kimi.ai/` HTTP 200, Titel „Kimi AI with K3" — **der Titel ist Marketing; auf kimi.ai läuft in der Regel nur 2.6, nicht K3** (Operator-Wort 2026-10-08). Der Chat ist login-pflichtig (Google/Telefon); FMHY: `kimi.ai`/`kimi.com` = „Google Login or Phone # Required". Login-freies K3 nur über `tryingopen` (dort 2026-10-07 nicht in der Liste).

## Aufruf-Formen (Kurz)

```
browser_open <group> <url> focus:false         # Tab öffnen (kein Fokus)
browser_query  <group> <selector> tabId:<id>   # Composer/Ref finden
browser_fill   <group> fields:[{selector,value}] tabId:<id>   # textarea füllen
browser_type   <group> ref:<ref> submit:true   # contenteditable füllen+senden
browser_click  <group> selector:<send> tabId:<id>
browser_get_text <group> tabId:<id>            # Antwort lesen (oder chrome-devtools_evaluate_script Slice)
browser_close  <group>                         # Runde schließen
```

Antwort ablegen als `state/stimmen/<TS>_<site>_<slug>.md` mit Kopf (Site, Modell, Zeit,
Tab-URL, Prompt); die Session verifiziert die Claims am Baum, nie die Stimme.

## Bedien-Lehre (gemessen 2026-10-09, Future)

Zwei Browser-Wege stehen bereit: die geforkte Bridge (`browser_*`) und der **chrome-devtools-MCP**
(`chrome-devtools_*`). Der MCP-Snapshot liefert exakte `uid`s — damit gelingt die Bedienung, an
der die Bridge oft scheitert. Die gemessenen Regeln:

- **React-Inputs brauchen Keyboard-Typing.** Der native Value-Setter (`browser_fill`/`fill`) setzt
  den Wert, triggert aber **keine** React-Events → der Senden-Button bleibt `disabled`
  (gemessen: Duck.ai, Google AI Studio, tryingopen). **Regel: `chrome-devtools_type_text` (Tastatur)
  in das fokussierte Feld, dann senden.**
- **Senden explizit klicken**, nicht nur Enter: Duck.ai „Senden" · Google AI Studio „Run"/Strg+Enter ·
  tryingopen „Send message" · Qwen `.message-input-right-button-send`.
- **Vor „pending" prüfen, ob die Frage angekommen ist:** Chat-Titel/Session-Liste (Z.ai
  „Top 2 success axes…", Perplexity-Session) belegen den Empfang; langsame Seats (Z.ai Deep Think,
  Mistral Vibe) brauchen Wartezeit, kein sofortiges `pending`.
- **Echte Login-Walls** (Kimi: WeChat/Telefon) sind ein Zugangs-Gate, kein Tempo — als
  `blocked account` benennen, nie als `pending`.
- **contenteditable-Seats** (Claude, Kimi `.chat-input-editor`, Mistral, Perplexity): erst per JS
  fokussieren, dann `type_text`.
