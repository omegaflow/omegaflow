<!--
  title: Auftrag — Klassen-Benchmark: die TE-Maschine gegen die publizierte PCMCI-Suite (TPR/FPR)
  class: auftrag
  date: 2026-09-08
  sha256: d630ae98990970f7908ad001a2ffaa69ca0482b1bc3a1727d761105787603305
  status: live
  see-also: docs/TODO.md docs/handover/handover-2026-09-08-atom-a-gpu-port.md docs/handover/handover-2026-09-08-nobel-dag-atom.md
-->

# Auftrag — Klassen-Benchmark: die TE-Maschine gegen die publizierte PCMCI-Suite (TPR/FPR)

## Der Auftrag

Die eine Messung, die das Superlativ „fortschrittlichste TE-Maschine" erst
beantwortbar macht: die breite TPR/FPR-Fläche von `pcmci_links` gegen die
publizierten Recovery-Raten der vollen synthetischen PCMCI-Benchmark-Suite
(Runge-Literatur) — und, wo zitierbar, gegen die Testbatterien von IDTxl und
Tigramite. Die Python-Regel schließt den Lauf fremden Codes aus: der Vergleich
läuft gegen publizierte Zahlen, nie gegen deren Code. Der Probe baut die Suite
nach der Prozessbeschreibung der Papers nach (zitierbar, kein Python) und legt
die eigenen TPR/FPR je Kante daneben. Erst diese Fläche trägt oder tötet das
Superlativ — bis dahin bleibt es ungemessen und gestrichen.

## Ausgangslage (gemessen — nicht neu suchen)

- Atom B ist gebaut: `pcmci_links` (Vorwärts-Elternsuche, Null-Ordnung vom
  TE-Lag getrennt) und `benjamini_hochberg` (FDR) stehen in
  `src/mathematikerin/te.rs`.
- Runge am Bz-Ankerfall reproduziert: `nobel_probe_bz` misst Bz als gemeinsamen
  Treiber von AE und Dst (4,3×/2,4× Schwelle, 2015–2026 stündlich).
- Der synthetische DAG-Benchmark (2026-09-07) deckt genau eine Kante:
  `pcmci_recovers_known_dag` in `te.rs`.
- Was fehlt: die breite Fläche. Die publizierte Suite — Modellklassen,
  Prozessbeschreibungen, berichtete TPR/FPR-Tabellen — ist zu recherchieren,
  jede Zahl mit Fundstelle (DOI/bibcode), nichts aus dem Gedächtnis.

## Spielregeln der Sitzung (bindend)

1. **Eine abgeschlossene Sitzung ist ein Atom.** Keine `pending`s, keine
   deferrals, keine „for nows". Jeder geöffnete Punkt endet gebaut und
   committet ODER gemessen freigegeben — ein descoped trägt seinen Befund
   (der Befund selbst ist der Eintrag), ein „später" gibt es nicht.
2. **Delegation ist Pflicht.** Recherche und Analyse laufen an Subagenten:
   die publizierte Suite und ihre Recovery-Zahlen (grind-pro für die
   Kraft-Kanal-Recherche; general für parallele Einheiten), die
   Codebasis-Erkundung (explore). Die Sitzung hält den Force-Gate, die
   Architektur und den Commit — sie tut nicht selbst, was ein Subagent
   tragen kann.
3. **Kein Python.** Weder der Lauf fremden Codes noch eigene Skripte —
   Rust std-only + curl ist der ganze Stack.
4. **Commit-Disziplin:** kein Commit, solange offene Punkte der Arbeit
   ungelöst sind; das TODO-Register wird im selben Commit geführt.
5. **Die Messung entscheidet:** ein Superlativ, das die Fläche nicht trägt,
   wird gestrichen — nicht gerettet, nicht umformuliert.

## Rückgabe

Der Probe (zitierbarer Suite-Nachbau) + das gemessene Blatt (eigene TPR/FPR je
Kante neben den publizierten Zahlen, jede mit Fundstelle) + der Verdikt-Satz
über das Superlativ + der Befund `docs/befund/befund-klassen-benchmark-pcmci.md`
(`antwortet-auf` diesen Auftrag). Abgeschlossen heißt: committet, kein offener
Punkt.
