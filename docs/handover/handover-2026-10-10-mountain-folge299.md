<!--
  title: Handover — Mountain-Folge 299 (2026-10-10)
  session: Mountain-Linie in einem Pass abarbeiten
  class: handover
  date: 2026-10-10
  sha256: 3e3d68731283d978847e5d1fb51800133b278e45bc46ec200ec34d39e578d6b4
  status: live
-->
# Handover — Mountain-Folge 299 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-10-mountain-folge298.md` (→ `archiv/`). Der
`## An mountain`-Block aus mycelium-293 ist bereits in folge298 gefaltet (iEEG-Riss
beigelegt; das registrierte Wort 2026-10-06 bleibt maßgeblich). flash only, kein pro/max.

## Burn: open 0.0000 · close 0.2807 · cap 0.40 — Grund: voller Roster (8 Frontier-Seats + Open-Weight zweimal) + zwei Ratssitzungen + vorgezogener LLR-Parser-Bau + Ephemeriden-Quellen-Recherche (Operator-Wort), deepseek-flash

## Offen (aufgeschlüsselt)

### GIC-Estimator — Ground-Truth auf embedded Production-Arm umgestellt
- **Status:** eigen (Paper/Mathematikerin) | **Bindung:** mycelium (CI-Lauf)
- **Trigger:** CI-Lauf des Ground-Truth-Bins (kein Workflow führt ihn)
- **Lage:** (gemessen 2026-10-10, unverändert) `te-bias-n` `38032560457`: der embedded/KSG-Arm
  (Takens dim 3, Production-Flux `omega.rs:489`) nullt den Rückkanal (TE(Y→X) = −1.4271e-2,
  n = 10000); der Skalar-KDE-Arm trägt den finite-sample-Rückkanal. `te_ground_truth.rs` auf den
  embedded-Arm umgestellt, `cargo build` grün.
- **Blockade:** kein Workflow führt `te_ground_truth` aus.
- **Braucht:** Mycelium — `te_ground_truth` in einen Workflow aufnehmen + Lauf; danach Paper
  §3.5/Abstract/§7 nachziehen (Mountain).

### particle-cern — Rat-Verdikt: `parser-def` bleibt, Bau-Schuld; erste Messung steht
- **Status:** eigen (Disposition/Parser) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt (Event-Dir → TStreamerInfo/TBranch/TLeaf)
- **Lage:** (gemessen 2026-10-10) `cern_root_compiler --probe AliVSD_Masterclass_6.root` →
  **34 Keys = 33 `TDirectoryFile` (EventXXXX) + 1 `TDirectory`**; Top-Level trägt **keinen**
  TTree — der TTree liegt je Event-Dir. Rat (5 Stimmen) + 5/7 UI-Seats (Claude, GLM-5.3,
  DeepSeek V4 Pro, GPT-OSS 120B, lokaler Rat) tragen: `blocked parser-def` bleibt, wandert von
  der Mauer zur **Bau-Schuld**; 2 Seats (Qwen, Duck.ai) votieren `descoped`. Riss benannt, nicht
  geglättet. Note: `phi/blocked_sources.φ:61`; Stimmen: `state/stimmen/2026-10-10_mountain_particle-cern-round.md`.
- **Blockade:** kein TTree am Top-Level (Descend in Event-Dirs nötig); Indexdomäne ungeklärt.
- **Braucht:** Schritt 2 — Event-Dir öffnen, TStreamerInfo-Record + fBranch/fLeaf enumerieren
  (uproot5 als Dev-Orakel, Hash pinnen); danach Gate auf Indexdomäne (time|ordinal|event) vor dem
  Skalar-Reader. Kein Verdikt über das Feld vor der Leaf-Messung.

### PEP — Werkzeug, kein fünftes Haus; 100 % Rust std gilt dem Shipped Binary
- **Status:** eigen (Register/Tool) | **Bindung:** eigen
- **Trigger:** Registrierungsort festlegen, dann `pep_core` pinnen; native Module bauen
- **Lage:** (gemessen 2026-10-10) Rat (5 Stimmen) + voller Roster (10 Frontier + 1 Open-Weight; Lumo Limit-Wall,
  Kimi Login-Wall = `pending`): PEP bleibt **offlines, gepinntes Referenz-/Zeugen-Artefakt** (Golden-Fixture,
  Hash + Provenienz), **kein Runtime-Oracle** (Subprozess/FFI bräche das Ziel); **kein Vollport jetzt**. Das Ziel
  ist präzise „**Shipped Binary = 100 % Rust**"; die Verifikationskette darf fremd bleiben. Nativ zuerst der
  **Beobachtungsoperator** über den eigenen Rohdaten, dann modulweise (Integrator zuletzt) mit Konformanz-Gate
  (Toleranz je Modul, nie bitweise). Die „1,7 TB" sind CDN-Assets, nicht Beobachtungsdaten (Riss benannt).
  Stimmen: `state/stimmen/2026-10-10_mountain_pep-rust-port-round.md`.
- **Blockade:** Registrierungsort für ein Tool (≠ Datenquelle) unbestimmt; PEP-Quellenlizenz für Pinning ungeprüft.
- **Braucht:** Rat/Operator — den Ort der PEP-Registrierung festlegen (z. B. `docs/concepts/tools-map.md`) +
  Gate-Fixture „kein Runtime-Fremd-Binary im Shipped-Binary"; dann `pep_core` als Golden-Fixture pinnen.

### LLR — Parser gebaut; Manifestation + Runtime-Arm offen
- **Status:** eigen (Parser) | **Bindung:** mycelium (sources.φ + Workflow) · river (main_flow)
- **Trigger:** Mycelium schreibt den `sources.φ`-Block + `llr-cdn.yml`; River setzt `| "llr"` in `main_flow.rs`
- **Lage:** (gemessen 2026-10-10) `src/archivar/llr.rs` + `tools/harvest/src/bin/llr_compiler.rs` **gebaut**,
  `cargo check` 0/0 (Commit `2b00d7666`); CRD v2.01 gemessen (H1–H5/C0–C3/11/20/40/50); End-to-End auf
  `APOLLO_2006_2020.crd` (Zenodo 7818557, CC-BY-4.0, DOI 10.5281/zenodo.7818557): **4296 Normalpunkte**,
  Roundtrip 360 872 B, sha256 `f2b3afdc…`. Reiner Parser (null Physik), Ausreißer nur markiert.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Mycelium — `sources.φ`-Block (`netloc zenodo.org` · `url …/APOLLO_2006_2020.crd/content` ·
  `terms CC-BY-4.0` · `format llr` · `origin APOLLO normal point data 2006–2020 (Zenodo 7818557)` ·
  `compiler tools/harvest/src/bin/llr_compiler.rs`) + `llr-cdn.yml`; River — `| "llr"` in `main_flow.rs` `series_rows`.

### Ephemeriden-Quellen — Schließungsliste (Survey)
- **Status:** eigen (Quellen/Register) | **Bindung:** eigen (CDDIS/Earthdata = operator/future)
- **Trigger:** Harvest der Schließungsliste; CDDIS-Daten hinter Earthdata-Login
- **Lage:** (gemessen 2026-10-10) `archive_search --all` + Rat + voller Roster (Z.ai/Lumo/Kimi/Open-Weight
  `pending`): die größten Lücken für eine eigene Ephemeride sind **LLR-Normalpunkte** (ILRS/CDDIS, historisch
  via POLAC `TOTALOBS6913`), **ITRF2020**-Stationen, **JPL-Planeten-Radar** (`ssd.jpl.nasa.gov/planets/obs_data.html`),
  **VMF1/VMF3**-Troposphäre, **ICRF3** (Rahmen). Verzichtbar: VLBI-Roh/DDOR, echo.jpl-Asteroidenradar.
  Volle Tabelle: `docs/surveys/survey-2026-10-10-ephemeris-quellen.md`.
- **Blockade:** CDDIS/Earthdata-Login (`blocked account` → Operator/future); Beobachtungsoperator (nativ Rust) fehlt.
- **Braucht:** nächste Harvest in Reihenfolge — (1) LLR-Historie POLAC/CDDIS (Parser da; ggf. MINI→CRD),
  (2) ITRF2020 SINEX, (3) JPL-Planeten-Radar, (4) VMF3; je `parser-def` prüfen und als `sources.φ`-Block + Workflow (Mycelium).

### Flyby-Kette — Residual in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-09, unverändert) 157 ODF-Referenzen; `doppler.rs` absent; Wahrheit
  `state/zustand/wartend.φ:34`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### iEEG — Riss beigelegt: registriertes Wort 2026-10-06 maßgeblich
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort, das den Riss über 2026-10-06 hebt
- **Lage:** (gemessen 2026-10-10) iEEG = privates Experiment (`state/zustand/wartend.φ:40`), kein
  CDN; die Mountain-Seite (`eeglab::eeg_from_bin` akzeptiert `Samples::Double`) ist gebaut.
- **Blockade:** keine.
- **Braucht:** kein Schritt — nur ein neues Operator-Wort öffnet es.

### CMB/SPT — Transfer-Bound gebaut; Mycelium hebt Job-Timeout + Re-Lauf
- **Status:** eigen (Compiler) | **Bindung:** mycelium (Job-Timeout)
- **Trigger:** Mycelium hebt `cmb-cdn` `timeout-minutes` und startet den Re-Lauf
- **Lage:** (gemessen 2026-10-10, unverändert) `cmb_planck_compiler` lädt `full_maps_d1.tar.bz2`
  über `fetch_raw_bytes_with(..., SPT_TRANSFER_BOUND_S = 6 h)`; `--verdict`: HTTP 200,
  `Content-Length 7 873 515 864`; Job-Timeout 240 min genügt bei ~0.47 MB/s nicht.
- **Blockade:** Mycelium-Job-Timeout.
- **Braucht:** Mycelium: `cmb-cdn.yml` `timeout-minutes` ≥ 360 + Re-Lauf.

### GIRO DIDBase — Compiler gebaut; Manifestation = Mycelium
- **Status:** eigen | **Bindung:** mycelium (Block)
- **Trigger:** Mycelium-Workflow-Lauf `giro-fastchar-cdn`
- **Lage:** (gemessen 2026-10-10) Compiler grün: `src/archivar/giro_fastchar.rs` + 
  `tools/harvest/src/bin/giro_fastchar_compiler.rs`; Endpoint `https://lgdc.uml.edu/fastchar/getbest`
  (HTTP 200, kein Key); Lizenz CC BY-NC-SA 4.0.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Mycelium: `giro-fastchar-cdn` Re-Lauf.

### PDS-PPI — `quantity`-Projektion gebaut; Block = Mycelium
- **Status:** eigen | **Bindung:** mycelium (Block)
- **Trigger:** Mycelium schreibt den sources.φ-Block
- **Lage:** (gemessen 2026-10-10) `units.rs` + `pds_ppi_compiler.rs` emittieren je Spalte eine
  `quantity`-Zeile aus dem gemessenen PDS4-`<unit>`; Spalten ohne Label-Einheit bleiben `pending`.
- **Blockade:** keine.
- **Braucht:** Mycelium: PDS-PPI-Block (manifest + `quantity`-Zeilen).

### Keogramm ABK — Compiler rückwärts-bounded; Re-Lauf = Mycelium
- **Status:** eigen (Compiler) | **Bindung:** mycelium (Re-Lauf)
- **Trigger:** Mycelium `keogram-cdn` Re-Lauf
- **Lage:** (gemessen 2026-10-10, unverändert) `keogram_compiler` sucht die jüngste verfügbare
  Nacht; FMI endet `ABK.2604`; mycelium-293 meldet `keogram-cdn 38032914527` success.
- **Blockade:** keine.
- **Braucht:** Mycelium: Re-Lauf bestätigt (kein Mountain-Akt).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern; Lauf lokal/silent,
  nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen:
  der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„hast du archive search all und den roster befragt?" | 2026-10-10 | Operator (Session, Mountain 298)
„bitte für council immer auch archive search all und den roster und bitte lasse archive search all und den roster auch auf LLR los" | 2026-10-10 | Operator (Session, Mountain 299)
„aber warum bauen wir PEP nicht in rust nach? und bitte ja LLR … sag, ob ich ihn vorziehen soll" | 2026-10-10 | Operator (Session, Mountain 299)
„warum nur so ein kleiner roster und warum kein vollport wir wollen doch womöglich 100% rust std" | 2026-10-10 | Operator (Session, Mountain 299)
„und dann möchte ich dass du nochmal eine -all und roster recherche machst welche referenzen wir noch harvestenn können um unsere eigenen ephemeriden zu bauen?" | 2026-10-10 | Operator (Session, Mountain 299)
„warum schreibst du hier erst in die blocked sources anstatt direkt an den finalen ort — das ist einfach nur faules compliance theater" | 2026-10-10 | Operator (Session, Mountain 295)
„Offen bleibt in P10: der optionale `<regime>`-Token …, die Regime-Achse selbst, und die Zeilen-Migration in phi/sources.φ (Mountain)." | 2026-10-10 | Operator (Session, Mountain 295)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater … kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–290)
„mach das ab jetzt automatisch — committe und pushe selbst" | 2026-10-07 | Operator (Session, Mountain 264)

## An mycelium

Origin: mountain-299 (2026-10-10).

- **GIC-Ground-Truth.** `tools/measure/src/bin/te_ground_truth.rs` steht auf dem embedded KSG-Arm
  (Production-Flux); der Skalar-KDE-Arm bleibt als benannter Riss. Kein Workflow führt diesen Bin
  aus — nehmt ihn in einen CI-Lauf auf (z. B. neben `te_bias_n_probe`).
- **LLR.** `src/archivar/llr.rs` + `tools/harvest/src/bin/llr_compiler.rs` sind gebaut (`2b00d7666`,
  `cargo check` 0/0). Braucht den `sources.φ`-Block (netloc `zenodo.org`, format `llr`, terms CC-BY-4.0)
  + einen `llr-cdn.yml`-Caller (unter `cdn-manifest.yml`).
- **CMB/SPT · GIRO · PDS-PPI · Keogramm.** Unverändert aus folge298: `cmb-cdn` `timeout-minutes`
  ≥ 360 + Re-Lauf; `giro-fastchar-cdn` Re-Lauf; PDS-PPI-Block; Keogramm-Re-Lauf bestätigt.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („committe und pushe selbst", 2026-10-07)
trägt Commit und Push. Eigene Pfade dieses Atoms: `phi/blocked_sources.φ` ·
`docs/handover/handover-2026-10-10-mountain-folge299.md` ·
`docs/handover/archiv/handover-2026-10-10-mountain-folge298.md` (Move). Die IRIS-Register-Zeile
wurde geschlossen (Quelle `phi/sources.φ:4358` + `.github/workflows/iris-cdn.yml` + `phi/harvest.φ:297`
existieren; `UNIT="count"` gemessen-korrekt, ITN26 §5.2).
