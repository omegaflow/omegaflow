<!--
  title: Handover — Ernte-Folge 25 (Stand 2026-09-15)
  session: Ernte-Folge 25
  class: handover
  date: 2026-09-15
  sha256: a24d23f9366b5c14f373b4e00d29e3aa6363cd7d5bb9824f11136c8dca14c0bf
  status: live
-->
# Handover — Ernte-Folge 25 (2026-09-15)

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
- LASzip-Decoder steht; Konsument fehlt (Rats-Verdikt: keine Nadel trägt eine
  Punktwolke) → pending.
- GK2A AMI: Compiler GKA1 gebaut, kein Skalar-Feld-Konsument (Tor 1) → pending;
  kein sources.φ-`format` registriert. GOES-16 ABI: Compiler gebaut, aber
  `noaa-goes16` ist EINGEFROREN (2025/097) — live sind `noaa-goes19`/`-goes18`;
  Himawari-8 ebenso (endet 2025/11) → `noaa-himawari9`. (Schritt: Ziel-Buckets
  in Compiler + Workflow.)
- US-CRN: Compiler + `format us_crn_hourly` + CDN-Line registriert; offen ist
  nur der CI-Dispatch (gated Push + Consent).
- Babamul: anonym weiter 401; Signup-Fluss aus dem SPA-JS gemessen
  (`/api/babamul/signup` → activation_code per Mail → `/activate` → Passwort →
  `/tokens` + `/kafka-credentials`); braucht Operator-Email, Marker
  `BABAMUL_API_TOKEN`/`BABAMUL_KAFKA_USERNAME`/`_PASSWORD`.
- WFAU VSA/WSA/OSA/SSA: Host `tap.roe.ac.uk` jetzt TCP-tot (2026-09-12 noch
  200); NOIRLab spiegelt `vhs_dr5`/`ukidss_dr11plus`; kein Konsument → pending.
- FITS: `P`-Format dekodiert; Rice-Dekompression fehlt (nur nötig, wenn ein
  Konsument Pixel braucht).
- NODD-NRS: Bucket von AWS S3 auf GCS gezogen (`registry.opendata.aws` 404,
  `storage.googleapis.com/noaa-passive-bioacoustic` 200) — AWS-Route +
  Harvester-`DEFAULT_BUCKET` gebrochen. (Schritt: GCS-Pfad registrieren +
  Harvester gegen GCS messen.)

## GES-DISC OAuth — der client_id-Fluss fehlt

- Die EDL-S3-Wiring ist gebaut: `fetch_s3_whole` + `sigv4_whole_headers` in
  `src/archivar/range.rs`, gebunden an das `s3://`-Scheme in
  `src/archivar/fetch.rs`. Bearer-DAACs (podaac/nsidc/lpdaac) signieren gegen
  us-west-2 — `EARTHDATA_EDL_TOKEN` öffnet alle drei `s3credentials`-Endpunkte
  (HTTP 200 + accessKeyId, gemessen 2026-09-15); die PODAAC-„client_id
  fehlt"-Notiz war stale. Offen: GES-DISC (`gesdisc`/`goldsmr*`) verlangt den
  OAuth-`client_id`-Fluss; kein client_id in `.secrets.local` →
  `edl_s3_credentials_for` gibt für OAuth ehrlich None. (Schritt: client_id
  beschaffen — urs.earthdata.nasa.gov OAuth-App — dann
  `S3CredentialRoute::OAuth` in `range.rs` verdrahten.)

## CDN-Manifestation (gated — nur der CI-Manifestator lädt hoch)

- Die neuen/geänderten Compiler warten auf den `--ci-mode`-Dispatch je Quelle
  (nach Push + Consent): himawari, goes, gk2a, uscrn, cosmic, maxi, isc,
  nexrad, noaa-ocs-hydrodata, gdp, superdarn, onc, wod, cors, vlass, euclid —
  plus die neuen cors/eri. (Schritt: Push, dann `gh workflow run
  <name>-cdn.yml`.)

## Netz-Census (Instrument steht, Ernte offen)

- `source_latency_census` + `source-census.yml` gebaut. Offen: erster
  CI-Dispatch (`gh workflow run source-census.yml`) — gated (Push + Consent).
- Alternativen-Recherche getragen (Befund
  `phi/pipeline/research/agent_output/source_census_tail_2026-09-14.φ` +
  `phi/reports/source_latency_census_tail_probe.φ`): der Tail ist
  Backend-/CGI-/Gateway-Latenz, nicht Geo. Einziger Gewinn: DONKI-WS-Sibling
  `kauai.ccmc.gsfc.nasa.gov/DONKI/WS/get/CME` (keyless, 1,31 s vs 1,81 s).
  (Schritt: sources.φ-URL auf den Sibling stellen.)
  Council-Restpunkt (späteres Atom): curl-`-sS`-stderr könnte die aufgelöste
  URL in den CI-Log schreiben.

## Bau-Gaps (Inventur)

- 19 Punkte ohne Suchbedarf (Route bekannt, Konsument/Parser/Ernte fehlt) —
  gelistet in `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md`
  §1. (Schritt: je Punkt der Register-Notiz in `phi/blocked_sources.φ` folgen.)
- Bestätigte Bau-Gaps aus dem Taucherlauf: 350 OCS-Surveys + `.tif`-LZW,
  COSMIC-2-Tages-Tarballs, GDP parquet-zstd, ERI JPEG-in-TIFF (5040×5040,
  Compression 7), ONC mat5-Dump (frequency 1921 Bins, nicht 512×250).

## Ernte-Nachlauf

- Argovis FWHM ungemessen (0.0 benannt). MPC-Shard: UnnObs-Dispatch + shard-url
  (Operator-Wort). Fink-Konus dead. Broker/GW-Positionen pending. Witness-CDN +
  Gaia-Dispatch gated (Push + Consent).
- ADS + Space-Track + Celestrak: Routen GEMESSEN (Befund
  `phi/pipeline/research/agent_output/ads_spacetrack_celestrak_2026-09-14.φ`).
  Dispositionen stehen aus (schreibbar):
  - ADS (`api.adsabs.harvard.edu`) lebt token-gated (`NASA_ADS_TOKEN`); Messung
    = Literatur-Katalog → `decline no-physical-force`; die zwei `dead
    404`-Einträge in dead_sources.φ sind Fehl-URLs, kein Tot.
  - Space-Track (`www.space-track.org`) lebt (`SPACETRACK_USER/PASS`); TLE =
    Orbit-Fit → `decline derived-orbit-fit`/`superseded-by-ephemeris`.
  - Celestrak `EOP-All.csv` lebt via Proton, direkt ip-blocked —
    Erdorientierung (gravity) = echter Feld-Kandidat. (Schritt: Feldblock +
    Compiler + CDN.)
- Gaia DR3 XP-Spektren (GAVO `dc.g-vo.org`, `gdr3spec.spectra`): Konto-Frage
  geklärt (Markus Demleitner 2026-09-08: keine Auth nötig; PENDING = fehlender
  PHASE=RUN-Post, in `tap_compiler` gebaut + verifiziert). Pilot (k=6144) +
  parallax>20-Teilmenge (34 947 Sterne, zwei Sync-Bänder) registriert. OFFEN:
  vollständige Survey (Millionen) — Async läuft NICHT (anonyme UWS-Jobs
  IP-gebunden, verwaissen mit dem CI-Runner, gemessen); Pfad = gebandetes Sync
  über `gaia_xp_compiler --source-range <lo> <hi>`. (Schritt: Band-Raster
  festlegen + Workflow über die Bänder fahren.)
- JVO skynode-TAP akari/irsf/nobeyama/saga: anonym 200 VOTable nur unter
  `/skynode/do/tap/<node>/sync` (Basis-URL 404) — pending (Tor 1, kein
  Konsument). GHRC-DAAC + ARPANSA-UV + NOAA CDO/GEDI/NSIDC/PODAAC-SWOT sind
  entblockt (Key/EDL-Token 200) → `ausstehend kandidat` in
  `phi/pipeline/ledger.φ`. Offen: Ernte/Compiler + Konsument je Quelle.
- VLASS: offene Tabelle ist `cirada.VLASS_Source` (votable+csv 200, anonym);
  `cirada.VCSS` = 403. CORS `.24S` = teqc-QC (nicht SBF). NOIRLab: Gaia DR4
  noch nicht erschienen (≥ Dez 2026), Wiedervorlage 2026-12-02 hält.
- Pro-Taucherlauf über die 10 descoped + AQS (key) + Babamul (account)
  (2026-09-15, drei grind-pro mit godmode): alle 10 Descopes HALTEN (kein
  Nadel-Konsument; ESO tap_cat ist ein Content-Descope, der Host lebt; WFAU
  4× TCP-tot, NOIRLab-Spiegel `vhs_dr5`/`ukidss_dr11plus` aktiv; JVO
  alma/hitomi nur unter `/sync`; DARTS-hitomi jetzt HTTP 0). **AQS: keylose
  Bulk-Route GEFUNDEN** (vorgenerierte AirData-CSVs,
  `aqs.epa.gov/aqsweb/airdata/download_files.html` → 200; sie tragen
  FRM/FEM/POC — die Granularität, die `openaq_pm25_ugm3` nicht trägt) →
  entblockt → ledger-Kandidat; die API bleibt key-gated. Babamul bleibt
  blocked account (Kafka/API credential-gated, Signup mail-gated).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
