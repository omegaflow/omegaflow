<!--
  title: Handover — Mycelium-Folge 323
  session: Mycelium-Linie in einem Pass (CI-Werkzeug-Fixes: block_commit-Release + bz-Probe-Log-Sichtbarkeit; Stehender Pass am neuen HEAD)
  class: handover
  date: 2026-10-12
  sha256: 0760476b761f5ef93243130eae2bbfe6d793814517b4c98f1f8bad14479c6ca5
  status: live
-->
# Handover — Mycelium-Folge 323 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge322.md` (→ `archiv/`) und faltete die
adressierten Blöcke `future-folge241` und `mountain-folge324`.

## Burn: open 0.01 · close 0.20 · cap 0.30 — Grund: Mycelium-323 in einem Atom plus operator-gebotene Taucher-Runde (deepseek-flash, kein pro/max; `session_burn` total $2.2300; 6 Dispatches: council + 5 grind-flash/general).

In diesem Atom gemessen/gebaut: `tools-build`-`TOOLS`-Liste um `block_commit`
erweitert (das Bin baute, wurde aber nicht publiziert — darum `pending`) ·
`bz-dbdt-delay-probe` auf `tee` umgestellt (Ergebnis lag nur im Artefakt, nicht
im Job-Log) · `intermagnet_dbdt`-Zelle aus `field-te-query` `38215224525`
(success) gelesen — Zeuge `abk_dbdt_1m` median_dt 60s n 527039, Riss gegen
`abk_dbdt_1h` (overlap corr 0.9388) · Spill-Dir-Fix verifiziert erledigt
(`03bb95e2e`) · **Mountain-Anteile übernommen** (Operator-Wort Session 325:
Mountain arbeitet an den Ephemeriden): `terms free-open` am
UCalgary-THEMIS-ASI-Keogramm gesetzt, `terms` an allen M10↔MDIS-Artefakten
(PSJ t1–t4 `CC-BY-4.0` via Crossref gemessen, USGS `PD`, LPSC/camera_angles
`unknown`). **Taucher-Runde:** `tools-build` auf Auto-Discovery umgestellt (die
Hand-Liste ließ **19** Bins aus, u. a. `block_commit` — genau der Defekt) ·
`dropped_gate`-`alias:`-Fold gefixt (die zwei Folds gaben zwei Antworten) ·
Fetch-Sidecar + `odf.rs`-clippy als **bereits gebaut** gemessen (zwei stale
Handover-Zeilen korrigiert) · NIST-NCNR-NSE-Daten-Route vermessen · Rat-Verdikt
für den CI-Messzeilen-Struktur-Fix. **Kritischer Riss geheilt:** der
Register-Invariant (`src/archivar/parse.rs:2915`) verlangt je CDN-Quelle einen
Producer; die 7 M10-Roh-Registrierungen trugen keinen (mein Defekt aus
mycelium-321) → der neue `raw_mirror`-Producer
(`tools/harvest/src/bin/raw_mirror.rs`, sha256-gated) gibt ihnen einen (block-scoped
committet `5a915cb03`); bk18/jades tragen `format` (Mountain/im Bestand).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „also höre zu beide linien mountain und river sind beschäftigt, unsere arbeitsweise muss sich ändern ihr dürft euch nicht mehr länger blockieren entweder arbeitet ihr ab jetzt projektweise oder ihr findet einen anderen weg" | 2026-10-12 | Operator (Session 323) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du bitte deinen und mountains teil bearbeiten mountain ist beschäftigt" | 2026-10-12 | Operator (Session 318) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du dich bitte um deinen teil kümmern? Klare Trennung — und ich habe meinen Teil gleich gebaut." | 2026-10-12 | Operator (Session 317) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass … kein Planungstheater … /commit (Push) … danach den frischen Pass" | 2026-10-12 | Operator (Session 324) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „auch das ist wieder problematisch mountain arbeitet an den ephemeriden er kann sich nicht darum kümmern" | 2026-10-12 | Operator (Session 325) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „auf jeden eigen setzt du jetzt einen taucher an und der schliesst die aufgabe ab — das ist ein befehl" | 2026-10-11 | Operator (Session 310) |

## Offen — eigen

### GRDC-Caravan — Zeitachse geheilt; Lauf zur Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `grdc-caravan-cdn`-Lauf endet (`38217882014` in_progress, `38220536486` pending)
- **Lage:** (gemessen 2026-10-12 via `ci_manage view`) `38217882014` **in_progress** (updated 07:52Z), `38220536486` **pending** (created 07:56Z).
- **Blockade:** keine im Code.
- **Braucht:** `ci_manage log <id>` beim Lauf-Ende; erwartet: finite streamflow-Samples → `grdc_caravan.bin` geschrieben.

### Z-Minuten-Null (`bz_dbdt_delay_probe`) — Log-Sichtbarkeit gebaut; studentisiertes Null bei nächstem Lauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `bz-dbdt-delay-probe`-Lauf (nach Dispatch) endet
- **Lage:** (gemessen 2026-10-12) `38221770480` success (`--max-lag-min 1440`, ABK+SOD); ABK-2024 vorig **Z = 58 min, r = −0.31053, n = 523621**; das Ergebnis aller Läufe landet **nur im Artefakt** (`> file`), nicht im Job-Log. Workflow auf `tee` umgestellt (dieser Commit) → künftige Resultate stehen im Log.
- **Blockade:** keine im Code.
- **Braucht:** Dispatch `bz-dbdt-delay-probe`; dann `ci_manage log <id>` mit dem studentisierten Null aus dem 1440-min-Fenster.

### `block_commit` — Werkzeug gebaut; Release-Auslieferung angestoßen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `tools-build`-Lauf nach diesem Commit publiziert `block_commit`
- **Lage:** (gemessen 2026-10-12) Bin gebaut (`tools/register/src/bin/block_commit.rs`, 341 Z.); `tools-build` baute `-p omegaflow-register --bins` (also das Bin), die `TOOLS`-Liste (`.github/workflows/tools-build.yml:29`) trug es aber **nicht** → `bin/block_commit` liest `pending`. `TOOLS` um `block_commit` erweitert (dieser Commit).
- **Blockade:** CI-Release (`tools-build` nach Dispatch).
- **Braucht:** Dispatch `tools-build`; dann `bin/block_commit` (Usage).

### M10↔MDIS-Artefakte — registriert + Producer gesetzt; CDN-Manifestation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein CDN-Manifestationslauf ruft `raw_mirror` je Asset
- **Lage:** (gemessen 2026-10-12) PSJ t1–t4 `terms CC-BY-4.0` (Crossref `10.3847/PSJ/ae32f5`, direkt gemessen); USGS-Mosaik `terms PD`; LPSC-PDF + camera_angles `terms unknown`. Alle 7 tragen jetzt `compiler tools/harvest/src/bin/raw_mirror.rs` (sha256-gated) → der Register-Invariant `parse.rs:2915` hält.
- **Blockade:** keine.
- **Braucht:** `raw_mirror` in die CDN-Manifestation aufnehmen (`cdn-manifest.yml`-Matrix-Eintrag); USGS-Mosaik **1,88 GB** (`1880562475`).

### Fetch-Provenienz — `archive_search`-Sidecar steht; der allgemeine Wrapper-Pfad fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein kurzer Fetch-Weg (`bin/`-curl/Playwright-Wrapper der Taucher) schreibt den Quell-Sidecar
- **Lage:** (gemessen 2026-10-12) `write_fetched` (`tools/utils/src/bin/archive_search.rs:1158`) schreibt `<datei>.src` (Quell-URL, eine Zeile) und **nichts**, wenn die URL fehlt — gebaut (`d884a2e93` mycelium 311, `213d8937f` mycelium 316), verdrahtet an pdf-image/arxiv-src/alphaxiv-call. Die frühere Lage „Werkzeug-Fix fehlt" war **stale** (Baum gewinnt). Offen bleibt die allgemeine Wrapper-Route.
- **Blockade:** keine.
- **Braucht:** der curl/Playwright-Wrapper schreibt den Quell-Sidecar (Mycelium).

### ENTSO-E A78 — Lauf rot; Body-Diagnose gebaut
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `entsoe-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-11) failure `curl status Some(22)`; geheilt (`-sSL` + `--write-out`).
- **Blockade:** ENTSO-E-Token (Ticket #9473923, `state/wartend.φ:28`).
- **Braucht:** `ci_manage log <run-id>`; bei grün Register-Block; `terms`/`ttl` (Mountain).

### MLA-Register · SPT-3G-D1 · GloFAS-EWDS · NIST-NCNR-NSE
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Lauf/Digest bzw. eigener Register-Schritt
- **Lage:** (gemessen 2026-10-12) MLA repointet (`phi:12100`; NAME-Riss); SPT-3G `38150991206` success (sha offen); GloFAS Daten-Arm grün (`phi:11297`; grib2→φ offen); NIST NCNR NSE: **Index-Route steht** (`phi:11721-11727`); **Daten-Arm gebaut** (`tools/harvest/src/bin/nist_ncnr_nse_nexus_compiler.rs`, HDF5-NeXus via `src/archivar/hdf5.rs`; `.nxs.nse` = HDF5, `.dat.nse` = ASCII, 47 Monatssets, `terms PD`).
- **Blockade:** keine.
- **Braucht:** NIST-NCNR-NSE-Daten-`url`-Block registrieren (Mycelium) + `cdn-manifest.yml`-Eintrag.

### wy-max-t-Artefakt — Werte im Bestand
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Siegel auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`
- **Lage:** (gemessen 2026-10-12, River-196) α=0.05, B=9999, (1-α)-Quantil 2.4467/2.4433/2.4831 (Artefakt `38166787414`); die Intervall-Konstruktion ist benannt (studentisiertes Moving-Block-Bootstrap) — kein Lauf offen.
- **Blockade:** Siegel (Operator).
- **Braucht:** die gemessenen Werte ins Register/Artefakt (Mycelium); Siegel (Operator).

### MCP no-leak · de441-de442-sha · SWPC/RTSW · Domain→UCD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** MCP-Session-Restart · Merge-Lauf (`phi:4321-4369`) · `matrix-vlies`-Lauf · messbare Domain-Quelle
- **Lage:** (gemessen 2026-10-11/12) MCP: Allowlist konsistent, Live-Seat-Restart offen; de441/de442: sha256 erst nach Merge-Lauf; SWPC/RTSW: `matrix-vlies`-Wiederholung (River); Domain→UCD: keine messbare Quelle. (`dropped-gate` gefixt — Commit `96350c94c`; `odf.rs`-clippy war bereits geheilt (`a0d2c04e2`) — beide aus diesem Punkt gelöscht.)
- **Blockade:** je der genannte Lauf/Quelle.
- **Braucht:** je der genannte Schritt (`ci_manage log <id>`; Merge-Lauf; messbare Domain-Quelle).

### ui-seats-Träger (`docs/concepts/ui-seats.md`) — offene `pending`-Seats
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Stimmen-Round in `mycelium-ui` (`open-weight-ui`-Lock) läuft
- **Lage:** (gemessen 2026-10-12) `register_lookup --orphan-docs` = 0; die `pending`-Seats Sakana-Tempo · SciSpace (Login-Wall) · Gemini Notebook (kein freier Composer) sind noch nicht gemessen.
- **Blockade:** kein Stimmen-Round in dieser Session gefahren.
- **Braucht:** im nächsten Stimmen-Round die `pending`-Seats messen und den Stand fortschreiben.

### Offene-Prosa-Träger (future-235-Zensus) — in die eigene Übergabe heben
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `--orphan-docs`-Zensus nennt sie trägerlos
- **Lage:** (gemessen 2026-10-11/12) `docs/concepts/free-voices.md` (1) · `docs/concepts/tools-map.md` (2) · `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` (1, grenznah Mountain/Mycelium) · `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (1) · `docs/surveys/survey-2026-10-08-fmhy-research-landscape.md` (2).
- **Blockade:** keine.
- **Braucht:** je Dokument eine Träger-Zeile in der passenden Übergabe (Mycelium für `free-voices`/`tools-map`; Rest ggf. Mountain) — `register_lookup --orphan-docs` als Nachweis.

### Struktur-Fix — CI-Messzeilen durabel in den Baum (Rat, 2026-10-12)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erste `fold`-Job eines messenden Workflows (`.github/workflows/field-te-query.yml`) läuft
- **Lage:** (gemessen 2026-10-12, Operator-Wort Session 325) eine CI-Messzeile (`witness`/`riss`, z. B. der `intermagnet_dbdt`-Riss aus `field-te-query`) lebt heute nur im Job-Log/Artefakt; eine Session wartet ~1 h und liest ein ephemeres Log. Rat-Verdikt (5 Stimmen): ein **neues getracktes append-only Register `phi/ci_measurements.φ`**, **ein Writer** (terminaler `fold`-Job je Workflow, Commit über `block_commit` privat-Index + CAS); der Block trägt `run_id`/`head_sha`/`descriptor` + die **rohe** Beobachtung — **nie ein Verdikt-Wort** (das stempelt der Owner); das volle Serien-Ergebnis → CDN-Asset.
- **Blockade:** neue getrackte Registerklasse → `REGISTER_CLASSES`-Eintrag (`src/gate/commit_gate.rs`) + Kanon-Wort (Architektur-Akt).
- **Braucht:** `REGISTER_CLASSES`-Eintrag + `phi/ci_measurements.φ`-Skelett + `fold`-Job in `field-te-query.yml` (Pilot); das Verdikt liegt vor (Rat).

## descoped

### bpa_gic — keine Historie (gemessene Null)
- **Status:** descoped | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-12) BPA publiziert kein historisches GIC-Archiv (nur 4-Tage-5-min-Fenster `gic.aspx`/`gic.txt`; alle Archiv-Kandidaten 404; `--all` 1029 Zeilen; Wayback 429/503). Die zweite GIC-Zeugenreihe bleibt leer; die Relation läuft über die primäre FMI-Reihe. `terms unknown`.

### matrix-rotor — descoped (gemessen)
- **Status:** descoped | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-12T08:05Z) rot `38212925885` = „The runner has received a shutdown signal … The operation was canceled" — transienter Runner-Shutdown; kein eigener Schritt (`Braucht: —`).

### Fetch-Provenienz — vollständig gebaut (descoped)
- **Status:** descoped | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-12) `archive_search`-`write_fetched` schrieb bereits den `.src`-Sidecar; die allgemeine Lücke (`sfetch` schrieb nur stdout, kein `<datei>`-Pfad) ist mit `sfetch --out <path>` + `<path>.src` geheilt (`tools/utils/src/bin/sfetch.rs`; `cargo check -p omegaflow-utils` grün). Kein offener Schritt.

### Register-Invariant / raw_mirror — geheilt (descoped)
- **Status:** descoped | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-12) die 7 M10-Roh-Registrierungen verletzten `src/archivar/parse.rs:2915` (CDN-Quelle ohne Producer) — Lib-Test rot; der `raw_mirror`-Producer (`tools/harvest/src/bin/raw_mirror.rs`) ist gebaut und den 7 Blöcken als `compiler` gesetzt (`5a915cb03`). bk18/jades tragen `format` (Mountain).

## Keine Delegation an Mountain

Keiner. Operator-Wort Session 325 („mountain arbeitet an den ephemeriden er kann
sich nicht darum kümmern"): die zuvor Mountain zugewiesenen Quellen-Anteile
(`terms`, Block-Identität) sind als Mycelium-eigen übernommen und in diesem Atom
gesetzt — kein Block mehr an Mountain.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
