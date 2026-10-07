<!--
  title: Handover — River-Folge 130 (2026-10-07)
  session: River-Folge 130
  class: handover
  date: 2026-10-07
  sha256: d034dacd7e00e7dcbb922db1445934ab2512c61e7ecf6a2e06e93293435b4d62
  status: live
-->
# Handover — River-Folge 130 (2026-10-07)

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

### Membran — Kraft-/Kanal-Agnostik: Rat-Wort **B** (Exposition pro `(force_type, aperture)`)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-07, River 127/129/130) **Gebaut (River 129, Operator-Wort):** `static/membrane.html` trägt keinen Body-Namen mehr — `const BODIES` entfernt; der Hüllen-Manifest `/membrane_bodies.txt` wird zur Laufzeit gelesen, pages-deploy schreibt ihn via `gen_bodies.sh --write` aus den Stage-Lines. Der Shader liest die gemessene Farbe (`color_index` → `/color_lut`); `ci==0` → weiß, LUT fehlt → neutrale Rampe. `extent` bleibt die Apertur. **Rat (5 Stimmen) + UI (Qwen) 2026-10-07: B** — Expositions-State pro `(force_type, aperture)`, **kein** Wire-Bit; A ist verweigert (die Anzeige wäre an der Stelle der Sache). Rat-Nebenachse: die Apertur bleibt auf `extent>0` (optische Größe, null-echt bei 0); Träger = `force_type`, Wert-innerhalb = `color_index`; fehlender Schlüssel → kein Level (0 honored), kein Default.
- **Blockade:** Start-Anker (schwarzes Feld/`scale 0`) hängt an Mountains fehlender span-Direktive; die Runde sendete nur Qwen (Claude/z.ai ohne Send-Pfad, Duck Tageslimit, Open-Weight-Seats unter Mountain-Lock).
- **Braucht:** Bau des per-Kanal-Expositions-State (`static/membrane.html` + `src/archivar/shaders.rs`, `lvl[force_type][aperture]`, α = 1−exp(−1/8)); dann pages-deploy-Lauf messen (`ci_manage status`).

### GIC-Stufe-2 — dB/dt abgeleitet; der Familien-Pool ist ein eigener Bau
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-07, River 127/128) `intermagnet_dbdt_compiler.rs:5-6,179` leitet `sqrt(dx²+dy²+dz²)` aus derselben `/best-avail/PT1M/xyzf`-Quelle ab; ABK/SOD-dbdt (`phi/sources.φ:2057-2085`) ist kein natives INTERMAGNET-Produkt. **Rat (5 Stimmen): A.** Gebaut: `series_dbdt` (`src/archivar/main_flow.rs:5925`); der Loader `field_te_query.rs` löst `intermagnet_dbdt_<station>` ab (`align_xyz` → `series_dbdt`, `load_dbdt_across_sources:1163`). **Explore (folge129, read-only):** der Stufe-2-Familien-Pool ist kein Ein-Zeilen-Bau — (i) `--stage2` existiert nirgends; (ii) `cgm_lat` wird nie zur Abfragezeit abgeleitet (nur Test-Fixture `:4880`); (iii) `state/river/gic-family-*.txt` liest niemand (nur Generator `cgm_lat_partition.rs:217`); (iv) `compute_max_t`/`null_matrix` teilen EINEN Draw der Länge `n = member[0].target.len()` über ALLE Member (OOB sonst) → Pool braucht eigenes `align_many` (`:4137`) + `joint_columns` (`:4211`) + eigenen `compute_max_t`-Call; (v) die Treiber-Zuordnung je Member ist ungeklärt (20-Stimmen: Target-Band trägt, Treiber unbandiert). Empfohlen: separate Familien-Invocation (neue Fn nahe `run_pair_matrix:4371`; `--stage2 family`-Arm in `main` ~4804) — Stufe 1 byte-identisch.
- **Blockade:** die Stufe-2-Designstücke (`cgm_lat`-Quelle zur Abfragezeit + Treiber-Mapping) sind ungebaut/ungeklärt → Rat.
- **Braucht:** Entscheidung `cgm_lat`-Quelle + Treiber-Mapping (Rat-Linse; laut Operator-Wort 2026-10-07 vor alle UI-Seats); dann der separate Familien-Bau (reuse `load_field_across_sources:1129`, `align_many:4137`, `joint_columns:4211`, `compute_max_t:2824`).

### Receiver-/em-Apertur, ozzy, Membran — CI-Verifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`/`ci-gate` grün am jeweiligen HEAD.
- **Lage:** (gemessen 2026-10-07T19:46Z via `ci_manage log 37676860062`) `ci-gate` rot an River-eigenem Code: clippy `neg_cmp_op_on_partial_ord` `src/archivar/rinex.rs:63` + E0382 moved value `src/mathematikerin/media.rs:85` (Test nutzt `m` nach `m.wire()`). **Geheilt (River 130):** `media.rs:10` `wire(&self)`; `rinex.rs:60-76` positive `a > 0.0`-Verzweigung. `cargo check` grün, `cargo fmt` grün. Gebaut: `series_dbdt` + Loader-Ableitung; ozzy `independence_verdict` (`ozzy.rs:146`); em-Apertur (`shaders.rs:186,211`); Membran per-Klasse-Belichtung.
- **Blockade:** CI-Queue.
- **Braucht:** den nächsten `ci-gate`-Lauf lesen (clippy/compile sollten grün sein); ozzy/em-Apertur bei grünem Lauf verifizieren.

### Bias-Audit Archivar/Mathematikerin — src-Rest und der Gate-Rückfall
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-07, River 127/129/130) Untersuchung `omegaflow-legacy/docs/concepts/remove-bias.md` (WP0–WP13). **Geheilt:** `channels.rs:1423` τ=∞; `nexrad.rs`/`rinex.rs`/`odp.rs`/`media.rs` (10→2). **Gate-Wall gefallen (Mountain 269, `170a4da44`):** der Fabrication-Fixture-Scan nimmt `#[cfg(test)]` aus → `src/weberin.rs` ist committbar. **`clean_tree --fail` = 2 (gemessen 2026-10-07 via `./target/debug/clean_tree`):** `src/weberin.rs:254/258` — die Produktions-Consts `INPOP_LINE_BODIES`/`EPM_LINE_BODIES` tragen Körper-Namen-Literale. Offen weiter: `matrix.rs:786` (Erd-Schiefe per-Körper), `motion.rs:77` (Nutation → Option), `shaders.rs:5-15` (WP11), **Riss A** (Medium-Werte-Provenienz `pending`).
- **Blockade:** Riss C (`kernel_extent`-Konflation, `membrane.rs:302-335`) vor dem Umbenennen entscheiden; Riss A (Provenienz der sechs Medium-Werte); Riss B (per-field `Option` vs. atomarer Block); `weberin`-Produktions-`"sun"`-Rahmen (Rat).
- **Braucht:** die `weberin`-Daten-Fassung (`inpop/epm_line_bodies()` + 4 Tool-Konsumenten) auf dem neuen, committbaren Gate; `clean_tree` 2→0; danach `matrix.rs:786`, `motion.rs:77`, `shaders.rs:5-15`. Auftrag `docs/auftrag/auftrag-bias-tilgung.md` (Operator-Wort 2026-10-07); Inventory `state/future/giftkarte-klassifiziert-src-2026-10-07.md` (162 Funde).

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

Origin: river-130.

- **Cross-line touch (Riss):** `.github/workflows/scripts/gen_bodies.sh` (BODIES-const → `--write <file>`) und `.github/workflows/pages-deploy.yml` (Step `--check static/membrane.html` → `--write _site/membrane_bodies.txt`) — CI-Recht Mycelium. Bitte prüfen/falten; die `--check`-Drift-Gate entfällt, weil der Manifest jetzt die einzige Quelle ist.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort 2026-09-29; kein Maschinen-Akt.
- **Förder-Bewerbungen Prototype Fund (Frist 30.11.) + EMAP (Frist 06.11.)** — Operator-Wort 2026-10-07: bleiben **LOCK**; Send = Operator-Hand; Voraussetzung = die Membran rendert (freie Presence + Kräfte).

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `src/archivar/rinex.rs` (clippy `neg_cmp_op_on_partial_ord` → positive Verzweigung)
- `src/mathematikerin/media.rs` (`wire(&self)` statt `wire(self)` — E0382 im Test)
- `AGENTS.md` (Operator-Wort 2026-10-07: „Ratsfragen vor ALLE UI-Chats")
- `docs/handover/handover-2026-10-07-river-folge130.md` (neu)
- `docs/handover/archiv/handover-2026-10-07-river-folge129.md` (Move)

## Burn: open 0.0000 · close 0.0843 (line $0.0843, deepseek-flash, `session_burn`, gemessen 2026-10-07) · cap 0.35 Grund: ein Mehr-Schritt-Atom — Operator trug in laufender Sitzung „Ratsfragen auch vor ALLE UI-Chats" nach; kein pro/max · Dispatches: `council` (1: Rat Membran-Expositions-Riss) + UI-Runde (Duck/Qwen/Claude/z.ai; nur Qwen sandte)
