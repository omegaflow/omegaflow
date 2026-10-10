<!--
  title: Handover — River-Folge 170 (2026-10-10)
  session: River-Folge 170
  class: handover
  date: 2026-10-10
  sha256: 75496c073920aaad83c283c851f57cd1be1ef6836cf1e524d1463ffd968336d5
  status: live
-->
# Handover — River-Folge 170 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward).

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** abgearbeitet; jeder wird **aufgeschlüsselt** geführt
(**Trigger** / **Lage** / **Blockade** / **Braucht**).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Nicht gebaut — die P10.2a-Token-Semantik ist nicht entschieden; ein geratener Bau wäre Fabrikation (A=A). bitte archive search all rat und roster" | 2026-10-10 | Operator (Session, River 165)
„wenn du den rat befragst befrage bitte auch die wissenschaft mit archive serahc alll und die ui und oenweight frontier voices" | 2026-10-09 | Operator (Session, River 145)
„warum nur duck … ich möchte dass du alle frontier chats befragst" | 2026-10-07 | Operator (Session, River 127)
„ja ich meine alle blöcke müssen korrekt sein dafür haben wir doch die wissenschaft" | 2026-10-09 | Operator (Session, River 160) — **P10-Wort**: jeder `field`-Block wird wissenschaftlich geprüft und korrekt etikettiert (Quantity | Mechanism | Medium)
„ja natürlich sonst wartest du doch bis zum st. nimmerleinstag" + „ja bitte ihr müsst das jetzt echt mal in den griff bekommen" | 2026-10-10 | Operator (Session, River 170) — den abgebrochenen `ci-gate` neu anstoßen; den CI-Burst in den Griff bekommen
„mach A" | 2026-10-10 | Operator (Session, River 170) — `ci-gate.yml`-`subset` von dem geteilten t420 auf GitHub-hosted `ubuntu-24.04-arm` mit per-SHA-Concurrency umziehen (Operator-Wort für den Linienwechsel in Myceliums Workflow-Pfad)
„beides" | 2026-10-10 | Operator (Session, River 170) — (1) die zwei fremden ci-gate-Roten heilen (`observer.rs`-Clippy-Lints, `eigene-ephemeride.md`-see-also) und (2) die `observer`→`receiver`-Frage als Rat-Frage aufsetzen; dazu den Audit „wo wurde der Observer-Bias wieder eingeschleust"

Verbatim: `state/operator-gespraeche/2026-10-10-river.md` und
`state/operator-gespraeche/2026-10-09-river.md`. Fortgeschrieben aus
`docs/handover/archiv/handover-2026-10-10-river-folge169.md` §Operator-Wort-Register.

## Träger (Prosa, eigene)

- `docs/concepts/kanal-ontologie-komplettbau.md` — der komplette Bauplan (P0–P10); P10.2a als gebaut nachgeführt (`:215`).
- `docs/concepts/archivar-mathematikerin.md` — der Wire/GPU-Force-Vertrag; Träger des kinetischen Rahmens.
- `state/stimmen/2026-10-10-river-p10.2a-token-semantik.md` — Rat + UI-/Open-Weight-Roster zur Token-Semantik.
- `state/stimmen/2026-10-10-river-advective-conserved.md`, `state/stimmen/2026-10-10-river-p92-rat.md`.
- `docs/blatt/blatt-gic-breitenband-familien.md` (`status: unsealed`) — Siegel = Operator-Wort, offen.
- `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md`, `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md`, `docs/paper/gic-causal-driver.md`, `docs/paper/flyby-path-2-addendum-2026-09-29.md`, `docs/concepts/remove-bias.md`.

## Offen (aufgeschlüsselt)

### Flyby-Kette — recon bleibt
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** `ephemeris-juice-recon` Wiedervorlage 2026-11-01 (ESA/ESOC publiziert die Post-Flyby-SPK). Wahrheit: `state/zustand/wartend.φ:34`.
- **Lage:** (gemessen 2026-10-08, River 138) RTSW gefüllt; kp `def` freigegeben; `omni2_bz` absent → `pending`; `δ = 0,168 km` steht, Δ + σ_recon brauchen die Post-Flyby-Rekonstruktion (`ephemeris_juice_recon.bin` 404).
- **Blockade:** ESA/ESOC-SPK + 1-σ-Kovarianz absent.
- **Braucht:** am 2026-11-01 `archive_search --verdict` auf den ESOC-recon-Pfad; dann `flyby_ephemeris_gate --recon <recon.bin> --sigma-recon <km>`.

### newell-omni-alignment
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `field-te-query`-Lauf (Job `matrix-newell-omni`, `.github/workflows/field-te-query.yml:150`). Wahrheit: `state/zustand/wartend.φ:42`.
- **Lage:** (gemessen via `wartend.φ:42`, Mountain 270, 2026-10-07) Mountain-Format-Arm steht; Job `matrix-newell-omni` **n=0**, `alignment pending`. Die Alignment-Maschine ist gebaut (`tools/measure/src/bin/field_te_query.rs:4148 align_many`, `:4511` „alignment stays pending — {reason}"); die Zellen bleiben n=0, weil keine gemeinsame Zeitachse (OMNI vs. RTSW-Join) anfällt.
- **Blockade:** keine deckungsgleiche Zeitachse (Daten-/Lauf-Warten).
- **Braucht:** `gh workflow run field-te-query.yml` → `matrix-newell-omni`-Zellen lesen; dann den Arm gegen `phi/pipeline/descriptors/newell_geospheric_omni.te` prüfen.

### vlies-matrix-alignment
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `field_te_query`-Lauf. Wahrheit: `state/zustand/wartend.φ:47`.
- **Lage:** (gemessen via `wartend.φ:47`, Mountain 270 / River 112) Lauf `37500311359` 15/15 Arme; Solar-/Magnetosphären-Zellen **n=0**.
- **Blockade:** Format-/Compiler-Arm für die deckungsgleiche Zeitachse.
- **Braucht:** den Arm gegen die Zellen prüfen — `tools/measure/src/bin/field_te_query.rs` + `phi/pipeline/descriptors/vlies_matrix.te`; erste Messung: einen Lauf mit dem Arm lesen.

## An mountain

Origin: river-170 (2026-10-10).

- **Rat-Verdikt (5 Stimmen, einstimmig): `observer` → `receiver`.** Umbenennung von Modul, Datei, Struct und Doku/Reduktionsort in **einem** Atom + `cargo check` — Name = Implementation; die Physik ist bereits Receiver-als-Worldline (explizite Parameter, absent → None). Grund ausgelöst durch Operator-Frage „warum zur hölle gibt es einen observer?".
- **Der neue Vantage-Vektor:** `observer.rs:4` `SUN_ICRS_KM=[0,0,0]` + `shapiro_sun_leg_s` (`:73-97,:130-131`) wählt die gravitierende Sonne als ICRS-Origin — Q5-Vantage+Default, noch unverdrahtet. Rat: die Masse als **expliziten Parameter** führen, Position aus `BodyProperties` (Ephemeriden-Binary); ist der ICRS-Ursprung gemeint, ist er der **SSB**, kein Körper → Reduktion verweigert (`None`), nie `[0,0,0]`.
- **Rest-Bias (Audit `explore` 2026-10-10, Rat-Tabelle):** (c) `weberin.rs:264-268` `frame_origin_name()` → hardcoded NAIF 10 = **Bias** → deklarierter Ursprung; (d) `weberin.rs:253-262` `woven_major_bodies()` `[1,2,4,5,6,7,8,301,399]` = **Bias** ohne registrierte Target-Liste; (b) `odp.rs:9` `const EARTH = include_str!("kernels/dsn_host.txt")` = Daten legitim, Const-Bindung **Bias** → Host als Parameter; (g) `port.rs:3761` `frame.starts_with("at sun")` = **Bias** → deklariertes Frame-Feld; (e) `gaia_sso.rs:15-30` `TNO_NAME` = legitim **bei Register-Zeile**; (f) `BODY_COMET`/`BODY_COMETELS` = legitimes Label. Geschlossen seit 2026-10-06: `frames.rs`-Defaults, `omega.rs:878`, `weberin.rs:1801`, `main_flow.rs`-Bypass, `membrane.html`-Trio.
- **CI-`register`-Rot aus deinem Handover:** `docs/handover/handover-2026-10-10-mountain-folge302.md:133/:139` — die Prosa in `## An river` trägt `see-also:` mitten im Satz; `path_reference_scan` (`file_refs`, scannt `see-also:` an jeder Zeilenstelle) meldet daraus 5 `MISS`. **Braucht:** den Satz so umformulieren, dass `see-also:` nicht als Token-Anfang mitten im Text steht (Punkt bleibt), oder den Scanner auf Header-/Zeilenanfangs-Direktiven einschränken. Lokal gemessen `cargo run -p omegaflow-register --bin path_reference_scan` → 5 MISS. (Eigene `eigene-ephemeride.md`-MISS sind geheilt, `91c74e73f`.)

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort 2026-09-29; kein Maschinen-Akt.
- **SuperDARN MAP/Globus Re-Submit** — Operator-Wort 2026-10-09: „warte bis zur glasfase"; `wartend.φ:8`.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session (River 170):

- `src/archivar/tiff.rs` (`apply_predictor` + `undo_predictor2/3` geheilt)
- `src/mathematikerin/observer.rs` (zwei Clippy-Lints geheilt — Operator-Wort „beides")
- `docs/concepts/eigene-ephemeride.md` (see-also auf das archivierte `mountain-301`)
- `.github/workflows/ci-gate.yml` (`subset` auf `ubuntu-24.04-arm`, per-SHA-Concurrency — Operator-Wort „mach A")
- `docs/concepts/self-hosted-runner.md` (Routing an den neuen Lauf angeglichen)
- `docs/handover/handover-2026-10-10-river-folge170.md` (neu)
- `docs/handover/archiv/handover-2026-10-10-river-folge169.md` (Move)

## Burn: open 0.0000 · close 0.2104 · cap 0.25 — Grund: CI-Forensik (Job-Ebene) + `subset`-Move (A) + Observer-Bias-Audit + Rat; alles flash, kein pro/max (gemessen `session_burn`)
