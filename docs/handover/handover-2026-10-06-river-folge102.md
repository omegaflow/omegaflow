<!--
  title: Handover — River-Folge 102 (2026-10-06)
  session: River-Folge 102
  class: handover
  date: 2026-10-06
  sha256: 9b2fdaf98f93b868f7254743a1087c57491c70c2974946c944e44052640a1ce4
  status: live
-->
# Handover — River-Folge 102 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so viele
Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext) trägt die
Anzahl. Fremde uncommittete Arbeit wird nie überschrieben; committet wird nur der
eigene Teil; ein Push sendet nur Commits.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die River-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … River besitzt die Membran-Pfade (`main_flow`, `omega.rs`-Feld, Window/Gaze)." | 2026-10-06 | Operator (Session, River 100; River 101 erneut) — session-weiter Delegations-Consent, nicht das Commit-Wort
„die Membran muss stehen, bevor irgendwo eine Förder-Bewerbung abgeschickt wird … bis `/membrane.html` die Punktwolke rendert (die Sonne als Anker sichtbar)" | 2026-10-05 | Operator (future-folge181, gefaltet)
„earth-wgs84/legacy-assumed können wir den nicht migrieren ich möchte eigentlich kein legacy haben / deklarieren" — der Volume-Frame ist Pflicht-Deklaration je Quelle, kein Legacy-Bucket, keine Migration | 2026-10-06 | Operator (Session, River 99)
Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-06-river-folge100.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/paper/gic-causal-driver.md` (`class: paper`) — §6: die Bias-Korrektur ist **gebaut**
  (`bias_column` `tools/measure/src/bin/field_te_query.rs:2619`, `TE_NEFF_THRESHOLD`
  `src/mathematikerin/te.rs:101`, Rat 2026-10-05); die Paper-Zeile
  `docs/paper/gic-causal-driver.md:689` („no reported value applies it yet — the wiring stays
  open") ist eine **Doc-Code-Riss** (der Baum trägt die Verdrahtung). Offen: BCa-Intervalle,
  vollständiger Kp-Kanal; die Bias-Kurve selbst s. `## Offen`.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`class: sheet`, `status: unsealed`) —
  offen bis zum Siegel: X, Z-Fenster, Bz-Schwelle; α-Ebene + Siegel = Operator-Wort.
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (`class: survey`) —
  nächste Schritte §5.
- `docs/auftrag/auftrag-universelles-vlies.md` (`class: auftrag`) — Offen: Bias-Kurve
  (estimator-fest), `ozzy`-Bau, Paar-Matrix, Ernte (§Lieferung).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (`class: survey`) — §7 geschlossen.
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` (`class: survey`) — Archäologie +
  Dreifach-Verdikt; Läufer für die Code-Punkte an Mountain/River/Sensory.
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` / `docs/auftrag/auftrag-flyby2-kette.md` —
  Offen: OMNI2 26 Zellen, ACE 3/14/16, kp `def`, Δ/σ_recon.
- `docs/concepts/exzellenz-konzept.md` (`class: concept`, `version: 1`) — Prüfmaßstab.

## Offen (aufgeschlüsselt)

### Membran-Sonne-Anker (Operator-Wort future-181; cross-line Mountain/Mycelium)
- **Status:** blockiert | **Bindung:** eigen (cross-line: Mountain, Mycelium)
- **Trigger:** Mountains `de_compiler`-GM-Landung (Maske Bit 11 / slot `f(11)`) + Mycelium-Remanifestation der DE440-`.bin`.
- **Lage:** (gemessen 2026-10-06, deployt `omegaflow.space`) Der Boot-Pfad ist repariert
  (`static/membrane.html`). Der Sonne/Erde/Mond-Anker fehlt weiterhin: `body_anchor_samples`
  (`src/archivar/membrane.rs:404`) emittiert nur bei `props.omega_g` (stype7) oder `props.gm`;
  die deployte `ephemeris_de440_earth.bin` trägt Maske bits 0–8, Bit 11 klar → Browser-Messung
  `nearCount(<1e13 m) = 0`. Rat 2026-10-06: Mechanismus (a) — `val` = gemessener GM,
  `force_type = 1.0`; der Riss ist „Maske sagt absent, Quelle trägt gm", `pending`.
- **Blockade:** der gemessene GM fehlt in der `.bin` — Mountains Parser-/`de_compiler`-Akt.
- **Braucht:** Mountain setzt slot `f(11)`/Maske Bit 11; Mycelium baut + manifestiert;
  Rivers Checkmark ist `nearCount(<1e13 m) > 0`.

### em-Apertur — Kanal-Identität statt Kernel-Proxy (Rat 2026-10-05; zwei Hände)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am eigenen HEAD (der GPU-Paritätstest läuft allein in
  `ci-check`, `cargo test --release --features browser_relay`, `.github/workflows/ci-check.yml:42`,
  nie in `ci-gate`).
- **Lage:** (gemessen 2026-10-06 via `ci_manage`) Der frühere Trigger-Lauf `ci-check
  37439442123 @6f00c6599` = **cancelled** (Push-Konkurrenz, `ci_manage view`); kein grüner
  `ci-check` am River-HEAD. River-Seite gebaut: WGSL-Gates
  `src/mathematikerin/shaders.rs:186`/`:211` auf `(u32(mt3.z) & 8u) != 0u`; Test
  `em_aperture_flux_bit_scales_and_kernel_proxy_does_not` (`src/mathematikerin/tests.rs`).
  Mountain-Seite gebaut (`mountain-239`), `shaders.rs` unberührt.
- **Blockade:** keine (eigene).
- **Braucht:** `ci-check` am eigenen HEAD abwarten (Stehender Pass); grün → Punkt fällt, rot →
  `ci_manage log <id>`.

### dB/dt–GIC-Relation Mäntsälä
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** fine-grain `fmi_gic`-Asset + NUR-Asset im CDN.
- **Lage:** (gemessen 2026-10-05) FMI-GIC CC BY 4.0, 1999–2023, Halt 2023-10-23, beste Qualität
  1999–April 2005, nicht-uniform; NUR nur als SuperMAG-Station. FMI/NUR-Ernte frei (gemessen
  2026-10-05, `mail_ledger.φ` record 235, Ari Viljanen); flacher Fit
  = `doi:10.5194/angeo-43-271-2025` (Eq. 43, Table 1).
- **Blockade:** die zwei Harvests (Mountain/Mycelium) + neuer Probe-Bin.
- **Braucht:** (1) `fmi_gic` fine-grain registriert+manifestiert; (2) NUR im CDN; (3) Probe
  dB/dt(NUR)–GIC(Mäntsälä) + Zahl in Paper §4/§6.

### Universelles Vlies — Bias-Kurve, `ozzy` + Alles-gegen-alles-Matrix
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner (arbeitbar bis zur Rats-Kante).
- **Lage:** (gemessen 2026-10-06) **Rat + Schwarm einig** (Rat 2026-10-06): die Bias-Tabelle ist
  ein exakter n-Lookup **je Estimator**, keine Interpolation, kein Skalar-Rückfall. **Gebaut
  (dieses Atom, River 102):** `BiasArm` (`src/mathematikerin/te.rs:73`) typisiert den Arm;
  `bias_column` (`tools/measure/src/bin/field_te_query.rs:2619`) liest allein die Tabelle des
  Arms — `BinnedHistogram` → `TE_BIAS_MK_BINNED` (`te.rs:63`, leer bis zur Messung,
  `table_pending`), nie der KDE-Sockel; Gates
  `gate_bias_arm_carries_its_own_table_never_the_scalar_socket` (`te.rs:4041`) und
  `bias_column_gates_on_n_eff_and_exact_n` (`field_te_query.rs:4559`). `cargo check` warnungsfrei;
  `cargo build -p omegaflow-measure --bin field_te_query` grün. Früher gebaut: embedded-Tabelle
  `TE_BIAS_MK_EMBEDDED` (`te.rs:45`, Gate `:4015`), Binned-Probe-Arm
  `tools/measure/src/bin/te_bias_n_probe.rs --estimator binned`.
- **Blockade:** die Binned-Zahlen kommen aus CI (`te-bias-n 37440043700` = **queued**, keine
  Runner-Kapazität — die vielen jaxa-gportal-Läufe stauen), nie lokal.
- **Braucht:** (1) `te-bias-n 37440043700` lesen (Trigger: Lauf grün), Binned-Kurve in
  `TE_BIAS_MK_BINNED` (`te.rs:63`) eintragen + Gate; (2) **embedded-Arm bleibt benannter Rest
  (gemessen, nicht vergessen):** `TE_BIAS_MK_EMBEDDED` ist mit dem *unbedingten*
  `topological_te_estimate` (`te.rs:3326`) gemessen; der einzige embedded-Konsument in
  `field_te_query` ist der *bedingte* `conditional_embedded_te_phase` (`te.rs:2947`, `cte`) —
  eine unbedingte Tabelle über einen bedingten Wert wäre eine fabrizierte Korrektur; nächster
  Schritt = bedingter embedded-Probe-Arm oder Rats-Wort; (3) 8 probe-Kanäle an den Draht +
  15×15-Lauf (`matrix full`, `fdr bh 0.05 over matrix`, `cell_te_and_surrogates`
  `field_te_query.rs:4120`, `MATRIX_BINS` `:3982`); (4) `ozzy` **auf** der Matrix.

### Agnosis — Membran-Trio (Rest (a))
- **Status:** wartend (fremd) | **Bindung:** eigen (cross-line: Mycelium, CI)
- **Trigger:** Mycelium/CI-Build-Time-Manifest der Hüllen-Pipeline (`static/membrane.html:43`).
- **Lage:** (gemessen 2026-10-06 via `sgrep`) Punkt (b) ist gebaut (`mountain-239`):
  `Volume.frame_body: Option<String>` (`src/archivar/volume.rs:155`), `Extract::Volume.frame_body`
  aus `at` (`src/archivar/parse.rs:353`, `types.rs:239`), `upload_volumes` konsumiert
  `v.frame_body` (`src/mathematikerin/omega.rs:872`, fehlt → `valid=0`), Test
  `volume_observer_declared_and_refused_when_absent` (`parse.rs:1813`), `"earth"`-Literal weg.
  Offen nur (a): `static/membrane.html:43` `const BODIES = ["earth","moon","sun"]` →
  Build-Time-Manifest (Kante: kein Fenster-Edit; Mycelium/CI stagt).
- **Blockade:** (a) ist Mycelium/CI (kein River-Fenster-Edit ohne Operator-Wort).
- **Braucht:** Mycelium ersetzt die Handkopie durch das generierte Manifest; s. `## An mycelium`.

## An mycelium

Origin: river folge101.

- **DE440-`.bin` remanifestieren.** Nach Mountains `de_compiler`-GM-Landung die
  `ephemeris_de440_{earth,moon,sun}.bin` (und die Geschwister) neu bauen und über die CI zur
  CDN bringen; `pages-deploy.yml` stagt sie same-origin. Checkmark ist Rivers Browser-Re-Messung
  `nearCount(<1e13 m) > 0`.
- **`static/membrane.html:43` BODIES-Handkopie** (`["earth","moon","sun"]`) → Build-Time-Manifest
  aus der Hüllen-Pipeline. Kein River-Fenster-Edit (Kante); der Punkt ist Rivers Agnosis-Rest (a).
- **Measured (river 102):** `flyby-odf-cdn` `37427673360 @2f44a6092` = **success**
  (`ci_manage view`) — der frühere `37305400435`-Fehler (leerer Census) ist abgelöst; die
  CI-Tafel des Stehenden Passes führt ihn noch als rot.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `src/mathematikerin/te.rs`
- `tools/measure/src/bin/field_te_query.rs`
- `docs/handover/handover-2026-10-06-river-folge102.md`
- `docs/handover/archiv/handover-2026-10-06-river-folge101.md` (Move aus `docs/handover/`)

Verifikation/Dispatches: `cargo check` warnungsfrei; `cargo fmt -- src/mathematikerin/te.rs
tools/measure/src/bin/field_te_query.rs`; `cargo build -p omegaflow-measure --bin field_te_query`
grün. `register_lookup --fired river` = em-Apertur (FIRED_UNGEMESSEN) → `ci-check 37439442123`
cancelled → bleibt wartend; `--stale river --persist 3` = 0; `--addressed river` = 2 (future-183
in Punkt Membran-Sonne-Anker + Operator-Wort gefaltet; mycelium-236 DE440 = Punkt 1, odf durch
`37427673360` = success gemessen aufgelöst). `open_points_check` folge101 = 0 format-gaps.
`session_burn` open/close s. u.

## Burn: open 0.0000 · close 0.0355 — `session_burn` River-Linie (top session „River-Linie in einem Pass starten", gemessen 2026-10-06)
