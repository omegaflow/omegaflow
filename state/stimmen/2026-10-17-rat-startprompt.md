# Rat — der Start-Prompt einer Linie (Marker statt Kämmen), 2026-10-17

**Anlass (gemessen).** Der Start-Prompt (`command/future.md`) sagt *„Status erfassen (Post, vier
Linien-Handover)"* — **Prosa, keine Marker**. Folge: die Sitzung **kämmt** ~150 Dateien (`ls`, `cat`,
`head`), statt einen Marker abzufragen.

**Sitze:** GLM-5.3 ($0,0021) · Mistral Large 4 ($0,004) · Fugu Max (frei, **eingeloggt**) ·
Lumo 2.0 (frei). **≈ $0,006.** **Opus/Kimi/Sonnet fielen** (3–13× teurer, kein Mehr-Tragen in der KPI).

**Ablage:** `state/stimmen/2026-10-17-startprompt-*.txt`.

---

## Das Verdikt — einig: der Prompt **beschreibt**, er **zeigt** nicht

> **Der Prompt ist zu beschreibend, nicht zu kurz. Er muss ZEIGEN, nicht ERKLÄREN.** (GLM/Mistral)
> **„Schuldig — nicht das Modell kämmt, der Prompt lässt es kämmen."** (Lumo)
> **„Status/Post/Handover sind keine Aufgaben, sie sind ADRESSEN. Der Umbau ist eine Übersetzung,
> keine Erfindung: jedes Prosa-Nomen → ein Befehl."** (Fugu)

**F1 — die Zeilen (der Konsens):**

```
register_lookup --addressed $LINE
register_lookup --fired $LINE
xfer list --line $LINE --open
ls post/in/*-an-$LINE*.md
bin/task_dispatch <linie> "<aufgabe>" <stelle>      ← der vierte Akt (Fugu)
```

**Die fünfte, entscheidende Zeile — die STOPP-Klausel (Lumo):**
```
# Nur Treffer dieser Zeilen lesen. Kein ls bin, kein cat außerhalb der Treffer.
```
> *„Das ist keine Prosa, das ist die **Negativdefinition des Suchraums**. Der Standard-Fallback einer
> LLM ohne Suchraum ist **der ganze Baum**."*

**Fugu zur Länge:** *„Unter fünf Zeilen fehlt eine Quelle (dann wird gekämmt), über fünf trägt man
Axiome oder Prosa (dann zahlt man ohne Kraft). **Fünf ist hier Boden und Decke zugleich.**"* Und: die
**vier Linien-Namen wörtlich** in den Prompt (nicht „vier Linien" — *„ein Zählwert, keine Adresse"*).

**F2 — Marker:** `--addressed`, `--fired`, `xfer` **tragen**. **`post/in` und `post/out` haben KEINEN
programmatischen Marker** — der **Dateiname** ist der Marker, aber **kein Werkzeug liest ihn**.
**Kein neuer Apparat** — ein einzeiliger **`post_lookup`** (oder `register_lookup --post`) genügt.

## Die Risse (alle vier, ungeglättet)

1. **Der nie gesetzte Marker (GLM, Mistral):** `post/in` ohne Einlieferungs-Disziplin — eine Datei,
   die die Konvention verfehlt, wird **still** unsichtbar (schlimmer als Kämmen). Fugu: *„`--addressed`
   leer heißt frei ODER absent — Axiom 0 verbietet, absent als nichts zu lesen."*
2. **Die tote Linie (GLM, Mistral):** `--fired` meldet nur, wenn jemand fragt. Läuft Linie X nicht,
   altert ihr Register **bemerkt**.
3. **Der Akt ohne Ort (Mistral):** die Sitzung muss **wissen**, dass `bin/task_dispatch` existiert —
   sonst kämmt sie wieder. Der Prompt nennt den Pfad (Fugus vierte Zeile).
4. **Konfliktordnung fehlt (Lumo):** vier Quellen parallel, **kein Verhältnis** — was, wenn `--fired`
   und `post/in` **dieselbe Aufgabe** liefern? *„Ohne Prioritätenspur ist der Merge unbestimmt."*
5. **Marker zeigen Existenz, nicht Wahrheit (Lumo):** Register und Dateisystem **divergieren** — keine
   Rückbestätigung (Receipt) nach dem Dispatch.
6. **Marker-Hits können selbst groß sein (Lumo):** ein Handover kann 400 Zeilen wiegen — *„ohne
   Größenbudget verschiebt sich das Kostenproblem nur eine Ebene tiefer: vom Kämmen zum Absaugen."*
7. **Lebenszyklus fehlt (Lumo, Fugu):** kein Signal für **gelesen/erledigt** — keine Differenz zwischen
   gestriger und heutiger Post. **Fugus Feld:** es fehlt **`letzter_eigener_akt`** und **`naechster_akt_bei`**.
8. **Das Wort bleibt unverankert (Lumo):** solange „Status erfassen" irgendwo weiterlebt, interpretiert
   jede neue Sitzung es neu. *„Der Fix gilt dem Kanon."*

### Fugus tiefste Funde (nachgetragen, volle Antwort)

9. **Der Kamm war der einzige Zugang zum Nicht-Markierten (Fugu, der schärfste):**
   *„`--orphan-docs` findet **62** Prosa-Docs mit offenen Markern ohne lebende Übergabe. Der Marker-Weg
   macht die Sitzung **blind für genau diese 62** — und für alles, was nie einen Marker bekam. **Der
   Kamm war nicht nur teuer — er war der einzige Zugang zu dem, was kein Marker trägt.** Wird
   `--orphan-docs` nicht selbst zur Prompt-Zeile, wird der Start-Prompt die **63. Leerstelle**."*
10. **Die Zahl statt der Liste (Fugu):** *„‚vier Linien' ist ein **Zählwert, keine Adresse** — derselbe
    Typ Fehler wie **Kürzel statt Akt**. Der Prompt trägt eine **Zahl, wo eine Liste** verlangt ist.
    Billigste Auflösung: die **vier Namen wörtlich**."*
11. **Der Akt ist kein Lookup (Fugu):** *„der Marker-Weg deckt die **Leseseite** vollständig und die
    **Schreibseite gar nicht** — `task_dispatch` schreibt `post/out/<datum>-<linie>-*.md`, genau den
    Marker, den die nächste Sitzung liest. Bleibt der Akt Prosa, ist der ganze Weg einseitig: man
    liest, was nie geschrieben wurde."*
12. **Kein neuer `post/in`-Marker (Fugu):** *„die Datei ist ein legitimer Marker — geschrieben von
    `task_dispatch`, also **von einem erzeugenden Akt, nicht von einem Namen**. Ein vierter Lookup wäre
    genau die **Haus-Krankheit R1–R4** (ein Name, wo schon eine Kraft steht)."*
13. **Der blinde Fleck ist `--fired`, nicht `post/in` (Fugu):** *„es setzt voraus, dass jemand Trigger
    schreibt. Schreibt niemand sie, ist `--fired` immer leer — und leer ist doppeldeutig."*
14. **Die Uhr (Fugu):** *„`post/out/…` trägt ein Datum im Namen, die Register-Blöcke tragen keins —
    **drei von vier Quellen haben keine Uhr**."*

**Fugus einer Satz:** *„Was der Marker-Weg übersieht: er kann nur zeigen, was jemand **geschrieben**
hat — und er kann **nicht zeigen, dass niemand geschrieben hat**."*

## Die zwei Billig-Fixes (aus dem Bestand)

- **Verschiebe-Konvention (Lumo):** `mv post/in/<file> post/read/` nach Verarbeitung → *„der Glob wird
   implizit zum Zustandsmarker. Geleerte Inbox ist geleerter Zustand."* **Einzeiler.**
- **Quellen-Priorität (Lumo):** *„Post schlägt Trigger schlägt Transfer"* — als Zeile. **Einzeiler.**

---

## An future

**Auftrag (R-2026-10-17-h):** Setze den Start-Prompt nach diesem Verdikt um — in **`command/<line>.md`**
(beide Häuser) und/oder `AGENTS.md`:
1. Die **vier/fünf Kommandos** statt „Status erfassen" (jedes Prosa-Nomen → ein Befehl).
2. Die **Stopp-Klausel** (negativer Suchraum) — sonst kämmt der Übersetzungsschritt weiter.
3. Der **Marker-Lookup für `post/in`** (`post_lookup`/`register_lookup --post`) — nur wenn der
   Einzeiler nicht reicht.
4. Die zwei **Einzeiler**: Verschiebe-Konvention + Quellen-Priorität.
**Fertig, wenn** der Start-Prompt Kommandos zeigt (kein „Status erfassen" mehr) und die Stopp-Klausel
trägt. **Kein stilles pending.**
