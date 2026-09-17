<!--
  title: Handover — Bau-Folge 70 (Stand 2026-09-18)
  session: Bau-Folge 70
  class: handover
  date: 2026-09-18
  sha256: aadaa5e677f54b6993fad59b5f3176c286d2473d0fc29142a2b1075381a7a191
  status: live
-->
# Handover — Bau-Folge 70 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18, HEAD ff212c28)

- **Postfach**: neuester `state/mail/mail_ledger.φ`-Eingang `1789670592`
  (2026-09-17, Rubin-Forum-Migrations-Thread — Publika-Kommentar, **kein
  Agenten-Eingang**). Keine `An bau`-Zeile in `docs/handover/post.md`.
- **CI am HEAD**: Watchdog 2026-09-17T23:30:13+02:00 — aktiv `ps1-cdn`
  35276288978 (in_progress), `ci-check` 35274865259 (queued), `harvest`
  35270867738 (in_progress); failed (fremde Linien): `harvest-dispatch`
  35274419367 / 35273075476, `pii-exposure` 35273689697, `harvest`
  35270996845, `swot-cdn` 35270820657.

## Offen

- **Scanned-/bild-only-PDFs → `vision`-OCR — `pending`** (kein Konsument hat es
  angefordert). `archive_search --pdf-text` liefert dort ehrlich `pending`
  (gemessen: `morabito.pdf`, `trk225_1996.pdf`, `124A.pdf`,
  `trk234-sis-2021.pdf`). Pfad: `archive_search --pdf-image <pdf|url> [--out
  <dir>]` → Dateien an `vision` (P6, liest nur). (Schritt: bei Bedarf
  `--pdf-image` + `vision`; kein Bau nötig, die Kette steht.)
- **`--pdf-text` bei Type0/Identity-H ohne ToUnicode — `pending`** (kein
  Handlungsschritt). Solche PDFs liefern `None`/`pending` statt Glyph-Müll
  (A = A); arXiv-Papiere deckt `--arxiv-src` exakt ab (gemessen: `2208.03865` →
  502 Zeilen LaTeX). (Schritt: keine Aktion — der arXiv-Weg trägt.)

## Benchmark (gemessen, session_burn)

- **Klasse „novel PDF-Parser" (kein registrierter Sieger)**: `grind-max`
  $0.4429 (1 Session, zwei Pässe) — Content-Stream-/ObjStm-/ToUnicode-CMap-
  Parser + Font-Kind-Gate, `pdf.rs` 747 → 2006 Zeilen. Kein Flash-Doppellauf
  (Flash trägt einen PDF-Parser nicht); Sieger eingetragen.
- **Routine (arXiv-e-print gzip+tar+.tex)**: `grind-flash` — Teil der 2
  grind-flash-Sessions, zusammen $0.0322.
- **Gemessenes Ergebnis**: `--pdf-text` liest `simple.pdf`, `njp105006.pdf`,
  `1808.05724.pdf`, `opus_thesis.pdf`, `sis2021.pdf` und die frische
  `verma`-URL (2159103 B). Wurzel des „kann keine PDFs lesen": `sfetch` ist ein
  HTML-Tool (`from_utf8_lossy` + `strip_tags`) und zerstörte Binär-PDFs →
  gefixt (`sfetch --raw`, Wrapper `bin/sfetch`); der saubere Weg ist
  `archive_search --pdf-text <url>` (binärsicher via `net::get`).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
