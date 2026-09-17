<!--
  title: Handover — Forschung-Folge 56 (Stand 2026-09-17)
  session: Forschung-Folge 56
  class: handover
  date: 2026-09-17
  sha256: 6a300bb2803bafaffb88e1de32af9424d808b977b967ee31c17aab12db80c28e
  status: live
-->
# Handover — Forschung-Folge 56 (2026-09-17)

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

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## ODF-Bande-Split `mro_odf` (Fix gebaut, CI-Lauf offen)

- Ursache gemessen (Job-Log 105089344016, run 35186417213): der Runner stirbt mit
  `##[error]The runner has received a shutdown signal` nach 7m58s, zweimal identisch
  (auch run 35159180825, 7m04s) — kein Daten-/Parse-Fehler. Der Compiler
  `tools/harvest/src/bin/mro_odf_compiler.rs` hielt alle 3554 Dateien in einem
  `merged: Vec<[f64; 9]>`: ~2.1·10⁸ rows × 72 B ≈ 15 GB resident vor dem ersten Shard
  → OOM auf dem 16-GB-Runner.
- Fix gebaut: `harvest_all` (globaler Merge) → `harvest_stream` (Chunks von `WORKERS`
  Dateien, je Datei sortiert, Streaming-Callback) + `flush_shard` (bei
  `PODF_SHARD_BUDGET` einen Shard schreiben/verifizieren/flushen); Speicher jetzt
  ~2 GB (ein Shard-Puffer + Chunk). Ein-Shard-Fall schreibt weiter `mro_odf.bin`.
  `cargo check -p omegaflow-harvest` clean. (Schritt: nach Push `gh workflow run
  planetary-odf-cdn.yml`, dann `gh run view <id>` + Job-Log; bei Erfolg den
  gedruckten `mro_odf_*`-Shard-φ-Block nehmen und den Ganzdatei-Block
  `phi/sources.φ:6763–6767` durch je einen Block je Shard ersetzen — Muster
  odyssey_odf 6769+; `refuse_shard_overlaps` verweigert Überlappung gleichen `format`.)

## CDN-Concurrency — Fix gebaut, Follow-up offen

- Fix (2026-09-17, Rat): der workflow-level Concurrency-Block der 6
  release-schreibenden Matrix-Workflows (`planetary-odf-cdn`, `gaia-xp-full-cdn`,
  `noaa-{isd,gsod,ghcn}-allstations-cdn`, `physionet-cdn`) ist nach **job-level**
  gezogen, Gruppe = die konkurrierende Ressource: `${{ github.workflow }}-${{ matrix.netloc }}-${{ matrix.asset }}`
  (planetary-odf), `${{ github.workflow }}-${{ matrix.chunk }}` (gaia-xp-full),
  `noaa-*-assets-${{ inputs.year }}-${{ inputs.month }}-${{ matrix.shard }}`
  (allstations), `physionet-cdn-<job>[-${{ matrix.chunk }}]` (physionet).
  `cancel-in-progress: false` unverändert. Ursprung gemessen: `08cbdb74` flachte die
  im Commit-Text genannte „same-asset namespace pairs"-Form auf `${{ github.workflow }}` ab.
  (Schritt: nächster Dispatch — ein Re-Dispatch läuft nicht mehr hinter einem fremden Job.)
- Regress gefixt (Rat, eine Wurzel): `08cbdb74` setzte auch `ci-check.yml`
  `cancel-in-progress` von `false` (Lektion `f8cdca96`: ein Lauf muss fertig werden,
  377 s → 22,6 s) zurück auf `true` → das Format-/clippy-Gate hungerte aus (seit
  `bc9d6a0b` kein vollendeter Lauf). Jetzt `false`. Lesson-Lock:
  `tools/register/src/bin/concurrency_contract.rs` (Register-Test, läuft in
  `cargo test -p omegaflow-register` im ci-check) hält `ci-check: false` + job-level
  für die 6 Matrix-Workflows, damit kein dritter Mass-Pass die Lektion überschreibt.
- Offen: `cancel-in-progress: false` schützt das Shard-Set, hält aber Stale fest. Auf
  `true` erst, wenn der `sharded`-Check **Vollständigkeit** statt Prefix prüft
  (`grep -q "^mro_odf_"` übersieht ein halbes Set). (Schritt: den Prefix-Check in
  `planetary-odf-cdn.yml` auf die erwarteten Shard-Namen härten — messbar erst nach
  einem erfolgreichen `mro_odf`-Lauf.)

## Format-Gate

- Forschung-eigene Dateien formatiert (Diffs aus CI-format-Lauf 35185912267
  übernommen): `src/archivar/fetch.rs`, `src/archivar/parse.rs`,
  `tools/measure/src/bin/tonga_lamb_crosscheck_probe.rs`. Offen bleiben die fremden
  Dateien desselben Laufs: `src/archivar/kcdc.rs:425` (ernte),
  `src/archivar/odf.rs:425` (bau), `tools/harvest/src/bin/{cassini,dart}_tnf_compiler.rs`,
  `tools/harvest/src/bin/las_compiler.rs`, `tools/register/src/bin/cdn_reconcile.rs:310`,
  `tools/utils/src/bin/archive_search.rs:437`. Die zwei neuen Kyoto-Dateien sind noch
  nicht durch einen format-Lauf gemessen. (Schritt: mit dem ci-check-Fix läuft der
  nächste Lauf durch — `gh run view` des ci-check auf dem gepushten SHA; fremde Dateien
  gehören ihren Linien.)

## Tonga-Blatt — benannte Pendings (§6)

- Drei im Blatt `docs/paper/tonga-lamb-crosscheck.md` §6 benannte Grenzen, jetzt als
  eigene Punkte (kein Run-Warten, echte Forschung):
  - **Ein Ohr** — die Kopplung 3.48 hPa/m ist an einer Station (Kwajalein) gelesen; ein
    zweites co-lokales Luft+Wasser-Paar fehlt. (Schritt: NOAA-CO-OPS-Stationen mit
    `air_pressure`+`water_level` im Lamb-Fenster sondieren — `archive_search`, CO-OPS-API.)
  - **BGR-Fensterbreite** — die direkte Detektion liegt 1360 s nach der Vorhersage; die
    Probe nimmt die nächste Detektion, nicht eine matched-filter/Array-Slowness-Ankunft.
    (Schritt: den BGR-Bin auf Slowness/Azimut-Solution prüfen — `tools/measure`, `geo.rs`.)
  - **Kyoto-Baseline** — die Kyoto-Extrema sind ohne Vor-Ankunfts-Baseline gelesen; eine
    Mehr-Tages-Baseline fehlt. (Schritt: `kyoto_pressure.bin` über mehrere Tage gegen die
    Ankunft legen — der Compiler trägt alle Member.)

## Docs-Pendings — Sichtung 2026-09-17 (jetzt machbar)

Sichtung von `docs/{concepts,paper,surveys}/` gegen die Entblockungen dieser Session
(ODF-Streaming, CDN-Concurrency, ci-check, Kyoto). Je Punkt die Fundstelle (read site)
+ nächster Schritt; die Blockade-Attribution ist die Sichtung, nicht neu gemessen.

- `docs/surveys/survey-2026-09-17-sonden-request-only.md:38` — Juno: Abgleich
  `juno_odf.bin` (post-EFB OCRU, PDS3 `JNOGRV_0001`) ↔ Bestand offen. (Schritt: die
  zwei merged-ODF-Stände gegen die Ernte legen.)
- `…:68–69` — Pioneer-ATDF dtype-12-Arm (`src/archivar/atdf.rs:503/636` hält nur
  dtype 1|2; `pioneer10_doppler_tracking_SC_23.asc` trägt DTYPE 12/13) + Juno-OCRU.
  (Schritt: dtype-12-Zweig mit gemessener Semantik bauen, dann Re-Harvest-Abgleich.)
- `docs/concepts/positive-maske.md:76` — Galileo-ODF Format 1 vs 2 ungemessen (kein
  lokales `galileo_odf.bin`). (Schritt: über `planetary-odf-cdn` ernten, Format messen.)
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md:57` — SuperDARN
  FITACF: erster CI-Manifest-Lauf offen. (Schritt: SuperDARN-CDN dispatchen.)
- `…:56` — NEXRAD Level II: Feld-System-Reader + CI-Manifest offen. (Schritt: Manifest
  dispatchen; Reader-Stand messen.)
- `docs/concepts/die-akteure-im-boden-und-wasser.md:68` — ETOPO1-Gitter (395 MB)
  manifestieren. (Schritt: Manifest-Workflow dispatchen.)
- `docs/surveys/survey-2026-09-13-weberin-quellen.md:188` — WWLLN netCDF: Compiler nur
  Gerüst (`netcdf.rs`); Manifest offen. (Schritt: Compiler fertigstellen, dann Manifest.)
- `docs/surveys/survey-2026-09-16-sonden-flotte.md:57–59` — Ulysses/BepiColombo/LRO:
  kein Register-Eintrag. (Schritt: `phi/sources.φ`-Eintrag + CDN.)
- Weiter blockiert (nicht durch die Session entblockt): `…kapitulationen…:51–55`
  GOES/Himawari (GSICS-Kalibrierung), GDP (`.zarr`), OCS (`.tif`-LZW), WOD (SOHM) —
  Parser/Kalibrierung; `…:65–66` LASzip-Chunk-Dekoder; `…:63–64` AQS/Babamul (Key/Account).

## Sonden request-only — Antworten offen (undatiert)

- Die fünf Anfragen (NSSDC/JPL-NAV) 2026-09-16 gesendet, keine Antwort
  (`state/mail/mail_ledger.φ:207–211`). (Schritt: bei Ledger-Eingang die Antwort
  lesen; öffnet eine Route → `sources.φ`-Eintrag + Ernte-Draft.)

## Paper / Präregistrierung (datiert)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
