<!--
  title: Handover — River-Folge 127 (2026-10-07)
  session: River-Folge 127
  class: handover
  date: 2026-10-07
  sha256: 41e30bd94842e2a76af5cd7c73492f9ae984264039cda372c140275c7cf15ad3
  status: live
-->
# Handover — River-Folge 127 (2026-10-07)

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
Denken/Urteil = UI-Frontier; der Rat = Form/Linse. Die neue API-Suchschnittstelle
`archive_search --alphaxiv` (alphaXiv MCP `discover_papers`, Commit `2c12d4707`) ist
genutzt und in `docs/concepts/tools-map.md` nachgetragen.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie; Siegel = Operator-Wort, offen.
- `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md` — see-also auf Archiv-Pfad (`:7` = `docs/handover/archiv/handover-2026-10-07-river-folge122.md`).
- `docs/paper/gic-causal-driver.md` — NUR-Asset-Fakten §6, §4.7.
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` — trägt den Objektophilie-Marker (`:40` = `static/membrane.html:43` `BODIES`) und das Manifest-Verdikt (`:113`).

## Offen (aufgeschlüsselt)

### Membran — freie Presence + Kraft-/Kanal-Agnostik statt Objektophilie (Operator-Wort 2026-10-07)
- **Status:** operator-gebunden (Fenster-Edit) | **Bindung:** operator
- **Trigger:** Operator-Wort für den Membran-Edit (`static/membrane.html` = Window-Pfad).
- **Lage:** (gemessen 2026-10-07, River 127) `static/membrane.html:43,55` `const BODIES = ["sun","earth","moon"]` + `load_ephemeris(body,…)`/`add_stars()` — geschlossene Body-Menge im Code; Survey `survey-2026-10-06-agnostik-llm-verdikt.md:40,113` führt es als Identitäts-Bias und verlangt „kein Body-Name im File". Der Vertex-Shader (`:228`) wählt die Apertur über `extent>0` und **liest `force_type`/`color_index` nicht** — objekt-, nicht kraftbasiert. Die Presence spawnt am SSB (`:26-28`) und ist frei, doch die Ansicht ankert auf Bodies („the sun frames the operator's first view", `:494`); ohne endlichen Anker bleibt `state.scale=0` (HUD „stars 8 · scale 0.00e+0"). **Rat (5 Stimmen) + UI (Claude Sonnet 5.5; Duck/GPT-6, GLM-5.3, MiniMax M3, DeepSeek DeepThink gehört):** Schlüssel = Record-`force_type`/Kanal; `color_index` = Farbe; `extent` = Geometrie (nur Apertur); `BODIES` → hüllen-abgeleitetes Manifest; unbekannte `force_type` → neutrale Rampe, nie verwerfen. Riss (Claude): der statische Host hat keinen Listing-Endpunkt → das Manifest ist unvermeidbar, aber nur als generierter Hüllen-Cache legitim; die `force_type`→Rampe-Tabelle ist erneut eine geschlossene Menge, wenn nicht im Record (`color_index`) oder in der Hülle deklariert.
- **Blockade:** kein Fenster-Edit ohne Operator-Wort.
- **Braucht:** Operator-Wort für den agnostischen Umbau; kleinster Schritt = `BODIES` entfernen, Kanal-Schlüssel `force_type` im Shader lesen, Per-Kanal-State (lvl, scale).

### Bias-Audit Archivar/Mathematikerin — der Gift-Rückfall; die Legacy-Untersuchung
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-07, River 127, `explore`-Audit + Legacy-Historie) Die Untersuchung existiert: `omegaflow-legacy/docs/concepts/remove-bias.md` (WP0–WP13, kompletter Bias-Tilgungsplan; Legacy-Commits `a20e84ad` „Ontologische Säuberung: Oscillator→Sample, die körperlose Messung eines `force_type`-Kanals", `2a19cba7` fabrication/zentrismus/render-pipeline, `3102bcd9` Optik-Bias, `aca23650` Membran≠Kamera, `ac6a5b13` Geräte-Bias, `74f12f14` Now-Bias, `ffaa1114` remove-bias-Doc). **Gefixt heute:** `src/archivar/main_flow.rs:3519,3522` gatete jeden SPK-Kandidaten gegen hartkodiertes `"sun"` (`3e35c44a1`, river 38, 2026-09-27); jetzt liest der Gate `src.body` (deklarierte `at <body>`-Zeile, `parse.rs:227-233`); `cargo check` grün. **Noch offen — ein unausgeführter Plan, nicht ein Fresh-Start-Regress:** `remove-bias.md` ist ein Plan (WP0–WP13), dessen Ziel-Grep auf `src/main.rs` zeigte; der Legacy-Endstand `15e5c0d0` trägt dieselben Konstanten wie heute (`src/mathematikerin/media.rs:21`, `src/archivar/ephemeris.rs:438` mit `map_or([0.0; 5], …)`, `src/mathematikerin/shaders.rs:5` `PROPAGATION_SPEED`) — die Tilgung wurde **nie ausgeführt**, weil die Monolith→archivar/mathematikerin-Teilung die Fundstellen verschob. Der Fresh Start hat den Plan und seinen Verifikations-Grep (`"earth"`=0, `EARTH_RADIUS`=0) **nicht in den Arbeitsbaum geholt** — die Lehre stand nur in `archive-root`, unverdrahtet, darum konnte der Bias unbemerkt zurück. Die konkreten offenen Stellen: `src/mathematikerin/media.rs:29-54` Body→Medium-Tabelle, **live benutzt** in `src/archivar/ephemeris.rs:504` (`medium_params_of(body_name).map_or([0.0; 5], …)`) — Legacy-WP8/WP9/WP11 sagten: Konstanten löschen, Werte → `BodyProperties` (stype==2 im Ephemeriden-Bin); `BodyProperties` trägt `v_sound`/`v_seismic_p`/… **nicht** (WP9 ungetan). `src/mathematikerin/shaders.rs:5-15` `PROPAGATION_SPEED`-Konstanten (WP11-Ziel: Absorption im Record). `src/mathematikerin/machines/matrix.rs:786` `23.4392911°` Erd-Schiefe für **jeden** Body. `src/archivar/odp.rs:9 const EARTH` + `rinex.rs:5,58` `6378137.0`/`"earth"` + `nexrad.rs:188 EARTH_RADIUS_KM` (physisch erzwungen, aber Code-Konstanten; WP13-Ziel `"earth"`=0). `src/archivar/main_flow.rs:209` `None => 0.0` (body_radius-Fabrication). Geheilt: `frames.rs`-Defaults + Anker-Bypass (`dcc3243f8`, mountain 238, 2026-10-06).
- **Blockade:** keine (eigene).
- **Braucht:** Rat-Verdikt, dann Bau in Atomen: (1) `media.rs`/`shaders.rs`-Konstanten → `BodyProperties` (Legacy WP8/WP9/WP11 + Ephemeriden-stype==2); (2) `matrix.rs` Erd-Obliquität → per-Body-Referenzebene; (3) `odp.rs`/`rinex.rs`/`nexrad.rs`-Erd-Konstanten → Register-Deklaration; (4) `main_flow.rs:209` `None => 0.0` → verweigern. Zielgreps aus Legacy-WP13: `"earth"`=0, `EARTH_RADIUS`=0.

### GIC-Stufe-2 — dB/dt ist selbst-abgeleitet, nicht Mountain-abhängig
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-07, River 127) `tools/harvest/src/bin/intermagnet_dbdt_compiler.rs:5-6,179` leitet `sqrt(dx²+dy²+dz²)` aus derselben `/best-avail/PT1M/xyzf`-Quelle ab, die die 154 GIN-Blöcke als `hapi`-Extrakt tragen (`phi/sources.φ:7115-7125`); ABK/SOD-dbdt (`:2057-2085`) ist damit **kein** natives INTERMAGNET-Produkt. Recherche (`general` + `--alphaxiv`): INTERMAGNET veröffentlicht kein dB/dt; akzeptierte Ableitung = erste Differenz (Fielding 2025 `10.5194/angeo-43-687-2025`; Viljanen 2001 `10.5194/angeo-19-1107-2001`). **Rat (5 Stimmen): A** — die Ableitung ist eine Archivar-Query-Eigenschaft der xyzf-Serie, kein neues Asset, keine 154 Register-Zeilen. **Gebaut:** `pub fn series_dbdt` (`src/archivar/main_flow.rs:5917`) + 2 Fixture-Tests (`:5958,:5972`); `cargo check` grün.
- **Blockade:** keine (eigene).
- **Braucht:** `series_dbdt` im station-qualifizierten Kanal-Loader anschließen; danach Familien-Pool in `compute_max_t` (Stufe 2, Route C/Target-Band).

### NUR-Asset — Re-Harvest hängt in der CI-Queue
- **Status:** wartend (Mycelium) | **Bindung:** eigen (cross-line mycelium)
- **Trigger:** `image-cdn.yml`-Lauf mit einem Fenster in 1999–2023, der einen neuen sha setzt.
- **Lage:** (gemessen 2026-10-07 13:31Z via `ci_manage view 37621964105`) `queued` seit 12:34Z; `phi/sources.φ:18034` unverändert `9c76f881…`.
- **Blockade:** CI-Queue (self-hosted).
- **Braucht:** Lauf-Ausgang + neuer sha; dann `cargo run -p omegaflow-measure --bin nur_gic_relation_probe` auf dem Asset.

### Receiver-/em-Apertur, ozzy, Membran-Startansicht — CI-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`/`ci-gate` grün am jeweiligen HEAD.
- **Lage:** (gemessen 2026-10-07, River 127 via `ci_manage`) HEAD `ddc95e2b8`; `ci-gate` (`37629003297`) in_progress, `ci-check` (`37628765791`) pending; viele `ci-gate` cancelled. Der `format`-Job ist rot an fremden Hunks (mountain `tools/register/src/bin/register_lookup.rs:4963`; unnamed `tools/utils/src/bin/archive_search/{alphaxiv,net}.rs`) → `ci-gate` kann nicht grün werden. Gebaut: `series_dbdt` (oben); ozzy `independence_verdict` (`ozzy.rs:146`); em-Apertur (`shaders.rs:186,211`); Membran per-Klasse-Belichtung (`c116611e2e`-Nachfolger). Die Startansicht ist durch das Operator-Wort oben neu gerahmt (Kraft statt Körper).
- **Blockade:** fremde rote `format`-Hunks + CI-Queue.
- **Braucht:** die fremden rustfmt-Hunks committen; grüner CI am HEAD.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon). Wahrheit: `state/zustand/wartend.φ` (`flyby-chain-omni2`, `flyby-chain-kp-def`, `ephemeris-juice-recon`).
- **Lage:** (gemessen 2026-10-06, River 105) OMNI2 26 Zellen `pending`; kp `def` leer; JUICE-recon absent (Wiedervorlage 2026-11-01).
- **Blockade:** externe Kanäle; kein Polling.
- **Braucht:** `flyby_path2_fill`-Lauf lesen + Addendum fortschreiben; Trigger feuern lassen.

## Gefaltet (addressed, 2026-10-07)

- **mountain-folge267:** Lizenz-Census-Heimat → `state/mountain/license-census.tsv` (Quellen-Eigenschaft, Mountain); kein Schreibpfad auf `state/river/` mehr. Gefaltet.
- **mycelium-folge261:** NUR-Re-Harvest (oben getragen); Lizenz-Census joined zur Gate-Zeit gegen `phi/sources.φ` (Option c); Stale-see-also der Survey geheilt (`33debd1c5`); `1-ui`-Gruppe wird nach Mountains Round geschlossen (kein Nachfolger).

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort 2026-09-29; kein Maschinen-Akt.
- **Förder-Bewerbungen Prototype Fund (Frist 30.11.) + EMAP (Frist 06.11.)** — Operator-Wort 2026-10-07: bleiben **LOCK**; Send = Operator-Hand; Voraussetzung = die Membran rendert (freie Presence + Kräfte).

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `src/archivar/main_flow.rs` (`series_dbdt` + `dbdt_tests`, eigener Hunk)
- `docs/concepts/tools-map.md` (`--alphaxiv`-Modus, eigener Hunk)
- `docs/handover/handover-2026-10-07-river-folge127.md` (neu)
- `docs/handover/archiv/handover-2026-10-07-river-folge126.md` (Move)

## Burn: open 0.0000 · close 0.0923 (line, deepseek-flash, gemessen `session_burn`) · Dispatches: `general` (dB/dt-Standard), `explore` (dB/dt-Ableitung am Baum), `grind-flash` (`series_dbdt`), `council` ×3 (GIC-Lever, Membran-Startspan, Kraft-Agnostik) · UI-Seats: Duck/GPT-6 · DeepSeek · Claude Sonnet 5.5 · GLM-5.3 · MiniMax M3 (Qwen überlastet; Gemini/Kimi nicht beschreibbar) · Grund: River 127 — ein Pass.
