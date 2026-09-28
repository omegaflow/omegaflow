<!--
  title: Handover — River-Folge 52 (2026-09-28)
  session: River-Folge 52
  class: handover
  date: 2026-09-28
  sha256: 7959069e51aa845915becdfdf7e69721b74ce5703238e65c43cac41859a6744d
  status: live
-->
# Handover — River-Folge 52 (2026-09-28)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert:
`state/zustand/standing-pass.md`. Nur eigene Arbeit: pfad-begrenzter Commit,
fremde uncommittete Arbeit unangetastet.

Diese Session konsumierte `handover-2026-09-28-river-folge51.md` (nach `archiv/`).
In diesem Atom: den benannten Bau-Atom-Check der Blatt-Membran-Bindung gebaut —
`gate_scalar_probe_path_parity_with_topological_membrane` (`src/mathematikerin/te.rs`):
Richtung + Signifikanz des skalaren Probe-Pfads (`transfer_entropy_lag`) gegen die
topologische Membran (`topological_te_phase`) auf der kausalen Fixture; `cargo check`
0 Fehler / 0 Warnungen. Der Lauf steht in CI.

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

## Offen (aufgeschlüsselt)

### GPU-Readback map/unmap + Lade-Membran — CI-Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein ci-check-Lauf auf einem HEAD ≥ Readback-Fix, der **abschließt**.
- **Lage:** (gemessen 2026-09-28 ~06:10Z via `ci_manage status`) `36377277112`/`36380327273` sind `cancelled` (HEAD-Vorlauf, kein Code-Rot); der Push dieses Atoms (`39ba46b13`) startete `36385116484` ci-check (pending). Kein abgeschlossener grüner Nachweis im Fenster. Die Lade-Membran ist lokal headless grün (folge50).
- **Blockade:** keine.
- **Braucht:** `ci_manage log 36385116484` **einmal** nach Laufende (kein Polling) — grün = Punkt geschlossen; rot = `ci_manage log <id>` auswerten.

### Flyby-Path-2 — Füll-Lauf
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** Perigäum 2026-09-28 **11:45:12 UTC ± 10 s** (geozentrisch ≈ 0,00010039 AU = 15 018 km; JUICE ist Horizons `-28`); Snapshots 28.09. 12:00 + 29.09. 00:00 UTC.
- **Lage:** (gemessen 2026-09-28 via CI-Timestamps ≈05:10Z) Termin noch nicht fällig — der 29.09.-Snapshot existiert noch nicht; Fill-Bin `tools/measure/src/bin/flyby_path2_fill.rs` gebaut (0 Warnungen), Tube ±12 h, Perigäum-Zelle 13, alle Zellen `pending`.
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

### Blatt-Membran-Bindung — Paritäts-Check gebaut, Pflichten (1)–(3) offen
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** eigen (die geerbten Pflichten (1)–(3) sind die Arbeit).
- **Lage:** (gemessen 2026-09-28 via general + `cargo check`) Der **benannte Bau-Atom-Check ist gebaut**: `gate_scalar_probe_path_parity_with_topological_membrane` (`src/mathematikerin/te.rs`, ~:3193) prüft Richtung + Signifikanz von `transfer_entropy_lag` (skalarer Probe-Pfad) gegen `topological_te_phase` (Membran) auf der kausalen Fixture; `cargo check` 0/0. Die drei geerbten Pflichten sind in den Blatt-Probes **teilweise** gebaut: (1) BH existiert (`te.rs:1465`), angewandt in `nobel_probe_bz.rs:217/237` + `nobel_probe_laic.rs:197`, andere Probes nutzen max-null-Familien (`bz_blatt_probe.rs:356`, `nobel_probe_corona.rs:739/755`, `pair_te_screen.rs:530`); (2) Lag-Sweep in `bz_blatt_probe.rs:370-395` + `nobel_probe_bz --max-lag`, aber nicht für die Membran (nur MI-Lag); (3) KDE-h-Sensitivität nur skalar (`trishuli_kde_sensitivity_probe.rs`), nicht für `embedded_silverman`/die WGSL-H-Skala. Kein Blatt-Paar ist an `te_probe` gebunden (Feed = `probe_ring`, `omega.rs:1611-1626`).
- **Blockade:** keine für (1)–(3); der Paar-Zuschnitt ist Operator-Wort (Future-Queue).
- **Braucht:** (1) eine gemeinsame BH/Familien-Korrektur über die getestete Paarmenge je Blatt-Probe; (2) Lag-Sweep der Membran über die MI-Lag-Spanne; (3) h-Sensitivitätslauf gegen `embedded_silverman`; (4) Zuschnitt = Operator-Wort → dann Paar-Registrierung am Einstieg `te_probe` (`omega.rs:458`/`:1621`).

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
  Mycelium). Destination: `docs/handover/handover-2026-09-28-mountain-folge192.md`,
  Herkunft: river folge51/52. Die ttl-Verdikt-Zeile + das no-cadence-Sprachloch
  wurden bereits gefaltet.

## An Future (Operator-Akt — Absender-Zeile, Aufenthalt = Eigentum)

| Punkt | Destination | Herkunft | Lage |
|---|---|---|---|
| Blatt-Zuschnitt — welches Blatt-Paar (ENSO Wind↔SST / Bz→Kp / LAIC) an `te_probe` gebunden wird | Future-Operator-Queue | river folge51/52 (`## Offen`, Blatt-Membran-Bindung (4)) | Operator-Wort; bis dahin bleibt die Paar-Registrierung ungebaut (`omega.rs:1621` feed = `probe_ring`) |

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
