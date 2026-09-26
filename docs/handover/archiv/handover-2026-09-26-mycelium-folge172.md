<!--
  title: Handover — Mycelium-Folge 172 (2026-09-26)
  session: Mycelium-Folge 172
  class: handover
  date: 2026-09-26
  sha256: 9d66b05a54cdfb03743f504b35ac822073b6c4b2c786237afaa06bf97901f4dc
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

Die vier Diver-Routen wurden integriert: Lasair→ZTF, sensor.community-Spiegel (maps),
GOSAT-Konto, src.pas→esc.pithia.eu (`8dc95bb41`); b2find 154 Stationen + Pre-CDN-
`source_keyer` (`318e456d2`); MODIS LST ×3 frei (`e244131f1`); PurpleAir `declined`
und Rubin/LHAASO/NED `descoped` (`8c6d5af95`).

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### SuperDARN FITACF — CI-Manifestation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `superdarn-cdn 36268001985`.
- **Lage:** (gemessen 2026-09-26) Quellenblock registriert (`phi/sources.φ:9461`, `format superdarn_fitacf`, compiler-Verweis), `blocked_sources.φ:39` → `released`; Workflow `superdarn-cdn.yml` rewrite + Dispatch. DMAP-Parser + hdw.dat-Geolokation live (rkn 14 841 GeoRecords). MAP-Globus `20e8a751…` `ACTIVE`.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view 36268001985`; MAP-Quellenblock (Zenodo netCDF) als Folgeschritt.

#### sensor.community-Spiegel ernten
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/sources.φ`.
- **Lage:** (gemessen 2026-09-26) `api.sensor.community` ip-blocked; Proton-freie Spiegel `maps.sensor.community/data/v2/data.json` 200 (8 620 890 B, sha256 `5dd31cd2…`) + `archive.sensor.community/` (`{YYYY}/{YYYY-MM-DD}_{typ}_sensor_{id}.csv.gz`).
- **Blockade:** keine.
- **Braucht:** Spiegel als Quelle registrieren + Compiler/Arm.

#### IRIS/EarthScope EMC — netCDF-4-Arm + Lizenz
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/blocked_sources.φ:335` (ORPHAN → dieser Träger).
- **Lage:** (gemessen 2026-09-26) `https://data.earthscope.org/archive/seismology/products/emc/netcdf/` 200 (117 652 B); HDF5/netCDF-4 only (`GLAD-M35.r0.1-n4c.nc` 343 763 392 B); 3D-Modelle ohne CSV; Volume-Arm 8 Dateien; netcdf-Katalog-Arm fehlt; **Lizenz absent**.
- **Blockade:** netCDF-4-Arm + Lizenzmessung.
- **Braucht:** netCDF-4-Arm (`src/archivar/hdf5.rs`) + Lizenz messen.

#### src.pas → esc.pithia.eu ernten
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/pipeline/ledger.φ:10`.
- **Lage:** (gemessen 2026-09-26) `pithia.cbk.waw.pl` Host tot (direct+Proton 000); Ersatz `esc.pithia.eu/data-collections/` 200 anonym.
- **Blockade:** keine.
- **Braucht:** esc.pithia.eu-Detailseiten je Collection ernten + Quellenblock.

#### Mariner 10 NAIF SPK — Eintrag prüfen (`phi/blocked_sources.φ:57`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/blocked_sources.φ:57`.
- **Lage:** (gemessen 2026-09-25) `M10_archive_1.bsp` (51 200 B, HTTP 200); `ephemeris_mariner10.bin` gebaut (`mariner10-ephemeris-cdn 36181036369`, 648 B, sha256 `7a…`).
- **Blockade:** keine — möglicherweise bereits geschlossen.
- **Braucht:** Eintrag gegen das gebaute Asset prüfen; bei geschlossen → `released`.

#### CDN-Workflows nohrsc/eri — Lauf-Stand
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Dispatch `nohrsc_snowfall-cdn.yml` / `eri-cdn.yml`.
- **Lage:** (gemessen 2026-09-26) ogimet geschlossen (`ogimet-cdn 36246913114` success); nohrsc `36236054115`, eri `36236056155` dispatcht, Ergebnis offen.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view 36236054115` / `36236056155`.

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
- **Lage:** (gemessen 2026-09-26) `git_sha=4dc366472` Vorfahr von HEAD → stale; `tools-build 36246886892` success — Manifest beim nächsten Pass re-messen.
- **Blockade:** CI-Lauf.
- **Braucht:** `sread target/release/.tools_manifest --limit 1` vs `git rev-parse HEAD`.

#### Pre-CDN params — korrigierte Direktiven in die Queue
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Queue-Pass `phi/pipeline/queue/sources_potential_pre-cdn_9k_richest.φ`.
- **Lage:** (gemessen 2026-09-26) `source_keyer.rs` gebaut (`318e456d2`); 41/41 gefunden, 34 Direktiven in die Queue geschrieben; 3 Riss gelöst (TIRM/TLON/VALL), 1 Riss (MCQG) geführt.
- **Blockade:** keine.
- **Braucht:** Keyer-Output gegenprüfen + MCQG-Riss führen.

#### DEMETER — Download serverseitig zu
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CDPP-Dateistatus `online` (`phi/blocked_sources.φ:76`).
- **Lage:** (gemessen 2026-09-26) `PUT /orders/18387/retry` 200 (autonom, wirkungslos); Status unverändert `DONE_WITH_WARNING`, 0 verfügbar / 96978 Fehler, alle Dateien `online:false`, Download 500/0 B. `restart` (Neuanlage) = operator-gebunden.
- **Blockade:** CDPP-seitiges `online:false`.
- **Braucht:** `restart`-Befehl (Operator) oder Wiedervorlage, wenn CNES die Dateien bereitstellt.

#### GOSAT-GW — Wide/L2 + Quellenblock
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** Register-Pass `phi/sources.φ`.
- **Lage:** (gemessen 2026-09-26) Compiler gebaut (`1e3aeebc7`, L1B Fokus B1-3 median → `G3L1`, live 3648 Records, `cargo check` 0/0, 8 Tests). Offen: GWT3W_L1B (1,1 GB/Datei > Leseschranke), L2_GHG/NO2 (Search 0 Dateien), Band3-Artefakt-Ursache (pending).
- **Blockade:** Wide/L2 ungemessen.
- **Braucht:** GWT3F_L1B-Quellenblock in `phi/sources.φ`; Wide/L2 als eigene Punkte.

#### MODIS LST CMG — CI-Manifestation + Reader
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `modis-cdn 36267998951`.
- **Lage:** (gemessen 2026-09-26) erster Lauf `36265129017` **failure** — nur 1-MiB-Probe, DD-Kette der 42-MB-Granule reicht weiter; `bearer_probe` → **Voll-Fetch** geheilt (`2df078304`, lokal 1 541 913 Records / 61 676 528 B). Re-Dispatch `36267998951`. Offen: `format modis_lst_cmg`-Reader in `extract.rs`; NBIT/SKPHUFF/SZIP-Coder pending.
- **Blockade:** CI-Lauf + Reader.
- **Braucht:** `ci_manage view 36267998951`; Reader-Arm `modis_lst_cmg`.

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
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | nächster Schritt: `state/mail/mail_ledger.φ` auf MPI-FKF/LAB_A-Antwort (`smail`).
- `docs/surveys/survey-2026-09-26-secrets-inventar.md` | Dispositionen committet; Namens-Disposition trägt die Future-Übergabe.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | ausstehend nur Wiedervorlage 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | offen: Re-Check 3 Force-Kanal + 4 pending + `arvo-registry.sci.am` | nächster Schritt: `archive_search --verdict` je Host.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offen: Disposition der 55 undocumented `stale_pending` | nächster Schritt: `docs/specs/cdn_orphan_verdicts.json` je Netloc disponieren.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | offen: Ziel-Layout `knowledge/`+`backups/` | nächster Schritt: Operator-Wort zum Layout.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen: NOAA-NRS passive-bioacoustic Quell-Entscheidung | nächster Schritt: Register-Eintrag + Compiler.

## Abschluss

Operator-Akte (NSSDCA-Send, NED-Submit, Fremdmodell-Wort) → future-Linie, nicht Mycelium („Aufenthalt = Eigentum").

Commit-Wort (`/commit`) steht aus.
