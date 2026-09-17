<!--
  title: Handover — Bau-Folge 66 (Stand 2026-09-17)
  session: Bau-Folge 66
  class: handover
  date: 2026-09-17
  sha256: bf92f8d8bc0b2734bfe0027cafcd53a3937262609003aff20bd2c5b8f9df59a7
  status: live
-->
# Handover — Bau-Folge 66 (2026-09-17)

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
Wartestellungen (`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser
und werden nie als Handlungsschritt geführt; gibt es keinen abarbeitbaren
undatierten Punkt, sagt die Session das. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (gemessen 2026-09-17, HEAD f781f6ee)

- Postfach: neuester `state/mail/mail_ledger.φ`-Eintrag `1789662457`
  (2026-09-17 16:27Z) — GitHub Support aktualisiert Ticket 4761801
  (GC/PII-Route, adressiert an die entscheid-Linie); davor `1789650823`
  (Rubin-Forum-Zusammenfassung), `1789650050` (AW an T. Keller, TRISP/MLZ:
  er sendet die NSE I(q,t)-Daten in einigen Tagen). Die fünf Sonden-Anfragen
  (Rubin, Sotgiu, …) unverändert.
- CI am HEAD: Watchdog-Snapshot 18:09:50Z — 7 aktiv (mariner-occlt-cdn,
  pioneer-cell-census, allwise-cdn, demeter-cdn, physionet-cdn, gaia-xp-full
  queued); failed `pii-exposure` `35230489744`, `paper-check`
  `35228278279`/`35224760511` (fremde Linien).
- Zustand-Ledger: `docs/zustand/external-state.md` trägt weiter fremde
  uncommittete Zeilen (Postfach/PII/CI @ `bc9d6a0b`); der Postfach-Eintrag
  kennt den Keller-Eingang nicht — nicht angefasst.

## Offen

- **`vision`-Lesetest des PDF-Extraktors — `blockiert`** (härtester undatiert).
  Das PATH-`archive_search` trägt den Modus `--pdf-image` nicht (`archive_search
  --help` nennt ihn nicht), die Quelle aber schon (`tools/utils/src/bin/
  archive_search.rs:184`, Hilfe `:473`, `pdf_images` in `archive_search/pdf.rs:71`)
  — der Release-Binary ist vor dem Feature gebaut. (Schritt: nach dem nächsten
  `archive_search`-Release-Build `archive_search --pdf-image
  docs/reference/dsn_trk-2-18.1988-10-15.pdf --out /tmp/opencode/pdfimages`,
  dann den extrahierten `.jpg`-Pfad an `vision` reichen.)
- **`docs/zustand/external-state.md` — `wartend`** (fremde uncommittete Zeilen
  @ `bc9d6a0b`). Die besitzende Linie faltet den Postfach-Eintrag beim nächsten
  Pass. (Schritt: nicht anfassen.)

## Benchmark (gemessen, session_burn)

- Live-Marker-Atom: `grind-pro` (Urteil+Bau, ein Kontext) **$0.1706** —
  `time_markers()` als eine Render-Quelle, Fabrikations-Gate. Preamble-Angleichung
  (4 Kommandos): `grind-flash`. Flash-first für die Mechanik bestätigt; der
  Urteil-Lauf blieb bei `grind-pro` (kein `max`-Doppellauf).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
