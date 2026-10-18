# Rat-Frage R-2026-10-18-c — Wie wird ein Pass zur Arbeit statt zur Faltung?

**Die Frage in einem Satz:** Ein Linien-Pass liest heute Post und Handover, faltet den Transport und schreibt seine Übergabe neu — er *delegiert* aber keine offenen Punkte an Agenten und *schließt* sie nicht. Welche Architektur macht aus „lesen + falten" ein „übernehmen + ausführen + belegen"?

## Die gemessene Lage (2026-10-18)

**Die offene Arbeit liegt nicht in den Handovers, sondern in Docs ohne Träger:**

| Maß | Wert |
|---|---|
| offene Zeilen in Prosa-Docs | **418** (in 179 Docs) |
| **Orphan-Docs** (offene Marker, kein lebender Handover-Träger) | **61** |
| Orphan-Register-Einträge je Linie (`--orphans --owner`) | **0** |
| stale Punkte (über 3 Pässe identische Lage) | **0** |

**Was das heißt:** Die Handover tragen wenige, sich ändernde Punkte — deshalb `--stale` = 0. Die **418 offenen Zeilen in 61 ungetragenen Docs** haben dagegen **keinen Besitzer**. Wer keinen Punkt besitzt, kann ihn nicht delegieren.

**Und die Pässe arbeiten durchaus:** die letzten Commits zeigen P10-Migrationen, GRS80-Messungen, Routen-Pins, Ledger-Quittungen. Die Arbeit **passiert** — sie schrumpft nur nicht die offene Menge, weil die offene Menge woanders liegt als die Arbeit.

**Zweite Messung, die daran hängt (R-i):** `closed <n>` misst heute nur `###`-Punkte unter `## Offen*`; die lebenden Handover schreiben Bullets → der Zähler liest **0**. Solange das so ist, kann kein Pass „geschlossen" *belegen*, auch wenn er arbeitet.

## Die Frage

1. **Wer besitzt die 61 Orphan-Docs** — eine Linie hausweit, jede Linie ihre eigenen, oder die Leitstelle? (Gemessen: heute niemand.)
2. **Was ist ein Pass?** Ist er ein *Lese- und Falt-Akt* (heute) oder ein *Arbeits-Akt* mit mindestens einem geschlossenen Punkt?
3. **Wie wird delegiert?** Soll jeder offene Punkt **Subagent + Fertig-Kriterium** nennen — und der Pass startet ihn? (Gemessen 2026-10-18: ein **freier Subagent** liest den Offen-Stand zahlentreu und kostet $0.)
4. **Wie wird Geschlossenheit belegt**, bevor R-i gebaut ist — Zitat, Commit-Hash, oder ein Quittungs-Ledger?

## Die Optionen

- **A — Besitz + Delegations-Kontrakt.** Jeder offene Punkt trägt `owner` + `subagent` + `done-when`; der Pass startet den Subagenten, prüft das Kriterium und schließt. Ein Pass ohne geschlossenen Punkt ist kein Pass.
- **B — Eine Warteschlange.** `register_lookup --open` wird zur Arbeitsliste; jeder Pass nimmt die ersten N Punkte, arbeitet sie ab und schreibt eine Quittung. Die Handover bleiben knapp, die Queue trägt die Menge.
- **C — Rollentrennung.** Die Linie *dispatched* nur (Punkt → Subagent + Kriterium), ein freier Worker führt aus, die Linie *verifiziert und schließt*. Zwei getrennte Akte, zwei getrennte Belege.
- **D — Der Pass ist eine Pipeline** (siehe Nachtrag): messen → **sofort** N Punkte parallel an Subagenten → prüfen → schließen. Falten ist Nebenwirkung, nicht Zweck.

Bitte: **Verdikt + der stärkste Riss** — insbesondere: wo wird aus „delegieren" ein Weg, Verantwortung abzugeben, ohne sie zu tragen? Keine Umsetzung — die ist Folge, nicht Frage.

## Nachtrag — der Operator präzisiert (2026-10-18)

> „Wenn ich die Linien gestartet habe und sie den Stand gemessen haben, können sie doch **direkt ihre Arbeit an Agenten delegieren**."

**Das ist der Kern, und er ist schärfer als Frage 2.** Der Pass hat heute die Reihenfolge:

```
messen  →  Transport falten  →  Handover neu schreiben  →  „fertig"
```

Gemeint ist:

```
messen  →  SOFORT delegieren (N Punkte → N Subagenten, parallel)
        →  Ergebnisse prüfen (Kriterium je Punkt)
        →  schließen (Beleg)
```

**Die neue Frage 5:** Soll der Pass nach dem Messen **direkt** delegieren — und ist die Parallelität (N Subagenten gleichzeitig) Teil des Kontrakts, oder muss sie gedrosselt sein (Rate-Limits, Kosten, Verwirrung durch viele gleichzeitige Schreiber)?

**Die neue Option D — der Pass ist eine Pipeline:** Messen erzeugt eine Arbeitsliste; jeder Punkt geht sofort an einen Subagenten mit Fertig-Kriterium; die Linie sammelt, prüft und schließt. Falten ist **Nebenwirkung**, nicht Zweck.

**Der gemessene Vorbehalt dazu:** Freie Subagenten tragen Mechanik (2026-10-18: zahlentreu, $0) — aber **sie tragen kein Urteil**. Ein Pass, der alles delegiert, delegiert also auch die Punkte, die ein Urteil brauchen. Die Frage an den Rat: **welche Punkte dürfen delegiert werden, welche nicht?**

## Der freie Schwarm sagt (Riss-Generator, $0, zwei Läufe)

**Lauf 1 — Verdikt C (Rollentrennung), Riss:**
> *„Der Pass übergibt die **Hand** an den Subagenten, aber nicht die **Haut** — die Verantwortung für die Qualität. Das erzeugt eine **Resignations-Delegation**: die 418 Punkte wandern in unsichtbare Subagenten-Queues, ohne dass jemand für die tatsächliche Reduktion haftet."*

**Lauf 2 — Verdikt A+D (Besitz + Pipeline), Riss:**
> *„Wenn der Subagent das Fertig-Kriterium definiert oder der Pass das Ergebnis ohne eigenes Urteil schließt, wird der Pass ein **Durchlauferhitzer**: `closed <n>` steigt formal, aber die reale Komplexität sinkt nicht."*

**Der Dissens im Schwarm, ungeglättet:** ob die **Leitstelle** Default-Owner der 61 Orphan-Docs ist oder **jede Linie ihre eigenen**; und ob die Parallelität unbegrenzt ist (nur durch Kontextfenster) oder auf **5–10 Punkte je Zyklus gedrosselt** werden muss.
