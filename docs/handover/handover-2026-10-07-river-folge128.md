<!--
  title: Handover — River-Folge 128 (2026-10-07)
  session: River-Folge 128
  class: handover
  date: 2026-10-07
  sha256: 68836b083caf316f335175454ccf8fcb12c0059c6dd6e82c913ec0f04023156e
  status: live
-->
# Handover — River-Folge 128 (2026-10-07)

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
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-river-folge126.md` §Operator-Wort-Register — gefaltet, nicht kopiert. Verbatim: `state/operator-gespraeche/2026-10-07-river.md`.

## Stimmen-Rolle (gemessen 2026-10-07)

Recherche trägt `voice-deepseek`; strikt lokal nur DeepSeek-flash (`opencode.json`);
Denken/Urteil = UI-Frontier; der Rat = Form/Linse. Die API-Suchschnittstelle
`archive_search --alphaxiv` (alphaXiv MCP `discover_papers`) ist in
`docs/concepts/tools-map.md` nachgetragen.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie; Siegel = Operator-Wort, offen.
- `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md` — see-also auf Archiv-Pfad (`:7` = `docs/handover/archiv/handover-2026-10-07-river-folge122.md`, `33debd1c5`).
- `docs/paper/gic-causal-driver.md` — NUR-Asset-Fakten §6, §4.7.
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` — Objektophilie-Marker (`:40` = `static/membrane.html:43` `BODIES`) und Manifest-Verdikt (`:113`).
- `docs/concepts/remove-bias.md` — der Bias-Tilgungsplan (WP0–WP13), live gestellt 2026-10-07; die WP13-Zielgreps sind als `fabrication`-Fixtures gebaut (`src/gate/commit_gate_vocab.json`, Commit `47706add5`).

## Offen (aufgeschlüsselt)

### Membran — freie Presence + Kraft-/Kanal-Agnostik statt Objektophilie (Operator-Wort 2026-10-07)
- **Status:** operator-gebunden (Fenster-Edit) | **Bindung:** eigen
- **Trigger:** Operator-Wort für den Membran-Edit (`static/membrane.html` = Window-Pfad).
- **Lage:** (gemessen 2026-10-07, River 127) `static/membrane.html:43,55` `const BODIES = ["sun","earth","moon"]` + `load_ephemeris(body,…)`/`add_stars()` — geschlossene Body-Menge im Code; Survey `survey-2026-10-06-agnostik-llm-verdikt.md:40,113` führt es als Identitäts-Bias und verlangt „kein Body-Name im File". Der Vertex-Shader (`:228`) wählt die Apertur über `extent>0` und **liest `force_type`/`color_index` nicht** — objekt-, nicht kraftbasiert. Die Presence spawnt am SSB (`:26-28`) und ist frei, doch die Ansicht ankert auf Bodies („the sun frames the operator's first view", `:494`); ohne endlichen Anker bleibt `state.scale=0` (HUD „stars 8 · scale 0.00e+0"). **Rat (5 Stimmen) + UI (Claude Sonnet 5.5; Duck/GPT-6, GLM-5.3, MiniMax M3, DeepSeek DeepThink gehört):** Schlüssel = Record-`force_type`/Kanal; `color_index` = Farbe; `extent` = Geometrie (nur Apertur); `BODIES` → hüllen-abgeleitetes Manifest; unbekannte `force_type` → neutrale Rampe, nie verwerfen. Riss (Claude): der statische Host hat keinen Listing-Endpunkt → das Manifest ist unvermeidbar, aber nur als generierter Hüllen-Cache legitim; die `force_type`→Rampe-Tabelle ist erneut eine geschlossene Menge, wenn nicht im Record (`color_index`) oder in der Hülle deklariert.
- **Blockade:** kein Fenster-Edit ohne Operator-Wort.
- **Braucht:** Operator-Wort für den agnostischen Membran-Edit; kleinster Schritt = `BODIES` entfernen, Kanal-Schlüssel `force_type` im Shader lesen, Per-Kanal-State (lvl, scale). Dazu die Sichtbarmachung aus future-193 (`register_lookup --addressed river`): `static/membrane.html:578` verschluckt das boolean von `load_ephemeris`; `record.count` nach `extent > 0.0` aufschlüsseln; die `absent:`-Statuszeile (`:594-599`) wird vom Frame-`loop()` (`:544`) überschrieben; die Rust-`eprintln!`s (`membrane.rs:407/415`) erreichen die Browser-Konsole nicht — die leere Anker-Menge soll sichtbar werden statt als ehrliches Schwarz zu lesen.

### Bias-Audit Archivar/Mathematikerin — der Gift-Rückfall; die Legacy-Untersuchung
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-07, River 127, `explore`-Audit + Legacy-Historie) Die Untersuchung existiert: `omegaflow-legacy/docs/concepts/remove-bias.md` (WP0–WP13, kompletter Bias-Tilgungsplan; Legacy-Commits `a20e84ad`, `2a19cba7`, `3102bcd9`, `aca23650`, `ac6a5b13`, `74f12f14`, `ffaa1114`). **Gefixt (River 127):** `src/archivar/main_flow.rs:3519,3522` gatete jeden SPK-Kandidaten gegen hartkodiertes `"sun"` (`3e35c44a1`, river 38); jetzt liest der Gate `src.body` (deklarierte `at <body>`-Zeile, `parse.rs:227-233`). **Noch offen — ein unausgeführter Plan:** `src/mathematikerin/media.rs:29-54` Body→Medium-Tabelle, live benutzt in `src/archivar/ephemeris.rs:504` (`medium_params_of(body_name).map_or([0.0; 5], …)`); `BodyProperties` trägt `v_sound`/`v_seismic_p`/… nicht (WP9 ungetan). `src/mathematikerin/shaders.rs:5-15` `PROPAGATION_SPEED`-Konstanten (WP11-Ziel: Absorption im Record). `src/mathematikerin/machines/matrix.rs:786` `23.4392911°` Erd-Schiefe für **jeden** Body. `src/archivar/odp.rs:9 const EARTH` + `rinex.rs:5,58` `6378137.0`/`"earth"` + `nexrad.rs:188 EARTH_RADIUS_KM`. `src/archivar/main_flow.rs:209` `None => 0.0` (body_radius-Fabrication). Geheilt: `frames.rs`-Defaults + Anker-Bypass (`dcc3243f8`, mountain 238).
- **Blockade:** keine (eigene).
- **Braucht:** die vier Atome, jetzt mit dem UI-Fold (`state/river/2026-10-07_medium-bodyproperties-stimmen.md`, gelesen 2026-10-07): **4 Seats geantwortet** (DeepSeek · Claude · Qwen · Kimi K3 via `tryingopen`, alle auf (a)=Rat konvergent), drei Risse — A Wert-Herkunft (`stype==2` nur aus echten Quellen je Wert, sonst `pending`; PREM radiusabhängig), B None-Kodierung des 6×f64-Blocks (Kimi: **per-field `Option`**, nicht atomar; neue `stype`-ID mit Einheiten, alte `2` nur Legacy-Alias), C `kernel_extent` (`src/archivar/membrane.rs:302-335`) benutzt die fünf Medium-Werte als Kernel-Reichweiten (Kimi: Konflation wie gebaut, legitim nur als **eine benannte Herleitung** `kernel_extent(m,k,dt,pol)`) — Atom (1) ist darum kein reines Umbenennen, der Kernel/Medium-Schnitt braucht einen eigenen Bau. Reihenfolge: (1) `media.rs` löschen, `BodyProperties` aus `stype==2` speisen, `ephemeris.rs:504`/`horizons_compiler.rs:159` aus den Props, `map_or([0.0;5])` → verweigern; Riss C vor dem Umbenennen entscheiden; (2) `matrix.rs` Erd-Obliquität → per-Body-Referenzebene; (3) `odp.rs`/`rinex.rs`/`nexrad.rs`-Erd-Konstanten → Register-Deklaration; (4) `main_flow.rs:209` `None => 0.0` → verweigern. Die WP13-Zielgreps sind gebaut (`47706add5`: `"earth"`, `EARTH_RADIUS`, `6378137.0`, `6378136.6`, `111319.0`, `0.40909`, `280.460`, `360.985`, `DEMO_KEY`, `V_SOUND_288`, `V_P_GRANITE`, `V_S_GRANITE`, `D_AIR`, `ALPHA_AIR`, `force_constants`); jede Neueinführung greift das `fabrication`-Gate.

### GIC-Stufe-2 — dB/dt selbst-abgeleitet; der Loader liest jetzt ab
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-07, River 127/128) `intermagnet_dbdt_compiler.rs:5-6,179` leitet `sqrt(dx²+dy²+dz²)` aus derselben `/best-avail/PT1M/xyzf`-Quelle ab, die die 154 GIN-Blöcke als `hapi`-Extrakt tragen (`phi/sources.φ:7115-7125`); ABK/SOD-dbdt (`:2057-2085`) ist damit **kein** natives INTERMAGNET-Produkt. Recherche (`general` + `--alphaxiv`): INTERMAGNET veröffentlicht kein dB/dt; akzeptierte Ableitung = erste Differenz (Fielding 2025 `10.5194/angeo-43-687-2025`; Viljanen 2001 `10.5194/angeo-19-1107-2001`). **Rat (5 Stimmen): A** — die Ableitung ist eine Archivar-Query-Eigenschaft der xyzf-Serie, kein neues Asset, keine 154 Register-Zeilen. **Gebaut (River 127):** `pub fn series_dbdt` (`src/archivar/main_flow.rs:5925`) + 2 Fixture-Tests. **Gebaut (River 128):** der station-qualifizierte Kanal-Loader in `tools/measure/src/bin/field_te_query.rs` löst `intermagnet_dbdt_<station>` jetzt ab: `align_xyz` (gemeinsame Epochen der drei `intermagnet_xyz_{x,y,z}_nt_<station>`-Serien) → `series_dbdt` (60 s); Hook in `load_field_across_sources`, wenn kein nativer Quellen-Block das Kanal-Namen trägt. `cargo build -p omegaflow-measure --bin field_te_query` grün.
- **Blockade:** keine (eigene).
- **Braucht:** Familien-Pool in `compute_max_t` (`field_te_query.rs:2784`, Stufe 2, Route C/Target-Band): je Familie ein studentisiertes WY-max-t über den vollen Target-Band-Pool, gemeinsame Surrogat-Draws, Stufe 1 byte-identisch (Form aus folge123/124/125); Kanal-Namen `intermagnet_dbdt_<station>` öffnen den Bestand jetzt ab.

### Receiver-/em-Apertur, ozzy, Membran-Startansicht — CI-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`/`ci-gate` grün am jeweiligen HEAD.
- **Lage:** (gemessen 2026-10-07T16:11Z via `ci_manage`) `ci-gate 37649548173` **success** @ `7e5853864` — der rustfmt-Fix ist committet, der `format`-Diff ist geheilt. `ci-check 37649548180` pending; `tools-build 37649548250` in_progress. Gebaut: `series_dbdt` + Loader-Ableitung (oben); ozzy `independence_verdict` (`ozzy.rs:146`); em-Apertur (`shaders.rs:186,211`); Membran per-Klasse-Belichtung (`c116611e2e`-Nachfolger). Die Startansicht ist durch das Operator-Wort oben neu gerahmt (Kraft statt Körper).
- **Blockade:** CI-Queue.
- **Braucht:** `ci-check`-Ausgang; ozzy/em-Apertur bei grünem Lauf verifizieren.

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

## Gefaltet (addressed, 2026-10-07)

- **future-folge193:** Membran-Sichtbarmachung (der leere Anker soll sichtbar werden statt als ehrliches Schwarz zu lesen) — als Braucht in den Membran-Punkt gefaltet; die Quelle heilt Mountain (`## An mountain`).
- **mountain-folge268:** Lizenz-Census-Heimat → `state/mountain/license-census.tsv` (Quellen-Eigenschaft, Mountain); `state/river/` trägt keine Kopie mehr (`glob state/river/*` = leer), `license_census.rs:7` zeigt auf den neuen Pfad. Kein Schreibpfad auf `state/river/` mehr. Gefaltet.
- **mycelium-folge262:** NUR-Re-Harvest (oben getragen); Survey-see-also `33debd1c5` geheilt (`:7` zeigt auf `archiv/…folge122`); `1-ui`-Gruppe nach Mountains Round schließen, kein Nachfolger. Gefaltet.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort 2026-09-29; kein Maschinen-Akt.
- **Förder-Bewerbungen Prototype Fund (Frist 30.11.) + EMAP (Frist 06.11.)** — Operator-Wort 2026-10-07: bleiben **LOCK**; Send = Operator-Hand; Voraussetzung = die Membran rendert (freie Presence + Kräfte).

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `tools/utils/src/bin/archive_search/alphaxiv.rs` + `tools/utils/src/bin/archive_search/net.rs` (rustfmt, `7e5853864`)
- `tools/measure/src/bin/field_te_query.rs` (`series_dbdt`-Loader-Ableitung, eigener Hunk)
- `docs/handover/handover-2026-10-07-river-folge128.md` (neu)
- `docs/handover/archiv/handover-2026-10-07-river-folge127.md` (Move)

## Burn: open 0.0000 · close 0.1268 (line, deepseek-flash, `session_burn`-Sessionzeile „River-Übergabe in einem Pass abarbeiten", gemessen 2026-10-07) · kein pro/max · Dispatches: keine (eigener Bau: rustfmt-Fix `7e5853864` + `series_dbdt`-Loader + WP13-Fixtures `47706add5` + UI-Fold Medium/`kernel_extent`)
