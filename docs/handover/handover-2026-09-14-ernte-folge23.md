<!--
  title: Handover — Ernte-Folge 23 (Stand 2026-09-14)
  session: Ernte-Folge 23
  class: handover
  date: 2026-09-14
  sha256: f6e2beeef3870840306394aaf486401c8b4030b89e03140d919f7edb590a2292
  status: live
-->
# Handover — Ernte-Folge 23 (2026-09-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt.

## EDL — die Sperre ist halb gelöst (S3-Wiring fehlt)

- Der EDL-Token (`omegaflow.space`, gültig bis 30.09.2026) ist gültig; der
  generische `urs.…/api/users/s3credentials`-Endpoint verlangt jetzt `client_id`,
  die DAAC-spezifischen Endpoints (PODAAC/NSIDC/LPDAAC/ASF/ORNL/ASDC/LAADS,
  HTTP 200 gemessen) akzeptieren den Bearer-Token direkt. `S3CredentialRoute` +
  `edl_s3_credentials_for(bucket, token)` in `src/archivar/range.rs` gebaut.
  **Offen:** `fetch_s3_range` ist nirgends verdrahtet (kein Aufrufer) — die fünf
  EDL-S3-Routen (GRACE-FO/SWOT, SMAP, CDDIS IONEX, GES-DISC, AppEEARS) sind noch
  nicht ziehbar. (Schritt: `s3://`-Scheme im Fetch-Pfad — `src/archivar/fetch.rs`
  / `main_flow.rs` — an `fetch_s3_range` + `edl_s3_credentials_for(bucket,
  {EARTHDATA_EDL_TOKEN})` binden; GES-DISC braucht den OAuth-client_id-Fluss.)

## CDN-Manifestation (gated — nur der CI-Manifestator lädt hoch)

- Die neuen/geänderten Compiler warten auf den `--ci-mode`-Dispatch je Quelle
  (nach Push + Consent): himawari (Radianz), goes (GSICS), gk2a, uscrn, cosmic,
  maxi, isc, nexrad, noaa-ocs-hydrodata, gdp, superdarn, onc, wod, cors, vlass,
  euclid. (Schritt: Push, dann `gh workflow run <name>-cdn.yml`.)

## Ernte-Linie — Reader stehen, Compiler fehlen

- ERI (JPEG-in-TIFF Compression 7), CORS (RINEX-2/Hatanaka/SBF), VLASS
  (FITS-Struktur): die Reader stehen (Bau-Folge 28, committet); die Compiler
  fehlen. (Schritt: je Compiler in `tools/harvest/src/bin/`; ERI braucht zuerst
  den std-only JPEG-in-TIFF-Decoder.)
- LASzip-Decoder steht (Bau-Folge 28); der Konsument fehlt (Rats-Verdikt: keine
  Nadel trägt eine Punktwolke) → pending.
- GK2A AMI + GOES-16 ABI: Compiler gebaut, aber kein Skalar-Feld-Konsument
  (Tor 1) → pending, kein `sources.φ`-Block.
- US-CRN: Compiler + `format us_crn_hourly` registriert; CDN-Manifestation offen.
- AQS: OpenAQ-S3-Bulk keylos (HTTP 200) + Apify-Actor
  `nexgensignal/air-quality-monitor-records`; Compiler pending.
- Babamul: anonym weiter 401; Token-gated (`BABAMUL_API_TOKEN`).
- WFAU VSA/WSA: NOIRLab spiegelt `vhs_dr5`/`ukidss_dr11plus`; kein Konsument →
  pending.
- FITS: `P`-Format (variable-length) dekodiert jetzt (`fits.rs`); die
  Rice-Dekompression der komprimierten Bild-Nutzlast fehlt (nur nötig, wenn ein
  Konsument Pixel braucht).

## Netz-Census (Instrument steht, Ernte offen)

- `source_latency_census` (tools/measure) + `source-census.yml` gebaut: ein
  resolved-URL Range-GET je Quelle, dns/connect/tls/ttfb als Option (absent
  bleibt absent), Report `phi/reports/source_latency_census.φ`, langsamer Tail
  als eigener p90 je Host-Familie. Offen: erster CI-Dispatch
  (`gh workflow run source-census.yml`) + die Alternativen-Recherche für den
  langsamen Tail (SOURCE_PORT §9: Sibling-Endpoints, Proton-Exit, Wayback).
  Council-Restpunkt (späteres Atom): curl-`-sS`-stderr könnte die aufgelöste
  URL in den CI-Log schreiben.

## Bau-Gaps (Inventur)

- 19 Punkte ohne Suchbedarf (Route bekannt, Konsument/Parser/Ernte fehlt) —
  gelistet in `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md`
  §1. (Schritt: je Punkt der Register-Notiz in `phi/blocked_sources.φ` folgen.)

## Ernte-Nachlauf (unverändert offen)

- Argovis FWHM ungemessen (0.0 benannt). MPC-Shard: UnnObs-Dispatch + shard-url
  (Operator-Wort). Fink-Konus dead. Broker/GW-Positionen pending. ADS +
  Space-Track: Route offen, Compiler fehlt. Witness-CDN + Gaia-Dispatch gated
  (Push + Consent).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
