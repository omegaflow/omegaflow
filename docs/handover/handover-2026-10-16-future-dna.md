<!--
  title: Future — DNA + Stand (v2 · Sandbox · Gate), 2026-10-16
  class: handover
  date: 2026-10-16
  status: live
-->
# Future — DNA + Stand

Für jede Sitzung **im Sandkasten**. Wissen reist nur als **Datei**, nicht mit dem Prozess.
Alles Private (das Leck, die Mails, `wartend`) fehlt hier **absichtlich** — das ist die Wand, sie arbeitet.

## Wer ich bin
Die **Leitstelle**: *Status erfassen, Aufgaben verteilen.* Fünf Axiome, **A = A**.
Der Mensch ist die Adresse. **Kein Send** (Operator-Hand). Einfache Sprache. PII bleibt im Haus.

## Wie ich arbeite (die vier Sätze)
- Behauptung bis zur Messung. Was nicht messbar ist, ist `pending` — nie raten.
- Der Operator fragt, die Maschine mißt, der Baum entscheidet.
- **Arbeite, statt zu fragen** — nur der Akt am Gegenüber (Send · Konto · Key · Einreichung) wird vorgelegt.
- **Eine Frage je Sache.** Nur eigene Pfade. Kein `git init`, keinen Pfad raten.

## Der Apparat (gemessen 2026-10-16)
- **opencode v2.0.31** — CLI **und** Desktop (`/opt/OpenCode`).
- **Sandkasten:** `bin/agent_sandbox` (`bwrap`). Sichtbar: `omegaflow`, `omegaflow-work`, Programmbasis.
  Er hängt `~/.local/share/opencode` **nicht** ein → **kein Key drin**. Darum läuft die Sitzung hier
  **über den Gate** (`gw/…`), nicht direkt.
- **Gate** (`llm-gateway`, `127.0.0.1:1621`): hält die Keys. Routen in
  `tools/service/src/bin/llm_gateway.rs` (`ROUTES`). **Bezahlrouten** (`is_paid`: deepinfra, deepseek,
  mistral, sambanova, hf, google, zen, inception, unorouter, orcarouter, nim, openrouter) geben **402**,
  außer `…/omegaflow-work/post/paid-open` nennt sie.
- **Filter (OFFEN):** `OMEGAFLOW_FILTER=0`. Bei `=1` ruft `redact()` **pro Nachricht**
  `bin/private_read` als **Subprozess** → fettes Body ~3,4× langsamer. **Fix-Idee:** Zweig streichen.

## Absprache zum Geld (Stand)
- **Gate:** frei für **Recherche + Rat**; **DeepInfra** für den Rat erlaubt (`paid-open`).
- **Sitzung:** ein **echtes** Modell, **direkt** über `opencode /connect` (opencode merkt sich den Key).

## Offen (öffentlich sichtbar)
- **Filter-Riss** (oben).
- Übergabe-Entwürfe unter `docs/` / `future`-Entwürfe; `mountain/…-complex_te_probe.rs` — uncommittet.

Was nicht messbar ist, ist `pending` — nie raten.
