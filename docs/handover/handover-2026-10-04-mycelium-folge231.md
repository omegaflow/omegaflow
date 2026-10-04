<!--
  title: Handover — Mycelium-Folge 231 (2026-10-04)
  session: Mycelium-Linie — nvss/CLPDS/iEEG-Fixes, goes_euvs-Alignment, Exposom-Arme, Stehender Pass
  class: handover
  date: 2026-10-04
  sha256: 4dc19ebeed607f9c8696c4f244db1f638fe78a8a3d0efe50fdad083331cf1370
  status: live
-->
# Handover — Mycelium-Folge 231 (2026-10-04)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-04-mycelium-folge230.md` (→ `archiv/`).

## Burn: open 0.0064 · close 0.10 (session_burn, gemessen; Mycelium-Linie + 1 flash-Taucher)

## Operator-Wort-Register

- Wort | 2026-10-02 | „ich meine glm 5.3 max mit deep search ist echt gut das sollten wir intensiver nutzen" | Quelle: future-folge169 (`state/operator-gespraeche/2026-10-02-future-folge169.md`) — GLM-5.3 Deep Think Max + Deep Search als **erster** Kanal für Tiefen-Recherche.
- Wort | 2026-10-02 | „glm claude und kimi im chat liefern die besten recherchergebnisse" | Quelle: future-folge169 — **Recherche-Trio** (`chat.z.ai` · `claude.ai` · `kimi.ai`) = erster Kanal.
- Wort | 2026-10-02 | „kimi.ai mit k3 geht nicht es geht nur kimi k3 in tryingopen 4000 zeichen i kimi.ai ist es schnell (schätze 2.6)" | Quelle: future-folge169.
- Wort | 2026-10-02 | „für sonnet 5.5 search geht auch immer arena" | Quelle: future-folge169.
- Wort | 2026-10-02 | „nein genug mit den Sondenanfragen. Die Ernte sollten natürlich eingeholt werden." | Quelle: future-folge169.
- Wort | 2026-10-02 | „… ihr macht umfangreiche läufe und dann kastriert ihr sie … so funktioniert forschung nicht" | Quelle: future-folge169 — **kein Top-N**, vollständige Klassifikation.
- Wort | 2026-10-02 | „ich kann es mir beim besten willen nicht vorstellen, dass wir nicht an die daten kommen — bitte fahre jetzt starke legale geschütze auf" | Quelle: future-folge169.
- Wort | 2026-10-02 | „füll" / „bitte auch nochmal losschicken" (GSICS/KASI) | Quelle: future-folge169.
- Wort | 2026-10-01 | „ich habe dir nicht erlaubt zu committen und zu pushen" | Quelle: Mycelium-Session 216.
- Wort | 2026-10-01 | „stehen lassen aber das wort ist du bist die letzte linie die committed das muss sitzen" | Quelle: Mycelium-Session 216 — Mycelium committet **als letzte** Linie, nur mit dem `/commit`-Wort.
- Wort | 2026-10-01 | „bitte nicht nur messen und verschleppen sondern bearbeiten messen und bearbeiten ist die prämisse mein dauerhaftes wort" | Quelle: Mycelium-Session 216.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.
- Wort | 2026-09-30 | „verschleppen und nicht eigenes ist verboten" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „du committest immer als letzter also warte" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „vorbestehend ist verboten mein wort" | Quelle: mountain-209.
- Wort | 2026-10-01 | „ja möchte ich" | Quelle: Mycelium-Session 215 — VCO-rs-Register auf das PDS4-20190704-Asset umstellen.
- Wort | 2026-10-03 | „§1-Compiler-Hosts verdiktet: vizier.cfa keep · noaa-eri-pds declined · dachs.fai.kz declined · gsaweb keep · ws.cadc keep." | Quelle: Operator-Session 2026-10-03 (deckt mountain-folge229:183-196) — ausgeführt in Mycelium-226.

## Offen — eigen

### nvss-cdn — release-create gefixt, Lauf läuft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `nvss-cdn 37233581603` completed → `ci_manage log <id>`
- **Lage:** (gemessen 2026-10-04 via `ci_manage log 37218852211`) das Kompilat ist grün (8 RA-Chunks, SkyServer-Arm); Abbruch bei `gh release upload ssd.jpl.nasa.gov-nvss` → `release not found`. **Gefixt:** `nvss-cdn.yml` legt den Familien-Release an; Register-`url` auf `-nvss` + `origin`-Zusatz `xmatch=V/154/sdss16`.
- **Blockade:** Runner-Queue.
- **Braucht:** `ci_manage log 37233581603`; bei Grün `nvss.json`-`sha256` + `url`-Rebind messen.

### SUDEP ds004100 — EDF-Lauf läuft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `openneuro-cdn 37233586379` (ds004100) completed → `ci_manage log <id>`
- **Lage:** (gemessen 2026-10-04) EDF-Zweig gebaut (`3377dfd83`); der frühere „Erfolgslauf" `37231228641` war **ds005034** (Idempotenz `sub-02_ses-…`); kein `ds004100`-Block in `sources.φ`.
- **Blockade:** Runner-Queue.
- **Braucht:** `ci_manage log 37233586379`; bei Grün `format`/`sha256`/`url`-Block in `sources.φ`.

### CLPDS-Annex — `--with-annex` gefixt, Lauf läuft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `clpds-cdn 37233584228` completed → `ci_manage log <id>`
- **Lage:** (gemessen 2026-10-04) `clpds-cdn.yml:40` rief nur `--out . --ci-mode`, der Annex-Zweig (`clpds_compiler.rs:649`) lief nie. **Gefixt:** `--with-annex`.
- **Blockade:** Runner-Queue.
- **Braucht:** Log lesen; `clpds_annex.jsonl` im Register nachziehen.

### iEEG-Ernte — Backend 503 (Server-Kapazität)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** iEEG-Backend erholt sich → `gh workflow run ieeg-cdn.yml -f dataset=09_14_limbic_seizure_374`
- **Lage:** (gemessen 2026-10-04 lokal `cargo run … ieeg_compiler` → `ieeg: getId http 503`; CI `37235356150` = failure) der signierte REST-Weg trägt; **der Dienst antwortet 503** („back-end server is at capacity", nicht Auth). Zugleich war der frühere CI-`void` ein zweiter, echter Fehler: die hdr/body-Tempdateien lagen hardcoded unter `/tmp/opencode/`, das im Runner fehlt → **gefixt** (`std::env::temp_dir()` + `create_dir_all`), der `secret()`-Leser liest jetzt auch `IEEG_USER`/`IEEG_PASS` aus der Env (CI).
- **Blockade:** iEEG-Backend überlastet (Server).
- **Braucht:** warten auf Server-Kapazität, dann re-dispatch; bei Grün `format ieeg_edf`-Block + `sha256`; 4D-Anker je Elektrode = Mountain/River.

### Exposom-Arme (WQP + EEA-noise) — gebaut, Register steht, CI läuft
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `wqp-cdn 37236691679` / `eea-noise-cdn 37236694323` completed → `ci_manage log <id>`
- **Lage:** (gemessen 2026-10-04, flash-Taucher) Arm + Workflow + Register gebaut: `wqp_result_compiler.rs` (WQP-Result-CSV/Zip, `waterqualitydata.us`, 319 525 B, 9 926 Records) und `eea_noise_compiler.rs` (EEA ArcGIS layer 76 `City_Area_Noise`, `eea.europa.eu`, 347 425 B, 15 154 Records); Register-Blöcke (`url`/`format`/`origin`/`compiler`/`at`/`ttl`) in `sources.φ`. `cargo check` 0/0, beide Bins grün, `commit_check` grün.
- **Blockade:** Runner-Queue; `sha256` erst nach dem Lauf.
- **Braucht:** Logs lesen; `sha256` beider Assets in `sources.φ`; WQP-Vokabular-Riss = Mountain (s. `## An mountain`).

### Register-Träger `ledger.φ:2`/`:6` — Port-Artefakte
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Port-Artefakt `stage/sources_potential_pre-cdn_{9k_richest,params}_converted.φ` erzeugt
- **Lage:** (gemessen 2026-10-04) `phi/pipeline/ledger.φ:2` = 825 Blöcke, `:6` = 63 Blöcke; `glob phi/pipeline/stage/*` = **leer** — die in `index.φ:3-4` als `erledigt` notierten `stage/…_converted.φ` fehlen am Träger (gitignored, nicht reproduziert). `src/archivar/port.rs` ist aktuell durch eine andere Linie uncommittet modifiziert.
- **Blockade:** Port-Lauf (Konverter-Spec = Mountain) + laufende Fremd-Edit an `port.rs`.
- **Braucht:** Port-Lauf `cargo run -- --port phi/pipeline/queue/<korpus>.φ phi/pipeline/stage/<korpus>_converted.φ` (Bin-Bindung über SOURCE_PORT.md §5); dann Register-Zeilen.

## CI-Lage (fact level, gemessen 2026-10-04)

- **grün:** `euvs-cdn 37235359207` (Asset `goes_euvs.bin` liegt nun auf `ncei.noaa.gov`, `--sniff` sha256 `45b9c0ae…` == Register) · `swpc-mirror-cdn 37235378718` · `pages-deploy 37223715722` · `openneuro-cdn 37231228641` (ds005034).
- **rot:** `ieeg-cdn 37235356150` (503, s. o.) · `auto-dispatch 37235337903` (nicht-eigen, `unread`) · frühere `ci-check 37223709589`/`37220591967` (VerdictLine-Scope, Träger River, s. `## An river`).
- **in flight:** `nvss-cdn 37233581603` · `clpds-cdn 37233584228` · `openneuro-cdn 37233586379` · `wqp-cdn 37236691679` · `eea-noise-cdn 37236694323` · `dsn-cdn 37231588840` · `placebo-ave-cdn 37231225779` · `ps1-cdn 37228914571` · `hips-png-cdn 37225669618` · `allwise-cdn 37225741223` · `tools-build 37235337896` · `ci-check 37236691571`; je `unread`, `ci_manage log <id>` bei Abschluss.

## Orphan-Zensus

`register_lookup --orphan-docs` = 0 · `register_lookup --orphans` = 0 (measured 2026-10-04).

## LOCK

(kein Eintrag.)

## An river

Origin: mycelium-folge231. **Routed — nicht-eigen; deine Disposition:**

- **`ci-check` rot (VerdictLine-Scope):** `cargo test` E0422/E0433 — `VerdictLine`/`VerdictWord` nicht im Scope (`src/mathematikerin/tests.rs:1375/1377/1401/1406`; der Test `a_direction_witness_series_…` importiert nur `AstroSample, AstroSeries`). Braucht: `use crate::archivar::weberin_verdicts::{VerdictLine, VerdictWord};` im Test; der `ci-check` am mycelium-HEAD prüft es.

## An mountain

Origin: mycelium-folge231. **Routed — nicht-eigen; deine Disposition:**

- **WQP-Vokabular-Riss:** `src/archivar/wqp_result.rs` matcht die **kurzen** `CharacteristicName`-Werte (`Temperature`, `Dissolved oxygen`, `Conductivity`, `Nitrate-N`), die öffentliche WQP-Vokabel ist aber `Temperature, water` · `Dissolved oxygen (DO)` · `Specific conductance` · `Nitrate`. Gemessen: 6/10 Komponenten lösen gegen das Ernte-Asset auf, 4 bleiben durch das Vokabular absent (nicht durch fehlende Daten). Braucht: Reader-Namen auf die lange WQP-Vokabel + den Query im neuen `wqp_result_compiler.rs` koppeln.
- **`goes_euvs`-Alignment vollzogen:** Compiler/Workflow/Register nun `ncei.noaa.gov` (Asset dort, sha256 identisch). Das Alt-Asset auf `ssd.jpl.nasa.gov` ist damit ein Orphan — `cdn_reconcile`-Disposition.

## An future

Origin: mycelium-folge231. **Routed — Operator-Akt (per-Akt-Wort), keine Maschinen-Hand:**

- **EMM-Cognito (future-177):** `EMM_COGNITO_CLIENT_ID` leer im CI-Env; Client-ID aus der SPA `n5e6d97bl4ba76rrdtm0qaq6n`, dazu ein frischer Refresh-Token. Braucht das Operator-Wort für das Repo-Secret (Wert = Secret).

## An sensory

Origin: mycelium-folge231.

- **Hinweis:** der Commit `8229bb3ad` (mycelium 231b) trug den von der Sensory-Session gestagten Rename `docs/handover/{ => archiv}/handover-2026-10-04-sensory-folge229.md` mit — mein `git commit` war (entgegen der Regel) nicht pfad-begrenzt. Kein Inhalt verloren; ab jetzt committet Mycelium pfad-begrenzt. Eure Dateien `docs/handover/handover-2026-10-04-sensory-folge230.md` und die offenen `M`-Pfade bleiben unberührt euer.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators. Der Stehende Pass wird **nach** dem
Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
