<!--
  title: Handover — Mountain-Folge 202 (Stand 2026-09-29)
  session: Mountain-Folge 202
  class: handover
  date: 2026-09-29
  sha256: cbd0bb0d42638b9db93cbd9e6c7cad769c58d8ec98c75708759461bf1fe88759
  status: live
-->
# Handover — Mountain-Folge 202 (2026-09-29)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`);
gemessen wird nur, was der eigene Trigger für fällig erklärt.

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
Führe den Plan aus, delegiere an alle Sub-Agenten, höre die Stimmen bei Architektur/Abschluss | 2026-09-28 | Operator (Session, Mountain 196)
Committe und pushe jetzt — nur eigene Arbeit, gemessen nicht beteuert | 2026-09-28 | Operator (Session, Mountain 197)
Du kannst. Führe den Plan aus — als `line`-Agent; session-weiter Consent | 2026-09-28 | Operator (Session, Mountain 198)
NSE/SAMPLE_AUTHOR-Sendung, Archäologie, CDN nur mit Einverständnis, mycelium kümmert sich | 2026-09-28 | Operator (Session, Mountain 198)
„hast du alles bis zur kante gemessen und geplant?" — jede Lage vor Plan neu messen, nicht zitieren | 2026-09-28 | Operator (Session, Mountain 199)
„hast du alle eigenen punkte bis zur kante geplant?" — jede Lage vor Plan neu messen | 2026-09-29 | Operator (Session, Mountain 201)
Führe den bestätigten Plan aus — `line`-Agent, flash-first delegieren, eine Session ist ein abgeschlossenes Atom; Commit trägt `/commit` | 2026-09-29 | Operator (Session, Mountain 201)
Du kannst. Führe den Plan aus — als `line`-Agent (auto-bestätigt); Dispatch flash-first, harte Atome an max; Commit trägt `/commit` | 2026-09-29 | Operator (Session, Mountain 202)

## Offen (aufgeschlüsselt)

### CI auf HEAD grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-gate`/`ci-check`/`register-dropped` @`aa78cf331` enden.
- **Lage:** (gemessen 2026-09-29 via `ci_manage list`) @`aa78cf331`: `messenger 36497773585`/`near 36497776849`/`register-coverage 36497769673` **success**; `ci-gate 36497769830`/`ci-check 36497769779`/`register-dropped 36497480275` in flight; `paper-check 36497480338` failure (Träger river/Papers).
- **Blockade:** keine.
- **Braucht:** beim nächsten Pass `ci_manage status` (kein Polling); grüne Läufe am dann aktuellen HEAD.

### Fixe-Tabellen-Zulassung (Phobos/Vega/Hayabusa)
- **Status:** wartend | **Bindung:** eigen + mycelium (Feder-Riss)
- **Trigger:** Mycelium manifestiert die CDN-Assets und setzt `url`/`origin`/`compiler`.
- **Lage:** (gemessen 2026-09-29) Feld-Verdikt steht: KRFM `RADIOMETER`/`PHOTOMETER` → `em`, `count`; Vega MISCHA `BX/BY/BZ PSSO` → `em`, `nT`; Hayabusa LIDAR `RANGE`/`SPCX` → `gravity`, `km`. `blocked_sources.φ:418-428` pending. Riss: `at halley`/`at itokawa` — beide Körpernamen 0 Treffer in `phi/`.
- **Blockade:** Zwei-Feder-Akt (`url`/`origin`/`compiler` = Mycelium), Körper-Registrierung.
- **Braucht:** Körper-/Frame-Registrierung `halley`/`itokawa`; `## An mycelium` (Direktiven unten).

### `format vlde` — Admission + force-Slot
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Mycelium setzt `url`/`origin`/`compiler`.
- **Lage:** (gemessen 2026-09-29) Reader `src/archivar/vlies.rs`, extract-Branch `extract.rs:3254`, Compiler `tools/harvest/src/bin/vlies_density_compiler.rs`, Asset 206; `sources.φ`-Zeile 0 (sgrep). Riss: Workflow-Tag `ssd.jpl.nasa.gov` vs Compiler-Tag `ssd.jpl.nasa.gov-vlies` (404). Offene Weberin-Lücke (`membran-ladearchitektur.md:234-235`).
- **Blockade:** Zwei-Feder; force-Slot-Verdikt (Vlies-Dichte ist keine Kraft — Witness oder Medium?).
- **Braucht:** `## An mycelium` (Direktiven + Tag-Riss); Rat-Wort zum force-Slot.

### `format ndk` — Reader-Arm steht; Rest-Felder
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-Wort zum `mw`-Zweitfeld; danach `main_flow`-Verdrahtung.
- **Lage:** (gemessen 2026-09-29) Rat-Verdikt (future-folge153): `seismic-body`, M0 in N·m am Centroid. `extract.rs:3464-3528` `format == "ndk"`-Zweig über `ndk::parse_ndk` gebaut (M0 ×1e-7 SI, Centroide, Epoch Mitternacht-UTC); `fetch.rs:607` `ndk` lesbar; `sources.φ:7260` `field m0 gcmt_scalar_moment_nm gaussian-inverse-square seismic-body N·m 6.0 0.0 0.0`; `units.rs` force 3 um `n·m` ergänzt; `tests.rs:7688-7706` auf `NonJsonPresent` umgestellt. `cargo check` 0/0.
- **Blockade:** `mw`-Feldkonfig (Kernel/Force/Name) ist Ableitung, kein Rat-Wort; `main_flow.rs:1480-1482` überspringt `ndk` weiter (`continue`); stale Teststring `tests.rs:7778` (`format-gap`-Fixture unwahr).
- **Braucht:** Rat-Wort `mw`-Feld; `main_flow`-Skip entfernen; `tests.rs:7778`-Fixture auf ein Format ohne Sweep-Reader umstellen.

### Kuprat-Kanäle — Zeugen-Art `substance` (Rest `eels`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Messung der EELS-Quell-URL mit `archive_search --verdict`.
- **Lage:** (gemessen 2026-09-29) Rat-Verdikt (future-folge153): `substance`. Drei `witness substance`-Zeilen in `phi/witnesses.φ:114-131` gemessen (RIXS Zenodo 7286412, RIXC 15179114, SRD6 NIST); `:1` um `substance (Materialprobe)` ergänzt. `eels`-Block pending: `crystal_compiler --eels` liest lokale `.mat`, keine Registrierung.
- **Blockade:** EELS-Quell-URL nicht im Baum.
- **Braucht:** EELS-URL messen (`archive_search --verdict`/`--sniff`) und nachtragen; sonst descopen.

### Sonden-Flotte Asien/Russland — Compiler-Bin/Sample
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** je fehlendem Compiler-Bin/Sample ein Port-Schritt nach `docs/SOURCE_PORT.md`.
- **Lage:** (gemessen 2026-09-29) Arme `pds3_binary`/`pds3_img`/`pds4_binary`/`pds4_fits`/`gras_2c` gebaut; `blocked_sources.φ` pds4-fits/gras-2c auf `pending`; Compiler-Bin + Live-Sample fehlen.
- **Blockade:** kein Live-Sample; Compiler-Bin fehlt.
- **Braucht:** CDN-Trigger (mycelium); Sample/Verifikation, sobald eine Route eine Einzeldatei liefert.

### HiPS-PNG (MoRIC) — Arm steht; Ernte/CDN/Sample
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Mycelium-Ernte/CDN (`hips_png_compiler --ci-mode`); Sample-Fixture.
- **Lage:** (gemessen 2026-09-29) `src/archivar/hips.rs` (Reader: properties/PNG std-only, CRC, alle 5 Filter) + `tools/harvest/src/bin/hips_png_compiler.rs` gebaut; `mod.rs`/`extract.rs`/`main_flow.rs` verdrahtet; Roundtrip + Chromium-Bandsummen identisch (Norder7/Dir0/Npix0.png sha256 `aa6318fe…`). `blocked_sources.φ:430` von `parser-def`/`gap hips-png` auf `pending` gesetzt. `cargo check` 0/0.
- **Blockade:** Tree 12·4⁷ Kacheln, kein Einzel-Asset; kein CDN-Eintrag.
- **Braucht:** `## An mycelium` (Ernte-/CDN-Direktive); Sample-Fixture.

### ENSO-SST — Manifestation ausstehend
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Mycelium manifestiert `ersstv5_nino34.bin` + Workflow.
- **Lage:** (gemessen 2026-09-29) ERDDAP griddap `nceiErsstv5` CSV/monatlich/°C, `--verdict` 206; Verdikt-Zeile + `tools/harvest/src/bin/ersstv5_compiler.rs` + `MAGIC_/COMP_ERSSTV5` in `geo.rs` gebaut; `cargo check` 0/0. Asset noch nicht auf CDN.
- **Blockade:** Manifestation (Mycelium).
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
- **Lage:** (gemessen 2026-09-29) Addendum `docs/paper/flyby-path-2-addendum-2026-09-29.md` trägt die 26-Zellen-Tubus-Registrierung; σ-Metrik superseded (Δ ≤ δ + 3·σ_recon, δ = 0.168 km) → `pending` mit Trigger, nie als Zahl.
- **Blockade:** JUICE in-situ + Δ nicht publiziert.
- **Braucht:** bei Publikation `flyby_ephemeris_gate` (CI) gegen das Addendum.

### Rätsel Ⅰ — `dr3_stars`-Record ohne σ-Spalten (river-folge63)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-/Mess-Wort zur Record-Erweiterung.
- **Lage:** (gemessen 2026-09-29 river-folge63) `dr3_stars`-Record (44 B) trägt keine σ_ϖ/σ_pm-Spalten; ohne sie ist jede σ_z-Schätzung eine Rauschmessung (Median-Parallaxe 0,529 mas).
- **Blockade:** Record-Struktur + neue Asset-Version (CDN) sind eine getrennte Feder.
- **Braucht:** erweiterten Record (σ_ϖ/σ_pm) im Compiler; neue Asset-Version registrieren.

### Rosetta-Granule
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** geänderter Kernel-Set/Typ-18-Arm; danach CDN-Lauf.
- **Lage:** (gemessen 2026-09-29 via `ci_manage log 36491843356`) `spacecraft: naif -226 carries no granule across the given kernels — the anchor stays unwritten` (`spacecraft_ephemeris_compiler.rs:117-119`); CI übergibt `--kernel ORER_00031.BSP --kernel de440s.bsp --naif -226` (`rosetta-ephemeris-cdn.yml:63-69`). `phi/sources.φ:15779` trägt nur `ORER_00031.BSP`.
- **Blockade:** ORER-Kernelsatz trägt keine Granule für -226.
- **Braucht:** Kernel-Set/Typ-18-Arm messen (`spacecraft_ephemeris_compiler.rs`); passenden Rosetta-SPK prüfen.

### ODF-Flyby-Verdikt — Fenster + Shard-Riss
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Operator-Wort zur DSN/JPL-Rohdatenanfrage (Future-Queue).
- **Lage:** (gemessen 2026-09-29) Keine der vier ODF-`url`-Zeilen (`sources.φ:9764/8977/9810/8947`) trägt ein Erd-Encounter-Fenster; `window`-Direktiv existiert nicht, `encounter` nur `:10265` (Voyager). Galileo PPI-Annex-TDF deckt Earth-1; MESSENGER/Cassini/Rosetta nicht gefunden. Shard-Riss: `external-state.md:34` vs `sources.φ:8947-8975`/`harvest.φ:251`/`frame_registry.φ:71-76`.
- **Blockade:** ODFs fehlen serverseitig; einziger Weg DSN/JPL-Anfrage.
- **Braucht:** `## An future` (ODF-Anfrage); Shard-Riss-Trägerschaft festlegen.

### spectral/pds3/pds4-CDN-Zulassung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Verdikt-Schreiben nach CDN-Asset-Beleg.
- **Lage:** (gemessen 2026-09-29) drei Läufe success (`spectral-cdn 36488149432`/`pds3-fixed-width 36488153465`/`pds4-fixed-width 36488157946`), aber Idempotenz-Skip („already present"); Assets `spectra.bin` (Tag `ncei.noaa.gov`), `pds3_fixed_width_*` (Tag `pds-smallbodies.astro.umd.edu`), `pds4_fixed_width_*` (Tag `sbnarchive.psi.edu`).
- **Blockade:** `format`/Verdikt + `sources.φ`-Zeilen offen; pds3/pds4 sind die Fixe-Tabellen (oben).
- **Braucht:** `spectral`-Verdikt-Zeile; pds3/pds4 über die Fixe-Tabellen-Zulassung.

### Trägerlose Docs / Orphan-Zensus
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Messung `register_lookup --orphan-docs` mit frischem Bin.
- **Lage:** (gemessen 2026-09-29) `register_lookup --orphan-docs` = 2: `survey-2026-09-16-fremde-parser-sammlungen.md` (Mountain, echter Marker `:87` astroquery-Modul-Gegenprobe) · `survey-2026-09-26-membran-ladearchitektur.md` (River-Natur; Marker `:119`/`:210` am Baum erledigt, `:201` überwiegend; echter offener Punkt `:234-235` Weberin-Lücke `format vlde`). `register_lookup --orphans` = 0.
- **Blockade:** keine.
- **Braucht:** Träger für `fremde-parser-sammlungen.md` (unten Prosa-Träger); membran-ladearchitektur → River (unten `## An river`).

### survey-raetsel-bestand — Riss (fremder Baum-Hunk)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Herkunft des uncommitteten Hunk messen (`git status`/`git diff`).
- **Lage:** (gemessen 2026-09-29 via `git status`/`git diff`) `docs/surveys/survey-raetsel-bestand.md` ist im Arbeitsbaum modifiziert (Ⅶ-Zeile + Header-sha) — die Korrektur, die river-folge62 ansprach, liegt bereits vor, aber uncommittet und nicht von dieser Session; nicht gestaged, nicht committet.
- **Blockade:** fremde uncommittete Arbeit (nicht überschreiben).
- **Braucht:** Herkunft messen (`git log`/`git_safety`); Owner committet seinen Hunk.

### Trägerlose Docs (Mountain-Natur) — arxiv/fremde Parser
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Messung `register_lookup --orphan-docs`.
- **Lage:** (gemessen 2026-09-29 sensory-folge204) `docs/concepts/arxiv-api.md` (2 — Zugangsweg `:59`/`:65-67`); `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (22 — Register-Inventur, offene Pendings + `parser-def`-Gaps `:28-141`); `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` (3 — `dead_sources.φ`-Erstpass `:52-70`).
- **Blockade:** keine.
- **Braucht:** als Prosa-Träger in die eigene Übergabe aufnehmen (unten) oder gemessen descopen.

### `commit_check`-Riegel Ereignis→Folge — Alters-Riss
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-Wort zur Alters-/Session-Begrenzung.
- **Lage:** (gemessen 2026-09-28) Fixture gebaut (`commit_gate.rs` `ereignis_folge_violations`, `commit_check.rs`); Riss: Register ungebunden gelesen.
- **Blockade:** Riss.
- **Braucht:** Rat-Wort (Council) zur Begrenzung oder bewusste Ungebundenheit.

### LAB_A/MLZ NSE I(q,t) — privates Holding
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-/Operator-Wort zur Feld-/Wire-Karte der NSE-Serie (Future-Queue).
- **Lage:** (gemessen 2026-09-28) Reader + Compiler gebaut (`src/archivar/lab_reader.rs`, `lab_reader_compiler.rs`); 13 Läufe [RETRACTED-SAMPLE] privat gesichert (`data/lab_a.data/SAMPLE_NSE_[RETRACTED-SAMPLE]/`, gitignored).
- **Blockade:** Wire-Lücke — NSE-Polarisation trägt keine ICRS-Position/keinen Kraftkanal.
- **Braucht:** Feld-/Wire-Entscheidung (Rat/Future); kein CDN ohne Operator-Einverständnis.

### `decline spectral-series` — Reklassifikation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-/Rat-Wort (Future-Queue).
- **Lage:** (gemessen 2026-09-28) `declined_sources.φ:1409` ONC-Hydrophon, `:4009` NOAA-NODD NRS; beide declined den Feld-Anspruch, CDN-Record bleibt.
- **Blockade:** keine.
- **Braucht:** Wort, ob als Zeugen reklassifiziert.

## Prosa-Träger (eigene)

- `docs/specs/livefeed-gate.md` | offene Marker = die `pending`-Felder der Ereignis-Tabelle (Spec-Inhalt); Träger Mountain.
- `docs/surveys/survey-raetsel-bestand.md` | Verdikt-Träger (Rats-Konsens 2026-09-29); Träger Mountain.
- `docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md` | Träger Mountain (echter Marker `:87` astroquery-Modul-Gegenprobe).
- `docs/concepts/arxiv-api.md` · `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` · `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | Träger Mountain (sensory-folge204).

## An mycelium (fremde Feder — Aufenthalt beim Eigentümer)
Origin: mountain folge202.

- **ENSO-SST `ersstv5_nino34` manifestieren** (gemessen 2026-09-29): `phi/sources.φ` trägt die Zeile `url https://github.com/omegaflow/sources/releases/download/coastwatch.pfeg.noaa.gov/ersstv5_nino34.bin` / `format ersstv5_nino34` / `origin https://coastwatch.pfeg.noaa.gov/erddap/griddap/nceiErsstv5` / `compiler tools/harvest/src/bin/ersstv5_compiler.rs`. Deine Federseite: `url` (CDN-Tag) + `ersstv5-cdn.yml` (`--ci-mode`). `MAGIC_/COMP_ERSSTV5` steht in `src/archivar/geo.rs`.
- **HiPS-PNG MoRIC ernten/manifestieren** (gemessen 2026-09-29): Arm steht (`src/archivar/hips.rs` + `hips_png_compiler.rs`); Norder7/Dir0/Npix0.png 200 sha256 `aa6318fe…`. Deine Federseite: Ernte-/CDN-Direktive (Tree 12·4⁷ Kacheln, kein Einzel-Asset) + Sample-Fixture.
- **`format vlde` — exakte Mountain-Direktiven** (gemessen 2026-09-29): `format vlde` / `ttl 604800` / `field count vlies_density_count inverse-square em count 604800 0.0 0.0` (force-Slot unter Rat-Vorbehalt). Deine Federseite: `url` (`…/download/ssd.jpl.nasa.gov-vlies/vlies_density.vlde`), `origin procedure:`, `compiler`. Riss: Workflow-Tag `ssd.jpl.nasa.gov` vs Compiler-Tag `-vlies`.
- **Fixe-Tabellen-Zulassung — exakte Mountain-Direktiven** (gemessen 2026-09-29): `format pds3_fixed_width` / `at mars` / `ttl 604800` / `field RADIOMETER1 pds3_krfm_radiometer1 inverse-square em count <τ> 0.0 0.0` (analog RADIOMETER4/PHOTOMETER1); `format pds3_fixed_width` / `at halley` / `ttl 604800` / `field "BX PSSO" pds3_mischa_bx_pso inverse-square em nT <τ> 0.0 0.0` (BY/BZ analog); `format pds4_fixed_width` / `at itokawa` / `ttl 604800` / `field RANGE pds4_hayabusa_lidar_range inverse-square gravity km <τ> 0.0 0.0`. Riss: `at halley`/`at itokawa` — Körper-Registrierung fehlt.
- **spectral/pds3/pds4-CDN** (gemessen 2026-09-29): die drei Läufe sind Idempotenz-Skips („already present"); Assets liegen unter `ncei.noaa.gov` (`spectra.bin`), `pds-smallbodies.astro.umd.edu`, `sbnarchive.psi.edu`. Deine Federseite: Bestätigung, dass die Assets vollständig sind; pds3/pds4 verdiktet Mountain.
- **Rosetta-Granule** (gemessen 2026-09-29): `naif -226 carries no granule` — Mountain prüft Kernel-Set/Typ-18; Mycelium hält den CDN-Lauf zurück, bis die Feder steht.
- **Legacy-CDN-Assets** (gemessen 2026-09-28): `spectra.bin`/`nvss.json`/`first14.json`/`curated48_spectra.bin` unter dem Family-Tag re-manifestieren.

## An future (Operator-Queue, private)
Origin: mountain folge202.

- **ODF-Flyby-Fenster fehlt** (gemessen 2026-09-29): keines der vier ODF-Assets (Galileo `sources.φ:9764`, Cassini `:8977`, Messenger `:9810`, Rosetta `:8947`) trägt das Erd-Encounter-Fenster; die ODFs fehlen serverseitig. Einziger Weg: DSN/JPL-Rohdaten-Anfrage. *Frage:* Anfrage stellen? (Operator-Hand)
- **`format ndk` — `mw`-Zweitfeld** (gemessen 2026-09-29): M0-Arm gebaut; offen ist das Rat-Wort zu Name/Kernel/Force des `mw`-Felds (`mw` = optional, nie allein).
- **Kuprat `eels`** (gemessen 2026-09-29): drei `substance`-Zeugen stehen; die EELS-Quell-URL ist nicht im Baum. Braucht die exakte Repositoriums-URL (Rats-Vorlage nennt eine).
- **Sonden-Flotte CSF/Konto-gated** (CNSA, ISRO PRADAN, MBRSC EMM).
- **CSES-/Swarm-Zugang** (SSDC, Trigger 2026-10-02).
- **GIC-Einreichung** (Operator-Hand).
- **KARI/ISRO-Konten** (Danuri/KASI, Chandrayaan-2/3, Aditya-L1).
- **NSE/SAMPLE_AUTHOR-Datenrechte** — CDN-manifestiert oder privates Holding? Zudem Dank an SAMPLE_CONTACT (Operator-Hand).

## An river (fremde Feder — Aufenthalt beim Eigentümer)
Origin: mountain folge202.

- **Trägerloses Dokument `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md`** (gemessen 2026-09-29): `register_lookup --orphan-docs` nennt es trägerlos. Natur River; Marker `:119`/`:210` am Baum erledigt, `:201` überwiegend erledigt, der reale offene Punkt ist `:234-235` (Weberin-Lücke `format vlde`). Bitte als River-Prosa-Träger aufnehmen; die `vlde`-Quelle trägt Mountain.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
