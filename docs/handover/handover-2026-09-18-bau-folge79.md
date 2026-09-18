<!--
  title: Handover — Bau-Folge 79 (Stand 2026-09-18)
  session: Bau-Folge 79
  class: handover
  date: 2026-09-18
  sha256: 4ebd46475ea9676759d0a6e59be783877caf535810a62b11daff9b9c310a1b7d
  status: live
-->
# Handover — Bau-Folge 79 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18, HEAD 4dbe4805)

- **Postfach** — `post.md` leer (nur Header), **keine `An bau`-Zeile**.
  `state/mail/mail_ledger.φ` letzter Eingang `1789689115` (Rubin-Forum, kein
  Agenten-Eingang).
- **CI am HEAD** — Watchdog-Snapshot 08:02: `te-gate` `35313041295`,
  `ci-check` `35309762475` in_progress; `harvest`-Läufe fremder Linien.
  bau-relevante Verdikte ausstehend (s. u.).
- **HEAD** `4dbe4805` (== `origin/main`); Sicherheitsnetz
  `refs/safety/1789714815`.

## FUGIN — Cube-Compiler + CDN ausstehend (härtester undatiert)

- Parser gebaut: `src/archivar/fugin.rs` `parse_fugin_cube` (NAXIS=3
  848x848x462 BITPIX=-32 GLON/GLAT-SFL VRAD CDELT3=650 m/s BUNIT=K → moment-0
  K·m/s, `Position::StateVector` via `galactic_to_icrs`; NAXIS=2=RMS
  übersprungen), `extract.rs` `format fugin_cube`, Tests. Offen: Compiler, der
  je `access_url`-Cube den FITS liest und einen CDN-Bin schreibt, dann
  `sources.φ`-Eintrag + Workflow-Matrix. (Schritt:
  `fugin_compiler.rs` nach `drs_fits_compiler.rs`-Vorlage, Cube-Auswahl
  (MAPID/LINE) je Messung, `convert_to_si` K·m/s, `archive_search --verdict`
  der `access_url`.) · `pending`

## TE-Gate — Gate-Verdikt ausstehend

- Offen ist das Gate-Verdikt und der Rename (c). (Schritt: `ci_manage view
  35314110831` einmal lesen; grün → Rename
  `arx_restricted_surrogate_conditional` nach grünem Gate (Name=Implementation);
  rot → rote Zelle + FN-Arm `found/meas ≥ 0.5` beider Richtungen ist die
  Messung, Design verbreitern, Verweigerung nie aufweichen.) · `wartend`

## Red main / Kanon-Gate — CI-Verifikation ausstehend

- Kanon-Gate (`phi/canon.φ` + Gate in `commit_check.rs`) steht am Baum; wartet
  auf den grünen `ci-check`-Lauf des gepushten SHA. (Schritt: `ci_manage view
  35314164053` einmal lesen; grün → geschlossen.) · `wartend`

## Offen (kein Handlungsschritt)

- **Bindings-Prosa** `phi/bindings/{dust-maske,bathymetrie-gebco}.φ` —
  Architektur-Akt mit Operator-/Council-Wort. · `operator-gebunden`
- **`opencode.json`** — fremder uncommitteter Hunk. · `operator-gebunden`
- **Scanned-/bild-only-PDFs → `vision`-OCR** und **`--pdf-text`
  Type0/Identity-H ohne ToUnicode** — kein Bau nötig. · `pending`

## Benchmark

- FUGIN-Parser → `grind-pro` (fits.rs `FitsImage` + `galactic_to_icrs` lagen
  vor → kein `max` nötig), IGRA-Routing → `grind-flash`, ODF-Verdikt/Bau →
  `grind-pro`, ARPANSA-Koordinaten → `grind-pro`. Kein Doppellauf: die
  Parser-Klasse ist durch den NUL-CSV-flash-Sieg (folge77) entschieden, die
  Routine-Klasse durch die gemessene 8-Profil-Messung (2026-09-16).

## Geteilter Baum — eigener Pfad-Satz

- **Eigener Commit-Pfad:** `src/archivar/fugin.rs` (neu),
  `src/archivar/arpansa.rs`, `src/archivar/extract.rs`,
  `src/archivar/fetch.rs`, `src/archivar/main_flow.rs`,
  `src/archivar/port.rs`, `src/archivar/mod.rs`, `src/archivar/tests.rs`,
  `tools/harvest/src/bin/pathfinder_odf_compiler.rs` (neu),
  `.github/workflows/planetary-odf-cdn.yml`, `phi/sources.φ`,
  `phi/blocked_sources.φ`,
  `docs/handover/handover-2026-09-18-bau-folge79.md`,
  `docs/handover/archiv/handover-2026-09-18-bau-folge78.md` (Move).
- **Fremd (nicht anfassen):** `AGENTS.md`, `docs/SOURCE_PORT.md`,
  `opencode.json`, `docs/handover/post.md`,
  `tools/register/src/bin/register_lookup.rs`, `src/archivar/hdf5.rs`,
  `src/archivar/range.rs`,
  `tools/harvest/src/bin/{swot_l2_lr_ssh,gedi_l2a,icesat2_atl03}_compiler.rs`,
  die fremden Handover-Renames/Deletes (entscheid/forschung) und
  `handover-2026-09-18-{entscheid-folge46,forschung-folge78,ernte-folge79}.md`.
  Nie ein nacktes `git commit`.

## Abschluss

Nach dem Push `planetary-odf-cdn.yml` dispatchen (Workflow geändert:
`pathfinder_odf`-Matrix-Zeile). Vor Commit/Push: das Commit-Wort des Operators
(`/commit`) — der gemessene Abschluss-Check läuft dann mit Commit und Push.
`/consent` ist der session-weite Consent (Delegation), nie das Commit-Wort.
