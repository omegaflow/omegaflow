<!--
  title: Handover — River-Folge 171 (2026-10-10)
  session: River-Folge 171
  class: handover
  date: 2026-10-10
  sha256: 193e398314cd20fa2801734f821342aa93e1f49a054e3b25b0ef49f8264e8ad6
  status: live
-->
# Handover — River-Folge 171 (2026-10-10)

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
`docs/handover/archiv/handover-2026-10-10-river-folge170.md` §Operator-Wort-Register.

## Träger (Prosa, eigene)

- `docs/concepts/kanal-ontologie-komplettbau.md` — der komplette Bauplan (P0–P10); P10.2a als gebaut nachgeführt (`:215`).
- `docs/concepts/archivar-mathematikerin.md` — der Wire/GPU-Force-Vertrag; Träger des kinetischen Rahmens.
- `state/stimmen/2026-10-10-river-p10.2a-token-semantik.md` — Rat + UI-/Open-Weight-Roster zur Token-Semantik.
- `state/stimmen/2026-10-10-river-advective-conserved.md`, `state/stimmen/2026-10-10-river-p92-rat.md`.
- `docs/blatt/blatt-gic-breitenband-familien.md` (`status: unsealed`) — Siegel = Operator-Wort, offen.
- `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md`, `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md`, `docs/paper/gic-causal-driver.md`, `docs/paper/flyby-path-2-addendum-2026-09-29.md`, `docs/concepts/remove-bias.md`.
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` — Asservatenkammer, Mountain-302 per `## An river` an River zugewiesen (`de2004534`, 2026-10-10); Träger. Nächster Schritt River (Stein = `field_te_query`): die 8 probe-gelesenen Kanäle (RTSW/SWPC · EVE 1032/131 · QBO `qbo_30hpa` · D20 `d20_thermocline` · Kp `magnetosphere_kp_3h` · Swarm HAPI · EEG ds007822/ds007471 · Newell `dΦ/dt`) in die Matrix heben und die Matrix als Störungs-Experiment betreiben (`:103-104`).

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

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort 2026-09-29; kein Maschinen-Akt.
- **SuperDARN MAP/Globus Re-Submit** — Operator-Wort 2026-10-09: „warte bis zur glasfase"; `wartend.φ:8`.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session (River 171):

- `docs/handover/handover-2026-10-10-river-folge171.md` (neu)
- `docs/handover/archiv/handover-2026-10-10-river-folge170.md` (Move)

## Burn: open 0.0000 · close 0.0251 — River 171 (deepseek-flash, kein pro/max; gemessen `session_burn`; Fenster 14 Sessions total $0.5107)
