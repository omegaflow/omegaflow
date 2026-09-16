<!--
  title: Handover — Bau-Folge 40 (Stand 2026-09-16)
  session: Bau-Folge 40
  class: handover
  date: 2026-09-16
  sha256: a221ba411180c5bb618d92d5a14bb9969fcb89da2f8bc1a4559ce28c6860bd1a
  status: live
-->
# Handover — Bau-Folge 40 (2026-09-16)

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

## TE-Gate — Wurzel gemessen, Rise gefixt; residual/ksg-Tail offen

- Wurzel (gemessen 2026-09-16): nicht der Surrogat-Null leckt — das Negativ-Set ist
  kontaminiert. `gate_common_driver` baut `y_t = a·y_{t-1} + 0.5·Σ z_{d,t-1}`; durch
  die z-AR(a)-Erinnerung ist `z_{d,t-lag} → … → y_t` ein ECHTER indirekter Fluss bei
  jedem lag (gemessen n=50000 binned: TE(z→y,lag2) = 0.0132 bei a=0.9, 3× die direkte
  lag-1-Kante 0.0044; FP-Zerlegung n=1000 a=0.9 shift/binned: 18 von 30 FPs sind
  z→y lag2). Der Null meldet für diese Paare korrekt ~0; sie wurden als FP gezählt.
- Fix (`src/mathematikerin/te.rs` `gate_fpr_cells_from`): die ganze z→y-Familie
  (alle lags) ist causal candidate, nur die conditionally independent pairs sind
  negative. Lokal verifiziert: n=150 binned (shift/block/xshift) grün; n=400
  shift/binned Rise a=0→a=0.9 = −0,55pp (vorher +2,80pp), block/binned +0,82pp,
  alle FPR < 8 %.
- Offen: residual/ksg-Tail. Nach dem Fix bleibt residual marginal (binned a=0.9
  ~7,3–8,2 %, Grenze 8 %); KSG (CI 13,52 % vor Fix) ist lokal nicht messbar — KSG
  n=400 lief > 20 min, abgebrochen. (Schritt: n=1000-Gates in `te-gate.yml` laufen
  lassen; bleibt residual/ksg > 8 %, den residual-Null-Tail als eigenes Atom bauen —
  NICHT den 8-%-Schnitt senken, `mean+2sd` ist bereits das 97,7-Perzentil.) CI-Issue
  13 offen lassen.
- Benchmark flash vs. max auf diesem Atom: flash lieferte die Wurzel-Diagnose
  (gemessen, $ —); max wurde mid-Atom abgebrochen (nur `diag_*`-Scaffolding, kein
  Fix) → nicht gewertet. (Schritt: max-Lauf auf dasselbe Atom neu ansetzen.)

## Parser-Gap A — DRS-Registrierung entblockt; TNF/ODF-Konsument offen

- DRS-FITS: die fremde Session-Grenze auf `phi/sources.φ` ist überschritten
  (committet) — das Drag-free-Granulat ist jetzt registrierbar. Der Compiler steht
  (`tools/harvest/src/bin/drs_fits_compiler.rs`, `CDN_TAG heasarc.gsfc.nasa.gov`,
  DRSF-Asset 24-B-Records = gx/gy/gz [m/s²]). Offen ist die sources.φ-Feldabbildung
  für das 3-Vektor-Δg. (Schritt: `phi/blocked_sources.φ` `drs-fits` entblocken →
  `phi/sources.φ`-Block mit dem Vektor-Feld-Schema → CI-CDN.)
- TNF: `tnf_compiler.rs` gebaut; offen echter Lauf + Registrierung +
  Membran-Konsument (Operator).
- ODF Juno/Magellan/MGS/MRO/Odyssey/MESSENGER/Mars Express/Rosetta + ODR Voyager:
  gebaut, Konsument offen (Operator).

## Register-Digest — Bau-Linie

- `--live`-Namensstimme (Operator: `--open` oder bleibt); der sichtbare
  Überseh-Korpus gehört je in seine Linie.
- Legacy-Repo-Run gemessen: `register_lookup --history --legacy
  <archive-root>/omegaflow-legacy` → 1159 Treffer.

## Code/Infra

- HRV/Puls-Oszillator-Bindung: physischer ESP32-Träger (on hold) + ein
  End-zu-End-Test.
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
