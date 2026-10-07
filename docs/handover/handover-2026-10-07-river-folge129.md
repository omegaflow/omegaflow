<!--
  title: Handover — River-Folge 129 (2026-10-07)
  session: River-Folge 129
  class: handover
  date: 2026-10-07
  sha256: 58d509ccc84a1e0d186c0673a55395daa4a43a00a3dde4ea94bf3388fee815ee
  status: live
-->
# Handover — River-Folge 129 (2026-10-07)

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
„auf jeden fall agnostoisch dein vorgänger hat doch schon eine umfangreiche gibt und bias untersuchung gemacht ist die schon wiedre vergessen?" | 2026-10-07 | Operator (Session, River 129) — Wort für den agnostischen Membran-Edit (Object-Bias raus, Kräfte/Kanäle an)
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-river-folge128.md` §Operator-Wort-Register — gefaltet, nicht kopiert. Verbatim: `state/operator-gespraeche/2026-10-07-river.md`.

## Stimmen-Rolle (gemessen 2026-10-07)

Recherche trägt `voice-deepseek`; strikt lokal nur DeepSeek-flash (`opencode.json`);
Denken/Urteil = UI-Frontier; der Rat = Form/Linse. Die API-Suchschnittstelle
`archive_search --alphaxiv` (alphaXiv MCP `discover_papers`) ist in
`docs/concepts/tools-map.md` nachgetragen.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie; Siegel = Operator-Wort, offen.
- `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md` — see-also auf Archiv-Pfad geheilt (`:7`).
- `docs/paper/gic-causal-driver.md` — NUR-Asset-Fakten §6, §4.7.
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` — Objektophilie-Verdikt (`:113` BODIES→Manifest; `:96` operationaler Test `grep static/`). **Angewandt 2026-10-07 (River 129).**
- `docs/concepts/remove-bias.md` — der Bias-Tilgungsplan (WP0–WP13); WP13-Fixtures gebaut (`47706add5`).

## Offen (aufgeschlüsselt)

### Membran — Kraft-/Kanal-Agnostik statt Objektophilie (Operator-Wort 2026-10-07)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-07, River 127/129) **Gebaut (River 129, Operator-Wort):** `static/membrane.html` trägt **keinen Body-Namen** mehr — `const BODIES` ist entfernt; der Hüllen-Manifest `/membrane_bodies.txt` wird zur Laufzeit gelesen und pages-deploy schreibt ihn via `gen_bodies.sh --write` aus den Stage-Lines (Hülle = einziges Zulassungskriterium, kein Hand-Rang). Der Shader liest die **gemessene Farbe**: `color_index` (`props[id*4+2].z`, kanonisch `static/constants.js:144`) → `/color_lut` (Bindung 3, `parseColorLut`); `ci==0` → weiß (kein Farbwert), LUT fehlt → neutrale Magnituden-Rampe, nie verworfen. `extent` bleibt die Apertur (Finite vs. Punkt — Daten-Eigenschaft, kein Body-Name). Verifiziert: JS-Syntax `node --check` grün; `cargo check` unberührt (nur `static/` + `.github/`).
- **Riss:** per-`force_type`-Exposition (der Rat-Schlüssel) kollidiert mit der `extent`-Apertur-Trennung (Sterne/Bodies mischen die Magnituden) → braucht ein Apertur-Bit im Record oder Per-Force-Level **innerhalb** jeder Apertur. Die Farbe ist kanalisiert, die Exposition noch nicht. → Rat.
- **Blockade:** Riss (Exposition); der Start-Anker (schwarzes Feld/`scale 0`) hängt an Mountains fehlender span-Direktive.
- **Braucht:** Rat-Wort zum Expositions-Riss; dann `force_type`-Per-Kanal-State. Und: nach dem Push den pages-deploy-Lauf messen (Manifest-Datei + gerenderte Kanäle), `ci_manage status`.

### Bias-Audit Archivar/Mathematikerin — der Gift-Rückfall; die Legacy-Untersuchung
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-07, River 127/129) Die Untersuchung existiert: `omegaflow-legacy/docs/concepts/remove-bias.md` (WP0–WP13). **Geheilt (River 129):** `channels.rs:1423` τ=0.0 → `f64::INFINITY` (ω=0 heißt keine Schwingung, τ unendlich — Haus-Konvention, s. `:1411`). **Noch offen — ein unausgeführter Plan:** `src/mathematikerin/media.rs:29-54` Body→Medium-Tabelle, live benutzt in `src/archivar/ephemeris.rs:504` (`medium_params_of(body_name).map_or([0.0; 5], …)`); `src/mathematikerin/shaders.rs:5-15` `PROPAGATION_SPEED`-Konstanten (WP11-Ziel: Absorption im Record). `src/mathematikerin/machines/matrix.rs:786` `23.4392911°` Erd-Schiefe für **jeden** Body. `src/archivar/odp.rs:9 const EARTH` (**offen**). `rinex.rs:5,58` `6378137.0`/`"earth"` (**geheilt River 129:** `ecef_to_geodetic` nimmt `(a,e2)`; Empfänger-Körper aus `src.body`/`frame_body_name`, Ellipsoid aus `BodyProperties.radius_m`+`flattening` — `body_ephemerides` in den Fetch-Worker gereicht). `nexrad.rs:188 EARTH_RADIUS_KM` (**geheilt River 129:** `nexrad_gate_position`/`great_circle_destination` nehmen `host_radius_km`). `clean_tree`: 10 → 5 Treffer. `src/archivar/motion.rs:77` `(0.0,0.0,0.0)`-Nutations-Fallback → Option. Geheilt zuvor: `main_flow.rs:3519/3522` SPK-Gate liest `src.body`; `main_flow.rs:209` `None=>0.0` → verweigern (`f1fc9fda3`); `main_flow.rs:207` clippy `question_mark` (River 129).
- **Blockade:** Riss C (`kernel_extent`-Konflation, `membrane.rs:302-335` liest die fünf Medium-Werte als Kernel-Reichweiten) vor dem Umbenennen entscheiden; Riss A (Herkunft/Provenienz der sechs Medium-Werte) + Riss B (per-field `Option` vs. atomarer Block).
- **Braucht:** die `src`-Bias-Arbeit nach Auftrag `docs/auftrag/auftrag-bias-tilgung.md` (Operator-Wort 2026-10-07; Träger `src` = River; Inventory `state/future/giftkarte-klassifiziert-src-2026-10-07.md`, 162 Funde). **`clean_tree --fail`-Abnahme (gemessen 2026-10-07 via `./target/debug/clean_tree`): 10 → 5 Treffer.** Rest (je Gegenüber gemessen): `media.rs:33` + MEDIA-TABLE → **Rat** (Riss A/B/C; `BodyProperties`-Kernel-Felder sind `f64`, kein `Option` → Wire-Kontrakt-Akt); `odp.rs:9` → die `pioneer_*`-Tools importieren `EARTH` (8 Importstellen) und rufen `downlink_rate`/`station_velocity` (10 Aufrufe) → die Station-→Körper-Bindung gehört als Daten (kernels-TSV) an `dsn_station`, dann Tool-Caller (**Mountain/Mycelium**); `weberin.rs:254,258` → sind die Fetch-Listen der `weberin_*`-Tools (**Mountain/Mycelium**, Daten-Manifest). Eigene offene: `media.rs` (Rat), `matrix.rs:786`, `motion.rs:77`. **Fünf Bias-Klassen — kein eigener Code-Token (gemessen 2026-10-07):** `Observer-as-vantage`/`Station-privilege` kollabieren auf die Körper-Namen-Literale (schon tokenisiert); `Now-default`/`Client-Server`/`Camera-default` haben **keinen** Produktions-Token im Baum (`Instant::now()` = Timeouts, `client`/`server` nur in Relay-Tests) — ein Marker wäre ein leeres Tor. Ehrlich `pending` bis Schritt 3 je Klasse den konkreten Token nennt.

### GIC-Stufe-2 — dB/dt abgeleitet; der Familien-Pool ist ein eigener Bau (Explore folge129)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-07, River 127/128) `intermagnet_dbdt_compiler.rs:5-6,179` leitet `sqrt(dx²+dy²+dz²)` aus derselben `/best-avail/PT1M/xyzf`-Quelle ab; ABK/SOD-dbdt (`phi/sources.φ:2057-2085`) ist **kein** natives INTERMAGNET-Produkt. **Rat (5 Stimmen): A.** Gebaut: `series_dbdt` (`src/archivar/main_flow.rs:5925`); der Loader `field_te_query.rs` löst `intermagnet_dbdt_<station>` ab (`align_xyz` → `series_dbdt`, `load_dbdt_across_sources:1163`). **Explore (folge129, read-only):** der Stufe-2-Familien-Pool ist **kein** Ein-Zeilen-Bau — (i) `--stage2` existiert nirgends; (ii) `cgm_lat` wird nie zur Abfragezeit abgeleitet (nur Test-Fixture `:4880`); (iii) `state/river/gic-family-*.txt` liest niemand (nur Generator `cgm_lat_partition.rs:217`); (iv) `compute_max_t`/`null_matrix` teilen EINEN Draw der Länge `n = member[0].target.len()` über ALLE Member (OOB sonst) → Pool braucht eigenes `align_many` (`:4137`) + `joint_columns` (`:4211`) + eigenen `compute_max_t`-Call; (v) die Treiber-Zuordnung je Member ist ungeklärt (20-Stimmen: Target-Band trägt, Treiber unbandiert). Empfohlen: separate Familien-Invocation (neue Fn nahe `run_pair_matrix:4371`; `--stage2 family`-Arm in `main` ~4804) — Stufe 1 byte-identisch.
- **Blockade:** die Stufe-2-Designstücke (`cgm_lat`-Quelle zur Abfragezeit + Treiber-Mapping) sind ungebaut/ungeklärt → Rat.
- **Braucht:** Entscheidung `cgm_lat`-Quelle + Treiber-Mapping (Rat-Linse); dann der separate Familien-Bau (reuse `load_field_across_sources:1129`, `align_many:4137`, `joint_columns:4211`, `compute_max_t:2824`).

### Receiver-/em-Apertur, ozzy, Membran-Startansicht — CI-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`/`ci-gate` grün am jeweiligen HEAD.
- **Lage:** (gemessen 2026-10-07T18:26Z via `ci_manage status`) `ci-check 37661810074` in_progress; `ci-gate 37661810257` **failure** @17:49 — gemessener Grund (`ci_manage log`): clippy `src/archivar/main_flow.rs:207` (River, in folge129 geheilt: `body_props?.radius_m`) **+** `src/archivar/goes16_mag.rs:37` (Mountain-Carrier; Mountains Fix liegt gestaged im Baum). Gebaut: `series_dbdt` + Loader-Ableitung; ozzy `independence_verdict` (`ozzy.rs:146`); em-Apertur (`shaders.rs:186,211`); Membran per-Klasse-Belichtung.
- **Blockade:** CI-Queue.
- **Braucht:** `ci-check`/`ci-gate`-Ausgang lesen; ozzy/em-Apertur bei grünem Lauf verifizieren.

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

## An mycelium

Origin: river-129.

- **Cross-line touch (Riss):** `.github/workflows/scripts/gen_bodies.sh` (BODIES-const → `--write <file>`; Hand-Rang `rank=(sun earth moon)` entfernt, Workflow-Reihenfolge = Hülle) und `.github/workflows/pages-deploy.yml` (Step: `--check static/membrane.html` → `--write _site/membrane_bodies.txt`). Das ist eigentlich dein CI-Recht; das Operator-Wort „agnostisch" brauchte die Manifest-Daten an der Membran — der Body-Name-freie Weg ist nur so zu schließen. Bitte prüfen/falten; die `--check`-Drift-Gate entfällt, weil der Manifest jetzt die einzige Quelle ist.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort 2026-09-29; kein Maschinen-Akt.
- **Förder-Bewerbungen Prototype Fund (Frist 30.11.) + EMAP (Frist 06.11.)** — Operator-Wort 2026-10-07: bleiben **LOCK**; Send = Operator-Hand; Voraussetzung = die Membran rendert (freie Presence + Kräfte).

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `src/archivar/main_flow.rs` (clippy `question_mark`; `body_ephemerides` in den Rinex-Fetch-Worker)
- `src/archivar/channels.rs` (τ=∞ statt 0.0, Bias)
- `src/archivar/nexrad.rs` (EARTH_RADIUS_KM → `host_radius_km`-Parameter)
- `src/archivar/rinex.rs` (Empfänger-Körper aus Daten, Ellipsoid aus `BodyProperties` — clean-tree 10→5)
- `static/membrane.html` (agnostisch: Manifest-Fetch, gemessene Farbe via `/color_lut`)
- `.github/workflows/scripts/gen_bodies.sh` + `.github/workflows/pages-deploy.yml` (Manifest-Schreibpfad)
- `docs/handover/handover-2026-10-07-river-folge129.md` (neu)
- `docs/handover/archiv/handover-2026-10-07-river-folge128.md` (Move)

## Burn: open 0.0000 · close 0.2279 (line $0.1931 + explore $0.0348, deepseek-flash, `session_burn`, gemessen 2026-10-07) · cap 0.25 Grund: ein Mehr-Schritt-Atom — Operator trug in laufender Sitzung die agnostische Membran und danach die src-Bias-Abnahme („weiter") nach, kein Ein-Schritt-Auftrag · kein pro/max · Dispatches: `explore` (1, read-only: GIC-Familien-Pool-Pfad)
