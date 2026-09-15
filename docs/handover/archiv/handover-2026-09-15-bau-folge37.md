<!--
  title: Handover — Bau-Folge 37 (Stand 2026-09-15)
  session: Bau-Folge 37
  class: handover
  date: 2026-09-15
  sha256: 252dc802ecbd2addcea1cc5e5b09102c72d8f3fb1f2cc81a8da80a43c843d6ac
  status: live
-->
# Handover — Bau-Folge 37 (2026-09-15)

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

## TE-Gate — n=1000-Lauf noch in Arbeit

- Lauf `35003040901` (dispatch 2026-09-15, HEAD `21c9b3d`) trägt die neuen
  n=1000-Gates (`gate_fpr_autocorrelation_restricted_null_binned_n_1000` und
  `…_xshift_null_binned_n_1000`); Status in_progress (20:04). Der Vorlauf
  `34896734026` (`97542a7`) maß nur die 7 alten und scheiterte (shift binned a=0,9
  FPR 9,05 %, residual ksg 13,33 %). (Schritt: `gh run view 35003040901 --log`,
  FPR-Tabelle lesen; `gh issue close 13` nur bei haltendem Gate.)
- n=150: RestrictedPermutation (FPR ≤ 5,50 %) und XShift (≤ 4,25 %, kein Anstieg)
  halten. (Schritt: `src/mathematikerin/te.rs`, `gate_fpr_autocorrelation_*`.)

## Die Compiler-Quellen — Manifestation und lis_otd offen

- GKA1/GAB1/GDPT + atdf (PASF) + himawari_hsd (AHI1) haben Leser-Arme in
  `extract.rs` (Serie, `parse_series`/`component_name`), verdrahtet in `main_flow.rs`;
  GDPT über den GEO-Pfad mit Position. Offen: `lis_otd` — kein Compiler/Component
  (Blitzenergie em J) gebaut. (Schritt: `lis_otd_compiler.rs` nach `iss_lis`-Vorbild,
  dann Component + geo/format-Arm.)
- Die 14 Blöcke sind nicht gemergt: die 8 TAP-Blöcke sind probe-pflichtig (Schemata/Aliase),
  die 6 Compiler-Blöcke brauchen erst die CDN-Manifestation (`--ci-mode`). (Schritt:
  Probe-Läufe, dann Merge; `phi/pipeline/research/agent_output/sources14_2026-09-15.φ`.)

## Parser-Gap A — DRS/TNF-Verifikation + Konsument offen

- DRS-FITS: `drs_fits_compiler.rs` gebaut; offen Verifikation am echten Granulat (`pending`) +
  `phi/sources.φ`-Registrierung. (Schritt: echter FITS-Lauf per `curl -r`, dann sources.φ-Block.)
- TNF: `tnf_compiler.rs` gebaut; offen echter Lauf + Registrierung + Membran-Konsument (Operator).
  (Schritt: Lauf gegen `nhpc_rex_*.tnf`.)
- ODF Juno/Magellan/MGS/MRO/Odyssey/MESSENGER/Mars Express/Rosetta + ODR Voyager: gebaut,
  Konsument offen (Operator).

## Register-Digest — Bau-Linie

- `--live`-Namensstimme (Operator: `--open` oder bleibt); der sichtbare Überseh-Korpus gehört
  je in seine Linie.
- Legacy-Repo-Run gemessen: `register_lookup --history --legacy <archive-root>/omegaflow-legacy`
  → 1159 Treffer.

## Code/Infra

- HRV/Puls-Oszillator-Bindung: physischer ESP32-Träger (on hold) + ein End-zu-End-Test.
- 40-byte bins recompilation; mirror-research ~2.300-Quellen-Migration; GLO-30 DEM;
  4d-membrane-Archäologie. (Schritt: `docs/concepts/archivar-mathematikerin.md` bzw. `mirror-research.md`.)

## RINEX/CORS, Binding, Mail-Fang, Membran (Operator)

- RINEX/CORS: Konsument benennen (Parser grün, `cors_compiler`/`cors-cdn.yml` stehen).
- Binding: Konsument-Benennung; kein sources.φ-Block ohne Membran-Konsument.
- Mail-Fang: `wrangler kv namespace create MAIL_QUEUE` → Id eintragen → `wrangler deploy`.
- ESP32-Modul: on hold; BOM `docs/specs/mantis-shrimp-bom.md`.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
