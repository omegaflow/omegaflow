<!--
  title: Handover — River-Folge 123 (2026-10-07)
  session: River-Folge 123
  class: handover
  date: 2026-10-07
  sha256: 364c3d8ef659bd232465438d71cfb13979777196c4de637a1d285a52adc2473c
  status: live
-->
# Handover — River-Folge 123 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | 2026-10-07 | Operator (Session, River 123) — Session-Start, Delegations-Consent
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-river-folge122.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Stimmen-Rolle (gemessen 2026-10-07)

Recherche (Wissenschafts-/Forschungslandschaft, Netz) trägt **`voice-deepseek`**; **strikt lokal nur
DeepSeek-flash** (`opencode.json`: alle nicht-DeepSeek-Agenten + alle pro/max deaktiviert); Denken/Urteil
= UI-Frontier. Der **Rat** bleibt Form/Linse (fünf exklusive Perspektiven). Benchmark:
`state/benchmark/2026-10-07-recherche-stimmen.md`.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie; Siegel = Operator-Wort, offen.
- `docs/surveys/survey-2026-10-03-exzellenz-gate.md` — Label geschlossen (`:111`, gemessen 2026-10-07).
- `docs/paper/gic-causal-driver.md` — NUR-Asset-Zeile eingetragen (2026-10-07, sha `e8915d29…`).

## Offen (aufgeschlüsselt)

### NUR-dB/dt–GIC-Relation — Datenkanal manifestert; Relation der nächste Bau-Atom
- **Status:** autonom (eigener Bau) | **Bindung:** eigen
- **Trigger:** keiner — eigener Schritt; nächster Bau-Atom nach dem Format-Fix.
- **Lage:** (gemessen 2026-10-07, River 123 via `archive_search --sniff`/`--verdict`) NUR 10 s dB/dt ist CDN-Asset `fmi_image_mag_nur.bin` (`phi/sources.φ:18030`): 15 551 948 B, sha256 `9c76f881d33e5e2d0c84b60b1641d4a7262f0039e43d3795a0dd8e9044714e0e`, HTTP 206 (stage-1 direct). Paper §6 (`docs/paper/gic-causal-driver.md:613-618`) trägt die Asset-Fakten; die sub-daily Relation selbst ist ungemessen.
- **Blockade:** kein Reader/Probe-Bin für `fmi_image_mag_dxdt` × `fmi_gic` (GIC stündliche Buckets, NUR 10 s) — kein vorhandener Probe konsumiert das NUR-Asset (gemessen: `sgrep image_mag tools/measure` = 0).
- **Braucht:** neuer bounded Bau: Probe-Bin, der NUR-dx/dt lädt + stündlich aggregiert, gegen `fmi_gic`-Buckets (1999–2023) korreliert (Viljanen 2025 Eq. 43), Zahl → Paper §4; `cargo run -p omegaflow-measure --bin <name>`.

### GIC-Familien — Rat-Verdikt Route C (Stufe 2); Generator + Familien-Kanal-Listen gebaut; Stufe-2-Pool wartet auf dB/dt-Bestand
- **Status:** wartend (Mountain-dB/dt) | **Bindung:** eigen (cross-line mountain)
- **Trigger:** per-Station-dB/dt-Netz (Mountain) bzw. Operator-/Rats-Wort für den Stufe-2-Lever.
- **Lage:** (gemessen 2026-10-07, River 121/122, `council`) **Route C (gewählt):** Stufe 1 bleibt global (`matrix full` + `fdr bh over matrix`); die Familie lebt **allein in Stufe 2** als Member-Pool des `compute_max_t` (`field_te_query.rs:2784`), abgeleitet zur Abfragezeit aus `cgm_lat` + fixierten Grenzen; Gruppierung **Target-Band** (Rat + starke Modelle), Treiber bleibt unbandiert. **Stufe-2-Form (20-Stimmen-Rat + `archive_search`, 2026-10-07):** Modus `--stage2 family` nach der Matrix; je Familie ein studentisiertes WY-max-t über den **vollen** Target-Band-Pool, **gemeinsame Surrogat-Draws**; Stufe 1 (BH/BY) byte-identisch. α: Strong-FWER je Familie über das max; bandübergreifend max über die drei Familien-Maxima (exakt) bzw. Bonferroni α/3 (konservativ); Šidák ungedeckt. **Zwei Risse:** (1) unvollständiger Pool → Null konditional/provisorisch, Pool-Version ins Ergebnis, fehlende Station `pending`; (2) Null NUR über den vollen Pool, nie über die Stufe-1-Überlebenden (Auswahl-Leckage). **Gebaut (River 121):** `tools/measure/src/bin/cgm_lat_partition.rs` (`--from-tsv --emit-dir`) → `state/river/gic-family-{auroral,sub-auroral,mid-latitude}.txt` (154 = 31+25+98, disjunkt, `unassigned 0`); Deckungstest grün. **Riss #4 (gemessen):** nur ABK 1h/1m + SOD 1h tragen `field intermagnet_dbdt` (`phi/sources.φ:2051-2079`); die 154 GIN-Blöcke tragen `intermagnet_xyz_x/y/z_nt`. Literatur: `docs/surveys/survey-2026-10-07-fwer-te-landschaft.md`; Rohmaterial `state/river/gic-stage2-20-stimmen-2026-10-07.md`.
- **Blockade:** per-Station-dB/dt-Netz fehlt (Mountain, Quellen-Eigenschaft).
- **Braucht:** (1) Mountain: 154 per-Station-dB/dt-Kanäle (s. `## An mountain`); (2) danach Familien-Pool in `compute_max_t` (Stufe 2) — der nächste Bau-Atom nach NUR; (3) CGM-Provenienz CPL/TTB: `phi/sources.φ` führt CPL `cgm_lat 11.23` / TTB `cgm_lat -2.62` aus dem `bgs-quasi-dipole`-Fallback (`cgm_lat_partition.rs:165`, QD ≠ CGM; OMNIWeb-VITMO-Endpunkt weist |lat| < 20° ab, gemessen 2026-10-07) — Braucht lokales AACGM/IGRF-Bin.

### `ozzy` — Negative Fuzzy Engine (CPU-Floor + GPU-Wire gebaut; CI-Verifikation offen)
- **Status:** wartend (CI) | **Bindung:** eigen
- **Trigger:** `ci-check`/`ci-gate` grün am jeweiligen HEAD.
- **Lage:** (gemessen 2026-10-07, River 122) `independence_verdict` (`ozzy.rs:146`) trägt A-Test + B-Diagnose + Known-Answer-Gates; `TE_SURR_FLOOR = 99` (`te.rs:3450`); GPU-Wire `TE_SERIES_COUNT = 101` + WGSL-Parität. `register_lookup --fired` meldet den Punkt (FIRED_UNGEMESSEN); CI am jeweiligen HEAD `queued`/`unread` (`ci_manage` — kein Polling).
- **Blockade:** keine (eigene); CI-Runner/Queue.
- **Braucht:** grüner `ci-check`/`ci-gate` am HEAD (Stehender Pass).

### em-Apertur — Kanal-Identität statt Kernel-Proxy
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am HEAD.
- **Lage:** (gemessen 2026-10-07, River 122) River-/Mountain-Seite gebaut (`shaders.rs:186`/`:211`); `register_lookup --fired` meldet FIRED; CI `queued`/`unread`.
- **Blockade:** keine (eigene); CI-Runner/Queue.
- **Braucht:** grüner `ci-check` am HEAD (Stehender Pass).

### Membran-Startansicht — zwei Aperturen (Parity-Fix gebaut; CI-Verifikation offen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`/`wasm-parity` grün am Fix-HEAD `116611e2e`; danach Pages-Deploy + Browser-Sicht.
- **Lage:** (gemessen 2026-10-07 via `ci_manage list`) Parity-Fix committed (`116611e2e` river 114); CI-Deploy trägt den Bau noch nicht.
- **Blockade:** CI-Lauf-Ausgang `unread` (Stehender Pass/`ci_manage`, kein Polling).
- **Braucht:** CI-grün; Pages-Deploy; Browser-Sicht auf `omegaflow.space/membrane.html`. Offen: `state.lvl` global über beide Aperturen; `MembraneLookup.add_stars` panikt bei Re-Init (Riss, kein Repro ohne WASM/Browser).

### Receiver-Apertur — Sub-Pixel für ALLE Radiatoren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Direktive `span` auf der `at <body>`-Zeile (Mountain).
- **Lage:** (gemessen 2026-10-06, River 109/110) sichtbarer Pfad halb geheilt; SPAN/Apertur-Architektur entschieden (Rat + 6 UI-Modelle). Risse: `SPAN/N` ungemessen; `Aperture` → `span_m`.
- **Blockade:** großer Umbau (per-Fragment `source_contrib`).
- **Braucht:** `span`-Direktive (Mountain); Brücke in `static/membrane.html`; Invarianz-/Energieerhaltungs-Test; danach alle fünf Radiatoren.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon). Wahrheit: `state/zustand/wartend.φ` (`flyby-chain-omni2`, `flyby-chain-kp-def`, `ephemeris-juice-recon`).
- **Lage:** (gemessen 2026-10-06, River 105) OMNI2 26 Zellen `pending`; kp `def` leer; JUICE-recon absent (Wiedervorlage 2026-11-01).
- **Blockade:** externe Kanäle; kein Polling.
- **Braucht:** `flyby_path2_fill`-Lauf lesen + Addendum fortschreiben; Trigger feuern lassen; Δ/σ_recon.

### `1-ui` (Alt-UI-Gruppe, River-Besitz) — nach Mountains Runde schließen
- **Status:** wartend (Mountain) | **Bindung:** eigen (cross-line mountain)
- **Trigger:** Mountains Runde ohne `1-ui`-Bedarf.
- **Lage:** (gemessen 2026-10-07 via `register_lookup --addressed river`) Brücke meldet `group "1-ui" is owned by another client`; Mountain braucht sie gerade noch. Uniform bleiben nur `<line>-ui` (JIT) + `open-weight-ui` (Lock `state/zustand/ui-open-weight.lock`); kein `shared-ui`-Nachfolger.
- **Blockade:** Mountain hält die Gruppe.
- **Braucht:** nach Mountains Round `1-ui` schließen (Bridge); kein Nachfolger.

## An mountain

Origin: river-123.

- **Lizenz-Pending je Netloc (Rat-Verdikt, 2026-10-07, Option (c)):** der Lizenz-Census lebt nirgends als Datei — er wird zur Gate-Zeit aus den getrackten `terms`-Direktiven in `phi/sources.φ` abgeleitet. Jeder der ~160 baum-gemessenen Netlocs ohne `terms`-Direktive trägt eine `pending`-Zeile im Dispositions-Register (`phi/blocked_sources.φ`), Eigentümer du (Quellen-Identität); die Zahl wird live gezählt, nie gespeichert. **Braucht:** die `terms`-Vokabel ist geschlossen (CC-BY-4.0, CC-BY-NC-SA-4.0, ODbL-1.0, PDDL-1.0, PD, own-work, free-open); ein fremder String ist ein Riss. **Hinweis:** `phi/sources.φ` trägt 143 `terms`-Treffer (Rats-Messung), die 162-Zahl ist über Netlocs, nicht über Direktiven.

## An mycelium

Origin: river-123.

- **Lizenz-Census-Generator umhängen (Rat-Verdikt, 2026-10-07, Option (c)):** `license_census.rs` joint zur Gate-Zeit **direkt gegen `phi/sources.φ`** (nicht gegen `state/river/license-census.tsv`; `state/` ist gitignored → CI-Tor strukturell unbaubar). **CI-Drift-Tor:** (i) jeder `terms`-Wert ∈ geschlossener Vokabel; (ii) jeder Quellenblock ohne `terms` bildet auf eine lebende `pending`-Dispositions-Zeile ab (Mountains Teil); (iii) Tree-Count − `terms`-Count = Zahl der `pending`-Lizenz-Zeilen. Alles aus getrackten Dateien — keine zweite Wahrheit. Der `state/`-Pregate bleibt Komplement/Stehender-Pass-Prüfung, nie das Tor. **Riss benannt:** Census 162 vs Baum 160 ist der eingetretene Snapshot-Drift; kein Snapshot mehr.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `tools/measure/src/bin/cgm_lat_partition.rs` (Format-Fix, schließt Stehender-Pass-`format`-Zeile)
- `docs/paper/gic-causal-driver.md` (NUR-Asset-Fakten §6 + header-sha)
- `docs/handover/handover-2026-10-07-river-folge123.md` (neu)
- `docs/handover/archiv/handover-2026-10-07-river-folge122.md` (Move)

## Burn: open 0.0000 · close 0.0250 · cap 0.20 (Operator-Wort 2026-10-07) · Grund: River 123 — Line-Session (deepseek-flash), ein Pass. **Erster Pass:** `--fired river` 2 Punkte (`ozzy` FIRED_UNGEMESSEN, `em-apertur` FIRED), CI am HEAD `queued` (`ci_manage`) — kein grüner Lauf; `--stale` 0; `--addressed river` 2 Blöcke (mountain-264, mycelium-259) gefaltet; `open_points_check` clean (1 word-carried `:62` = known). **Gemessen/gebaut:** `cargo fmt` setzt `cgm_lat_partition.rs` auf die fmt-Form (die 3 format-Diff-Sites :194/:230/:457 geheilt) — `cargo build -p omegaflow-measure --bin cgm_lat_partition` grün; ein versehentlicher rustfmt-Zusatz am fremden `register_lookup.rs` rückgängig gemacht (nur eigene Pfade). `archive_search --sniff`/`--verdict` am NUR-Asset: HTTP 206, 15 551 948 B, sha `9c76f881…` == Register. **Rat (flash, 2026-10-07):** Lizenz-Census-Heimat = Option (c), `phi/sources.φ` als einzige Quelle, 162-pending je Netloc (Mountain), CI-Tor aus getrackten Registern; Verdikt in Handover + `## An mountain`/`## An mycelium`. Kein pro/max-Dispatch, kein Fenster-Edit, kein Send. **Nicht gebaut:** NUR-dB/dt–GIC-Probe (nächster Bau-Atom); `src/archivar/units.rs` + Tree von fremder Session berührt — nicht angefasst.
