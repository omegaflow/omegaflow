<!--
  title: Handover — River-Folge 46 (2026-09-27)
  session: River-Folge 46
  class: handover
  date: 2026-09-27
  sha256: 282c1facf53704d48d055b60bba07892853a09bca861722f99ca5f1cb96bafdc
  status: live
-->
# Handover — River-Folge 46 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Nur eigene Arbeit: pfad-begrenzter Commit, fremde uncommittete
Arbeit unangetastet; gepusht wird, sobald `origin/main` Vorfahr von HEAD ist.
Kein Rang; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** (mit Messstempel) /
**Blockade** / **Braucht**. Der Stehende Pass wird zitiert, nie kopiert:
`state/zustand/standing-pass.md`.

Diese Session konsumierte `handover-2026-09-27-river-folge45.md` (nach `archiv/`).

## Operator-Wort-Register

- gic-Paper-Einreichung: höchste Priorität | 2026-09-27 | Operator-Wort.
- Geräte-Zugriff: vor jedem Zugriff fragen (adb/BT) | 2026-09-26 | Operator-Wort.
- Harte-Läufe-LOCK aufgehoben | 2026-09-26 | Operator-Wort.
- HTTPS ja | 2026-09-26 | Operator-Wort folge36.
- „die Kante bin ich" — jede Linie arbeitet bis zur Kante des Operators; Wert, Wort, Dritt-Akt und Send bleiben seine Hand | 2026-09-27 | Operator (Future-Session).
- ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben — Verbreitung im selben Atom | 2026-09-27 | Operator (Future-Session).
- RX100-Kalibrierer descoped — „über exif weg": Luminanz über den Kamera-EXIF-Weg (K=12.5), die K-Messung ist Rivers Schritt | 2026-09-27 | Operator (Future-Session).
- vC 945 von Mantis Shrimp getrennt | 2026-09-26 | Operator-Wort folge36.
- Einzelbefehle liefern | 2026-09-26 | Operator-Wort folge36.
- UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | 2026-09-27 | Operator (Session, Mountain).
- Entscheidungen nie als Liste vorlegen — eine Liste ist keine Entscheidungshilfe; jede Entscheidung braucht eine aussagekräftige Erklärung | 2026-09-27 | Operator (Future-Session).
- D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | 2026-09-27 | Operator (Session, Mountain).
- „Du kannst" River-Folge 46 — Plan ausführen, an die Taucher delegieren; flash-first, kein Pro-Solo | 2026-09-27 | session-weiter Delegations-Consent (`/consent`), nicht das Commit-Wort.
- RX100 war nur ein Gedanke — Quelle zurückgezogen, Code-Entfernung offen | 2026-09-27 | Operator (Session, Mountain).

## Verweise (Prosa mit offenen Markern)

- `docs/auftrag/auftrag-gic-einreichung.md` — Einreich-Paket (Träger des gic-Punkts).
- `docs/paper/gic-causal-driver.md` — GIC-Papier.
- `docs/paper/flyby-path-2-preregistration-revised.md` — revidierte Präregistrierung (Träger dieses Punkts).
- `docs/paper/flyby-path-2-preregistration.md` — Siegel (Trajektorie-Hashes, unberührt).
- `docs/paper/flyby-path-2-falsification-metric-addendum.md` — superseded als Benotungsinstanz.
- `docs/auftrag/auftrag-flyby2-kette.md` — Füll-Kette (deskriptiv, nicht mehr Benotungsinstanz).
- `docs/surveys/survey-2026-09-23-geraete-anbindung-radiatoren.md` — Geräte-Inventare, offene Messpunkte.
- `docs/surveys/survey-2026-09-20-browser-anbindung.md` — vier Browser-Pfade.
- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` — presence-only Ladearchitektur.

## Offen (aufgeschlüsselt)

### GIC-Paper-Einreichung — Vorbereitung fertig, Kante beim Operator
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort „gic einreichen" (Future-Operator-Queue #11), nach der Gegenlesung (Future-Queue #10).
- **Lage:** (gemessen 2026-09-27 via `sread` + `omega_sh sha` + `cargo run … export_latex -- --check`) die sechs AGU-Lücken sind **geschlossen**: Key Points (`gic-causal-driver.md:19`), Plain Language Summary (`:25`), Open Research mit Data- + Software-Availability (`:485`), COI (`:505`), Acknowledgements (`:509`); die frühere „FEHLT"-Liste im Auftrag war ungemessen und widerlegt. Provider-URLs (SWPC/CDAWeb/INTERMAGNET, je HTTP 200 via `archive_search --verdict`) + Software-Repo-URL (`github.com/omegaflow/omegaflow`) eingetragen; 6 Gutachter-Kandidaten mit Affiliation/E-Mail/ORCID vorbereitet (`state/paper/gic-gutachter-2026-09-27.md`, privat); Auftrag auf den gemessenen Stand gezogen (`auftrag-gic-einreichung.md`, sha `d2ecfbac…`); `export_latex --check` für `gic-causal-driver` grün.
- **Blockade:** keine (autonome Vorbereitung abgeschlossen).
- **Braucht:** Operator-Wort; Reihenfolge steht in Future-Queue #10 (anonymisierte Gegenlesung) → #11 (GEMS-Einreichung + ESSOAr, Operator-Hand). Kein Send durch die Maschine.

### Flyby-Path-2 — Füll-Lauf
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** Perigäum 2026-09-28 **11:45:12 UTC ± 10 s** (geozentrische Distanz ≈ 0,00010039 AU = 15 018 km; JUICE ist Horizons `-28`).
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
- **Braucht:** nach dem Flyby `--recon` (Post-Flyby-Arc) + `--sigma-recon` (veröffentlichte 1-σ) → `cargo run -p omegaflow-measure --bin flyby_ephemeris_gate -- --recon <arc> --sigma-recon <km>`; Riß gegen **beide** Hashes (`aeb3c82f…` sealed, `eee376ef…` CDN) tragen.
## D5-Orphan-Residuum (gefaltet 2026-09-27, Mycelium)

Genuin-offene Punkte trägerloser Docs mit Rivers Natur (Messproben/Rendering).
Quelle: `state/mountain-185-orphan-doc-nachzug.patch` + `register_lookup --orphan-docs`.

- `survey-2026-09-17-omegaflow-legacy-konzepte.md:40,60-64` — Minkowski-4D-Gewichtung (ds²): kein `minkowski` in `src` → Δ-Probe mit/ohne ds².
- `…-legacy-konzepte.md:45,104` — Delay Spectrum (lag-Matrix): nicht portiert → measure-Probe bauen.
- `…-legacy-konzepte.md:46` — Total Coherence Integration: teilweise (Permeability TE-getrieben) → Integral messen.
- `…-legacy-konzepte.md:111` — SI-Wahrheits-Konsole: `force_type` → Einheit statt Debug-Werte → im Anzeigepfad bauen.
- `survey-messpunkt-verteilung.md:88` — Multipol-Fehler je Cluster-Radius/Kernel: Kandidat 6 verworfen → Fixture messen.
- `survey-messpunkt-verteilung.md:100` — R_struct-Verlauf je 7 Kernel-Formen → numerisch bestätigen.
- `survey-messpunkt-verteilung.md:94` — a-posteriori hierarchisch (Quadtree/CVT/Sparse Grids) → Messung gegen analytische Platzierung.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
