<!--
  title: Handover — River-Folge 126 (2026-10-07)
  session: River-Folge 126
  class: handover
  date: 2026-10-07
  sha256: 19ae921b06f417fa3486c802b6c069a49cd86491255ce41bd38ef2c266ee3f91
  status: live
-->
# Handover — River-Folge 126 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„bitte delegiere und arbeite deine punkte bis zur kante ab wenn du rat brauchst schicke recherche tucher los und befrage die frontier chats" | 2026-10-07 | Operator (Session, River 126) — Arbeit bis zur Kante; Rat → Recherche + Frontier-Chats
„was soll das mach einefach neue tabs auf wenn welche belegt sind" | 2026-10-07 | Operator (Session, River 126) — UI-Konsultation in neuen Tabs, nicht in belegten Chats
„bitte delegiere für die arbeit, recherche, und UI chats" | 2026-10-07 | Operator (Session, River 125) — Dispatch-/Delegations-Consent
„warum nur ein ui seat? hast du gesehen wie viele tabs offen sind?" | 2026-10-07 | Operator (Session, River 125) — **alle** offenen UI-Seats nutzen, nicht einen
„…das ist wichtig wir haben mächtige stimmen 1-ui gehört niemandem" | 2026-10-07 | Operator (Session, River 123) — `1-ui` ist **nicht** linien-fremd
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-river-folge125.md` §Operator-Wort-Register — gefaltet, nicht kopiert. Verbatim-Quelle: `state/operator-gespraeche/2026-10-07-river.md`.

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
- **Lage:** (gemessen 2026-10-07 via `ci_manage view 37621964105`) der Re-Harvest-Lauf steht weiter `queued` (self-hosted; erstellt 12:34Z), `start=20031029`, `days=3`, `force=true`; `phi/sources.φ:18034` führt unverändert `sha256 9c76f881…`. `fmi_gic.bin` endet 2023-10-01 UTC → 0 Überlapp mit dem alten 30-Tage-Fenster; die Relation wurde auf dem Halloween-Sturm 2003-10-29…31 gemessen: 72 h, Peak 240.9 nT/10s / 57.05 A, **r = 0.9448**, OLS 0.2118·|ΔX/10s| + 1.755 (Paper §4.7).
- **Blockade:** das manifestierte Asset-Fenster (Mycelium-Ernte/CI-Queue).
- **Braucht:** Lauf-Ausgang + neuer sha in `phi/sources.φ:18030`; dann `cargo run -p omegaflow-measure --bin nur_gic_relation_probe` auf dem Asset selbst.

### GIC-Familien — Route C (Stufe 2); Generator + Kanal-Listen gebaut; Stufe-2-Pool wartet auf dB/dt-Bestand
- **Status:** wartend (Mountain-dB/dt) | **Bindung:** eigen (cross-line mountain)
- **Trigger:** per-Station-dB/dt-Netz (Mountain) bzw. Operator-/Rats-Wort für den Stufe-2-Lever.
- **Lage:** (gemessen 2026-10-07, River 121/122; unverändert) **Route C:** Stufe 1 global; die Familie allein in Stufe 2 als Member-Pool des `compute_max_t` (`field_te_query.rs:2784`), abgeleitet zur Abfragezeit aus `cgm_lat`; Gruppierung Target-Band. Stufe-1 byte-identisch. Risse: (1) unvollständiger Pool → Null provisorisch, Pool-Version im Ergebnis, fehlende Station `pending`; (2) Null nie über Stufe-1-Überlebende; (4) nur ABK 1h/1m + SOD 1h tragen `field intermagnet_dbdt` (`phi/sources.φ:2051-2079`), die 154 GIN-Blöcke tragen `intermagnet_xyz_x/y/z_nt`. Gebaut: `tools/measure/src/bin/cgm_lat_partition.rs` → `state/river/gic-family-*.txt` (154 = 31+25+98, disjunkt). CGM-Provenienz (CPL/TTB) auf der Partitionsseite geschlossen.
- **Blockade:** per-Station-dB/dt-Netz fehlt (Mountain, Quellen-Eigenschaft).
- **Braucht:** Mountain: 154 per-Station-dB/dt-Kanäle; danach Familien-Pool in `compute_max_t` (Stufe 2).

### Receiver-Apertur — Sub-Pixel für ALLE Radiatoren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Direktive `span` auf der `at <body>`-Zeile (Mountain).
- **Lage:** (gemessen 2026-10-07 via `grind-flash` + Baum) die entschiedene SPAN/Apertur-Architektur ist **nicht im Code**: `span_m` existiert als Symbol nicht; `Aperture` (`src/archivar/types.rs:291`) ist ein unverwandtes Enum `{None,Flux}`; `N = state.receptors = Math.min(canvas.width, canvas.height)` (`static/membrane.html:74,404`) ist ein Viewport-Ausdruck ohne deklarierten Wert; das alte `VIEW_SPAN_M/400` ist gestrichen und im Code absent.
- **Blockade:** großer Umbau (per-Fragment `source_contrib`) + fehlende `span`-Direktive.
- **Braucht:** `span`-Direktive (Mountain); Deklaration des numerischen N/Apertur-Mappings; danach Brücke in `static/membrane.html` + Invarianz-/Energieerhaltungs-Test.

### `ozzy` — Negative Fuzzy Engine (CPU-Floor + GPU-Wire gebaut; CI-Verifikation offen)
- **Status:** wartend (CI) | **Bindung:** eigen
- **Trigger:** `ci-check`/`ci-gate` grün am jeweiligen HEAD.
- **Lage:** (gemessen 2026-10-07, River 126 via `ci_manage status`) HEAD `14ae2fa56`; `ci-check 37626937396` `pending`; die vorherigen `ci-check`-Läufe `cancelled` (Concurrency). Gebaut: `independence_verdict` (`ozzy.rs:146`) mit A-Test + B-Diagnose + Known-Answer-Gates; `TE_SURR_FLOOR = 99` (`te.rs:3450`); GPU-Wire `TE_SERIES_COUNT = 101` + WGSL-Parität.
- **Blockade:** keine (eigene); CI-Runner/Queue.
- **Braucht:** grüner `ci-check`/`ci-gate` am HEAD (Stehender Pass).

### em-Apertur — Kanal-Identität statt Kernel-Proxy
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am HEAD.
- **Lage:** (gemessen 2026-10-07, River 122) River-/Mountain-Seite gebaut (`shaders.rs:186`/`:211`); CI am HEAD `pending`.
- **Blockade:** keine (eigene); CI-Runner/Queue.
- **Braucht:** grüner `ci-check` am HEAD (Stehender Pass).

### Membran-Startansicht — zwei Aperturen (per-Klasse-Belichtung gebaut; CI-/Browser-Verifikation offen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`/`wasm-parity` grün; danach Pages-Deploy + Browser-Sicht.
- **Lage:** (gemessen 2026-10-07, River 126) der zuvor offene „`state.lvl` global über beide Aperturen" ist **geheilt**: `static/membrane.html` relaxiert die Belichtung je Apertur (`state.lvl` Anker / `state.lvl_star` Sterne), Diskriminator derselbe deklarierte `extent > 0.0` wie die Skala; die `VP`-Uniform trägt ein fünftes `levels`-vec4 (`vpBuf` 80 B). **Rat (fünf Stimmen) + vier UI-Seats (DeepSeek Chat, MiniMax M3, Claude Sonnet 5.5, Gemini 3.1 Pro) einig: per-class** — ein globales max ließe einen hellen Stern die Sonne dimmen; die absolute Relation bleibt im Record. JS-Syntax `node --check` grün. Der Parity-Fix `116611e2e` (river 114) ist unverändert; `add_stars` ohne Panik-Pfad (`src/wasm.rs:46-52`).
- **Blockade:** CI-Lauf-Ausgang (Stehender Pass/`ci_manage`, kein Polling); Pages-Deploy.
- **Braucht:** CI-grün; Pages-Deploy; Browser-Sicht auf `omegaflow.space/membrane.html`.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon). Wahrheit: `state/zustand/wartend.φ` (`flyby-chain-omni2`, `flyby-chain-kp-def`, `ephemeris-juice-recon`).
- **Lage:** (gemessen 2026-10-06, River 105) OMNI2 26 Zellen `pending`; kp `def` leer; JUICE-recon absent (Wiedervorlage 2026-11-01).
- **Blockade:** externe Kanäle; kein Polling.
- **Braucht:** `flyby_path2_fill`-Lauf lesen + Addendum fortschreiben; Trigger feuern lassen; Δ/σ_recon.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `static/membrane.html` (per-Klasse-Belichtung, eigener Hunk)
- `docs/handover/handover-2026-10-07-river-folge126.md` (neu)
- `docs/handover/archiv/handover-2026-10-07-river-folge125.md` (Move)

## Burn: open 0.0000 · close 0.0721 (line, deepseek-flash, gemessen `session_burn`) · Dispatches: `general` (Zwei-Apertur-Exposure-Recherche), `council` (Rat: lvl per Apertur → Option b), UI-Seats DeepSeek/MiniMax/Claude/Gemini per-class; Duck.ai/GLM generierten, Qwen überlastet, Kimi-Prompt verrutscht (gemessener Stand) · Grund: River 126 — ein Pass.
