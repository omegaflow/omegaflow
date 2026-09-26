<!--
  title: Handover — Mycelium-Folge 170 (2026-09-26)
  session: Mycelium-Folge 170
  class: handover
  date: 2026-09-26
  sha256: a3c4b12949fbc0a66cd65e4851d0323b210df634b897f49c264fa0d1ad605a62
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
- **Lage:** (gemessen 2026-09-26 via `mail_digest --last 6` + `glob`) `state/mail/mail_ledger.φ` ist absent; folge168 zitierte dieselbe Datei mit `:74-78` (NSSDC) — der Riss bleibt in diesem Atom unaufgelöst (die Mailbox wurde nicht geöffnet).
- **Blockade:** `mail_digest`-Ledgerpfad ungemessen.
- **Braucht:** erste Messung: `smail`-Inbox lesen, Ledgerpfad klären.

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
