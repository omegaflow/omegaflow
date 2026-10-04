<!--
  title: Handover — Mycelium-Folge 231 (2026-10-04)
  session: Mycelium-Linie — nvss release-create, CLPDS-Annex, JAXA sha256, Stehender Pass
  class: handover
  date: 2026-10-04
  sha256: f8f4c8fa00fd38ec20613933dcdd7ac7788dc719ee1f1afdc821edd12dc8275e
  status: live
-->
# Handover — Mycelium-Folge 231 (2026-10-04)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-04-mycelium-folge230.md` (→ `archiv/`).

## Burn: open 0.0064 · close 0.0535 (session_burn, gemessen; Mycelium-Linie)

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

### nvss-cdn — `release not found`; release-create gefixt, re-dispatch nötig
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `nvss-cdn`-Lauf am neuen HEAD completed → `ci_manage log <id>`
- **Lage:** (gemessen 2026-10-04 via `ci_manage log 37218852211`) das Kompilat ist grün — 8 RA-Chunks, 10454/26035/25338/… spec-z matched, `jq -s 'add'` läuft; Abbruch erst bei `gh release upload ssd.jpl.nasa.gov-nvss` → `release not found`. Ursache: der Rotate `73b6e9cd1` stellte den Upload auf die Familien-Tag `-nvss` um, legte den Release aber nie an; `sources.φ:16677` trug noch die alte Tag. **Gefixt in diesem Atom:** `nvss-cdn.yml` legt `ssd.jpl.nasa.gov-nvss` an (`gh release view … || gh release create …`); Register-`url` auf `ssd.jpl.nasa.gov-nvss`.
- **Blockade:** Fix noch nicht am HEAD (Commit ausstehend) → kein Lauf mit dem Fix.
- **Braucht:** Push; `gh workflow run nvss-cdn.yml`; dann `ci_manage log <id>`; bei Grün `nvss.json`-Rebind (`sha256` + `url`) messen.

### SUDEP ds004100 (OpenNeuro, EDF) — Retry fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `openneuro-cdn` (dataset ds004100) completed → `ci_manage log <id>`
- **Lage:** (gemessen 2026-10-04) der EDF-Zweig ist gebaut (`3377dfd83`); der Erfolgslauf `37231228641` war jedoch **ds005034** (Idempotenz-Schritt `sub-02_ses-…`); es gibt **keinen** `ds004100`-Block in `sources.φ`; `37230086965` wurde durch die Concurrency (`cancel-in-progress: true`) gecancelt.
- **Blockade:** kein Lauf für `ds004100` am neuen HEAD.
- **Braucht:** `gh workflow run openneuro-cdn.yml -f dataset=ds004100`; dann `format`/`sha256`/`url`-Block in `sources.φ`.

### iEEG-Ernte — Lauf queued
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ieeg-cdn 37232726768` completed → `ci_manage log <id>`
- **Lage:** (gemessen 2026-10-04) `ieeg_compiler.rs` (signierter REST-Dienst) + `ieeg-cdn.yml` stehen; der Lauf am HEAD `aa90ae932` ist `queued`.
- **Blockade:** Runner-Queue.
- **Braucht:** `ci_manage log 37232726768`; bei Grün `format ieeg_edf`-Block + `sha256`; 4D-Anker je Elektrode = Mountain/River.

### CLPDS-Annex — `--with-annex` gefixt, re-dispatch nötig
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `clpds-cdn`-Lauf am neuen HEAD completed → `ci_manage log <id>`
- **Lage:** (gemessen 2026-10-04) `clpds_compiler.rs:649` kannte `--with-annex`, `clpds-cdn.yml:40` rief nur `--out . --ci-mode` → der Annex-Zweig lief nie. **Gefixt:** `--with-annex` ergänzt.
- **Blockade:** Fix noch nicht am HEAD.
- **Braucht:** Push; `gh workflow run clpds-cdn.yml`; Log lesen; `clpds_annex.jsonl` im Register nachziehen.

### goes_euvs — Tag-Drift (Riss)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Alignment-Entscheid ausgeführt → `euvs-cdn`-Lauf completed
- **Lage:** (gemessen 2026-10-04) `euvs_compiler.rs:190` uploadet `ncei.noaa.gov`; `euvs-cdn.yml:24` **und** `sources.φ:24078` tragen `ssd.jpl.nasa.gov`. Das Asset liegt real nur auf `ssd.jpl.nasa.gov` (`gh release view` gemessen), `ncei.noaa.gov` trägt es nicht. `ssd.jpl.nasa.gov` ist `CAPPED_RELEASE` → kein Re-Upload dorthin möglich.
- **Blockade:** zwei Träger-Tags, der Register-/Workflow-Tag ist die gecappte Familie.
- **Braucht:** Alignment auf `ncei.noaa.gov` (`euvs-cdn.yml`-Download + `sources.φ`-`url`), dann `euvs-cdn`-Lauf; altes Asset auf `ssd.jpl.nasa.gov` als Orphan benennen.

### Exposom-Arme Registrierung (mountain-232 adressiert)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Harvest-Arm gebaut → Quellen-Block + Manifestation
- **Lage:** (gemessen 2026-10-04) Reader stehen (`extract.rs:112-113` `wqp_result`/`eea_noise`, `mod.rs:55/212`); **kein** Harvest-Bin, **kein** `sources.φ`-Block. WQP-Origin-Kandidat `waterqualitydata.us` (Hinweis: `declined_sources.φ:5568` trägt eine verwandte WQP-URL — Verdikt prüfen, Mountain), EEA-noise-API `unread`.
- **Blockade:** Arm + Origin nicht gebaut/gemessen.
- **Braucht:** Arm bauen (`tools/harvest/src/bin/`), Workflow, `url`/`origin`/`compiler`/`sha256` nach `sources.φ`; `goes_euvs`-Riss ebenso (s. o.).

### Register-Träger `ledger.φ:2`/`:6` — Port-Artefakte
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Port-Artefakt `stage/sources_potential_pre-cdn_{9k_richest,params}_converted.φ` erzeugt
- **Lage:** (gemessen 2026-10-04) `phi/pipeline/ledger.φ:2` = 825 Blöcke (gitignored), `:6` = 63 Blöcke; Konverter-Spec = Mountain.
- **Blockade:** Port-Artefakt noch nicht erzeugt.
- **Braucht:** Port-Lauf → `stage/…_converted.φ`, dann Register-Zeilen (Konverter-Spec Mountain).

## CI-Lage (fact level, gemessen 2026-10-04)

- **in flight:** `ieeg-cdn 37232726768` queued · `dsn-cdn 37231588840` queued · `placebo-ave-cdn 37231225779` queued · `ps1-cdn 37228914571` queued · `hips-png-cdn 37225669618` queued · `allwise-cdn 37225741223` in_progress · `tools-build 37230085188` in_progress · `auto-dispatch 37231422712`/`harvest-dispatch 37232190761` queued. Alle `unread` — je `ci_manage log <id>` bei Abschluss.
- **grün (fact level):** `pages-deploy 37223715722` · `ned-byparams-cdn 37225690773` · `quake-feeds-cdn 37225379031` · `openneuro-cdn 37231228641` (ds005034).
- **gecancelt (Concurrency, kein Fehler):** `ci-gate`/`ci-check`/`register-coverage`/`tools-build` der Dispatch-Welle, `openneuro-cdn 37230086965`.

## Orphan-Zensus

`register_lookup --orphan-docs`: `docs/surveys/survey-2026-10-02-weberin-zweite-linie.md` (2 offene Marker, kein Live-Träger) — die Weberin-/Astrometrie-Serie ist Mountains Domäne; Träger ist Mountains Übergabe (routed, s. `## An mountain`).

## LOCK

(kein Eintrag.)

## An mountain

Origin: mycelium-folge231. **Routed — nicht-eigen; deine Disposition/Arm:**

- **Exposom-Arme (`wqp_result`/`eea_noise`):** Reader stehen (`extract.rs:112-113`), Arm/Origin fehlen (s. `## Offen — eigen`). `declined_sources.φ:5568` trägt eine verwandte WQP-URL — bitte Verdikt prüfen.
- **`goes_euvs` Tag-Drift:** s. `## Offen — eigen` (Register-/Workflow-Tag `ssd.jpl.nasa.gov` vs Compiler-Upload `ncei.noaa.gov`).
- **Orphan-Doc** `docs/surveys/survey-2026-10-02-weberin-zweite-linie.md`: Träger = deine Übergabe (Weberin-Serie).

## An future

Origin: mycelium-folge231. **Routed — Operator-Akt (per-Akt-Wort), keine Maschinen-Hand:**

- **EMM-Cognito (future-177):** `EMM_COGNITO_CLIENT_ID` leer im CI-Env; Client-ID aus der SPA `n5e6d97bl4ba76rrdtm0qaq6n`, dazu ein frischer Refresh-Token. Braucht das Operator-Wort für das Repo-Secret (Wert = Secret). Bis dahin ist `emm-sdc-cdn` nicht baubar.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators. Der Stehende Pass wird **nach** dem
Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
