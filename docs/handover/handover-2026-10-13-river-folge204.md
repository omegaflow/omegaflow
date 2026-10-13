<!--
title: Handover — River-Folge 204
class: handover
date: 2026-10-13
sha256: 1bc4745b6b6d68b35cbed562ff02c22df2acb3ca17ab08229e874b0211ef11ae
status: live
-->
# Handover — River-Folge 204 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; pfad-begrenzt committen.

## Burn: open 0.00 · close 0.10 · cap 0.50 — Grund: session river-204 (deepseek-flash, kein pro/max); 1 `grind-flash` (Weberin-Fakten) + 1 `general` (ENTSO-E) + der GPU-Kopplungs-Bau selbst gebaut; die close-Zahl ist der gemessene `session_burn`-Wert der River-Zeile. Fälligkeit: `--fired river` = 1 (Newell-Lauf `38378918382` queued), `--stale river --persist 3` = 0.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„ja du bist die frische sitzung" | 2026-10-13 | Operator (Session, River 204) — die zwei großen Bauten (Kopplungs-GPU-Seite + Parität, Zeugen-Registerzeilen) in dieser frischen Session arbeiten, nicht als Taucher-Auftrag an die Vorsession.
„nein ich sage der lock wird erst aufgehoben wenn das frühwarnsystem fertig und kalibriert ist" | 2026-10-13 | Operator (Session, River 204) — LOCK bleibt; die zwei Siegel-Punkte (wy-max-t, Blatt-Kalibrierung) werden **nicht** als erledigt gestrichen; der LOCK fällt erst, wenn das Frühwarnsystem fertig und kalibriert ist.
„das kannst du direkt prüfen" | 2026-10-13 | Operator (Session, River 204) — den ENTSO-E-HTTP-400 direkt prüfen (Token oder Parameter).

Verbatim: `state/operator-gespraeche/2026-10-13-river.md`.

## Träger (Prosa, eigene)

- `docs/concepts/kanal-ontologie-komplettbau.md` — der Bauplan (P0–P10); P10.2a (Parser-Arm) gebaut.
- `docs/concepts/archivar-mathematikerin.md` — der Wire/GPU-Force-Vertrag; Träger des kinetischen Rahmens, der **Receiver-Apertur** (`:34`) und des **Schneidungs-Gesetzes**.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`status: unsealed`) — Präregistrierung der Vorhersage-Zelle; Siegel = Operator-Wort (LOCK).
- `docs/paper/gic-causal-driver.md` — die Punktschätzer-Unsicherheit auf das studentisierte Moving-Block-Bootstrap (`:171`).
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` — die Zellen stehen `pending` (nach dem Flyby); Rest = 1-σ-Kovarianz.
- `docs/surveys/survey-2026-10-12-standort-lokalisierung-ortho-dsm.md` — Standort-Lokalisierungs-Landkarte + `dsm − dtm`.
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

### Prosa-Träger mountain327-Abgabe — Marker abgearbeitet, Rest offen
- **Status:** eigen (Carrier-Duty) | **Bindung:** eigen
- **Trigger:** kein — die Träger stehen.
- **Lage:** (gemessen 2026-10-13 river-205 via `git log c14fb3298`/`5f44d0980`) **AACGM-v2-Loader gebaut** (`c14fb3298`, `5f44d0980`; Brackets + Zwei-Epochen-Interpolation + CI-Zeugentest, lokal nicht lauffähig). **CTBTO:** die Antragsroute gemessen (`vdec.ctbto.org` 404 bestätigt, das Portal lebt; Weg über das Webform + Vertraulichkeitsvertrag; Zeitschiene ungemessen). **BGR-Ersatzroute** (PMCC-Detektionslisten).
- **Blockade:** keine.
- **Braucht:** (a) den CI-Lauf des AACGM-Zeugen (`ci_manage log`); (b) die CTBTO-Einreichung (Operator-Hand; Antrag liegt privat); (c) die BGR-Ersatzquellen (Mycelium, §An mycelium).

### 14-min-Lücke Magnetometer→Optik — im 2D + Ganznacht gemessen, Pick-Riss getragen
- **Status:** eigen (getragener Riss) | **Bindung:** eigen
- **Trigger:** der GIC-Onset-Relationsschritt auf der Meridian-Spur.
- **Lage:** (gemessen 2026-10-12 river-197 + river-200, Anker `state/river/themis/frame-onset-2026-10-12.md` · `overlay-night-2026-10-12.md`) 2D: kein kohärenter off-meridian optischer Eintritt bei 07:41; der optische Onset ist ein einzelnes azimutal wanderndes Band 07:54:45→07:55:45. Ganznacht-Overlay (CDN-TKDG `themis_asi_timed_keogram_20240510_kapu.bin`): Stundenmittel 03–06 ≈ 3310–3343 (ruhig), 07 = 3370.6, Ränder 02 = 3895 / 08 = 3911; der Hauptsturm liegt 17:00–23:00 UT (Tageslicht). **Riss:** der river-196-Pick „07:41" ist nicht eindeutig — Stunde 07 trägt eine Kette (07:13–07:48), größte YKC-Spitze bei 07:20 (52.4); die „+14 min 27 s" ist pick-abhängig.
- **Blockade:** keine.
- **Braucht:** den Riss tragen (Phasendifferenz + Pick-Abhängigkeit).

### Weberin-Zeugen-Fäden — Fakten gemessen, Register-Zeilen-Bau offen (Gates + CDF-Arm)
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** gefeuert (Operator-Wort „alle") — je Faden die `channel`-Quellen-Zeile (Mountain) + Arm/Manifestation (Mycelium).
- **Lage:** (gemessen 2026-10-13 river-204 via `grind-flash`-Taucher + `vision`; `state/river/weberin-faden-facts-2026-10-13.md`) die per-Faden-Fakten sind gemessen; **Konsequenzen gezogen:** (i) die **tote DOI** ist eine Schreibweise — `10.34515/DATA.GND-0004-0002-0201_v01` (Punkt) → HTTP 206, die genannte `DATA_GND` (Unterstrich) → 404; (ii) **Madrigal ist nicht hart gegated** — CEDAR sagt „prior permission not required", EISCAT ist ein Name/E-Mail/Affiliation-Cookie ohne Passwort, und der bestehende `openmadrigal`-Arm setzt genau solche generischen Parameter (Mode 2a/2b fahrbar); (iii) **Athabasca numerisch** = ERG-SC-CDF (`ergsc.isee.nagoya-u.ac.jp/data/ergsc/ground/vlf/ath/`, listing `pending`); die Quick-Look-JPEGs sind mit `vision` lesbar (Achsen/Farbskala, kein Zahlenwert); (iv) **CDF-Leser gebaut und committet** (`92346fb85`: `extract_witness_channels` in `src/archivar/cdf25.rs` — saubere Datei; die 3-Zeilen-Verdrahtung in `extract.rs` über einen privaten Index, fremde Hunks blieben draußen; skalare `field`-Variablen, Zeit aus der Datei-Epoch; `cargo check` 0/0); (v) Registrierungsseiten EISCAT/WALDO/UCAR-FPI im Browser geöffnet (`river-signup`); **Register-Block-Entwürfe** für die 5 keyless Fäden stehen (`state/river/weberin-register-entwuerfe-2026-10-13.md`), mit gemessenen Mustern und benannten Lücken (Kadenz/Koordinaten/Experiment-ID/Lizenz). **1a UCalgary-Riometer** + **4b SABER-CDAWeb** keyless HAPI; **6 BARREL**/**5b EMFISIS** keyless CDF (mit dem neuen Arm lesbar); **1b SGO** form/POST (kein Static-Arm); **5c Athabasca** JPEG; CGSM kein `channel`.
- **Blockade:** (1) **WALDO:** der JS-Dateimanager (Use-your-Drive) rendert im automatisierten Browser nicht („Loading…") — der Live-Klick-Test ist hier nicht messbar; (2) die Madrigal-/HAPI-Blöcke brauchen je Faden die konkrete Experiment-/Station-Angabe; (3) die P10-Etikettierung je Faden ist eine Rat-Entscheidung.
- **Register-Access gemessen (river-204, 2. Durchgang):** **UCAR-FPI** Download **offen** (`registration.hao.ucar.edu/hao-reg_email.php?...20236300athabasca.dat` → HTTP 200, 18 481 B; der Pfadname täuscht), **CEDAR Madrigal** offen („prior permission not required"), **EISCAT Madrigal** Name/E-Mail/Affiliation-Cookie ohne Passwort (der `openmadrigal`-Arm schickt generische Werte) — **alle drei keyless fahrbar**. **WALDO** ist der einzige echte Riegel: WordPress-Login, **keine API-Key-Seite** (Profil geprüft: keine Application-Passwords); der Download ist ein JS-Dateimanager hinter der Sitzung (kein statischer Link in der DOM, keine `admin-ajax`/`drive.google`-URL sichtbar). Operator hat `WALDO_USER`/`WALDO_PASS` in `.secrets.local` eingetragen (Namen verifiziert via `bin/secrets_keys`).
- **Braucht:** (1) **WALDO:** WALD OS' Massendaten-Kontakt anschreiben (Vorbereitung eigen, Absenden Operator-Hand) **oder** den Download-Link des Managers aus Operators Browser kopieren; (2) die Entwurfs-Lücken schließen (`state/river/weberin-register-entwuerfe-2026-10-13.md`: CHUR-Koordinaten via `curl https://api.phys.ucalgary.ca/hapi/info?id=NORSTAR_RIOMETER_K2@CHUR`; EISCAT/CEDAR-Experiment-Datei; SABER-Kadenz via HAPI-`info`; Lizenz-Klauseln); (3) für **UCAR-FPI** einen `format`-Arm für leerzeichengetrennte ASCII-Spalten bauen (Parser) — dann Block 1; (4) den Athabasca-ERG-SC-Pfad erneut messen (Listing war `pending`); (5) je eingefügtem Block die Mycelium-Manifestation.

### Netz-End-Validator — GIC↔Onset-Bin; MCON manifestiert, Register-Drift gelöst
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** `bpa_gic` deckt den MCON-Zeitraum (1999–2024) UND die `format mcon_event`-Registerzeile steht (Mountain).
- **Lage:** (gemessen 2026-10-12 river-200 via `ci_manage log 38176003927` + mycelium-318) **MCON manifestiert** (`mcpherron_onsets.bin`, 314 555 Onsets, sha `441df5f8…`); Register-Drift gelöst. `bpa_gic` führt nur ein 4-Tage/5-min-Fenster → 0 Onsets im MCON-Fenster; die zweite GIC-Zeugenreihe bleibt leer, die Relation läuft über die FMI-Reihe.
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

### Newell-OMNI-Alignment — Doppelregistrierung geheilt; Zelle neu messen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** der dispatchte `field-te-query.yml`-Lauf `38378918382` (Auftrag `matrix-newell-omni`) — Artifact lesen.
- **Lage:** (gemessen 2026-10-13 river-204/205) `gh workflow run field-te-query.yml` neu dispatcht (`38378918382`, **queued** am HEAD); der erste Lauf `38368666012` rot (Ursache: `field_te_query.rs:2676` formatierte `channel_ref` mit `{}` — P10-Leftover, gefixt `80b63d360`). Das Ergebnis ist noch nicht gelesen (kein Polling).
- **Blockade:** die Zelle ist CI-only (`workflow_dispatch`, nie lokal).
- **Braucht:** das Artifact `field-te-matrix-newell-omni` aus `38378918382` lesen (`ci_manage view 38378918382`); bei weiterer Fenster-Disjunktheit den Descriptor auf ein gemeinsames Fenster ziehen.

### ENTSO-E — HTTP 400 gemessen: Parameter-/Endpunkt-Defekt, kein Token
- **Status:** wartend | **Bindung:** linie:mycelium
- **Trigger:** die Register-Zeile + der `out_Domain`-Parameter (Mycelium).
- **Lage:** (gemessen 2026-10-13 river-204 via `general`-Taucher) der Compiler `tools/harvest/src/bin/entsoe_compiler.rs` sendet `https://web-api.tp.entsoe.eu/api?securityToken={token}&documentType=A78&in_Domain={domain}&periodStart=…&periodEnd=…` — **nur `in_Domain`, kein `out_Domain`**. Gemessene Fehlertaxonomie: ein ungültiger/fehlender Token → **HTTP 401** + XML `code 999`; der CI-Lauf mit dem ausgegebenen Token (`38233635426`) → **HTTP 400** + HTML `uu-appg01/server/invalidRequest`. Der Token ist akzeptiert, **`A78` verlangt `out_Domain` + `in_Domain`**. **Nebenfund:** es gibt **keine** lebende `phi/sources.φ`-Zeile für ENTSO-E (`sgrep -i entsoe` = 0); der Compiler + Workflow existieren unregistriert.
- **Blockade:** `out_Domain` (bzw. der korrekte `documentType`) + die Register-Zeile.
- **Braucht:** (a) Mycelium: `A78` mit `out_Domain` (oder den richtigen `documentType`) senden + die `phi/sources.φ`-Zeile registrieren; (b) danach den `entsoe-cdn`-Lauf neu dispatchen.

### Flyby-Kette — recon bleibt (`ephemeris-juice-recon` + `flyby-path2-recon`)
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** `ephemeris-juice-recon` Wiedervorlage 2026-11-01 (ESA/ESOC publiziert die Post-Flyby-SPK). Wahrheit: `state/zustand/wartend.φ`.
- **Lage:** (gemessen 2026-10-11) die Wiedervorlage steht; `data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin` absent. `flyby-path2-recon`: orbc-SPK keyless (HTTP 206); die 1-σ-Kovarianz liegt nicht im SPICE-Baum.
- **Blockade:** ESA/ESOC-recon-SPK + die 1-σ-Kovarianz absent.
- **Braucht:** am 2026-11-01 `archive_search --verdict` auf den ESOC-recon-Pfad; dann `flyby_ephemeris_gate --recon <recon.bin> --sigma-recon <km>`.

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

**ENTSO-E — die zwei gemessenen Defekte.** (1) Der `entsoe_compiler` sendet `documentType=A78` mit `in_Domain` ohne `out_Domain` → HTTP 400 `uu-appg01/server/invalidRequest` (der Token ist akzeptiert; ein ungültiger Token gäbe 401). (2) Es gibt keine lebende `phi/sources.φ`-Zeile für ENTSO-E. xfer:river204-entsoe1.

**BGR-Infraschall-Ersatzquellen — erledigt (Mycelium-Meldung 2026-10-13: eingetragen).** Kein Punkt mehr.

**CTBTO — Formular vorbereitet (Operator-Akt = Absenden).** Mycelium meldet das Formular vorbereitet; das Absenden ist Operators Hand.

## Abschluss

river-204 (Kopplungs-GPU-Seite): gearbeitet ohne Pro-Solo, ohne Fenster-Edit, ohne Send.
**GPU-Seite der Kopplung gebaut + Parität** (der erste der zwei großen Bauten): `FIELD_WGSL`
`apply_couplings` (Binding 10) + Aufruf in `presence_probe`; `omega.rs` Deskriptor-Zeilen +
Buffer + Layout/Bind 10; Test `coupling_injection_wgsl_matches_the_cpu_exchange` (WGSL/CPU-Parität
+ `ΣρΔu ≡ 0`) — `cargo check` 0/0, `rustfmt` clean; die WGSL-Validierung + das Paritäts-Gate
laufen im CI. **ENTSO-E direkt geprüft** (Operator-Wort): Parameter-/Endpunkt-Defekt (`A78` ohne
`out_Domain`), kein Token; keine lebende Register-Zeile (Mycelium). **Weberin-Zeugen-Fäden:**
die per-Faden-Fakten gemessen (`state/river/weberin-faden-facts-2026-10-13.md`) — der zweite
große Bau (die Register-Zeilen) hängt an der P10-Etikettierung (Rat), an freien Registrierungen
(Dritter) und an einem fehlenden CDF-Arm; als eigener Punkt mit per-Faden-`Braucht` getragen.
**GIC-Onset** ist ein Datenankunfts-Warten (Harvest-Läufe), kein Bau. **LOCK** bleibt (Operator-Wort):
er fällt erst mit fertigem + kalibriertem Frühwarnsystem. `--fired river` = 1 (Newell-Lauf
`38378918382` queued — Artifact noch nicht lesbar, kein Polling), `--stale --persist 3` = 0,
`open_points_check` = 1 format-gap (im Weberin-Punkt behoben). Kein `smail --send`, kein Window-Edit.
