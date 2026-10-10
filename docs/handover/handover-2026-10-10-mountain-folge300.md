<!--
  title: Handover — Mountain-Folge 300 (2026-10-10)
  session: Mountain-Linie in einem Pass — Ephemeriden-Harvest 1-4 (Parser)
  class: handover
  date: 2026-10-10
  sha256: 951ebb2689303f21911f81e9e2535389977de24d3a8dca3ff18f9b6e3f2b73e9
  status: live
-->
# Handover — Mountain-Folge 300 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-10-mountain-folge299.md` (→ `archiv/`).
flash only, kein pro/max.

## Burn: open 0.0000 · close 0.0712 · cap 0.50 — Grund: line + 7 Sub-Agenten (4× grind-flash Parser-Bau ITRF/LLR-MINI/radar/VMF3, 2× general Konnektivitäts-Recon, 1× explore particle-cern-Struktur, 1× general PEP-Lizenz); kein pro/max; deepseek-flash (gemessen `session_burn`, line-Session „Mountain-Linie in einem Pass abarbeiten")

## Offen (aufgeschlüsselt)

### Ephemeriden-Harvest 1-4 — vier Parser gebaut; Manifestation = Mycelium
- **Status:** eigen (Parser) | **Bindung:** mycelium (sources.φ + Workflows)
- **Trigger:** Mycelium schreibt die vier `sources.φ`-Blöcke + die CDN-Workflows
- **Lage:** (gemessen 2026-10-10) Operator-Wort „direkt 1-4" abgearbeitet; `cargo check` 0/0:
  (1) **LLR-MINI** — `llr::parse_mini` in `src/archivar/llr.rs`; `/tmp/opencode/OMCD6985.DAT` →
  **3604 Normalpunkte**, 0 marked (nachdem `station_known_mini` die ILRS-5-digit-Codes
  71110/71111/71112/01910/56610/07941/70610 aufnahm), bin 302744 B, sha256 `3a81ba7a…`.
  (2) **ITRF2020 SINEX** — `src/archivar/itrf_sinex.rs` + `itrf_sinex_compiler.rs`;
  `ITRF2020-IVS-TRF.SSC` → **154 Stationen** (XYZ+VEL, bin sha256 `8dd67252…`).
  (3) **planetary radar** — `src/archivar/planetary_radar.rs` + Compiler; venus/mercur/mars.rad.txt →
  **1134 / 75 / 577** Zeilen (bin sha256 `8f1e29a5…` / `6ea6df38…` / `ef59333a…`).
  (4) **VMF3** — `src/archivar/vmf3.rs` + Compiler (site + grid); `2026002.vmf3_r` → 1052 Zeilen/263 Stationen;
  `VMF3_20260101.H00` → 64800 Zellen. Kein Bin im CDN (Compiler bauen ihn im CI `--ci-mode`).
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Mycelium — die vier `sources.φ`-Blöcke + Workflows (siehe `## An mycelium`).

### Beobachtungsoperator + Fit — Produzent-Pfad (Rat + Roster)
- **Status:** eigen (Bau-Vorbereitung) | **Bindung:** eigen
- **Trigger:** Operator-Wort für den ersten begrenzten Bau (Eine-Zwei-Wege-Doppler-Bahn-Kette)
- **Lage:** (gemessen 2026-10-10) Recherche `archive_search --all` (`state/mountain/2026-10-10_observer-operator_recherche.txt`, 405 Z.; Moyer 2000/DSN-ODP, Thornton 2000, DE440/441, INPOP06/08, MESSENGER/Verma 2013) + Rat (5 Stimmen) + 7 UI-Seats (Qwen, Z.ai/GLM-5.3, Claude Sonnet 5.5, DeepSeek, Mistral/Vibe, MiniMax M3, DeepSeek V4 Pro; Duck.ai leer / Gemini nicht gesendet / Lumo Limit / Kimi Kontingent = `pending`; Inkling/GLM 5.3 im shared Seat nicht gelesen) konvergieren: **Operator/Kalibration zuerst, Fit zuletzt**; Kette Zeit (UTC→TT→TDB) → Station (ITRF2020 + Displacements) → Rahmen (EOP-basiert ITRF→GCRS→ICRS, CIO) → Lichtzeit (Zwei-Wege-Iteration + Shapiro) → Medien je Leg (VMF3, IONEX) → Observable; Residuum gegen eine eingefrorene Zeugen-Ephemeride (DE440) messen, **kein LSQ**, bis das Residuum publiziertes Niveau reproduziert. Stimmen: `state/stimmen/2026-10-10_mountain_observer-operator-round.md`. Risse (nicht geglättet): Ort der Kette (Archivar vs. Mathematikerin); Referenz-Kernel Anker (Verma) vs. Zeuge (River/Sensory); erster Körper (Doppler vs. Viking-Range); Kalibration als Schritt 0 (DeepSeek) vs. Operator als Schritt 0.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Operator-Wort für den Start des ersten begrenzten Baus — **eine Zwei-Wege-Doppler-Bahn** (`odf.rs`+`doppler.rs`) durch die native Kette → ICRS → Residuum gegen DE440, kein LSQ; die Architektur-Achse „Ort der Kette" entscheidet der Rat mit dem Operator.

### LLR — Parser CRD + MINI gebaut; Manifestation + Runtime-Arm offen
- **Status:** eigen (Parser) | **Bindung:** mycelium (Block+Workflow) · river (main_flow)
- **Trigger:** Mycelium schreibt den `sources.φ`-Block (POLAC MINI + Zenodo CRD) + `llr-cdn.yml`; River setzt `| "llr"` in `main_flow.rs` `series_rows`
- **Lage:** (gemessen 2026-10-10) `src/archivar/llr.rs` parst CRD v2.01 **und** POLAC-MINI (fixed-width 90, cols gemessen); `llr_compiler` End-to-End auf Zenodo-CRD (4296 NP, `f2b3afdc…`) und auf OMCD6985 MINI (3604 NP); reiner Parser (null Physik). Kein `llr`-Registereintrag.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Mycelium — `sources.φ`-Block + `llr-cdn.yml`; River — `| "llr"` in `main_flow.rs`. (Details `## An mycelium`.)

### particle-cern — Bau-Schuld: TStreamerInfo/TBranch/TLeaf fehlen
- **Status:** eigen (Disposition/Parser) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt (TStreamerInfo-Record eines Event-TTree decodieren)
- **Lage:** (gemessen 2026-10-10) `src/archivar/root.rs` (322 Z.) parst Header/TKey/TDirectory-Keyliste grün; TTree wird bewusst verweigert (`root.rs:235` „TTree scalar decode pending: TStreamerInfo streamers (fBranches/fLeaves), TBasket free-segment" — auch als Test `root.rs:308` gepinnt). Kein ROOT-Sample im Baum (`glob **/*.root` leer); einziger Konsument `tools/harvest/src/bin/cern_root_compiler.rs --probe`. `AliVSD_Masterclass_6.root` = 34 Keys, kein Top-Level-TTree. `inflate` liegt vor; die Lücke ist der Object-Streamer, nicht zlib. Rat 2026-10-10: `blocked parser-def` bleibt (Bau-Schuld), Indexdomäne ungemessen.
- **Blockade:** TStreamerInfo-Serialisierungsformat ungedecodiert; Indexdomäne (time|ordinal|event) ungated.
- **Braucht:** Schritt 2 — `parse_tree` um TStreamerInfo + fBranches/fLeaves erweitern; dann Gate auf Indexdomäne vor dem Skalar-Reader. Verdikt erst nach Leaf-Messung. (Kein Python-Orakel: uproot5 entfällt, Python ist strukturell verweigert.)

### PEP — Lizenz gemessen: CC BY-NC-SA (Autorenpaper); Register-Ort offen
- **Status:** eigen (Pinning erlaubt) | **Bindung:** rat (Register-Ort)
- **Trigger:** Rat-Wort für den Register-Ort eines Code-Zeugen
- **Lage:** (gemessen 2026-10-10) PEP = **Planetary Ephemeris Program** (Fortran; nicht „Python Evaluation Package"), `github.com/jbattat/pep_core`, ASCL 2306.027. GitHub-API `"license": null` und kein LICENSE-File — die Lizenz steht **im Autorenpaper** `2021AJ....162...78C` (DOI 10.3847/1538-3881/ac00ac): „The PEP source code … are now all publicly available via GitHub, and are distributed under a **Creative Commons Attribution-NonCommercial-ShareAlike license**" (Battat 2021b) — also **CC BY-NC-SA**, dieselbe Lizenz wie omegaflow außerhalb `src/`. Pinning/Mirroring in Baum/CDN ist damit **erlaubt** (Attribution, nicht-kommerziell, ShareAlike); nicht all-rights-reserved. Kein `tool`-Register in `phi/canon.φ`; `phi/witnesses.φ` trägt nur Daten-Zeugen (`record`/`force`), keinen Code-Zeugen. Rat 2026-10-10: PEP bleibt offline gepinntes Referenz-/Zeugen-Artefakt, kein Runtime-Oracle, kein Vollport; Ziel „Shipped Binary = 100% Rust". Nebenbefund (`--crates`/`--librs`, crates.io-API 2026-10-10): keine PEP-Rust-Umsetzung. Die Rust-Ephemeriden-Landschaft gemessen — **keine als Abhängigkeit des Shipped Binary** (bleibt `wgpu`/`pollster`/`serialport`; eine fremde Ephemeriden-Bibliothek ist fremde Intelligenz, und den SPK-Reader haben wir selbst: `bsp_reader`/`ephemeris`/`pck`/`lsk`): `empyrean` 0.10.0 (BSD-3, Asteroiden-/Kometen-OD+Propagation, AD) → höchstens offline Zeuge (OD-Gegenprobe, PEP-Muster), kein Vorrat; `rust-jpl` 0.0.1-alpha (MIT, DE441-Reader) und `astrodyn_ephemeris` 0.2.0 (MIT/Apache, DE4xx-SPICE-Reader) → redundant zum eigenen `bsp_reader`; `swisseph-rs` 0.2.1 (**AGPL-3.0**, Rust-Port der Swiss Ephemeris) → descope (viral, unvereinbar mit PolyForm NC/CC BY-NC-SA); `pleiades-events` 0.9.0 (MIT/Apache, Astrologie-Event-Finding) → descope; `adam_core_rs_kernel_data` 0.5.8 (MIT, Kernel-Daten-Resolver für adam_core, Python-Wheel) → kein Physik-Beitrag.
- **Blockade:** Register-Ort für einen Code-Zeugen unbestimmt (keine `tool`-Klasse im Canon).
- **Braucht:** Rat — Register-Ort festlegen (neue `tool`-Klasse oder `phi/*.φ`, Canon-Akt) + Gate-Fixture „kein Runtime-Fremd-Binary im Shipped-Binary"; dann `pep_core` als Golden-Fixture pinnen (CC BY-NC-SA-Zeile + Paper-Zitat als Lizenzquelle).

### GIC-Paper — Trigger: Mycelium-Artefakt
- **Status:** wartend | **Bindung:** mycelium (Träger folge295 `#te-ground-truth`)
- **Trigger:** `te-bias-n`-Lauf `38038722712` Abschluss → Mycelium meldet den Ground-Truth-Abschnitt
- **Lage:** (gemessen 2026-10-10) `te_ground_truth` ist in `.github/workflows/te-bias-n.yml:48` aufgenommen (mycelium-294); kein Mountain-Schritt bis zum Artefakt.
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
- **Lage:** (gemessen 2026-10-10) iEEG = privates Experiment (`state/zustand/wartend.φ:40`), kein CDN; die Mountain-Seite (`eeglab::eeg_from_bin` akzeptiert `Samples::Double`) ist gebaut.
- **Blockade:** keine.
- **Braucht:** kein Schritt — nur ein neues Operator-Wort öffnet es.

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
„warum schreibst du hier erst in die blocked sources anstatt direkt an den finalen ort — das ist einfach nur faules compliance theater" | 2026-10-10 | Operator (Session, Mountain 295)
„Offen bleibt in P10: der optionale `<regime>`-Token …, die Regime-Achse selbst, und die Zeilen-Migration in phi/sources.φ (Mountain)." | 2026-10-10 | Operator (Session, Mountain 295)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater … kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–290)
„mach das ab jetzt automatisch — committe und pushe selbst" | 2026-10-07 | Operator (Session, Mountain 264)

## An mycelium

Origin: mountain-300 (2026-10-10).

- **Ephemeriden-Harvest 1-4 — vier `sources.φ`-Blöcke + Workflows.** Alle vier Parser sind gebaut und `cargo check` 0/0. Bitte je einen Block + einen `*-cdn.yml`-Caller (unter `cdn-manifest.yml`) schreiben:
  - **LLR** (POLAC MINI + Zenodo CRD): `netloc zenodo.org` · url `https://github.com/omegaflow/sources/releases/download/zenodo.org/llr.bin` · `format llr` · `origin https://web.archive.org/web/…/http://polac.obspm.fr/observations/OMCD6985.DAT` (bzw. Zenodo 7818557 CRD) · `compiler tools/harvest/src/bin/llr_compiler.rs` · `terms unknown`/CC-BY-4.0 Zenodo · `at moon`.
  - **ITRF2020 SINEX**: `netloc itrf.ign.fr` · url `…/itrf.ign.fr/sinex.bin` · `format sinex` · `origin https://itrf.ign.fr/ftp/pub/itrf/itrf2020/ITRF2020-IVS-TRF.SSC` · `compiler tools/harvest/src/bin/itrf_sinex_compiler.rs` · `terms attribution Z. Altamimi et al. (ITRF2020, IGN)` · `at earth` · `ttl 31536000` (frozen release).
  - **planetary radar**: `netloc iaaras.ru` · url `…/iaaras.ru/planetary_radar.bin` · `format planetary_radar` · `origin https://web.archive.org/web/20190218230236id_/http://iaaras.ru:80/media/observations/venus.rad.txt` (mercur/mars analog) · `compiler tools/harvest/src/bin/planetary_radar_compiler.rs` · `terms free-open https://iaaras.ru/en/usage/` · `ttl 86400`. **Riss:** iaaras.ru live unerreichbar (Wayback); die JPL-planets-Seite hat tote Goldstone-Links; die lebende JPL-Radar-DB ist der Small-Body-JSON-API `https://ssd-api.jpl.nasa.gov/sb_radar.api`. Der Block braucht ein `at` (Frame) — Register-Entscheidung, nicht fabriziert.
  - **VMF3**: `netloc vmf.geo.tuwien.ac.at` · url `…/vmf.geo.tuwien.ac.at/vmf3.bin` · `format vmf3` · `origin https://vmf.geo.tuwien.ac.at/trop_products/VLBI/VMF3/VMF3_OP/daily/2026/2026002.vmf3_r` (+ GRID analog) · `compiler tools/harvest/src/bin/vmf3_compiler.rs` · `terms attribution TU Wien VMF Data Server` · `at earth`.
- **LLR-Runtime-Arm** (an River): `| "llr"` in `main_flow.rs` `series_rows`.
- **Unverändert aus folge299:** `giro-fastchar-cdn` Re-Lauf; PDS-PPI-Block (manifest + `quantity`-Zeilen); `keogram-cdn` Re-Lauf bestätigt; `cmb-cdn` Re-Lauf (Timeout ist auf 360 gesetzt, `cmb-cdn.yml:17`).

## An river

Origin: mountain-300 (2026-10-10).

- **`src/archivar/parse.rs:2489` auto-deref (`QuantityRole::parse(*role)` → `parse(role)`).** Fremder Fix im eigenen Atom: der `ci-gate`-clippy-Lauf zu `df0579550` (`ci_manage log 38042562305`) verlangte `-D warnings`-konform; mycelium-295 hat die Lint-Liste an Mountain geroutet. Der `*`-Deref im Funktionsargument ist entfernt (Auto-Deref `&&str → &str`), der Tuple-Deref `Ok((*role, …))` bleibt (kein Auto-Deref im Tupel). Semantik unverändert; `cargo check` 0/0. Die P10-`QuantityRole`-Schicht ist river-eigen — bitte am eigenen Pass verifizieren.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („committe und pushe selbst", 2026-10-07)
trägt Commit und Push. **`ci-gate`-Regression aus `df0579550` behoben** (`ci_manage log 38042562305`):
`itrf_sinex.rs:467` Testliteral `-.139…` → `-0.139…` **Syntaxfehler im `#[cfg(test)]`-Block** —
nacktes `cargo check` kompiliert die Test-Cfg nicht, `cargo test`/clippy schon (Lehre: nach einem
Parser-Bau, dessen Tests nie lokal liefen, ist die Test-Cfg ungemessen); clippy `-D warnings`:
`llr.rs` (needless_lifetimes:128, collapsible_if:278/287/397/406, manual_unwrap_or_default:347 →
benannter Sentinel `STATION_UNKNOWN` statt `unwrap_or_default`, das das Gate blockt), `vmf3.rs`
(collapsible_if:149/157). Eigene Pfade dieses Atoms: `src/archivar/llr.rs` · `src/archivar/mod.rs` ·
`src/archivar/itrf_sinex.rs` · `src/archivar/planetary_radar.rs` · `src/archivar/vmf3.rs` ·
`tools/harvest/src/bin/llr_compiler.rs` · `tools/harvest/src/bin/itrf_sinex_compiler.rs` ·
`tools/harvest/src/bin/planetary_radar_compiler.rs` · `tools/harvest/src/bin/vmf3_compiler.rs` ·
`docs/handover/handover-2026-10-10-mountain-folge300.md` ·
`docs/handover/archiv/handover-2026-10-10-mountain-folge299.md` (Move).
