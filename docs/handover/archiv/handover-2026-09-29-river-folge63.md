<!--
  title: Handover — River-Folge 63 (2026-09-29)
  session: River-Folge 63
  class: handover
  date: 2026-09-29
  sha256: ac3f3131486facfd0c7e0fe9ca91426ba6c359ae6126c0283c0a96cea660b565
  status: live
-->
# Handover — River-Folge 63 (2026-09-29)

Dieses Register trägt nur Offenes — git trägt, was gemacht wurde. Der Stehende Pass
wird zitiert, nie kopiert: `state/zustand/standing-pass.md`. Nur eigene Arbeit:
pfad-begrenzter Commit; fremde uncommittete Arbeit unangetastet (in dieser Session
concurrent: archivar `extract.rs`/`geo.rs`/`main_flow.rs`/`tests.rs`, `src/archivar/hips.rs`,
`tools/register/src/bin/register_lookup.rs`, `phi/sources.φ` (ersstv5-Zeile `:11111`,
`juice_cog` `:7151`), der ENSO-Compiler und `hips_png_compiler.rs`, laufende
Handover-Rotation mountain/mycelium/sensory).

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
„hast du alle eigenen punkte bis zur kante geplant?" | 2026-09-29 | Operator (Session, River 63)
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt) … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-29 | Operator (Session, River 63) — session-weiter Delegations-Consent

## Offen (aufgeschlüsselt)

### TE-Merge — CI-Verifikation (τ_c-Rename geheilt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`/`ci-gate`-Lauf auf dem HEAD nach diesem Commit schließt ab.
- **Lage:** (gemessen 2026-09-29 River 63) River 61s `tau_x`→`tau_c`-Rename in
  `TopologicalVerdict` (`te.rs:3093`) hatte 11 Measure-Bins gebrochen; geheilt:
  10 `.tau_x`→`.tau_c`-Printer/Nutzer (`pioneer_residue_diagnose`, `bison_cycle`,
  `wso_cycle`, `gong_cycle`, `hmi_cycle`, `bison_basu`, `mitdb_te`, `pioneer_drift_te`,
  `pioneer_link_correction`) und 2 `topological_te_estimate_frozen`-Aufrufer
  (`hyperscanning_group_te.rs:308/:1110/:1745`, 6. Argument = `find_cross_mi_lag`).
  Test-Tie-Prämisse war falsch (lag 4 MI=1,0 > lag 2 ≈0,999): `te.rs:4422` von
  `t%4` auf `t%2` korrigiert (echter Tie, first-max); `cargo check --tests -p
  omegaflow-measure` 0/0, `cargo check` 0/0.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <id>` am neuen HEAD; bleibt rot, den gemessenen Riss
  nennen, nie fudgen.

### WGSL-Naht — `te_compute` Horizont
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` auf dem neuen HEAD grün (dann ohne äußeren Anlass dispatchbar).
- **Lage:** (gemessen 2026-09-28, Rat Q3) die CPU-Membran misst am Kreuz-Horizont τc,
  die WGSL `te_compute` (`shaders.rs:751/766`) weiter an (tx,ty); Produktion trägt
  `TE_KSG_K_PROD=0` (GPU-KSG-Slots absent), die Roh-Paritäts-Gates halten.
- **Blockade:** keine.
- **Braucht:** `find_cross_mi_lag` in `te_compute` spiegeln + Horizont-Parity-Gate.

### ENSO TE-Probe
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** gefeuert (gemessen 2026-09-29 River 63) — `ersstv5_nino34` ist
  registriert (`phi/sources.φ:11111-11117`: `url` + `origin` ERDDAP `nceiErsstv5` +
  `compiler tools/harvest/src/bin/ersstv5_compiler.rs` + `field ersstv5_nino34_ssta`).
- **Lage:** (gemessen 2026-09-29 River 63 via `register_lookup ersstv5`) die
  Quellenzeile steht (uncommittet, Mountain/Mycelium in flight); der Compiler
  existiert als untracked Datei. Der Blatt-/Paar-Zuschnitt ist operator-gebunden
  (`state/zustand/wartend.φ:23` `blatt-zuschnitt | Operator`, Aufnehmer river).
- **Blockade:** CDN-Manifestation (Mycelium) + Operator-Zuschnitt des Paares.
- **Braucht:** Mycelium fährt `ersstv5_compiler --ci-mode` + Release-Asset; dann die
  Probe (OMNI `omni_hro_imf_bz_gsm_nt` × `ersstv5_nino34_ssta`) nach
  `nobel_probe_corona.rs` binden — der Zuschnitt kommt vor den Operator.

### Rätsel Ⅰ — Jeans-Engine, Ratsbeschluss (Modi ii–v)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** keine (nächster Schritt ist die Messung/der Bau).
- **Lage:** (gemessen 2026-09-29 River 63) Zensus-Modus gebaut und still gelaufen —
  `tools/measure/src/bin/jeans_residuum_probe.rs`: 1 704 587 Sterne, 274 156 belegte
  50-pc-Zellen, davon 8 246 (3,0 %) ≥ 32 Sterne; Median N_total je Zelle = 1; nur 408
  belegte Zellen (0,15 %) tragen einen Stern mit ϖ > 5 mas; Median der zellweisen
  Median-Parallaxe 0,529 mas (~1,9 kpc). **50 pc trägt den Schätzer im Allgemeinen nicht.**
- **Ratsbeschluss (2026-09-29, einzige Registerzeile des Rates):** ein Bin,
  drei Modi. Kanonisch ist die **vertikale 1D-Jean-Form über die Zwischengröße K_z**
  (`d(νσ_z²)/dz = −ν K_z`, dann `ρ_dyn = −(1/4πG)·dK_z/dz + Rotationsterm`), nicht
  sphärisch, nicht die Midplane-Näherung; **σ_z** aus der galaktischen
  Breitenkomponente (`μ_b → v_b = 4,74·μ_b/ϖ`), rv nur Kontrolllinie; **Fehlerkorrektur
  Pflicht** (`σ_z²(est) = σ_z²(obs) − ⟨σ²_err⟩`; σ_z²(obs) ≤ ⟨σ²_err⟩ → `absent`, nie
  0,0, kein Clipping — negatives R(V) am Midplane ist echt); **symmetrisierte
  |z|-Bins, ≥ 32 Sterne/Bin, zentrierte Differenzen**, alles `Option`/`absent`.
  Reihenfolge: Zensus → Compiler-Fehler-Atom (σ_ϖ/σ_pm im `dr3_stars`-Record, neue
  Asset-Version, CDN) → Schätzer → Fixture-Eichung + Literatur-Parität (ρ_dyn(0) ≈
  0,09–0,10 M☉/pc³, Holmberg & Flynn 2000 / McKee et al. 2015) → erst dann eine
  `ledger.φ`-Zeile mit Datenstufe im Namen. Dissens: Reihenfolge Schätzung ↔
  Fehler-Atom (Sensory: verfrüht), Rotationsterm als benannte Literaturkonstante (nicht
  als Messung).
- **Blockade:** keine; die Voxelgröße ist aus dem Zensus zu setzen (cell_size aus live
  data, nicht 50 pc aus Prosa).
- **Braucht:** Voxelgröße aus dem Zensus setzen; Compiler-Fehler-Atom (Mountain Asset +
  Mycelium Manifestation); dann Schätzer-Modus nach obigem Beschluss.

### flyby-path2-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ESOC publiziert einen SKD `v474+` mit `juice_cog_000115_…` oder einer
  dedizierten `*recon*`-SPK in `spiftp.esac.esa.int/data/SPICE/JUICE/kernels/spk/`
  (Enumeration via `sfetch --links` zeigt Version > 000114 oder `*recon*`).
- **Lage:** (gemessen 2026-09-29 River 62, general) **ABSENT**: Enumeration endet bei
  `juice_cog_000114_230416_261003_v01.bsp`; `data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin`
  fehlt; Gate `data/flyby2/gate-juice-2026-09-28.json` recon/sigma/verdict `pending`,
  `riss: false`. Der `juice_cog`-Register-Riss ist geheilt (`phi/sources.φ:7151` trägt
  jetzt `…000114…v01.bsp`).
- **Blockade:** Publikation fehlt.
- **Braucht:** nach Publikation `flyby_ephemeris_gate --recon
  data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin --sigma-recon <km>`.

## An mycelium
Origin: river folge63.

- **ENSO-Manifestation:** die `ersstv5_nino34`-Quellenzeile steht (uncommittet,
  `phi/sources.φ:11111`); sobald committet, `ersstv5_compiler --ci-mode` fahren und das
  Release-Asset `coastwatch.pfeg.noaa.gov/ersstv5_nino34.bin` manifestieren.

## An mountain
Origin: river folge63.

- **Rätsel Ⅰ Compiler-Fehler-Atom:** der `dr3_stars`-Record (44 B) trägt keine
  σ_ϖ/σ_pm-Spalten; der Zensus misst, dass ohne sie jede σ_z-Schätzung eine
  Rauschmessung ist (Median-Parallaxe 0,529 mas). Braucht: erweiterter Record + neue
  Asset-Version (CDN), als getrennte Feder.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort. Pfad-begrenzte
Commit-Pfade dieser Session:

`src/mathematikerin/te.rs` ·
`tools/measure/src/bin/a_posteriori_placement_probe.rs` ·
`tools/measure/src/bin/jeans_residuum_probe.rs` ·
`tools/measure/src/bin/pioneer_residue_diagnose.rs` ·
`tools/measure/src/bin/bison_cycle_probe.rs` ·
`tools/measure/src/bin/wso_cycle_probe.rs` ·
`tools/measure/src/bin/gong_cycle_probe.rs` ·
`tools/measure/src/bin/hmi_cycle_probe.rs` ·
`tools/measure/src/bin/bison_basu_probe.rs` ·
`tools/measure/src/bin/mitdb_te_probe.rs` ·
`tools/measure/src/bin/pioneer_drift_te_probe.rs` ·
`tools/measure/src/bin/pioneer_link_correction_probe.rs` ·
`tools/measure/src/bin/hyperscanning_group_te.rs` ·
`docs/paper/laic-arrow-direction.md` ·
`docs/paper/signal-cone-audit-sheet.md` ·
`docs/surveys/survey-messpunkt-verteilung.md` ·
`docs/surveys/survey-2026-09-28-raetsel-zensus.md` ·
`docs/surveys/survey-raetsel-bestand.md` ·
`docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` ·
`docs/handover/handover-2026-09-29-river-folge63.md` ·
`docs/handover/archiv/handover-2026-09-29-river-folge62.md`.
