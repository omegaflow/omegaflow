<!--
  title: Handover — Mycelium-Folge 177 (2026-09-27)
  session: Mycelium-Folge 177
  class: handover
  date: 2026-09-27
  sha256: 320997f43f2e006d738fc9ea2c3c89fedde9d8fc209680c057e4534d6d76b0d3
  status: archivist
-->
# Handover — Mycelium-Folge 177 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `wartend` | `operator-gebunden` |
`blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-27-mycelium-folge176.md`.

## Operator-Wort-Register

- Wort | 2026-09-26 | „all" — session-weiter Consent (`mycelium_go`), Delegation an alle Taucher.
- Wort | 2026-09-27 | „DEMETER ist Sensory" → DEMETER bleibt aus der Mycelium-Übergabe (Sensory führt ihn).
- Wort | 2026-09-27 | „du machst GOSAT" → GOSAT bleibt Mycelium-Punkt.
- Wort | 2026-09-27 | „monthly + 8-day als per-Granule-Serie bauen; daily descopen" → modis-cdn per-Granule + Manifest; daily descoped.
- Wort | 2026-09-27 | „volume" — gibt die φ-Direktive `volume` (3D-Gitter-Ingest) frei.
- Wort | 2026-09-27 | „Du kannst" (Phase-2-Ausführung bestätigt) — session-weiter Delegations-Consent; nicht das Commit-Wort.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### GOSAT-GW GWT3F_L1B — 4096-Granule-Gate reißt (Empty/Overflow-Fix wirkt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `gosat-cdn`-Lauf grün (nach Re-Dispatch am Fix).
- **Lage:** (gemessen 2026-09-27 via Browser/Job-Log `36302098032`) der
  Empty/Overflow/Void-Fix wirkt (Bisektion läuft sauber); der Lauf reißt am neuen
  Gate: `gosat_tanso3_compiler: GWT3F_L1B 2024-01-01..2026-12-31 carries 10313
  granules — over the 4096 granule bound` (`MAX_SEARCH_RESULTS = 1 << 12`,
  `gosat_tanso3_compiler.rs:21`, Gate `:1186`).
- **Blockade:** keine.
- **Braucht:** Fenster-Jahres-Sharding (Compiler oder Workflow), dann Re-Dispatch;
  bei success sha256 in `phi/sources.φ` bzw. `phi/blocked_sources.φ`.

#### modis-cdn — Release-Cap (1000 Assets) erreicht, Serien-Manifeste fehlen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `modis-cdn`-Lauf `36283216818` failed.
- **Lage:** (gemessen 2026-09-27 via Browser/Job-Logs + GH-API) Lauf `36283216818`
  **failed**: das CDN-Release `data.lpdaac.earthdatacloud.nasa.gov` hat exakt 1000
  Assets erreicht → jeder weitere Upload `HTTP 422: file_count limited to 1000
  assets per release`; monthly 2017–2022 gemessen rot, `series-manifest` nie
  gestartet; `modis_lst_cmg_8day.manifest`/`_monthly.manifest` existieren am CDN
  nicht.
- **Blockade:** keine.
- **Braucht:** Release-Sharding / zweites Release, dann Re-Dispatch.

#### secrets-inventar — Prosa-Träger, 6 Lebendquellen-Disposition
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** `phi/sources.φ` bzw. `phi/declined_sources.φ` Disposition der 6 Lebendquellen gesetzt.
- **Lage:** (gemessen 2026-09-27) kanonischer Träger ist DIESE Linie; der `§Offen`-Marker des Surveys `docs/surveys/survey-2026-09-26-secrets-inventar.md` ist offen.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict` je der 6 Quellen (GFW, GOSAT-GW, IGETS, Rubin, Babamul, Movebank); danach Disposition in `phi/sources.φ` bzw. `phi/declined_sources.φ`; dann `docs/surveys/survey-2026-09-26-secrets-inventar.md` §Offen schließen.

#### archive_search-Arme — Live-Befund (Rest: EPIPE-Fläche)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Live-Test 2026-09-27 (general-Lauf, jeder Arm des `--help` einmal gemessen).
- **Lage:** (gemessen 2026-09-27) 50+ Arme OK; in diesem Atom direkt gefixt: `--cc` `url=`-Form (`net.rs` `cc_target` + Test), `--ena`-Syntax-Hinweis (help), `--arxiv`-Botschaft (frische 406-Messung — auch ohne `max_results`; live ist `--arxiv-oai`); `--supermag` server-seitig (`external-state.md`-Zeile); `--librs` 403-Fallback greift; `--brave` 402 Quota. `sgrep`-EPIPE-Panik gefixt (`out!`-Makro).
- **Blockade:** keine.
- **Braucht:** die EPIPE-Sicherung der `archive_search`-println-Fläche (`--git | head` panikt, gemessen) — dasselbe `out!`-Muster über die archive_search-Bins; `cargo check -p omegaflow-utils` 0/0; Re-Test via CI.

#### Voyager 1/2 closed-loop Doppler — Anfrage läuft, Register-Note korrigiert
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort von `gsfc-dl-nssdca-request@mail.nasa.gov` (PSNO-00007).
- **Lage:** (gemessen 2026-09-27 via `state/mail/mail_ledger.φ:74`) die Anfrage wurde 2026-09-16 13:47 UTC gesandt (die frühere `(:207)`-Referenz war stale); `blocked_sources.φ:54`-Note auf `→ Antwort offen` korrigiert (Tag `[wartend]`).
- **Blockade:** keine.
- **Braucht:** Trigger abwarten (Cook/Antworten siehe `state/zustand/wartend.φ`).

### Termin

#### termin-Punkte — re-verdict
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27 via `archive_search --verdict`) `pithia.cbk.waw.pl/tap` 200; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine (Wiedervorlage).
- **Braucht:** `archive_search --verdict <url>`; bei Erholung `*-cdn.yml` dispatchen.

#### BepiColombo bc_mpo_more — Termin
- **Status:** termin | **Bindung:** termin:2027-04-01
- **Trigger:** 2027-04-01 (Science-Phase-Beginn).
- **Lage:** (gemessen 2026-09-26) MORE-Cruise nicht öffentlich, Freigabe April 2027; Ticket `YYM-342-97327`; `blocked_sources.φ:53` Freigabe-Anfrage an psahelp.
- **Blockade:** Freigabe (dritter).
- **Braucht:** Wiedervorlage 04/2027.

## Träger (Prosadokumente)

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | `mariner-occlt`-CDN-Dispatch geschlossen (gemessen); offen: native ODF-Serien-Arm + die vier request-only-Routen | nächster Schritt: `archive_search --verdict` beim Trigger.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | SAMPLE_CONTACT (MPI-FKF/LAB_A) sagte LAB_A-`I(q,t)`-Daten zu, danach kein Eingang | wartend auf Mail-Eingang (kein Nachfassen).
- `docs/surveys/survey-2026-09-26-secrets-inventar.md` | Namens-Disposition — Träger ist DIESE Linie (eigener Punkt oben) | nächster Schritt: `--verdict` je der 6 Quellen.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | nur `Wiedervorlage 2026-12-02` bindet; übrige Body-Marker sind Nachzug-Staleness | nächster Schritt: 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending weiter tot | nächster Schritt: `--verdict` je Host beim Trigger.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offen: Step 5 (CDN-kanonisch, destruktiv → Operator-Wort) | nächster Schritt: Klassen-Zensus messen.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Migrationsplan-Vorlage steht; stoppt am Operator-Wort | nächster Schritt: Operator-Wort zum Layout `knowledge/`+`backups/`.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen nur §7 Roh-Korpora/Scratch-Disposition | nächster Schritt: Operator-Wort.

## Abschluss

Commit-Wort (`/commit`) offen — Phase 1/2 lief unter `/consent`/„Du kannst" (Delegation).
