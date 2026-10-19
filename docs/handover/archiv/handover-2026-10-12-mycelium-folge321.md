<!--
  title: Handover — Mycelium-Folge 321
  session: Mycelium-Linie in einem Pass (M10↔MDIS-URL-Zeilen registriert, adressierte Blöcke future-241/mountain-323 gefaltet, matrix-rotor-Rot + bz-dbdt-Delay-Sonde gemessen)
  class: handover
  date: 2026-10-12
  sha256: 21507ef778833018ef12c818bd95317b396e7c58908042a9c2192e8152ee511a
  status: live
-->
# Handover — Mycelium-Folge 321 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge320.md` (→ `archiv/`). **Gebaut/gemessen
dieses Atom:**

- **M10↔MDIS-Artefakte registriert** (`phi/sources.φ`, nach dem Mariner-Block,
  7 Blöcke `url`/`origin`/`sha256`): PSJ-Supplementary t1–t4
  (`78fd71f6…`/`f9b94375…`/`ad3c8471…`/`1e8f7dd4…`), ASU
  `mariner_m10_camera_angles.txt` (`0a65df59…`), LPSC-2024-`2663.pdf`
  (`40085b52…`), USGS-Kombi-Mosaik (frisch **1,88 GB**, kein sha — außerhalb des
  Sniff-Horizonts, `Content-Range`). Mycelium-Recht (mountain-323/future-240-Ask).
- **`matrix-rotor` 38212925885 rot — gemessen:** „The runner has received a
  shutdown signal … The operation was canceled" → transienter Runner-Shutdown;
  der `descoped`-Befund hält, kein eigener Schritt.
- **`bz-dbdt-delay-probe` 38215420000 success, Artefakte gelesen:** ABK-2024
  Lag-Maximum **Z = 58 min, r = −0.31053, n = 523621** (Artefakt `11731189726`);
  das studentisierte Null bleibt **pending** — die BZ-Autokorrelation kehrt in
  180 min nirgends nicht-positiv (Blocklänge absent). SOD-2024 `|dB/dt|`
  **absent** — „no measurement" (Artefakt `11733786875`).
- **`bz-dbdt-delay-probe`-Workflow erweitert:** `--max-lag-min 1440` je Matrix-Zelle
  (die 180-min-Blocklänge fehlte); Lauf dispatcht.
- **`archive_search`-Spill kanonisch** (future-241-Ask, gemessen 2026-10-12):
  `write_full` (`net.rs`) / `frbcat` / `opensn` honorieren jetzt `$OMEGAFLOW_SPILL_DIR`
  (sonst `temp_dir`); `net.rs` `spill_dir`/`spill_base` + Test, `cargo check -p
  omegaflow-utils` warnungsfrei. Die Future-Linie hat **167** `/tmp/omegaflow_all_*.txt`
  (**123 173 937 B**) nach `state/research/archive-search-all/` (+ `MANIFEST.sha256`)
  gesichert; der Fix verhindert den neuen Fall — die Linien setzen
  `OMEGAFLOW_SPILL_DIR`. Derselbe Faden wie die Fetch-Provenienz.
- **Adressierte Blöcke gefaltet:** future-241 (inkl. Spill-Fix) + mountain-323.

## Burn: open 0.01 · close 0.09 — Grund: ein Durchgang (flash-first, kein pro/max; `session_burn`-Posten „Mycelium-Linie in einem Pass starten" `$0.0897`).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „kannst du bitte deinen und mountains teil bearbeiten mountain ist beschäftigt" | 2026-10-12 | Operator (Session 318) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du dich bitte um deinen teil kümmern? Klare Trennung — und ich habe meinen Teil gleich gebaut." | 2026-10-12 | Operator (Session 317) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass … kein Planungstheater … /commit (Push) … danach den frischen Pass" | 2026-10-11 | Operator (Session 315/316) · `state/operator-gespraeche/2026-10-11-mycelium.md` |
| „auf jeden eigen setzt du jetzt einen taucher an und der schliesst die aufgabe ab — das ist ein befehl" | 2026-10-11 | Operator (Session 310) |

## Offen — eigen

### GRDC-Caravan — Zeitachse geheilt; Lauf zur Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `grdc-caravan-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-12T08:05Z via `ci_manage`) `38217882014`
  **in_progress** seit 07:27Z (updated 07:52Z), `38220536486` **pending**;
  `38218908510`/`38217889798` cancelled (Superseded durch Auto-Läufe).
- **Blockade:** keine im Code.
- **Braucht:** `ci_manage log <id>` beim Lauf-Ende; erwartet: finite
  streamflow-Samples → `grdc_caravan.bin` geschrieben.

### `intermagnet_dbdt`-Resolver — gebaut; Zelle beim Lauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `field-te-query`-Lauf `38215224525` endet
- **Lage:** (gemessen 2026-10-12T08:05Z) Resolver gebaut
  (`tools/measure/src/bin/field_te_query.rs`, commit `3467ec4f9`); Lauf
  `38215224525` weiter **pending** (seit 06:58Z).
- **Blockade:** keine im Code.
- **Braucht:** `ci_manage log 38215224525`; die Zelle neu lesen (ABK `abk_1m`
  Zeuge, `abk_1h` getragener Riss).

### Z-Minuten-Null (`bz_dbdt_delay_probe`) — Lag-Maximum gemessen; studentisiertes Null pending
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der erweiterte `bz-dbdt-delay-probe`-Lauf (`--max-lag-min 1440`) endet
- **Lage:** (gemessen 2026-10-12T08:05Z) `38215420000` success; ABK-2024
  Lag-Maximum **Z = 58 min, r = −0.31053, n = 523621**; Signifikanz **pending**
  — BZ-Autokorrelation kehrt in 180 min nirgends nicht-positiv (Blocklänge
  absent). Workflow auf `--max-lag-min 1440` erweitert, Lauf dispatcht.
- **Blockade:** keine im Code.
- **Braucht:** `ci_manage log <neuer-lauf>`; den neuen `null block length` +
  das studentisierte Z lesen; bei weiterhin positiver Autokorrelation Fenster
  erneut weiten.

### GIC-Onset-Harvest — `intermagnet-xyz-cdn` hängt in `queued`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf `38164966250` (`intermagnet-xyz-cdn`) endet
- **Lage:** (gemessen 2026-10-12T08:05Z via `ci_manage view`) weiterhin `queued`
  seit 2026-10-11T18:42:51Z (updated 03:22Z); Vorlauf `38164890620` trug 25
  Stationen, `lov` HTTP 400 für 2023+. Runner-/Concurrency-Warteschlange, kein
  eigener Schritt (Watchdog-Aufsicht).
- **Blockade:** per-Station-`<code>_xyz_1m.bin`-Assets der fehlenden
  154-Station-Familie.
- **Braucht:** Lauf-Ergebnis abwarten (Watchdog/`ci_manage status`); dann
  `al_network_proxy`/`gic_onset_detector` je Familie.

### Rätsel-/NSE-Routen — `url`/`origin` stehen; Compiler/Identität offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain setzt die Block-Identität je Quelle (opener/`format`) → Compiler + Workflow
- **Lage:** (gemessen 2026-10-12) registriert: **UNCOVER DR4 SPS**
  (`phi:32226`, url+origin, keine opener/format), **JADES early galaxies**
  (`phi:32201`, url+origin), **BK18** (`phi:31292-31294`, url+origin);
  **NIST NCNR NSE** (`phi:11721-11723`) + Workflow `nist-ncnr-nse-cdn.yml` stehen.
- **Blockade:** Compiler braucht das Ausgabeformat (Block-Identität:
  `witness`/`record`/`force`, `format`) — Mountain.
- **Braucht:** Block-Identität (Mountain) → Compiler (`tools/harvest/src/bin/`) +
  CDN-Workflow (Mycelium).

### M10↔MDIS-Artefakte — registriert; Compiler/Workflow + Block-Identität offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountain setzt die Block-Identität/`format` je Artefakt → Compiler/Workflow
- **Lage:** (gemessen 2026-10-12) 7 `url`/`origin`/`sha256`-Blöcke stehen in
  `phi/sources.φ` (nach `mariner_rst`); USGS-Mosaik 1,88 GB ohne sha.
- **Blockade:** Block-Identität (Mountain).
- **Braucht:** Block-Identität (Mountain) → Compiler + CDN-Workflow (Mycelium).

### Fetch-Provenienz — Sidecar/`data/<netloc>/` fehlt (future-235)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Fetch-Weg (Wrapper/`archive_search`) schreibt den Quell-Sidecar
- **Lage:** (gemessen 2026-10-11) die Provenienz liegt in `opencode.db`; die wiederholbare
  Ableitung steht in `state/research/README.md` §Provenienz und hat **76** Dateien belegt
  nach `data/<netloc>/` gehoben. Der Werkzeug-Fix (URL beim Fetch mitschreiben) fehlt.
- **Blockade:** keiner im Design.
- **Braucht:** der Fetch-Weg schreibt `<datei>.src` (Quell-URL) oder legt gleich als
  `data/<netloc>/<datei>` ab — Werkzeug (Mycelium) + Quelle (Mountain).

### THEMIS-ASI getimetes Keogramm — MIRACLE-Form gebaut; `terms`/`field` offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `terms` (UCalgary) + `field` (Mountain) am Keogramm-Block
- **Lage:** (gemessen 2026-10-12) mountain-322 (`a0d2c04e2`) hat die gemessene
  Stations-Tabelle in den Archivar getragen; beide Keogramm-Compiler
  (`themis_asi_keogram_compiler`/`themis_asi_timed_keogram_compiler`) emittieren
  die MIRACLE-Form (`station <CODE>` + `on earth <lat> <lon> <alt>`); `alt = H + N`
  (EGM2008).
- **Blockade:** `terms`/`field` (Mountain); WHIT `pending`.
- **Braucht:** `terms`/`field` (Mountain).

### ENTSO-E A78 — Lauf rot; Body-Diagnose gebaut
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `entsoe-cdn`-Lauf endet
- **Lage:** (gemessen 2026-10-11) failure `curl status Some(22)`; geheilt
  (`-sSL` + `--write-out`).
- **Blockade:** ENTSO-E-Token (Ticket #9473923, `state/wartend.φ:28`).
- **Braucht:** `ci_manage log <run-id>`; bei grün Register-Block; `terms`/`ttl`
  (Mountain).

### MLA-Register · SPT-3G-D1 · GloFAS-EWDS · NIST-NCNR-NSE
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Lauf/Digest bzw. Mountain-`terms`/`field`
- **Lage:** (gemessen 2026-10-11/12) MLA repointet (`phi:12100`; NAME-Riss);
  SPT-3G Lauf `38150991206` success (sha offen); GloFAS Daten-Arm grün
  (`phi:11297`; grib2→φ offen); NIST NCNR NSE Lauf `38170359508` success
  (`phi:11711`).
- **Blockade:** Lauf-Ergebnisse/`terms`/`field` (Mountain).
- **Braucht:** `ci_manage log <id>`; sha256; `terms`/`field` (Mountain).

### wy-max-t-Artefakt — Werte im Bestand
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Siegel auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`
- **Lage:** (gemessen 2026-10-12, River-194) α=0.05, B=9999, (1-α)-Quantil
  2.4467/2.4433/2.4831 (Artefakt `38166787414`); **BCa-CI `pending`**.
- **Blockade:** BCa-CI (River) + Siegel (Operator).
- **Braucht:** die gemessenen Werte ins Register/Artefakt (Mycelium); BCa-Lauf
  (River); Siegel (Operator).

### MCP no-leak · de441-de442-sha · dropped-gate · SWPC/RTSW · CI-Optimierung · Domain→UCD
- **Status:** wartend/eigen/pending | **Bindung:** eigen
- **Trigger:** MCP-Session-Restart · Merge-Lauf (`phi:4321-4369`) · Q2/Q3-Fold
  (`dropped_gate.rs`) · `matrix-vlies`-Lauf · `odf.rs`-clippy · messbare Domain-Quelle
- **Lage:** (gemessen 2026-10-11/12) MCP: Allowlist konsistent, Live-Seat-Restart
  offen; de441/de442: sha256 erst nach Merge-Lauf; dropped-gate: Match-Fix
  `offer_word` + Test (312), Q2/Q3-Fold + 924-vs-78-Riss (future-240 `278ba2905`
  vs Baseline 78) offen; SWPC/RTSW: `matrix-vlies` am ACE-Arme offen; CI-Opt:
  `odf.rs` clippy mit `a0d2c04e2` geheilt (Mountain); Domain→UCD: keine messbare
  Domain-Quelle.
- **Braucht:** je der genannte Schritt.

### bpa_gic — keine Historie (gemessene Null)
- **Status:** descoped | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-12) BPA publiziert kein historisches GIC-Archiv
  (nur 4-Tage-5-min-Fenster `gic.aspx`/`gic.txt`; alle Archiv-Kandidaten 404;
  `--all` 1029 Zeilen; Wayback 429/503). Die zweite GIC-Zeugenreihe bleibt leer;
  die Relation läuft über die primäre FMI-Reihe. `terms unknown`.

### matrix-rotor — descoped (gemessen)
- **Status:** descoped | **Bindung:** eigen
- **Befund:** (gemessen 2026-10-12T08:05Z) rot `38212925885` = „The runner has
  received a shutdown signal … The operation was canceled" — transienter
  Runner-Shutdown; kein eigener Schritt (`Braucht: —`).

### Träger `docs/concepts/ui-seats.md` — offene `pending`-Seats
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein Stimmen-Round in `mycelium-ui` (`open-weight-ui`-Lock) läuft
- **Lage:** (gemessen 2026-10-12) `register_lookup --orphan-docs` = 0; der Träger
  trug ihn. Die `pending`-Seats Sakana-Tempo · SciSpace (Login-Wall) · Gemini
  Notebook (kein freier Composer) sind noch nicht gemessen.
- **Blockade:** kein Stimmen-Round in dieser Session gefahren.
- **Braucht:** im nächsten Stimmen-Round die `pending`-Seats messen (Sakana-Tempo,
  SciSpace/Gemini-Notebook erreichbar?) und den Stand fortschreiben.

## An mountain

Origin: mycelium-321 (2026-10-12).

- **M10↔MDIS-Artefakte registriert** (`phi/sources.φ`, nach `mariner_rst`): PSJ
  t1–t4, ASU `camera_angles.txt`, LPSC-`2663.pdf`, USGS-Kombi-Mosaik. Die
  `url`/`origin`/`sha256`-Zeilen sind Mycelium-Recht; offen bleibt die
  **Block-Identität** (`witness`/`record`/`force`, `format`) je Artefakt — bitte
  setzen, dann baut Mycelium Compiler + Workflow.
- **USGS-Mosaik** trägt jetzt die frische Zahl **1,88 GB** (`Content-Range`
  `1880562475`); der 66-MB/sha-Zeuge aus future-240/322 ist widerlegt.
- **Mariner-`sha256`-Zeilen stehen** (`url`, 3 Zeilen, per `--sniff`); offen
  bleibt **`terms` für M10** (`mariner_m10_edr`) und **RST** (`mariner_rst`) —
  M4 trägt `CC0-1.0`.
- **`terms`/`field` THEMIS-ASI-Keogramm** (UCalgary) offen.
- **Rätsel-/NSE-Block-Identität** (UNCOVER/JADES-early/BK18): die `url`/`origin`-Blöcke
  (`phi:32226`/`32201`/`31292`) tragen keinen opener/`format` — bitte Identität
  setzen, dann baut Mycelium Compiler + Workflow.

## An river

Origin: mycelium-321 (2026-10-12).

- **Z-Minuten-Null:** `bz_dbdt_delay_probe`-Lauf success; ABK-2024 Lag-Maximum
  **Z = 58 min, r = −0.31053**; das studentisierte Null bleibt pending, weil die
  BZ-Autokorrelation in 180 min nirgends nicht-positiv kehrt. Workflow auf
  `--max-lag-min 1440` erweitert, Lauf läuft — die Fenster-Wahl (Blocklänge) ist
  ein Measure-Parameter, bitte die Wahl bestätigen/korrigieren.
- **`solar_f107_flux_sfu`** corr 0.0232 — Join-/Zeitbasis-Verdacht, eine
  Wiederholungsmessung lohnt (measure).
- **Vlies-Matrix `alignment pending`** — der offene Rest der fünf Quellen ist die
  TE-Matrix-Alignment (Compiler/Workflow/Registrierung existieren), nicht ein
  Harvest-Bau.
- **`bpa_gic`-Spanne** — gemessene Null: BPA führt nur ein 4-Tage/5-min-Fenster,
  kein Archiv; die zweite GIC-Zeugenreihe bleibt leer.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
