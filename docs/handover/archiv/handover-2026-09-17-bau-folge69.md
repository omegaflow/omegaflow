<!--
  title: Handover — Bau-Folge 69 (Stand 2026-09-17)
  session: Bau-Folge 69
  class: handover
  date: 2026-09-17
  sha256: a0b89dd04b6cdbf7f0cde44343fb05289d4ccd37ae6360ff2f6e7f91ceab4b9d
  status: live
-->
# Handover — Bau-Folge 69 (2026-09-17)

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
der härteste undatierte); die Session arbeitet so viele ab wie möglich.
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-17, HEAD 3a498e3c)

- **Postfach**: neuester `state/mail/mail_ledger.φ`-Eingang `1789670592`
  (2026-09-17, Rubin-Forum-Migrations-Thread — Publika-Kommentar, **kein
  Agenten-Eingang**); davor `1789662457` (GitHub-Support #4761801 GC-Thread).
  Keine `An bau`-Zeile in `docs/handover/post.md`. Der Zustand-Ledger
  `docs/zustand/external-state.md` wurde in dieser Session von der
  entscheid-Linie frisch gemessen (`3a498e3c`) — zitiert, nicht angefasst.
- **CI am HEAD**: Watchdog-Snapshot 2026-09-17T22:26:12+02:00 — 5 queued
  (`harvest` 35270658104, `harvest-dispatch` 35270640849, `ned-cdn`
  35270527004, `auto-dispatch` 35270522987, `ci-check` 35268354412), 2
  in_progress (`pioneer-cell-census` 35266366575, `health-check`
  35245084696). `ci_manage list` 21:03Z: `ci-check` 35274466859 pending,
  `goes-cdn` 35274196575 success, `cdn-reconcile` 35273461901 success,
  `gaia-sso-cdn` 35272160298 success. failed (fremde Linien): `harvest`
  35270996845, `harvest-dispatch` 35273075476, `pii-exposure` 35273689697
  (Exposition bleibt — entscheid-Linie).

## Offen

- **Kein abarbeitbarer undatierter Punkt.** Die frühere Wartestellung
  `docs/zustand/external-state.md` (fremde uncommittete Zeilen) ist erloschen:
  der Baum trägt die Datei committet (`3a498e3c`), `git status` zeigt keine
  Änderung daran — gestrichen, kein Ersatzpunkt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
