<!--
  title: Handover — Mycelium-Folge 172 (2026-09-26)
  session: Mycelium-Folge 172
  class: handover
  date: 2026-09-26
  sha256: 9f638a92db6145a8197b28db65b37b4bb4f85fde1676138081f10b0e14596475
  status: live
-->
# Handover — Mycelium-Folge 172 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge. Sortierung: erst Akteur (**Linie** | **Rat** | **Operator** |
**Dritter**), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
**Trigger** / **Lage** / **Blockade** / **Braucht**. Status-Tag: `autonom` |
`operator-gebunden` | `blockiert` | `wartend` | `termin` | `LOCK`.

Diese Session konsumierte `handover-2026-09-26-mycelium-folge171.md`.

Geschlossen: Postfach-Ledger-Fix (`eb20dad55`); die vier TAP-`parser-def`
(gavo/padc/voparis/skvo) — Twins nach `phi/sources.φ`, auf `descoped` (live 200
verifiziert, `f889b29c3`); **DECaPS2 Dataverse** — anonymer noirlab-TAP live 200,
`blocked_sources.φ:79` descoped (`f805e33ec`); **AllWISE async-UWS komplett gebaut** — Arm (`src/archivar/uws.rs` + `allwise.rs`
+ `allwise_tap_compiler.rs`, live Job 23572491 COMPLETED, 5000 Zeilen →
325 008-B-Bin), Consumer (`extract.rs`-Arm + `main_flow.rs`-Handler + Gate-Test,
`cargo check` 0/0), Quelle `phi/sources.φ:9717`, CI-Job `allwise-cdn.yml`
( Dispatch `36238458915` ). **GOES-19 ABI geklärt**: `HEADER_BYTES 12 + REC_BYTES 56
= 68` B = genau ein gültiger Granule-Record — kein Riss. Die **Pre-CDN-Host-Verdikte waren
bereits integriert** (alle 9 Kandidaten in `declined_sources.φ:1865-2135`); die
„kein Treffer"-Zeilen der Verdict-Datei sind überholt.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### CDN-Workflows hamqsl/ogimet/nohrsc/eri — Lauf-Stand
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Dispatch `ogimet-cdn.yml` / `nohrsc_snowfall-cdn.yml` / `eri-cdn.yml`.
- **Lage:** (gemessen 2026-09-26) `ogimet_compiler` lokal **36 synop records (9 rows), roundtrip parses**; gsynres-H `<h4>` trägt `Latitude: 40-47-59N / Longitude: 124-10-00W / Altitude: 13 m` — der CI-Fehler `36233768757` war **transient, kein parser-gap**. Re-Dispatch `ogimet-cdn 36236220099`, `nohrsc 36236054115`, `eri 36236056155`.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view 36236220099` / `36236054115` / `36236056155`.

#### kernel-flatten — de441-Carrier
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** kernel-flatten-Lauf.
- **Lage:** (gemessen 2026-09-26) Alt-Rot `36224127426` (`de441 base absent`); `phi/sources_index.φ` + `phi/pipeline/frame_registry.φ` gitignored (`.gitignore:73`).
- **Blockade:** CI-Lauf + fehlendes de441-Asset.
- **Braucht:** `ci_manage list` filter kernel-flatten; nach grünem Flatten de441-Bins + Zeilen nach de440-Muster in `phi/sources.φ`.

#### Artefakt-Frische (tools-latest) — stale
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neuer `tools-build`-Lauf am HEAD.
- **Lage:** (gemessen 2026-09-26 via `sread target/release/.tools_manifest`) `git_sha=4dc366472` ist Vorfahr von HEAD → stale; jüngster `tools-build 36235676159` success.
- **Blockade:** CI-Lauf.
- **Braucht:** `sread target/release/.tools_manifest --limit 1` vs `git rev-parse HEAD`.

#### Zustand-due (Mycelium-Klasse) — CDN-Assets
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** due in `docs/zustand/external-state.md`.
- **Lage:** (gemessen 2026-09-26) `dr3_stars.bin` 200 (75 001 828 B) — Re-Dispatch `gaia-cdn 36236057714`; `rpw_efield.bin` 200 (2 931 048 B, sha256 `f87c77ec…` identisch); `voyager_odr` Shards 200; Rosetta ODF geschlossen.
- **Blockade:** CI-Lauf (gaia).
- **Braucht:** `ci_manage view 36236057714`.

#### AllWISE async-UWS — CI-Manifestation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `allwise-cdn 36238458915` (allwise-tap-Job).
- **Lage:** (gemessen 2026-09-26) Arm + Consumer (`extract.rs`/`main_flow.rs`, `cargo check` 0/0) + Quelle `phi/sources.φ:9717` + CI-Job `allwise-cdn.yml` gebaut; Lauf dispatcht. `blocked_sources.φ:223` noch `pending`.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view 36238458915`; bei success `blocked_sources.φ:223` → `released` umtragen.

#### Pre-CDN params — source-Name-Drift (Riss)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Auflösung der source-Name-Drift `phi/pipeline/index.φ:39`.
- **Lage:** (gemessen 2026-09-26, grind-pro) die 41 als „fehlend" gemessenen params-Direktiven sind überwiegend `source`-Namen, die **systemisch driften** (`irail→biosphere_ripe_bgp_default_route`, `copernicus→geosphere_usgs_earthquakes_ingv`, `dsn→astro_orbital_fireballs` …); ein verbatim-Merge schriebe falsche Namen ein = Fabrication. Die 15er-`imag-data.bgs.ac.uk`-HAPI-Linien tragen je 2–3 widersprechende Namen (CLF: `cmo` vs `clf_hapi`) — Riss, nicht geglättet.
- **Blockade:** source-Name-Drift (Riss).
- **Braucht:** Drift-Ursache im Scanner/Generator auflösen; erst dann Direktiven ziehen. Den 15er-Riss als Riss führen.

#### Pre-CDN pool-lose Blöcke — Extraktion
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Extraktions-Pass `stage/pre-cdn_join_report.txt`.
- **Lage:** (gemessen 2026-09-26) 4 877 pool-lose Blöcke (eigene Extraktion, blockade); 729 richest-URLs ohne Register.
- **Blockade:** keine.
- **Braucht:** 4 877 Blöcke klassifizieren + disponieren.

#### b2find intermagnet — Fanout lat/lon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Koordinaten-Quelle `stage/b2find_intermagnet_candidates.φ`.
- **Lage:** (gemessen 2026-09-26) HAPI-Catalog 200 (1 398 859 B), **154 Stationen** extrahiert → `phi/pipeline/stage/b2find_intermagnet_stations.φ`; HAPI-`info` trägt **keine** lat/lon; bestehende INTERMAGNET-Arm e in `phi/sources.φ:1597/5603`.
- **Blockade:** INTERMAGNET-Stationskoordinaten-Quelle nicht lokalisiert.
- **Braucht:** Koordinaten-Registry lokalisieren (`archive_search --github intermagnet`), lat/lon je Station nachziehen.

#### pipeline/index.φ verifiziert-Pools
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/pipeline/index.φ:75-97`.
- **Lage:** (gemessen 2026-09-26) `grind_vires_catalog` 7/8 + `grind_arcgis_index` 16/17 bereits in `sources.φ` gemergt; offen nur die 0-count-Pools (`oai_arxiv`, `b2find_intermagnet_catalog`, `terrapulse`, `esa_geomagnetic`, `archeology_gaps`, `copernicus`).
- **Blockade:** keine.
- **Braucht:** 0-count-Pools als leer/disponiert führen oder Kandidaten ernten.

#### SuperDARN Ernte + Globus-Mirror-Transfer FAILED
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Transfer nach Globus-Task `af68c4f1-b601-11f1-b9a2-0affd5e180af` (FAILED 2026-09-25).
- **Lage:** (gemessen 2026-09-26) `superdarn.ca/data-download` direct 200 / proton 200; Globus-Task ohne anonymen Statuskanal (OAuth2). MAP 6 561 Dateien/21,93 GB → `data/superdarn/map`.
- **Blockade:** Globus-Transfer-OAuth-Token.
- **Braucht:** Token; danach Transfer neu anstoßen + Asset `--sniff` + registrieren.

#### Voyager 1/2 — closed-loop Doppler (`phi/blocked_sources.φ:49`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/blocked_sources.φ:49`.
- **Lage:** (gemessen 2026-09-26) PDS-Rings `voyager_rss_raw/` `--verdict` **wieder 200** (1 584 B; war 504); V2 `radio_science_rss=2` dirs kein Cruise-ODF; TRK-2-34/ODF/ATDF request-only.
- **Blockade:** kein offener ODF-Endpunkt.
- **Braucht:** `--verdict` auf die V2-`radio_science_rss`-Verzeichnisse; ODF-Weg oder `request-only`-Verdikt.

#### ivo://src.pas — 500
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Backend-Erholung `phi/pipeline/ledger.φ:10`.
- **Lage:** (gemessen 2026-09-26 via `--verdict`) `pithia.cbk.waw.pl/tap/tables` direct 500 / proton 500, kein Wayback.
- **Blockade:** Backend-Fehler.
- **Braucht:** `--verdict` beim nächsten Pass; Termin 2026-10-02.

#### GOSAT-GW — pending
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Datenverfügbarkeit `https://www.gosat-gw.nies.go.jp`.
- **Lage:** (gemessen 2026-09-26 via `--verdict`) direct absent / proton absent; nur Wayback 2022-09-06.
- **Blockade:** kein offener Endpoint.
- **Braucht:** `--verdict` beim nächsten Pass.

#### Lasair-LSST Broker
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Backend-Erholung `api.lasair.lsst.ac.uk`.
- **Lage:** (gemessen 2026-09-26 via `--verdict`) direct keine Antwort / proton 404 / kein Wayback; Frontend 200; Token vorhanden.
- **Blockade:** Upstream-Backend.
- **Braucht:** `--verdict` beim nächsten Pass.

#### DEMETER Order 18387 — Product-GET 500
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Backend `regards.cnes.fr`.
- **Lage:** (gemessen 2026-09-26 via `--verdict`) `regards.cnes.fr/api/v1/rs-order` direct 403 / proton 403 (WAF); `DONE_WITH_WARNING`, filesInError 96978.
- **Blockade:** CNES-Backend (WAF).
- **Braucht:** `--verdict` beim nächsten Pass; Termin 2026-09-28.

#### api.sensor.community — ip-blocked
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `source-census`-Lauf.
- **Lage:** (gemessen 2026-09-26 via `--verdict`) direct 403 / proton 403 / kein Wayback — ip-blocked; kein Exit-Wechsel (Operator-Wort 2026-09-25).
- **Blockade:** CI-Lauf / lokaler IP-Block.
- **Braucht:** Proton-freier CI-Puls via `source_latency_census --blocked`.

#### Arbeitsbaum-Formatierung — Autorschaft ungemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Pass `git diff`.
- **Lage:** (gemessen 2026-09-26) reine `cargo fmt`-Umbauten in `skydirection.rs`/`kbo_residue_probe.rs`/`rixs_cuprate_probe.rs`/`suprastrom_form_probe.rs` (fremd, unangetastet).
- **Blockade:** keine.
- **Braucht:** beim nächsten Pass zuordnen (eigene behalten, fremde unangetastet).

#### EMODNET HFRADAR NADR — Termin-Re-Messung (`phi/sources.φ:1830`)
- **Status:** termin | **Bindung:** eigen
- **Trigger:** 2026-10-19.
- **Lage:** (gemessen 2026-09-24, `docs/zustand/external-state.md:43`) Asset registriert; Re-Messung offen.
- **Blockade:** Fälligkeit.
- **Braucht:** `--sniff` bei Fälligkeit.

#### NED ByParams-Harvest — Token-Wiedervorlage
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** NED-Token-Antwort `state/mail/mail_ledger.φ:295-296`.
- **Lage:** (gemessen 2026-09-26) `ned.ipac.caltech.edu/byparams` 200 (42 118 B), 90-min-Limit; Token-Bitte raus; Ziel `NEDTAP.objdir` ~11–19 Mio rows.
- **Blockade:** NED-Token-Antwort.
- **Braucht:** Token-Antwort; dann Declination-Band-Harvest bauen.

### Operator

#### Fremdmodell-Benchmark — vorbereitet bis zur Kante
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (per Akt).
- **Lage:** (gemessen 2026-09-26) Entwurf `state/benchmark/fremdmodell-benchmark-2026-09-26.md` bis zur Kante (Ziel chat.z.ai GLM-5.3).
- **Blockade:** fehlender per-Akt-Consent.
- **Braucht:** Operator-Wort: `Fremdmodell-Benchmark Akt 1 (z.ai GLM-5.3) — ausführen.`

### Dritter

#### BepiColombo bc_mpo_more — Freigabe-Anfrage
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Antwort `psahelp@cosmos.esa.int`.
- **Lage:** (gemessen 2026-09-18, Register-note `phi/blocked_sources.φ:44`) `release_date 2099-01-01`, `data?PRODUCT` 403; Anfrage raus; `bc_mpo_mag` anonym offen.
- **Blockade:** ESA-Freigabe.
- **Braucht:** Wiedervorlage Antwort.

#### NSSDC-Anfragen — Wiedervorlage
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** NSSDCA-Antwort `state/mail/mail_ledger.φ`.
- **Lage:** (gemessen 2026-09-26) vier Anfragen 2026-09-16 raus; keine NSSDCA-Antwort; `PSPA-00605` ungesendet.
- **Blockade:** Antwort des NSSDCA.
- **Braucht:** Wiedervorlage-Frist; `PSPA-00605` (Send = Operator-Hand).

#### termin-Punkte — re-verdict
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-09-28 (DEMETER) / 2026-10-02 (übrige) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-26 via `--verdict`) keine Erholung bei `regards.cnes.fr` (403) / `pithia.cbk.waw.pl` (500) / `api.lasair.lsst.ac.uk` (404); `psa.esa.int/psa-tap/tap/`, `superdarn.ca/data-download` 200 offen.
- **Blockade:** WAF/Backend bzw. offene Produktfreigabe.
- **Braucht:** `--verdict <url>`; bei Erholung den jeweiligen `*-cdn.yml`-Lauf dispatchen.

#### SSDC / Limadou (`phi/pipeline/ledger.φ:14`)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Berechtigungs-Akt/Klärung `phi/pipeline/ledger.φ:14` (dritter).
- **Lage:** (gemessen 2026-09-26) `limadou.ssdc.asi.it/` 200; Login ok, aber „Permission Denied" für omegaflow-Account.
- **Blockade:** SSDC-Berechtigung.
- **Braucht:** Berechtigungs-Akt / Account-Klärung (Operator-Hand) oder Descope mit Messung.

## Träger (Prosadokumente)

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | nächster Schritt: `gh workflow run mariner-occlt-cdn.yml`.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | nächster Schritt: `state/mail/mail_ledger.φ` auf MPI-FKF/TRISP-Antwort (`smail`).
- `docs/surveys/survey-2026-09-26-secrets-inventar.md` | Dispositionen committet; Namens-Disposition trägt die Future-Übergabe.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | ausstehend nur Wiedervorlage 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | offen: Re-Check 3 Force-Kanal + 4 pending + `arvo-registry.sci.am` | nächster Schritt: `archive_search --verdict` je Host.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offen: Disposition der 55 undocumented `stale_pending` | nächster Schritt: `docs/specs/cdn_orphan_verdicts.json` je Netloc disponieren.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | offen: Ziel-Layout `knowledge/`+`backups/` | nächster Schritt: Operator-Wort zum Layout.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen: NOAA-NRS passive-bioacoustic Quell-Entscheidung | nächster Schritt: Register-Eintrag + Compiler.

## Abschluss

Commit-Wort (`/commit`) steht aus.
