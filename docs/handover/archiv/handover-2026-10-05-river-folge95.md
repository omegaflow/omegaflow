<!--
  title: Handover — River-Folge 95 (2026-10-05)
  session: River-Folge 95
  class: handover
  date: 2026-10-05
  sha256: f1f680128f54e871a12e2aabce747f9b49569e840f821a222cc7cca8de16e04d
  status: live
-->
# Handover — River-Folge 95 (2026-10-05)

Dieses Register trägt nur Offenes — git trägt, was gemacht wurde. Der Stehende Pass
wird zitiert, nie kopiert: `state/zustand/standing-pass.md`.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die River-Linie **in einem Pass** …" + „River besitzt die Membran-Pfade …" | 2026-10-05 | Operator (Session, River 95) — session-weiter Delegations-Consent, nicht das Commit-Wort
(Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-05-river-folge94.md` §Operator-Wort-Register — gefaltet, nicht kopiert.)

## Träger (Prosa, eigene)

- `docs/paper/gic-causal-driver.md` (`class: paper`) — §6 trägt die kalibrierte
  Westfall–Young-max-T-Null (α = 0.05/0.01). Offen: Verdrahtung der Bias-Korrektur
  (`TE_NEFF_THRESHOLD`), BCa-Intervalle, vollständiger Kp-Kanal.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`class: sheet`, `status: unsealed`) —
  offen bis zum Siegel: X, Z-Fenster, Bz-Schwelle; α-Ebene + Siegel = Operator-Wort.
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (`class: survey`) —
  nächste Schritte §5.
- `docs/auftrag/auftrag-universelles-vlies.md` (`class: auftrag`) — Offen: `ozzy`-Bau,
  Paar-Matrix, Ernte (§Lieferung).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (`class: survey`) — §7 geschlossen.
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` / `docs/auftrag/auftrag-flyby2-kette.md` —
  Offen: OMNI2 26 Zellen, ACE 3/14/16, kp `def`, Δ/σ_recon.
- `docs/concepts/exzellenz-konzept.md` (`class: concept`, `version: 1`) — Prüfmaßstab.

## Offen (aufgeschlüsselt)

### Membrane — First Light (Reparatur gebaut, Deploy-Verifikation offen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `pages-deploy`-Lauf nach dem Push von `static/membrane.html`.
- **Lage:** (gemessen 2026-10-05 via CDP an Operator-Chrome) `https://omegaflow.space/membrane.html`
  bleibt auf „anchoring bodies…": alle Assets HTTP 200 (dr3_stars
  95 424 168 B, je Ephemeride 6 629 784 B), WebGPU-Gerät ok, Shader/Pipeline/BindGroup
  gebaut, `requestAnimationFrame(loop)` **scheduled, aber der Callback läuft nie**
  (`__cbruns = 0` — ein occludierter/Background-Tab suspendiert rAF). Ein manueller
  `loop()`-Aufruf rendert `stars 101` (die Körper-Anker sun/earth/moon sind enthalten,
  schwarzes Feld korrekt vermieden); die erste `lookup.query` baut den Präsenz-Hull-Hash
  (~15 s, danach 1–200 ms/Frame). Ursache: rAF-Gate vor dem ersten Feld + Status noch
  „anchoring bodies…" während der synchronen ersten Query. Fix in `static/membrane.html`
  (HEAD-Vorarbeit River 91): erste Feldmessung direkt nach `initGPU()` statt über rAF,
  Status „measuring the first field…" vor der ersten Query; `node --check` 0 Fehler.
- **Blockade:** keine.
- **Braucht:** nach dem Push `pages-deploy` beobachten (`ci_manage list`),
  `https://omegaflow.space/membrane.html` im Vordergrund öffnen und den Status
  `stars N · scale … · t … s TDB` sehen (Sonne als heller Anker am Ursprung). Der
  95-MB-Gaia-Vollkatalog + Hash-Bau (~30 s bis First Light) bleibt als offener
  Performance-Punkt zu benennen — eine Presence-Hull-only-Ladung im WASM wäre die
  Architektur-Frage (Rat).

### em-Apertur — Kernel-Zensus (gemessen; Riss)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner.
- **Lage:** 25 z-tragende `em`-Feldzeilen in 18 Blöcken (gemessen 2026-10-05 via
  grind-flash-Zensus `awk` über `phi/sources.φ`). 3 Koordinaten-Träger
  (`ned_redshift_z`, `ned_byparams_redshift_z`, `ned_gitter_redshift_z`) — im
  Working Tree bereits `inverse-linear` (Kernel 6), am HEAD noch
  `gaussian-inverse-square` (Kernel 1); 22 fluss-Kernel (0/1). Apertur-Gate
  `kernel_id ∈ {0,1}` in `src/archivar/spatial.rs:760-768`, GPU
  `src/mathematikerin/shaders.rs:182-185/204-207`. **Riss:** der Kernel ist ein
  Proxy, nicht die Wertklasse — `magnetar_period_s`, `planck_psz2_snr`, `frb_dm_em`,
  `frb_dispersion`, `frb_scatter_ms`, `sn_max_mag`, `sdss_photoobj_psfmag_{r,g,i}`,
  `bat_fluence_erg_cm2` sind nicht fluss-wertig, bekämen aber `(1+z)⁻²`.
- **Blockade:** `phi/sources.φ` ist uncommittet von einer Parallel-Session berührt
  (Zeilennummern driften) — kein Schreibakt von River dort.
- **Braucht:** Rat-Verdikt: Gate auf `kernel_id` oder auf die gemessene Wertklasse
  (Kanal-Identität statt Kernel-Proxy); danach CI-Test (Kernel 6, z=1 → unskaliert;
  Kernel 1, z=1 → 0.25; CPU/GPU-Parität) — CI, nie lokal.

### EMM Frame-Bundle-Arm (`emm_exi_l2a`)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner.
- **Lage:** kein `emm`/`exi`-Zweig in `src/archivar/main_flow.rs` (gemessen 2026-10-05
  via explore); die Quelle `phi/sources.φ:9039-9042` (`format emm_exi_l2a`, `at mars`,
  `ttl 604800`) fällt durch die ganze Dispatch-Kette; der Workflow `emm-sdc-cdn.yml`
  dispatcht `emm_sdc_compiler --instrument exi --data-level l2a` (Asset grün,
  `sha256 6f379edf…`). Nächstes Template: die `drs_fits`-Gruppe `main_flow.rs:2885`
  (Parser `series_named`/`series_rows` `:3015-3017`).
- **Blockade:** die Quelle trägt noch keine `field`-Zeilen (Register-Seite) und
  `phi/sources.φ` ist von einer Parallel-Session berührt.
- **Braucht:** (1) das Output-Format von `tools/harvest/src/bin/emm_sdc_compiler.rs`
  lesen; (2) `match`-Arm + Parser/Abbildung in `main_flow.rs` nach `drs_fits`-Muster;
  (3) `field`-Zeilen in `phi/sources.φ` (Mountain, Aufenthalt = Eigentum);
  `cargo check` 0/0.

### ADVECTIVE_BASE_SPEED / Presence-Bit (Rat 2026-10-05)
- **Status:** eigen | **Bindung:** eigen (cross-line: Archivar-Writer)
- **Trigger:** keiner.
- **Lage:** (gemessen 2026-10-05 via `git status`) `flat_propagation_speed`
  (`src/archivar/membrane.rs:488-492`) gibt bei
  `advection == 0.0` `Some(ADVECTIVE_BASE_SPEED = 1.0)` statt absent; der
  `presence`-Slot (25 / `meta[14]`) trägt heute nur das Phase-Bit. Die
  Archivar-Seite (Presence-Bit-Writer in `parse.rs`/`types.rs`/`membrane.rs`/
  `units.rs`) liegt uncommittet im geteilten Working Tree. Test
  `src/archivar/membrane.rs:765-768` asserted noch `Some(ADVECTIVE_BASE_SPEED)`.
- **Blockade:** Writer (Register/parse/pack) muss Bit 2 setzen; ohne ihn liest ein
  Reader ohne Bit für alle Datensätze `absent`.
- **Braucht:** Writer landet; dann Bit-2-Lesung in `shaders.rs`/`membrane.rs` (die
  `1.0`-Substitution fällt, abwesende Advektion = kein Term, gemessene 0 = Speed 0);
  Test `:765-768` anpassen; CPU/GPU-Parität. `docs/concepts/archivar-mathematikerin.md`
  liegt dafür bereits modifiziert vor (Parallel-Session).

### dB/dt–GIC-Relation Mäntsälä
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** fine-grain `fmi_gic`-Asset + NUR-Asset im CDN.
- **Lage:** (gemessen 2026-10-05) FMI-GIC CC BY 4.0, 1999–2023, Halt 2023-10-23,
  beste Qualität 1999–April 2005, nicht-uniform; NUR nur als SuperMAG-Station.
- **Blockade:** die zwei Harvests (Mountain/Mycelium) + neuer Probe-Bin.
- **Braucht:** (1) `fmi_gic` fine-grain registriert+manifestiert; (2) NUR im CDN;
  (3) Probe dB/dt(NUR)–GIC(Mäntsälä) + Zahl in Paper §4/§6.

### Universelles Vlies — `ozzy` + Alles-gegen-alles-Matrix
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner (arbeitbar bis zur Rats-Kante).
- **Lage:** (gemessen 2026-10-05 via `fd -i ozzy` = leer, `sgrep -i ozzy` nur Spec)
  `ozzy` ist Spec, keine Quelldatei; `field_te_query` fährt Einzel-Paare. Der
  universelle Rahmen steht (ICRS baryzentrisch, DE/INPOP/EPM, Gaia DR3, 2MRS).
- **Blockade:** keine (Rat-Verdikt 2026-10-05 liegt; Matrix-Form steht am Baum).
- **Braucht:** (Reihenfolge) (1) Bias-Kurve über n messen (`te_bias_n_probe`
  erweitern) + `TE_BIAS_MK_EMBEDDED` → (2) die 8 probe-gelesenen Kanäle an den Draht +
  15×15-Lauf (`matrix full`, `fdr bh 0.05 over matrix`) → (3) `ozzy`-Bibliothek;
  (4) Netz-Null-CI (B ≥ 1/α). Ernte parallel (Mountain/Mycelium/Future) — nicht Rivers
  Hand. **Ozzy läuft auf der Matrix, nicht davor.**

## An mountain (Feld-Gesetz + Register)

Origin: river folge95.

- **Enclosure-Hülle — Konservativität (Riss).** `law_bounds` (`spatial.rs:243-274`)
  liefert je Sample `Φ·(v_instant + resid_ema)`; der Spannen-Hull-Fix
  (`law_bounds_over_span` `spatial.rs:276-321`, `build_asteroid_samples` `:449-451`,
  Test `tests.rs:3336`/`:3475`) liegt uncommittet im geteilten Working Tree — bitte
  landen; der Stern-Pfad (`spatial.rs:665`) bleibt bei `law_bounds`.
- **absorption-/advection-Slots.** `absorption = 0.0` bei allen field-erzeugenden
  Direktiven → Beer-Lambert inaktiv, patch-levy-Tail (kernel_id 5) unerreichbar;
  `first`-Direktive `phi/sources.φ:181` trägt `advection = 400000.0`. **Braucht:**
  Entscheidung — bleibt `absorption` überall 0.0; ist `ADVECTIVE_BASE_SPEED = 1.0`
  gewollt? (Der Rat-Entscheid zum Presence-Bit berührt dies.)
- **em-Apertur — 3 Koordinaten-Träger.** `ned_redshift_z`/`ned_byparams_redshift_z`/
  `ned_gitter_redshift_z` sind im Working Tree bereits `inverse-linear` (Kernel 6),
  am HEAD noch Kernel 1 — Register-Edit landen.
- **Neue Quellen-Zeilen (Auswertung §5, Taucher-gemessen).** UHSLC, IOC-SLSM,
  Madrigal, EIDA-Routing, MTG-LI als `sources.φ`-Kandidaten; GOES-18-Bucket um 16/19
  ergänzen.
- **Nicht-point-event-Zeugen + Probes-Wanderung:** abgeleitete `axis value`-Serien
  für `gbco`/`gmrt`/`rixs`; `solar_causal_graph`/`signal_cone_audit`/`laic`/
  `trishuli_gauge` als Source/Field führen oder benannt descopen.
- **iEEG-Elektroden:** Harvest-Arm `ieeg_edf` steht; Elektroden-Koordinaten +
  Kanal-Matrix registrieren/ableiten.
- **FMI-GIC fine-grain-Zeile:** `phi/sources.φ` Stunden-Peaks; fine-grain-Assetname +
  Bucket nennen.

## An mycelium (Transport)

Origin: river folge95.

- **B-Membran `continue-on-error: true`:** Verdikt von Mycelium folge233 übernommen —
  der wasm-Build bleibt `continue-on-error: true`, `pages-deploy.yml:39` unverändert.
- **FMI-GIC fine-grain + NUR-Harvest** (Rivers dB/dt–GIC hängt daran).
- **BGI AGrav + CEEIN C9 + nordische GIC-DB manifestieren** (Auswertung §5),
  GOES-18-Bucket um 16/19 erweitern.
- **Vier Serien-Assets (rixs/gbco/gmrt/gl30):** Witness-Epochen-Endpunkte;
  gl30/SRTM15+/GHSL descoped.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `static/membrane.html`
- `state/operator-gespraeche/2026-10-05-river.md` (gitignored)
- `docs/handover/handover-2026-10-05-river-folge95.md`
- `docs/handover/archiv/handover-2026-10-05-river-folge94.md` (Move aus `docs/handover/`)

## Burn: open 0.0000 · close 0.0828 · cap 0.25 — `session_burn` River-Linie (line-Session „River-Übergabe in einem Pass abarbeiten"; 1 `grind-flash`-Zensus + 1 `explore`-Lagecheck). Grund: Membran-First-Light-Reparatur (gemessene rAF-Suspension + stale Status), em-Apertur-Kernel-Zensus (3 Koordinaten-Träger / Riss), Presence-Bit- und EMM-Arm-Lage gefaltet.
