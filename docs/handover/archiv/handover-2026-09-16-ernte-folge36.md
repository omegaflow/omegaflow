<!--
  title: Handover — Ernte-Folge 36 (Stand 2026-09-16)
  session: Ernte-Folge 36
  class: handover
  date: 2026-09-16
  sha256: 394877b6abdad9b4b96b97cac8f162b0b732288c2c26752814f48569e108ef63
  status: live
-->
# Handover — Ernte-Folge 36 (2026-09-16)

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

## EEA-AQ Parquet — der eine härteste undatierte Punkt

- EEA-AQ-Observations (`eeadmz1-downloads-api-appservice.azurewebsites.net/ParquetFile/urls`,
  Ledger `parser-gap`): GET 405 (POST-only). Oszillator-Gate accept — PM10/PM2.5/
  O3/NO2/SO2/CO µg/m3 (diffusion, τ 3600 stündlich). Der einzige der 10
  `verifiziert`-Einträge mit zweifelsfreier Messung, der nicht nach `sources.φ`
  kann: POST-Vertrag undiscovered + kein Extrakt-Arm für
  POST→Klartext-URL-Liste→per-File-Parquet (`parquet.rs` liest, kein Arm).
  (Schritt: POST-Body-Schema + Zweitfetch-Arm, dann Block in `sources.φ`.)

## Tor 1 — Compiler stehen, Konsumenten fehlen

- ERI/VLASS/CORS-Compiler gebaut; Konsumenten fehlen. (Schritt: Konsument
  benennen — Operator.)
- LASzip-Decoder steht; Konsument fehlt. (Schritt: Konsument — Operator.)
- FITS `P`-Format dekodiert; Rice fehlt. (Schritt: Rice nur bei Pixel-Konsument.)
- JVO skynode-TAP (akari/irsf/nobeyama/saga): anonym nur unter
  `/skynode/do/tap/<node>/sync`. (Schritt: Konsument/Proxy — Operator.)
- WFAU VSA/WSA `superseded-by-integrated`; SuperCOSMOS-pm-Kandidat
  (Wiedervorlage 2026-12-02 schweigt).
- Babamul: Zugang steht — `Authorization: Bearer <BABAMUL_API_TOKEN>` an
  `/api/babamul/objects?object_id=…` bzw. `/api/babamul/surveys/{survey}/alerts`
  HTTP 200 (gemessen 2026-09-15, `blocked_sources.φ`). (Schritt: Konsument — Operator.)

## Parser-Magic — Gaps benannt (Konsument fehlt)

- Gap 1 + Gap 8 (Auto-frame aus `lat_key`/`lon_key`; `map` als Frame-Indikator):
  brauchen `Frame::Data` in `types.rs` + das `flush!()`-Gate in `parse.rs`; ein
  neues Enum-Arm bricht 9 erschöpfende `match frame`-Arme (`membrane.rs`,
  `main_flow.rs`, `matrix.rs`, `port.rs`), und `Data` hat keinen Körper-Anker.
  (Schritt: Konsument für per-Zeile-Datenkoordinaten — Operator.)
- Gap 12 (Category/Group-Vererbung): kein `group`-Direktiv; Curation deklariert
  Defaults je Quelle. (Schritt: Gruppen-Schema entwerfen oder als Curation
  streichen — misst, ob eine Quelle die Vererbung braucht.)

## CDN-Manifestation (gated — nach Push + Consent)

- Dispatch je Quelle: celestrak-eop, goes, himawari, gk2a, uscrn, cosmic, maxi,
  isc, nexrad, noaa-ocs-hydrodata, gdp, superdarn, onc, wod, cors, vlass, eri;
  + `source-census.yml`. (Schritt: `gh workflow run <wf>` nach Push + Consent.)
- celestrak-eop: fremd-staged; nicht angefasst. (Schritt: fremde Staging
  auflösen, dann erster Dispatch.)
- Gaia DR3 XP Voll-Survey: `gaia-xp-full-cdn.yml` + `gaia_xp_merge.rs`; GAVO-Konto
  nicht nötig. (Schritt: `gh workflow run` nach Push + Consent.)

## Ernte-Nachlauf (gemessen 2026-09-15)

- GES-DISC OAuth: `.secrets.local` trägt nur `EARTHDATA_EDL_TOKEN`, keinen
  `client_id`. (Schritt: client_id beim Operator, dann `S3CredentialRoute::OAuth`
  in `range.rs`.)
- MPC-Shard UnnObs-Dispatch + shard-url (Operator); Fink-Konus dead.
- GHRC-DAAC: anonym nur Egress-Login-Seite (200); CMR provider `GHRC_DAAC` live;
  `blocked_sources.φ` pending. (Schritt: Produkt + Konsument — Operator.)
- GEDI L2A / NSIDC ICESat-2 ATL03 / PODAAC SWOT L2 SSH: `s3credentials` öffnet
  mit EDL-Token (SigV4), Requester-Pays-Buckets; `hdf5.rs`/`netcdf.rs` + `range.rs`
  lesen; kein Compiler. `blocked_sources.φ` pending. (Schritt: Compiler + Konsument.)
- ARPANSA-UV: anonyme Route 200 (XML, 17 Stationen); kein XML-Wert-Reader →
  `blocked_sources.φ` `blocked parser-def xml`. (Schritt: XML-Reader + Compiler +
  Konsument.)
- NOAA CDO: Route steht (`sources.φ:1184`); anonymes token-gate re-gemessen.
  (Schritt: Compiler + Konsument.)
- VLASS `cirada.VLASS_Source`: TAP-Query GET anonym 200 (echte CSV: RA/DEC/Fluss,
  Recheck 2026-09-15); bare GET 400, POST 303. (Schritt: Compiler + Konsument.)
- HAWC: `data.hawc-observatory.org` sendet nur das Leaf-Zert → `curl` verify 60;
  `-k` → 200. (Schritt: TLS-Kette fixen oder Ausnahme — Operator.)
- LHAASO: `www.lhaaso.ac.cn` DNS tot; `english.ihep.cas.cn/lhaaso/` 200.
  (Schritt: IHEP-Pfad als Registrierungsroute — Operator.)
- NOIRLab Gaia DR4 ≥ Dez 2026 (Wiedervorlage 2026-12-02 schweigt).
- Survey §1 trägt 26 Pendings.

## Ledger — offene Routen (Stand 2026-09-16)

- `dachs.fai.kz/tap`, `vo.lmd.jussieu.fr/tap`: Backend down (500/000), Schema-
  Discovery unmöglich. (Schritt: Re-Check, `archive_search --verdict <url>`.)
- `pithia.cbk.waw.pl/tap`: Re-Messung widersprüchlich (flash pool-closed, pro 200).
  (Schritt: dritte Messung, dann Schema-Discovery.)
- `cdpp-archive.cnes.fr/`: REGARDS-OSS-Portal, kein Datenendpoint. (Schritt:
  REGARDS `rs-catalog`-Route + Dataset-URN.)
- `limadou.ssdc.asi.it/`: CAS-Login; SSDC_USER/SSDC_PASS lokal. (Schritt:
  Daten-API/Format-Route mit den Creds.)
- `erddap.emodnet-physics.eu HFRADAR_NADR_Totals`: griddap, letzter Zeitschritt
  null. (Schritt: griddap-URL-Form + Live-Kadenz.)
- `erddap.osupytheas.fr HFRADAR_grid_copernicus`: Snapshot Juni 2022, alle null.
  (Schritt: Gitter-Probe auf non-null; sonst dead/decline.)
- `hfrnode.eu/`, `hfrnet-tds.ucsd.edu/`, `mercator.env.nm.gov` (2×): direkt
  transient. (Schritt: `archive_search --verdict <url>` mit Proton-Exit.)
- `userquery.linea.org.br/tap`, `ia2-tap.oats.inaf.it:8080/wgetap`: VOTable/CSV-
  only, kein TAP-JSON. (Schritt: `tap`-Inhaltsprobe + physikalischer Positionswert.)

## Zugangsanfragen — Ernte-Verdikt (an die Entscheid-Linie)

Gemessen 2026-09-15. Verdikt je versandter Anfrage:
- entbehrlich/redundant/geschlossen (streichen): NOIRLab, JSOC, LPF, GAVO, BiSON
  (anonyme Routen 200; offen nur die Tabelle), IGETS (`igets.bin` in
  `sources.φ:8039`), TOAR (WOUDC liefert dieselben WMO-Daten).
- hält (Antwort/Kontakt offen): CSES-Limadou (L2-Zugang lokal), NSE/Haug
  (Rohdaten descoped), Rubin RSP (RSP via Fink-LSST anonym 200).
(IGETS-Reader erledigt — `src/archivar/geo.rs` `IGT1` + `extract.rs:166`; offen
sind nur `IGETS_USER`/`IGETS_PASS` im SFTP-Compiler. BiSON: Route + Compiler
stehen, `sources.φ`-Block + CDN gated nach Push + Consent.)
(Schritt: Entscheid-Linie streicht die entbehrlichen Wartepunkte.)

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
- Akteure (Route-Research 2026-09-15): Erdmoden/Radon — keine Quelle
  benannt. Flotten-Scatter = Messprodukt (Inputs CMT `sources.φ:8275` +
  `fdsn_waveform`), Stationsterm +5,69 s = berechnet — Herkunft in
  `handover-2026-09-09-tiefenphasen-flotte.md` benennen.
- Daten-Holdings: `abk_dbdt_1h_*`, kegel-log, GIC/corona; new_horizons/voyager1/2
  976-B-Platzhalter; ~50-G-Backup-Ziel-Layout. (Schritt: erste Messung — Holdings
  lesen, Herkunft je Stück.)
- Orphan-Verdicts: 55 `stale_pending` disponiert; 14 `repo_tag` ohne Register-Heim.
  Offen ist nur die repo_tag-CDN-Bereinigung. (Schritt: Operator-Wort; je Asset
  Byte-Vergleich CDN-Digest ↔ Repo-Raw.)
- Korpus-Rest (Route-Research 2026-09-15): seismische Worldlines, HF-Radar
  (SuperDARN/CODAR), BGC-Argo, GLO-30, SuperDARN-FITACF, NOAA-NRS, BPA-GIC, GIC —
  registriert/declined → erledigt. Offen: Gravimeter (IGETS-DB gemessen
  2026-09-15 — Daten nur via sftp `igetsftp.gfz.de`, Registrierung, Lizenz
  CC BY 4.0; bereits registriert `sources.φ:8039`, `igets.bin` → erledigt),
  Hydroakustik (CTBTO vDEC account-gated), Blitz (WWLLN `blocked_sources.φ`
  account; GLD360 `declined_sources.φ` commercial), VHE-Teleskope
  (H.E.S.S./MAGIC/VERITAS nur HTML-Portale; HESS `declined registry`), BiSON
  (Route + Compiler; CDN gated), mirror-research ~2300
  (Bau-Linie `concepts/mirror-research.md`).
- RegTAP-Wiegen (2026-09-15, Ledger): `userquery.linea.org.br/tap` und
  `ia2-tap.oats.inaf.it:8080/wgetap` wiegen `http` (VOTable/CSV-only, kein
  TAP-JSON) — bleiben `ausstehend`, keine Disposition ohne `tap` + physikalischen
  Positionswert.

## Benchmark (flash-first, gemessen 2026-09-16)

- Ledger-Disposition 10 `verifiziert` · flash: ESAC-HSA-Schema (`hsa.pacs_point_source_070`,
  flux mJy) + CODAR-Datenroute (`VELU`/`VELV` live) gemessen → 2 disponibel ·
  pro: konservativ, HSA als decline geraten (nicht gemessen), keine neue Route ·
  Sieger: flash (mehr gemessen, günstiger).
- Gap 6 `window`-Direktiv · flash: Patch mit `unix_to_tdb`-Vergleich, Parse-Fehler
  still übersprungen · pro: `tdb_to_unix`-Vergleich + `report_anomaly` im Parse-Arm
  (Fehlerkanal-konform, 0-Kanon: nicht-konvertierbarer Epoch fällt) · beide fingen
  den TDB↔Unix-Einheitenfehler (`Channel.epoch` ist TDB s seit J2000) · Sieger:
  pro (Fehlerkanal + honest-absent; flash gleich korrekt, weniger streng).

## Baum

- Fremde Arbeit live (nicht angefasst): `src/mathematikerin/te.rs`,
  `tools/register/src/bin/number_audit.rs`; Entscheid-Linie hat
  `handover-2026-09-16-entscheid-folge15.md` → `archiv/` gestaged und
  `handover-2026-09-16-entscheid-folge16.md` angelegt.
- Ernte-Linie: `folge25`–`folge35` liegen in `docs/handover/archiv/`; live ist
  `folge36`.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
