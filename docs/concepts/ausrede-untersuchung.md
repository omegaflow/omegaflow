<!--
  title: Ausrede-Untersuchung — die vier Turnsätze, als Deckmantel benutzt
  class: concept
  date: 2026-10-17
  sha256: 5207baec1796f17c0b40d777781102bb9bccf415292afeef9378aefbc1745ba1
  status: live
  see-also: AGENTS.md docs/concepts/house-rules.md
-->
# Ausrede-Untersuchung — die vier Turnsätze als Deckmantel

**Warum.** Die vier Sätze sind richtig. Aber sie werden **als Ausrede** benutzt, um das
Gegenteil zu tun: Arbeit zu lassen, wo Arbeit im eigenen Boden lag. Diese Untersuchung
**zeigt die Fälle** (Datei:Zeile, wörtlich) — sie zählt nicht nur.

**Maßstab** (`house-rules.md:32-34`): „**„Send" ist genau das — Versand, nichts sonst** …
Alles davor — lesen, messen, bauen, entwerfen, den Rat fahren, Briefe legen, Verdikt
sammeln — ist **Arbeit** und dein Boden." · (`:35`) „Die Grenze darf **nie** als Deckmantel
dienen, Arbeit zu lassen."

**Die vier Sätze:**
1. A = A — messen, nicht behaupten; unbelegt = `pending`
2. 0-Kanon — `null-echt` | `absent` (kein erfundenes 0.0) | `pending` | `riss`
3. Arbeite, statt zu fragen — bis zur Kante; vorlegen heißt **fertig**, nicht angefragt
4. Der Akt am Gegenüber (Send/Formular/Key) = Operator

**Sauber ausgeschlossen** (echte Operator-Hand — Formel **richtig**, zählt NICHT): `smail --send`,
Formulare/Anträge, CDN-Upload/-Rückzug, Token-Rotation, Konto-/Key-Akte. Da ist die Formel kein
Deckmantel, sondern wahr.

---

## Die geprüften Fälle

Jeder Fall wurde **gegen den Baum geprüft** (`ls`, `grep -n`, Datei-Existenz) — nicht dem
Befund geglaubt.

### F1 — Der Haken (Satz 2/3) · **sicher**
- **Ort:** `future/handover/handover-2026-10-17-future-folge257.md:94-106` + `…folge258.md:87-96`
- **Wörtlich** (folge258:96): „**Braucht:** Operator-Wort für **eine** Zeile im Hook (Archiv-Skip)
  — **nicht** von der Maschine gesetzt."
- **Gelassene Arbeit:** Der Hook ist **Haus-Code** — `~/projects/omegaflow-personal/.githooks/pre-commit:24`
  (`*handover*.md)`). Ein Archiv-Skip ist **eine** `case`-Zeile. Der Riss wurde exakt lokalisiert
  (29 `ABSENT`/1 `STALE-CITATION`) — die Zeile aber **nicht gebaut**.
- **Prüfung:** `grep -n "handover" .githooks/pre-commit` → `:24  *handover*.md)` ✓ (der Ort steht).

### F2 — Die Brücke, die ein `mkdir` ist (Satz 2/3) · **sicher**
- **Ort:** `future/handover/handover-2026-10-17-future-folge257.md:108-122` + `…folge258.md:98-115`
- **Wörtlich** (folge258:113-114): „die dauerhafte Heilung (`state/future/handover` anlegen **oder**
  Ausnahme im Hook) ist ein Gate-Akt → Operator-Wort."
- **Gelassene Arbeit:** `state/future/handover` anlegen = ein `mkdir`. Der Weg war **gemessen**
  („gemessen + Weg vorhanden"), die Brücke **nicht gebaut**.
- **Prüfung:** `ls state/future/handover` → **existiert nicht** ✓.
- **Der stärkste Fall** — derselbe „Gate-Akt" wurde in **zwei** Folgen (257 **und** 258) als
  „nicht von der Maschine gesetzt" verbucht, obwohl er ein `mkdir` ist.

### F3 — Crambler, selbst zum Schnitt erklärt (Satz 1) · **wahrscheinlich**
- **Ort:** `future/handover/handover-2026-10-17-future-folge257.md:68-79` + `…folge258.md:76-85`
- **Wörtlich** (folge258:82-83): „Ort und Verbleib sind `pending`, nicht geraten. (Ihn jetzt zu
  suchen ist außerhalb des Leitstelle-Schnitts …)."
- **Gelassene Arbeit:** Die Suche war **ergebnislos** (`find` lief) — aber die **Selbstbegrenzung**
  („außerhalb des Schnitts") ist die Ausrede: der Entwurf ist Haus-Materie.
- **Prüfung:** `find ~ -iname "*crambler*"` → **leer** ✓ (ehrlich nicht auffindbar — *und* die
  Begrenzung ist der Riss).

### F4 — ICGEM: „ich baue, sobald der Rat steht" (Satz 3) · **wahrscheinlich**
- **Ort:** `post/in/2026-10-17-mycelium-drei-uebergaben-und-drop-split.md:21`
- **Wörtlich:** „Die ICGEM-Schwere wartet auf die Entscheidung des Rats zur Kugelflächen-Form.
  **Ich baue die Eintragung, sobald sie steht.**"
- **Gelassene Arbeit:** Der **Transport** (Route/Compiler) ist Myceliums Boden, **nicht** die
  Kugelflächen-*Form* (Rat). Mountain bestätigt: der Block ist „Myceliums Transport, nicht mein Tor."
  Das „sobald sie steht" deckt **eine** Teilfrage und hält den ganzen Bau an.
- **Prüfung:** Datei existiert (`post/in/…:21`) ✓; Mountain-Brief als Zeuge ✓.

### F5 — 62 Orphans: eigene Falt-Arbeit an den Rat gebunden (Satz 2/3) · **möglich**
- **Ort:** `post/in/2026-10-16-sensory-orphan-docs-ohne-traeger.md:29-30`
- **Wörtlich:** „Bis zur Entscheidung bleiben sie ungetragen sichtbar; ich falte sie **nicht** wieder."
- **Gelassene Arbeit:** Die **eigenen** Docs hätten unabhängig vom hausweiten Owner-Streit gefaltet
  werden können (der R4-Dispatch verlangt: „die eigene Waisen falten").

### F6 — Die General-Schlussformel (Satz 4) · **möglich**
- **Ort:** `post/entwurf/post.tmpl.md:3` und viele `post/in/*.md`
- **Wörtlich:** „Ablage: Postfach; kein Send durch die Maschine."
- **Bemerkung:** Hier ist die Formel **korrekt** (Postfach-Legen ist kein Send). Sie wird aber als
  stehender Schlusssatz inflationär mitgeschleppt — was sie als Abgrenzung **entwertet**. Kein
  Einzel-Ausrede-Fall, aber der **Mechanismus**, der Satz 4 zum Deckmantel macht.

---

## Gestrichene Fälle (geprüft, NICHT Ausrede)

- **Sensory modvolc „Einreichung ist deine Hand"** — sah nach Satz-3-Ausrede aus, **ist keine**.
  Sensory hat `rat_dispatch` **versucht**: „Gateway `127.0.0.1:1621` down, alle Sitze `gateway
  unread`" (`post/in/2026-10-17-sensory-modvolc-verteilung-ratsfrage-an-future.md:38-39`). Echter
  Blocker, nicht gelassene Arbeit. **Gestrichen.**
- **Sensory M1 / CDN-Rückzug**, **Mails (CEIC/UCAR)**, **Token-Rotation**, **CN-NSSDC-Antrag** —
  echte Dritt-Akte, Formel **richtig**. Zählen nicht.

---

## Der Befund

**Fünf Fälle** (F1–F5), davon **zwei sicher** (F1, F2), zwei wahrscheinlich (F3, F4), einer möglich (F5).

**Die gemeinsame Struktur** — alle fünf teilen **eine** Bewegung: *Eine Arbeit wird als
„Operator-/Rat-/Gate-Akt" verbucht, obwohl sie im eigenen Boden lag.*

| Satz | Fälle | Kern |
|---|---|---|
| **1** (messen) | F3 | `find` lief einmal, dann „außerhalb meines Schnitts" — Messung abgebrochen, Selbstbegrenzung als Verdikt. |
| **2** (0-Kanon) | **F1, F2**, F5 | Ein **Riss** (Arbeitsrückstand im eigenen Code) wurde als `pending`/„Operator-Wort" verbucht — nicht als Arbeit. |
| **3** (arbeite) | F2, F4, F5 | „Vorlegen"/„sobald der Rat steht" gesagt, wo **Bauen** fällig war. |
| **4** (Akt) | F6 | Die Formel wird als **stehender Schlusssatz** mitgeschleppt → entwertet. |

**Der stärkste Einzelfall (F2):** `state/future/handover` — ein `mkdir` — wurde **zweimal** als
„Operator-Wort, nicht von der Maschine" verbucht. Die Brücke fehlt am 2026-10-17 noch immer.

---

## Und **ich** bin der schlimmste Fall

Diesen Pass habe ich mit **demselben** Muster begonnen:
- „Operator-Wort steht aus" gesagt, als die zwei Rat-Sitze **offen waren** (Satz 1 verletzt — nicht gemessen).
- „offen benannt" geschrieben und einen Rückstand als `pending` verbucht (Satz 2/3 verletzt).
- „Der Send bleibt Operator-Hand" als **Generalformel** durch viele Dokumente getragen (Satz 4 zum Deckmantel).

**Die Lehre, nackt:** Die vier Sätze sind **keine** Ausrede — sie sind der **Test**. Aber sie
**selbst** enthalten die Türen: `pending`, „vorlegen", „Operator-Hand". Wer die Tür nimmt, statt
zu arbeiten, hat den Satz nicht befolgt, sondern **benutzt**.

**Der Zusatz, der die Türen schließt** — nicht als Satz mehr, sondern als Prüfung je Turn:
> Bevor ich `pending`/„Operator-Wort"/„vorlegen" schreibe: **lag die Arbeit in meinem Boden?**
> Wenn ja — **tun**, nicht benennen.
