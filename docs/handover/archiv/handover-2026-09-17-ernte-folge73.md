<!--
  title: Handover — Ernte-Folge 73 (Stand 2026-09-17)
  session: Ernte-Folge 73
  class: handover
  date: 2026-09-17
  sha256: 882ab539ea59540ec4f6613b6197679e6c7841d21d60290da5f6feb14db6450e
  status: live
-->
# Handover — Ernte-Folge 73 (2026-09-17)

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

## Stehender Pass (gemessen 2026-09-17)

- **HEAD** `964d18b9`; `origin/main == HEAD` (Fast-Forward). Basis beim Start war
  `ab45b3f3`; eine Fremd-Linie committete während der Session `964d18b9`
  (`entscheid: fold the folge26 ledger orphan …`, folge39) und pushte.
  Fremd im Baum (nicht angefasst): `docs/zustand/external-state.md`, drei
  gestagte Handover-Renames `handover-2026-09-16-*`.
- **CI** — live (`ci_manage list`, 2026-09-17): rosetta `harvest` `35270867738`
  **in_progress** (head `dc9291ad`); `cdn-reconcile` `35273461901` **queued**
  (Dispatch folge72); `harvest-dispatch` `35272157253` success; `harvest`
  `35272163623` success (`catalog_gaia_sso`); mehrere `harvest` failure
  (`ulysses_atdf` „Second witness"); `harvest-dispatch` `35273075476` failure
  (head `dbe647d1`).
- **Postfach** — `state/mail/mail_ledger.φ` ohne neuen Eingang (letzter Eintrag
  2026-09-16); `post.md` trägt keine ernte-Zeile — die in folge72 genannte
  `ephemeris_epm`-Post-Zeile ist am Baum absent (überholt, nicht neu gesetzt).

## Pipeline-Port (härtester undatiert)

- **Roh-Kandidaten-Korpora portieren** — 10 Korpora, ~3.683 Blöcke
  (`sources_new_untested_*` ×7 + `sources_astro/earth/exotic_untested_*`), seit
  diesem Atom als `ausstehend`/`queue` in `phi/pipeline/ledger.φ` registriert;
  offen bleibt die inhaltliche Arbeit `--port + Sweep, dann Disposition`.
  Kleinster zuerst: `sources_new_untested_183l.φ` (19 Blöcke), dann `2k` (176),
  `candidate-staging` (49). (Schritt: `docs/SOURCE_PORT.md`, `phi/pipeline/prompt.φ`;
  `grind-flash`.) · offen

## Harvest-Architektur

- **rosetta_odf Dispatch-Beweis** `termin` — `harvest` `35270867738` in_progress
  (gemessen 2026-09-17); bei success `phi/harvest.φ`-Block-note. Schritt:
  `ci_manage view 35270867738` einmal nach Abschluss. · nicht auswählbar bis Lauf endet
- **ulysses_atdf / lro_trk / goes16_abi — CDN-Manifestation offen** — Assets
  fehlen (HTTP 404 gemessen; keine `phi/sources.φ`-Blöcke). ulysses: `harvest`
  `35270658104` + `35270996845` failure (compile success, „Second witness"-
  Schritt failure). lro/goes: `harvest-dispatch` `35273075476` failure, kein
  `harvest`-Lauf trug das Format. Schritt: `gh run view <id> --log-failed` (bzw.
  `ci_manage view <id>`), Ursache messen, Fix, re-dispatch — `grind-pro`. · offen
- **Fünf Familien-Blöcke** — `gedi_l2a`/`icesat2_atl03`/`swot_l2_lr_ssh`
  `operator-gebunden` (protected-Bucket-403); `dl3_skymap`/`juno_ocru_odf`
  `wartend` (Auslöser = Asset-Messung). · nicht auswählbar
- **rosetta ungelaufene Pfade** `wartend` (Auslöser = Lauf success);
  **`auto-dispatch`** `wartend` (zuletzt falten). · nicht auswählbar

## Quellen-Routen

- **gedi/icesat2/swot protected-Bucket-403** `operator-gebunden` — CMR-Granule →
  direktes `GetObject` oder Operator-Datenabkommen (SWOT-EULA) — `research-max`.
- **`ephemeris_epm`** — fremde Linie (`bau`); die folge72 genannte Post-Zeile ist
  absent (überholt).

## Parser-Gap (am Baum offen, nicht im Vor-Handover)

- **EPA AQS `daily_88101_{year}.zip`** — der `map`-Feld-Key „Arithmetic Mean"
  trägt ein Leerzeichen und ist damit als map-field-Key unschreibbar; `rows` löst
  den Spalten-Index, ankert aber alle Oszillatoren am Frame-Punkt (kein per-row
  lat/lon). Schritt: Parser-Arm für Leerzeichen-führende map-Field-Keys bauen,
  dann `daily_88101_{year}.zip`-Block in `phi/sources.φ` registrieren —
  `grind-pro`. · offen

## Benchmark

- **`grind-flash`** (Register-Lücke, `ledger.φ`) — $0.0077 (`session_burn`);
  10 Korpora registriert, Dateipräsenz + Größe per `archive_search --index`
  gemessen.
- **`grind-flash`** (CDN-Registerduties, `sources.φ`) — $0.0469 (`session_burn`);
  rosetta in_progress (nichts geschrieben), 3 kompilierte Assets 404 (nichts
  geschrieben), 3 reife Kandidaten-CDN-Assets gemessen vorhanden (3 Hunks
  geschrieben).
- Kein pro/max-Lauf (beide Aufgaben routine, flash-first); kein gedoppelter Lauf
  (zwei verschiedene Aufgaben, keine Benchmark-Klasse).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `phi/pipeline/ledger.φ` (Register-Lücke + 3 CDN-Abschlüsse),
  `phi/sources.φ` (OBSEA/sondehub/gliders CDN-Asset-Notizen),
  `docs/handover/handover-2026-09-17-ernte-folge73.md` (+ archiviertes
  `handover-2026-09-17-ernte-folge72.md`).
- **Fremd (nicht anfassen):** `docs/zustand/external-state.md` (modifiziert), die
  gestagten `handover-2026-09-16-*`-Renames. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
