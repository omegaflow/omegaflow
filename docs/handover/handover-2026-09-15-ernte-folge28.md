<!--
  title: Handover — Ernte-Folge 28 (Stand 2026-09-15)
  session: Ernte-Folge 28
  class: handover
  date: 2026-09-15
  sha256: bd5aeec880cc97ec70040b4cf18b757d2ffd02f61628dcf149aafaf79897565f
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

## Register-Digest-Überführung (stand in keiner lebenden Übergabe — gemessen)

- LAIC-Pfeil-Reader: CSES-SPA-Login, MiniSEED-Decoder, DEMETER order-flow+parser,
  COSMIC netCDF, TEC-GIM LZW, Ereignisraten-Instrument. (Schritt:
  `papers/laic-arrow-direction.md` lesen, je Baustein Reader/Compiler benennen.)
- QuakeML-1.2-Parser, BGC `/bgcargoplus`, IGETS-SFTP, vDEC-Roh, ONC-Token,
  TNS-anonym, JUNO-Release, TA-Vollkatalog. (Schritt:
  `survey-2026-09-13-weberin-quellen.md` — je Quelle Route + Reader.)
- HAWC/LHAASO registrieren, HAWC-TLS-Kette, LSST-Fink-504 neu messen,
  Fink/ALeRCE-Persistenz, Lasair (Wiedervorlage 2026-09-18 schweigt), Hinson
  1997. (Schritt: erste Messung je Host — curl.)
- Werkzeug-Lücken (ehrlich benannt): S3-Scheme (GRACE-FO/SWOT),
  ODF-Doppler-Extract (Galileo), FITS/TDAT-Reader (AMS-02),
  Parquet/GRIB-2/OPeNDAP. (Schritt: je Reader bauen oder als pending eintragen.)
- Parser-Magic 8 Lücken: auto-frame lat/lon_key, extent per force, cmap,
  window bounding, constant lat/lon_key, map-frame indicator, category
  inheritance, extent-zero. (Schritt: `concepts/parser-magic.md` — je Lücke
  einen Fall benennen.)
- INTERMAGNET / IONEX-GIM / direkter GIC ausstehend. (Schritt:
  `concepts/der-kausalpfeil.md` — Kanal je Quelle festlegen.)
- Akteure: Flotten-Scatter+CMT, Stationsterm +5,69 s, Tonga-Luftgang, W-Phase M9,
  ETOPO1-CDN (395 MB), MiniSEED-Konsolidierung, Erdmoden, DART, Grundwasser,
  Gravimeter-SFTP, Radon. (Schritt: je Akteur Quelle + Route.)
- Code-TE-Drift: 5 deutsche Tool-Namen umbenennen — `doppel_anomalie_compiler`,
  `mseed_messen`, `pioneer11_negativ_fuzzy_probe`, `pioneer_text_korrelation`,
  `s1_post_erfassen`. (Schritt: `survey-2026-09-02-code-te-drift.md`, dann
  `git mv` + Register-Einträge.)
- Daten-Holdings: `abk_dbdt_1h_*`, kegel-log, GIC/corona; new_horizons/voyager1/2
  976-B-Platzhalter; ~50-G-Backup-Ziel-Layout. (Schritt: erste Messung —
  Holdings lesen, Herkunft je Stück benennen.)
- Orphan-Verdicts: 55 undokumentierte `stale_pending`; Step-4 CI-Dedupe neu
  scopen; 14 undokumentierte `repo_tag`. (Schritt:
  `survey-2026-09-03-orphan-verdicts.md` — je Verdict dokumentieren/stretchen.)
- Korpus-Rest (gegen eigenen Stand geprüft, neu): thread-matrix-Lücken
  (Gravimeter, Infraschall, Hydroakustik, seismische Worldlines, HF-Radar, GIC,
  Blitz, BGC-Argo, VHE-Teleskope); WWLLN-netcdf/BPA-GIC; BiSON-Tabelle;
  tmp-opencode-Scratch/SuperDARN-FITACF/NOAA-NRS; codestruktur `tools/`-Check;
  mirror-research ~2300-Quellen-Migration; GLO-30 DEM. (Schritt: je Treffer
  Quelle + Route, sonst streichen.)

## Baum

- Fremde Session weiter live: committet `7c66bff` (Voyager-Saturn-Reader +
  ODF-Compiler) und rotiert die Forschung-Handover selbst. `bau33` trägt noch
  den offenen P8-Punkt (`commit_gate.rs` template_slang/SECURITY-Ausnahme,
  uncommittet) → bleibt live. Fremde Staging nicht angefasst.
- Ernte-Linie konsumiert/archiviert: `folge25`, `folge26`, `folge27` liegen in
  `docs/handover/archiv/`; live ist nur `folge28`.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
