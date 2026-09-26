<!--
  title: Handover — Mycelium-Folge 170 (2026-09-26)
  session: Mycelium-Folge 170
  class: handover
  date: 2026-09-26
  sha256: f4a97ba6c3fd063b32faf8e365df1fdc6918486d63b3a21931e7fb36a45eacc8
  status: live
-->
# Handover — Mycelium-Folge 170 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Keine Rangfolge. Sortierung: erst Akteur (**Linie** | **Rat** | **Operator** |
**Dritter**), dann chronologisch nach `Lage`-Datum. Jeder Punkt aufgeschlüsselt:
**Trigger** / **Lage** / **Blockade** / **Braucht**. Status-Tag: `autonom` |
`operator-gebunden` | `blockiert` | `wartend` | `termin` | `LOCK`.

Diese Session konsumierte `handover-2026-09-26-mycelium-folge169.md`, committete die
eigenen +9 (`GOSAT-GW` `pending`, `Rubin/LSST` `blocked account`) path-scoped und schob
folge169 ins Archiv. Mountain `cc991b751` (gemessen: `phi/sources.φ` +88,
`phi/blocked_sources.φ` −169/+33) ist gepusht; HEAD == `origin/main`.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### Neue Ports hamqsl/ogimet/nohrsc — CDN-Workflows gebaut, Lauf offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Post-Push-CI (`harvest-dispatch`).
- **Lage:** (gemessen 2026-09-26 via grind-flash) `.github/workflows/hamqsl-cdn.yml`, `.github/workflows/ogimet-cdn.yml`, `.github/workflows/nohrsc_snowfall-cdn.yml` neu; `phi/harvest.φ` +3 Blöcke; `harvest_reg --check` → 29 block(s) in order (exit 0); `cargo check -p omegaflow-harvest` grün; `eri-cdn.yml:39` `eri_compiler`→`noaa_eri_compiler`. Die drei Workflows wurden nach dem Push dispatcht: `hamqsl-cdn` → `36233766455`, `ogimet-cdn` → `36233768757`, `nohrsc_snowfall-cdn` → `36233772273`. `eri-cdn.yml` lief bereits (`36232783435`).
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view <id>` der drei Läufe; bei Rot `ci_manage log <id>`. Ungemessen: YAML-/Lauf-Validität der drei neuen Workflows (nie gelaufen).

#### CI am HEAD + Artefakt-Frische
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Lauf am HEAD `97b75797a`.
- **Lage:** (gemessen 2026-09-26 via `ci_manage list` + Watchdog-Snapshot) kein roter Lauf am HEAD; `tools-latest` Manifest `304bf81a` < HEAD (stale); 08:33-Roten (`zigbee-host 36230138135`, `paper-check 36230128388`) stehen auf Parent `b30323da`, nicht am HEAD. Nach Push `tools-build` → `36233776812` dispatcht.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view <id>`, bei Rot `ci_manage log <id>`.

#### kernel-flatten — `--retry` 2→3 + Rerun; de441-Carrier
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** kernel-flatten-Lauf `36233774828`.
- **Lage:** (gemessen 2026-09-26 via research-max/grind-flash) Alt-Rot `36224127426`: `de441 base absent` (`select_system("planets")`), Crawl 5m20s unvollständig; gemessene Ursache: Root-Listings `naid.jpl.nasa.gov`/`ssd.jpl.nasa.gov` returned void; `fetch_text` trägt jetzt `--retry 3`. Nach Push dispatcht (`36233774828`). `phi/sources_index.φ` ist gitignored (Crawl-Output). de441-CDN-Assets nie manifestiert; `de441-cdn-watch.yml` wartet auf die ≥183-MB-Generation.
- **Blockade:** CI-Lauf + fehlendes Asset.
- **Braucht:** `ci_manage view 36233774828`; nach grünem Flatten de441-Bins + `url/format/origin`-Zeilen nach de440-Muster (`ephemeris_de441_{sun,earth,moon}.bin`) in `phi/sources.φ` — vor Asset-Existenz wäre ein Eintrag Fabrication.

#### gap-Orphans 4× TAP + Register-Aufenthalt
- **Status:** wartend | **Bindung:** eigen→mountain
- **Trigger:** Parser-Arm.
- **Lage:** (gemessen 2026-09-26) vier `pending`-TAP als `blocked parser-def json` registriert (`phi/blocked_sources.φ:146,150,154,158`), echte Spalten im `note`; `register_lookup --orphans` nennt `phi/blocked_sources.φ:222` `[mycelium]` (irsa TAP) ohne Aufenthalt; `phi/pipeline/ledger.φ:14` SSDC `ausstehend` `[mycelium]` ohne Handover-Platz.
- **Blockade:** kein Parser-Arm (unit-auto-detect) für die vier TAP-JSON-Formen.
- **Braucht:** Arm bauen (mountain) oder Twin-Einträge `phi/sources.φ` (`gavo:7458`, `padc:7469`, `voparis:7480`, `skvo:9526`) mitziehen; `:222`/`ledger:14` Aufenthalt prüfen.

#### Register-Aufenthalt: GOSAT-GW (`phi/blocked_sources.φ:328`) + AllWISE (`:227`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Register-/Harvest-Pass.
- **Lage:** (gemessen 2026-09-26 via `register_lookup --orphans`) beide `[mycelium]` `pending` und ohne Handover-Aufenthalt (orphan). GOSAT-GW (`https://www.gosat-gw.nies.go.jp`) in diesem Atom +9 committet: Homepage absent (nur Wayback 2022-09-06), kein Daten-Endpoint. AllWISE (`https://irsa.ipac.caltech.edu/TAP`) allsky_4band_p3as_psd: sync-Stall; async-UWS Job 23542882 COMPLETED (1 Zeile, VOTable 1.3).
- **Blockade:** keine.
- **Braucht:** GOSAT-GW bleibt `pending` bis Datenverfügbarkeit (Register-Duty); AllWISE async-UWS-Arm messen und Eintrag führen.

#### Lasair-LSST Broker — offen (`phi/blocked_sources.φ:26`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Backend-Erholung api.lasair.lsst.ac.uk (502).
- **Lage:** (gemessen 2026-09-20, Register-note) `api.lasair.lsst.ac.uk/api` 502 über Proton-Exit (direct 000); Frontend 200; ZTF-Zwilling `lasair-ztf.lsst.ac.uk/api/objects` 401; `LASAIR_LSST_TOKEN` vorhanden (41 Z.) → kein Key-Gap; Token unmessbar bis Upstream erholt.
- **Blockade:** Upstream-Backend.
- **Braucht:** `archive_search --verdict https://api.lasair.lsst.ac.uk/api` beim nächsten Pass.

#### SuperDARN — Ernte offen (`phi/blocked_sources.φ:39`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Globus-Transfer.
- **Lage:** (gemessen 2026-09-22, Register-note) Globus (MAP/fitacf_25/fitacf_30/rawacf); MAP 6.561 Dateien/21,93 GB → `data/superdarn/map` (Task af68c4f1). FITACF `phi/sources.φ:9465`. RAWACF gewährt (Mail 1790021001/1790020962), via FRDR kompiliert → `phi/sources.φ:8032`; Endpoint 200 offen.
- **Blockade:** keine.
- **Braucht:** MAP-Ernte abschließen + in `phi/sources.φ` registrieren.

#### BepiColombo bc_mpo_more — Freigabe-Anfrage (`phi/blocked_sources.φ:44`)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Antwort psahelp@cosmos.esa.int.
- **Lage:** (gemessen 2026-09-18, Register-note) `release_date 2099-01-01` (89434/89517 proprietär), `data?PRODUCT` → 403, kein Konto-Gate; Freigabe-Anfrage raus. `bc_mpo_mag` anonym offen (5020 Zeilen seit 2024-03-12).
- **Blockade:** ESA-Freigabe.
- **Braucht:** Wiedervorlage Antwort; `bc_mpo_mag` bei Bedarf in `phi/sources.φ`.

#### NAIF M10 — frame_registry-Route offen (`phi/blocked_sources.φ:57`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Register-Pass.
- **Lage:** (gemessen 2026-09-25, Register-note) `M10_archive_1.bsp` 51200 B HTTP 200; `ephemeris_mariner10.bin` gebaut (mariner10-ephemeris-cdn 36181036369; 648 B, sha256 `7ad8b8bf`). Offen: **keine mariner10-Route in `frame_registry.φ`**.
- **Blockade:** keine.
- **Braucht:** mariner10-Route in `frame_registry.φ` eintragen.

#### DEMETER Order 18387 — Product-GET 500 (`phi/blocked_sources.φ:75`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** regards.cnes.fr Backend/Order-Freigabe.
- **Lage:** (gemessen 2026-09-25, Register-note) Order `DONE_WITH_WARNING`; `statusDate 2026-09-25T15:08Z`; `filesInErrorCount 96978`; `availableFilesCount 0`; product GET `…/files/<md5>` HTTP 500 / 0 B; `online:false`.
- **Blockade:** CNES-Backend (WAF).
- **Braucht:** `archive_search --verdict https://regards.cnes.fr/api/v1/rs-order` beim nächsten Pass.

#### DECaPS2 Dataverse — Ernte offen (`phi/blocked_sources.φ:79`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Harvest-Pass.
- **Lage:** (gemessen 2026-09-26, Register-note) `doi:10.7910/DVN/K88GFI` Größe 309865090866 B / 100 `fits.gz` (größte 3159522164 B); Parser `phi/sources.φ:9628`; anonymer TAP-Weg `decaps_dr2.object`; Ernte pending.
- **Blockade:** keine.
- **Braucht:** anonymen TAP-Weg ernten + in `phi/sources.φ` registrieren.

#### ivo://src.pas/tap — 500 (`phi/pipeline/ledger.φ:10`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Backend-Erholung src.pas.
- **Lage:** (gemessen 2026-09-21, Register-note) PL-Exit (Operator-Wort): `/tap` HTTP 200; `/tap/tables` HTTP 500 (PostgreSQL localhost:5…).
- **Blockade:** Backend-Fehler.
- **Braucht:** `archive_search --verdict` auf `/tap/tables` beim nächsten Pass.

#### CARRIER_DRIFT `phi/blocked_sources.φ::gap:curation` carrier=13 live=4
- **Status:** wartend | **Bindung:** eigen→mountain
- **Trigger:** nächster Register-Pass.
- **Lage:** (gemessen 2026-09-26 via `register_lookup --open`) der Klassen-Träger nennt 13, live sind 4 Einträge der Klasse `gap:curation`.
- **Blockade:** keine.
- **Braucht:** Trägerzahl gegen Register messen und angleichen (oder als `orphan`-Verdikt führen).

#### Arbeitsbaum-Formatierung — Autorschaft ungemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Pass.
- **Lage:** (gemessen 2026-09-26 via `git diff`) reine `cargo fmt`-Umbauten: `nohrsc_snowfall_compiler.rs`/`ogimet_compiler.rs` (als eigene Ports mitcommittet), `skydirection.rs`/`kbo_residue_probe.rs`/`rixs_cuprate_probe.rs`/`suprastrom_form_probe.rs` (ausgeschlossen); **wer** sie erzeugte, ist nicht gemessen — die Zuordnung „eigene Ports vs. fremd" ist eine Ableitung, keine Messung.
- **Blockade:** keine.
- **Braucht:** beim nächsten Pass zuordnen (eigene Ports behalten, fremde unangetastet).

#### Postfach — Ledger-Widerspruch (riss)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Pass / Mail-Eingang.
- **Lage:** (gemessen 2026-09-26, Future-Folge129) `state/mail/mail_ledger.φ` **existiert** — `wc -l -c` = 164 Zeilen / 1 189 190 B (Einträge mit sehr langen Zeilen); folge168 zitierte `:74-78` (NSSDC). Zwei Messungen widersprechen sich: die Datei ist vorhanden, `mail_digest --last 6` meldet „ledger absent". Die **Ursache** des Widerspruchs ist **ungemessen** — kein Arm ist benannt.
- **Blockade:** keine.
- **Braucht:** erste Messung: `mail_digest`-Ledgerpfad (`tools/service/src/bin/mail_digest.rs`) messen; erst dann den Widerspruch einem Arm zuweisen. Bis dahin die Inbox per `smail`/`read` lesen.

#### NSSDC-Anfragen — Wiedervorlage
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** NSSDCA-Antwort.
- **Lage:** (gemessen 2026-09-26, `state/mail/mail_ledger.φ:74-78`) vier Anfragen 2026-09-16 raus (PSNO-00007, PSCM-00009, PSPG-00011/00457, Juno/Cassini an Asmar); keine Antwort/Bounce; `PSPA-00605` ungesendet. Aus folge168 übernommen — in diesem Atom nicht neu gemessen.
- **Blockade:** Antwort des NSSDCA.
- **Braucht:** Wiedervorlage-Frist setzen; `PSPA-00605` senden (Send = Operator-Hand).

#### api.sensor.community — CI-Puls
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `source-census`-Lauf.
- **Lage:** (gemessen 2026-09-26, aus folge167/168 übernommen — in diesem Atom **nicht** neu gemessen) direct 403 / Proton 403 (ip-blocked); `phi/blocked_sources.φ` `blocked ip-blocked`.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view <id>`.

#### SuperDARN-Mirror — Globus-Transfer FAILED, Neustart
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-26 via `state/mail/mail_ledger.φ:288`) Globus-Transfer
  `af68c4f1-b601-11f1-b9a2-0affd5e180af` „SuperDARN Mirror to omegaflow": 4994 Dateien /
  34 126 266 995 B übertragen, dann **FAILED** (Completion 2026-09-25 10:23 UTC); die
  Benachrichtigung trägt „do not reply" — kein Send nötig. Aufgenommen aus
  Future-Folge 129: die Korrespondenz-/Consent-Linie führt keine Asset-/Transfer-Akte.
- **Blockade:** keine.
- **Braucht:** den abgebrochenen Globus-Mirror-Transfer neu anstoßen; das Ziel-Asset
  anschließend per `archive_search --sniff`/`--verdict` messen und in `phi/sources.φ`
  registrieren.

### Operator

#### Fremdmodell-Benchmark — vorbereitet bis zur Kante
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (per Akt).
- **Lage:** (gemessen 2026-09-26 via grind-flash) per-Akt vollständig vorbereitet; Entwurf `state/benchmark/fremdmodell-benchmark-2026-09-26.md` (Prompt, Antwortschlüssel Score 0–7, Metrik Korrektheit/Dauer, Ziel-Modell chat.z.ai GLM-5.3, Sekundärarm claude.ai).
- **Blockade:** fehlender per-Akt-Consent.
- **Braucht:** Operator-Wort: `Fremdmodell-Benchmark Akt 1 (z.ai GLM-5.3) — ausführen.`

### Dritter

#### termin-Punkte — re-verdict 2026-09-26
- **Status:** termin | **Bindung:** dritter
- **Trigger:** 2026-09-28 (DEMETER) / 2026-10-02 (übrige).
- **Lage:** (gemessen 2026-09-26 via grind-flash, `archive_search --verdict`):
  `regards.cnes.fr/api/v1/rs-order` 403 direct+Proton (WAF) ·
  `pithia.cbk.waw.pl/tap/tables` direct+Proton no-response (vorher 500) ·
  `lpf.esac.esa.int/lpfsa-sl/data-action` 500 ·
  `api.lasair.lsst.ac.uk/api/` direct no-response / Proton 404 ·
  `psa.esa.int/psa-tap/tap/` 200 offen ·
  `superdarn.ca/data-download` 200 offen ·
  `limadou.ssdc.asi.it/` 200 offen. **Keine Erholung** — kein `gh workflow run` ausgelöst.
- **Blockade:** WAF/Backend bzw. offene Produktfreigabe.
- **Braucht:** `archive_search --verdict <url>` zu den genannten Daten; bei Erholung den jeweiligen `*-cdn.yml`-Lauf dispatchen.

## Träger (Prosadokumente)

Die ersten zwei Träger-Schritte sind aus folge168 übernommen und in diesem Atom **nicht** neu gemessen.

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | nächster Schritt: `gh workflow run mariner-occlt-cdn.yml`.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | nächster Schritt: `state/mail/mail_ledger.φ` auf MPI-FKF/TRISP-Antwort (`smail`).
- `docs/surveys/survey-2026-09-26-secrets-inventar.md` | Dispositionen gemessen (GFW declined, GOSAT-GW `pending`, Rubin `blocked account` committet); die verbleibende Namens-Disposition trägt die Future-Übergabe (`state/funding/handover/handover-2026-09-26-future-folge129.md`).

## Abschluss

Commit-Wort (`/commit`) steht aus.
