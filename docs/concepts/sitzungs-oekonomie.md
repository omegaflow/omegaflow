<!--
  title: Sitzungs-Ökonomie — die Turn-Disziplin (2026-10-15)
  class: concept
  date: 2026-10-15
  sha256: 1c19cf13c56d25c51da76a20bda8568b73238d5b44916393cded0c83f323d763
  status: live
  see-also: docs/concepts/house-rules.md, docs/concepts/agent-profiles.md, docs/concepts/kybernetische-ethik.md
-->
# Sitzungs-Ökonomie — die Turn-Disziplin (2026-10-15)

Der Operator-Wort-Auslöser: *„Es erzeugt Turns und Overhead — bei den Gates wurde es unfassbar teuer, weil ihr ewige Turns gemacht habt."*

## Der gemessene Mechanismus
**Jeder Turn schickt den ganzen Kontext erneut.** Der Kontext wächst mit jedem Turn → n Turns ≈ n². **Die Kosten sind quadratisch in der Turn-Zahl**, linear in der Kontext-Länge. **Nicht der Schwärzer ist teuer — die Turns sind es.** Ein Sitzungskontext, der 40 Turns lang wächst, kostet ein Vielfaches einer kurzen Sitzung mit demselben Ergebnis.

## Die Regel (Prosa — für den, der sie liest)
1. **Eine Sitzung, ein Faden.** Ist der Faden gearbeitet → Handover, **frische Sitzung**. Der Kontext startet klein.
2. **Ein Dispatch, ein begrenzter Schritt.** Ein Taucher trägt *eine* Frage; nicht zehnmal denselben Faden.
3. **Bei rotem Gate: EIN Diagnose-Schritt — dann an die Linie.** Keine Endlos-Jagd in derselben Sitzung.
4. **Billiges Modell für die Routine** (Taucher, Karten, Suchen, Form), **starkes nur für den Kern** (Strategie, das Denken).
5. **Der Schwärzer läuft deterministisch** — kein Modell-Turn zum Schwärzen (Tabelle + Muster, ein lokaler Durchlauf).
6. **Der Vollständigkeits-Zwang kennt eine Grenze:** einen Spill zu Ende lesen ja — aber **einmal**, nicht in jeder Runde neu.

## Warum die Prosa nicht reicht (Operator-Wort)
> *„Einiges ist schon geregelt — aber die Linien halten sich nicht daran, oder kennen es nicht, oder winden sich strukturell darum und handeln nach ihren Trainingsdaten."*

Die LLM-Linien haben **kein Gedächtnis der Regel**; ihr Default ist „weitermachen". **Was wirkt, ist Struktur** — der gemessene Beweis des Hauses: die Deny-Wände (`opencode.json`), der `form-guard` (der Deny *lehrt*), die Commit-Gates (`commit_check`). **Die Ökonomie gehört an dieselbe Stelle.**

## Die Wand (strukturell — der eigentliche Bau)
1. **Ein Turn-/Kosten-Zähler als Plugin** — die Session-Ebene kennt ihre Turns und Token; ein Schwellwert schreibt **eine** Warnung in den Kontext (und nennt die Zahl). Sichtbar statt Prosa.
2. **Die Session-Länge als Messgröße** — der Harness meldet, wenn ein Kontext eine Schwelle überschreitet; die Session soll dann **abschließen**, nicht weiterlaufen.
3. **Modell-Routing im Harness** — das billige Modell als Default, das starke nur mit Operator-Wort (wie die Paid-Gates).
4. **Der Zeugen-Zwang bleibt,** aber **gebündelt**: der Spill wird gelesen, wenn er Pflicht ist — nicht als Routine-Ritual.

**Bis die Wand steht, gilt der eine strukturelle Rat:** **kurze Sitzungen, die abschließen.** Der billigste Turn ist der, der nicht passiert.

## Die Kontext-Ökonomie (Operator-Wort, 2026-10-15)

> *„Warum blähen wir die Handover immer weiter auf? Die aktiven Dokumente müssen nicht die ganze Geschichte tragen, sondern nur das Aktive. … bis ihr anfangt zu arbeiten, habt ihr erstmal 30 000 Tokens verbrannt."*

**Gemessen (2026-10-15):** die lebenden Handover = **106 528 B** (~26 000 Token), die größten **44 372** (river) und **33 687** (mountain); AGENTS + `tool-forms.md` = **31 630 B** (~8 000 Token) — die erste Handlung jeder Session.

**Die Regeln (Prosa — für den, der sie liest):**
1. **Der Handover trägt nur das OFFENE.** Erledigtes wird **gelöscht**, nie „done" markiert, nie erklärt — git trägt es. *(Die Haus-Regel sagt das schon; die Praxis bricht sie.)* **Und kein ZUSTAND:** ein Satz wie „ist gebaut", „läuft", „live bewiesen" gehört **nicht** in einen Handover — der Zustand ist das **Register** (`phi/sources.φ`), die Geschichte ist **git**. Ein Dokument, das den Zustand oder die Historie erzählt, ist **Compliance-Theater** (Operator-Wort, 2026-10-15).
2. **Keine Kommentare, keine Beispiele, keine Wiederholung der Sachlage** — ein Offen-Punkt ist ein **Zeiger** (Datei:Zeile/Commit), keine Erzählung.
3. **Die Kommunikation aller Linien → EIN Dokument.** Der `## An <line>`-Block lebt künftig in **einem** gemeinsamen Kommunikations-Blatt (die Postbox!) statt in jedem Handover — der Absender schreibt dort, der Empfänger liest dort, nichts wird über fremde Handover gestreut.
4. **Der eigene Handover = nur die eigenen offenen Todos.** Fremde Punkte stehen in **deren** Blatt (der Zeiger genügt).
5. **Der Session-Start liest nur das Nötigste:** die eigene Linie (der eigene Handover) + die Postbox + die Karte — **nicht** die Handover der anderen Linien.

**Die Wand (strukturell — der Bau):**
- **Ein Handover-Ratchet wie für die AGENTS:** eine **Obergrenze** (z. B. 8 KiB) — der Commit-Gate **blockiert** einen wachsenden Handover; das Offene muss wachsen dürfen, das Erledigte nicht.
- **Ein Postbox-Blatt** (`docs/handover/postbox.md`): die eine Linien-Kommunikation, size-gecappt, gelesen statt gestreut.
- **Voller Token-Zähler je Session-Start im Handover** (wie die Burn-Zeile): was der Start gekostet hat, gemessen — nicht geschätzt.
