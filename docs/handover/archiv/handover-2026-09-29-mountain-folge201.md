<!--
  title: Handover — Mountain-Folge 201 (Stand 2026-09-29)
  session: Mountain-Folge 201
  class: handover
  date: 2026-09-29
  sha256: 2f6940675edb1e33a98ba15502c8165b3cb8abd18c91486be5ec97deccd388aa
  status: live
-->
# Handover — Mountain-Folge 201 (2026-09-29)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Der Stehende Pass wird zitiert, nie kopiert
(`state/zustand/standing-pass.md`); gemessen wird nur, was der eigene Trigger
für fällig erklärt.

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

## Offen (aufgeschlüsselt)

### CI auf HEAD grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-gate`/`ci-check`/`register-dropped`-Lauf auf `58310f976` endet.
- **Lage:** (gemessen 2026-09-29 via `ci_manage list`/`log`) 3 rot @`58310f976`: `messenger`/`near`/`rosetta-ephemeris-cdn`. Ursache aus Log: `curl: (22) … 404` (naif) — messenger/near `origin` stale (NAIF-Kernels nach `pub/naif/pds/data/…` verschoben, korrigierter Pfad 206), rosetta = Compiler-Granule (`naif -226 carries no granule`, `unread`). `ci-gate`/`ci-check`/`register-dropped` in_progress.
- **Blockade:** keine.
- **Braucht:** messenger/near-Route + `origin`-Zeilen an mycelium (`## An mycelium`); rosetta-Compiler-Arm (`spacecraft_ephemeris_compiler.rs`) messen; danach `ci_manage list` am neuen HEAD.

### Fixe-Tabellen-Zulassung (Phobos/Vega/Hayabusa)
- **Status:** wartend | **Bindung:** eigen + mycelium (Feder-Riss)
- **Trigger:** Mycelium manifestiert die CDN-Assets und setzt `url`/`origin`/`compiler`.
- **Lage:** (gemessen 2026-09-29) Feld-Verdikt steht: KRFM `RADIOMETER`/`PHOTOMETER` → `em`, unit `count` (bare INTEGER); Vega MISCHA `BX/BY/BZ PSSO` → `em`, `nT`; Hayabusa LIDAR `RANGE`/`SPCX` → `gravity`, `km`; `INCIDENCE/EMISSION/PHASE/FOV/N` = witness. `blocked_sources.φ:417-427` pending. Riss: `at halley`/`at itokawa` — beide Körpernamen 0 Treffer in `phi/`.
- **Blockade:** Zwei-Feder-Akt (`url`/`origin`/`compiler` = Mycelium), Körper-Registrierung.
- **Braucht:** `## An mycelium` (exakte `format`/`field`/`ttl`/`at`-Direktiven); Körper-/Frame-Registrierung `halley`/`itokawa`.

### `format vlde` — Admission + field/ttl
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Mycelium setzt `url`/`origin`/`compiler`.
- **Lage:** (gemessen 2026-09-29) Reader `src/archivar/vlies.rs`, extract-Branch `extract.rs:3254`, Compiler `tools/harvest/src/bin/vlies_density_compiler.rs`, Asset 206; `sources.φ`-Zeile 0 (sgrep). Riss: Workflow `vlies-density-cdn.yml:23` lädt vom Tag `ssd.jpl.nasa.gov`, Compiler lädt hoch auf `ssd.jpl.nasa.gov-vlies`.
- **Blockade:** Zwei-Feder; force-Slot-Verdikt (Vlies-Dichte ist keine Kraft).
- **Braucht:** `## An mycelium` (exakte Direktiven + Tag-Riss).

### Kuprat-Kanäle — Zeugen-Art `Substance`
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Mycelium re-manifestiert die vier Bins unter den neuen Magics; Beleg `phi/witnesses.φ`.
- **Lage:** (gemessen 2026-09-28) `WitnessKind::Substance` gebaut; laufende CDN-Bins tragen `0xCF86`.
- **Blockade:** Re-Manifest fehlt.
- **Braucht:** Mycelium → vier `witness substance`-Zeilen.

### Sonden-Flotte Asien/Russland — Compiler-Bin/Sample
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** je fehlendem Compiler-Bin/Sample ein Port-Schritt nach `docs/SOURCE_PORT.md`.
- **Lage:** (gemessen 2026-09-29) Arme `pds3_binary`/`pds3_img`/`pds4_binary`/`pds4_fits` (`src/archivar/pds4_fits.rs`) und `gras_2c` (`src/archivar/gras_2c.rs`) gebaut, `cargo check` 0/0; `blocked_sources.φ` pds4-fits/gras-2c auf `pending`; Compiler-Bin + Live-Sample fehlen.
- **Blockade:** kein Live-Sample; Compiler-Bin fehlt.
- **Braucht:** CDN-Trigger (mycelium); Sample/Verifikation, sobald eine Route eine Einzeldatei liefert.

### Asien/Russland-Kandidaten — Riss `mat-v5`
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Route der Arm/Compiler nach `docs/SOURCE_PORT.md`.
- **Lage:** (gemessen 2026-09-29) `sgrep gap mat-v5 phi/blocked_sources.φ` → 0; der Token-Eintrag behauptete „kein MAT-Reader", aber `src/archivar/mat5.rs::parse_mat5` und `matfile.rs::parse_mat` stehen — Riss geheilt (`blocked_sources.φ` gap-Token-Note korrigiert). Kein `extract()`-Arm `mat-v5` und keine `gap`-Direktive.
- **Blockade:** Arm-Wiring nicht angefordert.
- **Braucht:** Entscheidung, ob ein `mat-v5`-Arm (aus `mat5.rs`) gebaut wird; sonst Token descopen.

### HiPS-PNG-Quelle (CDS/Aladin MoRIC) — Reader-Arm
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ein HiPS-Reader-Arm nach `docs/SOURCE_PORT.md`; Beleg `phi/blocked_sources.φ::gap:hips-png ×1`.
- **Lage:** (gemessen 2026-09-29) `phi/blocked_sources.φ` parser-def/gap `hips-png` (MoRIC); CDS/Aladin Tianwen-1 MoRIC dir 206, properties 206 (HiPS png, order 7, frame mars); kein Einzeldatei-Asset.
- **Blockade:** Reader-Arm fehlt.
- **Braucht:** HiPS-Reader-Arm; Klassen-Träger `phi/blocked_sources.φ::gap:hips-png ×1`.

### `decline spectral-series` — Reklassifikation (Operator-Frage)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-/Rat-Wort.
- **Lage:** (gemessen 2026-09-28) `declined_sources.φ:1409` ONC-Hydrophon, `:4009` NOAA-NODD NRS; beide declined den Feld-Anspruch, CDN-Record bleibt.
- **Blockade:** keine.
- **Braucht:** Wort, ob als Zeugen reklassifiziert.

### commit_check-Riegel Ereignis→Folge — Alters-Riss
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-Wort zur Alters-/Session-Begrenzung.
- **Lage:** (gemessen 2026-09-28) Fixture gebaut (`commit_gate.rs` `ereignis_folge_violations`, `commit_check.rs`); Riss: Register ungebunden gelesen.
- **Blockade:** Riss.
- **Braucht:** Rat-Wort zur Begrenzung oder bewusste Ungebundenheit.

### LAB_A/MLZ NSE I(q,t) — privates Holding
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-/Operator-Wort zur Feld-/Wire-Karte der NSE-Serie.
- **Lage:** (gemessen 2026-09-28) Reader + Compiler gebaut (`src/archivar/lab_reader.rs`, `lab_reader_compiler.rs`); 13 Läufe [RETRACTED-SAMPLE] privat gesichert (`data/lab_a.data/SAMPLE_NSE_[RETRACTED-SAMPLE]/`, gitignored).
- **Blockade:** Wire-Lücke — NSE-Polarisation trägt keine ICRS-Position/keinen Kraftkanal.
- **Braucht:** Feld-/Wire-Entscheidung (Rat/Operator, in `## An future`); kein CDN ohne Operator-Einverständnis.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte `state/zustand/wartend.φ:3`)
- **Trigger:** `NED_BYPARAMS_TIMEOUT_TOKEN` per Mail.
- **Lage:** (gemessen 2026-09-28) zwei NED-Einträge im Ledger, kein Token.
- **Blockade:** Token fehlt.
- **Braucht:** mit Token den ByParams-Job fahren.

### Legacy-CDN-Assets `ssd.jpl.nasa.gov` — Disposition
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Mycelium re-manifestiert `spectra.bin`/`nvss.json`/`first14.json`/`curated48_spectra.bin` unter den Family-Tag.
- **Lage:** (gemessen 2026-09-28) die vier Register-`url`-Zeilen 404; Assets 200 unter Legacy-Tag.
- **Blockade:** Re-Manifest (mycelium).
- **Braucht:** danach Verdikt (löschen/halten in `dead_sources.φ`).

### `format ndk` — Reader-Arm fehlt
- **Status:** operator-gebunden (Feldentscheidung) | **Bindung:** eigen (Code) + future (Feld)
- **Trigger:** die NDK-Feld-Entscheidung des Operators.
- **Wort:** erwartet — welcher NDK-Skalar (`mw` am Centroid / `m0` / kein Skalar) in die 9 Kraft-Medien eingeht.
- **Lage:** (gemessen 2026-09-29) `src/archivar/ndk.rs` trägt `mw:34`/`scalar_moment_dyne_cm:30`/`parse_ndk:122`; ein `extract()`-Arm `ndk` ist absent (`tests.rs:7691` `format-gap`; `phi/sources.φ:7256` `format ndk` ohne `field`).
- **Blockade:** Feldentscheidung (Operator).
- **Braucht:** Operator-Wort → `extract()`-Arm + Test.

### auftrag-flyby2-kette — σ-Metrik
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der Fill-Run dieses Atoms; danach die σ-Metrik-Regel des revidierten Blatts.
- **Lage:** (gemessen 2026-09-29) Addendum `docs/paper/flyby-path-2-addendum-2026-09-29.md` (sha256 `0a3cc4ef…`) trägt die gefüllte 26-Zellen-Tubus-Registrierung (`data/flyby2/tube-juice-2026-09-28.json`), RTSW 23 h 56 min Retention gehalten; σ-Metrik superseded (Δ ≤ δ + 3·σ_recon, δ = 0.168 km) → `pending` mit Trigger (JUICE in-situ/Δ publiziert), nie als Zahl.
- **Blockade:** JUICE in-situ + Δ nicht publiziert.
- **Braucht:** bei Publikation `flyby_ephemeris_gate` (CI) gegen das Addendum.

### Benannte Risse (fremde Feder)
- **Status:** wartend | **Bindung:** eigen (nur Benennung)
- **Trigger:** kein eigener; Beleg `phi/blocked_sources.φ:88`.
- **Lage:** (gemessen 2026-09-28) DEMETER: gültig ist `phi/blocked_sources.φ:88` (Konto/`orderToken`); UA-identisch 403/403. `format vlde` = Mycelium.
- **Blockade:** fremde Feder.
- **Braucht:** Adressierung (`## An mycelium`).

### Unträgerlose Docs / Orphan-Zensus-Divergenz
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Messung mit frischem Bin; Beleg `register_lookup --orphan-docs` (0, gemessen 2026-09-29).
- **Lage:** (gemessen 2026-09-29) `register_lookup --orphan-docs` = 0 — Divergenz zur Vor-Runde (12). Die sensor/river-adressierten Docs (`arxiv-api.md`, `kapitulationen-pendings-inventur.md`, `dead-sources-relevanz.md`, `fremde-parser-sammlungen.md`) tragen jetzt oder der Bin hinkt (Freshness).
- **Blockade:** keine.
- **Braucht:** Re-Messung mit frischem Bin; bei 0 sind die Punkte geschlossen.

## Prosa-Träger (eigene)

- `docs/specs/livefeed-gate.md` | offene Marker = die `pending`-Felder der Ereignis-Tabelle (Spec-Inhalt); Träger Mountain.
- `docs/surveys/survey-raetsel-bestand.md` | Verdikt-Träger (Rats-Konsens 2026-09-29: Blatt + Survey-Zeile sind die Registerklasse; kein `phi/*.φ`); Träger Mountain.

## An mycelium (fremde Feder — Aufenthalt beim Eigentümer)
Origin: mountain folge201.

- **Fixe-Tabellen-Zulassung — exakte Mountain-Direktiven** (gemessen 2026-09-29): `format pds3_fixed_width` / `at mars` / `ttl 604800` / `field RADIOMETER1 pds3_krfm_radiometer1 inverse-square em count <τ> 0.0 0.0` (analog RADIOMETER4/PHOTOMETER1); `format pds3_fixed_width` / `at halley` / `ttl 604800` / `field "BX PSSO" pds3_mischa_bx_pso inverse-square em nT <τ> 0.0 0.0` (BY/BZ analog); `format pds4_fixed_width` / `at itokawa` / `ttl 604800` / `field RANGE pds4_hayabusa_lidar_range inverse-square gravity km <τ> 0.0 0.0`. Deine Federseite: `url` (CDN-Tag), `origin`, `compiler`. Riss: `at halley`/`at itokawa` — Körper-Registrierung fehlt.
- **`format vlde` — exakte Mountain-Direktiven** (gemessen 2026-09-29): `format vlde` / `ttl 604800` / `field count vlies_density_count inverse-square em count 604800 0.0 0.0` (extract `:3254` liest `fc.name`; τ spiegelt `sources.φ:8993`). Deine Federseite: `url` (`…/download/ssd.jpl.nasa.gov-vlies/vlies_density.vlde`), `origin procedure:`, `compiler`. Riss: Workflow-Tag `ssd.jpl.nasa.gov` vs Compiler-Tag `-vlies`.
- **ephemeris-cdn messenger/near — `origin` stale** (gemessen 2026-09-29): `phi/sources.φ:15611` (msgr) und `:15653` (near) tragen den Vor-Archiv-Pfad → 404. Korrigiert (206): `…/pub/naif/pds/data/mess-e_v_h-spice-6-v1.0/messsp_1000/…msgr_040803_120516_140823_od268sc_0.bsp`, `…/pub/naif/pds/data/near-a-spice-6-v1.0/nearsp_1000/…near_cruise_nav_v1.bsp`. Workflows `.github/workflows/messenger-ephemeris-cdn.yml:49` und `.github/workflows/near-ephemeris-cdn.yml:49` zugleich. `rosetta-ephemeris-cdn` ist separat (Compiler-Granule `naif -226`, `unread`).
- **4 `pending`-Konten-Träger** (gemessen 2026-09-28): `phi/blocked_sources.φ:373/377/381/385` von `blocked account` → `pending`; Träger `phi/blocked_sources.φ::pending ×4`.
- **Legacy-CDN-Assets** (gemessen 2026-09-28): `spectra.bin`/`nvss.json`/`first14.json`/`curated48_spectra.bin` unter Family-Tag re-manifestieren.

## An future (Operator-Queue, private)
Origin: mountain folge201.

- **ODF-Flyby-Fenster fehlt** (gemessen 2026-09-29): keines der vier ODF-Assets (Galileo `sources.φ:9764`, Cassini `:8977`, Messenger `:9810`, Rosetta `:8947`) trägt das Erd-Encounter-Fenster; die ODFs fehlen serverseitig (Messenger `data-odf/` 2007–2015, Rosetta 2005/2007/2009 pending). Einziger Weg: DSN/JPL-Rohdaten-Anfrage. *Frage:* Anfrage stellen? (Operator-Hand)
- **NDK-Feld-Entscheidung** — welcher Skalar in die 9 Kraft-Medien (`src/archivar/ndk.rs`, kein `extract()`-Arm).
- **Sonden-Flotte CSF/Konto-gated** (CNSA, ISRO PRADAN, MBRSC EMM).
- **CSES-Zugang** (SSDC, Trigger 2026-10-02).
- **GIC-Einreichung** (Operator-Hand).
- **KARI/ISRO-Konten** (Danuri/KASI, Chandrayaan-2/3, Aditya-L1).
- **NSE/SAMPLE_AUTHOR-Datenrechte** — CDN-manifestiert oder privates Holding? Zudem Dank an SAMPLE_CONTACT (Operator-Hand).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
