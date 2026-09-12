<!--
  title: Handover — Ernte-Folge VI (Stand 2026-09-12)
  session: Ernte-Folge VI
  class: handover
  date: 2026-09-12
  sha256: cf3de39aa1251d1ad347291bd4e0280ac46d3b4eebb84b415a2c4717735ce839
  status: live
-->
# Handover — Ernte-Folge VI (2026-09-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## CDN-Manifestation (Duty)

- igets.bin — Run 34634693904 gemessen: conclusion cancelled, Asset 404 → redispatcht
  (Run 34684040604, igets-cdn.yml); Wächter: igets.bin auf igetsftp.gfz.de (HTTP 200).
- eso-harps — `--csv` ergänzt (CSV-only-Host) + dispatcht (Run 34684315717); Wächter:
  harps_rvcat.json auf ssd.jpl.nasa.gov (HTTP 200). Feldblock steht (sources.φ Z.6052).

## Ernte

- Hi-net — `HINET_PASS` weiter absent (.secrets.local gemessen) — Operator.
- noaa-jpss — EULA-Operator-Akt weiter offen (EDL-Token gültig bis 2026-09-30, Download
  403 EULA-Acceptance); ein Granule-URL-Probe misst den wahren Stand.
- pioneer10_skyfreq.bin — Asset liegt (HTTP 206), keine sources.φ-Zeile; Konsument/Format
  (atdf) ungeklärt → Force-Gate offen.
- ESO tap_obs — Feldblöcke offen (nur tap_cat/HARPS bebaut).

## TAP-Klassifikation

- 7 ausstehend — sync-GET gemessen (2026-09-12): alle sieben Basis-URL 200, aber
  tap_schema-sync 500 (tap.roe.ac.uk wsa/vsa/osa/ssa, dachs.fai.kz) bzw. kein TAP-JSON
  (vo.lmd.jussieu.fr, pithia.cbk.waw.pl) → Funktion NICHT bestätigt; bleiben absent.
  Der Re-Probe-Takt trägt sie (kein offener Bau-Punkt).

## Register

- eve-cdn.yml — trägt noch lasp.colorado.edu (Beschreibung Z.26, idempotence-Zeile Z.44)
  + Asset-Default eve2011_lines.bin (≠ registriertes eve_lines_2011.bin); eve_compiler
  CDN_TAG ist jetzt ssd, der Workflow hinkt nach (gemessen, unberührt).
- data/-Bug-Familie — pioneer_doppler_compiler.rs:126, galileo_atdf_compiler.rs:94,
  pioneer11_odf_compiler.rs:163 schreiben data/spdf…, legen aber nur data/ an (dieselbe
  ENOENT-Klasse wie der pioneer-atdf-Fix c08d15b). Fremde Linien, benannt.
- electric Re-Kuratierung (aus Bau überführt) — 8 Feldzeilen in `phi/sources.φ`
  deklarieren noch `gaussian-inverse-square electric` (kernel 1); der Code trägt
  kernel 0 (inverse-square). Re-Kuratierung der 8 Zeilen (Source-Pfad
  `docs/SOURCE_PORT.md`).

## Votable-TAP (aus Entscheid-Folge III überführt)

- Compiler-Tranche für die 8 überlebenden Votable-Kataloge — je Katalog Feldblock
  (TAP_SCHEMA-Spalten) + `<name>-cdn.yml` + CDN-Manifestation + sources.φ-Eintrag
  erst mit gemessenem Feldblock: ALMA EU, SkyMapper, CASDA, MACHO, MUSE-Wide,
  WiggleZ, CADC youcat, LAMOST DR11. Die Disposition und die 18 Inventare
  (`tap_index_<label>.φ`) stehen in `phi/blocked_sources.φ` / `phi/pipeline/catalog/`.
- Zwei Fehlschläge nachmessen: WGE-SDSS (`ia2-tap.oats.inaf.it:8080/wgetap`,
  QUERY_STATUS=ERROR IllegalArgument) und LIneA (`userquery.linea.org.br`,
  Schema-Name ≠ tap_schema).
- Die 54 älteren `tap_index_*.φ` (Pass 2026-09-10) sind untracked, aber in
  `MANIFEST.φ` `visible` — mitcommitten oder die MANIFEST-Zeilen revidieren.

## Abschluss

Nur eigene Dateien committet (b0d5d63): eve_compiler.rs, regtap_census.rs,
eso-harps-rvcat-cdn.yml. Fremde uncommittete Arbeit im Baum (gll-ck-cdn.yml,
tap_compiler.rs, sources.φ, src/*, measure/*, ernte-folge4-Umzug) bleibt unberührt.
Dispatch getragen: Run 34684040604 (igets), Run 34684315717 (eso-harps); die Wächter
oben misst die nächste Session.
