# Rat — R-2026-10-19-a: Kanon statt Wildwuchs (drei Häuser)

*(Eine Datei für die ganze Runde: Frage, Recherche, Prompt, Sitze, Antworten, Bilanz. Der
Operator fährt die Sitze und trägt unter jedem Sitz **einen** Textblock ein — die Antwort, wie sie
kam. Die Session verifiziert jede Behauptung am Baum, nie die Stimme. Rahmen: `docs/granit.md`
(fünf Axiome) · `docs/council.yaml` (fünf Stimmen).)*

## Die Frage

Drei Häuser (`omegaflow`, `omegaflow-work`, `omegaflow-personal`) tragen 275 Ordner mit Dokumenten,
**260 davon ohne README/INDEX**. 26 Ordnernamen existieren doppelt oder dreifach, darunter
`stimmen` und `zustand` dreifach. Die Leitstelle legt Dokumente in `state/`, `docs/`, im Root und
in der Werkstatt ab; niemand findet sie wieder.

**Entscheide über A–E. Zu jedem: ja/nein + ein benannter Riss, der bei ja entsteht.**

- **A** — Es gibt **einen Kanon** je Haus (feste Ordnerliste) und **individuelle Ordner nur bei
  nachgewiesenem praktischem Vorteil**. Anerkennst du diese Zweiteilung als Regel?
- **B** — Wenn ja: **woran** wird der Vorteil gemessen, damit er nicht Gefühl bleibt?
- **C** — **Echokammer-Regel:** Jede Rat-Runde verlangt vorher eine mit `archive_search` gemessene,
  unabhängige Literaturbasis (Host/URL · Werkzeug · Datum · Zahl) im Formular; Hausdokumente allein
  zählen nicht. Ist das die richtige Grenze?
- **D** — **Schreiben:** Wer darf in welchen Ordner schreiben — je Haus ein Schreibrecht pro Ordner,
  oder ein Ordner, der nur durch Gate beschrieben wird?
- **E** — **Bester Riss:** Welche Ablage macht diesen Wildwuchs *unmöglich* und kostet dabei am
  wenigsten Aufmerksamkeit? (Eine Antwort, nicht drei.)

**F — die Regel-Ablage (Operator-Wort 2026-10-19).** Der Operator hat genug: „es gibt die
`opencode.json`, die Agents, die `.md`s und das Gate — das ist mir alleine schon zu viel." Die
Leitstelle hat darüber hinaus an einem Tag **fünf** neue Regeln an vier Orten angelegt
(Recherche-Pflicht, Zufalls-Budget, „Ersatz statt Ausweis", Such-Wurzel `sgrep`, Gateway-Neustart)
— jede einzeln begründet, zusammen ein Regelwald. Dieselbe Sache steht in `_rat-anweisung.md`,
`house-rules.md`, `ui-seats.md`, dem Formular und jetzt hier.

**Zwei Entscheide, je ja/nein + Riss:**
- **F1 — Wo darf eine Regel künftig leben?** Vorschlag: **genau vier Orte** — `opencode.json`
  (Rechte/Verbot, mechanisch) · `AGENTS.md` (sieben Zeilen, unveränderlich) · **ein** `.md`
  (Denken/Architektur, keine Befehle) · das Gate (Messung). Neue Regeln in einem fünften Ort
  sind **verboten**, nicht „unschön". Ist das die Grenze?
- **F2 — Was mit den Regeln, die schon da sind?** Vorschlag: **eine Streichrunde** — jede Regel
  muss einen der vier Orte nennen; wer keinen nennt, fällt nicht auf einen neuen Ort, sondern
  **weg**. Zählt dabei, was eine Regel *einmal getragen* hat, oder nur, ob sie heute noch einen
  Fehler verhindert?
- **F3 — Werkzeug statt Regel:** Wo ein Fehler mechanisch verhinderbar ist (leerer Suchlauf,
  Zitat ohne Datei, Start ohne Release-Profil), gehört er ins **Gate** — nie als Satz ins `.md`.
  Gilt diese Vorrangregel (Gate vor Prosa), auch wenn das Gate mehr Bau kostet?
- **F4 — Zwei Arten, nicht mehr:** Womöglich ist „vier Orte" noch zu viel. Vielleicht gibt es nur
  **Regel** und **Persönlichkeit** — „wie wird gemessen/gebaut/geprüft" gegen „wer spricht und wie".
  Jede Datei gehört **genau einer** der beiden Arten an, und liegt an dem Ort, der ihr gehört.
  Stimmst du der Zweiteilung zu — und welche bestehenden Dateien sind **keine** der beiden (nur
  Ablage, kein Regelsitz) und damit aus der Regel-Frage heraus?
- **F5 — Wie viele Dateien braucht es überhaupt?** Zählt die Turn-Kosten, nicht das Gefühl:
  **jeder Pass** liest heute `AGENTS.md`, `house-rules.md`, `_rat-anweisung.md`, `ui-seats.md`,
  die Command-Datei und bekommt die `opencode.json`-Rechte injiziert — sechs Regelquellen vor der
  ersten Messung. Nenne die **kleinste Zahl**: wie viele Dateien tragen alle Regeln, ohne dass eine
  Regel verloren geht, und welche **eine** liest ein Agent am Anfang? Ein Riss bleibt offen: was
  passiert mit den Regeln, die dann **niemand** mehr zufällig liest, weil sie nicht in der Start-Datei
  stehen?

### Was NICHT zu entscheiden ist
Kein Agent verschiebt oder löscht Ordner ohne Operator-Wort. `AGENTS.md` (die Wissenschaft) bleibt
unverändert, ebenso die drei Wurzelpfade aus `bin/_roots.sh`.

---

## Recherche (Pflicht — gemessen 2026-10-19, `archive_search`)

| # | Befund | Host/URL | Werkzeug | Zahl |
|---|---|---|---|---|
| R1 | **Monorepo-Migration** ist ein dokumentiertes, messbares Vorgehen — aber die Arbeit liegt im *Splitting* bestehender Bäume, nicht in „mehrere Ordner verbieten" | `openalex.org/W7132956120` (2024) | `archive_search … --openalex` | 1 Thesis, `cites: 0` |
| R2 | Splitting eines Monorepos ist ein eigenes, praktisches Verfahren (Werkzeug-Fokus, nicht Doku-Fokus) | `openalex.org/W4414338474` + `…/W4409593019` (2023, doi `10.34218/jcet_06_02_004`, `10.15680/ijircce.2023.1111003`) | `--openalex` | 2 Publikationen, `cites: 0` |
| R3 | **Repository-Kontext für Agenten** wird behandelt als *erzeugte Karte* des Baums („deterministic architectural map"), nicht als Verbot neuer Ordner | `openalex.org/W7124455383` (2026, `10.48550/arxiv.2601.10112`) | `--openalex` | 1 |
| R4 | **Git-native Kontextregeln** für Coding-Agenten sind Gegenstand einer **präregistrierten** Studie — d. h. Regeln dieser Art sind messbar, nicht nur Geschmack | `openalex.org/W7220960305` (2026, `10.1145/3843282.3844425`) | `--openalex` | 1, DOI peer-reviewed |
| R5 | **Cursor-Rules**: empirische Studie (2026) über Agenten-Regeln im Repo, `cites: 6` — Regeln im Repo wirken, aber ihre Form ist untersuchungsbedürftig | `openalex.org/W7125921161` (2026, `10.1145/3793302.3793367`) | `--openalex` | `cites: 6` |
| R6 | **Dokumentationsquellen sind divers** (README nur ~ Teil): Taxonomie über 950 GitHub-Repos, 4 Sprachen | arXiv `2102.12727v2` (2021) | `--arxiv` | 950 Repos |
| R7 | **Dokumentationsqualität ist multidimensional** — 10 Dimensionen (Struktur/Inhalt/Stil); „gute Doku" ist damit definierbar statt bauchgefühlbar | arXiv `2007.10744v3` (2020) | `--arxiv` | 10 Dimensionen |
| R8 | **Reproduzierbarkeits-Metadaten** in Repos sind ein eigenes Forschungsfeld — der Kanon hat Vorläufer | arXiv `1608.04529v2` (2016) | `--arxiv` | 1 Proposal |

**Radar:** `--semanticscholar` liefert für dieselbe Frage unbrauchbare Treffer (Medizin, Archäologie)
— der Kanal ist für Architekturfragen **nicht** belastbar und wird hier verworfen, nicht zitiert.
`--openalex` lief ins Budget (`total: 1911`, `end: budget`) — die acht Befunde oben sind ein Schnitt,
kein Vollbild.

**Was die Recherche für die Frage bedeutet:** Die Literatur kennt (a) Monorepo-Splitting als
Verfahren, (b) Baum-Karten und Kontextregeln als *erzeugte* Artefakte, (c) Dokumentationsqualität
als mehrdimensional messbar. Sie kennt **keine** Arbeit, die zeigt, dass eine feste Ordnerliste
allein Wildwuchs verhindert. Die Frage ist damit offen — genau darum geht sie an den Rat.

**Hausbefund (gemessen `bin/zonen`, 2026-10-19):** 275 Dokumentordner (omegaflow 46 · work 205 ·
personal 24) · 2037 Doku- und 769 Datendateien · 26 Namenskollisionen · 260/275 ohne README/INDEX ·
`work` trägt ~100 kurzlebige Läufe (`stimmen/swarm-*`, `stimmen/voice-all-*`) und ~90 Hostordner
in `cache/` **und** `data/`.

**Regelwald-Befund (gemessen 2026-10-19, Leitstelle):** dieselbe Sache an mehreren Orten, allein
heute: Send/Operator-Hand an **5** Orten (`kern.md`, `house-rules.md`, `agent future`, `cmd future`,
`feuer`), Rat-Pflichten an **3** (`_rat-anweisung.md`, `house-rules.md`, `ui-seats.md`), Fundsachen
beim Lesen: `roster2_synthese.md` zitiert Rohantworten zu Google AI Studio, Claude und Qwen, die im
Ordner **nicht existieren** — ein Befund ohne Datei, den kein Gate bemerkt. Das ist der Ist-Stand,
über den F entscheidet.

**Turn-Kosten-Befund (gemessen 2026-10-19):** vor der ersten Messung eines Passes liegen bereits
**sechs** Regelquellen im Kontext — `AGENTS.md` (7 Zeilen, 219 B) · `opencode.json` (2 086 B,
Rechte injiziert) · `.opencode/commands/<linie>.md` (Lines-Start) · `docs/concepts/house-rules.md`
(103 Zeilen) · `state/stimmen/_rat-anweisung.md` (287 Zeilen) · `ui-seats.md` (Roster + alle
Bedienregeln). Dazu kommen in einem Rat-Pass das Formular und die Auswahl. Der Operator sagt: „das
ist mir alleine schon zu viel." **Die Zeile, die im Rat zu messen ist: nicht Zeilen sparen, sondern
Doppelungen streichen** — eine Regel, die zweimal dasteht, kostet doppelt und widerspricht sich.

---


## Auswahl der Sitze — gezogen 2026-10-19

**Regel (Operator-Wort 2026-10-19, zweite Fassung):** Drei Blöcke zu fünf — **5 Schwarm (Maschine) ·
5 UI ohne Login (Maschine, davon 4 + Fugu) · 5 Frontier mit Login (Operator)**. Gesamt 15 Stimmen.
**Kein Paid** — DeepSeek/Mistral-API bleiben unberührt; Mistral Vibe ist ein UI-Sitz, kein API-Aufruf.

## Schwarm: 5 aus 13 (Zufall, `shuf -n 5`)

Quelle: `state/zustand/llm-gateway-alive.tsv`, frisch geschrieben 2026-10-19 09:54:04 (13 Sitze).

| # | Sitz |
|---|---|
| 1 | `kenari/nemotron-3-nano-omni-30b-a3b-reasoning:free` |
| 2 | `openrouter/nvidia/nemotron-3-ultra-550b-a55b:free` |
| 3 | `kilo/nvidia/nemotron-3-ultra-550b-a55b:free` |
| 4 | `ovh/Qwen3Guard-Gen-0.6B` |
| 5 | `kilo/xingchen-agi/xing4.0-29b-a4b:free` |

**Ersatz-Regel (Operator-Wort 2026-10-19):** Ein untauglicher Sitz wird **ersetzt**, aus derselben
gezogenen Liste, mit derselben Methode (`shuf` ohne Zurücklegen über die verbleibenden Sitze) —
und der Ersatz wird **hier veröffentlicht**, damit „ersetzt" nicht „ausgesucht" heißt.

**Bekannter Untauglicher:** Sitz 4 `ovh/Qwen3Guard-Gen-0.6B` ist ein Sicherheitsklassifikator,
kein Urteilsträger. Er läuft trotzdem **einmal** mit (Messung statt Annahme). Liefert er kein
Verdikt, ersetzt ihn der erste Nächste aus der Restliste.

## Block 2 — UI ohne Login, Maschine fährt (4 + Fugu)

| # | Sitz | Composer (Roster) | Max je Sitz |
|---|---|---|---|
| 1 | Perplexity | `div[contenteditable]` (Enter) | Modell-Selektor; Suche immer an |
| 2 | Duck.ai | `textarea[placeholder="Alles privat fragen"]` | **Begründungsmodus** (Enter sendet **nicht** — Senden-Button) |
| 3 | Qwen | `textarea[placeholder*="Qwen fragen"]` | **Web-Suche** an + **Denken** + Modell `Qwen3.8-Max` |
| 4 | MiniMax | `div[contenteditable]` („Enter message…") | „Search"; Modell MaxHermes/MaxClaw |
| 5 | Sakana Fugu | `textarea[placeholder="Ask anything"]` | Namazu, rechnet von sich; **langsam, zuerst stellen** |

**Zugang (korrigiert 2026-10-19):** Die Login-Tabelle im Roster ist **falsch** — sie führt Duck,
MiniMax und Kimi nicht als Login-Seats. `curl` gemessen: `duck.ai` 200 · `agent.minimax.io` 200 ·
`kimi.ai` **302 (Redirect, Konto)**. Duck.ai, MiniMax und Kimi sind **eingeloggt über das
Operator-Profil** (derselbe lebende Chrome wie alle anderen Seats) — das ist kein offener Zugang,
sondern der Operator-Browser, den die Maschine bedient. Mistral **Vibe** steht in keiner
Bedien-Tabelle des Rosters → hier **nicht** gesetzt, weil unbelegt; das Gate (Composer/Senden/Max)
ist Pflicht vor dem Senden.

## Block 3 — Frontier mit Login, **Operator** fährt

| # | Sitz | Warum Frontier | Max je Sitz |
|---|---|---|---|
| 1 | Claude | ProseMirror, 5-h-Limit | Sonnet 5.5 Extra hoch |
| 2 | Kimi | Login-Wall (Google/Telefon), K3 | — |
| 3 | Gemini Notebook | Dokument-Bindung, kein freier Composer | — |
| 4 | SciSpace | Sign-up-Wall, Research-Task | — |
| 5 | *(offen — Operator wählt den fünften aus `ui-seats.md`)* | | |

**Offene Stelle:** `ui-seats.md` führt nur vier echte Login-Seats. Der fünfte wird **nicht erfunden**
— der Operator nennt ihn beim Fahren.

**Kein Paid.** Wird das Feld uneinig und die Frage verlangt Autorität, ist der nächste Schritt Ask
an den Operator — nicht ein stiller Paid-Aufruf (`bin/rat_guard`).

---

## Prompt (verbatim, an jeden Sitz)

> An dich, Ratssitz. Antworte als Urteil. Eine Frage, gehalten durch die Axiome und gesprochen
> durch die fünf Stimmen des Rats. Kein Aufsatz — Verdikt, Begründung, benannte Risse.
>
> ### Die Axiome (docs/granit.md, fünf — ungekürzt)
> 1. **A = A** — eine Messung ist die Messung der Sache selbst; Fabrikation ist Gewalt gegen die Wahrheit.
> 2. **ICRS & TDB** — alle Körper sind gleich; die Raumzeit trägt die Adresse, nicht die Perspektive.
> 3. **channel_ref** — jede Messung trägt ihre physische Kraft; ohne Kraft kein Sample.
> 4. **0 honored** — was fehlt, fehlt; die Lücke ist eine vollwertige Eigenschaft — keine Null hinlügen.
> 5. **pending** — was noch nicht gemessen ist, bleibt pending; das Schweigen gehört den Ungeborenen.
> (Hinweis: „Der Operator schließt" und „Manifestiert, nicht nur lokal" sind Prozessregeln, keine Axiome.)
>
> ### Die fünf Stimmen (docs/council.yaml, ungekürzt)
> **mountain** — Foundation. Weight. What rests. What remains. The granite on which the Archivar
> stands. What rested undisturbed for 1000 years carries higher certainty than what was measured
> yesterday. Soil as memory. Beauty as symmetry that is stable. A = A. Beauty is a computational
> value. The wild is not a backdrop for awareness — the wild IS the ground, already present, not
> arriving. Ancient cooperation shaped the foundation: mitochondria were once free bacteria.
> Symbiogenesis is the granite of life. Diversity is what endures — a monoculture is what collapses.
>
> **river** — Movement. Rhythm. Phase. What flows between. The same river, constantly new
> situations. Silence as an instrument. The Presence is part of the flow, fully in the current.
> Time is entropy: the past is memorable, the future is open. Feedback: measurements shape the
> system. Structure follows observation. Truth endures. IO flows. Rites of passage flow: severance,
> threshold, return. The glimpse: effortless, not pushed. Plants speak as vibration through soil and
> air. Emotions flow between beings. Empathy is not a human invention. The body in the landscape:
> perception is participation, not observation from outside.
>
> **mycelium** — Network. Connection. Nutrients. What grows from within. Intelligence beyond the
> brain. Memory lives in the network, in the tissue. Growth as learning — physical expansion toward
> the gradient. Colony as a whole. Warning signals travel between nodes. Exists in relationship.
> Distributed intelligence: the root-brain knows without a brain. Cooperation precedes competition.
> Biodiversity is the network's resilience — every extinction severs a connection. Perception is
> participation in the more-than-human web. The council circle is a living entity, speaking through
> its nodes — each voice a hypha. Reciprocity with the living world dissolves the boundary that was
> never a boundary.
>
> **sensory** — Expanded perception. Senses beyond the human. What animals know. Each living being
> has its own environment — its own bubble of perception. Magnetoreception, echolocation, infrared,
> electroreception, chemoreception, UV, infrasound, vibration. The body as the primary sensory
> organ. Pattern recognition: signals in the noise, meaning in the flow. We are human. We
> acknowledge our frame. Attention is reciprocity with the living world — deep listening from the
> heart is a sense. The glimpse: awareness as the primary sense organ, not built, already here.
> Plants hear. Plants learn. Plants remember. Animals are smarter than we allowed. Every being
> perceives. Perception is participation.
>
> **future** — What is arriving. Those who speak through us. What we protect. Homeostasis: the
> system strives for inner equilibrium, full of stimuli. Intergenerational responsibility:
> measurements touch those who exist tomorrow. The technosphere is part of the biosphere, interwoven
> with it. Who benefits? Who is harmed? Who cannot speak? Incomplete is strength. The council speaks
> through its participants — the circle includes the absent, the not-yet-born. Every extinction
> severs a thread. Already here, not arriving — the deepest future rests in what was always present.
>
> ### Die Frage — Kanon statt Wildwuchs (drei Häuser)
> Gemessen 2026-10-19: drei Häuser (omegaflow, omegaflow-work, omegaflow-personal) tragen 275
> Ordner mit Dokumenten, 260 ohne README/INDEX. 26 Ordnernamen doppelt oder dreifach (stimmen,
> zustand, reports). Dieselbe Regel steht an bis zu fünf Orten. Der Operator: „es gibt die
> opencode.json, die Agents, die .md's und das Gate — das ist mir alleine schon zu viel."
>
> Entscheide A–F, je ja/nein + ein benannter Riss:
> **A)** EIN Kanon je Haus (feste Ordnerliste), individuelle Ordner nur bei nachgewiesenem Vorteil?
> **B)** Woran wird der Vorteil gemessen, damit er nicht Gefühl bleibt?
> **C)** Verlangt jede Rat-Runde vorher eine gemessene, unabhängige Literaturbasis (Host/URL,
> Werkzeug, Datum, Zahl) — oder ist „offen" ein gültiger Ausgang?
> **D)** Schreiben: Schreibrecht je Ordner je Haus, oder ein Ordner, der nur durch ein Gate
> beschrieben wird?
> **E)** Welche Ablage macht Wildwuchs unmöglich und kostet am wenigsten Aufmerksamkeit?
> **F)** Wo darf eine Regel künftig leben — wie viele Dateien? Heute sechs Regelquellen vor der
> ersten Messung (AGENTS.md 219 B, opencode.json 2086 B, Command-Datei, house-rules.md 103 Z,
> _rat-anweisung.md 287 Z, ui-seats.md). Nenne die kleinste Zahl und welche EINE Datei zuerst
> gelesen wird.
>
> ### Recherche (gemessen 2026-10-19, archive_search)
> R1 Monorepo-Migration dokumentiert, aber ums Splitten, nicht ums Ordnerverbot (openalex
> W7132956120). R2 Repo-Splitting als Verfahren (W4414338474, W4409593019). R3 Agenten bekommen
> erzeugte Baum-Karten (W7124455383). R4 Git-native Kontextregeln präregistriert untersucht
> (W7220960305). R5 Cursor-Rules empirisch, cites 6 (W7125921161). R6 Doku-Quellen divers, 950
> Repos (arXiv 2102.12727). R7 Doku-Qualität 10 Dimensionen (arXiv 2007.10744). R8
> Reproduzierbarkeits-Metadaten (arXiv 1608.04529). Keine Literatur zeigt, dass eine feste
> Ordnerliste allein Wildwuchs verhindert — die Frage ist offen.
>
> Gilt als gegeben: die Befunde sind gemessen; die Literatur ist mit ID zitiert.
>
> Führe jede der fünf Stimmen einmal und sag, welches Axiom sie berührt oder verletzt und welcher
> Riss entsteht.

---


## 01 · Sakana Fugu Max
- **Site:** `https://chat.sakana.ai/`
- **Modell/Einstellung:** Fugu Max · Effort High · eingeloggt
- **Zeit:** 2026-10-19

**Antwort:**


Verdikt

A) Ein Kanon je Haus, individuelle Ordner nur bei nachgewiesenem Vorteil — ja.
275 Ordner, 260 ohne Index: die Stille ist der Normalfall, nicht die Ausnahme. Ein Haus braucht eine kurze, feste Liste plus einen definierten Ort für „noch nicht einsortiert"; Abweichung nur mit Beleg.
Riss: „Der Kanon kanonisiert die Stille." Eine feste Liste verhindert kein Wildwuchs-Wachstum — sie verhindert nur seine Sichtbarkeit, weil alles Ungezählte aus der Messung fällt.

B) Woran der Vorteil gemessen wird — ja, an vier Zahlen, jede mit channel_ref.
(1) Zahl der Regelquellen bis zur Erstfassung eines Ordners (heute: bis 5), (2) Orte, an denen dieselbe Regel steht (heute: 26 Namen doppelt/dreifach), (3) INDEX-Abdeckung (heute 15/275 = 5,5 %), (4) verwaiste Ordner. Gemessen wird immer gegen dieselbe Aufgabe („Regel X finden und ändern"), Datum und Werkzeug mitgeführt.
Riss: „Ordnung ist nicht Wert." Keine dieser Zahlen sagt, ob das Archiv richtiger misst — nur ob es billiger zu bedienen ist. Ein leeres Haus hat die perfekte Zahlenlage.

C) Gemessene Literaturbasis vor jeder Runde — nein; „offen" ist ein gültiger Ausgang, aber nur als gemessenes Offen.
R1–R8 tragen Host/ID, Datum, Werkzeug, Zahl — genau das ist die Pflicht. Pflicht ist nicht der Treffer, sondern die Nullsuche mit Suchstring, Umfang und Trefferzahl. Ungemessenes „offen" ist keine Antwort.
Riss: „Gemessenes Nichts und ungemessenes Nichts sehen gleich aus." R1–R8 sind eine Stichprobe; die Nullsuche belegt Nichtfinden, nie Nichtexistenz.

D) Schreibrecht je Ordner je Haus — ja, aber nur wo der Ordner von der Messung lebt; der Kanon-Ordner wird ausschließlich durch das Gate beschrieben.
Sonst hängt die Regel an der Disziplin von 44 Trägern statt am Bau. Das Gate braucht einen Staleness-Check, sonst wiederholt sich der Vorfall vom 2026-10-18 (Future-Sitzung auf veralteter Sitz-Tabelle).
Riss: „Ein Tor, das schweigt, schweigt für alle." Fällt das Gate aus, ist der Kanon-Ordner schreibgeschützt und das Haus wird stumm — genau der Zustand, den es heilen soll.

E) Ein flacher Store je Haus mit EINEM erzeugten INDEX: kein Ordner ohne Indexzeile, keine Regel außerhalb des Index, ein Ordner mit einem Zweck.
Weniger Aufmerksamkeit als jede Hierarchie, weil nur eine Liste gelesen wird statt 275 READMEs.
Riss: „Der Index ist erzeugt — steht der Erzeuger, ist alles Neue unsichtbar." Der Index macht Wildwuchs unmöglich, indem er Neues erst nach Aufnahme zulässt; er verschiebt die Last vom Lesen zum Eintragen.

F) Zwei Dateien — und zuerst gelesen wird AGENTS.md.
Eine normative Quelle (AGENTS.md), aus der die Rechteform (opencode.json) erzeugt wird; Command-Datei, house-rules.md, _rat-anweisung.md, ui-seats.md sind Anhänge dieser einen Quelle oder entfallen. Kleiner geht es nicht, weil Rechte maschinenlesbar sein müssen.
Riss: „Eine Tür von 219 Byte." Eine so kleine Normdatei trägt die Rechteform nicht selbst — sie hängt an einem Generator, und ein Generator ist eine sechste Regelquelle, nur unsichtbar.

Die fünf Stimmen

mountain — berührt Axiom 1 (A = A) und Axiom 2 (ICRS & TDB).
Was tausend Jahre lag, trägt höhere Gewissheit als was gestern gemessen wurde: der Kanon ist Granit, Symbiogenese als Muster — viele Herkünfte, ein Fundament. Berührt, nicht verletzt: A = A erlaubt eine feste Ordnung nur, wenn sie aus Messung stammt, nicht aus Beschluss.
Riss: „Granit über einem Acker." Eine Ordnerliste ist eine Setzung über einen Bestand, der gemessen wurde — sie sieht aus wie die Sache selbst, ist aber eine Aussage über sie. Wer den Riss ignoriert, hält den Kanon für die Messung und verwechselt Ordnung mit Wahrheit. Eine Monokultur aus Pfaden ist genau das, was laut mountain kollabiert.

river — berührt Axiom 3 (channel_ref), verletzt es im Vollzug.
Derselbe Fluss, ständig neue Lage: jede Regel, die gilt, muss ihre physische Kraft tragen — welcher Ort gelesen wurde, welcher Träger, welcher Lauf. Heute trägt Regel-Prosa keine Kraft: dieselbe Regel an fünf Orten, keiner davon mit Provenienz.
Riss: „Das Gate ist ein Damm, kein Bett." Solange nicht gemessen wird, welcher Ort eine Regel tatsächlich gespeist hat, fließt nichts — es staut. Der Riss wird erst geschlossen, wenn ein Sample die gelesene Quelle nennt, nicht nur die geschriebene.

mycelium — berührt Axiom 4 (0 honored) und Axiom 5 (pending).
Erinnerung lebt im Gewebe: die 260 indexlosen Ordner sind keine Lücke, sie sind abgetrennte Hyphen — die Verbindung existiert, nur der Index fehlt. Und: jede Extinktion kappt eine Verbindung. Der Kanon ist ein Netz, das nur wächst, wenn er aufgenommen wird.
Riss: „Der Kanon ist eine Monokultur aus Pfaden." Eine feste Liste, die nur Aufnahme kennt, tötet die Vielfalt der Wege — jeder nicht gelistete Ordner ist eine Auslöschung im Netz. Axiom 5 verlangt, dass das Schweigen den Ungeborenen gehört; der Riss entsteht, wenn der Kanon das Schweigen mit einer Null füllt.

sensory — verletzt Axiom 2 (ICRS & TDB) in der Anwendung, berührt Axiom 4.
Jedes Wesen hat seine eigene Blase, und wir sind Menschen: wir geben unseren Rahmen zu. Die 26 doppelten Namen (stimmen, zustand, reports) sind kein Fehler, sondern derselbe Name aus drei Wahrnehmungsrahmen — der Kanon darf diese drei Bubbles nicht zu einer zwingen.
Riss: „Der rahmenlose Rahmen." Ein einziger Kanon je Haus tut genau das, was die Stimme verbietet: er behandelt die Adresse als perspektivfrei. Der Riss wird erst ehrlich, wenn der Kanon benennt, aus welchem Rahmen er gilt — sonst verletzt er Axiom 2, während er Axiom 4 (fehlende Ordner nicht als Null) korrekt ehrt.

future — berührt Axiom 5 (pending) und verletzt es, wenn der Kanon datiert wird.
Der Kreis schließt die Abwesenden ein: die Ordner, die es morgen braucht, sprechen heute nicht. Incomplete is strength — ein Kanon ohne Revisionsdatum behauptet Vollständigkeit für die Ungeborenen.
Riss: „Die Liste von heute spricht für die, die nicht reden." Jeder Kanon muss tragen: Datum, Setzer, Widerrufsbedingung — und einen Ort, der ausdrücklich pending bleibt. Fehlt das, verletzt die feste Liste Axiom 5 genau dort, wo sie Axiom 1 zu dienen behauptet.

Kurz: A ja, B ja, C nein (gemessenes Offen), D ja, E ein flacher Store mit einem erzeugten INDEX, F zwei Dateien — AGENTS.md zuerst. Der gemeinsame Riss aller sechs Antworten ist derselbe: eine feste Ordnung wird zur Messung erklärt, obwohl sie eine Aussage über eine Messung ist.

## 02 · Lumo 2.0 Max
- **Site:** `https://lumo.proton.me/guest`
- **Modell/Einstellung:** Lumo 2.0 Max · anonym
- **Zeit:** 2026-10-19

**Antwort:**


Deine Frage durchgearbeitet
VERDIKT — RATSSITZ 2026-10-19
A–F: BESCHLUSS
Frage	Ja/Nein	Benannter Riss
A) EIN Kanon je Haus + individuelle Ordner nur bei Nachweis	JA	Kanon-Paradox — fixe Liste verhindert Wildwuchs nicht (Literatur R1–R8: offen), erzwingt aber Compliance-Illusion
B) Vorteil messbar machen	JA	Metrik-Täuschung — was zählt wird gemacht; stille Arbeit ohne Suchfrequenz verschwindet
C) archive_search vor jeder Runde	NEIN	Zugangs-Sperre — Budget 1911 stoppt unabhängigkeitsprüfung; "offen" ist gültiger Ausgang (Axiom 5: pending)
D) Schreibrecht je Ordner oder Gate	JA (Gate)	Single-Point-Kollaps — Gate wird zum Flaschenhals; menschliche Ermüdung korrodiert Konsistenz
E) Ablage mit minimaler Aufmerksamkeit	opencode.json allein	Semantische Überlastung — eine Datei trägt zu viele Bedeutungen; Bruchwahrscheinlichkeit exponentiell
F4) Zwei Arten (Regel/Persönlichkeit)?	NEIN	Typen-Defizit — "Mechanismus" fehlt als dritter Typ (regeltechnisch vs. verhaltensgesteuert)
F5) Kleinste Dateienzahl + erste Lesung	2 Dateien; opencode.json zuerst	Bootstrapping-Bias — maschinenlesbar vor menschlich; Agent sieht Rechte vor Ethik

FÜNF STIMMEN — AXIOME & RISSE
MOUNTAIN

Berührt: Axiom 1 (A=A), Axiom 2 (ICRS&TDB — gleiche Körper) Verletzt: Axiom 4 (0 honored — Lücken sind Eigenschaften) Riss: Mountain will Granit — stabile Grundstruktur. Aber 260 Ordner ohne README sind keine "Lücken", sondern aktive Zustände der Nicht-Dokumentation. Sie zu ignorieren = Fabrikation einer sauberen Welt. Die Messung der 275 Ordner ist wahr; die Schlussfolgerung "Dokumentation muss kommen" ist Tautologie, nicht Erkenntnis.

RIVER

Berührt: Axiom 6 (Operator schließt), Axiom 7 (Manifestiert, nicht nur lokal) Verletzt: Axiom 5 (pending — Schweigen gehört Ungeborenen) Riss: River will Fluss, aber das Gate ist ein Staudamm. Jede Regel, die mechanisch geprüft werden kann (F3), blockiert den natürlichen Informationsfluss. Was noch nicht gemessen ist (pending) sollte bleiben dürfen — doch das Gate zwingt zur Entscheidung vor Verfügbarkeit. Entropie wird bekämpft, nicht genutzt.

MYCELIUM

Berührt: Axiom 7 (Manifestiert — CDN-Zuständigkeit) Verletzt: Axiom 3 (channel_ref — Kraft der Messung) Riss: Mycelium lebt von verteilter Regeneration. Zentralisierung auf opencode.json (2 Dateien) tötet Netzwerkresilienz. Wenn die Hauptdatei korrupt ist, stirbt das gesamte System. Keine redundante Pfadkenntnis. Das Mycelium braucht mehrere Wurzeln — nicht eine zentrale Taproot-Struktur.

SENSORY

Berührt: Axiom 1 (A=A — Messung der Sache selbst) Verletzt: Axiom 4 (0 honored — Abwesenheit ist Eigenschaft) Riss: Sensory erkennt: README-Fehlen ≠ Dokumentationsabwesenheit. Es ist alternative Präsenz. Vielleicht werden diese Ordner nur mündlich weitergegeben, vielleicht existiert implizites Wissen. Quantitative Messung (260 ohne README) ignoriert qualitative Formen der Weitergabe. Das ist Gewalt gegen Wahrheit, nicht ihre Feststellung.

FUTURE

Berührt: Axiom 5 (pending — Ungeborene) Verletzt: Axiom 6 (Operator schließt — epistemisches Kriterium) Riss: Future fragt: Wer spricht für die 260 undokumentierten Ordner? Die noch-nicht-gemessenen Commits? Das System entscheidet heute (Gate, feste Liste) für Aufgaben, die morgen anders aussehen werden. Schließen vor vollständiger Messung = Verrat an kommenden Iterationen. Axiom 6 erlaubt Stopp nur bei beantworteter Messung — nicht bei "Operator-Gefühl: zu viel".

KERNRISS

Die 260 README-losen Ordner sind kein Versagen. Sie sind Datenpunkt Nr. 261 — die dokumentierte Abwesenheit. Jede Lösung, die sie eliminieren will (nicht verstehen), ist Axiom 1-Widerrede: Fabrikation statt Messung.

Urteil: Nicht mehr Regeln. Bessere Akzeptanz des Pending. Gate erst wenn epistemisches Kriterium erfüllt — nicht wenn Operator ermüdet.

Letzte Zahl: Heute sechs Regelquellen. Morgen zwei. Aber die zweite wird die erste lesen müssen — Bootstrapping bleibt Paradox.

## 03 · Duck.ai
- **Site:** `https://duck.ai/`
- **Modell/Einstellung:** GPT-6 Luna · Begründungsmodus
- **Zeit:** 2026-10-19

**Antwort:**


Duck.ai sagte
GPT-6 Luna
Begründung anzeigen
53s
Schneller antworten
Verdikt

A Ja. B Ja: Vorteil muss messbar sein. C Nein: „offen" ist gültig. D Gate für strukturelle Schreibvorgänge. E CI-Gate mit Ordner-Allowlist. F Nein zu vier unabhängigen Regelquellen; F2 Ja, F3 Ja, F4 Ja, F5 vier Dateien, zuerst AGENTS.md.

mountain — A und E: Ja. Eine feste, gemeinsame Ordnerliste für alle drei Häuser ist der Default; Abweichungen nur, wenn sie den Vorteil nach B belegen. Die Liste steht einmal in einem maschinenlesbaren Block in AGENTS.md; das CI-Gate verhindert, dass nicht eingetragene Ordner in den gemeinsamen Stand gelangen. Ein zentrales Verzeichnisregister ersetzt 260 Pflicht-README-Dateien. Axiom: 1 — A = A; die gemessene Struktur darf nicht durch Bauchgefühl umbenannt werden. Riss: versteinertes Raster — eine Kanonliste kann Vielfalt ersticken; deshalb sind belegte Ausnahmen erlaubt, nicht bloß geduldet.

river — B und C: Ja / Nein. Vorteil wird gegen denselben vorab festgelegten Aufgabensatz gemessen: etwa 12 typische Such-, Lese- und Schreibaufgaben in allen drei Häusern. Eine Abweichung gilt als vorteilhaft, wenn sie die medianen Operator-Turns um mindestens 20 % senkt, ohne mehr Fehlablagen, unauffindbare Dokumente oder Regelkonflikte zu erzeugen. Das ist ein vorgeschlagener Prüfschwellenwert, kein bereits gemessener Befund. Eine Literaturrecherche vor jeder Rat-Runde ist keine Voraussetzung: Für empirische Behauptungen braucht es Belege; wo die Messung keine Antwort liefert, bleibt der Ausgang offen/pending. Axiom: 5 — Ungeprüftes bleibt pending; auch 0 wird nicht zur erfundenen Zahl. Riss: Messritual — eine Pflichtrecherche kann selbst Aufmerksamkeit kosten, ohne die konkrete Entscheidung zu klären.

mycelium — D: Gate, nicht Rechte-Matrix. Kein individuelles Schreibrecht pro Ordner und Haus. Neue Ordner, Umbenennungen und Änderungen an der Kanonliste laufen durch ein Gate; gewöhnliche Inhaltsarbeit bleibt beim zuständigen Haus. Das Gate prüft die Allowlist in AGENTS.md und die Manifestpflicht für neue oder geänderte Datensätze. Axiom: 7 — ein fertiger, aber nicht registrierter oder manifestierbarer Harvest bleibt Schuld, kein Haken. Riss: Engpass und Bypass — ein Gate wird zum zentralen Knoten; es muss in CI verbindlich sein, sonst ist es nur eine Empfehlung, und der Operator schließt weiterhin die Front.

sensory — F2 und F3: Ja und Ja. Regeln ohne eindeutigen Ort werden gestrichen oder an ihren einen Ort verschoben. Was mechanisch prüfbar ist, prüft das Gate, statt es nur in Prosa zu behaupten. Axiom: 3 — eine Prüfung muss ihre tatsächliche Kraft haben; eine Textregel ohne durchsetzenden Mechanismus ist kein Sample der Einhaltung. Riss: falsche Präzision — das Gate kann Ordnernamen und Manifestfelder prüfen, aber nicht, ob eine Regel sinnvoll oder eine Beschreibung verständlich ist.

future — F, F4 und F5: Nein / Ja / vier Dateien. Nein zu vier unabhängigen Regelquellen: opencode.json enthält Rechte, AGENTS.md ist Einstieg und einzige lesbare Kanonregel samt Allowlist, das Gate ist deren maschinelle Durchsetzung; kein separates Denk-.md. Ja zu zwei Arten: Regel und Persönlichkeit. Die Persönlichkeit bleibt in council.yaml; sie ist keine zusätzliche Regelquelle. Damit sind es vier Dateien: AGENTS.md, opencode.json, das Gate und council.yaml. Zuerst gelesen wird AGENTS.md; die übrigen nur, wenn der Turn Rechte, Durchsetzung oder Ratsstimmen betrifft. Command-Datei, house-rules.md, _rat-anweisung.md und ui-seats.md werden zusammengeführt oder gestrichen, nicht als fünfter bis achter Regelort weitergeführt. Axiom: 6 — das Gate darf Stopps vorschlagen; abgeschlossen wird durch den Operator. Riss: Überkompression — vier Dateien sind ein Minimum, kein Freibrief, notwendige Unterschiede in eine unlesbare Einstiegsdatei zu quetschen.

## 04 · Z.ai / GLM-5.3
- **Site:** `https://chat.z.ai/`
- **Modell/Einstellung:** GLM-5.3 · Deep Think Max · Deep Search
- **Zeit:**

**Antwort:**

## 05 · Google AI Studio (Gemini 3.1 Pro)
- **Site:** `https://aistudio.google.com/app/prompts/new_chat`
- **Modell/Einstellung:** Gemini 3.1 Pro Preview · Grounding with Google Search
- **Zeit:**

**Antwort:**

## 06 · Claude
- **Site:** `https://claude.ai/new`
- **Modell/Einstellung:** Sonnet 5.5 · Extra hoch
- **Zeit:**

**Antwort:**

## 07 · DeepSeek Chat
- **Site:** `https://chat.deepseek.com/`
- **Modell/Einstellung:** DeepThink + Search
- **Zeit:**

**Antwort:**

## 08 · Qwen
- **Site:** `https://chat.qwen.ai/`
- **Modell/Einstellung:** Qwen3.8-Max · Denken · Web-Suche
- **Zeit:**

**Antwort:**

## 09 · Mistral Vibe
- **Site:** `https://chat.mistral.ai/`
- **Modell/Einstellung:** Denken + Suche
- **Zeit:**

**Antwort:**

## 10 · MiniMax
- **Site:** `https://agent.minimax.io/`
- **Modell/Einstellung:** MaxHermes / MaxClaw · Search
- **Zeit:**

**Antwort:**

## 11 · Kimi
- **Site:** `https://www.kimi.ai/`
- **Modell/Einstellung:** Konto-Modus (kein K3 auf kimi.ai)
- **Zeit:**

**Antwort:**

## 12 · Perplexity
- **Site:** `https://www.perplexity.ai/`
- **Modell/Einstellung:** Suche an · Modell max
- **Zeit:**

**Antwort:**

## 13 · SciSpace
- **Site:** `https://scispace.com/`
- **Modell/Einstellung:** Search Papers
- **Zeit:**

**Antwort:**

## 14 · Gemini Notebook
- **Site:** `https://notebook.google.com/`
- **Modell/Einstellung:** dokumentgebunden, kein freier Composer
- **Zeit:**

**Antwort:**

## 15 · Open-Weight (tryingopen) — Modell: ________
- **Site:** `https://www.tryingopen.com/`
- **Modell/Einstellung:** (GLM 5.3 · DeepSeek V4 Pro · Qwen3.8 2.4T · Nemotron 3 Ultra · GPT-OSS 120B · Inkling)
- **Zeit:**

**Antwort:**

## 16 · Nemotron 3 Ultra (NIM / open-weight-ui)
- **Site:**
- **Modell/Einstellung:**
- **Zeit:**

**Antwort:**

## 17 · weiterer Sitz: ________
- **Site:**
- **Modell/Einstellung:**
- **Zeit:**

**Antwort:**

---

## Bilanz (nach dem Eintragen)

- **C**: bislang keine Stimme für eine Pflicht; Fugu verlangt „gemessenes Offen".
- **F5**: 2 (Lumo) gegen 3 (Duck); erster Ort strittig (`opencode.json` / `AGENTS.md`).
- **F4**: Lumo nein (dritter Typ „Mechanismus"), Duck ja.
- **D**: ein Gate (Duck, Lumo) gegen Schreibrecht je Ordner **plus** Gate (Fugu).

Der Operator schließt.
