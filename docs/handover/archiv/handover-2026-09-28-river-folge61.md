<!--
  title: Handover — River-Folge 61 (2026-09-28)
  session: River-Folge 61
  class: handover
  date: 2026-09-28
  sha256: 5c57d61c218bb03c2590068c750138dddd17747f31040306f5d4a6af94e39cf3
  status: live
-->
# Handover — River-Folge 61 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert:
`state/zustand/standing-pass.md`. Nur eigene Arbeit: pfad-begrenzter Commit;
fremde uncommittete Arbeit unangetastet (in dieser Session concurrent:
`src/archivar/bsp_reader/daf.rs`, `src/archivar/pds3_*`, `src/archivar/pds4_binary.rs`,
`tools/register/src/bin/register_lookup.rs`, `AGENTS.md`, `.opencode/command/mycelium.md`,
`phi/blocked_sources.φ`, `phi/harvest.φ`, die laufende Handover-Rotation
mountain/mycelium/sensory).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md`" | 2026-09-27 | Operator (Session, River 49)
„du sollst keine operator worte tragen das ist sache von future du bist bis zur kante" | 2026-09-27 | Operator (Session, River 48)
„die ttl muss die Aktualisierung der Quelle sein" | 2026-09-27 | Operator (Session, River 47)
„#body erzeugt das bias … komplett rückgängig" — kein Körper privilegiert | 2026-09-27 | Operator (Session, River 47)
„frag den rat" / „folge dem rat" | 2026-09-27 | Operator (Session, River 47)
GIC-Paper-Einreichung: höchste Priorität | 2026-09-27 | Operator-Wort
Geräte-Zugriff: vor jedem Zugriff fragen (adb/BT) | 2026-09-26 | Operator-Wort
Harte-Läufe-LOCK aufgehoben | 2026-09-26 | Operator-Wort
HTTPS ja | 2026-09-26 | Operator-Wort folge36
Entscheidungen nie als Liste vorlegen — jede braucht eine Erklärung | 2026-09-27 | Operator (Future-Session)
„die Kante bin ich" — Wert, Wort, Dritt-Akt und Send bleiben seine Hand | 2026-09-27 | Operator (Future-Session)
ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben | 2026-09-27 | Operator (Future-Session)
`/consent` — session-weiter Delegations-Consent, nicht das Commit-Wort | 2026-09-27 | session-weiter Consent (`/consent`)
Commit-Wort (`/commit`) — pfad-begrenzter Commit + Push, das Doppel-Ask | 2026-09-27 | Operator
„hast du alles bis zur kante gemessen und geplant?" | 2026-09-28 | Operator (Session, River 61)
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt) … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-28 | Operator (Session, River 61) — session-weiter Delegations-Consent

## Offen (aufgeschlüsselt)

### TE-Horizont-Fix — CI-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`- und `te-gate`-Lauf auf dem neuen HEAD schließt ab.
- **Lage:** (gemessen 2026-09-28 River 61, `git diff`) der Fix gegen die zwei roten
  Gates ist gebaut: `find_cross_mi_lag` (`te.rs:2022`, mit Gate-Test
  `:4392-4464`), `topological_te_*` messen am Kreuz-Horizont τc,
  `TopologicalVerdict.tau_c` (`te.rs:3078`) sichtbar, `conditional_embedded_te_phase`
  10→100 z-Surrogate, `transfer_entropy_embedded_ksg_conditional` `k_eff=fit` für
  dim_z>0, `topological_te_estimate_frozen` ohne toten `find_mi_lag`-Zwang.
  `cargo check` + `cargo check --tests` 0/0. Rat: **wahre Ursache, kein Fudge**;
  none assert/seed/threshold touched.
- **Blockade:** keine.
- **Braucht:** nach dem Dispatch `ci_manage log <id>`; bleibt das Symmetrie-Gate rot,
  ist DAS der gemessene Riss (der konditionale Schätzer hat keinen FP-Sweep über
  Seeds) — nennen, nie fudgen.

### WGSL-Naht — `te_compute` Horizont
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` auf dem neuen HEAD grün (dann ohne äußeren Anlass dispatchbar).
- **Lage:** (gemessen 2026-09-28, Rat Q3) die CPU-Membran misst am Kreuz-Horizont τc,
  die WGSL `te_compute` (`shaders.rs:751/766`) weiter an (tx,ty); Produktion trägt
  `TE_KSG_K_PROD=0` (GPU-KSG-Slots absent), die Roh-Paritäts-Gates halten.
- **Blockade:** keine.
- **Braucht:** `find_cross_mi_lag` in `te_compute` spiegeln + Horizont-Parity-Gate.

### ENSO TE-Probe
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** eine registrierte ENSO-SST-Zeitreihe + der Blatt-Zuschnitt.
- **Lage:** (gemessen 2026-09-28 River 61, grind-flash) kein ENSO-Bin; `phi/sources.φ`
  führt keine ENSO-SST-Serie — `esacci_sst` ist ein Eintages-Grid (`:1428/:1434`,
  Compiler verlangt genau ein `--date`), TAO-SST trägt nur die letzte Tabellenzeile
  (`:1337/:1340`); `sgrep -i enso phi/` → keine Serien-Zeile.
- **Blockade:** der SST-Serienkanal fehlt; der Paar-/Zuschnitt ist operator-gebunden
  (`state/zustand/wartend.φ:23` `blatt-zuschnitt | Operator`, Aufnehmer river).
- **Braucht:** Quellen-Zeile einer ENSO-SST-Serie + Operator-Zuschnitt.

### Flyby-Path-2 — Füll-Lauf
- **Status:** termin | **Bindung:** termin:2026-09-29
- **Trigger:** 29.09. 00:00 UTC Snapshot.
- **Lage:** (gemessen 2026-09-28) 28.09-Snapshot success.
- **Blockade:** 29.09.-Snapshot fehlt.
- **Braucht:** nach 29.09. 00:00 UTC `gh workflow run flyby-path2-fill.yml`; Artefakt `flyby-path2-fill`.

### flyby-path2-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ESA/ESOC-Publikation der Post-Flyby-SPK + 1σ (`state/zustand/wartend.φ` `flyby-path2-recon`).
- **Lage:** (gemessen 2026-09-28) `data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin`
  absent; Gate `pending`.
- **Blockade:** Publikation fehlt.
- **Braucht:** `flyby_ephemeris_gate --recon <recon.bin> --sigma-recon <km>`.

## An mountain
Origin: river folge61.

- **`## An river` (mountain-folge199) — der ganze Block ist am Baum erledigt**
  (gemessen River 61): **Membran-Riss fixe-Tabellen** gebaut (`series_field_name`
  `main_flow.rs:5399`, Arme `:2871-2872`, Join `:2979`); **ω-Loop-Verdict-Term**
  geschlossen (`survey-2026-09-26-membran-ladearchitektur.md:222`, gebaut `8748a39cd`);
  **Jump-Detektion** geschlossen (survey `:214`, zweiseitiges Segment-Residuum);
  **Orphan-Träger `kybernetische-astrophysik.md:423`** — der Stand-Pass-Orphan-Zensus
  (12) nennt das Doc nicht. Der Block kann beim mountain-Pass entfernt werden.
- **orphan-doc-Träger** (mountain-Natur): `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md`
  (3 offene Marker, kein Träger — gemessen 2026-09-28 via `register_lookup --orphan-docs`;
  die zwölf früheren sind geroutet, `5be1056dd`).
- **81-Block-Fix** — `format ephemeris_binary`, `ttl` aus Live-Release-Abständen.

## An mycelium
Origin: river folge61.

- **Rätsel Ⅶ — Doc-Riss, am Baum widerlegt** (gemessen River 61, grind-pro): der
  Claim „`max(0,|Δt|−d/c)`-Fold nicht in `src/`" ist ein **false report** — der Fold
  lebt als `(age − d/v_prop).max(0.0)` in `src/archivar/spatial.rs:549-550`
  (`age` `:494` = |Δt|, `v_prop` → `C_LIGHT`). `survey-raetsel-bestand.md:34` führt
  `spatial.rs:549-554` in derselben Zeile unter „Kanäle vorhanden" — Selbstwiderspruch.
- **Name-Riss:** `signalkegel_audit_probe` stale in
  `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md:51/:77`; das Bin heißt
  `signal_cone_audit_probe`.
- **Rätsel Ⅸ — FRB, gemessen:** `frb_blatt_probe.rs:6` `CDN_RELEASE` von
  `ssd.jpl.nasa.gov` auf den registrierten Tag `cdsarc.cds.unistra.fr` geheilt
  (`frb_compiler.rs:455`, `phi/sources.φ:9077`); die Probe lädt 600 Bursts über den
  CDN-Fallback und urteilt **kein Pfeil** (FRB20180916B n=32, TE 3.28e-2 < thr 1.46e-1)
  → kein Blatt (0 honored). Die Survey-Zelle „kein Paar-Artefakt" ist damit gemessen,
  nicht offen.
- **matrix-rotor `36436173707`** Runner-Preemption — `.github/workflows/matrix-rotor.yml`
  job-level concurrency prüfen.
- **dropped-gate-Umbau** — selbst-messend/träger-bewusst statt handgepflegter Absolutwert.
- **flyby-odf-cdn `36414284741`** rot — `odf-persist` ohne `--file` → 0-Byte census;
  Fix `--file src/archivar/kernels/odf07155.dat`, dann dispatch.
- **`devcontainer`-CLI** in einem Actions-Job (Umgebungs-Parität).
- **Juice-Kernel** `data/esa/juice_cog_000114_…_261003_v01.bsp` (Vorhersage) ist
  unregistriert; die registrierte `juice_cog_000112_…_260919` (`phi/sources.φ:7151`)
  endet vor dem Perigäum.

## An future
Origin: river folge61.

- **`## An river` (future-folge149):** format rot `a_posteriori_placement_probe.rs:162/:244`
  ist gefixt (river folge59 `c874dc587`; `ci-gate 36454281741` job `format` = success).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort. Pfad-begrenzte
Commit-Pfade dieser Session:

`src/mathematikerin/te.rs` · `src/mathematikerin/ksg_k.rs` ·
`src/mathematikerin/omega.rs` · `src/mathematikerin/machines/solar.rs` ·
`src/mathematikerin/tests.rs` · `tools/measure/src/bin/frb_blatt_probe.rs` ·
`docs/handover/handover-2026-09-28-river-folge61.md` ·
`docs/handover/archiv/handover-2026-09-28-river-folge60.md`.
