<!--
  title: Handover — Bau-Folge 55 (Stand 2026-09-16)
  session: Bau-Folge 55
  class: handover
  date: 2026-09-16
  sha256: dbf03a227f92657444ee96cadd46e8368d958f409cf6a00fe8004fe2c755d277
  status: live
-->
# Handover — Bau-Folge 55 (2026-09-16)

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

## TE-Gate-Arx — der pending Lauf trägt die konditionalen Gates

- Der dispatchte Lauf `35139346318` ist `cancelled`; ein te-gate-Lauf steht
  `pending` (`35139361939`, dispatcht 2026-09-16). (Schritt: `gh run view
  35139361939` — grün: Punkte zu; rot: der Assert-Text nennt die Zelle.)
- Phase-Null: `gate_fpr_autocorrelation_phase_null_binned_n_1000` im selben Lauf
  lesen. (Schritt: `gh run view 35139361939`.)
- Block-Null: `block_sweep_n1000`-Abschnitt im selben Lauf lesen — hält eine
  Blocklänge den FPR-Anstieg ≤ 2pp → sie bleibt; hält keine →
  `multi_force_te_probe.rs:74` auf Arx, Block ausmustern. (Schritt: `gh run view
  35139361939`.)

## DRS-FITS — Workflow dispatcht, Messung offen

- Lauf `35143343902` (dispatcht 2026-09-16) trägt das Granulat
  `drs_20160102_093513__20160103_165224.fits` (HTTP 200, 47 378 880 B). (Schritt:
  `gh run view 35143343902`, dann `archive_search --sniff` des neuen Assets.)

## Star-Katalog — gaia-cdn Lauf pending

- Lauf `35139352096` ist `pending` (Asset `dr3_stars.bin`, 75 001 828 B, sha256
  `fb9a1408…bcfbb`). (Schritt: `gh run view 35139352096`, dann `archive_search
  --sniff https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/dr3_stars.bin`.)

## Katalog-Konsumenten — zwei Register-Schulden nach dem Bau

- `las`: der CRS-Dekoder (`projection_crs`, GeoKeyDirectory 34735 / WKT 2112) ist
  gebaut und committet (`ff69e51f`); die Quellen-Registrierung des `las`-Assets
  steht aus. (Schritt: `las`-Quelle in `phi/sources.φ` eintragen.)
- `nexrad_level2`: der Site-Anker (29-B-Block, `present`-Flag, `stid`+lat/lon/alt)
  ist gebaut; das manifestierte `nexrad_level2.bin` ist damit **stale** (alter
  8-B-Kopf). (Schritt: `nexrad_level2_compiler --ci-mode`, dann
  `gh workflow run nexrad-level2-cdn.yml`.)

## ODR — Voyager-Serie sharded, Workflow dispatcht

- Der Serien-Job lädt die 484 `.ODR`-Dateien in ~1-GiB-Shards
  (`voyager_odr_s{ord}.bin`, `SHARD_BUDGET = 1 << 30`); Lauf `35143340703`
  (dispatcht 2026-09-16). Nach dem Lauf die Shard-Zeilen eintragen. (Schritt:
  `gh run view 35143340703`, dann Register-Zeilen in `phi/sources.φ`.)
- Offen bleiben: Galileo 12-bit kein Record im PPI-Archiv; `year_full` 00–89
  `None` (Jahrhundert ungemessen); Voyager Decimation>1 unverifiziert. (Schritt:
  `sgrep "year_full\|decimation" src/archivar/galileo_odr.rs src/archivar/voyager_odr.rs`.)

## CI-Format-Gate — Bau-Core formatiert, fremde bleiben

- Formatiert und committet (`ff69e51f`): `src/archivar/{dl3,fits,flac,ifms_agc,units}.rs`;
  `src/gate/commit_gate.rs` war bereits konform. `src/mathematikerin/s2.rs` und
  `src/archivar/tests.rs` waren von fremder Arbeit geteilt/überschrieben → nicht
  angefasst.
- Fremd unformatiert bleiben `src/archivar/vtscat.rs`,
  `tools/measure/src/bin/{aia_ladder_probe,trishuli_gauge_probe}.rs`,
  `tools/utils/src/bin/archive_search.rs` und die ernte-eigenen harvest-Compiler.
  (Schritt: die jeweilige Linie formatiert ihre eigene Datei.)

## HRV/Puls-Oszillator-Bindung

- Physischer ESP32-Träger (on hold) + End-to-End-Test. (Schritt: `src/archivar/hrv.rs`.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
