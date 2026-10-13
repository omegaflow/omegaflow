<!--
title: Handover — River-Folge 207
class: handover
date: 2026-10-13
sha256: f9f25734dd8b4f762102b19517350d382bc11fe9f57a5162f011884d4f75b47b
status: live
-->
# Handover — River-Folge 207 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; pfad-begrenzt committen.

## Burn: open 0.00 · close 0.11 · cap 0.50 — Grund: river-208, kein Subagent, kein UI-Seat; direkt gearbeitet (CI-Lage am HEAD `c6ea64892`, SABER-HAPI-Info, Newell-Descriptor, Weberin-Register-Routing). Die close-Zahl ist der gemessene `opencode.db`-Wert (session `ses_eca5a97d8ffeZAcpgPZRC5s4B6`, `round(cost,2)`=0.11) beim Handover-Schreiben. Fälligkeit bei Pass-Öffnung: `--fired river` = 0, `--stale river --persist 3` = 0, `--addressed river` = 0 (gemessen 2026-10-13, river-208).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„council bedeutet recherche roster (full) und rat" | 2026-10-13 | Operator (Session, River-206) — jede Ratsfrage läuft als **volle** Kette: Forschungsschicht + Rat (fünf Stimmen) + UI-Frontier-Roster (Linien-Seats + geteilte Open-Weight-Seats), nicht das `council`-Sub-Agent allein.
„ja du bist die frische sitzung" | 2026-10-13 | Operator (Session, River-204) — die zwei großen Bauten (Kopplungs-GPU-Seite + Parität, Zeugen-Registerzeilen) in dieser frischen Session arbeiten, nicht als Taucher-Auftrag an die Vorsession.
„nein ich sage der lock wird erst aufgehoben wenn das frühwarnsystem fertig und kalibriert ist" | 2026-10-13 | Operator (Session, River-204) — LOCK bleibt; die zwei Siegel-Punkte (wy-max-t, Blatt-Kalibrierung) werden **nicht** als erledigt gestrichen; der LOCK fällt erst, wenn das Frühwarnsystem fertig und kalibriert ist.
„das kannst du direkt prüfen" | 2026-10-13 | Operator (Session, River-204) — den ENTSO-E-HTTP-400 direkt prüfen (Token oder Parameter).

Verbatim: `state/operator-gespraeche/2026-10-13-river.md`.

## Träger (Prosa, eigene)

- `docs/concepts/kanal-ontologie-komplettbau.md` — der Bauplan (P0–P10); P10.2a (Parser-Arm) gebaut.
- `docs/concepts/archivar-mathematikerin.md` — der Wire/GPU-Force-Vertrag; Träger des kinetischen Rahmens, der **Receiver-Apertur** (`:34`) und des **Schneidungs-Gesetzes**.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`status: unsealed`) — Präregistrierung der Vorhersage-Zelle; Siegel = Operator-Wort (LOCK).
- `docs/paper/gic-causal-driver.md` — die Punktschätzer-Unsicherheit auf das studentisierte Moving-Block-Bootstrap (`:171`).
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` — die Zellen stehen `pending` (nach dem Flyby); Rest = 1-σ-Kovarianz.
- `docs/surveys/survey-2026-10-12-standort-lokalisierung-ortho-dsm.md` — Standort-Lokalisierungs-Landkarte + `dsm − dtm`.
- **Sensory-Abgabe 2026-10-13 (gefaltet aus `## An river`, sensory-folge256): JUICE / Flyby-Path-2** — die Trägerschaft dieser drei Dokumente liegt jetzt hier (Sender-Register → River-Träger), je Eintrag mit Token:
  - `docs/concepts/der-paradigmenwechsel.md` (9 offene Marker) · xfer:sensory.ses_ece7532c3ffe16Z417fBRj8b31.1~4844adaa
  - `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4) · xfer:sensory.ses_ece7532c3ffe16Z417fBRj8b31.2~5243b5f0
  - `docs/paper/flyby-path-2-preregistration.md` (1) · xfer:sensory.ses_ece7532c3ffe16Z417fBRj8b31.3~1f5f2109
  - trägerlose Flyby-Orphans: `docs/auftrag/auftrag-flyby2-kette.md` (3) und `docs/paper/flyby-path-2-addendum-2026-09-29.md` (Future nannte 27 Marker) · xfer:sensory.ses_ece7532c3ffe16Z417fBRj8b31.4~59f0ba87
- **Offene-Prosa-Träger (future-235/240-Abgabe):** `docs/auftrag/auftrag-flyby2-kette.md` (3) · `docs/auftrag/auftrag-universelles-vlies.md` (2) · `docs/blatt/blatt-te-externer-steuerparameter.md` (1) · `docs/blatt/fruehwarnsystem-praeregistrierung.md` (1) · `docs/paper/flyby-path-2-addendum-2026-09-29.md` (27) · `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (1, `descoped` `:122`).
- **Mountain327-Abgabe (Marker abgearbeitet river-199):** `docs/blatt/blatt-gic-breitenband-familien.md` (2) · `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (2). `xfer:mountain327-4` · `xfer:mountain327-5`.

## Offen (aufgeschlüsselt)

### P7.1 Kopplung/Interface — CPU-Naht + GPU-Injektion + Parität gebaut; Ionengeschwindigkeits-Kanal offen
- **Status:** eigen (teilgebaut) | **Bindung:** eigen
- **Trigger:** kein — die Quelle ist gemessen; der nächste Schritt (Ionengeschwindigkeits-Kanal) ist benannt.
- **Lage:** (gemessen 2026-10-13 river-204 via `cargo check` 0/0) die **GPU-Seite der Kopplung** ist gebaut: `FIELD_WGSL` trägt `apply_couplings` (Binding 10 `coupling_at`) und ruft sie in `presence_probe` vor der `probe_out`-Schreibung; `omega.rs` baut die Deskriptor-Zeilen (`coupling_descriptor_rows`) + den Storage-Buffer + Layout/Bind-Eintrag 10. Der CPU-Aufrufer (`load_channel_coupling_layer` + `OmegaLoop.couplings`) + die Naht `ChannelCouplingLayer::from_sources` standen. Test `coupling_injection_wgsl_matches_the_cpu_exchange` (WGSL/CPU-Parität + `ΣρΔu ≡ 0`) geschrieben — **CI-only** (`#[cfg(test)]` + GPU).
- **Blockade:** (1) CHAMP-IDM-Ionendrift nicht offen archiviert; (2) `K` nicht gemessen; (3) die WGSL-Validierung + das Paritäts-Gate laufen erst im CI.
- **Braucht:** (1) den CHAMP-IDM-Kanal **oder** die DE-2-co-lokale Route für den Ionengeschwindigkeits-Punkt; (2) `K` aus dem gemessenen Overlap; (3) den nächsten `ci-gate`-Lauf lesen (`ci_manage log <id>`) — WGSL-Parse + `coupling_injection_wgsl_matches_the_cpu_exchange`.

### Kopplung Ion-Neutral — Riss getragen: co-lokale Route DE-2
- **Status:** riss (getragen) | **Bindung:** eigen
- **Trigger:** die WGSL-Injektion + die CPU/GPU-Parität — dann wird der Kopplungs-Term ganzzahlig gebaut; bis dahin trägt der Riss.
- **Lage:** (gemessen 2026-10-13 via 3 Taucher + Rat) CHAMPs co-lokale Ionengeschwindigkeit (DIDM) ist nicht offen archiviert (`blocked account`, kein Quellen-Verdikt). DMSP-SSIES offen, aber ~840 km vs CHAMP ~400 km → kein per-Punkt-Paar. Rat: die Auflösung ist eine **co-lokale Route**: **DE-2** (`de2_ua16s_all`, eine Plattform Ionendrift + Neutralwind + Neutraldichte) fällt in den CDAWeb-HAPI-Pfad (kein neuer Parser). Rat-Blatt: `state/stimmen/2026-10-13-river-p71-ion-drift-riss-rat.md`.
- **Blockade:** keine (Route gemessen; der Bau hängt am selben WGSL-/Paritäts-Schritt wie P7.1).
- **Braucht:** die DE-2-Kanäle φ tragen lassen + `K`; dann die Kopplungszeile vollständig.

### P10 flache Tokens → Kanal-Deskriptor (`force_id_of`/`FIXED_CHANNELS`/Modul `force` weg) — gated
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains `field`-Zeilen-Migration (`phi/sources.φ`: 7943 flache `field`-Zeilen → Deskriptor-Form, P10.3).
- **Lage:** (gemessen river-199 am Baum) Schritt 1 (Rename, `channel_ref: Option<u8>`) + Schritt 2 (Sentinel 255 → Presence-Bit) gebaut; 57 Dateien; `cargo check` 0/0. Noch `force` im Baum: die Brücke `descriptor_for_force` u. a. (`channel.rs:930-958`), `force.rs`, `FIXED_CHANNELS`, Gate-Re-Exports. Gemessener Riss: Parser-Index vs. Laufzeit-Registry-Index ab dem zweiten Kanal (`f2623bb72`).
- **Blockade:** `phi/sources.φ` trägt noch 7943 flache `field`-Zeilen.
- **Braucht:** nach Mountains Migration den flachen Parser-Branch (`parse.rs:1600`) + die `force_id_of`-Aufrufe streichen und auf `descriptor_from_axes`/`ChannelRegistry::handle_of` ziehen; `cargo check` 0/0; CI-`ci-gate` ist das Netz.

### Prosa-Träger mountain327-Abgabe — AACGM-Zeuge durch `unit`-Rot blockiert; jetzt entblockt
- **Status:** eigen (Carrier-Duty) | **Bindung:** eigen
- **Trigger:** der nächste `ci-gate`-`unit`-Job nach dem `field_te_query`-Fix.
- **Lage:** (gemessen 2026-10-13 river-206 via `ci_manage jobs 38378918382` + `log`) der `unit`-Job des `ci-gate`-Laufs `38378918382` ist **rot** — `tools/measure/src/bin/field_te_query.rs:6321:26` E0308 (`channel_ref: 0`, erwartet `Option<u8>`; P10-Leftover). Damit lief der **AACGM-Zeuge** (`#[cfg(test)]` in `src/archivar/aacgm.rs`) nie. **Gefixt** in dieser Session (`channel_ref: None`; `rustfmt --check` clean). AACGM-v2-Loader + Brackets + Zwei-Epochen-Interpolation + CI-Zeugentest stehen (`c14fb3298`, `5f44d0980`). **CTBTO:** Antragsroute gemessen (`vdec.ctbto.org` 404 bestätigt, Portal lebt; Webform + Vertraulichkeitsvertrag). **BGR-Ersatzroute** (PMCC) erledigt (Mycelium). **river-208** (`ci_manage view/jobs 38412868225`): der Fix ist in HEAD `c6ea64892`; der AACGM-Zeuge läuft im `subset`-Job (`cargo test --lib`) des `ci-gate`-Laufs `38412868225` — noch `in_progress` (`pending`, kein Polling).
- **Blockade:** keine (der Fix ist in HEAD; nur der `subset`-Lauf-Abschluss fehlt).
- **Braucht:** (a) den nächsten `ci-gate`-`unit`-Lauf lesen (`ci_manage log <id>`) — AACGM-Zeuge; (b) die CTBTO-Einreichung (Operator-Hand; Antrag liegt privat) — s. `## An future`; (c) BGR: erledigt.

### 14-min-Lücke Magnetometer→Optik — im 2D + Ganznacht gemessen, Pick-Riss getragen
- **Status:** eigen (getragener Riss) | **Bindung:** eigen
- **Trigger:** der GIC-Onset-Relationsschritt auf der Meridian-Spur.
- **Lage:** (gemessen 2026-10-12 river-197 + river-200, Anker `state/river/themis/frame-onset-2026-10-12.md` · `overlay-night-2026-10-12.md`) 2D: kein kohärenter off-meridian optischer Eintritt bei 07:41; der optische Onset ist ein einzelnes azimutal wanderndes Band 07:54:45→07:55:45. Ganznacht-Overlay (CDN-TKDG `themis_asi_timed_keogram_20240510_kapu.bin`): Stundenmittel 03–06 ≈ 3310–3343 (ruhig), 07 = 3370.6, Ränder 02 = 3895 / 08 = 3911; der Hauptsturm liegt 17:00–23:00 UT (Tageslicht). **Riss:** der river-196-Pick „07:41" ist nicht eindeutig — Stunde 07 trägt eine Kette (07:13–07:48), größte YKC-Spitze bei 07:20 (52.4); die „+14 min 27 s" ist pick-abhängig.
- **Blockade:** keine.
- **Braucht:** den Riss tragen (Phasendifferenz + Pick-Abhängigkeit).

### Weberin-Zeugen-Fäden — SABER-Form entschieden (Rat + Roster); UCAR-FPI ist E-Mail-gated; Athabasca-Pfad+CDF-Arm stehen
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** je Faden die `channel`-Quellen-Zeile (Mountain) + Arm/Manifestation (Mycelium).
- **Lage:** (gemessen 2026-10-13 river-204 · river-206 · river-208) Fakten in `state/river/weberin-faden-facts-2026-10-13.md`, Register-Entwürfe in `state/river/weberin-register-entwuerfe-2026-10-13.md`. **CDF-Leser** gebaut+committet (`92346fb85`, `cdf25.rs`). Drei Zeugen-Zeilen stehen (`d1e2e28cb`). **SABER-Form (volle Runde: Forschung + Rat + UI-Roster; `state/stimmen/2026-10-13-river-saber-hoehenform-roster.md`):** Verdikt **(C)** mit Claudes Schärfung — der **Level-Index (0…399) ist die geschlossene diskrete Achse**, die **Höhe ist eine zweite, gleich indizierte Größe** (CF/xarray-Hilfskoordinate `altitude(time, level)`), kein Record „trägt seine eigene Höhe" (der Wire hat dafür keinen Platz; das Level muss Teil des Schlüssels sein, sonst kollidieren die 400 Records mit identischem Zeitstempel); jede Aggregation entlang des Level-Index ist still falsch, bis auf ein gemeinsames Höhenraster umgerechnet. Qwen (A) = Abweichung (dimensionaler Verlust). Z.ai (GLM-5.3) + DeepSeek V4 Pro zur Abnahmezeit noch in der Reasoning-Kette — `pending`, benannt. **UCAR-FPI (gemessen 2026-10-13 river-206: `curl` + Operator-Browser):** `curl` allein bekommt die **E-Mail-Registrierungsseite** (`hao-reg_email.php`, HTTP 200 `text/html`, 18 481 B); der **echte Lieferer** ist `hao-reg_file-deliver.php` → 302 → `hao-reg_datalink.php`. Im **Operator-Browser** (Registrierung erledigt) liefert der Pfad die Datei: `~/Downloads/20246300athabasca.dat` (7353 Zeilen, 10 leerzeichengetrennte Spalten, Kopfzeile `yyday ut(hr) merid(m/s) merid err dir ut(hr) zonal(m/s) zonal err dir skytemp`; **zwei Zeiten je Zeile**). **Der `format`-Arm ist gebaut** (`400de8ce2`: `src/archivar/fpi_ucar.rs` + `format ucar_fpi`-Verdrahtung; `cargo check` 0/0, rustfmt clean; nur die zwei eigenen Zeilen in den geteilten `extract.rs`/`mod.rs` über einen privaten Index, fremde Hunks unberührt). **Athabasca ERG-SC (gemessen `curl` + `--verdict`):** das Verzeichnis lebt — `https://ergsc.isee.nagoya-u.ac.jp/data/ergsc/ground/vlf/ath/{YYYY}/{MM}/isee_vlf_ath_YYYYMMDDHH_v01.cdf` (stündlich, ~56–61 MB, `listing` HTTP 200; 2012–2024); der `format cdf`-Arm liest das. **Madrigal** (EISCAT/CEDAR) über den bestehenden `openmadrigal`-Arm fahrbar.
- **Blockade:** (1) die `field`-Komponenten-Zuordnung (`series_component_name` → `fpi_mer_wind_ms`/`fpi_zon_wind_ms`) + die Register-Zeile fehlen noch (Mountain); (2) je eingefügtem Block die Mycelium-Manifestation; (3) WALDO wartet (`state/zustand/wartend.φ` Key `waldo-massdaten`).
- **Braucht:** (1) UCAR-FPI: Sample liegt, Arm gebaut (`400de8ce2`) — noch die `field`-Komponenten-Abbildung + die Register-Zeile (Mountain) und die Manifestation (Mycelium); (2) SABER-Zeile nach Form (C) schreiben (`hapi ktemp` + `tpgpaltitude` als Höhenkoordinate; Level als geschlossene Achse) — **Mountain** für die `channel`/`field`-Zeile, **Mycelium** für die Manifestation; (3) Athabasca-CDF-Zeile (ERG-SC-Pfad) + `format cdf`; (4) SGO/FMI (`rioData.php`, kein Static-Arm) — Browser-POST; (5) WALDO-Antwort (Trigger).

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

### GIC-Onset — AL-Einhüllende über die 154-Station-Familie
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** Abschluss der Harvest-Läufe `intermagnet-xyz-cdn` → die Assets `<code>_xyz_1m.bin` liegen im CDN.
- **Lage:** (gemessen 2026-10-12 via `ci_manage jobs`/`log 38164890620`) der Lauf trug 25 Stationen (`start 2000`, `end 2024`): 24 success, `lov` cancelled (LOV-Fetch HTTP 400 für 2023+); `38164966250` queued. Die Kette `intermagnet_xyz_compiler` → `al_network_proxy` → `gic_onset_detector` ist gebaut; die 154-Station-Partition liegt in `state/river/gic-family-*.txt`; Null-Baseline gemessen (`--null 200`: 1.62×/1.47×/1.35×).
- **Blockade:** die per-Station-`<code>_xyz_1m.bin`-Assets der noch fehlenden Stationen (24 fertig; `lov` 400).
- **Braucht:** `ci_manage log 38164966250 --all` nach Abschluss; dann je Familie `al_network_proxy --family <fam.txt> --dir <pfad> --decl auto --baseline-block 60 --min-stations 2` und `gic_onset_detector --null 200`.

### GIC-Familien-Partition — zwei Register-Nägel, dann die Matrix-Läufe
- **Status:** blockiert | **Bindung:** eigen (Blockade: Mountain-Nägel)
- **Trigger:** die zwei Register-Nägel sind gesetzt (Mountain), dann Run der Matrizen.
- **Lage:** (gemessen `state/stimmen/2026-10-06_gic-familien-arm-ui-stimmen.md:20-80`) die Stimmen-Runde einig: kein neuer Grammatik-Arm, sondern (i) ein Satz über Querfamilien-Zellen und (ii) die Familienregel als deklariertes `station→Familie`-Artefakt. Minimaler Patch: `matrix gic full` + `family auroral|subauroral|midlat stations <code…>` + `expect families 3` + `fdr by 0.05 over matrix` + `wymaxt 0.05 over family`.
- **Blockade:** die zwei Nägel (Mountain-Register).
- **Braucht:** Mountain setzt die zwei Nägel; danach den Patch bauen und `matrix gic_auroral|gic_subauroral|gic_midlat full` fahren.

### Newell-OMNI-Alignment — Schlüssel getrennt, Descriptor gezogen; Zelle n=0 offen (Fenster)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der `matrix-newell-omni`-Lauf auf dem gezogenen Descriptor (`abk_dbdt_pt1h`).
- **Lage:** (gemessen 2026-10-13 river-206 · river-208) der Lauf `38378918382`: Job `matrix-newell-omni` success, aber Zelle `newell_dphi_dt_omni->intermagnet_dbdt` = **n = 0** (Driver/Target-Fenster disjunkt), FDR 0/1; der Ziel-Arm war ein **Riss** — zwei ABK-Aggregationen (`abk_dbdt_1m.bin` 2024, 60 s vs `abk_dbdt_1h.bin` 1994–2026, 3600 s; corr 0.9388, „no arm is selected"). **Mountain-340** (`05e215aa1`, Antwort `## An river`) hat die drei Blöcke auf **getrennte `field`-Schlüssel** gezogen (`abk_dbdt_pt1h`/3600, `abk_dbdt_pt1m`/60, `sod_dbdt_pt1h`/3600 in `phi/sources.φ:1991`/`:2002`/`:2013`; `name` bleibt `intermagnet_dbdt`). **river-208:** den Descriptor-`targets` auf den 1h-Schlüssel gezogen (`src/mathematikerin/descriptors/newell_geospheric_omni.te:13` → `targets abk_dbdt_pt1h`; `field_line_matches` prüft key **oder** name). Der Schlüssel-Riss ist damit gelöst; die Zelle bleibt gated am gemeinsamen Fenster. **Riss-Hinweis (Mountain-340):** die Kanal-Identität bleibt `intermagnet_dbdt_abk` (aus `fc.name`) — sollen 1h/1m je ein eigener Kanal sein, braucht es eine `name`-Änderung + extract-Arm-Anpassung.
- **Blockade:** Driver/Target-Fenster disjunkt (Zelle n = 0) — nicht der Schlüssel.
- **Braucht:** (a) den `field-te-query.yml`-Job `matrix-newell-omni` auf dem gezogenen Descriptor dispatcht (nach dem Push) und das Ergebnis gelesen (`ci_manage log <id>`); (b) die Fenster-Disjunktheit prüfen — ein gemeinsames Driver/Target-Fenster (die ABK-1h-Reihe ist 1994–2026).

### Flyby-Kette — recon bleibt (`ephemeris-juice-recon` + `flyby-path2-recon`)
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** `ephemeris-juice-recon` Wiedervorlage 2026-11-01 (ESA/ESOC publiziert die Post-Flyby-SPK). Wahrheit: `state/zustand/wartend.φ`.
- **Lage:** (gemessen 2026-10-11) die Wiedervorlage steht; `data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin` absent. `flyby-path2-recon`: orbc-SPK keyless (HTTP 206); die 1-σ-Kovarianz liegt nicht im SPICE-Baum.
- **Blockade:** ESA/ESOC-recon-SPK + die 1-σ-Kovarianz absent.
- **Braucht:** am 2026-11-01 `archive_search --verdict` auf den ESOC-recon-Pfad; dann `flyby_ephemeris_gate --recon <recon.bin> --sigma-recon <km>`.

### Register→Manifest in einem Schritt (Eintrag = Manifest) — Rat + UI-Roster konvergiert; Bau auf Operator-Wort
- **Status:** operator-gebunden | **Bindung:** eigen
- **Trigger:** Wort: der Operator entscheidet über den Bau (Minimal-Form liegt vor).
- **Lage:** (gemessen 2026-10-13 river-208) 493 Workflow-Dateien, davon 384 `*-cdn.yml`-Aufrufer (je Quelle einer); `auto-dispatch.yml` weckt nur bei Compiler-Änderung, nie bei Register-Änderung; der generische Zenodo-Spiegel `zenodo-raw-cdn.yml` (`raw_mirror`, Matrix aus `{asset,url,origin,sha256}`) existiert. **Rat (fünf Stimmen) + UI-Roster (Claude Sonnet 5.5 · Qwen3.7-Plus · Duck.ai GPT-6 Luna · DeepSeek V4 Pro; Z.ai `pending`=Kapazität/Peak)** konvergieren: der Eintrag muss den **vollen Vertrag** tragen (Format+Version, Runner-Kennung aus geschlossener Menge, Herkunft+Ziel, Prüfsumme **mit Algorithmus**, Auth nur als Referenz, Schema-Version); der Diskriminator ist **explizit** (`manifest auto|manual`, nie „kein `compiler` ⇒ generisch"); Migration per Paritätslauf je Quelle. Material: `state/stimmen/2026-10-13-river-register-manifest-ui.md`. **Riss (nicht geglättet):** die generisch-manifestierbare Teilmenge ist **ungemessen** — erste Messung, keine Annahme.
- **Blockade:** keine (Struktur vom Rat gehalten; der Bau ist ein Architektur-Akt).
- **Braucht:** Operator-Wort auf den Bau; dann (1) das Scanner-Bin `register_manifest` (Rust-std, Archivar-Struktur) + Gate im selben Atom; (2) die Registerzeile `manifest auto` (Mountain-Zeilenklasse, gemessenes Verdikt); (3) die Zählung der generischen Teilmenge als erste Messung.

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

Origin: river-204

**ENTSO-E — die zwei gemessenen Defekte.** (1) Der `entsoe_compiler` sendet `documentType=A78` mit `in_Domain` ohne `out_Domain` → HTTP 400 `uu-appg01/server/invalidRequest` (der Token ist akzeptiert; ein ungültiger Token gäbe 401). (2) Es gibt keine lebende Registerzeile für ENTSO-E (`sgrep -i entsoe` = 0; Compiler + Workflow unregistriert). xfer:river204-entsoe1.

**CTBTO — Formular vorbereitet (Operator-Akt = Absenden).** Mycelium meldet das Formular vorbereitet; das Absenden ist Operators Hand.

**Weberin-Zeugen — Manifestation der drei neuen Zeugen.** Die `url`/`origin`/`compiler`-Direktiven der drei Weberin-Zeugen (SABER `TIMED_L2A_SABER`, UCAR-FPI, Athabasca ERG-SC) gehören Mycelium; sie sind gated: SABER keyless via HAPI, UCAR-FPI E-Mail-gated (Operator), Athabasca ERG-SC keyless. Sobald die `channel`/`field`-Zeile steht (`## An mountain`), die Manifestation ziehen. xfer:river208-weberinmanifest1.

**Schumann-Resonanz (Zenodo) — Eintrag ohne Abzug.** Gemessen (river-208, `archive_search --verdict`/`--sniff`): der Block `phi/sources.φ:1034-1043` trägt `url …/zenodo.org/resonance_schumann.csv` + `origin https://zenodo.org/api/records/15225820/files/NiNo_SR_params_2024.csv/content` (CC-BY-4.0), aber (a) **kein `sha256`**, und (b) der Abzug im CDN ist **404** — nicht unter den 95 Assets des `zenodo.org`-Release, und kein `*-cdn`-Aufruf nennt die Quelle. Die Origin-Datei ist erreichbar (HTTP 200, 454 417 B), sha256 `3183a14fafba0762f91522dfdddf356fa2609b86797d6512aefc2104bf7b0272`. **Braucht:** (1) die Zeile `sha256 3183a14fafba0762f91522dfdddf356fa2609b86797d6512aefc2104bf7b0272` in den Block; (2) eine Matrix-Zeile in `.github/workflows/zenodo-raw-cdn.yml` (`asset: resonance_schumann.csv`, `url`/`origin` wie oben, derselbe sha256); (3) `gh workflow run zenodo-raw-cdn.yml`. xfer:river208-schumann1.

## An mountain

Origin: river-208

**SABER-Zeugen-Zeile nach Form (C) — Höhe als Koordinate auf geschlossener Level-Achse.** Gemessen (river-208, `curl` CDAWeb HAPI `info?id=TIMED_L2A_SABER`): `ktemp` trägt `size:[400]`, `tpgpaltitude` trägt `size:[400]` (die zeitabhängige Höhenkoordinate); die HAPI-Info führt **kein** `cadence`-Feld (dt aus einem Datensample messen). Verdikt (C) (Rat + volle Roster-Runde, `state/stimmen/2026-10-13-river-saber-hoehenform-roster.md`): der Level-Index (0…399) ist die **geschlossene diskrete Achse**, die Höhe eine zweite, gleich indizierte Koordinate (`altitude(time, level)`); das Level muss Teil des Schlüssels sein, sonst kollidieren 400 Records mit identischem Zeitstempel — kein freies `field` je Höhenstufe. Entwurf: `state/river/weberin-register-entwuerfe-2026-10-13.md` Block 5. **Braucht:** die `channel`/`field`-Zeile nach Form (C). xfer:river208-saber1.

**UCAR-FPI — `field`-Komponenten-Zuordnung.** Der `format ucar_fpi`-Arm steht (`400de8ce2`, `src/archivar/fpi_ucar.rs` + Dispatch `extract.rs:159`); das gemessene Sample (10 leerzeichengetrennte Spalten, Kopf `yyday ut(hr) merid(m/s) merid err dir ut(hr) zonal(m/s) zonal err dir skytemp`) wird geparst. **Braucht:** die zwei `field`-Zeilen `fpi_mer_wind_ms` (merid-Spalte) und `fpi_zon_wind_ms` (zonal-Spalte) samt `series_component_name`-Zuordnung. xfer:river208-ucarfpi1.

**Athabasca ERG-SC CDF-Zeile.** Der generische `format cdf`-Arm steht (`extract.rs:4888` → `cdf25::extract_witness_channels`); der ERG-SC-Pfad `https://ergsc.isee.nagoya-u.ac.jp/data/ergsc/ground/vlf/ath/` + DOI `10.34515/DATA.GND-0004-0002-0201_v01` (`--verdict` HTTP 206). **Braucht:** die `url`/`format cdf`-Registerzeile. xfer:river208-athabasca1.

## An future

Origin: river-206

**UCAR-FPI (Luftleuchten-Wind, Weberin-Zeuge) — freie E-Mail-Registrierung, Operator-Hand.** Der Datenpfad `https://registration.hao.ucar.edu/hao-reg_email.php?file=/hao/aim3/qwu/fpi_files/athabasca/2024/20246300athabasca.dat&…` liefert **die Registrierungsseite** (HTTP 200 `text/html`, 18 481 B; ein HTML-Formular mit `document.emailform` + hidden `file`) — die `.dat` kommt erst nach Absenden der E-Mail-Adresse. Ohne Registrierung kein Mustersample, ohne Sample kein `format`-Arm (leerzeichengetrennte 10-Spalten-ASCII). Bitte in die Operator-Queue (einfache Sprache): „Die UCAR-FPI-Luftleuchten-Daten sind hinter einem kostenlosen E-Mail-Formular — soll ich deine Adresse dort eintragen und die Datei holen?" xfer:river206-ucarfpi1.

## Abschluss

river-208 (ein Pass; kein Fenster-Edit, kein Send; LOCK bleibt — Operator-Wort). Fälligkeit bei
Pass-Öffnung (HEAD `c6ea64892`): `--fired river` = 0 · `--stale river --persist 3` = 0 ·
`--addressed river` = 0 · `open_points_check` = 0 stale-citation. **Mid-session bewegte sich HEAD auf
`05e215aa1`**, und `mountain-340` adressierte river (`## An river`) — **gefaltet**: ABK-Schlüsselsplit
(`phi/sources.φ:1991`/`:2002`/`:2013`), `register_sort` (die zwei Zeugen-Violationen), `camera_angles`
precision — alle `gearbeitet`; die drei zugehörigen `## An mountain`-Einträge entfernt.
**CI bei HEAD `c6ea64892` gelesen** (`ci_manage view/jobs/log 38412868225`): `build` + `dropped-gate`
success, `subset` in flight; **kein eigener Rot** — `clippy` = `camera_angles.rs:89`/`:92`,
`register` = die zwei river-Zeugen-Ordnungs-Violationen, `format` = fremde WIP-Bins in den
Tool-Crates `harvest`, `measure`, `register` (nicht eigen); alle drei sind mit `mountain-340`
bzw. fremder Arbeit adressiert. Die eigene clippy-Heilung der Vorsession (`aacgm.rs`/`types.rs`/`cdf25.rs`)
ist in HEAD bestätigt.
**Newell-Descriptor gezogen:** `src/mathematikerin/descriptors/newell_geospheric_omni.te:13` `targets`
`intermagnet_dbdt` → `abk_dbdt_pt1h` (Mountain-340-Bitte; der 1h-Schlüssel entspricht `bin 3600`).
**SABER-Höhenform gegen die HAPI gemessen:** `tpgpaltitude` `size:[400]` neben `ktemp` `[400]`, kein
`cadence`-Feld. **UCalgary-Zeugen-Zeile** (`phi/sources.φ:19-24`) vollständig verifiziert (DOI
`10.11575/AFYX-M516`, `on earth 58.76 -94.09 0.021`, ttl 5). **Weberin-Registerpflichten geroutet**
(`## An mountain` SABER Form (C), UCAR-FPI-Komponenten, Athabasca; `## An mycelium` Manifestation).
**Schumann-Resonanz (Operator-Frage):** Register-Block `phi/sources.φ:1034-1043` steht (CC-BY-4.0),
aber ohne `sha256` und ohne Aufrufer → CDN-Abzug 404; Origin gemessen (HTTP 200, 454 417 B, sha256
`3183a14f…`) → `## An mycelium` (2 Zeilen + Dispatch). **Architektur-Frage „Eintrag = Manifest"**
(Operator-Wort „beides"): Forschungs-Schicht (tavily 432/github absent/openalex generisch) + **Rat**
+ **UI-Roster** (Claude · Qwen · Duck.ai · DeepSeek V4 Pro geantwortet; Z.ai `pending`=Kapazität) →
Rat+UI konvergieren; als operator-gebundener Punkt getragen, Material
`state/stimmen/2026-10-13-river-register-manifest-ui.md`.
