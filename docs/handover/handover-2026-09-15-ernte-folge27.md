<!--
  title: Handover — Ernte-Folge 27 (Stand 2026-09-15)
  session: Ernte-Folge 27
  class: handover
  date: 2026-09-15
  sha256: c01802d999b08a7084dd96110b30dd64f33b974942ea2f1adbc3dbc18a45e98f
  status: live
-->
# Handover — Ernte-Folge 27 (2026-09-15)

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

- ERI/VLASS/CORS-Compiler gebaut; ERI trägt jetzt eine Index-Route
  (`--index`/`--granule`), kein Skalar-Feld-Konsument → pending; `ERI1` noch
  nicht in `zeuge.rs::magic_identity`. (Schritt: magic_identity + Feld-Leser,
  wenn ein Konsument benannt ist — Operator.)
- GK2A `GKA1`, GOES `GAB1`, GDP `GDPT`: kein Leser in `src/archivar/` → pending.
  (Schritt: Leser je Magic in `src/archivar/` bauen, wenn ein Konsument benannt
  ist.)
- LASzip-Decoder steht; Konsument fehlt → pending. (Schritt: Konsument benennen
  — Operator.)
- Babamul: `api_token` am `/api/babamul`-Pfad 200, `/api/alerts`+`/api/objects`
  401 → pending. (Schritt: Auth-Mechanismus von `/api/alerts` messen — curl mit
  dem Token gegen den Pfad.)
- WFAU VSA/WSA `superseded-by-integrated`, OSA/SSA `dead unreachable`;
  SuperCOSMOS-pm-Kandidat (Wiedervorlage 2026-12-02).
- FITS: `P`-Format dekodiert; Rice-Dekompression fehlt. (Schritt: Rice nur
  bauen, wenn ein Konsument Pixel braucht.)
- NODD-NRS `decline spectral-series` (c3f33f6); Route auf GCS korrigiert. Sitz
  `phi/declined_sources.φ`.

## Celestrak-EOP — Compiler steht, Dateien fremd-staged, Dispatch blockiert

- `celestrak_eop_compiler` + `src/archivar/celestrak_eop.rs` (Magic `EOP1`)
  stehen im Baum, sind aber fremd-staged: beide als `D` (staged deletion) plus
  untracked Re-Creation im Index — fremd gehalten, nicht anfassen. (Schritt:
  erst die fremde Staging auflösen, dann erster `gh workflow run
  celestrak-eop-cdn.yml` nach Push + Consent; das CDN-Asset `celestrak_eop.bin`
  fehlt noch.)

## GES-DISC OAuth — der client_id-Fluss fehlt

- EDL-S3-Wiring steht (`s3://`-Scheme, `sigv4_whole_headers` in `range.rs`);
  Bearer-DAACs (podaac/nsidc/lpdaac) signieren über `EARTHDATA_EDL_TOKEN`
  (200). Gemessen: `.secrets.local` trägt nur `EARTHDATA_EDL_TOKEN`, keinen
  `client_id`. (Schritt: client_id beim Operator — urs.earthdata.nasa.gov
  OAuth-App — dann `S3CredentialRoute::OAuth` in `range.rs` verdrahten.)

## CDN-Manifestation (gated — nur der CI-Manifestator lädt hoch)

- `uscrn-cdn.yml` gebaut (Release `www.ncei.noaa.gov`, `AK_Aleknagik_1_NNE`,
  `--lsk kernels/naif0012.tls`); `euclid` ist TAP-only (`format tap`,
  sources.φ:6509) und hat keinen Compiler — kein Dispatch-Eintrag.
- Dispatch je Quelle nach Push + Consent: celestrak-eop, goes, himawari, gk2a,
  uscrn, cosmic, maxi, isc, nexrad, noaa-ocs-hydrodata, gdp, superdarn, onc,
  wod, cors, vlass, eri.

## Netz-Census (Instrument steht, Ernte offen)

- DONKI-URL auf den keyless Sibling gestellt (200). (Schritt: erster
  `gh workflow run source-census.yml` nach Push + Consent.)

## Bau-Gaps (Inventur)

- GDP parquet-zstd: Decoder fehlt (`parquet.rs:927` codec != 0 → Unhandled,
  `zarr.rs:293` blosc zstd → Unhandled); kein zstd-Decoder im std-only-Stack —
  groß. (Schritt: zstd-Decoder, dann die Codec-Arme verdrahten.)
- COSMIC-2 Tages-Enumeration: `cosmic-cdn.yml:29` hartkodiert einen Tag.
  (Schritt: Tages-Raster/Enumeration im Workflow.)
- Survey §1 (`survey-2026-09-14-kapitulationen-pendings-inventur.md`) trägt 26
  Pendings, nicht 19 — die Handover-Zahl „19" war falsch, korrigiert.
- ONC mat5: gemessen — der Compiler liest die Geometrie aus der Datei
  (`onc_hydrophone_compiler.rs:122–128`, kein 512×250-Konstant); der echte
  Dump trägt 1921 Frequenz-Bins. Offen: `docs/specs/spectral-oscillator.md:57`
  trägt noch „512 bins × 250 Hz" (Zeile 72 flaggt es als unbestätigt).
  (Schritt: Zeile 57 auf die gemessene 1921-Bin-Geometrie stellen.)

## Ernte-Nachlauf

- Argovis FWHM gemessen: pro Kanal **nicht publiziert** (OCR-504 nur „10 oder
  20 nm", PAR breitbandig, CDOM Fluorometer) — `bin_width` bleibt 0.0 benannt,
  keine Annahme.
- MPC-Shard: UnnObs-Dispatch + shard-url (Operator-Wort). Fink-Konus dead.
- Broker/GW-Positionen pending. Witness-CDN + Gaia-Dispatch gated (Push +
  Consent).
- Gaia DR3 XP-Spektren: Voll-Survey über Band-Raster (`gaia_xp_compiler
  --source-range <lo> <hi>`); Async (UWS) läuft nicht, Sync-GAVO ~20000 Zeilen.
  (Schritt: Band-Raster festlegen + Workflow über die Bänder.)
- JVO skynode-TAP (akari/irsf/nobeyama/saga): anonym 200 nur unter
  `/skynode/do/tap/<node>/sync` → pending (Tor 1).
- GHRC-DAAC + ARPANSA-UV + NOAA CDO/GEDI/NSIDC/PODAAC-SWOT entblockt →
  Ernte/Compiler + Konsument offen.
- VLASS `cirada.VLASS_Source`: gemessen 200 auf `ws-uv.canfar.net/youcat/sync`
  (nicht der CADC-Host, der 404 gibt).
- NOIRLab: Gaia DR4 noch nicht erschienen (≥ Dez 2026), Wiedervorlage
  2026-12-02 hält.
- giveup_scan: 4770 Fundstellen (declined 3465 / descoped 564 / gated 415 /
  honest-face 187); 154 klassenlose `decline `-Zeilen (trailing space) in
  `phi/declined_sources.φ` (nicht `dead_sources.φ`). (Schritt:
  `cargo run -p omegaflow-utils --bin giveup_scan --summary` gegen die Register
  prüfen, die 154 Zeilen in `declined_sources.φ` fixen.)

## Baum

- Der Baum ist nicht ruhig. Gemessener Index-Zustand: die fremde Bau-Session
  ist live (HEAD `b3fe2da`, ein Push während dieses Atoms; Token-Rename
  `GH_TOKEN`→`OMEGAFLOW_TOKEN` über ~180 Workflows plus ein rustfmt-Pass noch
  uncommitted). Fremd-staged, nicht anfassen: `folge26` als `D` + untracked
  Re-Creation (keine Archiv-Kopie), `folge25` als `RM` archiv→live plus
  untracked Archiv-Kopie, die beiden celestrak-Dateien als `D` + untracked.
- `folge26` ist von dieser Session konsumiert; seine Rotation ist fremd
  gehalten — unangetastet. (Schritt: nach ruhigem Baum `folge26` ins
  `docs/handover/archiv/` verschieben.)
- Uncommittete eigene Arbeit dieses Atoms (bei ruhigem Baum committen):
  `.github/workflows/uscrn-cdn.yml`,
  `tools/harvest/src/bin/eri_compiler.rs`,
  `tools/harvest/src/bin/noaa_ocs_hydrodata_compiler.rs`.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
