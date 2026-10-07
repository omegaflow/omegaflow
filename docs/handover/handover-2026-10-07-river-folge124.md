<!--
  title: Handover — River-Folge 124 (2026-10-07)
  session: River-Folge 124
  class: handover
  date: 2026-10-07
  sha256: 26a6c34c19dc3803510b108f41a8c5dcc757649578731779cd0cd8d6dd34d82f
  status: live
-->
# Handover — River-Folge 124 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„bitte delegiere an taucher ich möchte dass du endlich mal fertig wirst" | 2026-10-07 | Operator (Session, River 124) — Dispatch-/Delegations-Consent
„…das ist wichtig wir haben mächtige stimmen 1-ui gehört niemandem" | 2026-10-07 | Operator (Session, River 123) — `1-ui` ist **nicht** linien-fremd
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-river-folge123.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Stimmen-Rolle (gemessen 2026-10-07)

Recherche trägt **`voice-deepseek`**; **strikt lokal nur DeepSeek-flash**
(`opencode.json`); Denken/Urteil = UI-Frontier. Der **Rat** bleibt Form/Linse.
Benchmark: `state/benchmark/2026-10-07-recherche-stimmen.md`.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie; Siegel = Operator-Wort, offen.
- `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md` — see-also auf Archiv-Pfad gebogen (2026-10-07); CI-`path_reference_scan` grün.
- `docs/paper/gic-causal-driver.md` — NUR-Asset-Fakten §6, §4.7 (dB/dt–GIC, r = 0.9448).

## Offen (aufgeschlüsselt)

### NUR-Asset — 30-Tage-Fenster überlappt die GIC-Reihe nicht (Re-Harvest läuft)
- **Status:** wartend (Mycelium) | **Bindung:** eigen (cross-line mycelium)
- **Trigger:** `image-cdn.yml`-Lauf mit einem Fenster in 1999–2023, der einen neuen sha setzt.
- **Lage:** (gemessen 2026-10-07 via `ci_manage view 37621964105`) der Re-Harvest-Lauf steht `queued` (self-hosted), `start=20031029`, `days=3`, `force=true`; das Register `phi/sources.φ:18034` führt weiter `sha256 9c76f881…` (unverändert). `fmi_gic.bin` endet 2023-10-01 UTC → 0 Überlapp mit dem alten 30-Tage-Fenster; die Relation wurde daher auf dem Halloween-Sturm 2003-10-29…31 gemessen: 72 h, Peak 240.9 nT/10s / 57.05 A, **r = 0.9448**, OLS 0.2118·|ΔX/10s| + 1.755 (Paper §4.7).
- **Blockade:** das manifestierte Asset-Fenster (Mycelium-Ernte/CI-Queue).
- **Braucht:** Lauf-Ausgang + neuer sha in `phi/sources.φ:18030`; dann `cargo run -p omegaflow-measure --bin nur_gic_relation_probe` auf dem Asset selbst (Reproduktion).

### GIC-Familien — Rat-Verdikt Route C (Stufe 2); Generator + Kanal-Listen gebaut; Stufe-2-Pool wartet auf dB/dt-Bestand
- **Status:** wartend (Mountain-dB/dt) | **Bindung:** eigen (cross-line mountain)
- **Trigger:** per-Station-dB/dt-Netz (Mountain) bzw. Operator-/Rats-Wort für den Stufe-2-Lever.
- **Lage:** (gemessen 2026-10-07, River 121/122) **Route C:** Stufe 1 global; die Familie allein in Stufe 2 als Member-Pool des `compute_max_t` (`field_te_query.rs:2784`), abgeleitet zur Abfragezeit aus `cgm_lat`; Gruppierung Target-Band. Stufe-2-Form (20-Stimmen-Rat): je Familie ein studentisiertes WY-max-t über den vollen Target-Band-Pool, gemeinsame Surrogat-Draws; Stufe 1 byte-identisch. Zwei Risse: (1) unvollständiger Pool → Null provisorisch, Pool-Version im Ergebnis, fehlende Station `pending`; (2) Null nie über Stufe-1-Überlebende. Gebaut: `tools/measure/src/bin/cgm_lat_partition.rs` → `state/river/gic-family-{auroral,sub-auroral,mid-latitude}.txt` (154 = 31+25+98, disjunkt). **Riss #4:** nur ABK 1h/1m + SOD 1h tragen `field intermagnet_dbdt` (`phi/sources.φ:2051-2079`); die 154 GIN-Blöcke tragen `intermagnet_xyz_x/y/z_nt`. Literatur: `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md`; Rohmaterial `state/river/gic-stage2-20-stimmen-2026-10-07.md`.
- **Blockade:** per-Station-dB/dt-Netz fehlt (Mountain, Quellen-Eigenschaft).
- **Braucht:** (1) Mountain: 154 per-Station-dB/dt-Kanäle; (2) danach Familien-Pool in `compute_max_t` (Stufe 2); (3) CGM-Provenienz CPL/TTB: `phi/sources.φ` führt CPL `cgm_lat 11.23` / TTB `cgm_lat -2.62` aus dem `bgs-quasi-dipole`-Fallback (`cgm_lat_partition.rs:165`, QD ≠ CGM; OMNIWeb-VITMO weist |lat| < 20° ab) — Braucht lokales AACGM/IGRF-Bin.

### `ozzy` — Negative Fuzzy Engine (CPU-Floor + GPU-Wire gebaut; CI-Verifikation offen)
- **Status:** wartend (CI) | **Bindung:** eigen
- **Trigger:** `ci-check`/`ci-gate` grün am jeweiligen HEAD.
- **Lage:** (gemessen 2026-10-07, River 122) `independence_verdict` (`ozzy.rs:146`) trägt A-Test + B-Diagnose + Known-Answer-Gates; `TE_SURR_FLOOR = 99` (`te.rs:3450`); GPU-Wire `TE_SERIES_COUNT = 101` + WGSL-Parität. CI `queued`/`unread`.
- **Blockade:** keine (eigene); CI-Runner/Queue.
- **Braucht:** grüner `ci-check`/`ci-gate` am HEAD (Stehender Pass).

### em-Apertur — Kanal-Identität statt Kernel-Proxy
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am HEAD.
- **Lage:** (gemessen 2026-10-07, River 122) River-/Mountain-Seite gebaut (`shaders.rs:186`/`:211`); CI `queued`/`unread`.
- **Blockade:** keine (eigene); CI-Runner/Queue.
- **Braucht:** grüner `ci-check` am HEAD (Stehender Pass).

### Membran-Startansicht — zwei Aperturen (Parity-Fix `116611e2e`; CI-Verifikation offen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`/`wasm-parity` grün; danach Pages-Deploy + Browser-Sicht.
- **Lage:** (gemessen 2026-10-07 via `grind-flash` + Baum) Parity-Fix `116611e2e` (river 114); CI-Deploy trägt den Bau noch nicht. **Riss gemessen:** die Notiz „`MembraneLookup.add_stars` panikt bei Re-Init" (river-123:72) ist **nicht baumlich gedeckt** — `add_stars` hat keinen Panik-Pfad (`src/wasm.rs:46-52`: Empty-Guard + `extend` + `hash=None`), und es existiert **kein Re-Init**: `boot()` läuft genau einmal (`static/membrane.html:585`), der Konstruktor erhält den leeren Slice (`wasm.rs:42`). Der beobachtete Fehler stammt **namhaft** aus dem veralteten lokalen `pkg/`-Bundle (gitignored) — als Verdacht benannt, nicht baum-gemessen: dessen Stand trägt `add_stars` womöglich noch nicht, gegen den die Seite `lookup.add_stars` ruft (`membrane.html:569`); der CI-Build (`pages-deploy.yml:42`, `wasm-pack --target web`) baut frisch aus `src/wasm.rs`.
- **Blockade:** CI-Lauf-Ausgang `unread` (Stehender Pass/`ci_manage`, kein Polling).
- **Braucht:** CI-grün; Pages-Deploy; Browser-Sicht auf `omegaflow.space/membrane.html`. Offen: `state.lvl` global über beide Aperturen.

### Receiver-Apertur — Sub-Pixel für ALLE Radiatoren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Direktive `span` auf der `at <body>`-Zeile (Mountain).
- **Lage:** (gemessen 2026-10-06, River 109/110) sichtbarer Pfad halb geheilt; SPAN/Apertur-Architektur entschieden (Rat + 6 UI-Modelle). Risse: `SPAN/N` ungemessen; `Aperture` → `span_m`.
- **Blockade:** großer Umbau (per-Fragment `source_contrib`).
- **Braucht:** `span`-Direktive (Mountain); Brücke in `static/membrane.html`; Invarianz-/Energieerhaltungs-Test; danach alle fünf Radiatoren.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon). Wahrheit: `state/zustand/wartend.φ` (`flyby-chain-omni2`, `flyby-chain-kp-def`, `ephemeris-juice-recon`).
- **Lage:** (gemessen 2026-10-06, River 105) OMNI2 26 Zellen `pending`; kp `def` leer; JUICE-recon absent (Wiedervorlage 2026-11-01).
- **Blockade:** externe Kanäle; kein Polling.
- **Braucht:** `flyby_path2_fill`-Lauf lesen + Addendum fortschreiben; Trigger feuern lassen; Δ/σ_recon.

### `1-ui` — starke UI-Seats, kein Linien-Eigentum (Operator-Wort 2026-10-07)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner (Operator-Wort liegt vor).
- **Lage:** (gemessen 2026-10-07) die Brücke meldet `group "1-ui" is owned by another client`, doch der Operator hat gewortet: **`1-ui` gehört niemandem**; die starken Seats sind offen (praktisch über `river-ui` erreicht). Uniform bleiben `<line>-ui` (JIT) + `open-weight-ui`.
- **Blockade:** keine.
- **Braucht:** keine Schließung — `1-ui` als geteilte Starke-Seat-Gruppe führen; Myceliums „nach Mountain schließen" ist überholt.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md` (see-also auf Archiv-Pfad; heilt `ci-gate`/`register` `path_reference_scan`)
- `docs/handover/handover-2026-10-07-river-folge124.md` (neu)
- `docs/handover/archiv/handover-2026-10-07-river-folge123.md` (Move)

## Burn: open 0.0000 · close 0.0205 (line, deepseek-flash, gemessen `session_burn`) · zwei grind-flash-Dispatches (`path_reference_scan`-Verifikation, `add_stars`-Re-Init-Grabung) · Grund: River 124 — Line-Session, ein Pass.
