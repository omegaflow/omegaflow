<!--
  title: Handover — River-Folge 196 (2026-10-12)
  session: River-Linie in einem Pass — THEMIS-ASI kapu-PGM-Kopf gemessen (49.3922 / −82.3198 / 0.0 km), die Projektionskoeffizienten als konstant (1.5/0/0/0 über 4 Stationen/Epochen), die Skymap-Bodenwahrheit extrahiert (Lib-Parser idl_save, IDL-SAVE v4; Einheit Grad) und die meridionale Projektion gebaut (skymap_fit; Meridian-Schnitt in themis_asi_timed_keogram --skymap); FOV-Riss geschlossen (Mende 2008: 170° Vollwinkel / 160° auf 110 km projiziert; Donovan 2006 trägt keine Zahl); der PGM-Höhen-Riss als ellipsoidische Höhe aufgelöst und die Stations-Tabelle selbst korrigiert (16/24 nominal 0.0; ATHA/EKAT/PGEO/MCGR gesetzt, Operator-Wort); die MIRACLE-Form ist bereits gebaut (Mountain a0d2c04e2, Lauf kapu gemessen); CI-Läufe field-te-query 38215224525 (pending) und bz-dbdt-delay-probe 38215420000 (queued) weiter ausstehend
  class: handover
  date: 2026-10-12
  sha256: ff8c6123bd125338c452f4f01f5ec404f61917ab2a26d95330f5b5630fc5e823
  status: live
-->
# Handover — River-Folge 196 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward).

## Burn: open 0.0 · close 0.33 · cap 0.50 — Grund: River-196 (deepseek-flash, kein pro/max); Linien-Session `$0.1308` + 2 `general` (Projektionsform $0.0402 · geschärfter Skymap-Pass $0.0637) + 5 `grind-flash`-Bau-Dispatches (IDL-SAVE-Parser · Lib-Refactor `idl_save` · `skymap_fit` · Keogramm-Meridian · Einheiten-/Off-map-Fix); `session_burn`-DB parallel (nicht sauber isoliert). Fälligkeit: `--fired river` = 1 (Vlies-Matrix-Alignment, Lauf noch `pending`), `--stale river --persist 3` = 0, `open_points_check` F195 = 0 absent / 0 stale-citations; `--addressed river` = 3 Blöcke in F195 gefaltet. Gearbeitet: kapu-PGM-Kopf + Koeffizienten-Variabilität (4 Stationen/Epochen), Skymap-Extraktion (Lib-Parser, Einheit Grad korrigiert), Fit + Keogramm-Meridian verdrahtet. Kein `smail --send`, kein Window-Edit.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Nicht gebaut — die P10.2a-Token-Semantik ist nicht entschieden; ein geratener Bau wäre Fabrikation (A=A). bitte archive search all rat und roster" | 2026-10-10 | Operator (Session, River 165)
„wenn du den rat befragst befrage bitte auch die wissenschaft mit archive serahc alll und die ui und oenweight frontier voices" | 2026-10-09 | Operator (Session, River 145)
„warum nur duck … ich möchte dass du alle frontier chats befragst" | 2026-10-07 | Operator (Session, River 127)
„ja ich meine alle blöcke müssen korrekt sein dafür haben wir doch die wissenschaft" | 2026-10-09 | Operator (Session, River 160) — **P10-Wort**: jeder `field`-Block wird wissenschaftlich geprüft und korrekt etikettiert (Quantity | Mechanism | Medium)
„ja natürlich sonst wartest du doch bis zum st. nimmerleinstag" + „ja bitte ihr müsst das jetzt echt mal in den griff bekommen" | 2026-10-10 | Operator (Session, River 170) — den abgebrochenen `ci-gate` neu anstoßen; den CI-Burst in den Griff bekommen
„mach A" | 2026-10-10 | Operator (Session, River 170) — `ci-gate.yml`-`subset` von dem geteilten t420 auf GitHub-hosted `ubuntu-24.04-arm` mit per-SHA-Concurrency umziehen
„beides" | 2026-10-10 | Operator (Session, River 170) — (1) die zwei fremden ci-gate-Roten heilen; (2) die `observer`→`receiver`-Frage als Rat-Frage aufsetzen; Audit „wo wurde der Observer-Bias wieder eingeschleust"
„C (Skalenspektrum je Paar) ist der nächste begrenzte Schritt — Architekturfrage → Linse der fünf Stimmen, bevor gebaut; danach D (Summary-Graph/PCMCI, Runge 2105.10381/1702.07077). Die Seats: B ohne Skalenannotation wäre die stille Fabrikation. bitte --all rat und max roster" | 2026-10-10 | Operator (Session, River 172)
„Nächster Schritt: D2 durch die fünf Stimmen (Rat), dann Skeleton/MCI bauen. bitte --all max roster und rat" | 2026-10-10 | Operator (Session, River 172)
„bitte --all max roster und rat" | 2026-10-10 | Operator (Session, River 174)
„z.ai immer glm 5.3 max gibt es auch in tryong open genauso wie qwen" | 2026-10-10 | Operator (Session, River 174)
„mit archive search dem rat und max roster (alle und warten bedenke tryingopen1)" | 2026-10-11 | Operator (Session, River 181)
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie **in einem Pass** … /commit ist das Commit-Wort — beide getrennt." | 2026-10-11 | Operator (Session, River 188/191/192/193) — session-weiter Delegations-Consent, nicht das Commit-Wort.
„ok dann ist es pending … dann warten wir bis wir zugelassen sind kannst du nochmal in state schauen wir hatten da eigentlich einige netzbetreiber" | 2026-10-11 | Operator (Session, River 188) — ENTSO-E/NERC bleiben `pending`; auf die Zulassung warten, bis dahin die Netzbetreiber aus `state` referenzieren (nicht re-suchen).
„mountain ist sehr beschäftigt und mycelium auch … kümmer dich bitte selbst drum" | 2026-10-11 | Operator (Session, River 191) — **River übernimmt den Vlies-Matrix-Alignment-Punkt selbst** (SWPC/RTSW-Archivreihe finden + registrieren + `matrix-vlies` neu), auch in Mountain- (`phi/sources.φ` url) und Mycelium-Domäne (Manifestation/Compiler).
„ja" | 2026-10-11 | Operator (Session, River 192) — (1) den SPAN-Feld-Edit bauen; (2) die vier verwaisten `cargo fmt`-Hünke (Mountain-316/Mycelium-312-Rückstand) als Haus-Dienst committen.
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie **in einem Pass** … /commit ist das Commit-Wort — beide getrennt." | 2026-10-12 | Operator (Session, River 194) — session-weiter Delegations-Consent, nicht das Commit-Wort.
„Starte die River-Linie **in einem Pass** … Fahr die River-eigenen Teile weiter (BCa, Vlies-Restarm, Z-Null-Zahl im CI) — sie hängen an nichts von mir. Der Resolver steht; sobald `38215224525` grün gelesen ist, ist die Antworseite des Blatts frei. … mach bitte deinen teil" | 2026-10-12 | Operator (Session, River 195) — Work-Order, session-weiter Delegations-Consent, nicht das Commit-Wort.
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie **in einem Pass** … /commit ist das Commit-Wort — beide getrennt." | 2026-10-12 | Operator (Session, River 196) — session-weiter Delegations-Consent, nicht das Commit-Wort.
„ja bitte selbst mountain ist beschäftigt ich glaube wir müssen projektownerschaften einführen nicht aufgabenownerschaften das reibt so dermassen" | 2026-10-12 | Operator (Session, River 196) — (1) River setzt die Stations-Tabellen-Korrektur **selbst** (fremder Schreibakt auf Operator-Wort); (2) **Architekturfrage: Projekt-Ownerschaft statt Aufgaben-Ownerschaft** (Rat/Operator).

Verbatim: `state/operator-gespraeche/2026-10-12-river.md`, `2026-10-11-river.md`,
`2026-10-10-river.md` und `2026-10-09-river.md`.

## Träger (Prosa, eigene)

- `docs/concepts/kanal-ontologie-komplettbau.md` — der komplette Bauplan (P0–P10); P10.2a (der Parser-Arm ist Rivers Teil) als gebaut nachgeführt (`:215`).
- `docs/concepts/archivar-mathematikerin.md` — der Wire/GPU-Force-Vertrag; Träger des kinetischen Rahmens + der **Receiver-Apertur**-Kontrakt-Zeile (`:34`) + des **Schneidungs-Gesetzes** (Faltung/Varianzsumme); sha `3f395ea1…`.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`status: unsealed`) — Präregistrierung der Vorhersage-Zelle; **river-195**: die Intervall-Konstruktion benannt (kein Block-BCa; studentisiertes Moving-Block-Bootstrap, delete-a-block-Jackknife-SE). Siegel = Operator-Wort.
- `docs/paper/gic-causal-driver.md` — **river-195**: die Punktschätzer-Unsicherheit von „joint stationary bootstrap (BCa)" auf das studentisierte Moving-Block-Bootstrap umgestellt (`:171`); sha neu.
- `docs/concepts/der-paradigmenwechsel.md` (9 offene Marker) — von sensory-255 gefaltet (Träger-Abgabe JUICE/Flyby-Path-2; Wartend.φ `flyby-path2-recon` Aufnehmer river).
- `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4) — von sensory-255 gefaltet.
- `docs/paper/flyby-path-2-preregistration.md` (1) — von sensory-255 gefaltet.
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` — die Zellen der Flugbahn stehen `pending` (nach dem Flyby); der `pending`-Rest ist die 1-σ-Kovarianz (siehe §Flyby-Kette).
- `docs/surveys/survey-2026-10-12-standort-lokalisierung-ortho-dsm.md` — **river-196**: die generalisierte Standort-Lokalisierungs-Landkarte (Orthofoto · DSM/DOM · Street-Level) + Methode `dsm − dtm`; Carrier dieser Survey. Sie ergänzt `docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md` (deren Orthofoto-Lücke ihr Nachtrag `:402-411` benennt).
- **Offene-Prosa-Träger (future-235/future-240-Abgabe, River-Carrier):** `docs/auftrag/auftrag-flyby2-kette.md` (3) · `docs/auftrag/auftrag-universelles-vlies.md` (2) · `docs/blatt/blatt-te-externer-steuerparameter.md` (1) · `docs/blatt/fruehwarnsystem-praeregistrierung.md` (1, `status: unsealed`) · `docs/paper/flyby-path-2-addendum-2026-09-29.md` (27) · `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (1 — Marker `:122` gemessenes `descoped`). `register_lookup --orphan-docs` = 0.

## Netzbetreiber-Referenz (state, nicht re-run)

Gemessen 2026-10-11. Quellen: `state/future/space-weather-service-kante.md` §Adressat · `state/river/gic-sources-broad-2026-10-11.md` · `state/river/registrierungen-2026-10-11.md` · `state/future/funding-register-fuenf-achsen-2026-10-09.md`.

- **ENTSO-E** (EU, Dach 40 TSOs) — `token pending` (A78-Compiler gebaut). **Fingrid** (FI) — Token + Compiler; `fingrid-cdn 38162332055`. **NESO** (UK) — keyless CKAN 200, kein Outage-Datensatz. **BGS** (UK) — nur B_GIC-Index/Modell. **Statnett** (NO) — PDF-only. **Amprion** (DE) — keyless CSV-Outage-API. **TenneT** (DE) — 403 geo-block. **50Hertz** (DE) — PDF-only. **TransnetBW** (DE) — delegiert an die ENTSO-E-Plattform (`10YDE-ENBW-----N`). **Svenska Kraftnät** (SE) — CKAN, kein Outage.

Operator-Wort 2026-10-11: auf die Zulassung warten (ENTSO-E/NERC `pending`); die Netzbetreiber aus `state` referenzieren, nicht re-suchen.

## Offen (aufgeschlüsselt)

### Vlies-Matrix-Alignment — Fix committet, Lauf 38215224525 liest
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss des `field-te-query`-Laufs `38215224525` (HEAD `3467ec4f9`).
- **Lage:** (gemessen 2026-10-12 via `ci_manage list` + `ci_manage view`) der Fix `f018f4df0` ist committet (per-URL-Memo + Retry gegen den 522-B-Stub); Lauf `38215224525` um 06:58Z dispatcht, **river-196: noch `pending`** (`ci_manage status`). Vorher: `alignment pending` 126 → **50** (fünf Reihen ohne Quellenzeile: `qbo_30hpa_ms`/`d20_thermocline_depth_m`/`swarm_absolute_vertical_tec_tecu`/`eve_1032_line_irradiance`/`eve_131_line_irradiance`); `solar_wind_density_cm3` echt absent (`Np = -1.0E31` fill → 0 honored).
- **Blockade:** keine (Fix committet).
- **Braucht:** `ci_manage log 38215224525` lesen; erwartet: `bt`/`speed` lösen, `alignment pending` fällt auf die absent-source-Reihen. Der distinct-URL-Throttle (SWE) bleibt tiefer in `src/archivar/fetch.rs` (Backoff auf 522-Stub), benannt.

### Disjunkter optischer Onset-Zeuge (THEMIS-ASI) — Kamera-Modell + Skymap-Projektion gemessen
- **Status:** eigen (Messung/Bau) | **Bindung:** eigen
- **Trigger:** der GIC-Onset-Relationsschritt auf der Meridian-Spur (Blatt/Antwortseite); die Manifestation der Keogramm-Assets (`--ci-mode`, Mycelium).
- **Lage:** (gemessen 2026-10-12, Artefakt `state/river/themis/calibration-2026-10-12.md`) Die Pixel→Sky-Abbildung ist **keine** Formel, sondern das per-Station-**UCalgary-Skymap** (`.sav`). Orientierung (am Code): roh row0=N, Spalte 0=Ost. **FOV-Riss geschlossen** (Mende et al. 2008, Volltext gemessen: **170°** = Vollwinkel der Anforderung „circular 170 full angle" Tab. 1; **160°** = „160-degree total field projected to 110 km altitude" §3; physischer Disk **~184°** aus dem Skymap — drei Definitionen, dieselbe Quelle; die früher als 160-Quelle genannte GBO-PDF trägt keine Zahl). **Donovan 2006 trägt keine FOV-Zahl** — der frei zugängliche Autoren-PDF (`aurora.phys.ucalgary.ca/donovan/pdfs/donovan_themis_asi_jastp_2006.pdf`) wurde im Volltext gelesen; die FOV existiert dort nur als 110-km-Fußabdruck (Fig. 3), keine Gradzahl → das `pending` ist **geschlossen**. **(river-196) kapu-PGM-Kopf gelesen:** kapu = **49.3922 / −82.3198 / 0.0 km**, `x0=392,y0=153`, aspect 0.5181; atha/themis02 (2026-10-01) = 54.72 / 246.72 (0–360) / 676 m. **(river-196) Koeffizienten-Variabilität gemessen:** `a1/a3/b2/b4 = 1.5/0/0/0` **konstant** über 4 Stationen/Epochen (gill 2014+2024, kapu 2024, atha 2026); nur `Optical center` variiert (382/150, 392/153, 376/142). **(river-196) Skymap `.sav` extrahiert:** `state/river/themis/kapu_20240909_skymap.sav` (IDL-SAVE v4, sha `adc8208a…`) via neuem Parser `tools/measure/src/bin/idl_sav_dump.rs`; `FULL_ELEVATION`/`FULL_AZIMUTH` als 256×256 f32 gedumpt (`state/river/themis/kapu_20240909_full_{elevation,azimuth}.txt`), `SITE_MAP_*` verifiziert (49.392212 / 277.6803 / 0). Das Skymap trägt die Projektion selbst als Array: `OPTICAL_PROJECTION` (9×2 f64) `[0,165,0,-3.98,0,-0.002,0,0,0 | 1,0,…]`, `CCD_CENTER=[390.7,146.6]`, `OPTICAL_ORIENTATION=[10,-1,0]`, `IMG_FLIP=[1,0]`, `FULL_MAP_ALTITUDE=[90/110/150 km]`; Generator `CODE_USED = write_skymap_file.pro` (IDL, intern). Skymap-URL-Form kampagnen-basiert (kapu-Kampagne `20240909`). **(river-196, Bau)** der Parser ist als Lib `tools/measure/src/idl_save.rs` (`load_skymap`) herausgezogen; Fit `skymap_fit`: Zentrum (= Zenit) row 121 / col 126, `θ_max = 91.99°` (Disk ~184°), **kein** Modell unter 10° RMS (linear 12.6 / stereografisch 10.7 / kubisch 10.3), `OPTICAL_PROJECTION` keinem Modell zuordenbar; Keogramm `themis_asi_timed_keogram --skymap` verdrahtet — der Meridian driftet über die Zeilen Spalte **106 → 148**, off-map-Zeilen 249–255 als `None`/NaN (kein stiller Default). **Einheit korrigiert: `FULL_ELEVATION`/`FULL_AZIMUTH` in Grad** (nicht rad; früherer Doc-Eintrag war falsch). **(river-196) Optischer Meridian-Onset gemessen:** kapu 2024-05-10, Keogramm 06:50–08:15 UT (1720 Frames, Meridian-Schnitt via `--skymap`), **Onset 2024-05-10T07:55:27.008Z** (Σ|Δprofil| = 11680, Fenstermax 12336 @ 07:55:27→07:55:30). Die river-192-Amplitude „+16.3" ist unter der Σ|Δ|-Metrik **nicht** reproduzierbar (Riss benannt, nicht geglättet). **GIC-Paarung `pending`:** FMI-GIC endet **2023-10-01** (CDN `fmi_gic_1min.bin`) bzw. `man202301-09` (2024 → 404) — **kein** GIC-Zeuge für 2024-05-10; dazu **geographischer Riss:** FMI-GIC = Mäntsälä/Finnland, kapu = Ontario (49.4°N/82.4°W) → der kanadische Zeuge läuft über **INTERMAGNET 1-min** (nächster Schritt, `intermagnet_xyz_compiler`).
- **Blockade:** keine (a1..b4 als konstant gemessen; die Skymap-Bodenwahrheit ist extrahiert und der Meridian-Schnitt verdrahtet).
- **Braucht:** (a) **erledigt** — beide Keogramm-Compiler emittieren die MIRACLE-Form (`station <CODE>` + `on earth <lat> <lon> <alt>`; Stations-Tabelle `src/archivar/themis_asi.rs`, Mountain `a0d2c04e2`; Lauf kapu 2024-05-10 gemessen: `on earth 49.392 -82.32 179.94`). (b) **erledigt** (kapu lat/lon/alt gemessen). (c) **erledigt** — FOV-Riss geschlossen (Mende 2008). (d) **erledigt** — Meridian-Schnitt in `themis_asi_timed_keogram --skymap` verdrahtet, Fit (`skymap_fit`) gemessen. **Offen (Mycelium):** die Keogramm-Assets über `--ci-mode` auf den CDN manifestieren (Register-Duty). **(e) Bauplan-Spur — erledigt (gemessen):** keine **öffentlichen** Site-/Gebäudepläne; die **CDR-Decks** (`themisftp/…/Reviews/`, `.ppt` via `soffice`→PDF→`--pdf-text`+Vision) tragen die **Instrumenten**-Konstruktion — die bemaßte **„Sun Shield Design"** (SSL; `CEPR05` Folie 12 = CDR Mende Folie 26; **Ø5.200 / 3.676 / 3.620 / 1.1 / 2.400 / 1.250**, Einheit auf der Zeichnung unbelegt) und **Gehäuse 34″×44″×44″** (CEPR10 Folie 23); Allison-Park-Enclosure-Renderings ohne Bemaßung. Anker `state/river/themis/calibration-2026-10-12.md` §Baupläne. Bauwerks-/Fundamentpläne bleiben **nicht öffentlich** (gemessene Lücke).

### Netz-End-Validator — GIC↔Onset-Bin; MCON manifestiert, Register-Drift gelöst
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** `bpa_gic` deckt den MCON-Zeitraum (1999–2024) UND die `format mcon_event`-Registerzeile steht (Mountain).
- **Lage:** (gemessen 2026-10-12 via `ci_manage log 38176003927` + mycelium-318) **MCON manifestiert:** `mcpherron_onsets.bin`, 314 555 Onsets, `format mcon_event`, sha `441df5f8…`. **Register-Drift gelöst** (mycelium-318): `phi/sources.φ:21545-21551` trägt `mcpherron_onsets.bin`/`format mcon_event`/`terms CC-BY-4.0`. `bpa_gic` führt nur ein **4-Tage/5-min-Fenster** (2026-10-03..06), **kein Archiv** (gemessene Null) → 0 Onsets im MCON-Fenster; die zweite GIC-Zeugenreihe bleibt leer, die Relation läuft über die FMI-Reihe.
- **Blockade:** `bpa_gic` deckt den MCON-Zeitraum nicht (kein Archiv; nur FOIA).
- **Braucht:** (a) `bpa_gic`-Harvest auf 1999–2024 ausdehnen (Mycelium) — oder den MCON-Zeitraum verwerfen und die Relation auf die FMI-Reihe stützen; (b) die Surrogat-Null-Form vor jeder Behauptung durch den Rat.

### wy-max-t — α als Family-Bound gemessen; Blatt-Siegel offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Siegel auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`.
- **Lage:** (gemessen 2026-10-12 via `gh run download 38166787414` + `wy_max_t_probe --combine`) der Westfall–Young-max-T-Null ist gemessen, α = 0.05, pooled B = 9999: (1-α)-Quantil **ABK-2024 = 2.4467 · ABK-2025 = 2.4433 · SOD-2024 = 2.4831** (Selftest 2.5856). Family-Maximum cleart an allen drei Punkten. family-clearing: `Bz→dB/dt` (3/3), `dB/dt→Speed` (3/3), `dB/dt→Density` (3/3), `dB/dt→Bz` (ABK-2024, SOD-2024), `Speed→dB/dt` (ABK-2025). MDE je Member gemessen. α steht im Blatt (`:78-85`).
- **Blockade:** das Blatt-Siegel ist Operator-Wort.
- **Braucht:** Operator-Siegel; die gemessenen Werte im GitHub-Artefakt `38166787414` in den Bestand (Mycelium).

### Receiver-Apertur (SPAN) — Schritt 3 offen
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** der nächste `ci-gate`-Lauf (Paritäts-Gate `field_spatial_fold_wgsl_cpu_parity`); danach Schritt 3.
- **Lage:** (gemessen 2026-10-12 via `cargo check` 0/0 + `git log` `5a3e629e7`) **Schritt 2 (Faltung)** gebaut und committet (river 192-193): `σ_eff² = extent² + global_scale²` in `field_spatial`/`field_spatial_grad` (WGSL) + CPU-Spiegel; `max(extent, scale)` gestrichen; Kontrakt `:34` nachgezogen.
- **Blockade:** das Paritäts-Gate ist ein Test-Target — `cargo check` kompiliert `#[cfg(test)]` nicht lokal; es läuft erst im CI.
- **Braucht:** (a) den nächsten `ci-gate`-Lauf lesen (`ci_manage log`), das Paritäts-Gate bestätigen; (b) **Schritt 3:** die deklarierte `ReceiverAperture` (`src/mathematikerin/receiver.rs`) an die Query-Naht (`vp.surface.w = global_scale`) binden — die deklarierte Blende je Receiver, nie in den Record; (c) die Operator-Browser-Messung (Fenster-Edit = Operator-Wort).

### GIC-Onset — AL-Einhüllende über die 154-Station-Familie
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** Abschluss der Harvest-Läufe `intermagnet-xyz-cdn` → die Assets `<code>_xyz_1m.bin` liegen im CDN.
- **Lage:** (gemessen 2026-10-12 via `ci_manage jobs`/`log 38164890620`) der Lauf trug 25 Stationen (`start 2000`, `end 2024`): 24 success, **`lov` cancelled** (LOV-Fetch HTTP 400 für 2023+); `38164966250` queued (same-workflow-Concurrency). Die Kette `intermagnet_xyz_compiler` → `al_network_proxy` → `gic_onset_detector` ist gebaut; die 154-Station-Partition liegt in `state/river/gic-family-*.txt` (31/25/98); Null-Baseline gemessen (`--null 200`: 1.62×/1.47×/1.35×).
- **Blockade:** die per-Station-`<code>_xyz_1m.bin`-Assets der noch fehlenden Stationen (24 fertig; `lov` 400).
- **Braucht:** `ci_manage log 38164966250` nach Abschluss; dann je Familie `al_network_proxy --family <fam.txt> --dir <pfad> --decl auto --baseline-block 60 --min-stations 2` und `gic_onset_detector --null 200`. (CDN-Registrierung + McPherron-`terms CC-BY-4.0` = Mountain/Mycelium.)

### GIC-Familien-Partition — zwei Register-Nägel, dann die Matrix-Läufe
- **Status:** blockiert | **Bindung:** eigen (Blockade: Mountain-Nägel)
- **Trigger:** die zwei Register-Nägel sind gesetzt (Mountain), dann Run der Matrizen.
- **Lage:** (gemessen `state/stimmen/2026-10-06_gic-familien-arm-ui-stimmen.md:20-80`) die Stimmen-Runde (GLM-5.3, qwen 3.8 Max, Kimi) einig: **kein** neuer Grammatik-Arm, sondern (i) ein Satz über Querfamilien-Zellen und (ii) die Familienregel als deklariertes `station→Familie`-Artefakt. Minimaler Patch: `matrix gic full` + `family auroral|subauroral|midlat stations <code…>` + `expect families 3` + `fdr by 0.05 over matrix` + `wymaxt 0.05 over family`.
- **Blockade:** die zwei Nägel (Mountain-Register); der Grammatik-Patch ist aus dem Verdikt noch nicht gebaut.
- **Braucht:** Mountain setzt die zwei Nägel; danach den Patch bauen und `matrix gic_auroral|gic_subauroral|gic_midlat full` fahren.

### Newell-OMNI-Alignment — Resolver gebaut, Lauf 38215224525 liest die Zelle neu
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss des `field-te-query`-Laufs `38215224525`.
- **Lage:** (gemessen 2026-10-12) der Resolver ist gebaut (`field_te_query.rs` `load_field_across_sources`: per-`station_code`-Scope, feinster native Zeuge als Ergebnis, gröberer Aggregationszeuge als getragener Riss statt `Err` — Mycelium `3467ec4f9`); Rat-Verdikt „Regel statt Rename" (`state/stimmen/2026-10-12_river-intermagnet-identitaet-rat.md`). Vorher: die Zelle `newell_dphi_dt_omni->intermagnet_dbdt` war **floor (p=1)** — `abk_dbdt_1m` (60 s) vs `abk_dbdt_1h` (3600 s), corr 0.9388. **river-196: Lauf `38215224525` noch `pending`.**
- **Blockade:** keine (Resolver gebaut).
- **Braucht:** `ci_manage log 38215224525` → die Zelle `newell_geospheric_omni` neu lesen; erwartet: ABK `abk_1m` als Zeuge, `abk_1h` als getragener Riss.

### Flyby-Kette — recon bleibt (`ephemeris-juice-recon` + `flyby-path2-recon`)
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** `ephemeris-juice-recon` Wiedervorlage 2026-11-01 (ESA/ESOC publiziert die Post-Flyby-SPK). Wahrheit: `state/zustand/wartend.φ`.
- **Lage:** (gemessen 2026-10-11) die Wiedervorlage 2026-11-01 steht; `data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin` absent. `flyby-path2-recon`: orbc-SPK keyless `https://spiftp.esac.esa.int/data/SPICE/JUICE/kernels/spk/juice_orbc_000116_230414_310721_v01.bsp` (HTTP 206); die **1-σ-Kovarianz liegt nicht im SPICE-Baum**.
- **Blockade:** ESA/ESOC-recon-SPK + die 1-σ-Kovarianz absent.
- **Braucht:** am 2026-11-01 `archive_search --verdict` auf den ESOC-recon-Pfad; dann `flyby_ephemeris_gate --recon <recon.bin> --sigma-recon <km>`.

### Blatt-Kalibrierung — BCa benannt; Z-Null-Zahl aus CI 38215420000
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** (a) Operator-Siegel; (b) Abschluss des `bz-dbdt-delay-probe`-Laufs `38215420000`.
- **Lage:** (gemessen 2026-10-12) **BCa benannt** — die Intervall-Konstruktion des Punktschätzers steht im Blatt + Papier: **kein „Block-BCa"**, getragen wird ein **studentisiertes Moving-Block-Bootstrap** (delete-a-block-Jackknife Künsch 1989 als SE; Moving-Block-Perzentil als 1.-Ordnungs-Form; Subsampling Politis–Romano als Default) — Rat + Claude + DeepSeek V4 Pro + Nemotron + GPT-OSS konvergent (`state/stimmen/2026-10-12_river-bca-z-rat.md`). **Z-Null gebaut** (`bz_dbdt_delay_probe.rs`: 60-min-Shift + delete-one-Block-Jackknife-SE + studentisiertes Lag-Maximum); Workflow `bz-dbdt-delay-probe.yml` (Mycelium `f664c1b96`). **river-196: Lauf `38215420000` noch `queued`** (ABK/SOD 2024, `--n-surr 200`).
- **Blockade:** die Z-Null-Zahl braucht den CI-Lauf (Fetch/Compute).
- **Braucht:** `ci_manage log 38215420000` → die kalibrierte Z-Zahl ins Blatt (der Deskriptiv-Wert `Z = 142 min, r = 0.0223` von 2026-10-05 wartet darauf); α/Bz-Schwelle + Siegel = Operator-Wort (nach Future geroutet).

## An mountain

Origin: river-195/196 (2026-10-12) — Messungen/Befunde, die Mountains Register/Archivar betreffen (Sender-Register; die Owner-Session faltet).

- **MCON-Registerzeile — erledigt (mycelium-318).** `phi/sources.φ` trägt jetzt `mcpherron_onsets.bin`/`format mcon_event`/sha `441df5f8…`/`terms CC-BY-4.0`. Kein Rest.
- **`intermagnet_dbdt`-Identitäts-Riss — erledigt (Mycelium `3467ec4f9`).** Der Resolver (Regel statt Rename) ist gebaut: per-`station_code`-Scope, feinster `median_dt` als Zeuge, gröberer als getragener Riss. Register/Wire unangetastet. Verdikt: `state/stimmen/2026-10-12_river-intermagnet-identitaet-rat.md`.
- **`solar_f107_flux_sfu` — kein Identitäts-Riss, ein Zeitbasis-Defekt (gemessen).** (2026-10-12, `general`-Taucher) Der SWPC-Textarm liefert **TDB/J2000** (`lsk.unix_to_tdb`, `extract.rs:6506`, `parse.rs:2349`), das Penticton-`f107`-Asset **rohe Unix-Sekunden** (`src/archivar/f107.rs:22`, `day*86400`); feste Verschiebung `≈ 9.4673e8 s ≈ 30,0 a` → SWPC-2026 paart mit Penticton-1996 → r≈0.02 (mycelium-318 nennt corr 0.0232). **Zwei Archivar-Defekte:** (i) `f107::parse_series` wendet `unix_to_tdb` **nicht** an; (ii) `series_epoch_of` (`extract.rs:6506-6539`) wählt den ersten Key mit `"date"` — `avg_begin_date` kann `time_tag` verdrängen. **Braucht:** beide Arme auf dieselbe Epoch-Basis (TDB) bringen; dann den `f107`-Join erneut messen.
- **MCON-Spanne 1966–2025 vs Parser-Ende 2024-05-29** — nominale Beschreibung vs Parser-Fenster; für die BPA (2026) irrelevant.
- **THEMIS-ASI Stations-Tabelle (`src/archivar/themis_asi.rs`) — River hat sie (Operator-Wort 2026-10-12) direkt korrigiert.** Fremder Schreibakt, beide Enden genannt: der PGM-Kopf `#"Geodetic altitude"` ist die **ellipsoidische Höhe** (pina 247=h247.63, tpas 236=h240.59), **16/24 nominal `0.0`**; die Tabelle `h=H+N` nutzt Stadt-/Flughafen-H. Gesetzt: **ATHA** 511.82→**549.82** (AUGO I; PGM 676 = AUGO II) · **EKAT** lon −109.333→**−110.6703** / h 398.83→**430.77** · **PGEO** 561.26→**756.26** · **MCGR** 112.57→**102.57**. Anker `state/mycelium/themis_asi_stations.md`, `state/river/themis/calibration-2026-10-12.md` §Höhen-Riss/§LiDAR. **HRDEM (NRCan, 1 m LiDAR, keyless, orthometrisch CGVD2013) bestätigt ATHA = 549.82** (571.19 m DTM − N 21.18); Cover 10/24 (Alberta/Ontario/…). **Korrigierter Koordinaten-Riss:** die PGM-Koordinate (54.72/−113.28) liegt auf der **Stadt** Athabasca, nicht auf dem AUGO-I-Dom (Rev-B 54.714/−113.314) — der frühere Satz „PGM-lat/lon auf AUGO I" war falsch. **Grenze:** LiDAR gibt die Geländehöhe, nicht die Antennenhöhe (~2–3 m, `pending`); die Restunsicherheit dominiert die **Koordinate** (bis 2 km/55 m), nicht das DEM. **Orthofoto** (ESRI World Imagery ~33 cm/px, Vision) löst den Dom **nicht** auf (2–3 m ≈ 6–9 px), stützt aber das Rev-B-Umfeld (offene Mähfläche westlich des Campus; Kleinkörper back-projiziert ~54.7141/−113.3148). Haus-Survey `docs/surveys/survey-2026-10-10-worldwide-lidar-quellen.md` deckt Kanada-LiDAR (CanElevation S3/Dataset, LidarBC, ON, QC), **keine Orthofotos**; ihr ist der Orthofoto-Befund als Nachtrag-Zeile zugestellt (river-196). **DSM (`dsm − dtm`, HRDEM 1 m native EPSG:3979):** lokalisiert einen **isolierten ~1.6–1.8 m-Dom-Kandidaten bei 54.7140/−113.3142** (±10 m, ~15 m WNW Rev-B) — an der **Auflösungsgrenze**, im geglätteten 4326 weg → **`pending`** (lokalisiert, nicht unabhängig bestätigt). Dienst-Riss: 3979 vs 4326 ~**+35.5 m** absolut (hebt sich in `dsm−dtm` heraus). **Street-Level: keine Coverage** (Mapillary key-needed, KartaView/Panoramax 0, Google SV leer). Bonus: UCalgary-Site-Fotos des AUGO+1-m-Doms. Kein Code liest `Geodetic altitude`/`site_map_altitude`.

## An future

Origin: river-195/196 (2026-10-12) — Operator-Akt, den diese Session nicht selbst führen darf (Sender-Register; die Owner-Session faltet).

- **wy-max-t-Blatt-Siegel** — `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`status: unsealed`): der Westfall–Young-max-T-Null ist gemessen (α=0.05, pooled B=9999; (1-α)-Quantil 2.4467/2.4433/2.4831; `Bz→dB/dt` family-clearing an allen drei Punkten; Artefakt `38166787414`). Die Vorbereitung ist River-eigen; **das Siegel setzt der Operator**.
- **Street-Level-Keys (Zugang) — Operator-Frage.** Für Standort-Lokalisierung fehlen zwei **keyed** Street-Level-Quellen: (a) **Mapillary-Token** (kostenlos, Konto), (b) **Google-Cloud-Street-View-Static-API-Key**. Beide sind heute `key-needed` (`state/zustand/wartend.φ`-Klasse); die offenen Alternativen (KartaView, Panoramax) sind keyless, hatten am THEMIS-Standort aber **0 Bilder**. **Entschieden 2026-10-12 (Operator):** **(a) Mapillary — erledigt:** App registriert (Scope `read`), in `.secrets.local` liegt `MAPILLARY_ACCESS_TOKEN`; **nur dieser wird gelesen** (`sgrep -i mapillary` = nur der Reader) — `MAPILLARY_CLIENT_SECRET`/`_AUTHORIZATION_URL` liegen **ungenutzt** (kein OAuth-Flow nötig; Werte nie gelesen). **(b) Google — entfällt:** der Operator will **keine Karte** hinterlegen (Street-View-Static verlangt Billing) → keine Google-Street-View-Spur. **Gebaut (eigen):** `tools/measure/src/bin/mapillary_coverage.rs` (Graph API v4, liest `MAPILLARY_ACCESS_TOKEN`, `--bbox` / `--at <lat,lon> --span-deg <Δ>`, kein Erdradius-Konstante — Gate WP13; Token nie gedruckt). **Gemessen:** **ATHA 54.714/−113.314 `images 0`** (kein Street-Level am THEMIS-Standort — die keyless-Schätzung bestätigt); Kontrolle **Paris/Tokyo/Frankfurt lieferte beim Bau Bilder** (Token+Query gültig); **danach antwortet `graph.mapillary.com/images` global HTTP 500** (auch leere Ozean-Bboxes; externer Dienst-Zustand, kein Code-Defekt — benannt, kein stiller Default). Kein Send, kein Konto durch die Maschine.

## Abschluss

Der Commit steht aus — er wartet auf das Operator-Wort (`/commit`); `/consent` ist der
session-weite Delegations-Consent, nie das Commit-Wort.
