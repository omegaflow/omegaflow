<!--
  title: Handover — River-Folge 99 (2026-10-06)
  session: River-Folge 99
  class: handover
  date: 2026-10-06
  sha256: d573af716970be40bca076851e698dbb73d824145a9ea3445ee695f477a05f64
  status: live
-->
# Handover — River-Folge 99 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so viele
Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext) trägt die
Anzahl. Fremde uncommittete Arbeit wird nie überschrieben; committet wird nur der
eigene Teil; ein Push sendet nur Commits.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die River-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … River besitzt die Membran-Pfade (`main_flow`, `omega.rs`-Feld, Window/Gaze)." | 2026-10-06 | Operator (Session, River 98/99) — session-weiter Delegations-Consent, nicht das Commit-Wort
„die Membran muss stehen, bevor irgendwo eine Förder-Bewerbung abgeschickt wird … bis `/membrane.html` die Punktwolke rendert (die Sonne als Anker sichtbar)" | 2026-10-05 | Operator (future-folge181, gefaltet)
Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-06-river-folge98.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/paper/gic-causal-driver.md` (`class: paper`) — §6: die Bias-Korrektur ist **gebaut**
  (`bias_column` `tools/measure/src/bin/field_te_query.rs:2619`, `TE_NEFF_THRESHOLD`
  `src/mathematikerin/te.rs:51`/Gate `:67`, Rat 2026-10-05); die Paper-Zeile
  `docs/paper/gic-causal-driver.md:689` („no reported value applies it yet — the wiring stays
  open") ist eine **Doc-Code-Riss** (der Baum trägt die Verdrahtung). Offen: BCa-Intervalle,
  vollständiger Kp-Kanal; die Bias-Kurve selbst s. `## Offen`.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`class: sheet`, `status: unsealed`) —
  offen bis zum Siegel: X, Z-Fenster, Bz-Schwelle; α-Ebene + Siegel = Operator-Wort.
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (`class: survey`) —
  nächste Schritte §5.
- `docs/auftrag/auftrag-universelles-vlies.md` (`class: auftrag`) — Offen: Bias-Kurve,
  `ozzy`-Bau, Paar-Matrix, Ernte (§Lieferung).
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
  nie in `ci-gate`; Korrektur 2026-10-06 — der frühere Trigger nannte `ci-gate`, so konnte er nie
  grün feuern).
- **Lage:** (gemessen 2026-10-06 via `ci_manage`) HEAD `045711091`; `ci-gate 37429979869
  @045711091` = failure, Ursachen **fremd** (dropped-gate; clippy `src/archivar/units.rs:549`;
  register); `ci-check 37429979937 @045711091` = **pending**. River-Seite gebaut: WGSL-Gates
  `src/mathematikerin/shaders.rs:186`/`:211` auf `(u32(mt3.z) & 8u) != 0u`; Test
  `em_aperture_flux_bit_scales_and_kernel_proxy_does_not` (`src/mathematikerin/tests.rs`).
- **Blockade:** keine (eigene).
- **Braucht:** `ci-check` 37429979937 Ergebnis (Stehender Pass); grün → Punkt fällt, rot →
  `ci_manage log 37429979937`.

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
- **Lage:** (gemessen 2026-10-06) **Rat + Schwarm 6/12 + Claude + GLM einig** (Rat 2026-10-06;
  Schwarm: gemini, dots, ling, nemotron, inkling, kilo; die übrigen Routen rate-limitiert;
  Claude + z.ai GLM via UI):
  `TE_BIAS_MK_EMBEDDED` wird eine **n-Tabelle** (analog `TE_BIAS_MK`/`te_bias_m_k`), exakter
  Lookup, keine Interpolation; gemessen mit dem **eingebetteten KSG-Estimator**
  (`topological_te_estimate`, dim 3, auto-MI-τ, K=4 — der Produktionsarm `omega.rs:489`), nicht
  mit dem KDE-Arm. Die τ-Abhängigkeit ist ein benannter Riss. **Refinement (Claude/GLM):**
  auto-τ läuft *innerhalb* der Messung (pro Realisation neu), sonst kalibriert man eine andere
  Pipeline als die produktive; n außerhalb der Tabelle → `refused`, **kein Rückfall auf den
  Skalar**; die Tabelle ist an (dim, K, τ-Politik) gebunden — Konfigurationswechsel = Neumessung.
  **Gebaut:** `te.rs:45` jetzt
  `TE_BIAS_MK_EMBEDDED: &[(usize,f64)] = &[]` + `te_bias_m_k_embedded`; `te_bias_n_probe` um
  `--estimator embedded --dim` erweitert; `.github/workflows/te-bias-n.yml` fährt den scalaren +
  den eingebetteten Arm über `800…10000` inkl. produktiver n `6000,8546`.
- **Blockade:** keine (Bau steht); die Zahlen kommen aus CI, nie lokal.
- **Braucht:** (1) `te-bias-n`-Lauf lesen, Tabelle in `te.rs` eintragen; (2) `bias_column`
  (`field_te_query.rs:2619`) estimator-fest machen (KDE-Sockel nie über KSG-Wert); (3) 8 probe-Kanäle
  an den Draht + 15×15-Lauf (`matrix full`, `fdr bh 0.05 over matrix`); (4) `ozzy` **auf** der Matrix.
  **Ozzy läuft auf der Matrix, nicht davor.**

### Agnosis — Membran-Trio & Presence-Volume (River-Hand)
- **Status:** eigen | **Bindung:** eigen (cross-line: Mountain, Sensory)
- **Trigger:** keiner (arbeitbar bis zur Rats-Kante).
- **Lage:** (gemessen 2026-10-06) Verdikt `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md`.
  **Rat 2026-10-06 (bestätigt vom Schwarm 6/12 + Claude + GLM):** der korrekte Rahmen ist
  SSB/ICRS(+TDB) als kanonischer Elternrahmen **plus** ein je Volume **deklarierter Observer**
  als Kind; der feste `"earth"`-Rahmen ist der Bias (körperfest, zeitabhängig, erdperiodisch —
  für Sonne/Mars plausibel aussehende Fehlwerte); fehlt die Deklaration → `refused`, nie Default.
  **Schema (Claude/GLM):** `frame = {Körper, Figur, Achsenkonvention, vertikaler Bezug, Epoche
  (TDB), Rotations-/Ephemeridenmodell}` + Transformationskette zu ICRS/SSB; Altbestand einmalig
  als `earth-wgs84`/`legacy-assumed` **deklarieren** (Migration, kein Laufzeit-Default).
  Offen: (a) `static/membrane.html:43` `const BODIES = ["earth","moon","sun"]` →
  Build-Time-Manifest aus der Hüllen-Pipeline (Kante: kein Fenster-Edit; Mycelium/CI stagt);
  (b) `src/mathematikerin/omega.rs:878` hart `"earth"` → deklarativer Rahmen.
- **Blockade:** keine (Rat liegt); (b) ist mehrzeilig (Volume-Record/Kontrakt).
- **Braucht:** (b) vier Berührpunkte, ein Atom — (1) `src/archivar/volume.rs` `Volume.frame_body:
  Option<String>`; (2) `Extract::Volume`/`types.rs`/`parse.rs` tragen den Frame-Body,
  `main_flow.rs:3982` von URL-Name auf den deklarierten Body; (3) `omega.rs:776-830`+`:861-892`
  `ensure_volumes`/`upload_volumes` — `"earth"`-Literal durch `vol.frame_body` ersetzen,
  `icrs_to_body_geodetic` (`motion.rs:402`); (4) Test `volume_observer_declared_and_refused_when_absent`
  + ADR. `shaders.rs` bleibt unberührt (null Body-Namen).

### flyby-odf-Census — Probe-Stream vs. Workflow
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `flyby-odf-cdn` 37427673360 Ergebnis.
- **Lage:** (gemessen 2026-10-06) Fixture `src/archivar/kernels/odf07155.dat` = 88704 B;
  `./target/debug/odf_census_probe` liefert **2228 valide TRK-2-34-Records** (scid 236,
  data_type 11/12/13/37, dss_rx 14/43/63); Census auf **stdout** (`odf_census_probe.rs:87-96`) —
  die Datei ist nicht mehr leer; der Workflow `.github/workflows/flyby-odf-cdn.yml:29-30` leitet
  stdout + `test -s`-Guard, kein `2>/dev/null` mehr. Verdict bleibt `pending` (kein signed
  range-rate-Residuum ohne Doppler-Modell).
- **Blockade:** keine.
- **Braucht:** Run 37427673360 aus dem Stehenden Pass; grün → Punkt fällt.

## An mountain

Origin: river folge99.

- **clippy `trim_split_whitespace`** (`src/archivar/units.rs:549`,
  `epoch.trim().split_whitespace()`, gemessen via `ci_manage log 37429979869` @`045711091`):
  der rote `ci-gate`. **Braucht:** `.trim()` entfernen (`split_whitespace` ignoriert
  Rand-Weißraum bereits).
- **EMM `emm_exi_l2a` Unit-Riss** (`phi/sources.φ:17281`): Einheit `count`; der uncommittete Arm
  (`bunit_declares_count`, `COMP_COUNT` in `src/archivar/emm_exi.rs`) schließt den Riss und
  kompiliert. **Braucht:** Einheiten-Entscheid am Register.
- **`bat_fluence_erg_cm2`-Apertur-Riss** (`phi/sources.φ:17812`): als `aperture:flux`
  deklariert, während die River-Zeile es als nicht-Fluss führt. **Braucht:** Mountain-Verdikt
  (eine Zeile).

## An mycelium

Origin: river folge99.

- **DE440-`.bin` remanifestieren.** Nach Mountains `de_compiler`-GM-Landung die
  `ephemeris_de440_{earth,moon,sun}.bin` (und die Geschwister) neu bauen und über die CI zur
  CDN bringen; `pages-deploy.yml` stagt sie same-origin. Checkmark ist Rivers Browser-Re-Messung
  `nearCount(<1e13 m) > 0`.
- **`flyby-odf-cdn`** — Workflow sauber (kein `2>/dev/null`, `test -s`-Guard, `--clobber`,
  gemessen 2026-10-06); Run `37427673360` = **queued**. Ergebnis aus dem Stehenden Pass,
  kein Polling.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `src/mathematikerin/te.rs`
- `tools/measure/src/bin/te_bias_n_probe.rs`
- `.github/workflows/te-bias-n.yml`
- `docs/handover/handover-2026-10-06-river-folge99.md`
- `docs/handover/archiv/handover-2026-10-06-river-folge98.md` (Move aus `docs/handover/`)

Verifikation/Dispatches: `cargo check` clean, `cargo build -p omegaflow-measure --bin te_bias_n_probe`
clean, `cargo fmt --` auf die zwei Quelldateien; `./target/debug/odf_census_probe` lief lokal
(2228 valide Records, silent). Rat gelaufen (`council`, 2026-10-06) + Schwarm 6/12
(gemini/dots/ling/nemotron/inkling/kilo einig; qwen/zai/zen rate-limitiert, gptoss/agnes/kenari
abgebrochen) + Claude + z.ai GLM (UI, Operator-vermittelt) — alle einig.
`register_lookup --fired river` = em-Apertur (FIRED_UNGEMESSEN) → gemessen;
`--addressed river` = 3 (future-181, mountain-238, mycelium-235) — gefaltet.

## Burn: open 0.0000 · close 0.1036 — `session_burn` River-Linie (top session „River-Linie Übergabe in einem Pass abarbeiten", gemessen 2026-10-06)
