<!--
  title: Handover — River-Folge 173 (2026-10-10)
  session: River-Folge 173
  class: handover
  date: 2026-10-10
  sha256: 95e55aeb3483c5a2ba0d5076979aa630705c8ade5ae3368da85aee574464e4d8
  status: live
-->
# Handover — River-Folge 173 (2026-10-10)

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
„C (Skalenspektrum je Paar) ist der nächste begrenzte Schritt — Architekturfrage → Linse der fünf Stimmen, bevor gebaut; danach D (Summary-Graph/PCMCI, Runge 2105.10381/1702.07077). Die Seats: B ohne Skalenannotation wäre die stille Fabrikation. bitte --all rat und max roster" | 2026-10-10 | Operator (Session, River 172) — C vor dem Bau durch die fünf Stimmen + Max-Roster; `--all`-Recherche voran
„Nächster Schritt: D2 durch die fünf Stimmen (Rat), dann Skeleton/MCI bauen. bitte --all max roster und rat" | 2026-10-10 | Operator (Session, River 172) — D2 (Summary-Graph/PCMCI) durch Rat + Max-Roster, `--all` voran

Verbatim: `state/operator-gespraeche/2026-10-10-river.md` und
`state/operator-gespraeche/2026-10-09-river.md`. Fortgeschrieben aus
`docs/handover/archiv/handover-2026-10-10-river-folge172.md` §Operator-Wort-Register.

## Träger (Prosa, eigene)

- `docs/concepts/kanal-ontologie-komplettbau.md` — der komplette Bauplan (P0–P10); P10.2a als gebaut nachgeführt (`:215`).
- `docs/concepts/archivar-mathematikerin.md` — der Wire/GPU-Force-Vertrag; Träger des kinetischen Rahmens.
- `state/stimmen/2026-10-10-river-p10.2a-token-semantik.md` — Rat + UI-/Open-Weight-Roster zur Token-Semantik.
- `state/stimmen/2026-10-10-river-advective-conserved.md`, `state/stimmen/2026-10-10-river-p92-rat.md`.
- `state/stimmen/2026-10-10-river-te-joint-grid-research.md` (`archive_search --all`), `state/stimmen/2026-10-10-river-te-joint-grid-rat-roster.md` (Rat + Roster, Riss).
- `state/stimmen/2026-10-10-river-c-skalen-spektrum-research.txt` (`archive_search --all` zu C), `state/stimmen/2026-10-10-river-c-skalenspektrum-rat.md` (Rat, 5 Stimmen, C-Verdikt), `state/stimmen/2026-10-10-river-c-skalenspektrum-roster.md` (Max-Roster: Claude · Qwen3.8 2.4T · Qwen · Z.ai/GLM-5.3 Deep Think Max · Duck/Claude Haiku · open-weight GLM 5.3 — alle geantwortet; Duck-Send-Ursache gemessen).
- `state/stimmen/2026-10-10-river-d2-pcmci-research.txt` (`archive_search --all` zu D2: Runge `2003.03685` PCMCI+, CEDAR, AutoCause), `state/stimmen/2026-10-10-river-d2-pcmci-rat.md` (Rat, 5 Stimmen, D2-Verdikt), `state/stimmen/2026-10-10-river-d2-pcmci-roster.md` (Max-Roster: Claude · Qwen3.7-Plus · Duck/Claude Haiku · Z.ai/GLM-5.3 Deep Think Max · Qwen3.8-Max — alle geantwortet; Duck GPT-6 blockiert durch Tageslimit).
- `src/mathematikerin/mci.rs` — der MCI-Wrapper (P3): `TeBinnedCi` (binned-TE-CiTest, Label „TE-gestützt/linear"), `PanelView` (lagged panel), `mci_window_links` (PC1 + MCI + BY).
- `docs/blatt/blatt-gic-breitenband-familien.md` (`status: unsealed`) — Siegel = Operator-Wort, offen.
- `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md`, `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md`, `docs/paper/gic-causal-driver.md`, `docs/paper/flyby-path-2-addendum-2026-09-29.md`, `docs/concepts/remove-bias.md`.
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` — Asservatenkammer (11 offen), Mountain-303 per `## An river` an River als Träger bestätigt (2026-10-10), von River 173 gefaltet. Nächster Schritt River (Stein = `field_te_query`): die 8 probe-gelesenen Kanäle (RTSW/SWPC · EVE 1032/131 · QBO `qbo_30hpa` · D20 `d20_thermocline` · Kp `magnetosphere_kp_3h` · Swarm HAPI · EEG ds007822/ds007471 · Newell `dΦ/dt`) in die Matrix heben und die Matrix als Störungs-Experiment betreiben (`:103-104`).

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
- **Lage:** (gemessen 2026-10-10, River 173, via `ci_manage log 38062303232` / `38082624048`) Mountain-Format-Arm steht; Job `matrix-newell-omni` **n=0**. Die Ursache der roten Trigger-Läufe ist gemessen: der Test-Target von `field_te_query` kompiliert nicht — der `FieldConfig`-Testhelfer trägt kein `band_id` (E0063, `tools/measure/src/bin/field_te_query.rs:5481`), seit `types.rs:383` das Feld führt. Behoben (Commit `f7f44f7cc`, River 173); Re-Dispatch `38086122792` (queued).
- **Blockade:** der per-Paar-Joint gegen `intermagnet_dbdt` (Auflösung/Überlappung), sichtbar als `alignment pending` in der Zellentafel.
- **Braucht:** Lauf `38086122792` → `matrix-newell-omni`-Zellen lesen; dann den Arm gegen `phi/pipeline/descriptors/newell_geospheric_omni.te` prüfen.

### vlies-matrix-alignment
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `field_te_query`-Lauf. Wahrheit: `state/zustand/wartend.φ:47`.
- **Lage:** (gemessen am Lauf `37500311359` Log, 2026-10-06; via `wartend.φ:47` Mountain 270 / River 112) 15/15 Arme, alle **210** Zellen enumeriert (`pool 15`, `expect cells 210 matched`). Die Ursache der leeren Zellen ist gemessen: das per-Paar-Joint scheitert an der Auflösung — `alignment pending` (`goes_xrs_xrsa->solar_wind_speed_km_s`, `solar_wind_density_cm3->solar_wind_speed_km_s`), `unadjusted_below_floor` (n < `TE_FLOOR`), und `resolution pending` mit dem literalen Paar `12x2678400` (`aia_304_dn->ersstv5_nino34`). Die RTSW/SWPC-Kanäle sind rollende 1-min-Dateien (`rtsw_mag_1m.json`/`rtsw_wind_1m.json`, `phi/sources.φ:213-227`, `ttl 60`), QBO/ERSST monatlich — kein gemeinsames `bin 86400`-Gitter. Der Riss ist rat-verdiktet (2026-10-10, `state/stimmen/2026-10-10-river-te-joint-grid-rat-roster.md`): **B block-aggregiert aufs gröbere Gitter** + **C native per-Paar-Skala** + **D Summary-Graph**, nie up-/down-sampeln. Der Bau ist durch: **B, C-Schritt 1, C2-Kern, C2b, D1** (River 172) und **D2 P1/P2/P3** (`parcorr.rs` `659e03b32`, `pc.rs` `6a86820ed`, `mci.rs` `4207a1265`; `--panel … --stage2 mci` verdrahtet, `d9d090c3f`).
- **Blockade:** der Joint-Grid-Arm — **eine** Auflösung für das ganze Vlies (`bin 86400`), die der 1-min- und der Monats-Reihe nicht gleichzeitig genügt.
- **Braucht:** Lauf `38086122792` → die C-Rungtabelle lesen und gegen die deklarierte Panel-Skala filtern; der Window-Graph über `field_te_query --panel <a,b,...> --stage2 mci` (gebaut), dann den Summary-Graph (min-p-Lag annotiert, Zyklen erlaubt) nach dem Window-Graph. Test-Verifikation des MCI-Wrappers läuft über `ci-gate` (`cargo test --lib`) am HEAD `d9d090c3f`.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort 2026-09-29; kein Maschinen-Akt.

## Burn: open 0.0000 · close 0.0797 — session_burn (River F173, deepseek-flash; drei grind-flash-Taucher separat)
