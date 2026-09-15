<!--
  title: Handover — Bau-Folge 39 (Stand 2026-09-15)
  session: Bau-Folge 39
  class: handover
  date: 2026-09-15
  sha256: c7c741e8f4d928da6d4bcb4e1b45d9d811a1190ab20edca14a6f1a4126e03f50
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

- DRS-FITS: Diagnose abgeschlossen (gemessen 2026-09-15). Spaltenzuordnung bestätigt:
  DST11077-79 = LTP1-Kraftvektor [N], DST11083-85 = LTP2-Kraftvektor [N] (FITS-Header
  TTYPE/TFORM/TUNIT, Spalten-Offset 176), Δg = (F2−F1)/1.928 kg korrekt (1.928 kg =
  LPF-Testmasse). Die Nullen im n=1-Granulat sind operativ, kein Parser-Gap: der DCS füllt
  die LTP-Kraftvektoren nur im Drag-free-Betrieb; Jan–Jul-2016-Granulate sind null-gefüllt.
  Erstes nicht-null Granulat `drs_20160806_235657__20160814_114429.fits` (Modus DFLLF);
  Compiler-Lauf darauf: 111811 Zeilen, |dg| bis 3.073e-9 m/s², roundtrip parst.
  (Schritt: Drag-free-Granulat als Quelle registrieren — `phi/blocked_sources.φ` `drs-fits`
  entblocken, `phi/sources.φ`-Block; `phi/sources.φ` trägt fremde uncommittete Arbeit →
  Registrierung wartet auf deren Session-Grenze.)
- TNF: `tnf_compiler.rs` gebaut; offen echter Lauf + Registrierung + Membran-Konsument
  (Operator). (Schritt: Lauf gegen `nhpc_rex_*.tnf`.)
- ODF Juno/Magellan/MGS/MRO/Odyssey/MESSENGER/Mars Express/Rosetta + ODR Voyager: gebaut,
  Konsument offen (Operator).

## Die 6 Compiler-Quellen — CDN-Manifestation offen

- 5 der 6 Compiler-Assets sind manifestiert (dispatch 2026-09-15, alle `completed/success`,
  Assets auf dem CDN geprüft): `gk2a_ami_rad.bin`, `goes_abi_rad.bin`,
  `himawari_ahi_counts.bin`, `gdp_drifter.bin`, `pioneer10_skyfreq.bin`.
- `lis-otd-cdn.yml` neu angelegt (Muster `iss-lis-cdn.yml`, NETLOC `ghrc.nasa.gov`,
  EDL-Token, naif-LSK, `--ci-mode`); Compiler gegen echtes OTD-Granulat verifiziert
  (`otdlip_1995.103_daily.tar` → 6608 flashes, 396488 B, roundtrip parst).
  (Schritt: Workflow pushen, dann `gh workflow run lis-otd-cdn`; `lis_otd.bin` fehlt noch
  auf dem CDN.)
- Nach der Manifestation: die 6 Blöcke aus
  `phi/pipeline/research/agent_output/sources14_2026-09-15.φ` in `phi/sources.φ` mergen —
  wartet auf `lis_otd.bin` und auf die Session-Grenze der fremden uncommitteten
  sources.φ-Änderung.

## TE-Gate — n=1000-Lauf noch in Arbeit

- Lauf `35003040901` (dispatch 2026-09-15, HEAD `21c9b3d`) trägt die n=1000-Gates
  (`gate_fpr_autocorrelation_restricted_null_binned_n_1000` und `…_xshift_null_binned_n_1000`);
  Status in_progress (gemessen 2026-09-15). (Schritt: `gh run view 35003040901 --log`,
  FPR-Tabelle lesen; `gh issue close 13` nur bei haltendem Gate.)
- n=150: RestrictedPermutation (FPR ≤ 5,50 %) und XShift (≤ 4,25 %, kein Anstieg) halten.
  (Schritt: `src/mathematikerin/te.rs`, `gate_fpr_autocorrelation_*`.)

## Benchmark — offene Bau-Aufgabe flash vs. pro

- DRS-FITS-Diagnose (identischer Wortlaut, read-only): flash (grind-flash, deepseek-v4-flash)
  $0.0369 (in 42133, out 9982, reasoning 27014, cache_read 2810240) vs. pro (grind-pro,
  deepseek-v4-pro) $0.0856 (in 68524, out 9625, reasoning 41691, cache_read 3080448) —
  pro 2,3× teurer. Beide gleichwertig (Mapping bestätigt, Nullen operativ, Drag-free-Granulat
  benannt); flash lief den Compiler zusätzlich end-to-end (111811 Zeilen, |dg| 3.073e-9 m/s²).
  Sieger: flash → die Aufgabe bleibt bei flash.

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
