<!--
  title: Handover — River-Folge 53 (2026-09-28)
  session: River-Folge 53
  class: handover
  date: 2026-09-28
  sha256: 97ce2452ed5abc1ed0a131d1d6272685f6d28cb2ddc3979c7de5a6701ca0890d
  status: live
-->
# Handover — River-Folge 53 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert:
`state/zustand/standing-pass.md`. Nur eigene Arbeit: pfad-begrenzter Commit,
fremde uncommittete Arbeit unangetastet.

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
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt) … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort." | 2026-09-28 | Operator (Session, River 52) — session-weiter Delegations-Consent
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt) … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort." | 2026-09-28 | Operator (Session, River 53) — session-weiter Delegations-Consent
„Committe und pushe jetzt — nur deine eigene Arbeit, gemessen nicht beteuert. Dieser Befehl ist das Commit-Wort des Operators (das Doppel-Ask) …" | 2026-09-28 | Operator (Session, River 53) — Commit-Wort (Doppel-Ask)
„fixxen vor verschleppen, mein wort!!!!" | 2026-09-28 | Operator (Session, River 53) — Fix-Wort (die zwei roten Gates jetzt bauen, nicht registrieren)

## Offen (aufgeschlüsselt)

### GPU-Readback map/unmap + Lade-Membran — CI-Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein ci-check-Lauf auf einem HEAD ≥ Readback-Fix, der **abschließt**.
- **Lage:** (gemessen 2026-09-28 07:27Z via `ci_manage log 36385567226`) der ci-check-Lauf auf `302063d36` schloss **rot** ab (2 fehlende Tests, s. eigener Punkt unten); der Readback-Test selbst steht nicht unter den Fehlern. Der Push `b45550527` startete `36398092149` ci-check (pending) — der grüne Readback-Nachweis steht noch aus. Den vollen CI-Stand trägt der Stehende Pass (`state/zustand/standing-pass.md`, zitiert). Die Lade-Membran ist lokal headless grün (folge50).
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36398092149` **einmal** nach Laufende (kein Polling) — Readback-Test grün = Punkt geschlossen.

### Zwei rote River-Gates — Membran-Parität + No-TE-Tick
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein ci-check-Lauf auf dem Fix-HEAD, der **abschließt**.
- **Lage:** (gemessen 2026-09-28 via `cargo check`) Fix gebaut: die Paritäts-Fixture ist von der unter Phasen-Randomisierung degenerierten Sinus-Fixture auf das breitbandige AR(1)+Rausch-Paar umgestellt (`causal_pair_ar`, dieselbe Fixture wie `gate_fn_bias`/`split_recording`), die Schwelle über 100 Surrogate stabilisiert; der No-TE-Tick-Test liest das Delta-Integral **vor** `tick()` (nach `tick()` ist `prev_probe_omega == probe_omega`, das Integral war 0, erwartet 0,0649 = `target·alpha`). `cargo check` 0 Fehler / 0 Warnungen.
- **Blockade:** keine.
- **Braucht:** `ci_manage log <id>` **einmal** auf dem Fix-Lauf — grün = beide Gates geschlossen; rot = Log auswerten.

### Flyby-Path-2 — Füll-Lauf
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** Perigäum 2026-09-28 **11:45:12 UTC ± 10 s** (geozentrisch ≈ 0,00010039 AU = 15 018 km; JUICE ist Horizons `-28`); Snapshots 28.09. 12:00 + 29.09. 00:00 UTC.
- **Lage:** (gemessen 2026-09-28 ~06:20Z via CI-Timestamps) Termin noch nicht fällig — der 29.09.-Snapshot existiert noch nicht; Fill-Bin `tools/measure/src/bin/flyby_path2_fill.rs` gebaut (0 Warnungen), Tube ±12 h, Perigäum-Zelle 13, alle Zellen `pending`.
- **Blockade:** 29.09.-Snapshot fehlt; kein Workflow registriert — der Lauf muss zum Zeitpunkt erfolgen.
- **Braucht:** nach 29.09. 00:00 UTC `sfetch https://services.swpc.noaa.gov/json/rtsw/rtsw_mag_1m.json` + `…/rtsw_wind_1m.json` → `data/services.swpc.noaa.gov/`, dann `cargo run -p omegaflow-measure --bin flyby_path2_fill -- --flyby juice --snapshots data/services.swpc.noaa.gov/`.

### Flyby-Path-2 — DSN-Status am Perigäum
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** 2026-09-28, ~11:45 UTC (Perigäum).
- **Lage:** (gemessen 2026-09-27 via `archive_search --playwright` + `sfetch eyes.nasa.gov/dsn/data/dsn.json`) `pending` — die Seite rendert keine lesbare Tracking-Tabelle (JS/canvas); der Live-Feed (09:48:57Z) trägt keinen JUICE-Eintrag.
- **Blockade:** keine.
- **Braucht:** am 28.09. `archive_search --playwright https://eyes.nasa.gov/dsn/dsn.html` — „tracked (Station X, Band Y)" oder „nicht getrackt".

### Flyby-Path-2 (revised) — δ gemessen, Benotung offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neuer `release` der Post-Flyby-Ephemeride (CDN `ephemeris_juice.bin`) samt 1-σ-Kovarianz — Wochen nach dem Vorbeiflug.
- **Lage:** (gemessen 2026-09-27 via `cargo build` + Bin-Lauf) `flyby_ephemeris_gate` gebaut; beide Zeugen lesen `placed`; **δ = 0,1684732 km** (168 m, DE441 vs DE442; `phi/sources.φ:3448`/`:3469`). Δ/σ_recon `pending` — die Post-Flyby-Daten fehlen.
- **Blockade:** keine.
- **Braucht:** nach dem Flyby `--recon` (Post-Flyby-Arc) + `--sigma-recon` (veröffentlichte 1-σ) → `cargo run -p omegaflow-measure --bin flyby_ephemeris_gate -- --recon <arc> --sigma-recon <km>`; Riß gegen **beide** Hashes („aeb3c82f…" sealed, „eee376ef…" CDN) tragen.

### Total-Coherence — Complexity-Term pending
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** eine physikalische Definition + Datenquelle für `complexity`.
- **Lage:** (gemessen 2026-09-28, River-Folge 51, Rat) der per-Oszillator-Integral ist in den Live-Pfad gebaut (`omega.rs` Breath-Zweig; Gate-Test `the_no_te_tick_hears_each_oscillator_not_the_sum`); der Complexity-Term `1/(1+Σ(complexity·weight)/Σ(weight))` hat keine Definition und keine Datenquelle — die Legacy-Identifier `complexity`/`takens` erscheinen nicht im Baum.
- **Blockade:** die physikalische Definition des Terms fehlt; eine Datenquelle dafür existiert nicht.
- **Braucht:** `complexity` als gemessene Größe definieren (Rat); bis dahin `pending` (Registerzeile, nie gebaut).

## An Mountain (gemessen, fremde Feder)

- (gemessen 2026-09-27 via Rat + general-read-only) Der **81-Block-Fix**
  (`format ephemeris_binary`, `ttl 86400`) gehört Mountain: Release-skalen-
  Prüfintervall, gemessen aus Live-Release-Abständen oder als
  Potenz-von-2-Untergrenze 2²⁵ s ≈ 388 d. **AGENTS.md-Präzisierung im selben Atom**
  (Prinzip: gemessene Quellen-Eigenschaft → Mountain; Materialisierung/Transport →
  Mycelium). Destination: `docs/handover/handover-2026-09-28-mountain-folge193.md`,
  Herkunft: river folge51/52/53. Die ttl-Verdikt-Zeile + das no-cadence-Sprachloch
  wurden bereits gefaltet.

## An Future (Operator-Akt — Absender-Zeile, Aufenthalt = Eigentum)

| Punkt | Destination | Herkunft | Lage |
|---|---|---|---|
| Blatt-Zuschnitt — welches Blatt-Paar (ENSO Wind↔SST / Bz→Kp / LAIC) an `te_probe` gebunden wird | Future-Operator-Queue | river folge51/52/53 (`## Offen`, Blatt-Membran-Bindung (4)) | Operator-Wort; die geerbten Pflichten (1)–(3) sind gebaut (`te.rs`), die Paar-Registrierung am Einstieg `te_probe` bleibt bis zum Zuschnitt ungebaut (`omega.rs` feed = `probe_ring`) |

## Verweise (Prosa mit offenen Markern)

- `docs/auftrag/auftrag-gic-einreichung.md` — Einreich-Paket (Träger des gic-Punkts; Vorbereitung die Kante erreicht, Einreichung in Future-Queue #8).
- `docs/paper/gic-causal-driver.md` — GIC-Papier (paper-check grün; Vorbereitung für Einreichung).
- `docs/paper/flyby-path-2-preregistration-revised.md` — revidierte Präregistrierung (Träger dieses Punkts).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` — presence-only Ladearchitektur.
- `docs/surveys/survey-messpunkt-verteilung.md` — Messpunkt-Verteilung (D5-Marker gemessen, offene Kandidaten-Fragen §9).
- `docs/auftrag/auftrag-flyby2-kette.md` — getragen durch die bestehenden Flyby-Path-2-Punkte (Füll-Lauf, DSN-Status, revised); die Kette selbst steht, keine Duplikat-Zeile.
- `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md` — Silence-Map, Betti-0, Delay und Minkowski sind gebaut; Certainty `descoped`, Total-Coherence-Integral gebaut (Complexity `pending`) — Blöcke oben.
- `docs/concepts/blatt-papier-beweis.md` — die Rat-Reihenfolge der geerbten Pflichten (1)–(3) ist gebaut (`b45550527`); der benannte Paritäts-Check ist in CI **rot** (s. eigener Punkt) — die Membran-Parität ist nicht belegt; Membran-Bindung `pending` auf den Zuschnitt (4), Operator-Wort.
- `docs/surveys/survey-fortschritt.md` — §C je Punkt gemessen: Deep-Lieferung/Zell-Achse-Quantisierung/Relay-Trailer/Fovea `descoped`, Deep-Upload-Stille/Rgba8Unorm `ueberholt`.
- `docs/concepts/kybernetische-astrophysik.md` — lebendes Konzept (12 Nadeln); die Marker sind konzeptionelle `pending`-Prosa, kein neuer Handlungsschritt.
- `docs/concepts/pfeiler-der-architektur.md` — kein offener Punkt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
