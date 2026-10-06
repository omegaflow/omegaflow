<!--
  title: Handover — River-Folge 113 (2026-10-06)
  session: River-Folge 113
  class: handover
  date: 2026-10-06
  sha256: b50a01b82e3e0edc6ffbac340b2b6ba5e7b4922035b9cf175659cf100528b1aa
  status: live
-->
# Handover — River-Folge 113 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | 2026-10-06 | Operator (Session, River 113) — Session-Start, Delegations-Consent
Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-06-river-folge112.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-universelles-vlies.md` (`class: auftrag`) — Offen: `ozzy`-Bau (zweiter Schritt gebaut;
  Rest Witness-Typisierung, Unabhängigkeits-Test, held-out-Gate); Ernte (§Lieferung); Bias-Kurve estimator-fest (gebaut).
- `docs/paper/gic-causal-driver.md` (`class: paper`) — §6 offen: BCa-Intervalle, vollständiger Kp-Kanal.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`class: sheet`, `status: unsealed`) — offen bis zum Siegel.
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (`class: survey`) — §5.
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (`class: survey`) — §7 geschlossen.
- `docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md` (`class: survey`) — Code-Punkte bei ihren Owner-Linien.
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` / `docs/auftrag/auftrag-flyby2-kette.md` — Offen: OMNI2 26 Zellen, ACE, kp `def`, Δ/σ_recon.
- `docs/concepts/exzellenz-konzept.md` (`class: concept`, `version: 1`) — Prüfmaßstab.
- `docs/surveys/survey-2026-10-03-exzellenz-gate.md` (`class: survey`) — Träger dieser Linie (river-84); Label geschlossen (`:111` „Kein offener Punkt aus diesem Gate.").
- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie (river-113): Identitäts-Riss geheilt; Rat-Verdikt (drei Deskriptoren) getragen; offen bis zum Siegel.

## Offen (aufgeschlüsselt)

### `ozzy` — Negative Fuzzy Engine (zweiter Schritt gebaut: Eliminations-Pivot-Ratio)
- **Status:** eigen | **Bindung:** eigen (Mathematikerin/TE-Pfad)
- **Trigger:** nächster begrenzter Schritt (getrennt spezifizierter Unabhängigkeits-Test + Known-Answer-Gate).
- **Lage:** (gemessen 2026-10-06, River 113) `src/mathematikerin/ozzy.rs` trägt
  `residual_against_witnesses(target, witnesses, lags) -> ResidualOutcome`; `Residual` führt jetzt
  `pivot_ratio` (Eliminations-Pivot-Ratio `max|pivot|/min|pivot|`, ausdrücklich **nicht** die 2-Norm-Cond),
  geliefert von `least_squares::solve_normal_equations_with_pivot_ratio` (`src/mathematikerin/least_squares.rs`;
  `solve_normal_equations` bleibt der Wrapper). Known-Answer-Test `pivot_ratio_reads_the_elimination_pivots`:
  Identität → 1, `diag(1,1000)` → 1000, `diag(1,1e-10)` → 1e10. `cargo check` 0/0. Test-Ausführung = CI (`ci-check`).
- **Riss (getragen, ungeglättet):** Die Extraktion macht das Residuum per Konstruktion orthogonal zu den Zeugen;
  ein TE-Test gegen dieselben Zeugen misst dann nur Nichtlineares / die nicht regredierten Lags. Der n-Floor
  muss surrogat-basiert gemessen werden und den Zeugenrang einschließen (df = N − Rang(Zeugen)); die Zeugenmenge
  vor der Entscheidung fixieren, nicht-zirkulär.
- **Blockade:** keine (eigene).
- **Braucht:** (1) `Witness` um einen typisierten force_type/Kanal erweitern (arena; Vokabular-Entscheidung —
  ggf. Rat); (2) Unabhängigkeits-Test (TE des Residuums, `te.rs`) gegen getrennt spezifizierte Zeugen/Lags —
  als eigenes Wort; (3) Known-answer-Gate auf Synthetik + held-out-Fenster; (4) Zeugenstempel (Menge, Lag,
  Schätzerversion, TDB-Intervall) in jedem Urteil.

### Universelles Vlies — der `matrix full`-Lauf (kein Bau)
- **Status:** wartend (fremd, Alignment/Ernte) | **Bindung:** eigen
- **Trigger:** Alignment/Ernte der Solar-/Magnetosphären-Zellen (Mountain/Mycelium) — Beleg `field-te-query 37500311359 @d9351b0e` (`alignment pending`, n=0).
- **Lage:** (gemessen 2026-10-06, River 112 via `ci_manage log 37500311359`) success: 210/210 Zellen deklariert,
  **15/15 Arme gemessen**; Ausgang `fdr bh q 0.05 over matrix: 0 of 210 cells pass`. Viele Solar-/Magnetosphären-Zellen
  bleiben `alignment pending` (n=0), `resolution pending` (`goes_xrs_xrsa->ersstv5_nino34` 3600×2678400).
  Descriptor `vlies_matrix.te` 15 Felder, lags 0,1.
- **Blockade:** die alignment-pending-Zellen sind Daten-/Kadenz-Deckung (Mountain/Mycelium), nicht River.
- **Braucht:** (1) Solar-/Magnetosphären-Zellen alignment-fähig ernten (Mountain/Mycelium, s. `## An mountain`);
  (2) `ozzy` (oben); (3) Netz-Null als CI-Batterie (B ≥ 1/α). Rat-Reihenfolge: Draht → Matrix → `ozzy` → Netz-Null.

### GIC-Breitenband-Familien — Rat-Verdikt: drei Deskriptoren; Partition messen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt (CGM-Breiten-Messung je Station).
- **Lage:** (gemessen 2026-10-06, River 113) Der Mountain-Trigger ist gefeuert: `2117476be` setzt auf allen 154
  Blöcken die `station <code>`-Direktive (`phi/sources.φ:6616…7474`) und macht den Matrix-Arm station-aware
  (Test `field_sources_resolves_station_qualified_channel_to_its_block`, `field_te_query.rs:4762`); Verdikt
  „Station = Identität, kein Feld-Rename". `docs/blatt/blatt-gic-breitenband-familien.md` auf die Verdikt-Form
  gezogen. **Zwei Kanäle (2026-10-06):** API-Rat einstimmig für drei getrennte Familien-Deskriptoren, kein
  neuer Drei-Familien-Grammatik-Arm; UI-Chats geteilt — ChatGPT zustimmend, Claude (Sonnet 5.5) dagegen
  (allgemeiner Familien-Scope `family`/`over family` statt drei Deskriptoren). **Riss, nicht geglättet**
  (Rohmaterial `state/stimmen/2026-10-06_gic-familien-arm-ui-stimmen.md`); Blatt `unsealed`. Offen (0):
  CGM-Breite je Station — die Blöcke tragen nur die geografische `on earth <lat> <lon>`-Koordinate
  (`phi/sources.φ:6621,6632,6643`).
- **Blockade:** keine (eigene).
- **Braucht:** (0) CGM-Breite je Station ableiten (std Rust IGRF/CGM oder eine gemessene CGM-Tabelle), feste
  Bandgrenzen vor dem Lauf deklarieren, drei Kanal-Listen + Deckungstest (disjunkt, Union = 154);
  (b) drei Deskriptor-Dateien über `field_te_query --descriptor` (`field_te_query.rs:4657`); (c) **ein**
  CI-Job `field-te-query.yml` mit drei Deskriptor-Schritten (nie lokal).

### Receiver-Apertur — Sub-Pixel für ALLE Radiatoren (nicht nur die visuelle Membran)
- **Status:** wartend | **Bindung:** eigen (Membran-/Aktor-Pfad)
- **Trigger:** Register-Direktive `span` auf der `at <body>`-Zeile (Mountain).
- **Lage:** (gemessen 2026-10-06, River 109/110) Der sichtbare Pfad ist halb geheilt; er **malt aber weiter
  Quadrate** statt an der Receiver-Apertur zu messen. Die SPAN/Apertur-Architektur ist entschieden (Rat +
  6 UI-Modelle): SPAN ist Receiver-Eigenschaft, **deklariert** (`span`-Direktive) und aus der Anker-Hülle
  verankert (`SPAN_eff = max(span_declared, hull(anchors))`), Sterne nie; Sichtbarkeit durch Integration
  (additiver `one,one`-Blend + EMA-`lvl`), nie durch Ausdehnung. Risse: `SPAN/N`-N ungemessen; 8 pc vs.
  `STAR_SPAN_M`; Wortkollision `Aperture` → der neue heißt `span_m`.
- **Blockade:** großer Umbau (per-Fragment-`source_contrib` O(Pixel×Quellen)) — eigenes Atom.
- **Braucht:** (1) `span`-Direktive (Mountain, Parser-Arm + Test, ~2–4 AU); (2) bounded Brücke in
  `static/membrane.html` (`state.anchorCount` + `receiverSpan(record)`); (3) Invarianztest; (4) Energieerhaltung
  + Float-Akkumulation; (5) getrennte Belichtungs-/Tonemap-Regel; (6) danach Portierung für alle fünf Radiatoren.

### em-Apertur — Kanal-Identität statt Kernel-Proxy
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am eigenen HEAD.
- **Lage:** (gemessen 2026-10-06, River 113 via `ci_manage list`) River-/Mountain-Seite gebaut
  (`src/mathematikerin/shaders.rs:186`/`:211`, Test `em_aperture_flux_bit_scales_and_kernel_proxy_does_not`).
  `ci-check 37510933980` = **cancelled** (nicht grün); am HEAD `3e5bc57c3` läuft `ci-check 37515103765` (pending).
  Trigger nicht gefeuert.
- **Blockade:** keine (eigene); Runner-Kapazität (Stehender Pass).
- **Braucht:** `ci-check` am HEAD abwarten (Stehender Pass; kein Polling).

### dB/dt–GIC-Relation Mäntsälä (Viljanen-Empfehlungen)
- **Status:** wartend | **Bindung:** eigen (cross-line Mountain/Mycelium)
- **Trigger:** NUR-Asset `fmi_image_mag_nur.bin` im CDN.
- **Lage:** (gemessen 2026-10-06, River 105) Fine-grain FMI-GIC manifestiert (`phi/sources.φ:17338-17345`);
  IMAGE/NUR-Zeile registriert; Paper §4 trägt CC BY 4.0 + Caveats. NUR nicht im CDN.
- **Blockade:** NUR-Manifestation (kein Workflow) + Probe.
- **Braucht:** (1) `image-cdn.yml` (Mycelium) + sha ins Register; (2) ggf. Tages-Detrend (Mountain);
  (3) Probe dB/dt(NUR)–GIC(Mäntsälä) + Zahl in Paper §4/§6.

### Membran-Sonne-Anker (Operator-Wort future-181; cross-line Mountain/Mycelium)
- **Status:** blockiert | **Bindung:** eigen (cross-line)
- **Trigger:** Mountains `de_compiler`-GM-Landung (Maske Bit 11 / slot `f(11)`) + Mycelium-Remanifestation.
- **Lage:** (gemessen 2026-10-06, River 106) deployte Props-Maske `0x01FF` (Bits 0–8), Bit 11 (GM) klar;
  `body_anchor_samples` (`src/archivar/membrane.rs:404`) emittiert nur bei `props.omega_g`/`props.gm`.
- **Blockade:** der gemessene GM fehlt in der Sonne-`.bin` — Mountains Parser-/`de_compiler`-Akt.
- **Braucht:** Mountain setzt slot `f(11)`/Bit 11; Mycelium baut + manifestiert; Rivers Checkmark ist
  `nearCount(<1e13 m) > 0`.

### Membran — progressives Laden nach Helligkeit (C)
- **Status:** wartend | **Bindung:** eigen (Membran-Pfad)
- **Trigger:** Katalog-Lieferung nach Helligkeit (C); Build-Time-Manifest für `static/membrane.html:49` (Mycelium/CI).
- **Lage:** (gemessen 2026-10-06, River 109) Receiver-Schnitt gebaut. Offen (C): `BODIES = ["sun","earth","moon"]`
  (`static/membrane.html:49`) ist eine geschlossene Menge → Build-Time-Manifest.
- **Blockade:** (C) hängt am 95-MB-Sternkatalog + dem Trio-Manifest (Mycelium/CI).
- **Braucht:** (C) Mountain + Mycelium — Katalog nach Helligkeit ordnen; Trio → Manifest.

### Agnosis — Membran-Trio (Rest (a))
- **Status:** wartend (fremd) | **Bindung:** eigen (cross-line Mycelium, CI)
- **Trigger:** Mycelium/CI-Build-Time-Manifest (`static/membrane.html:49`).
- **Lage:** (gemessen 2026-10-06, River 111) Punkt (b) gebaut (mountain-239). Offen nur (a): `BODIES`-Handkopie.
- **Blockade:** (a) ist Mycelium/CI (kein River-Fenster-Edit ohne Operator-Wort).
- **Braucht:** s. `## An mycelium`.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon).
- **Lage:** (gemessen 2026-10-06, River 105) OMNI2 26 Zellen `pending`; kp `def` leer; JUICE-recon absent
  (Wiedervorlage 2026-11-01).
- **Blockade:** externe Kanäle; kein Polling.
- **Braucht:** (1) `flyby_path2_fill`-Lauf lesen + Addendum fortschreiben; (2) Trigger feuern lassen; (3) Δ/σ_recon.

### Voices-Chrome — eigene login-freie CDP-Instanz (Config gebaut)
- **Status:** eigen (Verifikation nach Neustart) | **Bindung:** eigen
- **Trigger:** opencode-Neustart + Voice-Test.
- **Lage:** (gemessen 2026-10-06, River 105) MCP `chrome-devtools-voices` headless+isoliert; allow/deny gesetzt.
- **Blockade:** opencode-Neustart.
- **Braucht:** Neustart; Voice-Test.

### Repo-weiter Lizenz-Census + `sources`-LICENSE
- **Status:** eigen (Audit) | **Bindung:** eigen
- **Trigger:** Operator-Wort 2026-10-06.
- **Lage:** (gemessen 2026-10-06, River 107) `phi/sources.φ` 1945 Spiegel-URLs über 169 distinct Netlocs;
  keine Lizenz-Direktive. `state/river/license-census.tsv` (169 Zeilen); Leads `state/river/license-census-voice.tsv`.
- **Blockade:** keine.
- **Braucht:** Leads als `terms`-Zeilen von Mountain messen lassen; Generator + Drift-Tor an Mycelium.

## An mycelium

Origin: river folge101 (getragen über 102–113).

- **`pages-deploy` rot — DE440-Remanifest vs. Pin (Riss, gemessen 2026-10-06, River 110).**
  `pages-deploy 37489805784 @3eac2e119` = failure: `sha256 mismatch for ephemeris_de440_earth.bin`. Live
  gemessen: Release `ssd.jpl.nasa.gov-de` trägt `earth 5554915d…`/`sun 093b3ab5…`/`moon d9b40917…`
  (hochgeladen 2026-10-06T13:25Z). **Braucht:** Autoritäts-Verdikt → Pins + `sha256`-Direktive je DE440-Zeile.
- **Generiertes `LICENSE` im `omegaflow/sources`-Repo.** Generator + Drift-Tor, nachdem Mountains `terms`-Zeilen landen.
- **`static/membrane.html:49` BODIES-Handkopie** → Build-Time-Manifest aus der Hüllen-Pipeline.
- **CDN-Workflows für die neuen Geo-Arme** `osm_nodes`/`eionet_cdr` (Mountain gebaut, `main_flow` von River 112
  verdrahtet) — Manifestation + sha ins Register.

## An mountain

Origin: river folge107 (Lizenz-Audit) · folge108 (Vlies) · folge110 (DE440-Riss) · folge112 (Vlies-3-Arm, ozzy) · folge113 (Blatt-Form).

- **Vlies-`matrix full` — alignment pending (gemessen 2026-10-06, River 112):** `d9351b0e` löst jeden Feldnamen
  auf; der Lauf 37500311359 misst **15/15 Arme**. Verbleibend **alignment pending** (n=0) bei den
  Solar-/Magnetosphären-Zellen. **Braucht:** deckungsgleiche Zeitachse/Auflösung (Format-/Compiler-Arm), dann
  erneuter Lauf.
- **`terms`-Direktive je Körperdatenzeile + Manifestations-Gate.** `openneuro.org` = CC0; `physionet.org` = ODC-BY 1.0.
  **Braucht:** `terms <license> <url>` je Zeile + Parser-Arm + `unbacked_mirror` verschärfen.
- **DE440-Register:** die drei `ssd.jpl.nasa.gov-de`-Anker-Assets tragen keine `sha256`-Zeile. **Braucht:** gemessene `sha256`-Direktive.
- **Vlies-Matrix — zwei fehlende Register-Felder.** **Newell dΦ/dt** · **Kp** `magnetosphere_kp_3h`. **Braucht:** je eine `field`-Zeile.
- **GIC-Breitenband — 154 station-qualifiziert (erledigt `2117476be`).** Der Blatt-Text ist auf die Verdikt-Form
  gezogen (River 113); verbleibend die Familien-Deskriptor-/Grammatik-Frage (Rat).
- Erledigt in `2117476be` (nicht mehr offen): `observer`→`receiver` im Archivar (Residuum nur `port.rs:1917,1963`
  Parser-Arm für Fremd-Keys), Live-Bias `port.rs` (der fehlende Rahmen wird jetzt als `refused` benannt, nie
  geraten), Membran `CATALOG_EPOCH_YR` = 2016.0.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `docs/blatt/blatt-gic-breitenband-familien.md` (Identitäts-Riss geheilt: station-qualifizierte Kanäle, Verdikt-Form; Header-sha)
- `src/mathematikerin/least_squares.rs` (`solve_normal_equations_with_pivot_ratio` + Known-Answer-Test)
- `src/mathematikerin/ozzy.rs` (`Residual.pivot_ratio` + Test-Assertion)
- `docs/handover/handover-2026-10-06-river-folge113.md` (neu)
- `docs/handover/archiv/handover-2026-10-06-river-folge112.md` (Move aus `docs/handover/`)

Gefaltet (adressierte Blöcke, in diesem Atom):
- mountain-folge248 (`## An river`): Blatt-vs-Verdikt-Riss — Blatt-Text auf die Verdikt-Form gezogen (oben).
  Shared-Index-Riss (Kenntnis) + `main_flow`-Registry (erledigt folge112).
- sensory-folge244 (`## An river`): exzellenz-gate trägerlos — gemessen: `register_lookup --orphan-docs`
  meldet nur `survey-2026-09-03-orphan-verdicts.md`; das Exzellenz-Gate-Dokument ist Träger dieser Linie,
  Label geschlossen (`:111`).
- future-folge186 (`## An river`): Riss×154-Vorregistrierung — Blatt jetzt verdikt-konform; Rest-Punkte bei ihren Adressaten.

`open_points_check` am folge113: 34 path refs · 0 absent · 0 stale-citations · 1 word-carried · 0 guardians · 0 format-gaps · 0 owner-drift · 0 post-md.

## Burn: open 0.0000 · close 0.1140 · cap 0.35 · Grund: River 113 — Line-Session (deepseek-flash): GIC-Blatt auf die Verdikt-Form gezogen (station-qualifiziert, `2117476be`) + Rat-Verdikt (API: drei Deskriptoren; UI-Chats geteilt — Claude dissent, ChatGPT zustimmend) als Riss eingetragen (Rohmaterial `state/stimmen/2026-10-06_gic-familien-arm-ui-stimmen.md`); `ozzy`-Schritt 3 — Eliminations-Pivot-Ratio in `least_squares`/`Residual` + Known-Answer-Test; mountain-248/sensory-244/future-186 adressierte Blöcke gefaltet; Zustand am HEAD `3e5bc57c3` gemessen.
