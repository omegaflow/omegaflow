<!--
  title: Handover — River-Folge 171 (2026-10-10)
  session: River-Folge 171
  class: handover
  date: 2026-10-10
  sha256: 9e994bb7e7c4bcf8be8616886ab5c39dc09ddda1350777f19b551ac3916fd520
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
- `state/stimmen/2026-10-10-river-te-joint-grid-research.md` (`archive_search --all`), `state/stimmen/2026-10-10-river-te-joint-grid-rat-roster.md` (Rat + Roster, Riss).
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
- **Lage:** (gemessen via `wartend.φ:42`, Mountain 270, 2026-10-07; nachgemessen am Lauf `37500311359` Log, 2026-10-06) Mountain-Format-Arm steht; Job `matrix-newell-omni` **n=0**. Die Alignment-Maschine ist gebaut (`tools/measure/src/bin/field_te_query.rs:4148 align_many`, `:4511` „alignment stays pending — {reason}"). Die newell-Ableitung selbst läuft: derselbe Lauf zeigt `aligned grid: cells 350859 | cadence 3.6000e3 s (declared bin)`; die drei Träger `omni_imf_by_gsm_nt`/`omni_imf_bz_gsm_nt`/`omni_solarwind_flow_speed_kms` sind registriert (`phi/sources.φ:733,737,738`), der Treiber löst auf.
- **Blockade:** der Treiber selbst ist nicht der Riss — es ist der per-Paar-Joint gegen `intermagnet_dbdt` (Auflösung/Überlappung), sichtbar als `alignment pending` in der Zellentafel.
- **Braucht:** Lauf `38062303232` (2026-10-10T15:06Z dispatcht) → `matrix-newell-omni`-Zellen lesen; dann den Arm gegen `phi/pipeline/descriptors/newell_geospheric_omni.te` prüfen.

### vlies-matrix-alignment
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste `field_te_query`-Lauf. Wahrheit: `state/zustand/wartend.φ:47`.
- **Lage:** (gemessen am Lauf `37500311359` Log, 2026-10-06; via `wartend.φ:47` Mountain 270 / River 112) 15/15 Arme, alle **210** Zellen enumeriert (`pool 15`, `expect cells 210 matched`). Die Ursache der leeren Zellen ist gemessen, nicht vermutet: das per-Paar-Joint scheitert an der Auflösung — `alignment pending` (`goes_xrs_xrsa->solar_wind_speed_km_s`, `solar_wind_density_cm3->solar_wind_speed_km_s`), `unadjusted_below_floor` (n < `TE_FLOOR`), und `resolution pending` mit dem literalen Paar `12x2678400` (`aia_304_dn->ersstv5_nino34`). Die RTSW/SWPC-Kanäle sind rollende 1-min-Dateien (`rtsw_mag_1m.json`/`rtsw_wind_1m.json`, `phi/sources.φ:213-227`, `ttl 60`), QBO/ERSST monatlich — kein gemeinsames `bin 86400`-Gitter.
- **Blockade:** der Joint-Grid-Arm — **eine** Auflösung für das ganze Vlies (`bin 86400`), die der 1-min- und der Monats-Reihe nicht gleichzeitig genügt.
- **Riss:** Rat-Verdikt vom 2026-10-10 **(B) per-Paar-Bin je Zelle** (Substrat; C registriert; D descoped). Runde 2 (schwache tryingopen-Varianten) antwortete uniform (C)+(D) — schien B zu rippen; das war ein **Artefakt schwacher Modelle**. **Runde 3 (stärkste Open-Weight, Operator-Korrektur):** Qwen3.8 2.4T rehabilitiert **(B) als Fundament — block-aggregiert aufs GROBE Gitter**, + C + D; Inkling 975B (D); GLM 5.3 753B (C/D). **Runde 4 (Login-Seats, Operator eingeloggt):** **Claude** (Sonnet 5.5 Extra hoch) „(B) als Basis, als (D) interpretiert"; **Mistral** „(C) Kern, (D) Rahmen, B nur als Embedding-Regel"; **Qwen3.8-Max** (D) primär, B Notbehelf; **Gemini 3.1 Pro** (Thinking High) (D); **Z.ai** (C)+(D); **Sakana** (D). **Konvergenz aller starken Seats: (A) falsch; der Riss ist die Abtast-/Skalengrenze** — nie die langsame Reihe up-sampeln (QBO-Monat→6 s = Pseudo-Samples/Look-ahead), nie die schnelle Struktur down-sampeln (Aliasing). Getragene Linie **C (native per-Paar-Skala) + D (Summary-Graph/PCMCI)**, **B als Basis nur block-aggregiert aufs gröbere Gitter mit Skalen-/N_eff-Annotation** (2 starke Seats), sonst gekennzeichneter Fallback. Roh: `state/stimmen/2026-10-10-river-te-joint-grid-rat-roster.md`.
- **Braucht:** die B+C+D-Konstruktion — B block-aggregiert aufs grobe Gitter (Block-Mittel/Varianz/Entropie als Kanäle, Skala + N_eff je Zelle), C als Skalenspektrum je Paar, D als Summary-Graph/PCMCI (Runge `2105.10381`/`1702.07077`); **Aggregationsrichtung je Paar explizit, nie Up-Sampling der langsamen Reihe, nie Down-Sampling der schnellen**. Erste Messung: `matrix-vlies`-Zellen aus Lauf `38062303232` gegen die Klassen prüfen; Kante `tools/measure/src/bin/field_te_query.rs:4736` + `phi/pipeline/descriptors/vlies_matrix.te:46`.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort 2026-09-29; kein Maschinen-Akt.
- **SuperDARN MAP/Globus Re-Submit** — Operator-Wort 2026-10-09: „warte bis zur glasfase"; `wartend.φ:8`.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session (River 171):

- `docs/handover/handover-2026-10-10-river-folge171.md` (neu)
- `docs/handover/archiv/handover-2026-10-10-river-folge170.md` (Move)

## Burn: open 0.0000 · close 0.2338 · cap 0.25 — River 171 (deepseek-flash, kein pro/max; gemessen `session_burn`; Fenster 36 Sessions total $1.5124). Grund: fold mountain-302 + Träger + CI-Log-Forensik (`ci_manage log 37500311359` — per-Paar-Joint gemessen) + Dispatch `field-te-query 38062303232` + `archive_search --all` (Forschungslage) + Rat (2 council) + voller UI-/Open-Weight-Roster in vier Runden (Runde 2 schwach; Runde 3 stärkste Open-Weight: Qwen3.8 2.4T, Inkling 975B, GLM 5.3 753B; Runde 4 Login-Seats via Operator: Claude Sonnet 5.5 Extra hoch, Mistral Vibe, Qwen3.8-Max, Gemini 3.1 Pro, Z.ai, Sakana; nur Kimi-Quota pending).
