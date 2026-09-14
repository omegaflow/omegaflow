<!--
  title: Handover — Ephemeriden-Output-Pfad (Stand 2026-09-14)
  session: Ephemeriden-Output-Pfad
  class: handover
  date: 2026-09-14
  sha256: 01e1f53954f1f7bf8cd38c049759c41e5e138edfc18f2ee23f7913f816786aa2
  status: live
-->
# Handover — Ephemeriden-Output-Pfad (2026-09-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

## Push wartet (Baum nicht ruhig)

- Der Commit der drei Compiler-Anpassungen steht; Push wartet, weil der Baum
  fremde Arbeit trägt — `src/archivar/{extract,geo,range,tests,zeuge}.rs`,
  `phi/sources.φ`, `phi/blocked_sources.φ`, diverse docs, und `bau27.md` ist
  umbenannt UND weiter modifiziert (die fremde Session arbeitet noch am selben
  Pfad). (Schritt: `git status` leer fahren, dann `/commit` mit dem Consent-Wort)

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
