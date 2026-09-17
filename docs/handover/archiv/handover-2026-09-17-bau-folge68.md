<!--
  title: Handover — Bau-Folge 68 (Stand 2026-09-17)
  session: Bau-Folge 68
  class: handover
  date: 2026-09-17
  sha256: fefac3941b4819573be38832295dbf6cf8844768c6699aa349158a2b3f7153b4
  status: live
-->
# Handover — Bau-Folge 68 (2026-09-17)

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

## Stehender Pass (gemessen 2026-09-17, HEAD 8d821322)

- Postfach: neuester `state/mail/mail_ledger.φ`-Eintrag `1789662457`
  (2026-09-17 16:27Z) — GitHub Support aktualisiert Ticket 4761801
  (GC/PII-Route, entscheid-Linie); keine neue bau-Zeile. Die fünf
  Sonden-Anfragen offen.
- CI am HEAD: Watchdog-Snapshot 2026-09-17T19:21:56Z — 5 aktiv (allwise-cdn
  `35263717637`, ned-cdn `35258227020`, ps1-cdn `35257451098`, health-check
  `35245084696`, planetary-odf-cdn `35231817955`); failed `swot-cdn`
  `35251359490`, `gedi-cdn` `35250788545`, `ci-check` `35250778775` (fremde
  Linien).
- Zustand-Ledger `docs/zustand/external-state.md`: trägt fremde uncommittete
  Zeilen — nicht angefasst (siehe Offen).

## Offen

- **`docs/zustand/external-state.md` — `wartend`** (fremde uncommittete Zeilen;
  die besitzende Linie faltet den Eintrag beim nächsten Pass). (Schritt: nicht
  anfassen.)

## Befund (gemessen, umgesetzt)

- Der als „`read`-Tool-Größenlimit für FlateDecode-Bilder" geführte Punkt ist
  **widerlegt und behoben**. Die Messung am Baum ergab: es gibt kein Byte-/
  Dimensionslimit — ein valides 16×16-PNG (116 B) liest, ein malformtes
  16×16-PNG (836 B) bricht ab. Ursache: `png_wrap` schrieb den nackten Raster
  als PNG-Scanlines **ohne Filterbyte pro Zeile** (IDAT inflatet zu 768 B statt
  784 B @ 16×16×3) — kein gültiges PNG. Fix in
  `tools/utils/src/bin/archive_search/pdf.rs:325` (Filter None `0x00` je
  Scanline vor `zlib_stored`), Gate-Test
  `png_idat_carries_one_filter_byte_per_scanline` im selben Atom. Die
  Passthrough-Hypothese (Original-zlib als IDAT) ist gemessen falsch. Nachher:
  `read` liest die PNGs (16×16 bis 20,9 MB). `cargo check` 0 Fehler/0 Warnungen.

## Benchmark (gemessen, session_burn)

- PDF-FlateDecode-Fix: `grind-flash` (Flash) fand in einem Lauf die wahre
  Ursache (malformtes PNG statt read-Größenlimit) und implementierte Fix +
  Gate-Test; `cargo check` 0/0. Kein pro/max-Doppellauf — die gemessene
  Routine-Klasse ist geschlossen (Flash-Sieger, AGENTS.md; gemessen flash
  $0.0008–0.0017 gegen pro/max $0.0041–0.0090), die Flash-Antwort war
  vollständig und korrekt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
