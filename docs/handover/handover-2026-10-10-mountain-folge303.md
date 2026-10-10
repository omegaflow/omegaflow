<!--
  title: Handover — Mountain-Folge 303 (2026-10-10)
  session: Mountain-Linie in einem Pass — Q(t)-Präzession gebaut (Nutation benannt-absent), ROOT-Sample gemessen, Vantage-Bias (g) geheilt, FMHY-Kandidaten klassifiziert
  class: handover
  date: 2026-10-10
  sha256: 48e89334a199cd59a89efe958cc0e55ba16e3ff4e97958a0cb396fea767cf25a
  status: live
-->
# Handover — Mountain-Folge 303 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-10-mountain-folge302.md` (→ `archiv/`).
flash only, kein pro/max.

## Burn: open 0.0000 · close 0.16 · cap 0.50 — Grund: line 0.0351 + 4 Diver (FMHY general 0.0364 · vantage-Bias grind-flash 0.0298 · ROOT general 0.0271 · Q(t)-Bau grind-flash); deepseek-flash, kein pro/max; Aggregat 19 Sessions $0.7006

## Offen (aufgeschlüsselt)

### Beobachtungsoperator + Fit — Q(t) Präzession gebaut; Nutation N(t) offen
- **Status:** eigen (Bau läuft) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt (IAU 2000A Nutation, SOFA `iauNut06a`)
- **Lage:** (gemessen 2026-10-10, `cargo check` 0/0) `src/mathematikerin/receiver.rs` trägt jetzt `cirs_to_gcrs(r_cirs_km, tdb_jd)` (IAU 2006 Fukushima–Williams-Präzession `P=Rx(-eps_a)·Rz(-psi_b)·Rx(phi_b)·Rz(gamma_b)`, SOFA `iauP06a`/`iauPnm06a`) + `itrf_to_icrs` (Komposition `cirs_to_gcrs(itrf_to_cirs(...))`) + 5 Tests (Norm-Erhaltung · J2000-Identität bis Frame-Bias · Orthogonalität · NaN→None · Komposition). **N(t) = IAU 2000A Nutation ist NAMED-ABSENT** (1365-Term-Serie überschreitet einen begrenzten Schritt) — kein stiller Zero, im `CIRS_TO_GCRS_CONVENTION`-String und im Modul-Endkommentar benannt (N=I); `cirs_to_icrs` ist nie definiert. Der ältere `ITRF_TO_CIRS_CONVENTION`-Satz „Q(t) is absent" bleibt für `itrf_to_cirs`' eigenen Ausgabeframe (CIRS) wahr.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** eigener begrenzter Dispatch — die IAU 2000A-Nutationsserie als Datentabelle + `nutation_matrix_n06a`, in `cirs_to_gcrs` einsetzen, Test gegen SOFA-Referenzwerte. Danach 2a (ODF-Kalibration), 2c-medium (VMF3/IONEX je Leg), 2d (Residuum-Bin `pioneer10_odf.bin` gegen DE440, kein LSQ).

### particle-cern — ROOT-Sample gemessen; Teil B (TStreamerElement/Branches) offen
- **Status:** eigen (Parser) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt (TStreamerElement-/TBranch-Decode aus dem Header)
- **Lage:** (gemessen 2026-10-10, Diver + `archive_search --verdict` bestätigt) Teil A `parse_streamer_info_header` steht (`cargo check` 0/0, Fixture-Test). Zwei gemessene ROOT-Kandidaten (CC0, magic `root`, TTree+`StreamerInfo` im Binärkopf): Record 401 `MasterclassData.root` 1 289 541 B, `https://opendata.cern.ch/record/401/files/MasterclassData.root` (HTTP 200 bestätigt), sha256 `8694a2ed…039b`, DOI `10.7483/OPENDATA.LHCb.E7EJ.JUWR`; Record 12361 `SMHiggsToZZTo4L.root` 42 400 229 B, sha256 `78b93558…aab5`, DOI `10.7483/OPENDATA.CMS.8FLU.UIQJ`. Lokale Arbeitskopien außerhalb des Baums: `/tmp/opencode/mc.bin`, `/tmp/opencode/rootmagic.bin`.
- **Blockade:** `parse_tree` bleibt verweigert, bis Teil B gebaut ist; die alte Blockade „kein ROOT-Sample im Baum" ist geheilt (Sample gemessen).
- **Braucht:** Teil B — `parse_tree` um TStreamerElement + fBranches/fLeaves erweitern, gegen `MasterclassData.root` (Record 401, schnell) verifizieren; Gate auf Indexdomäne vor dem Skalar-Reader.

### Vantage-Rest-Bias — (g) geheilt; (b)(c)(d) offen
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt je Ort
- **Lage:** (gemessen 2026-10-10, `cargo check` 0/0) (g) `src/archivar/port.rs:3761` **geheilt** — `frame.starts_with("at sun")` ist durch `declared_frame_class` ersetzt: Direktive `at <body>` → Barycenter, `on <body>` → Surface; leer/unklassifizierbar → `pending` (Block unberührt), nie ein Default-Glied (die Sonne ist nicht mehr privilegiert). (b) `src/archivar/odp.rs:9` `const EARTH = include_str!(…)` — Host als Parameter verlangt Signaturänderung von `station_velocity`/`downlink_rate` + 10 fremde Bins (`tools/measure/src/bin/pioneer*`, `galileo_*`, `orientation_probe`). (c) `src/weberin.rs:264-268` `frame_origin_name()` NAIF 10 — verlangt Feld in `Weberin`+`WeberinFeed` + 6 fremde Bins. (d) `src/weberin.rs:253-262` `woven_major_bodies()` `[1,2,4,5,6,7,8,301,399]` — verlangt ein Register + Träger + 4 fremde Bins.
- **Blockade:** (b)(c)(d) je ein breiter Call-Site-Refactor über die eigene Datei hinaus.
- **Braucht:** je Ort ein eigener begrenzter Dispatch mit benanntem Datei-/Bin-Scope. Grenzlinie (fünf Stimmen): der Code wählt keinen Körper — jeder Name/Position/Ursprung/Rahmen ist deklariert oder registriert, sonst `pending`, nie ein Default.

### FMHY-Quellen-Kandidaten-Pool (future-222) — klassifiziert; Verdikt offen
- **Status:** eigen (Quellen-Verdikt) | **Bindung:** mycelium (Ernte nach Verdikt)
- **Trigger:** je Klasse ein begrenzter Verdikt-Schritt nach `docs/SOURCE_PORT.md`
- **Lage:** (gemessen 2026-10-10, Diver; Liste bis Ende gelesen) `state/future/source-kandidaten-fmhy-2026-10-10.md` (445 Z.): 405 total / 402 NEW. Klassen: **research-data 74** (72 NEW) · **tool/service 301** (300) · **bypass-mirror 23** · **dead/nav 7**. 28 research-data-Einträge gemessen (`--verdict`+`--sniff`): u. a. ChinaRxiv 200, Neliti 403→blocked ip-blocked, All About Circuits 403→blocked, Open Textbook Library 200, IntechOpen 206. Die 23 bypass-mirror (Sci-Hub, Sci-Bot, PDFiles, Studocu/Exam-Downloader, Telegram-Kanäle …) sind eine **UrhG-§95a-/Umgehungs-Verdikt-Klasse**, keine Auslassung. **Operator-Wort 2026-10-10:** die 23 Einträge sind aus der Liste entfernt (445→422 Z.) und als Klasse `decline redistribution` verdiktet (`phi/declined_sources.φ`, Klasse über `fmhy.net`, keine Einzel-URL getrackt). **Erweiterung Operator-Wort 2026-10-10:** FMHY 35/35 Sektionen durch, **587** `🌐`-Sammelseiten-Verweise, 5 tief durchsucht (~55 physikalische Ressourcen, ~30 NEW); reichste Sektionen `/misc` + `/educational`; die meisten FMHY-NEW sind Viewer über registrierten Backends. Relevante NEW-Lücken am Baum verifiziert (nicht in `phi/sources.φ`): `losc.ligo.org`, `emsc.eu`, `imos.aodn.org.au`, `ngmdb.usgs.gov`, `worldclim.org`, `gadm.org`, `NOAA SURFRAD`, `awesome-lidar`, `KeepTrack` — Kandidaten-Notiz `state/mountain/fmhy-extension-2026-10-10.md`. **Tool-/Wissens-Scan (Operator-Frage 2026-10-10):** 13 Tool-Sektionen (10 575 Z.) + 10 Wissens-Seiten (7 203 Z.) durch; 58 Tool-Treffer (~45 netto neu: CyberChef/ImHex/Wireshark, tesseract/OCRmyPDF/Marker, DuckDB, nvtop, ParaView/MeshLab/tev, shellcheck/lychee) + 101 Wissens-Treffer (~60 Kern: NASA NTRS, Feynman, OpenStax, MathWorld, Falstad, KiCad); FMHY trägt kein astro-/scientific-Computation-Tooling — Notiz `state/mountain/fmhy-tools-wissen-2026-10-10.md`.
- **Blockade:** kein Verdikt geschrieben (nur klassifiziert/gemessen).
- **Braucht:** research-data- + Erweiterungs-Klasse: Register-Zeilen (Zulassung/`blocked`/`declined`) aus der Messung schreiben — ein begrenzter Dispatch, beginnend mit Gravitation/Seismik/Ozean (LIGO OSC, EMSC, IMOS). Tool-/Wissens-Scan: Werkzeuge sind kein `phi/`-Quellen-Verdikt — der Operator entscheidet über lokale Installation der netto-neuen Werkzeuge (Hardware/Consent).

### FMHY-Routing — Arm vs `phi` vs `tools` (Untersuchung)
- **Status:** eigen (Architektur) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt je Riss/Route
- **Lage:** (gemessen 2026-10-10, Diver + Rat fünf Stimmen) Grenze ist die **Manifestations-Achse**: Query → `archive_search`-Arm (`--help` = Kanon); Messwert (`field` + Einheit + Cadence) → `phi/sources.φ` + Compiler; Operator (Bytes-Werkzeug) → `tools/`/Notiz; Blick/Portal → Lead. 23 Schnittstellen gemessen (12 Query · 9 Bulk · 2 Portal). **Riss 1 (EMSC) + Riss 2 (Copernicus) am Baum geklärt:** `sources.φ:250` trägt `field magnitude/depth` (volle Quelle, Arm = Dualität); CDS/DEM sind gebaut (declined = separater `browser.dataspace`-Viewer). Offen: Riss 3 (KNMI/Mindat 401 **mit** Token), Riss 4 (Overpass-Admission), Riss 5 (Referenz vs. Messwert). Notiz `state/mountain/routing-untersuchung-2026-10-10.md`. **Messungen 2026-10-10:** Riss 3 (KNMI/Mindat) — kein Key in `.secrets.local` → `pending` (Operator-Route); Riss 4 (Overpass) — HTTP 406 direkt+Proton → bot-gated `pending`; Riss 5 — SURFRAD/ECAD/DWD/WorldClim = Messwert, GADM/Worldview = Referenz/Anzeige. **UI-Runde gefahren** (`mountain-ui`: Duck Haiku · Claude Sonnet · Qwen · Z.ai GLM, alle geantwortet): Achse tragfähig, aber **Manifestation statt Quelle** klassifizieren + **Lizenz-/Zugangs-Gate vor** der Achse + **Kontext-Achse** ergänzen — Konvergenz mit dem Rat (Riss = fehlende Gate-Achse). **Korrektur (Operator-Frage 2026-10-10):** die „12 Query"-Klassifikation war zu mechanisch („hat API" ≠ Arm); Präzedenz **`--supermag`/`--heasarc` = Dual** (Quelle + Arm), `--opencellid` = reiner Arm. Die Kandidaten sind Messquellen (→ `phi`) oder Referenz/Modell/decline — **kein neuer Arm klar gerechtfertigt**; die blockierte Arm-Hälfte ist gegenstandslos (max. optionales EMSC-Dual).
- **Blockade:** Arm-Hälfte gegenstandslos (kein neuer Arm gerechtfertigt); Riss 4 (Overpass) bot-gated `pending`. **Incident 2026-10-10:** der frisch freigegebene KNMI-EDR-Schlüssel lief beim strukturellen Auslesen der Tyk-Erfolgsseite (`browser_query` auf einen `<p>`) in den Session-Transcript → nach Secret-Hygiene als **veröffentlicht** zu behandeln; der Wert liegt als `KNMI_API_KEY` → **Rotation empfohlen** (neuer Schlüssel, Operator-Hand). Lehre: eine gerade ausgestellte Schlüssel-Seite wird nie ausgelesen — der Operator kopiert direkt. → Operator-Wort 2026-10-10: **Risiko benannt und akzeptiert** (keine Rotation; öffentliche read-only Non-Commercial-Daten).
- **Braucht:** (a) **Korrektur: kein neuer Arm ist klar gerechtfertigt** (Präzedenz supermag/heasarc = Dual; die Kandidaten sind Messquellen → `phi`) — die blockierte Arm-Hälfte ist gegenstandslos, max. ein optionales EMSC-Dual. (b) **Fünf `phi`-Compiler stehen — Mountain-Arbeit** (Parser-Konstruktion), je `cargo check` 0/0 + end-to-end gemessen: `surfrad_compiler.rs` · `ecad_compiler.rs` · `dwd_cdc_compiler.rs` · `worldclim_compiler.rs` · `aodn_compiler.rs` (alle in `tools/harvest/src/bin/`). Nur die `phi/sources.φ`-Manifestations-Direktiven (`url`/`origin`/`compiler`) je Quelle sind **Mycelium-Recht** → `## An mycelium`. (c) **KNMI gebunden via EDR (live 2026-10-10):** der Operator hat den EDR-Zugang freigeschaltet; `knmi_compiler.rs` fetcht `/edr/v1/collections/10-minute-in-situ-meteorological-observations/position` (CoverageJSON) — **77 Stationen**, Felder `knmi_air_temperature` (K) · `knmi_wind_speed`/`_gust` (m/s) · `knmi_precipitation_intensity` (kg/m²s), `.bin` roundtrip-geprüft, `cargo check` 0/0. `KNMI_API_KEY` trägt jetzt den EDR-Schlüssel (die Open-Data-Route ist damit weg — EDR ist der gewählte Weg). Quelle bereit für die `## An mycelium`-Registrierung. **Mindat:** `mindat_probe.rs` — Token erkannt, Konto **Level 0**; Aktivierungs-/API-Antrag **eingereicht** → `wartend` (`state/zustand/wartend.φ: mindat-api-level1`).

### Asservatenkammer — Mountain-Träger (9 Doks)
- **Status:** eigen (Register/Träger) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt je Dok
- **Lage:** (gemessen 2026-10-10, Zensus `docs/surveys/survey-2026-10-10-asservatenkammer-zensus.md`) Die 9 Mountain-Doks tragen jetzt je einen Namenträger in dieser Übergabe (siehe Block). Fremd verteilt via `## An`: `exposom-matrix` → sensory, `stoerungs-experiment` → river, `research-api-mcp` → mycelium.
- **Blockade:** ohne ersten Schritt bleibt ein Dok trägerlos.
- **Braucht:** je Dok der genannte erste Schritt:
  - `survey-2026-09-14-kapitulationen-pendings-inventur.md` | 32 offen | Register-Inventur der aufgegebenen/offenen Quellen gegen `phi/*.φ`
  - `survey-2026-10-08-open-sources-delta.md` | 18 offen | Gegen-Audit gegen `phi/sources.φ`
  - `survey-2026-09-03-orphan-verdicts.md` | 15 offen (156 Orphan-Releases) | Disposition je Register-Gap (Step 3)
  - `survey-2026-10-09-redistribution-alternativen.md` | 10 offen (41 `redistribution`-declined) | freie Alternative je Quelle
  - `survey-2026-10-09-domaenen.md` | 8 offen | Domänen-Landschaft, Lücken als Lücken messen
  - `survey-2026-09-16-fremde-parser-sammlungen.md` | 6 offen | fremde Parser-/Compiler-Sammlungen vergleichen
  - `survey-2026-10-08-fmhy-research-landscape.md` | 5 offen | in die FMHY-Verdikt-Klasse falten (Punkt oben)
  - `survey-2026-10-07-fmhy-forschungsschicht.md` | 2 offen | FMHY-Wiki vermessen → FMHY-Verdikt-Klasse
  - `survey-2026-09-17-omegaflow-legacy-konzepte.md` | 2 offen | verlorene, heute entblockbare Konzepte durchgehen

### PEP + Tudat — Schritt 0 (Präzession) gebaut; Nutation offen
- **Status:** eigen (Register + Bau) | **Bindung:** eigen
- **Trigger:** Nutation N(t) (Schritt-0-Rest), dann Schritt 1 (Pioneer-10-Residuum gegen DE440)
- **Lage:** (gemessen 2026-10-10) Zwei Träger-Doks, namentlich: `docs/concepts/eigene-ephemeride.md` (die Vision: eine eigene Ephemeride aus allen Zeugen; Wert = `witness_set`/Unabhängigkeit; O−C gegen ein Haus ist `fit-residuum`, nie `blindtest`) und `docs/surveys/survey-2026-10-10-ephemeris-quellen.md` (Referenz-Landschaft). Die vier Häuser (DE440 / INPOP19a / EPM2021 / PETREL19) sind Zeugen und längst als normale `ephemeris_*`-Quellen registriert. **PEP** (`github.com/jbattat/pep_core`) und **Tudat** (arXiv:2510.23179) sind nur Lizenz-/Verfahrens-**Referenz** (nie im Shipped Binary), `reference` als `origin`-Herkunftsmarker — der Rat (Option a, neue Klasse) ist vom Baum überstimmt (PETREL19-Präzedenz: ein Programm lebt als `origin`/`compiler` auf einer Datenzeile, nie als eigene Registerklasse; unser eigenes Werk wird nach seinem Erzeuger benannt). **Schritt 0 Teil:** `cirs_to_gcrs` (Präzession) gebaut; Q(t) ist noch precession-only → die Nutation ist der verbleibende Schritt-0-Rest.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Nutation (wie Punkt 1), dann Schritt 1 — Pioneer-10-ODF durch die Kette, Residuum gegen DE440, kein Fit.

### ci-check-Kern-Tests — wartend
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ci-check/ci-gate-Lauf am HEAD nach dem tiff-Heil-Commit
- **Lage:** (gemessen 2026-10-10, `ci_manage status`) HEAD `1504aaf0b`; die ci-gate-Läufe sind `queued`/`pending` (38061716952, 38061666923, 38061298302, …); register-coverage `completed/cancelled`. Kein Testausführungs-Ergebnis am HEAD gemessen. Die vier Tests (ck/extract/igrf/hdf4) sind im Baum geheilt; der lib-test-Compile-Blocker (`apply_predictor`) ist seit `3c4786a3b` geheilt.
- **Blockade:** kein CI-Ergebnis (Runner-Queues).
- **Braucht:** nach eigenem Push `ci_manage list` + `ci_manage log <ci-gate-id>` — die vier Tests am HEAD prüfen.

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
- **Lage:** (gemessen 2026-10-10, unverändert) `te_ground_truth` in `.github/workflows/te-bias-n.yml:48`; kein Mountain-Schritt bis zum Artefakt. Wahrheit `state/zustand/wartend.φ`.
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
„hast du archive search all und den roster befragt?" | 2026-10-10 | Operator (Session, Mountain 298)
„bitte für council immer auch archive search all und den roster und bitte lasse archive search all und den roster auch auf LLR los" | 2026-10-10 | Operator (Session, Mountain 299)
„aber warum bauen wir PEP nicht in rust nach? und bitte ja LLR … sag, ob ich ihn vorziehen soll" | 2026-10-10 | Operator (Session, Mountain 299)
„warum nur so ein kleiner roster und warum kein vollport wir wollen doch womöglich 100% rust std" | 2026-10-10 | Operator (Session, Mountain 299)
„und dann möchte ich dass du nochmal eine -all und roster recherche machst welche referenzen wir noch harvestenn können um unsere eigenen ephemeriden zu bauen?" | 2026-10-10 | Operator (Session, Mountain 299)
„bitte commit und übergabe in einer frischen session dann direkt 1-4" | 2026-10-10 | Operator (Session, Mountain 299)
„Starte die Mountain-Linie in einem Pass … bitte frage mit --all und max roster und dem rat klären Abgeschlossen und übergeben" | 2026-10-10 | Operator (Session, Mountain 301)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater … kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–290)
„mach das ab jetzt automatisch — committe und pushe selbst" | 2026-10-07 | Operator (Session, Mountain 264)
„ich glaube reference ist passender oder aber warum haben wir mit allen anderen ephemeriden compilern kein problem aber mit dem PEP nachbau schon?" | 2026-10-10 | Operator (Session, Mountain 302)
„aber müssen wir es pep nennen wenn wir unsere eigenen ephemeriden nennen und dann gab es ja noch das zweite ephemeriden tool haben wir das alles?" | 2026-10-10 | Operator (Session, Mountain 302)
„nein das meine ich nicht ach mensch geh doch mal in das ephemeriden vision dok" | 2026-10-10 | Operator (Session, Mountain 302)
„bypass-mirror 23 (UrhG-§95a- was bedeutet das die möchte ich bitte raus haben keine fragwürdigen links" | 2026-10-10 | Operator (Session, Mountain 303)
„sehr gut denn die seite birgt grosse schätze aber halt auch problematische" | 2026-10-10 | Operator (Session, Mountain 303)
„das waren jetzt die datenseiten aber hast du auch die tools und wissens seiten nochmal gescannt?" | 2026-10-10 | Operator (Session, Mountain 303)
„können wir nun eine untersuchung machen was davon als arme in archive search sollte, was in tools und was in phi dateien?" | 2026-10-10 | Operator (Session, Mountain 303)
„ja bitte Nächste Schritte (in der Übergabe): Riss 3/4/5 messen → je Arm ein net.rs-Zweig + cargo check, je B-Kandidat url+Compiler+field; dann die UI-Runde in eigener Gruppe, dann das Routing als Register-Zeile." | 2026-10-10 | Operator (Session, Mountain 303)
„aber gehören die da wirklich rein? GPlates · GWOSC/LIGO · Navy Weather · Mindat · KNMI · Overpass · EMSC (dual)" | 2026-10-10 | Operator (Session, Mountain 303)
„warum sind die in Das eigentliche offene Stück bleibt: … compiler sind deine aufgabe" | 2026-10-10 | Operator (Session, Mountain 303)
„kannst du bitte Operator-Queue (Future): KNMI-/Mindat-Key … wie die keynamen heissen" | 2026-10-10 | Operator (Session, Mountain 303)
„mindat und knmi sind eingeloggt bitte bereite bis zur Kante vor" | 2026-10-10 | Operator (Session, Mountain 303)
„sind beide eingetragen" | 2026-10-10 | Operator (Session, Mountain 303)
„ich bin in mindat eingeloggt bitte prüfe was fehlt" | 2026-10-10 | Operator (Session, Mountain 303)
„ok genau das hatte ich schon gemacht" | 2026-10-10 | Operator (Session, Mountain 303)
„bitte nochmal warst im timeout" | 2026-10-10 | Operator (Session, Mountain 303)
„ich hab ihn eingegeben" | 2026-10-10 | Operator (Session, Mountain 303)
„aber ganz ehrlich wie problematisch ist das?" | 2026-10-10 | Operator (Session, Mountain 303)
„akzeptiert" | 2026-10-10 | Operator (Session, Mountain 303)

## An mycelium

Origin: mountain-303 (2026-10-10).

- **Asservatenkammer (Träger-Vorschlag):** `docs/surveys/survey-2026-10-08-research-api-mcp.md` (3 offen) — Consensus/Elicit/SciSpace/Perplexity als Recherche-APIs; Auth-Route/Keys = Mycelium-Domäne. Bitte einen Träger-Punkt mit nächstem Schritt setzen. Zensus: `docs/surveys/survey-2026-10-10-asservatenkammer-zensus.md`.
- **SURFRAD-Compiler gebaut (Routing B):** `tools/harvest/src/bin/surfrad_compiler.rs` (gemessen 2026-10-10: SURFRAD-`*.dat` 1440 Z./Tag geparst, Felder `surfrad_shortwave_down` + `surfrad_direct_normal` W/m², hourly, `cargo check` 0/0; Sample `https://gml.noaa.gov/aftp/data/radiation/surfrad/tbl/2024/tbl24001.dat` HTTP 200). Bitte die `phi/sources.φ`-Zeile setzen — `url` (CDN-Asset) + `origin https://gml.noaa.gov/aftp/data/radiation/surfrad/` + `compiler tools/harvest/src/bin/surfrad_compiler.rs` + `field` — plus die Manifestations-Workflow. Grenzfall (Rat): Mountain trägt die Verdikt-/`ttl`-Seite, Mycelium die Materialisierung (`url`/`origin`/`compiler`) — kein stiller Schreibakt.
- **Vier weitere `phi`-Quellen (Compiler stehen — Mountain-Arbeit):** je `phi/sources.φ`-Zeile (`url` CDN-Asset + `origin` + `compiler` + `field`) + Manifestations-Workflow:
  - `ecad_compiler.rs` — ECA&D daily stations (`https://knmi-ecad-assets-prd.s3.amazonaws.com/download/ECA_blend_{tx,rr}.zip`), `eca_tx/tn/tg` (K, 86400), `eca_rr` (kg/m²).
  - `dwd_cdc_compiler.rs` — DWD CDC daily KL (`https://opendata.dwd.de/climate_environment/CDC/observations_germany/climate/daily/kl/`), `dwd_air_temperature_{mean,max,min}` (K), `dwd_surface_pressure` (Pa), `dwd_wind_{speed_mean,gust_max}` (m/s), `dwd_precipitation_height` (m), `dwd_relative_humidity` (1), `dwd_vapour_pressure` (Pa).
  - `worldclim_compiler.rs` — WorldClim 2.1 (`https://geodata.ucdavis.edu/climate/worldclim/2_1/base/wc2.1_10m_{tavg,tmin,tmax,prec}.zip`), `worldclim_{tavg,tmin,tmax}` (K), `worldclim_prec` (mm), monatlich.
  - `aodn_compiler.rs` — IMOS/AODN THREDDS/OPeNDAP (`https://thredds.aodn.org.au/thredds/dodsC/…`), `aodn_temperature` (K), `aodn_salinity` (PSU), `aodn_velocity_{u,v}` (m/s), `aodn_wave_height` (m).
  - `knmi_compiler.rs` — KNMI **EDR** (`https://api.dataplatform.knmi.nl/edr/v1/collections/10-minute-in-situ-meteorological-observations/position`, CoverageJSON), `knmi_air_temperature` (K), `knmi_wind_speed`/`_gust` (m/s), `knmi_precipitation_intensity` (kg/m²s), **77 Stationen** (live 2026-10-10).

## An river

Origin: mountain-303 (2026-10-10).

- **Asservatenkammer (Träger-Vorschlag):** `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (11 offen) — Operator-Vision „Stein ins Wasser" (`field_te_query`); Feld/TE = deine Domäne, bitte einen Träger-Punkt setzen.
- **`src/archivar/parse.rs:2489` auto-deref bleibt** (aus folge300/301): `QuantityRole::parse(*role)` → `parse(role)`; der Tuple-Deref `Ok((*role, …))` bleibt. Die P10-`QuantityRole`-Schicht ist river-eigen — bitte am eigenen Pass verifizieren.

## An sensory

Origin: mountain-303 (2026-10-10).

- **Asservatenkammer (Träger-Vorschlag):** `docs/surveys/survey-2026-10-04-exposom-matrix.md` (82 offene Marker) — somatisch/psychosomatische Exposom-Quellenmatrix, „eine Zeile je Krankheitsklasse = eine TE-Messung". Der Gegenstand ist die Weberin/Gesundheits-Zeugen-Linie — bitte einen Träger-Punkt mit erstem begrenztem Schritt setzen. Zensus: `docs/surveys/survey-2026-10-10-asservatenkammer-zensus.md`.

## An future

Origin: mountain-303 (2026-10-10).

- **Keine offenen Operator-Akte.** Die zwei Zugangswerte sind eingetragen (`KNMI_API_KEY` = EDR-Schlüssel, `MINDAT_TOKEN`); der beim Ausstellen in den Transcript gelaufene KNMI-EDR-Schlüssel ist per Operator-Wort 2026-10-10 **als Risiko akzeptiert** (keine Rotation; Begründung: öffentliche read-only Non-Commercial-Daten, reines Quota-Token). Mindat → `wartend` (`state/zustand/wartend.φ: mindat-api-level1`).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch", 2026-10-07)
trägt Commit und Push. **Dieses Atom (Mountain 303):** Q(t)-Präzession gebaut
(`src/mathematikerin/receiver.rs`: `cirs_to_gcrs` + `itrf_to_icrs` + 5 Tests, `cargo check` 0/0;
Nutation benannt-absent) · ROOT-Sample gemessen (Record 401/12361, CC0) · Vantage-Bias (g)
geheilt (`src/archivar/port.rs`) · FMHY-Kandidaten klassifiziert (405/402; 28 research-data
gemessen) · Asservatenkammer: 9 Mountain-Träger gesetzt. **Geteilter Baum:** fremde uncommittete
Hunks (`AGENTS.md`, `docs/concepts/tools-map.md`, `opencode.json`, `tools/utils/src/bin/archive_search*`)
nicht angefasst. Eigene Pfade: `src/mathematikerin/receiver.rs` · `src/archivar/port.rs` ·
`docs/handover/handover-2026-10-10-mountain-folge303.md` ·
`docs/handover/archiv/handover-2026-10-10-mountain-folge302.md` (Move) · `phi/declined_sources.φ` (Operator-Wort 2026-10-10: 23 bypass-Mirror-Einträge aus der Kandidatenliste entfernt, Klasse `decline redistribution`).
