<!--
  title: Handover — Ernte-Folge 21 (Stand 2026-09-13)
  session: Ernte-Folge 21
  class: handover
  date: 2026-09-13
  sha256: 314d2362cd2c555caefe554375eddc5e3f86dbc18e97ff839efd85153d5f80ea
  status: live
-->
# Handover — Ernte-Folge 21 (2026-09-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## CDN-Manifestation — offene Dispatches

Alle Entblockt-Compiler + ihre CDN-YMLs stehen (gebaut, an echten Quellen gemessen,
cargo check 0 Warnungen). OFFEN: der `--ci-mode`-Dispatch je Quelle (nach Push +
Consent): maxi, cors, vlass, euclid (sources.φ-Block, kein Compiler), isc, goes,
himawari, nexrad, onc, wod, cosmic, noaa-ocs-hydrodata, gdp, superdarn.

## Per-Compiler offene Fäden (gemessen benannt)

- MAXI: Rows-Zeitachse (MJD→TDB) pending; Position via `--ra/--dec`; `.max1` nicht
  ans Feld verdrahtet.
- ISC: ISCB-magic absent in `zeuge.rs` `magic_identity` (Zeugen-Gate pending);
  sources.φ-Block pending.
- Euclid: redshift/photometrie-Join (object_id → phz_physical_parameters) pending.
- COSMIC: TDB-Bindung (`unix_to_tdb`) pending; kein sources.φ-Feldblock.
- CORS: CRX1-Membran-Verbrauch (extract/series) nicht verdrahtet; Hatanaka `.d` /
  SBF `.S`-Reader.
- GOES: GSICS-Kalibrierung pending (L1b-Radiance, calib benannt, nie fabriziert).
- Himawari: Kalibrier-Block 5 undekodiert (counts, nie radiance); Georeferenzierung
  (Blöcke 3/4) nicht gespiegelt.
- NEXRAD: Kern-Feldsystem-Reader für `format nexrad_level2`; Site-Anker (site-id →
  lat/lon ICRS).
- ONC: Position aus locations-API (im YML); CDN-Dispatch.
- WOD: Voll-Harvest-Schleife über Jahr×Instrument; oxygen-Naming unverifiziert.
- OCS: LZW-TIFF-Lücke (tiff.rs decodiert Compression 5 nicht); ERI = Imagery,
  nicht elevation.
- GDP: Voll-Datensatz-Manifestation; parquet ZSTD-Decode (core std-only, zstd-Crate
  nicht in den Kern-Reader verlinkt).
- SuperDARN: Chisham-virtuelle-Höhe (aktuell Kugelmodell, benannt); rollendes Fenster.
- VLASS: sources.φ/witnesses.φ-Registrierung; Komponenten-Kataloge + QL-epoch FITS.

## Ernte-Nachlauf

- Argovis `/bgcargoplus` — gebaut: sources.φ-Block (5 radiometrische Felder,
  `data=`-Uppercase `_ADJUSTED_RO` gemessen 200), zwei `convert_to_si`-Arme
  (`w/m^2/nm`→×1e9, `micromolequanta/m^2/sec`→×1e-6 Präfix), em-Force-Registry,
  `FieldConfig.freq/bin_width` (down_irradiance trägt c/λ; downwelling_par broadband
  freq 0 ehrlich). OFFEN: Radiometer-FWHM (bin_width) ungemessen — bleibt 0.0 benannt,
  nicht fabriziert.
- Gaia-Alerts — gebaut: SKD1 in `magic_identity` + `--gaia-alerts`-Merge-Arm +
  skydirection-cdn-Wiring. OFFEN: der `gaia-alerts-cdn`- und `skydirection-cdn`-Dispatch.
- MPC-Shard: UnnObs-Job dispatcht + misst (a) unnobs.bin-Größe (b) inflate-Durchsatz
  (c) Upload-Zeit; erst damit Shard-url-Einträge (Operator-Wort) + NumObs-Job.
- Witness-CDN: hawc/lhaaso/gaia-alerts + skydirection (Merge) dispatch.
- HAWC-TLS: gemessen geschlossen (Bundle = System-Store + Let's-Encrypt YR1 + ISRG
  Root YR, `curl --cacert` → 200); `hawc-cdn.yml` trägt noch `curl -k` — der
  `--cacert`/`OMEGAFLOW_CA_BUNDLE`-Umbau ist benannt.
- Fink-Konus: re-gemessen 2026-09-13 dead (api.lsst 000, api.ztf 500) — bleibt
  pending (witnesses.φ notiert).
- Broker/GW-Positionen: pending (gemessene Absenz).

## Externe Blocker

- AQS EPA: Key-Mail fehlt; `smail_recv` läuft (Webhook 127.0.0.1:1619) — erneut
  auslösen.
- Babamul: Credentials fehlen.

## Wand-Party — offene Wege (Session 2026-09-14)

- FOUND, ohne Ernte/Konsument: Scopus (free non-commercial API-Key,
  `dev.elsevier.com`), Dimensions (freie Metrics-API, `metricssignup`), NOAA
  MarineCadastre AIS (CC0, historisch), NOAA GLM (Blitz, `noaa-goes16` S3),
  ThingSpeak (öffentliche Channels anonym), GNIP (Mendeley `75tsccprd2` +
  `waterisotopesDB.org`), Semantic Scholar (keyless + `influentialCitationCount`/
  `tldr`). Schritt: je Quelle die Speisekammer-Tore + ein benannter Konsument, dann
  `phi/sources.φ`.

## Parser-Lücken — gemessene Crates

- LASzip (`laz 0.13.0` pure Rust + `laszip-sys`), JPEG-in-TIFF (`oxiarc-tiff`/
  `oxideav-tiff`), RINEX-2.11/Hatanaka (`rinex 0.22.0`, `crx2rnx`), GSICS
  (netCDF-GPRC + JMA-CSV), GLO-30 (geotiff-reader/cloudtiff; AWS
  `copernicus-dem-30m`). Schritt: je Gap den Reader gegen den gemessenen Endpoint
  bauen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene Abschluss-Check.
