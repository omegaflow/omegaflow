<!--
  title: Handover — Ernte-Folge 26 (Stand 2026-09-15)
  session: Ernte-Folge 26
  class: handover
  date: 2026-09-15
  sha256: 28209f7cdc09eb352386cdb5ca14356803554d2c195387e5ed4f143ff914ca04
  status: live
-->
# Handover — Ernte-Folge 26 (2026-09-15)

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
  `vlass_compiler`); kein Skalar-Feld-Konsument → pending (ERI-MAGIC `ERI1`
  noch nicht in `zeuge.rs::magic_identity`). (Schritt: `magic_identity` +
  Feld-Leser, wenn ein Konsument benannt ist.)
- LASzip-Decoder steht; Konsument fehlt → pending.
- GK2A AMI: Compiler GKA1 gebaut, kein Konsument → pending. GOES-19 ABI +
  Himawari-9 AHI: Ziel-Buckets auf die lebenden gestellt
  (`noaa-goes19`/`noaa-himawari9`, gemessen 200; Granule-Key `..._G19_...`
  bzw. `HS_H09_...` gemessen) — kein Konsument → pending; CDN-Dispatch gated.
- Babamul: api_token gilt am `/api/babamul`-Pfad (200); `/api/alerts` +
  `/api/objects` bleiben 401 (Auth-Mechanismus) → pending (Tor 1/Ernte).
- WFAU VSA/WSA/OSA/SSA: Host `tap.roe.ac.uk` TCP-tot (curl 000); OSA/SSA als
  `dead unreachable` registriert, VSA/WSA `superseded-by-integrated`
  (NOIRLab-Spiegel) mit Konsument-Kandidat (SuperCOSMOS-pm als zweite
  unabhängige pm-Linie, Wiedervorlage 2026-12-02).
- FITS: `P`-Format dekodiert; Rice-Dekompression fehlt (nur nötig, wenn ein
  Konsument Pixel braucht).
- NODD-NRS: `decline spectral-series` (1195-Bin-Serie schließt das Gate,
  c3f33f6); Route auf GCS korrigiert (`storage.googleapis.com/noaa-passive-bioacoustic`),
  Harvester GCS-nativ (`NETLOC=storage.googleapis.com`). Sitz:
  `phi/declined_sources.φ`.

## Celestrak-EOP — Compiler gebaut, CDN-Dispatch offen

- `celestrak_eop_compiler` + `src/archivar/celestrak_eop.rs` (Magic `EOP1`,
  `ut1_utc`/`pmx`/`pmy`, gravity, `at earth`) + `sources.φ`-Block +
  `celestrak-eop-cdn.yml` gebaut; `cargo check` 0 Fehler/0 Warnungen, 7 Tests
  grün. CSV nur via Proton erreichbar (direkt ip-blocked): 23 633 `O`-Zeilen,
  `P`-Zeilen (Forecast) + pre-1972 (Leap-Table void) verworfen, kein 0.0-Pad.
  OFFEN: erster `--ci-mode`-Dispatch (`gh workflow run celestrak-eop-cdn.yml`)
  — gated (Push + Consent); das CDN-Asset `celestrak_eop.bin` fehlt noch.

## GES-DISC OAuth — der client_id-Fluss fehlt

- Die EDL-S3-Wiring ist gebaut (`fetch_s3_whole` + `sigv4_whole_headers` in
  `src/archivar/range.rs`, `s3://`-Scheme in `src/archivar/fetch.rs`).
  Bearer-DAACs (podaac/nsidc/lpdaac) signieren gegen us-west-2 —
  `EARTHDATA_EDL_TOKEN` öffnet alle drei `s3credentials`-Endpunkte (HTTP 200,
  gemessen 2026-09-15). Offen: GES-DISC (`gesdisc`/`goldsmr*`) verlangt den
  OAuth-`client_id`-Fluss; kein client_id in `.secrets.local`. (Schritt:
  client_id beschaffen — urs.earthdata.nasa.gov OAuth-App — dann
  `S3CredentialRoute::OAuth` in `range.rs` verdrahten.)

## CDN-Manifestation (gated — nur der CI-Manifestator lädt hoch)

- Die neuen/geänderten Compiler warten auf den `--ci-mode`-Dispatch je Quelle
  (nach Push + Consent): celestrak-eop, goes (goes19), himawari (himawari9),
  gk2a, uscrn, cosmic, maxi, isc, nexrad, noaa-ocs-hydrodata, gdp, superdarn,
  onc, wod, cors, vlass, euclid, eri. (Schritt: Push, dann
  `gh workflow run <name>-cdn.yml`.)

## Netz-Census (Instrument steht, Ernte offen)

- DONKI-URL auf den keyless Sibling gestellt
  (`kauai.ccmc.gsfc.nasa.gov/DONKI/WS/get/{CME,FLR}`; gemessen 200, Shape deckt
  `cmeAnalyses.0.{speed,latitude,longitude}` + FLR `classType`). Offen: erster
  `source-census.yml`-Dispatch (`gh workflow run source-census.yml`) — gated
  (Push + Consent). Council-Restpunkt (späteres Atom): curl-`-sS`-stderr könnte
  die aufgelöste URL in den CI-Log schreiben.

## Bau-Gaps (Inventur)

- 19 Punkte ohne Suchbedarf (Route bekannt, Konsument/Parser/Ernte fehlt) —
  gelistet in `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md`
  §1. (Schritt: je Punkt der Register-Notiz in `phi/blocked_sources.φ` folgen.)
- Bestätigte Bau-Gaps: 350 OCS-Surveys + `.tif`-LZW, COSMIC-2-Tages-Tarballs,
  GDP parquet-zstd, ERI JPEG-in-TIFF (5040×5040, Compression 7), ONC mat5-Dump
  (frequency 1921 Bins, nicht 512×250).

## Ernte-Nachlauf

- Argovis FWHM ungemessen (0.0 benannt). MPC-Shard: UnnObs-Dispatch + shard-url
  (Operator-Wort). Fink-Konus dead. Broker/GW-Positionen pending. Witness-CDN +
  Gaia-Dispatch gated (Push + Consent).
- Gaia DR3 XP-Spektren (GAVO `dc.g-vo.org`, `gdr3spec.spectra`): Pilot +
  parallax>20-Teilmenge registriert. OFFEN: vollständige Survey — Async läuft
  nicht (anonyme UWS-Jobs IP-gebunden); Pfad = gebandetes Sync über
  `gaia_xp_compiler --source-range <lo> <hi>`. (Schritt: Band-Raster festlegen
  + Workflow über die Bänder fahren.)
- JVO skynode-TAP akari/irsf/nobeyama/saga: anonym 200 nur unter
  `/skynode/do/tap/<node>/sync` — pending (Tor 1). GHRC-DAAC + ARPANSA-UV +
  NOAA CDO/GEDI/NSIDC/PODAAC-SWOT entblockt → Ernte/Compiler + Konsument offen.
- VLASS: offene Tabelle `cirada.VLASS_Source` (votable+csv 200, anonym);
  `cirada.VCSS` = 403. CORS `.24S` = teqc-QC. NOIRLab: Gaia DR4 noch nicht
  erschienen (≥ Dez 2026), Wiedervorlage 2026-12-02 hält.
- giveup_scan-Sediment: 4553 Fundstellen (declined 3277 / descoped 550 /
  gated 397 / honest-face 183) — Aufräum-Atom. (Schritt:
  `cargo run -p omegaflow-utils --bin giveup_scan --summary` je Klasse gegen die
  Register prüfen.)
- Baum ist nicht ruhig: die fremde Bau-Session arbeitet/committet parallel
  (HEAD `24fb03a`, zwei Commits während dieses Atoms); Push zurückgestellt.
  `handover-2026-09-15-ernte-folge25.md` bleibt live (trägt einen fremden,
  uncommitteten Ein-Zeilen-Hunk) — nicht verschoben.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
