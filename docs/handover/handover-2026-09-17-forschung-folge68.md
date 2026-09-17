<!--
  title: Handover — Forschung-Folge 68 (Stand 2026-09-17)
  session: Forschung-Folge 68
  class: handover
  date: 2026-09-17
  sha256: 519e368f3e49680068b5463c36cb8ce21ef2e64c5c68b5d88d3ce07448dda597
  status: live
-->
# Handover — Forschung-Folge 68 (2026-09-17)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits,
der Arbeitsbaum darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht. Der
Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist der
härteste undatierte); die Session arbeitet so viele ab wie möglich. Wartestellungen
(`wartend`) sind kein Auswahlpunkt — sie nennen nur ihren Auslöser und werden nie
als Handlungsschritt geführt. Jeder Punkt trägt seinen Status-Tag
(`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## Stehender Pass (automatisch, keine Auswahl)

- **Postfach** — keine neue Zeile an die Forschung-Linie: `post.md` trägt nur die
  `An entscheid`-WWLLN-Zeile. Kein neuer externer Eingang.
- **CI-Status @Session-Start `dc9291ad`** (`ci_manage list`/`view`): Ulysses-Lauf
  `35270658104` **failure** — die Ursache ist die Workflow-Syntax
  `timeout-minutes: fromJSON`, nicht der Arm; der Fix `dc9291ad` landete nach dem
  Dispatch. Der Fix löste auto-dispatch aus: `harvest-dispatch 35270851153` +
  `harvest`-Läufe (`35270867738`, `35270996845`); die Ulysses-Manifestation läuft
  ohne eigenes Zutun. Census `35266366575` **cancelled** (19:41→20:26Z), kein
  Nachfolger im Lauf-Fenster. `pii-exposure`/`paper-check` unverändert.
- **Zustand-Ledger** — `docs/zustand/external-state.md` ist fremd-modifiziert
  (uncommittet) — nicht angefasst; die PII-/CI-Zeilen bleiben fremde Hunks.

## Ulysses X-Band-Recovery (`pending`, härtester undatiert)

- Der S-Band-Korpus trägt keinen X-Multiplier/Offset; die S-Band-Kette ist gebaut,
  X-Records werden gezählt und übersprungen (kein erfundener Wert). (Schritt:
  X-Band-Konstanten aus der Ulysses-DSN-Dokumentation oder Vergleichsliteratur
  messen — `archive_search --brave`/`--playwright`, dann `reduce_uly_skyfreq` um
  die X-Kette erweitern; `grind-pro`.)

## LRO utF-Parser — gebaut, Manifestation offen

- Formatdatei lokalisiert (nicht PDS PPI — der geratene Pfad war 404): MIT/imbrium
  `http://imbrium.mit.edu/LRORS/LABEL/LRO_TRK.FMT` (23513 B, HTTP 200; 29 Spalten,
  75-Byte-Records, SIC=59). Arm `src/archivar/lro_utf.rs`
  (`reduce_lro_trk`/`parse_series`, 9 Tests), Dispatch `extract.rs` `lro_trk`,
  Compiler `tools/harvest/src/bin/lro_trk_compiler.rs` (rekursive Listing
  `/LRORS/DATA/TRK/`), `phi/harvest.φ`-Block `lro_trk` (`asset fehlt`). `cargo
  check` grün (0/0). (Schritt: Lauf-Abschluss des auto-dispatchen harvest — sha256
  + Größe messen, `phi/harvest.φ` auf `asset present` flippen + `phi/sources.φ`-Block;
  nicht neu dispatchen.)

## GOES-16 ABI — gebaut, Manifestation offen

- Shared Bucket-Index `src/archivar/goes_abi/s3.rs` (ListBucket `?prefix=ABI-L1b-RadC/`
  ohne Auth, HTTP 200), dünner `tools/harvest/src/bin/goes16_abi_compiler.rs` (tag
  `noaa-goes16.s3.amazonaws.com`), `goes_abi.rs` erweitert (parse_granule/write_bin
  als shared Pfad), `phi/harvest.φ`-Block `goes16_abi` (`asset fehlt`). `cargo check`
  grün. (Schritt: Lauf-Abschluss — sha256/Größe messen, `phi/sources.φ`-Block.)

## §4 fsky-Census — `wartend`

- Lauf `35266366575` (Kreuz-Rang + r²-LS @`98fbaeb1`) **cancelled**, kein
  Nachfolger. Trigger = neuer Lauf-Abschluss. (Schritt: bei neuem Lauf einmal
  `ci_manage view <id>` + `gh run download <id>` → `pioneer-cell-census.txt`;
  Kreuz-Rang + r²-Peak gegen die Zwei-Arm-Frage deuten.)

## BepiColombo — `wartend`

- Radio-Science-Bundle `bc_mpo_more/` lebt, `data/` 404 (Cruise). Trigger =
  `data/`-Manifestation. (Schritt: bei Manifestation PDS4-TNF/ODF-Parser nach dem
  tatsächlichen Datentyp.)

## CDN-Concurrency Follow-up — `wartend`

- Der Prefix-Check in `planetary-odf-cdn.yml` prüft Vollständigkeit nicht;
  `cancel-in-progress: false` bleibt. Trigger = erfolgreicher `mro_odf`-Lauf.
  (Schritt: auf die erwarteten Shard-Namen härten, messbar nach dem Lauf.)

## WWLLN — Lizenz (`operator-gebunden`)

- Thunder-Hour (Zenodo-Spiegel `records/10725446`, CC BY-SA 4.0; Quell-Lizenz
  „research (non-commercial) use"). Post-Zeile `An entscheid` steht. (Schritt: bei
  Operator-Wort den Zenodo-Spiegel in `phi/sources.φ` registrieren.)

## NSE/Haug — Rohdaten zugesagt (`wartend`)

- Keller-Antwort gesendet (17.09.). Trigger = Dateieingang. (Schritt: bei Eingang
  `nse_haug_trisp`-Quelle + Compiler + `sources.φ`; 0-Kanon: kein Asset ohne Datei.)

## Legacy-Konzepte (`operator-gebunden`, hintenangestellt)

- Silence-Map-Probe, vC-Definition L:53, Certainty, TDA/Betti-0, Minkowski als 4.,
  Nostr hinten (`survey-2026-09-17-omegaflow-legacy-konzepte.md`). (Schritt: bei
  Wiederaufnahme den Rat-Erster-Atom bauen — `tools/measure/src/bin/silence_map_probe.rs`.)

## Paper / Präregistrierung (`termin`)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Benchmark

- **LRO utF-Parser + GOES-16 ABI-Manifest** durch `grind-max` (novel parser
  construction / shared bucket-index compiler): Arm + Register in einem Kontext,
  `cargo check` grün (0/0), Rücklese-Beweis (`sread`/`glob`), beide Quell-URLs als
  HTTP 200 nachgemessen. Klassen-Sieger `grind-max` ist bereits registriert
  (Ulysses ATDF) — kein Doppellauf.
- **Format-Falsifikation** (Ulysses ATDF vs ODF/TRK-2-34; LRO utF vs TRK-2-34)
  bleibt flash-taugliche Routine-Klasse: kein registrierter Sieger, keine
  Kontroverse — kein Doppellauf.
- Lehre bestätigt: beide Schreib-Aufträge lieferten die Rücklese-Beweispflicht
  (`sread`/`glob`/`cargo check`).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `src/archivar/lro_utf.rs`,
  `src/archivar/goes_abi.rs`, `src/archivar/goes_abi/s3.rs`,
  `src/archivar/mod.rs`, `src/lib.rs`, `src/archivar/extract.rs`,
  `src/archivar/main_flow.rs`,
  `tools/harvest/src/bin/lro_trk_compiler.rs`,
  `tools/harvest/src/bin/goes16_abi_compiler.rs`,
  `tools/harvest/src/bin/goes_abi_compiler.rs`, `phi/harvest.φ`,
  `phi/pipeline/ledger.φ`, `docs/handover/handover-2026-09-17-forschung-folge68.md`
  (+ archiviertes `handover-2026-09-17-forschung-folge67.md`).
- `phi/harvest.φ` trägt eine fremde parallele Änderung (gaia-Note) — nur der eigene
  `lro_trk`/`goes16_abi`-Hunk gehört in den Commit.
- Fremde uncommittete Arbeit (nicht anfassen): `docs/SOURCE_PORT.md`,
  `docs/handover/post.md`, `docs/zustand/external-state.md`, die gestagten
  `archiv/`-Renames, `docs/handover/handover-2026-09-17-entscheid-folge38.md`.
  Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
