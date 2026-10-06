<!--
  title: Handover — River-Folge 104 (2026-10-06)
  session: River-Folge 104
  class: handover
  date: 2026-10-06
  sha256: 60cc94e55517c9517c1b423b8ab2d6dad90a1207d6e7d28337700d5571e62ed5
  status: live
-->
# Handover — River-Folge 104 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so viele
Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext) trägt die
Anzahl. Fremde uncommittete Arbeit wird nie überschrieben; committet wird nur der
eigene Teil; ein Push sendet nur Commits.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die River-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … River besitzt die Membran-Pfade (`main_flow`, `omega.rs`-Feld, Window/Gaze)." | 2026-10-06 | Operator (Session, River 100–104) — session-weiter Delegations-Consent, nicht das Commit-Wort
„die Membran muss stehen, bevor irgendwo eine Förder-Bewerbung abgeschickt wird … bis `/membrane.html` die Punktwolke rendert (die Sonne als Anker sichtbar)" | 2026-10-05 | Operator (future-folge181, gefaltet)
„earth-wgs84/legacy-assumed können wir den nicht migrieren ich möchte eigentlich kein legacy haben / deklarieren" — der Volume-Frame ist Pflicht-Deklaration je Quelle, kein Legacy-Bucket, keine Migration | 2026-10-06 | Operator (Session, River 99)
„kannst du die frage bitte noch den voices und glm und claude online chat geben" — die Ratsfrage zusätzlich an die Schwarm-Stimmen, glm und Claude-online | 2026-10-06 | Operator (Session, River 104)
Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-06-river-folge103.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/paper/gic-causal-driver.md` (`class: paper`) — §6: die Bias-Korrektur ist **gebaut**
  (`bias_column` `tools/measure/src/bin/field_te_query.rs:2619`, `TE_NEFF_THRESHOLD`
  `src/mathematikerin/te.rs:101`, Rat 2026-10-05); die Paper-Zeile
  `docs/paper/gic-causal-driver.md:689` („no reported value applies it yet — the wiring stays
  open") ist eine **Doc-Code-Riss** (der Baum trägt die Verdrahtung). Offen: BCa-Intervalle,
  vollständiger Kp-Kanal.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`class: sheet`, `status: unsealed`) —
  offen bis zum Siegel: X, Z-Fenster, Bz-Schwelle; α-Ebene + Siegel = Operator-Wort.
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (`class: survey`) — §5.
- `docs/auftrag/auftrag-universelles-vlies.md` (`class: auftrag`) — Offen: Bias-Kurve
  (estimator-fest), `ozzy`-Bau, Paar-Matrix, Ernte (§Lieferung).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (`class: survey`) — §7 geschlossen.
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` (`class: survey`) — Läufer für die
  Code-Punkte an Mountain/River/Sensory.
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` / `docs/auftrag/auftrag-flyby2-kette.md` —
  Offen: OMNI2 26 Zellen, ACE 3/14/16, kp `def`, Δ/σ_recon.
- `docs/concepts/exzellenz-konzept.md` (`class: concept`, `version: 1`) — Prüfmaßstab.

## Offen (aufgeschlüsselt)

### Universelles Vlies — Bias-Kurve, `ozzy` + Alles-gegen-alles-Matrix
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** `te-bias-n` Re-Lauf `37443829535` (queued) grün.
- **Lage:** (gemessen 2026-10-06, River 104)
  - **Binned-Kurve gelandet:** `TE_BIAS_MK_BINNED` (`src/mathematikerin/te.rs:63`) trägt die
    8 gemessenen Punkte aus `te-bias-n 37440043700 @72c9348ce` — (800, 2.715e-3) ·
    (1260, 2.393e-3) · (1600, 1.507e-3) · (2200, −5.634e-4) · (4000, 1.762e-3) ·
    (6000, 7.967e-4) · (8546, −4.494e-4) · (10000, 0). Gate
    `gate_te_bias_binned_near_unbiased_reference_and_refusal`; die zwei field_te_query-Tests
    auf `table_pending` wurden auf die gemessene Tabelle nachgezogen.
  - **Matrix-Zelle reicht echtes `kde_n_eff` durch** (Rat 2026-10-06 + Stimmen + glm + claude,
    alle (a), Reihenfolge target-first): `CellTe { te, n_eff, surrogates }`,
    `cell_te_and_surrogates(target, driver, lags, …)` (`field_te_query.rs:4122`); der Aufruf
    `matrix_cell_bias(v, o.n, o.cond_n, Some(o.n as f64))` ist ersetzt durch `o.n_eff`; der
    Report führt eine `n_eff`-Spalte. Die Raw-`n`-Als-`n_eff`-Fabrikation (das Gate konnte nie
    verweigern) ist weg.
  - **Pfeil-Riss geschlossen:** `transfer_entropy_conditional_binned_n(A,B)` misst `B→A`
    (gemessen: `te.rs:6421` koppelt x→y und ruft `binned(&y,&x)`; ebenso `transfer_entropy_lag`).
    Die Matrix rief `binned(driver, target)` = `t→d` für die Zelle `d->t` — eine umgekehrte
    Achse. Jetzt `binned(target, driver)` = `d→t`. Gate
    `matrix_cell_measures_the_arrow_of_its_target_driver_order` (`field_te_query.rs`).
  - **Kalibrier-Riss benannt:** `te_bias_n_probe.rs:223` maß `kde_n_eff(&x,&y)` (driver-first),
    alle Produktionskonsumenten `(target,driver)` (`field_te_query.rs:2758`, `wy_max_t.rs:474`).
    Die Probe ist auf `(target,driver)` korrigiert; `TE_NEFF_THRESHOLD` (18.166) ist **noch** die
    driver-first-Kalibrierung.
  - Früher (River 103): `BiasArm` (`te.rs:73`), `bias_column`, Gate
    `bias_table_only_on_unconditional_cells`.
- **Blockade:** die 8 probe-Kanäle sind nicht am Draht.
- **Braucht:**
  (1) `te-bias-n 37443829535` grün lesen → das n=800-`kde_n_eff`-Mittel im korrigierten
  `(target, driver)`-Auftrag in `TE_NEFF_THRESHOLD` (`src/mathematikerin/te.rs:101`) setzen + Gate.
  (2) **Binned-Floor geliehen:** 18.166 ist scalar-KDE-kalibriert; für die
  binned-Histogramm-Zelle ist der geeignete `n_eff` (Histogramm-Besetzung, nicht KDE-Bandbreite)
  nicht kalibriert — Follow-up: binned-spezifischen Floor aus der Probe ableiten.
  (3) **conditional-embedded Probe bleibt `pending`** (Verdikt 2): Trigger = cte ist
  Produktionskonsument; die Probe muss `conditional_embedded_te_estimate` (Punktschätzer) über
  coupled Hénon mit Konditionsreihe messen, **nicht** die Phase-Surrogat-Maschinerie. `cond_n > 0`
  wird von `matrix_cell_bias` als `unadjusted_conditional` geführt, nicht auf der binned-Zelle.
  (4) 8 probe-Kanäle an den Draht + 15×15-Lauf (`matrix full`, `fdr bh 0.05 over matrix`,
  `cell_te_and_surrogates` `field_te_query.rs:4122`).
  (5) `ozzy` **auf** der Matrix (`auftrag-universelles-vlies.md` §Lieferung).
  (6) optional: Newtype `KdeNEff(f64)`, damit `o.n as f64` an der `n_eff`-Stelle unrepresentable wird.

### em-Apertur — Kanal-Identität statt Kernel-Proxy (Rat 2026-10-05; zwei Hände)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am eigenen HEAD.
- **Lage:** (gemessen 2026-10-06 via `ci_manage view`) `ci-check 37440906455 @d4b4a8e93` =
  **pending**. River-Seite gebaut: WGSL-Gates `src/mathematikerin/shaders.rs:186`/`:211` auf
  `(u32(mt3.z) & 8u) != 0u`; Test `em_aperture_flux_bit_scales_and_kernel_proxy_does_not`
  (`src/mathematikerin/tests.rs`). Mountain-Seite gebaut (`mountain-239`).
- **Blockade:** keine (eigene).
- **Braucht:** `ci-check 37440906455` abwarten (Stehender Pass; kein Polling); grün → Punkt fällt,
  rot → `ci_manage log 37440906455`.

### dB/dt–GIC-Relation Mäntsälä
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** fine-grain `fmi_gic`-Asset + NUR-Asset im CDN.
- **Lage:** (gemessen 2026-10-05) FMI-GIC CC BY 4.0, 1999–2023, Halt 2023-10-23, nicht-uniform;
  NUR nur als SuperMAG-Station. FMI/NUR-Ernte frei (`mail_ledger.φ` record 235, Ari Viljanen);
  flacher Fit = `doi:10.5194/angeo-43-271-2025` (Eq. 43, Table 1).
- **Blockade:** die zwei Harvests (Mountain/Mycelium) + neuer Probe-Bin.
- **Braucht:** (1) `fmi_gic` fine-grain registriert+manifestiert; (2) NUR im CDN; (3) Probe
  dB/dt(NUR)–GIC(Mäntsälä) + Zahl in Paper §4/§6.

### Membran-Sonne-Anker (Operator-Wort future-181; cross-line Mountain/Mycelium)
- **Status:** blockiert | **Bindung:** eigen (cross-line: Mountain, Mycelium)
- **Trigger:** Mountains `de_compiler`-GM-Landung (Maske Bit 11 / slot `f(11)`) + Mycelium-Remanifestation
  der DE440-`.bin`.
- **Lage:** (gemessen 2026-10-06, deployt `omegaflow.space`) Der Boot-Pfad ist repariert
  (`static/membrane.html`). Der Sonne/Erde/Mond-Anker fehlt: `body_anchor_samples`
  (`src/archivar/membrane.rs:404`) emittiert nur bei `props.omega_g` (stype7) oder `props.gm`; die
  deployte `ephemeris_de440_earth.bin` trägt Maske bits 0–8, Bit 11 klar → Browser-Messung
  `nearCount(<1e13 m) = 0`. Rat 2026-10-06: Mechanismus (a) — `val` = gemessener GM,
  `force_type = 1.0`; der Riss ist „Maske sagt absent, Quelle trägt gm", `pending`.
- **Blockade:** der gemessene GM fehlt in der `.bin` — Mountains Parser-/`de_compiler`-Akt.
- **Braucht:** Mountain setzt slot `f(11)`/Maske Bit 11; Mycelium baut + manifestiert;
  Rivers Checkmark ist `nearCount(<1e13 m) > 0`.

### Agnosis — Membran-Trio (Rest (a))
- **Status:** wartend (fremd) | **Bindung:** eigen (cross-line: Mycelium, CI)
- **Trigger:** Mycelium/CI-Build-Time-Manifest der Hüllen-Pipeline (`static/membrane.html:43`).
- **Lage:** (gemessen 2026-10-06) Punkt (b) ist gebaut (`mountain-239`):
  `Volume.frame_body: Option<String>` (`src/archivar/volume.rs:155`), `Extract::Volume.frame_body`
  aus `at` (`src/archivar/parse.rs:353`, `types.rs:239`), `upload_volumes` konsumiert `v.frame_body`
  (`src/mathematikerin/omega.rs:872`, fehlt → `valid=0`), Test
  `volume_observer_declared_and_refused_when_absent` (`parse.rs:1813`). Offen nur (a):
  `static/membrane.html:43` `const BODIES = ["earth","moon","sun"]` → Build-Time-Manifest.
- **Blockade:** (a) ist Mycelium/CI (kein River-Fenster-Edit ohne Operator-Wort).
- **Braucht:** s. `## An mycelium`.

## An mycelium

Origin: river folge101 (getragen über folge102/103/104).

- **DE440-`.bin` remanifestieren.** Nach Mountains `de_compiler`-GM-Landung die
  `ephemeris_de440_{earth,moon,sun}.bin` (und die Geschwister) neu bauen und über die CI zur
  CDN bringen; `pages-deploy.yml` stagt sie same-origin. Checkmark ist Rivers Browser-Re-Messung
  `nearCount(<1e13 m) > 0`.
- **`static/membrane.html:43` BODIES-Handkopie** (`["earth","moon","sun"]`) → Build-Time-Manifest
  aus der Hüllen-Pipeline. Kein River-Fenster-Edit (Kante); der Punkt ist Rivers Agnosis-Rest (a).
- **Measured (river 102):** `flyby-odf-cdn` `37427673360 @2f44a6092` = **success**
  (`ci_manage view`) — der frühere `37305400435`-Fehler (leerer Census) ist abgelöst.

## LOCK

- keine.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `src/mathematikerin/te.rs`
- `tools/measure/src/bin/field_te_query.rs`
- `tools/measure/src/bin/te_bias_n_probe.rs`
- `docs/handover/handover-2026-10-06-river-folge104.md`
- `docs/handover/archiv/handover-2026-10-06-river-folge103.md` (Move aus `docs/handover/`)

Verifikation/Dispatches: `cargo check` grün; `cargo build -p omegaflow-measure --bin
field_te_query` und `--bin te_bias_n_probe` grün, ohne Warning; `cargo fmt -- <eigene Pfade>`
sauber. Rat (Council) 2026-10-06 zum `n_eff`-Riss: Verdikt (a), zudem die Pfeil-Inversion und der
Kalibrier-Order-Riss benannt — am Baum gemessen (`te.rs:6421`, `transfer_entropy_lag`) und
umgesetzt. Stimmen `voice-gemini`/`gptoss`/`nemotron`/`dots`/`ling`/`agnes`/`zen`/`kenari`/`zai`
sowie glm und claude: alle (a), target-first. `te-bias-n 37443829535` dispatcht. `register_lookup
--fired river` = em-apertur (Trigger `ci-check 37440906455` pending) → bleibt wartend; `--stale
river --persist 3` = 0; `--addressed river` = 2 (future-183, mycelium-238), gefaltet;
`open_points_check folge103` = 0 format-gaps. `git_safety --snapshot` s. u.

## Burn: open 0.0000 · close 0.0737 — Session-Burn (`session_burn`: eigene Session-Zeile „River-Linie in einem Pass starten" $0.0737; Runde total 0.3607 → 0.5462; Council $0.0279 + 9 Stimmen/glm/claude; gemessen 2026-10-06)
