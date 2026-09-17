<!--
  title: Handover — Bau-Folge 67 (Stand 2026-09-17)
  session: Bau-Folge 67
  class: handover
  date: 2026-09-17
  sha256: 81f2e710a7c26b35854dd14e8381bd4e999dcd2c2b10d583a672b90277a5cd49
  status: live
-->
# Handover — Bau-Folge 67 (2026-09-17)

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

## Stehender Pass (gemessen 2026-09-17, HEAD 5f9cd61b)

- Postfach: neuester `state/mail/mail_ledger.φ`-Eintrag `1789662457`
  (2026-09-17 16:27Z) — GitHub Support aktualisiert Ticket 4761801
  (GC/PII-Route, entscheid-Linie); keine neue bau-Zeile. Davor `1789650823`
  (Rubin-Forum-Summary), `1789650050` (AW an T. Keller: er sendet die
  TRISP/NSE-I(q,t)-Daten in einigen Tagen). Die fünf Sonden-Anfragen offen.
- CI am HEAD: Watchdog-Snapshot 18:09:50Z — 7 aktiv (mariner-occlt-cdn,
  pioneer-cell-census, allwise-cdn, demeter-cdn, physionet-cdn, gaia-xp-full
  queued); failed `pii-exposure` `35230489744`, `paper-check`
  `35228278279`/`35224760511` (fremde Linien).
- Zustand-Ledger `docs/zustand/external-state.md`: trägt weiter fremde
  uncommittete Zeilen @ `bc9d6a0b` — nicht angefasst (siehe Offen).

## Offen

- **`read`-Tool-Größenlimit für extrahierte FlateDecode-Bilder — `pending`.**
  Der PDF-Extraktor ist verifiziert: der Wrapper `bin/archive_search`
  (`bin/archive_search:9-19`) baut den Release-Binary bei Staleness neu, danach
  trägt `archive_search --help` die `images:`-Zeile und `--pdf-image` läuft.
  JPEG-Extraktionen liest `vision` vollständig (838×558/875×656, 26–65 KB);
  FlateDecode-PNGs scheitern am `read`-Größenlimit (`could not be resized below
  the image size limit`, 1,7 MB @ 875×656). Ursache am Baum: `png_wrap` schreibt
  das IDAT via `zlib_stored` (`tools/utils/src/bin/archive_search/pdf.rs:333,370`)
  — gültiges, aber unkomprimiertes PNG. (Schritt: erste Messung — die
  Byte-Schwelle des `read`-Tools mit gestuften JPEG-Größen bestimmen; danach
  entscheiden, ob `pdf.rs` einen echten Deflate-Kompressor bekommt (kein Deflate
  im Baum — `src/archivar/inflate.rs` ist nur Inflate) oder der Extraktor
  FlateDecode-Raster als JPEG re-enkodiert.)
- **`docs/zustand/external-state.md` — `wartend`** (fremde uncommittete Zeilen
  @ `bc9d6a0b`; der Postfach-Eintrag kennt den Keller-Eingang nicht). Die
  besitzende Linie faltet den Eintrag beim nächsten Pass. (Schritt: nicht
  anfassen.)

## Benchmark (gemessen, session_burn)

- PDF-Extraktor-Lesetest: die Mechanik (Wrapper-Rebuild + Extraktion) lief direkt
  in der Session; `vision` (Flash-Vision) las drei Bilder in 3 Läufen für
  **$0.0065** — Einzeltier, kein flash/pro-Doppellauf (keine Vergleichsklasse).
  Der als `blockiert` geführte Punkt löste sich über den sanktionierten Wrapper
  selbst; kein Operator-/CI-Build nötig.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
