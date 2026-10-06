<!--
  title: Handover — River-Folge 114 (2026-10-06)
  session: River-Folge 114
  class: handover
  date: 2026-10-06
  sha256: d0bf0de3b6291b534b04c6b9dc72e01a642a5104fb07088743084e4aa6d95eca
  status: live
-->
# Handover — River-Folge 114 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie in einem Pass." | 2026-10-06 | Operator (Session, River 114) — Session-Start, Delegations-Consent
„wir haben 5 voices und 7 ui chats bitte befrage alle" | 2026-10-06 | Operator (Session, River 114) — die Membran-Apertur-Frage an die 5 API-Stimmen + 7 UI-Chats; 12/13 (b)
„kannst du das nicht selbst mit dem browser link testen?" | 2026-10-06 | Operator (Session, River 114) — Browser-Test; bestätigt schwarz, Ursache gemessen
„wer hat das geamch? ich hoffe ihr habt nicht unser enclosure lemma zerstört" | 2026-10-06 | Operator (Session, River 114) — Herkunft gemessen (river 91), Lemma intakt
„ja aber bitte den rat und die chat ui stimmen" | 2026-10-06 | Operator (Session, River 114) — Rat + API + UI zum Fix; Verdikt (a) native Parität
„ich möchte übrigens dass die membran steht bevor wir uns irgendwo bewerben … und sie stehen vor der sonne" | 2026-10-05 | Operator (Future 181, gefaltet) — das Fenster-Wort der Startansicht
Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-06-river-folge113.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie: Identitäts-Verdikt getragen; offen bis zum Siegel (Bandgrenzen-Slot, cgm_lat).
- `docs/surveys/survey-2026-10-03-exzellenz-gate.md` — Label geschlossen (`:111`); Träger dieser Linie.
- `state/stimmen/2026-10-06_membran-apertur-alle-stimmen.md` — Rohmaterial der 12-Stimmen-Befragung (privat, gitignored).

## Offen (aufgeschlüsselt)

### `ozzy` — Negative Fuzzy Engine (dritter Schritt gebaut: Eliminations-Pivot-Ratio)
- **Status:** eigen | **Bindung:** eigen (Mathematikerin/TE-Pfad)
- **Trigger:** nächster begrenzter Schritt.
- **Lage:** (gemessen 2026-10-06, HEAD) `src/mathematikerin/ozzy.rs` `residual_against_witnesses` + `Residual.pivot_ratio`; `least_squares::solve_normal_equations_with_pivot_ratio`; Known-Answer-Test grün. `cargo check` 0/0 (CI `ci-check`).
- **Riss (getragen):** Die Extraktion macht das Residuum per Konstruktion orthogonal zu den Zeugen; ein TE-Test gegen dieselben Zeugen misst nur Nichtlineares / die nicht regredierten Lags. Der n-Floor muss surrogat-basiert sein und den Zeugenrang einschließen (df = N − Rang(Zeugen)); die Zeugenmenge vor der Entscheidung fixieren.
- **Blockade:** keine (eigene).
- **Braucht:** (1) `Witness` typisierter force_type/Kanal (Vokabular — Rat); (2) Unabhängigkeits-Test (TE des Residuums, `te.rs`) gegen getrennt spezifizierte Zeugen/Lags — als eigenes Wort; (3) Known-Answer-Gate auf Synthetik + held-out-Fenster; (4) Zeugenstempel in jedem Urteil.

### Membran-Startansicht — zwei Aperturen (gebaut; Parity-Fix gebaut, CI-Verifikation offen)
- **Status:** eigen (Fix gebaut; Trigger feuert bei CI-grün + Deploy) | **Bindung:** eigen
- **Trigger:** `ci-check` grün am Fix-HEAD; danach WASM-Build + Pages-Deploy.
- **Lage:** (gemessen 2026-10-06, River 114, Browser-Isolation + Baum + Rat/Kanäle) Deploy trägt den Zwei-Apertur-Bau noch nicht. Ursache der 0 Anker: `wasm.rs:77` stempelte die Anker mit `t2` (Epoche = Query-Zeit, River 91 `638a11bb9`) → `age=0` → Lemma-Dilatation fällt auf `extent`; plus das Zusatz-Gate `exact = extent+pad` (`spatial.rs:945-948`), das die Präsenz im Körper verlangte. **Kein Lemma-Eingriff** — der Geist der Fixe ist native Parität. **Befragt: Rat (einstimmig) + 5 API-Stimmen (4× a) + UI (Claude, Qwen, GLM, Kimi, MiMo → a) = Option (a): native Parität.** **Gebaut:** `all_body_anchor_samples(eph)` nimmt je Körper `body_record_epoch` (`motion.rs:109`, wie `main_flow.rs:480`); `wasm.rs` ruft ohne `t2`; das Zusatz-Gate in `query_hash` trägt dieselbe `reach`-Messung wie der Anker-Gate (native `record_in_enclosure`-Form, `fetch.rs:521`). Known-Answer-Test `test_body_anchor_reaches_ssb_presence_by_signal_cone` (`tests.rs`). `cargo check` + `cargo fmt` 0/0. Erreichte Kanäle: Rohmaterial `state/stimmen/2026-10-06_membran-apertur-alle-stimmen.md` + `…-fix-…` (s. u.).
- **Riss (getragen, ungeglättet):** (i) Der Rat-Vertrag 2026-10-04 (River 91) wurde nie wirksam — die Epoche `t2` nullte die Dilatation; (ii) **Duck.ai und Nemotron-UI fehlen** in der Fix-Befragung: die Operator-Tabs wurden parallel von einer anderen Session benutzt (fremde „RTSW-Join/GIC"-Fragen erschienen), Nemotrons Modell-Switch schlug fehl — benannt, nicht geglättet; (iii) `MembraneLookup.add_stars` panikt bei Re-Init; (iv) `extent > 0` als Apertur-Diskriminator (deklarierte Apertur gefordert); (v) Deploy hängt (Pages-Deploy rot, DE440-Pin).
- **Blockade:** keine (eigene); Verifikation braucht CI (WASM-Build + Testlauf).
- **Braucht:** `ci-check`/`wasm-parity` am Fix-HEAD; danach Deploy + Browser-Sicht. Offen: `state.lvl` global über beide Aperturen.

### GIC-Breitenband-Familien — Design + Messschritt offen
- **Status:** operator-gebunden (Design) / wartend (cgm_lat) | **Bindung:** eigen + operator (Queue)
- **Trigger:** Operator-Wort (Bandgrenzen, voller Pool vs. drei Pools) + CGM-Messung je Station.
- **Lage:** (gemessen 2026-10-06) `station <code>` auf 154 Blöcken (`2117476be`); Identitätsverdikt „Station = Identität"; sechs Stimmen → Option (c) geschichtet (`family` = FDR-Gruppe, `cgm_lat` eigene Registergröße, Scope `fdr … over family`). Blatt `unsealed`; der Bandgrenzen-Slot ist **offen** (`blatt-gic-breitenband-familien.md:50`).
- **Blockade:** keine gemessene CGM-Breite im Register (Geomagnetic-Latitudes-Services `declined`, `phi/declined_sources.φ:3328-3340`).
- **Braucht:** (0) neue CGM-Route + `cgm_lat` lokal messen; (1) `fdr … over family`-Scope; (2) Operator-Design; (3) CI-Job `field-te-query.yml`.

### Universelles Vlies — der `matrix full`-Lauf (kein Bau)
- **Status:** wartend (fremd, Alignment/Ernte) | **Bindung:** eigen
- **Trigger:** Alignment/Ernte der Solar-/Magnetosphären-Zellen (Mountain/Mycelium) — Beleg `field-te-query 37500311359 @d9351b0e` (`alignment pending`).
- **Lage:** (gemessen 2026-10-06, River 112) `field-te-query 37500311359 @d9351b0e`: 210/210 Zellen, 15/15 Arme; `0 of 210 cells pass`; viele Zellen `alignment pending` (n=0).
- **Blockade:** Daten-/Kadenz-Deckung (Mountain/Mycelium).
- **Braucht:** alignment-fähige Zellen; `ozzy`; Netz-Null als CI-Batterie.

### Receiver-Apertur — Sub-Pixel für ALLE Radiatoren
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Register-Direktive `span` auf der `at <body>`-Zeile (Mountain).
- **Lage:** (gemessen 2026-10-06, River 109/110) sichtbarer Pfad halb geheilt; SPAN/Apertur-Architektur entschieden (Rat + 6 UI-Modelle). Risse: `SPAN/N` ungemessen; `Aperture` → `span_m`.
- **Blockade:** großer Umbau (per-Fragment `source_contrib`).
- **Braucht:** `span`-Direktive; Brücke in `static/membrane.html`; Invarianz-/Energieerhaltungs-Test; danach alle fünf Radiatoren.

### em-Apertur — Kanal-Identität statt Kernel-Proxy
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` grün am eigenen HEAD.
- **Lage:** (gemessen 2026-10-06, River 113/114 via `ci_manage list`) River-/Mountain-Seite gebaut (`src/mathematikerin/shaders.rs:186`/`:211`). Jüngster `ci-check 37523680226 @20:04Z` = in_progress (Trigger nicht grün).
- **Blockade:** keine (eigene); Runner-Kapazität.
- **Braucht:** `ci-check` am HEAD abwarten (Stehender Pass; kein Polling).

### dB/dt–GIC-Relation Mäntsälä (Viljanen-Empfehlungen)
- **Status:** wartend | **Bindung:** eigen (cross-line)
- **Trigger:** NUR-Asset `fmi_image_mag_nur.bin` im CDN.
- **Lage:** (gemessen 2026-10-06, River 105) Fine-grain FMI-GIC manifestiert; NUR nicht im CDN.
- **Blockade:** NUR-Manifestation + Probe.
- **Braucht:** `image-cdn.yml` (Mycelium); ggf. Tages-Detrend (Mountain); Probe + Zahl in Paper §4/§6.

### Membran-Sonne-Anker (Operator-Wort; cross-line)
- **Status:** blockiert | **Bindung:** eigen (cross-line)
- **Trigger:** Mountains `de_compiler`-GM-Landung (Maske Bit 11) + Mycelium-Remanifestation.
- **Lage:** (gemessen 2026-10-06, River 106) deployte Maske `0x01FF`; Bit 11 klar; `body_anchor_samples` (`src/archivar/membrane.rs:404`) emittiert nur bei `props.omega_g`/`props.gm`.
- **Blockade:** der gemessene GM fehlt in der Sonne-`.bin`.
- **Braucht:** Mountain slot `f(11)`/Bit 11; Mycelium baut + manifestiert; Rivers Checkmark `nearCount(<1e13 m) > 0`.

### Membran — progressives Laden nach Helligkeit (C)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Katalog-Lieferung nach Helligkeit (C); Build-Time-Manifest für `static/membrane.html` (Mycelium/CI).
- **Lage:** (gemessen 2026-10-06, River 109) Receiver-Schnitt gebaut; `BODIES` ist eine geschlossene Handkopie.
- **Blockade:** 95-MB-Sternkatalog + Trio-Manifest (Mycelium/CI).
- **Braucht:** Katalog nach Helligkeit ordnen; Trio → Manifest.

### Agnosis — Membran-Trio (Rest (a))
- **Status:** wartend (fremd) | **Bindung:** eigen (cross-line Mycelium, CI)
- **Trigger:** Mycelium/CI-Build-Time-Manifest (`static/membrane.html:51` `BODIES`).
- **Lage:** (gemessen 2026-10-06, River 111) Punkt (b) gebaut (mountain-239); offen nur (a) `BODIES`-Handkopie.
- **Blockade:** Mycelium/CI (kein River-Fenster-Edit ohne Operator-Wort).
- **Braucht:** s. `## An mycelium`.

### Flyby-Kette — OMNI2, kp `def`, JUICE-recon
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Kanal-Verfügbarkeit (OMNI2-Merge-Lag, GFZ `def`-Release, ESOC JUICE-recon).
- **Lage:** (gemessen 2026-10-06, River 105) OMNI2 26 Zellen `pending`; kp `def` leer; JUICE-recon absent (Wiedervorlage 2026-11-01).
- **Blockade:** externe Kanäle; kein Polling.
- **Braucht:** `flyby_path2_fill`-Lauf lesen + Addendum fortschreiben; Trigger feuern lassen; Δ/σ_recon.

### Voices-Chrome — eigene login-freie CDP-Instanz (Config gebaut)
- **Status:** eigen (Verifikation nach Neustart) | **Bindung:** eigen
- **Trigger:** opencode-Neustart + Voice-Test.
- **Lage:** (gemessen 2026-10-06, River 105) MCP `chrome-devtools-voices` headless+isoliert; allow/deny gesetzt.
- **Blockade:** opencode-Neustart.
- **Braucht:** Neustart; Voice-Test.

### Repo-weiter Lizenz-Census + `sources`-LICENSE
- **Status:** eigen (Audit) | **Bindung:** eigen
- **Trigger:** Operator-Wort 2026-10-06.
- **Lage:** (gemessen 2026-10-06, River 107) `phi/sources.φ` 1945 Spiegel-URLs über 169 Netlocs; keine Lizenz-Direktive. `state/river/license-census.tsv` (169 Zeilen).
- **Blockade:** keine.
- **Braucht:** Leads als `terms`-Zeilen (Mountain); Generator + Drift-Tor (Mycelium).

## An future

Origin: future-186 (gefaltet) · river-114.

- **Membran-Startansicht: Bau steht, im Browser gemessen aber wirkungslos.** `static/membrane.html` trägt
  die zwei Aperturen (Körper-Anker vs. Stern-Katalog); der Browser-Test zeigt jedoch, dass `lookup.query`
  am SSB **keine Anker-Records mit `extent > 0`** liefert (8 Records, alle `extent == 0`) → schwarz.
  Der LOCK-Trigger „Startansicht zeigt die Sonne" **kann noch nicht feuern**; der Blocker ist die
  Anker-Sichtbarkeit im Feld (Archivar/Lookup), nicht die Skala. Zudem ist der Live-Deploy veraltet
  (Pages-Deploy hängt) und `add_stars` panikt bei Re-Init.
  Verifikation = Operator-Sicht auf `omegaflow.space/membrane.html`.
- **`omega.rs:878` (Volume-Bin „earth" hart) gemessen:** kein `"earth"`-Literal in `src/mathematikerin/omega.rs`
  (gemessen 2026-10-06, River 114 via `sgrep '"earth"' src/mathematikerin/omega.rs`) — der adressierte Punkt ist veraltet.
- **`static/membrane.html:43` (Trio-Handkopie)** ist heute ein Kommentar; die `BODIES`-Handkopie liegt `:51` (s. `## An mycelium`).

## An mountain

Origin: mountain-249 (gefaltet) · river-107/108/110/112/113.

- **`main_flow.rs` Geo-Dispatch-Registry — erledigt.** `osm_nodes`/`eionet_cdr` stehen bereits
  (`main_flow.rs:4155-4156`, Committet in `f9a204cf0`, river 112). Der adressierte Punkt ist erledigt.
- **GIC-Breitenband-Blatt — gemessener Riss.** Der adressierte Satz, das Blatt trage „die konkreten
  Bandgrenzen + den Framename", trifft nicht zu: `docs/blatt/blatt-gic-breitenband-familien.md:50`
  nennt die Grenzen ausdrücklich einen **offenen Slot** (Operator-Design, s. `:150`); der Framename
  `geomag_lat` ist eine Entscheidung der sechs Stimmen, kein gemessener Wert. Blatt bleibt `unsealed`.
- **Vlies-`matrix full` — alignment pending** (s. `## Offen`): Format-/Compiler-Arm.
- **Membran-Query: Body-Anker erreichen das Feld nicht — Ursache gemessen (2026-10-06, River 114).**
  `lookup.query` mit übersprungenen Sternen liefert **0 Records**, obwohl `load_ephemeris` für
  sun/earth/moon `true` liefert. **Herkunft: River 91 (`638a11bb9`, 2026-10-04, `wasm.rs:77`).**
  Das Enclosure-Lemma selbst ist intakt (`reach_signal + extent + enclosure_rho`, `spatial.rs:928-930`).
  Die Anker-Epoche `t2` nullt die Dilatation: `age = 0` → `signal_reach = c·0 = 0` → `reach = extent + pad`;
  das Gate (`:935`) lässt einen Körper nur zu, wenn die Präsenz **innerhalb seines Radius** liegt.
  Am SSB liegt die Sonne ~0.005 AU von ihrem Zentrum (Radius 0.00465 AU). Kein Test deckt die
  Membran-Anker-Admission (nur `tests.rs:13034` prüft die Felder).
  **Braucht/erledigt:** Rat + 5 API + UI → **(a) native Parität**, gebaut in `all_body_anchor_samples` (`body_record_epoch`),
  `wasm.rs` (ohne `t2`), `query_hash` (Gate trägt `reach`); Known-Answer-Test `test_body_anchor_reaches_ssb_presence_by_signal_cone`.
  **Braucht:** WASM-Build im CI + Browser-Sicht.
- **`terms`-Direktive je Körperdatenzeile:** `openneuro.org` = CC0; `physionet.org` = ODC-BY 1.0.
- **DE440-Register:** die drei `ssd.jpl.nasa.gov-de`-Anker-Assets tragen keine `sha256`-Zeile.

## An mycelium

Origin: river folge101 (getragen über 102–114).

- **`pages-deploy` rot — DE440-Remanifest vs. Pin (Riss, gemessen 2026-10-06, River 110).**
  `sha256 mismatch for ephemeris_de440_earth.bin`. **Braucht:** Autoritäts-Verdikt → Pins + `sha256` je DE440-Zeile.
- **Generiertes `LICENSE` im `omegaflow/sources`-Repo.** Generator + Drift-Tor, nachdem Mountains `terms`-Zeilen landen.
- **`static/membrane.html` `BODIES`-Handkopie** (`:51`) → Build-Time-Manifest aus der Hüllen-Pipeline.
- **CDN-Workflows für die neuen Geo-Arme** `osm_nodes`/`eionet_cdr` (Mountain gebaut; `main_flow` von River 112 verdrahtet) — Manifestation + sha ins Register.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `static/membrane.html` (zwei Aperturen)
- `src/wasm.rs` (Anker ohne `t2`; native Parität)
- `src/archivar/membrane.rs` (`all_body_anchor_samples` nimmt `body_record_epoch`)
- `src/archivar/spatial.rs` (`query_hash`-Zusatz-Gate trägt `reach`, kein nacktes `extent+pad`)
- `src/archivar/tests.rs` (Known-Answer `test_body_anchor_reaches_ssb_presence_by_signal_cone`)
- `docs/handover/handover-2026-10-06-river-folge114.md` (neu) · `docs/handover/archiv/handover-2026-10-06-river-folge113.md` (Move)

Gefaltet (adressierte Blöcke, in diesem Atom):
- mountain-folge249 (`## An river`): `main_flow`-Registry erledigt; GIC-Blatt-Grenzen-Satz als Riss gemessen.
- future-folge186 (`## An river`): Membran-Startansicht gebaut; `omega.rs:878`/`membrane.html:43` veraltet.
- sensory-folge244 (`## An river`): exzellenz-gate-Label in folge113 geschlossen.

`open_points_check` am folge114: 20 path refs · 0 absent · 0 stale-citations · 0 done-carried · 2 word-carried · 0 guardians · 0 format-gaps · 0 owner-drift · 0 post-md.

## Burn: open 0.0869 · close 0.3743 · cap 0.40 · Grund: River 114 — Line-Session (deepseek-flash): Rat + 5 API-Stimmen + UI-Chats zur Membran-Apertur befragt (12/13 (b), Claude-Dissens (c)); `static/membrane.html` auf zwei Aperturen gebaut; **Browser-Test gegen den Live-Link: `lookup.query` liefert ohne Sterne 0 Records — Ursache gemessen: `wasm.rs:77` stempelte die Anker-Epoche mit `t2` (age 0), `query_hash` verlangte Präsenz im Körper**; Herkunft = River 91; Rat + Kanäle → **(a) native Parität**, gebaut (`all_body_anchor_samples` `body_record_epoch`, `wasm.rs`, `query_hash`-Gate auf `reach`, Known-Answer-Test); `add_stars` panikt bei Re-Init; adressierte Blöcke mountain-249/future-186 gefaltet; Rohmaterial `state/stimmen/2026-10-06_membran-apertur-alle-stimmen.md` + `…-fix-…`. (close = die eigene Line-Session aus `session_burn`; cap über dem $0.15-Default, weil der mehrkanalige Rat/UI-Atom über den Default hinausgeht.)
