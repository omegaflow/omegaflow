<!--
  title: Handover — River-Folge 28 (2026-09-25)
  session: River-Folge 28
  class: handover
  date: 2026-09-25
  sha256: 4995967fd4b87335405d726c1d6ce708f1e89516794d7030cd21ecc485fc932f
  status: live
-->
# Handover — River-Folge 28 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks — committet wird pfad-begrenzt, fremde
uncommittete Arbeit wird nie überschrieben; gepusht wird, sobald der eigene
Commit steht und `origin/main` Vorfahr von HEAD ist.

Es gibt keine Rangfolge und keinen `härtesten Punkt` — die offenen Punkte werden
**parallel** von Agenten abgearbeitet. Jeder Punkt trägt **Trigger** / **Lage**
(mit Messstempel) / **Blockade** / **Braucht**. Sortierung: **erst logisch nach
Akteur (wer handelt) — Linie | Rat | Operator | Dritter —, dann chronologisch
(Messdatum)**. **Vorbereitung ≠ Akt:** ein operator-gebundener Punkt wird
getrennt geführt — Vorbereitung (autonom, dispatcht) und Akt (operator-gebunden).

## Stehender Pass (gemessen 2026-09-25, River-Folge 28)

- **HEAD** `4f90217ed` („future folge119: merge cdn reconciliation before push");
  `git_safety --snapshot` == HEAD; `open_points_check ….river-folge27.md`: 17
  Pfad-Refs, 0 absent.
- **Postfach** (`mail_digest --last 6`): `state/mail/mail_ledger.φ` lesbar, 6
  Eingänge — Operator-Weiterleitung „WG: SuperDARN Mirror to omegaflow"
  (2026-09-25), GitHub-Support „Re: Request to purge/GC unreachable" (2026-09-25),
  ORCID-Verify, Exa-Login, GitHub-OAuth-App hinzugefügt, Rubin-Forum-Digest
  (2026-09-24). Eingetragen in
  `docs/zustand/external-state.md` (Postfach).
- **CI** (`ci_manage list`/`view`): `ci-check 36152720260` @HEAD `4f90217ed`
  **rot** — `clippy` rot (`src/archivar/port.rs:1731` identische if-Blöcke),
  `test` rot (`archivar::tests::test_diagnose_no_samples`, `src/archivar/tests.rs:6839`);
  `tools-build 36152720047` success; `paper-check`/`corpus-te` rot;
  `health-check 36140394227` **rot** (6/8 `verify` success, externer
  Shutdown); `bz-retro-probe 36136095463` **in_progress** (>5 h). Eintrag in
  `docs/zustand/external-state.md` (CI-Status).
- `register_lookup --open`: 118 Docs, 600 offene Zeilen, **0 `owner=river`**, 1
  unverifiable, 22 zustand due, 19 orphan; pipeline: ledger 2, index 9.

## Offen (erst logisch nach Akteur, dann chronologisch)

### Linie handelt (eigen)

#### gic-causal-driver — Neu-Messung nach Härtung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss des Laufs `bz-retro-probe 36136095463`.
- **Lage:** (gemessen 2026-09-25 via `ci_manage view`) der Lauf ist `in_progress`
  seit 12:38Z (>5 h); die Jobs `hourly`/`minute` tragen noch kein Log/Artefakt.
  Die 7 Härtungen der folge26 stehen im Baum (`cargo check` 0/0).
- **Blockade:** keine — der schwere Lauf gehört nach CI.
- **Braucht:** `ci_manage view 36136095463` nach Abschluss; Artefakt
  `bz-blatt-minute.txt` / der hourly-Lauf.

#### GIC-Paarung — NUR (Nurmijärvi) statt „Mäntsälä"
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 via `research-max` `sfetch`/`archive_search`) die
  Prämisse ist **widerlegt**: die IMAGE-Station `MAS` ist **Masi, Norwegen**
  (69.46 N 23.70 E, `space.fmi.fi/image/www/?page=stations`) — ~1000 km von
  Mäntsälä; ein co-lokiertes Mäntsälä-Magnetogramm existiert nicht (→ der alte
  Punkt ist `descoped`). Der echte co-lokierte Nachbar ist **NUR Nurmijärvi**
  (60.50 N 24.65 E, ≈32 km zum GIC-Ort 60.6/25.2). Route **live**, keyless:
  `https://space.fmi.fi/image/www/data_download.php?starttime=YYYYMMDD[HH]&length=<min>&format=text&stations=NUR&sample_rate=10`
  (2 Kopfzeilen + `YYYY MM DD HH MM SS <code> X Y Z` nT, `99999.9` = Lücke =
  skip; Abdeckung 1992-01-01→heute; Überlapp mit `fmi_gic.bin` 1999–2023).
- **Blockade:** keine — der ABK/SOD-Ersatz (68.4/67.4 N) ist damit ablösbar.
- **Braucht:** NUR-Route in `phi/sources.φ` registrieren (SOURCE_PORT) + dB/dt-
  Paarung in `bz_blatt_probe.rs` (NUR statt ABK/SOD); 10-s nativ → Differenz
  direkt.

#### Minuten-Archiv — OMNI HAPI (`OMNI_HRO_1MIN`)
- **Status:** wartend | **Bindung:** eigen (Source-Port → Mycelium)
- **Trigger:** Harvest/Registrierung des OMNI-HAPI-Assets.
- **Lage:** (gemessen 2026-09-25 via `general`) bester 1-min-Retro-Kandidat:
  `OMNI_HRO_1MIN` über CDAWeb HAPI, **live**:
  `https://cdaweb.gsfc.nasa.gov/hapi/data?id=OMNI_HRO_1MIN&time.min=<ISO>&time.max=<ISO>&format=csv`
  (Info `.../hapi/info?id=OMNI_HRO_1MIN`); Solarwind-Spalten ab 1995 bis
  2026-09-03, 1-min CSV. Bulk-Altweg `.../pub/data/omni/high_res_omni/monthly_1min/omni_minYYYYMM.asc`
  (1981-01→2026-09, fixed-column ASCII). Das 22-h-Fenster ist damit aufhebbar.
- **Blockade:** keine.
- **Braucht:** OMNI-HAPI als Quelle registrieren (SOURCE_PORT, Träger Mycelium) +
  Compiler; dann Minuten-Ensemble.

#### health-check — voller grüner Shard-Lauf
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der nächste abgeschlossene health-check-Lauf.
- **Lage:** (gemessen 2026-09-25 via `ci_manage view 36140394227`) **6/8**
  `verify`-Shards success; `verify (4)` **rot** durch externen
  Runner-Shutdown (`The runner has received a shutdown signal`), `verify (3)`
  cancelled — **kein** Assertion-/Exit-Fehler. Die `concurrency`-Gruppe steht
  auf `cancel-in-progress: false` (`.github/workflows/health-check.yml:3-5`); der
  Stau ist der 3-h-cron gegen 2h16m-Läufe, nicht `cancel-in-progress`.
- **Blockade:** wartet auf einen Lauf ohne externen Shard-Abriss.
- **Braucht:** `ci_manage view <nächster health-check-Lauf>`; bei erneutem
  externen Cancel ist der Beleg GitHub-seitig, kein Baum-Fix.

#### TLS im Relay (wireless) — externer Terminator steht
- **Status:** wartend | **Bindung:** eigen/operator
- **Trigger:** kabelloser Sensor wird gebraucht.
- **Lage:** (2026-09-25 via `sgrep`) Spec `docs/specs/relay-tls-terminator.md` +
  `bin/relay-tls.stunnel.conf` gebaut (stunnel `0.0.0.0:1619` → `127.0.0.1:1618`);
  rustls ist pure Rust, kein Kern-Umbau.
- **Blockade:** geräteseitige CA operator-gebunden.
- **Braucht:** Operator: lokale CA + Leaf-Cert (SAN = LAN-IP), `stunnel
  bin/relay-tls.stunnel.conf`, `bin/omegaflow` ohne `OMEGAFLOW_HIDDEN`,
  `https://<lan-ip>:1619`.

### Operator handelt

#### Sonnenfarbe: erster Lauf `color: measured`
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum sichtbaren/hidden Lauf.
- **Lage:** der Farbmodus ist gebaut (Commit `2de359982`, Ancestor von HEAD);
  Parity-Gate `gpu_lut_index_mirror_matches_color_for_ci` (`spectral.rs:780`)
  grün. **Ungemessen:** die WGSL-Ausführung im Lauf.
- **Blockade:** GPU-Lauf = Heavy compute (Regel: CI/Operator-Wort).
- **Braucht:** der erste Lauf, der `color: measured` rendert — zusammen mit der
  adb-reverse-Route.

#### Sensor-Bindung vC-Permeabilität
- **Status:** operator-gebunden (wartet auf Hardware) | **Bindung:** operator
- **Trigger:** Smartwatch + Mantis-Shrimp sind angeschlossen (Operator-Wort
  2026-09-20: „erst wenn alles fertig ist").
- **Lage:** der Permeabilitäts-Pfad steht (`src/mathematikerin/omega.rs:200/349`),
  der HRV-Reader ist gebaut (`src/archivar/ble.rs:715/926`).
- **Blockade:** Hardware fehlt.
- **Braucht:** `OMEGAFLOW_HIDDEN=1 OMEGAFLOW_PERM_LOG=<pfad> cargo run --release`,
  dann `perm_target_probe --live <pfad>`.

#### Akt: LAN-Sensorik adb-reverse-Route (sichtbarer Lauf)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum sichtbaren Lauf.
- **Lage:** die Vorbereitung steht (gemessen 2026-09-25 via `sread tools-build.yml`):
  Relay kompiliert im PATH-Bin, Bind `0.0.0.0:1618` (`relay.rs:11/73`),
  `hidden = env("OMEGAFLOW_HIDDEN").is_ok()` (`main_flow.rs:612`), Consent
  `/consent?ja` (`relay.rs:396`).
- **Blockade:** Heavy compute (Regel: CI, nie lokal).
- **Braucht:** der **Operator** führt aus: `adb devices && adb reverse tcp:1618
  tcp:1618`, dann `bin/omegaflow` **ohne** `OMEGAFLOW_HIDDEN`; `http://127.0.0.1:1618`
  öffnen, Consent bestätigen, messen: `sensor: N` N>0 **und** Vibration
  (`static/radiator.js:105`).

#### Akt: Sensor-Hardware beschaffen (Bestellung)
- **Status:** operator-gebunden | **Bindung:** operator (Beschaffung)
- **Trigger:** Operator bestellt die BOM und schließt die Hardware an.
- **Lage:** die BOM ist bestellfertig (gemessen 2026-09-25 via
  `archive_search --playwright`, `docs/specs/mantis-shrimp-bom.md`): alle 9
  Outdoor-Positionen mit Item-ID; die EUR-Spalte ist EZB-abgeleitet.
- **Blockade:** Beschaffung/Kosten (Operator-Gegenüber); kein Node-Teil vorhanden.
- **Braucht:** Operator bestellt die BOM-Positionen (AliExpress-Login) und liest
  den EUR-Preis gegen; die Session baut den Node bei gemessenem Vorhandensein.

### Dritter handelt

#### Flyby-Path-2-Kette
- **Status:** termin:2026-09-28 | **Bindung:** termin (Owner Forschung-Linie, archiviert folge140 — Nachfolge-Linie ungeklärt)
- **Trigger:** Perigäum 2026-09-28.
- **Lage:** (gemessen 2026-09-23 via `sread`) die präregistrierte
  JUICE-Erdpassage-Kette wartet auf das Perigäum; RTSW-Retention (1 m mag/wind)
  **~24 h** (`docs/auftrag/auftrag-flyby2-kette.md`).
- **Blockade:** keine — wartet auf das Perigäum (der Trigger).
- **Braucht:** fill-run ≤24 h nach der ersten Perigäum-Zelle (RTSW/ACE Minuten,
  Kp ≤3 h, Swarm ≤1 d; je Messwert `source`+`active`).

### Fremd gemessen (Route an die Eigentümer-Linie)

- **CI-Red `ci-check 36152720260` @`4f90217ed`** — beide Reds gehören **Mountain**
  (gemessen via `git blame`/`git log -S`): `clippy` `src/archivar/port.rs:1731`
  (identische if-Blöcke `eccentricity`/`rho_cos_phi` → `("gravity","1",604800.0)`)
  aus **Mountain folge159** (`c4592e347a`); `test`
  `archivar::tests::test_diagnose_no_samples` (`src/archivar/tests.rs:6839`,
  „got: data-present (non-JSON body: HTML/XML/text)") aus **Mountain folge161**
  (`0881a2751`) — das Fixture `"\u{1f}\u{8b}gzip payload"` kodiert `\u{8b}` als
  UTF-8 `C2 8B`, die gzip-Magic `[0x1f,0x8b]` (`fetch.rs:590`) matcht nicht.
  `future folge119` (`d8b2eb4d7`) berührte `tests.rs` nur in Z.404 (`km/s`), nicht
  diese Reds. River hat die Ursache isoliert; Fix gehört Mountain (fremde Dateien,
  nicht angefasst).
- **SuperDARN-Mirror rot** (Operator-Weiterleitung 2026-09-25) → Mirror-Workflow,
  Eigentümer Mycelium.
- **GitHub-Support PII-GC** „Re: Request to purge/GC unreachable" → Auftrag
  `docs/auftrag/auftrag-pii-history-rewrite.md`, Eigentümer Future/Operator.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
