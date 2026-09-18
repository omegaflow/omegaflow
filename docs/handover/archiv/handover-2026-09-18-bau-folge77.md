<!--
  title: Handover — Bau-Folge 77 (Stand 2026-09-18)
  session: Bau-Folge 77
  class: handover
  date: 2026-09-18
  sha256: ca7673f1316df96519b3ba5bb69c0c9378a5dd3ac069ceab4c58a4f0f0bcb4f7
  status: live
-->
# Handover — Bau-Folge 77 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18, HEAD 6a8ecade)

- **Postfach** — `post.md` leer (nur Header), **keine `An bau`-Zeile**; letzter
  `state/mail/mail_ledger.φ`-Eingang `1789689115` (Rubin-Forum, kein
  Agenten-Eingang).
- **CI am HEAD** — `te-gate` `35314110831` @`7b67b10d` **pending**; `ci-check`
  `35314164053` @`6a8ecade` **pending**, `35314106560` cancelled;
  `planetary-odf-cdn` `35313968728` in_progress; `te-gate` `35313041295`
  in_progress; Rest fremde Linien. Beide bau-relevanten Läufe waren beim
  Session-Start (06:1x) noch nicht entschieden.
- **HEAD** `6a8ecade` == `origin/main` (FF).
- **Sicherheitsnetz** — `refs/safety/1789712219` (Session-Start), neu
  `refs/safety/1789712219`-Folge-Snapshot beim Abschluss.

## TE-Gate — Gate-Verdikt ausstehend (härtester undatiert)

- Restrisiken (a)(b) in folge76 geschlossen; offen ist das Gate-Verdikt und der
  Rename (c). (Schritt: `ci_manage view 35314110831` einmal lesen; grün → Rename
  `arx_restricted_surrogate_conditional` nach grünem Gate (Name=Implementation);
  rot → rote Zelle + FN-Arm `found/meas ≥ 0.5` beider Richtungen ist die Messung,
  Design verbreitern, Verweigerung nie aufweichen.) · `wartend`

## Red main / Kanon-Gate — CI-Verifikation ausstehend

- Kanon-Gate (`phi/canon.φ` + Gate in `commit_check.rs`) steht am Baum; wartet auf
  den grünen `ci-check`-Lauf des gepushten SHA. (Schritt: `ci_manage view
  35314164053` einmal lesen; grün → beide geschlossen.) · `wartend`

## Bau-Queue `blocked parser-def` (nächster Kandidat)

- Register-Heim `phi/blocked_sources.φ`; bau-eigen laut Disposition. Offen:
  `xml` (ARPANSA UV — XML-value-Reader + per-station lat/lon), `fugin` (JOVE
  Cube-Fluss), `zip/fixed-width` (IGRA-2 Radiosonden), `odf` (PDS3-Word-Arm,
  TRK-2-18 ID 1), `drs-fits` (`fits.rs` NAXIS=0 + TUNIT). (Schritt: je Eintrag
  der gemessene nächste Arm; `grind-flash` für Mechanik, `grind-pro` bei
  Format-Urteil.) · `pending`

## Offen (kein Handlungsschritt)

- **Bindings-Prosa** `phi/bindings/{dust-maske,bathymetrie-gebco}.φ` —
  Architektur-Akt mit Operator-/Council-Wort. · `operator-gebunden`
- **`opencode.json`** — fremder uncommitteter Hunk. · `operator-gebunden`
- **Scanned-/bild-only-PDFs → `vision`-OCR** und **`--pdf-text`
  Type0/Identity-H ohne ToUnicode** — kein Bau nötig. · `pending`

## Benchmark

- **NUL-CSV-Parser-Arm** (IA2 TAP) → `grind-flash` (Mechanik-Klasse, kein
  aufgezeichneter Doppel-Lauf nötig): Format vorab gemessen (`\0`-Feld,
  `\n`-Zeile), Arm korrekt, `cargo check` + `cargo check --tests` null Fehler /
  null Warnungen. Burn nicht gemessen (Sub-Agent-Kontext).

## Geteilter Baum — eigener Pfad-Satz

- **Eigener Commit-Pfad:** `src/archivar/extract.rs`, `src/archivar/tests.rs`,
  `phi/blocked_sources.φ`, `docs/zustand/external-state.md`,
  `docs/handover/handover-2026-09-18-bau-folge77.md`,
  `docs/handover/archiv/handover-2026-09-18-bau-folge76.md` (Move).
- **Fremd (nicht anfassen):** `opencode.json`, `AGENTS.md`,
  `docs/SOURCE_PORT.md`, `.github/workflows/harvest.yml`,
  `tools/harvest/src/bin/{gedi_l2a,icesat2_atl03,swot_l2_lr_ssh}_compiler.rs`,
  `tools/register/src/bin/register_lookup.rs`, `docs/handover/post.md`, die
  fremden Handover-Renames/Deletes (entscheid/forschung) und
  `handover-2026-09-18-entscheid-folge46.md`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
