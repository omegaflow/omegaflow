<!--
  title: Handover — River-Folge 135 (2026-10-08)
  session: River-Folge 135
  class: handover
  date: 2026-10-08
  sha256: 846b35aed0300c26386117de703f4cd29eddc0ce3b581353e29a308d7f3d9e70
  status: live
-->
# Handover — River-Folge 135 (2026-10-08)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„warum nur duck … ich möchte dass du alle frontier chats befragst" | 2026-10-07 | Operator (Session, River 127) — alle offenen UI-Seats, nicht einer
„es kommen doch keine sterne oder die sonne oder der mars an der presence an es kommen die kräfte also die kanäle/oszillatoren an ich glaube ihr habt irgendwann wieder die objektophilie eingeführt und euch vom agnostizismus wegbewegt" | 2026-10-07 | Operator (Session, River 127) — Kraft-/Kanal-Agnostik statt Objekt-Render
„es gibt keine sonne erde mond die presence kann sich frei durch das 4d block universum bewegen … sie spawnt nur am SSB weil euer bias sonst noch größer wäre von da kann sie sich völlig frei bewegen" | 2026-10-07 | Operator (Session, River 127) — freie Presence-Weltlinie, SSB-Spawn, keine Objekte
„Die Förder-Bewerbungen bleiben LOCK … Send bleibt deine Hand" | 2026-10-07 | Operator (Session, River 127) — Prototype Fund (30.11.) + EMAP (06.11.) bleiben LOCK
„auf jeden fall agnostoisch dein vorgänger hat doch schon eine umfangreiche gibt und bias untersuchung gemacht ist die schon wiedre vergessen?" | 2026-10-07 | Operator (Session, River 129) — Wort für den agnostischen Membran-Edit
„bitte ratsfragen auch vor ALLE UI chats bringen" | 2026-10-07 | Operator (Session, River 130) — Ratsfragen vor alle UI-Seats; als Regel in `AGENTS.md` eingetragen
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-river-folge133.md` §Operator-Wort-Register — gefaltet, nicht kopiert. Verbatim: `state/operator-gespraeche/2026-10-07-river.md`.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie; Siegel = Operator-Wort, offen.
- `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md` — see-also auf Archiv-Pfad geheilt (`:7`).
- `docs/paper/gic-causal-driver.md` — §4.7/§6 NUR-Asset-Fakten (Folge 133 geheilt: neuer sha/Zeile/Fenster).
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` — Objektophilie-Verdikt (`:113` BODIES→Manifest; `:96` operationaler Test `grep static/`). Angewandt 2026-10-07 (River 129).
- `docs/concepts/remove-bias.md` — der Bias-Tilgungsplan (WP0–WP13); WP13-Fixtures gebaut (`47706add5`).

## Offen (aufgeschlüsselt)

### Membran — Kraft-/Kanal-Agnostik: Rat-Wort B (Exposition pro `(force_type, aperture)`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains span-Direktive (Presence-Hull-/Star-Grid-span im Datenkontrakt, `docs/concepts/archivar-mathematikerin.md`) → erster nicht-schwarzer Render.
- **Lage:** (gemessen 2026-10-07, River 127–133) `static/membrane.html` trägt keinen Body-Namen mehr; der Hüllen-Manifest `/membrane_bodies.txt` wird zur Laufzeit gelesen, pages-deploy schreibt ihn via `gen_bodies.sh --write`. Der Shader liest die gemessene Farbe (`color_index` → `/color_lut`); `ci==0` → weiß, LUT fehlt → neutrale Rampe. Der per-Kanal-Expositions-State (`state.lvl` als `Float32Array(9·2)` über `(force_type, aperture)`, Storage-Bindung 4 `exposure`, Relaxation α = 1−exp(−1/8); Apertur = wire `extent` >0 Anker / 0 Sterne, fehlender Schlüssel → 0, kein Wire-Bit) ist gebaut (River 131). Deploy-Trigger gefeuert: `pages-deploy 37685135772` success an `c28ce137d`.
- **Blockade:** der Start-Anker (schwarzes Feld/`scale 0`) hängt an Mountains fehlender span-Direktive.
- **Braucht:** Mountains span-Direktive (s. `## An mountain`); dann den ersten Render gegen den deployten `membrane.html` prüfen.

### E0061 Harvest-Ellipsoid — Empfänger aus dem Register (Rat: Route b) — gebaut (Commit in diesem Atom)
- **Status:** eigen | **Bindung:** eigen · mountain (cors-Registerzeile)
- **Trigger:** —
- **Lage:** (gemessen 2026-10-08, `grind-flash` + eigener `cargo check`/`cargo build`) `ecef_to_geodetic(x,y,z,a,e2)` (`rinex.rs:4`). Gebaut: `body_ellipsoid_of(&BodyEphemeris) -> Option<(f64,f64)>` (`rinex.rs:19`; `a=radius_m`, `f=flattening?`, `e2=2f−f²`, absent → `None`); `receiver_body_for_compiler(bin) -> Option<String>` (`:30`, liest `at`/`on` des Blocks, dessen `compiler`-Basename = bin); `receiver_ellipsoid_for_compiler(bin) -> Option<(f64,f64)>` (`:57`, Register-Body → `cdn::body_url` → `fetch_raw_bytes` → `parse_ephemeris_binary`). 6 Call-Sites umgestellt (cses_scm `:166`, cses_hpm `:161`, cses_efd `:324`, cors `:93`, cors_rinex `:86`, champ_plpt `:74`); None → Position absent (0 honored). Kein WGS84-/`"earth"`-Literal. `cargo check` 0/0; alle 6 Bins `Finished`. **Korrektur:** `cors_rinex_compiler` **hat** einen Register-Block (`phi/sources.φ:10589-10590`, `at earth`); nur `cors_compiler` hat keinen.
- **Blockade:** (a) `ephemeris_earth.bin` muss `flattening` (aus `radii_c`) tragen — lokal ungeprüft; fehlt es, bleiben alle 6 absent (0 honored). (b) `cors_compiler`-Registerzeile (Mountain).
- **Braucht:** ein `cargo run -p omegaflow-harvest --bin cors_rinex_compiler` auf kleinem Input (oder der CI-Lauf) prüft die end-to-end-flattening; `cors_compiler`-Block `at earth` (Mountain).

### `em nmgy`-Riss (aus future-199 gefaltet)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eine gemessene Band-Konversion auf W/m²/Hz (`phi/sources.φ`) + `pivot`-Direktiv.
- **Lage:** (gemessen 2026-10-08 via `register_lookup --addressed river`) `phi/sources.φ:19611-19618` `flux_g` trägt `quantity scale nmgy` (Legacy DR10 g, native DECam g); pivot-λ 4808.49 Å (SVO FPS `CTIO/DECam.g`), 1 nMgy = 3.631e-32 W m⁻² Hz⁻¹ (AB, λ-unabhängig). `nmgy` steht nur in `allowed_units_for_quantity(3)`, in keiner Kraft-Liste → ein `em` wäre `Physics Mismatch`; `scale` bleibt korrekt, ein Wechsel nach `em` bräuchte ein `pivot`-Direktiv. BASS-90Prime-g-pivot `pending`.
- **Blockade:** fehlende Band-Konversion (Messung) + `pivot`-Direktiv (Mountain).
- **Braucht:** die Band-Konversion messen; dann `quantity scale nmgy` → `em` mit `pivot`-Direktiv.

### CI-Verifikation — Receiver/em-Apertur, ozzy, Membran (River 132)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-gate` grün am HEAD.
- **Lage:** (gemessen 2026-10-08T13:48Z via `ci_manage log 37787079378` am HEAD `6f956c460`) `ci-gate` failure, zwei Jobs: **`register`** — `url-order violation within ttl 2592000: usgs_comcat_m45.bin placed after hadisst_sst.bin` (`phi/sources.φ`, Mountain); **`dropped-gate`** — `current 62 | pinned 925 | new 2`; die zwei neuen Drop-Keys (lokal reproduziert: `register_lookup --dropped-keys | sort -u` gegen `docs/zustand/dropped-legacy-baseline.txt`) sind `für presence-hull- river schritt span-direktive star-grid-apertur` und `--lpf 200 application arm cgi end fef1238b6 fits gov gsfc hdu heasarc keyless lpf lpf-heasarc nasa registrieren selector start steht` — Token-Bags, keine Pfad-Keys. Rivers Code ist grün: `format`/`build`/`clippy` liefen in `37738482986` durch.
- **Blockade:** `register` = Mountain (USGS-Block im TTL-Fenster); `dropped-gate` = Mountain/Mycelium (Baseline/Token-Bag).
- **Braucht:** Mountain sortiert den USGS-Block; Mountain/Mycelium zieht die dropped-Baseline nach oder löst die 2 Drops; danach neuen `ci-gate`-Lauf lesen (`ci_manage status`).

### Instrumentierung der Membran (aus future-199 gefaltet)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste Membran-Render (nach Mountains span-Direktive, `static/membrane.html`).
- **Lage:** (gemessen 2026-10-07 via `register_lookup --addressed river`) `static/membrane.html:578` verschluckt das boolean von `load_ephemeris`; `record.count` ist nach `extent > 0.0` aufzuschlüsseln; die `absent:`-Statuszeile (`:594-599`) wird vom Frame-`loop()` (`:544`) überschrieben; die Rust-`eprintln!`s (`membrane.rs:407/415`) erreichen die Browser-Konsole nicht.
- **Blockade:** wartet auf Mountains span-Direktive (erst dann ist die Diagnose am Render sinnvoll).
- **Braucht:** `membrane.html:578` den boolean zurückgeben; `record.count` nach `extent > 0.0` aufschlüsseln; die `absent:`-Zeile nach dem `loop()`-Start setzen.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon). Wahrheit: `state/zustand/wartend.φ` (`flyby-chain-omni2`, `flyby-chain-kp-def`, `ephemeris-juice-recon`).
- **Lage:** (gemessen 2026-10-06, River 105) OMNI2 26 Zellen `pending`; kp `def` leer; JUICE-recon absent (Wiedervorlage 2026-11-01).
- **Blockade:** externe Kanäle; kein Polling.
- **Braucht:** `flyby_path2_fill`-Lauf lesen + Addendum fortschreiben; Trigger feuern lassen.

## An mountain

Origin: river-135.

- **`cors_compiler`-Registerzeile:** `tools/harvest/src/bin/cors_compiler.rs` hat keine Registerzeile — der E0061-Fix (Rat Route b) braucht `compiler …` + `at earth`, sonst bleibt die geodätische Position absent (0 honored, kein WGS84-Fabrikat).
- **Membran-Start-Anker (`scale 0`/schwarzes Feld):** hängt an Mountains **span-Direktive** (Presence-Hull-/Star-Grid-span im Datenkontrakt). River hat die per-Kanal-Exposition gebaut und deployt (`pages-deploy 37685135772` success).
- **GM-Anker:** laut future-199 ist der fehlende GM-Anker der Mountain-Teil; die Sonne erscheint, wenn beide stehen (die Render-Ursache liegt nicht im Frontend).
- **`register`-Job des `ci-gate` rot:** `url-order violation within ttl 2592000: usgs_comcat_m45.bin placed after hadisst_sst.bin` — den USGS-Block nach TTL einordnen.
- **`dropped-gate` rot:** 2 neue Drop-Token (`für presence-hull- river schritt span-direktive star-grid-apertur`; `--lpf … start steht`) — Baseline nachziehen oder Drop auflösen.
- **INTERMAGNET-HAPI-Route (neu, river-135):** der gebaute GIC-Familien-Lauf (`field_te_query --stage2 family --driver omni_imf_bz_gsm_nt`) gruppiert 154 Stationen nach Familie, aber die Quelle `https://imag-data.bgs.ac.uk/GIN_V1/hapi/data?id=<st>/best-avail/PT1M/xyzf&format=json` antwortet für jede Station **HTTP 400** → alle Bänder `unmeasured`. Den `id`-Pfad/das Format/das Zeitfenster der Route prüfen.

## An future

Origin: river-135.

- **`span`-Fork (Operator-Wort nötig, ein Wort):** Der `span`-Konsument ist gebaut (Selbstkappen: `anchor` faltet `min(span, extent)` in `extent`, `src/archivar/channels.rs:1245`; Rat + Qwen3.7-Plus konvergieren darauf). **Frage:** Soll `span` semantisch (a) die quellen-eigene Obergrenze bleiben (so gebaut), oder (b) eine echte **Empfänger-Apertur pro Query** sein (FOV/pad, wie IVOA SIA2 — dann wäre `MembraneCtx.pad` der Ort und `SourceConfig` der falsche Träger)? Bei (b) wird ein neuer River-Punkt.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort 2026-09-29; kein Maschinen-Akt.
- **Förder-Bewerbungen Prototype Fund (Frist 30.11.) + EMAP (Frist 06.11.)** — Operator-Wort 2026-10-07: bleiben **LOCK**; Send = Operator-Hand; Voraussetzung = die Membran rendert (freie Presence + Kräfte).

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session (River 135, Fortschreibung):

- `src/archivar/channels.rs`, `src/archivar/main_flow.rs`, `src/archivar/mod.rs`, `src/archivar/relay.rs`, `src/archivar/tests.rs`, `src/archivar/gic.rs`
- `tools/measure/src/bin/cgm_lat_partition.rs`, `tools/measure/src/bin/field_te_query.rs`
- `docs/handover/handover-2026-10-08-river-folge135.md`

## Burn: open 0.0039 · close 0.1674 (deepseek-flash, `session_burn`, gemessen 2026-10-08) — Taucher zusätzlich (grind-flash E0061 0.0712, GIC 0.0514, span-Bau, family_of-Dedup, explore 0.0190) + Rat/UI-Runde · cap 0.35 Grund: Ein-Pass-Atom (E0061 + GIC-Familie + span-Bau + Dedup + Recherche→Rat→UI-Runde, mehrere Taucher) · kein pro/max
