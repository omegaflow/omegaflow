<!--
  title: Handover — Mycelium-Folge 322
  session: Mycelium-Linie in einem Pass (Stehender Pass am neuen HEAD, mountain-324 gefaltet, UCalgary-Keogramm-Block registriert)
  class: handover
  date: 2026-10-12
  sha256: 4fa23e3c33237714a54e9352f78bd2b648062e5c54c9c484fb4b09cc35ea8119
  status: live
-->
# Handover — Mycelium-Folge 322 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge321.md` (→ `archiv/`) und faltete den
adressierten Block `mountain-324`.

## Burn: open 0.01 · close 0.06 — Grund: Mycelium-322 in einem Atom (deepseek-flash, kein pro/max; `session_burn`-Posten „Mycelium-Linie in einem Pass starten" $0.0581, gemessen 2026-10-12T08:26Z).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „also höre zu beide linien mountain und river sind beschäftigt, unsere arbeitsweise muss sich ändern ihr dürft euch nicht mehr länger blockieren entweder arbeitet ihr ab jetzt projektweise oder ihr findet einen anderen weg" | 2026-10-12 | Operator (Session 323) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du bitte deinen und mountains teil bearbeiten mountain ist beschäftigt" | 2026-10-12 | Operator (Session 318) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du dich bitte um deinen teil kümmern? Klare Trennung — und ich habe meinen Teil gleich gebaut." | 2026-10-12 | Operator (Session 317) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass … kein Planungstheater … /commit (Push) … danach den frischen Pass" | 2026-10-11 | Operator (Session 315/316) · `state/operator-gespraeche/2026-10-11-mycelium.md` |
| „auf jeden eigen setzt du jetzt einen taucher an und der schliesst die aufgabe ab — das ist ein befehl" | 2026-10-11 | Operator (Session 310) |

## Offen — eigen

### GRDC-Caravan — Zeitachse geheilt; Lauf zur Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `grdc-caravan-cdn`-Lauf endet (`38217882014` in_progress, `38220536486` pending)
- **Lage:** (gemessen 2026-10-12T08:25Z via `ci_manage`) `38217882014` **in_progress** seit 07:27Z (updated 07:52Z), `38220536486` **pending**.
- **Blockade:** keine im Code.
- **Braucht:** `ci_manage log <id>` beim Lauf-Ende; erwartet: finite streamflow-Samples → `grdc_caravan.bin` geschrieben.

### `intermagnet_dbdt`-Resolver — gebaut; Zelle beim Lauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `field-te-query`-Lauf `38215224525` endet
- **Lage:** (gemessen 2026-10-12T08:25Z) Resolver gebaut (`tools/measure/src/bin/field_te_query.rs`, commit `3467ec4f9`); Lauf `38215224525` **queued** seit 06:58Z (updated 08:19Z) — noch nicht beendet.
- **Blockade:** keine im Code.
- **Braucht:** `ci_manage log 38215224525`; die Zelle neu lesen (ABK `abk_1m` Zeuge, `abk_1h` getragener Riss).

### Z-Minuten-Null (`bz_dbdt_delay_probe`) — Lag-Maximum gemessen; studentisiertes Null pending
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erweiterte `bz-dbdt-delay-probe`-Lauf (`--max-lag-min 1440`) endet
- **Lage:** (gemessen 2026-10-12T08:25Z) `38221770480` **in_progress** seit 08:17Z; voriger `38215420000` success, ABK-2024 **Z = 58 min, r = −0.31053, n = 523621**; Signifikanz pending — BZ-Autokorrelation kehrt in 180 min nirgends nicht-positiv.
- **Blockade:** keine im Code.
- **Braucht:** `ci_manage log 38221770480`; studentisiertes Null aus dem 1440-min-Fenster.

### M10↔MDIS-Artefakte — `url`/`origin`/`sha256` registriert; Compiler/Workflow + Block-Identität offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain setzt die Block-Identität/`format` je Artefakt → Compiler/Workflow
- **Lage:** (gemessen 2026-10-12) die 7 `url`/`origin`/`sha256`-Blöcke stehen in `phi/sources.φ` (nach `mariner_rst`); USGS-Mosaik **1,88 GB** (`Content-Range 1880562475`), kein sha (außerhalb des Sniff-Horizonts).
- **Blockade:** Block-Identität (`witness`/`record`/`force`, `format`) je Artefakt (Mountain).
- **Braucht:** Block-Identität (Mountain) → Compiler + CDN-Workflow (Mycelium).

### Fetch-Provenienz — Sidecar/`data/<netloc>/` fehlt (future-235)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Fetch-Weg (Wrapper/`archive_search`) schreibt den Quell-Sidecar
- **Lage:** (gemessen 2026-10-11) die Provenienz liegt in `opencode.db`; `state/research/README.md` §Provenienz belegt **76** Dateien nach `data/<netloc>/`; der Werkzeug-Fix (URL beim Fetch mitschreiben) fehlt.
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
- **Lage:** (gemessen 2026-10-11/12) MLA repointet (`phi:12100`; NAME-Riss); SPT-3G `38150991206` success (sha offen); GloFAS Daten-Arm grün (`phi:11297`; grib2→φ offen); NIST NCNR NSE `38170359508` success (`phi:11711`).
- **Blockade:** Lauf-Ergebnisse/`terms`/`field` (Mountain).
- **Braucht:** `ci_manage log <id>`; sha256; `terms`/`field` (Mountain).

### wy-max-t-Artefakt — Werte im Bestand
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Siegel auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`
- **Lage:** (gemessen 2026-10-12, River-196) α=0.05, B=9999, (1-α)-Quantil 2.4467/2.4433/2.4831 (Artefakt `38166787414`); die Intervall-Konstruktion ist benannt (studentisiertes Moving-Block-Bootstrap, kein „Block-BCa") — kein Lauf offen.
- **Blockade:** Siegel (Operator); die gemessenen Werte ins Register/Artefakt (Mycelium).
- **Braucht:** die gemessenen Werte ins Register/Artefakt (Mycelium); Siegel (Operator).

### MCP no-leak · de441-de442-sha · dropped-gate · SWPC/RTSW · CI-Optimierung · Domain→UCD
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** MCP-Session-Restart · Merge-Lauf (`phi:4321-4369`) · Q2/Q3-Fold (`dropped_gate.rs`) · `matrix-vlies`-Lauf · `odf.rs`-clippy · messbare Domain-Quelle
- **Lage:** (gemessen 2026-10-11/12) MCP: Allowlist konsistent, Live-Seat-Restart offen; de441/de442: sha256 erst nach Merge-Lauf; dropped-gate: Match-Fix offen; SWPC/RTSW: `matrix-vlies`-Wiederholung (River); CI-Optimierung: `odf.rs`-clippy; Domain→UCD: keine messbare Quelle.
- **Blockade:** je der genannte Lauf/Quelle (MCP-Neustart, Merge-Lauf, `matrix-vlies`, `odf.rs`-clippy, Domain-Quelle).
- **Braucht:** je der genannte Schritt (`ci_manage log <id>`; Merge-Lauf; `cargo clippy`; messbare Domain-Quelle).

### ui-seats-Träger (`docs/concepts/ui-seats.md`) — offene `pending`-Seats
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Stimmen-Round in `mycelium-ui` (`open-weight-ui`-Lock) läuft
- **Lage:** (gemessen 2026-10-12) `register_lookup --orphan-docs` = 0; die `pending`-Seats Sakana-Tempo · SciSpace (Login-Wall) · Gemini Notebook (kein freier Composer) sind noch nicht gemessen.
- **Blockade:** kein Stimmen-Round in dieser Session gefahren.
- **Braucht:** im nächsten Stimmen-Round die `pending`-Seats messen (Sakana-Tempo, SciSpace/Gemini-Notebook erreichbar?) und den Stand fortschreiben.

### `block_commit` — Werkzeug gebaut; Release-Auslieferung pending
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `tools-build`-Lauf publiziert `block_commit` in `tools-latest`
- **Lage:** (gemessen 2026-10-12) `tools/register/src/bin/block_commit.rs` gebaut (341 Z., `cargo check -p omegaflow-register` warnungsfrei); der Wrapper `bin/block_commit` steht; bis zur Release-Auslieferung liest der Wrapper `pending`.
- **Blockade:** CI-Release (`tools-build`).
- **Braucht:** `tools-build`-Lauf nach diesem Commit; dann `bin/block_commit` (Usage).

## descoped

### bpa_gic — keine Historie (gemessene Null)
- **Status:** descoped | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-12) BPA publiziert kein historisches GIC-Archiv (nur 4-Tage-5-min-Fenster `gic.aspx`/`gic.txt`; alle Archiv-Kandidaten 404; `--all` 1029 Zeilen; Wayback 429/503). Die zweite GIC-Zeugenreihe bleibt leer; die Relation läuft über die primäre FMI-Reihe. `terms unknown`.

### matrix-rotor — descoped (gemessen)
- **Status:** descoped | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-12T08:05Z) rot `38212925885` = „The runner has received a shutdown signal … The operation was canceled" — transienter Runner-Shutdown; kein eigener Schritt (`Braucht: —`).

## An mountain

Origin: mycelium-322 (2026-10-12).

- **M10↔MDIS-Artefakte** (`phi/sources.φ`, nach `mariner_rst`): die
  `url`/`origin`/`sha256`-Zeilen stehen (Mycelium-Recht). Offen bleibt die
  **Block-Identität** (`witness`/`record`/`force`, `format`) je Artefakt — bitte
  setzen, dann baut Mycelium Compiler + Workflow. USGS-Mosaik trägt die frische
  Zahl **1,88 GB** (`Content-Range` `1880562475`); der 66-MB-Wert ist widerlegt.
- **THEMIS-ASI-Keogramm (UCalgary):** der Block steht (nach dem
  Berkeley-`themis_asi`-Block) — `url` (CDN-JPG), `origin`
  (`…/stream2/2026/10/09/atha_themis02/20261009__atha_themis02_full-keogram.pgm.jpg`,
  HTTP 206 via `--verdict`), `compiler themis_asi_keogram_compiler.rs`,
  `format themis_asi_keogram`, `ttl 604800`. Bitte **`terms free-open` + `field`**
  setzen; `terms free-open` ist gemessen (`data/how_to_cite.html`). Die
  per-Bild-`sha256`/`station`/`on earth`-Zeilen kommen aus dem
  `emit` des Compilers beim Manifestationslauf.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
