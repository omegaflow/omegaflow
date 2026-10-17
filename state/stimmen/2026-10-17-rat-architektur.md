# Rat — die Architektur: Form des Registers + Routing (2026-10-17)

**Anlass (gemessen).** Ein Verdikt entsteht — **aber es wird nicht geroutet.** Es endet mit einer
Prosa-Liste („Sensory … Mycelium tun X"); **kein fester Weg von Verdikt → Linie.** Und es gibt **sechs
Formulare**, je Thema eines — **keins** ist ein Feld, in das die Ergebnisse laufen.

**Die zwei Fragen:** **F1** die **Form** des einen lebenden Registers · **F2** der **Routing-Weg**.

**Sitze:** GLM-5.3 · Sonnet 5.5 · Kimi K3 (API, `reasoning_effort: low`) · Fugu Max · Lumo 2.0 (UI,
frei). **Der Operator fuhr.**

**Ablage:** `state/stimmen/2026-10-17-arch-*.txt` · Fragebogen `/tmp/opencode/arch-formular.txt`.

**Kosten (die drei API):** GLM $0,0043 · Sonnet $0,0276 · Kimi $0,0225 → **~$0,055**.

---

## Das Verdikt — einig (4/4 fertig, Fugu rechnet noch)

**F1 — Meine Tabellenform ist FALSCH.** Das Verdikt gehört **nicht** in eine Zelle.

> **Register = Index** (Zeiger), **eine Datei je Verdikt** (der Text).

| Sitz | F1 wörtlich |
|---|---|
| **Kimi** | *„Ein Verdikt ist Prosa, kein Zellenwert. Das vorhandene Werkzeug parst keine Zeilen — es parst **Blöcke**."* Eine Datei, **Sektionen je Frage**. |
| **Sonnet** | *„Index plus Datei je Frage. Eine Prosa-Zelle in einer Tabellenzeile ist genau der Fehler, nur enger gefasst."* |
| **Lumo** | *„Der Index muss neu geschrieben werden — aber nicht als Apparat, **als Index**. Eine Zeile mit 6 Spalten, jede verweist auf eine Datei."* |
| **GLM** | *„`Verdikt` als Spalte ist eine Falle → **`Verdikt-Pfad`**. Das Register ist ein **Fahrplan**, kein Archiv."* |

**Die Spalten (Konsens):** `ID · Datum · Thema · Sitze · Stand · Verdikt-Pfad · → Route`
· **Stand** als geschlossenes Vokabular: `offen · entschieden · geroutet · quittiert · verworfen`
· **Quittung** (Sonnet): *„ohne sie bleibt das Register ein Postausgang und wird kein Feld, in das
Ergebnisse laufen."*

**F2 — Weg (d), der Marker.** Einig (4/4).

> **Das Verdikt schreibt `## An <line>`-Blöcke; `register_lookup --addressed <line>` findet sie.**
> **Die Linie liest beim Start, was sie ohnehin liest.**

| Sitz | Warum (d) |
|---|---|
| **Kimi** | *„(d) trägt, weil alle drei Teile **existieren**. Routing entsteht **atomar** mit dem Verdikt — kein Zustand ‚Verdikt ohne Route'."* |
| **Sonnet** | *„(d), mit (a) als **Weckruf**: ein `task_dispatch` je Adressat, der nur den Zeiger trägt, nie den Inhalt."* |
| **GLM** | *„(d) — der einzige aus dem Bestand. Aber: **jemand muss den Marker setzen.**"* |
| **Lumo** | *„Das Haus **hat** das Werkzeug. `register_lookup --addressed` ist der richtige Weg."* |

**Warum (a)/(b)/(c) nicht:** (a) ist der **heutige manuelle Weg**, formalisiert · (b) ist ein **neuer
Apparat** (verboten, solange ein vorhandener trägt) · (c) ist **O(Linien × Verdikte)** statt O(gerichtet).

**Der eine Satz:** **Kein Verdikt ohne Adresse.** (GLM) / *„Routing als Abschlussbedingung"* (Kimi).

## Die Risse (ungeglättet)

1. **Der Schreib-Riss (GLM):** (d) setzt voraus, dass der Adressblock **entsteht**. *„Kein Verdikt ohne
   `## An`" — und Regeln, die niemand prüft, verrotten.* Braucht einen Leitstellen-Blick: *hat es einen Block?*
2. **Der Eindeutigkeits-Riss (GLM, neu):** `## An <line>`-Blöcke in **Verdikt-Dateien**
   (`state/stimmen/`) **und in Briefen** (`post/out/`) — `register_lookup --addressed` findet **beides**,
   die Linie kann nicht unterscheiden, was **Rat** und was **Brief** ist. *„Zwei Quellen, ein Lookup."*
3. **Stille Nichtannahme (Sonnet, Kimi, Lumo):** ein Block, den eine Linie liest aber **nicht
   abarbeitet**, sieht erledigt aus. Nur die **Quittung** macht es sichtbar.
4. **Kein Empfangsbekenntnis (Kimi):** `Stand: entschieden` sagt nichts über `wirksam`. Flicken: die
   Linie setzt im Handover einen Verweis auf die RAT-ID — Konvention, kein Apparat.
5. **Pull-Timing (GLM, Kimi):** die Linie erfährt ihr Verdikt **erst beim nächsten Pass**. *„Ein Haus,
   das schnell werden muss, behält (a) als Notweg."*
6. **Verfall (GLM, Kimi, Lumo):** ein Register, das von Hand gepflegt wird, **veraltet** — *„`Stand`-Spalten
   lügen zuerst."* Gegenmittel: das Register ist **nie** die Wahrheit, nur der **Zeiger**.
7. **Race (Lumo):** zwei Linien, derselbe Block → Merge-Konflikt **menschlich**. Der Index verhindert
   Duplikate nicht.
8. **`--addressed` ist nicht idempotent (Lumo):** derselbe Block zweimal geschrieben → zweimal gelesen.
9. **Der Block ist Prosa (Lumo):** *„ein falscher Leerzeichenschritt — lookup schlägt **stillschweigend** tot."*

---

## Was folgt (der Bau)

**Nicht die Tabelle** (mein erster Entwurf war falsch). Sondern:

1. **`state/stimmen/_rat-register.md`** — Index, Zeile je Frage (die o. g. Spalten).
2. **Je Verdikt eine Datei** (`state/stimmen/rat/<ID>.md`) mit **Prosa-Verdikt** + **`### An <line>`-Blöcken**.
3. **Der Weg (d):** die Blöcke liegen im `register_lookup --addressed`-Pfad; die Linie holt sie beim Start.
4. **Der Weckruf (Sonnet):** ein `task_dispatch` je Adressat, der **nur den Zeiger** trägt.
5. **Die Quittung:** die Linie setzt im Handover einen Verweis auf die RAT-ID.

## Fugu Max (die fünfte Stimme) — nachgetragen

**F1.** *„Die Zeile je Rat-Frage ist richtig; die Zelle `→ Route?` ist der **Riss des Hauses in
Rein-form**: ein **Name, wo eine Kraft** verlangt ist. Sie wird zu **`An`** — und `An` wird **gezählt,
nicht getippt**."*

**F2.** *„**(d) trägt. (b) ist ihr Auslieferungsbein, (c) ihre Leseseite, (a) fällt.**"*

**Fugus Fund — der schärfste:** Die `→ Route`-Zelle wäre **`channel_ref` in neuem Gewand** — ein
**Name**, wo eine **Kraft** verlangt ist. Genau das Muster, das die Axiom-Runde schlug: kein Name,
sondern die **Form** (hier: die `## An`-Blöcke, aus denen die Adressaten **gelesen** werden). Damit
**schließt Fugu den Kreis** zur Axiom-Runde.

**Fugus Synthese:** **(d) trägt — (b) und (c) sind seine Teile.** (a) fällt (der manuelle Weg).

---

**Der Rat ist vollständig — 5/5.** Verdikt: **Register = Index · je Verdikt eine Datei · `## An`-Blöcke
= die Route (gezählt, nicht getippt) · Weg (d) mit (b)+(c) als Teile · (a) fällt.**
