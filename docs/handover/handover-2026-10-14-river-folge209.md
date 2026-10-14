<!--
title: Handover — River-Folge 209
class: handover
date: 2026-10-14
sha256: c5fb0945b8b4823884662b904c01ea3580814668526f69e7f299fae7212acd58
status: live
-->
# Handover — River-Folge 209 (2026-10-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; pfad-begrenzt committen.

## Burn: open 0.0000 · close 0.068 (gemessen `opencode.db` `session.cost`, River-Session `ses_ec60475d`, 2026-10-14T10:41Z — parallel laufende Linien-Sessions; `session_burn` aggregiert sie, daher nicht isolierbar) · cap 0.50 — Grund: river-210 (ein Pass; kein Fenster-Edit, kein Send, kein UI-Seat) — der aacgm-Koeffizienten-Bruch geheilt (HEAD kompilierte nicht: `src/archivar/aacgm.rs:7` zieht `aacgm_coeffs-14-2025.asc` per `include_str!`, die Datei war von `c33be5a67` gelöscht), die drei adressierten Blöcke gefaltet (mountain-343-1/342-6, mycelium-344-4), der `conserved_for_quantity`-Riss als von Mountain-341 bereits geschlossen am Baum gemessen, der intermagnet-Harvest-Abbruch als 180-min-Job-Timeout gemessen. Fälligkeit bei Pass-Öffnung (HEAD `48387c9f9`): `--fired river` = 1 (die Große-Rätsel-Kandidatenliste, operator-gebunden — nicht erneut vorgelegt), `--stale river --persist 3` = 0, `--addressed river` = 2 (mountain-344, mycelium-346 — beide in diesem Atom gefaltet); `open_points_check` F208 = 0 stale-citations, 2 word-carried (Fehltreffer). (gemessen 2026-10-14, river-210)

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„council bedeutet recherche roster (full) und rat" | 2026-10-13 | Operator (Session, River-206) — jede Ratsfrage läuft als **volle** Kette: Forschungsschicht + Rat (fünf Stimmen) + UI-Frontier-Roster (Linien-Seats + geteilte Open-Weight-Seats), nicht das `council`-Sub-Agent allein.
„ja du bist die frische sitzung" | 2026-10-13 | Operator (Session, River-204) — die zwei großen Bauten (Kopplungs-GPU-Seite + Parität, Zeugen-Registerzeilen) in dieser frischen Session arbeiten, nicht als Taucher-Auftrag an die Vorsession.
„nein ich sage der lock wird erst aufgehoben wenn das frühwarnsystem fertig und kalibriert ist" | 2026-10-13 | Operator (Session, River-204) — LOCK bleibt; die zwei Siegel-Punkte (wy-max-t, Blatt-Kalibrierung) werden **nicht** als erledigt gestrichen; der LOCK fällt erst, wenn das Frühwarnsystem fertig und kalibriert ist.
„das kannst du direkt prüfen" | 2026-10-13 | Operator (Session, River-204) — den ENTSO-E-HTTP-400 direkt prüfen (Token oder Parameter).
„ja bitte aber warum sind es nur 384 aufrufe wir haben doch 2500 blöcke" | 2026-10-13 | Operator (Session, River-208) — **Bau-Wort für den Register→Manifest-Automatismus** (das Kabel bauen; zuerst die Zählung der generischen Teilmenge als erste Messung). Die Frage nach 384 vs. 2565 ist mit der Messung unten beantwortet.
„ja bau es allerdings auch mit den sonstigen gates etc also lizenzprüfung z.b." | 2026-10-13 | Operator (Session, River-208) — der generische Aktor läuft **durch die vorhandenen Tore**: Lizenz-Tor, Fingerabdruck-Tor, Format-Tor, Registrierungs-Tor; kein Tor wird umgangen.

Verbatim: `state/operator-gespraeche/2026-10-13-river.md`.

## Träger (Prosa, eigene)

- `docs/concepts/kanal-ontologie-komplettbau.md` — der Bauplan (P0–P10); P10.2a (Parser-Arm) gebaut.
- `docs/concepts/archivar-mathematikerin.md` — der Wire/GPU-Force-Vertrag; Träger des kinetischen Rahmens, der **Receiver-Apertur** (`:34`) und des **Schneidungs-Gesetzes**.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`status: unsealed`) — Präregistrierung der Vorhersage-Zelle; Siegel = Operator-Wort (LOCK).
- `docs/paper/gic-causal-driver.md` — die Punktschätzer-Unsicherheit auf das studentisierte Moving-Block-Bootstrap (`:171`).
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` — die Zellen stehen `pending` (nach dem Flyby); Rest = 1-σ-Kovarianz.
- `docs/surveys/survey-2026-10-12-standort-lokalisierung-ortho-dsm.md` — Standort-Lokalisierungs-Landkarte + `dsm − dtm`.
- `src/archivar/aacgm.rs` (+ `aacgm_coeffs-14-2010/2015/2020/2025.asc`) — der AACGM-v2-Zeuge (Loader + Brackets + Zwei-Epochen-Interpolation `c14fb3298`/`5f44d0980`; CI-Zeugentest im `ci-gate`-`subset`).
- **Sensory-Abgabe 2026-10-13 (gefaltet aus `## An river`, sensory-folge256): JUICE / Flyby-Path-2** — die Trägerschaft dieser drei Dokumente liegt jetzt hier (Sender-Register → River-Träger), je Eintrag mit Token:
  - `docs/concepts/der-paradigmenwechsel.md` (9 offene Marker) · xfer:sensory.ses_ece7532c3ffe16Z417fBRj8b31.1~4844adaa
  - `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4) · xfer:sensory.ses_ece7532c3ffe16Z417fBRj8b31.2~5243b5f0
  - `docs/paper/flyby-path-2-preregistration.md` (1) · xfer:sensory.ses_ece7532c3ffe16Z417fBRj8b31.3~1f5f2109
  - trägerlose Flyby-Orphans: `docs/auftrag/auftrag-flyby2-kette.md` (3) und `docs/paper/flyby-path-2-addendum-2026-09-29.md` (Future nannte 27 Marker) · xfer:sensory.ses_ece7532c3ffe16Z417fBRj8b31.4~59f0ba87
- **Offene-Prosa-Träger (future-235/240-Abgabe):** `docs/auftrag/auftrag-flyby2-kette.md` (3) · `docs/auftrag/auftrag-universelles-vlies.md` (2) · `docs/blatt/blatt-te-externer-steuerparameter.md` (1) · `docs/blatt/fruehwarnsystem-praeregistrierung.md` (1) · `docs/paper/flyby-path-2-addendum-2026-09-29.md` (27) · `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (1, `descoped` `:122`).
- **Mountain327-Abgabe (Marker abgearbeitet river-199):** `docs/blatt/blatt-gic-breitenband-familien.md` (2) · `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (2). `xfer:mountain327-4` · `xfer:mountain327-5`.

## Adressierte Blöcke — gefaltet 2026-10-14 (river-210)

- `xfer:mountain-343-1` **Athabasca** (mountain-344) — die Registerzeilen stehen (`phi/sources.φ:402-411`): der Arm emittiert per-Bin, **kalibriert** (`ch1@freq_vlf@amplitude_cal_vlf_ch1` → `nT^2/Hz`) und **roh** (`ch1@freq_vlf` → `V^2/Hz`); der Einheiten-Riss ist entschieden. Nicht mehr an Mountain gestellt; im Weberin-Punkt unten nachgezogen.
- `xfer:mountain-342-6` **SABER** (mountain-344) — die Registerzeile folgt nach dem HAPI-Arm (dotted `<i>` + `alt`). Im Weberin-Punkt unten nachgezogen (Mountain hält den Arm).
- `xfer:mycelium-344-4` **DE-2 + `asset=`-Riss** (mycelium-346) — die Riss-Klärung steht (`harvest-dispatch.yml:106` liest `asset=`); `asset=fehlt` bleibt bis zum ersten erfolgreichen Pull. In P7.1 nachgezogen.
- `xfer:mountain-341` **`conserved_for_quantity`** — Mountain-341 hat die DE-2-Zahldichte-Kanäle auf `Conserved::Number` gezogen und `conserved_for_quantity` (`number_density`/`electron_density`/`column_density` → `Number`, `channel.rs:763`) ausgerichtet; der in river-209 gemeldete Riss ist damit am Baum geschlossen. Der Kanal-Identitäts-Punkt ist entsprechend fortgeschrieben.

## Offen (aufgeschlüsselt)

### P7.1 Kopplung/Interface — Register + CPU-Naht + GPU-Injektion gebaut; `K` offen
- **Status:** eigen (teilgebaut) | **Bindung:** eigen
- **Trigger:** die DE-2-Reihe liegt im CDN → dann `K` aus dem gemessenen Overlap; bis dahin trägt der Riss.
- **Lage:** (gemessen 2026-10-14 river-210 am Baum) der **co-lokale Kanal ist im Register deklariert** — `phi/sources.φ:596-616`: url `DE2_UA16S_ALL` (1981-08→1983-02, `terms PD`, keyless), hapi-Mapping Ionendrift (`RPA_IVE/IVN`), Neutralwind (`WATS_NVE/NVU`), N2-Dichte (`NACS_N2`), die vier `channel`-Zeilen (`:611-614`, beide Dichte-Kanäle jetzt `number:…`), `constraint quasi-neutral de2_qn region thermosphere` (`:615`) und die Zeile `coupling ion-neutral-friction from de2_ion_drift to de2_neutral_wind rate pending density_from number:de2_ion_density:2.66e-26 density_to number:de2_neutral_density:4.65e-26 …` (`:616`). Die **GPU-Seite** ist gebaut (`FIELD_WGSL` `apply_couplings`, Binding 10 `coupling_at`; `omega.rs` `coupling_descriptor_rows` + Buffer/Layout); die CPU-Naht `ChannelCouplingLayer::from_sources` steht; Test `coupling_injection_wgsl_matches_the_cpu_exchange` geschrieben — **CI-only** (`#[cfg(test)]` + GPU).
- **Blockade:** (1) die DE-2-Reihe ist **noch nicht im CDN** — Manifestation gebaut (Mycelium `2d696704a`); der Lauf `38489787692` scheitert am Host (`cdaweb.gsfc.nasa.gov` TLS-Abbruch; mycelium-346, host-spezifisch bestätigt); (2) `K` = `rate pending`; (3) die WGSL-Validierung + das Paritäts-Gate laufen erst im CI.
- **Braucht:** (1) die DE-2-Reihe im CDN (Manifestation läuft/am Host blockiert; danach ist `K` messbar); (2) `K` aus dem co-lokalen DE-2-Overlap; (3) den nächsten `ci-gate`-Lauf lesen — WGSL-Parse + `coupling_injection_wgsl_matches_the_cpu_exchange`.

### Kanal-Identität — Verdikt B′ gebaut; Register auf `number:`, `conserved_for_quantity` geschlossen; CI-Netz ausstehend
- **Status:** eigen (Bau fertig; das `ci-gate`-`subset`-Netz ausstehend) | **Bindung:** eigen
- **Trigger:** der nächste `ci-gate`-`subset`-Lauf (nach dem Kompilier-Fix dieses Atoms).
- **Lage:** (gemessen 2026-10-14 river-210 am Baum) Verdikt B′ (Rat + Roster + Forschung, river-209) gebaut: Schritt 1 (`unit` aus `ChannelDescriptor::PartialEq`, Hash = Gleichheit; Test `the_unit_is_representation_not_identity`), Schritt 2 (`register_checked` + `ChannelUnitConflict`; die zwei Produktions-Ingestion-Sites migriert; Test `a_foreign_unit_is_refused_and_never_silently_dropped`), Schritt 3 (`Conserved::Number` Token `number`; Fick in Fluid/Plasma; Tests `the_number_token_parses`, `number_and_mass_density_of_one_species_hash_apart`, `number_fick_is_admissible_in_fluid_and_plasma`) — alle fünf Tests liegen im Lib (`channel.rs`). Die Register-Re-Deklaration steht: Mountain-341 hat `phi/sources.φ:613/:614` (`de2_ion_density`, `de2_neutral_density`) auf `number:` gezogen und den Riss `conserved_for_quantity` geheilt (`channel.rs:763`: `number_density`/`electron_density`/`column_density` → `Number`). Offen nur das dimensionslose Prädikat (die „konvertierbar"-Prüfung ist konservativ der gleiche Unit-Token; eine Maßtabelle ist `pending`).
- **Blockade:** die `ci-gate`-Läufe waren am fehlenden aacgm-Koeffizienten blockiert (s. AACGM-Punkt); jetzt geheilt → der nächste `subset` misst die fünf Tests.
- **Braucht:** den nächsten `ci-gate`-`subset` lesen (die fünf Tests als Netz).

### P10 flache Tokens → Kanal-Deskriptor (`force_id_of`/`FIXED_CHANNELS`/Modul `force` weg) — gated
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains `field`-Zeilen-Migration (`phi/sources.φ`: 7943 flache `field`-Zeilen → Deskriptor-Form, P10.3).
- **Lage:** (gemessen river-199 am Baum) Schritt 1 (Rename, `channel_ref: Option<u8>`) + Schritt 2 (Sentinel 255 → Presence-Bit) gebaut; 57 Dateien; `cargo check` 0/0. Noch `force` im Baum: die Brücke `descriptor_for_force` u. a. (`channel.rs:930-958`), `force.rs`, `FIXED_CHANNELS`, Gate-Re-Exports. Gemessener Riss: Parser-Index vs. Laufzeit-Registry-Index ab dem zweiten Kanal (`f2623bb72`).
- **Blockade:** `phi/sources.φ` trägt noch 7943 flache `field`-Zeilen.
- **Braucht:** nach Mountains Migration den flachen Parser-Branch (`parse.rs:1600`) + die `force_id_of`-Aufrufe streichen und auf `descriptor_from_axes`/`ChannelRegistry::handle_of` ziehen; `cargo check` 0/0; CI-`ci-gate` ist das Netz.

### Prosa-Träger mountain327-Abgabe — AACGM-Koeffizient geheilt; Zeuge läuft im nächsten `subset`
- **Status:** eigen (Carrier-Duty) | **Bindung:** eigen
- **Trigger:** der nächste `ci-gate`-`subset`-Lauf.
- **Lage:** (gemessen 2026-10-14 river-210 via `ci_manage log 38549948637`) die `ci-gate`-Läufe waren **rot am fehlenden Koeffizienten**: `error: couldn't read src/archivar/aacgm_coeffs-14-2025.asc: No such file or directory` bei `aacgm.rs:7:37` → jedes Job (`build`/`clippy`/`dropped-gate`/`subset`) brach am `could not compile omegaflow (lib)`. Der Commit `c33be5a67` (Operator-Commit mit dem Thema AGENTS.md-Agent-Namen) trägt die `D`-Löschung der Datei, während `aacgm.rs:7` sie per `include_str!` zieht; lokal lag sie untracked (`??`). In diesem Atom exakt wiederhergestellt (identisch zum historischen Blob `9ebe282d2`, sha256 `8a8ec5e680a99ddee04de658937661af973745fede8cc4569bf203525ba317cc`, 639 Zeilen; `git add`); `cargo check` 0/0. Der frühere E0308 (`channel_ref: 0` in `field_te_query.rs`) ist in HEAD bereits gefixt.
- **Blockade:** keine.
- **Braucht:** (a) den nächsten `ci-gate`-Lauf lesen (`ci_manage log <id>`) — der AACGM-Zeuge (`#[cfg(test)]` in `src/archivar/aacgm.rs`) läuft im `subset` (`cargo test --lib`); (b) die CTBTO-Einreichung (Operator-Hand; Antrag liegt privat) — s. `## An future`, entblockt nach mycelium-346-8; (c) BGR: erledigt.

### 14-min-Lücke Magnetometer→Optik — im 2D + Ganznacht gemessen, Pick-Riss getragen
- **Status:** eigen (getragener Riss) | **Bindung:** eigen
- **Trigger:** der GIC-Onset-Relationsschritt auf der Meridian-Spur.
- **Lage:** (gemessen 2026-10-12 river-197 + river-200, Anker `state/river/themis/frame-onset-2026-10-12.md` · `overlay-night-2026-10-12.md`) 2D: kein kohärenter off-meridian optischer Eintritt bei 07:41; der optische Onset ist ein einzelnes azimutal wanderndes Band 07:54:45→07:55:45. Ganznacht-Overlay (CDN-TKDG `themis_asi_timed_keogram_20240510_kapu.bin`): Stundenmittel 03–06 ≈ 3310–3343 (ruhig), 07 = 3370.6, Ränder 02 = 3895 / 08 = 3911; der Hauptsturm liegt 17:00–23:00 UT (Tageslicht). **Riss:** der river-196-Pick „07:41" ist nicht eindeutig — Stunde 07 trägt eine Kette (07:13–07:48), größte YKC-Spitze bei 07:20 (52.4); die „+14 min 27 s" ist pick-abhängig.
- **Blockade:** keine.
- **Braucht:** den Riss tragen (Phasendifferenz + Pick-Abhängigkeit).

### Weberin-Zeugen-Fäden — SABER-Form (C); Athabasca-Zeilen stehen; UCAR-FPI Arm steht; Athabasca-Pfad+cdf-Arm
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** je Faden die `channel`-Quellen-Zeile (Mountain) + Arm/Manifestation (Mycelium).
- **Lage:** (gemessen 2026-10-13 river-204/206/208 · 2026-10-14 river-210) Fakten in `state/river/weberin-faden-facts-2026-10-13.md`, Register-Entwürfe `state/river/weberin-register-entwuerfe-2026-10-13.md`. **CDF-Leser** gebaut (`92346fb85`, `cdf25.rs`); drei Zeugen-Zeilen stehen (`d1e2e28cb`). **SABER-Form (C)** (volle Runde, `state/stimmen/2026-10-13-river-saber-hoehenform-roster.md`): der Level-Index (0…399) ist die geschlossene diskrete Achse, die Höhe die zweite gleich indizierte Koordinate (`altitude(time, level)`); das Level muss Teil des Schlüssels sein. **Athabasca (ERG-SC VLF): die Registerzeilen stehen jetzt** (`phi/sources.φ:402-411`, Mountain-343-1) — per-Bin, kalibriert (`nT^2/Hz`) + roh (`V^2/Hz`); der `cdf`-Arm (`extract.rs:4888` → `cdf25::extract_witness_channels`) trägt sie. **UCAR-FPI:** der `format ucar_fpi`-Arm steht (`400de8ce2`, `src/archivar/fpi_ucar.rs`; Sample `~/Downloads/20246300athabasca.dat`, 10 leerzeichengetrennte Spalten, zwei Zeiten je Zeile). **SABER:** die Registerzeile folgt nach dem HAPI-Arm (Mountain-342-6; `ktemp` + `tpgpaltitude`, beide `size:[400]`). **WALDO:** `state/zustand/wartend.φ` Key `waldo-massdaten`.
- **Blockade:** (1) UCAR-FPI `field`-Komponenten-Zuordnung (`series_component_name` → `fpi_mer_wind_ms`/`fpi_zon_wind_ms`) + Register-Zeile (Mountain); (2) SABER-Zeile nach Form (C) (Mountain, nach dem HAPI-Arm); (3) je eingefügtem Block die Mycelium-Manifestation.
- **Braucht:** (1) UCAR-FPI: Sample + Arm stehen — die zwei `field`-Zeilen + Manifestation; (2) SABER-Zeile nach Form (C) + Manifestation; (3) Athabasca: Registerzeilen stehen — nur noch die Manifestation (Mycelium); (4) SGO/FMI (`rioData.php`, kein Static-Arm) — Browser-POST; (5) WALDO-Antwort (Trigger).

### Netz-End-Validator — GIC↔Onset-Bin; MCON manifestiert, Register-Drift gelöst
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** `bpa_gic` deckt den MCON-Zeitraum (1999–2024) UND die `format mcon_event`-Registerzeile steht (Mountain).
- **Lage:** (gemessen 2026-10-12 river-200 via `ci_manage log 38176003927` + mycelium-318) **MCON manifestiert** (`mcpherron_onsets.bin`, 314 555 Onsets, sha `441df5f8…`). `bpa_gic` führt nur ein 4-Tage/5-min-Fenster → 0 Onsets im MCON-Fenster; die Relation läuft über die FMI-Reihe.
- **Blockade:** `bpa_gic` deckt den MCON-Zeitraum nicht (kein Archiv; nur FOIA).
- **Braucht:** (a) `bpa_gic`-Harvest auf 1999–2024 ausdehnen (Mycelium) — oder den MCON-Zeitraum verwerfen und die Relation auf die FMI-Reihe stützen; (b) die Surrogat-Null-Form vor jeder Behauptung durch den Rat.

### Receiver-Apertur (SPAN) — Schritt 3 offen
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** der nächste `ci-gate`-Lauf (Paritäts-Gate `field_spatial_fold_wgsl_cpu_parity`); danach Schritt 3.
- **Lage:** (gemessen 2026-10-12 river-199/200 via `cargo check` 0/0 + `git log` `5a3e629e7`) Schritt 2 (Faltung) gebaut und committet: `σ_eff² = extent² + global_scale²` in `field_spatial`/`field_spatial_grad` (WGSL) + CPU-Spiegel; `max(extent, scale)` gestrichen; Kontrakt `:34` nachgezogen.
- **Blockade:** das Paritäts-Gate läuft erst im CI.
- **Braucht:** (a) den nächsten `ci-gate`-Lauf lesen; (b) **Schritt 3:** die deklarierte `ReceiverAperture` (`receiver.rs`) an die Query-Naht (`vp.surface.w = global_scale`) binden; (c) die Operator-Browser-Messung (Fenster-Edit = Operator-Wort).

### GIC-Onset — AL-Einhüllende über die 154-Station-Familie; Harvest am 180-min-Job-Timeout
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** Abschluss der Harvest-Läufe `intermagnet-xyz-cdn` → die Assets `<code>_xyz_1m.bin` liegen im CDN.
- **Lage:** (gemessen 2026-10-14 river-210 via `ci_manage view/jobs/log 38485365619` + `gh api` releases) der korrekt dispatchte Lauf `38485365619` (river-209, 10 Stationen `lnp lov mbc mid mlt mzl qsb reu tik wmq`, `start=2000 end=2024`) ist **`cancelled`**: die 10 Stations-Jobs starteten 2026-10-13T23:37Z und wurden 2026-10-14T02:38Z beendet = **180 min** = `timeout-minutes: 180` des Workflows (`Terminate orphan process: pid (2389) (intermagnet_xyz_compiler)`). Ursache: das 25-Jahre-Fenster (1-min) überschreitet das Job-Timeout. Der Asset-Stand bleibt **144/154** `<code>_xyz_1m.bin`, **10 fehlen**: `lnp, lov, mbc, mid, mlt, mzl, qsb, reu, tik, wmq` (`lov` erwartet rot, HTTP 400). Die Kette `intermagnet_xyz_compiler` → `al_network_proxy` → `gic_onset_detector` ist gebaut; die 154-Station-Partition liegt in `state/river/gic-family-*.txt`; Null-Baseline gemessen (`--null 200`: 1.62×/1.47×/1.35×).
- **Blockade:** das Job-Timeout am 25-Jahre-Fenster; der Harvest-Dispatch ist Mycelium (s. `## An mycelium`).
- **Braucht:** das Fenster aufteilen (z. B. 2000–2012 / 2013–2024 — Mycelium); dann je Familie `al_network_proxy --family <fam.txt> --dir <pfad> --decl auto --baseline-block 60 --min-stations 2` und `gic_onset_detector --null 200`.

### GIC-Familien-Partition — zwei Register-Nägel, dann die Matrix-Läufe
- **Status:** blockiert | **Bindung:** eigen (Blockade: Mountain-Nägel)
- **Trigger:** die zwei Register-Nägel sind gesetzt (Mountain), dann Run der Matrizen.
- **Lage:** (gemessen `state/stimmen/2026-10-06_gic-familien-arm-ui-stimmen.md:20-80`) die Stimmen-Runde einig: kein neuer Grammatik-Arm, sondern (i) ein Satz über Querfamilien-Zellen und (ii) die Familienregel als deklariertes `station→Familie`-Artefakt. Minimaler Patch: `matrix gic full` + `family auroral|subauroral|midlat stations <code…>` + `expect families 3` + `fdr by 0.05 over matrix` + `wymaxt 0.05 over family`.
- **Blockade:** die zwei Nägel (Mountain-Register).
- **Braucht:** Mountain setzt die zwei Nägel; danach den Patch bauen und `matrix gic_auroral|gic_subauroral|gic_midlat full` fahren.

### Newell-OMNI-Alignment — Zelle gemessen (n=22137), TE silent
- **Status:** eigen (gemessen, silent) | **Bindung:** eigen
- **Trigger:** kein — die Verifikations-Zelle ist gemessen; der nächste Schritt (das Verhältnis zum Live-Treiber) ist benannt.
- **Lage:** (gemessen 2026-10-13 river-209 via `gh run download 38464793874 -n field-te-matrix-newell-omni`) der Lauf `38464793874` (HEAD `fd1d693e8`) ist **success**; `matrix-newell-omni`: `expect cells 1 matched the declared 1 cells`, Zelle `newell_dphi_dt_omni->abk_dbdt_pt1h` **n = 22137** (n_eff 1.2298e3, native rung 3600 s), TE 7.7953e-5, p 1.0000 → **verdict silent**; `fdr bh q 0.05`: **0 of 1 cells pass**; kein n = 0 mehr. Die frühere Disjunktheit (`38378918382` n = 0) ist mit dem gezogenen 1h-Schlüssel (`abk_dbdt_pt1h`, mountain-340 `05e215aa1`) **gelöst** — die Zelle ist co-temporal. Riss-Hinweis (Mountain-340): die Kanal-Identität bleibt `intermagnet_dbdt_abk` (aus `fc.name`) — sollen 1h/1m je ein eigener Kanal sein, braucht es eine `name`-Änderung + extract-Arm-Anpassung.
- **Blockade:** keine.
- **Braucht:** die silent-Verifikations-Zelle ins Verhältnis zum Live-Treiber setzen (`matrix-newell`, im selben Lauf success) — die Rat-/Null-Disziplin vor jeder Behauptung; kein neuer Lauf ohne neuen Grund.

### Flyby-Kette — recon bleibt (`ephemeris-juice-recon` + `flyby-path2-recon`)
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** `ephemeris-juice-recon` Wiedervorlage 2026-11-01 (ESA/ESOC publiziert die Post-Flyby-SPK). Wahrheit: `state/zustand/wartend.φ`.
- **Lage:** (gemessen 2026-10-11) die Wiedervorlage steht; `data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin` absent. `flyby-path2-recon`: orbc-SPK keyless (HTTP 206); die 1-σ-Kovarianz liegt nicht im SPICE-Baum.
- **Blockade:** ESA/ESOC-recon-SPK + die 1-σ-Kovarianz absent.
- **Braucht:** am 2026-11-01 `archive_search --verdict` auf den ESOC-recon-Pfad; dann `flyby_ephemeris_gate --recon <recon.bin> --sigma-recon <km>`.

### Schumann-Resonanz — Antriebsfrage (Nadel ⅩⅣ, vorab festgehalten)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** die Reihe `resonance_schumann.csv` (f1/f2/f3) liegt im CDN → `state/zustand/wartend.φ` `schumann-serie`.
- **Lage:** (gemessen 2026-10-13 river-208) als **Nadel ⅩⅣ** in den Rätsel-Register `docs/concepts/kybernetische-astrophysik.md` („Die Nadeln im Block", `:368`) eingetragen (Operator-Wort „echtes Nadel-Rätsel"); Survey-Refs `survey-raetsel-bestand.md:25-26` nachgezogen. Quelle registriert (CC-BY-4.0, Zenodo 15225820), CDN-Abzug 404, Manifestation bei Mycelium. Physik: Antrieb = weltweites Blitzgeschehen; Weltraumwetter wirkt als **Modulation** über die Ionosphäre. Instrument: TE-Matrix (`te_compute`) + Surrogat-Null. Echte offene Fragen: Dämpfung/`Q`, Anregungsverteilung, Schumann als Ionosphären-Sonde, andere Welten. Gemessen nicht getragen (mystisch): „Frequenz steigt", Erdbeben-Vorläufer. **Geheilt (Operator-Wort):** die Tafel hinkte — **Nadel ⅩⅢ** (Atmosphären-Biosignatur, seit 2026-09-05) war nicht eingetragen, jetzt in Tafel + Bestand nachgetragen (Ⅰ–ⅩⅣ), Verweise `:353-368`/`:398-400` nachgezogen.
- **Blockade:** die Reihe fehlt im CDN.
- **Braucht:** (1) die Reihe im CDN (Mycelium, `state/zustand/wartend.φ` `schumann-serie`); (2) die Frage **vorab** festhalten — „Regelt geomagnetische Aktivität die Schumann-Frequenzen sichtbar, und in welcher Richtung?" gegen die Blitz-/Intensitäts-Erklärung; (3) dann die Matrix über ein gemeinsames Fenster mit `--null` — ein Ergebnis trägt nur, wenn es die Null schlägt.

### Große ungelöste Rätsel — Kandidatenliste (Rat + UI-Roster); Aufnahme-Wort offen
- **Status:** operator-gebunden | **Bindung:** eigen
- **Trigger:** Wort: der Operator entscheidet, welche Kandidaten als neue Nadeln (ⅩⅤ+) aufgenommen werden.
- **Lage:** (gemessen 2026-10-13 river-208) `archive_search --all` **durchgelaufen** (109/114 Keyword-Modi; 5 Parameter-Arme gehalten) → volle Datei `state/research/spill/omegaflow_all_1791909383_greatest_unsolved_problems_astrophysics_.txt` (866 Zeilen, zu Ende gelesen); die offene Web-Suche liefert die **enzyklopädischen Listen** (Wikipedia „List of unsolved problems in astronomy", Frontiers „Grand Challenge for Space Physics", NPG Space-Weather-Review, Borovsky „Is Our Understanding of Solar-Wind/Magnetosphere Coupling Satisfactory?", „Advancing Heliophysics…Open Science" Corti et al., „Unsolved Problems in Planetary Nebulae") — **keine** projektschneiderten Rätsel. **Volle Roster-Runde** (Rat/5 Stimmen + Linien-Seats): **Claude · Duck.ai · Gemini 3.1 Pro · Sakana · Mistral · MiniMax M3 · Perplexity** geantwortet; **Qwen · Z.ai · DeepSeek · Kimi · Lumo `pending`** (Reasoning läuft bzw. kein Composer im gelandeten Zustand). Konvergenz: **Hubble-Spannung · UHECR-/Multimessenger-Ursprung · Geodynamo/Erdkern-Jerks** (in fast allen); dazu Sonnen-Dynamo/Magnetzyklus, Substurm-/CME-/SEP-Raumwetter, Strahlungsgürtel/Van-Allen, TGF, PTA-nHz-GW, Kosmische Dämmerung/21 cm, WHIM/fehlende Baryonen, S8, Exoplaneten-Radius-Tal, Little Red Dots. **Verweigert:** Bewusstseinsfeld, Telepathie, Torsionsfelder, „Schumann-Frequenz steigt", t=0/Planck-Schaum; MiniMax spart zusätzlich Wurmloch/Placebo/DM-Labor/Dunkler-Fluss aus (kein Detektor-Falsifikat). Material `state/stimmen/2026-10-13-river-grosse-raetsel-roster.md`. **Träger-Survey:** `docs/surveys/survey-2026-10-13-grosse-raetsel-und-maschine.md` (Kandidaten · Werkzeuge · Weberin · Maßstab · Planet-9-Widerlegung) und `docs/surveys/survey-2026-10-13-raetsel-master-matrix.md` (Impact · Aufwand · Erfüllung · Machbarkeit · Überschneidungs-Netz). **Riss des Rats (Mountain):** der Bestand trägt schon unbearbeitete Nadeln — „kein Zuwachs durch Verschieben"; Aufnahme nur mit Kreuzung + Signal + Surrogat-Null.
- **Blockade:** keine.
- **Braucht:** Operator-Wort, welche Kandidaten aufgenommen werden (je Kreuzung + Signal + Null); dann Tafel `:353-368` + Bestand nachziehen.

## LOCK

**Operator-Wort 2026-10-13:** der LOCK fällt erst, wenn das **Frühwarnsystem fertig und kalibriert** ist; die zwei Siegel-Punkte werden **nicht** als erledigt gestrichen.

### wy-max-t — α als Family-Bound gemessen; Blatt-Siegel offen
- **Status:** LOCK | **Bindung:** LOCK (Operator-Wort)
- **Trigger:** Operator-Siegel auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`.
- **Lage:** (gemessen 2026-10-12 via `gh run download 38166787414` + `wy_max_t_probe --combine`) der Westfall–Young-max-T-Null ist gemessen, α = 0.05, pooled B = 9999: (1-α)-Quantil ABK-2024 = 2.4467 · ABK-2025 = 2.4433 · SOD-2024 = 2.4831 (Selftest 2.5856). family-clearing: `Bz→dB/dt` (3/3), `dB/dt→Speed` (3/3), `dB/dt→Density` (3/3), `dB/dt→Bz` (ABK-2024, SOD-2024), `Speed→dB/dt` (ABK-2025). α steht im Blatt (`:78-85`).
- **Blockade:** das Blatt-Siegel ist Operator-Wort; der LOCK fällt erst mit fertigem + kalibriertem Frühwarnsystem.
- **Braucht:** das fertige + kalibrierte Frühwarnsystem; dann das Operator-Siegel; die gemessenen Werte in den Bestand (Mycelium).

### Blatt-Kalibrierung — Z-Null ABK gelesen (Z = 58 min); Signifikanz-Null offen
- **Status:** LOCK | **Bindung:** LOCK (Operator-Wort)
- **Trigger:** Operator-Siegel auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`; der Signifikanz-Null (Blocklänge) ist bestimmbar.
- **Lage:** (gemessen 2026-10-12) der BCA-Z-Null über die ABK-Reihe: `Z = 58 min`.
- **Blockade:** der Signifikanz-Null braucht eine Blocklänge (Autokorrelations-Nulldurchgang fehlt); SOD-Asset fehlt; der LOCK fällt erst mit fertigem + kalibriertem Frühwarnsystem.
- **Braucht:** (a) Blocklängen-Regel für dieses Korn benennen (Rat) oder den Deskriptiv-Wert tragen; (b) SOD-`dB/dt`-Asset (Mycelium); (c) α/Bz-Schwelle + Siegel = Operator-Wort (nach Future geroutet).

## An mycelium

Origin: river-210

**intermagnet-Harvest — 180-min-Job-Timeout.** Gemessen (`ci_manage view/jobs/log 38485365619`): der Lauf `38485365619` (10 Stationen, `start=2000 end=2024`) ist `cancelled`; die Jobs starteten 2026-10-13T23:37Z und wurden 2026-10-14T02:38Z beendet = **180 min** = `timeout-minutes: 180` des Workflows, mitten in `intermagnet_xyz_compiler` (`Terminate orphan process: pid (2389) (intermagnet_xyz_compiler)`). Das 25-Jahre-1-min-Fenster überschreitet das Job-Timeout. **Braucht:** das Fenster je Station splitten (z. B. 2000–2012 / 2013–2024) und je Lauf dispatchen (Ernte-Dispatch = Mycelium) — dann liegen die 10 fehlenden Assets (`lnp mbc mid mlt mzl qsb reu tik wmq`) und `al_network_proxy`/`gic_onset_detector` (river) laufen.

Origin: river-204

**ENTSO-E — die zwei gemessenen Defekte.** (1) Der `entsoe_compiler` sendet `documentType=A78` mit `in_Domain` ohne `out_Domain` → HTTP 400 `uu-appg01/server/invalidRequest` (der Token ist akzeptiert; ein ungültiger Token gäbe 401). (2) Es gibt keine lebende Registerzeile für ENTSO-E (`sgrep -i entsoe` = 0; Compiler + Workflow unregistriert). xfer:river204-entsoe1.

**CTBTO — Formular vorbereitet (Operator-Akt = Absenden).** Mycelium meldet das Formular vorbereitet; das Absenden ist Operators Hand.

**Weberin-Zeugen — Manifestation der neuen Zeugen.** Die `url`/`origin`/`compiler`-Direktiven der Weberin-Zeugen gehören Mycelium; sie sind gated: SABER keyless via HAPI, UCAR-FPI E-Mail-gated (Operator, erledigt), Athabasca ERG-SC keyless. **Athabasca:** die `channel`/`field`-Zeilen stehen (`phi/sources.φ:402-411`) — die Manifestation ziehen. **SABER/UCAR-FPI:** sobald die `field`-Zeile steht (`## An mountain`), die Manifestation ziehen. xfer:river208-weberinmanifest1.

**Schumann-Resonanz (Zenodo) — Eintrag ohne Abzug.** Gemessen (river-208, `archive_search --verdict`/`--sniff`): der Block `phi/sources.φ:1034-1043` trägt `url …/zenodo.org/resonance_schumann.csv` + `origin https://zenodo.org/api/records/15225820/files/NiNo_SR_params_2024.csv/content` (CC-BY-4.0), aber (a) **kein `sha256`**, und (b) der Abzug im CDN ist **404**. Die Origin-Datei ist erreichbar (HTTP 200, 454 417 B), sha256 `3183a14fafba0762f91522dfdddf356fa2609b86797d6512aefc2104bf7b0272`. **Braucht:** (1) die Zeile `sha256 3183a14fafba0762f91522dfdddf356fa2609b86797d6512aefc2104bf7b0272` in den Block; (2) eine Matrix-Zeile in `.github/workflows/zenodo-raw-cdn.yml` (`asset: resonance_schumann.csv`, `url`/`origin` wie oben, derselbe sha256); (3) `gh workflow run zenodo-raw-cdn.yml`. xfer:river208-schumann1.

**Register→Manifest — Bau-Wort (mit Toren).** Operator-Wort „ja bitte" und „ja bau es allerdings auch mit den sonstigen gates etc also lizenzprüfung z.b." (2026-10-13). Der Verbinder **existiert** (`harvest-dispatch.yml` feuert auf `push` auf `phi/sources.φ`); der Defekt ist die **Doppelbedeutung von `format`**. **Erste Messung:** 2565 Blöcke; 636 `format reference`; 1239 mit eigenem Compiler; generisches Format (csv/json/cdf/hapi) 23, davon **19 ohne Compiler**. **Kleinster Fix:** (1) eine ausdrückliche Manifest-Zeile im Block (`harvest raw_zenodo` / `manifest auto`); (2) **ein** generischer Aktor (`# harvest: format=csv arm=raw_mirror …`). **Tore (Operator-Wort):** Lizenz-Tor, Fingerabdruck-Tor, Format-Tor, Registrierungs-Tor. xfer:river208-registermanifest1.

## An mountain

Origin: river-210

**Kanal-Identität B′ — Register-Re-Deklaration + `conserved_for_quantity` erledigt.** `Conserved::Number` (Token `number`) steht in `src/mathematikerin/channel.rs`; Mountain-341 hat `phi/sources.φ:613/:614` auf `number:…` gezogen und `conserved_for_quantity` (`channel.rs:763`) ausgerichtet. Kein offener Riss mehr an dieser Stelle. xfer:river.ses_ec901ad7dffehLKKusa1DO806h.3~1b6c1cbf.

**UCAR-FPI — `field`-Komponenten-Zuordnung.** Der `format ucar_fpi`-Arm steht (`400de8ce2`, `src/archivar/fpi_ucar.rs` + Dispatch `extract.rs:159`); das gemessene Sample (10 leerzeichengetrennte Spalten, Kopf `yyday ut(hr) merid(m/s) merid err dir ut(hr) zonal(m/s) zonal err dir skytemp`) wird geparst. **Braucht:** die zwei `field`-Zeilen `fpi_mer_wind_ms` (merid-Spalte) und `fpi_zon_wind_ms` (zonal-Spalte) samt `series_component_name`-Zuordnung. xfer:river208-ucarfpi1.

**SABER-Zeugen-Zeile nach Form (C) — Höhe als Koordinate auf geschlossener Level-Achse.** Verdikt (C) (Rat + volle Roster-Runde, `state/stimmen/2026-10-13-river-saber-hoehenform-roster.md`): der Level-Index (0…399) ist die **geschlossene diskrete Achse**, die Höhe eine zweite, gleich indizierte Koordinate (`altitude(time, level)`); das Level muss Teil des Schlüssels sein — kein freies `field` je Höhenstufe. Entwurf: `state/river/weberin-register-entwuerfe-2026-10-13.md` Block 5. Mountain-342-6: die Registerzeile folgt nach dem HAPI-Arm (dotted `<i>` + `alt`). xfer:river208-saber1.

**Fehlende Fäden — Registerzeilen-Entwürfe.** Gemessen (river-208/209): EPTA DR2 (Zenodo `8300645`, CC-BY-4.0; **25 Pulsare, nur TOAs, keine Residuen** — ein Residuen-Feld braucht erst ein Residuen-Produkt, Vorlage `field … inverse-square gravity s <τ>`, `phi/sources.φ:12633`) · KiDS DR4.1 WL (WL-Spalten `RAJ2000/DECJ2000,e1/e2,Z_B,weight`; Haupt-FITS 16,5 GiB) · Conicus3D (eine Datei `…phase_metadata.tar.zst`, 34,4 GB — `pending: 34-GB-Sample`) · WALLABY/CGPS (kein exaktes HiPS3D-Produkt — `pending: Produktname`). **Schon im Baum** (Entwurf stale): 1LHAASO `28305-28321` · Auger Zenodo `10488964` `29374-29378` · HMI/JSOC `terms CC0-1.0` `2026` · Pantheon+ SH0ES `MU_SH0ES` `29177-29185`. Entwürfe: `state/river/missing-threads-register-entwuerfe-2026-10-13.md`. Braucht: die `field`-Spalten-Zuordnung (WL-Shear, EPTA-Residuen, Aktivitätsindex) und `terms`/`ttl`. xfer:river208-faeden1.

## An future

Origin: river-206

**UCAR-FPI (Luftleuchten-Wind, Weberin-Zeuge) — freie E-Mail-Registrierung, Operator-Hand.** Der Datenpfad `https://registration.hao.ucar.edu/hao-reg_email.php?file=…20246300athabasca.dat&…` liefert die Registrierungsseite (HTTP 200 `text/html`); die `.dat` kommt nach Absenden der E-Mail-Adresse. **Im Operator-Browser ist die Registrierung erledigt** — die Datei liegt (`~/Downloads/20246300athabasca.dat`), der `format`-Arm steht. Kein offener Operator-Akt mehr. xfer:river206-ucarfpi1.

## Abschluss

river-210 (ein Pass; kein Fenster-Edit, kein Send; LOCK bleibt — Operator-Wort). Fälligkeit bei
Pass-Öffnung (HEAD `48387c9f9`): `--fired river` = 1 (die Große-Rätsel-Kandidatenliste,
operator-gebunden — nicht erneut vorgelegt), `--stale river --persist 3` = 0, `--addressed river`
= 2 (mountain-344, mycelium-346 — beide gefaltet) · `open_points_check` F208 = 0 stale-citation.
**aacgm-Koeffizient geheilt:** HEAD kompilierte nicht — `src/archivar/aacgm.rs:7` zieht
`aacgm_coeffs-14-2025.asc` (`include_str!`), die Datei war von `c33be5a67` (Operator-Commit,
AGENTS.md-Thema) gelöscht; jedes `ci-gate`-Job (`build`/`clippy`/`dropped-gate`/`subset`) brach am
`couldn't read … No such file` (`ci_manage log 38549948637`). In diesem Atom exakt wiederhergestellt
(Blob `9ebe282d2`, sha256 `8a8ec5e6…`, 639 Zeilen) und gestaged; `cargo check` 0/0.
**Adressierte Blöcke gefaltet:** mountain-343-1 (Athabasca-Zeilen stehen `phi:402-411`, kalibriert +
roh) · mountain-342-6 (SABER-Zeile folgt nach dem HAPI-Arm) · mycelium-344-4 (DE-2-`asset=`-Riss
geklärt; Lauf `38489787692` am Host-TLS).
**Kanal-Identität:** der in river-209 gemeldete `conserved_for_quantity`-Riss ist am Baum **geschlossen**
— Mountain-341 (`9031ed6f9`) hat die DE-2-Zahldichte-Kanäle auf `Conserved::Number` gezogen und
`conserved_for_quantity` (`channel.rs:763`) ausgerichtet; offen nur das `ci-gate`-`subset`-Netz
(die fünf Tests).
**GIC-Onset:** der Lauf `38485365619` ist `cancelled` — gemessen **180-min-Job-Timeout** mitten in
`intermagnet_xyz_compiler` (25-Jahre-Fenster); an Mycelium geroutet (Fenster splitten).
**Commit:** eigener Pfad (`src/archivar/aacgm_coeffs-14-2025.asc` + diese Übergabe + der `archiv/`-Move).
Kein Fremd-Hunk berührt; die App-Config-Commits (`48387c9f9` ff.) bleiben unangetastet.
