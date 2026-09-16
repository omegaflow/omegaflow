<!--
  title: Handover — Forschung-Folge 49 (2026-09-16)
  class: handover
  date: 2026-09-16
  sha256: 600a6fbc8c47f9f08071fd8853a46dd560e1e19794c1697d3914d6930b613b77
  status: live
-->
# Handover — Forschung-Folge 49 (2026-09-16)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatiert); die Session arbeitet so viele ab wie möglich.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Paper / §4.5 — Monats-`posfrac` (härtester undatiert)

- Lauf `35145670301` (`aia-ladder-probe`, headSha dec20342, gemessen 2026-09-16)
  war in_progress; der Vorgänger `35140498561` trug den Monats-Block noch nicht.
  (Schritt: `gh run download 35145670301 --name aia-ladder-2015`, den Monats-Block
  `JJJJ-MM | N events | posfrac F (pos/tot) | mean D` lesen und `MIN–MAX` in
  `docs/paper/corona-heating-ladder.md:317–320` setzen; kein Polling.)

## ODF-Bande-Split — Konsument

- Lauf `35139594201` (`planetary-odf-cdn`) ohne Job-Start (queued, gemessen
  2026-09-16). (Schritt: `gh run view 35139594201 --log`, den gedruckten φ-Block
  nehmen und **jeden ganzen 5-Zeilen-Block** in `phi/sources.φ:6723–6733`
  ersetzen — nur `mro_odf` und `odyssey_odf` tragen einen Shard-Zweig;
  `refuse_shard_overlaps` in `src/archivar/parse.rs` verweigert Überlappung
  gleichen `format`.)

## Paper / Präregistrierung

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: `docs/paper/flyby-path-2-preregistration.md`.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
