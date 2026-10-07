<!--
  title: Handover — River-Folge 123 (2026-10-07)
  session: River-Folge 123
  class: handover
  date: 2026-10-07
  sha256: b2d11db9acc1dbebcd0b0985ef0fe4080082b9df574db5a6ffd81914a6debe5f
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
„…das ist wichtig wir haben mächtige stimmen 1-ui gehört niemandem" | 2026-10-07 | Operator (Session, River 123) — `1-ui` ist **nicht** linien-fremd; die starken UI-Seats sind offen (Bridge-Meldung „owned by another client" ist für den Zugriff kein Verdikt)
Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-07-river-folge122.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Stimmen-Rolle (gemessen 2026-10-07)

Recherche (Wissenschafts-/Forschungslandschaft, Netz) trägt **`voice-deepseek`**; **strikt lokal nur
DeepSeek-flash** (`opencode.json`: alle nicht-DeepSeek-Agenten + alle pro/max deaktiviert); Denken/Urteil
= UI-Frontier. Der **Rat** bleibt Form/Linse (fünf exklusive Perspektiven). Benchmark:
`state/benchmark/2026-10-07-recherche-stimmen.md`.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie; Siegel = Operator-Wort, offen.
- `docs/surveys/survey-2026-10-03-exzellenz-gate.md` — Label geschlossen (`:111`, gemessen 2026-10-07).
- `docs/paper/gic-causal-driver.md` — NUR-Asset-Fakten §6, neue §4.7 (dB/dt–GIC-Relation, r = 0.9448), Zitat-Korrektur Viljanen→Juusola (2026-10-07).

## Offen (aufgeschlüsselt)

### NUR-Asset — 30-Tage-Fenster überlappt die GIC-Reihe nicht (Re-Harvest)
- **Status:** wartend (Mycelium) | **Bindung:** eigen (cross-line mycelium)
- **Trigger:** neuer `image-cdn.yml`-Lauf mit einem Fenster in 1999–2023.
- **Lage:** (gemessen 2026-10-07 via `nur_gic_relation_probe`) das Asset `fmi_image_mag_nur.bin` trägt 30 Tage **2023-12-31…2024-01-30 UTC** (259 199 Sätze, 10 s); `fmi_gic.bin` endet **2023-10-01 UTC** → **0 überlappende Stunden**. Die Relation selbst wurde daher auf dem Halloween-Sturm `2003-10-29…31` gemessen (NUR 10 s × GIC stündlich): 72 alignierte Stunden, Peak-Stunde 2003-10-29T06:00Z |ΔX/10s| = 240.9 nT/10s / |I| = 57.05 A, **Pearson r = 0.9448**, OLS |GIC| = 0.2118·|ΔX/10s| + 1.755 A — eingetragen in Paper §4.7.
- **Blockade:** das manifestierte Asset-Fenster (Mycelium-Ernte).
- **Braucht:** `image-cdn.yml`-Lauf mit `--start` in 1999–2023 (z. B. `20031029`, `--days ≥ 3`) + sha zurück ins Register (`phi/sources.φ:18030`); danach die Relation auf dem Asset selbst reproduzierbar.

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

### `1-ui` — starke UI-Seats, kein Linien-Eigentum (Operator-Wort 2026-10-07)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner (Operator-Wort liegt vor).
- **Lage:** (gemessen 2026-10-07) die Brücke meldet `group "1-ui" is owned by another client`, doch der Operator hat gewortet: **`1-ui` gehört niemandem**; die starken Seats (Claude Sonnet 5.5, GLM, Qwen) sind offen. Praktisch erreicht: dieselben Seats über `river-ui` geöffnet (gleiches Profil) — **Claude Sonnet 5.5** hat die dB/dt–GIC-Methodenfrage beantwortet. Uniform bleiben `<line>-ui` (JIT) + `open-weight-ui`; `1-ui` ist keine linien-fremde Gruppe.
- **Blockade:** keine.
- **Braucht:** keine Schließung — `1-ui` als geteilte Starke-Seat-Gruppe führen; Myceliums „nach Mountain schließen" ist durch das Operator-Wort überholt.

## An mountain

Origin: river-123.

- **Lizenz-Pending je Netloc (Rat-Verdikt, 2026-10-07, Option (c)):** der Lizenz-Census lebt nirgends als Datei — er wird zur Gate-Zeit aus den getrackten `terms`-Direktiven in `phi/sources.φ` abgeleitet. Jeder der ~160 baum-gemessenen Netlocs ohne `terms`-Direktive trägt eine `pending`-Zeile im Dispositions-Register (`phi/blocked_sources.φ`), Eigentümer du (Quellen-Identität); die Zahl wird live gezählt, nie gespeichert. **Braucht:** die `terms`-Vokabel ist geschlossen (CC-BY-4.0, CC-BY-NC-SA-4.0, ODbL-1.0, PDDL-1.0, PD, own-work, free-open); ein fremder String ist ein Riss. **Hinweis:** `phi/sources.φ` trägt 143 `terms`-Treffer (Rats-Messung), die 162-Zahl ist über Netlocs, nicht über Direktiven.

## An mycelium

Origin: river-123.

- **Lizenz-Census-Generator umhängen (Rat-Verdikt, 2026-10-07, Option (c)):** `license_census.rs` joint zur Gate-Zeit **direkt gegen `phi/sources.φ`** (nicht gegen `state/river/license-census.tsv`; `state/` ist gitignored → CI-Tor strukturell unbaubar). **CI-Drift-Tor:** (i) jeder `terms`-Wert ∈ geschlossener Vokabel; (ii) jeder Quellenblock ohne `terms` bildet auf eine lebende `pending`-Dispositions-Zeile ab (Mountains Teil); (iii) Tree-Count − `terms`-Count = Zahl der `pending`-Lizenz-Zeilen. Alles aus getrackten Dateien — keine zweite Wahrheit. Der `state/`-Pregate bleibt Komplement/Stehender-Pass-Prüfung, nie das Tor. **Riss benannt:** Census 162 vs Baum 160 ist der eingetretene Snapshot-Drift; kein Snapshot mehr. *(Deckung: `archive_search`-Recherche 2026-10-07 — W3C PROV, Buneman et al. ICDT 2001, Event Sourcing, dbt compute-from-source; Option (c) ist der Lehrbuchpfad gegen Update-Anomalien.)*
- **NUR-Asset re-harvest (dB/dt–GIC-Kette):** `fmi_image_mag_nur.bin` trägt nur 30 Tage **2023-12-31…2024-01-30 UTC**, `fmi_gic.bin` endet **2023-10-01 UTC** → kein Überlapp. **Braucht:** `image-cdn.yml`-Lauf mit `--start` in 1999–2023 (z. B. `20031029`, `--days ≥ 3`, `--stations NUR`, `--sample-rate 10`) + sha zurück ins Register (`phi/sources.φ:18030`); die Relation ist auf dem Halloween-Sturm bereits gemessen (Paper §4.7, r = 0.9448).

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `tools/measure/src/bin/cgm_lat_partition.rs` (Format-Fix, schließt Stehender-Pass-`format`-Zeile)
- `tools/measure/src/bin/nur_gic_relation_probe.rs` (neu — dB/dt–GIC-Relation, §4.7)
- `docs/paper/gic-causal-driver.md` (NUR-Asset §6, neue §4.7, Zitat-Korrektur)
- `docs/handover/handover-2026-10-07-river-folge123.md` (neu)
- `docs/handover/archiv/handover-2026-10-07-river-folge122.md` (Move)

## Burn: open 0.0000 · close 0.1109 · cap 0.20 (Operator-Wort 2026-10-07) · Grund: River 123 — Line-Session (deepseek-flash), ein Pass. **Erster Pass:** `--fired river` 2 Punkte (`ozzy`/`em-apertur`, CI `queued`/`unread`); `--stale` 0; `--addressed river` 2 Blöcke gefaltet; `open_points_check` clean (1 word-carried). **Format-Fix:** `cargo fmt` auf `cgm_lat_partition.rs` (3 Diff-Sites :194/:230/:457), Build grün; fremder rustfmt-Zusatz `register_lookup.rs` 2× rückgängig gemacht. **Gebaut/gemessen:** `nur_gic_relation_probe.rs` (neuer Bin) — Asset-Probe: NUR 30 Tage 2023-12-31…2024-01-30 UTC, GIC-Ende 2023-10-01 UTC → **0 Überlapp** (Riss); Relation auf Halloween-Sturm 2003-10-29…31 gemessen: 72 h, Peak 240.9 nT/10s / 57.05 A, **r = 0.9448**, OLS 0.2118·|ΔX/10s| + 1.755 → Paper §4.7. **Recherche-Schicht (Operator-Wort):** `general`-Flash-Recherche (2) — (1) Zitat-Riss: der Anker ist **Juusola et al. 2025** (nicht Viljanen), Eq. 43 ist 3-Komponenten-Regression @10 s (CC 0.80) ≠ unser Skalar-Peak; Paper korrigiert; (2) Register-Einzige-Quelle: Option (c) gedeckt durch W3C PROV/Buneman/dbt/Event-Sourcing. **Rat (flash):** Lizenz-Census Option (c). **UI-Kanal (Operator-Wort, „1-ui gehört niemandem"):** starke Seats im selben Profil über `river-ui` befragt — **Claude Sonnet 5.5** (Volltext: Erstautor **Juusola**, Viljanen letzter; Eq. 43 = 3-Komponenten-Fit a_x=−1.69/a_y=−2.73/a_z=−0.23, Fit 29.10. 08:00–31.10. 23:59:50 UT, CC=0.80 auf 2-h-Hold-out), **Qwen3.8-Max** (Juusola bestätigt, Koeffizienten exakt bestätigt; Stunden-Peak kein Standard, official „inflation of r"), **Duck.ai/GPT-6 Luna** (Juusola; Peak-Skalar gültig deskriptiv, keine Transfer-Funktion), **DeepSeek V4 Pro** (Peak-Skalar legitim für Extremwert-Statistik, nicht für Transfermodell; **Riss:** liest das Paper als SECS/3D-Geoelektrik und kann Eq. 43 ohne Volltext nicht reproduzieren — 3 Volltext-Lesungen (Claude/Qwen/flash) bestätigen es; Riss benannt, nicht geglättet). Limitationen: Richtung verloren, Peak-Entkopplung, **GIC-Kette nicht unabhängig (NUR in beiden Größen)**, r-Inflation durch Max-Aggregation, kein Out-of-Sample — alle in Paper §4.7 gefaltet. **Gegengelesen (Operator-Kritik):** danach die **Arbeit gebaut**, nicht nur Dokument. Kein pro/max-Dispatch, kein Fenster-Edit, kein Send. Fremde Baum-Hunks (`units.rs`, `phi/sources.φ`, `commit_gate*`, `AGENTS.md`, `intermagnet_dbdt_compiler.rs`) nicht angefasst.
