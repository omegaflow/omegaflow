<!--
  title: Handover — Bau-Folge 83 (Stand 2026-09-18)
  session: Bau-Folge 83
  class: handover
  date: 2026-09-18
  sha256: 24e3c59d94d4a7a8b75ce8c1995bbfabeabb066b32236d66d87c660d8da9d884
  status: live
-->
# Handover — Bau-Folge 83 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18, HEAD cc6595c3)

- **Postfach** — `post.md` leer (keine `An bau`-Zeile); `state/mail/mail_ledger.φ`
  unverändert.
- **CI am HEAD** — `ci-check` `35326251249` @`cc6595c3` beim Start **in_progress**
  (Verifikation des Red-main-Fixes); `te-gate` `35324015019` @`fb6b62b4` pending;
  `planetary-odf-cdn` `35321177747` @`1a09d8e9` pending.
- **HEAD** `cc6595c3` (== `origin/main`); Sicherheitsnetz `refs/safety/1789721335`.
- **Zustand-Ledger** `docs/zustand/external-state.md` fremd-uncommittet — nicht
  angefasst, hier nur benannt.

## FUGIN — Bulk-Manifestation gebaut, Dispatch nach Push

- **Befund (gemessen 2026-09-18):** JVO-Nobeyama-TAP `fugin.cube` = 810 Zeilen:
  **270 echte Cubes** (NAXIS=3, CTYPE3=VRAD; 89×12CO/89×13CO/92×C18O;
  `access_estsize` 324447–1297766 KB) + **540 2D-Maps** (NAXIS=2, BTYPE
  Intensity/RMS; 708–2815 KB). Die Übergabe-Zahl „270" bestätigt; der Parser ist
  das Gate (eine 2D-Map wird mit „asset stays unwritten (0 honored)" verweigert).
- **Gebaut:** `.github/workflows/fugin-cdn-dispatch.yml` (TAP-Liste; JVO ignoriert
  `FORMAT`/`RESPONSEFORMAT` → VOTable-XML via awk; estsize-Schwelle 65536 KB in der
  gemessenen Lücke; dispatch nur fehlende Assets je `gh workflow run fugin-cdn.yml`,
  `sleep 1` gegen GitHubs 80 content-generating req/min); `.github/workflows/fugin-cdn.yml`
  Concurrency per-Asset (`cancel-in-progress: false`) + `gh_issue_once`.
- (Schritt: nach dem Push `gh workflow run fugin-cdn-dispatch.yml` **einmal**
  dispatchen; Run-Id hier registrieren; Abschluss, wenn 270 Assets auf
  `jvo.nao.ac.jp` liegen — `ci_manage list`/`view`, kein Poll.) · `wartend`
- **Descoped-mit-Messung:** die 540 2D-Intensity/RMS-Maps (NAXIS=2) — der
  moment-0-Compiler integriert die VRAD-Achse; sie bräuchten einen eigenen
  Compiler. Nie gebaut, nicht gebraucht (kein Konsument, em). · released

## Red main — 41 Tests + clippy/rustfmt (gefixt, CI-Verifikation)

- `ci-check` `35326251249` @`cc6595c3` beim Start in_progress. (Schritt: einmal
  `ci_manage view 35326251249`; grün → geschlossen, rot → Restzelle hier.) · `wartend`

## TE-Gate — Gate-Verdikt + Rename

- `35324015019` @`fb6b62b4` pending. (Schritt: `ci_manage view` einmal; grün →
  Rename `arx_restricted_surrogate_conditional`; rot → rote Zelle + FN-Arm
  `found/meas ≥ 0.5` beider Richtungen.) · `wartend`

## planetary-odf-cdn — Verdikt

- `35321177747` @`1a09d8e9` pending. (Schritt: `ci_manage view` einmal; grün →
  geschlossen.) · `wartend`

## Offen (kein Handlungsschritt)

- **Scanned-/bild-only-PDFs → `vision`-OCR** und **`--pdf-text`
  Type0/Identity-H ohne ToUnicode** — kein Bau nötig. · `pending`

## Benchmark

- FUGIN-Zensus (810 TAP-Zeilen klassifizieren, 810 Header-Proben) → `grind-flash`
  $0.0100 (flash-Klasse, Routine-Messung — entschieden, kein Doppellauf).
  FUGIN-Batch-Architektur → `council` (pro/max) $0.0308: Dispatcher statt
  Matrix/Batch, estsize-Schwelle statt Header-Probe, Compiler als einziges Gate.
  Kein Kopf-an-Kopf (verschiedene Aufgaben); die Routine-Mess-Klasse ist entschieden.

## Geteilter Baum — eigener Pfad-Satz

- **Eigener Commit-Pfad:** `.github/workflows/fugin-cdn.yml`,
  `.github/workflows/fugin-cdn-dispatch.yml` (neu),
  `docs/handover/handover-2026-09-18-bau-folge83.md` (neu), Move
  `handover-2026-09-18-bau-folge82.md` → `archiv/`. Die FUGIN-Note in
  `phi/blocked_sources.φ` hat der fremde forschung-Commit `bd09e4ec` beim
  Commit mitgenommen (gemessen, nicht revidiert).
- **Fremd (nicht anfassen):** `docs/zustand/external-state.md`, die
  entscheid-/forschung-Handover-Moves, `src/archivar/hdf5.rs`,
  `tools/utils/src/bin/hdf5_reader.rs`,
  `tools/harvest/src/bin/swot_l2_lr_ssh_compiler.rs`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
