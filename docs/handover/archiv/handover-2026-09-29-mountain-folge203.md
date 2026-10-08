<!--
  title: Handover — Mountain-Folge 203 (Stand 2026-09-29)
  session: Mountain-Folge 203
  class: handover
  date: 2026-09-29
  sha256: bb025a116ec7d2df9f40de1eb8ce4df4303d6cdd71aff3df5c977aee7074f807
  status: live
-->
# Handover — Mountain-Folge 203 (2026-09-29)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert
(`state/zustand/standing-pass.md`); gemessen wird nur, was der eigene Trigger für
fällig erklärt.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„ja aus dem register aufstellen" — die offene Liste streng aus dem Register | 2026-09-27 | Operator (Session, Mountain 187)
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Session, Mountain 187)
„warum fixt du nicht anstatt zu verschleppen?" — arbeitbare Schritte werden im Atom gebaut, nicht getragen | 2026-09-28 | Operator (Session, Mountain 190)
„kannst du das bitte fixen" — der `--verdict`-Werkzeugdefekt wird im Atom geheilt | 2026-09-28 | Operator (Session, Mountain 194)
„bitte fixen statt verschleppen" — no-cadence-Sprachloch und ttl der 81 SPK-Blöcke im Atom gebaut | 2026-09-28 | Operator (Session, Mountain 194)
„was sagt der rat?" — Rat zur Design-Frage | 2026-09-28 | Operator (Session, Mountain 194)
Hier ausführen, keine Rangfolge, flash-first delegieren — session-weiter Consent (Delegation) | 2026-09-28 | Operator (Session, Mountain 195)
„hast du alles bis zur kante gemessen und geplant?" — jede Lage vor Plan neu messen, nicht zitieren | 2026-09-28 | Operator (Session, Mountain 199)
Führe den bestätigten Plan aus — `line`-Agent, flash-first delegieren, eine Session ist ein abgeschlossenes Atom; Commit trägt `/commit` | 2026-09-29 | Operator (Session, Mountain 202)
„warum hast du in der letzten runde nur so wenig geschafft? … verschleppst von Runde zu Runde" — jeder eigen-schließbare Schritt läuft im Atom; ein `Trigger`, der die eigene Messung ist, ist kein Warten | 2026-09-29 | Operator (Session, Mountain 203)
Du kannst. Führe den Plan aus — als `line`-Agent (auto-bestätigt); dispatch flash-first; Commit trägt `/commit` | 2026-09-29 | Operator (Session, Mountain 203)

## Offen (aufgeschlüsselt)

### CI auf HEAD grün
- **Status:** wartend | **Bindung:** eigen + fremde Linie (river/te.rs)
- **Trigger:** `ci-gate`/`register-coverage`/`ci-check` am dann aktuellen HEAD enden.
- **Lage:** (gemessen 2026-09-29 via `ci_manage log`) @`69eef305a`: `paper-check 36503272446`/`tools-build` success; rot `register-coverage 36503272495` (`ORPHAN_COMMITTED phi/blocked_sources.φ:430 [mycelium] … MoRIC` — Träger fehlt), `ci-gate 36503272430` (clippy `hips.rs:299/338/408` + `te.rs:4413`; dropped-gate baseline 1082 | current 1113), `hinet-cdn 36498426236` (Quelle publiziert die `cont`-Anfrage nicht). Die hips-Clipys und die dropped-Baseline sind in diesem Atom geheilt; `te.rs:4413` liegt als fremder uncommitteter Hunk in Rivers Arbeitsbaum.
- **Blockade:** `blocked_sources.φ:430`-Träger (mycelium); hinet-Quelle extern.
- **Braucht:** grüne Läufe am neuen HEAD; `## An mycelium` (blocked_sources-Träger + hinet).

### Fixe-Tabellen-Zulassung (Phobos/Vega/Hayabusa)
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Registrierung der Körper `halley`/`itokawa` + `url`/`origin`/`compiler` (mycelium).
- **Lage:** (gemessen 2026-09-29) Assets 200 auf der CDN: pds3 `pds-smallbodies.astro.umd.edu` (386 Assets, `pds3_fixed_width_krfm.bin` verdict 206), pds4 `sbnarchive.psi.edu` (9 Assets, 206); `sources.φ` trägt **0** `pds3_fixed_width`/`pds4_fixed_width`-Zeilen. `at halley`/`itokawa` inert: `body_ephemerides` füllt sich nur aus einer `ephemeris_binary`-Quelle (`extract.rs:2932`, `main_flow.rs:1308`); `halley` 0 Treffer in `phi/`, `itokawa` 4 SPK-Kernels in `sources_index.φ`, beide ohne Zeile in `kernels/naif_body_ids.tsv`.
- **Blockade:** Zwei-Feder; `phi/sources.φ` trägt einen fremden uncommitteten Hunk (river) — diese Session schreibt nicht hinein.
- **Braucht:** (eigen) `naif_body_ids.tsv`-Zeile je Körper + `ephemeris_halley.bin`/`ephemeris_itokawa.bin` via `horizons_compiler.rs:634` (Halley) bzw. Hayabusa-SPK (Itokawa); dann `sources.φ`-Block `url …/ssd.jpl.nasa.gov-ephemeris/ephemeris_<name>.bin` / `format ephemeris_binary` / `compiler ephemeris_compiler.rs` / `at <name>` / `no-cadence`; `url`/`origin`/`compiler` = mycelium.

### `format vlde` — Admission + force-Slot
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Rat-Wort zum force-Slot; Mycelium setzt `url`/`origin`.
- **Lage:** (gemessen 2026-09-29) Reader `src/archivar/vlies.rs`, extract-Branch `extract.rs:3254`, Compiler `tools/harvest/src/bin/vlies_density_compiler.rs`, Asset 206 unter Tag `ssd.jpl.nasa.gov`. Tag-Riss **geheilt** (Compiler-Upload `ssd.jpl.nasa.gov-vlies` → `ssd.jpl.nasa.gov`, Compiler `:348`); `sources.φ`-Zeile 0. Offene Weberin-Lücke (`membran-ladearchitektur.md:234-235`).
- **Blockade:** force-Slot-Verdikt (Vlies-Dichte ist keine Kraft — Witness oder Medium?).
- **Braucht:** Rat-Wort zum force-Slot; `## An mycelium` (`url`/`origin`).

### `format ndk` — Rat-Wort zum `mw`-Zweitfeld
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-Wort zu Name/Kernel/Force des `mw`-Felds.
- **Lage:** (gemessen 2026-09-29) Reader-Arm `extract.rs:3464-3528` über `ndk::parse_ndk`; `main_flow`-Skip in diesem Atom entfernt und ein `ndk`-Fetch-Arm gebaut (`main_flow.rs`, nach dem `vlde`-Arm); `tests.rs:7778`-Fixture auf `grib2` umgestellt; `cargo check` 0/0. `mw` fließt mit der abgeleiteten Konfig (`kernel 3`, `force 4`, Name `gcmt_mw`) — noch ohne Rat-Bestätigung.
- **Blockade:** `mw`-Feldkonfig ist Ableitung, kein Rat-Wort.
- **Braucht:** Rat-Wort zur `mw`-Feldkonfig (Name/Kernel/Force).

### Rosetta-Granule — Typ-18-Arm oder Horizons-Pfad
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-Wort zum Typ-18-Arm vs. Horizons-Route (`ephemeris_rosetta.bin`).
- **Lage:** (gemessen 2026-09-29) `ORER_00031.BSP` trägt 16 Segmente `target -226`, alle Typ 18 (`center 399`, Earth-1, et 163288864–163465233); der BSP-Reader (`ephemeris.rs:346`, `spk.rs:54`) führt nur 1/2/3/9/13/20 → Granule leer. Kein reader-gestütztes Rosetta-S/C-SPK in `phi/`; die Horizons-Route trägt bereits `ssd.jpl.nasa.gov-horizons/ephemeris_rosetta.bin` (verdict 206, `horizons_compiler.rs:18` `("-226","rosetta",…)`); der Workflow zeigt auf `…/ssd.jpl.nasa.gov-ephemeris/ephemeris_rosetta.bin` (404).
- **Blockade:** keine reader-gestützte Rosetta-SPK; Typ-18 (ESA-Hermite) ist ein neuer Arm.
- **Braucht:** Rat-Wort/Entscheidung: neuen Typ-18/19-Arm bauen (hartes Atom) oder `rosetta-ephemeris-cdn.yml` auf das vorhandene Horizons-Asset richten.

### Sonden-Flotte Asien/Russland — Compiler-Bin/Sample
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** je fehlendem Compiler-Bin/Sample ein Port-Schritt nach `docs/SOURCE_PORT.md`.
- **Lage:** (gemessen 2026-09-29) Arme `pds3_binary`/`pds3_img`/`pds4_binary`/`pds4_fits`/`gras_2c` gebaut; `blocked_sources.φ` pds4-fits/gras-2c auf `pending`; Compiler-Bin + Live-Sample fehlen.
- **Blockade:** kein Live-Sample; Compiler-Bin fehlt.
- **Braucht:** CDN-Trigger (mycelium); Sample/Verifikation, sobald eine Route eine Einzeldatei liefert.

### HiPS-PNG (MoRIC) — Arm steht; Ernte/CDN/Sample
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Mycelium-Ernte/CDN (`hips_png_compiler --ci-mode`); Sample-Fixture.
- **Lage:** (gemessen 2026-09-29) `src/archivar/hips.rs` + `hips_png_compiler.rs` gebaut; `blocked_sources.φ:430` auf `pending`; Norder7/Dir0/Npix0.png 200 sha256 `aa6318fe…`. Der Eintrag ist zugleich der `register-coverage`-Orphan (`ORPHAN_COMMITTED`, Träger fehlt).
- **Blockade:** Tree 12·4⁷ Kacheln, kein Einzel-Asset; kein CDN-Eintrag.
- **Braucht:** `## An mycelium` (Ernte-/CDN-Direktive + Träger); Sample-Fixture.

### ENSO-SST — Manifestation ausstehend
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Mycelium manifestiert `ersstv5_nino34.bin` + Workflow.
- **Lage:** (gemessen 2026-09-29) ERDDAP griddap `nceiErsstv5` CSV/monatlich/°C, `--verdict` 206; Verdikt-Zeile + `ersstv5_compiler.rs` + `MAGIC_/COMP_ERSSTV5` (`geo.rs`) gebaut; Asset nicht auf CDN.
- **Blockade:** Manifestation (mycelium).
- **Braucht:** `## An mycelium` (`ersstv5-cdn.yml`, `COMP_/MAGIC_`-Feder).

### Legacy-CDN-Assets `ssd.jpl.nasa.gov` — Disposition
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Mycelium re-manifestiert `spectra.bin`/`nvss.json`/`first14.json`/`curated48_spectra.bin` unter den Family-Tag.
- **Lage:** (gemessen 2026-09-28) die vier Register-`url`-Zeilen 404; Assets 200 unter Legacy-Tag.
- **Blockade:** Re-Manifest (mycelium).
- **Braucht:** danach Verdikt (löschen/halten in `dead_sources.φ`).

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte `state/zustand/wartend.φ:3`)
- **Trigger:** `NED_BYPARAMS_TIMEOUT_TOKEN` per Mail.
- **Lage:** (gemessen 2026-09-28) zwei NED-Einträge im Ledger, kein Token.
- **Blockade:** Token fehlt.
- **Braucht:** mit Token den ByParams-Job fahren.

### auftrag-flyby2-kette — σ-Metrik
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** JUICE in-situ + Δ publiziert (`docs/paper/flyby-path-2-addendum-2026-09-29.md`).
- **Lage:** (gemessen 2026-09-29) Addendum trägt die 26-Zellen-Tubus-Registrierung; σ-Metrik superseded (Δ ≤ δ + 3·σ_recon, δ = 0.168 km) → `pending` mit Trigger.
- **Blockade:** JUICE in-situ + Δ nicht publiziert.
- **Braucht:** bei Publikation `flyby_ephemeris_gate` (CI) gegen das Addendum.

### Rätsel Ⅰ — `dr3_stars`-Record ohne σ-Spalten (river-folge63)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-/Mess-Wort zur Record-Erweiterung.
- **Lage:** (gemessen 2026-09-29 river-folge63) `dr3_stars`-Record (44 B) trägt keine σ_ϖ/σ_pm-Spalten; ohne sie ist jede σ_z-Schätzung eine Rauschmessung (Median-Parallaxe 0,529 mas).
- **Blockade:** Record-Struktur + neue Asset-Version (CDN).
- **Braucht:** erweiterten Record (σ_ϖ/σ_pm) im Compiler; neue Asset-Version registrieren.

### ODF-Flyby-Verdikt — Fenster + Shard-Riss
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Operator-Wort zur DSN/JPL-Rohdatenanfrage (Future-Queue).
- **Lage:** (gemessen 2026-09-29) Keine der vier ODF-`url`-Zeilen (`sources.φ:9764/8977/9810/8947`) trägt ein Erd-Encounter-Fenster; `window` fehlt, `encounter` nur `:10265` (Voyager). Galileo PPI-Annex-TDF deckt Earth-1; MESSENGER/Cassini/Rosetta nicht gefunden. Shard-Riss: `external-state.md:34` vs `sources.φ:8947-8975`/`harvest.φ:251`/`frame_registry.φ:71-76`.
- **Blockade:** ODFs fehlen serverseitig; einziger Weg DSN/JPL-Anfrage.
- **Braucht:** `## An future` (ODF-Anfrage); Shard-Riss-Trägerschaft.

### pds3/pds4-CDN-Verdikt
- **Status:** wartend | **Bindung:** eigen (über die Fixe-Tabellen oben)
- **Trigger:** `sources.φ`-Zeilen nach Körper-Registrierung.
- **Lage:** (gemessen 2026-09-29) `spectral` ist bereits registriert (`sources.φ:2416-2422`, verdict 206) — der folge202-Punkt war stale. pds3/pds4-Assets liegen auf der CDN, `sources.φ`-Zeilen fehlen (siehe Fixe-Tabellen).
- **Blockade:** `phi/sources.φ` trägt einen fremden uncommitteten Hunk.
- **Braucht:** Fixe-Tabellen-Zulassung.

### `commit_check`-Riegel Ereignis→Folge — Alters-Riss
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-Wort zur Alters-/Session-Begrenzung.
- **Lage:** (gemessen 2026-09-28) Fixture gebaut (`commit_gate.rs` `ereignis_folge_violations`, `commit_check.rs`); Riss: Register ungebunden gelesen.
- **Blockade:** Riss.
- **Braucht:** Rat-Wort (Council) zur Begrenzung oder bewusste Ungebundenheit.

### LAB_A/MLZ NSE I(q,t) — privates Holding
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-/Operator-Wort zur Feld-/Wire-Karte der NSE-Serie (Future-Queue).
- **Lage:** (gemessen 2026-09-28) Reader + Compiler gebaut (`lab_reader.rs`, `lab_reader_compiler.rs`); 13 Läufe [RETRACTED-SAMPLE] privat gesichert (`data/lab_a.data/…`, gitignored).
- **Blockade:** Wire-Lücke — NSE-Polarisation trägt keine ICRS-Position/keinen Kraftkanal.
- **Braucht:** Feld-/Wire-Entscheidung (Rat/Future); kein CDN ohne Operator-Einverständnis.

### `decline spectral-series` — Reklassifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-/Rat-Wort (Future-Queue).
- **Lage:** (gemessen 2026-09-28) `declined_sources.φ:1409` ONC-Hydrophon, `:4009` NOAA-NODD NRS; beide declined den Feld-Anspruch, CDN-Record bleibt.
- **Blockade:** keine.
- **Braucht:** Wort, ob als Zeugen reklassifiziert.

## Prosa-Träger (eigene)

- `docs/specs/livefeed-gate.md` | offene Marker = die `pending`-Felder der Ereignis-Tabelle; Träger Mountain.
- `docs/surveys/survey-raetsel-bestand.md` | Verdikt-Träger (Rats-Konsens 2026-09-29); Träger Mountain.
- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | Träger Mountain (echter Marker `:87` astroquery-Modul-Gegenprobe).

## An mycelium (fremde Feder — Aufenthalt beim Eigentümer)
Origin: mountain folge203.

- **`blocked_sources.φ:430`-Träger** (gemessen 2026-09-29, `register-coverage 36503272495`): der MoRIC-Eintrag ist `ORPHAN_COMMITTED` ([mycelium]) — kein Live-Handover trägt ihn; darum rot. Bitte als Punkt in dein Register aufnehmen.
- **hinet-cdn** (gemessen 2026-09-29, `36498426236`): `cont`-Status wurde nie „Available" — die Quelle publiziert die Anfrage nicht; externer Wait, die Quelle publiziert die Anfrage nicht. Aufnehmer: mycelium.
- **`format vlde`**: `url https://github.com/omegaflow/sources/releases/download/ssd.jpl.nasa.gov/vlies_density.vlde`, `origin procedure:`, `compiler vlies_density_compiler.rs`. Der Tag-Riss ist geheilt (Compiler lädt jetzt `ssd.jpl.nasa.gov`).
- **ENSO-SST**: `ersstv5-cdn.yml` (`--ci-mode`); `MAGIC_/COMP_ERSSTV5` steht in `src/archivar/geo.rs`.
- **HiPS-PNG MoRIC**: Ernte-/CDN-Direktive + Sample-Fixture (Tree 12·4⁷ Kacheln, kein Einzel-Asset).
- **Fixe-Tabellen**: pds3/pds4-Assets auf der CDN; Mycelium setzt `url`/`origin`/`compiler`, sobald Mountain die Körper `halley`/`itokawa` registriert hat.
- **Legacy-CDN-Assets**: `spectra.bin`/`nvss.json`/`first14.json`/`curated48_spectra.bin` unter dem Family-Tag re-manifestieren.
- **Rosetta**: den CDN-Lauf zurückhalten, bis der Typ-18/Horizons-Entscheid steht.

## An future (Operator-Queue, private)
Origin: mountain folge203.

- **ODF-Flyby-Fenster fehlt** (gemessen 2026-09-29): keines der vier ODF-Assets trägt das Erd-Encounter-Fenster; die ODFs fehlen serverseitig. Einziger Weg: DSN/JPL-Rohdaten-Anfrage. *Frage:* Anfrage stellen? (Operator-Hand)
- **`format ndk` — `mw`-Zweitfeld** (gemessen 2026-09-29): M0-Arm gebaut und verdrahtet; offen ist das Rat-Wort zu Name/Kernel/Force des `mw`-Felds.
- **Sonden-Flotte CSF/Konto-gated** (CNSA, ISRO PRADAN, MBRSC EMM).
- **CSES-/Swarm-Zugang** (SSDC, Trigger 2026-10-02).
- **GIC-Einreichung** (Operator-Hand).
- **KARI/ISRO-Konten** (Danuri/KASI, Chandrayaan-2/3, Aditya-L1).
- **NSE/SAMPLE_AUTHOR-Datenrechte** — CDN-manifestiert oder privates Holding? Zudem Dank an SAMPLE_CONTACT (Operator-Hand).

## An river (fremde Feder — Aufenthalt beim Eigentümer)
Origin: mountain folge203.

- **Trägerlose Docs** (gemessen 2026-09-29, `register_lookup --orphan-docs` = 3, alle River-Natur): `docs/concepts/kybernetische-astrophysik.md` (realer Punkt: per-Voxel Jeans-Engine `:51-55/:355`, in `river-folge63:81` gebaut, aber der Doc-Pfad ist nicht als Träger genannt), `docs/surveys/survey-2026-09-17-omegaflow-legacy-konzepte.md` (2 Marker = Substring „wartet" in „erwartete", kein offener Punkt), `docs/surveys/survey-fortschritt.md` (1 Marker = nur die Sektions-Kopfzeile). Bitte je eine `descoped`/Träger-Zeile aufnehmen, damit sie nicht erneut orphanen.
- **`te.rs:4413` clippy** (`manual_memcpy`, blockiert `ci-gate`): liegt bereits als dein uncommitteter Hunk im Arbeitsbaum — nur zur Kenntnis.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
