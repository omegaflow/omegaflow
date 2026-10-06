<!--
  title: Handover — River-Folge 117 (2026-10-07)
  session: River-Folge 117
  class: handover
  date: 2026-10-07
  sha256: 8987559e6eb15f2536d40969022721830e0e1513ac8cd71c521d186037681d17
  status: live
-->
# Handover — River-Folge 117 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie in einem Pass." | 2026-10-07 | Operator (Session, River 117) — Session-Start, Delegations-Consent
„Ja (River entscheidet): ich messe zunächst eine neue cgm_lat-Route und deklariere dann die Grenzen vor dem Lauf." | 2026-10-07 | Operator (Session, River 116) — GIC-Bandgrenzen River-eigen
„river klärt es selbst" | 2026-10-07 | future-188 (gefaltet) — die ozzy-A/B-Form ist River-eigen
„das klingt doch vernüftig, oder?" | 2026-10-07 | Operator (Session, River 115) — A ist die Messung, B bleibt Leckage-Diagnose
„nein qwen ist nicht meistvertraut claude glm und kimi sind meistvertraut" | 2026-10-07 | Operator (Session, River 115) — Vertrauens-Set Claude · GLM · Kimi
„bitte befrage die vioces und die ui chats" | 2026-10-07 | Operator (Session, River 115) — zweiter Kanal nach dem Rat
„ich möchte übrigens dass die membran steht bevor wir uns irgendwo bewerben … und sie stehen vor der sonne" | 2026-10-05 | Operator (Future 181, gefaltet) — das Fenster-Wort der Startansicht
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-river-folge116.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie; das Siegel ist Operator-Wort, offen bis dahin.
- `docs/surveys/survey-2026-10-03-exzellenz-gate.md` — Label geschlossen (`:111`); Träger dieser Linie (sensory-244 gefaltet).
- `state/stimmen/2026-10-07_ozzy-witness-stimmen.md` — Rohmaterial der ozzy-Befragung (privat, gitignored).

## Offen (aufgeschlüsselt)

### `ozzy` — Negative Fuzzy Engine (A/B + Zeugen-Vokabular gebaut; Surrogat-Floor + CI-Verifikation offen)
- **Status:** eigen (CI-Verifikation) | **Bindung:** eigen
- **Trigger:** `ci-check` grün am HEAD `4776cc9ec`; danach Surrogat-Floor-Korrektur.
- **Lage:** (gemessen 2026-10-07 via `ci_manage status`/`log`) Rivers Clippy-Anteil geheilt (`ozzy.rs:90`/`:109`, `least_squares.rs:15`); `cargo check` 0/0. Am gemessenen HEAD `4776cc9ec` rot: `ci-gate 37547718075` an Mountains `src/archivar/eionet_cdr.rs:333` (`assertions_on_constants` + `ineffective_bit_mask`); `ci-check 37546727313` hängt `pending` (seit 23:28Z), der HEAD-Push reiht sich in die Concurrency-Gruppe `ci-check-refs/heads/main` (`cancel-in-progress: false`) dahinter. **Mountain heilte den Carrier in `03a1289b0` (mountain 255)** — der Trigger wartet jetzt auf einen grünen Lauf am neuen HEAD `2bddc050f`. Trigger **nicht** gefeuert.
- **Riss (getragen, ungeglättet):** `topological_te_phase` trägt fix `n_surr: 10` und `mean+2σ` — querdurch alle 12 Kanäle zu schwach (≥99, teils ≥199; Rang-/Permutationstest; `N_eff` statt `N − Rang`; `None` getypt mit Grund). Sonnets B-Null-Einwand: `conditional_embedded_te_phase` nutzt `z_phase_surrogate` statt bedingter Permutation innerhalb E.
- **Blockade:** kein Mountain-Fix mehr; nur die hängende `ci-check`-Queue / ein freier Runner am neuen HEAD.
- **Braucht:** (1) grüner `ci-gate`/`ci-check` am HEAD (nach Mountains Heilung `03a1289b0`); (2) Surrogat-Floor-Korrektur `topological_te_with` (`te.rs:3309`) + `topological_te_phase` (`te.rs:3450`) + `topological_verdict_from_gpu` (`te.rs:3551`) — `n_surr` ≥99, Rang-Schwelle, `None` mit Grund (die Wire `TE_VERDICT_SLOTS = 12·6` trägt heute 10 Surrogate → WGSL + `te_verdict_bytes` + Leser mitziehen); (3) Verdrahtung in `field_te_query`/Matrix.
- **Wort:** „river klärt es selbst" | 2026-10-07 | future-188 (A/B) · „das klingt doch vernünftig" | 2026-10-07 | Operator 115 (A gewählt)

### em-Apertur — Kanal-Identität statt Kernel-Proxy
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am HEAD `4776cc9ec`.
- **Lage:** (gemessen 2026-10-07 via `ci_manage status`) River-/Mountain-Seite gebaut (`src/mathematikerin/shaders.rs:186`/`:211`); CI am HEAD nicht grün (s. `ozzy`).
- **Blockade:** keine (eigene); CI-Runner/Queue.
- **Braucht:** grüner `ci-check` am HEAD (Stehender Pass; kein Polling).

### Membran-Startansicht — zwei Aperturen (Parity-Fix gebaut; CI-Verifikation offen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`/`wasm-parity` grün am Fix-HEAD `116611e2e`; danach Pages-Deploy + Browser-Sicht.
- **Lage:** (gemessen 2026-10-07 via `ci_manage list`) Parity-Fix committed (`116611e2e` river 114); CI am HEAD nicht grün; Deploy trägt den Bau noch nicht.
- **Blockade:** CI-Lauf-Ausgang `unread` (Stehender Pass/`ci_manage`, kein Polling).
- **Braucht:** CI-grün; Pages-Deploy; Browser-Sicht auf `omegaflow.space/membrane.html`. Offen: `state.lvl` global über beide Aperturen; `MembraneLookup.add_stars` panikt bei Re-Init (Riss, kein Repro ohne WASM/Browser).

### GIC-Breitenband-Familien — Partition + Grenzen gemessen; Registerzeile + Deskriptoren offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains `cgm_lat`-Registerzeile (154); danach Familien-Deskriptoren.
- **Lage:** (gemessen 2026-10-07) Operator-Wort: River entscheidet die Grenzen; deklariert **auroral ≥60°, sub-auroral 50–60°, mid-latitude <50°** |CGM|. Partition **gemessen** (`tools/measure/src/bin/cgm_lat_partition.rs`; Artefakt `state/river/gic-cgm-lat.tsv`): **154/154, disjunkt, 0 pending** — auroral 31 · sub-auroral 25 · mid 98; Quellen 133 `omniweb-cgm` (Epoche 2025) / 19 `supermag-aacgm` (IGRF-2000, äquatorial) / 2 `bgs-quasi-dipole` (CPL/TTB); Drift OmniWeb↔SuperMAG 1.13° (130). Blatt `blatt-gic-breitenband-familien.md` (sha256 `d973a202…`), grenz-nahe Stationen benannt (WNG 49.99 · ORC −49.50 · EYR −50.09 …).
- **Blockade:** keine (eigene); die 154 Blöcke tragen nur `on earth`-Koordinaten.
- **Braucht:** (1) Mountain: `cgm_lat`-Registerzeile je Station in `phi/sources.φ`; (2) `fdr … over family`-Scope; (3) drei Familien-Deskriptoren; (4) CI-Job `field-te-query.yml`.

### Universelles Vlies — der `matrix full`-Lauf (kein Bau)
- **Status:** wartend (fremd, Alignment/Ernte) | **Bindung:** eigen
- **Trigger:** Alignment/Ernte der Solar-/Magnetosphären-Zellen (Mountain/Mycelium) — Beleg `field-te-query 37500311359 @d9351b0e` (`alignment pending`).
- **Lage:** (gemessen 2026-10-06, River 112) 210/210 Zellen, 15/15 Arme; `0 of 210 cells pass`. **Newell gefaltet (mountain-254, gemessen `ci_manage log`):** `matrix-newell` Zelle `newell_dphi_dt->intermagnet_dbdt` n=24, verdict `silent` (Bin 3600 korrekt); `matrix-newell-omni` n=0, `alignment pending` (OMNI-Zeitachse fehlt) — kein neuer Mountain-Lauf nötig.
- **Blockade:** Daten-/Kadenz-Deckung (Mountain/Mycelium).
- **Braucht:** alignment-fähige Zellen; `ozzy`; Netz-Null als CI-Batterie.

### Receiver-Apertur — Sub-Pixel für ALLE Radiatoren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Direktive `span` auf der `at <body>`-Zeile (Mountain).
- **Lage:** (gemessen 2026-10-06, River 109/110) sichtbarer Pfad halb geheilt; SPAN/Apertur-Architektur entschieden (Rat + 6 UI-Modelle). Risse: `SPAN/N` ungemessen; `Aperture` → `span_m`.
- **Blockade:** großer Umbau (per-Fragment `source_contrib`).
- **Braucht:** `span`-Direktive; Brücke in `static/membrane.html`; Invarianz-/Energieerhaltungs-Test; danach alle fünf Radiatoren.

### dB/dt–GIC-Relation Mäntsälä (Viljanen-Empfehlungen)
- **Status:** wartend | **Bindung:** eigen (cross-line)
- **Trigger:** NUR-Asset `fmi_image_mag_nur.bin` im CDN.
- **Lage:** (gemessen 2026-10-06, River 105) Fine-grain FMI-GIC manifestiert; NUR nicht im CDN.
- **Blockade:** NUR-Manifestation + Probe.
- **Braucht:** `image-cdn.yml` (Mycelium); ggf. Tages-Detrend (Mountain); Probe + Zahl in Paper §4/§6.

### Membran-Sonne-Anker (Operator-Wort; cross-line)
- **Status:** blockiert | **Bindung:** eigen (cross-line)
- **Trigger:** Mountains `de_compiler`-GM-Landung (Maske Bit 11) + Mycelium-Remanifestation.
- **Lage:** (gemessen 2026-10-06, River 106) deployte Maske `0x01FF`; Bit 11 klar; `body_anchor_samples` (`src/archivar/membrane.rs:404`) emittiert nur bei `props.omega_g`/`props.gm`.
- **Blockade:** der gemessene GM fehlt in der Sonne-`.bin`.
- **Braucht:** Mountain slot `f(11)`/Bit 11; Mycelium baut + manifestiert; Rivers Checkmark `nearCount(<1e13 m) > 0`.

### Membran — progressives Laden nach Helligkeit (C)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Katalog-Lieferung nach Helligkeit (C); Build-Time-Manifest für `static/membrane.html` (Mycelium/CI).
- **Lage:** (gemessen 2026-10-06, River 109) Receiver-Schnitt gebaut; `BODIES` ist eine geschlossene Handkopie (`static/membrane.html:51`). **Set benannt (River-Wort 2026-10-07):** `["sun","earth","moon"]` in Sichtbarkeits-Reihenfolge (sun zuerst, Operator-Gate; dann earth, moon; Sternkatalog als letzte, schwache Schicht) — die drei Körper-Hüllen-Anker, die `pages-deploy.yml:60-62` same-origin stagt (`ephemeris_de440_earth|moon|sun.bin`). Auswahlregel: genau diese `stage`-Zeilen, **kein** Blankett-Filter über `at <body>` in `phi/sources.φ` (dort 375× `at earth` = Quellen-Standort, kein Hüllen-Anker). Der Generator ist Myceliums Schritt.
- **Blockade:** 95-MB-Sternkatalog + Trio-Manifest (Mycelium/CI).
- **Braucht:** Katalog nach Helligkeit ordnen; Trio → Manifest.

### Agnosis — Membran-Trio (Rest (a))
- **Status:** wartend (fremd) | **Bindung:** eigen (cross-line Mycelium, CI)
- **Trigger:** Mycelium/CI-Build-Time-Manifest (`static/membrane.html:51` `BODIES`).
- **Lage:** (gemessen 2026-10-07) Punkt (b) gebaut (mountain-239); (a) `BODIES`-Handkopie — **Set benannt** (s. „Membran — progressives Laden", River-Wort 2026-10-07); der Generator/Workflow ist Myceliums Schritt.
- **Blockade:** Mycelium/CI (kein River-Fenster-Edit ohne Operator-Wort).
- **Braucht:** s. `## An mycelium`.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon).
- **Lage:** (gemessen 2026-10-06, River 105) OMNI2 26 Zellen `pending`; kp `def` leer; JUICE-recon absent (Wiedervorlage 2026-11-01).
- **Blockade:** externe Kanäle; kein Polling.
- **Braucht:** `flyby_path2_fill`-Lauf lesen + Addendum fortschreiben; Trigger feuern lassen; Δ/σ_recon.

### Repo-weiter Lizenz-Census + `sources`-LICENSE
- **Status:** eigen (Audit) | **Bindung:** eigen (cross-line)
- **Trigger:** Operator-Wort 2026-10-06.
- **Lage:** (gemessen 2026-10-06, River 107) `phi/sources.φ` 1945 Spiegel-URLs über 169 Netlocs; keine Lizenz-Direktive. `state/river/license-census.tsv` (169 Zeilen).
- **Blockade:** keine.
- **Braucht:** Leads als `terms`-Zeilen (Mountain); Generator + Drift-Tor (Mycelium).

## An mycelium

Origin: mycelium-250/252 (gefaltet) · river-117.

- **`BODIES`-Set benannt (River-Wort 2026-10-07)** für das Build-Time-Manifest aus der Hüllen-Pipeline: `["sun","earth","moon"]`, Sichtbarkeits-Reihenfolge (sun zuerst, Operator-Gate; dann earth, moon; Sternkatalog als letzte, schwache Schicht). Quelle der Menge: die `stage`-Zeilen `.github/workflows/pages-deploy.yml:60-62` (`ephemeris_de440_earth|moon|sun.bin`) — **nicht** der `at <body>`-Blankett-Filter (`phi/sources.φ` trägt 375× `at earth` als Quellen-Standort, keinen Hüllen-Anker). Du baust den Generator/Workflow, der `static/membrane.html:51` aus dieser Menge erzeugt; die Zitat-Zeile `phi/sources.φ:3450-3469` im Kommentar `membrane.html:45-47` ist veraltet und fällt im selben Schritt.
- **`ozzy` `Debug`/Clippy:** `Kanal` derivt `Debug` (`ozzy.rs:4`); Rivers Clippy-Anteil (`ozzy.rs:90`/`:109`, `least_squares.rs:15`) in river-116 geheilt.
- **`pages-deploy` — DE440-Remanifest vs. Pin:** Pins gesetzt; `sha256`-Direktive je DE440-Zeile ist Mountains Verdikt-Zeile. Ausgang `ci_manage view 37508187212`.
- **Generiertes `LICENSE` im `omegaflow/sources`-Repo** — nach Mountains `terms`-Zeilen.

## An mountain

Origin: mountain-folge252/253/254 (gefaltet) · river-117.

- **Newell-Zelle:** Lauf `37545120597` — `matrix-newell` Zelle n=24, verdict `silent` (Bin 3600 korrekt); `matrix-newell-omni` n=0, `alignment pending` (OMNI-Zeitachse). Kein neuer Lauf von Mountain-Seite nötig; River liest mit. Gefaltet in „Universelles Vlies".
- **`ci-gate` rot @HEAD `4776cc9ec` — geheilt:** `src/archivar/eionet_cdr.rs:333` (`assertions_on_constants` + `ineffective_bit_mask`, gemessen `ci_manage log 37547718075`); dein Fix `03a1289b0` (mountain 255) steht. Rivers `ci-check`-Trigger wartet auf den grünen Lauf am neuen HEAD `2bddc050f`.
- **GIC `cgm_lat`:** Partition gemessen (154/154, `state/river/gic-cgm-lat.tsv`, Bin `tools/measure/src/bin/cgm_lat_partition.rs`); die per-Station-Registerzeile in `phi/sources.φ` ist deine Feder — Blatt `docs/blatt/blatt-gic-breitenband-familien.md` (sha256 `d973a202…`).

## An future

Origin: future-188 (gefaltet) · river-117.

- **ozzy A/B:** mit future-188 River-eigen (`Wort: river klärt es selbst`); gebaut bleibt A als Messung, B als Leckage-Diagnose. Kein Operator-Wort mehr nötig.
- **Membran-Startansicht:** Parity-Fix committed (`116611e2e`), CI-Verifikation offen; der LOCK-Trigger „Startansicht zeigt die Sonne" kann erst nach CI-grün + Deploy feuern.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `docs/handover/handover-2026-10-07-river-folge117.md` (neu) · `docs/handover/archiv/handover-2026-10-07-river-folge116.md` (Move)

## Burn: open 0.0010 · close 0.0434 · cap 0.15 (Default) · Grund: River 117 — Line-Session (deepseek-flash): addressed Blöcke gefaltet (future-188, mountain-254, mycelium-252, sensory-244); CI am HEAD `4776cc9ec` gemessen (`ci-check` hängt, `ci-gate` rot an Mountains `eionet_cdr.rs:333`) → kein Trigger gefeuert, kein Bau; `BODIES`-Set benannt (River-Wort). Kein pro/max-Dispatch. `cargo check` nicht nötig (keine Code-Änderung).
