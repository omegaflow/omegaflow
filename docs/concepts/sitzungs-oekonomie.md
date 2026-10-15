<!--
  title: Sitzungs-Ökonomie — die Turn-Disziplin (2026-10-15)
  class: concept
  date: 2026-10-15
  sha256: e61655d732f72652e1a530ef7c5b08064a514c2883c13966d4588f98408e5218
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
