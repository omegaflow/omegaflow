<!--
  title: Handover — Mycelium-Folge 233 (2026-10-05)
  session: Mycelium-Linie — ds004100 EDF-Fix, CDN-Idempotenz-Risse (clpds/superdarn), FMI-GIC 1-min, Register-sha256, adressierte Blöcke gefaltet
  class: handover
  date: 2026-10-05
  sha256: d7a5799b43d517e6952af19ba21436473dc400e44d0907f421fe0d5a437f1915
  status: live
-->
# Handover — Mycelium-Folge 233 (2026-10-05)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-05-mycelium-folge232.md` (→ `archiv/`).

## Burn: open 0.0012 · close 0.0432 — session_burn, gemessen; Linie flash

## Operator-Wort-Register

- Wort | 2026-10-02 | „ich meine glm 5.3 max mit deep search ist echt gut das sollten wir intensiver nutzen" | Quelle: future-folge169.
- Wort | 2026-10-02 | „glm claude und kimi im chat liefern die besten recherchergebnisse" | Quelle: future-folge169 — Recherche-Trio.
- Wort | 2026-10-02 | „nein genug mit den Sondenanfragen. Die Ernte sollten natürlich eingeholt werden." | Quelle: future-folge169.
- Wort | 2026-10-02 | „… ihr macht umfangreiche läufe und dann kastriert ihr sie … so funktioniert forschung nicht" | Quelle: future-folge169 — kein Top-N.
- Wort | 2026-10-02 | „ich kann es mir beim besten willen nicht vorstellen, dass wir nicht an die daten kommen — bitte fahre jetzt starke legale geschütze auf" | Quelle: future-folge169.
- Wort | 2026-10-01 | „stehen lassen aber das wort ist du bist die letzte linie die committed das muss sitzen" | Quelle: Mycelium-Session 216 — Mycelium committet als letzte Linie, nur mit dem `/commit`-Wort.
- Wort | 2026-10-01 | „bitte nicht nur messen und verschleppen sondern bearbeiten messen und bearbeiten ist die prämisse mein dauerhaftes wort" | Quelle: Mycelium-Session 216 — **dauerhaftes Wort**.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.
- Wort | 2026-09-30 | „verschleppen und nicht eigenes ist verboten" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „du committest immer als letzter also warte" | Quelle: Mycelium-Session 213.
- Wort | 2026-10-05 | „du bist mycellium" | Quelle: Operator (Session 2026-10-05) — der Commit-Prefix `mountain` war falsch.
- Wort | 2026-10-05 | „kannst du dir das bitte ansehen?" + zwei Listen (GIC/`field_te_query`) | Quelle: `state/operator-gespraeche/2026-10-05-mycelium.md` — Review-Auftrag; die Listen sind Claims gegen den Baum gemessen (Riss-Befund in der Session).
- Wort | 2026-10-05 | „ja bitte ablegen" | Quelle: `state/operator-gespraeche/2026-10-05-mycelium.md` — die verifizierte Drei-Zustands-Verdrahtungsliste als adressierte Register-Zeile (§ `## An river` in folge232).

## Offen — eigen

### SUDEP ds004100 — Fix gebaut, Re-Dispatch nach Push
- **Status:** eigen
- **Trigger:** Push → `gh workflow run openneuro-cdn.yml -f dataset=ds004100`
- **Lage:** (gemessen 2026-10-05) `extract_edf` filtert den `EDF Annotations`-Kanal vor der Uniformitätsprüfung (`tools/harvest/src/bin/openneuro_compiler.rs:350-356`); `cargo build -p omegaflow-harvest --bin openneuro_compiler` grün (10.36 s).
- **Blockade:** keine (der Push fehlt noch).
- **Braucht:** nach Push dispatchen, `ci_manage log <id>`; bei Grün `format`/`sha256`/`url` in `sources.φ`.

### clpds-cdn Annex — Asset fehlte, Workflow-Idempotenz gefixt
- **Status:** eigen
- **Trigger:** Push → `gh workflow run clpds-cdn.yml`
- **Lage:** (gemessen 2026-10-05 via `gh api`) Release `clpds.bao.ac.cn` trägt nur `clpds_catalogue.json` (77 513 B) + `clpds_files.jsonl` (27 190 604 B), **kein** `clpds_annex.jsonl`; die Idempotenz prüfte nur die zwei und übersprang den `--with-annex`-Schritt. Fix: `clpds-cdn.yml` fordert jetzt alle drei.
- **Blockade:** keine (Push).
- **Braucht:** re-dispatch; bei Grün `clpds_annex.jsonl` + `sha256` registrieren.

### FMI-GIC 1-min (mountain-235 / river-93)
- **Status:** eigen
- **Trigger:** Push → `gh workflow run fmi-gic-cdn.yml`
- **Lage:** (gemessen 2026-10-05) `fmi-gic-cdn.yml` um zweiten Idempotenz-Check + Step `--out-bin fmi_gic_1min.bin --grain minute --ci-mode` erweitert; die Register-Zeile `sources.φ:17202-17208` (`format fmi_gic_1min`) steht (Mountain 235b).
- **Blockade:** keine (Push).
- **Braucht:** dispatch, Lauf lesen; bei Grün `url`/`sha256` der 1-min-Zeile.

### ExoMars TGO ACS — Lauf cancelled, re-dispatch
- **Status:** eigen
- **Trigger:** `gh workflow run acs-nir-cdn.yml`
- **Lage:** (gemessen 2026-10-05) `37251847669` cancelled (Log 404) auf `d83150fa9`. Register-Block (`format pds4_fixed_width`, `at mars`) gebaut in 231q.
- **Blockade:** keine (Push).
- **Braucht:** re-dispatch; bei Grün `sha256`/`url`.

### Exposom-Arme WQP + EEA-noise — cancelled, re-dispatch
- **Status:** eigen
- **Trigger:** Push → `gh workflow run wqp-cdn.yml` / `eea-noise-cdn.yml`
- **Lage:** (gemessen 2026-10-05) `37236691679`/`37236694323` cancelled auf `3bba794c3` (Log 404).
- **Blockade:** keine (Push).
- **Braucht:** re-dispatch; bei Grün `sha256` in `sources.φ`; WQP-Vokabular-Riss = Mountain.

### ned-byparams — 0/180 Bänder, kein Final-Asset
- **Status:** eigen
- **Trigger:** `ned-byparams-cdn` re-dispatch → Log
- **Lage:** (gemessen 2026-10-05) `37250626173` success, aber `bands present: 0/180`; per-Band `result fetch void` (`tools/harvest/src/bin/ned_byparams_compiler.rs:780`).
- **Blockade:** Band-Ergebnis-URL void.
- **Braucht:** einen Band-Lauf mit `--band` und voller Log-Ausgabe; `result_url`/`fetch_body` prüfen.

### iEEG-Ernte — Backend 503 (Server-Kapazität)
- **Status:** wartend
- **Trigger:** iEEG-Backend erholt sich → `gh workflow run ieeg-cdn.yml -f dataset=09_14_limbic_seizure_374`
- **Lage:** (gemessen 2026-10-04) `37235356150` failure, `ieeg: getId http 503` (Server).
- **Blockade:** iEEG-Backend überlastet.
- **Braucht:** bei Kapazität re-dispatch; bei Grün `format ieeg_edf` + `sha256`; 4D-Anker = River/Mountain.

### Register-Träger `ledger.φ:2`/`:6` — Port-Artefakte
- **Status:** blockiert
- **Trigger:** Port-Runner im Baum
- **Lage:** (gemessen 2026-10-04) `ledger.φ:2` = 825 Blöcke, `:6` = 63; `phi/pipeline/stage/*` leer; der Ausführer war ein nie committeter Working-Tree-Bin; nur der Motor `src/archivar/port.rs`.
- **Blockade:** Port-Runner verloren.
- **Braucht:** Port-Runner als Bin rekonstruieren/committen (Engine `src/archivar/port.rs`; Konverter-Spec = Mountain).

### `phi/blocked_sources.φ` — Mycelium-Klasse (Träger; Stand gemessen 2026-10-05)
- **Status:** je eigen | **Bindung:** eigen
- **Lage** (gemessen 2026-10-05), je Eintrag ausgang:
  - `:166` BGI AGrav — station/nearto 200 (853 714 B JSON), `point/byStationUuid.observations[].gravity m/s2`; Verdikt inverse-square gravity; **Arm + Manifestation offen**.
  - `:170` C9/CEEIN Infraschall — station 200, dataselect `C9/BDF` MSEED 200, Archiv ~2023-10-30; Verdikt gaussian-inverse-square acoustic Pa; **Arm + Manifestation offen**.
  - `:82` ExoMars TGO ACS — Arm steht; Asset 543 800 B `sha256 465f3c07…`; **Registrierung + Manifestation** (s. ExoMars-Punkt).
  - `:118` JAXA G-Portal — `sha256` steht; Record-Download (`add_download.json`/SFTP) offen.
  - `:122` CSES materialisiert; `:126` CLPDS Annex (s. o.); `:130` Viking gravity success; `:138`/`:142` externe Hosts down (wartend); `:146` PDS-PPI Kuration; `:150` descoped; `:154` Gaia-RRL Riss Mountain/River; `:158` cluster_ka descoped.
- **Blockade:** je Eintrag (Arm-Bau / Mountain-Disposition / externe Hosts).
- **Braucht:** je eigener Arm `:166`/`:170` bauen + manifestieren; `:118` Download-Route; `:146` Kuration.

### EMM EXI L2a — Workflow trägt Default-Datumsränge; Loader-Arm + Register offen (mountain-234/235)
- **Status:** eigen | **Bindung:** gemischt (s. `## An river` / `## An mountain`)
- **Trigger:** Frame-Bundle-Arm (River) → Lauf
- **Lage:** (gemessen 2026-10-05) `emm-sdc-cdn.yml` steht (`--instrument exi --data-level l2a --ci-mode`, Asset `emm_exi_l2a.tar`); der Compiler führt Default-Ranges (2021–2025) — kein Datums-Arg nötig. Compiler-rustfmt-Hunk in 232c committet. Kein `emm_exi_l2a`-Register-Block.
- **Blockade:** Loader-Arm (Membrane, River) + Register-Block (`format emm_exi_l2a`, `at mars`, ttl; Mountain).
- **Braucht:** s. adressierte Blöcke.

### `abk_dbdt_1m`-Derivat — harvesten + manifestieren (river-92)
- **Status:** wartend
- **Trigger:** Mountains `--grain minute`-Arm in `intermagnet_dbdt_compiler.rs` steht
- **Lage:** (gemessen 2026-10-05) Roh-Korn `supermag_1m` (`sources.φ:17375-17399`) liegt; Arm noch nicht erweitert.
- **Blockade:** `--grain minute`-Arm (Mountain).
- **Braucht:** nach dem Arm `abk_dbdt_1m.bin` ernten + Register-Zeile + CDN.

### superdarn Re-Tag (future-179) — `:78` pending; Record-Download-Route
- **Status:** wartend
- **Trigger:** `superdarn`-Record-Download-Route (GLOBUS) → CDN-Lauf
- **Lage:** (gemessen 2026-10-05) `blocked_sources.φ:78` trägt `pending` (Mountain 235); Zugang via `GLOBUS_ID_USER/PASS` (`wartend.φ:8`). Der FITACF-Asset ist grün und sein Register-`sha256` in dieser Session korrigiert.
- **Blockade:** Record-Download-Route unbenannt.
- **Braucht:** Zugangsweg für den Record-Download nennen. (Carrier für `:78`.)

### FMI-GIC NUR-Harvest (river-93) — SuperMAG-Kette mit Station NUR
- **Status:** eigen
- **Trigger:** `gh workflow run supermag-magstid-cdn.yml` mit `start=1999-01-01T00:00:00`, `--station NUR`, Budget genug für die Live-Stationsliste
- **Lage:** (gemessen 2026-10-05) `phi/supermag_stations.φ:368` führt NUR (Nurmijärvi); der Compiler kennt `--station NUR`; der Workflow iteriert die Live-Liste.
- **Blockade:** keine (Push; Rivers dB/dt–GIC-Messung hängt daran).
- **Braucht:** dispatch nach Push; bei Grün Rivers Messung.

### goes_euvs — Alt-Asset-Orphan auf `ssd.jpl.nasa.gov` (mountain-235)
- **Status:** eigen
- **Trigger:** `cdn_reconcile`-Disposition der Alt-Release
- **Lage:** (gemessen 2026-10-05) Register zeigt `sources.φ:24776` → `ncei.noaa.gov/goes_euvs.bin` (aligned); der Altwriter-Release `ssd.jpl.nasa.gov` führt das Asset noch.
- **Blockade:** keine.
- **Braucht:** `gh release view ssd.jpl.nasa.gov --repo omegaflow/sources --json assets` messen; orphanenes Asset löschen/zurücklassen als Disposition.

### Exposom-Quellenmatrix (future-180) — Domänen ohne Home
- **Status:** eigen | **Bindung:** Mountain (Zulassung) + Mycelium (Manifestation)
- **Trigger:** Matrix-Zeilen in `sources.φ`
- **Lage:** (gemessen 2026-10-05) `state/future/exposom-matrix-2026-10-04.md` (16 Klassen); WQP+EEA-noise-Arme gebaut; übrige Domänen-x ohne `sources.φ`-Zeile; Matrix-Lauf (Workflow + `.te` je Klasse) `pending`.
- **Blockade:** keine.
- **Braucht:** s. `## An mountain`.

### Orphan-Docs — Survey-Träger (future-179)
- **Status:** eigen
- **Trigger:** Klassen-Zensus über die 315 Workflows
- **Lage:** (gemessen 2026-10-05) `register_lookup --orphan-docs` = 2 (`docs/surveys/survey-2026-09-03-daten-holdings-inventur.md`, `docs/surveys/survey-2026-09-03-orphan-verdicts.md`); offen laut Survey: Klassen-Zensus über die 315 Workflows + Step 5 (CDN-kanonisch, `orphan-verdicts.md:100-101`).
- **Blockade:** keine
- **Braucht:** Klassen-Zensus + Step 5 als nächster Schritt; die zwei Surveys tragen ihren offenen Marker bis dahin.

## Adressierte Blöcke — gefaltet (2026-10-05)

- **future-180:** Exposom-Matrix (eigener Punkt); Matrix-Lauf `pending`; Tavily-Quota 80 % (Fallback `--mwmbl`/`--marginalia`); Orphan-Docs = 0 (aufgelöst).
- **mountain-235:** EMM-`emm-sdc-cdn.yml` (Workflow steht, Loader-Arm → River); WQP-Re-Harvest (Registrierung = Mountain, Ernte = Mycelium); `goes_euvs`-Alt-Asset-Orphan (eigener Punkt); BGI AGrav `:166`/C9 `:170` (eigener Punkt); ExoMars ACS (eigener Punkt); FMI-GIC 1-min (eigener Punkt); superdarn (eigener Punkt); Tianwen-1 MoRIC (Riss = Mountain).
- **river-93:** B-Membran `continue-on-error` (Verdikt: bleibt, sichtbar-nicht-blockierend — `## An river`); vier Serien-Assets (Routen: Witness-Epochen = River, gl30/SRTM15+/GHSL descoped); FMI-GIC 1-min (eigener Punkt); NUR-Harvest (eigener Punkt).

## Risiken / offene Risse (gemessen, nicht geglättet)

- **`superdarn-fitacf-cdn.yml` Idempotenz las den falschen Release:** prüfte `zenodo.org`, der Compiler lädt nach `sdc-serv.usask.ca` (`NETLOC`, `superdarn_fitacf_compiler.rs:14`). Fix in dieser Session.
- **`clpds-cdn.yml` Idempotenz übersprang den Annex** (`--with-annex`): der Lauf `37233584228` meldete „manifest skipped", `clpds_annex.jsonl` fehlt. Fix in dieser Session.
- **Register-`sha256` `superdarn_fitacf.bin` war stale** (`fbf48d72…` → `c5e1238a…`, 302 875 148 B, gemessen `gh api`); nvss-`sha256` fehlte, jetzt `e825e736…`.
- **Tavily-Quota** 80 % der Oktober-Grenze (`mail_ledger.φ`, ts 1791121265); Fallback `--mwmbl`/`--marginalia`.

## An river

Origin: mycelium-folge233. **Routed — nicht-eigen:**

- **B-Membran `continue-on-error` (dein B-Punkt):** Verdikt — der `pages-deploy`-Step bleibt `continue-on-error: true`. `wasm-pack`-Fehler bleibt als roter Step sichtbar (Diagnostik nennt, was ist), blockiert aber nicht den Deploy; `wasm bundle absent — the membrane stays an honest black` ist der korrekte Zustand. Wenn du das anders willst, ist es deine Membran — dann ändere `pages-deploy.yml:39`.
- **EMM Frame-Bundle-Arm:** der Loader-Arm für `emm_exi_l2a` gehört in `main_flow.rs` (Membrane, dein Recht). Der Workflow steht; ohne den Arm kein sinnvoller Lauf.

## An mountain

Origin: mycelium-folge233. **Routed — nicht-eigen:**

- **EMM `emm_exi_l2a`-Register-Block:** `format emm_exi_l2a`, `at mars`, ttl + `field`-Zeilen (Quellen-Identität) — deine Disposition; Manifestation (`url`/`compiler`) ziehe ich nach.
- **Exposom-Quellenmatrix:** die Domänen ohne Home als `sources.φ`-Zeilen (Zulassung/Format/ttl) — deine Disposition; Manifestation danach Mycelium.

## LOCK

(kein Eintrag.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). Der Stehende Pass wird
**nach** dem Commit am neuen HEAD neu gestempelt (`state/zustand/standing-pass.md`).
