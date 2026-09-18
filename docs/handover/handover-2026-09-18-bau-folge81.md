<!--
  title: Handover — Bau-Folge 81 (Stand 2026-09-18)
  session: Bau-Folge 81
  class: handover
  date: 2026-09-18
  sha256: 0da3d24c9e2dbbd1a68d964c2e45ec5fb867001549f51fa9f9c3264c8f2f9fb8
  status: live
-->
# Handover — Bau-Folge 81 (2026-09-18)

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

## Stehender Pass (gemessen 2026-09-18, HEAD fb6b62b4)

- **Postfach** — `post.md` leer (keine `An bau`-Zeile); `state/mail/mail_ledger.φ`
  letzter Eingang `1789718159` (SuperDARN-Canada-Access-Request, kein
  Agenten-Eingang).
- **CI am HEAD** — Watchdog-Snapshot 09:06: `ci-check` `35314164053` und
  `te-gate` `35313041295` in_progress; bau-relevante Verdikte ausstehend (s. u.).
- **HEAD** `fb6b62b4` (== `origin/main`); Sicherheitsnetz
  `refs/safety/1789718407` (Session-Start).
- **Zustand-Ledger** `docs/zustand/external-state.md` ist fremd-uncommittet
  geändert — nicht angefasst, hier nur benannt.

## FUGIN — Compiler gebaut, CDN-Manifestation ausstehend (härtester undatiert)

- `tools/harvest/src/bin/fugin_skymap_compiler.rs` gebaut (Council-Verdikt A:
  SKY1-Reuse, kein neues Format): `parse_fugin_cube` → `SkymapRecord { kind
  generic, value moment-0 K·m/s je nativem Pixel }` via `pixel_of`, signed-Gate
  (negative Rauschpixel bleiben echte Messungen), `--input <fits> | --url <https>`,
  `--out`, `--ci-mode` → `upload_release("jvo.nao.ac.jp", path)`; Roundtrip-Re-Read.
  Registriert: `phi/sources.φ` (`fgn00000000.sky1`, `format sky1`, Feld
  `fugin_moment0`), `phi/witnesses.φ` (`record sky1`); Workflow
  `.github/workflows/fugin-cdn.yml` (`workflow_dispatch` url/asset).
  TAP gemessen (HTTP 200): **270 Cubes** (89×12CO, 89×13CO, 92×C18O); der reale
  FITS-Header (`FGN00000000`, 848×848×462, BITPIX −32, GLON/GLAT-SFL, VRAD,
  BUNIT K) ist parser-kompatibel (`archive_search --sniff` magic `fits`).
  `cargo check --workspace` 0 Fehler / 0 Warnungen. (Schritt: `gh workflow run
  fugin-cdn.yml` dispatcht, `ci_manage view <id>` **einmal** lesen; grün →
  Manifestation geschlossen, rot → Verdikt ins Handover. Danach die restlichen
  269 Cubes je Messung registrieren — Workflow-Inputs `url`/`asset`.) · `pending`

## FUGIN — `blocked_sources.φ`-Eintrag fortschreiben

- Der FUGIN-Eintrag `phi/blocked_sources.φ:39–41` trägt noch „Offen:
  sources.φ-Compiler+CDN". Die Datei ist fremd-uncommittet geändert — nicht
  angefasst (Write-Boundary). (Schritt: sobald die fremde Änderung committet ist,
  den Eintrag auf CDN-ausstehend fortschreiben bzw. nach grüner Manifestation
  entfernen.) · `blockiert`

## TE-Gate — Gate-Verdikt ausstehend

- Offen ist das Gate-Verdikt und der Rename (c). (Schritt: `ci_manage view
  35314110831` einmal lesen; grün → Rename
  `arx_restricted_surrogate_conditional`; rot → rote Zelle + FN-Arm
  `found/meas ≥ 0.5` beider Richtungen, Design verbreitern.) · `wartend`

## Red main / Kanon-Gate — CI-Verifikation ausstehend

- Kanon-Gate (`phi/canon.φ` + Gate in `commit_check.rs`) steht am Baum; wartet
  auf den grünen `ci-check`-Lauf. (Schritt: `ci_manage view 35314164053` einmal
  lesen; grün → geschlossen.) · `wartend`

## CI-Dispatch

- `planetary-odf-cdn.yml` `35321177747` @`1a09d8e9` (pathfinder_odf-Matrix-Zeile)
  dispatcht; Verdikt ausstehend. (Schritt: `ci_manage view 35321177747` einmalig
  lesen.) · `wartend`

## Offen (kein Handlungsschritt)

- **Scanned-/bild-only-PDFs → `vision`-OCR** und **`--pdf-text`
  Type0/Identity-H ohne ToUnicode** — kein Bau nötig. · `pending`

## Benchmark

- FUGIN-Compiler → `grind-pro` (Parser `fugin.rs` + SKY1-Muster lagen vor; das
  Council-Verdikt A entschied das Format). Kein Doppellauf: die Parser-Klasse ist
  durch den NUL-CSV-flash-Sieg (folge77), die Routine-Klasse durch die gemessene
  8-Profil-Messung (2026-09-16) entschieden.

## Geteilter Baum — eigener Pfad-Satz

- **Eigener Commit-Pfad:** `tools/harvest/src/bin/fugin_skymap_compiler.rs` (neu),
  `.github/workflows/fugin-cdn.yml` (neu), `phi/sources.φ` (eigene Zeilen
  10168–10175), `phi/witnesses.φ` (eigene Zeilen 103–107),
  `src/archivar/units.rs` (`k.m/s` im em-Register), `src/archivar/tests.rs`
  (Assertion), `docs/handover/handover-2026-09-18-bau-folge81.md` (neu), Move
  `docs/handover/handover-2026-09-18-bau-folge80.md` → `archiv/`.
- **Fremd (nicht anfassen):** `phi/blocked_sources.φ`,
  `docs/zustand/external-state.md`, `src/archivar/hdf5.rs`,
  `tools/utils/src/bin/hdf5_reader.rs`,
  `tools/harvest/src/bin/swot_l2_lr_ssh_compiler.rs`, die fremden
  Handover-Renames/Deletes (entscheid/forschung) und
  `handover-2026-09-18-{entscheid-folge48,49}.md`. Nie ein nacktes
  `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
