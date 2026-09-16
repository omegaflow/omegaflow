<!--
  title: Handover — Bau-Folge 52 (Stand 2026-09-16)
  session: Bau-Folge 52
  class: handover
  date: 2026-09-16
  sha256: 6e70a7e5e1b5867017c7a74c177914b490701012b9c2db06f3f81e575918c35a
  status: live
-->
# Handover — Bau-Folge 52 (2026-09-16)

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

## TE-konditional-Arx — geschlossen; die nächste Messung steht aus

- Der konditionale Arx-Null ist gebaut (`arx_restricted_surrogate_conditional`,
  `arx_restricted_surrogate_2` in `src/mathematikerin/te.rs`) und die zwei
  Corona-kde-Pfade (`conditional_te_stats_lagged`, `conditional_te_stats_lagged_2`)
  plus der `_n`-Dispatch (`TeNull::Arx`, `conditional_te_surrogates_n`) sind
  umgestellt. Bekannte Eigenschaft (der Code-Kommentar ist gate-verboten, sie steht
  hier): der Fit läuft auf der beobachteten Reihe und absorbiert c's Vergangenheit,
  während der Schätzer nur auf das gleichzeitige c[t] konditioniert — bei
  x–c-Kreuzkorrelation re-routet ein Teil von x's Projektion durch c's Vergangenheit
  in die Surrogate, der Null ist dort konservativ.
- **Nächste Messung (Rat):** ein konditionales FP/FN-Gate für die direkten Pfade —
  FPR-Arm über a ∈ {0, 0,5, 0,9} × ρ(x,c) ∈ {0, 0,5, 0,9}, n=1000, Null Arx,
  FPR ≤ 8 % und Anstieg ≤ 2pp; FN-Arm ≥ 50 %; Fallback-Arm: der
  `shuffle_series`-Fallback feuert bei n ∈ {64, 128, 256} null Mal, sonst leitet
  sich die Fit-Ordnung aus Live-Daten ab (`max_lag ≤ n/8`) oder der Fallback wird
  eine Verweigerung (`pending`, kein Shuffle). In `te.rs` `#[cfg(test)]`
  `#[ignore]`, Lauf in `te-gate.yml`. (Schritt: das Gate bauen + in `te-gate.yml`
  dispatchen.)

## Phase-Null — Spektral-Placebo

- `TeNull::Phase` bleibt als Spektral-Placebo: Verbraucher sind nur
  `placebo_pair_eeg_probe`, `rest.rs` und der pcmci-Benchmark-Arm `--null 3`; kein
  Produktions-Signifikanz-Konsument. Das binned n=1000-Gate misst es. (Der
  Code-Kommentar am Null-Arm ist gate-verboten — die Eigenschaft steht hier.)

## TeNull::Residual im konditionalen Dispatch — alter Defekt

- `conditional_te_surrogates_n` `TeNull::Residual` ruft weiter
  `residual_surrogate_conditional_lagged_n` (Plain-Shuffle, Re-Color mit der
  Original-Reihe — der 19,51-%-Defekt). Erreichbar über den pcmci-Benchmark-Arm.
  (Schritt: als benannte Kalibrier-Kontrolle mit dem n=1000-Rekord behalten oder
  streichen — nie still.)

## Block-Null — der Sweep entscheidet

- `multi_force_te_probe.rs:74` läuft auf `TeNull::Block`; Block ignoriert `conds`
  (alte Defektklasse in konditionalem Gewand). `block_sweep_n1000` misst, ob eine
  Blocklänge den FPR-Anstieg ≤ 2pp hält: hält eine → sie bleibt; hält keine →
  Probe auf Arx, Block ausmustern. (Schritt: den `block_sweep_n1000`-Abschnitt des
  te-gate-Runs lesen.)

## TE-Gate-Arx — die n=1000-Verifikation läuft

- Der Arx-Switch ist committet (`1337c6c1`); die Verifikation läuft als
  te-gate-Run `35129638318` @ `1337c6c1` (in_progress), `35129679496` (pending).
  (Schritt: `gh run view 35129638318` — grün: Punkt zu; rot: der Assert-Text nennt
  die Zelle.)

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

- CI-`format`-Status des gepushten SHA offen (`src/gate/commit_gate.rs:540`).
  external-state `CI-Status` ist fällig (HEAD-Wechsel). (Schritt: Check-Runs des
  gepushten SHA messen; `external-state.md` fortschreiben.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
