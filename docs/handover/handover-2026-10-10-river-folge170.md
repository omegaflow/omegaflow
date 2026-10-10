<!--
  title: Handover — River-Folge 170 (2026-10-10)
  session: River-Folge 170
  class: handover
  date: 2026-10-10
  sha256: 7d3c33e47346ede5c14b20e452af8249823ad38ed0a60d11ddde90aeff24913f
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

## An mycelium

Origin: river-170 (2026-10-10).

- **CI per-SHA-Verdikt bleibt unter Commit-Burst dauerhaft `pending` — die grünen Compile-Gates werden vom Run-Conclusion `cancelled` maskiert.** Gemessen (`ci_manage jobs` + GitHub-API): die ci-gate-Läufe `38058693008`/`38058676102`/`38058628757`/`38058460066` (SHAs `d97a74993`/`256ea61db`/`4a8ec9740`/`f8f332884`, alle Nachfahren von `3c4786a3b`) tragen `build`/`clippy`/`format`/`register` = **success**, `subset` = **cancelled** → Run-Conclusion `cancelled`. Ursache: `subset` (T420) hat `concurrency: ci-gate-subset-${{ github.ref }}` mit `cancel-in-progress: true`, und der Watchdog cancelt zusätzlich jedes **queued** ci-gate mit `head_sha != tip`; `ci_gate_register` (`REQUIRED_CHECK=subset`) liest das als `pending`, `state/zustand/ci-gate.φ` trägt seit 2026-10-09 nur 3 `pending`-Zeilen (nicht fortgeschrieben). Bei anhaltendem Burst erreicht **kein** SHA ein `green`. **Braucht (Mycelium + Rat-Linse):** den durable per-SHA-Verdikt aus `build`+`clippy` (Compile-Gate) ableiten und `subset` separat `pending` führen — oder `subset` unter Burst queuen statt canceln.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort 2026-09-29; kein Maschinen-Akt.
- **SuperDARN MAP/Globus Re-Submit** — Operator-Wort 2026-10-09: „warte bis zur glasfase"; `wartend.φ:8`.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session (River 170):

- `src/archivar/tiff.rs` (`apply_predictor` + `undo_predictor2/3` geheilt)
- `docs/handover/handover-2026-10-10-river-folge170.md` (neu)
- `docs/handover/archiv/handover-2026-10-10-river-folge169.md` (Move)

## Burn: open 0.0000 · close 0.0712 — River 170 (deepseek-flash, kein pro/max; gemessen `session_burn`)
