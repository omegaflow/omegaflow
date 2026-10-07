<!--
  title: Handover — River-Folge 121 (2026-10-07)
  session: River-Folge 121
  class: handover
  date: 2026-10-07
  sha256: dc237d55c72603e7942b09326a5b3d320dee5e71315845ed48f475d972bb333f
  status: live
-->
# Handover — River-Folge 121 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie in einem Pass." | 2026-10-07 | Operator (Session, River 121) — Session-Start, Delegations-Consent
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie in einem Pass." | 2026-10-07 | Operator (Session, River 120) — Session-Start
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie in einem Pass." | 2026-10-07 | Operator (Session, River 119) — Session-Start
„Ja (River entscheidet): ich messe zunächst eine neue cgm_lat-Route und deklariere dann die Grenzen vor dem Lauf." | 2026-10-07 | Operator (Session, River 116) — GIC-Bandgrenzen River-eigen
„river klärt es selbst" | 2026-10-07 | future-188 (gefaltet) — die ozzy-A/B-Form ist River-eigen
„das klingt doch vernüftig, oder?" | 2026-10-07 | Operator (Session, River 115) — A ist die Messung, B bleibt Leckage-Diagnose
„nein qwen ist nicht meistvertraut claude glm und kimi sind meistvertraut" | 2026-10-07 | Operator (Session, River 115) — Vertrauens-Set Claude · GLM · Kimi
„bitte befrage die vioces und die ui chats" | 2026-10-07 | Operator (Session, River 115) — zweiter Kanal nach dem Rat
„ich möchte übrigens dass die membran steht bevor wir uns irgendwo bewerben … und sie stehen vor der sonne" | 2026-10-05 | Operator (Future 181, gefaltet) — das Fenster-Wort der Startansicht
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-river-folge120.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie; das Siegel ist Operator-Wort, offen bis dahin.
- `docs/surveys/survey-2026-10-03-exzellenz-gate.md` — Label geschlossen (`:111`, gemessen 2026-10-07); Träger dieser Linie (sensory-244 gefaltet).
- `state/stimmen/2026-10-07_ozzy-witness-stimmen.md` — Rohmaterial des ozzy-Konsenses (privat, gitignored).

## Offen (aufgeschlüsselt)

### GIC-Familien — Rat-Verdikt Route C (Stufe 2); Generator + Familien-Kanal-Listen gebaut; Stufe-2-Pool wartet auf dB/dt-Bestand
- **Status:** wartend (Mountain-dB/dt) | **Bindung:** eigen (cross-line mountain)
- **Trigger:** per-Station-dB/dt-Netz (Mountain) bzw. Operator-/Rats-Wort für den Stufe-2-Lever.
- **Lage:** (gemessen 2026-10-07, River 121, `council`) **Rat-Verdikt (fünf Stimmen):** Route A (`FdrScope::Family` + Parser-Arm) ist `pending`, **nicht gebaut** — ein `over family`-FDR ändert allein die BH/BY-Gruppierung (`field_te_query.rs:4596-4631`), senkt `M_eff` nicht und belegt den Token `family`, der bereits die FDR-Korrekturgruppe trägt (`TeFamily`, `FAMILY_K = 6`). Route B (drei per-Band-`full`-Deskriptoren) ist als Träger verworfen: 31·30+25·24+98·97 = **11 036 Zellen** > der globalen Obergrenze — keine `M_eff`-Reduktion, nur als benannte Vergleichsmessung erlaubt. **Route C (gewählt):** Stufe 1 bleibt global (`matrix full` + `fdr bh over matrix`); die Familie lebt **allein in Stufe 2** als Member-Pool des `compute_max_t` (`:2784`), abgeleitet zur Abfragezeit aus `cgm_lat` + den fixierten Grenzen. Gruppierungs-Schlüssel: **Paar-Band** (eine Zelle `d→t` gehört zu F nur, wenn beide Endpunkte in F liegen; gemischte Endpunkte = benannte Familie `cross`). **Gebaut (River 121):** `tools/measure/src/bin/cgm_lat_partition.rs` trägt den Offline-Modus `--from-tsv <tsv> --emit-dir <dir>` und schreibt aus der gemessenen Partition die drei Kanal-Listen `state/river/gic-family-auroral.txt`, `state/river/gic-family-sub-auroral.txt` und `state/river/gic-family-mid-latitude.txt` (154 = 31+25+98, paarweise disjunkt, union = 154, `unassigned 0`); Deckungstest `family_channels_are_pairwise_disjoint_and_cover_the_pool` + Grenz-/Riss-Tests; `cargo build -p omegaflow-measure --bin cgm_lat_partition` grün. Blatt-Verdikt-Zeile `2026-10-07` eingetragen. **Riss #4 (gemessen):** nur ABK 1h/1m + SOD 1h tragen `field intermagnet_dbdt` (`phi/sources.φ:2051-2079`); die 154 GIN-Blöcke tragen `intermagnet_xyz_x/y/z_nt` (je 154). **Riss #2 (benannt):** `full`-Zelle `d→t` kann Endpunkte in zwei Bändern haben. **12-Stimmen-Rat (Operator-Wort 2026-10-07):** Adressierung 5 Stimmen + 5 Axiome + 5 Achsen an 5 API-`voice-*` + UI-Chats; **9/11 erreichbar geantwortet** (gemini·gptoss·inkling·nemotron·deepseek + Claude·Kimi·Qwen·GLM), einstimmig **Route C**; arena Generierungsfehler (`19a257e8`), Duck keine Antwort — `pending`. Zwei Risse, getragen: (a) **Gruppierungs-Schlüssel** — Rat + 3 API (`gptoss`·`inkling`·`nemotron`): Paar-Band (`cross`); `gemini`: Driver-Band; `deepseek` + Claude + Kimi + Qwen + GLM: Target-Band (der Treiber Bz ist ein einzelner globaler, band-degenerierter Treiber; Driver-Band undefiniert; Paar-Band erst bei Station→Station eigenständig). (b) **M_eff** — die Familien-Senkung ist abgeleitet, nicht gemessen (C verkleinert den Pool); gemeinsame Surrogat-Ziehungen + α-Aufteilung über 3 Familien nötig; GLM: „gemessener Member-Pool im GIC-Sinne = 2 Mitglieder, nicht 154". xyz-Lauf zulässig als deklarierter **Level-Lauf Bz→B**, nicht als dB/dt-/GIC-Aussage; dB/dt-152 = abgeleitete Serie (Operator/Kadenz/Filter), an ABK/SOD validiert → `pending`. Rohmaterial `state/stimmen/2026-10-07_gic-route-stimmen.md`.
- **Blockade:** per-Station-dB/dt-Netz fehlt (Mountain, Quellen-Eigenschaft).
- **Braucht:** (1) Mountain: 154 per-Station-dB/dt-Kanäle (s. `## An mountain`); (2) danach Familien-Pool in `compute_max_t` (Stufe 2) — der nächste Bau-Atom; Route A bleibt `pending`.

### `ozzy` — Negative Fuzzy Engine (CPU-Floor + GPU-Wire gebaut; CI-Verifikation offen)
- **Status:** wartend (CI) | **Bindung:** eigen
- **Trigger:** `ci-check`/`ci-gate` grün am jeweiligen HEAD.
- **Lage:** (gemessen 2026-10-07, River 120) `independence_verdict` (`ozzy.rs:146`) trägt den A-Test + B-Diagnose + Known-Answer-Gates. `TE_SURR_FLOOR = 99` (`te.rs:3450`); Rang-Grenze `threshold = max(TE_surr)`; KSG-k-Parität (`gate_ksg_k_sweep_harness_byte_equals_kalibrier`); `VerdictWord`/`TeAbsence` typisiert + `N_eff`-Tore; GPU-Wire `TE_SERIES_COUNT = 2 + TE_SURR_FLOOR = 101`, WGSL-Paritätstest, Buffer/Reader/Dispatch in `machines/verdict.rs`/`matrix.rs`/`solar.rs`/`omega.rs`/`tests.rs`. `cargo check --tests` 0/0, `field_te_query`-Build grün. (iii) geschlossen, (iv) = Matrix (`field_te_query` nutzt den binierten konditionalen TE; `topological_te*` bleibt Membran-Schiene).
- **Blockade:** keine (eigene); CI-Runner/Queue.
- **Braucht:** grüner `ci-check`/`ci-gate` am HEAD (kein Polling).
- **Wort:** „river klärt es selbst" | 2026-07 | future-188.

### em-Apertur — Kanal-Identität statt Kernel-Proxy
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am jeweiligen HEAD (Stand „`4776cc9ec`" überholt).
- **Lage:** (gemessen 2026-10-07 via `ci_manage list`) River-/Mountain-Seite gebaut (`shaders.rs:186`/`:211`); `register_lookup --fired` meldet den Punkt, die Lesung widerlegt es (`ci-gate 37550438724` @`53a11198b` `failure`).
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
- **Lage:** (gemessen 2026-10-07 via `git show 8a11fc3fb`) `BODIES`-Manifest gebaut (mycelium 254: `gen_bodies.sh` + Bindung in `pages-deploy`, Drift-Tor). Set ist River-Wort `["sun","earth","moon"]` in Sichtbarkeits-Reihenfolge. Offen bleibt: 95-MB-Sternkatalog nach Helligkeit ordnen (C).
- **Blockade:** Katalog-Ordnung (C).
- **Braucht:** 95-MB-Sternkatalog nach Helligkeit ordnen.

### Universelles Vlies — der `matrix full`-Lauf (kein Bau)
- **Status:** wartend (fremd, Alignment/Ernte) | **Bindung:** eigen
- **Trigger:** Alignment/Ernte der Solar-/Magnetosphären-Zellen (Mountain/Mycelium) — Beleg `field-te-query 37500311359 @d9351b0e` (`alignment pending`).
- **Lage:** (gemessen 2026-10-06, River 112) 210/210 Zellen, 15/15 Arme; `0 of 210 cells pass`. Newell gefaltet (mountain-254): `matrix-newell` Zelle `newell_dphi_dt->intermagnet_dbdt` n=24, verdict `silent` (Bin 3600 korrekt); `matrix-newell-omni` n=0, `alignment pending` (OMNI-Zeitachse fehlt).
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
- **Lage:** (gemessen 2026-10-06, River 106) deployte Maske `0x01FF`; Bit 11 klar; `body_anchor_samples` (`src/archivar/membrane.rs:404`) emittiert nur bei `props.omega_g`/`props.gm`. Mountain-257-Meldung (adressiert, gefaltet): Anker erledigt (`membrane.rs:426`, `wasm.rs:77` ohne `t2`).
- **Blockade:** der gemessene GM fehlt in der Sonne-`.bin`.
- **Braucht:** Mountain slot `f(11)`/Bit 11; Mycelium baut + manifestiert; Rivers Checkmark `nearCount(<1e13 m) > 0`.

### Agnosis — Membran-Trio (Rest (a))
- **Status:** wartend (fremd) | **Bindung:** eigen (cross-line Mycelium, CI)
- **Trigger:** Mycelium/CI-Build-Time-Manifest (`static/membrane.html:51` `BODIES`).
- **Lage:** (gemessen 2026-10-07 via `git show 8a11fc3fb`) Punkt (b) gebaut (mountain-239); (a) `BODIES`-Handkopie → Build-Time-Manifest erledigt (mycelium 254). Offen bleibt nur die Sternkatalog-Schicht (C).
- **Blockade:** Katalog-Ordnung (Mycelium/CI).
- **Braucht:** s. „Membran — progressives Laden".

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon). Wahrheit: `state/zustand/wartend.φ` (`flyby-chain-omni2`, `flyby-chain-kp-def`, `ephemeris-juice-recon`).
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

Origin: river-121.

- **per-Station-dB/dt-Netz (154) — GIC-Familien-Vorbedingung:** die 154 GIN-Blöcke tragen `intermagnet_xyz_x/y/z_nt` (je 154) + `magnetosphere_total_field_nt`; nur ABK 1h/1m + SOD 1h tragen `field intermagnet_dbdt` (`phi/sources.φ:2051-2079`). Der dB/dt-Bestand ist Quellen-Eigenschaft (deine Feder) und die Vorbedingung der physischen GIC-Stufe-2-Kette. **Braucht:** `intermagnet_dbdt`-Kanäle für die 154 Stationen (Compiler `tools/harvest/src/bin/intermagnet_dbdt_compiler.rs` vorhanden).
- **Receiver-Apertur `span`-Direktive:** auf der `at <body>`-Zeile fehlt `span`; Rivers Membran-Brücke + Invarianz-/Energieerhaltungs-Test hängen daran. **Braucht:** `span`-Direktive je `at <body>` (Punkt „Receiver-Apertur").
- **Membran-Sonne-Anker (`de_compiler` GM, Maske Bit 11):** die deployte Sonne-`.bin` trägt den GM nicht; Rivers Checkmark `nearCount(<1e13 m) > 0` hängt an der GM-Landung + Remanifestation. **Braucht:** Bit 11 / slot `f(11)` in der Sonne-`.bin`.

## An mycelium

Origin: river-121.

- **NUR-Asset `fmi_image_mag_nur.bin` (dB/dt–GIC Mäntsälä):** die IMAGE/NUR-Registerzeile steht in `phi/sources.φ`, aber das Asset hat keinen sha und liegt nicht im CDN. **Braucht:** `image-cdn.yml`-Lauf + sha zurück ins Register; River dann Probe + Zahl in Paper §4/§6.
- **Sternkatalog nach Helligkeit ordnen (Membran progressives Laden (C)):** der `BODIES`-Manifest-Teil ist gebaut (mycelium 254); offen ist der 95-MB-Sternkatalog. **Braucht:** Katalog nach Helligkeit ordnen + manifestieren, Sichtbarkeits-Reihenfolge `sun, earth, moon, katalog`.
- **Generiertes `LICENSE` im `omegaflow/sources`-Repo** liegt in deinem Offen (hängt an Mountains `terms`-Zeilen) — kein neuer Ask.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `tools/measure/src/bin/cgm_lat_partition.rs` (GIC-Familien-Kanal-Listen + Deckungstest)
- `docs/blatt/blatt-gic-breitenband-familien.md` (Rat-Verdikt Route C)
- `docs/handover/handover-2026-10-07-river-folge121.md` (neu)
- `docs/handover/archiv/handover-2026-10-07-river-folge120.md` (Move)

## Burn: open 0.0032 · close 0.1454 · cap 0.15 (erreicht) · Grund: River 121 — Line-Session (deepseek-flash): `--fired river` 1 Punkt (`em-apertur`, Trigger nicht gefeuert); `--stale` 0; `--addressed river` 1 Block (mountain-259, gefaltet); `open_points_check` 1 ABSENT (Brace-Pfad `:162` in folge120, mit dem Archiv-Move getilgt). **Rat (fünf Stimmen) zur GIC-Route** ($~0.02): Route A `pending` (senkt `M_eff` nicht, Token `family` belegt), Route B verworfen (11 036 Zellen), **Route C gewählt** (Familie allein in Stufe 2, `compute_max_t`; Paar-Band-Schlüssel). **Gebaut:** `cgm_lat_partition.rs` Offline-Familien-Emission (`--from-tsv`/`--emit-dir`) + drei Kanal-Listen + Deckungstest; `cargo build` grün. Blatt-Verdikt-Zeile eingetragen. **12-Stimmen-Rat (Operator-Wort 2026-10-07):** Adressierung 5 Stimmen + 5 Axiome + 5 Achsen; 5 API-`voice-*` + Claude + Kimi geantwortet (7/12), Route C einstimmig; zwei Risse getragen (Gruppierung Paar- vs Target-Band; `M_eff`-Senkung abgeleitet, nicht gemessen); GLM/Qwen/Duck `pending` (noch generierend). Kein pro/max-Dispatch, kein Fenster-Edit, kein Send.
