<!--
  title: Survey — Fremdmodell-Bedienung über den chrome-devtools-MCP (Stand 2026-09-24)
  class: survey
  date: 2026-09-24
  sha256: 40b7839f65aebc06624f59ce40bbb620fd283273d2bb0e1cbdafe8c85cd69216
  status: live
  see-also: docs/paper/flyby-path-2-preregistration.md
-->
# Survey — Fremdmodell-Bedienung über den `chrome-devtools`-MCP (2026-09-24)

Zweck: wie GLM (z.ai), Claude (claude.ai) und Arena-Modelle als **unabhängige
Prüfstimme** über den MCP bedient wurden — exakte Befehle, Fähigkeiten, Grenzen.
Eine Bot-Aussage ist **Stimme, nie Messung**; lokal verifiziert.

## Konten / Modelle (gemessen)

- **z.ai** (`chat.z.ai`, angemeldet): Modelle **GLM-5.2**, **GLM-5.3**,
  **GLM-5.3-Flash**. Wahl über Button mit `aria-label="Select a model"`
  (auf einem frischen Chat aktiv, im laufenden Chat `disabled`).
- **claude.ai** (angemeldet): Modell **Sonnet 5 Hoch** (Button „Modell: …").
- **arena.ai** (Gast nutzbar): **133** Modelle, **kein Opus**; Direct war zeitweise
  gestört (nur der Router `Max` antwortete).

## Bedienprotokoll (chrome-devtools MCP)

1. `chrome-devtools_list_pages` — Tabs lesen.
2. `chrome-devtools_new_page {url, background:true}` — frischer Chat, ohne
   Fokus-Klau.
3. Composer fokussieren — `chrome-devtools_evaluate_script`:
   `const t=document.querySelector('textarea'); t.focus();`
   (z.ai: `textarea`, Platzhalter „How can I help you today?"; claude.ai:
   `[contenteditable=true]`, „Schreibe deinen Prompt an Claude").
4. Senden:
   - **z.ai:** `chrome-devtools_type_text {pageId, text, submitKey:"Enter"}`
     (echte Tasten). Langer Text reißt den Request-Timeout — die Aktion läuft
     serverseitig weiter, danach navigiert der Tab auf `/c/<uuid>`.
   - **claude.ai:** Text als **synthetischer Paste** in das `contenteditable`:
     `const dt=new DataTransfer(); dt.setData('text/plain',TEXT);
     ed.dispatchEvent(new ClipboardEvent('paste',{clipboardData:dt,bubbles:true,cancelable:true}));`
     dann `chrome-devtools_press_key {key:"Enter"}`.
5. **Anhang** (Dokument statt tippen, bis ~50 KB):
   - z.ai: `chrome-devtools_take_snapshot {pageId, filePath}` → uid der Drop-Zone
     (`generic "Up to 10 files, Max 50 MB per file"`) →
     `chrome-devtools_upload_file {pageId, uid, filePaths:[abs-pfad]}`.
   - claude.ai: Datei-Input vorhanden; `upload_file` auf den „Dateien … hinzufügen"-
     Button scheiterte (öffnet Menü) — **Volltext per Paste** ist dort der Weg.
6. **Antwort lesen:** `evaluate_script` → `document.body.innerText`; bei Claude
   gezielt die letzte `.font-claude-message`. Der Text steht oft **nach** dem
   Prompt-Ende — per `lastIndexOf(<Prompt-Schluss>)` schneiden.
7. Aufräumen: `chrome-devtools_close_page {pageId}`.

## Fähigkeiten / Grenzen (gemessen 2026-09-24)

- **GLM z.ai:** „Deep Think"-Reasoning, Antwort ~30 s bis ~3 min; GLM-5.2 in
  sauberem Deutsch, **GLM-5.3 driftet teils ins Englische**. Nimmt `type_text`
  und Anhang.
- **Claude:** schnell, knapp, Deutsch; nimmt synthetischen Paste; Anhang über
  Datei-Input.
- **Synthetic Paste:** claude.ai **akzeptiert**, z.ai **verwirft** (0 Zeichen).
  Programmatisches `value`-Setzen + `input`-Event registriert React auf **beiden**
  nicht → Nachricht wird nicht gesendet.
- **MCP:** langer `type_text` → Request-Timeout (Aktion läuft weiter);
  **Netzausfall blockiert den MCP** vollständig (Ursache der früheren Timeouts).
- **Kontext:** Anhang bis ~50 KB je Dokument.

## Consent

Jeder Prompt ist ein Schreib-Akt bei Dritten (z.ai → Zhipu, claude.ai →
Anthropic) → per-Akt-Consent; PII nur redigiert oder gar nicht; Ausgaben sind
Spuren, keine Messungen.
