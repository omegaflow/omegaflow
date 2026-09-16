<!--
  title: Handover — Ernte-Folge 37 (Stand 2026-09-16)
  session: Ernte-Folge 37
  class: handover
  date: 2026-09-16
  sha256: 6e1a5346583825a15685c83e5727fad56599e6722dc3f66637cd884ed8e23ed7
  status: live
-->
# Handover — Ernte-Folge 37 (2026-09-16)

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
  Ledger `eeadmz1`.)

## Ledger — offene Routen (gemessen 2026-09-16)

- `dachs.fai.kz/tap`, `vo.lmd.jussieu.fr/tap`, `pithia.cbk.waw.pl/tap`: TAP-Front
  HTTP 200, DB-Backend down (PostgreSQL/„connection pool is closed") →
  Re-Check. (Schritt: `archive_search --verdict <url>`.)
- `hfrnode.eu`: apex DNS NXDOMAIN; Route `erddap.hfrnode.eu` live (griddap .dds
  200). Dataset + Block offen. (Schritt: ERDDAP-Dataset wählen.)
- `hfrnet-tds.ucsd.edu`: Host direkt+Proton unreachable (SSL reset), Parent
  `hfrnet.ucsd.edu` 200 (Zertifikat abgelaufen). (Schritt: Re-Check.)
- `mercator.env.nm.gov` (2×): Proton US 200, Layer 0 = Stationen
  (Punktgeometrie + Metadaten), kein AQI-Wert am Layer. (Schritt:
  Messwert-Tabelle/Join.)
- `limadou.ssdc.asi.it`: Account gültig, CAS-Login 200, aber „Permission Denied"
  — PI-Freigabe (Sotgiu) offen. (Schritt: Antwort abwarten.)
- `erddap.emodnet-physics.eu HFRADAR_NADR_Totals`: Route steht
  (Brackets URL-encoded `%5B/%5D`), EWCT −0.337 (2024-09-19). (Schritt:
  Live-Kadenz/letzter Zeitschritt.)
- `userquery.linea.org.br/tap`, `ia2-tap.oats.inaf.it:8080/wgetap`: Route +
  physikalischer Wert gemessen (linea VOTable ra/dec; ia2 CSV
  ra/dec/psfMag/photoz). Fluss-Feld + Block offen. (Schritt: Feld-Auswahl +
  `sources.φ`.)

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
- GEDI L2A / NSIDC ICESat-2 ATL03 / PODAAC SWOT L2 SSH: `s3credentials` öffnet
  mit EDL-Token (SigV4), Requester-Pays-Buckets; `hdf5.rs`/`netcdf.rs` +
  `range.rs` lesen; kein Compiler. (Schritt: Compiler + Konsument.)
- NOAA CDO: Route steht (`sources.φ:1184`). (Schritt: Compiler + Konsument.)
- VLASS `cirada.VLASS_Source`: TAP-Query GET anonym 200 (echte CSV: RA/DEC/Fluss).
  (Schritt: Compiler + Konsument.)
- HAWC: `data.hawc-observatory.org` sendet nur das Leaf-Zert → `curl` verify 60;
  `-k` → 200. (Schritt: TLS-Kette fixen oder Ausnahme — Operator.)
- LHAASO: `www.lhaaso.ac.cn` DNS tot; `english.ihep.cas.cn/lhaaso/` 200.
  (Schritt: IHEP-Pfad als Registrierungsroute — Operator.)
- NOIRLab Gaia DR4 ≥ Dez 2026 (Wiedervorlage 2026-12-02 schweigt).
- Survey §1 trägt 26 Pendings.

## CDN-Manifestation (gated — nach Push + Consent)

- Dispatch je Quelle: celestrak-eop, goes, himawari, gk2a, uscrn, cosmic, maxi,
  isc, nexrad, noaa-ocs-hydrodata, gdp, superdarn, onc, wod, cors, vlass, eri;
  + `source-census.yml`. (Schritt: `gh workflow run <wf>` nach Push + Consent.)
- celestrak-eop: fremd-staged; nicht angefasst. (Schritt: fremde Staging
  auflösen, dann erster Dispatch.)
- Gaia DR3 XP Voll-Survey: `gaia-xp-full-cdn.yml` + `gaia_xp_merge.rs`; GAVO-Konto
  nicht nötig. (Schritt: `gh workflow run` nach Push + Consent.)

## Zugangsanfragen — Ernte-Verdikt (an die Entscheid-Linie)

Gemessen 2026-09-15. Verdikt je versandter Anfrage:
- entbehrlich/redundant/geschlossen (streichen): NOIRLab, JSOC, LPF, GAVO, BiSON
  (anonyme Routen 200; offen nur die Tabelle), IGETS (`igets.bin` in
  `sources.φ:8039`), TOAR (WOUDC liefert dieselben WMO-Daten).
- hält (Antwort/Kontakt offen): CSES-Limadou (L2-Zugang lokal), NSE/Haug
  (Rohdaten descoped), Rubin RSP (RSP via Fink-LSST anonym 200).
- (IGETS-Reader erledigt — `src/archivar/geo.rs` `IGT1` + `extract.rs:166`; offen
  sind nur `IGETS_USER`/`IGETS_PASS` im SFTP-Compiler. BiSON: Route + Compiler
  stehen, `sources.φ`-Block + CDN gated nach Push + Consent.)
- (Schritt: Entscheid-Linie streicht die entbehrlichen Wartepunkte.)

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
- RegTAP-Wiegen (2026-09-15, Ledger): `userquery.linea.org.br/tap` und
  `ia2-tap.oats.inaf.it:8080/wgetap` wiegen `http` (VOTable/CSV-only, kein
  TAP-JSON) — bleiben `ausstehend`, keine Disposition ohne `tap` + physikalischen
  Positionswert. (2026-09-16 gemessen: physikalischer Positionswert liegt vor —
  Fluss-Feld + Block offen, siehe Ledger.)

## Benchmark (flash-first, gemessen 2026-09-16)

- Ledger-Disposition 10 `verifiziert` · flash: ESAC-HSA-Schema + CODAR-Datenroute
  gemessen → 2 disponibel · pro: konservativ, HSA als decline geraten (nicht
  gemessen), keine neue Route · Sieger: flash (mehr gemessen, günstiger).
- Gap 6 `window`-Direktiv · flash: Patch mit `unix_to_tdb`-Vergleich, Parse-Fehler
  still übersprungen · pro: `tdb_to_unix`-Vergleich + `report_anomaly` im Parse-Arm
  · Sieger: pro (Fehlerkanal + honest-absent; flash gleich korrekt, weniger streng).

## Baum

- Fremde Arbeit live (nicht angefasst): `src/mathematikerin/te.rs`,
  `tools/register/src/bin/number_audit.rs`, `src/archivar/*`.
- Ernte-Linie: `folge25`–`folge36` liegen in `docs/handover/archiv/`; live ist
  `folge37`.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
