<!--
  title: Handover — Mountain-Folge 298 (2026-10-10)
  session: Mountain-Linie in einem Pass abarbeiten
  class: handover
  date: 2026-10-10
  sha256: c90211cc4be2b0e5099920015bbd3cf15030a195570172fb4e136e1244598094
  status: live
-->
# Handover — Mountain-Folge 298 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-10-mountain-folge297.md` (→ `archiv/`) und
faltete den `## An mountain`-Block aus `handover-2026-10-10-mycelium-folge293.md`
(iEEG-Riss beigelegt: das registrierte Wort 2026-10-06 bleibt maßgeblich).
flash only, kein pro/max.

## Burn: open 0.0000 · close 0.0585 — gemessen via session_burn (Linien-Session $0.0585, deepseek-flash, kein pro/max)

## Offen (aufgeschlüsselt)

### GIC-Estimator — Ground-Truth auf embedded Production-Arm umgestellt
- **Status:** eigen (Paper/Mathematikerin) | **Bindung:** mycelium (CI-Lauf)
- **Trigger:** CI-Lauf des Ground-Truth-Bins (kein Workflow führt ihn)
- **Lage:** (gemessen 2026-10-10) `te-bias-n` `38032560457` ausgewertet:
  der embedded/KSG-Arm (Takens dim 3, der Production-Flux `omega.rs:489`) nullt den
  Rückkanal — TE(Y→X) = −1.4271e-2 bei n = 10000 (5/5 Replikate), Forward
  TE(X→Y) = 2.5064e-1; der Skalar-KDE-Arm trägt den finite-sample-Rückkanal
  (TE(Y→X) = 3.4374e-2) und die n-abhängige Bias-Kurve. `te_ground_truth.rs` auf den
  embedded-Arm umgestellt (10 Phasen-Surrogate analog zum Skalar-Arm; der Skalar-Arm
  bleibt als benannter Riss mitgeführt); `cargo build -p omegaflow-measure --bin
  te_ground_truth` grün. Kein 99-Surrogat-Lauf lokal (schwer, CI).
- **Blockade:** kein Workflow führt `te_ground_truth` aus.
- **Braucht:** Mycelium — `te_ground_truth` in einen Workflow aufnehmen + Lauf;
  danach Paper §3.5/Abstract/§7 auf den Production-Arm nachziehen (Mountain).

### blocked_sources.φ — particle-cern ROOT-TTree: Riss behoben, Object-Streamer offen
- **Status:** eigen (Disposition) | **Bindung:** eigen
- **Trigger:** ROOT-TTree/Object-Streamer-Decode
- **Lage:** (gemessen 2026-10-10) Der Riss ist behoben: die Entry mischte die ATLAS-URL
  (`opendata.atlas.cern`) mit dem ALICE-Sample; `url` → `https://opendata.cern.ch/record/1120`
  (HTTP 200; `AliVSD_Masterclass_6.root`, CC0-1.0). Der Gap bleibt der ROOT-**Object-Streamer**
  (`TStreamerInfo`/`TBuffer` + `TBranch`/`TLeaf`/`TBasket`; `root.rs:222` benannte
  Verweigerung), **nicht** die Kompression: zlib steht (`archivar::inflate`, `hdf4.rs:887`).
  Header/TKey/`--probe` (`cern_root_compiler`) steht. 0 Consumer.
- **Blockade:** TTree/Branch-Decode (Object-Streamer fehlt global); nur flash verfügbar.
- **Braucht:** Object-Streamer bauen (`TBuffer` + version-pinned 53006 `TBranch`/`TLeaf`/
  `TBasket`) + `extract.rs`-Arm — hartes Atom; oder Council-Verdikt, ob der `parser-def`-Zustand bleibt.

### Flyby-Kette — Residual in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-09, unverändert) 157 ODF-Referenzen; `doppler.rs` absent;
  Wahrheit `state/zustand/wartend.φ:34`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### iEEG — Riss beigelegt: registriertes Wort 2026-10-06 maßgeblich
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort, das den Riss über 2026-10-06 hebt
- **Lage:** (gemessen 2026-10-10) mycelium-293 (adressiert) bestätigt den Riss; das
  registrierte Operator-Wort 2026-10-06 (`state/zustand/wartend.φ:40`: iEEG = privates
  Experiment, kein CDN) bleibt maßgeblich, der mountain-296-Auftrag ist gestrichen. Die
  Mountain-Seite (`eeglab::eeg_from_bin` akzeptiert `Samples::Double`) ist gebaut.
- **Blockade:** keine.
- **Braucht:** kein Schritt — nur ein neues Operator-Wort öffnet es.

### CMB/SPT — Transfer-Bound gebaut; Mycelium hebt Job-Timeout + Re-Lauf
- **Status:** eigen (Compiler) | **Bindung:** mycelium (Job-Timeout)
- **Trigger:** Mycelium hebt `cmb-cdn` `timeout-minutes` und startet den Re-Lauf
- **Lage:** (gemessen 2026-10-10) `cmb_planck_compiler` lädt
  `full_maps_d1.tar.bz2` jetzt über `fetch_raw_bytes_with(..., SPT_TRANSFER_BOUND_S = 6 h)`.
  `--verdict`: HTTP 200, `Content-Length 7 873 515 864`; kein direkter FITS/Mirror.
  Der `cmb-cdn` Job-Timeout 240 min genügt bei ~0.47 MB/s (~4.65 h) nicht.
- **Blockade:** Mycelium-Job-Timeout.
- **Braucht:** Mycelium: `cmb-cdn.yml` `timeout-minutes` ≥ 360 + Re-Lauf.

### IRIS — Rohpixel, kein Feld; BUNIT-Riss benannt
- **Status:** eigen (Register) | **Bindung:** mycelium (Block)
- **Trigger:** Rat-Verdikt zum `count`-Token / sources.φ-Block
- **Lage:** (gemessen 2026-10-10) `iris_compiler.rs` liest kein BUNIT;
  `src/archivar/iris.rs:6` `UNIT="count"` ist eine Baum-Konstante, nicht die Quelle
  (BUNIT `Corrected DN`). Rohe DN ist kein physikalisches Feld → keine `field`/`quantity`-Zeile.
- **Blockade:** kein Descriptor für rohe DN.
- **Braucht:** Mycelium: IRIS sources.φ-Zeile + Workflow; die `UNIT`-Konstante klären.

### GIRO DIDBase — Compiler gebaut; Manifestation = Mycelium
- **Status:** eigen | **Bindung:** mycelium (Block)
- **Trigger:** Mycelium-Workflow-Lauf `giro-fastchar-cdn`
- **Lage:** (gemessen 2026-10-10) Compiler gebaut und grün: `src/archivar/giro_fastchar.rs`
  (MAGIC `GIFC`) + `tools/harvest/src/bin/giro_fastchar_compiler.rs`; Endpoint
  `https://lgdc.uml.edu/fastchar/getbest` (HTTP 200, `text/plain`, kein Key); realer Lauf
  `--station JR055 --start 2012-07-02T21:00:00Z --stop 2012-07-03T03:00:00Z` → 24 Records,
  Roundtrip 608 B; Lizenz CC BY-NC-SA 4.0.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Mycelium: `giro-fastchar-cdn` Re-Lauf.

### PDS-PPI — `quantity`-Projektion gebaut; Block = Mycelium
- **Status:** eigen | **Bindung:** mycelium (Block)
- **Trigger:** Mycelium schreibt den sources.φ-Block
- **Lage:** (gemessen 2026-10-10) `units.rs` + `pds_ppi_compiler.rs` emittieren je Spalte
  eine `quantity`-Zeile aus dem gemessenen PDS4-`<unit>`; Spalten ohne Label-Einheit
  bleiben `pending`.
- **Blockade:** keine.
- **Braucht:** Mycelium: PDS-PPI-Block (manifest + `quantity`-Zeilen).

### Keogramm ABK — Compiler rückwärts-bounded gebaut; Re-Lauf = Mycelium
- **Status:** eigen (Compiler) | **Bindung:** mycelium (Re-Lauf)
- **Trigger:** Mycelium `keogram-cdn` Re-Lauf
- **Lage:** (gemessen 2026-10-10) `keogram_compiler` sucht bei leerem Fenster die jüngste
  verfügbare Nacht (`--lookback-days`, Default 400); FMI endet `ABK.2604`; mycelium-293
  meldet `keogram-cdn 38032914527` success.
- **Blockade:** keine.
- **Braucht:** Mycelium: Re-Lauf bestätigt (kein Mountain-Akt).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern; Lauf
  lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut,
  `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer
  Kovariat-Varianz.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„warum schreibst du hier erst in die blocked sources anstatt direkt an den finalen ort — das ist einfach nur faules compliance theater" | 2026-10-10 | Operator (Session, Mountain 295)
„Offen bleibt in P10: der optionale `<regime>`-Token (wird derzeit benannt verweigert — keine Descriptor-Achse dafür), die Regime-Achse selbst, und die Zeilen-Migration in phi/sources.φ (Mountain)." | 2026-10-10 | Operator (Session, Mountain 295)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater … kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–290)
„mach das ab jetzt automatisch — committe und pushe selbst" | 2026-10-07 | Operator (Session, Mountain 264)

## An mycelium

Origin: mountain-298 (2026-10-10).

- **GIC-Ground-Truth.** `tools/measure/src/bin/te_ground_truth.rs` ist auf den
  embedded KSG-Arm (Production-Flux) umgestellt; der Skalar-KDE-Arm bleibt als
  benannter Riss mitgeführt. Kein Workflow führt diesen Bin aus — nehmt ihn in einen
  CI-Lauf auf (z. B. neben `te_bias_n_probe`), damit der Ground-Truth-Verdict
  reproduzierbar ist.
- **CMB/SPT · Keogramm · iEEG.** Unverändert aus folge297: `cmb-cdn` `timeout-minutes`
  ≥ 360 + Re-Lauf; Keogramm-Re-Lauf bestätigt (`38032914527` success); iEEG = kein
  Mountain-Akt (registriertes Wort 2026-10-06).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („committe und pushe selbst",
2026-10-07) trägt Commit und Push. Eigene Pfade dieses Atoms:
`tools/measure/src/bin/te_ground_truth.rs` · `phi/blocked_sources.φ` ·
`docs/handover/handover-2026-10-10-mountain-folge298.md` ·
`docs/handover/archiv/handover-2026-10-10-mountain-folge297.md` (Move).
