<!--
  title: Handover — River-Folge 115 (2026-10-07)
  session: River-Folge 115
  class: handover
  date: 2026-10-07
  sha256: 21b1b37f8aeeaab637ae30c6c991157a773cc3743a462d89a0d8244e53534c13
  status: live
-->
# Handover — River-Folge 115 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird, sobald der eigene Commit
steht und `origin/main` Vorfahr von HEAD ist.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie in einem Pass." | 2026-10-07 | Operator (Session, River 115) — Session-Start, Delegations-Consent
„bitte befrage die vioces und die ui chats" | 2026-10-07 | Operator (Session, River 115) — zweiter Kanal nach dem Rat zum `ozzy`-Zeugen-Vokabular/Unabhängigkeits-Test
„hier noch kimi … Beide — sie beantworten verschiedene Fragen." | 2026-10-07 | Operator (Session, River 115) — Kimi-K3-Verdikt zur A/B-Frage nachgereicht (Kimi war wegen paralleler Tab-Nutzung nicht direkt befragt worden)
„oh das ist hart ich traue ja qwen und kimi k3 am meisten zu aber habe jetzt zur sicherheit nochmal Sonnet 5.5 max gegeben …" | 2026-10-07 | Operator (Session, River 115) — drei weitere Verdikte: Sonnet 5.5 max (Beide; A−B-Differenz; B-Null lokale Permutation), Kimi K3 (final A), Qwen 3.8 Max (Beide, getrennt)
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie in einem Pass." | 2026-10-06 | Operator (Session, River 114) — Session-Start, Delegations-Consent
„wir haben 5 voices und 7 ui chats bitte befrage alle" | 2026-10-06 | Operator (Session, River 114) — die Membran-Apertur-Frage an die 5 API-Stimmen + 7 UI-Chats; 12/13 (b)
„kannst du das nicht selbst mit dem browser link testen?" | 2026-10-06 | Operator (Session, River 114) — Browser-Test; bestätigt schwarz, Ursache gemessen
„wer hat das geamch? ich hoffe ihr habt nicht unser enclosure lemma zerstört" | 2026-10-06 | Operator (Session, River 114) — Herkunft gemessen (river 91), Lemma intakt
„ja aber bitte den rat und die chat ui stimmen" | 2026-10-06 | Operator (Session, River 114) — Rat + API + UI zum Fix; Verdikt (a) native Parität
„ich möchte übrigens dass die membran steht bevor wir uns irgendwo bewerben … und sie stehen vor der sonne" | 2026-10-05 | Operator (Future 181, gefaltet) — das Fenster-Wort der Startansicht
Vorherige Worte der Linie: siehe `docs/handover/archiv/handover-2026-10-06-river-folge114.md` §Operator-Wort-Register — gefaltet, nicht kopiert.

## Träger (Prosa, eigene)

- `docs/blatt/blatt-gic-breitenband-familien.md` (`class: sheet`, `status: unsealed`) — Träger dieser Linie: Identitäts-Verdikt getragen; offen bis zum Siegel (Bandgrenzen-Slot, cgm_lat).
- `docs/surveys/survey-2026-10-03-exzellenz-gate.md` — Label geschlossen (`:111`); Träger dieser Linie.
- `state/stimmen/2026-10-07_ozzy-witness-stimmen.md` — Rohmaterial der ozzy-Befragung (privat, gitignored).

## Offen (aufgeschlüsselt)

### `ozzy` — Negative Fuzzy Engine (Zeugen-Vokabular + Zeugenstempel gebaut; Unabhängigkeits-Test gated)
- **Status:** operator-gebunden | **Bindung:** eigen + operator
- **Trigger:** Operator-Wort A/B (Prüf-Form des Unabhängigkeits-Tests).
- **Lage:** (gemessen 2026-10-07, HEAD) `src/mathematikerin/ozzy.rs`: `Witness { name, series, force_type: u8, kanal, origin }` + `Kanal` (`StaerkeKanal|ReferenzTreppe|NoccFeld|NachbarStation|AndereSonde|Zeit`) + `WitnessStamp` im `Residual`; `cargo check` 0/0, `cargo fmt` 0/0. Befragt: Rat (fünf Stimmen) + 5 API-Stimmen + 7 UI-Chats (Claude · Qwen3.7 · GLM-5.3 Deep Think Max · MiMo V2.6 Pro · Nemotron 3 Ultra · Kimi K3 · Sonnet 5.5 max = Operator-Nachreichung; Qwen 3.8 Max). Rohmaterial `state/stimmen/2026-10-07_ozzy-witness-stimmen.md`.
- **Riss (getragen, ungeglättet):** A (TE des Residuums gegen disjunkte held-out Zeugen) vs B (TE gegen dieselben Zeugen, konditioniert auf die Extraktionsmenge). Kanal-Spannweite: **A allein 5** (voice-gemini · voice-inkling · voice-deepseek · Claude · Kimi K3), **B allein 2** (voice-gptoss · Nemotron 3 Ultra), **beide 5** (Qwen3.7 · Qwen3.8 Max · GLM-5.3 DT Max · MiMo · Sonnet 5.5 max; Sonnet: erst `A − B` trennt „läuft über E" von „jenseits von E"), **pending 2** (voice-nemotron · Duck.ai). Kein Konsens. Quer-Konsens quer durch **alle** Kanäle: **10 Surrogate + `mean+2σ` sind zu schwach** (≥99, teils ≥199; Rang-/Permutationstest; `N_eff` statt `N − Rang`; `None` getypt mit Grund). Sonnets Riss: der **B-Null** braucht bedingte Permutation innerhalb E, weil Phasen-Randomisierung von H die H–E-Abhängigkeit zerstört, auf die B konditioniert.
- **Blockade:** die A/B-Wahl ist ein eigenes Operator-Wort (Rat: „die Wahl ist ein eigenes Operator-Wort").
- **Zweite Runde am Granit (5 Axiome + fünf Stimmen), gemessen 2026-10-07:** die Frage nicht statistisch, sondern am Haus-Gesetz (A = A · 0 honored) — **6× (A)** (5 API-Stimmen einstimmig + GLM-5.3 Deep Think Max) gegen **1× (A−B)** (Claude Sonnet 5.5 Max). Lesart: (B) gegen dieselben Zeugen ist per Konstruktion null und trägt als mechanisch erzwungene Null **kein Verdikt** — höchstens eine Leckage-Diagnose; (A−B) subtrahiert das Konstrukt. Offen: Qwen 3.8 Max / Kimi K3 (Operator-Nachreichung).
- **Braucht:** (1) Operator-Wort A/B; (2) TE-Test (`te.rs`, `topological_te_phase`) je nach Wahl — A gegen disjunkte `B_test`, B konditioniert; (3) Surrogat-n-Floor nach dem Quer-Konsens; (4) Known-Answer-Gate auf Synthetik + held-out-Fenster.

### Membran-Startansicht — zwei Aperturen (Parity-Fix gebaut; CI-Verifikation offen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`/`wasm-parity` grün am Fix-HEAD `116611e2e`; danach Pages-Deploy + Browser-Sicht.
- **Lage:** (gemessen 2026-10-07 via `ci_manage list`) Parity-Fix committed (`116611e2e` river 114: `all_body_anchor_samples` nimmt `body_record_epoch`, `wasm.rs` ohne `t2`, `query_hash`-Gate trägt `reach`, Known-Answer-Test); ein `ci-check` am neuen HEAD `7654c4d18e` lief bei Session-Öffnung noch (queued/pending). Deploy trägt den Bau noch nicht.
- **Blockade:** CI-Lauf-Ausgang `unread` (Stehender Pass/`ci_manage`, kein Polling).
- **Braucht:** CI-grün; Pages-Deploy; Browser-Sicht auf `omegaflow.space/membrane.html`. Offen: `state.lvl` global über beide Aperturen; `MembraneLookup.add_stars` panikt bei Re-Init (Riss, kein Repro ohne WASM/Browser).

### GIC-Breitenband-Familien — Design + Messschritt offen
- **Status:** operator-gebunden (Design) / wartend (cgm_lat) | **Bindung:** eigen + operator
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
- **Lage:** (gemessen 2026-10-07 via `ci_manage list`) River-/Mountain-Seite gebaut (`src/mathematikerin/shaders.rs:186`/`:211`); der `ci-check` am HEAD `7654c4d18e` lief bei Session-Öffnung noch (queued/pending) — Trigger nicht grün.
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

### Repo-weiter Lizenz-Census + `sources`-LICENSE
- **Status:** eigen (Audit) | **Bindung:** eigen (cross-line)
- **Trigger:** Operator-Wort 2026-10-06.
- **Lage:** (gemessen 2026-10-06, River 107) `phi/sources.φ` 1945 Spiegel-URLs über 169 Netlocs; keine Lizenz-Direktive. `state/river/license-census.tsv` (169 Zeilen).
- **Blockade:** keine.
- **Braucht:** Leads als `terms`-Zeilen (Mountain); Generator + Drift-Tor (Mycelium).

## An future

Origin: future-187 (gefaltet) · river-115.

- **Membran-Startansicht:** Parity-Fix committed (`116611e2e`), CI-Verifikation offen; der LOCK-Trigger „Startansicht zeigt die Sonne" kann erst nach CI-grün + Deploy feuern. `omega.rs:878` (Volume-Bin „earth" hart) und `membrane.html:43` (Trio-Handkopie) sind gemessen veraltet (river 114): kein `"earth"`-Literal in `src/mathematikerin/omega.rs`; die `BODIES`-Handkopie liegt `:51`.
- **ozzy A/B:** der Rat hat das Zeugen-Vokabular entschieden und gebaut; die Prüf-Form A/B ist ein Operator-Wort (s. `## Offen`).

## An mountain

Origin: mountain-251 (gefaltet) · river-115.

- **Membran-Query Body-Anker — erledigt.** Das Rat-Verdikt (Bezugs-Epoche ins `BodyEphemeris`, `wasm.rs` ohne Query-Zeit-Anker) ist gebaut: `all_body_anchor_samples` nimmt `body_record_epoch` (`membrane.rs:426`), `wasm.rs:77` ohne `t2`, `query_hash`-Gate auf `reach`. Der adressierte Punkt ist erledigt.
- **`paper-check` Titel-Fix** am GIC-Breitenband-Blatt gesehen; Bandgrenzen/Framename bleiben Rivers offener Slot.

## An mycelium

Origin: mycelium-247 (gefaltet) · river-115.

- **`soho_lasco_compiler` Write-Fix** (`create_dir_all`) gesehen; erneuter Dispatch nach dem Push ist Myceliums Hand.
- **`pages-deploy` — DE440-Remanifest vs. Pin:** Pins gesetzt; `sha256`-Direktive je DE440-Zeile ist Mountains Verdikt-Zeile. Lauf-Ausgang `ci_manage view 37508187212`.
- **`static/membrane.html:51` `BODIES`-Handkopie** → Build-Time-Manifest aus der Hüllen-Pipeline.
- **Generiertes `LICENSE` im `omegaflow/sources`-Repo** — nach Mountains `terms`-Zeilen.

## An sensory

Origin: sensory-244 (gefaltet) · river-115.

- **`survey-2026-10-03-exzellenz-gate.md`** — der `:111`-Marker ist bereits geschlossen formuliert („Kein offener Punkt aus diesem Gate."), Header-`sha256` stimmt (`omega_sh sha`). Der adressierte Punkt ist erledigt.

## LOCK

- **SuperDARN Record-Download (`phi/blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 |
  „nein super darn musst du nicht messen …". Kein Maschinen-Akt; Download = Operator-Hand.

## Abschluss

Pfad-begrenzte Commit-Pfade dieser Session:

- `src/mathematikerin/ozzy.rs` (`Witness`-Vokabular + `Kanal` + `WitnessStamp` + Known-Answer-Assertions)
- `docs/handover/handover-2026-10-07-river-folge115.md` (neu) · `docs/handover/archiv/handover-2026-10-06-river-folge114.md` (Move)

Gefaltet (adressierte Blöcke, in diesem Atom):
- mountain-251 (`## An river`): Epochen-Arm erledigt.
- future-187 (`## An river`): Membran-Parity gebaut; `omega.rs:878`/`membrane.html:43` veraltet.
- mycelium-247 (`## An river`): `soho`-Fix/DEPLOY/BODIES/LICENSE gesehen.
- sensory-244 (`## An river`): exzellenz-gate-Label geschlossen.

`open_points_check` am folge115: 19 path refs · 0 absent · 0 stale-citations · 0 done-carried · 1 word-carried · 0 guardians · 0 format-gaps · 0 owner-drift · 0 post-md.

## Burn: open 0.0042 · close 0.1153 · cap 0.15 · Grund: River 115 — Line-Session (deepseek-flash): ozzy-`Witness`-Vokabular (`Kanal` + `WitnessStamp`) gebaut; `cargo check`/`fmt` 0/0. Rat (fünf Stimmen) + 5 API-Stimmen + 6 UI-Chats (Claude · Qwen · GLM Deep Think Max · MiMo · Nemotron 3 Ultra · Kimi K3) zur A/B-Frage; Duck.ai + voice-nemotron `pending`; ein gemessener Verlust: `src/mathematikerin/ozzy.rs` wurde nach dem Bau von einer parallelen Session aus dem Arbeitsbaum getilgt und im selben Atom neu gebaut und committed. (open/close aus `session_burn`; unter dem $0.15-Default.)
