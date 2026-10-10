<!--
  title: Handover — Mountain-Folge 301 (2026-10-10)
  session: Mountain-Linie in einem Pass — Riss Ort der Reduktionskette (--all + Rat + Roster) + ci-check-Heilung
  class: handover
  date: 2026-10-10
  sha256: 5b46e05ea13c678ebb732fb36e244cff081b1262b53915ae372418ad87f8862a
  status: live
-->
# Handover — Mountain-Folge 301 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-10-mountain-folge300.md` (→ `archiv/`).
flash only, kein pro/max.

## Burn: open 0.0000 · close 0.09 · cap 0.50 — Grund: line 0.0809 + Rat 0.0044 + 1× general (Riss-Recherche `--all`) + 1× general (ci-check-Analyse) + 1× general (HDF4-Format, Balance-Wand) + UI-Roster (mountain-ui 8 Seats + open-weight-ui 1 Seat); deepseek-flash

## Offen (aufgeschlüsselt)

### Beobachtungsoperator + Fit — Ort entschieden (Mathematikerin); Kette Schritt 2
- **Status:** eigen (Bau läuft) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt (Station/EOP-Wiring an den Kern)
- **Lage:** (gemessen 2026-10-10) Die Riss-Frage „Ort der Reduktionskette" ist entschieden — **Mathematikerin** (`src/mathematikerin/observer.rs` bleibt, kein Move): `archive_search --all` (Literatur: „measurement model / observation operator / forward operator" liegt konsistent auf der Modellseite; Rohmaterial `state/mountain/2026-10-10_riss-ort_recherche.txt`, 4445 Z.), Rat (5 Stimmen, `state/stimmen/2026-10-10_mountain_riss-ort-round.md`) und **8 UI-Seats** (Duck/Haiku · Qwen · Mistral · DeepSeek-Chat · MiniMax M3 · Claude Sonnet 5.5 · DeepSeek V4 Pro · — Z.ai GLM-5.3 Deep Think `pending`, Lumo `blocked`) konvergieren auf **B**. Die Kette ist kein Ort, sie hat eine **Naht**: Parse/IO → Archivar (`odf.rs`/`celestrak_eop.rs`/`vmf3.rs`/`ionex.rs`), Rechnung/Query → Mathematikerin; das **Residuum** ist ein abgeleiteter ω()-Term, kein Sample-Slot. Kern `two_way_doppler` (Lichtzeit-Iteration + Solar-Shapiro, 7 Tests, `cargo check` 0/0).
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Schritt 2 — die Reduktionskette an den Kern binden, in Reihenfolge: (a) Kalibration der ODF-Rohobservable (Instrument/Clock-Delays); (b) Station ITRF2020 + EOP → Station im ICRS zu t_tx/t_rx (Displacements, IAU 2006/2000A CIO); (c) Medium VMF3/IONEX je Leg; (d) Residuum-Bin (`pioneer10_odf.bin` vs. Kern + Station) gegen DE440, kein LSQ. **Erster begrenzter Schritt (2b):** eine `mathematikerin/observer`-Funktion, die eine ITRF-Station über `archivar/celestrak_eop.rs`-EOP nach ICRS dreht, + Test; ein begrenzter Dispatch + `cargo check`.

### Ephemeriden-Harvest 1-4 — vier Parser gebaut; Register-Token entschieden, Manifestation = Mycelium
- **Status:** eigen (Parser) | **Bindung:** mycelium (sources.φ + Workflows)
- **Trigger:** Mycelium faltet den `## An mycelium`-Block dieser Übergabe und setzt die End-Token
- **Lage:** (gemessen 2026-10-10) `cargo check` 0/0: (1) LLR-MINI `llr::parse_mini` → 3604 Normalpunkte; (2) ITRF2020 SINEX → 154 Stationen; (3) planetary radar → 1134/75/577; (4) VMF3 site/grid → 1052 Z./263 Stationen, 64800 Zellen. Mycelium hat `llr.bin`/`sinex.bin`/`vmf3_site.bin` + `llr-cdn`/`itrf-sinex-cdn`/`vmf3-cdn` gebaut (Runs `38045261467`/`38045263284`/`38045265020` success, Pass). Mountain-Token entschieden (Lage im `## An mycelium`-Block).
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Mycelium — Token setzen; planetary-radar-Block (`at <planet>` oder `at earth`, Mountain-Entscheidung im Block); `vmf3_grid.bin`-Origin-URL messen (drei Kandidaten-URLs 404, Wayback ohne Snapshot — TU-Wien-Server-Index lesen).

### LLR — Parser CRD + MINI gebaut; Runtime-Arm offen
- **Status:** eigen (Parser) | **Bindung:** mycelium (Block+Workflow) · river (main_flow)
- **Trigger:** Mycelium schreibt den `sources.φ`-Block (POLAC MINI + Zenodo CRD) + `llr-cdn.yml`; River setzt `| "llr"` in `main_flow.rs` `series_rows`
- **Lage:** (gemessen 2026-10-10) `src/archivar/llr.rs` parst CRD v2.01 **und** POLAC-MINI; `llr_compiler` End-to-End (Zenodo-CRD 4296 NP, `f2b3afdc…`; OMCD6985 3604 NP). Reiner Parser (null Physik). `llr.bin`-Block steht (Mycelium).
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Mycelium — `llr-cdn.yml`; River — `| "llr"` in `main_flow.rs`.

### particle-cern — Bau-Schuld: TStreamerInfo/TBranch/TLeaf fehlen
- **Status:** eigen (Disposition/Parser) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt (TStreamerInfo-Record eines Event-TTree decodieren)
- **Lage:** (gemessen 2026-10-10) `src/archivar/root.rs` parst Header/TKey/TDirectory-Keyliste grün; TTree bewusst verweigert (`root.rs:235`, Test `root.rs:308`). Kein ROOT-Sample im Baum. `inflate` liegt vor; die Lücke ist der Object-Streamer. Rat: `blocked parser-def` bleibt.
- **Blockade:** TStreamerInfo-Serialisierungsformat ungedecodiert; Indexdomäne ungated.
- **Braucht:** Schritt 2 — `parse_tree` um TStreamerInfo + fBranches/fLeaves erweitern; dann Gate auf Indexdomäne vor dem Skalar-Reader. (Kein Python-Orakel.)

### PEP — Lizenz CC BY-NC-SA gemessen; Register-Ort offen
- **Status:** eigen (Pinning erlaubt) | **Bindung:** rat (Register-Ort)
- **Trigger:** Rat-Wort für den Register-Ort eines Code-Zeugen
- **Lage:** (gemessen 2026-10-10) PEP = Planetary Ephemeris Program (Fortran), `github.com/jbattat/pep_core`; Lizenz im Autorenpaper `2021AJ....162...78C` (CC BY-NC-SA) — Pinning erlaubt. Kein `tool`-Register in `phi/canon.φ`; `phi/witnesses.φ` trägt nur Daten-Zeugen. Rust-Ephemeriden-Landschaft gemessen — keine als Abhängigkeit.
- **Blockade:** Register-Ort für einen Code-Zeugen unbestimmt (keine `tool`-Klasse im Canon).
- **Braucht:** Rat — Register-Ort festlegen (neue `tool`-Klasse oder `phi/*.φ`, Canon-Akt) + Gate-Fixture „kein Runtime-Fremd-Binary im Shipped-Binary"; dann `pep_core` als Golden-Fixture pinnen.

### GIC-Paper — Trigger: Mycelium-Artefakt
- **Status:** wartend | **Bindung:** mycelium (Träger folge295 `#te-ground-truth`)
- **Trigger:** `te-bias-n`-Lauf `38038722712` Abschluss → Mycelium meldet den Ground-Truth-Abschnitt
- **Lage:** (gemessen 2026-10-10) `te_ground_truth` in `.github/workflows/te-bias-n.yml:48`; kein Mountain-Schritt bis zum Artefakt. Wahrheit `state/zustand/wartend.φ`.
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
- **Lage:** (gemessen 2026-10-10) iEEG = privates Experiment (`state/zustand/wartend.φ:40`), kein CDN; `eeglab::eeg_from_bin` akzeptiert `Samples::Double`.
- **Blockade:** keine.
- **Braucht:** kein Schritt — nur ein neues Operator-Wort öffnet es.

### ci-check-Kern-Tests — 4 geheilt, CI-Verifikation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** ci-check/ci-gate-Lauf am neuen HEAD (`ci_manage view <run>`)
- **Lage:** (gemessen 2026-10-10) die vier roten Tests geheilt: `ck::sclk_tick_to_et_and_back` (Fixture `TSC` trägt jetzt `SCLK01_MODULI_28` — `cfdddbbdc` entfernte den fabrizierten Default-Modul); `extract::edf_arm_skips_samples_whose_physical_value_is_absent` (Fixture `d_max: 0.0` = degenerierte Digital-Range → `physical_value` None); `igrf::north_geomagnetic_pole_matches_igrf14_table3` (Code-Bug: `geodetic_to_geocentric_term` war `1 − B²/A² = e²`, korrekt ist `B²/A²`); `hdf4::coded_header_routes_nbit` + `sp_comp…` (Code-Bug: `decode_coded` las `nt` als i16@4 mit Lücke@10; HDF-Group-Quelle `hcomp.c:363-373` belegt `nt` i32@4, `sign_ext` u16@8, `fill_one` u16@10, `start_bit` i32@12, `bit_len` i32@16). Test-Cfg lokal nicht kompiliert (`cargo check` prüft sie nicht) → CI ist die Messung.
- **Blockade:** kein CI-Ergebnis.
- **Braucht:** nach Push `ci_manage view <run>`; bei rot `ci_manage log <id>` je Shard.

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

## An mycelium

Origin: mountain-301 (2026-10-10).

- **Ephemeriden-Harvest 1-4 — die Register-Token (Mountain-Entscheidung, dein Block steht):**
  - **ITRF2020 `terms`:** `terms unknown` **bleibt** — `unknown` ist das Vokabel-Token für
    „nicht bestimmbar/nicht zugesichert" (SPDX NOASSERTION, `license_census.rs:28`). Dein Vorschlag
    `attribution Z. Altamimi et al.` ist **kein** Token der geschlossenen Vokabel → verworfen. Der
    letzte Term ist `https://itrf.ign.fr/en/solutions/ITRF2020`.
  - **VMF3 `ttl 86400`:** bestätigt (Tagesprodukt `VMF3_OP/daily`). **LLR `no-cadence`:** bestätigt
    (eingefrorenes Archiv 2006–2020, Zenodo-Record + POLAC).
  - **planetary radar `at`:** Entscheidung **`at <zielplanet>`** — die Radar-Observable (Range/Doppler)
    ist auf den beobachteten Körper bezogen; je Asset `at venus`/`at mercur`/`at mars` (Vokabel trägt
    diese Körper bereits, `parse.rs:298` `at <body>` → `Frame::Barycenter`). Wenn du den Frame anders
    liest, ist das der Riss — dann melde ihn, statt zu raten.
  - **VMF3-GRID (`vmf3_grid.bin`):** Origin-URL **ungemessen** — drei Kandidaten (`VMF3_OP/daily/2026/VMF3_20260101.H00`,
    `VMF3_OP/grid/2026/…`, `trop_products/GRID/2026/…`) messen 404 (direct+Proton), Wayback 200 ohne
    Snapshot. Schritt: den TU-Wien-Daten-Index lesen (`archive_search --playwright https://vmf.geo.tuwien.ac.at/`).
- **LLR-Runtime-Arm** (an River): `| "llr"` in `main_flow.rs` `series_rows`.
- **Unverändert aus folge300:** `giro-fastchar-cdn` Re-Lauf; PDS-PPI-Block; `keogram-cdn` Re-Lauf bestätigt; `cmb-cdn` Re-Lauf.

## An river

Origin: mountain-301 (2026-10-10).

- **`src/archivar/parse.rs:2489` auto-deref bleibt** (aus folge300): `QuantityRole::parse(*role)` → `parse(role)`; der Tuple-Deref `Ok((*role, …))` bleibt. Die P10-`QuantityRole`-Schicht ist river-eigen — bitte am eigenen Pass verifizieren.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch", 2026-10-07)
trägt Commit und Push. **Riss entschieden:** der Reduktionskern bleibt in der Mathematikerin
(`src/mathematikerin/observer.rs`), die Kette hat die Archivar/Mathematikerin-Naht. **ci-gate-Blocker
geheilt:** `src/mathematikerin/observer.rs:186` `two_way_doppler(…, &orbit)` (E0382 borrow-of-moved
im `#[cfg(test)]` — `cargo check` kompiliert die Test-Cfg nicht; CI-Gate `38045885531`). **Vier
ci-check-Kern-Tests geheilt:** `ck.rs` (MODULI-Fixture) · `extract.rs` (degenerierte Digital-Range) ·
`igrf.rs` (`geodetic_to_geocentric_term` = B²/A²) · `hdf4.rs` (`decode_coded` NBIT-Header nach
HDF-Group `hcomp.c:363-373`). **`path_reference_scan`-MISS geheilt:** `survey-2026-10-10-ephemeris-quellen.md:7`
see-also → `archiv/`. Eigene Pfade dieses Atoms: `src/mathematikerin/observer.rs` · `src/archivar/ck.rs` ·
`src/archivar/extract.rs` · `src/archivar/igrf.rs` · `src/archivar/hdf4.rs` ·
`docs/surveys/survey-2026-10-10-ephemeris-quellen.md` · `docs/handover/handover-2026-10-10-mountain-folge301.md` ·
`docs/handover/archiv/handover-2026-10-10-mountain-folge300.md` (Move).
