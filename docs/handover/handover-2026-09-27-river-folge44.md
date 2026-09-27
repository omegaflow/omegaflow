<!--
  title: Handover — River-Folge 44 (2026-09-27)
  session: River-Folge 44
  class: handover
  date: 2026-09-27
  sha256: ef7b4ad1cdf5c886788b0691ba0cfc7b982e105f986bcd8ba0af1e7f09f60e99
  status: live
-->
# Handover — River-Folge 44 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Nur eigene Arbeit: pfad-begrenzter Commit, fremde uncommittete
Arbeit unangetastet; gepusht wird, sobald `origin/main` Vorfahr von HEAD ist.
Kein Rang; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** (mit Messstempel) /
**Blockade** / **Braucht**. Der Stehende Pass wird zitiert, nie kopiert:
`state/zustand/standing-pass.md`.

Diese Session konsumierte `handover-2026-09-27-river-folge43.md` (nach `archiv/`).

## Operator-Wort-Register

- gic-Paper-Einreichung: höchste Priorität | 2026-09-27 | Operator-Wort.
- Geräte-Zugriff: vor jedem Zugriff fragen (adb/BT) | 2026-09-26 | Operator-Wort.
- Harte-Läufe-LOCK aufgehoben | 2026-09-26 | Operator-Wort.
- HTTPS ja | 2026-09-26 | Operator-Wort folge36.
- „die Kante bin ich" — jede Linie arbeitet bis zur Kante des Operators; Wert, Wort, Dritt-Akt und Send bleiben seine Hand | 2026-09-27 | Operator (Future-Session).
- ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben — Verbreitung im selben Atom | 2026-09-27 | Operator (Future-Session).
- vC 945 von Mantis Shrimp getrennt | 2026-09-26 | Operator-Wort folge36.
- Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36.
- „Du kannst" River-Folge 44 | 2026-09-27 | session-weiter Delegations-Consent (`/consent`), nicht das Commit-Wort.

## Verweise (Prosa mit offenen Markern)

- `docs/paper/flyby-path-2-preregistration-revised.md` — revidierte Präregistrierung (Träger dieses Punkts).
- `docs/paper/flyby-path-2-preregistration.md` — Siegel (Trajektorie-Hashes, unberührt).
- `docs/paper/flyby-path-2-falsification-metric-addendum.md` — superseded als Benotungsinstanz.
- `docs/auftrag/auftrag-flyby2-kette.md` — Füll-Kette (deskriptiv, nicht mehr Benotungsinstanz).
- `docs/auftrag/auftrag-gic-einreichung.md` — Einreich-Paket (Träger des gic-Punkts).
- `docs/paper/gic-causal-driver.md` — GIC-Papier.
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` — Geräte-Inventare, offene Messpunkte.
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

### Flyby-Path-2 (revised) — δ gemessen, Benotung offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein neuer `release` der Post-Flyby-Ephemeride (CDN `ephemeris_juice.bin`) samt 1-σ-Kovarianz — Wochen nach dem Vorbeiflug.
- **Lage:** (gemessen 2026-09-27 via `cargo build` + Bin-Lauf) `flyby_ephemeris_gate` gebaut (`tools/measure/src/bin/flyby_ephemeris_gate.rs`, 0 Warnungen). Beide Zeugen lesen `placed`: sealed `aeb3c82f…`, renewed `eee376ef…`; gemeinsames Band aus Juice/Earth-Schnittmenge, 997 h-Samples. **δ = 0,1684732 km** (168 m, DE441 vs DE442; Quellen `phi/sources.φ:3448`/`:3469`, sealed Juice — die Editionen sind bereits registriert). Register `data/flyby2/gate-juice-2026-09-28.json` (lokal, gitignored). Δ/σ_recon `pending` — die Post-Flyby-Daten fehlen.
- **Blockade:** keine.
- **Braucht:** nach dem Flyby `--recon` (Post-Flyby-Arc) + `--sigma-recon` (veröffentlichte 1-σ) → `cargo run -p omegaflow-measure --bin flyby_ephemeris_gate -- --recon <arc> --sigma-recon <km>`; Riß gegen **beide** Hashes (`aeb3c82f…` sealed, `eee376ef…` CDN) tragen.

### clippy `-D warnings` — River-Dateien (getragen von Mountain)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`-Lauf am HEAD `ab0a1faa` (`36315178308`, pending) lesbar.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36310976945`, SHA `452d406d`, + `sgrep` am HEAD) `src/archivar/main_flow.rs` `too_many_arguments` (`:139`, 8/7) + `question_mark` (`:181`, `:189`, `:309`); `src/mathematikerin/actuators.rs` `chunks_exact_to_as_chunks` (`:543`, `:569`, `:644`); `src/mathematikerin/tests.rs` `chunks_exact_to_as_chunks` (`:1978` `chunks_exact_mut`, `:2107`).
- **Blockade:** kein lokales clippy.
- **Braucht:** `ci_manage log 36315178308` am HEAD lesen; die genannten Zeilen heilen (`chunks_exact(N)` → `as_chunks::<N>().0` bzw. `as_chunks_mut::<N>().0`, `?`-Operator, Argumente bündeln).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite Consent (Delegation), nie das Commit-Wort.
