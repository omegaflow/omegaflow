<!--
  title: Handover — Bau-Folge 78 (Stand 2026-09-18)
  session: Bau-Folge 78
  class: handover
  date: 2026-09-18
  sha256: dbfea70f21e1a87bc21e3e25cdfaa130d75e1ca5f825a8615be9838455c7d71a
  status: live
-->
# Handover — Bau-Folge 78 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18, HEAD dd5a7762)

- **Postfach** — `post.md` leer (nur Header), **keine `An bau`-Zeile**; letzter
  `state/mail/mail_ledger.φ`-Eingang `1789689115` (Rubin-Forum, kein
  Agenten-Eingang), beim Session-Read kein neuer Eingang.
- **CI am HEAD** — `te-gate` `35314110831` @`7b67b10d` **pending**;
  `ci-check` `35314164053` @`6a8ecade` **in_progress**; Rest fremde Linien.
  Beide bau-relevanten Läufe beim Session-Start (08:3x) noch nicht entschieden.
- **HEAD** `dd5a7762` (folge76-Move); `origin/main` Vorfahr.
- **Sicherheitsnetz** — `refs/safety/1789713114` (Session-Start).

## FUGIN — Cube-Fluss-Parser fehlt (härtester undatiert)

- `phi/blocked_sources.φ:42-44`; JVO Nobeyama FUGIN `fugin.cube` ist eine
  obscore-Cube-Tabelle, der Kontinuums-Fluss steckt in FITS-Cubes hinter
  `access_url`. (Schritt: `archive_search --verdict <access_url>` je Cube
  messen; FITS-Cube-Fluss-Parser (NAXIS=3, `fits.rs`-Basis) bauen;
  `grind-pro` für das Format-Urteil, `grind-max` falls neuartig.) · `pending`

## Parser-Def-Queue — Rest nach Folge 78

- **`xml`** (ARPANSA UV) — Reader gebaut (`src/archivar/arpansa.rs` `parse_uv_xml`,
  `extract.rs` `format arpansa|uvxml`, Tests). Offen: Compiler + CDN-Registrierung
  in `phi/sources.φ`; per-station lat/lon `pending` (XML trägt keine Koordinaten).
  (Schritt: `arpansa_uv_compiler.rs` nach `drs_fits_compiler.rs`-Vorlage,
  `Position::Source`; Koordinaten-Tabelle nur mit gemessener Quelle.) · `pending`
- **`igra_zip`** — fixed-width-Parser gebaut (`src/archivar/igra.rs`
  `parse_igra`, `extract.rs` `igra_zip`-Arm, Tests; Force-Spiegel sondehub
  `sources.φ:1366-1371`). Offen: `igra_zip`-Binary-Fetch-Routing in
  `main_flow.rs:2843`, `fetch.rs:770`, `port.rs:1780` (csv_zip-Spiegel) vor
  `sources.φ`-Eintrag. (Schritt: die drei Stellen um `igra_zip` erweitern,
  `cargo check`, dann `sources.φ`.) · `pending`
- **`odf`** — Word-Arm geschlossen (`odf.rs:56` fmt=bits(129,131), `:62` fmt==1
  1988-SIS, `:74` else; Tests `:1993`,`:2024`). Offen: `lro_odf`/`pathfinder_odf`/
  `ulysses_odf`-Compiler fehlen (nur `lro_trk`/`ulysses_atdf`). (Schritt:
  `archive_search --verdict` der PDS-ODF-Pfade je Mission, dann Compiler nach
  `galileo_odf_compiler.rs`-Vorlage.) · `pending`

## TE-Gate — Gate-Verdikt ausstehend

- Restrisiken (a)(b) in folge76 geschlossen; offen ist das Gate-Verdikt und der
  Rename (c). (Schritt: `ci_manage view 35314110831` einmal lesen; grün → Rename
  `arx_restricted_surrogate_conditional` nach grünem Gate (Name=Implementation);
  rot → rote Zelle + FN-Arm `found/meas ≥ 0.5` beider Richtungen ist die Messung,
  Design verbreitern, Verweigerung nie aufweichen.) · `wartend`

## Red main / Kanon-Gate — CI-Verifikation ausstehend

- Kanon-Gate (`phi/canon.φ` + Gate in `commit_check.rs`) steht am Baum; wartet auf
  den grünen `ci-check`-Lauf des gepushten SHA. (Schritt: `ci_manage view
  35314164053` einmal lesen; grün → geschlossen.) · `wartend`

## Offen (kein Handlungsschritt)

- **Bindings-Prosa** `phi/bindings/{dust-maske,bathymetrie-gebco}.φ` —
  Architektur-Akt mit Operator-/Council-Wort. · `operator-gebunden`
- **`opencode.json`** — fremder uncommitteter Hunk. · `operator-gebunden`
- **Scanned-/bild-only-PDFs → `vision`-OCR** und **`--pdf-text`
  Type0/Identity-H ohne ToUnicode** — kein Bau nötig. · `pending`

## Benchmark

- **XML-Arm** (ARPANSA) → `grind-flash` $0.0296 (`session_burn`), Format vorab
  gemessen (`<location id>` + 7 Kind-Tags), korrekt; **IGRA-Arm** → `grind-pro`
  $0.1093 — das Force-Gate-Mapping (sondehub-Spiegel) verlangt das pro-Profil,
  kein flash-Doppel nötig: die Parser-Klasse ist durch den NUL-CSV-flash-Sieg
  (folge77) entschieden.

## Geteilter Baum — eigener Pfad-Satz

- **Eigener Commit-Pfad:** `src/archivar/arpansa.rs` (neu), `src/archivar/igra.rs`
  (neu), `src/archivar/extract.rs`, `src/archivar/mod.rs`,
  `src/archivar/tests.rs`, `phi/blocked_sources.φ`,
  `docs/handover/handover-2026-09-18-bau-folge78.md`,
  `docs/handover/archiv/handover-2026-09-18-bau-folge77.md` (Move).
- **Fremd (nicht anfassen):** `AGENTS.md`, `docs/SOURCE_PORT.md`,
  `opencode.json`, `docs/handover/post.md`,
  `tools/register/src/bin/register_lookup.rs`, die fremden
  Handover-Renames/Deletes (entscheid/forschung) und
  `handover-2026-09-18-entscheid-folge46.md`. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
