<!--
  title: Handover — River-Folge 132 (2026-10-07)
  session: River-Folge 132
  class: handover
  date: 2026-10-07
  sha256: 6025786bd2eb62c70e934772d347a1f9cf2858978ee7edb93c3e610be0a9f903
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

## Stimmen-Runde (River 132) — Bias-Audit-Entscheidungen

Forschungsschicht (`archive_search`): `--ads "nutation series Earth orientation mean pole model J2000"` → IAU2000A-Precession/Ekliptik (`2003A&A...400..785B`); `--crossref "planetary regolith seismic velocity thermal diffusivity"` → Seismic velocity characterisation of planetary simulants (EPSC2026), Thermal diffusivity of olivine (`10.1016/0012-821x(75)90059-x`) — Medium-Werte sind gemessene Gesteins-/Regolith-Eigenschaften, keine Kraftart-Konstanten.

Rat (council, 2 Runden) + UI-Frontier (`river-ui`: Claude Sonnet 5.5, Qwen3.7-Plus). Konvergenz:

- **Nutation:** fehlender `NUT_PREC`-Block im PCK = vollständiges lineares Modell (`pck.rs:57-71` korrekt, 0 null-echt); `BodyProperties.nut_ra/nut_dec` sind produzentenlos (nur `PckBody`, `pck.rs:121`) → **gelöscht** (vorgeschlagene Struktur, nicht verdrahtet); River-132-`None` bei vorhandener Serie + fehlendem Granulat bleibt. Inventar-Label `giftkarte:347` ist eine Muster-Behauptung (das Feld hatte keinen Produzenten) → Riss benannt, nicht geglättet.
- **`kepler.rs`:** null-echt — J2000-Ekliptik-Definition heliozentrischer Elementsätze (5 Caller verifiziert: `cometels`, `dastcom`, `extract`, `kbo`, `mpcorb`) → umbenannt `ECLIPTIC_J2000_OBLIQUITY_DEG` + Standort-Name (Kommentar verboten: Code-Gate; Name = Implementation).
- **`PROPAGATION_SPEED` (`shaders.rs`):** `c` für 0/1/8 null-echt; 2–7 sind Erd-Medium-Werte → absent (`0.0` → keine Zeitkorrektur, `val_eff_at:116`). Medium-Daten reisen künftig wie Kraft 7 über den `advection`-Slot (gebauter Präzedenzfall), kein neuer Wire-Slot.
- **`kernel_extent`:** `kernel_id` ist die Form-Achse (Gauß/erfc/exp/Levy), nicht die Medium-Achse; der Gravitations-Radius wird doppelt gezählt (`body_term` `fetch.rs:554-558` + `kernel_extent` `force_type==1`) → entflechten (nächster Schritt).
- **Open-Weight-Seats (`open-weight-ui`, Lock river 132 gesetzt/zurückgegeben, Tabs nach der Runde geschlossen):** Kimi K3 + 4× GLM 5.3 Flash (Z.ai-Cloud-Fallback). Konvergent: Nutation = abwesende Messung/toter Platzhalter → `nut_ra` löschen (Single Source: Chebyshev); Ausbreitungsgeschwindigkeit = Medium-Eigenschaft am Körper (b), die `c`-Einträge bleiben Kraftart-Konstanten; `kernel_id` ist die falsche Medium-Achse, Gravitations-Radius doppelt gezählt (`max` statt Summe). Ein Seat trug noch eine vorige Frage (Commit-Gate/Frame-Origin) — nicht durchmischt.
- **Nicht-Antworten (gemessen):** Duck.ai Tageslimit erreicht („Wird in 4h zurückgesetzt"); Z.ai (Frontier, `river-ui`) GLM-5.3 hat in ~3 min keine Antwort gerendert → `pending`, nicht wiederholt.

## Offen (aufgeschlüsselt)

### Membran — Kraft-/Kanal-Agnostik: Rat-Wort **B** (Exposition pro `(force_type, aperture)`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `pages-deploy`-Lauf am neuen `membrane.html`.
- **Lage:** (gemessen 2026-10-07, River 127/129/131/132) **Gebaut (River 129, Operator-Wort):** `static/membrane.html` trägt keinen Body-Namen mehr — `const BODIES` entfernt; der Hüllen-Manifest `/membrane_bodies.txt` wird zur Laufzeit gelesen, pages-deploy schreibt ihn via `gen_bodies.sh --write` aus den Stage-Lines. Der Shader liest die gemessene Farbe (`color_index` → `/color_lut`); `ci==0` → weiß, LUT fehlt → neutrale Rampe. **Gebaut (River 131):** der per-Kanal-Expositions-State — `state.lvl` als `Float32Array(9·2)` über `(force_type, aperture)`, Storage-Bindung 4 `exposure`, im Vertex-Shader `lvl = exposure[force_type][aperture]`, Relaxation α = 1−exp(−1/8); Apertur = wire `extent` (>0 Anker, 0 Sterne); fehlender Schlüssel → 0, kein gezeichneter Level, kein Default; **kein** Wire-Bit. `extent` bleibt die Apertur; Träger = `force_type`. **Gemessen (River 132):** `pages-deploy` `37679754275` brach am `sha256 mismatch for ephemeris_de440_earth.bin` ab — **außerhalb der Linie**: `mycelium-folge264` hat die Pins in `816841a60` mit dem Register synchronisiert (`8b8998bd…`); der Membran-Schritt selbst (`cp static/membrane.html` + `gen_bodies.sh --write`) läuft durch. Der Deploy des neuen `membrane.html` ist damit nur noch an einen frischen `pages-deploy`-Lauf gebunden.
- **Blockade:** der Start-Anker (schwarzes Feld/`scale 0`) hängt an Mountains fehlender span-Direktive.
- **Braucht:** den frischen `pages-deploy`-Lauf lesen (`ci_manage status`; River 132 dispatcht: `37685135772`); den Start-Anker an Mountains span-Direktive.

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
- **Lage:** (gemessen 2026-10-07, River 127/129/130/132) Untersuchung `omegaflow-legacy/docs/concepts/remove-bias.md` (WP0–WP13). **Geheilt:** `channels.rs:1423` τ=∞; `nexrad.rs`/`rinex.rs`/`odp.rs`/`media.rs`; `weberin.rs:254/258` Literal-Listen → `woven_major_bodies()` (River 131, `clean_tree` 2→0). **Geheilt (River 132):** (1) `src/archivar/motion.rs:66` `orientation_angles_at` → `Option<(f64,f64,f64)>` — bei vorhandener Nutation (`bp.nutation.is_some()`) und fehlendem Granulat wird die Orientierung **abwesend** statt mit erlogenen `(0,0,0)` geliefert (das `eprintln "deltas carry zero"` entfernt); die Aufrufer `iau_rotate_to_icrs`, `body_fixed_vector_to_icrs`, `body_fixed_to_icrs_smooth`, `icrs_to_body_fixed` tragen `?`. (2) `src/mathematikerin/machines/matrix.rs:786` — die hartkodierte Erd-Schiefe `23.4392911` entfernt; die Rotation des Relativvektors in den **Anker-Rahmen** über die neue pub-Fn `motion.rs::rotate_icrs_to_body_frame(anchor_bp, r, jd)` (aus dem `else`-Zweig von `icrs_to_body_fixed` herausgezogen, ohne Baryzentrum-Subtraktion); `lon = by.atan2(bx)`; fehlende Anker-Pole → `return`. **Rat (5 Stimmen, River 132) Verdikt (a):** die Serie ist als *Longitude im Anker-Rahmen* gemeint, nicht als Erd-Ekliptik-Länge; die Erd-Ekliptik ist für erd-fremde Anker ein Riss. Semantische Korrektur → die Erd-Serie ist neu zu messen. **Geheilt (River 132, Rat + UI + Recherche konvergent — s. §Stimmen-Runde):** (3) `src/archivar/kepler.rs` `OBLIQUITY_DEG` → `ECLIPTIC_J2000_OBLIQUITY_DEG` + Standort-Name (Kommentar verboten: Code-Gate; Name = Implementation) (null-echt). (4) `BodyProperties.nut_ra/nut_dec` produzentenlos → gelöscht; `orientation_angles_at` summiert `nut_ra`/`nut_dec` = `0.0` direkt; 20 Literale bereinigt; `nutation_sum` ohne Aufrufer entfernt. (5) `src/mathematikerin/shaders.rs` `PROPAGATION_SPEED`: `C_VACUUM` nur für 0/1/8, 2–7 = `0.0` (Erd-Medium-Werte entfernt → absent, `val_eff_at:116` = keine Zeitkorrektur); `AUDIO_SPEED_AIR` entfernt. `cargo check` grün, 0 Warnungen. **Riss (nicht geglättet):** `state/future/giftkarte-klassifiziert-src-2026-10-07.md:347` nannte `motion.rs:82/86` FABRICATION; gemessen hatte das Feld keinen Produzenten — die Muster-Behauptung bleibt als Riss benannt.
- **Blockade:** der Medien-Datenpfad ist nicht gebaut — `BodyProperties` stype==2 parst **5** f64 (`motion.rs:667/688`), `remove-bias.md` WP12 schreibt **6** (Stride-Lücke gemessen); Riss A (Provenienz der sechs Medium-Werte); Riss C (`kernel_extent`-Konflation) vor dem Umbenennen.
- **Braucht:** `motion.rs:667` stype==2 auf 6 f64 / `pos += 48` korrigieren (WP12) und die sechs Medium-Werte benennen (`v_sound`, `v_seismic_p`, `v_seismic_s`, `alpha_thermal`, `d_diffusion`, `v_advective`), statt der Kernel-Namen (`gaussian_inverse_square` …) — sie reisen über den `advection`-Slot (Kraft-7-Präzedenz), kein neuer Wire-Slot; `membrane.rs:302` `kernel_extent` → `medium_reach` entflechten und den `force_type==1`-Radius-Zweig löschen (Doppelzählung `fetch.rs:554-558`); den `kepler.rs`-PROD-POISON-Eintrag als `descoped` mit Befund registrieren (Mountain). Auftrag `docs/auftrag/auftrag-bias-tilgung.md` (Operator-Wort 2026-10-07); Inventory `state/future/giftkarte-klassifiziert-src-2026-10-07.md`.

### `medium_reach`-Reste (nach `b3dd7fb4c` + `0bc9af118`)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-07, River 132) **Gebaut/committet:** `b3dd7fb4c` (`MediumParams` → `BodyEphemeris.medium`; `kernel_extent` → `medium_reach(force_type, age, medium, advection)`; Gravitations-Radius-Zweig gefallen; Front einmal; `AUDIO_SPEED_AIR` → `AIR_SOUND_SPEED_M_S`; zwei `unwrap_or(0.0)` geheilt), `0bc9af118` (Matrix-Gate auf `age = |t_presence − meta.epoch|` + `meta.advection`), **`402aa6ee3`** (`flat_propagation_speed`-Erd-Konstanten entfernt → Kraft 2–6 absent, CPU = GPU-Schnitt), **`78eb65d67`** (`EnclosureField.body_props` write-only entfernt). Der „5→6-Stride" war ein Falschbefund (Writer/Parser/TSV = 5). `cargo check --tests` grün. Die zwei ehemals am geteilten `tests.rs` hängenden Schritte liefen nach Mountain's `rights`-Commit (`87aed0b14`) in diesem Pass.
- **Blockade:** keine.
- **Braucht:** **Riss:** `0bc9af118` erweiterte die State-Magic `OMX3` um `epoch`/`advection` ohne Bump auf `OMX4` (die Assertion `machines/tests.rs:296 assert saved.starts_with(b"OMX3")`); die State-Datei ist lokal/gitignored, der Loader best-effort — der saubere Abschluß wäre ein `OMX4`-Bump samt Assertion.

### CI-Gate-Rot auf `9f582918b` — drei geheilt, einer Mountain's
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** neuer `ci-gate`-Lauf am HEAD `78eb65d67`.
- **Lage:** (gemessen 2026-10-07 via `ci_manage log 37691117550` + `37692475312`) fünf Ursachen: (1) clippy `src/weberin.rs:473/482` `map_or(false, …)` → `is_some_and`; (2) format `src/mathematikerin/machines/matrix.rs:750`; (3) path_reference_scan `docs/concepts/remove-bias.md:7` see-also → `docs/handover/archiv/handover-2026-10-07-river-folge127.md` — **geheilt `ab5fd7511`**; (4) clippy `src/archivar/igrf.rs:165` `needless_range_loop` + (5) format `igrf.rs:194/284` — **geheilt `d121288c4`**. Der `dropped-set-gate`-Schlüssel (`auftrag auftrag-bias-tilgung bias-tor …`) ist **Mountain's Bias-Tor-Punkt** (`handover-2026-10-07-mountain-folge270.md:131`) — nicht River.
- **Blockade:** die Dropped-Set-Baseline/der Bias-Tor gehört Mountain.
- **Braucht:** neuen `ci-gate`-Lauf am HEAD `78eb65d67` lesen (`ci_manage status`); Mountain's Teil bleibt dessen.

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

- **`cgm_lat`-Registerzeile je Station (GIC-Stufe-2-Vorbedingung):** der Familien-Bau (`--stage2 family`) liest das Familien-Label live aus `SourceConfig.cgm_lat` (`src/archivar/types.rs:439`, `parse.rs:1692`) — die Partitions-TSV `state/river/gic-cgm-lat.tsv` ist Träger, kein Query-Quellort. Für die 2 QD-Stationen (CPL/TTB, `phi/sources.φ:6324,7669`) ist `cgm_lat` noch nicht als Registerzeile deklariert. Bitte die `cgm_lat`-Zeile je Station setzen (Quellen-Eigenschaft), dann kann River den Familien-Bau dispatchen. (Der igrf.rs-Parse-Fehler aus meiner Session ist mit `c28ce137d` geschlossen — zurückgezogen.)
- **`kepler.rs`-PROD-POISON-`descoped`:** der Befund liegt vor — J2000-Ekliptik-Definition heliozentrischer Elemente, fünf Caller verifiziert, jetzt `ECLIPTIC_J2000_OBLIQUITY_DEG` mit Standort-Name (Kommentar verboten: Code-Gate; Name = Implementation) (River 132). Bitte den Inventar-Eintrag (`giftkarte:133`) mit diesem Befund auf `descoped` setzen (Quellen-Verdikt = Mountain).
- **Membran-Start-Anker (`scale 0`/schwarzes Feld):** hängt an Mountains **span-Direktive** (Presence-Hull-/Star-Grid-span im Datenkontrakt). River 127/129/132 hat die per-Kanal-Exposition gebaut; ohne den Start-Anker bleibt das erste Bild schwarz. Bitte den Schritt der span-Direktive nennen bzw. setzen — River hängt daran.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort 2026-09-29; kein Maschinen-Akt.
- **Förder-Bewerbungen Prototype Fund (Frist 30.11.) + EMAP (Frist 06.11.)** — Operator-Wort 2026-10-07: bleiben **LOCK**; Send = Operator-Hand; Voraussetzung = die Membran rendert (freie Presence + Kräfte).

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session (River 132):

- `src/archivar/motion.rs` (Nutation → `Option`; `rotate_icrs_to_body_frame` herausgezogen; producerlose `nut_ra`/`nut_dec` gelöscht)
- `src/archivar/kepler.rs` (`ECLIPTIC_J2000_OBLIQUITY_DEG` + Standort-Name (Kommentar verboten: Code-Gate; Name = Implementation))
- `src/mathematikerin/shaders.rs` (`PROPAGATION_SPEED`: 2–7 absent; `AUDIO_SPEED_AIR` entfernt)
- `src/mathematikerin/machines/matrix.rs` (Erd-Schiefe entfernt, Anker-Rahmen-Rotation)
- `src/archivar/tests.rs`, `src/mathematikerin/tests.rs`, `src/mathematikerin/machines/tests.rs`, `src/mathematikerin/s2.rs` (Test-Literale bereinigt)
- `docs/handover/handover-2026-10-07-river-folge132.md`
- `docs/handover/archiv/handover-2026-10-07-river-folge131.md` (Move)

## Burn: open 0.0000 · close 0.0474 (line, deepseek-flash, `session_burn`, gemessen 2026-10-07) · cap 0.35 Grund: Ein-Pass-Atom — Bias-Fixes (Nutation/Matrix/Kepler/Shaders) + Recherche + 2 Rat-Runden + UI-Frontier-Runde; kein pro/max · Dispatches: `council` (3: Matrix-Schiefe, Nutation/Kepler, Shaders/Kernel), `grind-flash` (3: Kepler-Rename, Nut-Feld-Löschung, Shaders-Konstanten)
