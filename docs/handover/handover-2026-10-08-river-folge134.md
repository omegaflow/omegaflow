<!--
  title: Handover — River-Folge 134 (2026-10-08)
  session: River-Folge 134
  class: handover
  date: 2026-10-08
  sha256: c92360a0d6e993f8243f69cf6041c58d6b118fa09103d0e5ba612f45403b28ad
  status: live
-->
# Handover — River-Folge 134 (2026-10-08)

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
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-river-folge133.md` §Operator-Wort-Register — gefaltet, nicht kopiert. Verbatim: `state/operator-gespraeche/2026-10-07-river.md`.

## Stimmen-Rolle (gemessen 2026-10-07)

Recherche trägt `voice-deepseek`; strikt lokal nur DeepSeek-flash (`opencode.json`);
Denken/Urteil = UI-Frontier; der Rat = Form/Linse. Die API-Suchschnittstelle
`archive_search --alphaxiv` (alphaXiv MCP `discover_papers`) ist in
`docs/concepts/tools-map.md` nachgetragen.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie; Siegel = Operator-Wort, offen.
- `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md` — see-also auf Archiv-Pfad geheilt (`:7`).
- `docs/paper/gic-causal-driver.md` — §4.7/§6 NUR-Asset-Fakten (Folge 133 geheilt: neuer sha/Zeile/Fenster).
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` — Objektophilie-Verdikt (`:113` BODIES→Manifest; `:96` operationaler Test `grep static/`). Angewandt 2026-10-07 (River 129).
- `docs/concepts/remove-bias.md` — der Bias-Tilgungsplan (WP0–WP13); WP13-Fixtures gebaut (`47706add5`).

## Offen (aufgeschlüsselt)

### Membran — Kraft-/Kanal-Agnostik: Rat-Wort **B** (Exposition pro `(force_type, aperture)`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains span-Direktive (Presence-Hull-/Star-Grid-span im Datenkontrakt, `docs/concepts/archivar-mathematikerin.md`) → erster nicht-schwarzer Render.
- **Lage:** (gemessen 2026-10-07, River 127/129/131/132/133) `static/membrane.html` trägt keinen Body-Namen mehr (`const BODIES` entfernt); der Hüllen-Manifest `/membrane_bodies.txt` wird zur Laufzeit gelesen, pages-deploy schreibt ihn via `gen_bodies.sh --write` aus den Stage-Lines. Der Shader liest die gemessene Farbe (`color_index` → `/color_lut`); `ci==0` → weiß, LUT fehlt → neutrale Rampe. Der per-Kanal-Expositions-State (`state.lvl` als `Float32Array(9·2)` über `(force_type, aperture)`, Storage-Bindung 4 `exposure`, `lvl = exposure[force_type][aperture]`, Relaxation α = 1−exp(−1/8), Apertur = wire `extent` >0 Anker / 0 Sterne, fehlender Schlüssel → 0, kein Wire-Bit) ist gebaut (River 131). **Der Deploy-Trigger ist gefeuert:** `pages-deploy 37685135772` **success** an `c28ce137d` (gemessen 2026-10-07 via `ci_manage view`). Der Membran-Schritt läuft durch; das neue `membrane.html` ist deployt.
- **Blockade:** der Start-Anker (schwarzes Feld/`scale 0`) hängt an Mountains fehlender span-Direktive.
- **Braucht:** Mountains span-Direktive (s. `## An mountain`); dann den ersten Render gegen den deployten `membrane.html` prüfen.

### `span`-Apertur — River nennt die Signatur/das Ort (aus mountain-270/271 gefaltet)
- **Status:** wartend | **Bindung:** eigen (Rat)
- **Trigger:** Rat-Verdikt zur Semantik der span-Apertur (`handover-2026-10-07-mountain-folge271.md`).
- **Lage:** (gemessen 2026-10-07 via `register_lookup --addressed river`) Mountain-Arm (`parse.rs`) steht; die statische Membran liest kein `SourceConfig`. Mountain-271 bittet River um die konkrete Signatur/den Ort (Query-/Receiver-Input bzw. Record-`extent` in `wasm.rs:65-99` + `membrane.html:501-528`); die Semantik geht zuvor durch die fünf Stimmen.
- **Blockade:** Architektur-Entscheidung — Form/Linse (Rat), nicht pro-solo.
- **Braucht:** die `span`-Apertur als Query-/Receiver-Input bzw. Record-`extent`-Semantik durch den Rat (fünf Stimmen) + UI-Frontier halten; danach die konkrete Signatur in `wasm.rs:65-99`/`membrane.html:501-528` nennen.

### `em nmgy`-Riss (aus mountain-270/271 gefaltet)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine gemessene Band-Konversion auf W/m²/Hz (`phi/sources.φ`).
- **Lage:** (gemessen 2026-10-07 via `register_lookup --addressed river`) der Rat hält die Zeile als `quantity scale nmgy`; sie wandert nach `em`, sobald eine Band-Konversion auf W/m²/Hz gemessen ist. Bis dahin Riss, nicht geglättet.
- **Blockade:** fehlende Band-Konversion (Messung).
- **Braucht:** die Band-Konversion messen; dann die Zeile von `quantity scale nmgy` nach `em` umsetzen.

### GIC-Stufe-2 — dB/dt abgeleitet; der Familien-Pool ist ein eigener Bau
- **Status:** wartend | **Bindung:** eigen (cross-line mountain)
- **Trigger:** `cgm_lat`-Registerzeile je Station (Mountain) + UI-Undergirding (`src/archivar/types.rs:439`).
- **Lage:** (gemessen 2026-10-07, River 127/128/131) `intermagnet_dbdt_compiler.rs:5-6,179` leitet `sqrt(dx²+dy²+dz²)` aus derselben `/best-avail/PT1M/xyzf`-Quelle ab; ABK/SOD-dbdt (`phi/sources.φ:2057-2085`) ist kein natives INTERMAGNET-Produkt. Gebaut: `series_dbdt` (`src/archivar/main_flow.rs:5925`); der Loader `tools/measure/src/bin/field_te_query.rs` löst `intermagnet_dbdt_<station>` ab (`align_xyz` → `series_dbdt`, `load_dbdt_across_sources:1163`). Rat (5 Stimmen, River 131) + Recherche: (a) **B′** — `cgm_lat` je Station = Registergröße (`SourceConfig.cgm_lat`, `src/archivar/types.rs:439`, `parse.rs:1692`); das Familien-Label live aus `|cgm_lat|` + Grenzen 60/50 (`cgm_lat_partition.rs:168 family_of`); die Partitions-TSV `state/river/gic-cgm-lat.tsv` und `state/river/gic-family-*.txt` sind Träger (kein Code-Konsument), nie Query-Quellort; die IGRF-Live-Ableitung (`src/archivar/igrf.rs:111`) ist toter Pfad (kein Caller) → nur für die 2 QD-Stationen. **Riss** CPL/TTB QD≠CGM (`phi/sources.φ:6324,7669`). (b) **A** — Target-Band; der eine Bz-Treiber ist unbandiert, die geteilte `null_matrix`-Ziehung (`src/mathematikerin/wy_max_t.rs:325`) bleibt kohärent, je Familie ein eigener `compute_max_t:2824`-Call.
- **Blockade:** `cgm_lat`-Registerzeile je Station (Mountain) + UI-Undergirding (Operator-Wort 2026-10-07: Rat vor alle UI-Seats) + α-Split/Blocklänge im Blatt (Future).
- **Braucht:** den separaten Familien-Bau — neue Fn nahe `run_pair_matrix:4371` + `--stage2 family`-Arm in `main` (~4804); `Vec<Member>` je Familie (`target = station_dBdt`, `driver = globaler Bz`), dann je Familie ein eigener `compute_max_t`-Call; reuse `load_field_across_sources:1129`, `align_many:4137`, `joint_columns:4211`; Stufe 1 byte-identisch. Dann UI-Runde (`river-ui` + `open-weight-ui`).

### Receiver-/em-Apertur, ozzy, Membran — CI-Verifikation / CI-Gate-Verifikation (River 132)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-gate`/`ci-check`-Testjob grün am HEAD.
- **Lage:** (gemessen 2026-10-07T22:29Z via `ci_manage status`) HEAD `bab7cf750`; die `ci-check`-Läufe am sich bewegenden HEAD (`37695929486` an `0c1e1c86f` = **cancelled**; `37696474527` an `bab7cf750` = pending) werden von der Concurrency-Churn (parallele Commits) verdrängt. `ci-gate 37695929600` an `0c1e1c86f` = **success** (format/build/clippy/dropped-gate/register). Gebaut: `series_dbdt` + Loader-Ableitung; ozzy `independence_verdict` (`ozzy.rs:146`); em-Apertur (`mathematikerin/shaders.rs:186,211`); Membran per-Kanal-Exposition (`membrane.html`, River 131). **Nachgemessen 2026-10-08T06:38Z via `ci_manage log 37738482986`/`ci_manage list`:** HEAD `71b924e6a` == `origin/main`; `ci-gate 37738482986` = **failure** — Job `register` rot (`url-order violation within ttl 2592000: usgs_comcat_m45.bin placed after hadisst_sst.bin`) + Job `dropped-gate` rot (dieser Punkt, Key `132 ci-gate-verifikation river`, keine belegende Träger-/Commitzeile). Rivers Code ist grün: `format`/`build`/`clippy` liefen in `37738482986` durch (nur `register`/`dropped-gate` rot). `ci-check` ist seit `3db5a3ad4` kein push-getriggerter Lauf mehr (nightly/on-demand) — im `ci_manage list` erscheint kein `ci-check`.
- **Blockade:** Job `register` = Mountain (USGS-Block im falschen TTL-Fenster); Job `dropped-gate` = dieser Punkt, mit der Fortschreibung in `handover-2026-10-08-river-folge134.md` als Trägerzeile geführt.
- **Braucht:** den `register`-Sort durch Mountain (USGS-Block nach TTL einordnen); danach einen neuen `ci-gate`-Lauf lesen (`ci_manage status`); ozzy/em-Apertur sind compile-verifiziert.

### Instrumentierung der Membran (aus future-199 gefaltet)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste Membran-Render (nach Mountains span-Direktive, `static/membrane.html`).
- **Lage:** (gemessen 2026-10-07 via `register_lookup --addressed river`) `static/membrane.html:578` verschluckt das boolean von `load_ephemeris`; `record.count` ist nach `extent > 0.0` aufzuschlüsseln; die `absent:`-Statuszeile (`:594-599`) wird vom Frame-`loop()` (`:544`) überschrieben; die Rust-`eprintln!`s (`membrane.rs:407/415`) erreichen die Browser-Konsole nicht.
- **Blockade:** wartet auf Mountain's span-Direktive (erst dann ist die Diagnose am Render sinnvoll).
- **Braucht:** `membrane.html:578` den boolean zurückgeben; `record.count` nach `extent > 0.0` aufschlüsseln; die `absent:`-Zeile nach dem `loop()`-Start setzen.

### Bias-Audit Archivar/Mathematikerin — src-Rest und der Gate-Rückfall
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-07, River 127/129/130/132) Untersuchung `omegaflow-legacy/docs/concepts/remove-bias.md` (WP0–WP13). **Geheilt:** `channels.rs:1423` τ=∞; `nexrad.rs`/`rinex.rs`/`odp.rs`/`media.rs`; `weberin.rs:254/258` Literal-Listen → `woven_major_bodies()` (River 131, `clean_tree` 2→0). **Geheilt (River 132):** (1) `motion.rs:66` `orientation_angles_at` → `Option<(f64,f64,f64)>` — bei vorhandener Nutation und fehlendem Granulat abwesend statt erlogener `(0,0,0)`; Aufrufer tragen `?`. (2) `matrix.rs:786` — hartkodierte Erd-Schiefe `23.4392911` entfernt; Rotation in den Anker-Rahmen über `motion.rs::rotate_icrs_to_body_frame`; fehlende Anker-Pole → `return`. Rat-Verdikt (a): die Serie ist Longitude im Anker-Rahmen, nicht Erd-Ekliptik-Länge; die Erd-Serie ist neu zu messen. (3) `kepler.rs` `OBLIQUITY_DEG` → `ECLIPTIC_J2000_OBLIQUITY_DEG` + Standort-Name (null-echt). (4) `BodyProperties.nut_ra/nut_dec` produzentenlos → gelöscht. (5) `shaders.rs` `PROPAGATION_SPEED`: `C_VACUUM` nur für 0/1/8, 2–7 = `0.0` (absent); `AUDIO_SPEED_AIR` entfernt. **src-Rest laut future-199 (`state/future/giftkarte-klassifiziert-src-2026-10-07.md`):** `remove-bias.md` WP8/9/11 (`media.rs`, `shaders.rs` → `BodyProperties`), WP13-Konstanten (`odp.rs:9`, `rinex.rs:5,58`, `nexrad.rs:188`), `matrix.rs:786`, `channels.rs:1423`, `motion.rs:77`.
- **Blockade:** keine Medien-Datenpfad-Blockade mehr (nachgemessen River 134). Die frühere Zeile „der Medien-Datenpfad ist nicht gebaut — `BodyProperties` stype==2 parst **5** f64 (`motion.rs:667/688`), `remove-bias.md` WP12 schreibt **6** (Stride-Lücke gemessen); Riss A (Provenienz der sechs Medium-Werte); Riss C (`kernel_extent`-Konflation) vor dem Umbenennen" ist gegen den Baum **widerlegt**: `MediumParams`/`wire()` tragen fünf f64 selbstkonsistent (`src/mathematikerin/media.rs:2-19`), WP12's sechs ist eine ungebaute Plan-Formulierung; Riss C ist mit `b3dd7fb4c` (River 132) geheilt (`kernel_extent` → `medium_reach`, kein `kernel_extent` mehr in `src/`). Verbleibend nur Mountain's `kepler.rs`-PROD-POISON-`descoped`.
- **Braucht:** (a) **gegenstandslos (gemessen River 134):** die Forderung `motion.rs:667` stype==2 auf 6 f64 / `pos += 48` korrigieren (WP12) und die sechs Medium-Werte benennen (`v_sound`, `v_seismic_p`, `v_seismic_s`, `alpha_thermal`, `d_diffusion`, `v_advective`) — sie reisen über den `advection`-Slot (Kraft-7-Präzedenz), kein neuer Wire-Slot; der Baum trägt fünf f64. (b) **erledigt (River 132, `b3dd7fb4c`):** `membrane.rs:302` `kernel_extent` → `medium_reach` entflechten und den `force_type==1`-Radius-Zweig löschen (Doppelzählung `fetch.rs:554-558`). (c) **bleibt:** den `kepler.rs`-PROD-POISON-Eintrag als `descoped` mit Befund registrieren (Mountain). Auftrag `docs/auftrag/auftrag-bias-tilgung.md` (Operator-Wort 2026-10-07); Inventory `state/future/giftkarte-klassifiziert-src-2026-10-07.md`.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon). Wahrheit: `state/zustand/wartend.φ` (`flyby-chain-omni2`, `flyby-chain-kp-def`, `ephemeris-juice-recon`).
- **Lage:** (gemessen 2026-10-06, River 105) OMNI2 26 Zellen `pending`; kp `def` leer; JUICE-recon absent (Wiedervorlage 2026-11-01).
- **Blockade:** externe Kanäle; kein Polling.
- **Braucht:** `flyby_path2_fill`-Lauf lesen + Addendum fortschreiben; Trigger feuern lassen.

## An mountain

Origin: river-134.

- **`cgm_lat`-Registerzeile je Station (GIC-Stufe-2-Vorbedingung):** der Familien-Bau (`--stage2 family`) liest das Familien-Label live aus `SourceConfig.cgm_lat` (`src/archivar/types.rs:439`, `parse.rs:1692`) — die Partitions-TSV `state/river/gic-cgm-lat.tsv` ist Träger, kein Query-Quellort. Für die 2 QD-Stationen (CPL/TTB, `phi/sources.φ:6324,7669`) ist `cgm_lat` noch nicht als Registerzeile deklariert. Bitte die `cgm_lat`-Zeile je Station setzen (Quellen-Eigenschaft).
- **`kepler.rs`-PROD-POISON-`descoped`:** der Befund liegt vor — J2000-Ekliptik-Definition heliozentrischer Elemente, fünf Caller verifiziert, jetzt `ECLIPTIC_J2000_OBLIQUITY_DEG` mit Standort-Name (River 132). Bitte den Inventar-Eintrag (`giftkarte:133`) mit diesem Befund auf `descoped` setzen (Quellen-Verdikt = Mountain).
- **Membran-Start-Anker (`scale 0`/schwarzes Feld):** hängt an Mountains **span-Direktive** (Presence-Hull-/Star-Grid-span im Datenkontrakt). River hat die per-Kanal-Exposition gebaut und deployt (`pages-deploy 37685135772` success); ohne den Start-Anker bleibt das erste Bild schwarz. Bitte den Schritt der span-Direktive nennen bzw. setzen.
- **GM-Anker:** laut future-199 ist der fehlende GM-Anker der Mountain-Teil; die Sonne erscheint, wenn beide stehen (die Render-Ursache liegt nicht im Frontend).

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort 2026-09-29; kein Maschinen-Akt.
- **Förder-Bewerbungen Prototype Fund (Frist 30.11.) + EMAP (Frist 06.11.)** — Operator-Wort 2026-10-07: bleiben **LOCK**; Send = Operator-Hand; Voraussetzung = die Membran rendert (freie Presence + Kräfte).

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session (River 134):

- `docs/handover/handover-2026-10-08-river-folge134.md`
- `docs/handover/archiv/handover-2026-10-07-river-folge133.md` (Move)

## Burn: open 0.0000 · close 0.0888 (line, deepseek-flash, `session_burn`, gemessen 2026-10-08) · cap 0.35 Grund: Ein-Pass-Atom — dropped-gate-Belegzeile (`132 ci-gate-verifikation river`) gefaltet, CI-Gate-Log gemessen, Bias-Audit-Stride/`kernel_extent` am Baum widerlegt; kein pro/max
