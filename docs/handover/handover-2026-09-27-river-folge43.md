<!--
  title: Handover — River-Folge 43 (2026-09-27)
  session: River-Folge 43
  class: handover
  date: 2026-09-27
  sha256: 47050c3764a6ca70b2259a9b53b74d852f0349ddb1785b512e1b911761df98c4
  status: live
-->
# Handover — River-Folge 43 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Nur eigene Arbeit: pfad-begrenzter Commit, fremde uncommittete
Arbeit unangetastet; gepusht wird, sobald `origin/main` Vorfahr von HEAD ist.
Kein Rang; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** (mit Messstempel) /
**Blockade** / **Braucht**. Der Stehende Pass wird zitiert, nie kopiert:
`state/zustand/standing-pass.md`.

Diese Session konsumierte `handover-2026-09-27-river-folge42.md` (nach `archiv/`).

## Operator-Wort-Register

- gic-Paper-Einreichung: höchste Priorität | 2026-09-27 | Operator-Wort.
- RX100-K-Beschaffung: kein Kauf vor Förderung | 2026-09-27 | Operator-Wort (Träger `mountain-folge176.md:32`).
- Geräte-Zugriff: vor jedem Zugriff fragen (adb/BT) | 2026-09-26 | Operator-Wort.
- Harte-Läufe-LOCK aufgehoben | 2026-09-26 | Operator-Wort.
- HTTPS ja | 2026-09-26 | Operator-Wort folge36.
- vC 945 von Mantis Shrimp getrennt | 2026-09-26 | Operator-Wort folge36.
- Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36.
- Mantis Shrimp LOCK | 2026-09-26 | Operator-Wort folge36.
- „Du kannst" River-Folge 43 | 2026-09-27 | session-weiter Delegations-Consent (`/consent`), nicht das Commit-Wort.

## Verweise (Prosa mit offenen Markern)

- `docs/paper/flyby-path-2-preregistration-revised.md` — revidierte Präregistrierung (neu; Träger dieses Punkts).
- `docs/paper/flyby-path-2-preregistration.md` — Siegel (Trajektorie-Hashes, unberührt).
- `docs/paper/flyby-path-2-falsification-metric-addendum.md` — superseded als Benotungsinstanz.
- `docs/auftrag/auftrag-flyby2-kette.md` — Füll-Kette.
- `docs/auftrag/auftrag-gic-einreichung.md` — Einreich-Paket (Träger des gic-Punkts).
- `docs/paper/gic-causal-driver.md` — GIC-Papier.
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` — Geräte-Inventare, offene Messpunkte (Z. 346-352).
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` — vier Browser-Pfade.
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` — presence-only Ladearchitektur.

## Offen (aufgeschlüsselt)

### Flyby-Path-2 — Füll-Lauf
- **Status:** termin:2026-09-28 | **Bindung:** termin
- **Trigger:** Perigäum 2026-09-28 **11:45:12 UTC ± 10 s** (gemessen 2026-09-27 via JPL Horizons `COMMAND='-28'`, `CENTER='500@399'`, `QUANTITIES='20'`; geozentrische Distanz ≈ 0,00010039 AU = 15 018 km. **JUICE ist `-28`, nicht `-61`** — `-61` löst zu Juno auf).
- **Lage:** (gemessen 2026-09-27 via `cargo build`) Fill-Bin `tools/measure/src/bin/flyby_path2_fill.rs` gebaut (0 Warnungen); Tube ±12 h; Perigäum-Zelle 13, alle Zellen `pending` — ehrlicher Vor-Flyby-Zustand.
- **Blockade:** kein Workflow registriert — der Lauf muss zum Zeitpunkt erfolgen.
- **Braucht:** RTSW-Snapshots ~28.09. 12:00 + 29.09. 00:00 UTC (`sfetch https://services.swpc.noaa.gov/json/rtsw/rtsw_mag_1m.json` + `…/rtsw_wind_1m.json` → `data/services.swpc.noaa.gov/`), dann `cargo run -p omegaflow-measure --bin flyby_path2_fill -- --flyby juice --snapshots data/services.swpc.noaa.gov/`.

### Flyby-Path-2 — DSN-Status am Perigäum
- **Status:** termin:2026-09-28 | **Bindung:** termin
- **Trigger:** 2026-09-28.
- **Lage:** (gemessen 2026-09-27 via `archive_search --playwright` + `sfetch eyes.nasa.gov/dsn/data/dsn.json`) `pending` — die Seite rendert keine lesbare Tracking-Tabelle (JS/canvas); der Live-Feed (09:48:57Z) trägt keinen JUICE-Eintrag. Tracking ja/nein ist daraus nicht messbar.
- **Blockade:** keine.
- **Braucht:** am 28.09. erneut `archive_search --playwright https://eyes.nasa.gov/dsn/dsn.html` (bzw. die DSN-Statusseite) — „tracked (Station X, Band Y)" oder „nicht getrackt".

### Flyby-Path-2 (revised) — Gate-Bin + Benotung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neuer `release` der Post-Flyby-Ephemeride (CDN `ephemeris_juice.bin`) samt 1-σ-Kovarianz — Wochen nach dem Vorbeiflug.
- **Lage:** (gemessen 2026-09-27 via Rat + `omega_sh sha`) revidierte Präregistrierung versiegelt (`docs/paper/flyby-path-2-preregistration-revised.md`, body sha `f084a151…`); Fehlschlag-Regel Δ > δ + 3·σ_recon fixiert. Das Gate-Bin `flyby_ephemeris_gate` existiert nicht; δ und σ_recon `pending`.
- **Blockade:** δ braucht die DE441/DE442-Kernel-Registrierung — als eigener Punkt an Mycelium getragen (`handover-2026-09-27-mycelium-folge179.md`).
- **Braucht:** `flyby_ephemeris_gate` in `tools/measure` bauen; δ messen; bei Publikation Δ gegen **beide** Hashes (`aeb3c82f…` sealed, `eee376ef…` CDN) laufen lassen, Riß tragen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite Consent (Delegation), nie das Commit-Wort.
