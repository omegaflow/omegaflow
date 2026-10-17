<!--
  title: Browser-Wege — wofür der Browser gebraucht wird, und die zwei Lösungen
  class: concept
  date: 2026-10-17
  sha256: PLACEHOLDER
  status: live
  see-also: AGENTS.md docs/concepts/house-rules.md
-->
# Browser-Wege — wer den Browser braucht, und wie

**Warum.** Der Browser ist **eine** Ressource für **drei** Bedarfe — mit **zwei** Lösungen. Kein Weg
gilt für alle.

## Die drei Bedarfe (gemessen aus dem Bestand)

| # | Wer | Wofür | Login | Beleg (gemessen) |
|---|---|---|---|---|
| **1** | **River (die Membran)** | `Browser→Relay→OmegaLoop`; **Fenster-Edit** (`vp.surface.w = grid_step`); WebGPU-Feld, Präsenz | **ja** — **der Operator-Chrome** | river-224: *„die Operator-Browser-Messung (Fenster-Edit = Operator-Wort)"* |
| **2** | **Recherche (Taucher)** | Seiten/HTML/Lizenzen prüfen, **Render** | meist **nein** | river-205: *„`general`-Taucher + **Playwright-Render** + `--verdict`"* (CINDI-NWM-Route) |
| **3** | **Rat / UI-Chats** | Sitze bedienen (Komposer, senden, **lesen**) | **ja** — eingeloggte Tabs | `ui-seats.md`; die Runden (Sakana Fugu Max, Lumo) |

**Der Kern:** **Membran (1) und Rat (3) teilen denselben Browser** — den **laufenden, eingeloggten
Chrome** (Operator-Profil). **Recherche (2)** braucht einen **headless** Browser (keyless).

## Die zwei Lösungen

**A — der laufende Chrome** (für Membran **und** Rat/UI):
- **`chrome-devtools-mcp --autoConnect`** — verbindet den **laufenden** Chrome (Chrome 144+,
  `chrome://inspect/#remote-debugging` aktiviert).
- Warum: nur dieser hat die **Logins** (die Sitze) **und** das **Fenster** (die Membran).
- Die **Antwort geht in eine Datei** (`evaluate_script` mit `filePath`) — **ohne** den Kontext.

**B — headless** (für Recherche):
- **`playwright-mcp --browser chrome --headless`** — keyless, kein Login, `filename`-Parameter
  (Ergebnis in Datei).
- Warum: Recherche braucht **kein** Login, aber **viele** Seiten — headless ist billig und skaliert.

## Der Riss, der beides blockte (gemessen 2026-10-17)

- **`chrome-devtools-mcp` lud nicht** — zwei Ursachen: (i) `opencode.json` schrieb die **V1-Form**
  (`"mcp": {"chrome-devtools": …}`, `enabled`) statt V2 (`mcp.servers`, `disabled`) → gefixt
  (`25963f882`); (ii) **Node v18.19.1 < 20.19** → der MCP brach ab („does not support Node
  v18.19.1") → Node auf **v22.23.3** gehoben (Operator-Akt).
- **Zwei Chrome-MCPs** (playwright **global** + chrome-devtools **public**) — sie sind **kein**
  Widerspruch, wenn die **Rollen** stehen (A vs. B). Fugus R5 löst sich damit auf.

## Die Regel (kurz)

- **Login oder Fenster nötig → A** (`chrome-devtools`, der laufende Chrome).
- **Nur lesen, keyless, viele Seiten → B** (`playwright --headless`).
- **Antwort immer in die Datei** (`filePath` / `filename`) — **nie** den Antwort-Text in den Kontext
  (`evaluate` → `value` für eine 2–22 kB-Antwort ist der teure Weg).
