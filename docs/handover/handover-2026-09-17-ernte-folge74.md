<!--
  title: Handover — Ernte-Folge 74 (Stand 2026-09-17)
  session: Ernte-Folge 74
  class: handover
  date: 2026-09-17
  sha256: 559f3a7e0af447264fe99ae6067f9e5aace7d59d5d71952d75f015637d90cb67
  status: live
-->
# Handover — Ernte-Folge 74 (2026-09-17)

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

- **HEAD** `3a498e3c` beim Start; während der Session pushte die `bau`-Linie
  `4ed4e2d6` + `5fb16b35` (Handover-68-Rotation) → HEAD == `origin/main` ==
  `5fb16b35`. Fremd im Baum (nicht angefasst): `docs/concepts/tools-map.md`
  (modifiziert), die fremde archive_search-Arbeit
  `tools/utils/src/bin/archive_search.rs` +
  `tools/utils/src/bin/archive_search/arxiv_src.rs`, die drei gestagten
  `handover-2026-09-16-*`-Renames.
- **CI** — Watchdog-Snapshot 22:26 + `ci_manage view`: rosetta `harvest`
  `35270867738` **in_progress** (head `dc9291ad`); die drei
  Manifestations-Läufe (ulysses `35270658104`/`35270996845`, harvest-dispatch
  `35273075476`) sind rot und werden von diesem Atom gefixt.
- **Postfach** — keine ernte-Zeile in `post.md`; kein neuer Agenten-Eingang
  (`external-state.md` Postfach-Zeile @2026-09-17).

## Pipeline-Port (härtester undatiert)

- **Roh-Kandidaten-Korpora portieren** — 10 Korpora, ~3.683 Blöcke. Der
  `--port`-Konverter war defekt und ist in diesem Atom gefixt: `src/archivar/port.rs`
  — tau aus `ttl/10` (gemessen an `master_converted.φ`: 14.579 von 14.579
  Extraktzeilen tragen tau == ttl/10), Frame `on earth <lat> <lon> <alt>`
  (alt 0.0 als Platzhalter, Position kommt aus den Keys), `flatten`-Arm,
  ttl-Doppel-Emission entfernt. Offen bleibt: (a) die Korpora tragen kein
  `force`-Direktiv → `default_kernel_for("")` = None → jeder `field_in` fällt;
  pro Block müssen `force`/Kernel/Unit gesetzt werden (Oszillator-Gate); (b) das
  lokale Release-Binär `target/release/omegaflow` ist stale — lokaler Build ist
  verbannt, kein CI-Workflow baut/verteilt das Binär, also braucht der Re-Port
  ein frisches Binär. Kleinster Korpus `sources_new_untested_183l.φ` (19;
  mechanisch portiert 1 `lastline`-Block, der an non-JSON-Body probe-voidet).
  (Schritt: `force` je Block in `phi/pipeline/queue/` setzen, dann mit frischem
  Binär `--port` + `--probe`; `grind-pro`.) · offen
- **Ledger-Noten der 10 Korpora** — tragen den alten „--port + Sweep"-Schritt,
  der force-Gap ist unbenannt. (Schritt: nach dem force-Setzen die `note` auf den
  gemessenen Stand ziehen.) · offen

## Harvest-Architektur

- **ulysses_atdf / lro_trk / goes16_abi — CDN-Manifestation** `wartend`
  (Auslöser = CI-Läufe). Fixes stehen: `src/archivar/atdf.rs` (Band-Feld Item 11
  = DOWNLINK_BAND statt 10 = STATION, Regressionstest) — committet in `7bf17ada`
  (forschung-Linie, die den X-Band-Arm baute); `phi/harvest.φ` (`lro_trk`
  alphabetisch vor `ulysses_atdf` — das `harvest_reg`-Gate brach den Dispatch ab);
  drei `phi/sources.φ`-Blöcke registriert (`goes16_abi` → `format goes_abi`).
  Dispatched 2026-09-17: `harvest` `ulysses_atdf` `35277931000`, `lro_trk`
  `35277933828`, `goes16_abi` `35277937121` (plus auto `harvest-dispatch`
  `35277699933`). (Schritt: `ci_manage view <id>` einmal; bei success sha256/Größe
  messen, `asset present` in `phi/harvest.φ` + sha256 in `phi/sources.φ`
  nachtragen.) · wartend
- **rosetta_odf Dispatch-Beweis** `termin` — `harvest` `35270867738` in_progress
  (head `dc9291ad`). (Schritt: `ci_manage view 35270867738` einmal nach Abschluss.) · wartend
- **Fünf Familien-Blöcke** — `gedi_l2a`/`icesat2_atl03`/`swot_l2_lr_ssh`
  `operator-gebunden` (protected-Bucket-403); `dl3_skymap`/`juno_ocru_odf`
  `wartend` (Auslöser = Asset-Messung). · nicht auswählbar
- **rosetta ungelaufene Pfade** `wartend` (Auslöser = Lauf success);
  **`auto-dispatch`** `wartend`. · nicht auswählbar

## Quellen-Routen

- **gedi/icesat2/swot protected-Bucket-403** `operator-gebunden` — CMR-Granule →
  direktes `GetObject` oder Operator-Datenabkommen (SWOT-EULA) — `research-max`.
- **`ephemeris_epm`** — fremde Linie (`bau`).

## Parser-Gap / Register

- **EPA AQS** — geschlossen: per-row lat/lon existiert seit `2f5a83ea`
  (`src/archivar/extract.rs` `resolve_col`), der Block ist registriert
  (`phi/sources.φ:1352`, `rows` + lat 5/lon 6/epoch 11 + field 16 per
  Spalten-Index); der verbatim-Key-Arm (`src/archivar/parse.rs` `split_directive`,
  Keys in `"..."`) ist gebaut; `docs/specs/sources-v2-spec.md` §1 dokumentiert
  die Konvention; der `phi/pipeline/ledger.φ`-Eintrag ist auf `disponiert`
  gezogen. Rest: den `disponiert`-Eintrag im nächsten Pass entfernen; das
  `{year}`-Template liefert 2026, der neueste File ist 2025 (Lag). (Schritt:
  Ledger-Eintrag streichen.) · offen (klein)

## Benchmark

- `grind-flash` (Pipeline-Port-Diagnose) fand den `--port`-Konverter-Defekt
  (37/38 Blöcke gedroppt); `grind-pro` (EPA-AQS-Parser-Arm) baute den
  verbatim-Key-Tokenizer; `grind-pro` (CDN-Failure-Diagnose) fand das
  atdf-Bandfeld (Item 10 vs 11) und das `harvest_reg`-Order-Gate. Drei
  verschiedene Aufgaben — kein gedoppelter Lauf, keine Benchmark-Klasse
  geschlossen. Kandidat für eine Dopplung: CDN-Failure-Diagnose flash gegen pro
  (ungeprüft).
- Der `--port`-Fix ist per `cargo check` (0/0) verifiziert; die Korpora-Portierung
  selbst ist noch nicht am Baum.

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `src/archivar/atdf.rs`, `src/archivar/parse.rs`,
  `src/archivar/port.rs`, `src/archivar/tests.rs`, `phi/harvest.φ`,
  `phi/sources.φ`, `phi/pipeline/ledger.φ`, `docs/specs/sources-v2-spec.md`,
  `docs/handover/handover-2026-09-17-ernte-folge74.md` (+ archiviertes
  `handover-2026-09-17-ernte-folge73.md`).
- **Fremd (nicht anfassen):** `docs/concepts/tools-map.md`,
  `tools/utils/src/bin/archive_search.rs`,
  `tools/utils/src/bin/archive_search/arxiv_src.rs`, die drei gestagten
  `handover-2026-09-16-*`-Renames. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
