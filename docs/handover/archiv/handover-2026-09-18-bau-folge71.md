<!--
  title: Handover — Bau-Folge 71 (Stand 2026-09-18)
  session: Bau-Folge 71
  class: handover
  date: 2026-09-18
  sha256: 4805da13e69690c1551579501d15cc72dcce0c8b829d14365949102aafdc6952
  status: live
-->
# Handover — Bau-Folge 71 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18, HEAD 3b7aa5a1)

- **Postfach**: letzter `state/mail/mail_ledger.φ`-Eingang `1789670592`
  (2026-09-17, Rubin-Forum-Migrations-Thread — Publika-Kommentar, **kein
  Agenten-Eingang**). Keine `An bau`-Zeile in `docs/handover/post.md`.
- **CI am HEAD**: Watchdog-Snapshot 2026-09-18T00:34 — aktiv `allwise-cdn`
  (queued), `harvest` ×2 + `ps1-cdn` + `ci-check` (in_progress); failed (fremde
  Linien): `harvest`, `harvest-dispatch`, `pii-exposure`.
- **HEAD** `3b7aa5a1` == `origin/main` (ernte-Folge 75: ulysses/goes-Diagnose,
  goes16-`_M6C`-Fix, `ulysses_atdf_x` registriert). Fremd im Baum (nicht
  angefasst): `src/archivar/atdf.rs` (M, next-same-band-Paarung), drei gestagte
  `handover-2026-09-16-*`-Renames.
- **Earthdata-Token-Hook** live verifiziert (2026-09-18): `archive_search
  --sniff` der geschützten PODAAC-GRACE-URL → 401 ohne Token, 200 mit Token
  (84 701 036 B, sha256 `8bd14764…`); `.secrets.local` trägt einen gültigen
  `EARTHDATA_EDL_TOKEN`. Kein offener Punkt.

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

## Benchmark (gemessen)

- **Routine-Verifikation** (Earthdata-Token-Hook live, `grind-flash`, ein Lauf):
  die Klasse „Routine-Agent" ist per `AGENTS.md` geschlossen (flash 2.4–11×
  günstiger bei identischem Ergebnis) — kein Doppellauf.
- **Klasse „novel PDF-Parser"**: Sieger `grind-max` ($0.4429, folge70) bleibt
  eingetragen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
