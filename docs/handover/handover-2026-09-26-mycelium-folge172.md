<!--
  title: Handover — Mycelium-Folge 172 (2026-09-26)
  session: Mycelium-Folge 172
  class: handover
  date: 2026-09-26
  sha256: 883eac029f431eea3b85fb97ff980399d0c63044d5a3480ce931aebd82dcbd13
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
(gavo/padc/voparis/skvo, `f889b29c3`); DECaPS2 (`f805e33ec`); **AllWISE
async-UWS komplett** — Arm (`c8e87ab2e`) + Consumer (`extract.rs`/`main_flow.rs`)
+ Quelle `phi/sources.φ:9717` + CI-Job, Dispatch `36238458915`; **GOES-19**
(68 B = 12+56 = ein gültiger Granule-Record).

**Vier harte Taucher (2026-09-26) lieferten Routen/Koordinaten — Register-Integration
steht aus, weil `phi/sources.φ`/`blocked_sources.φ` fremd-schmutzig sind und der
Git-Index fremd-gestagte Arbeit trägt (nur pfad-begrenzte Commits möglich):**
- **b2find lat/lon GELÖST**: `https://wdcapi.bgs.ac.uk/metadata/observatory-metadata?intermagnet=true&historical_instruments=false` (200, 658 399 B, JSON) → **154/154** Stationen (`aae 9.035 38.77 2441 …`) in `phi/pipeline/stage/b2find_intermagnet_stations_latlon.φ`.
- **Pre-CDN params-Drift Root-Cause**: `phi/pipeline/queue/sources_potential_pre-cdn_params.φ` ist **positionsbasiert** (+1-Versatz), ererbt aus dem externen 60k-Korpus `$HOME/backup/archive/omegaflow/omegaflow_archeology/sources/sources_recovery_cdn-merged_60k_lost-blocks.φ`; **kein Generator im Code**. Korrigiertes File `phi/pipeline/stage/pre-cdn_params_missing_in_richest_corrected.φ`: 32/41 korrekt, 5 pending, 4 Riss; **15er-Riss aufgelöst** (je `magnetosphere_intermagnet_<id>_hapi`).
- **SuperDARN Routen**: FITACF `https://sdc-serv.usask.ca/data/{YYYY}/{MM}/{YYYYMMDD}.{hhmm}.{ss}.{radar}.{ch}.fitacf.bz2` (Radare sas/rkn/cly/pgr/inv; Dateiliste per POST `radar=…&date=…` an `https://superdarn.ca/data-download`); MAP anonym via Zenodo „SuperDARN Grid netCDF" (340 CC0-Records).
- **Backends**: **Lasair** LSST tot → **ZTF-Zwilling lebt** (`lasair-ztf.lsst.ac.uk/api/query/` + Token 200); **GOSAT-GW** Route+Credential verifiziert (`product.gosat-gw.nies.go.jp` CUI search/download 200); **src.pas** Host tot → `esc.pithia.eu/data-collections/` 200 anonym; **api.sensor.community** ip-blocked → Spiegel `maps.sensor.community/data/v2/data.json` (200, 8,6 MB) + `archive.sensor.community/`; **DEMETER** keine anonyme Route (WAF, direkt+Proton 403) — `blocked` bleibt.
- **NED** ByParams-Plan bis Kante (Formfelder, 180 Declination-Bänder, exakter curl); **Token fehlt**.
- **NSSDCA**: keine Antwort; `PSPA-00605` ist eine Dataset-ID (kein offener Send); Juno-Anfrage ging an JPL-NAV (Asmar). **ESA BepiColombo**: Antwort da → Freigabe „April 2027" → **termin 2027-04**. **SSDC**: kein account-freier Weg (NEDC-Registrierung = Operator-Hand).

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### Register-Integration der neuen Routen (Folge-Auftrag)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** `phi/sources.φ`/`blocked_sources.φ` frei (kein fremder Working-Tree-Diff).
- **Lage:** (gemessen 2026-09-26) die vier Taucher-Routen sind register-reif; die Register-Dateien tragen fremde uncommittete Änderungen, der Index fremd-gestagte Arbeit (`star-dmax-probe.yml`, `browser-extension/`, river-folge37, spatial-/gate-Refactor) → pfad-begrenzter Commit unmöglich ohne Fremd-Sweep.
- **Blockade:** fremder Shared-Tree/Index.
- **Braucht:** je Route einen Block nachsetzen: GOSAT (`blocked_sources.φ:326` → `released`, Route+Credential), Lasair (`:27` → `descoped`, ZTF-Twin), src.pas (`ledger.φ:10` → `esc.pithia.eu`), sensor.community (`:316` → Spiegel `maps.sensor.community`), SuperDARN FITACF+MAP als neue Quellen, b2find-Koordinaten + Katalog-Block.

#### CDN-Workflows hamqsl/ogimet/nohrsc/eri — Lauf-Stand
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Dispatch `ogimet-cdn.yml` / `nohrsc_snowfall-cdn.yml` / `eri-cdn.yml`.
- **Lage:** (gemessen 2026-09-26) `ogimet_compiler` lokal **36 synop records, roundtrip parses**; der CI-Fehler war transient, kein parser-gap. Re-Dispatch `ogimet-cdn 36236220099`, `nohrsc 36236054115`, `eri 36236056155`.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view 36236220099` / `36236054115` / `36236056155`.

#### AllWISE async-UWS — CI-Manifestation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `allwise-cdn 36238458915` (allwise-tap-Job).
- **Lage:** (gemessen 2026-09-26) Arm + Consumer (`cargo check` 0/0) + Quelle + CI-Job gebaut; Lauf dispatcht. `blocked_sources.φ:223` noch `pending`.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view 36238458915`; bei success `blocked_sources.φ:223` → `released`.

#### kernel-flatten — de441-Carrier
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** kernel-flatten-Lauf.
- **Lage:** (gemessen 2026-09-26) Alt-Rot `36224127426` (`de441 base absent`); `phi/sources_index.φ` + `phi/pipeline/frame_registry.φ` gitignored.
- **Blockade:** CI-Lauf + fehlendes de441-Asset.
- **Braucht:** `ci_manage list` filter kernel-flatten; nach grünem Flatten de441-Bins + Zeilen nach de440-Muster.

#### Artefakt-Frische (tools-latest) — stale
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neuer `tools-build`-Lauf am HEAD.
- **Lage:** (gemessen 2026-09-26) `git_sha=4dc366472` ist Vorfahr von HEAD → stale.
- **Blockade:** CI-Lauf.
- **Braucht:** `sread target/release/.tools_manifest --limit 1` vs `git rev-parse HEAD`.

#### b2find — Fanout-Einträge bauen (Koordinaten liegen vor)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Integration `phi/sources.φ`/`blocked_sources.φ`.
- **Lage:** (gemessen 2026-09-26) 154/154 Koordinaten in `phi/pipeline/stage/b2find_intermagnet_stations_latlon.φ`.
- **Blockade:** Register frei.
- **Braucht:** Fanout-Blöcke je Station aus Code+lat/lon bauen (HAPI-`data`-URL + Position).

#### Pre-CDN params — korrigierte Direktiven integrieren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register/Queue `phi/sources.φ` frei.
- **Lage:** (gemessen 2026-09-26) `pre-cdn_params_missing_in_richest_corrected.φ`: 32/41 korrekt, 5 pending, 4 Riss. Der fehlende Baustein ist ein **deterministischer URL→source-Name-Keyer** (Mapping-Tabelle), sonst bleibt jede Neu-Extraktion positionsanfällig.
- **Blockade:** kein Keyer; Queue-Korpus korrupt.
- **Braucht:** Keyer bauen (tools/) oder korrigiertes File in die Queue übernehmen.

#### DEMETER — WAF, keine anonyme Route
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Backend `regards.cnes.fr`.
- **Lage:** (gemessen 2026-09-26) `rs-order` direct 403 / Proton 403; CDAWeb 0 DEMETER; AMDA Konto-gated. `blocked_sources.φ:76` `pending`.
- **Blockade:** CNES-WAF.
- **Braucht:** Wiedervorlage; alternativ Konto-Weg (Operator).

#### Voyager 1/2 — closed-loop Doppler (`phi/blocked_sources.φ:49`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/blocked_sources.φ:49`.
- **Lage:** (gemessen 2026-09-26) PDS-Rings `voyager_rss_raw/` wieder 200; V2 `radio_science_rss=2` dirs kein Cruise-ODF; TRK-2-34/ODF/ATDF request-only.
- **Blockade:** kein offener ODF-Endpunkt.
- **Braucht:** `--verdict` auf die V2-Verzeichnisse; ODF-Weg oder request-only.

#### Arbeitsbaum-Formatierung — Autorschaft ungemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Pass `git diff`.
- **Lage:** (gemessen 2026-09-26) reine fmt-Umbauten in `skydirection.rs`/3 Probe-Dateien (fremd, unangetastet).
- **Blockade:** keine.
- **Braucht:** beim nächsten Pass zuordnen.

#### EMODNET HFRADAR NADR — Termin-Re-Messung (`phi/sources.φ:1830`)
- **Status:** termin | **Bindung:** eigen
- **Trigger:** 2026-10-19.
- **Lage:** (gemessen 2026-09-24, `external-state.md:43`) Asset registriert; Re-Messung offen.
- **Blockade:** Fälligkeit.
- **Braucht:** `--sniff` bei Fälligkeit.

### Dritter

#### BepiColombo bc_mpo_more — Termin
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2027-04-01 (Freigabe zum Science-Phase-Beginn).
- **Lage:** (gemessen 2026-09-26) Antwort PSA (Bentley) + PI (Iess): MORE-Cruise nicht öffentlich, Freigabe April 2027 nach Peer-Review. Ticket `YYM-342-97327`.
- **Blockade:** Peer-Review/ESA.
- **Braucht:** Wiedervorlage 04/2027, dann MORE-`data_raw`/`calibration_raw` ernten.

#### SSDC / Limadou — wartend auf die neue CSES-02-Prozedur
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur `limadou.ssdc.asi.it` (Sotgiu 2026-09-16).
- **Lage:** (gemessen 2026-09-26) Session als `omegaflow` gültig; `query.php` „Permission Denied". Sotgiu (mail_ledger.φ:79): Umbau für CSES-02, „wait a few weeks"; wir warten (:80).
- **Blockade:** Portal-Umbau (dritter).
- **Braucht:** neue Prozedur abwarten; beim Trigger `query.php` re-messen.

#### termin-Punkte — re-verdict
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-09-28 (DEMETER) / 2026-10-02 (übrige) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-26 via `--verdict`) keine Erholung bei `regards.cnes.fr` (403) / `pithia.cbk.waw.pl` (500) / `api.lasair.lsst.ac.uk` (404); Ersatzrouten gefunden (esc.pithia.eu, lasair-ztf).
- **Blockade:** WAF/Backend bzw. Produktfreigabe.
- **Braucht:** `--verdict <url>`; bei Erholung den `*-cdn.yml`-Lauf dispatchen.

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

Operator-Akte (NSSDCA-Send, NED-Submit, Fremdmodell-Wort) → future-Linie, nicht Mycelium („Aufenthalt = Eigentum").

Commit-Wort (`/commit`) steht aus.
