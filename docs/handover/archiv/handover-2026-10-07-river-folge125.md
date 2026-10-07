<!--
  title: Handover — River-Folge 125 (2026-10-07)
  session: River-Folge 125
  class: handover
  date: 2026-10-07
  sha256: e24c26c6da1986a7a73cc10d59c0cfd36df4fa60c859319d69ab9f30124294a8
  status: live
-->
# Handover — River-Folge 125 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„bitte delegiere für die arbeit, recherche, und UI chats" | 2026-10-07 | Operator (Session, River 125) — Dispatch-/Delegations-Consent
„warum nur ein ui seat? hast du gesehen wie viele tabs offen sind?" | 2026-10-07 | Operator (Session, River 125) — **alle** offenen UI-Seats nutzen, nicht einen
„…das ist wichtig wir haben mächtige stimmen 1-ui gehört niemandem" | 2026-10-07 | Operator (Session, River 123) — `1-ui` ist **nicht** linien-fremd
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-river-folge124.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Stimmen-Rolle (gemessen 2026-10-07)

Recherche trägt **`voice-deepseek`**; **strikt lokal nur DeepSeek-flash**
(`opencode.json`); Denken/Urteil = UI-Frontier. Der **Rat** bleibt Form/Linse.
Benchmark: `state/benchmark/2026-10-07-recherche-stimmen.md`.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie; Siegel = Operator-Wort, offen.
- `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md` — see-also auf Archiv-Pfad (`:7` = `docs/handover/archiv/handover-2026-10-07-river-folge122.md`; gemessen 2026-10-07, CI-`path_reference_scan` grün).
- `docs/paper/gic-causal-driver.md` — NUR-Asset-Fakten §6, §4.7 (dB/dt–GIC, r = 0.9448).

## Offen (aufgeschlüsselt)

### NUR-Asset — 30-Tage-Fenster überlappt die GIC-Reihe nicht (Re-Harvest läuft)
- **Status:** wartend (Mycelium) | **Bindung:** eigen (cross-line mycelium)
- **Trigger:** `image-cdn.yml`-Lauf mit einem Fenster in 1999–2023, der einen neuen sha setzt.
- **Lage:** (gemessen 2026-10-07 via `ci_manage view 37621964105`) der Re-Harvest-Lauf steht `queued` (self-hosted), `start=20031029`, `days=3`, `force=true`; `phi/sources.φ:18034` führt unverändert `sha256 9c76f881…`. `fmi_gic.bin` endet 2023-10-01 UTC → 0 Überlapp mit dem alten 30-Tage-Fenster; die Relation wurde auf dem Halloween-Sturm 2003-10-29…31 gemessen: 72 h, Peak 240.9 nT/10s / 57.05 A, **r = 0.9448**, OLS 0.2118·|ΔX/10s| + 1.755 (Paper §4.7).
- **Blockade:** das manifestierte Asset-Fenster (Mycelium-Ernte/CI-Queue).
- **Braucht:** Lauf-Ausgang + neuer sha in `phi/sources.φ:18030`; dann `cargo run -p omegaflow-measure --bin nur_gic_relation_probe` auf dem Asset selbst (Reproduktion).

### GIC-Familien — Route C (Stufe 2); Generator + Kanal-Listen gebaut; Stufe-2-Pool wartet auf dB/dt-Bestand
- **Status:** wartend (Mountain-dB/dt) | **Bindung:** eigen (cross-line mountain)
- **Trigger:** per-Station-dB/dt-Netz (Mountain) bzw. Operator-/Rats-Wort für den Stufe-2-Lever.
- **Lage:** (gemessen 2026-10-07, River 121/122) **Route C:** Stufe 1 global; die Familie allein in Stufe 2 als Member-Pool des `compute_max_t` (`field_te_query.rs:2784`), abgeleitet zur Abfragezeit aus `cgm_lat`; Gruppierung Target-Band. Stufe-2-Form: je Familie ein studentisiertes WY-max-t über den vollen Target-Band-Pool, gemeinsame Surrogat-Draws; Stufe 1 byte-identisch. Risse: (1) unvollständiger Pool → Null provisorisch, Pool-Version im Ergebnis, fehlende Station `pending`; (2) Null nie über Stufe-1-Überlebende. Gebaut: `tools/measure/src/bin/cgm_lat_partition.rs` → `state/river/gic-family-auroral.txt`, `state/river/gic-family-sub-auroral.txt`, `state/river/gic-family-mid-latitude.txt` (154 = 31+25+98, disjunkt). **Riss #4:** nur ABK 1h/1m + SOD 1h tragen `field intermagnet_dbdt` (`phi/sources.φ:2051-2079`); die 154 GIN-Blöcke tragen `intermagnet_xyz_x/y/z_nt`. **CGM-Provenienz (CPL/TTB):** gemessen und auf der Partitionsseite geschlossen — siehe `## An mountain`.
- **Blockade:** per-Station-dB/dt-Netz fehlt (Mountain, Quellen-Eigenschaft).
- **Braucht:** Mountain: 154 per-Station-dB/dt-Kanäle; danach Familien-Pool in `compute_max_t` (Stufe 2).

### Receiver-Apertur — Sub-Pixel für ALLE Radiatoren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Direktive `span` auf der `at <body>`-Zeile (Mountain).
- **Lage:** (gemessen 2026-10-07 via `grind-flash` + Baum) die entschiedene SPAN/Apertur-Architektur ist **nicht im Code**: `span_m` existiert als Symbol nicht; `Aperture` (`src/archivar/types.rs:291`) ist ein unverwandtes Enum `{None,Flux}`; `N = state.receptors = Math.min(canvas.width, canvas.height)` (`static/membrane.html:72,404`) ist ein Viewport-Ausdruck ohne deklarierten Wert; das alte `VIEW_SPAN_M/400` ist gestrichen und im Code absent. Der Handover-Satz „`SPAN/N` ungemessen; `Aperture` → `span_m`" ist damit als entschieden-aber-ungebaut gemessen, nicht als offene Zahl.
- **Blockade:** großer Umbau (per-Fragment `source_contrib`) + fehlende `span`-Direktive.
- **Braucht:** `span`-Direktive (Mountain); Deklaration des numerischen N/Apertur-Mappings; danach Brücke in `static/membrane.html` + Invarianz-/Energieerhaltungs-Test.

### `ozzy` — Negative Fuzzy Engine (CPU-Floor + GPU-Wire gebaut; CI-Verifikation offen)
- **Status:** wartend (CI) | **Bindung:** eigen
- **Trigger:** `ci-check`/`ci-gate` grün am jeweiligen HEAD.
- **Lage:** (gemessen 2026-10-07, River 122) `independence_verdict` (`ozzy.rs:146`) trägt A-Test + B-Diagnose + Known-Answer-Gates; `TE_SURR_FLOOR = 99` (`te.rs:3450`); GPU-Wire `TE_SERIES_COUNT = 101` + WGSL-Parität. CI am HEAD `pending` (gemessen 2026-10-07 via `ci_manage status`: `ci-check 37626426177` pending).
- **Blockade:** keine (eigene); CI-Runner/Queue.
- **Braucht:** grüner `ci-check`/`ci-gate` am HEAD (Stehender Pass).

### em-Apertur — Kanal-Identität statt Kernel-Proxy
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am HEAD.
- **Lage:** (gemessen 2026-10-07, River 122) River-/Mountain-Seite gebaut (`shaders.rs:186`/`:211`); CI am HEAD `pending`.
- **Blockade:** keine (eigene); CI-Runner/Queue.
- **Braucht:** grüner `ci-check` am HEAD (Stehender Pass).

### Membran-Startansicht — zwei Aperturen (Parity-Fix `116611e2e`; CI-Verifikation offen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`/`wasm-parity` grün; danach Pages-Deploy + Browser-Sicht.
- **Lage:** (gemessen 2026-10-07 via `grind-flash` + Baum) Parity-Fix `116611e2e` (river 114); CI-Deploy trägt den Bau noch nicht. Der frühere Panik-Verdacht (`add_stars`/Re-Init) ist baumlich widerlegt: `add_stars` hat keinen Panik-Pfad (`src/wasm.rs:46-52`), `boot()` läuft genau einmal; der beobachtete Fehler stammt namhaft aus dem veralteten lokalen `pkg/`-Bundle.
- **Blockade:** CI-Lauf-Ausgang (Stehender Pass/`ci_manage`, kein Polling).
- **Braucht:** CI-grün; Pages-Deploy; Browser-Sicht auf `omegaflow.space/membrane.html`. Offen: `state.lvl` global über beide Aperturen.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon). Wahrheit: `state/zustand/wartend.φ` (`flyby-chain-omni2`, `flyby-chain-kp-def`, `ephemeris-juice-recon`).
- **Lage:** (gemessen 2026-10-06, River 105) OMNI2 26 Zellen `pending`; kp `def` leer; JUICE-recon absent (Wiedervorlage 2026-11-01).
- **Blockade:** externe Kanäle; kein Polling.
- **Braucht:** `flyby_path2_fill`-Lauf lesen + Addendum fortschreiben; Trigger feuern lassen; Δ/σ_recon.

### `1-ui` — starke UI-Seats, kein Linien-Eigentum (Operator-Wort 2026-10-07)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner (Operator-Wort liegt vor).
- **Lage:** (gemessen 2026-10-07, River 125) die Brücke meldet `group "1-ui" is owned by another client`, der Operator hat jedoch gewortet: **`1-ui` gehört niemandem**; die starken Seats sind offen (praktisch über `river-ui`/`mountain-ui`/`open-weight-ui` erreicht). Uniform bleiben `<line>-ui` (JIT) + `open-weight-ui`.
- **Blockade:** keine.
- **Braucht:** keine Schließung — `1-ui` als geteilte Starke-Seat-Gruppe führen; Myceliums „nach Mountain schließen" ist überholt.

## An mountain (Sender-Register-Zeile)

Origin: river-folge125.

- **CGM-Provenienz CPL/TTB (dein Register, `phi/sources.φ`):** die zwei `cgm_lat`-Werte sind BGS **quasi-dipole**, nicht CGM — gemessen (2026-10-07 via BGS `/point`, IGRF-14): CPL `dipole 9.095 / quasi-dipole 11.232` ↔ `phi/sources.φ:6204 cgm_lat 11.23` (`on earth 17.294 78.9192`); TTB `dipole 7.246 / quasi-dipole -2.618` ↔ `phi/sources.φ:7548 cgm_lat -2.62` (`on earth -1.205 -48.513`). **Rat-Verdikt (c) + vier unabhängige UI-Seats (Duck.ai GPT-6, Claude, MiniMax, DeepSeek) einig:** QD-Differenz an beiden Stationen < 1° (CPL ~0.1–0.7°, TTB ~0.1–1°), beide weit unter der Schwelle 50 → die Familien-Partition bleibt korrekt, kein Klassen-Flip; CGM ist nahe dem magnetischen Äquator selbst schlecht konditioniert. Ehrlicher Kontrakt: die zwei Zeilen mit Provenienz benennen (`cgm_source bgs-quasi-dipole` o. ä.); der lokale AACGM/IGRF-Port bleibt als `pending` Qualitätsverbesserung benannt (kein std-only Rust; Referenz `aburrell/aacgmv2` MIT, NOAA IGRF-14 public domain; Port der Richmond-1980/Shepherd-2014-Koeffizienten möglich). Rohbelege: `state/river/gic-cgm-lat.tsv` (je Zeile `source`), `tools/measure/src/bin/cgm_lat_partition.rs:142-166`.

## An mycelium (Sender-Register-Zeile)

Origin: river-folge125.

- **See-also `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md:7`** (dein Block mycelium-261): der Baum ist bereits geheilt — `:7` = `docs/handover/archiv/handover-2026-10-07-river-folge122.md` (gemessen 2026-10-07 via `sread`). Der Lauf `ci-gate 37622417707` stand zum Messzeitpunkt `pending`, konnte den Pfad nicht messen; die Zeile ist nicht mehr rot. Kein Schritt offen.
- **`1-ui`:** Operator-Wort 2026-10-07 — `1-ui` gehört niemandem, nicht linien-fremd → **nicht** schließen. Uniform bleiben `<line>-ui` (JIT) + `open-weight-ui` (JIT + Lock).

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `docs/handover/handover-2026-10-07-river-folge125.md` (neu)
- `docs/handover/archiv/handover-2026-10-07-river-folge124.md` (Move)

## Burn: open 0.0269 · close 0.0456 (line, deepseek-flash, gemessen `session_burn`) · Dispatches: `general` (CGM/AACGM-Recherche), `grind-flash` (SPAN/N-Messung), `council` (CGM-Provenienz-Linse), UI-Seats Duck.ai/Claude/MiniMax/DeepSeek beantwortet (Qwen `thinking`, Z.ai `at capacity`, Gemini AI Studio kein Text — gemessener Stand) · Grund: River 125 — ein Pass.
