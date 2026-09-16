<!--
  title: Handover — Ernte-Folge 40 (Stand 2026-09-16)
  session: Ernte-Folge 40
  class: handover
  date: 2026-09-16
  sha256: ad9851cee287c17e9dfff125572937872c8587c72d6f76d652d9e01a94fd5c8d
  status: live
-->
# Handover — Ernte-Folge 40 (2026-09-16)

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

## Compiler-Formate ohne Leser — der eine härteste undatiere Punkt

Ernte-Folge 40 hat sechs CDN-Assets registriert, deren `format` der Archivar
noch nicht liest. Die Compiler stehen (`tools/harvest/src/bin/`), die Magic und
Record-Layouts sind dort gemessen; es fehlt je Format der Reader/Consumer im
`src/archivar` (der `geo.rs`-Magic-Dispatch ist das Muster). Die Quellen ohne
Reader: `catalog_vlass_tap_source`/`catalog_vlass_tap_component` (VLST/VLCT,
16-B-Header magic+count+freq 3.0e9, f64-Records, `vlass_tap_compiler.rs`),
`noaa_cdo_ghcnd_tmax` (NCDO, 8-B-Header, 4×f64, `noaa_cdo_compiler.rs`),
`gedi_l2a` (GED1, `gedi_l2a_compiler.rs`), `icesat2_atl03` (AT31,
`icesat2_atl03_compiler.rs`), `swot_l2_lr_ssh` (SWS1,
`swot_l2_lr_ssh_compiler.rs`). (Schritt: Reader je Format + Field-Key-Mapping
nach `src/archivar`, Muster `geo.rs`; danach erste Messung.)

## CDN-Manifestation (gated — nach Push + Consent)

- Fünf neue Workflows manifestieren die Compiler-Assets und werden vom
  `auto-dispatch.yml` (7081d084) in diesem Push selbst dispatcht:
  `vlass-tap-cdn.yml`, `noaa-cdo-cdn.yml`, `gedi-cdn.yml`, `icesat2-cdn.yml`,
  `swot-cdn.yml`. (Schritt: `gh run view` der auto-dispatched Runs; kein
  Polling in der Session.)
- `auto-dispatch.yml` dispatcht jeden in diesem Push geänderten Workflow
  selbst; nur unveränderte Ziele manuell: celestrak-eop, goes, himawari,
  gk2a, uscrn, cosmic, maxi, isc, nexrad, noaa-ocs-hydrodata, gdp, superdarn,
  onc, wod, cors, vlass, eri + `source-census.yml`. (Schritt: `gh workflow run
  <wf>` nach Push + Consent.)
- celestrak-eop: fremd-staged; nicht angefasst. (Schritt: fremde Staging
  auflösen, dann erster Dispatch.)
- Gaia DR3 XP Voll-Survey: `gaia-xp-full-cdn.yml` + `gaia_xp_merge.rs`;
  GAVO-Konto nicht nötig. (Schritt: `gh workflow run` nach Push + Consent.)

## Sensor-Welle — offene Re-Checks (gemessen 2026-09-16)

- `erddap.emso.eu`-Index — Dataset `OBSEA_moored_buoy_meteo_L1c` seit
  2026-05-12 eingefroren, ein lebendes EMSO-Dataset am Index wählen.
- `mercator.env.nm.gov` AQI — Table 1 `SDE_ADMIN.aqi_t60_recent` liefert echte
  Werte, aber Snapshot 2023-11-07 (max=min, N=19), `max(DATE_TIME)` erneut
  messen.
- `erddap.emodnet-physics.eu HFRADAR_NADR_Totals` — Familie steht seit
  ~2026-07-30/31, `maxTime` erneut messen.
- SondeHub-`serial` und IOOS-Glider-Dataset sind serial-/deployment-spezifisch →
  Kadenz-Re-Check-Duty, kein Dauer-URL.
- IGRA-2 Radiosonde bleibt `blocked_sources.φ parser-def
  zip/fixed-width-text` (kein IGRA-Parser).

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
- HAWC: `data.hawc-observatory.org` sendet nur das Leaf-Zert → `curl` verify 60;
  `-k` → 200. (Schritt: TLS-Kette fixen oder Ausnahme — Operator.)
- LHAASO: `www.lhaaso.ac.cn` DNS tot; `english.ihep.cas.cn/lhaaso/` 200.
  (Schritt: IHEP-Pfad als Registrierungsroute — Operator.)
- NOIRLab Gaia DR4 ≥ Dez 2026 (Wiedervorlage 2026-12-02 schweigt).
- Survey §1 trägt 26 Pendings.

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
- Ernte-Linie: `folge25`–`folge39` liegen in `docs/handover/archiv/`; live ist
  `folge40`.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
