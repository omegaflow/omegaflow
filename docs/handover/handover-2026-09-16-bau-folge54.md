<!--
  title: Handover — Bau-Folge 54 (Stand 2026-09-16)
  session: Bau-Folge 54
  class: handover
  date: 2026-09-16
  sha256: aef58fffc9c41849404ae184367c911e3e4c3ee262b417fbf46d3b96770d9085
  status: live
-->
# Handover — Bau-Folge 54 (2026-09-16)

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

## TE-Gate-Arx — das konditionale FP/FN-Gate läuft; die CI-Messung liest der nächste Run

- Run `35139346318` @ `699d9905` (dispatcht 2026-09-16) trägt die konditionalen
  Gates `gate_conditional_arx_fpr_fn_n1000` + `gate_conditional_arx_2_fpr_n1000`;
  die älteren Arx-Runs `35129638318`/`35129679496` tragen `block_sweep_n1000`.
  (Schritt: `gh run view 35139346318` — grün: Punkte zu; rot: der Assert-Text
  nennt die Zelle.)
- Eigenschaft (Council): die FPR-Auflösung bei 100 Trials ist grob —
  Clopper-Pearson-95 %-CI für 8 % ≈ [3,5, 15,2] %, der Anstieg ±3pp; der Null ist
  konservativ (Fit absorbiert c's Vergangenheit, der Schätzer konditioniert nur
  auf das gleichzeitige c[t]).
- `TeNull::Residual` ist gestrichen (2026-09-16): `residual_surrogate_conditional_lagged_n`
  entfernt, die Enum-Variante weg, der pcmci-Benchmark-Arm `--null residual`
  ist ein benannter `exit(1)` (kein stiller Shuffle, kein Remap). Die nicht-gelaggte
  Schwester `residual_surrogate_conditional` (live-Konsumenten) bleibt.

## Phase-Null — Spektral-Placebo

- `TeNull::Phase` bleibt als Spektral-Placebo: Verbraucher nur
  `placebo_pair_eeg_probe`, `rest.rs` und der pcmci-Benchmark-Arm; kein
  Produktions-Signifikanz-Konsument. Das binned n=1000-Gate misst es.
  (Schritt: `gate_fpr_autocorrelation_phase_null_binned_n_1000` im te-gate-Run
  lesen.)

## Block-Null — der Sweep entscheidet

- `multi_force_te_probe.rs:74` läuft auf `TeNull::Block`; Block ignoriert `conds`.
  `block_sweep_n1000` misst, ob eine Blocklänge den FPR-Anstieg ≤ 2pp hält:
  hält eine → sie bleibt; hält keine → Probe auf Arx, Block ausmustern.
  (Schritt: den `block_sweep_n1000`-Abschnitt des te-gate-Runs lesen.)

## DRS-FITS — Granulat korrigiert, CDN-Manifestation offen

- `.github/workflows/drs-fits-cdn.yml:14` zeigt jetzt auf
  `drs_20160102_093513__20160103_165224.fits` (HTTP 200, 47 378 880 B, trägt
  `SCI_SCIENCE_1Hz` mit `ESA00001/2` + `DST11077-79/83-85` — gemessen); die
  `phi/sources.φ` url-line ist entsprechend fortgeschrieben (die alte Aug-Granule
  asset 404te). (Schritt: `gh workflow run drs-fits-cdn.yml`, dann
  `archive_search --sniff` des neuen Assets.)

## Star-Katalog — gaia-cdn Re-Dispatch läuft

- Run `35139352096` (dispatcht 2026-09-16); external-state gaia-Zeile korrigiert
  (75 001 828 B, sha256 `fb9a1408…bcfbb`). (Schritt: `gh run view 35139352096`,
  dann `archive_search --sniff https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/dr3_stars.bin`.)

## Katalog-Konsumenten — zwei pending (fünf descoped registriert)

- `las`: der Header trägt keine CRS — Schritt: den `LASF_Projection`-VLR
  (GeoKeyDirectory 34735 / WKT 2112) in `src/archivar/las/mod.rs:76-92` dekodieren,
  dann registrieren.
- `nexrad_level2`: der Standort ist aufgelöst (`nexrad_site` `src/archivar/nexrad.rs:354`),
  aber der 44-B-Record trägt ihn nicht — Schritt: lat/lon/alt in den Record tragen
  oder am Standort ankern (`nexrad_level2_compiler.rs:199-208`).
- VLASS, `catalog_dcom5`, `des_y6`, `ossos`, `twomrs` sind `descoped` im Register
  (2026-09-16, mit Befund).

## ODR — Serie/Granulat und Semantik

- Offen: Granulat für die 484-Datei-Serie (13,61 GB > 2-GB-Limit); `sample_count`-
  Semantik; Galileo 12-bit kein Record im PPI-Archiv; `year_full` 00–89 `None`;
  Voyager Decimation>1 unverifiziert. (Schritt: Granulat wählen + Workflow erweitern.)

## HRV/Puls-Oszillator-Bindung

- Physischer ESP32-Träger (on hold) + End-zu-End-Test. (Schritt: `src/archivar/hrv.rs`.)

## CI-Format-Gate

- Fremd unformatiert bleiben `src/archivar/{dl3,fits,flac,ifms_agc,tests,units,vtscat}.rs`,
  `src/mathematikerin/s2.rs`, `tools/measure/src/bin/{aia_ladder_probe,trishuli_gauge_probe}.rs`,
  `tools/utils/src/bin/archive_search.rs` und die ernte-eigenen harvest-Compiler.
  (Schritt: rustfmt-Diff aus dem `ci-check`-format-Job anwenden — jede Linie ihre
  eigene Datei.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
