<!--
  title: Handover — Forschung-Folge 56 (Stand 2026-09-17)
  session: Forschung-Folge 56
  class: handover
  date: 2026-09-17
  sha256: e8977ea6cc5423f220f12d0c98fdaca62f3a3093fe7419fb185e10aca2e7f16c
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

## Tonga Blatt + Siegel — Rat steht, Blatt offen (härtester undatiert)

- Der Rat (2026-09-17): **Blatt ja, Siegel ja — mit benannter Grenze.** Das Siegel
  deckt nur die Fenster-Werte am einen Ohr (Kwajalein); die unabhängigen
  Detektionskanäle (Kyoto-Wellenform, BGR-Detektion) bleiben ungesiegelt; beide sind
  `pending` mit Schritt, nicht `absent`. Die überholte Zeile „220115.txt absent" wird
  im Blatt durch die Messung ersetzt (A = A; sie wäre Fabrikation). Die
  Global-Extrema-Methode aus Handover 54 bleibt verworfen (3.67 hPa/m war kein Lamb).
- Gemessen (CI-Artefakt `tonga-lamb-crosscheck`, SHA 321925d8): Druck-Fenster-Peak
  +2.29 hPa bei 07:42:00, measured−predicted **208 s**; Wasser-Trog −0.659 m bei
  10:00:00, Luft→Wasser-Lag 8280 s; Kopplung **3.48 hPa/m** (ein Ohr, Fenster-Extrema).
- Probe + Workflow korrigiert: `KYOTO_DAY_MEMBER` = `data/220115.txt` (gemessen: das
  Archiv trägt `data/22MMDD.txt`); Workflow holt den BGR-IS52-2022-Bin vor dem Lauf
  vom CDN (`GH_TOKEN` im Job-env). (Schritt: nach Push `gh workflow run
  tonga-lamb-crosscheck.yml`, dann `gh run view <id>` + Artefakt lesen; die
  vollständigen Kanäle sind das Blatt-Material, dann das Blatt unter
  `docs/paper/tonga-lamb-crosscheck.md` mit dem Rat-Wortlaut als Grenze schreiben.)

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

## Kyoto-Zenodo — Quelle gehoben, Manifestation offen

- Gemessen: Zenodo 8098323 trägt 30 Member `data/220102.txt` … `data/220131.txt`
  (220101 fehlt laut Zenodo-Beschreibung: Windows-PC-Ausfall am 1.1.2022); Spalten
  `#YYYY MM DD HH MM SS.SSS TEMP PRES HUMID` (PRES Spalte 7, hPa).
  Compiler `tools/harvest/src/bin/kyoto_pressure_compiler.rs` gebaut;
  `phi/sources.φ:6256–6260` von `reference` auf `format kyoto_pressure` gehoben
  (`at earth`, `field surface_pressure_hpa … gaussian-inverse-square acoustic hPa 300`);
  Workflow `.github/workflows/kyoto-pressure-cdn.yml` angelegt. `cargo check
  --workspace` clean. (Schritt: nach Push `gh workflow run kyoto-pressure-cdn.yml`,
  dann `gh run view <id>` — `kyoto_pressure.bin` auf release `zenodo.org` prüfen.)

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
