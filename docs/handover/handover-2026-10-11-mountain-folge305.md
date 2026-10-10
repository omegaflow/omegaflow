<!--
  title: Handover — Mountain-Folge 305 (2026-10-11)
  session: Mountain-Linie in einem Pass — Nutation IAU 2000A (Schritt 0 komplett), particle-cern Teil B (parse_tree), Vantage-Rest-Bias (b)(c)(d), ci-gate-Clippy-Lints, open-lidar-data-φ-Block, FMHY-7-Verdikte, Asservatenkammer 3/9
  class: handover
  date: 2026-10-11
  sha256: 9253b618f535a846cbd3344a126f14246b5a2bf39cc754283c4f69def3ecd60d
  status: live
-->
# Handover — Mountain-Folge 305 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-10-mountain-folge304.md` (→ `archiv/`).
flash only, kein pro/max; vier begrenzte Dispatches (Nutation, particle-cern,
Vantage, Clippy-Heilung) + zwei Register-Dispatches (Asservatenkammer, FMHY).

## Burn: open 0.0 · close 0.164 · cap 0.2 — Grund: line session with 7 bounded dispatches (grind-flash/general), deepseek-flash, kein pro/max

## Offen (aufgeschlüsselt)

### Beobachtungsoperator + Fit — Q(t) UND N(t) gebaut; Schritt 1 (Pioneer-10-Residuum) offen
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt — Pioneer-10-ODF durch die Kette, Residuum gegen DE440 (kein Fit)
- **Lage:** (gemessen 2026-10-11, `cargo check` 0/0) `src/mathematikerin/receiver.rs` trägt jetzt die volle IAU-2000A-Nutation (678 luni-solare + 687 planetare Terme), `nutation_angles_2000a`, `nutation_angles_06a` (P03-Korrektur), `nutation_matrix_n06a` (Rx(-(ε_a+Δε))·Rz(-Δψ)·Rx(ε_a)); in `cirs_to_gcrs` als `R = P·N` eingesetzt. Tests gegen SOFA/ERFA-Referenz (J2000, JD 2453736.5, 2020-01-01). `src/mathematikerin/receiver.rs`.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Schritt 1 — Pioneer-10-ODF durch die Kette, Residuum gegen DE440; die vier neuen Nutation-Tests im CI bestätigen (Toleranz <1e-13 rad aus verifizierter Transkription, kein lokaler Clippy-Beweis).

### particle-cern — Teil B (Branches) gebaut; TStreamerElement-Liste + Baskets offen
- **Status:** eigen (Parser) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt (TStreamerElement-Klassen-Ref-Map + Baskets)
- **Lage:** (gemessen 2026-10-11, `cargo check` 0/0) `src/archivar/root.rs` +500: ROOT-Block-Dekompression (`inflate_zlib` RFC 1950/1951 + `decompress_object` 9-Byte-Frame) + `parse_tree` → `TreeIndex{branches: name/leaf_type/offset}` mit Indexdomänen-Gate; gegen `MasterclassData.root` (Record 401, sha256 `8694a2ed…039b`) verifiziert — 152 Branches, erste `D0_MINIP` letzte `PVNTRACKS`. `cern_root_compiler.rs` unangetastet.
- **Blockade:** TStreamerElement-Elementliste nutzt ROOTs Klassen-Ref-Map (nach dem ersten Objekt kein `kNewClassTag`); verworfen statt unverifiziert ausgeliefert. Baskets (`fBasketBytes`/`fBasketEntry`) offen.
- **Braucht:** TStreamerElement-Decode + Basket-Reader; dann Gate auf Indexdomäne vor dem Skalar-Reader.

### Vantage-Rest-Bias — (b)(c)(d) geheilt
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** — (geheilt)
- **Lage:** (gemessen 2026-10-11, `cargo check` 0/0, `--tests` 0/0) (b) `src/archivar/odp.rs` `const EARTH` entfernt → `dsn_host() -> Option` liest `kernels/dsn_host.txt`; `station_velocity`/`downlink_rate` tragen den Host als Parameter (12 Call-Sites gezogen). (c) `src/weberin.rs` `frame_origin_name()` → registriertes Feld `frame_origin` in `Weberin`/`WeberinFeed` (`kernels/frame_origin.txt`). (d) `woven_major_bodies()` → Register-Datei `kernels/woven_major_bodies.txt` statt Liste. **Riss (gemessen):** `kernels/naif_body_ids.tsv:10` = `10 sun` — der alte Default war die Sonne, nicht die Erde.
- **Blockade:** keine.
- **Braucht:** nichts; die NAIF-10-Riss-Notiz ist verdikt-relevant (Frame-Origin-Deklaration).

### FMHY-Quellen-Verdikt — 7 research-data-Verdikte geschrieben; Rest der Klasse offen
- **Status:** eigen (Quellen-Verdikt) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Verdikt-Schritt je Teilklasse
- **Lage:** (gemessen 2026-10-11, `archive_search --verdict`) `losc.ligo.org` → `blocked parser-def` gap `gwf-strain` (`phi/blocked_sources.φ`); `imos.aodn.org.au` + NOAA SURFRAD sind die Fünf-Klima-Quellen (Compiler stehen, `phi/sources.φ`-Registrierung pending, siehe Mycelium-φ-Blöcke) — keine Block-Register-Zeile. 4 `decline` Portale (`phi/declined_sources.φ`: emsc.eu registry/katalog, ngmdb.usgs.gov registry/katalog, gadm.org infrastructure, worldclim.org registry/katalog). **Korrektur:** `worldclim.org` ist das Projekt-Portal; die Raster liegen am Datenhost `geodata.ucdavis.edu` (`worldclim_compiler.rs` steht, pending-Registrierung) — Datenklasse (interpolierte Klimatologie) offen benannt.
- **Blockade:** die drei `pending` sind echte Messwerte ohne Arm/Compiler (LOSC GW-Strain HDF5/GWF; IMOS THREDDS NetCDF ~30 TB; SURFRAD Text/CSV em).
- **Braucht:** restliche research-data-Klasse (74, 72 NEW) + Erweiterungs-Klasse nach `docs/SOURCE_PORT.md`; je `pending` der Compiler/Arm.

### Asservatenkammer — 3 von 9 Doks gemessen (erster Schritt)
- **Status:** eigen (Register/Träger) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt je weiterem Dok
- **Lage:** (gemessen 2026-10-11) `survey-2026-09-14-kapitulationen-pendings-inventur.md` 31/32 gedeckt (Rest NOAA ERI imagery `phi/pipeline/catalog/noaa_nodd_disposition.φ:36`); `survey-2026-10-08-open-sources-delta.md` 11/13 (Rest SSDC Limadou `ledger.φ:15`, LiteBIRD); `survey-2026-09-03-orphan-verdicts.md` 12/15 (Rest 3 `github.com-*` netlocs). Header-sha per `omega_sh sha` nachgerechnet.
- **Blockade:** ohne ersten Schritt trägerlos.
- **Braucht:** die restlichen 6 Doks (`survey-2026-10-09-redistribution-alternativen.md` 10 · `survey-2026-10-09-domaenen.md` 8 · `survey-2026-09-16-fremde-parser-sammlungen.md` 6 · `survey-2026-10-08-fmhy-research-landscape.md` 5 · `survey-2026-10-07-fmhy-forschungsschicht.md` 2 · `survey-2026-09-17-omegaflow-legacy-konzepte.md` 2).

### Mycelium-φ-Blöcke — open-lidar-data-Block steht; fünf Klima-Quellen offen
- **Status:** eigen (Register/Verdikt) | **Bindung:** eigen (CI-Lauf: mycelium)
- **Trigger:** erster CI-Lauf je Klima-Workflow (sha256), dann `ecad`-Riss entscheiden
- **Lage:** (gemessen 2026-10-11) **open-lidar-data-Block steht** in `phi/sources.φ` (terms `Gratis-Open-Data-Licentie-Vlaanderen-1.2`, `at earth`, `ttl 604800`, `field las_*`, sha256 `e6560344…`, compiler `open_lidar_data_compiler.rs`). **VNP46A3-Block steht** (`:1996`). Fünf Klima-Quellen (SURFRAD/ECAD/DWD/WorldClim/AODN): Compiler stehen (`tools/harvest/src/bin/{surfrad,ecad,dwd_cdc,worldclim,aodn}_compiler.rs`), Verdikt-Zeilen fehlen (kein sha256 ohne ersten Lauf). `ecad`-Riss (`ecad.eu` vs. Quellhost `knmi-ecad-assets-prd.s3.amazonaws.com`) offen.
- **Blockade:** offene `terms`/`at`/`ttl`/`field` je Block; `register_sort` verlangt die `ttl`-Zeile.
- **Braucht:** je Block `url`+`origin`+`compiler`+`format`+`sha256` (Mycelium-Lauf) + Mountain-Verdikt `terms`/`at`/`ttl`/`field`.

### FMHY-Routing — Arm-Hälfte gegenstandslos; Riss 4 bot-gated
- **Status:** eigen (Architektur) | **Bindung:** eigen
- **Trigger:** Riss 4/5-Abschluss
- **Lage:** (gemessen 2026-10-10, unverändert) Grenze ist die Manifestations-Achse: Query → `archive_search`-Arm; Messwert → `phi/sources.φ` + Compiler; Operator-Werkzeug → `tools/`; Blick/Portal → Lead. Riss 4 (Overpass — HTTP 406 direkt+Proton → bot-gated `pending`). **KNMI gebunden via EDR (live, 77 Stationen, `knmi_compiler.rs`); Mindat Token erkannt, Konto Level 0 → wartend** (`state/zustand/wartend.φ`).
- **Blockade:** Riss 4 bot-gated.
- **Braucht:** Riss 4-Abschluss (bot-gated) oder Descope.

### PEP + Tudat — Schritt 0 komplett (Präzession + Nutation); Schritt 1 offen
- **Status:** eigen (Register + Bau) | **Bindung:** eigen
- **Trigger:** Schritt 1 (Pioneer-10-Residuum gegen DE440)
- **Lage:** (gemessen 2026-10-11) Nutation gebaut (siehe oben). Träger-Doks: `docs/concepts/eigene-ephemeride.md`, `docs/surveys/survey-2026-10-10-ephemeris-quellen.md`. Vier Häuser (DE440/INPOP19a/EPM2021/PETREL19) als `ephemeris_*` registriert; PEP/Tudat nur Lizenz-/Verfahrens-`reference`.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Schritt 1 — Pioneer-10-ODF durch die Kette, Residuum gegen DE440, kein Fit.

### ci-check-Kern-Tests / ci-gate-Rot — mci/parcorr/receiver geheilt; `pc.rs` offen (fremde aktive Session)
- **Status:** wartend | **Bindung:** fremde Linie (pc.rs)
- **Trigger:** `pc.rs`-commit der Fremd-Session → ci-gate-Lauf am HEAD
- **Lage:** (gemessen 2026-10-11, `ci_manage log 38089096103`) 13 Clippy-Lints unter `RUSTFLAGS=-D warnings`. Geheilt und committet: `mci.rs` (type_complexity ×2, neg_cmp, needless_range_loop), `parcorr.rs` (excessive_precision, neg_cmp ×2, needless_range_loop ×3), `receiver.rs` (too_many_arguments, bereits folge304). **Offen:** `pc.rs:22/49/66` — die Heilung liegt im Arbeitsbaum, ist aber mit einer **aktiven fremden** uncommitteten `pc_stable_skeleton_screened`-Restrukturierung verschmolzen (`+50/-14`, wachsend), darum nicht committet.
- **Blockade:** eine parallele Linie schreibt `src/mathematikerin/pc.rs` gerade uncommittet.
- **Braucht:** die Fremd-Session committet `pc.rs` (mit der Lint-Heilung); danach `ci_manage list` + `ci_manage log <ci-gate-id>` — den grünen Lauf messen.

### Flyby-Kette — Residual in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-09, unverändert) 157 ODF-Referenzen; `doppler.rs` absent; Wahrheit `state/zustand/wartend.φ:34`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### iEEG — registriertes Wort 2026-10-06 maßgeblich
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort, das den Riss über 2026-10-06 hebt
- **Lage:** (gemessen 2026-10-11, unverändert) iEEG = privates Experiment (`state/zustand/wartend.φ:40`), kein CDN.
- **Blockade:** keine.
- **Braucht:** kein Schritt — nur ein neues Operator-Wort öffnet es.

### GIC-Paper — Trigger: Mycelium-Artefakt
- **Status:** wartend | **Bindung:** mycelium (Träger folge295 `#te-ground-truth`)
- **Trigger:** `te-bias-n`-Lauf `38038722712` Abschluss → Mycelium meldet den Ground-Truth-Abschnitt
- **Lage:** (gemessen 2026-10-11, unverändert) `te_ground_truth` in `.github/workflows/te-bias-n.yml:48`.
- **Blockade:** kein CI-Ergebnis.
- **Braucht:** nach Mycelium-Meldung — Paper §3.5/Abstract/§7 nachziehen.

### Exposom-Matrix → TE-Paar-Feed — Repräsentativpunkt ist Annahme-Akt
- **Status:** wartend | **Bindung:** eigen (Mountain-Feder)
- **Trigger:** extern gedeckter Repräsentativpunkt je Zeile gesetzt (Operator-Hand; Operator-Wort 2026-10-10)
- **Lage:** (gemessen 2026-10-10) Kein offener Datensatz trägt eine im Datensatz gemessene Koordinate; x-Kern-Serien (OpenAQ/Open-Meteo/NASA POWER) stehen wie gemessen.
- **Blockade:** der Repräsentativpunkt ist eine wissenschaftliche Annahme, keine Messung.
- **Braucht:** je Zeile den extern gedeckten Repräsentativpunkt — dann `phi/sources.φ`-Zeile (Mountain-Feder), dann Te-Paar-CI-Feed (Mycelium).

### blocked_sources — Klassenträger + zwei aufgelöste Risse
- **Status:** eigen (Bau/Register) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt
- **Lage:** (gemessen 2026-10-11) `phi/blocked_sources.φ` trägt nur noch **1** offene `blocked parser-def`-Klasse: `phi/blocked_sources.φ::gap:particle-cern ×1`. Zwei als „fehlender Arm" geführte Einträge waren **falsch** (der Baum gewinnt): (a) `pds-ppi-manifest` — der Arm steht (`tools/harvest/src/bin/pds_ppi_compiler.rs`, `phi/harvest.φ:522` format `pds_ppi`, Workflow `pds-ppi-cdn.yml`; EPN-TAP `vo-pds-ppi.igpp.ucla.edu/tap/sync` 200, 219 Tabellen/141 084 Granulen); (b) `gwf-strain` — `src/archivar/hdf5.rs` + `tools/harvest/src/bin/gwosc_compiler.rs` lesen die HDF5-Strain-Datei bereits.
- **Braucht:** (1) particle-cern — TBranchElement-v9-Branch-Decode in `src/archivar/root.rs` (`walk_trees` steht, 68 innere TTrees); (2) PDS-PPI — Manifest-CI-Lauf `pds-ppi-cdn.yml` + Register-Zeile (Mycelium); (3) GWOSC/LOSC — Registrierung in `phi/sources.φ` + Kraft-Admission + Vorzeichen-Riss (Strain ist signiert, Leser-Predikat `value >= 0.0` verwirft negative Werte); **Riss** zwei Compiler für eine Quelle (`gwosc_compiler.rs` GWOS 3-Feld getrackt · `losc_compiler.rs` LOSC 26×f64 neu) — Operator/Rat entscheidet.

### GWOSC-Strain — Operator-Wort 2026-10-11: A (eigene Klasse, quantity); Source-Port offen
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-11) **Operator-Wort: A** — eigene Klasse (Krümmung/Gezeiten) als `quantity`-Feld (Kind `relative`, dimensionslos, ohne Kraft: `CHANNEL_REF_QUANTITY` 255 + `PRESENCE_FLAG_QUANTITY` 16.0; `units.rs:519` erlaubt `relative|1|dn|count|%`). Recherche + Rat + Roster tragen das (6/6 „signiert trägt" + „26-Kontrakt bleibt"; 5/6 „eigene Klasse"). „Vorzeichen"-Riss war ein **Doku-Riss** — Leser `spatial.rs:766` (`val.abs()` NaN-Gate) trägt den Sign; `archivar-mathematikerin.md:31` korrigiert. `losc_compiler.rs` emittiert die Klasse jetzt über `--class <force|quantity-kind>` (Quantity-Kind → force 255 + presence 16.0), `SLOT_TTL` gesetzt (vorher ttl=0 → jeder Record verworfen; force=em fabriziert); `gwosc_compiler.rs` (GWOS 3-Feld, kein Leser) descoped.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** den Source-Port unter A: `phi/harvest.φ`-Arm (`format losc`) + Workflow `gwosc-cdn.yml` + CI-Lauf (H-H1-HDF5 → `.bin`) + `phi/sources.φ`-Block mit `quantity losc_strain losc_strain point relative 1 <τ> 0.0 0.0` (sha256 nach dem Lauf).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern; Lauf lokal/silent,
  nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen:
  der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„mach das ab jetzt automatisch — committe und pushe selbst" | 2026-10-07 | Operator (Session, Mountain 264)
„bitte die grossen punkte parallel mit agenten abarbeiten und ABSCHLIESSEN, NICHT VERSCHLEPPEN" | 2026-10-11 | Operator (Session, Mountain 305)
„bypass-mirror 23 (UrhG-§95a- was bedeutet das die möchte ich bitte raus haben keine fragwürdigen links" | 2026-10-10 | Operator (Session, Mountain 303)
„können wir nun eine untersuchung machen was davon als arme in archive search sollte, was in tools und was in phi dateien?" | 2026-10-10 | Operator (Session, Mountain 303)
„aber ganz ehrlich wie problematisch ist das?" · „akzeptiert" (KNMI-Key-Risiko) | 2026-10-10 | Operator (Session, Mountain 303)
„zudem schreibt das die Kante ist — ehrlich: Der nächste Schritt ist kein weiterer Messschritt von mir. Er ist ein wissenschaftlicher Annahme-Akt: für jede Zeile müsste ein extern gedeckter Repräsentativpunkt gesetzt werden (Region→Gitterpunkt / Stadt→Station) — das ist eine Annahme, keine Messung, und gehört nicht in meinen autonomen Bereich (die x-Kern-Serien OpenAQ/Open-Meteo/NASA POWER bleiben wie in der Matrix gemessen). Erst mit diesem Punkt wird die Zeile ja und darf (Mountain-Feder) in phi/sources.φ, dann (Mycelium) in den Te-Paar-CI-Feed. Das steht als Braucht im Träger." | 2026-10-10 | Operator (Session, Mountain 304)
„ich habe keine ahnung deshalb wollte ich dass du wissenschaft und den roster befragst und der scheint ja A zu bevorzugen" (= GWOSC-Strain eigene Klasse / `quantity relative`) | 2026-10-11 | Operator (Session, Mountain 305)

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch", 2026-10-07)
trägt Commit und Push. **Dieses Atom (Mountain 305):** vier begrenzte Bau-Dispatches +
zwei Register-Dispatches, flash only.
- **Nutation N(t) gebaut** (`receiver.rs`, volle IAU 2000A 1365 Terme; `R=P·N`) — Schritt 0 komplett.
- **particle-cern Teil B gebaut** (`src/archivar/root.rs`: zlib-Inflate + `parse_tree`, gegen `MasterclassData.root` verifiziert).
- **Vantage-Rest-Bias (b)(c)(d) geheilt** (odp.rs/weberin.rs + zwei neue Kernel-Träger).
- **ci-gate-Clipy-Lints geheilt** (mci.rs/parcorr.rs/receiver.rs); `pc.rs` bleibt der aktiven Fremd-Session.
- **open-lidar-data-φ-Block geschrieben** (terms/at/ttl/field aus gemessenem License-Link).
- **FMHY**: 7 research-data-Verdikte (3 pending, 4 declined Portale); worldclim.org als Portal korrigiert.
- **Asservatenkammer**: 3 von 9 Doks mit erstem Schritt gemessen (Header-sha via `omega_sh sha`).
- **Register-Putz `phi/blocked_sources.φ`:** 3 stale `pending` gelöst (HI 21cm, CMB LAMBDA/PLA, Blinkverse — `phi/sources.φ`-Blöcke stehen bei `:10930`/`:11436`/`:11547`/`:10876`); der 24-zeilige gap-Token-Kanon entfernt (kein Code liest ihn, kein offener Eintrag referenzierte einen Token); 5 `pending`-Dubletten der Wartezeilen konsolidiert (BepiColombo-MORE, Voyager, Mariner 10, Viking, Juno stehen schon in `state/zustand/wartend.φ:11/:6/:15/:16/:18`). Register jetzt **1 `blocked parser-def`** (particle-cern, TBranchElement-v9-Decode offen); die zwei anderen „Arme" (PDS-PPI-Manifest, GWOSC-Strain) existierten bereits — Register-Eintrag war veraltet, korrigiert.
**Geteilter Baum:** `src/mathematikerin/pc.rs` wird von einer parallelen Linie uncommittet
restrukturiert — nicht angefasst/committet. Ebenso fremd uncommittet: `tools/utils/src/bin/archive_search/osf.rs`.
Eigene committete Pfade: `src/mathematikerin/receiver.rs` · `src/mathematikerin/mci.rs` ·
`src/mathematikerin/parcorr.rs` · `src/archivar/root.rs` · `src/archivar/odp.rs` · `src/weberin.rs` ·
`src/archivar/kernels/frame_origin.txt` · `src/archivar/kernels/woven_major_bodies.txt` ·
`tools/measure/src/bin/{galileo_elevation_match,galileo_floor_elevation_burst,orientation_probe,
pioneer10_paper_chain_retrace,pioneer10_surrogate_subpeak_null,pioneer11_odf_residuum,
pioneer_doppler_moyer,pioneer_doppler_moyer_navio,pioneer_link_correction_probe,
pioneer_navio_residuum,pioneer_odp,pioneer_residue_diagnose,weberin_body_verdict,
weberin_mpc_spk_verdict,weberin_verdicts_compiler}.rs` · `phi/sources.φ` ·
`phi/blocked_sources.φ` · `phi/declined_sources.φ` · die drei Asservatenkammer-Surveys ·
diese Übergabe · der Archiv-Move folge304.
