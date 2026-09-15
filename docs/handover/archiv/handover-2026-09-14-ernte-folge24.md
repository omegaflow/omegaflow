<!--
  title: Handover — Ernte-Folge 24 (Stand 2026-09-14)
  session: Ernte-Folge 24
  class: handover
  date: 2026-09-14
  sha256: f69037c54ce26b99ebdd20639402ea97e1f0192683adfba289b207332c10f6bb
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
  CI-Dispatch (`gh workflow run source-census.yml`) — gated (Push + Consent).
- Alternativen-Recherche getragen (Befund
  `phi/pipeline/research/agent_output/source_census_tail_2026-09-14.φ` +
  `phi/reports/source_latency_census_tail_probe.φ`): der Tail ist
  Backend-/CGI-/Gateway-Latenz, nicht Geo — der Proton-Exit ist bei allen 8
  gemessenen Hosts langsamer als die direkte Route. Einziger Gewinn:
  DONKI-WS-Sibling `kauai.ccmc.gsfc.nasa.gov/DONKI/WS/get/CME` (keyless,
  1,31 s vs 1,81 s Gateway). (Schritt: sources.φ-URL auf den Sibling stellen.)
  Council-Restpunkt (späteres Atom): curl-`-sS`-stderr könnte die aufgelöste
  URL in den CI-Log schreiben.

## Bau-Gaps (Inventur)

- 19 Punkte ohne Suchbedarf (Route bekannt, Konsument/Parser/Ernte fehlt) —
  gelistet in `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md`
  §1. (Schritt: je Punkt der Register-Notiz in `phi/blocked_sources.φ` folgen.)

## Ernte-Nachlauf

- Argovis FWHM ungemessen (0.0 benannt). MPC-Shard: UnnObs-Dispatch + shard-url
  (Operator-Wort). Fink-Konus dead. Broker/GW-Positionen pending. Witness-CDN +
  Gaia-Dispatch gated (Push + Consent).
- ADS + Space-Track + Celestrak: Routen GEMESSEN (Befund
  `phi/pipeline/research/agent_output/ads_spacetrack_celestrak_2026-09-14.φ`).
  Dispositionen stehen aus (schreibbar, kein Block mehr):
  - ADS (`api.adsabs.harvard.edu`) lebt token-gated (`NASA_ADS_TOKEN` in
    `.secrets.local`); Messung = Literatur-Katalog → `decline
    no-physical-force`; die zwei `dead 404`-Einträge in dead_sources.φ sind
    Fehl-URLs (GET auf Root / POST-only-Endpoint), kein Tot.
  - Space-Track (`www.space-track.org`) lebt (`SPACETRACK_USER/PASS`
    funktionieren); TLE = Orbit-Fit → `decline derived-orbit-fit`/
    `superseded-by-ephemeris` (konsistent mit celestrak-TLE).
  - Celestrak `EOP-All.csv` (`archeology_gaps_index.φ` Z.93) lebt via Proton,
    direkt ip-blocked — Erdorientierung (Polbewegung/UT1-UTC/LOD/Nutation,
    gravity) = echter Feld-Kandidat, nicht TLE-Disposition. (Schritt:
    Feldblock + Compiler + CDN; 6-stellige-NORAD-Lücke aufgelöst — 549
    Objekte ≥ 100000.)
- AQS: litmus REDUNDANT (OpenAQ-S3-Bulk = OpenAQs eigenes csv.gz-Archiv, kein
  AQS-Rohdatenbestand; Skalar pm25 µg/m³ diffusion lebt via
  `openaq_pm25_ugm3`) → `decline superseded-by-openaq` in declined_sources.φ
  geschrieben; AQS-Note in blocked_sources.φ korrigiert (keine keylose
  AQS-Rohroute, bleibt key-gated).
- blocked key entblockt (2026-09-15): NOAA CDO + GEDI + NSIDC-ATL03 +
  PODAAC-SWOT (2×) verlassen blocked_sources.φ (Key/EDL-Token öffnet die
  token-/s3credentials-Route, HTTP 200) → `ausstehend kandidat` in
  phi/pipeline/ledger.φ. Offen: Ernte/Compiler + Konsument je Quelle.
- blocked account + ip-blocked abgearbeitet (2026-09-15, zwei Taucher):
  - ENTBLOKT → `pending`/ledger: GHRC-DAAC (EDL-Token öffnet ghrcw-protected,
    OTD-Tar + LIS-netCDF 200), JVO skynode-TAP akari/irsf/nobeyama/saga
    (Pfad-Fehlmessung: der echte Endpunkt ist `/skynode/do/tap/<node>/sync`
    → anonym 200 VOTable), ARPANSA-UV (`uvdata.arpansa.gov.au/xml/uvvalues.xml`
    200, 17 Stationen). Offen: Ernte/Compiler + Konsument.
  - DESCOPED: JVO alma (Spiegel von ALMA EU), JVO hitomi (nur ivoa.obscore).
  - DECLINED: AMS-Meteors (`superseded-by-integrated`, JPL-Fireball + GLM
    tragen die Bolide-Energie), Sentinel Hub (`redistribution`, Planet Labs
    kommerziell), worldtimeapi (`reference`), GDELT (`no-physical-force`),
    IODA (2×) + SEDAC GRAND_Dams (`infrastructure`).
  - BLEIBT: Babamul (Operator-Email nötig; Signup-Fluss aus dem SPA-JS
    gemessen, Marker BABAMUL_API_TOKEN/KAFKA_USER/PASS).
- Gaia DR3 XP-Spektren (GAVO `dc.g-vo.org`, `gdr3spec.spectra`): Konto-Frage
  geklärt (Markus Demleitner 2026-09-08: keine Auth nötig; der PENDING-Stand
  war der fehlende PHASE=RUN-Post — in `tap_compiler` gebaut + end-to-end
  verifiziert; Auftrag archiviert: `docs/auftrag/archiv/gavo-dc-account-anfrage.md`).
  Stand: Pilot (pixel k=6144 → `xp_pilot_p6144.bin`) + parallax>20-Teilmenge
  (34 947 Sterne, zwei gebandete Sync-Requests, `gaia-xp-cdn.yml`) registriert.
  OFFEN: die vollständige Survey (Millionen Quellen) — ein Async-Harvest läuft
  NICHT: anonyme UWS-Jobs sind IP-gebunden und verwaissen mit dem CI-Runner
  (gemessen, im Workflow dokumentiert); der Pfad ist gebandetes Sync über
  `gaia_xp_compiler --source-range <lo> <hi>`. (Schritt: Band-Raster festlegen
  + Workflow über die Bänder fahren.)
- Godmode-Taucherlauf über pending (25) + descoped (10) (2026-09-15, fünf
  grind-flash-Taucher mit VPN/secrets/godmode; keine Seite verlangte
  Registrierung). Kernbefunde + nächste Schritte:
  - GOES-16 (`noaa-goes16`) + Himawari-8 (`noaa-himawari8`) sind EINGEFROREN
    (2025/097 bzw. 2025/11) → Compiler-Ziel auf `noaa-goes19`/`-goes18` bzw.
    `noaa-himawari9`. (Schritt: Ziel-Buckets in Compiler + Workflow.)
  - NODD-NRS-Bucket von AWS S3 auf GCS gezogen (AWS 404, GCS 200) → AWS-Route
    + Harvester-`DEFAULT_BUCKET` gebrochen. (Schritt: GCS-Pfad registrieren +
    Harvester gegen GCS messen.)
  - CORS `.24S` = teqc-QC, nicht Septentrio SBF (korrigiert); VLASS
    `cirada.VCSS` = 403, offen bleibt `cirada.VLASS_Source` (votable+csv 200);
    ONC 1921 Frequenz-Bins statt 512×250; US-CRN-CDN-Line registriert.
  - WFAU OSA/SSA/VSA/WSA: Host jetzt TCP-tot (2026-09-12 noch 200); JVO
    akari/irsf/nobeyama/saga nur unter `/sync` (Basis 404); NOIRLab Gaia DR4
    noch nicht erschienen (≥ Dez 2026, Wiedervorlage hält).
  - Bau-Gaps bestätigt: 350 OCS-Surveys, COSMIC-2-Tages-Tarballs, GDP
    parquet-zstd, ERI JPEG-in-TIFF (5040×5040, Compression 7), ONC mat5-Dump.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
