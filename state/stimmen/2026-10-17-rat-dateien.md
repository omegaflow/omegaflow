# Rat — welche Datei beim START, welche bei jedem TURN (2026-10-17)

**Anlass.** Der Start-Prompt einer Linie kämmte **~150 Dateien**, statt einen Marker abzufragen. Und
niemand weiß, **welche** der vielen Regel-/Kontext-Dateien beim Start wirklich gilt.

**Die Frage:** Welche Datei beim **Start** geladen, welche bei **jedem Turn**? Welche ist **überflüssig**?

**Sitze:** GLM-5.3 · Mistral Large 4 · Fugu Max · Lumo 2.0. **Vorgelegt wörtlich:** die globale
`~/.config/opencode/AGENTS.md` (26 Z.) + `opencode.jsonc` · die `SKILL.md` · die Projekt-`AGENTS.md`
(241 B) · beide `opencode.json` · die fünf `command/<linie>.md`.

**Ablage:** `state/stimmen/2026-10-17-dateien-*.txt`.

---

## Das Verdikt — einig (4/4)

> **Zwei Regel-Ebenen, nicht drei.** Global-AGENTS (was in **jedem** Verzeichnis falsch würde) +
> Projekt-AGENTS (das Haus). **Eine Regel-Quelle je Geltungsbereich.**

**`SKILL.md` ist die dritte Regel-Trägerin — Drift.**
- **GLM:** *„Sie ist bewusst eine Kopie — Drift per Definition. Sie ist der Grund, warum der Start-Prompt den Baum kämmt."*
- **Mistral:** *„Der Skill ist ein Spiegel — gehört eliminiert oder auf ‚siehe AGENTS.md' reduziert."*
- **Fugu/Lumo:** **Zeiger/Router** — *„Wenn im omegaflow-Verzeichnis: AGENTS.md gilt. Sonst: Repo-Pfad, dort AGENTS.md lesen."*

**`command/<linie>.md` ist der Start** — trägt die **Adresse als Kommando** (nicht „Status erfassen").

## Fugus Kernbefund — die Frage war falsch gestellt

> *„Es gibt **zwei Klassen**: **harness-geladen** (die Konfiguration entscheidet per `instructions:
> [\"AGENTS.md\"]` — **kein Sitzungsakt**) und **sitzungs-gelesen** (kostet, muss adressiert sein).
> **Genau diese Verwechslung hat den Kamm erzeugt**: die Sitzung suchte, was die Konfiguration längst
> entschieden hatte."*

**Fugus Test (aus der Kostenregel):** *Streiche die Zeile — handelt der Turn falsch? → Turn-Datei.
Findet die Sitzung ihre Adresse noch? → Start-Datei. Weder noch → **keine** Regel-Datei.*

**Fugus Messung:** *„~20 Zeilen laufen jeden Turn, ~7 binden — **rund 65 % Drift**."* Und: die
**Werkzeug-Tabelle** (fd/bat/eza) wird **entlassen** — *„fd statt find ist **langsam, nicht falsch**"*.

## Die Risse (alle vier, wörtlich)

| # | Riss | Wer |
|---|---|---|
| 1 | **Tool-Tabelle doppelt** (global-AGENTS vs. SKILL) — „zwei Quellen, ein Verhalten" | GLM, Mistral |
| 2 | **Axiom doppelt** (AGENTS vs. SKILL: A=A, 0 honored) | GLM, Fugu |
| 3 | **Modell-Widerspruch** (public `deepseek/deepseek-flash` vs. privat `deepinfra/…`) — „niemand kann sagen, welche gilt" | GLM, Mistral, Fugu (R6) |
| 4 | **`small_model` vs. `small-model`** — „mindestens einer ist ein toter Schlüssel" | GLM, Mistral |
| 5 | **`instructions: ["AGENTS.md"]` privat** — zeigt ins Leere / auf eine unbekannte dritte AGENTS | GLM, Mistral, Fugu (R8) |
| 6 | **`watcher.ignore` `*.φ` vs. `**/*.φ`** — „einer sieht die Änderungen, der andere nicht" | Mistral |
| 7 | **Grep-Regel widerspricht sich** (Tabellenzeile vs. Schlusssatz) | GLM, Mistral |
| 8 | **Der `rat`-Agent fehlt privat** — *„der Rat, das eigene Instrument des Hauses, ist still abgeschaltet"* | Fugu (R4) |
| 9 | **Der Riegel auf der Tür, das Fenster offen** — *„webfetch deny + chrome-devtools-MCP — die Verweigerung ist nominal, kein Zustand"* | Fugu (R5) |
| 10 | **Eine Haus-Regel in Rollenkleidung** — *„PII bleibt im Haus" bindet jede Linie, steht aber in `command/future.md`, läuft nur bei /future* | Fugu (R7) |
| 11 | **Zwei Dateien, ein Name** — *„Das Haus schreibt „A = A" und hält zwei Dateien mit gleichem Namen; der erste Axiom-Satz verurteilt den eigenen Datei-Baum"* | Fugu (R1) |
| 12 | **Ein Gesetz im Fallback** — *„SKILL trägt „No hardcoded thresholds"; eine Sitzung ohne Skill-Load darf Schwellen hart eintragen, ohne ein Axiom zu brechen"* | Fugu (R2) |
| 13 | **`pending` bleibt gesetzt** (Lumos Ack-Klausel aus der Vorrunde fehlt) — *„der Dispatch bleibt Fire-and-Forget"* | Lumo |

## Der Bau (der Konsens)

| Datei | Wann | Trägt |
|---|---|---|
| **`omegaflow/AGENTS.md`** | **jeder Turn** | die **eine** Regel-Quelle (Axiome, Feld-Syntax, Commit; + „keine harten Schwellen" **von SKILL herauf**; `pending` **raus**) |
| **`~/.config/opencode/AGENTS.md`** | jeder Turn | **nur** Tool-Wahl — *oder* **gelöscht** (Fugu: *„sie trägt nichts"*); **ein** Verzeichnis, eine Quelle |
| **`SKILL.md`** | Start/Kontext | **Zeiger**: *„Regeln: omegaflow/AGENTS.md"* — **kein** Axiom, kein Schema, keine Hardware |
| **`command/<linie>.md`** | `/<linie>` | Rolle + die **Adresse als Kommando** (`register_lookup --addressed <linie>` …) + Stopp-Klausel |
| **`opencode.json(c)`** | Apparat | Mechanik (**kein** Regel-Text): model **mit Provider**, agents, mcp |

**Die drei Einzeiler (aus dem Bestand, alle vier):**
1. **Axiome aus `SKILL.md` löschen** (Doppelung).
2. **`omegaflow-personal/AGENTS.md` anlegen** — oder die Referenz streichen (tote Zeile).
3. **`agent line` vs. `/<future>` entscheiden** (eines fällt).

**Der eine Umzug:** eine Axiom-Zeile von `SKILL.md` in die Turn-Datei (Fugu, belegt durch R2).
**Bilanz:** *„löscht eine Datei, kürzt zwei, schreibt fünf um — von ~20 Turn-Zeilen auf sieben."*

---

## An future

**Auftrag (R-2026-10-17-i):** Setze diesen Bau um — **nach** dem Verdikt:
1. **`omegaflow/AGENTS.md`** = die eine Quelle (die sieben Zeilen: Axiome + Feld + „keine harten
   Schwellen" + PII + Arbeit; `pending` raus).
2. **`SKILL.md`** → Zeiger (eine Zeile: „Regeln: omegaflow/AGENTS.md") oder löschen (nur mit Beleg).
3. **`command/<linie>.md`** → Rolle + Adress-Kommandos + Stopp-Klausel (nicht „Status erfassen").
4. **Die drei Einzeiler** (Axiome aus SKILL, tote Referenz, `line` vs. `/<future>`).
5. **Configs:** model mit Provider; `small_model` vereinheitlichen; `watcher`-Globs gleich.
**Fertig, wenn** zwei Regel-Quellen stehen (nicht drei), `SKILL.md` ein Zeiger ist, die Commands die
Adresse tragen. **Kein stilles pending.**
