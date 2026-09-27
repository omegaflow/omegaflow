<!--
  title: Handover — River-Folge 45 (2026-09-27)
  session: River-Folge 45
  class: handover
  date: 2026-09-27
  sha256: 385c51cf0dedeb0a009e08acb7b99a55ed1621cbf0011d588e68577851494a45
  status: live
-->
# Handover — River-Folge 45 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Nur eigene Arbeit: pfad-begrenzter Commit, fremde uncommittete
Arbeit unangetastet; gepusht wird, sobald `origin/main` Vorfahr von HEAD ist.
Kein Rang; jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** (mit Messstempel) /
**Blockade** / **Braucht**. Der Stehende Pass wird zitiert, nie kopiert:
`state/zustand/standing-pass.md`.

Diese Session konsumierte `handover-2026-09-27-river-folge44.md` (nach `archiv/`).

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
- „Du kannst" River-Folge 45 — Plan ausführen, an die Taucher delegieren | 2026-09-27 | session-weiter Delegations-Consent (`/consent`), nicht das Commit-Wort.
- UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | 2026-09-27 | Operator (Session, Mountain).
- D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | 2026-09-27 | Operator (Session, Mountain).

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
- **Status:** termin | **Bindung:** termin
- **Trigger:** Perigäum 2026-09-28 **11:45:12 UTC ± 10 s** (gemessen 2026-09-27 via JPL Horizons `COMMAND='-28'`, `CENTER='500@399'`, `QUANTITIES='20'`; geozentrische Distanz ≈ 0,00010039 AU = 15 018 km. **JUICE ist `-28`, nicht `-61`** — `-61` löst zu Juno auf).
- **Lage:** (gemessen 2026-09-27 via `cargo build`) Fill-Bin `tools/measure/src/bin/flyby_path2_fill.rs` gebaut (0 Warnungen); Tube ±12 h; Perigäum-Zelle 13, alle Zellen `pending` — ehrlicher Vor-Flyby-Zustand.
- **Blockade:** kein Workflow registriert — der Lauf muss zum Zeitpunkt erfolgen.
- **Braucht:** RTSW-Snapshots ~28.09. 12:00 + 29.09. 00:00 UTC (`sfetch https://services.swpc.noaa.gov/json/rtsw/rtsw_mag_1m.json` + `…/rtsw_wind_1m.json` → `data/services.swpc.noaa.gov/`), dann `cargo run -p omegaflow-measure --bin flyby_path2_fill -- --flyby juice --snapshots data/services.swpc.noaa.gov/`.

### Flyby-Path-2 — DSN-Status am Perigäum
- **Status:** termin | **Bindung:** termin
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

### clippy `-D warnings` — River-Dateien (geheilt, CI-Bestätigung offen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`-Lauf am neuen HEAD (nach dem River-Commit) lesbar.
- **Lage:** (gemessen 2026-09-27 via `cargo check`) neun Stellen geheilt: `src/archivar/main_flow.rs` `PresenceGateCtx` bündelt `raw_presence_gate` (8→4 Argumente), `?`-Operator an `Frame::Surface`/`Frame::Barycenter`/`acoustic_sink`; `src/mathematikerin/actuators.rs` `as_chunks::<4>().0` (543/569/644); `src/mathematikerin/tests.rs` `as_chunks[_mut]::<4>().0` (1978/2107); `src/archivar/tests.rs` Aufrufer angepasst. `cargo check` 0 Fehler/0 Warnungen. Der locale clippy lief nicht.
- **Blockade:** kein lokales clippy; Lauf am neuen HEAD pending.
- **Braucht:** `ci_manage log <lauf-id>` am neuen HEAD — clippy-Block grün, sonst die Reststellen heilen.

### RX100-Luminanz — EXIF-Weg gebaut, CI-Bestätigung + K-Vorbehalt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check`-Lauf am neuen HEAD (Test-Target) + `cargo test --lib rx100` liest.
- **Lage:** (gemessen 2026-09-27 via `cargo check` + Bin-Lauf) Rat-ratifiziert (Verdikt 2026-09-27) gebaut: `src/archivar/rx100.rs` `exif_exposure` (TIFF-IFD, Exif-IFD-Pointer, beide Endianness; absent → `None`); der SSDP-/Sony-ScalarWebAPI-Block (:72–132) fiel (Konsumentenlosigkeit gemessen); `tools/harvest/src/bin/rx100_compiler.rs` `--jpeg <path>`, Bin-Zeit aus `DateTimeOriginal` (nie `SystemTime::now()`); Band am gemessenen Record-Schreibort `src/archivar/extract.rs:207` `series_rows` → `(5.45e14, 3.2e14)` für `rx100_luminance`, sonst `SPECTRAL_NO_BAND`. Reziprozitäts-Tor + 8 EXIF-Parser-Tests; Einzel-Bin-Lauf auf neutralem Fixture → 58.8 cd/m², Roundtrip parst. **K=12.5 bleibt ISO-2720-Vorgabe, ungemessen** — ohne Referenz-Luminanzmeter nicht messbar; das Reziprozitäts-Tor misst nur die Form. Die Register-Deklaration `phi/sources.φ:122` liegt als Direkt-Edit in Mountain-Folge 183 (Feld-Slots `freq`/`bin_width` absent gegen den geschriebenen Draht).
- **Blockade:** `cargo test` ist lokal verweigert; die Tests laufen erst im CI-Testlauf nach dem Commit.
- **Braucht:** `ci_manage log <lauf-id>` am neuen HEAD (Test-Target `cargo test --lib rx100`); ein reales RX100-JPEG für die CDN-Manifestation ist Operator-Hand (Future-Queue #7, `RX100 VA vorhanden`).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite Consent (Delegation), nie das Commit-Wort.
