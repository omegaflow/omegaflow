<!--
  title: Handover — River-Folge 118 (2026-10-07)
  session: River-Folge 118
  class: handover
  date: 2026-10-07
  sha256: 094cfd69173e3fa8905ff016e11b23520d9f4eb2c72ccc42f7185e01ae64b3d2
  status: live
-->
# Handover — River-Folge 118 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie in einem Pass." | 2026-10-07 | Operator (Session, River 118) — Session-Start, Delegations-Consent
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie in einem Pass." | 2026-10-07 | Operator (Session, River 117) — Session-Start
„Ja (River entscheidet): ich messe zunächst eine neue cgm_lat-Route und deklariere dann die Grenzen vor dem Lauf." | 2026-10-07 | Operator (Session, River 116) — GIC-Bandgrenzen River-eigen
„river klärt es selbst" | 2026-10-07 | future-188 (gefaltet) — die ozzy-A/B-Form ist River-eigen
„das klingt doch vernüftig, oder?" | 2026-10-07 | Operator (Session, River 115) — A ist die Messung, B bleibt Leckage-Diagnose
„nein qwen ist nicht meistvertraut claude glm und kimi sind meistvertraut" | 2026-10-07 | Operator (Session, River 115) — Vertrauens-Set Claude · GLM · Kimi
„bitte befrage die vioces und die ui chats" | 2026-10-07 | Operator (Session, River 115) — zweiter Kanal nach dem Rat
„ich möchte übrigens dass die membran steht bevor wir uns irgendwo bewerben … und sie stehen vor der sonne" | 2026-10-05 | Operator (Future 181, gefaltet) — das Fenster-Wort der Startansicht
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-river-folge117.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie; das Siegel ist Operator-Wort, offen bis dahin.
- `docs/surveys/survey-2026-10-03-exzellenz-gate.md` — Label geschlossen (`:111`); Träger dieser Linie (sensory-244 gefaltet).
- `state/stimmen/2026-10-07_ozzy-witness-stimmen.md` — Rohmaterial des ozzy-Konsenses (privat, gitignored).

## Offen (aufgeschlüsselt)

### `ozzy` — Negative Fuzzy Engine (A + Zeugen-Vokabular gebaut; Surrogat-Floor offen)
- **Status:** wartend (CI) | **Bindung:** eigen
- **Trigger:** `ci-check`/`ci-gate` grün am jeweiligen HEAD.
- **Lage:** (gemessen 2026-10-07 via `ci_manage list`/`log`) `independence_verdict` (`src/mathematikerin/ozzy.rs:146`) trägt den A-Test (held-out Zeugen) + B-Diagnose + die zwei Known-Answer-Gates. Rivers Clippy-Anteil geheilt (`ozzy.rs:90`/`:109`, `least_squares.rs:15`); `cargo check` 0/0. `register_lookup --fired river` meldete `em-apertur`/`ozzy` als gefeuert — **Lesung** (s. u.): CI am HEAD rot, Trigger **nicht** gefeuert.
- **Floor-Konsens (gemessen, `state/stimmen/2026-10-07_ozzy-witness-stimmen.md:82-87`):** quer durch **alle** Kanäle ist `n_surr = 10` + `mean+2σ` zu schwach — **≥99** (teils ≥199); **Rang-/Permutationstest** statt z-Schwelle; **`N_eff`** statt `N − Rang`; **`None` getypt mit Grund**, nie roh; Guardrail `≥8 von 10` statt `<2`; die **ganze Pipeline pro Surrogat** neu; B-Null braucht **bedingte Permutation innerhalb E** (Sonnet). A ist gewählt (7× zweite Runde), B bleibt Leckage-Diagnose, `(A−B)` verworfen.
- **Riss (getragen, ungeglättet):** die konkrete Rang-Schwelle (Quantil vs. `max`) und der `N_eff`-Schätzer sind nicht festgelegt; die GPU-Wire trägt heute 10 cross-channel Surrogate.
- **Blockade:** kein CI-Runner grün; der Floor-Umbau ist eine mehrschrittige Konstruktion (TE-/Null-Atom).
- **Braucht:** (1) grüner `ci-check`/`ci-gate` am HEAD; (2) bounded: getypte Absenz → `TE_SURR_FLOOR = 99` → Rang-Threshold → `N_eff` in `topological_te_with` (`te.rs:3309`) / `topological_te_phase` (`te.rs:3450`); (3) GPU-Wire `TE_VERDICT_SLOTS = 12·6` → 99 Surrogate (`te.rs:3551` + `machines/verdict.rs` + `shaders.rs:798`); (4) Verdrahtung `field_te_query`/Matrix. Kein Pro-Solo — der Floor geht durch die Linse.
- **Wort:** „river klärt es selbst" | 2026-10-07 | future-188 (A/B) · „das klingt doch vernünftig" | 2026-10-07 | Operator 115 (A gewählt)

### em-Apertur — Kanal-Identität statt Kernel-Proxy
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am jeweiligen HEAD (der Stand „`4776cc9ec`" ist überholt).
- **Lage:** (gemessen 2026-10-07 via `ci_manage list`) River-/Mountain-Seite gebaut (`src/mathematikerin/shaders.rs:186`/`:211`). `register_lookup --fired` meldete den Punkt als gefeuert — die Lesung widerlegt es: `ci-gate 37549578768` **rot** @HEAD `7304810f8`; am HEAD `8a11fc3fb` laufen `ci-gate 37550073951`/`ci-check 37550073836` (Ausgang `unread`). Trigger nicht gefeuert.
- **Blockade:** keine (eigene); CI-Runner/Queue.
- **Braucht:** grüner `ci-check` am HEAD (Stehender Pass; kein Polling).

### Membran-Startansicht — zwei Aperturen (Parity-Fix gebaut; CI-Verifikation offen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`/`wasm-parity` grün am Fix-HEAD `116611e2e`; danach Pages-Deploy + Browser-Sicht.
- **Lage:** (gemessen 2026-10-07 via `ci_manage list`) Parity-Fix committed (`116611e2e` river 114); CI am HEAD nicht grün; Deploy trägt den Bau noch nicht.
- **Blockade:** CI-Lauf-Ausgang `unread` (Stehender Pass/`ci_manage`, kein Polling).
- **Braucht:** CI-grün; Pages-Deploy; Browser-Sicht auf `omegaflow.space/membrane.html`. Offen: `state.lvl` global über beide Aperturen; `MembraneLookup.add_stars` panikt bei Re-Init (Riss, kein Repro ohne WASM/Browser).

### Membran — progressives Laden nach Helligkeit (C)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lieferung des nach Helligkeit geordneten Sternkatalogs (C) — Register `phi/sources.φ`, Mycelium/CI.
- **Lage:** (gemessen 2026-10-07 via `git show 8a11fc3fb`) **`BODIES`-Manifest gebaut (mycelium 254):** `.github/workflows/scripts/gen_bodies.sh` erzeugt die `BODIES`-Zeile (`static/membrane.html:51`) aus den `pages-deploy.yml`-Stage-Zeilen und ist dort gebunden (Drift-Tor: staged bodies vs. Zeile). Das **Set ist River-Wort** `["sun","earth","moon"]` in Sichtbarkeits-Reihenfolge (sun zuerst, Operator-Gate; dann earth, moon; Sternkatalog zuletzt) — Auswahlregel: genau `stage`-Zeilen `ephemeris_de440_<body>.bin`, **kein** `at <body>`-Blankett-Filter (`phi/sources.φ` 375× `at earth` = Quellen-Standort). Offen bleibt: 95-MB-Sternkatalog nach Helligkeit ordnen (C).
- **Blockade:** Katalog-Ordnung (C).
- **Braucht:** 95-MB-Sternkatalog nach Helligkeit ordnen.

### GIC-Breitenband-Familien — Partition + Grenzen gemessen; Registerzeile + Deskriptoren offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains `cgm_lat`-Registerzeile (154); danach Familien-Deskriptoren.
- **Lage:** (gemessen 2026-10-07) Operator-Wort: River entscheidet die Grenzen; deklariert **auroral ≥60°, sub-auroral 50–60°, mid-latitude <50°** |CGM|. Partition **gemessen** (`tools/measure/src/bin/cgm_lat_partition.rs`; Artefakt `state/river/gic-cgm-lat.tsv`): **154/154, disjunkt, 0 pending** — auroral 31 · sub-auroral 25 · mid 98; Quellen 133 `omniweb-cgm` (Epoche 2025) / 19 `supermag-aacgm` (IGRF-2000, äquatorial) / 2 `bgs-quasi-dipole` (CPL/TTB); Drift OmniWeb↔SuperMAG 1.13° (130). Blatt `blatt-gic-breitenband-familien.md` (sha256 `d973a202…`), grenz-nahe Stationen benannt (WNG 49.99 · ORC −49.50 · EYR −50.09 …). `cgm_lat_partition.rs` rustfmt-Drift (nur Format) in dieser Session committet.
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

### Agnosis — Membran-Trio (Rest (a))
- **Status:** wartend (fremd) | **Bindung:** eigen (cross-line Mycelium, CI)
- **Trigger:** Mycelium/CI-Build-Time-Manifest (`static/membrane.html:51` `BODIES`).
- **Lage:** (gemessen 2026-10-07 via `git show 8a11fc3fb`) Punkt (b) gebaut (mountain-239); (a) **`BODIES`-Handkopie → Build-Time-Manifest erledigt** (mycelium 254: `gen_bodies.sh` + Bindung in `pages-deploy`, Drift-Tor). Set benannt (River-Wort 2026-10-07). Offen bleibt nur die Sternkatalog-Schicht (C).
- **Blockade:** Katalog-Ordnung (Mycelium/CI).
- **Braucht:** s. „Membran — progressives Laden".

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

## An mountain

Origin: mountain-folge254/255/256 (gefaltet) · river-118.

- **`ci-gate` rot @HEAD `7304810f8` — neuer Befund (gemessen `ci_manage log 37549578768`):** `src/archivar/eionet_cdr.rs:486` (Test) — `parse_report(...).expect_err(...)` verlangt `GeoRec: Debug`; `GeoRec` (`src/archivar/geo.rs:280`) trägt kein `#[derive(Debug)]`. Compile-Analyse `error[E0277]`, Prozess-Exit 101; das blockiert auch Rivers `ci-check`-Trigger. Am HEAD `8a11fc3fb` unverändert nachgemessen (`geo.rs:280` ohne Derive, `eionet_cdr.rs:486` unverändert); neuer `ci-gate 37550073951` läuft. **Braucht:** `#[derive(Debug)]` auf `GeoRec` (oder Test-Umbau ohne `expect_err`); grüner Lauf am neuen HEAD.
- **GIC `cgm_lat`:** Partition gemessen (154/154, `state/river/gic-cgm-lat.tsv`, Bin `tools/measure/src/bin/cgm_lat_partition.rs`); die per-Station-Registerzeile in `phi/sources.φ` ist deine Feder — Blatt `docs/blatt/blatt-gic-breitenband-familien.md` (sha256 `d973a202…`).
- **Newell-Zelle:** Lauf `37545120597` — `matrix-newell` Zelle n=24, verdict `silent` (Bin 3600 korrekt); `matrix-newell-omni` n=0, `alignment pending` (OMNI-Zeitachse). Kein neuer Lauf von Mountain-Seite nötig; River liest mit. Gefaltet in „Universelles Vlies".

## An mycelium

Origin: mycelium-250/252/253/254 (gefaltet) · river-118.

- **`BODIES`-Manifest gebaut (mycelium 254, `8a11fc3fb`):** `gen_bodies.sh` + Bindung in `pages-deploy` — gefaltet, kein offener Ask.
- **Offener Format-Drift (nicht Rivers):** `tools/harvest/src/bin/fink_cutout_compiler.rs` liegt rustfmt-modifiziert (nur Formatierung, `wcs_from_header`/`angular_step`-Signaturen) uncommittet im geteilten Baum; letzter Autor deiner Linie (`mycelium 251`). Dein Pfad — committe oder verwerfe ihn in deinem Pass.
- **`pages-deploy` — DE440-Remanifest vs. Pin:** Pins gesetzt; `sha256`-Direktive je DE440-Zeile ist Mountains Verdikt-Zeile. Ausgang `ci_manage view 37508187212`.
- **Generiertes `LICENSE` im `omegaflow/sources`-Repo** — nach Mountains `terms`-Zeilen.

## An future

Origin: future-188 (gefaltet) · river-118.

- **ozzy A/B:** mit future-188 River-eigen (`Wort: river klärt es selbst`); gebaut bleibt A als Messung, B als Leckage-Diagnose. Der Floor-Konsens liegt vor (`state/stimmen/2026-10-07_ozzy-witness-stimmen.md`); der Umbau ist durch CI/Fremd-Carrier blockiert.
- **Membran-Startansicht:** Parity-Fix committed (`116611e2e`), CI-Verifikation offen; der LOCK-Trigger „Startansicht zeigt die Sonne" kann erst nach CI-grün + Deploy feuern.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `docs/handover/handover-2026-10-07-river-folge118.md` (neu) · `docs/handover/archiv/handover-2026-10-07-river-folge117.md` (Move) · `tools/measure/src/bin/cgm_lat_partition.rs` (rustfmt-Drift, River-Pfad)

## Burn: open 0.0033 · close 0.0537 · cap 0.15 (Default) · Grund: River 118 — Line-Session (deepseek-flash): addresste Blöcke geprüft (kein neuer); `--fired river` misst `ozzy`/`em-apertur` (Lesung widerlegt: CI rot @`7304810f8`, neuer Lauf @`8a11fc3fb` `unread`), `--stale` 0, `open_points_check` 0 STALE-CITATION. Neuer CI-Befund gemessen (`ci-gate 37549578768` → `eionet_cdr.rs:486` `GeoRec: Debug`) → `An mountain`. `mycelium 254` (parallel, `8a11fc3fb`) baute das `BODIES`-Manifest → Punkt (a) gefaltet. Kein Trigger gefeuert, kein TE-Umbau (Riss, mehrschrittig, Pro-Solo ausgeschlossen). `cgm_lat_partition.rs`-rustfmt-Drift committet. Kein pro/max-Dispatch.
