<!--
  title: Handover — Bau-Folge 39 (Stand 2026-09-15)
  session: Bau-Folge 39
  class: handover
  date: 2026-09-15
  sha256: 7151c999d067338c17feb7e6fc3c717202f6d8304e7130d7df73c1be2f230dde
  status: live
-->
# Handover — Bau-Folge 39 (2026-09-15)

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

## Parser-Gap A — DRS-Verifikation + TNF/ODF-Konsument offen

- DRS-FITS: `drs_fits_compiler` läuft am echten Granulat (drs_20160124_000015__20160130_235906.fits,
  4640 Zeilen, roundtrip parst), aber die LTP-Kraftspalten DST11077-11085 lesen alle 0,0
  → |dg| = 0 (gemessen: Roh-Bytes am berechneten Spalten-Offset 176 sind null; die
  zweite Datei drs_20160213…fits trägt gar keine `SCI_SCIENCE_1Hz`-HDU). (Schritt: eine
  SCI-Datei mit nicht-null LTP-Kräften messen — die 9-Tage-Datei
  `drs_20160102_093513__20160111_070510.fits` (302 MB) per Download/`curl -r`, oder die
  Spaltenzuordnung gegen das LPF-DRS-Handbuch halten; `src/archivar/fits.rs`.)
- TNF: `tnf_compiler.rs` gebaut; offen echter Lauf + Registrierung + Membran-Konsument
  (Operator). (Schritt: Lauf gegen `nhpc_rex_*.tnf`.)
- ODF Juno/Magellan/MGS/MRO/Odyssey/MESSENGER/Mars Express/Rosetta + ODR Voyager: gebaut,
  Konsument offen (Operator).

## Die 6 Compiler-Quellen — CDN-Manifestation offen

- Die 6 Compiler-Blöcke (atdf/gk2a_ami/goes_abi/himawari_hsd/gdp_drifter/lis_otd) sind
  gebaut und im Staging-Block korrigiert (lis_otd = `field rad lis_otd_flash_radiance_uj_sr
  … uJ/sr`; himawari-URL = `himawari_ahi_counts.bin`), aber nicht im CDN manifestiert.
  (Schritt: die `*-cdn.yml`-Workflows dispatchen — gk2a/goes/himawari/gdp/pioneer-atdf
  stehen; für lis_otd fehlt ein Workflow → anlegen nach `gdp-cdn.yml`-Muster,
  `ghrc.nasa.gov`, `--ci-mode`.)
- Nach der Manifestation: die 6 Blöcke aus
  `phi/pipeline/research/agent_output/sources14_2026-09-15.φ` in `phi/sources.φ` mergen.

## TE-Gate — n=1000-Lauf noch in Arbeit

- Lauf `35003040901` (dispatch 2026-09-15, HEAD `21c9b3d`) trägt die n=1000-Gates
  (`gate_fpr_autocorrelation_restricted_null_binned_n_1000` und `…_xshift_null_binned_n_1000`);
  Status in_progress (gemessen 2026-09-15). (Schritt: `gh run view 35003040901 --log`,
  FPR-Tabelle lesen; `gh issue close 13` nur bei haltendem Gate.)
- n=150: RestrictedPermutation (FPR ≤ 5,50 %) und XShift (≤ 4,25 %, kein Anstieg) halten.
  (Schritt: `src/mathematikerin/te.rs`, `gate_fpr_autocorrelation_*`.)

## Register-Digest — Bau-Linie

- `--live`-Namensstimme (Operator: `--open` oder bleibt); der sichtbare Überseh-Korpus
  gehört je in seine Linie.
- Legacy-Repo-Run gemessen: `register_lookup --history --legacy <archive-root>/omegaflow-legacy`
  → 1159 Treffer.

## Code/Infra

- HRV/Puls-Oszillator-Bindung: physischer ESP32-Träger (on hold) + ein End-zu-End-Test.
- 40-byte bins recompilation; mirror-research ~2.300-Quellen-Migration; GLO-30 DEM;
  4d-membrane-Archäologie. (Schritt: `docs/concepts/archivar-mathematikerin.md` bzw.
  `mirror-research.md`.)

## RINEX/CORS, Binding, Mail-Fang, Membran (Operator)

- RINEX/CORS: Konsument benennen (Parser grün, `cors_compiler`/`cors-cdn.yml` stehen).
- Binding: Konsument-Benennung; kein sources.φ-Block ohne Membran-Konsument.
- Mail-Fang: `wrangler kv namespace create MAIL_QUEUE` → Id eintragen → `wrangler deploy`.
- ESP32-Modul: on hold; BOM `docs/specs/mantis-shrimp-bom.md`.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
