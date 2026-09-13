<!--
  title: Handover — Ernte-Folge 20 (Stand 2026-09-13)
  session: Ernte-Folge 20
  class: handover
  date: 2026-09-13
  sha256: f69a45d9bc1a6eeb553e95622caab1fa317c019c43609e1f4174868d4a831301
  status: live
-->
# Handover — Ernte-Folge 20 (2026-09-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Entblockt — die Ernte-Duty (Blocked-Sources-Aufräumen, 2026-09-13)

`phi/blocked_sources.φ` ist von ~18 `blocked parser-def` auf 5 geschrumpft: 15
Lücken wurden per Taucher geschlossen (Parser gebaut + an echten Quellen gemessen,
committed). Für jede neu geöffnete Quelle bleibt die **Ernte** — der Konsument/
Compiler + die CDN-Manifestation. Jede trägt ihren Feldblock erst mit gemessenem
Konsumenten (Tor 1), nie die bloße Adresse:

- **MAXI** (Röntgenlichtkurve, headerless) — `extract.rs` headerless-Spalte +
  Rows-Frame; offen: sources.φ-Feldblock + Konsument.
- **NOAA CORS** (RINEX-2.11) — `rinex.rs` Epoch-Zeilen-PRNs; offen: Compiler + CDN.
- **cirada/VLASS** (FITS) — `fits.rs` typisierte Rows; offen: CADC-Vault-URL + Compiler.
- **Euclid-TAP** — `tap_compiler.rs` `{metadata,data}`; offen: Konsument.
- **ISC-Bulletin** (QuakeML) — `quakeml.rs`; offen: Compiler + CDN.
- **GOES-16 / GK2A** (geostationär) — `hdf5.rs` Object-Header-Fix + Projektion;
  offen: GSICS-Kalibrierung + Compiler.
- **Himawari HSD** — `hsd.rs` + `bzip2.rs`; offen: Kalibrierung + Compiler.
- **NEXRAD Level-II** — `nexrad.rs`; offen: Compiler.
- **ONC-Hydrophon** (MAT5) — `mat5.rs`; offen: Bin-Geometrie am ersten echten Run.
- **WOD** (netCDF-4 ragged) — `hdf5.rs`/`netcdf.rs`; offen: Compiler.
- **COSMIC** — netCDF-Granulat-Enumeration; offen: tar.gz-Reader.
- **ERI-TIFF / OCS-GeoPackage** — `tiff.rs` + `gpkg.rs`; offen: STAC-Konsument.
- **GDP-Drifter** — `parquet.rs` + `zarr.rs` + `range.rs` (Range-/Chunk-Streaming);
  offen: Drifter-Trajektorien-Compiler + CDN.
- **SuperDARN** (token-form) — `session.rs` POST→Token→GET; offen: Compiler.

## Blocker (extern gebunden)

- **AQS EPA** — Signup erfolgreich, Key-Mail noch nicht eingetroffen; der Empfänger
  `smail_recv` ist repariert (`target/release/smail_recv` gebaut, Service läuft,
  Webhook `127.0.0.1:1619`) — erneut auslösen.
- **Babamul** — Credentials fehlen (nicht in `.secrets.local`/`markiert.csv`).

## Weberin — offene Fäden (Ernte)

- MPC-Shard-Verdrahtung — der Shard-Modus im `mpcobs_compiler` steht (`--shard
  <prefix>`, packed-number-Bereich, 2³⁰ B je Shard, Name `<prefix>-<lo>-<hi>.bin`,
  Shard-Zahl live berechnet; 7 Tests grün) + `mpcobs-shard-cdn.yml` (UnnObs 455 MB,
  HEAD gemessen 455 768 106 B). OFFEN: der UnnObs-Job dispatcht und misst (a) wahre
  unnobs.bin-Größe (b) inflate-Durchsatz (c) Upload-Zeit; erst mit diesen drei Zahlen
  werden die Shard-`url`-Einträge in sources.φ (je Shard ein Eintrag, Operator-Wort)
  und der NumObs-Job (HEAD gemessen 9 109 431 211 B) registriert.

- Broker/GW-Positionen — pending (gemessene Absenz, re-gemessen 2026-09-13):
  gwosc/graceDB tragen keine anonyme ra/dec-Punktposition. Der Punkt bleibt pending.

## Ernte-Nachlauf (Surveys Weberin-Quellen, Folge)

- Argovis `/bgcargoplus` — live (JSON-Profil, 5 distinkte radiometrische Variablen,
  absent aus argo_bgc.bin: DOWN_IRRADIANCE380/412/490, DOWNWELLING_PAR [em], CDOM
  [diffusion]). OFFEN: der sources.φ-Profilblock (Spiegel der Zeile-1248-Grammatik)
  + die drei `convert_to_si`-Arme (TeV-1/cm2/s → W/m², W/m2/nm → W/m³,
  microMoleQuanta/m2/sec → W/m²) — der Block schreibt erst mit kuratierten Einheiten.

- Gaia-Alerts — kompiliert (`gaia_alerts.bin`, SKD1), Witness-Block steht in
  witnesses.φ (`record skydirection`). OFFEN: der Merge des Assets in
  `skydirections.bin` (`skydirection_compiler --gaia-alerts`-Arm) + SKD1-magic absent
  in `zeuge.rs` `magic_identity` (Zeugen-Gate pending).

- Witness-CDN-Manifestation — `hawc-cdn.yml`, `lhaaso-cdn.yml`, `gaia-alerts-cdn.yml`
  stehen unge-dispatcht; die `.sky1`/`.bin`-Assets manifestieren erst mit dem
  `--ci-mode`-Lauf (zusammen mit dem UnnObs-Job oben).

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene Abschluss-Check.
