<!--
  title: Handover — River-Folge 48 (2026-09-27)
  session: River-Folge 48
  class: handover
  date: 2026-09-27
  sha256: 68ba1c27c10e9e3ad08f7af66981a891913cb8c13d9a10deccb9f076878525fb
  status: live
-->
# Handover — River-Folge 48 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert:
`state/zustand/standing-pass.md`. Nur eigene Arbeit: pfad-begrenzter Commit,
fremde uncommittete Arbeit unangetastet.

Diese Session konsumierte `handover-2026-09-27-river-folge47.md` (nach `archiv/`).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„du sollst keine operator worte tragen das ist sache von future du bist bis zur kante" | 2026-09-27 | Operator (Session, River 48)
„warum holst du den si konsolen punkt schon wieder hoch — du hast doch schon den rat einberufen; rat und operator sind zwei paar schuhe" | 2026-09-27 | Operator (Session, River 48)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt) … Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort." | 2026-09-27 | Operator (Session, River 48)
„Erste Handlung: `sread docs/concepts/tool-forms.md`" | 2026-09-27 | Operator (Session, River 48)
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
„Du kannst" River-Folge 46/47 — Delegations-Consent, nicht das Commit-Wort | 2026-09-27 | session-weiter Consent (`/consent`)
Commit-Wort (`/commit`) — pfad-begrenzter Commit + Push, das Doppel-Ask | 2026-09-27 | Operator

## Offen (aufgeschlüsselt)

### GPU-Readback map/unmap — Fix gebaut, CI-Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-check`-Lauf am Commit dieser Session.
- **Lage:** (gemessen 2026-09-27 via Taucher-Messung + `cargo check` 0/0) die vier map/unmap-Lücken sind geschlossen: `omega.rs:955` (`probe_read.unmap()` je Aufruf, Erfolg und Timeout), `omega.rs:1180` (`read_buf.unmap()` im `s2_read_gpu`-Timeout), `scalar_te_gpu.rs:172` (Unmap im `run`-Timeout), `machines/matrix.rs:910` (Stale-Zweig unmappt `te_read_buf` statt ihn gemappt zu verwerfen). `te_probe`/`solar.rs` halten den Pufferzustand explizit, kein Doppel-Unmap. Der Fix ist **uncommittet** — die CI prüft ihn erst nach dem Push.
- **Blockade:** keine.
- **Braucht:** nach dem Push `gh workflow run ci-check.yml`; Ergebnis **einmal** lesen — `ci_manage view <id>` / `ci_manage log <id>` (kein Polling).

### Flyby-Path-2 — Füll-Lauf
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** Perigäum 2026-09-28 **11:45:12 UTC ± 10 s** (geozentrisch ≈ 0,00010039 AU = 15 018 km; JUICE ist Horizons `-28`).
- **Lage:** (gemessen 2026-09-27 via `cargo build`) Fill-Bin `tools/measure/src/bin/flyby_path2_fill.rs` gebaut (0 Warnungen); Tube ±12 h; Perigäum-Zelle 13, alle Zellen `pending` — ehrlicher Vor-Flyby-Zustand.
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

## An Mountain (gemessen, fremde Feder)

- (gemessen 2026-09-27 via Rat + general-read-only) **`ttl` in `phi/sources.φ` ist eine Verdikt-Zeile → Mountain, alleiniger Schreiber.** Rat 2026-09-27: das Operator-Wort („die ttl muss die Aktualisierung der Quelle sein"), `derive_ttl` (`src/archivar/port.rs:1237`) als Messinstrument aus der Quelle, das Flush-Gate (`src/archivar/parse.rs:80`) und der Backoff (`src/archivar/fetch.rs:298`, `2^failures`) tragen die operative Abweichung — Myceliums Liste bleibt `url`/`origin`/`compiler`/Tags. Kein permanenter Riss, kein Split-Modell.
- Der **81-Block-Fix** (`format ephemeris_binary`, `ttl 86400`) gehört Mountain: Release-skalen-Prüfintervall, gemessen aus Live-Release-Abständen oder als Potenz-von-2-Untergrenze 2²⁵ s ≈ 388 d. **AGENTS.md-Präzisierung im selben Atom** (Prinzip: gemessene Quellen-Eigenschaft → Mountain; Materialisierung/Transport → Mycelium).
- **Sprachloch:** kein „no-cadence"-Zustand im Parser (`ttl 0` → inaktiver Block, `parse.rs:80`) — als `pending` registrieren, nie mit einem falschen Puls übertünchen.

## Verweise (Prosa mit offenen Markern)

- `docs/auftrag/auftrag-gic-einreichung.md` — Einreich-Paket (Träger des gic-Punkts; Vorbereitung die Kante erreicht, Einreichung in Future-Queue #8).
- `docs/paper/gic-causal-driver.md` — GIC-Papier (13 Gegenlesungs-Punkte eingearbeitet 2026-09-27).
- `docs/paper/flyby-path-2-preregistration-revised.md` — revidierte Präregistrierung (Träger dieses Punkts).
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` — presence-only Ladearchitektur.
- `docs/surveys/survey-messpunkt-verteilung.md` — Messpunkt-Verteilung (D5-Marker gemessen, offene Kandidaten-Fragen §9).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
