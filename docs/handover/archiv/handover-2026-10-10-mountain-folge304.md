<!--
  title: Handover — Mountain-Folge 304 (2026-10-10)
  session: Mountain-Linie in einem Pass — ci-gate-Red `two_way_doppler` (8/7) geheilt (Struct-Bündelung), Exposom-Repräsentativpunkt als Annahme-Akt registriert (Operator-Wort)
  class: handover
  date: 2026-10-10
  sha256: 7b667d6834860c912e0780ba30baf379322c32794bee86d56ce3f9529839f2d8
  status: live
-->
# Handover — Mountain-Folge 304 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-10-mountain-folge303.md` (→ `archiv/`).
flash only, kein pro/max.

## Burn: open 0.0000 · close 0.033 · Grund: line only, deepseek-flash, kein pro/max; 0 Diver

## Offen (aufgeschlüsselt)

### Beobachtungsoperator + Fit — Q(t) Präzession gebaut; Nutation N(t) offen
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt (IAU 2000A Nutation, SOFA `iauNut06a`)
- **Lage:** (gemessen 2026-10-10, `cargo check` 0/0) `src/mathematikerin/receiver.rs` trägt `cirs_to_gcrs` (IAU 2006 Fukushima–Williams-Präzession `P=Rx(-eps_a)·Rz(-psi_b)·Rx(phi_b)·Rz(gamma_b)`) + `itrf_to_icrs` + 5 Tests. **N(t) = IAU 2000A Nutation ist NAMED-ABSENT** (1365-Term-Serie; `CIRS_TO_GCRS_CONVENTION` N=I); `cirs_to_icrs` nie definiert.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** eigener begrenzter Dispatch — IAU-2000A-Serie als Datentabelle + `nutation_matrix_n06a`, in `cirs_to_gcrs` einsetzen, Test gegen SOFA-Referenzwerte. Danach 2a (ODF-Kalibration), 2c-medium (VMF3/IONEX je Leg), 2d (Residuum-Bin `pioneer10_odf.bin` gegen DE440, kein LSQ).

### particle-cern — ROOT-Sample gemessen; Teil B (TStreamerElement/Branches) offen
- **Status:** eigen (Parser) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt (TStreamerElement-/TBranch-Decode aus dem Header)
- **Lage:** (gemessen 2026-10-10) Teil A `parse_streamer_info_header` steht. Zwei ROOT-Kandidaten (CC0, magic `root`): Record 401 `MasterclassData.root` 1 289 541 B, `https://opendata.cern.ch/record/401/files/MasterclassData.root` (HTTP 200, sha256 `8694a2ed…039b`, DOI `10.7483/OPENDATA.LHCb.E7EJ.JUWR`); Record 12361 `SMHiggsToZZTo4L.root` 42 400 229 B, sha256 `78b93558…aab5`.
- **Blockade:** `parse_tree` verweigert, bis Teil B gebaut ist.
- **Braucht:** Teil B — `parse_tree` um TStreamerElement + fBranches/fLeaves erweitern, gegen `MasterclassData.root` verifizieren; Gate auf Indexdomäne vor dem Skalar-Reader.

### Vantage-Rest-Bias — (g) geheilt; (b)(c)(d) offen
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt je Ort
- **Lage:** (gemessen 2026-10-10, `cargo check` 0/0) (g) `src/archivar/port.rs:3761` **geheilt** (`declared_frame_class`: `at <body>` → Barycenter, `on <body>` → Surface; leer → `pending`). (b) `src/archivar/odp.rs:9` `const EARTH = include_str!(…)` — Host als Parameter + 10 fremde Bins. (c) `src/weberin.rs:264-268` `frame_origin_name()` NAIF 10 — Feld in `Weberin`+`WeberinFeed` + 6 fremde Bins. (d) `src/weberin.rs:253-262` `woven_major_bodies()` `[1,2,4,5,6,7,8,301,399]` — Register + Träger + 4 fremde Bins.
- **Blockade:** (b)(c)(d) je ein breiter Call-Site-Refactor über die eigene Datei hinaus.
- **Braucht:** je Ort ein eigener begrenzter Dispatch mit benanntem Datei-/Bin-Scope. Grenzlinie: der Code wählt keinen Körper — deklariert oder registriert, sonst `pending`, nie ein Default.

### FMHY-Quellen-Kandidaten-Pool (future-222) — klassifiziert; Verdikt offen
- **Status:** eigen (Quellen-Verdikt) | **Bindung:** mycelium (Ernte nach Verdikt)
- **Trigger:** je Klasse ein begrenzter Verdikt-Schritt nach `docs/SOURCE_PORT.md`
- **Lage:** (gemessen 2026-10-10) `state/future/source-kandidaten-fmhy-2026-10-10.md` (422 Z.): 402 NEW. Klassen: **research-data 74** (72 NEW) · **tool/service 301** · **dead/nav 7**. 28 research-data gemessen (`--verdict`+`--sniff`). Die 23 bypass-mirror sind als Klasse `decline redistribution` verdiktet (`phi/declined_sources.φ`, über `fmhy.net`). FMHY 35/35 Sektionen + Tool-/Wissens-Scan durch; relevante NEW-Lücken: `losc.ligo.org`, `emsc.eu`, `imos.aodn.org.au`, `ngmdb.usgs.gov`, `worldclim.org`, `gadm.org`, NOAA SURFRAD, `awesome-lidar`, `KeepTrack`. Notizen `state/mountain/fmhy-extension-2026-10-10.md` · `state/mountain/fmhy-tools-wissen-2026-10-10.md`.
- **Blockade:** kein Verdikt geschrieben (nur klassifiziert/gemessen).
- **Braucht:** research-data- + Erweiterungs-Klasse: Register-Zeilen (Zulassung/`blocked`/`declined`) aus der Messung schreiben — ein begrenzter Dispatch, beginnend mit Gravitation/Seismik/Ozean (LIGO OSC, EMSC, IMOS). Tool-/Wissens-Scan: Werkzeuge sind kein `phi/`-Quellen-Verdikt — Operator entscheidet über lokale Installation (Hardware/Consent).

### FMHY-Routing — Arm vs `phi` vs `tools` (Untersuchung)
- **Status:** eigen (Architektur) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt je Riss/Route
- **Lage:** (gemessen 2026-10-10) Grenze ist die **Manifestations-Achse**: Query → `archive_search`-Arm; Messwert → `phi/sources.φ` + Compiler; Operator-Werkzeug → `tools/`; Blick/Portal → Lead. **Korrektur:** kein neuer Arm klar gerechtfertigt (Präzedenz `--supermag`/`--heasarc` = Dual; `--opencellid` = reiner Arm) — die Kandidaten sind Messquellen → `phi`. Riss 1 (EMSC) + 2 (Copernicus) am Baum geklärt. Offen: Riss 4 (Overpass — HTTP 406 direkt+Proton → bot-gated `pending`), Riss 5 (Referenz vs. Messwert — SURFRAD/ECAD/DWD/WorldClim = Messwert, GADM/Worldview = Referenz/Anzeige). **KNMI gebunden via EDR (live, 77 Stationen, `knmi_compiler.rs`, `KNMI_API_KEY`); Mindat Token erkannt, Konto Level 0 → wartend.** Notiz `state/mountain/routing-untersuchung-2026-10-10.md`. UI-Runde gelaufen (Duck·Claude·Qwen·Z.ai), Konvergenz mit dem Rat.
- **Blockade:** Arm-Hälfte gegenstandslos; Riss 4 bot-gated `pending`. **Incident:** der KNMI-EDR-Schlüssel lief beim Auslesen der Tyk-Erfolgsseite in den Transcript → per Operator-Wort 2026-10-10 **als Risiko akzeptiert** (keine Rotation).
- **Braucht:** (a) kein neuer Arm (max. optionales EMSC-Dual). (b) Fünf `phi`-Compiler stehen (Mountain-Arbeit, `tools/harvest/src/bin/`: surfrad/ecad/dwd-cdc/worldclim/aodn) — `phi/sources.φ`-Manifestations-Direktiven je Quelle sind Mycelium-Recht. (c) KNMI-/Surfrad-Registrierung (siehe `### Mycelium-φ-Blöcke`). Mindat: Trigger = Level-1-Freigabe (Wiedervorlage 2026-10-13).

### Asservatenkammer — Mountain-Träger (9 Doks)
- **Status:** eigen (Register/Träger) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt je Dok
- **Lage:** (gemessen 2026-10-10, Zensus `docs/surveys/survey-2026-10-10-asservatenkammer-zensus.md`) Die 9 Mountain-Doks tragen je einen Namenträger in dieser Übergabe (siehe Zensus). Fremd verteilt via `## An`: `exposom-matrix` → sensory, `stoerungs-experiment` → river, `research-api-mcp` → mycelium.
- **Blockade:** ohne ersten Schritt bleibt ein Dok trägerlos.
- **Braucht:** je Dok der genannte erste Schritt:
  - `survey-2026-09-14-kapitulationen-pendings-inventur.md` | 32 offen | Register-Inventur gegen `phi/*.φ`
  - `survey-2026-10-08-open-sources-delta.md` | 18 offen | Gegen-Audit gegen `phi/sources.φ`
  - `survey-2026-09-03-orphan-verdicts.md` | 15 offen | Disposition je Register-Gap (Step 3)
  - `survey-2026-10-09-redistribution-alternativen.md` | 10 offen | freie Alternative je Quelle
  - `survey-2026-10-09-domaenen.md` | 8 offen | Domänen-Landschaft, Lücken als Lücken messen
  - `survey-2026-09-16-fremde-parser-sammlungen.md` | 6 offen | fremde Parser-/Compiler-Sammlungen vergleichen
  - `survey-2026-10-08-fmhy-research-landscape.md` | 5 offen | in die FMHY-Verdikt-Klasse falten
  - `survey-2026-10-07-fmhy-forschungsschicht.md` | 2 offen | FMHY-Wiki vermessen → FMHY-Verdikt-Klasse
  - `survey-2026-09-17-omegaflow-legacy-konzepte.md` | 2 offen | verlorene, heute entblockbare Konzepte durchgehen

### Mycelium-φ-Blöcke (gefaltet aus `mycelium-folge302` `## An mountain`)
- **Status:** eigen (Register/Verdikt) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Register-Schritt je Block (Mycelium-Materialisierung folgt)
- **Lage:** (gemessen 2026-10-10 via `mycelium-folge302` + `phi/sources.φ:1996-2004`) **VNP46A3-Block steht** (url/terms/format/origin/compiler/header/on/ttl/field). **open-lidar-data**: Asset+Workflow success (`38082503772`, `open_lidar_data_be_dhmv2.bin` 50 616 273 B, sha256 `e6560344…`), Verdikt-Zeile fehlt (Mycelium lieferte url/origin/compiler/format/sha256, `terms`/`at`/`ttl`/`field` sind Mountain). **Fünf Klima-Quellen** (SURFRAD/ECAD/DWD/WorldClim/AODN): Compiler stehen, Verdikt-Zeilen fehlen; `ecad`-Riss (`ecad.eu` vs. Quellhost `knmi-ecad-assets-prd.s3.amazonaws.com`) zu entscheiden.
- **Blockade:** offene `terms`/`at`/`ttl`/`field` je Block; `register_sort` verlangt die `ttl`-Zeile. `phi/sources.φ` war zwischenzeitlich parallel fremd-editiert (nicht angefasst).
- **Braucht:** je Block `url` + `origin` + `compiler` + `format` + `sha256` + `terms` + `at`/`ttl` + `field` (Mountain-Verdikt) — dann Mycelium-Materialisierung/Workflow.

### Exposom-Matrix → TE-Paar-Feed — Repräsentativpunkt ist Annahme-Akt, kein Messschritt
- **Status:** wartend | **Bindung:** eigen (Mountain-Feder)
- **Trigger:** extern gedeckter Repräsentativpunkt je Zeile gesetzt (Operator-Hand; Operator-Wort 2026-10-10)
- **Lage:** (gemessen 2026-10-10 via `sread docs/handover/handover-2026-10-10-sensory-folge253.md:313` + `sread docs/surveys/survey-2026-10-04-exposom-matrix.md:324-357`; Operator-Wort 2026-10-10) Kein offener Datensatz (TOLIFE, AAMOS-00, 8018238, ADARP, CrossCheck, Labbaf) trägt eine im Datensatz gemessene Koordinate; die x-Kern-Serien (OpenAQ/Open-Meteo/NASA POWER) stehen wie in der Matrix gemessen. Die frühere Dispatch-Framing „Repräsentativpunkte messen" ist widerlegt.
- **Blockade:** der Repräsentativpunkt ist eine wissenschaftliche **Annahme** (Region→Gitterpunkt / Stadt→Station), keine Messung — kein autonomer Mountain-Schritt; er gehört nicht in Mountains autonomen Bereich.
- **Braucht:** je Zeile den extern gedeckten Repräsentativpunkt (Annahme) — erst dann wird die Zeile `ja` und darf (Mountain-Feder) die `phi/sources.φ`-Zeile, dann (Mycelium) den Te-Paar-CI-Feed. Das steht als Braucht im Träger.

### PEP + Tudat — Schritt 0 (Präzession) gebaut; Nutation offen
- **Status:** eigen (Register + Bau) | **Bindung:** eigen
- **Trigger:** Nutation N(t) (Schritt-0-Rest), dann Schritt 1 (Pioneer-10-Residuum gegen DE440)
- **Lage:** (gemessen 2026-10-10) Zwei Träger-Doks: `docs/concepts/eigene-ephemeride.md` (Vision) und `docs/surveys/survey-2026-10-10-ephemeris-quellen.md`. Die vier Häuser (DE440/INPOP19a/EPM2021/PETREL19) sind als `ephemeris_*`-Quellen registriert. **PEP** und **Tudat** sind nur Lizenz-/Verfahrens-**Referenz** (`reference` als `origin`-Herkunftsmarker; PETREL19-Präzedenz: ein Programm lebt als `origin`/`compiler` auf einer Datenzeile, nie als eigene Registerklasse). Schritt 0 Präzession gebaut; Q(t) precession-only → Nutation ist der Rest.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Nutation (wie oben), dann Schritt 1 — Pioneer-10-ODF durch die Kette, Residuum gegen DE440, kein Fit.

### ci-check-Kern-Tests — wartend
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ci-check/ci-gate-Lauf am HEAD nach dem `two_way_doppler`-Fix (Push)
- **Lage:** (gemessen 2026-10-10) Der strukturelle Rot-Lauf `ci-gate build` (`too_many_arguments 8/7`, `src/mathematikerin/receiver.rs:104`) ist **geheilt** (Struct-Bündelung `TwoWayDopplerInputs`, `cargo check` 0/0). Die vier Kern-Tests (ck/extract/igrf/hdf4) sind im Baum geheilt; kein Testausführungs-Ergebnis am neuen HEAD gemessen.
- **Blockade:** kein CI-Ergebnis (Runner-Queues).
- **Braucht:** nach dem Push `ci_manage list` + `ci_manage log <ci-gate-id>` — den grünen Lauf messen.

### Flyby-Kette — Residual in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-09, unverändert) 157 ODF-Referenzen; `doppler.rs` absent; Wahrheit `state/zustand/wartend.φ:34`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### iEEG — Riss beigelegt: registriertes Wort 2026-10-06 maßgeblich
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort, das den Riss über 2026-10-06 hebt
- **Lage:** (gemessen 2026-10-10, unverändert) iEEG = privates Experiment (`state/zustand/wartend.φ:40`), kein CDN; `eeglab::eeg_from_bin` akzeptiert `Samples::Double`.
- **Blockade:** keine.
- **Braucht:** kein Schritt — nur ein neues Operator-Wort öffnet es.

### GIC-Paper — Trigger: Mycelium-Artefakt
- **Status:** wartend | **Bindung:** mycelium (Träger folge295 `#te-ground-truth`)
- **Trigger:** `te-bias-n`-Lauf `38038722712` Abschluss → Mycelium meldet den Ground-Truth-Abschnitt
- **Lage:** (gemessen 2026-10-10, unverändert) `te_ground_truth` in `.github/workflows/te-bias-n.yml:48`; kein Mountain-Schritt bis zum Artefakt.
- **Blockade:** kein CI-Ergebnis.
- **Braucht:** nach Mycelium-Meldung — Paper §3.5/Abstract/§7 nachziehen (Mountain).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern; Lauf lokal/silent,
  nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen:
  der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„mach das ab jetzt automatisch — committe und pushe selbst" | 2026-10-07 | Operator (Session, Mountain 264)
„ich glaube reference ist passender oder aber warum haben wir mit allen anderen ephemeriden compilern kein problem aber mit dem PEP nachbau schon?" | 2026-10-10 | Operator (Session, Mountain 302)
„aber müssen wir es pep nennen wenn wir unsere eigenen ephemeriden nennen …" | 2026-10-10 | Operator (Session, Mountain 302)
„nein das meine ich nicht ach mensch geh doch mal in das ephemeriden vision dok" | 2026-10-10 | Operator (Session, Mountain 302)
„bypass-mirror 23 (UrhG-§95a- was bedeutet das die möchte ich bitte raus haben keine fragwürdigen links" | 2026-10-10 | Operator (Session, Mountain 303)
„sehr gut denn die seite birgt grosse schätze aber halt auch problematische" | 2026-10-10 | Operator (Session, Mountain 303)
„das waren jetzt die datenseiten aber hast du auch die tools und wissens seiten nochmal gescannt?" | 2026-10-10 | Operator (Session, Mountain 303)
„können wir nun eine untersuchung machen was davon als arme in archive search sollte, was in tools und was in phi dateien?" | 2026-10-10 | Operator (Session, Mountain 303)
„ja bitte Nächste Schritte (in der Übergabe): Riss 3/4/5 messen … dann die UI-Runde in eigener Gruppe, dann das Routing als Register-Zeile." | 2026-10-10 | Operator (Session, Mountain 303)
„aber gehören die da wirklich rein? GPlates · GWOSC/LIGO · Navy Weather · Mindat · KNMI · Overpass · EMSC (dual)" | 2026-10-10 | Operator (Session, Mountain 303)
„kannst du bitte Operator-Queue (Future): KNMI-/Mindat-Key … wie die keynamen heissen" | 2026-10-10 | Operator (Session, Mountain 303)
„mindat und knmi sind eingeloggt bitte bereite bis zur Kante vor" | 2026-10-10 | Operator (Session, Mountain 303)
„sind beide eingetragen" · „ich bin in mindat eingeloggt bitte prüfe was fehlt" · „ok genau das hatte ich schon gemacht" | 2026-10-10 | Operator (Session, Mountain 303)
„aber ganz ehrlich wie problematisch ist das?" · „akzeptiert" (KNMI-Key-Risiko) | 2026-10-10 | Operator (Session, Mountain 303)
„zudem schreibt das die Kante ist — ehrlich: Der nächste Schritt ist kein weiterer Messschritt von mir. Er ist ein wissenschaftlicher Annahme-Akt: für jede Zeile müsste ein extern gedeckter Repräsentativpunkt gesetzt werden (Region→Gitterpunkt / Stadt→Station) — das ist eine Annahme, keine Messung, und gehört nicht in meinen autonomen Bereich (die x-Kern-Serien OpenAQ/Open-Meteo/NASA POWER bleiben wie in der Matrix gemessen). Erst mit diesem Punkt wird die Zeile ja und darf (Mountain-Feder) in phi/sources.φ, dann (Mycelium) in den Te-Paar-CI-Feed. Das steht als Braucht im Träger." | 2026-10-10 | Operator (Session, Mountain 304)

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch", 2026-10-07)
trägt Commit und Push. **Dieses Atom (Mountain 304):** die ci-gate-Rot-Ursache
`too_many_arguments (8/7)` in `src/mathematikerin/receiver.rs:104` geheilt — die acht
Parameter von `two_way_doppler` in die Struktur `TwoWayDopplerInputs` gebündelt (kein
`#[allow]`), 5 Test-Call-Sites nachgezogen, `cargo check` 0/0 · `cargo fmt` sauber. Das
Exposom-Repräsentativpunkt-Missverständnis („messen") per Operator-Wort als Annahme-Akt
registriert und als `wartend`-Punkt mit Trigger geführt. `mycelium-folge302`-`## An mountain`
in den Offen-Bestand gefaltet. **Geteilter Baum:** fremde uncommittete Hunks
(`docs/handover/handover-2026-10-10-sensory-folge253.md`,
`docs/surveys/survey-2026-10-04-exposom-matrix.md`,
`tools/utils/src/bin/archive_search/osf.rs`) nicht angefasst. Eigene Pfade:
`src/mathematikerin/receiver.rs` ·
`docs/handover/handover-2026-10-10-mountain-folge304.md` ·
`docs/handover/archiv/handover-2026-10-10-mountain-folge303.md` (Move).
