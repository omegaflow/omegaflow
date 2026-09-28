<!--
  title: Handover — River-Folge 51 (2026-09-28)
  session: River-Folge 51
  class: handover
  date: 2026-09-28
  sha256: 89822b304b856f2479e1036f596fa3be3ba6f97a88e0a2fb555083aa63a201c0
  status: live
-->
# Handover — River-Folge 51 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert:
`state/zustand/standing-pass.md`. Nur eigene Arbeit: pfad-begrenzter Commit,
fremde uncommittete Arbeit unangetastet.

Diese Session konsumierte `handover-2026-09-28-river-folge50.md` (nach `archiv/`).
In diesem Atom: den per-Oszillator-Integral der Total-Coherence in den Live-Pfad
gebaut (`src/mathematikerin/omega.rs` Breath-Zweig — der Skalar über die Summe
verschwieg die gemessene per-Medium-Bewegung; Fixture 0.924/9 gegen Skalar 0.0),
Gate-Test `the_no_te_tick_hears_each_oscillator_not_the_sum` in der
mathematikerin-Suite; `orphan-certainty` als `descoped` geschlossen (der
Legacy-Spec selbst trägt die Ersatzform, `quantum`/`decay` ohne Live-Träger) und
`orphan-fortschritt` §C je Punkt gemessen/geschlossen; `orphan-blatt-membran` per
Rat `gehalten` (Pflicht-Reihenfolge). Drei Träger-Dokumente (blatt-papier-beweis,
survey-legacy-konzepte, survey-fortschritt) mit dem Befund und neuem sha256.
`cargo check` 0 Fehler / 0 Warnungen.

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
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt) … Commit und Push trägt `/commit`." | 2026-09-28 | Operator (Session, River 51) — session-weiter Delegations-Consent

## Offen (aufgeschlüsselt)

### GPU-Readback map/unmap + Lade-Membran — CI-Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check 36377277112` (head `97474363b`) und der Nachfolger auf dem neuen HEAD `9f8debcc3`.
- **Lage:** (gemessen 2026-09-28 via `ci_manage status`) der Readback-Fix ist gelandet; `36377277112` lief noch (Schritt `dropped-gate`, 04:28Z), der neue HEAD `9f8debcc3` startete einen weiteren `ci-check`. Die Lade-Membran ist lokal headless grün (folge50 gemessen).
- **Blockade:** keine.
- **Braucht:** `ci_manage log <id>` **einmal** nach Laufende (kein Polling).

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

### Blatt-Membran-Bindung — pending (Rat-Reihenfolge)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** eigen (die geerbten Pflichten (1)–(3) sind die Arbeit).
- **Lage:** (gemessen 2026-09-28 via `sread` + Rat) Probe-Muster `nobel_probe_corona.rs` steht, kanonische Referenz `src/mathematikerin/te.rs` (`transfer_entropy_lag` :96, `topological_te_phase` :2686); die Membran-Maschine `te_compute` läuft (`omega.rs:456` `te_probe`, gefüttert :491-492), aber kein Call-Site bindet ein Blatt-Paar hinein (`sgrep` 0). Bz/LAIC-Probes laufen den Offline-Skalar, ENSO-Probe fehlt; Zuschnitt = Operator-Wort (`docs/concepts/blatt-papier-beweis.md:78`). Rat 2026-09-28: `halten`.
- **Blockade:** keine.
- **Braucht:** (1) Mehrfachvergleichskorrektur über alle getesteten Paare, (2) Lag-Sweep, (3) KDE-Sensitivität gegen h, (4) Zuschnitt = Operator-Wort; dann Paar-Registrierung am Einstieg `te_probe` (`omega.rs:456`/`:491-492`). Benannter Check des Bau-Atoms: Schätzer-Parität (skalarer Probe-Pfad vs. topologische Membran-Maschine). (aus `docs/concepts/blatt-papier-beweis.md`)

### Total-Coherence — Complexity-Term pending
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** eine physikalische Definition + Datenquelle für `complexity`.
- **Lage:** (gemessen 2026-09-28, River-Folge 51, Rat) der per-Oszillator-Integral ist in den Live-Pfad gebaut (`omega.rs` Breath-Zweig; Gate-Test `the_no_te_tick_hears_each_oscillator_not_the_sum`); der Complexity-Term `1/(1+Σ(complexity·weight)/Σ(weight))` hat keine Definition und keine Datenquelle — die Legacy-Identifier `complexity`/`takens` erscheinen nicht im Baum.
- **Blockade:** keine Definition, keine Datenquelle.
- **Braucht:** `complexity` als gemessene Größe definieren; bis dahin `pending` (Registerzeile, nie gebaut). (aus `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md`)

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
- `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md` — Silence-Map, Betti-0, Delay und Minkowski sind gebaut; Certainty `descoped`, Total-Coherence-Integral gebaut (Complexity `pending`) — Blöcke oben.
- `docs/concepts/blatt-papier-beweis.md` — Membran-Bindung `pending` mit der Rat-Reihenfolge; Blatt-Zuschnitt = Operator-Wort.
- `docs/surveys/survey-fortschritt.md` — §C je Punkt gemessen: Deep-Lieferung/Zell-Achse-Quantisierung/Relay-Trailer/Fovea `descoped`, Deep-Upload-Stille/Rgba8Unorm `ueberholt`.
- `docs/concepts/kybernetische-astrophysik.md` — lebendes Konzept (12 Nadeln); die Marker sind konzeptionelle `pending`-Prosa, kein neuer Handlungsschritt.
- `docs/concepts/pfeiler-der-architektur.md` — kein offener Punkt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
