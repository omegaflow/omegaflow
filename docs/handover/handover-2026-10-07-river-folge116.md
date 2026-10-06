<!--
  title: Handover — River-Folge 116 (2026-10-07)
  session: River-Folge 116
  class: handover
  date: 2026-10-07
  sha256: a89e2dd07c3d4632dce97526407a6dc18e8492ef1bd43a14b8c839c18c59476e
  status: live
-->
# Handover — River-Folge 116 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie in einem Pass." | 2026-10-07 | Operator (Session, River 116) — Session-Start, Delegations-Consent
„river klärt es selbst" | 2026-10-07 | future-188 (gefaltet) — die ozzy-A/B-Form ist River-eigen, kein Operator-Wort; bleibt als „A ist die Messung, B bleibt Leckage-Diagnose" gebaut
„das klingt doch vernüftig, oder?" | 2026-10-07 | Operator (Session, River 115) — A ist die Messung, B bleibt Leckage-Diagnose
„nein qwen ist nicht meistvertraut claude glm und kimi sind meistvertraut" | 2026-10-07 | Operator (Session, River 115) — Vertrauens-Set Claude · GLM · Kimi
„bitte befrage die vioces und die ui chats" | 2026-10-07 | Operator (Session, River 115) — zweiter Kanal nach dem Rat
„Erste Handlung: `sread docs/concepts/tool-forms.md` …" | 2026-10-06 | Operator (Session, River 114) — Session-Start, Delegations-Consent
„ich möchte übrigens dass die membran steht bevor wir uns irgendwo bewerben … und sie stehen vor der sonne" | 2026-10-05 | Operator (Future 181, gefaltet) — das Fenster-Wort der Startansicht
Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-07-river-folge115.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie: Identitäts-Verdikt getragen; offen bis zum Siegel (Bandgrenzen-Slot, cgm_lat).
- `docs/surveys/survey-2026-10-03-exzellenz-gate.md` — Label geschlossen (`:111`); Träger dieser Linie.
- `state/stimmen/2026-10-07_ozzy-witness-stimmen.md` — Rohmaterial der ozzy-Befragung (privat, gitignored).

## Offen (aufgeschlüsselt)

### `ozzy` — Negative Fuzzy Engine (Zeugen-Vokabular + Zeugenstempel + Unabhängigkeits-Verdikt (A) gebaut; Known-Answer + Surrogat-Floor offen)
- **Status:** eigen (CI-Verifikation) | **Bindung:** eigen
- **Trigger:** `ci-check` grün am HEAD; danach Surrogat-Floor-Korrektur.
- **Lage:** (gemessen 2026-10-07 via `cargo check` 0/0, `ci_manage status`) A/B entschieden: A ist die Messung, B Leckage-Diagnose (Operator-Wort, in Code). Rivers Clippy-Anteil geheilt: `ozzy.rs:90`/`:109` `needless_range_loop` und `least_squares.rs:15` `type_complexity` gefixt; `cargo check` 0/0. Der `ci-check` am HEAD läuft noch (nicht grün → Trigger nicht gefeuert).
- **Riss (getragen, ungeglättet):** `topological_te_phase` trägt fix `n_surr: 10` und `mean+2σ` — querdurch alle 12 Kanäle zu schwach (≥99, teils ≥199; Rang-/Permutationstest; `N_eff` statt `N − Rang`; `None` getypt mit Grund). Sonnets B-Null-Einwand: `conditional_embedded_te_phase` nutzt `z_phase_surrogate` statt bedingter Permutation innerhalb E.
- **Blockade:** CI-Ausgang `unread` (kein Polling).
- **Braucht:** (1) `ci-check`-Ausgang (Stehender Pass/`ci_manage`); (2) Surrogat-Floor-Korrektur `topological_te_with` (`te.rs:3309`) + `topological_te_phase` (`te.rs:3450`) + `topological_verdict_from_gpu` (`te.rs:3551`) — `n_surr` ≥99, Rang-Schwelle, `None` mit Grund; (3) Verdrahtung in `field_te_query`/Matrix.
- **Wort:** „river klärt es selbst" | 2026-10-07 | future-188 (A/B) · „das klingt doch vernünftig" | 2026-10-07 | Operator 115 (A gewählt)

### em-Apertur — Kanal-Identität statt Kernel-Proxy
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am eigenen HEAD.
- **Lage:** (gemessen 2026-10-07 via `ci_manage status`) River-/Mountain-Seite gebaut (`src/mathematikerin/shaders.rs:186`/`:211`); `ci-check` am HEAD läuft noch (nicht grün).
- **Blockade:** keine (eigene); Runner-/Queue-Kapazität.
- **Braucht:** `ci-check` am HEAD abwarten (Stehender Pass; kein Polling).

### Membran-Startansicht — zwei Aperturen (Parity-Fix gebaut; CI-Verifikation offen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`/`wasm-parity` grün am Fix-HEAD `116611e2e`; danach Pages-Deploy + Browser-Sicht.
- **Lage:** (gemessen 2026-10-07 via `ci_manage list`) Parity-Fix committed (`116611e2e` river 114); `ci-check` am HEAD läuft noch (nicht grün). Deploy trägt den Bau noch nicht.
- **Blockade:** CI-Lauf-Ausgang `unread` (Stehender Pass/`ci_manage`, kein Polling).
- **Braucht:** CI-grün; Pages-Deploy; Browser-Sicht auf `omegaflow.space/membrane.html`. Offen: `state.lvl` global über beide Aperturen; `MembraneLookup.add_stars` panikt bei Re-Init (Riss, kein Repro ohne WASM/Browser).

### GIC-Breitenband-Familien — Design + Messschritt offen
- **Status:** wartend (cgm_lat) / Design offen | **Bindung:** eigen
- **Trigger:** CGM-Messung je Station (bzw. Operator-Wort zu den Bandgrenzen).
- **Lage:** (gemessen 2026-10-06) `station <code>` auf 154 Blöcken (`2117476be`); Identitätsverdikt „Station = Identität"; sechs Stimmen → Option (c) geschichtet (`family` = FDR-Gruppe, `cgm_lat` eigene Registergröße, Scope `fdr … over family`). Blatt `unsealed`; der Bandgrenzen-Slot ist **offen** (`blatt-gic-breitenband-familien.md:50`).
- **Blockade:** keine gemessene CGM-Breite im Register (Geomagnetic-Latitudes-Services `declined`, `phi/declined_sources.φ:3328-3340`).
- **Braucht:** (0) neue CGM-Route + `cgm_lat` lokal messen; (1) `fdr … over family`-Scope; (2) Bandgrenzen (Design); (3) CI-Job `field-te-query.yml`.

### Universelles Vlies — der `matrix full`-Lauf (kein Bau)
- **Status:** wartend (fremd, Alignment/Ernte) | **Bindung:** eigen
- **Trigger:** Alignment/Ernte der Solar-/Magnetosphären-Zellen (Mountain/Mycelium) — Beleg `field-te-query 37500311359 @d9351b0e` (`alignment pending`).
- **Lage:** (gemessen 2026-10-06, River 112) 210/210 Zellen, 15/15 Arme; `0 of 210 cells pass`; viele Zellen `alignment pending` (n=0).
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
- **Lage:** (gemessen 2026-10-06, River 109) Receiver-Schnitt gebaut; `BODIES` ist eine geschlossene Handkopie (`static/membrane.html:51`).
- **Blockade:** 95-MB-Sternkatalog + Trio-Manifest (Mycelium/CI).
- **Braucht:** Katalog nach Helligkeit ordnen; Trio → Manifest.

### Agnosis — Membran-Trio (Rest (a))
- **Status:** wartend (fremd) | **Bindung:** eigen (cross-line Mycelium, CI)
- **Trigger:** Mycelium/CI-Build-Time-Manifest (`static/membrane.html:51` `BODIES`).
- **Lage:** (gemessen 2026-10-06, River 111) Punkt (b) gebaut (mountain-239); offen nur (a) `BODIES`-Handkopie.
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

Origin: mycelium-250 (gefaltet) · river-116.

- **`ozzy` `Debug`/Clippy:** `Kanal` derivt `Debug` (`ozzy.rs:4`); Rivers Clippy-Anteil (`ozzy.rs:90`/`:109`, `least_squares.rs:15`) in river-116 geheilt. Mountain-Clippy (`eionet_cdr.rs`, `osm_pbf.rs`) in `b78c5d9e8` geheilt.
- **`pages-deploy` — DE440-Remanifest vs. Pin:** Pins gesetzt; `sha256`-Direktive je DE440-Zeile ist Mountains Verdikt-Zeile. Lauf-Ausgang `ci_manage view 37508187212`.
- **`static/membrane.html:51` `BODIES`-Handkopie** → Build-Time-Manifest aus der Hüllen-Pipeline.
- **Generiertes `LICENSE` im `omegaflow/sources`-Repo** — nach Mountains `terms`-Zeilen.

## An mountain

Origin: mountain-folge252/253 (gefaltet) · river-116.

- **Newell-Zelle:** Descriptor-Bin 3600 (`newell_geospheric.te:22`) + Lauf `37545120597` gesehen (registriert in `e08c4665a`); `tau_t` = native Ziel-Kadenz. River liest mit.
- **Membran-Query Body-Anker — erledigt** (`membrane.rs:426`, `wasm.rs:77` ohne `t2`).

## An future

Origin: future-188 (gefaltet) · river-116.

- **ozzy A/B:** die A/B-Wahl ist mit future-188 River-eigen (`Wort: river klärt es selbst`); gebaut bleibt A als Messung, B als Leckage-Diagnose. Kein Operator-Wort mehr nötig.
- **Membran-Startansicht:** Parity-Fix committed (`116611e2e`), CI-Verifikation offen; der LOCK-Trigger „Startansicht zeigt die Sonne" kann erst nach CI-grün + Deploy feuern.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `src/mathematikerin/ozzy.rs` (`needless_range_loop`-Fix)
- `src/mathematikerin/least_squares.rs` (`type_complexity`-Fix, `PivotSolution`)
- `docs/handover/handover-2026-10-07-river-folge116.md` (neu) · `docs/handover/archiv/handover-2026-10-07-river-folge115.md` (Move)

## Burn: open 0.0015 · close 0.0501 · cap 0.15 (Default) · Grund: River 116 — Line-Session (deepseek-flash): Rivers Clippy-Anteil geheilt (`ozzy.rs:90`/`:109`, `least_squares.rs:15`), `cargo check` 0/0; addressed Blöcke gefaltet (mountain-252/253 Newell, mycelium-250 ozzy-Clippy/Debug, future-188 A/B, sensory-244); Handover auf folge116 fortgeschrieben. Keine Sub-Dispatchs.
