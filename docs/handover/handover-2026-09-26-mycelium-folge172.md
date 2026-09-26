<!--
  title: Handover — Mycelium-Folge 172 (2026-09-26)
  session: Mycelium-Folge 172
  class: handover
  date: 2026-09-26
  sha256: d60f326b9ad320a56a6ea7796cd04bafadf619a543721e79238e90d141cd8c1c
  status: live
-->
# Handover — Mycelium-Folge 172 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge. Sortierung: erst Akteur (**Linie** | **Rat** | **Operator** |
**Dritter**), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
**Trigger** / **Lage** / **Blockade** / **Braucht**. Status-Tag: `autonom` |
`operator-gebunden` | `blockiert` | `wartend` | `termin` | `LOCK`.

Diese Session konsumierte `handover-2026-09-26-mycelium-folge171.md`.

Geschlossen in dieser Folge: Postfach-Ledger-Fix (committet `eb20dad55`); die vier
TAP-`parser-def` (gavo/padc/voparis/skvo) — Twin-URLs nach `phi/sources.φ`
gezogen, in `phi/blocked_sources.φ` auf `descoped` gestellt (dieser Commit).

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### CDN-Workflows hamqsl/ogimet/nohrsc/eri — Lauf-Stand
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Neu-Dispatch `ogimet-cdn.yml` / `nohrsc_snowfall-cdn.yml` / `eri-cdn.yml`.
- **Lage:** (gemessen 2026-09-26 via `ci_manage list` + lokaler Compiler) `ogimet_compiler --station 72594 --year 2026 --month 09 --day 25` läuft lokal **36 synop records (9 rows), roundtrip parses**; die gsynres-Rohantwort trägt `Latitude: 40-47-59N / Longitude: 124-10-00W / Altitude: 13 m` im `<h4>` — der CI-Fehler `36233768757` „no latitude/longitude" war **transient** (Header zum Messzeitpunkt abwesend), **kein parser-gap**. Re-Dispatch `ogimet-cdn 36236220099`, `nohrsc_snowfall-cdn 36236054115`, `eri-cdn 36236056155`.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view 36236220099` / `36236054115` / `36236056155`.

#### kernel-flatten — de441-Carrier
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** kernel-flatten-Lauf.
- **Lage:** (gemessen 2026-09-26 via `git log`) Alt-Rot `36224127426` (`de441 base absent`); `phi/sources_index.φ` + `phi/pipeline/frame_registry.φ` gitignored (`.gitignore:73`).
- **Blockade:** CI-Lauf + fehlendes de441-Asset.
- **Braucht:** `ci_manage list` filter kernel-flatten; nach grünem Flatten de441-Bins + `url/format/origin`-Zeilen nach de440-Muster in `phi/sources.φ`.

#### Artefakt-Frische (tools-latest) — stale
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neuer `tools-build`-Lauf am HEAD `6bdd646`.
- **Lage:** (gemessen 2026-09-26 via `sread target/release/.tools_manifest`) `git_sha=4dc366472` ist **Vorfahr** von HEAD `6bdd646` → der gepullte Bin-Satz ist stale (geerbte Tool-Semantik). Jüngster `tools-build 36235676159` success.
- **Blockade:** CI-Lauf / HEAD-Wechsel.
- **Braucht:** `sread target/release/.tools_manifest --limit 1` vs `git rev-parse HEAD` beim nächsten Pass.

#### Zustand-due (Mycelium-Klasse) — CDN-Assets
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** due im `docs/zustand/external-state.md`.
- **Lage:** (gemessen 2026-09-26) `dr3_stars.bin` `--sniff` HTTP 200, 75 001 828 B — Re-Dispatch `gaia-cdn 36236057714`; `rpw_efield.bin` `--sniff` 200, 2 931 048 B, sha256 `f87c77ec…` **identisch**; `voyager_odr` Shards 200 (Lauf `35143340703` success); Rosetta ODF geschlossen (`:33`).
- **Blockade:** CI-Lauf (gaia).
- **Braucht:** `ci_manage view 36236057714`; bei Erfolg notiert.

#### SuperDARN Ernte + Globus-Mirror-Transfer FAILED
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Transfer nach Globus-Task `af68c4f1-b601-11f1-b9a2-0affd5e180af` (FAILED 2026-09-25).
- **Lage:** (gemessen 2026-09-26) `superdarn.ca/data-download` `--verdict` direct 200 / proton 200; Globus-Task `af68c4f1` ohne anonymen Statuskanal (OAuth2) → kein Re-Trigger ohne Token. MAP 6.561 Dateien/21,93 GB → `data/superdarn/map`; FITACF `phi/sources.φ:9465`; RAWACF `phi/sources.φ:8032`.
- **Blockade:** Globus-Transfer-OAuth-Token.
- **Braucht:** Globus-Transfer-Token (Operator/Globus); danach Transfer neu anstoßen + Ziel-Asset `--sniff` + in `phi/sources.φ` registrieren.

#### DECaPS2 Dataverse — Ernte offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Harvest-Pass `phi/blocked_sources.φ:79`.
- **Lage:** (gemessen 2026-09-26) Dataverse-API `doi:10.7910/DVN/K88GFI` `--verdict` HTTP 200 (72 473 B); 309 865 090 866 B / 100 `fits.gz`; Parser `phi/sources.φ:9628`; anonymer TAP-Weg `decaps_dr2.object`.
- **Blockade:** keine.
- **Braucht:** anonymen TAP-Weg ernten + in `phi/sources.φ` registrieren.

#### Pre-CDN Lost-Blocks — Host-Verdikte nicht integriert
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/pipeline/index.φ:37/39`.
- **Lage:** (gemessen 2026-09-26 `stage/pre-cdn_join_report.txt`) 729 richest-URLs ohne Register, 37 params unregistriert, 4 877 pool-lose Blöcke; `stage/pre_cdn_host_verdict_2026-09-25.txt` (75 Z.) trägt Host-Verdikte.
- **Blockade:** keine.
- **Braucht:** Host-Verdikte in `phi/sources.φ`/`declined_sources.φ`/`blocked_sources.φ` integrieren; 4 877 pool-lose Blöcke (eigene Extraktion).

#### Pre-CDN params — 41 Direktiven + 15 Riss
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/pipeline/index.φ:39`.
- **Lage:** (gemessen 2026-09-26 `stage/pre-cdn_join_report.txt`) 41 params-Direktiven fehlen; 15 params-URLs mit konfliktierender Quelle (Riss: `imag-data.bgs.ac.uk` HAPI source drift).
- **Blockade:** HAPI-Source-Drift (Riss).
- **Braucht:** 41 Direktiven nachziehen; den 15er-Riss als Riss führen, nie glätten.

#### b2find intermagnet — Fanout-Stationsliste
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Harvest-Pass `stage/b2find_intermagnet_candidates.φ`.
- **Lage:** (gemessen 2026-09-26) HAPI-Catalog `https://imag-data.bgs.ac.uk/GIN_V1/hapi/catalog` 200 (1 398 859 B); **154 Stationen** extrahiert → `phi/pipeline/stage/b2find_intermagnet_stations.φ` (gitignored).
- **Blockade:** lat/lon fehlen im HAPI-Catalog (nur Stationscodes).
- **Braucht:** lat/lon je Station aus der INTERMAGNET-Registry nachziehen, Fanout-Einträge bauen.

#### pipeline/index.φ verifiziert-Pools
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/pipeline/index.φ:75-97`.
- **Lage:** (gemessen 2026-09-26 via `register_lookup --open`) acht `[mycelium] verifiziert`-Pools: `oai_arxiv`, `b2find_intermagnet_catalog`, `terrapulse_catalog`, `esa_geomagnetic_catalog`, `archeology_gaps_index`, `copernicus_catalog` (je 0), `grind_vires_catalog` (8), `grind_arcgis_index` (17).
- **Blockade:** keine.
- **Braucht:** Pools in den Harvest aufnehmen oder als leer/disponiert führen.

#### GOES-19 ABI radiance — Asset-Größe 68 B
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `goes-cdn.yml`-Lauf / Content-Konsistenzmessung.
- **Lage:** (gemessen 2026-09-26) `goes-cdn 36235688223` **success**; Record-Note `goes-cdn.yml:26` auf `calib=2` gezogen; `--sniff` `goes_abi_rad.bin` HTTP 200, **68 B**, sha256 `ef3b3e60…` — die 68 B sind ungemessen gegen den erwarteten Radiance-Umfang.
- **Blockade:** Content-Konsistenz ungemessen.
- **Braucht:** `goes_abi_compiler`-Rohlauf / Asset gegen `ABI-L1b-RadC`-Granule prüfen; 68 B als riss/pending führen, nie als gültigen Radiance-Umfang.

#### AllWISE — async-UWS Arm fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Arm `phi/blocked_sources.φ:223`.
- **Lage:** (gemessen 2026-09-26, Register-note) `allsky_4band_p3as_psd` sync-Stall; async-UWS Job 23542882 COMPLETED (1 Zeile, VOTable 1.3); Arm fehlt.
- **Blockade:** keine.
- **Braucht:** async-UWS-Arm für AllWISE bauen/messen.

#### Voyager 1/2 — closed-loop Doppler (`phi/blocked_sources.φ:49`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/blocked_sources.φ:49`.
- **Lage:** (gemessen 2026-09-26) V2 `radio_science_rss=2` dirs (saturn_encounter_data, saturn_occultation_medium_band), kein Cruise-ODF; TRK-2-34/ODF/ATDF request-only.
- **Blockade:** kein offener ODF-Endpunkt.
- **Braucht:** `--verdict` auf die V2-`radio_science_rss`-Verzeichnisse; ODF-Weg oder `request-only`-Verdikt.

#### ivo://src.pas — 500
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Backend-Erholung `src.pas` (`phi/pipeline/ledger.φ:10`).
- **Lage:** (gemessen 2026-09-26 via `--verdict`) `http://pithia.cbk.waw.pl/tap/tables` direct 500 / proton 500 (PostgreSQL), kein Wayback.
- **Blockade:** Backend-Fehler.
- **Braucht:** `--verdict /tap/tables` beim nächsten Pass; Termin 2026-10-02.

#### GOSAT-GW — pending
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Datenverfügbarkeit `https://www.gosat-gw.nies.go.jp`.
- **Lage:** (gemessen 2026-09-26 via `--verdict`) direct absent / proton absent; nur Wayback 2022-09-06, kein Daten-Endpoint; Credential vorhanden.
- **Blockade:** kein offener Endpoint.
- **Braucht:** `--verdict` beim nächsten Pass; bis dahin `pending`.

#### Lasair-LSST Broker
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Backend-Erholung `api.lasair.lsst.ac.uk`.
- **Lage:** (gemessen 2026-09-26 via `--verdict`) direct keine Antwort / proton HTTP 404 (179 B) / kein Wayback; Frontend 200; Token vorhanden.
- **Blockade:** Upstream-Backend.
- **Braucht:** `--verdict https://api.lasair.lsst.ac.uk/api/` beim nächsten Pass.

#### DEMETER Order 18387 — Product-GET 500
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Backend `regards.cnes.fr`.
- **Lage:** (gemessen 2026-09-26 via `--verdict`) `https://regards.cnes.fr/api/v1/rs-order` direct 403 / proton 403 (358 B) — WAF; `DONE_WITH_WARNING`, filesInError 96978, available 0.
- **Blockade:** CNES-Backend (WAF).
- **Braucht:** `--verdict` beim nächsten Pass; Termin 2026-09-28.

#### api.sensor.community — ip-blocked
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `source-census`-Lauf.
- **Lage:** (gemessen 2026-09-26 via `--verdict`) direct 403 (229 B) / proton 403 (229 B) / kein Wayback — ip-blocked; kein Exit-Wechsel (Operator-Wort 2026-09-25: Terms/§ 95a).
- **Blockade:** CI-Lauf / lokaler IP-Block.
- **Braucht:** Proton-freier CI-Puls via `source_latency_census --blocked`.

#### Arbeitsbaum-Formatierung — Autorschaft ungemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Pass `git diff`.
- **Lage:** (gemessen 2026-09-26 via `git status`) reine `cargo fmt`-Umbauten in `skydirection.rs`/`kbo_residue_probe.rs`/`rixs_cuprate_probe.rs`/`suprastrom_form_probe.rs` (fremd, unangetastet); Zuordnung „eigene Ports vs. fremd" ungemessen.
- **Blockade:** keine.
- **Braucht:** beim nächsten Pass zuordnen (eigene behalten, fremde unangetastet).

#### EMODNET HFRADAR NADR — Termin-Re-Messung (`phi/sources.φ:1830`)
- **Status:** termin | **Bindung:** eigen
- **Trigger:** 2026-10-19.
- **Lage:** (gemessen 2026-09-24, Zustands-Ledger `:43`) Asset `emodnet_hfr_nadr.bin` registriert; Re-Messung offen.
- **Blockade:** Fälligkeit.
- **Braucht:** `--sniff` bei Fälligkeit.

#### NED ByParams-Harvest — Token-Wiedervorlage
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** NED-Token-Antwort (`state/mail/mail_ledger.φ:295-296`).
- **Lage:** (gemessen 2026-09-26) NED-Helpdesk (Dave Cook) antwortet: kein FTP-Bulk; `https://ned.ipac.caltech.edu/byparams` `--verdict` 200 (42 118 B) mit 90-min-Limit; ByParams in Declination-Bänder splitten; **Token-Angebot** für Verlängerung — unsere Token-Bitte ist raus (`mail_ledger.φ:296`). Ziel `NEDTAP.objdir` ~11–19 Mio rows.
- **Blockade:** NED-Token-Antwort.
- **Braucht:** Token-Antwort abwarten; dann Declination-Band-Harvest bauen.

### Operator

#### Fremdmodell-Benchmark — vorbereitet bis zur Kante
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (per Akt).
- **Lage:** (gemessen 2026-09-26) Entwurf `state/benchmark/fremdmodell-benchmark-2026-09-26.md` liegt bis zur Kante (Prompt, Antwortschlüssel, Metrik, Ziel chat.z.ai GLM-5.3, Sekundär claude.ai).
- **Blockade:** fehlender per-Akt-Consent.
- **Braucht:** Operator-Wort: `Fremdmodell-Benchmark Akt 1 (z.ai GLM-5.3) — ausführen.`

### Dritter

#### BepiColombo bc_mpo_more — Freigabe-Anfrage
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Antwort `psahelp@cosmos.esa.int`.
- **Lage:** (gemessen 2026-09-18, Register-note `phi/blocked_sources.φ:44`) `release_date 2099-01-01` (89434/89517 proprietär), `data?PRODUCT` 403; Anfrage raus; `bc_mpo_mag` anonym offen.
- **Blockade:** ESA-Freigabe.
- **Braucht:** Wiedervorlage Antwort.

#### NSSDC-Anfragen — Wiedervorlage
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** NSSDCA-Antwort `state/mail/mail_ledger.φ`.
- **Lage:** (gemessen 2026-09-26) vier Anfragen 2026-09-16 raus (PSNO-00007, PSCM-00009, PSPG-00011/00457, Juno/Cassini); letzte Eingänge 2026-09-26 (NED, OpenAlex) — keine NSSDCA-Antwort; `PSPA-00605` ungesendet.
- **Blockade:** Antwort des NSSDCA.
- **Braucht:** Wiedervorlage-Frist; `PSPA-00605` (Send = Operator-Hand).

#### termin-Punkte — re-verdict
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-09-28 (DEMETER) / 2026-10-02 (übrige) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-26 via `--verdict`) keine Erholung bei `regards.cnes.fr` (403) / `pithia.cbk.waw.pl` (500) / `lpf.esac.esa.int` / `api.lasair.lsst.ac.uk` (404); `psa.esa.int/psa-tap/tap/`, `superdarn.ca/data-download` 200 offen.
- **Blockade:** WAF/Backend bzw. offene Produktfreigabe.
- **Braucht:** `--verdict <url>`; bei Erholung den jeweiligen `*-cdn.yml`-Lauf dispatchen.

#### SSDC / Limadou (`phi/pipeline/ledger.φ:14`)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Berechtigungs-Akt/Klärung `phi/pipeline/ledger.φ:14` (dritter).
- **Lage:** (gemessen 2026-09-26) `limadou.ssdc.asi.it/` 200; Portal + CAS-Login funktionieren mit `SSDC_USER`/`SSDC_PASS`, aber „Permission Denied" für den omegaflow-Account.
- **Blockade:** SSDC-Berechtigung.
- **Braucht:** Berechtigungs-Akt / Account-Klärung mit SSDC (Operator-Hand) oder Descope mit Messung.

## Träger (Prosadokumente)

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | nächster Schritt: `gh workflow run mariner-occlt-cdn.yml`.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | nächster Schritt: `state/mail/mail_ledger.φ` auf MPI-FKF/TRISP-Antwort (`smail`).
- `docs/surveys/survey-2026-09-26-secrets-inventar.md` | Dispositionen committet; Namens-Disposition trägt die Future-Übergabe.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | ausstehend nur GOES GSICS (`calib=2` gesetzt; 68-B-Asset offen) + Wiedervorlage 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | offen: Re-Check 3 Force-Kanal + 4 pending + `arvo-registry.sci.am` | nächster Schritt: `archive_search --verdict` je Host.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offen: Disposition der 55 undocumented `stale_pending` | nächster Schritt: `docs/specs/cdn_orphan_verdicts.json` je Netloc disponieren.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | offen: Ziel-Layout `knowledge/`+`backups/` | nächster Schritt: Operator-Wort zum Layout.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen: NOAA-NRS passive-bioacoustic Quell-Entscheidung | nächster Schritt: Register-Eintrag + Compiler.

## Abschluss

Commit-Wort (`/commit`) steht aus.
