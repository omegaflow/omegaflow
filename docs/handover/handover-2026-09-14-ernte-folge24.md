<!--
  title: Handover — Ernte-Folge 24 (Stand 2026-09-14)
  session: Ernte-Folge 24
  class: handover
  date: 2026-09-14
  sha256: 4990297fd72a163cfb454894b745e879aa9c08a936f13b5d0390b97d06e9c90b
  status: live
-->
# Handover — Ernte-Folge 24 (2026-09-14)

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

## Ernte-Linie — Compiler stehen, Konsumenten fehlen (Tor 1)

- ERI/CORS/VLASS-Compiler gebaut (`cors_compiler`, `eri_compiler`,
  `vlass_compiler` in `tools/harvest/src/bin/`); kein Skalar-Feld-Konsument →
  pending, kein `sources.φ`-Block (ERI-MAGIC `ERI1` noch nicht in
  `zeuge.rs::magic_identity`). (Schritt: `magic_identity` + Feld-Leser, wenn
  ein Konsument benannt ist.)
- AQS: OpenAQ-S3-Bulk keyless (HTTP 200) + Apify-Actor
  `nexgensignal/air-quality-monitor-records`; Compiler fehlt. (Schritt:
  Compiler in `tools/harvest/src/bin/` nach dem nexrad-Muster.)
- LASzip-Decoder steht; Konsument fehlt (Rats-Verdikt: keine Nadel trägt eine
  Punktwolke) → pending.
- GK2A AMI + GOES-16 ABI: Compiler gebaut, kein Skalar-Feld-Konsument (Tor 1)
  → pending.
- US-CRN: Compiler + `format us_crn_hourly` registriert; CDN-Manifestation
  offen.
- Babamul: anonym weiter 401; Token-gated (`BABAMUL_API_TOKEN`).
- WFAU VSA/WSA: NOIRLab spiegelt `vhs_dr5`/`ukidss_dr11plus`; kein Konsument →
  pending.
- FITS: `P`-Format dekodiert; Rice-Dekompression fehlt (nur nötig, wenn ein
  Konsument Pixel braucht).

## GES-DISC OAuth — der client_id-Fluss fehlt

- Die EDL-S3-Wiring ist gebaut: `fetch_s3_whole` + `sigv4_whole_headers` in
  `src/archivar/range.rs`, gebunden an das `s3://`-Scheme in
  `src/archivar/fetch.rs` (`fetch_raw`, `fetch_raw_bytes`,
  `fetch_raw_bytes_headers`). Bearer-DAACs (podaac/nsidc/lpdaac) signieren
  gegen us-west-2; anonyme NOAA-Buckets fahren virtual-host
  `{bucket}.s3.amazonaws.com` (curl `-L` folgt der Region). Offen: GES-DISC
  (`gesdisc`/`goldsmr*`) verlangt den OAuth-`client_id`-Fluss; kein client_id
  in `.secrets.local` → `edl_s3_credentials_for` gibt für OAuth ehrlich None
  (pending, kein fabrizierter Fluss). (Schritt: client_id beschaffen —
  urs.earthdata.nasa.gov OAuth-App — dann `S3CredentialRoute::OAuth` in
  `range.rs` verdrahten.)

## CDN-Manifestation (gated — nur der CI-Manifestator lädt hoch)

- Die neuen/geänderten Compiler warten auf den `--ci-mode`-Dispatch je Quelle
  (nach Push + Consent): himawari (Radianz), goes (GSICS), gk2a, uscrn, cosmic,
  maxi, isc, nexrad, noaa-ocs-hydrodata, gdp, superdarn, onc, wod, cors, vlass,
  euclid — plus die neuen cors/eri. (Schritt: Push, dann `gh workflow run
  <name>-cdn.yml`.)

## Netz-Census (Instrument steht, Ernte offen)

- `source_latency_census` + `source-census.yml` gebaut. Offen: erster
  CI-Dispatch (`gh workflow run source-census.yml`) + Alternativen-Recherche
  langsamer Tail (SOURCE_PORT §9: Sibling-Endpoints, Proton-Exit, Wayback).
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
