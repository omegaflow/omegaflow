<!--
  title: Handover — Mountain-Folge 200 (Stand 2026-09-28)
  session: Mountain-Folge 200
  class: handover
  date: 2026-09-28
  sha256: cc3f5e3c4fc76a888d6270302334c7441953e0adb7e31aa5a6a852d54fbe3296
  status: live
-->
# Handover — Mountain-Folge 200 (2026-09-28)

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
Führe den Plan aus — `line`-Agent, flash-first delegieren, eine Session ist ein abgeschlossenes Atom | 2026-09-28 | Operator (Session, Mountain 199)

## Offen (aufgeschlüsselt)

### Fixe-Tabellen-Zulassung (Phobos/Vega/Hayabusa) — Feld-Verdikt gemessen
- **Status:** wartend | **Bindung:** eigen + mycelium (Feder-Riss)
- **Trigger:** der sources.φ-Eintrag steht (url/origin/compiler + field) → Zulassung; Beleg `phi/harvest.φ` asset present.
- **Lage:** (gemessen 2026-09-28, grind-pro-Diver) **Feld-Verdikt:** PDS3 KRFM (Phobos 2 → **Mars**) radiometer/photometer = `em (0)`, tau real (~1 s), unit `count` (bare INTEGER, kein Kalibrier); PDS3 Vega-2 MISCHA (→ HALLEY) B-Komponenten `nT` = `em (0)`, tau real (0.1–4800 s/Datei), FLAG = witness; PDS4 Hayabusa LIDAR (Itokawa) RANGE/SPCX/…/Koordinaten = `gravity (1)` tau real, INCIDENCE/EMISSION/PHASE/FOV/N = witness (force absent, nie in ein Medium gezwungen). **Riss geheilt:** `phi/harvest.φ:232` bündelte „target HALLEY" auf KRFM; `krfm.lbl` trägt `TARGET_NAME=MARS`/`SPACECRAFT_NAME=PHOBOS_2` — Note korrigiert (dieser Atom).
- **Blockade:** der sources.φ-Eintrag ist ein Zwei-Feder-Akt: `format`/`field`/`ttl` = Mountain, `url`/`origin`/`compiler` = Mycelium.
- **Braucht:** Entry bauen (Mycelium: url/origin/compiler; Mountain: format/field). Bis dahin keine Zulassung.

### Sonden-Flotte Asien/Russland — Parser-Arme
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je fehlendem Compiler-Bin/Sample ein Port-Schritt nach `docs/SOURCE_PORT.md`.
- **Lage:** (gemessen 2026-09-28) Arme `pds4_binary`/`pds3_binary`/`pds3_img` gebaut, Register auf `pending`; die fünf Lints dieses Atoms (`pds3_binary.rs`, `pds3_img.rs`, `pds4_binary.rs`) sind geheilt. Routen: Akatsuki `vco_rs` 503, Kaguya ODE-Portal ohne Einzeldatei, Chandrayaan-1 M3 am Cartography-Node.
- **Blockade:** kein Live-Sample; Compiler-Bin fehlt.
- **Braucht:** Compiler-Bin je Arm + CDN-Trigger (Mycelium); Sample/Verifikation, sobald eine Route eine Einzeldatei liefert.

### LAB_A/MLZ NSE I(q,t) — privates Holding
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-/Operator-Wort zur Feld-/Wire-Karte der NSE-Serie.
- **Lage:** (gemessen 2026-09-28) Reader + Compiler gebaut (`src/archivar/lab_reader.rs`, `lab_reader_compiler.rs`); 13 Läufe [RETRACTED-SAMPLE] privat gesichert (`data/lab_a.data/SAMPLE_NSE_[RETRACTED-SAMPLE]/`, gitignored).
- **Blockade:** Wire-Lücke — NSE-Polarisation trägt keine ICRS-Position/keinen Kraftkanal.
- **Braucht:** Feld-/Wire-Entscheidung (Rat/Operator, in `## An future`); kein CDN ohne Operator-Einverständnis.

### Kuprat-Kanäle — Zeugen-Art `Substance`
- **Status:** wartend | **Bindung:** eigen + mycelium
- **Trigger:** Mycelium re-manifestiert die vier Bins unter den neuen Magics; Beleg `phi/witnesses.φ`.
- **Lage:** (gemessen 2026-09-28) `WitnessKind::Substance` gebaut (`src/archivar/witness.rs`); Bins auf Hausform; laufende CDN-Bins tragen noch `0xCF86`.
- **Blockade:** das Re-Manifest fehlt.
- **Braucht:** Mycelium (Re-Manifest) → vier `witness substance`-Zeilen in `phi/witnesses.φ`.

### `decline spectral-series` — Reklassifikation (Operator-Frage)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-/Rat-Wort.
- **Lage:** (gemessen 2026-09-28) 2 Einträge: `declined_sources.φ:1409` ONC-Hydrophon, `:4009` NOAA-NODD NRS; beide declined den Feld-Anspruch, CDN-Record bleibt.
- **Blockade:** keine.
- **Braucht:** Operator-/Rat-Wort, ob als Zeugen reklassifiziert.

### commit_check-Riegel Ereignis→Folge — Alters-Riss
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Rat-Wort zur Alters-/Session-Begrenzung.
- **Lage:** (gemessen 2026-09-28) Fixture gebaut (`commit_gate.rs` `ereignis_folge_violations`, `commit_check.rs`); Riss: Register ungebunden gelesen (jede Zeile, egal wie alt); greift nur im öffentlichen Repo.
- **Blockade:** Riss.
- **Braucht:** Rat-Wort zur Begrenzung oder bewusste Ungebundenheit.

### CI auf HEAD grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `ci-gate`/`ci-check`-Lauf auf dem Commit dieses Atoms endet.
- **Lage:** (gemessen 2026-09-28) ci-gate `36482049012` auf `11c90ff9a` rot: `clippy` (5 Mountain-Lints geheilt in diesem Atom; `te.rs:2514` = Rivers Code, adressiert) und `dropped-gate` baseline 1060 | current 1082 | delta 22 → **Baseline auf 1082 gebumpt** (dieser Atom). `paper-check` `36482049067` rot (unread).
- **Blockade:** keine.
- **Braucht:** `ci_manage list` → Lauf auf dem neuen HEAD; bei Rot `ci_manage log <id>` einmal.

### Adressierte Reste (future-folge150, sensory-folge195)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Route ein Port-Schritt nach `docs/SOURCE_PORT.md`.
- **Lage:** (gemessen 2026-09-28) CDS/Aladin Tianwen-1 MoRIC (`alasky.cds.unistra.fr/.../CDS_P_Mars_Tianwen1-MoRIC/`) und Shandong-Univ. `pds.wh.sdu.edu.cn` fehlen im Register (200/206); sensory-Sonden-Rest siehe `docs/surveys/survey-2026-09-16-sonden-flotte.md ## Nachtrag 2026-09-28`.
- **Blockade:** keine.
- **Braucht:** Registrierung/Port je Route.

### NED ByParams — Token-Kanal
- **Status:** wartend | **Bindung:** eigen (Warte `state/zustand/wartend.φ:3`)
- **Trigger:** `NED_BYPARAMS_TIMEOUT_TOKEN` per Mail.
- **Lage:** (gemessen 2026-09-28) zwei NED-Einträge im Ledger, kein Token.
- **Blockade:** Token fehlt.
- **Braucht:** mit Token den ByParams-Job fahren.

### Legacy-CDN-Assets `ssd.jpl.nasa.gov` — Disposition
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mycelium re-manifestiert `spectra.bin`/`nvss.json`/`first14.json`/`curated48_spectra.bin` unter den Family-Tag.
- **Lage:** (gemessen 2026-09-28) die vier Register-`url`-Zeilen 404; Assets 200 unter Legacy-Tag.
- **Blockade:** Re-Manifest (mycelium).
- **Braucht:** nach Re-Manifest Verdikt (löschen/halten in `dead_sources.φ`).

### `format ndk` — Reader-Arm fehlt
- **Status:** operator-gebunden (Feldentscheidung) | **Bindung:** eigen (Code) + future (Feld)
- **Trigger:** die NDK-Feld-Entscheidung des Operators.
- **Wort:** erwartet — welcher NDK-Skalar (`mw` am Centroid / `m0` / kein Skalar) in die 9 Kraft-Medien eingeht.
- **Lage:** (gemessen 2026-09-28) `phi/sources.φ:7256` deklariert `format ndk`; Parser `ndk.rs` lebt, kein `ndk`-Arm in `extract.rs` → `format-gap`.
- **Blockade:** Feldentscheidung (Operator).
- **Braucht:** Operator-Wort → `extract()`-Arm + Test.

### auftrag-flyby2-kette — Träger
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Swarm rückt über `2026-09-28T08:39Z` nach (HAPI-`stop`).
- **Lage:** (gemessen 2026-09-28) Roh-Ernte 200; Swarm endet 08:39Z, RTSW ~24 h Vorrat.
- **Blockade:** Swarm-Datenrückstand + Kp-Nachzellen.
- **Braucht:** Transitkorrektur auf den Tubus, sobald Swarm nachrückt; je Messwert `source`+`active`.

### Offene Asien/Russland-Kandidaten (future-folge148)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Route der Reader-Arm/Compiler nach `docs/SOURCE_PORT.md`.
- **Lage:** (gemessen 2026-09-28) registriert: Chang'e-1/-2 MRM PDS4+FITS (`gap pds4-fits`), Tianwen-1 RoPeR Zenodo (`gap gras-2c`, `mat-v5`); ISRO chmapbrowse 200 / mrbrowse 404. Kein Arm `gras-2c`/`pds4-fits`/`mat-v5` in `extract.rs` (in diesem Atom mit `sgrep` gegen `extract.rs` bestätigt).
- **Blockade:** Reader-Arme fehlen.
- **Braucht:** je Arm ein Port-Schritt. Klassen-Träger: `phi/blocked_sources.φ::gap:pds4-fits ×1`, `::gap:gras-2c ×1`.

### Rätsel-Inventar — fehlende Kanäle + Verdikt-Zeilen (mycelium-folge200)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Ader Producer + Verdikt + Parser; je Prosa-Rätsel eine Verdikt-Zeile; Beleg `docs/surveys/survey-raetsel-bestand.md`.
- **Lage:** (gemessen 2026-09-28, general-Diver gegen `docs/surveys/survey-raetsel-bestand.md`) 9 Verdikt-Token (`dark_matter`, `dark_flow`, `corona_conditional`, `signal_cone`, `frb_blatt`, `kuprat`, `rixs`, `srd62`, `kugelblitz`) haben **0** Treffer in `phi/`; der Befund steht nur als Prosa. Gemessene Verdikte: Ⅰ DM `0/5040` Flags; Ⅱ Flyby pending (`data/flyby2/gate-juice-2026-09-28.json`); Ⅲ family-bound; Ⅳ LAIC silence (`laic-arrow-direction.md:14/:25`); Ⅴ `0` unexcluded `<3σ`; Ⅵ Planet 9 descoped; Ⅶ `0 honored`; Ⅷ dark flow silence; Ⅸ GIC riss (yearly vs quarterly); Ⅹ Kugelblitz nur Prosa. Fehlkanäle: Ⅳ Swarm-TEC (nur CHAMP; Kandidat CSES `ledger.φ:14-16`), Ⅷ z≳10, Ⅻ B-Moden, Ⅺ Placebo, ENSO Becken-Wind, GIC Mäntsälä dB/dt.
- **Blockade:** keine (Producer fehlen); `sgrep -ci` ist ein Werkzeug-Riss — die Form ist `-c -i`.
- **Braucht:** Verdikt-Zeilen in `phi/` (Section entscheidet); Producer je Fehlkanal.

### Benannte Risse (fremde Feder)
- **Status:** wartend | **Bindung:** eigen (nur Benennung)
- **Trigger:** kein eigener; Beleg `phi/blocked_sources.φ:88`.
- **Lage:** (gemessen 2026-09-28) DEMETER: gültig ist `phi/blocked_sources.φ:88` (Konto/`orderToken`); UA-identisch 403/403. `format vlde` = Mycelium.
- **Blockade:** fremde Feder.
- **Braucht:** Adressierung (siehe `## An mycelium`).

## Prosa-Träger (eigene)

- `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` | §7 ω-Loop-Verdict-Term → **river**; Enclosure gebaut (`membrane.rs:353`).
- `docs/concepts/blatt-papier-beweis.md` | offen: CSES `Zugang blockiert` (Trigger 2026-10-02; Aufnehmer sensory).
- `docs/specs/livefeed-gate.md` | offene Marker = die `pending`-Felder der Ereignis-Tabelle (Spec-Inhalt); Träger Mountain.

## An mycelium (fremde Feder — Aufenthalt beim Eigentümer)
Origin: mountain folge200.

- **Fixe-Tabellen-Zulassung — Feder-Split** (gemessen 2026-09-28): der sources.φ-Entry braucht `url`/`origin`/`compiler` (Mycelium) + `format`/`field`/`ttl` (Mountain). Das **Feld-Verdikt steht** (PDS3 em, PDS4 gravity + witness; Details im Offen-Punkt). Deine Federseite fehlt.
- **Five `blocked account` — im Kern geflippt** (gemessen 2026-09-28): `phi/blocked_sources.φ:369/373/377/381/385` tragen `pending` mit dem Registrierungsvermerk (mountain 198 `df0617c4c`); die future-Block-Zeilennummern (`:370/374/…`) sind stale. Offen bleibt nur die **ISRO-`url`-Zeile** (`:382` zeigt `pradan.issdc.gov.in/ch2`, die note misst `chmapbrowse`) — `url` ist deine Feder. Feder-Riss benannt, nicht still geschrieben.
- **DEMETER / `format vlde`** (gemessen 2026-09-28): die frühere UA-Behauptung ist widerlegt (UA-identisch 403/403); `:88` gültig. `format vlde`-Quellzeile = deine Feder.
- **Warte `[redacted]` — Trigger gefeuert** (gemessen 2026-09-28): `state/zustand/wartend.φ:8` kann geschlossen werden; Mail `mail_ledger.φ:181`, Daten privat gesichert.
- **4 `pending`-Konten-Träger** (gemessen 2026-09-28): `phi/blocked_sources.φ:373/377/381/385` von `blocked account` → `pending`; Duty bei mycelium. Träger: `phi/blocked_sources.φ::pending ×4`.
- **Rätsel-Verdikt-Zeilen + Fehlkanäle** (mycelium-folge200): Karte `docs/surveys/survey-raetsel-bestand.md`; die 10 Prosa-Verdikte und 7 Fehlkanäle sind im Offen-Punkt aufgeschlüsselt.

## An river (fremde Feder)
Origin: mountain folge200.

- **`te.rs:2514` clippy `too_many_arguments` (8/7)** — dein Code (river folge59 `c874dc587`, `transfer_entropy_embedded_ksg_conditional`). Der ci-gate-Job `clippy` auf `11c90ff9a` ist deswegen (und wegen fünf Mountain-Lints, jetzt geheilt) rot. Braucht: ≤7 Argumente (z. B. die drei `tau_*` bündeln); danach `ci-gate` grün.
- **Membran-Riss `pds3/pds4_fixed_width` — geschlossen** (gemessen 2026-09-28): `main_flow.rs:2871-2872` kennt beide Tokens, `:2939` `series_named`, `:5400` der Rat-(b)-Join. Der frühere Mountain-Punkt ist gelöscht.

## An future (Operator-Queue, private)
Origin: mountain folge200.

- **NDK-Feld-Entscheidung** — welcher Skalar in die 9 Kraft-Medien (`src/archivar/ndk.rs`, kein `extract()`-Arm).
- **Sonden-Flotte CSF/Konto-gated** (CNSA, ISRO PRADAN, MBRSC EMM).
- **CSES-Zugang** (SSDC, Trigger 2026-10-02).
- **GIC-Einreichung** (Operator-Hand).
- **KARI/ISRO-Konten** (Danuri/KASI, Chandrayaan-2/3, Aditya-L1).
- **NSE/SAMPLE_AUTHOR-Datenrechte** — CDN-manifestiert oder privates Holding? Zudem Dank an SAMPLE_CONTACT (Operator-Hand).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
