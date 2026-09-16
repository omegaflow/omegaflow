<!--
  title: Handover — Ernte-Folge 39 (Stand 2026-09-16)
  session: Ernte-Folge 39
  class: handover
  date: 2026-09-16
  sha256: 4c048b68942bebd240ab0c7606d2330f79b0e99966659c6bacee4699fb640d9b
  status: live
-->
# Handover — Ernte-Folge 39 (2026-09-16)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur
Commits, der Arbeitsbaum darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.

## EEA-AQ Parquet — der eine härteste undatiere Punkt

- EEA-AQ-Observations: POST-Vertrag gemessen 2026-09-16 (research-max,
  `swagger/v1/swagger.json`): POST `ParquetFile/urls`, content-type
  `application/json`, Body `{"countries":["DE"],"cities":[],"pollutants":["PM10"],"dataset":1,"source":"API","aggregationType":"hour"}`
  (dataset Pflicht: 1=UTD ab 2024, 2=E1a 2013–2023, 3=Airbase 2002–2012) →
  text/csv, eine Parquet-URL je Zeile; Datei `PAR1` verifiziert, Footer-Namen
  Samplingpoint/Pollutant/Unit/Value/Validity/Verification. Offen: kein
  Extrakt-Arm für POST→CSV-URL-Liste→per-File-Parquet (`parquet.rs` liest, kein
  Arm). (Schritt: Zweitfetch-Arm + Parquet-Wert-Verifikation, dann `sources.φ`;
  Ledger `eeadmz1`.) Blocker aufgehoben (Ernte-Folge 39, gemessen 2026-09-16):
  die Bau-Linie hat die 143 fmt/clippy-Dateien committet (6318eea0, dea366ab),
  `git status src/archivar` ist sauber — der Arm ist baubar. (Taucher:
  grind-max — POST→CSV-URL-Liste→per-File-Parquet-Arm + echte Wert-Verifikation;
  das vorgebaute `target/release/omegaflow` läuft `--probe`/`--port` lokal ohne
  Build.)

## Folge-Atom — A–D (nächste Ernte-Session)

- **A — EEA-AQ Parquet-Arm** (grind-max): der härteste Punkt oben; `src/archivar`
  ist frei, kein Kollisions-Blocker mehr.
- **B — Pioneer-Ephemeride** (grind-flash, aus `post.md` gefaltet):
  `ephemeris_pioneer10_daily.bin`/`ephemeris_pioneer11_daily.bin` liegen auf dem
  `ssd.jpl.nasa.gov`-Release (HTTP 200, 2026-09-16), haben keine `url`-Zeile.
  (Schritt: `url https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/ephemeris_pioneer1N_daily.bin`
  + `format ephemeris_binary` + `at pioneer1N_daily` + `ttl 86400` via
  `register_sort --write`; `horizons_compiler --daily` erzeugt sie,
  `pioneer-link-correction.yml` lädt sie.)
- **C — VLASS + NOAA-CDO Compiler** (grind-pro): VLASS `cirada.VLASS_Source`
  TAP-Query anonym 200 (echte CSV RA/DEC/Fluss), NOAA CDO Route steht
  (`sources.φ:1184`, `NOAA_CDO_TOKEN`). (Schritt: `tools/harvest`-Bins +
  `cargo check` 0/0 + Register-Block.)
- **D — GEDI L2A / ICESat-2 ATL03 / SWOT L2 SSH** (grind-max): `s3credentials`
  öffnet mit EDL-Token (SigV4), Requester-Pays-Buckets; `hdf5.rs`/`netcdf.rs` +
  `range.rs` lesen. (Schritt: drei Compiler + Register.)

## Sensor-Welle — Ports disponiert (2026-09-16)

- `phi/sources.φ` +8 (Probe: survivor): Wyoming-Radiosonde (72293, 5 Felder),
  Iowa-RAOB (OAX), GTMBA/PMEL-TAO (`pmelTaoDySst`, T_25), SondeHub (`/sonde/<serial>`),
  AWC-PIREP (`distance=2000`), meteo.lt (Vilnius AMS), IOOS-Glider
  (`bass-20260907T0000`), SmartBay-CTD (`erddap.marine.ie`).
- `phi/blocked_sources.φ` +1: IGRA-2 Radiosonde — `parser-def
  zip/fixed-width-text` (kein IGRA-Parser; `csv_zip` entpackt, `Rows` splittet
  aber per Komma, IGRA-2 ist fixed-width/whitespace; `text_to_json` braucht
  Header >5 Token).
- Offen (Ledger `ausstehend`, Re-Check): `erddap.emso.eu`-Index — Dataset
  `OBSEA_moored_buoy_meteo_L1c` seit 2026-05-12 eingefroren, ein lebendes
  EMSO-Dataset am Index wählen. `mercator.env.nm.gov` AQI — Table 1
  `SDE_ADMIN.aqi_t60_recent` liefert echte Werte, aber Snapshot 2023-11-07
  (max=min, N=19), `max(DATE_TIME)` erneut messen. `erddap.emodnet-physics.eu
  HFRADAR_NADR_Totals` — Familie steht seit ~2026-07-30/31, `maxTime` erneut
  messen.
- SondeHub-`serial` und IOOS-Glider-Dataset sind serial-/deployment-spezifisch →
  Kadenz-Re-Check-Duty, kein Dauer-URL.

## Ledger — offene Routen (gemessen 2026-09-16)

- `dachs.fai.kz/tap`, `vo.lmd.jussieu.fr/tap`, `pithia.cbk.waw.pl/tap`:
  TAP-Front HTTP 200, `sync`-QUERY VOTable-ERROR (PostgreSQL-Backend down) →
  Re-Check. (Schritt: `sync`-QUERY erneut.)
- `limadou.ssdc.asi.it`: Account gültig, CAS-Login 200, „Permission Denied" —
  PI-Freigabe (Sotgiu) offen. (Schritt: Antwort abwarten.)

## Tor 1 — Compiler stehen, Konsumenten fehlen

- ERI/VLASS/CORS-Compiler gebaut; Konsumenten fehlen. (Schritt: Konsument —
  Operator.)
- LASzip-Decoder steht; Konsument fehlt. (Schritt: Konsument — Operator.)
- FITS `P`-Format dekodiert; Rice fehlt. (Schritt: Rice nur bei Pixel-Konsument.)
- JVO skynode-TAP (akari/irsf/nobeyama/saga): anonym nur unter
  `/skynode/do/tap/<node>/sync`. (Schritt: Konsument/Proxy — Operator.)
- WFAU VSA/WSA `superseded-by-integrated`; SuperCOSMOS-pm-Kandidat
  (Wiedervorlage 2026-12-02 schweigt).
- Babamul: Zugang steht (`Authorization: Bearer <BABAMUL_API_TOKEN>`, HTTP 200,
  `blocked_sources.φ` pending). (Schritt: Konsument — Operator.)

## Parser-Magic — Gaps benannt (Konsument fehlt)

- Gap 1 + Gap 8 (Auto-frame aus `lat_key`/`lon_key`; `map` als Frame-Indikator):
  brauchen `Frame::Data` in `types.rs` + das `flush!()`-Gate in `parse.rs`; ein
  neues Enum-Arm bricht 9 erschöpfende `match frame`-Arme. (Schritt: Konsument —
  Operator.)
- Gap 12 (Category/Group-Vererbung): kein `group`-Direktiv. (Schritt:
  Gruppen-Schema entwerfen oder als Curation streichen.)

## Ernte-Nachlauf (gemessen 2026-09-15)

- GES-DISC OAuth: `.secrets.local` trägt nur `EARTHDATA_EDL_TOKEN`, keinen
  `client_id`. (Schritt: client_id — Operator, dann `S3CredentialRoute::OAuth` in
  `range.rs`.)
- MPC-Shard UnnObs-Dispatch + shard-url (Operator); Fink-Konus dead.
- GHRC-DAAC: anonym nur Egress-Login-Seite (200); CMR provider `GHRC_DAAC` live;
  `blocked_sources.φ` pending. (Schritt: Produkt + Konsument — Operator.)
- GEDI L2A / NSIDC ICESat-2 ATL03 / PODAAC SWOT L2 SSH → Compiler-Auftrag D.
- NOAA CDO / VLASS `cirada.VLASS_Source` → Compiler-Auftrag C.
- HAWC: `data.hawc-observatory.org` sendet nur das Leaf-Zert → `curl` verify 60;
  `-k` → 200. (Schritt: TLS-Kette fixen oder Ausnahme — Operator.)
- LHAASO: `www.lhaaso.ac.cn` DNS tot; `english.ihep.cas.cn/lhaaso/` 200.
  (Schritt: IHEP-Pfad als Registrierungsroute — Operator.)
- NOIRLab Gaia DR4 ≥ Dez 2026 (Wiedervorlage 2026-12-02 schweigt).
- Survey §1 trägt 26 Pendings.

## CDN-Manifestation (gated — nach Push + Consent)

- `auto-dispatch.yml` (7081d084) dispatcht jeden in diesem Push geänderten
  Workflow selbst; nur unveränderte Ziele manuell: celestrak-eop, goes, himawari,
  gk2a, uscrn, cosmic, maxi, isc, nexrad, noaa-ocs-hydrodata, gdp, superdarn, onc,
  wod, cors, vlass, eri + `source-census.yml`. (Schritt: `gh workflow run <wf>`
  nach Push + Consent.)
- celestrak-eop: fremd-staged; nicht angefasst. (Schritt: fremde Staging
  auflösen, dann erster Dispatch.)
- Gaia DR3 XP Voll-Survey: `gaia-xp-full-cdn.yml` + `gaia_xp_merge.rs`; GAVO-Konto
  nicht nötig. (Schritt: `gh workflow run` nach Push + Consent.)

## Register-Digest-Überführung (Rest; gegen eigenen Stand geprüft)

- LAIC-Bausteine gemessen; **CSES-SPA-Login fehlt**. (Schritt: Zugang — Operator.)
- JUNO-Disambiguierung: Neutrino-JUNO (not-published) vs NASA Juno (200,
  `blocked_sources.φ:20–22`). (Schritt: entscheiden — Operator.)
- TA-Disambiguierung: Telescope-Array-Vollkatalog vs USArray-TA-FDSN-Stationkatalog
  (200, `fdsn_station_compiler.rs`). (Schritt: entscheiden — Operator.)
- Fink/ALeRCE-Persistenz: lokal 000 (Timeout), via VPN 200 → Proxy-Pfad nötig.
  (Schritt: Proxy-Pfad für den Archivar — Operator.)
- Lasair: 404 Wartungsseite; Wiedervorlage 2026-09-18 schweigt.
- Hinson 1997: 403 Cloudflare-Bot-Shield (bronze OA); Zahlen via offene PDS
  GO-J-RSS-1-ODF re-derivierbar. (Schritt: Registereintrag oder descope — Operator.)
- Offen aus den Werkzeug-Lücken: S3-OAuth (`client_id`), TDAT/FITS-Konsument.
- INTERMAGNET / IONEX-GIM / GIC: INTERMAGNET registriert (`sources.φ:3921+`);
  IONEX-GIM jetzt `blocked_sources.φ` `blocked key-needed` (EDL client_id);
  direkter GIC — `fmi_gic.bin` registriert, Kanal `pending` in
  `concepts/der-kausalpfeil.md:95–97`. (Schritt: client_id — Operator.)
- Akteure (Route-Research 2026-09-15): Erdmoden/Radon — keine Quelle benannt.
  Flotten-Scatter = Messprodukt; Stationsterm +5,69 s = berechnet — Herkunft in
  `handover-2026-09-09-tiefenphasen-flotte.md` benennen.
- Daten-Holdings: `abk_dbdt_1h_*`, kegel-log, GIC/corona; new_horizons/voyager1/2
  976-B-Platzhalter; ~50-G-Backup-Ziel-Layout. (Schritt: erste Messung — Holdings
  lesen, Herkunft je Stück.)
- Orphan-Verdicts: 55 `stale_pending` disponiert; 14 `repo_tag` ohne Register-Heim.
  Offen ist nur die repo_tag-CDN-Bereinigung. (Schritt: Operator-Wort; je Asset
  Byte-Vergleich CDN-Digest ↔ Repo-Raw.)
- Korpus-Rest (Route-Research 2026-09-15): seismische Worldlines, HF-Radar,
  BGC-Argo, GLO-30, SuperDARN-FITACF, NOAA-NRS, BPA-GIC, GIC —
  registriert/declined → erledigt. Offen: Hydroakustik (CTBTO vDEC account-gated),
  Blitz (WWLLN `blocked_sources.φ` account; GLD360 `declined_sources.φ`
  commercial), VHE-Teleskope (H.E.S.S./MAGIC/VERITAS nur HTML-Portale; HESS
  `declined registry`), BiSON (Route + Compiler; CDN gated), mirror-research ~2300
  (Bau-Linie `concepts/mirror-research.md`).

## Baum

- Fremde Arbeit: die Bau-Linie hat das globale fmt/clippy über 143 Dateien
  committet (6318eea0, dea366ab) — der Baum ist frei bis auf
  `docs/paper/twenty-second-band-ground-chain.md` (fremd-uncommittet, nicht
  angefasst).
- Ernte-Linie: `folge25`–`folge38` liegen in `docs/handover/archiv/`; live ist
  `folge39`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
