<!--
  title: Handover — River-Folge 119 (2026-10-07)
  session: River-Folge 119
  class: handover
  date: 2026-10-07
  sha256: 37678f3f3826f038dde684f11670371e3df6675594fdfa5ae8b6a5b7dbdbd106
  status: live
-->
# Handover — River-Folge 119 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie in einem Pass." | 2026-10-07 | Operator (Session, River 119) — Session-Start, Delegations-Consent
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie in einem Pass." | 2026-10-07 | Operator (Session, River 118) — Session-Start
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
- `docs/surveys/survey-2026-10-03-exzellenz-gate.md` — Label geschlossen (`:111`, gemessen 2026-10-07); Träger dieser Linie (sensory-244 gefaltet).
- `state/stimmen/2026-10-07_ozzy-witness-stimmen.md` — Rohmaterial des ozzy-Konsenses (privat, gitignored).

## Offen (aufgeschlüsselt)

### `ozzy` — Negative Fuzzy Engine (A + Zeugen-Vokabular gebaut; Surrogat-Floor-Design entschieden; CI-Verifikation offen)
- **Status:** wartend (CI) | **Bindung:** eigen
- **Trigger:** `ci-check`/`ci-gate` grün am jeweiligen HEAD.
- **Lage:** (gemessen 2026-10-07 via `ci_manage view`/`jobs` 37550438724) `independence_verdict` (`src/mathematikerin/ozzy.rs:146`) trägt den A-Test (held-out Zeugen) + B-Diagnose + die zwei Known-Answer-Gates. Rivers Clippy-Anteil geheilt (`ozzy.rs:90`/`:109`, `least_squares.rs:15`); `cargo check` 0/0. `register_lookup --fired river` meldet `ozzy` als gefeuert — **Lesung dieser Session widerlegt es:** `ci-gate 37550438724` @`53a11198b` `failure` (Jobs `clippy` + `format`), `ci-check 37550405491` `pending`; Trigger **nicht** gefeuert (kein Polling).
- **Rat-Entscheid (2026-10-07, Session 119, fünf Stimmen über den `council`-Agenten) — der Floor-Riss ist aufgelöst:**
  1. **Rang-Form:** `p = (1 + #{TE_surr ≥ TE_obs}) / (n_surr+1)`; bei `n_surr = 99` ist `p_min = 0.01` und die Regel fällt mit dem Maximum der Null zusammen: Kopplung ⇔ `TE_obs > max(TE_surr)`. α an `1/(n_surr+1)` gebunden, nicht frei gewählt (A = A, kein interpolierter Zwischenwert).
  2. **`N_eff`:** `τ_int = 1 + 2·Σ_{k=1}^{K} ρ_k`, `K` = erster `k` mit `ρ_k ≤ 0` (erster Nulldurchgang), `N_eff = N / τ_int` — std-only (Eigenmittel/-varianz/-autokovarianz), **pro Messung getrennt** für A und B. Zweites Tor: `df = N_eff − (dim·τ_max + 1) − rank(E)` (B zusätzlich `rank(E)`, A ohne); unter dem Boden → getypte Absenz, kein Verdikt. Zwei Tore (`N_eff`, dann `df`), nie zu einem Wert vermischt.
  3. **Typisierte Absenz:** Rückgabe `Result<VerdictWord, TeAbsence>`; `VerdictWord` = `Coupled`/`Independent`/`Leakage`; `TeAbsence` = `InsufficientSurrogates`/`InsufficientEffectiveSample`/`ZeroVariance`/`NotTestable`. Ein rohes `None`/`or 0` kompiliert dann nicht (`0` ist kein `VerdictWord`); `A − B` ist `TeAbsence`, wo `B` `TeAbsence` ist.
  4. **Zählwert:** `TE_SURR_FLOOR = 99` (Konstante benannt, nicht Session-Entscheidung).
- **Bounded step sequence (Landkarte gemessen 2026-10-07 via `explore`):**
  - (i) **CPU-Floor** in `te.rs`: `SurrogateSpec.n_surr = 10` (`:3464` phase, `:3486` arx), Defaults `:1735`/`:1744`/`:1789`/`:194`, Schwellen `mean+2σ` `:1805`/`:197`/`:3309`/`:3359`/`:3551`/`:883`/`:1277` → Rang-p + `N_eff`; typisierte Absenz `ozzy.rs:47-48` (`Option<f64>`), `te.rs:3456`/`:3495`.
  - (ii) **GPU-Wire:** `SERIES_COUNT` (`shaders.rs:435`) `12 → 101` (2 real + 99 Surrogat) bzw. Kernel-Schleife; `TE_SERIES_BYTES`/`TE_VERDICT_SLOTS`/`TE_KSG_SLOTS` (`machines/verdict.rs:3-6`) und Buffer-Sizing (`matrix.rs:556/562`, `solar.rs:389/395`, `omega.rs:1521/1527`, `tests.rs:189/195`, `te.rs:7897/7903`); Leser `&[f32; 72]` (`te.rs:3551`) und `2..12`-Schleifen (`matrix.rs:888`); Upload-Schleifen `for s in 0..10` (`omega.rs:521`, `matrix.rs:1011`).
  - (iii) **Gates:** Known-Answer `ozzy.rs:323`/`:359`, A-Test-Kern `:338-356`.
  - (iv) **Verdrahtung** `field_te_query`/Matrix.
- **Blockade:** kein CI-Runner grün; der Floor-Umbau ist eine mehrschrittige Konstruktion (TE-/Null-Atom). Kein Pro-Solo — der Rat hält sie; die Umsetzung läuft als sequenzierte bounded dispatches.
- **Braucht:** (1) grüner `ci-check`/`ci-gate` am HEAD; (2) die vier bounded Schritte (i)–(iv).
- **Wort:** „river klärt es selbst" | 2026-10-07 | future-188 (A/B) · „das klingt doch vernünftig" | 2026-10-07 | Operator 115 (A gewählt)

### em-Apertur — Kanal-Identität statt Kernel-Proxy
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am jeweiligen HEAD (der Stand „`4776cc9ec`" ist überholt).
- **Lage:** (gemessen 2026-10-07 via `ci_manage list`) River-/Mountain-Seite gebaut (`src/mathematikerin/shaders.rs:186`/`:211`). `register_lookup --fired` meldete den Punkt als gefeuert — die Lesung widerlegt es: `ci-gate 37550438724` @HEAD `53a11198b` `failure`; Trigger nicht gefeuert.
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
- **Lage:** (gemessen 2026-10-07) Operator-Wort: River entscheidet die Grenzen; deklariert **auroral ≥60°, sub-auroral 50–60°, mid-latitude <50°** |CGM|. Partition **gemessen** (`tools/measure/src/bin/cgm_lat_partition.rs`; Artefakt `state/river/gic-cgm-lat.tsv`): **154/154, disjunkt, 0 pending** — auroral 31 · sub-auroral 25 · mid 98; Quellen 133 `omniweb-cgm` (Epoche 2025) / 19 `supermag-aacgm` (IGRF-2000, äquatorial) / 2 `bgs-quasi-dipole` (CPL/TTB); Drift OmniWeb↔SuperMAG 1.13° (130). Blatt `blatt-gic-breitenband-familien.md` (sha256 `d973a202…`), grenz-nahe Stationen benannt (WNG 49.99 · ORC −49.50 · EYR −50.09 …). `cgm_lat_partition.rs` rustfmt-Drift (nur Format) in River 118 committet.
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
- **Lage:** (gemessen 2026-10-06, River 106) deployte Maske `0x01FF`; Bit 11 klar; `body_anchor_samples` (`src/archivar/membrane.rs:404`) emittiert nur bei `props.omega_g`/`props.gm`. Mountain-257-Meldung (adressiert, gefaltet): Anker erledigt (`membrane.rs:426`, `wasm.rs:77` ohne `t2`).
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

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `docs/handover/handover-2026-10-07-river-folge119.md` (neu) · `docs/handover/archiv/handover-2026-10-07-river-folge118.md` (Move)

## Burn: open 0.0032 · close 0.0507 · cap 0.15 (Default) · Grund: River 119 — Line-Session (deepseek-flash): addresste Blöcke geprüft (keine neuen, 118 gefaltet); `--fired river` misst `ozzy`/`em-apertur` — Lesung widerlegt (ci-gate 37550438724 @53a11198b `failure`), `--stale` 0, `open_points_check` 0 STALE-CITATION. **Rat-Linse** auf den ozzy-Floor-Riss (Council): Rang-p/Max-Regel bei 99, `N_eff = N/τ_int`, `Result<VerdictWord, TeAbsence>`, `TE_SURR_FLOOR = 99`; Code-Landkarte via `explore` (vier bounded Schritte). Kein pro/max-Dispatch. Kein Fenster-Edit, kein Send.
