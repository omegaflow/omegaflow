<!--
  title: Messzustände — zwei Achsen: was wissen wir, was tun wir (2026-10-21)
  class: concept
  date: 2026-10-21
  sha256: d92a1c0befd6f60d0ee6a2840ebc711d5ece2a7861f477e6fdc4b1fb7c66596d
  status: superseded
  owner: future
  ack: future 2026-10-21T13:40Z
  next: Wissenschafts-Survey, dann Rat der 13
  orphan-after: none
  see-also: docs/concepts/glossar.md docs/granit.md docs/surveys/survey-2026-10-21-riss-zensus-nach-test.md
-->
# Messzustände — zwei Achsen: was wissen wir, was tun wir

> **Abgelöst (2026-10-21):** kanonisch ist **v3** —
> `docs/concepts/messzustaende-v3.md` (drei Achsen: Messzustand, Lebenszyklus,
> Zugang). Dieses Blatt bleibt als v2-Historie.

**Das Problem.** Wir haben `pending`, `absent`, `riss`, `descoped`, `declined`,
`wartend` und `LOCK` jahrelang in einen Topf geworfen. Daraus wurden
Hintertüren: „begründetes pending", „gemessenes nicht gebraucht", `descoped`
als weiches Nein. Dieses Blatt trennt die Achsen.

## Achse 1 — Messzustand (was wissen wir?)

Es gibt **zwei** Zustände:

- **`pending`** — nicht gemessen. Der Default; trägt einen **Grund-Tag**:
  `nicht_gemessen | nicht_nachgewiesen | nicht_berichtet | nicht_zugänglich`.
- **`gemessen`** — eine Messung liegt vor.

`obergrenze`, `null-echt`, `absent` und `riss` sind **Qualifier von `gemessen`**,
keine eigenen Nicht-Mess-Zustände:

| Qualifier | Bedeutung | Bedingung |
|---|---|---|
| `obergrenze` (alias `zensiert`) | Obergrenze `< L_D` | gemessen, aber unter der Nachweisgrenze; nie 0 |
| `null-echt` | der Wert ist 0 | 0 über n gemessen; PDF schließt 0 ein |
| `absent` | die Sache existiert nicht | Nicht-Existenz belegt; **selten** |
| `riss` | Widerspruch | vergleichbar **und** Kettenfehler ausgeschlossen **und** Daten verschieden |

**`riss` ist kein „zwei Zahlen differieren".** Der Unterschied zweier
verschiedener Datenquellen oder Modelle (DE/INPOP/EPM) ist kein Riss. Ein Riss
ist erst echt, wenn dieselbe Sache unter denselben Bedingungen gemessen wurde,
die Kette geprüft ist und die Daten sich trotzdem widersprechen.

**`absent` ist die Ausnahme.** Die Vergangenheit zeigt: fast alles, was wie
„gibt es nicht" aussah, war „noch nicht gemessen" oder „Zugang fehlt". Ein
ungebautes Werkzeug ist `pending`, nicht `absent`.

## Achse 2 — Prozess-Tag (was tun wir?)

Diese Tags sind **orthogonal** zum Messzustand. Ein Punkt kann `pending` **und**
`LOCK` sein, oder `gemessen` **und** `declined`.

| Tag | Bedeutung | Bedingung |
|---|---|---|
| `offen` | Aufgabe | Aufgabenebene, kein Messzustand |
| `wartend` | äußerer Trigger fehlt | benannter Trigger; gehört nach `state/wartend.φ` |
| `LOCK` | Operator hat vertagt | Operator-Wort + Grund; ohne Wort: Parkplatz |
| `declined` | klares Nein | benannte Entscheidung mit Owner/Datum/Grund |

**Autolock ist kein eigener Tag.** Er ist `LOCK` mit dem Grund `privat` oder
`rechtlich`: die Ablage (`private`) und die Rechtsbindung sind der Grund, nicht
der Zustand.

**`descoped` gibt es nicht.** Es war nie ein Zustand. „Nicht im Fokus" ist
`pending`; „nein" ist `declined`; „wartet" ist `wartend`. Wer aus dem Fokus
nimmt, braucht ein `LOCK` — und ein `LOCK` ohne Wort ist ein Parkplatz.

**`declined` ist nicht `absent`.** `declined` ist unsere Entscheidung;
`absent` ist eine Messung. **`LOCK` ist nicht `declined`.** `LOCK` heißt
privat/vertagt, nicht abgelehnt.

## Die Regel

1. **Default ist `pending`.** Jede Lücke ist offen, bis sie gemessen ist.
2. **Ein `pending` schließt nur eine Messung**, keine Marke und kein Gate.
3. **Ein begründetes `pending` ist keine Auflösung.** Der Grund benennt die
   Lücke, er schließt sie nicht.
4. **`null-echt`, `absent` und `riss` setzen eine Messung voraus.**
5. **`descoped` ist abgeschafft.** Fokus = `LOCK` mit Wort; Nein = `declined`;
   Warten = `wartend`.
6. **Der Operator schließt** (Granit, Prozessregel 6). Die Maschine liefert
   Messungen und schlägt Stopps vor.

## Was daraus folgt

- **Stille Lücken** (offen ohne `pending`) sind der gefährlichste Fall; sie
  sehen nicht offen aus und müssen benannt werden.
- **Geliefert ≠ geschlossen.** Ein Auftrag kann geliefert sein und ein
  `pending` tragen; geschlossen wird er vom Operator.
- **Autolock ist kein Tag, sondern ein LOCK-Grund:** `private` oder rechtlich
  gelockt (Lizenz, Vertrag, Weitergabe). Ein frei zugänglicher technischer
  Zugang (`demeter`) ist kein Autolock.
