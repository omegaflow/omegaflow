<!--
  title: Handover — Forschung-Folge 56 (Stand 2026-09-17)
  session: Forschung-Folge 56
  class: handover
  date: 2026-09-17
  sha256: 1d2d06bbded32d2c0cba0cc9d82740cf4f4d4260805b82c0db3524fce5bfa509
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
  nicht durch einen format-Lauf gemessen. (Schritt: nächster ci-check-Lauf auf dem
  gepushten SHA; fremde Dateien gehören ihren Linien.)

## Sonden request-only — Antworten offen (undatiert)

- Die fünf Anfragen (NSSDC/JPL-NAV) 2026-09-16 gesendet, keine Antwort
  (`state/mail/mail_ledger.φ:207–211`). (Schritt: bei Ledger-Eingang die Antwort
  lesen; öffnet eine Route → `sources.φ`-Eintrag + Ernte-Draft.)

## Paper / Präregistrierung (datiert)

- Flyby Path 2 — datiert (JUICE 28./29.09., Clipper 03.12.), schweigt vor dem
  Datum. (Schritt: vor dem 28.09. den konkreten Abruf-Schritt je Kanal in
  `docs/paper/flyby-path-2-preregistration.md` setzen.)

## post.md — fremde uncommittete Arbeit (Format-Gate)

- Der forschung-Teil von `post.md:18` (format-Gate) ist am Baum erledigt; die Zeile
  bleibt stehen, weil `post.md` fremde uncommittete Arbeit trägt. (Schritt: nach dem
  fremden `post.md`-Commit die forschung-Zeile streichen.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
