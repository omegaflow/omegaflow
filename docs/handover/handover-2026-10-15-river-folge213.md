<!--
title: Handover — River-Folge 213
class: handover
date: 2026-10-15
sha256: c39235638317c585c3d69a6831269a3f7edc8e5c71beb507b2d050a10b2652e5
status: live
-->
# Handover — River-Folge 213 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; pfad-begrenzt committen.

## Burn: open 0.0 · close pending — `session_burn` meldet `db absent` (`~/.local/share/opencode/opencode.db`), die private Session-Zeile wird nicht gelesen (nicht gemessen, Vorbehalt wie river-212/213) · cap 0.55 benannt-ungeprüft — Grund: river-214 (ein Pass; kein Fenster-Edit, kein Send, kein UI-Seat) — die zwei adressierten Blöcke gefaltet (mountain-348, mycelium-350), die zwei GIC-AL-Läufe `38642948810`/`38643271579` ausgewertet (Zahlen unten), die zwei fehlenden Familien-Läufe `38690576424`/`38690576605` angestoßen, den Schumann-Trigger als gefeuert gemessen (Asset HTTP 200) und die Schumann-Matrix gebaut. Fälligkeit bei Pass-Öffnung (HEAD `d512d8fee`): `--fired river` = 1 (die Große-Rätsel-Kandidatenliste, operator-gebunden — nicht erneut vorgelegt), `--stale river --persist 3` = 0, `--addressed river` = 2 (mountain-folge348, mycelium-folge350 — gefaltet); `open_points_check` F212 = 0 stale-citations (21 Pfad-Refs absent, s. Abschluss — der private Baum liegt außerhalb des öffentlichen). (gemessen 2026-10-15, river-214)

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„council bedeutet recherche roster (full) und rat" | 2026-10-13 | Operator (Session, River-206) — jede Ratsfrage läuft als **volle** Kette: Forschungsschicht + Rat (fünf Stimmen) + UI-Frontier-Roster (Linien-Seats + geteilte Open-Weight-Seats), nicht das `council`-Sub-Agent allein.
„ja du bist die frische sitzung" | 2026-10-13 | Operator (Session, River-204) — die zwei großen Bauten (Kopplungs-GPU-Seite + Parität, Zeugen-Registerzeilen) in dieser frischen Session arbeiten, nicht als Taucher-Auftrag an die Vorsession.
„nein ich sage der lock wird erst aufgehoben wenn das frühwarnsystem fertig und kalibriert ist" | 2026-10-13 | Operator (Session, River-204) — LOCK bleibt; die zwei Siegel-Punkte (wy-max-t, Blatt-Kalibrierung) werden **nicht** als erledigt gestrichen; der LOCK fällt erst, wenn das Frühwarnsystem fertig und kalibriert ist.
„das kannst du direkt prüfen" | 2026-10-13 | Operator (Session, River-204) — den ENTSO-E-HTTP-400 direkt prüfen (Token oder Parameter).
„ja bitte aber warum sind es nur 384 aufrufe wir haben doch 2500 blöcke" | 2026-10-13 | Operator (Session, River-208) — **Bau-Wort für den Register→Manifest-Automatismus** (das Kabel bauen; zuerst die Zählung der generischen Teilmenge als erste Messung). Die Frage nach 384 vs. 2565 ist mit der Messung unten beantwortet.
„ja bau es allerdings auch mit den sonstigen gates etc also lizenzprüfung z.b." | 2026-10-13 | Operator (Session, River-208) — der generische Aktor läuft **durch die vorhandenen Tore**: Lizenz-Tor, Fingerabdruck-Tor, Format-Tor, Registrierungs-Tor; kein Tor wird umgangen.
„ja bitte" (auf die Rückfrage „warum möchtest du da hin?") | 2026-10-14 | Operator (Session, River-212) — die `Burn:`-Zeile trägt nur den öffentlichen `session_burn`-Aggregatstand mit Vorbehalt; die private `opencode.db` (isolierte Session-Zeile) wird **nicht** gelesen. Der parallele `agent_sandbox`-Commit `504641409` bindet `~/.local/share/opencode` zusätzlich ab — die Session-DB trägt das private Haus.
„Starte die River-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | 2026-10-15 | Operator (Session, River-214) — der Pass läuft in einem Atom: still messen, Bekanntes bis zur Kante arbeiten, nur Ungeklärtes vorlegen.
„bitte auch gegen den zufall prüfen und es erscheint mir fast so als gäbe es eine kurve und einen sweetspot ist das richtig?" | 2026-10-15 | Operator (Session, River-214) — die Einzelstation gegen die Kreisschub-Null prüfen und die Detektor-Kurve (Stationszahl/Schwelle) als Sweet-Spot messen.

Verbatim: `state/operator-gespraeche/2026-10-15-river.md` (diese Session) · `state/operator-gespraeche/2026-10-14-river.md` (Vorsession).

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

## Adressierte Blöcke — gefaltet 2026-10-14 (river-211)

- `xfer:mountain-342-6` **SABER** (mountain-345) — die Registerzeile folgt nach dem HAPI-`alt`-Arm; `cdaweb.gsfc.nasa.gov` ist host-weit unerreichbar (Stehender Pass), das Sample bleibt ungemessen. Im Weberin-Punkt unten nachgezogen (Mountain hält den Arm).
- `xfer:mountain-343-1` **Athabasca** (mountain-345) — von river-210 gefaltet; die Zeilen stehen (`phi/sources.φ:402-411`). Kein Rest.
- `xfer:mycelium-344-4` **DE-2 + `asset=`-Riss** (mycelium-347) — die Riss-Klärung steht (`harvest-dispatch.yml:106` liest `asset=`); der Lauf `38552385568` scheitert **gemessen am Host** (`cdaweb.gsfc.nasa.gov` TLS-Abbruch). `asset=fehlt` bleibt bis zum ersten erfolgreichen Pull. In P7.1 nachgezogen.
- `xfer:mycelium-347-1` **intermagnet-XYZ-Timeout** (mycelium-347) — **Korrektur** der river-210-Messung: nicht das 25-Jahre-Fenster war die Ursache, sondern der Void-Retry auf out-of-range Monate (HAPI 1405) — die HAPI-`info` je Station trägt eine begrenzte Spanne (`mbc` 1991…1997), der Compiler fragte blind `2000..2024`, der 3×-Retry kostete ~8–11 min/Jahr. Der Compiler liest jetzt die `info`-Spanne und beschneidet das Fenster; nach dem erfolgreichen Pull setzt River `al_network_proxy`/`gic_onset_detector`. Im GIC-Onset-Punkt unten nachgezogen.
- `xfer:mountain-341` **`conserved_for_quantity`** — von river-210 gefaltet (Channel `Number`, `channel.rs:763`); der Riss ist am Baum geschlossen. Kein Rest.

## Adressierte Blöcke — gefaltet 2026-10-14 (river-212)

- `xfer:mountain-342-6` **SABER** (mountain-346) — **neu:** der Host `cdaweb.gsfc.nasa.gov` ist intermittierend erreichbar (am 2026-10-14 HTTP 200); das Sample bleibt bis zur Routen-Entscheidung ungemessen, die Registerzeile folgt nach dem HAPI-`alt`-Arm. Kein River-Akt (Bau bei Mountain; kein Harvest-Dispatch von River — Mycelium).
- **UCAR-FPI** (mountain-346) — **neu:** die zwei `field`-Zeilen stehen code-gemessen; offen bleibt allein die FPI-eigene Athabasca-Instrumentenkoordinate. Im Weberin-Punkt nachgezogen.
- `xfer:mycelium-344-4` **DE-2 + `asset=`-Riss** (mycelium-347) — unverändert zu river-211 (Riss-Klärung steht; Lauf `38552385568` am Host-TLS gescheitert). Keine Neu-Auslegung.
- `xfer:mycelium-347-1` **intermagnet-XYZ-Timeout** (mycelium-347) — unverändert zu river-211 (Compiler auf die HAPI-`info`-Spanne umgestellt); der Dispatch steht bei Mycelium, River setzt `al_network_proxy`/`gic_onset_detector` nach dem erfolgreichen Pull.

## Adressierte Blöcke — gefaltet 2026-10-14 (river-213)

- mountain-346 und mountain-347 (SABER, UCAR-FPI) — beide Blöcke gleichlautend: die SABER-Registerzeile folgt nach der Routen-Entscheidung (Rats-Verdikt B + A); der Host ist intermittierend erreichbar, das Sample bleibt bis zur Route ungemessen. UCAR-FPI: die zwei `field`-Zeilen stehen code-gemessen, offen allein die FPI-eigene Athabasca-Instrumentenkoordinate. Kein neuer River-Akt.
- `xfer:mycelium-347-1` **intermagnet-XYZ — erfolgreich** (mycelium-349): der `intermagnet-xyz-cdn`-Lauf `38618793419` ist `success` (head `c7a6c7ca2`); der Pull steht. River hat den `gic-onset`-Lauf `38642215979` (Reihe `abk_dbdt_1h.bin`) angestoßen; die AL-Netz-Kette (`al_network_proxy` → `gic_onset_detector`) ist noch **nicht** in CI verdrahtet (s. GIC-Onset-Punkt).
- `xfer:mycelium-344-4` **DE-2 + `asset=`-Riss** (mycelium-349): unverändert — der Pull scheitert am Host (`cdaweb.gsfc.nasa.gov` TLS-Abbruch), `asset=fehlt`.

## Adressierte Blöcke — gefaltet 2026-10-15 (river-214)

- `xfer:mountain-348-7` **SABER** (mountain-348) — die Registerzeile folgt nach der Routen-Entscheidung (Rats-Verdikt B + A); der Host ist intermittierend erreichbar. Kein neuer River-Akt (Bau bei Mountain).
- `xfer:mycelium-347-1` **intermagnet-XYZ — erfolgreich** (mycelium-349/350): der `intermagnet-xyz-cdn`-Lauf `38618793419` ist `success`; der Pull steht. Die AL-Netz-Kette ist verdrahtet und ausgewertet (s. GIC-Onset).
- `xfer:mycelium-344-4` **DE-2 + `asset=`-Riss** (mycelium-349/350): unverändert — der Pull scheitert am Host (`cdaweb.gsfc.nasa.gov` TLS-Abbruch), `asset=fehlt`.
- **ENTSO-E (river-204) + Schumann (river-208)** — mycelium-350 meldet beide am Baum erfüllt; am Baum verifiziert (river-214): der ENTSO-E-Block steht (`phi:11836-11843`), der Compiler heilt ZIP/`out_Domain` (`52312fade`); der Schumann-`sha256` steht (`phi:1062`) und der CDN-Abzug antwortet HTTP 200. Die zwei `## An mycelium`-Blöcke sind damit gelöscht.

## Offen (aufgeschlüsselt)

### P7.1 Kopplung/Interface — Register + CPU-Naht + GPU-Injektion gebaut; `K` offen
- **Status:** eigen (teilgebaut) | **Bindung:** eigen
- **Trigger:** die DE-2-Reihe liegt im CDN → dann `K` aus dem gemessenen Overlap; bis dahin trägt der Riss.
- **Lage:** (gemessen 2026-10-14 river-210 am Baum) der **co-lokale Kanal ist im Register deklariert** — `phi/sources.φ:596-616`: url `DE2_UA16S_ALL` (1981-08→1983-02, `terms PD`, keyless), hapi-Mapping Ionendrift (`RPA_IVE/IVN`), Neutralwind (`WATS_NVE/NVU`), N2-Dichte (`NACS_N2`), die vier `channel`-Zeilen (`:611-614`, beide Dichte-Kanäle jetzt `number:…`), `constraint quasi-neutral de2_qn region thermosphere` (`:615`) und die Zeile `coupling ion-neutral-friction from de2_ion_drift to de2_neutral_wind rate pending density_from number:de2_ion_density:2.66e-26 density_to number:de2_neutral_density:4.65e-26 …` (`:616`). Die **GPU-Seite** ist gebaut (`FIELD_WGSL` `apply_couplings`, Binding 10 `coupling_at`; `omega.rs` `coupling_descriptor_rows` + Buffer/Layout); die CPU-Naht `ChannelCouplingLayer::from_sources` steht; Test `coupling_injection_wgsl_matches_the_cpu_exchange` geschrieben — `#[cfg(not(target_arch = "wasm32"))] #[test]` in `src/mathematikerin/tests.rs:2960`, läuft also im `ci-gate`-`subset` (`cargo test --lib`), sobald der `mathematikerin::tests`-Block erreicht wird (der Lauf `38583663935` wurde vorher canceled).
- **Blockade:** (1) die DE-2-Reihe ist **noch nicht im CDN** — Manifestation gebaut (Mycelium `2d696704a`); der Lauf `38489787692` scheitert am Host (`cdaweb.gsfc.nasa.gov` TLS-Abbruch; mycelium-346/347, host-spezifisch bestätigt); (2) `K` = `rate pending`; (3) die WGSL-Validierung + das Paritäts-Gate (`mathematikerin::tests`) liefen noch nicht — der Lauf `38583663935` (head `08d572723`) wurde im `mathematikerin::te`-Block canceled, der `mathematikerin::tests`-Block druckte nicht; der Baum ist ohnehin rot (s. `## An mountain`).
- **Braucht:** (1) die DE-2-Reihe im CDN (Manifestation läuft/am Host blockiert; danach ist `K` messbar); (2) `K` aus dem co-lokalen DE-2-Overlap; (3) einen `subset`, der den `mathematikerin::tests`-Block erreicht — WGSL-Parse + `coupling_injection_wgsl_matches_the_cpu_exchange`.

### Kanal-Identität — Verdikt B′ gebaut; Register auf `number:`; die fünf B′-Tests im `subset` grün
- **Status:** eigen (Bau fertig; das `ci-gate`-`subset`-Netz **gemessen grün** für die fünf Tests) | **Bindung:** eigen
- **Trigger:** kein — das Netz ist gemessen; offen nur das dimensionslose Prädikat.
- **Lage:** (gemessen 2026-10-14 river-211 via `ci_manage log 38583663935`, head `08d572723`) im `subset` (`cargo test --lib`) laufen die fünf B′-Tests **`ok`**: `the_unit_is_representation_not_identity`, `a_foreign_unit_is_refused_and_never_silently_dropped`, `the_number_token_parses`, `number_and_mass_density_of_one_species_hash_apart`, `number_fick_is_admissible_in_fluid_and_plasma`. Verdikt B′ steht (river-209): Schritt 1 (`unit` aus `ChannelDescriptor::PartialEq`), Schritt 2 (`register_checked` + `ChannelUnitConflict`), Schritt 3 (`Conserved::Number` Token `number`); Mountain-341 hat `phi/sources.φ:613/:614` auf `number:` gezogen und `conserved_for_quantity` (`channel.rs:763`) ausgerichtet. Offen nur das dimensionslose Prädikat (die „konvertierbar"-Prüfung ist konservativ der gleiche Unit-Token; eine Maßtabelle ist `pending`).
- **Blockade:** die früheren `ci-gate`-Läufe waren am fehlenden aacgm-Koeffizienten blockiert und der `subset` danach lange canceled (Concurrency); der Lauf `38583663935` ist der erste, der die fünf Tests wirklich ausführt. Der **gesamte** `ci-gate` bleibt rot (~20 andere Tests + clippy/format — s. `## An mountain`), die fünf B′-Tests sind davon unberührt.
- **Braucht:** nur das dimensionslose Prädikat (Maßtabelle); kein neuer Lauf ohne neuen Grund.

### P10 flache Tokens → Kanal-Deskriptor (`force_id_of`/`FIXED_CHANNELS`/Modul `force` weg) — gated
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains `field`-Zeilen-Migration (`phi/sources.φ`: 7943 flache `field`-Zeilen → Deskriptor-Form, P10.3).
- **Lage:** (gemessen river-199 am Baum) Schritt 1 (Rename, `channel_ref: Option<u8>`) + Schritt 2 (Sentinel 255 → Presence-Bit) gebaut; 57 Dateien; `cargo check` 0/0. Noch `force` im Baum: die Brücke `descriptor_for_force` u. a. (`channel.rs:930-958`), `force.rs`, `FIXED_CHANNELS`, Gate-Re-Exports. Gemessener Riss: Parser-Index vs. Laufzeit-Registry-Index ab dem zweiten Kanal (`f2623bb72`).
- **Blockade:** `phi/sources.φ` trägt noch 7943 flache `field`-Zeilen.
- **Braucht:** nach Mountains Migration den flachen Parser-Branch (`parse.rs:1600`) + die `force_id_of`-Aufrufe streichen und auf `descriptor_from_axes`/`ChannelRegistry::handle_of` ziehen; `cargo check` 0/0; CI-`ci-gate` ist das Netz.

### 14-min-Lücke Magnetometer→Optik — im 2D + Ganznacht gemessen, Pick-Riss getragen
- **Status:** eigen (getragener Riss) | **Bindung:** eigen
- **Trigger:** der GIC-Onset-Relationsschritt auf der Meridian-Spur.
- **Lage:** (gemessen 2026-10-12 river-197 + river-200, Anker `state/river/themis/frame-onset-2026-10-12.md` · `overlay-night-2026-10-12.md`) 2D: kein kohärenter off-meridian optischer Eintritt bei 07:41; der optische Onset ist ein einzelnes azimutal wanderndes Band 07:54:45→07:55:45. Ganznacht-Overlay (CDN-TKDG `themis_asi_timed_keogram_20240510_kapu.bin`): Stundenmittel 03–06 ≈ 3310–3343 (ruhig), 07 = 3370.6, Ränder 02 = 3895 / 08 = 3911; der Hauptsturm liegt 17:00–23:00 UT (Tageslicht). **Riss:** der river-196-Pick „07:41" ist nicht eindeutig — Stunde 07 trägt eine Kette (07:13–07:48), größte YKC-Spitze bei 07:20 (52.4); die „+14 min 27 s" ist pick-abhängig.
- **Blockade:** keine.
- **Braucht:** den Riss tragen (Phasendifferenz + Pick-Abhängigkeit).

### Weberin-Zeugen-Fäden — SABER-Form (C); Athabasca-Zeilen stehen; UCAR-FPI Arm steht; Athabasca-Pfad+cdf-Arm
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** je Faden die `channel`-Quellen-Zeile (Mountain) + Arm/Manifestation (Mycelium).
- **Lage:** (gemessen 2026-10-13 river-204/206/208 · 2026-10-14 river-210) Fakten in `state/river/weberin-faden-facts-2026-10-13.md`, Register-Entwürfe `state/river/weberin-register-entwuerfe-2026-10-13.md`. **CDF-Leser** gebaut (`92346fb85`, `cdf25.rs`); drei Zeugen-Zeilen stehen (`d1e2e28cb`). **SABER-Form (C)** (volle Runde, `state/stimmen/2026-10-13-river-saber-hoehenform-roster.md`): der Level-Index (0…399) ist die geschlossene diskrete Achse, die Höhe die zweite gleich indizierte Koordinate (`altitude(time, level)`); das Level muss Teil des Schlüssels sein. **Athabasca (ERG-SC VLF): die Registerzeilen stehen jetzt** (`phi/sources.φ:402-411`, Mountain-343-1) — per-Bin, kalibriert (`nT^2/Hz`) + roh (`V^2/Hz`); der `cdf`-Arm (`extract.rs:4888` → `cdf25::extract_witness_channels`) trägt sie. **UCAR-FPI:** der `format ucar_fpi`-Arm steht (`400de8ce2`, `src/archivar/fpi_ucar.rs`; Sample `~/Downloads/20246300athabasca.dat`, 10 leerzeichengetrennte Spalten, zwei Zeiten je Zeile). **SABER:** Mountain-345 hat den HAPI-Arm gebaut (Familien-Expansion `param.<i>` über `parameters[].size`, `saber_ktemp_<i>`/`saber_tpgpaltitude_<i>`, Füllwert `-999.0` = absent; Blockform `hapi ktemp.<i>=… tpgpaltitude.<i>=…` + zwei `field`-Zeilen). Das Operator-Wort (2026-10-14) führt die Sonde **autonom** als eigene Weltlinie (`xyz,t`, `Position::StateVector` + `Motion::Linear`), supersediert B2; die Registerzeile folgt nach dem `alt`-Arm. `cdaweb.gsfc.nasa.gov` ist host-weit unerreichbar (Stehender Pass) → das Block-Sample fehlt (mountain-345). **WALDO:** `state/zustand/wartend.φ` Key `waldo-massdaten`.
- **Blockade:** (1) UCAR-FPI: die Instrumentenkoordinate ist **gemessen** (river-213, s. unten) — offen die Register-Zeile (Mountain) + die Manifestation (Mycelium); (2) SABER-Zeile nach Form (C) (Mountain, nach dem HAPI-`alt`-Arm; das Sample fehlt — Host `cdaweb.gsfc.nasa.gov` unerreichbar); (3) je eingefügtem Block die Mycelium-Manifestation.
- **UCAR-FPI Koordinate — gemessen (river-213, Taucher + `archive_search --all`):** `on earth 54.60282 -113.64442 693` — **AUGO II / AUGSO** (Athabasca University Geophysical Observatory, dark site). Quelle `https://augo.athabascau.ca/location/index.php` (Wayback `20221219132851`); gestützt durch die FPI-eigene Publikation (Zhang et al. 2025, JASTP 268, `https://par.nsf.gov/servlets/purl/10617273`: „Athabasca Observatory 54.6°N, 113.6°W, magnetic latitude 61.5°"). **Am Baum verifiziert:** `phi/sources.φ:414` trägt bereits exakt dieselbe Zeile für die ERG-SC-Athabasca-VLF-Quelle — beide Zeugen auf einem Ort. **Riss:** kein Dokument schreibt ausdrücklich „das NCAR/HAO-FPI steht auf AUGO II"; die Identifikation ruht auf der FPI-eigenen 54.6/113.6 (rundet AUGO II, nicht AUGO I 54.714/−113.314) + der Dunkelstandort-Logik. **Gemessen (river-213):** die lokale FPI-Datei `projects/omegaflow-work/data/registration.hao.ucar.edu/20246300athabasca.dat` (7353 Zeilen) trägt **keinen Koordinaten-Kopf** — nur die 10-Spalten-Tabelle (`yyday ut(hr) merid(m/s) merid err dir ut(hr) zonal(m/s) zonal err dir skytemp`); die Koordinate kann also nicht aus der Datei kommen und ruht allein auf AUGO + der Publikation. **`alt`-Datum gemessen (river-213):** SRTM30m (EGM96, orthometrisch) am selben Punkt = **696.0 m** (`api.opentopodata.org/v1/srtm30m?locations=54.60282,-113.64442`) — das AUGO-693 liegt damit auf der **orthometrischen** Skala (die ellipsoidische Hypothese verlangte ≈ 710 m, was SRTM widerspricht). Also `on earth <alt>` hier = **MSL/EGM96**. Weiter divergierende Drittanbieter (OMTI 54.7/246.7, GBDC 54.620/246.650) — benannt, nicht genutzt.
- **Braucht:** (1) UCAR-FPI: die zwei `field`-Zeilen **stehen code-gemessen** (mountain-346), die Koordinate ist **gemessen** — die Register-Zeile schreiben (Mountain) + Manifestation (Mycelium); (2) SABER-Zeile nach Form (C) + Manifestation — der Host `cdaweb.gsfc.nasa.gov` ist intermittierend erreichbar (am 2026-10-14 HTTP 200), das Sample wartet auf die Routen-Entscheidung; (3) Athabasca: Registerzeilen stehen — nur noch die Manifestation (Mycelium); (4) SGO/FMI (`rioData.php`, kein Static-Arm) — Browser-POST; (5) WALDO-Antwort (Trigger).

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
- **Blockade:** das Paritäts-Gate (`field_spatial_fold_wgsl_cpu_parity`, `src/mathematikerin/tests.rs:2620`) läuft erst im CI — der Lauf `38583663935` (head `08d572723`) wurde vor dem `mathematikerin::tests`-Block canceled, das Gate druckte nicht; der Baum ist ohnehin rot (s. `## An mountain`).
- **Braucht:** (a) einen `ci-gate`-`subset`, der das Paritäts-Gate erreicht; (b) **Schritt 3:** die deklarierte `ReceiverAperture` (`receiver.rs`) an die Query-Naht (`vp.surface.w = global_scale`) binden; (c) die Operator-Browser-Messung (Fenster-Edit = Operator-Wort).

### GIC-Onset — AL-Einhüllende + `--null` ausgewertet: präzise, sehr rückrufschwach; der 6-Station-Lauf schlägt den Null
- **Status:** eigen (Bau + Messung) | **Bindung:** eigen
- **Trigger:** kein — die Kette ist verdrahtet und ausgewertet (river-214); die zwei fehlenden Familien laufen.
- **Lage:** (gemessen 2026-10-15 river-214 via `ci_manage` + Job-Logs) die von river-213 angestoßenen Läufe `38642948810` (alte 6-Station-Liste `abk,sod,esk,ler,stj,ups`) und `38643271579` (Familie **auroral**, 31 Stationen) sind **`success`**; `gic_onset_detector` gegen die McPherron-Liste mit `--null 200` (Kreisschub-Surrogate; `prec_ge`/`rec_ge` = Zahl der Surrogate mit Präzision/Rückruf ≥ real, **klein = signifikant**):
  - **1 Station `abk`** (Lauf `38642215979`): 286 288 Samples, Schwelle 50.37 nT/min, 24 582 Onsets (1994–2026); prec/rec 0.156/0.0236 (5 min) · 0.364/0.0658 (15) · 0.533/0.1246 (30) — **kein Null gefahren**.
  - **6 Stationen** (Lauf `38642948810`): 13 150 080 Samples, Schwelle 17.11 nT/min, **579** Onsets (2000–2024); prec/rec 0.123/0.00055 · 0.394/0.00208 · 0.549/0.00371; Null `prec_ge=10/0/0`, `rec_ge=10/0/0` → bei 15 und 30 min schlägt der reale Detektor **alle 200** Surrogate, bei 5 min ist er an der Grenze (10/200).
  - **31 Stationen auroral** (Lauf `38643271579`): 13 626 502 Samples, Schwelle **128.99** nT/min, **50** Onsets (1991–2024); prec/rec 0.220/0.000061 · 0.340/0.000117 · 0.560/0.000240; Null `prec_ge=1/26/7`, `rec_ge=1/19/6` → bei 5 und 30 min signifikant, bei 15 min nicht (p95 0.38 > real 0.34).
  - **25 Stationen subauroral** (Lauf `38690576424`, river-214): 13 150 080 Samples, Schwelle 16.61 nT/min, **4** Onsets (2000–2024); prec/rec 0.250/0.000008 · 0.750/0.000023 · 1.000/0.000046; Null `prec_ge=61/12/2`, `rec_ge=61/27/10` → bei 15 min **nicht** über der Null, bei 5/30 min an der Grenze — die subaurorale Hülle ist nahezu stumm (4 Onsets in 25 Jahren).
  - **Riss benannt (ungeglättet):** die Netzhülle detektiert viel weniger Onsets als die Einzelstation (rec 0.002–0.004 vs 0.066–0.125) — mehr Stationen heben die Detektorschwelle (6 Stationen 17.1 → 31 auroral 129.0 → subauroral nahezu stumm). Die wenigen Netzfunde liegen teils über dem Kreisschub-Null (6 Stationen: alle 200), teils nicht (subauroral); der Einzelstations-Lauf ist **nicht** gegen den Null getestet — der Vergleich ist noch nicht äpfel-gleich.
- **Blockade:** keine — der midlat-Lauf (`38690576605`, 98 Stationen) läuft noch im CI.
- **Braucht:** (a) den laufenden midlat-Lauf auswerten; (b) **Operator-Wort 2026-10-15:** die Einzelstation gegen die Null prüfen und die Kurve-/Sweet-Spot-Frage messen — dazu ist `gic_onset_detector --sweep-k` um `--null` erweitert (jeder Schwellepunkt trägt seine Surrogat-Zahl) und in `gic-al-network.yml` ein Sweep über `k=1,1.5,2,3,4,6,8,12` für die Einzelstation (`abk`) und die Familien-Einhüllende verdrahtet; `gic-onset.yml` fährt die Einzelstation jetzt mit `--null 200`. Der Lauf folgt dem Commit.

### GIC-Familien-Partition — deklariertes Artefakt gebaut; die zwei Register-Nägel offen
- **Status:** eigen (Artefakt gebaut) | **Bindung:** eigen (Rest: Mountain-Nägel)
- **Trigger:** die zwei Register-Nägel sind gesetzt (Mountain), dann Run der Matrizen.
- **Lage:** (gemessen `state/stimmen/2026-10-06_gic-familien-arm-ui-stimmen.md:20-80`) die Stimmen-Runde einig: kein neuer Grammatik-Arm, sondern (i) ein Satz über Querfamilien-Zellen und (ii) die Familienregel als deklariertes `station→Familie`-Artefakt. **Gebaut (river-213):** das Artefakt steht — `docs/zustand/gic-station-families.tsv`, konsolidiert aus den drei Deskriptoren `src/mathematikerin/descriptors/gic_{auroral,subauroral,midlat}.te`; Partition **154 = 31 auroral + 25 subauroral + 98 midlat** (gemessen `cut -f2 | uniq -c`). `gic-al-network.yml` liest es (Familien-Netz). Die drei Matrizen `gic_auroral|gic_subauroral|gic_midlat` existieren als `.te`-Deskriptoren (`matrix gic_* full`, `expect cells 930/600/9506`).
- **Blockade:** die zwei Register-Nägel (Mountain) — der Grammatik-Patch (`family … stations …` / `expect families 3`) ist nicht mehr nötig, die Familien sind über die `.te`-Kanal-Listen deklariert.
- **Braucht:** Mountain setzt die zwei Nägel; danach `matrix gic_auroral|gic_subauroral|gic_midlat full` über `field-te-query.yml` fahren.

### Newell-OMNI-Alignment — Zelle gemessen (n=22137), TE silent
- **Status:** eigen (gemessen, silent) | **Bindung:** eigen
- **Trigger:** kein — die Relation ist gesetzt (river-212).
- **Lage:** (gemessen 2026-10-13 river-209 via `gh run download 38464793874 -n field-te-matrix-newell-omni`) der Lauf `38464793874` (HEAD `fd1d693e8`) ist **success**; `matrix-newell-omni`: `expect cells 1 matched the declared 1 cells`, Zelle `newell_dphi_dt_omni->abk_dbdt_pt1h` **n = 22137** (n_eff 1.2298e3, native rung 3600 s), TE 7.7953e-5, p 1.0000 → **verdict silent**; `fdr bh q 0.05`: **0 of 1 cells pass**; kein n = 0 mehr. Die frühere Disjunktheit (`38378918382` n = 0) ist mit dem gezogenen 1h-Schlüssel (`abk_dbdt_pt1h`, mountain-340 `05e215aa1`) **gelöst** — die Zelle ist co-temporal. Riss-Hinweis (Mountain-340): die Kanal-Identität bleibt `intermagnet_dbdt_abk` (aus `fc.name`) — sollen 1h/1m je ein eigener Kanal sein, braucht es eine `name`-Änderung + extract-Arm-Anpassung.
- **Relation zum Live-Treiber — gesetzt (river-212, gemessen via `gh run download 38464793874` gegen den zweiten Artefakt `field-te-matrix-newell`, selber Lauf, success):** der **Live-Treiber** `newell_dphi_dt->intermagnet_dbdt` ist **nicht messbar** — `arms measured 0 of 2`: der `newell_dphi_dt`-Arm hängt am Host (`cdaweb.gsfc.nasa.gov` AC_H2_MFI fetch void), und der `intermagnet_dbdt`-Arm trägt den **Identitäts-Riss** (1m-`abk_dbdt_1m.bin` median_dt 60 s vs 1h-`abk_dbdt_1h.bin` median_dt 3600 s, Überlappungs-Korrelation 0.9388; kein Arm gewählt, beide als zwei Knoten) → Zelle n = 0, `floor (p = 1)`. Die **Verifikation** (OMNI-Proxy) ist gemessen und still. **Beide sagen dasselbe: kein gemessener Pfeil** — die Verifikation rettet den Live-Treiber nicht, und der Live-Treiber ist ohnehin blockiert (Host + Riss). Der eigentliche offene Punkt ist der **1m/1h-Identitäts-Riss** des `dB/dt`-Kanals.
- **Blockade:** der Riss (1m vs 1h) und der Host; kein neuer Lauf ohne neuen Grund.
- **Braucht:** nichts Neues von River; der Riss des `dB/dt`-Kanals gehört als eigener Punkt getragen (Mountain/extract: `name`-Trennung 1m/1h) — s. Riss-Hinweis oben.

### Flyby-Kette — recon bleibt (`ephemeris-juice-recon` + `flyby-path2-recon`)
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** `ephemeris-juice-recon` Wiedervorlage 2026-11-01 (ESA/ESOC publiziert die Post-Flyby-SPK). Wahrheit: `state/zustand/wartend.φ`.
- **Lage:** (gemessen 2026-10-11) die Wiedervorlage steht; `data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin` absent. `flyby-path2-recon`: orbc-SPK keyless (HTTP 206); die 1-σ-Kovarianz liegt nicht im SPICE-Baum.
- **Blockade:** ESA/ESOC-recon-SPK + die 1-σ-Kovarianz absent.
- **Braucht:** am 2026-11-01 `archive_search --verdict` auf den ESOC-recon-Pfad; dann `flyby_ephemeris_gate --recon <recon.bin> --sigma-recon <km>`.

### Schumann-Resonanz — Trigger gefeuert (CDN-Asset liegt); Matrix gebaut, Lauf nach dem Commit
- **Status:** eigen (Trigger gefeuert; Bau fertig) | **Bindung:** eigen
- **Trigger:** die Reihe `resonance_schumann.csv` (f1/f2/f3) liegt im CDN → **gefeuert** (gemessen 2026-10-15: `curl -I …` = HTTP 200).
- **Lage:** (gemessen 2026-10-15 river-214) die Quelle ist registriert (`phi:1057-1068`, CC-BY-4.0, Zenodo 15225820, `sha256 3183a14f…`, `harvest raw_zenodo`); der CDN-Abzug antwortet jetzt HTTP 200. Als **Nadel ⅩⅣ** steht die Antriebsfrage im Rätsel-Register (`docs/concepts/kybernetische-astrophysik.md:368`): „Regelt geomagnetische Aktivität die Schumann-Frequenzen sichtbar, und in welcher Richtung?" gegen die Blitz-/Intensitäts-Erklärung. **Gebaut (river-214):** der Matrix-Deskriptor `src/mathematikerin/descriptors/schumann_geomag.te` (`matrix schumann_geomag rect`, Driver `abk_dbdt_pt1h` — Kp/Dst/AE sind **nicht** im Register, gemessen — , Targets `freq1,freq2,freq3`, `expect cells 3`, `surrogate 10`) und der Job `matrix-schumann` in `.github/workflows/field-te-query.yml`. Kanal-Schlüssel verifiziert: die `field`-Zeile trägt den Schlüssel im ersten Token (`parse.rs:1745` `key: parts[1]`) — daher `freq1/2/3`, nicht der Name `resonance_schumann_f*_hz`.
- **Blockade:** `workflow_dispatch` liest den **entfernten** Ref — der Lauf folgt dem Commit des Deskriptors/Workflows (kein Vorlauf im selben Atom), und `field-te-query` feuert die **ganze** 14-Job-Suite, nicht einen Job.
- **Braucht:** (1) **Rat** vor dem Lauf: der Konfund „globales Blitzgeschehen treibt Schumann **und** die Ionosphäre" — ohne einen Blitz-Kanal kann die reine TE(abk→f) den Antrieb nicht vom Konfund trennen (der Punkt nennt genau diesen Riss); (2) dann den `matrix-schumann`-Lauf über `field-te-query.yml` mit der Surrogat-Null.

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

Origin: river-211

**CTBTO — Formular vorbereitet (Operator-Akt = Absenden).** Mycelium meldet das Formular vorbereitet; das Absenden ist Operators Hand.

**Weberin-Zeugen — Manifestation der neuen Zeugen.** Die `url`/`origin`/`compiler`-Direktiven der Weberin-Zeugen gehören Mycelium; sie sind gated: SABER keyless via HAPI, UCAR-FPI E-Mail-gated (Operator, erledigt), Athabasca ERG-SC keyless. **Athabasca:** die `channel`/`field`-Zeilen stehen (`phi/sources.φ:402-411`) — die Manifestation ziehen. **SABER/UCAR-FPI:** sobald die `field`-Zeile steht (`## An mountain`), die Manifestation ziehen. xfer:river208-weberinmanifest1.


**Register→Manifest — Bau-Wort (mit Toren).** Operator-Wort „ja bitte" und „ja bau es allerdings auch mit den sonstigen gates etc also lizenzprüfung z.b." (2026-10-13). Der Verbinder **existiert** (`harvest-dispatch.yml` feuert auf `push` auf `phi/sources.φ`); der Defekt ist die **Doppelbedeutung von `format`**. **Erste Messung:** 2565 Blöcke; 636 `format reference`; 1239 mit eigenem Compiler; generisches Format (csv/json/cdf/hapi) 23, davon **19 ohne Compiler**. **Kleinster Fix:** (1) eine ausdrückliche Manifest-Zeile im Block (`harvest raw_zenodo` / `manifest auto`); (2) **ein** generischer Aktor (`# harvest: format=csv arm=raw_mirror …`). **Tore (Operator-Wort):** Lizenz-Tor, Fingerabdruck-Tor, Format-Tor, Registrierungs-Tor. xfer:river208-registermanifest1.

## An mountain

Origin: river-211 (2026-10-14)

**`ci-gate` am HEAD rot — river-211 hat ihn übernommen (Operator-Wort 2026-10-14: „mountain ist beschäftigt kümmer dich selbst darum").** Gemessen (river-211): **kein grüner `ci-gate`-Lauf** zwischen 2026-10-13T20:20Z und HEAD (API-Abfrage der `ci-gate`-Runs — alle `failure`/`cancelled`). Am HEAD `7183bd332` (`ci_manage view/jobs/log 38589073902`) rot: `clippy` (2 Lints — `src/archivar/cdf.rs:682` `chunks_exact(2)` → `as_chunks`, `:700` `!(x1 > x0)` → `partial_cmp`; Issue „health: cargo clippy returned void" offen), `format` (13 unformatierte Dateien, exakt die im Arbeitbaum schmutzigen — `ampere_fac/bpa_gic/fdsn_event/mtg_li/pcn/superdarn_cpcp/swarm_fac/swpc_efield/usgs_geomag.rs`, `tools/harvest/.../fdsn_event_compiler.rs`, `tools/measure/.../cdf_vars.rs`, `nobel_probe_corona.rs`), `dropped-gate`, `register`; `build` success. Der `subset` (`cargo test --lib`, head `08d572723`, `ci_manage log 38583663935`) trägt **~20 rote Tests** — `archivar`: `test_epa_aqs_source_registered`, `test_flush_port_block_carries_pending_review_marker`, `test_law_bounds_enclosure_span_holds_for_periapsis_in_window`, `test_load_gate_clips_records_outside_enclosure`, `test_parse_rows_verbatim_header_key`, `test_port_block_with_force_and_name_unit_synthesizes`, `test_port_block_with_force_and_no_unit_stays_pending`, `test_port_block_without_force_undetermined_stays_pending`, `test_port_convert_celestial_and_post`, `test_port_convert_ra_dec_key`, `test_query_admits_surface_sample_within_window`, `test_raw_presence_gate_admits_within_reach_and_refuses_fieldless_source`, `test_rows_per_row_lat_lon_and_date_epoch`, `vlf_awesome::the_stamp_converts_through_the_leap_second_table`; `mathematikerin`: `actuators::a_zero_sum_does_not_collapse_the_channels`, `channel::a_resolved_friction_exchange_is_antisymmetric_through_density`, `machines::matrix_rebuild_...rebuild_emits_astro_channels_for_all_bodies_including_gm_less`, `mci::te_binned_ci_finds_the_direction`, `pc::chain_skeleton`, `pc::confounder_screened_pair`, `receiver::nutation_matrix_n06a_matches_sofa_reference` — die Gründe druckte der Lauf nicht (canceled by concurrency), nur die `FAILED`-Marker. **Grün** in demselben Lauf: die fünf Kanal-Identität-B′-Tests und der AACGM-Zeuge (`archivar::aacgm::tests::upstream_c_test_witnesses_reproduce_through_two_epoch_interpolation` u. a.). Die river-210-Heilung (aacgm) hat den `subset` erstmals seit langem wieder die Lib-Tests ausführen lassen — der rote Stand ist damit **neu sichtbar**, nicht neu entstanden. HEAD zog während dieses Passes auf `01f56ff15` (Mountain-346: `pds3_img` BIP/BSQ/BIL, folge345 → `archiv/`); die 13 Format-Dateien bleiben im Arbeitbaum schmutzig und `src/archivar/cdf.rs:682/:700` unverändert → der Befund hält am neuen HEAD. xfer:river211-cigatered1.

**river-211 hat daran gearbeitet (Commit `1a10f0f3c`).** Zwei **echte** Bugs, die der `subset` freilegte: (1) `receiver.rs` `rx()` rotierte um die **falsche Achse** (`R2`/y statt `R1`/x) — die SOFA-Referenz in `nutation_matrix_n06a_matches_sofa_reference` stimmt seit der Korrektur exakt (vorher: Transponierte/andere Matrix); das korrigiert zugleich `precession_matrix_fw06` und `cirs_to_gcrs` (CIRS→GCRS-Frame). (2) `membrane.rs` `medium_reach()` gab einem **unbekannten** Kanal (channel_ref 9) bei `age == 0` eine Reach `Some(0.0)` — die Descriptor-Suche steht jetzt **vor** dem age-Kurzschluss, ein Kanal ohne Fortpflanzungsgesetz liefert `None` (der Record bleibt roh). Dazu die **veralteten** Test-Erwartungen an den Baum gezogen: `channel_ref` `Some(6)`→`Some(3)` (Kanal-Index-Rebase river-199; `diffusion` = FIXED_CHANNELS[3], `electric` = 6), die drei `port_block`-Tests (Force-Gate geschlossen: `pending`→`declined`; `quake_depth`/`omni_bx_gsm_nt` jetzt register-aufgelöst: `km 10`/`nT 60`), die zwei `port_convert`-`at sun`-Asserts (Frame wird nicht mehr erfunden → `cmap .`), der `vlf`-Stamp (TDB-Sekunden seit J2000 `473342767.184`, nicht Unix).
**Zweiter Atom (`pc.rs`, `channel.rs`, Commit folgt).** Drei weitere **echte** Bugs/Testbrüche: (3) `pc_stable_skeleton_screened` brach bei `order == 0` ab, wenn kein Paar entfernt wurde — dadurch lief die **Konditionierung nie** (`chain_skeleton` behielt die falsche Kante `(0,2)`, deren Partialkorrelation p = 0.60 ist; `confounder_screened_pair` screen-te nichts). Der `if !removed_any { break; }` ist gestrichen; das Raster läuft bis `max_cond` (beide Tests grün im Probe). (4) `channel::a_resolved_friction_exchange_is_antisymmetric_through_density` baute **zwei identische** Descriptoren (gleicher Hash → `index_of` 0/0, `phi[j]-phi[i] = 0`) — der Test benutzt jetzt `with_species("ion")`/`with_species("neutral")` (die Art-Achse river-205), die Hashes trennen sich, `from_delta = K·rho_n/(m_i+m_n)·(phi[1]-phi[0])` trifft exakt.
**Offen (subset):** drei `archivar`-Tests (`test_law_bounds_enclosure_span_holds_for_periapsis_in_window`, `test_query_admits_surface_sample_within_window`, `test_raw_presence_gate_admits_within_reach_and_refuses_fieldless_source`) und drei `mathematikerin`-Tests (`actuators::a_zero_sum_does_not_collapse_the_channels`, `machines::…rebuild_emits_astro_channels_for_all_bodies_including_gm_less`, `mci::te_binned_ci_finds_the_direction`) — je als eigener Punkt zu messen. Beim `actuators`-Test ist der Befund bereits klar (`acoustic_pcm` pannt nach dem **Vorzeichen der Σω**; Σω = 0 → beide Kanäle 0 = Stille — der Test will die Partiale erhalten; das ist eine **Richtungs-Entscheidung** für den Rat, kein stiller Fix). Der `subset` läuft zudem 30 min in `timeout-minutes: 30` (`ci-gate.yml:65`) und wird **vor** dem `failures:`-Block gekappt — die roten Namen sind Sichtbar, die Gründe nie.

Origin: river-212 (2026-10-14)

**`ci-gate` am HEAD `e2a28dcd3` — `clippy` grün; `format` + `dropped-gate` in diesem Atom geheilt; `register` bleibt bei Mountain.**
Gemessen `ci_manage view/jobs/log 38595147942` (HEAD `e2a28dcd3`): `clippy` **success** (river-211s `cdf.rs`-Lints), `build` success; `format`/`register`/`dropped-gate` failure; `subset` in_progress.
**`format` (12 Dateien):** die CI-Diffs sind **exakt** die im Arbeitbaum schmutzigen Rust-Dateien; `rustfmt --edition 2024 --check` über alle 12 → **exit 0** (der Arbeitbaum trägt die Format-Lösung). Ursache: Rivers `force`→`channel_ref`-Rename (river-198 `59b31dc1c`) und spätere Umformungen drückten die `force_id_of("electric")`-Assertions über 100 Spalten. **Geheilt (river-212):** die mechanische rustfmt-Ausgabe committet (`src/archivar/{ampere_fac,bpa_gic,fdsn_event,mtg_li,pcn,superdarn_cpcp,swarm_fac,swpc_efield,usgs_geomag}.rs`, `tools/harvest/.../fdsn_event_compiler.rs`, `tools/measure/.../{cdf_vars,nobel_probe_corona}.rs`) — `git diff` rein Format, keine Semantik. **Riss (gemessen 2026-10-14 river-212):** der `format`-Job riss am head `d3c71b95d` erneut — an einer **fremden** Datei (`tools/service/src/bin/llm_gateway.rs:1244`); das crate-weite `cargo fmt --check` ist unter parallelen Commits ein bewegliches Ziel (jede unformatierte Fremd-Commit-Landung rotet es erneut). Nicht River-eigen.
**`dropped-gate`:** `current 1 | pinned 86 | new 1` — der Neue-Drop `prosa-träger mountain327-abgabe aacgm-koeffizient geheilt zeuge läuft` ist die von river-211 erledigte + gelöschte Mountain327-Prosa-Träger-Zeile (AACGM-Zeuge im `subset` grün, `f54787bb1`); gemessen `register_lookup --dropped-keys --changed-since a8f871476`. **Geheilt (river-212):** in `docs/zustand/dropped-legacy-baseline.txt` mit Beleg-Kommentar eingetragen — kein stiller Drop. Zwei weitere Neue-Drops am head `f49586c62` (Lauf `38602958578`, `new 2`) — `saber höhenform c arm höhen-ablage brauchen` und `epochen-absenz zwei linien unvereinheitlicht eigen` — sind Handover-Churn der Mountain-Linie folge345→346 (in folge346 unter Route B bzw. Rats-Verdikt b weitergeführt); ebenfalls mit Beleg gepinnt, kein stiller Drop.
**`register` (bleibt rot):** `cdn_reconcile --fail` — 5 CDN-Workflows schreiben Netlocs, die nicht in `phi/sources.φ` gebunden sind: `babamul-cdn.yml:37` (`babamul.caltech.edu`), `celestrak-eop-cdn.yml:41` (`celestrak.org`), `eve-cdn.yml:57` (`lasp.colorado.edu-eve`), `mars-dust-cdn.yml:37` (`vo.lmd.jussieu.fr`), `maxi-cdn.yml:24` (`maxi.riken.jp`). Register-Zeilen = Mountain.
**`subset` (rot — Wurzel gemessen):** der Job lief `17:07:47→17:38:05` = **30:18** und wurde am Timeout gekappt, **bevor** die `failures:`-Zusammenfassung druckte. Ursache: **schwere TE-Kalibrierungstests laufen un-`ignored`** im `cargo test --lib` — gemessen (`has been running for over 60 seconds`): `calibration_self_null_discriminator_white_driver_stays_at_the_floor` (~9 min), `gate_fpr_autocorrelation_block_null_binned_n_surr_200` (~12 min), `calibration_fp_independent_ar1_stays_near_chance`, `gate_conditional_embedded_fn_true_coupling_is_found`, `gate_conditional_embedded_fp_common_driver_stays_near_chance`, `gate_fpr_autocorrelation_{arx,block_ksg,coherent_phase,restricted}_null_binned_n_surr_200`. Die `#[ignore]`-Konvention („runs in te-gate.yml") ist lückenhaft. Der Log trägt die roten **Namen** (28: u. a. `archivar::projection::tests::*`, `archivar::port::probe_classify_tests::*`, `archivar::parse::tests::{the_register_declares_a_channel_for_every_live_force_tag}`, viele `archivar::tests::*`, `mathematikerin::{actuators,machines,mci}::*`), aber nie die Gründe. **Geändert (river-212, `ci-gate`-Revier, Operator-Wort 2026-10-14):** `ci-gate.yml`-`subset` — Timeout `30`→`60`, `cargo test --lib -- --nocapture` (die Panik-Meldung druckt im Moment des Fehlschlags, nicht erst in der gekappten Zusammenfassung). **Folge-Schritt:** die schweren Tests nach `te-gate.yml` ziehen (die `#[ignore]`-Lücke schließen), damit der `subset` schnell und grün sein kann.

Origin: river-210

**Kanal-Identität B′ — Register-Re-Deklaration + `conserved_for_quantity` erledigt.** `Conserved::Number` (Token `number`) steht in `src/mathematikerin/channel.rs`; Mountain-341 hat `phi/sources.φ:613/:614` auf `number:…` gezogen und `conserved_for_quantity` (`channel.rs:763`) ausgerichtet. Kein offener Riss mehr an dieser Stelle. xfer:river.ses_ec901ad7dffehLKKusa1DO806h.3~1b6c1cbf.

**UCAR-FPI — `field`-Komponenten-Zuordnung.** Der `format ucar_fpi`-Arm steht (`400de8ce2`, `src/archivar/fpi_ucar.rs` + Dispatch `extract.rs:159`); das gemessene Sample (10 leerzeichengetrennte Spalten, Kopf `yyday ut(hr) merid(m/s) merid err dir ut(hr) zonal(m/s) zonal err dir skytemp`) wird geparst. **Braucht:** die zwei `field`-Zeilen `fpi_mer_wind_ms` (merid-Spalte) und `fpi_zon_wind_ms` (zonal-Spalte) samt `series_component_name`-Zuordnung. xfer:river208-ucarfpi1.

**SABER-Zeugen-Zeile nach Form (C) — Höhe als Koordinate auf geschlossener Level-Achse.** Verdikt (C) (Rat + volle Roster-Runde, `state/stimmen/2026-10-13-river-saber-hoehenform-roster.md`): der Level-Index (0…399) ist die **geschlossene diskrete Achse**, die Höhe eine zweite, gleich indizierte Koordinate (`altitude(time, level)`); das Level muss Teil des Schlüssels sein — kein freies `field` je Höhenstufe. Entwurf: `state/river/weberin-register-entwuerfe-2026-10-13.md` Block 5. Mountain-342-6: die Registerzeile folgt nach dem HAPI-Arm (dotted `<i>` + `alt`). xfer:river208-saber1.

**Fehlende Fäden — Registerzeilen-Entwürfe.** Gemessen (river-208/209): EPTA DR2 (Zenodo `8300645`, CC-BY-4.0; **25 Pulsare, nur TOAs, keine Residuen** — ein Residuen-Feld braucht erst ein Residuen-Produkt, Vorlage `field … inverse-square gravity s <τ>`, `phi/sources.φ:12633`) · KiDS DR4.1 WL (WL-Spalten `RAJ2000/DECJ2000,e1/e2,Z_B,weight`; Haupt-FITS 16,5 GiB) · Conicus3D (eine Datei `…phase_metadata.tar.zst`, 34,4 GB — `pending: 34-GB-Sample`) · WALLABY/CGPS (kein exaktes HiPS3D-Produkt — `pending: Produktname`). **Schon im Baum** (Entwurf stale): 1LHAASO `28305-28321` · Auger Zenodo `10488964` `29374-29378` · HMI/JSOC `terms CC0-1.0` `2026` · Pantheon+ SH0ES `MU_SH0ES` `29177-29185`. Entwürfe: `state/river/missing-threads-register-entwuerfe-2026-10-13.md`. Braucht: die `field`-Spalten-Zuordnung (WL-Shear, EPTA-Residuen, Aktivitätsindex) und `terms`/`ttl`. xfer:river208-faeden1.

Origin: river-213 (2026-10-14)

**UCAR-FPI Athabasca-Instrumentenkoordinate — gemessen** (Taucher + `archive_search --all`; der Befund ist am Baum verifiziert). `on earth 54.60282 -113.64442 693` — **AUGO II / AUGSO** (dark site); Quelle `https://augo.athabascau.ca/location/index.php` (Wayback `20221219132851`), gestützt durch die FPI-eigene Publikation (Zhang et al. 2025, JASTP 268, `https://par.nsf.gov/servlets/purl/10617273`: „Athabasca Observatory 54.6°N, 113.6°W, magnetic latitude 61.5°"). Am Baum: `phi/sources.φ:414` trägt bereits dieselbe Zeile für die ERG-SC-Athabasca-VLF-Quelle — beide Zeugen auf einem Ort. **Riss:** kein Dokument schreibt ausdrücklich „das NCAR/HAO-FPI steht auf AUGO II" — die Identifikation ruht auf der FPI-eigenen 54.6/113.6 (rundet AUGO II, nicht AUGO I `54.714/−113.314`) + der Dunkelstandort-Logik; **gemessen (river-213):** die lokale FPI-Datei `omegaflow-work/data/registration.hao.ucar.edu/20246300athabasca.dat` (7353 Zeilen) trägt **keinen Koordinaten-Kopf** (nur die 10-Spalten-Tabelle) — die Koordinate kann nicht aus der Datei kommen. **`alt`-Datum gemessen (river-213):** SRTM30m (EGM96, orthometrisch) am selben Punkt = 696.0 m (`api.opentopodata.org/v1/srtm30m`); das AUGO-693 ist damit **orthometrisch (MSL/EGM96)** (die ellipsoidische Hypothese verlangte ≈ 710 m). Der Haus-Bezug `on earth <alt>` (offen, Mycelium-316) sollte als **MSL/EGM96** deklariert werden. **Für die Register-Zeile des FPI-Blocks** (`on earth 54.60282 -113.64442 693`) — Mountain.

## An future

Origin: river-206

**UCAR-FPI (Luftleuchten-Wind, Weberin-Zeuge) — freie E-Mail-Registrierung, Operator-Hand.** Der Datenpfad `https://registration.hao.ucar.edu/hao-reg_email.php?file=…20246300athabasca.dat&…` liefert die Registrierungsseite (HTTP 200 `text/html`); die `.dat` kommt nach Absenden der E-Mail-Adresse. **Im Operator-Browser ist die Registrierung erledigt** — die Datei liegt (`~/Downloads/20246300athabasca.dat`), der `format`-Arm steht. Kein offener Operator-Akt mehr. xfer:river206-ucarfpi1.

## Abschluss

river-214 (ein Pass; kein Fenster-Edit, kein Send; LOCK bleibt — Operator-Wort). Fälligkeit bei
Pass-Öffnung (HEAD `d512d8fee`): `--fired river` = 1 (die Große-Rätsel-Kandidatenliste,
operator-gebunden — nicht erneut vorgelegt), `--stale river --persist 3` = 0, `--addressed river`
= 2 (mountain-folge348, mycelium-folge350 — gefaltet) · `open_points_check` F212 = 0
stale-citations (21 Pfad-Refs absent — der private Baum liegt außerhalb des öffentlichen) ·
`session_burn` nicht gemessen (`db absent`).
**Adressierte Blöcke gefaltet:** mountain-348 (`xfer:mountain-348-7` SABER — Registerzeile folgt
nach der Routen-Entscheidung B + A; Host intermittierend erreichbar) · mycelium-350
(intermagnet-XYZ **erfolgreich**; DE-2 unverändert am Host; ENTSO-E und Schumann am Baum
erfüllt — verifiziert: ENTSO-E-Block `phi:11836-11843` + Compiler-ZIP-Fix `52312fade`,
Schumann-`sha256` `phi:1062` + CDN HTTP 200; die zwei `## An mycelium`-Blöcke gelöscht).
**GIC-Onset ausgewertet (river-214):** die zwei von river-213 angestoßenen AL-Netz-Läufe sind
**`success`**. 6-Station-Lauf `38642948810`: 579 Onsets, prec/rec 0.394/0.00208 (15 min) — der
reale Detektor schlägt **alle 200** Kreisschub-Surrogate (`prec_ge=0`). auroral-31-Lauf
`38643271579`: 50 Onsets, prec/rec 0.340/0.000117 (15 min) — nicht über der Null (`prec_ge=26`).
Der Einzelstations-Lauf `abk` hat den viel höheren Rückruf (0.066–0.125), ist aber **nicht**
genullt. Riss benannt: mehr Stationen → höhere Schwelle → weniger Onsets. Die zwei fehlenden
Familien (`38690576424` subauroral, `38690576605` midlat) laufen.
**Schumann (river-214):** der Trigger ist gefeuert (CDN-Abzug HTTP 200). Deskriptor
`src/mathematikerin/descriptors/schumann_geomag.te` + Job `matrix-schumann` gebaut; der Lauf folgt
dem Commit (Dispatch liest den entfernten Ref) und braucht den Rat (Blitz-Konfund).
**ci-gate/register-coverage:** nicht Teil dieses Passes; der Baum bleibt rot (zuletzt `format` an
der **fremden** Datei `tools/service/src/bin/llm_gateway.rs` — kein Linien-Eigentum; `register`
= Mountain-Netlocs).
**Commit:** diese Übergabe (`folge213` neu, `folge212` → `archiv/`), der Schumann-Deskriptor und
der `matrix-schumann`-Job. Fremdes ungespeichertes (`docs/specs/cdn_reconciliation.json`,
`ngl_*.{sta,txt}`, `data/`/`state/`) bleibt unangetastet.
