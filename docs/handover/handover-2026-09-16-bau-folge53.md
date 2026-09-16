<!--
  title: Handover — Bau-Folge 53 (Stand 2026-09-16)
  session: Bau-Folge 53
  class: handover
  date: 2026-09-16
  sha256: 2d9ecfa8800315d8a91f4897704bf67181279f8ca1dbbf3dbfe8a252687fa0ab
  status: live
-->
# Handover — Bau-Folge 53 (2026-09-16)

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

## TE-Gate-Arx — das konditionale FP/FN-Gate ist gebaut; seine CI-Messung steht aus

- Das konditionale FP/FN-Gate ist gebaut (Atom 53, `src/mathematikerin/te.rs`):
  `gate_conditional_arx_fpr_fn_n1000` (a × ρ(x,c) ∈ {0, 0,5, 0,9}², n=1000, Null
  Arx, FPR ≤ 8 %, Anstieg ≤ 2pp, FN-Arm ≥ 50 %), `gate_conditional_arx_2_fpr_n1000`
  (ρ=0,9, a-Achse + eigener FN-Arm) und `conditional_arx_fit_resolves_at_small_n`
  (n ∈ {64,128,256}, `max_lag = n/8`, beide Fit-Formen); in `te-gate.yml`
  dispatcht. Der `shuffle_series`-Fallback in
  `arx_restricted_surrogate_conditional`/`_2` ist eine Verweigerung geworden —
  die Signatur gibt `Option<Vec<f32>>`, `None` lässt den Surrogat aus (pending),
  kein Shuffle. (Schritt: den konditionalen Abschnitt des nächsten te-gate-Runs
  lesen — grün: Punkt zu; rot: der Assert-Text nennt die Zelle.)
- Eigenschaft (Council, 2026-09-16): die FPR-Auflösung bei 100 Trials ist grob —
  Clopper-Pearson-95 %-CI für 8 % ≈ [3,5, 15,2] %, der Anstieg ±3pp; der Null ist
  konservativ (Fit absorbiert c's Vergangenheit, der Schätzer konditioniert nur auf
  das gleichzeitige c[t]), das Gate misst diese Absorption, nicht eine enge
  Schwelle. Die Auflösung steht hier, nie implizit.
- Die zwei te-gate-Runs auf dem Arx-Switch laufen weiter: `35129638318` @
  `1337c6c1` (in_progress, u. a. `block_sweep_n1000`) und `35129679496` (pending).
  (Schritt: `gh run view 35129638318`.)
- Benchmark (gemessen): die `grind-max`-Delegation des konditionalen Gates kehrte
  leer zurück (kein edit, kein Ergebnis) — die Session baute das Gate selbst; die
  Delegation ist ein gemessener Fehlschlag, kein Beweis gegen max. (Schritt:
  `session_burn` für den Atom-Burn.)

## TeNull::Residual im konditionalen Dispatch — alter Defekt

- `conditional_te_surrogates_n` `TeNull::Residual` ruft weiter
  `residual_surrogate_conditional_lagged_n` (Plain-Shuffle, Re-Color mit der
  Original-Reihe — der 19,51-%-Defekt). Erreichbar über den pcmci-Benchmark-Arm.
  (Schritt: als benannte Kalibrier-Kontrolle mit dem n=1000-Rekord behalten oder
  streichen — nie still.)

## Phase-Null — Spektral-Placebo

- `TeNull::Phase` bleibt als Spektral-Placebo: Verbraucher sind nur
  `placebo_pair_eeg_probe`, `rest.rs` und der pcmci-Benchmark-Arm `--null 3`; kein
  Produktions-Signifikanz-Konsument. Das binned n=1000-Gate misst es. (Der
  Code-Kommentar am Null-Arm ist gate-verboten — die Eigenschaft steht hier.)
  (Schritt: `gate_fpr_autocorrelation_phase_null_binned_n_1000` im te-gate-Run
  lesen.)

## Block-Null — der Sweep entscheidet

- `multi_force_te_probe.rs:74` läuft auf `TeNull::Block`; Block ignoriert `conds`
  (alte Defektklasse in konditionalem Gewand). `block_sweep_n1000` misst, ob eine
  Blocklänge den FPR-Anstieg ≤ 2pp hält: hält eine → sie bleibt; hält keine →
  Probe auf Arx, Block ausmustern. (Schritt: den `block_sweep_n1000`-Abschnitt des
  te-gate-Runs lesen.)

## DRS-FITS — die Workflow-Granule ist Housekeeping-only

- Gemessen (2026-09-16): `.github/workflows/drs-fits-cdn.yml:14` zielt auf
  `drs_20160116_155329__20160123_160115.fits` (HTTP 200, 1 022 400 B) — diese
  trägt **kein** `SCI_SCIENCE_1Hz`, nur BINTABLE `HOUSEKEEPING` (116 Spalten);
  `ESA00001`/`DST11xxx` existieren nur in der PRIMARY-`HISTORY`-Prosa. Die
  Parser-Spalten (`drs_rows_from_table`, `src/archivar/fits.rs:1010`) sind die
  gemessenen echten Namen einer Voll-Wissenschafts-Granule — kein Remap. Die
  Diagnose (`drs_fits_compiler.rs:51`) nennt jetzt Tabelle + Spalten. (Schritt:
  `drs-fits-cdn.yml:14` auf eine Voll-Wissenschafts-Granule zeigen;
  Index-Zeit-Drift / Anker `at earth` bleibt.)

## Star-Katalog — gaia-cdn Re-Dispatch

- `.github/workflows/gaia-cdn.yml` trägt Checkout + Rust-Toolchain (committet in
  `1337c6c1`). Offen: Re-Dispatch + Verifikation des 44-B-Assets (Wert:
  `docs/zustand/external-state.md`). (Schritt: `gh workflow run gaia-cdn.yml`,
  dann `archive_search --sniff https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/dr3_stars.bin`.)

## Katalog-Konsumenten — Feld-Semantik offen

- Offen: VLASS (kein src-Modul, phi ohne Distanz/Parallaxe), `catalog_dcom5`
  (CometRec ohne GM), `des_y6` (nur `sigma_m`), `ossos` (Kepler ohne GM), `twomrs`
  (kein Distanzfeld), `las` (Frame/Anker fehlt), `nexrad_level2` (44-B-Bin ohne
  Standort). (Schritt: je Quelle Feld-Semantik messen oder `descoped`/`pending`.)

## ODR — Serie/Granulat und Semantik

- Offen: Granulat für die 484-Datei-Serie (13,61 GB > 2-GB-Limit); `sample_count`-
  Semantik; Galileo 12-bit kein Record im PPI-Archiv; `year_full` 00–89 `None`;
  Voyager Decimation>1 unverifiziert. (Schritt: Granulat wählen + Workflow erweitern.)

## smail_recv — leerer Body bei verschachteltem MIME

- Root cause (leerer Sotgiu-Reply) unbestätigt. (Schritt: bei erneutem leerem Body
  die Worker-`message.raw`-Quelle messen — `cloudflare/email_worker.js`.)

## HRV/Puls-Oszillator-Bindung

- Physischer ESP32-Träger (on hold) + End-zu-End-Test. (Schritt: `src/archivar/hrv.rs`.)

## CI-Format-Gate und external-state

- Bau-Datei `src/gate/commit_gate.rs:540` ist formatiert (Atom 53). Fremd bleiben
  `src/archivar/{dl3,fits,flac,ifms_agc,tests,units,vtscat}.rs`, `src/mathematikerin/s2.rs`,
  `tools/measure/src/bin/{aia_ladder_probe,trishuli_gauge_probe}.rs`,
  `tools/utils/src/bin/archive_search.rs` und die ernte-eigenen harvest-Compiler.
  (Schritt: rustfmt-Diff aus dem `ci-check`-format-Job anwenden — jede Linie ihre
  eigene Datei.)
- external-state `CI-Status` ist in diesem Atom fortgeschrieben (measured-at
  `e7795aea`); mit dem Atom-Push erneut fällig. (Schritt: Check-Runs des
  gepushten SHA messen; `external-state.md` fortschreiben.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
