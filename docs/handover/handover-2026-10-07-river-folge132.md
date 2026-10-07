<!--
  title: Handover — River-Folge 132 (2026-10-07)
  session: River-Folge 132
  class: handover
  date: 2026-10-07
  sha256: 001ded242883444d410536dc5d9edbab86b212490f0ff3d00fc41b5fcd4e2503
  status: live
-->
# Handover — River-Folge 132 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„warum nur duck … ich möchte dass du alle frontier chats befragst" | 2026-10-07 | Operator (Session, River 127) — alle offenen UI-Seats, nicht einer
„es kommen doch keine sterne oder die sonne oder der mars an der presence an es kommen die kräfte also die kanäle/oszillatoren an ich glaube ihr habt irgendwann wieder die objektophilie eingeführt und euch vom agnostizismus wegbewegt" | 2026-10-07 | Operator (Session, River 127) — Kraft-/Kanal-Agnostik statt Objekt-Render
„es gibt keine sonne erde mond die presence kann sich frei durchs 4d block universum bewegen … sie spawnt nur am SSB weil euer bias sonst noch größer wäre von da kann sie sich völlig frei bewegen" | 2026-10-07 | Operator (Session, River 127) — freie Presence-Weltlinie, SSB-Spawn, keine Objekte
„Die Förder-Bewerbungen bleiben LOCK … Send bleibt deine Hand" | 2026-10-07 | Operator (Session, River 127) — Prototype Fund (30.11.) + EMAP (06.11.) bleiben LOCK
„auf jeden fall agnostoisch dein vorgänger hat doch schon eine umfangreiche gibt und bias untersuchung gemacht ist die schon wiedre vergessen?" | 2026-10-07 | Operator (Session, River 129) — Wort für den agnostischen Membran-Edit
„bitte ratsfragen auch vor ALLE UI chats bringen" | 2026-10-07 | Operator (Session, River 130) — Ratsfragen vor alle UI-Seats; als Regel in `AGENTS.md` eingetragen
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-river-folge131.md` §Operator-Wort-Register — gefaltet, nicht kopiert. Verbatim: `state/operator-gespraeche/2026-10-07-river.md`.

## Stimmen-Rolle (gemessen 2026-10-07)

Recherche trägt `voice-deepseek`; strikt lokal nur DeepSeek-flash (`opencode.json`);
Denken/Urteil = UI-Frontier; der Rat = Form/Linse. Die API-Suchschnittstelle
`archive_search --alphaxiv` (alphaXiv MCP `discover_papers`) ist in
`docs/concepts/tools-map.md` nachgetragen.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie; Siegel = Operator-Wort, offen.
- `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md` — see-also auf Archiv-Pfad geheilt (`:7`).
- `docs/paper/gic-causal-driver.md` — NUR-Asset-Fakten §6, §4.7.
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` — Objektophilie-Verdikt (`:113` BODIES→Manifest; `:96` operationaler Test `grep static/`). Angewandt 2026-10-07 (River 129).
- `docs/concepts/remove-bias.md` — der Bias-Tilgungsplan (WP0–WP13); WP13-Fixtures gebaut (`47706add5`).

## Offen (aufgeschlüsselt)

### Membran — Kraft-/Kanal-Agnostik: Rat-Wort **B** (Exposition pro `(force_type, aperture)`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `pages-deploy`-Lauf am neuen `membrane.html`.
- **Lage:** (gemessen 2026-10-07, River 127/129/131/132) **Gebaut (River 129, Operator-Wort):** `static/membrane.html` trägt keinen Body-Namen mehr — `const BODIES` entfernt; der Hüllen-Manifest `/membrane_bodies.txt` wird zur Laufzeit gelesen, pages-deploy schreibt ihn via `gen_bodies.sh --write` aus den Stage-Lines. Der Shader liest die gemessene Farbe (`color_index` → `/color_lut`); `ci==0` → weiß, LUT fehlt → neutrale Rampe. **Gebaut (River 131):** der per-Kanal-Expositions-State — `state.lvl` als `Float32Array(9·2)` über `(force_type, aperture)`, Storage-Bindung 4 `exposure`, im Vertex-Shader `lvl = exposure[force_type][aperture]`, Relaxation α = 1−exp(−1/8); Apertur = wire `extent` (>0 Anker, 0 Sterne); fehlender Schlüssel → 0, kein gezeichneter Level, kein Default; **kein** Wire-Bit. `extent` bleibt die Apertur; Träger = `force_type`. **Gemessen (River 132):** `pages-deploy` `37679754275` brach am `sha256 mismatch for ephemeris_de440_earth.bin` ab — **außerhalb der Linie**: `mycelium-folge264` hat die Pins in `816841a60` mit dem Register synchronisiert (`8b8998bd…`); der Membran-Schritt selbst (`cp static/membrane.html` + `gen_bodies.sh --write`) läuft durch. Der Deploy des neuen `membrane.html` ist damit nur noch an einen frischen `pages-deploy`-Lauf gebunden.
- **Blockade:** der Start-Anker (schwarzes Feld/`scale 0`) hängt an Mountains fehlender span-Direktive.
- **Braucht:** einen frischen `pages-deploy`-Lauf messen (`ci_manage status`); den Start-Anker an Mountains span-Direktive.

### GIC-Stufe-2 — dB/dt abgeleitet; der Familien-Pool ist ein eigener Bau
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-07, River 127/128/131) `intermagnet_dbdt_compiler.rs:5-6,179` leitet `sqrt(dx²+dy²+dz²)` aus derselben `/best-avail/PT1M/xyzf`-Quelle ab; ABK/SOD-dbdt (`phi/sources.φ:2057-2085`) ist kein natives INTERMAGNET-Produkt. Gebaut: `series_dbdt` (`src/archivar/main_flow.rs:5925`); der Loader `tools/measure/src/bin/field_te_query.rs` löst `intermagnet_dbdt_<station>` ab (`align_xyz` → `series_dbdt`, `load_dbdt_across_sources:1163`). **Rat (5 Stimmen, River 131) + Recherche:** (a) **B′** — `cgm_lat` je Station = Registergröße (`SourceConfig.cgm_lat`, `src/archivar/types.rs:439`, `parse.rs:1692`); das Familien-Label live aus `|cgm_lat|` + Grenzen 60/50 (`cgm_lat_partition.rs:168 family_of`); die Partitions-TSV `state/river/gic-cgm-lat.tsv` und `state/river/gic-family-*.txt` sind Träger (kein Code-Konsument), nie Query-Quellort; die IGRF-Live-Ableitung (`src/archivar/igrf.rs:111`) ist toter Pfad (kein Caller) → nur für die 2 QD-Stationen. **Riss** CPL/TTB QD≠CGM (`phi/sources.φ:6324,7669`). (b) **A** — Target-Band; der eine Bz-Treiber ist unbandiert, die geteilte `null_matrix`-Ziehung (`src/mathematikerin/wy_max_t.rs:325`) bleibt kohärent, je Familie ein eigener `compute_max_t:2824`-Call.
- **Blockade:** keine Design-Blockade — Rat geklärt. Offen: **UI-Undergirding** (Operator-Wort 2026-10-07: Rat vor alle UI-Seats) + Vorbedingung `cgm_lat`-Registerzeile je Station (Mountain) + α-Split/Blocklänge im Blatt (Future).
- **Braucht:** den separaten Familien-Bau — neue Fn nahe `run_pair_matrix:4371` + `--stage2 family`-Arm in `main` (~4804); `Vec<Member>` je Familie (`target = station_dBdt`, `driver = globaler Bz`), dann je Familie ein eigener `compute_max_t`-Call; reuse `load_field_across_sources:1129`, `align_many:4137`, `joint_columns:4211`; Stufe 1 byte-identisch. Dann UI-Runde (river-ui + open-weight-ui).

### Receiver-/em-Apertur, ozzy, Membran — CI-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`-Testjob grün am HEAD.
- **Lage:** (gemessen 2026-10-07T20:34Z via `ci_manage status`) HEAD `150f11395`; `ci-check 37682973212` = **pending**, `ci-gate 37682973117` = in_progress, `tools-build 37682973174` = in_progress — der Testlauf ist noch nicht durch. Der Vorlauf am `e0708b53e` (`ci-gate 37679286994`) trug `ci-gate` format/**build**/**clippy**/dropped-gate = success. Gebaut: `series_dbdt` + Loader-Ableitung; ozzy `independence_verdict` (`ozzy.rs:146`); em-Apertur (`mathematikerin/shaders.rs:186,211`); Membran per-Kanal-Exposition (`membrane.html`, River 131).
- **Blockade:** CI-Queue.
- **Braucht:** `ci-check`-Lauf lesen (`ci_manage log 37682973212`), sobald terminal; ozzy/em-Apertur sind compile-verifiziert.

### Bias-Audit Archivar/Mathematikerin — src-Rest und der Gate-Rückfall
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-07, River 127/129/130/132) Untersuchung `omegaflow-legacy/docs/concepts/remove-bias.md` (WP0–WP13). **Geheilt:** `channels.rs:1423` τ=∞; `nexrad.rs`/`rinex.rs`/`odp.rs`/`media.rs`; `weberin.rs:254/258` Literal-Listen → `woven_major_bodies()` (River 131, `clean_tree` 2→0). **Geheilt (River 132):** (1) `src/archivar/motion.rs:66` `orientation_angles_at` → `Option<(f64,f64,f64)>` — bei vorhandener Nutation (`bp.nutation.is_some()`) und fehlendem Granulat wird die Orientierung **abwesend** statt mit erlogenen `(0,0,0)` geliefert (das `eprintln "deltas carry zero"` entfernt); die Aufrufer `iau_rotate_to_icrs`, `body_fixed_vector_to_icrs`, `body_fixed_to_icrs_smooth`, `icrs_to_body_fixed` tragen `?`. (2) `src/mathematikerin/machines/matrix.rs:786` — die hartkodierte Erd-Schiefe `23.4392911` entfernt; die Rotation des Relativvektors in den **Anker-Rahmen** über die neue pub-Fn `motion.rs::rotate_icrs_to_body_frame(anchor_bp, r, jd)` (aus dem `else`-Zweig von `icrs_to_body_fixed` herausgezogen, ohne Baryzentrum-Subtraktion); `lon = by.atan2(bx)`; fehlende Anker-Pole → `return`. **Rat (5 Stimmen, River 132) Verdikt (a):** die Serie ist als *Longitude im Anker-Rahmen* gemeint, nicht als Erd-Ekliptik-Länge; die Erd-Ekliptik ist für erd-fremde Anker ein Riss. Semantische Korrektur → die Erd-Serie ist neu zu messen. **Riss (nicht geglättet):** `state/future/giftkarte-klassifiziert-src-2026-10-07.md:347` klassifiziert `motion.rs:82/86` (`nut_ra`/`nut_dec` `None => 0.0`) als FABRICATION — `src/archivar/pck.rs:57-71` behandelt dasselbe `None` als vollständiges Modell ohne Nutationskorrektur (`pole_ra_at → Some(base)`); beide Implementierungen stimmen überein, die Klassifikation ist der Riss. Offen weiter: `shaders.rs:5-15` (WP11 — `PROPAGATION_SPEED`-Medien-Konstanten), Riss A (Provenienz der sechs Medium-Werte `pending`), sowie getrennt `kepler.rs:3` `OBLIQUITY_DEG` (Rat-Notiz: Erd-Bahn-Kontext, site-für-site auditen — null-echt oder derselbe Riss).
- **Blockade:** Riss C (`kernel_extent`-Konflation, `membrane.rs:302-335`) vor dem Umbenennen entscheiden; Riss A (Provenienz der sechs Medium-Werte); Riss B (per-field `Option` vs. atomarer Block); der `motion.rs:82/86`-Riss (Inventar vs. `pck.rs`) braucht ein Rat-/Operator-Wort, bevor er „geheilt" wird.
- **Braucht:** `shaders.rs:5-15` (WP11) — die Medien-Konstanten durch die wire-`absorption` ersetzen; `kepler.rs:3` site-für-site messen. Auftrag `docs/auftrag/auftrag-bias-tilgung.md` (Operator-Wort 2026-10-07); Inventory `state/future/giftkarte-klassifiziert-src-2026-10-07.md`.

### NUR-Asset — Re-Harvest hängt in der CI-Queue
- **Status:** wartend (Mycelium) | **Bindung:** eigen (cross-line mycelium)
- **Trigger:** `image-cdn.yml`-Lauf mit einem Fenster in 1999–2023, der einen neuen sha setzt.
- **Lage:** (gemessen 2026-10-07 13:31Z via `ci_manage view 37621964105`) `queued` seit 12:34Z; `phi/sources.φ:18034` unverändert `9c76f881…`.
- **Blockade:** CI-Queue (self-hosted).
- **Braucht:** Lauf-Ausgang + neuer sha; dann `cargo run -p omegaflow-measure --bin nur_gic_relation_probe` auf dem Asset.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon). Wahrheit: `state/zustand/wartend.φ` (`flyby-chain-omni2`, `flyby-chain-kp-def`, `ephemeris-juice-recon`).
- **Lage:** (gemessen 2026-10-06, River 105) OMNI2 26 Zellen `pending`; kp `def` leer; JUICE-recon absent (Wiedervorlage 2026-11-01).
- **Blockade:** externe Kanäle; kein Polling.
- **Braucht:** `flyby_path2_fill`-Lauf lesen + Addendum fortschreiben; Trigger feuern lassen.

## An mountain

Origin: river-132.

- **Konkurrierende uncommittete Arbeit blockiert `cargo check`:** `src/archivar/igrf.rs:229` (`pnm[n][n + 1] = (2 * n) as f64.sqrt() * …`) bricht die Krate („cast cannot be followed by a method call"; `((2 * n) as f64).sqrt()`). Zur Sessionzeit uncommittet (Billanz `session_burn`: die IGRF-Dispatches `grind-flash`/`general`); River hat die Datei nicht berührt. Bitte schließen. Neue ungetrackte `src/archivar/igrf14coeffs.txt` ebenso.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort 2026-09-29; kein Maschinen-Akt.
- **Förder-Bewerbungen Prototype Fund (Frist 30.11.) + EMAP (Frist 06.11.)** — Operator-Wort 2026-10-07: bleiben **LOCK**; Send = Operator-Hand; Voraussetzung = die Membran rendert (freie Presence + Kräfte).

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session (River 132):

- `src/archivar/motion.rs` (Nutation → `Option`; `rotate_icrs_to_body_frame` herausgezogen)
- `src/mathematikerin/machines/matrix.rs` (Erd-Schiefe entfernt, Anker-Rahmen-Rotation)
- `docs/handover/handover-2026-10-07-river-folge132.md`
- `docs/handover/archiv/handover-2026-10-07-river-folge131.md` (Move)

## Burn: open 0.0000 · close 0.0474 (line, deepseek-flash, `session_burn`, gemessen 2026-10-07) · cap 0.35 Grund: ein Ein-Pass-Atom — zwei bounded Bias-Fixes (`motion.rs` Nutation→Option, `matrix.rs` Anker-Rahmen) + eine Rat-Runde; kein pro/max · Dispatches: `council` (1: Rat Erd-Schiefe / matrix.rs:786)
