<!--
  title: Handover — Mycelium-Folge 323
  session: Mycelium-Linie in einem Pass (CI-Werkzeug-Fixes: block_commit-Release + bz-Probe-Log-Sichtbarkeit; Stehender Pass am neuen HEAD)
  class: handover
  date: 2026-10-12
  sha256: 882aeff9753ed05a2e6d93767431537427b2022763f06e9ec5265bf9bf1aaf5d
  status: live
-->
# Handover — Mycelium-Folge 323 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge322.md` (→ `archiv/`) und faltete die
adressierten Blöcke `future-folge241` und `mountain-folge324`.

## Burn: open 0.01 · close 0.06 — Grund: Mycelium-323 in einem Atom (deepseek-flash, kein pro/max; `session_burn`-Posten „Mycelium-Linie in einem Pass starten", gemessen 2026-10-12).

In diesem Atom gemessen/gebaut: `tools-build`-`TOOLS`-Liste um `block_commit`
erweitert (das Bin baute, wurde aber nicht publiziert — darum `pending`) ·
`bz-dbdt-delay-probe` auf `tee` umgestellt (Ergebnis lag nur im Artefakt, nicht
im Job-Log) · `intermagnet_dbdt`-Zelle aus `field-te-query` `38215224525`
(success) gelesen — Zeuge `abk_dbdt_1m` median_dt 60s n 527039, Riss gegen
`abk_dbdt_1h` (overlap corr 0.9388) · Spill-Dir-Fix verifiziert erledigt
(`03bb95e2e`).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „also höre zu beide linien mountain und river sind beschäftigt, unsere arbeitsweise muss sich ändern ihr dürft euch nicht mehr länger blockieren entweder arbeitet ihr ab jetzt projektweise oder ihr findet einen anderen weg" | 2026-10-12 | Operator (Session 323) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du bitte deinen und mountains teil bearbeiten mountain ist beschäftigt" | 2026-10-12 | Operator (Session 318) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du dich bitte um deinen teil kümmern? Klare Trennung — und ich habe meinen Teil gleich gebaut." | 2026-10-12 | Operator (Session 317) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass … kein Planungstheater … /commit (Push) … danach den frischen Pass" | 2026-10-12 | Operator (Session 324) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
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

### M10↔MDIS-Artefakte — `url`/`origin`/`sha256` registriert; Compiler/Workflow + Block-Identität offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain setzt die Block-Identität/`format` je Artefakt → Compiler/Workflow
- **Lage:** (gemessen 2026-10-12) PSJ-Supplementary t1–t4 (`phi:31416-31432`), USGS-Mosaik (`phi:31446-31447`, **1,88 GB** `1880562475`), LPSC-2024-PDF (`phi:31441-31442`) — alle `url`/`origin` stehen. Block-Identität (`witness`/`record`/`force`, `format`) je Artefakt fehlt (Mountain).
- **Blockade:** Block-Identität (Mountain).
- **Braucht:** Block-Identität (Mountain) → Compiler + CDN-Workflow (Mycelium).

### Fetch-Provenienz — Sidecar/`data/<netloc>/` fehlt (future-235)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Fetch-Weg (Wrapper/`archive_search`) schreibt den Quell-Sidecar
- **Lage:** (gemessen 2026-10-11) die Provenienz liegt in `opencode.db`; `state/research/README.md` §Provenienz belegt **76** Dateien nach `data/<netloc>/`; der Werkzeug-Fix (URL beim Fetch mitschreiben) fehlt. Der Spill-Ort ist geheilt (`OMEGAFLOW_SPILL_DIR`, `03bb95e2e`).
- **Blockade:** keiner im Design.
- **Braucht:** der Fetch-Weg schreibt `<datei>.src` (Quell-URL) oder legt gleich als `data/<netloc>/<datei>` ab — Werkzeug (Mycelium) + Quelle (Mountain).

### ENTSO-E A78 — Lauf rot; Body-Diagnose gebaut
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `entsoe-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-11) failure `curl status Some(22)`; geheilt (`-sSL` + `--write-out`).
- **Blockade:** ENTSO-E-Token (Ticket #9473923, `state/zustand/wartend.φ:28`).
- **Braucht:** `ci_manage log <run-id>`; bei grün Register-Block; `terms`/`ttl` (Mountain).

### MLA-Register · SPT-3G-D1 · GloFAS-EWDS · NIST-NCNR-NSE
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Lauf/Digest bzw. Mountain-`terms`/`field`
- **Lage:** (gemessen 2026-10-11/12) MLA repointet (`phi:12100`; NAME-Riss); SPT-3G `38150991206` success (sha offen); GloFAS Daten-Arm grün (`phi:11297`; grib2→φ offen); NIST NCNR NSE `38170359508` success (`phi:11711`). Die offene NSE-Route für das Kohärenz-Rätsel (`future-241`): `https://ncnr.nist.gov/pub/ncnrdata/nse/` (US-PD, HTTP 200) — noch keine `url`-Route.
- **Blockade:** Lauf-Ergebnisse/`terms`/`field` (Mountain).
- **Braucht:** `ci_manage log <id>`; sha256; NIST-NCNR-NSE-`url`-Route + `format` (Mountain identifiziert, Mycelium registriert); `terms`/`field` (Mountain).

### wy-max-t-Artefakt — Werte im Bestand
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Siegel auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`
- **Lage:** (gemessen 2026-10-12, River-196) α=0.05, B=9999, (1-α)-Quantil 2.4467/2.4433/2.4831 (Artefakt `38166787414`); die Intervall-Konstruktion ist benannt (studentisiertes Moving-Block-Bootstrap) — kein Lauf offen.
- **Blockade:** Siegel (Operator).
- **Braucht:** die gemessenen Werte ins Register/Artefakt (Mycelium); Siegel (Operator).

### MCP no-leak · de441-de442-sha · dropped-gate · SWPC/RTSW · CI-Optimierung · Domain→UCD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** MCP-Session-Restart · Merge-Lauf (`phi:4321-4369`) · Q2/Q3-Fold (`dropped_gate.rs`) · `matrix-vlies`-Lauf · `odf.rs`-clippy · messbare Domain-Quelle
- **Lage:** (gemessen 2026-10-11/12) MCP: Allowlist konsistent, Live-Seat-Restart offen; de441/de442: sha256 erst nach Merge-Lauf; dropped-gate: Match-Fix offen; SWPC/RTSW: `matrix-vlies`-Wiederholung (River); CI-Optimierung: `odf.rs`-clippy; Domain→UCD: keine messbare Quelle.
- **Blockade:** je der genannte Lauf/Quelle.
- **Braucht:** je der genannte Schritt (`ci_manage log <id>`; Merge-Lauf; `cargo clippy`; messbare Domain-Quelle).

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

## descoped

### bpa_gic — keine Historie (gemessene Null)
- **Status:** descoped | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-12) BPA publiziert kein historisches GIC-Archiv (nur 4-Tage-5-min-Fenster `gic.aspx`/`gic.txt`; alle Archiv-Kandidaten 404; `--all` 1029 Zeilen; Wayback 429/503). Die zweite GIC-Zeugenreihe bleibt leer; die Relation läuft über die primäre FMI-Reihe. `terms unknown`.

### matrix-rotor — descoped (gemessen)
- **Status:** descoped | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-12T08:05Z) rot `38212925885` = „The runner has received a shutdown signal … The operation was canceled" — transienter Runner-Shutdown; kein eigener Schritt (`Braucht: —`).

## An mountain

Origin: mycelium-323 (2026-10-12).

- **THEMIS-ASI-Keogramm (UCalgary)** — Block `phi:20916-20920` trägt `url`/`origin`/`compiler`/`format`/`ttl`; es fehlen `terms free-open` + `field` (Mountain-Recht). `terms free-open` ist gemessen (`data/how_to_cite.html`).
- **M10↔MDIS-Artefakte** — die `url`/`origin`/`sha256`-Zeilen stehen (`phi:31416-31447`). Offen bleibt die **Block-Identität** (`witness`/`record`/`force`, `format`) je Artefakt. USGS-Mosaik **1,88 GB** (`1880562475`); der 66-MB-Wert ist widerlegt.
- **NIST NCNR NSE** — offene NSE-Route für das Kohärenz-Rätsel: `https://ncnr.nist.gov/pub/ncnrdata/nse/` (US-PD, HTTP 200). Bitte Block-Identität/`format` setzen, dann registriert Mycelium die `url`-Route.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
