<!--
  title: Handover — Mycelium-Folge 168 (2026-09-26)
  session: Mycelium-Folge 168
  class: handover
  date: 2026-09-26
  sha256: df0e6a1340d86d833f8eadbbee7a8f0c396ec280d37bf2f964c07acf0960de98
  status: live
-->
# Handover — Mycelium-Folge 168 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge. Sortierung: erst Akteur (**Linie** | **Rat** | **Operator** |
**Dritter**), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
**Trigger** / **Lage** / **Blockade** / **Braucht**. Status-Tag: `autonom` |
`operator-gebunden` | `blockiert` | `wartend` | `termin` | `LOCK`.

Diese Session konsumierte `handover-2026-09-26-mycelium-folge167.md` und führte den
bestätigten Plan aus (9 Taucher + 3 Folge-Taucher). Gemessen in dieser Sitzung
(HEAD-Start `fa398b1f`, HEAD-Ende `b3ea42294` nach Commit+Push): ceic/TAP/K88GFI lebendig
bzw. query-def, kegel/corona `descoped`, Pioneer-ATDF-Phantom aufgelöst, Voyager-Rat-Verdikt
umgesetzt, harvest_reg-Ordnung repariert, hamqsl/ogimet/nohrsc-Port + NOAA-ERI-Umbenennung
gebaut; der pre-commit-Gate fand zwei Fabrikationsmuster in den neuen Compilern, beide
behoben (--out + Altitude nun Pflicht). Was offen bleibt, steht unten.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### CI-Zustand am gepushten `b3ea42294` — ungelesen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Lauf am HEAD `b3ea42294` (ci-check, harvest-dispatch, kernel-flatten).
- **Lage:** (gemessen 2026-09-26 via `git rev-parse`) `HEAD == origin/main == b3ea42294`
  (17 eigene Pfade, Push erfolgt). Der CI-Ausgang am neuen HEAD ist **ungelesen** — kein
  Poll (Regel), der Watchdog-Snapshot ist älter als der Push.
- **Blockade:** CI-Lauf + Snapshot-Alter.
- **Braucht:** `/tmp/opencode/ci_status.md` bzw. `ci_manage list` beim nächsten Pass; bei Rot
  `ci_manage log <id>`.

#### Neue Quellen-Ports — CDN-Manifestation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Schritt.
- **Lage:** (gemessen 2026-09-26 via grind-pro) `hamqsl`/`ogimet`/`nohrsc_snowfall`
  Compiler gebaut (`tools/harvest/src/bin/`), `sources.φ`-Blöcke geschrieben, pending-Zeilen
  aus `phi/blocked_sources.φ` entfernt; `eri_compiler` → `noaa_eri_compiler` umbenannt.
  Gemessen ist nur die Kompilierung (`cargo check` grün); **kein Compiler-Lauf gegen die
  Live-Quelle** — kein Asset geschrieben, keine Manifestation. Der TIFF-JPEG-Dekoder-Vorbau
  (`src/archivar/tiff.rs`) ist grind-pro-Behauptung, nicht selbst nachgemessen.
- **Blockade:** die Workflow-Lage ist **ungemessen** (das `glob` auf `.github/workflows`
  greift ins Leere — verstecktes Verzeichnis); offen, ob ein `*-cdn.yml` je Quelle existiert
  und wie kernel-flatten eine Quelle manifestiert.
- **Braucht:** die Workflow-Lage zuerst messen (`sgrep <format> .github/workflows`), dann je
  Quelle einen CDN-Workflow + ersten Manifest-Lauf dispatchen; prüfen, ob eine Workflow-Referenz
  den alten Namen `eri_compiler` trägt.

#### tools-latest stale
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** GitHub-API-Rate-Limit-Erholung / nächster Push.
- **Lage:** (gemessen 2026-09-26 via `sread target/release/.tools_manifest`) `git_sha=617f6183a2fe72915b7a36cf91b3d3e76740b187` < HEAD (damals `6de8fa3ff`; nach dem Push `b3ea42294` — **nicht** neu gemessen) → Bin-Satz stale.
- **Blockade:** gepulltes Artefakt hinkt dem Tree nach.
- **Braucht:** `gh workflow run tools-build.yml`; danach Manifest gegen `git rev-parse HEAD`.

#### harvest-dispatch — nächster CI-Lauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `harvest-dispatch`-Lauf.
- **Lage:** (gemessen 2026-09-26 via grind-flash) die zwei roten Runs `36224018471`,
  `36224114280` scheiterten an `harvest_reg: blocks out of order: 'wwlln_th' before 'bpa_gic'`;
  `phi/harvest.φ` repariert (bpa_gic nach `bc_mpo_mag`, wwlln_th ans Ende),
  `harvest_reg --check` → 26 block(s), all measured and in order, exit 0.
- **Blockade:** Bestätigung durch CI fehlt.
- **Braucht:** nächsten Lauf lesen; bei Rot `ci_manage log <id>`.

#### pre-cdn ci-check
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Lauf am HEAD `b3ea42294`.
- **Lage:** (gemessen 2026-09-26 via Watchdog, vor dem Push) ci-check `36224018458`
  in_progress; die alte Kennung `36192645925` ist überholt.
- **Blockade:** CI-Lauf.
- **Braucht:** `/tmp/opencode/ci_status.md`; bei Rot `ci_manage log <id>`.

#### Manifestations-Hashes (TOAR · Zenodo)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Manifest-Lauf (kernel-flatten / CDN).
- **Lage:** (gemessen 2026-09-26 via grind-flash) kernel-flatten `36224127426` rot:
  `planets: de441 base absent from the index — the full de441.bsp has no carrier`;
  Zenodo `data.zip` 23 506 041 274 B ohne Stream-sha256; TOAR sha256 `8fd55224…` 5-Serien-Sample.
- **Blockade:** Manifest-Lauf.
- **Braucht:** de441.bsp-Carrier im Index nachziehen; Full-Range-Manifest trägt die Hashes nach.

#### api.sensor.community — CI-Puls
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `source-census`-Lauf (in dieser Sitzung **nicht** gemessen — aus folge167 übernommen).
- **Lage:** (gemessen 2026-09-26) direct 403 / Proton 403 (ip-blocked);
  `phi/blocked_sources.φ` `blocked ip-blocked`; Report `source_latency_census.φ`.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view <id>`.

#### sources-Repo — Takt-Widerspruch (Archivar `<->` Doku)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Doku-Abgleich.
- **Lage:** (gemessen 2026-09-26 via grind-flash) `omegaflow/sources` ist **öffentlich**
  (Klon + unauthentifizierter API-200); `refresh.yml` läuft `cron: '17 */6 * * *'`
  (6 h) und ist **Python** (`scripts/refresh_all.py`); I02 = der Katalog-Mirror. `CI_REFRESH_S`
  existiert nicht im Code (grind-flash-Messung; die Fixture-Zeile `commit_gate_vocab.json:26`
  selbst nicht verifiziert) — der Name ist eine Gate-Fixture gegen eine fabrizierte
  5-min-Untergrenze. `archivar-mathematikerin.md:22`
  und `pfeiler-der-architektur.md:155-158` nennen 3 h (`health-check.yml` `0 */3 * * *`).
- **Blockade:** Widerspruch 3 h (Doku, omegaflow-eigen) vs. 6 h (sources-Repo-Mirror).
- **Braucht:** die zwei Takte explizit trennen (health-check 3 h vs. sources-refresh 6 h)
  und die Doku-Stellen + `CI_REFRESH_S`-Nennungen auf die gemessene Wahrheit korrigieren.

#### NSSDC-Anfragen — Wiedervorlage
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Antwort des NSSDCA.
- **Lage:** (gemessen 2026-09-26 via grind-flash, `state/mail/mail_ledger.φ:74-78`) vier
  Anfragen 2026-09-16 raus (PSNO-00007, PSCM-00009, PSPG-00011/00457, plus Juno/Cassini an
  Asmar); **keine** Antwort/Bounce; `PSPA-00605` **nicht gesendet**; keine datierte Frist.
- **Blockade:** Antwort des NSSDCA.
- **Braucht:** eine Wiedervorlage-Frist setzen; `PSPA-00605` senden (Send = Operator-Hand).

#### gap-Orphans — 4× TAP: Verdikt korrigiert (Register)
- **Status:** wartend | **Bindung:** eigen→mountain
- **Trigger:** Parser-Arm.
- **Lage:** (gemessen 2026-09-26 via grind-flash) die vier `pending`-TAP-Einträge sind
  **nicht** tot: der HTTP 500 ist ein ADQL-Spaltenfehler. In `phi/blocked_sources.φ:146,150,154,158`
  auf `blocked parser-def json` umgestellt, mit den gemessenen echten Spalten im `note`
  (ravedr4 → RAdeg,DEdeg,HRV,Teff_K; rcsed_fibermags → objid,corrfibmag_*; hyperleda.galaxies
  → ra,dec,mag,vr; ogle.lightcurves → object_id,magnitude). Register liefert diese Einträge
  owner-getaggt `mountain` — ein Sammel-Punkt über vier TAP-Arm-Qualifyer ist ein Parser-Arm
  (unit-auto-detect), noch keine `gap`-Direktive (Regel: ein gap-Träger über eine nicht
  erklärte Klasse hält nichts).
- **Blockade:** kein Parser-Arm für die vier TAP-JSON-Formen.
- **Braucht:**   Arm bauen (mountain-Linie) oder die `blocked parser-def`-Registrierung in den Twin-Einträgen
  von phi/sources.φ (gavo:7458, padc:7469, voparis:7480, skvo:9526 — gleiche falsche Spalten)
  mitziehen.

#### Secrets ohne Disposition — welche die Weberin braucht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Curation-/Weberin-Pass.
- **Lage:** (gemessen 2026-09-26 via `general`, Homepage) sechs lebende
  Datenquellen-Credentials in `.secrets.local` ohne `phi`-Disposition:
  `GFW_PASS` (Global Forest Watch / Global Nature Watch, `data-api.globalforestwatch.org` 200),
  `GOSAT_GW_MAIL`/`GOSAT_GW_PASS` (GOSAT-GW, NIES/JAXA; Homepage pending),
  `IGETS2_PASS`/`IGETS2_USER` (IGETS, GFZ ISDC/EOST), `RUBIN_PASS` (Rubin/LSST),
  `BABAMUL_*` (Babamul Alert-Broker, Caltech/Univ. Minnesota, Kafka),
  `MOVEBANK_*` (Movebank, Max-Planck Animal-Tracking). Quelle:
  `docs/surveys/survey-2026-09-26-secrets-inventar.md`.
- **Blockade:** keine.
- **Braucht:** je Quelle entscheiden — die Weberin braucht sie → registrieren
  (`phi/sources.φ` per `docs/SOURCE_PORT.md`) oder `declined`; die Infra-/LLM-Keys
  (`FLY_API_TOKEN`, `UNOROUTER_*`, `ZAI_*`, `GEMINI_API_KEY`, `CLOUDFLARE_*`) einem
  Konsumenten zuordnen oder entfernen.

### Operator

#### Fremdmodell-Benchmark — vorbereitet bis zur Kante
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (per Akt).
- **Lage:** (gemessen 2026-09-26 via grind-flash) per-Akt vollständig vorbereitet; Entwurf
  `state/benchmark/fremdmodell-benchmark-2026-09-26.md` (Prompt, Antwortschlüssel Score 0–7,
  Metrik Korrektheit/Dauer, Ziel-Modell chat.z.ai GLM-5.3, Sekundärarm claude.ai).
- **Blockade:** fehlender per-Akt-Consent.
- **Braucht:** Operator-Wort: `Fremdmodell-Benchmark Akt 1 (z.ai GLM-5.3) — ausführen.`

### Dritter

#### termin-Punkte — re-verdict 2026-09-26
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-09-28 (DEMETER) / 2026-10-02 (übrige).
- **Lage:** (gemessen 2026-09-26 via grind-flash):
  `regards.cnes.fr/api/v1/rs-order` 403 F5-ASM-WAF (blockiert) ·
  `pithia.cbk.waw.pl/tap/tables` 500 (blockiert) ·
  `lpf.esac.esa.int/lpfsa-sl/data-action` 500 (blockiert) ·
  `api.lasair.lsst.ac.uk/api/` kein direct-response / Proton 404 (blockiert) ·
  `psa.esa.int/psa-tap/tap/` 200 (offen) ·
  `superdarn.ca/data-download` 200 (offen; der Postfach-„SuperDARN Mirror FAILED" ist nicht reproduziert) ·
  `limadou.ssdc.asi.it/` 200 (offen).
- **Blockade:** WAF/Backend bzw. noch offene Produktfreigabe.
- **Braucht:** `archive_search --verdict <url>` zu den genannten Daten; bei Erholung den
  jeweiligen `*-cdn.yml`-Lauf dispatchen.

## Träger (Prosadokumente)

- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | kegel-/corona-Serie `descoped` (2026-09-26), Header-sha neu; nächster Schritt: keiner offen.
- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | Pioneer-ATDF dtype-12 als Phantom aufgelöst (kein Parser-Gap, folge63); nächster Schritt: `gh workflow run mariner-occlt-cdn.yml`.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | nächster Schritt: `state/mail/mail_ledger.φ` auf MPI-FKF/TRISP-Antwort (`smail`).
- `docs/concepts/mirror-research.md`, `survey-2026-09-03-orphan-verdicts.md`, `survey-2026-09-07-tmp-opencode-scan.md`, `survey-2026-09-14-kapitulationen-pendings-inventur.md`, `survey-2026-09-16-dead-sources-relevanz.md`, `survey-2026-09-16-fremde-parser-sammlungen.md` | abgeschlossene Messung; keine offene Handlung.

## Abschluss

Commit `b3ea42294` ist gepusht (17 eigene Pfade, `HEAD == origin/main`). **Pfad-Hazard
(erledigt, gemessen):** von den geteilten Dateien war nur `src/archivar/extract.rs` gemischt
(fremde `messenger_tnf`/`ams02`/`gll_rss`-Hunks); `phi/sources.φ` und `phi/blocked_sources.φ`
wurden hunk-genau gestaged, `phi/harvest.φ` sowie geo/main_flow/mod/voyager waren
vollständig eigen. Ein fremder gestageter Rename (`mountain-folge166 → archiv/`) wurde aus
dem Index auf HEAD zurückgesetzt.
**Benchmark:** die Routine-Recherche-Klasse ist geschlossen (gemessen 2026-09-16, flash siegt)
— zitiert, in dieser Sitzung kein Doppel-Lauf.
