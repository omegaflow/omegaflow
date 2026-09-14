<!--
  title: Handover — Ernte-Folge 22 (Stand 2026-09-14)
  session: Ernte-Folge 22
  class: handover
  date: 2026-09-14
  sha256: b64e32524744bbe072a9ab10754f05ab8c918a2ec99dfaccc2233765c414fa35
  status: live
-->
# Handover — Ernte-Folge 22 (2026-09-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Eine Session arbeitet so viele Punkte
ab wie möglich. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks;
gepusht wird erst, wenn der Baum ruhig ist und das Wort kommt.

## EarthData — die eine Sperre (EDL client_id)

- `s3credentials` = 401 „required client id missing": der EDL-Token ist gültig
  (exp 01.10.2026), aber ein legacy User-Token ohne `client_id`-Bindung. Diese
  eine Sperre blockiert fünf S3-Routen — GRACE-FO/SWOT (PODAAC), SMAP (NSIDC),
  CDDIS IONEX, GES-DISC, AppEEARS. Schritt: EDL-App registrieren → `client_id`
  → frischer Token; der SigV4-Reader (`fetch_s3_range`, Commit `546d39e`) steht.

## Per-Compiler Restfäden (gemessen benannt)

- NEXRAD: `nexrad_site`/`nexrad_gate_position` gebaut (src/archivar/nexrad.rs);
  die Feld-System-Verdrahtung fehlt. Schritt: `geo_series`-Arm für `format
  nexrad_level2` in extract.rs bauen.
- MAXI: `.max1` nicht ans Feld verdrahtet. Schritt: sources.φ `format maxi` +
  `series_parse_bin("maxi")`-Arm + zeuge-Magic `MAX1`.
- COSMIC: sources.φ-Feldblock + `geo_series_component_name("cosmic_ro", …)`-Arm.
  Schritt: registrieren + verdrahten (TDB-Bindung ist gebaut).
- CORS: CRX1-Membran-Verbrauch (`series_parse_bin("cors")`) fehlt; vollständiger
  Hatanaka-Decoder + SBF-Payload-Dekoder (PVTGeodetic/PVTCartesian) ohne echte
  `.S`-Referenz. Schritt: erst eine echte Septentrio-`.S` messen, dann Dekoder.
- GOES: GSICS angewandter Pfad an keinem echten Granule durchlaufen
  (`a_h_NRTH`/`b_h_NRTH` waren Fill −999). Schritt: ein Granule mit finiten
  Koeffizienten finden und messen.
- Himawari: HSD-Block-Offsets gegen satpy `ahi_hsd.py` verifiziert, nicht gegen
  ein Live-Granule. Schritt: ein echtes HSD-Granule lesen.
- WOD: SOHM-Heap-Shared-Messages (im WOD-Satz unbenutzt). Schritt: benennen, ob
  ein WOD-Granule sie nutzt.
- OCS: JPEG-in-TIFF (Compression 7) — Pixel bleiben leer, Georeferenzierung
  steht. Schritt: JPEG-Dekoder in pure std (kein Crate-loser Einbau bekannt).
- GDP: parquet-ZSTD-Decode (zstd-Crate nicht im Kern verlinkt); der Zarr-Pfad
  trägt den Voll-Datensatz. Schritt: ZSTD-Decode oder ehrlich benannt lassen.
- SuperDARN: rollendes Fenster offen (Chisham-Höhe ist gebaut). Schritt: bauen.
- VLASS: `--kind component` gebaut; FITS-Pfade (components_se.fits etc.)
  unverifiziert (404); `format vlass`-Parser im Archivar pending. Schritt:
  FITS-Pfade messen, dann Parser.
- Euclid: Redshift-Spaltenname unverifiziert (`photometric_redshift` ist
  Platzhalter; gemessen `q1.phz_catalogue`). Schritt: Spaltenname messen.
- ISC: ISCB-Magic ist gebaut (zeuge.rs); sources.φ-Block pending. Schritt:
  `format iscb` registrieren.

## Ernte-Nachlauf

- Argovis FWHM (bin_width) ungemessen — bleibt 0.0 benannt. Schritt: Radiometer
  messen oder 0.0 ehrlich tragen.
- MPC-Shard: UnnObs-Job dispatcht + misst, erst dann shard-url (Operator-Wort).
- HAWC-TLS: `hawc-cdn.yml` trägt `curl -k` — `--cacert`/`OMEGAFLOW_CA_BUNDLE`-Umbau.
- Fink-Konus: dead (api.lsst 000, api.ztf 500) — pending.
- Broker/GW-Positionen: pending (gemessene Absenz).
- ADS + Space-Track: Route offen (gemessen 2026-09-14, ref-auth-apis.md:133-134)
  — Compiler fehlt. Schritt: je einen Compiler bauen.
- Witness-CDN + Gaia-Alerts/skydirection Dispatch: gated (Push + Consent).

## Wand-Party — disponiert

- 4 decline (Scopus, Dimensions, ThingSpeak, Semantic Scholar) → dead_sources.φ.
- 3 pending ohne Konsument (MarineCadastre-AIS, GLM-Sibling, GNIP/Mendeley) →
  blocked_sources.φ. Schritt: ein Konsument benennen (keine Nadel frißt die
  Daten heute) oder der Pending bleibt die Disposition.

## CDN-Manifestation (gated)

- Die entblockten Compiler warten auf den `--ci-mode`-Dispatch je Quelle (nach
  Push + Consent): maxi, cors, vlass, euclid, isc, goes, himawari, nexrad, onc,
  wod, cosmic, noaa-ocs-hydrodata, gdp, superdarn.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene Abschluss-Check.
