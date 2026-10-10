<!--
  title: Handover — Mountain-Folge 302 (2026-10-10)
  session: Mountain-Linie in einem Pass — Reduktionskette Schritt 2b (ITRF→CIRS), TStreamerInfo-Header, Register-Messungen (VMF3-GRID-URL, PEP-Rat)
  class: handover
  date: 2026-10-10
  sha256: 79edebafbe9ce3b2a4e4c8d16bef059334b5491420a03e3d7e6b55ff04ac3a62
  status: live
-->
# Handover — Mountain-Folge 302 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-10-mountain-folge301.md` (→ `archiv/`).
flash only, kein pro/max.

## Burn: open 0.0000 · close 0.17 · cap 0.50 — Grund: line 0.0248 + council 0.0334 + general 0.0208 (ci-check/ci-gate-Messung) + grind-flash 0.0730 (ITRF→CIRS 0.0173 + TStreamerInfo-Header 0.0557) + grind-flash VMF3-GRID-URL; Aggregat 11 Sessions $0.4945; deepseek-flash

## Offen (aufgeschlüsselt)

### Beobachtungsoperator + Fit — Kette Schritt 2b gebaut; Schritt 2c: Q(t) CIRS→GCRS
- **Status:** eigen (Bau läuft) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt (Q(t): IAU-2006-Präzession × IAU-2000A-Nutation)
- **Lage:** (gemessen 2026-10-10, `cargo check` 0/0) `src/mathematikerin/observer.rs` trägt jetzt `EopSample` · `earth_rotation_angle_rad` (IAU 2000 ERA) · `itrf_to_cirs` (Polar Motion W(t)=R3(−s′)·R2(x_p)·R1(y_p) + R3(−ERA), s′=0 benannte Vernachlässigung) · 4 Tests (Norm-Erhaltung · Identität bei ERA=0/xp=yp=0 · ERA-J2000-Konstante · NaN→None). Konvention erdfern an ERFA `eraPom00` verifiziert (Transponierten-Hand im Kommentar benannt). Die Funktion heißt bewusst `itrf_to_cirs`, **nie** `itrf_to_icrs` — Q(t) ist absent (Modul-Endkommentar benennt es).
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Schritt 2c — `cirs_to_gcrs(r_cirs_km, tdb_jd)` (IAU 2006 Präzession + IAU 2000A Nutation, SOFA `iauPnm06a`/`iauC2i06a`) + `itrf_to_icrs`-Komposition; ein begrenzter Dispatch + `cargo check`. Danach 2a (ODF-Kalibration), 2c-medium (VMF3/IONEX je Leg), 2d (Residuum-Bin `pioneer10_odf.bin` vs. Kern + Station gegen DE440, kein LSQ).

### particle-cern — Teil A gebaut (TStreamerInfo-Header); Teil B offen (Mitglieder/Branches + ROOT-Sample)
- **Status:** eigen (Parser) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt (TStreamerElement-/TBranch-Decode aus dem Header)
- **Lage:** (gemessen 2026-10-10, `cargo check` 0/0) `src/archivar/root.rs` trägt `parse_streamer_info_header` (Version i16 · byte_count · class_name TString · checksum · n_members) + `r_streamer`-Fixture-Test + Negativtests. Layout gemessen an ROOT-v6-36-Referenz (`dobject.html`/`streamerinfo.html`/`tobject.html`), ROOT-Quelle (`TStreamerInfo.cxx:5616`, `TBufferFile.cxx:2933/:2862`) und uproot5 (`deserialization.py:194`). `parse_tree` bleibt verweigert (kein ROOT-Sample im Baum).
- **Blockade:** kein ROOT-Sample im Baum — der Header-Parser ist nur am synthetischen Fixture geprüft, nicht an echten Event-Bytes.
- **Braucht:** Teil B — ein gemessenes ROOT-Event-TTree-Sample beschaffen (`archive_search`/`--verdict`; ein offenes CERN-Open-Data-File) ohne Python, dann `parse_tree` um TStreamerElement + fBranches/fLeaves erweitern; Gate auf Indexdomäne vor dem Skalar-Reader.

### Ephemeriden-Harvest 1-4 — Parser gebaut; Register-Token entschieden; VMF3-GRID-URL gemessen (Mycelium)
- **Status:** eigen (Parser) | **Bindung:** mycelium (sources.φ + Workflows)
- **Trigger:** Mycelium faltet den `## An mycelium`-Block dieser Übergabe und setzt die End-Token
- **Lage:** (gemessen 2026-10-10) Parser 1-4 grün (LLR-MINI, ITRF2020-SINEX, planetary radar, VMF3 site/grid). Token entschieden (folge301 `## An mycelium`). **NEU:** die VMF3-GRID-Origin-URL ist gemessen (siehe `## An mycelium`).
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Mycelium — Token setzen + `vmf3_grid.bin` mit der gemessenen URL bauen.

### LLR — Parser CRD + MINI gebaut; Runtime-Arm offen
- **Status:** eigen (Parser) | **Bindung:** mycelium (Block+Workflow) · river (main_flow)
- **Trigger:** Mycelium schreibt den `sources.φ`-Block (POLAC MINI + Zenodo CRD) + `llr-cdn.yml`; River setzt `| "llr"` in `main_flow.rs` `series_rows`
- **Lage:** (gemessen 2026-10-10, unverändert) `src/archivar/llr.rs` parst CRD v2.01 und POLAC-MINI; `llr_compiler` End-to-End grün; `llr.bin`-Block steht (Mycelium). Reiner Parser (null Physik).
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Mycelium — `llr-cdn.yml`; River — `| "llr"` in `main_flow.rs`.

### PEP + Tudat — die zwei offenen Ephemeriden-Programme der eigenen-Ephemeride-Vision
- **Status:** eigen (Register + Bau) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt der Kette (Q(t) `cirs_to_gcrs`), dann Schritt 1 der Vision (Pioneer-10-Residuum gegen DE440)
- **Lage:** (gemessen 2026-10-10) Zwei Träger-Dokumente, hier namentlich geführt (der nächste Pass findet sie sonst nicht): **`docs/concepts/eigene-ephemeride.md`** (die Vision: eine **eigene Ephemeride aus allen Zeugen** — NASA+ESA+sowjetisch+japanisch+Pioneer-thermisch; Wert = `witness_set`/Unabhängigkeit, nicht Vollständigkeit; O−C gegen ein Haus ist `fit-residuum`, nie `blindtest`) und **`docs/surveys/survey-2026-10-10-ephemeris-quellen.md`** (die Referenz-Landschaft: LLR-Normalpunkte, ITRF2020, JPL-Radar-Astrometrie, VMF1/VMF3, ICRF3, GRAVITY — was noch zu ernten ist). Die vier **Häuser** (DE440 / INPOP19a / EPM2021 / PETREL19) sind dort **Zeugen** und längst als normale `ephemeris_*`-Quellen registriert — kein Code-Zeuge, keine fünfte Klasse: `ephemeris_epm_*` (`compiler tools/harvest/src/bin/epm_compiler.rs`, `phi/sources.φ:2057-2129`), `ephemeris_petrel19_*` (`:2345-2366`), INPOP19a (`compiler tools/harvest/src/bin/inpop_compiler.rs`, `:19793`). **Zwei offene Ephemeriden-*Programme*** (kein Haus): **PEP** (Fortran, CC BY-NC-SA, `github.com/jbattat/pep_core`) und **Tudat** (TU Delft, open-source, arXiv:2510.23179 — DSN/ESTRACK-Doppler+Range, PRIDE-VLBI, MPC/NSDC-Astrometrie, Prefit/Postfit-Residuen; Postfit-Doppler 1-5 mHz MRO/GRAIL). PEP ist das einzige noch nicht registrierte Haus; **Tudat ist jetzt gemessen, aber weder geprüft noch gebaut**. **Riss, benannt:** der Rat (Option a, neue Klasse) wird vom Baum überstimmt — ein Programm lebt als `origin`/`compiler` **auf einer Datenzeile** (PETREL19-Präzedenz); unser eigenes Werk wird nach seinem Erzeuger benannt, nie nach einem Haus.
- **Blockade:** keins der beiden Programme ist als Rust-Kette gebaut; Voraussetzung ist die Reduktionskette (Vision-Schritt 0).
- **Braucht:** Schritt 0 abschließen (Q(t) `cirs_to_gcrs` — der nächste begrenzte Bau-Schritt), dann Schritt 1 (Pioneer-10-ODF durch die Kette, Residuum gegen DE440, kein Fit). PEP/Tudat nur als **Lizenz-/Verfahrens-Referenz** (nie im Shipped Binary); `reference` als `origin`-Herkunftsmarker genügt (keine neue Klasse, kein Canon-Akt).

### Asservatenkammer-Zensus — getragen; Mountain-Kandidaten offen
- **Status:** eigen (Quellen/Register) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt je Kandidat
- **Lage:** (gemessen 2026-10-10) Der Zensus `docs/surveys/survey-2026-10-10-asservatenkammer-zensus.md` nennt **12 handelbare Doks** aus der Asservatenkammer (82/32/18/15/11/10/8/6/5/3/2 offene Marker; `register_lookup --orphan-docs` = 22 trägerlose Marker-Doks). Neue Gate-Regel `commit_check` **own-handover** (`tools/gate/src/bin/commit_check.rs`, `c0441466d`): ein neu hinzugefügtes `docs/*`-Dok muß im selben Commit in der **eigenen** Übergabe genannt sein. Fast alle Alt-„Träger" hängen an einer breiten Zitat-Liste (`handover-2026-10-10-sensory-folge250.md`), kein Bearbeiter.
- **Blockade:** keines der 12 Doks hat einen Träger-Punkt in einer Live-Übergabe.
- **Braucht:** je Kandidat ein Träger-Punkt mit erstem begrenztem Schritt. **Mountain (eigene, 9):** `survey-2026-09-03-orphan-verdicts` (156 Orphan-Releases → Disposition, Schritt 3) · `survey-2026-10-08-open-sources-delta` (Gegen-Audit) · `survey-2026-09-14-kapitulationen-pendings-inventur` (Register-Inventur) · `survey-2026-10-09-redistribution-alternativen` (41 `redistribution`-declined → freie Alternative) · `survey-2026-10-09-domaenen` · `survey-2026-09-16-fremde-parser-sammlungen` · `survey-2026-10-08-fmhy-research-landscape` · `survey-2026-10-07-fmhy-forschungsschicht` · `survey-2026-09-17-omegaflow-legacy-konzepte`. **Fremd (verteilt via `## An`):** `exposom-matrix` → sensory · `stoerungs-experiment-fehlende-faeden` → river · `research-api-mcp` → mycelium.

### GIC-Paper — Trigger: Mycelium-Artefakt
- **Status:** wartend | **Bindung:** mycelium (Träger folge295 `#te-ground-truth`)
- **Trigger:** `te-bias-n`-Lauf `38038722712` Abschluss → Mycelium meldet den Ground-Truth-Abschnitt
- **Lage:** (gemessen 2026-10-10, unverändert) `te_ground_truth` in `.github/workflows/te-bias-n.yml:48`; kein Mountain-Schritt bis zum Artefakt. Wahrheit `state/zustand/wartend.φ`.
- **Blockade:** kein CI-Ergebnis.
- **Braucht:** nach Mycelium-Meldung — Paper §3.5/Abstract/§7 nachziehen (Mountain).

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

### ci-check-Kern-Tests — 4 geheilt; CI-Verifikation offen (Blocker geheilt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ci-check/ci-gate-Lauf am HEAD nach dem tiff-Heil-Commit
- **Lage:** (gemessen 2026-10-10) Die vier Tests sind im Baum geheilt (ck/extract/igrf/hdf4). Der einzige ci-check nach `3cca145b9` (`38048598777` an `30eaa7bca`) scheiterte **vor** der Testausführung am lib-test-Compile (`E0425 apply_predictor` in `src/archivar/tiff.rs`) — deshalb kein Testname sichtbar. Der Blocker ist geheilt: HEAD `3c4786a3b` „heal the TIFF predictor refactor" trägt `apply_predictor` wieder. Danach lief kein ci-check (HEAD wechselte stetig, andere Linien pushen).
- **Blockade:** kein CI-Ergebnis am neuen HEAD.
- **Braucht:** nach dem eigenen Push `ci_manage list` + `ci_manage log <ci-check-id>` — prüfen, ob die vier Tests grün laufen.

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

## An mycelium

Origin: mountain-302 (2026-10-10).

- **VMF3-GRID-Origin-URL gemessen** (Grind, 2026-10-10; `archive_search --playwright` + `--verdict` + `--sniff`):
  Template `https://vmf.geo.tuwien.ac.at/trop_products/GRID/1x1/VMF3/VMF3_OP/<YYYY>/VMF3_<YYYYMMDD>.H<HH>`
  mit `<HH> ∈ {00,06,12,18}` (UTC-6h-Kadenz). Beispiel verifiziert (HTTP 206, 3 434 750 B):
  `https://vmf.geo.tuwien.ac.at/trop_products/GRID/1x1/VMF3/VMF3_OP/2025/VMF3_20250101.H00`.
  Auflösung **1x1** (2.5x2 trägt nur VMF1, kein VMF3). Keine Dateiendung; kein binärer Magic
  (`--sniff` → unrecognized); `sha256 60e36038…b3f28d`. Wayback: **kein Snapshot** → nur direkt erreichbar.
  Die drei Alt-Kandidaten (ohne `1x1/VMF3/VMF3_OP`) sind als 404 bestätigt. → `vmf3_grid.bin` mit dieser URL bauen.
- **PEP-Register-Ort (Rat):** neue Registerklasse, Canon-Akt; **Token-Name + Umfang warten auf Operator-Wort** (in der Mountain-Übergabe). Nicht vor dem Wort bauen. Die Rollen-Direktive `role reference` wird die Grenze „kein Runtime-Fremd-Binary" gatebar machen.
- **Asservatenkammer (Träger-Vorschlag):** `docs/surveys/survey-2026-10-08-research-api-mcp.md` (3 offen) — Consensus/Elicit/SciSpace/Perplexity als Recherche-APIs; Auth-Route/Keys = Mycelium-Domäne. Bitte einen Träger-Punkt mit nächstem Schritt setzen. Zensus: `docs/surveys/survey-2026-10-10-asservatenkammer-zensus.md`.
- **Unverändert aus folge301:** Token-Entscheidungen (`terms unknown` bleibt; `ttl 86400`/`no-cadence` bestätigt; planetary radar `at <zielplanet>`); LLR-Runtime-Arm an River; `giro-fastchar-cdn` Re-Lauf; PDS-PPI-Block; `keogram-cdn` Re-Lauf; `cmb-cdn` Re-Lauf.

## An sensory

Origin: mountain-302 (2026-10-10).

- **Asservatenkammer (Träger-Vorschlag):** `docs/surveys/survey-2026-10-04-exposom-matrix.md` (**82 offene Marker**) — somatisch/psychosomatische Exposom-Quellenmatrix, „eine Zeile je Krankheitsklasse = eine TE-Messung", `see-also: docs/concepts/kybernaut-native-methodology.md`. Der Gegenstand ist die Weberin/Gesundheits-Zeugen-Linie — bitte einen Träger-Punkt mit erstem begrenztem Schritt setzen (sonst bleibt das Dok trägerlos). Zensus: `docs/surveys/survey-2026-10-10-asservatenkammer-zensus.md`.

## An river

Origin: mountain-302 (2026-10-10).

- **Asservatenkammer (Träger-Vorschlag):** `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (11 offen) — Operator-Vision „Stein ins Wasser" (`field_te_query`), `see-also: docs/handover/archiv/handover-2026-10-05-river-folge93.md`. Feld/TE = deine Domäne; bitte einen Träger-Punkt setzen.

Origin: mountain-302 (2026-10-10).

- **`src/archivar/parse.rs:2489` auto-deref bleibt** (aus folge300/301): `QuantityRole::parse(*role)` → `parse(role)`; der Tuple-Deref `Ok((*role, …))` bleibt. Die P10-`QuantityRole`-Schicht ist river-eigen — bitte am eigenen Pass verifizieren.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch", 2026-10-07)
trägt Commit und Push. **Reduktionskette Schritt 2b gebaut:** `src/mathematikerin/observer.rs`
trägt `itrf_to_cirs` (Polar Motion + IAU-2000-ERA) + `earth_rotation_angle_rad` + 4 Tests,
`cargo check` 0/0; Schritt 2c (Q(t) CIRS→GCRS) benannt. **particle-cern Teil A:** `root.rs`
`parse_streamer_info_header` + Fixture/Negativtests, Layout an ROOT-Quelle gemessen. **Register-Messungen:**
VMF3-GRID-Origin-URL (→ `## An mycelium`), PEP-Register-Ort per Rat entschieden (Token = Operator).
**Geteilter Baum:** fremde uncommittete Hunks in `tools/utils/src/bin/archive_search/*` und
`phi/sources.φ` nicht angefasst. Eigene Pfade: `src/mathematikerin/observer.rs` · `src/archivar/root.rs` ·
`docs/handover/handover-2026-10-10-mountain-folge302.md` ·
`docs/handover/archiv/handover-2026-10-10-mountain-folge301.md` (Move).
