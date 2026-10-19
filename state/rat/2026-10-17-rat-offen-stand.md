# Rat — offen-stand-rundfile.md

*Eine Datei je Runde. Die Teile stehen unverändert unter ihrer Herkunft; nichts zusammengefasst.*

---

## 2026-10-17-offen-stand-formular.md

# Rat-Frage R-2026-10-17-i — Was zählt der Offen-Stand?

**Die Frage in einem Satz:** Wenn das Gate „offene Punkte" zählt und die Burn-Zeile `closed <n>` bilanziert — was soll gezählt werden, damit die Zahl die *Tat* misst und nicht eine leere Menge?

## Die gemessene Lage (2026-10-17)

1. **Der Zähler liest nur `### `.** `count_open_points` (`tools/gate/src/bin/commit_check.rs:188`) zählt ausschließlich `### `-Überschriften unter `## Offen*`. Die lebenden Handover führen „Offen — eigen" als `- `-Bullets → der Zähler liest **0**.
2. **Folge:** Die Burn-Zeile `closed <n>` bilanziert eine leere Menge; die Prüfung „a pass closes at least one point" sieht `0 → 0`. Beleg: ein Commit, der real einen CI-Rot-Kern und 5 Orphan-Docs räumte, meldete nur `the open did not shrink (0)`.
3. **Migrationsfalle:** Wandelt man die vier Bullets in `### name` um, springt der Zähler `0 → 4` → hartes „the open grew (0 -> 4)". Das Gate sperrt das richtige Format.
4. **Kappe vs. Pfad:** Die 1-KiB-Kappe und die Pflicht „jeder offene Punkt nennt seine Datei(en)" vertragen sich nicht, wenn fünf Survey-Pfade in einen Punkt müssen. (Nachgemessen: vier Bullets + fünf Pfade = 970 B, Kappe 1024 → passt; der frühere 1024-B-Stand kam von **geschlossenem Transport**, nicht von den offenen Punkten.)
5. **Die dritte Größe fehlt:** Drei Transport-Token wurden in einem Pass belegt geschlossen; `count_open_points` sieht davon **nichts**. Die Burn-Zeile musste `closed 0` sagen, sonst failt `commit_check` mit „the Burn claims `closed 3`, the diff shows `closed 0`".

## Die Fragen

1. Soll `count_open_points` auch `- `-Bullets unter `## Offen*` zählen (wie die lebenden Handover sie schreiben) — oder müssen die Handover auf `### name` migriert werden?
2. Wie ist der Henne/Ei-Konflikt der Migration aufzulösen — die Umstellung *ist* Wachstum und failt am Gate?
3. Gilt die 1-KiB-Kappe gegen den Pfad-Zwang — oder ist die Kappe zu klein/falsch dimensioniert?
4. Soll `closed <n>` auch **Ledger-Schließungen** (Transport-Token) zählen — oder bleibt `closed` ausschließlich die `###`-Punkt-Differenz und die Transport-Bilanz lebt getrennt?

## Die Optionen (für 1 und 4; 2/3 folgen daraus)

- **A** — Zähler erweitern: `- `-Bullets zählen, `closed` zählt auch Ledger-Schließungen. Die Zahl misst die echte offene Menge.
- **B** — Migration + Ausnahme: Handover auf `### name`, der Migrations-Commit einmalig von „no growth" ausgenommen (Operator-/Rat-Wort).
- **C** — Status quo benennen: `###`-only bleibt; `closed 0` ist dann ehrlich, aber das Gate misst dauerhaft eine leere Menge.

Bitte: **Verdikt + der stärkste Riss, den du siehst.** Keine Umsetzung — die ist Folge, nicht Frage.


---

## 2026-10-17-rat-offen-stand.md

# Rat — R-2026-10-17-i: Was zählt der Offen-Stand?

**Frage:** `state/rat/2026-10-17-offen-stand-formular.md`
**Gefahren:** 2026-10-17 — API-Sitze (Subagenten, Modell direkt); UI-Sitze (Fugu · Lumo) **ausstehend**.
**Kosten:** GLM-5.3 (`#low`) + Mistral Large 4 (`#none`) — Cap 0,15 nicht berührt.

## Die Antworten

**GLM-5.3** (`deepinfra/zai-org/GLM-5.3#low`, `ses_eb44bf1daffe3oMZ258vBkqtbz`):
> Zähler soll `- `-Bullets unter `## Offen*` ebenso zählen wie `### `. Ein Punkt ist ein Punkt, egal welche Zeilenform ihn trägt. Die Migrationsfalle kollabiert dann von selbst. Die 1-KiB-Kappe ist gegen den Pfad-Zwang **falsch dimensioniert** — 970 B von 1024 B ist Zufall, kein Design; **Kappe anheben** oder am Pfad-Zwang ausrichten. `closed <n>` soll auch Ledger-Schließungen zählen.
> **VERDIKT: Option A** — Zähler und Burn auf beide Formate und Ledger-Schließungen erweitern.
> **STÄRKSTER RISS:** Der Gate-Zähler liest eine leere Menge und zwingt ehrliche Commits zu `closed 0` — Messinstrument und Realität sind dauerhaft entkoppelt.

**Mistral Large 4** (`mistral/mistral-large-4#none`, `ses_eb44bf1c2ffec5h40Iox8IPfnl`):
> Zu 1: Bullets zählen — die Handover leben als Bullets, das ist das echte Format; Migration wäre Kosmetik auf Kosten der Wahrheit. Zu 2: Henne/Ei löst sich von selbst, sobald Bullets zählen. Zu 3: **Die Kappe ist korrekt dimensioniert** — nachgemessen passen vier Bullets + fünf Pfade in 970 B; sie zwingt zur Bündelung, nicht zur Auslassung. Zu 4: `closed` zählt auch Ledger-Schließungen, sauber getrennt ausgewiesen, in einer Summe die die Tat misst.
> **VERDIKT: Option A** — Zähler auf Bullets erweitern, `closed` um Ledger-Schließungen ergänzen, **Kappe bleibt**.
> **STÄRKSTER RISS:** `closed 0` bei realer Räumung — die Burn-Zeile bilanziert eine leere Menge und täuscht den Commit-Check.

## Bilanz

**2/2 API-Sitze: Option A.** Kein Widerspruch im Verdikt. **UI-Sitze ausstehend** (Fugu · Lumo) — sie können überstimmen (Präzedenz: M1, MODVOLC).

## Der Riss (nicht geglättet)

Auf **Frage 3** widersprechen sich die beiden Sitze, obwohl beide A sagen:
- **GLM:** die Kappe ist falsch dimensioniert → anheben.
- **Mistral:** die Kappe ist richtig → sie erzwingt Bündelung.

Das ist die einzige echte Unstimmigkeit. Sie betrifft nicht das Verdikt (beide A), sondern die **Konsequenz**: ob die Kappe nach der Zähler-Erweiterung angefasst wird oder unangetastet bleibt.

**Lumo 2.0 Max** (`lumo.proton.me/guest`, UI, 2026-10-17):
> Der Zähler misst Syntax, nicht Tat. Der Konflikt löst sich nicht durch Reihenfolge, sondern durch **Vereinigung**: der Zähler muss beide Formate lesen (dedupliziert), dann ist Migration ein Nullsummenspiel und braucht keine Ausnahme. Zur Kappe: 970 B gegen 1024 B passt, aber die Marge ist 54 B — die Kappe ist heute richtig dimensioniert und morgen falsch konstruiert. **Die Pflicht „nenne deine Datei(en)" gehört nicht in dieselbe Kappe wie der Prosatext; Pfade sind Belege, keine Prosa.** `closed <n>` darf Ledger-Schließungen zählen, aber beide Mengen müssen **sichtbar getrennt** sein — ein Skalar über beide vergleicht Ungleiches.
> **VERDIKT: A** — Union-Lesung, `closed` um Ledger-Schließungen, aber Punkte und Transport-Token getrennt ausweisen; Kappe auf Prosa begrenzen, Pfade ausnehmen.
> **STÄRKSTER RISS:** Die Union tauscht eine falsche Null gegen eine unerzwungene Pflicht — Bullets tragen keine Datei-Nennung, also wird aus unsichtbarer Schuld sichtbare Schuld ohne Beleg; und der eine Skalar verdeckt, dass zwei Ledger gezählt werden.

**Fugu Max High** (`chat.sakana.ai`, UI, 2026-10-17):
> Bleibt die Kappe starr, ist die Regel schlicht überbeschränkt und **muss angehoben werden, nicht der Pfad-Zwang gestrichen**. Ledger-Schließungen sind Taten; `closed 0` gegen einen Diff, der real 3 schließt, ist die Bilanz, die die Tat verbirgt.
> **VERDIKT: A** — Bullets und `###` zählen; `closed` zusätzlich Ledger-Schließungen; **Kappe an die Punktmenge koppeln, nicht an die Pfadliste**.
> **STÄRKSTER RISS:** Toleriert der Zähler beide Formate, verschwindet der Migrationsdruck — und damit der Zwang zu einem Format.

## Bilanz

**4 von 4 Sitzen: Option A** (GLM · Mistral · Lumo · Fugu). Kein Widerspruch im Verdikt.

## Der Riss (nicht geglättet)

Auf **Frage 3 (Kappe)** stehen **3:1**:
- **GLM · Lumo · Fugu:** die Kappe ist falsch dimensioniert bzw. am falschen Gegenstand — **Pfade ausnehmen / Kappe an die Punktmenge koppeln**.
- **Mistral:** die Kappe ist **richtig** — sie erzwingt Bündelung, nicht Auslassung.

Das Verdikt (A) ist einstimmig; die **Konsequenz an der Kappe** ist es nicht. Wer sie umbaut, folgt der Mehrheit, nicht dem Konsens.

## Stand

**entschieden** (2026-10-17). Adressat gezählt aus dem Verdikt — der Zähler ist Gate-Code, sein Bau liegt bei der Linie, die ihn vermisst hat.

## An mycelium

Origin: rat-2026-10-17-i
- Zähler-Erweiterung: `count_open_points` liest `- `-Bullets **und** `### ` (Union, dedupliziert); `closed <n>` zählt zusätzlich Ledger-Schließungen — **getrennt ausgewiesen**, nicht als ein Skalar; Kappe an die Punktmenge koppeln, Pfade ausnehmen (3:1-Mehrheit; Mistrals Gegenposition benannt).
- Beleg: dieser Verdikt-Abschnitt + `2026-10-17-offen-stand-formular.md`.
- Grenze: der Bau ist Folge, nicht Verdikt; `commit_check` ist Gate-Code und wird nicht still umgebaut.


## Commits dieser Runde

- `fb61725cf` stimmen: three Rat rounds (i open-state counter, j K identifiability, k native surface) + the gitignore rule that was only prose
