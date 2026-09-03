<!--
  title: Auftrag — MatrixMachine ins main-Register führen (Zustandszeile + Maschinen-Heimat)
  class: auftrag
  date: 2026-09-03
  status: pending
  see-also: archive-root/vanilla-dateidocs/handover/handover-2026-08-31-matrixmachine.md,
            src/mathematikerin/machines/matrix.rs, docs/auftrag/auftrag-maschinen-audits.md
-->

# Auftrag: MatrixMachine im main-Register

## Ausgangsmessung (2026-09-03)

Der Code ist auf main committet und eingebunden: `52eca21` (fresh start,
keine Stränge) trägt `src/mathematikerin/machines/` (`matrix.rs`, `solar.rs`,
`verdict.rs`, `tests.rs`); `omega.rs` verdrahtet die Maschine live
(`matrix: MatrixMachine`, Zeilen 140/231/853/910). Die historische
Strand-Kette (`akteure-portieren`, d3d69fe/5c23692/96eb606) ist im
fresh-start-Commit aufgegangen — die Frage „Strang oder main" ist erledigt.

## Was offen ist — die Register-Zeile, nicht der Code

- Die Geburtsurkunde `handover-2026-08-31-matrixmachine.md` (gehalten unter
  `archive-root/vanilla-dateidocs/handover/`) trägt das Zustandsfeld
  „commit auf dem Strang der Maschine" / „Code-Zustand: uncommittet" — ein
  Zustandsfeld, das gegen main `52eca21` falsch ist. Eine Zustandszeile ziehen:
  Code-Commit `<sha auf main>`, Teststand, Speisung — sonst lügt das
  Zustandsdokument über seinen eigenen Gegenstand (die Lehre der
  Frontmatter-dates, die nach Edits stehenblieben).
- Es gibt kein `docs/maschinen/`-Register und keine main-TODO-Zeile für die
  Maschine. Register-Heimat schaffen (die Lücke aus dem Chat-Verlauf:
  „kein Register für Maschinen — nur für Quellen, Arbeit, Funde, Messungen").
  Ein `docs/maschinen/`-Ordner entsteht nur bei Bedarf zur zweiten Maschine;
  eine Zeile in `docs/TODO.md` und die nachgezogene Urkunde genügen solange.
- Erster echter Konsument: 0 — legitimerweise `pending` (Anforderung treibt
  den Einsatz, nicht die Syntax). Als offene Pflicht benennen, nicht schließen.

## Sequenz

Ein Commit je Auftrag, main-Guard respektieren. Commits der Feldarbeit
eigenverantwortlich im Repo; kein Operator-Wort nötig (kein Siegel, keine
Mail, keine main-Ausnahme).

## Lieferung

Urkunde-Zustandszeile nachgezogen (Code-Commit <sha> gegen main `52eca21`
verifiziert); Maschine mit Status (428/428 gegen main nachgemessen, nicht
übernommen), Speisung, Syntax, 0 Konsumenten als `pending` in `docs/TODO.md`
registriert.

## Abschluss

Die MatrixMachine existiert vollständig im main-Register: committet,
getestet (nachgemessen), Zustandszeile wahr, erster Konsument offen benannt.
