<!--
  title: Handover — Bau-Folge 35 (2026-09-15)
  session: Bau-Folge 35
  class: handover
  date: 2026-09-15
  sha256: 869f511313915d4a04096e4b055768db4213b84d08ab2fc76e81062b81893a20
  status: live
-->
# Handover — Bau-Folge 35 (2026-09-15)

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

## TE-Gate — zwei Nullen halten n=150, n=1000 in CI offen

- `TeNull::RestrictedPermutation` (feinere Bins, `RESTRICTED_PERMUTATION_BINS = 32`,
  `src/mathematikerin/te.rs`) hält das n=150-Gate: FPR ≤ 5,50 % je Zelle, Anstieg 1,75 pp (D_Z=0) /
  1,27 pp (D_Z=4); der frühere Rollback (FPR 13,78 %) ist überholt, `pcmci_class_benchmark.rs` kompiliert.
- `TeNull::XShift` (X-Seiten-Null, aus der Entscheid-Diagnose: der KSG-Finite-Sample-Bias hängt an der
  Y-Vergangenheit, die jede Y-Randomisierung mitverändert; hier wird nur X zirkulär verschoben, Y/Z bleiben)
  hält n=150 schärfer: FPR ≤ 4,25 % je Zelle, Anstieg −0,75 pp (kein Anstieg).
- Beide n=1000-`#[ignore]`-Tests (`…_restricted_null_binned_n_1000`, `…_xshift_null_binned_n_1000`) stehen;
  der CI-Step `gate_fpr_autocorrelation` (`.github/workflows/te-gate.yml:27`) matcht sie per Substring —
  kein Workflow-Eingriff. Offen: `gh workflow run te-gate.yml`, FPR-Tabelle aus dem Lauf lesen, dann
  `gh issue close 13` bei einem haltenden n=1000-Gate. (Schritt: Dispatch; Lauf-Ausgabe.)

## Parser-Gap A — DRS/TNF-Compiler gebaut, Verifikation + Konsument offen

- DRS-FITS: `tools/harvest/src/bin/drs_fits_compiler.rs` gebaut (ruft `fits::drs_differential_acceleration`,
  `--input`/`--url` → `--out`, Magic `DRSF`, `--ci-mode`). Offen: Verifikation am echten Granulat
  (`pending`) + `phi/sources.φ`-Registrierung. (Schritt: echter FITS-Lauf per `curl -r`, dann sources.φ-Block.)
- TNF: `tools/harvest/src/bin/tnf_compiler.rs` gebaut (scannt SFDUs, `tnf_dt0`; Route gemessen:
  `pds-smallbodies.astro.umd.edu/holdings/pds4-nh_rex:pluto_tnf-v1.0/tnf/`). Offen: echter Lauf +
  Registrierung + Membran-Konsument (Operator). (Schritt: Lauf gegen `nhpc_rex_*.tnf`.)
- ODF Juno/Magellan/MGS/MRO/Odyssey/MESSENGER/Mars Express/Rosetta + ODR Voyager: gebaut, Konsument offen
  (Operator).

## Die 14 akzeptierten Quellen — Leser fehlen (mod.rs fremd)

- `GKA1`/`GAB1`/`GDPT` existieren nur im Compiler, kein Leser in `src/archivar/`. (Schritt: Leser je Magic —
  blockiert, solange `src/archivar/mod.rs` fremd-modifiziert ist; nur eigene Hunks.)
- `iss_lis`: gemessen (2026-09-15) — `NETLOC` ist ein CDN-Release-Tag, keine Route; die Granulat-Route
  (`--granule`, Workflow) läuft bereits über `data.ghrc.earthdata.nasa.gov`. Der Eintrag „tote Route
  ghrc.nasa.gov" ist überholt; ein Tag-Rename bräuchte `phi/sources.φ` (fremd). (Schritt: Tag-Rename
  zusammen mit der sources.φ-Zeile.)
- `gdp_drifter_compiler` läuft bereits über `noaa-oar-hourly-gdp-pds.s3.amazonaws.com` (S3-Route lebt).

## Register-Digest — Bau-Linie

- `repo_surveillance.rs::git_verdict()` trägt jetzt den Branch-Token (≠ `main` → `branch`).
- `register_lookup --history --term <term>` fährt `git log -S` über die Docs-Pfade (Blindfleck benannt).
- Offen: `--live`-Namensstimme (Operator: `--open` oder bleibt); Legacy-Repo-Run
  (`register_lookup --history --legacy <archive-root>/omegaflow-legacy`); der sichtbare Überseh-Korpus
  gehört je in seine Linie.

## Code/Infra

- 5 Tool-Renames: nur `mseed_measure`-Usage trug noch `mseed_messen` — korrigiert; die anderen vier
  existieren nicht mehr.
- Crate-`cargo check` (harvest/register/service/science/gate/utils/core): 0 Fehler, 0 Warnungen.
- HRV/Puls-Oszillator-Bindung: gemessen VORHANDEN in HEAD (`8c57757`: `ingress.rs` → `main_flow.rs` →
  `omega.rs` → `actuators.rs`); offen bleibt der physische ESP32-Träger (on hold) + ein End-zu-End-Test.
- 40-byte bins recompilation; mirror-research ~2.300-Quellen-Migration; GLO-30 DEM; 4d-membrane-Archäologie:
  unverändert offen. (Schritt: `docs/concepts/archivar-mathematikerin.md` bzw. `mirror-research.md`.)

## RINEX/CORS — Konsument offen

- Parser (`parse_rinex_obs`/`is_hatanaka`/`crx2rnx`) grün, `cors_compiler`/`cors-cdn.yml` stehen; der Rat
  hält den sources.φ-Block bis zur Konsument-Benennung. (Schritt: Konsument benennen — Operator.)

## Binding — Konsument-Benennung (Operator)

- Kandidaten bauen, Bindung bleibt benannter offener Punkt; kein sources.φ-Block ohne Membran-Konsument.

## Mail-Fang — Deployment offen (Operator-Wort)

- `cloudflare/email_worker.js` + `wrangler.toml` + `smail_recv.rs` gebaut, nicht deployed. (Schritt:
  `wrangler kv namespace create MAIL_QUEUE` → Id eintragen → `wrangler deploy`.)

## Membran

- ESP32-Modul — on hold (Operator-Wort). BOM: `docs/specs/mantis-shrimp-bom.md`.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene Abschluss-Check mit
Commit und Push (`/commit`).
