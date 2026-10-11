<!--
  title: Handover — River-Folge 179 (2026-10-11)
  session: River-Folge 179
  class: handover
  date: 2026-10-11
  sha256: f9c13dba1366c00bf67a7c069018c08572c9e9de6eaac664b43bc0b083de4acf
  status: live
-->
# Handover — River-Folge 179 (2026-10-11)

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

## Burn: open 0.0000 · close 0.0577 · cap 0.15 — Grund: River F179 Ein-Pass (deepseek-flash), kein pro/max, kein Diver; die eigene `line`-Session „River-Linie in einem Pass starten" steht bei `session_burn` $0.0577 (DB-Total $0.7161 zur Messzeit).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Nicht gebaut — die P10.2a-Token-Semantik ist nicht entschieden; ein geratener Bau wäre Fabrikation (A=A). bitte archive search all rat und roster" | 2026-10-10 | Operator (Session, River 165)
„wenn du den rat befragst befrage bitte auch die wissenschaft mit archive serahc alll und die ui und oenweight frontier voices" | 2026-10-09 | Operator (Session, River 145)
„warum nur duck … ich möchte dass du alle frontier chats befragst" | 2026-10-07 | Operator (Session, River 127)
„ja ich meine alle blöcke müssen korrekt sein dafür haben wir doch die wissenschaft" | 2026-10-09 | Operator (Session, River 160) — **P10-Wort**: jeder `field`-Block wird wissenschaftlich geprüft und korrekt etikettiert (Quantity | Mechanism | Medium)
„ja natürlich sonst wartest du doch bis zum st. nimmerleinstag" + „ja bitte ihr müsst das jetzt echt mal in den griff bekommen" | 2026-10-10 | Operator (Session, River 170) — den abgebrochenen `ci-gate` neu anstoßen; den CI-Burst in den Griff bekommen
„mach A" | 2026-10-10 | Operator (Session, River 170) — `ci-gate.yml`-`subset` von dem geteilten t420 auf GitHub-hosted `ubuntu-24.04-arm` mit per-SHA-Concurrency umziehen
„beides" | 2026-10-10 | Operator (Session, River 170) — (1) die zwei fremden ci-gate-Roten heilen; (2) die `observer`→`receiver`-Frage als Rat-Frage aufsetzen; Audit „wo wurde der Observer-Bias wieder eingeschleust"
„C (Skalenspektrum je Paar) ist der nächste begrenzte Schritt — Architekturfrage → Linse der fünf Stimmen, bevor gebaut; danach D (Summary-Graph/PCMCI, Runge 2105.10381/1702.07077). Die Seats: B ohne Skalenannotation wäre die stille Fabrikation. bitte --all rat und max roster" | 2026-10-10 | Operator (Session, River 172) — C vor dem Bau durch die fünf Stimmen + Max-Roster; `--all`-Recherche voran
„Nächster Schritt: D2 durch die fünf Stimmen (Rat), dann Skeleton/MCI bauen. bitte --all max roster und rat" | 2026-10-10 | Operator (Session, River 172) — D2 (Summary-Graph/PCMCI) durch Rat + Max-Roster, `--all` voran
„bitte --all max roster und rat" | 2026-10-10 | Operator (Session, River 174) — die Window-Graph-CI-Verdrahtung und die Panel-Skala Δ (D2-Rat-Verdikt Punkt 5) durch `--all`-Recherche + Rat (5 Stimmen) + Max-Roster
„z.ai immer glm 5.3 max gibt es auch in tryong open genauso wie qwen" | 2026-10-10 | Operator (Session, River 174) — Z.ai stets **GLM-5.3 Max**; `tryingopen` führt **GLM 5.3 Max** und **Qwen**

Verbatim: `state/operator-gespraeche/2026-10-10-river.md` und
`state/operator-gespraeche/2026-10-09-river.md`. Fortgeschrieben aus
`docs/handover/archiv/handover-2026-10-11-river-folge178.md` §Operator-Wort-Register.

## Träger (Prosa, eigene)

- `docs/concepts/kanal-ontologie-komplettbau.md` — der komplette Bauplan (P0–P10); P10.2a als gebaut nachgeführt (`:215`).
- `docs/concepts/archivar-mathematikerin.md` — der Wire/GPU-Force-Vertrag; Träger des kinetischen Rahmens.
- `state/stimmen/2026-10-10-river-window-graph-panel-rat.md` (Rat, 5 Stimmen), `…-roster.md`, `…-research.txt`.
- `state/stimmen/2026-10-10-river-d2-pcmci-rat.md` / `…-roster.md` / `…-research.txt`; `state/stimmen/2026-10-10-river-c-skalenspektrum-rat.md` / `…-roster.md` / `…-research.txt`.
- `state/stimmen/2026-10-11-river-panel-aufloesung-rat.md` — der Rat zur Auflösungspolitik von `load_field_across_sources` (Registerfix kanonisch; `first-load-wins` → gemessene Eigenschafts-Auswahl).
- `src/mathematikerin/descriptors/vlies_panel.te` + `vlies_matrix.te` (in `src/mathematikerin/descriptors/`).
- `src/mathematikerin/mci.rs` — `mci_window_graph` gibt `WindowGraph.screened` (das zweite Ohr).
- `src/mathematikerin/pc.rs` — `pc_stable_skeleton_screened` (Commit `10e0d1323`).
- `tools/measure/src/bin/field_te_query.rs` — `resolution_probe` (F179: nennt jeden registrierten Kandidaten inkl. Ladefehler) + Arm-Span-Nennung bei `panel pending`; der `load_field_across_sources`-Umbau (Rat Schritt 3) bleibt.
- `.github/workflows/field-te-query.yml` — Job `panel-vlies`.
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` — Asservatenkammer (11 offen).
- `docs/blatt/blatt-gic-breitenband-familien.md` (`status: unsealed`) — Siegel = Operator-Wort, offen.
- `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md`, `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md`, `docs/paper/gic-causal-driver.md`, `docs/paper/flyby-path-2-addendum-2026-09-29.md`, `docs/concepts/remove-bias.md`.

## Offen (aufgeschlüsselt)

### panel-vlies — Auflösung: Kandidaten werden jetzt benannt, Register/CDN bleiben
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Artefakt `field-te-panel-vlies.txt` des `field-te-query`-Neudispatches `38117177282` (HEAD `a7fe3a61b`, F179).
- **Lage:** (gemessen 2026-10-11, River 179 via `ci_manage log`/`gh api`/`sread`) Der Lauf `38105380164` (HEAD `2df1fee0f`) endet `panel pending — bin window carries no overlap between the arms`; die Log-Zeile ist `omni2_serie_1h.bin … 404` (CDN-Asset fehlt); eine `resolution`-Zeile fehlt, weil nur **ein** `omni_imf_bz_gsm_nt`-Kandidat lud. Register: drei Registrierungen — `:748` HAPI live `3600` (ohne `format`), `:1935` Tages-Bin `86400`, `:1950` 1h-Bin `86400` — **nicht dieselbe Größe**; die 1h-Bin-native ist `3600` (`omni2_compiler --decimate-min 60`). Weil der HAPI-live-Block kein `format` trägt, fällt `first-load-wins` auf den historischen Tages-Bin (endet 2026-08-06) → disjunkte Fenster gegen die SWPC-Live-Arme. **Gebaut (dieser Atom):** `resolution_probe` nennt jeden registrierten Kandidaten (median_dt oder Ladefehler); `run_summary_panel` nennt bei `panel pending` je Arm den Zeit-Span. `cargo build -p omegaflow-measure --bin field_te_query` grün.
- **Blockade:** der 1h-Bin ist auf dem CDN **404** (Manifestation fehlt, Mycelium) und die Register-Zeile deklariert ihn mit `86400` (Register-Lüge, Mountain); der Register-Hunk in `phi/sources.φ` ist **volatil** — bei Session-Start clean, am Sessionende `M` (14 Zeilen, parallele Session) — und wird von River nicht geschrieben.
- **Braucht:** nach Push `gh workflow run field-te-query.yml` (Job `panel-vlies`), Artefakt lesen → die neuen `resolution`-Zeilen nennen die Zeugen; danach Rat Schritt 3: `load_field_across_sources` (`:1249`) auf gemessene Auswahl (`median_dt` je Kandidat, feinster gleichartiger Zeuge, Zeuge benannt) umbauen.

### window-graph — die zwei Ohren stehen, Summary-Graph bleibt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner — eigener Bau-Schritt.
- **Lage:** (gemessen 2026-10-11, River 178) Schritt (2) steht: `mci_window_graph` gibt `WindowGraph.screened` zurück; `run_summary_panel` druckt die ParCorr-screeneten Paare; `cargo check` grün.
- **Blockade:** keine.
- **Braucht:** (Schritt 5, Rat `…window-graph-panel-rat.md:95`) `summary_graph_from_rungs()` — Quelle: die `matrix-vlies`-Rung-Zeilen (per-Paar native Skala), Annotation min-p-Lag, Assaad-artig (`Entropy 24(8):1156`, kein Resampling); eigenständiger Bau, kein Post-Processing des Window-Graphen. Offener Riss (nicht geglättet): D2-Verdikt Punkt 5 fordert „D liest die C-Rungtabelle", gebaut ist ein eigenständiges MCI auf einem Δ.

### Flyby-Kette — recon bleibt
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** `ephemeris-juice-recon` Wiedervorlage 2026-11-01 (ESA/ESOC publiziert die Post-Flyby-SPK). Wahrheit: `state/zustand/wartend.φ:34`.
- **Lage:** (gemessen 2026-10-08, River 138) RTSW gefüllt; kp `def` freigegeben; `omni2_bz` absent → `pending`; `δ = 0,168 km` steht, Δ + σ_recon brauchen die Post-Flyby-Rekonstruktion (`ephemeris_juice_recon.bin` 404).
- **Blockade:** ESA/ESOC-SPK + 1-σ-Kovarianz absent.
- **Braucht:** am 2026-11-01 `archive_search --verdict` auf den ESOC-recon-Pfad; dann `flyby_ephemeris_gate --recon <recon.bin> --sigma-recon <km>`.

## An mountain

Origin: river-179 (2026-10-11).

- **Register-Lüge `omni2_serie_1h.bin`** — (gemessen 2026-10-11 via `sgrep`/`sread`) `phi/sources.φ` registriert den 1h-Bin (`url …/omni2_serie_1h.bin`, `format omni2_serie`, `:1938`) mit **`86400`** in allen `field`-Zeilen (`:1950`); der Compiler `omni2_compiler` baut ihn mit `--decimate-min 60` (`decimate_s = 3600`). Die native Kadenz ist **3600**. Der Rat `state/stimmen/2026-10-11-river-panel-aufloesung-rat.md` nennt das die Register-Lüge (A ≠ A) und den Drei-Registrierungs-Defekt. **Der Register-Hunk in `phi/sources.φ` ist volatil** — bei Session-Start clean, am Sessionende `M` (14 Zeilen, parallele Session, gemessen F179) — River schreibt ihn nicht. Bitte: die `field`-Zeilen des 1h-Blocks auf `3600` setzen und die Drei-Registrierung (`:742` HAPI live ohne `format`; `omni2_serie.bin` `:1935` 86400; `omni2_serie_1h.bin`) entflechten.

## An mycelium

Origin: river-179 (2026-10-11).

- **CDN-Asset `omni2_serie_1h.bin` fehlt (404)** — (gemessen 2026-10-11 via `ci_manage log 38105380164`) `fetch_bytes_headers … omni2_serie_1h.bin curl: (22) 404`. Die Register-Zeile `phi/sources.φ:1938` nennt die Release-URL, das Asset ist auf dem CDN nicht manifestiert. Bitte über `omni2-cdn.yml` (`--decimate-min 60`, `asset omni2_serie_1h.bin`) manifestieren.
