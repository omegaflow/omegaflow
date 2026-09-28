<!--
  title: Handover — River-Folge 50 (2026-09-28)
  session: River-Folge 50
  class: handover
  date: 2026-09-28
  sha256: 7509adb70f2f0b24471038f1d8eb7b9746a8041c916936ed2cbbf6b6678b5eaa
  status: live
-->
# Handover — River-Folge 50 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert:
`state/zustand/standing-pass.md`. Nur eigene Arbeit: pfad-begrenzter Commit,
fremde uncommittete Arbeit unangetastet.

Diese Session konsumierte `handover-2026-09-27-river-folge49.md` (nach `archiv/`).
In diesem Atom (messend, ohne Quelltext-Edit): die Lade-Membran lokal headless
gemessen (`cargo run -p omegaflow-measure --bin membrane_hull_probe`, 2× stabil,
0/4 Pfade divergieren, `rho_star 2.529766e17 m`); den vollendeten `ci-check`
`36347555576` @`a457ed8c` gelesen (2 failed — River
`volume_probe_parity_masked_corner_and_plain` „gpu 0 cpu 2.5" + Mountain
`test_cache_fresh_cdn_stamp_equality_and_release_branch`) — der River-Red ist von
Mountain `491c6e61c` geheilt und gelandet (HEAD `97474363b` == `origin/main`); den
redundanten Dispatch `36375195304` zurückgenommen. Die Verifikation reitet auf
`ci-check 36377277112` @`97474363b`.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Erste Handlung: `sread docs/concepts/tool-forms.md`" | 2026-09-27 | Operator (Session, River 49)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt) … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." | 2026-09-27 | Operator (Session, River 49)
„du sollst keine operator worte tragen das ist sache von future du bist bis zur kante" | 2026-09-27 | Operator (Session, River 48)
„warum holst du den si konsolen punkt schon wieder hoch — du hast doch schon den rat einberufen; rat und operator sind zwei paar schuhe" | 2026-09-27 | Operator (Session, River 48)
„die ttl muss die Aktualisierung der Quelle sein" | 2026-09-27 | Operator (Session, River 47)
„#body erzeugt das bias … komplett rückgängig" — kein Körper privilegiert | 2026-09-27 | Operator (Session, River 47)
„frag den rat" — die Form der SI-Konsole dem Rat vorlegen | 2026-09-27 | Operator (Session, River 47)
„folge dem rat" — Kraft-**Name** statt Debug-Zahl, Einheiten-Symbol `absent` | 2026-09-27 | Operator (Session, River 47)
GIC-Paper-Einreichung: höchste Priorität | 2026-09-27 | Operator-Wort
Geräte-Zugriff: vor jedem Zugriff fragen (adb/BT) | 2026-09-26 | Operator-Wort
Harte-Läufe-LOCK aufgehoben | 2026-09-26 | Operator-Wort
HTTPS ja | 2026-09-26 | Operator-Wort folge36
Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36
vC 945 von Mantis Shrimp getrennt | 2026-09-26 | Operator-Wort folge36
UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | 2026-09-27 | Operator (Session, Mountain)
Entscheidungen nie als Liste vorlegen — jede braucht eine Erklärung | 2026-09-27 | Operator (Future-Session)
D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — Fakten direkt abarbeiten | 2026-09-27 | Operator (Session, Mountain)
RX100-Kalibrierer descoped („über exif weg", K=12.5) | 2026-09-27 | Operator (Future-Session)
RX100 war nur ein Gedanke — Quelle zurückgezogen | 2026-09-27 | Operator (Session, Mountain)
„die Kante bin ich" — Wert, Wort, Dritt-Akt und Send bleiben seine Hand | 2026-09-27 | Operator (Future-Session)
ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben | 2026-09-27 | Operator (Future-Session)
`/consent` — session-weiter Delegations-Consent, nicht das Commit-Wort | 2026-09-27 | session-weiter Consent (`/consent`)
Commit-Wort (`/commit`) — pfad-begrenzter Commit + Push, das Doppel-Ask | 2026-09-27 | Operator
„probiere C lokal /consent" | 2026-09-28 | Operator (Session, River 50)
„teste nochmal" | 2026-09-28 | Operator (Session, River 50)
„messe nochmal" | 2026-09-28 | Operator (Session, River 50)
„warum fixt du nicht anstatt zu verschleppen? eigentlich müsste es gefixt sein aber checke nochmal" | 2026-09-28 | Operator (Session, River 50)
„Committe und pushe jetzt — nur deine eigene Arbeit, gemessen nicht beteuert" | 2026-09-28 | Operator (Session, River 50) — Commit-Wort (Doppel-Ask)

## Offen (aufgeschlüsselt)

### GPU-Readback map/unmap + Lade-Membran — CI-Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check 36377277112` (head `97474363b`, `pending`).
- **Lage:** (gemessen 2026-09-28 via `ci_manage` + lokalem `cargo run`) der GPU-Readback-Fix
  ist mit `ddfe3e1bd` committet+gepusht. Der alte, vollendete `ci-check 36347555576`
  @`a457ed8c` war **failure**: 1890 passed, **2 failed** — River
  `mathematikerin::tests::volume_probe_parity_masked_corner_and_plain` („volume parity:
  gpu 0 cpu 2.5") und Mountain
  `archivar::tests::test_cache_fresh_cdn_stamp_equality_and_release_branch`
  (`src/archivar/tests.rs:6673`). Der River-Red ist **geheilt und gelandet**:
  `491c6e61c mountain 190 … heal the mathematikerin pole fixture and the geodetic NaN …
  (fix, do not carry)`, HEAD `97474363b` == `origin/main`; der Arbeitsbaum ist frei von
  Quelltext-Hunks (`cargo check` 0/0). Die Lade-Membran ist lokal headless grün (`cargo run -p
  omegaflow-measure --bin membrane_hull_probe`, 2× stabil, 0/4 divergieren). Der redundante
  Dispatch `36375195304` (alter HEAD `a457ed8c`) wurde zurückgenommen.
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36377277112` **einmal** nach Laufende (kein Polling).

### Flyby-Path-2 — Füll-Lauf
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** Perigäum 2026-09-28 **11:45:12 UTC ± 10 s** (geozentrisch ≈ 0,00010039 AU = 15 018 km; JUICE ist Horizons `-28`).
- **Lage:** (gemessen 2026-09-27 via `cargo build`) Fill-Bin `tools/measure/src/bin/flyby_path2_fill.rs` gebaut (0 Warnungen); Tube ±12 h; Perigäum-Zelle 13, alle Zellen `pending`.
- **Blockade:** kein Workflow registriert — der Lauf muss zum Zeitpunkt erfolgen.
- **Braucht:** RTSW-Snapshots ~28.09. 12:00 + 29.09. 00:00 UTC (`sfetch https://services.swpc.noaa.gov/json/rtsw/rtsw_mag_1m.json` + `…/rtsw_wind_1m.json` → `data/services.swpc.noaa.gov/`), dann `cargo run -p omegaflow-measure --bin flyby_path2_fill -- --flyby juice --snapshots data/services.swpc.noaa.gov/`.

### Flyby-Path-2 — DSN-Status am Perigäum
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** 2026-09-28.
- **Lage:** (gemessen 2026-09-27 via `archive_search --playwright` + `sfetch eyes.nasa.gov/dsn/data/dsn.json`) `pending` — die Seite rendert keine lesbare Tracking-Tabelle (JS/canvas); der Live-Feed (09:48:57Z) trägt keinen JUICE-Eintrag.
- **Blockade:** keine.
- **Braucht:** am 28.09. erneut `archive_search --playwright https://eyes.nasa.gov/dsn/dsn.html` — „tracked (Station X, Band Y)" oder „nicht getrackt".

### Flyby-Path-2 (revised) — δ gemessen, Benotung offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neuer `release` der Post-Flyby-Ephemeride (CDN `ephemeris_juice.bin`) samt 1-σ-Kovarianz — Wochen nach dem Vorbeiflug.
- **Lage:** (gemessen 2026-09-27 via `cargo build` + Bin-Lauf) `flyby_ephemeris_gate` gebaut; beide Zeugen lesen `placed`; **δ = 0,1684732 km** (168 m, DE441 vs DE442; `phi/sources.φ:3448`/`:3469`). Δ/σ_recon `pending` — die Post-Flyby-Daten fehlen.
- **Blockade:** keine.
- **Braucht:** nach dem Flyby `--recon` (Post-Flyby-Arc) + `--sigma-recon` (veröffentlichte 1-σ) → `cargo run -p omegaflow-measure --bin flyby_ephemeris_gate -- --recon <arc> --sigma-recon <km>`; Riß gegen **beide** Hashes („aeb3c82f…" sealed, „eee376ef…" CDN) tragen.

### orphan-certainty — Certainty quantum/decay (Legacy-Konzept)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** eigen (kein externer Trigger).
- **Lage:** (gemessen 2026-09-25, legacy-konzepte) L:53 lautet `certainty = exp(-vC/(g+ε))·quantum·decay`; heute lebt `perm_target(g, v_c) = tanh(v_c/(g+ε))` (`omega.rs:15`) als Permeability-Atem, `quantum`/`decay` fehlen im Baum. `exp(-x)` fällt monoton, `tanh(x)` steigt — ein wörtlicher Port invertiert den Atem.
- **Blockade:** keine.
- **Braucht:** den quantum/decay-Faktor als Permeability-Atem in `omega.rs` bauen (Richtung gemessen) oder als `descoped` mit Befund schließen. (aus `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md`)

### orphan-coherence — Total-Coherence per-Oszillator + Complexity
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** eigen.
- **Lage:** (gemessen River-Folge 47) `total_coherence_probe.rs` gebaut: `Σ_i perm_target(|ω_i|,|Δω_i|) = 0.924` gegen leeres Feld 0.0; die gebaute Permeability (ein TE-Skalar über die Summe) kollabiert auf 0.0 — per-Oszillator-Integral und Complexity-Term bleiben offen.
- **Blockade:** keine.
- **Braucht:** per-Oszillator-Integral + Complexity-Term in den Live-Pfad (`omega.rs`) tragen oder als Befund schließen. (aus `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md`)

### orphan-blatt-membran — Blatt-Papier-Beweis: Membran-Bindung
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** eigen.
- **Lage:** (gemessen 2026-09-28 via sread docs/concepts/blatt-papier-beweis.md:42) Probe-Muster steht (`nobel_probe_corona.rs`), kanonische Referenz `te.rs` (`transfer_entropy_lag`, `topological_te_phase`); „Die Membran-Bindung bleibt pending" (`blatt-papier-beweis.md:42`). Die drei Blätter (ENSO/Bz/LAIC) warten auf ihren Zuschnitt.
- **Blockade:** keine.
- **Braucht:** Blatt-Probe + Membran-Bindung bauen; Blatt-Zuschnitt → `wartend.φ`. (aus `docs/concepts/blatt-papier-beweis.md`)

### orphan-fortschritt — survey-fortschritt: offene Membran-Verbesserungen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** eigen.
- **Lage:** (gemessen 2026-08-16 via docs/surveys/survey-fortschritt.md) §C nennt offen: Deep-Lieferung richtungsbasiert, Zell-Achse, Relay-Trailer (gen u64 + 9×Ω), Deep-Upload-Stille, Rgba8Unorm-Nachmessung, Fovea als Budget-Kappe; `deep_dirty` existiert im heutigen Baum nicht mehr (`grep src` 0 Treffer) — teils überholt, ungemessen.
- **Blockade:** keine.
- **Braucht:** je Punkt gegen den heutigen Baum messen → schließen oder `descoped` mit Befund. (aus `docs/surveys/survey-fortschritt.md`)

## An Mountain (gemessen, fremde Feder)

- (gemessen 2026-09-27 via Rat + general-read-only) **`ttl` in `phi/sources.φ` ist eine
  Verdikt-Zeile → Mountain, alleiniger Schreiber.** Rat 2026-09-27: das Operator-Wort
  („die ttl muss die Aktualisierung der Quelle sein"), `derive_ttl`
  (`src/archivar/port.rs:1237`) als Messinstrument aus der Quelle, das Flush-Gate
  (`src/archivar/parse.rs:80`) und der Backoff (`src/archivar/fetch.rs:298`,
  `2^failures`) tragen die operative Abweichung — Myceliums Liste bleibt
  `url`/`origin`/`compiler`/Tags. Kein permanenter Riss, kein Split-Modell.
- Der **81-Block-Fix** (`format ephemeris_binary`, `ttl 86400`) gehört Mountain:
  Release-skalen-Prüfintervall, gemessen aus Live-Release-Abständen oder als
  Potenz-von-2-Untergrenze 2²⁵ s ≈ 388 d. **AGENTS.md-Präzisierung im selben Atom**
  (Prinzip: gemessene Quellen-Eigenschaft → Mountain; Materialisierung/Transport →
  Mycelium).
- **Sprachloch:** kein „no-cadence"-Zustand im Parser (`ttl 0` → inaktiver Block,
  `parse.rs:80`) — als `pending` registrieren, nie mit einem falschen Puls übertünchen.
- (gemessen 2026-09-27 via `git show 4783bbf57`) die 4 mathematikerin-Fälle in
  Mountains `## An River` (folge188) sind mit River `4783bbf57` **beglichen** —
  Mountains Session kann die Zeile streichen.

## Verweise (Prosa mit offenen Markern)

- `docs/auftrag/auftrag-gic-einreichung.md` — Einreich-Paket (Träger des gic-Punkts; Vorbereitung die Kante erreicht, Einreichung in Future-Queue #8).
- `docs/paper/gic-causal-driver.md` — GIC-Papier (paper-check grün; Vorbereitung für Einreichung).
- `docs/paper/flyby-path-2-preregistration-revised.md` — revidierte Präregistrierung (Träger dieses Punkts).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` — presence-only Ladearchitektur.
- `docs/surveys/survey-messpunkt-verteilung.md` — Messpunkt-Verteilung (D5-Marker gemessen, offene Kandidaten-Fragen §9).
- `docs/auftrag/auftrag-flyby2-kette.md` — getragen durch die bestehenden Flyby-Path-2-Punkte (Füll-Lauf, DSN-Status, revised); die Kette selbst steht, keine Duplikat-Zeile.
- `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md` — Silence-Map (`silence_map_probe.rs`), Betti-0 (`betti0_silverman_probe.rs`), Delay (`delay_spectrum_probe.rs`) und Minkowski (`minkowski_ds2_delta_probe.rs`) sind gebaut; offen nur Certainty + Total-Coherence (Blöcke oben).
- `docs/concepts/kybernetische-astrophysik.md` — lebendes Konzept (12 Nadeln); die Marker sind konzeptionelle `pending`-Prosa, kein neuer Handlungsschritt.
- `docs/concepts/pfeiler-der-architektur.md` — die genannten Codepfade existieren im Baum (`sense_membrane`, `gate_weigh`, `field_spatial`, `topological_te_phase`, `phase_randomized_surrogate`, `kepler.rs`, `hdf5.rs`, `spatial.rs`); kein offener Punkt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
