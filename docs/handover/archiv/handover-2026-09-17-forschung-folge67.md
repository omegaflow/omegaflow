<!--
  title: Handover — Forschung-Folge 67 (Stand 2026-09-17)
  session: Forschung-Folge 67
  class: handover
  date: 2026-09-17
  sha256: 64d43e96b446161618eaec907cbb2ededbf53a3aa3eedb0035c07310ff1086d8
  status: live
-->
# Handover — Forschung-Folge 67 (2026-09-17)

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

- **Postfach** — neue Ledger-Zeilen seit Folge 65: GitHub-Support-Ticket
  **#4761801** aktualisiert (unser Follow-up 2026-09-17 16:27Z, GC weiter offen);
  NSE/Haug-Antwort an Keller gesendet (17.09.); Rubin-Forum-Notifications.
  Kein Dateneingang. Zustand-Eintrag `docs/zustand/external-state.md:20` fällig —
  fremd-modifiziert, der besitzenden Linie überlassen (nicht angefasst).
- **CI-Status @Session-Start `d510d0fd`** (`ci_manage list` + Watchdog-Snapshot;
  HEAD rückte während der Session auf `990b8d79` — fremde ernte-Commits, gepusht):
  `pioneer-cell-census` **`35266366575` in_progress** (der §4-Lauf mit Kreuz-Rang
  + r²-LS, @`98fbaeb1`, seit 19:41Z); der alte Probe-Lauf `35264753611` success.
  `harvest`/`harvest-dispatch`/`paper-check`/`allwise`/`ned` success;
  `swot`/`gedi`/`swpc`/`quake-feeds` failure (gemessen). Zustand-Eintrag
  `docs/zustand/external-state.md:22` bei HEAD-Wechsel fällig — fremd-modifiziert.

## §4 fsky-Census — `wartend` (Lauf läuft)

- Lauf **`35266366575`** (Kreuz-Rang + r²-LS, @`98fbaeb1`) in_progress seit
  19:41Z. Trigger = Lauf-Abschluss. (Schritt: **einmal** lesen —
  `ci_manage view 35266366575`, dann `gh run download 35266366575` +
  `pioneer-cell-census.txt` — Kreuz-Rang + r²-Peak gegen die Zwei-Arm-Frage
  deuten: sub-dominant vorhanden ⇔ Um-Ranken, absent ⇔ in der NOCC-Kette erzeugt;
  bei Mehrdeutigkeit `research-max`.)

## Ulysses-ATDF — Arm gebaut, Manifestation offen

- Die Draft-Annahme „ODF/TRK-2-34" ist **widerlegt**: Ulysses SCE ist TRK-2-25
  ATDF (288-Byte-Records, 28/8064-Block; `2035036A.LBL`). Arm gebaut:
  `src/archivar/atdf.rs` `reduce_uly_skyfreq`/`parse_uly_series` +
  `extract.rs`-Dispatch `ulysses_atdf` + `main_flow.rs`; Compiler
  `tools/harvest/src/bin/ulysses_atdf_compiler.rs`; `phi/harvest.φ`-Block
  `ulysses_atdf` (`asset fehlt`, shard 1). `cargo check` core + harvest grün
  (0 Fehler, 0 Warnungen). (Schritt: Lauf **`35270658104`** @`d101b20a`
  (queued 20:24Z) — bei Abschluss sha256/Größe messen
  (`gh release view pds-ppi.igpp.ucla.edu --repo omegaflow/sources`), den
  `phi/harvest.φ`-Block auf `asset present` flippen + `phi/sources.φ`-Block mit
  sha256 anhängen; **nicht erneut dispatchen**.)
- **X-Band-Recovery-Kette `pending`** — der Korpus trägt keinen X-Multiplier/Offset;
  die S-Band-Kette ist gebaut (DSN-Ground-Segment), X-Records werden gezählt und
  übersprungen (im Diagnostik benannt, nie ein erfundener Wert). (Schritt:
  X-Band-Konstanten aus der Ulysses-DSN-Dokumentation oder Vergleichsliteratur
  messen, dann `reduce_uly_skyfreq` um die X-Kette erweitern.)

## LRO — utF-Parser offen (`blockiert parser`)

- Gemessen: Goddard **Universal Tracking Data Format (utF)**, 75-Byte-Records
  (`lsutdf_005901_103_2009_169_1548.trk`, 3600 B; `.lbl` RECORD_BYTES=75,
  `^STRUCTURE = "LRO_TRK.FMT"`, 28 Spalten). Kein Parser im Baum; die
  Draft-Annahme „TRK-2-34" ist widerlegt. (Schritt: `LRO_TRK.FMT` im
  PDS-`/document/` lokalisieren (404 am geratenen Pfad), das 28-Spalten-Layout
  lesen, dann `lro_odf_compiler.rs` + Register; `grind-max`.)

## GOES-16 — CI-Manifest offen (`blockiert parser`)

- Format sauber (Spiegel `goes_abi`, `GAB1`/netCDF; `src/archivar/goes_abi.rs`
  liest GOES-16-Koeffizienten), aber `goes_abi_compiler.rs` verlangt
  `--input`/`--url` und `harvest.yml` ruft nur `--bin <arm> -- --ci-mode` — ein
  dünner `goes16_abi_compiler` würde einen fehlschlagenden Lauf dispatchen.
  (Schritt: Bucket-Index/Manifest-Fetch in den ABI-Arm bauen (wie die
  ODF-Compiler selbst-fetchen), shared module + dünner `goes16_abi_compiler`
  (tag `noaa-goes16.s3.amazonaws.com`), dann Register — `phi/sources.φ` erst nach
  Manifestation; `grind-max`.)

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

- **Format-Falsifikation** (Ulysses ATDF vs ODF/TRK-2-34; LRO utF vs TRK-2-34)
  durch `grind-pro`: die Recherche fand die widerlegenden `.LBL`/`.FMT`-Messungen —
  eine Routine-Recherche-Klasse, `flash` trüge sie; kein Doppellauf (kein
  registrierter Sieger für diese Klasse, aber keine Kontroverse). Als Handover-Zeile
  notiert.
- **Ulysses-ATDF-Arm** durch `grind-max` (novel parser construction, harte
  Atom-Klasse): Arm + Register in einem Kontext, `cargo check` grün. Kein Doppellauf.
- Lehre (Report-Fälschung) bestätigt: beide Schreib-Aufträge lieferten die
  geforderte Rücklese-Beweispflicht (`sread`/`glob`/`cargo check`).

## Geteilter Baum — eigener Pfad-Satz

- Eigener Commit-Pfad: `src/archivar/atdf.rs`, `src/archivar/extract.rs`,
  `src/archivar/main_flow.rs`,
  `tools/harvest/src/bin/ulysses_atdf_compiler.rs`,
  `phi/pipeline/ledger.φ`,
  `docs/handover/handover-2026-09-17-forschung-folge67.md` (+ archiviertes
  `handover-2026-09-17-forschung-folge66.md`). `phi/pipeline/queue/grind_ulysses.φ`
  ist gitignored.
- `phi/harvest.φ` hat die ernte-Linie committet (`990b8d79`, „add the arm-not-format
  harvest blocks") — mein `ulysses_atdf`-Block ist darin enthalten; die Datei
  gehört daher nicht zu meinem Commit-Pfadsatz.
- Fremde uncommittete/gestagte Arbeit (nicht anfassen): `docs/zustand/external-state.md`,
  `phi/sources.φ`, `.github/workflows/ned-cdn.yml`, die vielen
  `tools/harvest/src/bin/*.rs` (ernte CDN-Void-Fixes),
  `docs/handover/handover-2026-09-17-ernte-folge70.md`, die gestagten
  `archiv/`-Renames. Nie ein nacktes `git commit`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
