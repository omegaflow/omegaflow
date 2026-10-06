<!--
  title: Handover — River-Folge 105 (2026-10-06)
  session: River-Folge 105
  class: handover
  date: 2026-10-06
  sha256: 12c114b0df5bf74a6fa9163b1ac02f61bc6959ffbc456b582e5c40726cdbd3bb
  status: live
-->
# Handover — River-Folge 105 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so viele
Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext) trägt die
Anzahl. Fremde uncommittete Arbeit wird nie überschrieben; committet wird nur der
eigene Teil; ein Push sendet nur Commits.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die River-Linie **in einem Pass** — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … River besitzt die Membran-Pfade (`main_flow`, `omega.rs`-Feld, Window/Gaze)." | 2026-10-06 | Operator (Session, River 100–105) — session-weiter Delegations-Consent, nicht das Commit-Wort
„die Membran muss stehen, bevor irgendwo eine Förder-Bewerbung abgeschickt wird … bis `/membrane.html` die Punktwolke rendert (die Sonne als Anker sichtbar)" | 2026-10-05 | Operator (future-folge181, gefaltet)
„earth-wgs84/legacy-assumed können wir den nicht migrieren ich möchte eigentlich kein legacy haben / deklarieren" — der Volume-Frame ist Pflicht-Deklaration je Quelle, kein Legacy-Bucket, keine Migration | 2026-10-06 | Operator (Session, River 99)
„kannst du die frage bitte noch den voices und glm und claude online chat geben" — die Ratsfrage zusätzlich an die Schwarm-Stimmen, glm und Claude-online | 2026-10-06 | Operator (Session, River 104)
Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-06-river-folge104.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/paper/gic-causal-driver.md` (`class: paper`) — §6 `:689` („no reported value
  applies it yet — the wiring stays open") gegen den Baum: `bias_column`
  (`tools/measure/src/bin/field_te_query.rs:2619`) liefert eine
  **Report-Site-Spalte**; der Report druckt selbst `bias (report site only; raw TE
  untouched; gate n_eff >= …)` (`:2933`). Zwei Lesarten — (i) „reported value" = die
  TE-Tabelle, die roh bleibt → das Papier stimmt; (ii) „reported value" = jede
  Ausgabe inkl. der Bias-Spalte → das Papier ist stale. **Riss**, beide Zeugen
  benannt, nicht geglättet. Offen: BCa-Intervalle, vollständiger Kp-Kanal.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`class: sheet`, `status: unsealed`) —
  offen bis zum Siegel: X, Z-Fenster, Bz-Schwelle; α-Ebene + Siegel = Operator-Wort.
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (`class: survey`) — §5.
- `docs/auftrag/auftrag-universelles-vlies.md` (`class: auftrag`) — Offen: Bias-Kurve
  (estimator-fest), `ozzy`-Bau, Paar-Matrix, Ernte (§Lieferung).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (`class: survey`) — §7 geschlossen.
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` (`class: survey`) — Punkt 1
  (`body_in_enclosure`-Bypass) gebaut: `dcc3243f8` (mountain 238) „agnosis anchor
  bypass removed"; die übrigen Code-Punkte liegen bei ihren Owner-Linien
  (`frames.rs`/`weberin.rs` → Mountain/Sensory; `membrane.html`-Trio → Mycelium/CI).
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` / `docs/auftrag/auftrag-flyby2-kette.md` —
  Offen: OMNI2 26 Zellen, ACE 3/14/16, kp `def`, Δ/σ_recon.
- `docs/concepts/exzellenz-konzept.md` (`class: concept`, `version: 1`) — Prüfmaßstab.

## Offen (aufgeschlüsselt)

### Universelles Vlies — Bias-Kurve, `ozzy` + Alles-gegen-alles-Matrix
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** `te-bias-n` Re-Lauf `37443829535` grün.
- **Lage:** (gemessen 2026-10-06, River 105 via `ci_manage view`)
  - **Binned-Kurve gelandet:** `TE_BIAS_MK_BINNED` (`src/mathematikerin/te.rs:63`) trägt die
    8 gemessenen Punkte aus `te-bias-n 37440043700 @72c9348ce`.
  - **Matrix-Zelle reicht echtes `kde_n_eff` durch** (`CellTe { te, n_eff, surrogates }`,
    `cell_te_and_surrogates(target, driver, lags, …)` `field_te_query.rs:4122`); der
    Report führt eine `n_eff`-Spalte.
  - **Pfeil-Riss geschlossen:** `binned(target, driver)` = `d→t`; Gate
    `matrix_cell_measures_the_arrow_of_its_target_driver_order`.
  - **Kalibrier-Riss benannt:** `te_bias_n_probe.rs:223` auf `(target,driver)` korrigiert;
    `TE_NEFF_THRESHOLD` = `Some(1.8166e1)` (`src/mathematikerin/te.rs:110`) ist **noch** die
    driver-first-Kalibrierung.
  - **Lauf-Stand:** `te-bias-n 37443829535 @2b7acb4d4` = **queued** (seit 09:33:54Z, Stand
    09:39Z; Vorgänger `37443567797` cancelled).
- **Blockade:** die 8 probe-Kanäle sind nicht am Draht.
- **Braucht:**
  (1) `te-bias-n 37443829535` grün lesen → das n=800-`kde_n_eff`-Mittel im korrigierten
  `(target, driver)`-Auftrag in `TE_NEFF_THRESHOLD` setzen + Gate.
  (2) binned-spezifischen Floor aus der Probe ableiten (Histogramm-Besetzung, nicht KDE-Bandbreite).
  (3) conditional-embedded Probe `pending`: `conditional_embedded_te_estimate` über coupled
  Hénon mit Konditionsreihe, nicht Phase-Surrogat.
  (4) 8 probe-Kanäle an den Draht + 15×15-Lauf (`matrix full`, `fdr bh 0.05 over matrix`).
  (5) `ozzy` **auf** der Matrix (`auftrag-universelles-vlies.md` §Lieferung).

### em-Apertur — Kanal-Identität statt Kernel-Proxy (Rat 2026-10-05; zwei Hände)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am eigenen HEAD.
- **Lage:** (gemessen 2026-10-06, River 105 via `ci_manage view`) `ci-check 37440906455
  @d4b4a8e93` = **cancelled** (Push-Konkurrenz, kein Verdikt); der aktuelle HEAD `b71658f`
  trägt `ci-check 37444174960` = **pending**. River-Seite gebaut: WGSL-Gates
  `src/mathematikerin/shaders.rs:186`/`:211` auf `(u32(mt3.z) & 8u) != 0u`; Test
  `em_aperture_flux_bit_scales_and_kernel_proxy_does_not`. Mountain-Seite gebaut (mountain-239).
- **Blockade:** keine (eigene); Runner-Kapazität (Stehender Pass).
- **Braucht:** `ci-check 37444174960` abwarten (Stehender Pass; kein Polling); grün → Punkt fällt,
  rot → `ci_manage log 37444174960`.

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
- **Lage:** (gemessen 2026-10-06, River 105)
  - **CDN-Assets erfüllt** (mycelium-240, gefaltet): `ephemeris_de440_{earth,moon,sun}.bin`
    same-origin HTTP 206, sha256 == `pages-deploy.yml:60-62`; `dr3_stars.bin` `745a3f71…`.
  - **Client-Pfad bootet** (River 105, `archive_search --playwright
    https://omegaflow.space/membrane.html`, 2 Läufe): Titel „omegaflow — membrane", HTTP 200,
    Status fortschreitend `loading stars… 27%`/`40%` — **kein** Hänger am Boot-Pfad.
  - Der Sonne/Erde/Mond-Anker fehlt weiter: `body_anchor_samples`
    (`src/archivar/membrane.rs:404`) emittiert nur bei `props.omega_g` (stype7) oder `props.gm`;
    die deployte `ephemeris_de440_earth.bin` trägt Maske bits 0–8, Bit 11 klar. Rat 2026-10-06:
    Mechanismus (a) — `val` = gemessener GM, `force_type = 1.0`; Riss „Maske sagt absent, Quelle
    trägt gm", `pending`. `omega.rs:872` konsumiert bereits `v.frame_body` (mountain-239).
- **Blockade:** der gemessene GM fehlt in der `.bin` — Mountains Parser-/`de_compiler`-Akt.
- **Braucht:** Mountain setzt slot `f(11)`/Maske Bit 11; Mycelium baut + manifestiert;
  Rivers Checkmark ist `nearCount(<1e13 m) > 0`.

### Membran — Ephemeriden-Fetch ohne Fortschritt (window edit → Operator-Wort)
- **Status:** operator-gebunden | **Bindung:** eigen (Membran-Pfad)
- **Trigger:** Operator-Wort für den `static/membrane.html`-Edit.
- **Lage:** (gemessen 2026-10-06, River 105 via `sread static/membrane.html:456-461`) `boot()`
  fetcht die drei `ephemeris_de440_<body>.bin` über `BODIES.map(async …)` **ohne Label**
  (`fetchBytes(path)`), während der Stars-Fetch ein Label trägt (`:434`). Der Status bleibt
  deshalb auf `anchoring bodies…` (`:452`) stehen, solange die drei Fetches laufen — derselbe
  eingefrorene Text, den der Operator als Hänger las. `fetchBytes` malt nur mit Label Fortschritt
  (`:100`). Die `.bin` sind groß (Survey: `cache/omegaflow_eph_sun.bin` 193.7 MB) → langer
  Fetch ohne sichtbare Aktivität.
- **Blockade:** `static/membrane.html` ist Membran-/Fenster-Pfad — kein Edit ohne Operator-Wort.
- **Braucht:** Operator-Wort; Edge liegt bereit: `fetchBytes("/ephemeris_de440_" + body + ".bin",
  "anchoring " + body + "…")` (Fortschritt je Körper).

### Agnosis — Membran-Trio (Rest (a))
- **Status:** wartend (fremd) | **Bindung:** eigen (cross-line: Mycelium, CI)
- **Trigger:** Mycelium/CI-Build-Time-Manifest der Hüllen-Pipeline (`static/membrane.html:43`).
- **Lage:** (gemessen 2026-10-06) Punkt (b) gebaut (mountain-239):
  `Volume.frame_body: Option<String>` (`src/archivar/volume.rs:155`), `Extract::Volume.frame_body`
  aus `at` (`src/archivar/parse.rs:353`), `upload_volumes` konsumiert `v.frame_body`
  (`src/mathematikerin/omega.rs:872`), Test `volume_observer_declared_and_refused_when_absent`.
  Offen nur (a): `static/membrane.html:43` `const BODIES = ["earth","moon","sun"]` →
  Build-Time-Manifest.
- **Blockade:** (a) ist Mycelium/CI (kein River-Fenster-Edit ohne Operator-Wort).
- **Braucht:** s. `## An mycelium`.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon (aus `state/zustand/wartend.φ` portiert)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon).
- **Lage:** (gemessen 2026-10-06, River 105; wartend.φ:34-36)
  - **OMNI2:** HAPI-Code 1201, alle 26 Zellen `pending`; Rohdatei
    `data/cdaweb.gsfc.nasa.gov/omni2-20260926-20260929.csv` liegt lokal
    (EXISTS gemessen 2026-10-06); Addendum `docs/paper/flyby-path-2-addendum-2026-09-29.md`
    §OMNI2 `:182` wartet auf den Merge-Lag.
  - **kp `def`:** `def`-Query leer; Prelim revidierte Zellen 23–25 auf 0.667; `def` `pending`.
  - **JUICE-recon:** `data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin` absent; Wiedervorlage
    2026-11-01, dann erneut `archive_search --verdict`.
- **Blockade:** externe Kanäle; kein Polling.
- **Braucht:** (1) `flyby_path2_fill`-Lauf (CI) lesen + Addendum-Zellen fortschreiben;
  (2) OMNI2/kp/JUICE-Trigger feuern lassen; (3) Δ/σ_recon post-flyby.

## An mycelium

Origin: river folge101 (getragen über folge102/103/104/105).

- **DE440-`.bin` remanifestieren.** Nach Mountains `de_compiler`-GM-Landung die
  `ephemeris_de440_{earth,moon,sun}.bin` (und die Geschwister) neu bauen und über die CI zur
  CDN bringen; `pages-deploy.yml` stagt sie same-origin. Checkmark ist Rivers Browser-Re-Messung
  `nearCount(<1e13 m) > 0`.
- **`static/membrane.html:43` BODIES-Handkopie** (`["earth","moon","sun"]`) → Build-Time-Manifest
  aus der Hüllen-Pipeline. Kein River-Fenster-Edit (Kante); Rivers Agnosis-Rest (a).
- **Measured (river 102):** `flyby-odf-cdn 37427673360 @2f44a6092` = **success**
  (`ci_manage view`) — der frühere `37305400435`-Fehler (leerer Census) ist abgelöst.

## LOCK

- keine.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `docs/handover/handover-2026-10-06-river-folge105.md`
- `docs/handover/archiv/handover-2026-10-06-river-folge104.md` (Move aus `docs/handover/`)

Verifikation/Dispatches: kein Rust-Eingriff in diesem Atom (alle eigenen Punkte
trigger-gebunden). `register_lookup --fired river` = `em-apertur` FIRED_UNGEMESSEN
(Trigger `ci-check` pending, nicht gefeuert); `--stale river --persist 3` = 0;
`--addressed river` = 2 (future-183, mycelium-240), gefaltet; `open_points_check
docs/handover/archiv/handover-2026-10-06-river-folge104.md` = 0 absent, 0 stale-citations;
`git_safety --snapshot` s. u. HEAD `b71658f` == `origin/main`; nur fremder
uncommitteter Hunk `opencode.json` im Baum (nicht berührt).

## Burn: open 0.0000 · close 0.0486 — Session-Burn (`session_burn`: eigene Session-Zeile „River-Linie starten und Übergabe abarbeiten" $0.0486; Runde total 0.5318 → 0.6641; gemessen 2026-10-06)
