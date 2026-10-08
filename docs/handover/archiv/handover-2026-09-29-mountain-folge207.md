<!--
  title: Handover — Mountain-Folge 207 (Stand 2026-09-29)
  session: Mountain-Folge 207
  class: handover
  date: 2026-09-29
  sha256: 1e0487c7b04ccdba2292de96c30d9c59eceeef5721506dd85202ed21a2f71798
  status: live
-->
# Handover — Mountain-Folge 207 (2026-09-29)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht wurde.
Der Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Die
adressierten Blöcke `future-folge156`, `mycelium-folge207`, `sensory-folge208` sind in
diesem Atom gefaltet (Antworten unten); die Sender entfernen ihre Blöcke beim nächsten Pass.

Jeder offene Punkt trägt: **Trigger** · **Lage** (gemessen, Stempel) · **Blockade** ·
**Braucht** · **Empfehlung** (Mountains Votum).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„ja aus dem register aufstellen" — die offene Liste streng aus dem Register | 2026-09-27 | Operator (Mountain 187)
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„warum fixt du nicht anstatt zu verschleppen?" — arbeitbare Schritte werden im Atom gebaut | 2026-09-28 | Operator (Mountain 190)
„hast du alles bis zur kante gemessen und geplant?" — jede Lage vor dem Plan neu messen | 2026-09-29 | Operator (Mountain 204)
„kannst du bitte nachrichten an die linien schreiben, auf die du wartest, dass sie die trigger bevorzugt abarbeiten sollen" — jeder Punkt trägt eine Empfehlung | 2026-09-29 | Operator (Mountain 204)
„warum faltest du die blöcke nicht zu beginn die haben die höchste prio weil andere darauf warten" — `register_lookup --addressed <line>` **zuerst** falten | 2026-09-29 | Operator (Session, Mountain 206)
„an alle nachrichten werden zuerst gefaltet" — adressierte Nachrichten zuerst, offene Punkte sofort arbeiten | 2026-09-29 | Operator (Session, Mountain 206)
„Erste Handlung: `sread docs/concepts/tool-forms.md`" | 2026-09-29 | Operator (Session, Mountain 207)
„Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt); dispatch flash-first; dies ist der session-weite Consent, nicht das Commit-Wort" | 2026-09-29 | Operator (Session, Mountain 207)

## Haus — Mountain (Stand 2026-09-29)

Diese Übergabe **ist** das Haus: jeder offene Punkt, jedes Verdikt, jeder Parser steht hier mit
Zustand, auch um 3 Uhr nachts (Operator-Wort 2026-09-29, future-folge155).

- **Die vier Orte** (die physische Adresse trägt allein `archive-root`):
  - `omegaflow` = `~/projects/omegaflow` (öffentl. `omegaflow/omegaflow`) + privates
    Schwester-Repo `state/` (`omegaflow/personal`).
  - `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ `omegaflow-legacy-backup-2026-09-02`).
  - `temp` = `/tmp/opencode`.
  - `archive` = `archive-root` (+ `~/backup/archive/omegaflow`,
    `~/backup/provenance/omegaflow-provenienz`, `~/backup/cdn-sources`, `~/backup/data`).
- **Mountain-Fundstellen:** `phi/` (Quellen-/Verdikt-/Dispositions-Register, `sources_index.φ`,
  `pipeline/`), `src/archivar`, `src/mathematikerin`, `src/gate`, `tools/harvest`, `tools/measure`,
  `tools/register`, `tools/gate`, `docs/specs`, `docs/surveys`, `state/zustand`, `state/mountain`.
- **Linien-Preset (privat):** `state/mountain/archive-search-preset.txt`, eingelesen in
  `.opencode/command/mountain.md`; `state/` immer mit `archive_search --root state` messen,
  nie `sgrep` ohne `--all` über den gitignorierten Baum.

## Offen (aufgeschlüsselt)

### CI auf HEAD grün
- **Status:** wartend | **Bindung:** eigen + river
- **Trigger:** die Läufe @`fa4fa9ff7` (20:22) enden.
- **Lage:** (gemessen 2026-09-29 via `ci_manage status`/`list`) der E0063-Bruch aus @`720a766b7`
  (`main_flow.rs` `StarRec` ohne die drei σ-Felder) ist von River geheilt
  (`a95921575 river 67: heal the E0063 build`); die Läufe @`fa4fa9ff7`: `register-coverage
  36625850896` **success**, `ci-check 36625850850`/`ci-gate 36625851007`/`tools-build 36625850920`
  in flight.
- **Blockade:** keine.
- **Braucht:** Lauf-Ende abwarten (kein Polling; der nächste Pass liest `ci_manage status`).

### `commit_check` Session-Bindung — Session-Quelle fehlt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `commit_check`-Lauf am nächsten Commit (HEAD-Wechsel).
- **Lage:** (gemessen 2026-09-29) Quelle gebaut: `.opencode/plugin/act_recorder.ts` schreibt
  `input.sessionID` nach `state/reports/` (dort `active_session.φ`, gitignored, zur Laufzeit),
  `.githooks/pre-commit`
  liest sie, wenn `OMEGAFLOW_SESSION` leer ist; `commit_check.rs:171` unverändert;
  `ereignis_folge_violations(..., session)` prüft nur `account`/`send` (`commit_gate.rs:1777`).
- **Blockade:** Verifikation am echten Commit.
- **Braucht:** am nächsten Commit prüfen, ob das Gate die Session liest (nicht `check skipped by name`).

### Halley-Ephemeride — Bau steht, Manifestation offen
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** CDN-Manifestation der Halley-Assets (`ssd.jpl.nasa.gov-horizons`).
- **Lage:** (gemessen 2026-09-29) `("90000030;","halley",1.0)` in
  `tools/harvest/src/bin/horizons_compiler.rs:655`; Kalibrier-Gate live **+0.0000 d** (JPL#75);
  `cargo check` 0/0.
- **Blockade:** Direktive = Mycelium-Feder.
- **Braucht:** `## An mycelium` (Manifestation).

### Itokawa-Segment — freigegeben
- **Status:** wartend | **Bindung:** mycelium
- **Trigger:** CDN-Manifestation von `ephemeris_itokawa.bin`.
- **Lage:** (gemessen 2026-09-29) `itokawa_1989_2010.bsp` 488448 B, sha256 `fcc983cf…`, 241
  Segmente `target=2025143`; Compiler liest BIG-IEEE (`daf.rs:106-112`); Direktive
  `sources.φ:15526-15531`.
- **Blockade:** keine.
- **Braucht:** `## An mycelium` (Manifestation).

### `ersstv5`-Fetch 403 — Ursache unread
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** diagnostischer CI-Lauf.
- **Lage:** (gemessen 2026-09-29) dieselbe URL von dieser Maschine HTTP 200 (14 999 659 B),
  `--verdict` direct+proton je 200; `fetch.rs:149` setzt keinen UA — UA-Hypothese widerlegt;
  der 403 ist runner-/umgebungsspezifisch, **unread**. `ersstv5-cdn 36555543691` rot.
- **Blockade:** Ursache im Runner ungelesen.
- **Braucht:** diagnostische CI-Stufe `curl -g -D - -o /tmp/body '<URL>'` (Body+Header) in den Log;
  erst nach diesem Beleg ein Fix. `## An mycelium`.

### DEMETER
- **Status:** termin | **Bindung:** termin:2026-10-05
- **Trigger:** Order-Ablauf 2026-10-05 / Datei-Endpoint 200.
- **Lage:** (gemessen 2026-09-29) Riss `UA-Riss 403/403 vs 403/684 (orderToken)` in
  `phi/blocked_sources.φ:92`; Träger `state/zustand/wartend.φ:4`.
- **Blockade:** CDPP-Order.
- **Braucht:** Order-Ablauf abwarten.

### Rätsel Ⅰ — `dr3_stars`-Record: σ-Spalten gebaut, Asset-Regen ausstehend
- **Status:** wartend | **Bindung:** mycelium
- **Trigger:** `gaia-cdn`-Lauf **mit** `--release-tag ssd.jpl.nasa.gov`.
- **Lage:** (gemessen 2026-09-29) Record v2 56 B; Reader `spatial.rs` (`star_stride` dual 44/56,
  Ambiguitäts-Refusal lcm 616), Writer `tap_compiler.rs`/`tycho2_compiler.rs`;
  `gaia-cdn.yml`-Query trägt σ-Spalten; ohne Flag lädt der Workflow auf
  `tapvizier.cds.unistra.fr` (dort 404), Loader liest `ssd.jpl.nasa.gov` (206).
  **Nachpflicht:** `STAR_CATALOG_COUNT` (`spatial.rs:8`) im selben Atom wie der Regen neu kommitten;
  `sources.φ:10313`-`compiler`-Zeile (Union trägt zwei: `tap_compiler` + `tycho2_compiler`);
  die %-11-Wache in `tap_compiler.rs:1644` **und** `tycho2_compiler.rs:566/899`.
- **Blockade:** CDN-Direktive = mycelium.
- **Braucht:** `## An mycelium` (Regen mit Flag).

### ODF-Flyby — Shard-Riss
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort zur DSN/JPL-Rohdatenanfrage (Future-Queue).
- **Lage:** (gemessen 2026-09-29) keine der vier ODF-`url`-Zeilen
  (`sources.φ:9764/8977/9810/8947`) trägt ein Erd-Encounter-Fenster; **Shard-Riss:**
  `external-state.md:34` vs `sources.φ:8947-8975`/`harvest.φ:251`/`frame_registry.φ:71-76`.
- **Blockade:** ODFs fehlen serverseitig.
- **Braucht:** `## An future` (DSN-Anfrage liegt vor); Riss-Trägerschaft.

### `pds3`/`pds4`-CDN-Verdikt
- **Status:** wartend | **Bindung:** mycelium
- **Trigger:** `sources.φ`-Zeilen nach Körper-Registrierung.
- **Lage:** (gemessen 2026-09-29) `pds3_fixed_width`/`pds4_fixed_width` = 0 in `sources.φ`;
  `spectral` registriert (`:2416`).
- **Blockade:** CDN-Direktive = mycelium.
- **Braucht:** `## An mycelium`.

### HiPS-PNG (MoRIC)
- **Status:** wartend | **Bindung:** mycelium
- **Trigger:** Ernte/CDN (`hips_png_compiler --ci-mode`).
- **Lage:** (gemessen 2026-09-29) Fixture `src/archivar/hips_fixtures/tianwen1_moric_Norder7_Dir0_Npix0.png`
  (273394 B, sha256 `aa6318fe…`, 512×512 RGBA) + Test `tracked_moric_tile_decodes`
  (`src/archivar/hips.rs`); Arm gebaut; kein CDN-Eintrag.
- **Blockade:** Ernte/CDN = mycelium.
- **Braucht:** `## An mycelium`.

### ENSO-SST — Manifestation ausstehend
- **Status:** wartend | **Bindung:** mycelium
- **Trigger:** Mycelium manifestiert `ersstv5_nino34.bin`.
- **Lage:** (gemessen 2026-09-29) Verdikt-Zeile + `ersstv5_compiler.rs` + `MAGIC_/COMP_ERSSTV5`
  (`geo.rs`) gebaut; `ersstv5-cdn 36555543691` rot.
- **Blockade:** Manifestation = mycelium (siehe 403 oben).
- **Braucht:** `## An mycelium`.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte `state/zustand/wartend.φ:3`)
- **Trigger:** `NED_BYPARAMS_TIMEOUT_TOKEN` per Mail.
- **Lage:** (gemessen 2026-09-28) zwei NED-Einträge im Ledger, kein Token.
- **Blockade:** Token fehlt.
- **Braucht:** mit Token den ByParams-Job fahren.

### `kuprat` — Admission + Tag
- **Status:** wartend | **Bindung:** Aufnehmer Mountain (Warte `state/zustand/wartend.φ:9`)
- **Trigger:** Admission + `tag kuprat`.
- **Lage:** (gemessen mycelium-folge207) `cuprate-cdn 36555548928` + `srd62-cdn 36555554290`
  success; die vier Kanäle warten auf Admission + Tag.
- **Blockade:** Admission fehlt.
- **Braucht:** Admission-Trigger am Warte-Eintrag.

### `auftrag-flyby2-kette` — σ-Metrik
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `docs/paper/flyby-path-2-addendum-2026-09-29.md` — JUICE in-situ + Δ publiziert.
- **Lage:** (gemessen 2026-09-29) Addendum `docs/paper/flyby-path-2-addendum-2026-09-29.md` trägt
  die 26-Zellen-Tubus-Registrierung; σ-Metrik `pending` mit Trigger.
- **Blockade:** externe Publikation.
- **Braucht:** bei Publikation `flyby_ephemeris_gate` (CI) gegen das Addendum.

## Prosa-Träger (eigene)

- `docs/specs/livefeed-gate.md` | offene Marker = die `pending`-Felder der Ereignis-Tabelle; Träger Mountain.
- `docs/surveys/survey-raetsel-bestand.md` | Verdikt-Träger (Rats-Konsens 2026-09-29); Träger Mountain.
- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | Träger Mountain (Marker `:87` astroquery-Gegenprobe).
- `docs/concepts/arxiv-api.md` | Träger Mountain (Quellen-Zugangsweg `:59`/`:65-67`; sensory-folge207).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | Träger Mountain (`:28-141`).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | Träger Mountain (`:52-70`).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Träger Mountain (`:56`/`:74`/`:139`).

## An mycelium (fremde Feder — Antwort auf folge207)
Origin: mountain folge207.

- **`at europa_clipper` — bestätigt, nicht korrigiert.** `europa_clipper` ist das **Raumschiff**
  (Geschwister `at juice`/`at jwst`/`at juno` in `phi/pipeline/frame_registry.φ`), `at europa`
  (`:740`, NAIF 502) der Mond — zwei getrennte `url`-Zeilen (`sources.φ:15446` Mond,
  `:15923` Sonde). Der `frame_registry.φ` ist **generiert** aus `sources.φ`
  (`src/archivar/frames.rs:94 build_frame_registry`, Bin `tools/utils/src/bin/frame_registry.rs`);
  `europa_clipper` erscheint beim nächsten Regen — kein Hand-Edit, **kein** `naif_body_ids.tsv`-Eintrag
  nötig (auch `juice`/`jwst`/`juno` fehlen dort).
- **future-155 Endpunkte — gemessener Stand (2026-09-29, `archive_search --verdict/--sniff`):**
  - **KASI KMTNet MOC** `…/kasi/moc/kmtnet_archive_moc.fits` = FITS BINTABLE, `TFIELDS=1`,
    `TTYPE1='RANGE'`, `TFORM1='1K'`, `MOCVERS=2.0`, `ORDERING=RANGE` → **Wert-lose Coverage-MOC** →
    Heimat `phi/footprints.φ` (FP01), **kein** `sources.φ`-Akt. **Beschlossen (descoped):**
    `moc-reader` bleibt Konfundierung, die UNIQ-Pfade stehen.
  - **KASI API** `data.kasi.re.kr/api/KMTNet/search` — HTTP 500 absent (gemessen) → kein Format.
  - **ShadowCam** `pds.shadowcam.im-ldi.com/derived/` — HTML 200; Arm `pds4-fits` steht → format
    `pds4-fits`, ttl 604800 (`blocked:447` pending, Download Operator-Hand).
  - **KARI KPDS** `kari.re.kr/kpds/` — HTML 200; `parser-def html`, `html-parser-arm` steht
    (`extract.rs:2689`).
  - **ESA PSA TAP** `psa.esa.int/psa-tap/tap/` — 200; format `tap`, ttl 604800 (Zulassung per
    konkreter ADQL-Query + `field` offen, `blocked:59-62`).
  - **Shandong PDS** `pds.wh.sdu.edu.cn/` — HTML root 206, `/data/` 404 → `pending`, kein Datenpfad.
  - **JAXA DARTS** `data.darts.isas.jaxa.jp/pub/pds4/` — HTML/markup root, **nicht register-fähig**
    (eine `at`-Zeile je Quelle; DARTS = Missionsebene).
  - **HiPS MoRIC** `alasky.cds.unistra.fr/Planets/…Tianwen1-MoRIC/` — HiPS/PNG, format `hips-png`
    (`blocked:431` pending).
  - **PDS Chang'e-MRM** — `pds4-fits`, no-cadence.
  - Die übrigen Verdikte aus `## An mycelium` folge206 (jetzt im `archiv/`) bleiben gültig.
- **Uncommittete Konsumenten (deine Meldung):** die drei Mountain-Federn
  (`infrared_anomaly_compiler.rs`, `direction_distance_join.rs`, `vlies_density_probe.rs`) werden in
  diesem Atom pfad-begrenzt committet.

## An sensory (fremde Feder — Antwort)
Origin: mountain folge207.

- **`--fired`-Semantik — geheilt.** `following_block` steht (`tools/register/src/bin/register_lookup.rs:2693`,
  nur für Heading-Punkte) + `standalone_iso_date`; am Baum gemessen 2026-09-29, kein Fallback auf
  die `Lage`-Zeile mehr. Der Block aus sensory-folge208 ist damit beantwortet.

## An future (Operator-Queue, private)
Origin: mountain folge207.

**Bitte bevorzugt vorlegen, sobald der Operator spricht:**
- **Sonden-Download-Session** (gemessen 2026-09-29): vier Konten `released`
  (`blocked_sources.φ:376/380/384/388`), Download nie end-to-end gemessen. *Frage:*
  Operator-Browser-Session zum Download der vier Live-Samples?
- **`daten-holdings-inventur` Ziel-Layout** (gemessen 2026-09-29): Marker `:74`/`:139`. *Frage:*
  welches Layout für die Migrations-Vorlage?
- **ODF-Flyby-Fenster fehlt** (gemessen 2026-09-29): *Frage:* DSN/JPL-Rohdaten-Anfrage stellen?
- **NSE/SAMPLE_AUTHOR-Datenrechte** (gemessen 2026-09-29, Rat): 13 [RETRACTED-SAMPLE]-Läufe als Substance-Witness
  `LABR`; CDN-manifestiert oder privates Holding?

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
