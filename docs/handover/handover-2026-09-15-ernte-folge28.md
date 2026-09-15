<!--
  title: Handover — Ernte-Folge 28 (Stand 2026-09-15)
  session: Ernte-Folge 28
  class: handover
  date: 2026-09-15
  sha256: 6faf357f02ffb671e91f092bcc295f925db4cc98684777ca63fda47c0b7ab8e7
  status: live
-->
# Handover — Ernte-Folge 28 (2026-09-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage.

## Tor 1 — Compiler stehen, Konsumenten fehlen

- ERI/VLASS/CORS-Compiler gebaut; ERI trägt die Index-Route. `ERI1` fehlt noch
  in `zeuge.rs::magic_identity` + Feld-Leser. (Schritt: magic_identity +
  Feld-Leser, wenn ein Konsument benannt ist — Operator.)
- GK2A `GKA1`, GOES `GAB1`, GDP `GDPT`: kein Leser in `src/archivar/`.
  (Schritt: Leser je Magic, wenn ein Konsument benannt ist.)
- LASzip-Decoder steht; Konsument fehlt. (Schritt: Konsument benennen — Operator.)
- FITS `P`-Format dekodiert; Rice fehlt. (Schritt: Rice nur bei Pixel-Konsument.)
- JVO skynode-TAP (akari/irsf/nobeyama/saga): anonym nur unter
  `/skynode/do/tap/<node>/sync` (Tor 1). (Schritt: Konsument/Proxy benennen.)
- WFAU VSA/WSA `superseded-by-integrated`; SuperCOSMOS-pm-Kandidat
  (Wiedervorlage 2026-12-02 schweigt).
- Babamul: Zugang steht — `Authorization: Bearer <BABAMUL_API_TOKEN>` liefert
  am echten Pfad `/api/babamul/objects?object_id=…` bzw.
  `/api/babamul/surveys/{survey}/alerts?start_jd=&end_jd=` (≤ 1 JD) HTTP 200
  (gemessen 2026-09-15); die 401 waren ein Pfad-Artefakt. (Schritt: Konsument
  benennen — Operator.)

## Register-Hygiene — giveup_scan (fremd-blockiert)

- `giveup_scan --summary` gemessen: 4780 Fundstellen (declined 3471); 154
  klassenlose `decline `-Zeilen (trailing space) in `phi/declined_sources.φ`
  bestätigt. Der Fix ist blockiert — die Datei ist fremd-staged (`MM`).
  (Schritt: nach ruhigem Baum jeder der 154 Zeilen die Klasse aus note/grind
  zuweisen, dann `giveup_scan --summary` gegenprüfen.)

## CDN-Manifestation (gated — nach Push + Consent)

- Dispatch je Quelle: celestrak-eop, goes, himawari, gk2a, uscrn, cosmic, maxi,
  isc, nexrad, noaa-ocs-hydrodata, gdp, superdarn, onc, wod, cors, vlass, eri;
  + `source-census.yml`. (Schritt: `gh workflow run <wf>` nach Push + Consent.)
- celestrak-eop: `celestrak_eop_compiler` + `src/archivar/celestrak_eop.rs`
  fremd-staged (D + untracked Re-Creation); nicht angefasst. (Schritt: fremde
  Staging auflösen, dann erster Dispatch.)
- Gaia DR3 XP Voll-Survey: `gaia-xp-full-cdn.yml` + `gaia_xp_merge.rs` gebaut
  (gemessenes Raster: 2^46 pro 2048-Pixel-Band, 98304 Bänder → 256 Chunks à 384
  Bänder; ein Asset je Chunk). (Schritt: `gh workflow run gaia-xp-full-cdn.yml`
  nach Push + Consent.)

## Ernte-Nachlauf

- GES-DISC OAuth: `.secrets.local` trägt nur `EARTHDATA_EDL_TOKEN`, keinen
  `client_id`. (Schritt: client_id beim Operator, dann `S3CredentialRoute::OAuth`
  in `range.rs` verdrahten.)
- GDP parquet-zstd: kein zstd-Decoder im std-only-Stack (`parquet.rs:927`,
  `zarr.rs:293` → Unhandled). (Schritt: zstd-Decoder, dann die Codec-Arme.)
- COSMIC-2 Tages-Raster im Workflow gebaut (`cosmic-cdn.yml`); ONC-Spec auf
  1921-Bins gestellt (`spectral-oscillator.md`).
- Argovis FWHM nicht publiziert → `bin_width` bleibt 0.0 benannt.
- MPC-Shard UnnObs-Dispatch + shard-url (Operator); Fink-Konus dead.
- Broker/GW-Positionen pending; Witness-CDN + Gaia-Dispatch gated.
- GHRC-DAAC + ARPANSA-UV + NOAA CDO/GEDI/NSIDC/PODAAC-SWOT entblockt →
  Ernte/Compiler + Konsument offen.
- VLASS `cirada.VLASS_Source`: 200 auf `ws-uv.canfar.net/youcat/sync`.
- NOIRLab Gaia DR4 ≥ Dez 2026 (Wiedervorlage 2026-12-02 schweigt).
- Survey §1 trägt 26 Pendings.

## Baum

- Fremde Session war live: Token-Rename committet (`2408ea2`, 176 Workflows).
  Fremd-uncommittet bleiben ~47 Dateien (rustfmt-Pass + Register-Hunks) und 14
  fremd-gestagte Pfade (celestrak_eop D + untracked, folge25/26-Rotation,
  phi-Register). Nicht angefasst.
- `folge27` (von dieser Session konsumiert) ist ins `docs/handover/archiv/`
  verschoben; `folge26` ist fremd-gehalten. (Schritt: nach ruhigem Baum
  `folge26` archivieren.)

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
