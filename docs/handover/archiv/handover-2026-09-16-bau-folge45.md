<!--
  title: Handover — Bau-Folge 45 (Stand 2026-09-16)
  session: Bau-Folge 45
  class: handover
  date: 2026-09-16
  sha256: 946d34a2618643702e1e18a6390dcfd590a1b2acabfac5266ac9e36a49b22678
  status: live
-->
# Handover — Bau-Folge 45 (2026-09-16)

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

## TE-Gate — Verifikation läuft (härtester undatierter Punkt)

- Der CI-Blocker ist gefallen: Ernte-Commit `625452e5` fixte `src/archivar/eea.rs`
  (i128-Overflow) und `parquet.rs`; der gepushte `b109f105` war am Compile rot.
  Der Rangtest-Lauf ist dispatcht: `te-gate` run `35092997862`
  (workflow_dispatch, ~1,5–2 h Laufzeit). (Schritt: den Lauf lesen — bleibt
  residual/ksg > 8 %, den gemessenen FPR-Boden als Register-Zeile tragen, kein
  weiterer Null-Wechsel ohne Messung.)

## Serien-Leser für die registrierten Formate (aus Post „An bau")

- `src/archivar/extract.rs` (`series_parse_bin`/`series_component_name`) + die
  `main_flow.rs`-`matches!`-Liste haben keinen Arm für `juno_odf`, `magellan_odf`,
  `mgs_odf`, `mro_odf`, `odyssey_odf`, `messenger_odf`, `mars_express_odf`,
  `rosetta_odf`, `voyager_odr`, `cors_rinex`, `tnf` (auch die schon registrierten
  `vex_odf`/`galileo_odf`/`dawn_odf`/`galileo_odr` sind inert), plus die sechs
  Compiler-Formate VLST/VLCT, NCDO, GED1, AT31, SWS1. (Schritt: Leser je Format +
  Field-Key-Mapping nach `src/archivar`, Muster `geo.rs`/`voyager_saturn`.)

## DRS-FITS — Format-Arm braucht zwei Messungen

- Der `drs_fits_compiler` (DRSF-Bin, 24-B-Records `[gx,gy,gz]`) ist gebaut; der
  Archivar-Arm fehlt. Der Versuch, ihn ohne Epoche/Position zu bauen, fabrizierte
  `t/lat/lon = 0.0` (Golf von Guinea, Epoche 0) — verworfen, die vier
  `src/`-Dateien sind wieder HEAD-identisch. Zwei ungemessene Fakten blockieren
  den Arm: (a) trägt `SCI_SCIENCE_1Hz` eine `TIME`-Spalte und in welchem
  Zeitsystem; (b) trägt sie Raumfahrzeug-Orbitspalten (der Geo-Kontrakt ist
  Surface-only; `Position::Barycenter` ist kein LPF-Anker). Der CDN-Workflow
  `.github/workflows/drs-fits-cdn.yml` steht. (Schritt: Range-GET 206 auf den
  Header von `drs_20160806_235657__20160814_114429.fits`, TTYPE1..140 über die
  `fits.rs`-Kette lesen.)
- Benchmark (2026-09-16): `grind-pro` baute den Arm und fabrizierte
  `t/lat/lon = 0.0`; `grind-max` erkannte die Fabrikation und entfernte den Arm —
  der pro-Lauf ist der Verlierer, der max-Lauf die Korrektur; der gemessene
  Gewinner ist „kein Arm ohne die zwei Messungen".

## CI — Rest

- `clippy` grün; den `test`-Job des jüngsten `ci-check`-Laufs lesen.
- `archive_search`-Flags (`--count`/`--case`/`--path`): CI-Verifikation des
  Release-Binärs. (Schritt: CI-Lauf.)

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

## Mail-Fang, Membran, Papier (Operator)

- Mail-Fang: `wrangler kv namespace create MAIL_QUEUE` → Id eintragen → `wrangler deploy`.
- ESP32-Modul: on hold; BOM `docs/specs/mantis-shrimp-bom.md`.
- 20-s-Bande-Papier: per-Papier-Release-Tag und Welt-Fassung-Branch absent.
  (Schritt: `git tag` + `git branch` anlegen.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
