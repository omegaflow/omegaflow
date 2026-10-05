<!--
  title: Handover — Mountain-Folge 236 (2026-10-05)
  session: Mountain-Folge 236
  class: handover
  date: 2026-10-05
  sha256: d7a92ac64f9959f8f2b7b182bdd30fbc6096cfa4df768e5f7687618e0a7c37e1
  status: live
-->
# Handover — Mountain-Folge 236 (2026-10-05)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-05-mountain-folge235.md` (→ `archiv/`),
faltete den neuen `## An mountain`-Block `future-180` (FMI-IMAGE) und arbeitete
die offenen Punkte per flash-first (line + 8 grind-flash/general) in **einem** Pass
ab. Die Adressaten river-93/sensory-232 sind in folge235 bereits gefaltet.

## Burn: open 0.0 · close 0.3443 · cap 0.35 Grund: Runde flash-first — Line + 8 grind-flash/general + council + research-max + Browser-UI-Stimmen, kein pro/max-Default

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„verschleppen und nicht eigenes ist verboten" | 2026-09-30 | Operator (Session, Mountain 213)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-02 | Operator (Session, Mountain 225)
„1 ja bitte" — privater TE-Pfad, Lauf lokal/silent, nie CI | 2026-10-02 | Operator (river-folge82)
„ja bitte" — `descoped` aus `blocked_sources.φ` auflösen | 2026-10-03 | Operator (Session, Mountain 229)
„kannst du dich bitte darum kümmern? 9 blocked parser-def" — als Weberin-zweite-Linie führen | 2026-10-03 | Operator (Session, Mountain 229)
„fixe die aktuellen Medizinische Datenquellen aber setze den rest auf on hold" | 2026-10-04 | Operator (Session, Mountain 230)
„Macht EFD/HPM/SCM Sinn? — Ja." | 2026-10-04 | Operator (Session, Mountain 230)
„also bitte alles umsetzen ich möchte nicht dass du etwas in die nächste runde nimmst was jetzt von agenten bearbeitet werden kann" | 2026-10-04 | Operator (Session, Mountain 232)
„braucht es dafür wirklich pro?" — flash-first; der Katalog-Rest per flash geschlossen | 2026-10-04 | Operator (Session, Mountain 232)
„braucht es pro?" — flash-first bestätigt | 2026-10-04 | Operator (Session, Mountain 233)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-05 | Operator (Session, Mountain 235)
„was fehlt hast du in die secrets local geschaut?" — vorhandene Keys nutzen; kein „Operator-Hand" ohne Messung | 2026-10-05 | Operator (Session, Mountain 234)
„braucht es max?" — flash-first erneut bestätigt; ExoMars-Parser per grind-flash gebaut | 2026-10-05 | Operator (Session, Mountain 235)
„braucht es pro und kannst du dir das bitte ansehen?" — flash-first; die zwei Punkte-Listen gegen den Baum messen | 2026-10-05 | Operator (Session, Mountain 235)
„messe nochmal den aktuellen zustand dann commit" — Atom 2: neue adressierte Blöcke falten, FMI-GIC-fein-grain bauen, em-Apertur messen, committen | 2026-10-05 | Operator (Session, Mountain 235)
„braucht es pro und max?" — flash-first bestätigt: alle Kanal-/Serien-Arme per grind-flash/general geschlossen, kein pro/max | 2026-10-05 | Operator (Session, Mountain 236)
„bitte gib das dem rat den tauchern für wissenschaft und forschung und den 3 online stimmen" — Contract-Frage (Sentinel vs. Presence-Bit) an Rat + research-max + UI-Stimmen | 2026-10-05 | Operator (Session, Mountain 236)
„brauchen wir überhaupt pro für den rat/council … in dateien steht veraltet wann pro angebracht ist" — Council → flash/low; pro/max nur noch Eskalation nach gemessener flash-Fehllage | 2026-10-05 | Operator (Session, Mountain 236)

## Offen (aufgeschlüsselt)

### FMI-IMAGE-Magnetometer (NUR) — Manifestation
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05 via grind-flash + `cargo check`) Quelle registriert
  (`phi/sources.φ:27042-27048`, `format fmi_image_mag`, `image_mag_compiler.rs`,
  `COMP_IMAGE_DXDT` in `src/archivar/geo.rs`); Endpunkt `space.fmi.fi/image/www/
  data_download.php`, NUR `60.50 24.65`, `-dX/dt` 10 s, CC BY 4.0; `main_flow.rs`
  Loader-Token gesetzt, `extract.rs:819` Component-Arm. `cargo check` 0/0.
- **Blockade:** Asset `fmi_image_mag_nur.bin` nicht auf CDN
- **Braucht:** An mycelium — `image-cdn`-Workflow (Muster `fmi-gic-cdn.yml`) +
  `gh workflow run image-cdn.yml` → `fmi_image_mag_nur.bin` auf Release
  `space.fmi.fi`.

### IERS EOP C04 LOD — Manifestation
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05 via grind-flash) Quelle registriert
  (`phi/sources.φ:27050-27053`, `format iers_eop_c04_lod`, `iers_eop_c04_compiler.rs`,
  `COMP_IERS_LOD` in `geo.rs`, `extract.rs` Component-Arm); 23136 LOD-Tage. Der
  EOP-C04-**Komposit** ist `declined` (`declined_sources.φ:2429`, „kein Skalarfeld;
  Komponenten einzeln auto-detect") — die LOD-Komponente ist konsistent mit den
  live UT1/PM-Zeilen (`sources.φ:6998-7000`). AAM ist **nicht** Teil von EOP C04.
- **Blockade:** Asset nicht auf CDN; AAM account-gated (`esm-db.eu`, blocked account)
- **Braucht:** An mycelium Manifestation `iers_eop_c04_lod.bin`.

### Enclosure-Hülle — Konservativität (Riss, gemessen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05 via grind-flash, 9 echte DASTCOM-Records aus
  `ssd.jpl.nasa.gov-dastcom/dastcom_asteroids.bin`) `law_bounds`
  (`src/archivar/spatial.rs:243-274`) sampelt nur am Epoche; über die 64·ttl-Spanne
  (`spatial.rs:863`) **brechen** 1566 Icarus (1.076×) und 3200 Phaethon (1.546×)
  die Hülle `Φ·(v_epoch+resid_ema)`; Periapsis liegt im 64-d-Fenster. Test
  `src/archivar/tests.rs:3336` (`test_law_bounds_enclosure_span_breaks_for_periapsis_in_window`),
  `cargo check --tests` 0/0.
- **Blockade:** keine
- **Braucht:** Bound auf ein **Per-Body-Spannenmaximum** (Spanne in
  `law_bounds`/`build_asteroid_samples` sampeln statt Φ·v_epoch) — Rivers
  Query-Horizont `64·ttl`; siehe `## An river`.

### gbco axis-value Serie — Manifestation
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05 via grind-flash) `gbco_series_compiler.rs` +
  `parse_ascii`/`AsciiVar` in `src/archivar/opendap.rs` gebaut; gbco 2023+2026 je
  201×201 formgleich, 40402 Zeilen emittiert, `--selftest` grün, `cargo check` 0/0.
  `rixs`-Emitter existiert bereits (`rixs_series_compiler.rs`, Konsument
  `field_te_query.rs:1793`). **gmrt descoped** (gemessen: `format=esriascii` +
  `&version=1.0` byte-identisch → kein zweiter Epoch-Block).
- **Blockade:** Serie nicht auf CDN; `format gbco_axis_value_text` hat keinen
  `main_flow`-Load-Arm (als Witness-Serie via `field_te_query` geladen)
- **Braucht:** An mycelium CDN-Manifestation `gbco_elevation_gebco_{2023,2026}.txt`
  (`data.ceda.ac.uk-gebco`), dann `phi/witnesses.φ:187`/`:192` URLs tauschen.

### declustered Mainshock-Set — Verdrahtung
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05 via grind-flash) Das Set existiert bereits
  (`phi/witnesses.φ:180-184`, `record erbq_mainshock`, 153/198, Gardner-Knopoff);
  `erbq_mainshock_compiler.rs` reproduziert es objektid-identisch. Die
  river-93-Zeile „Quelle fehlt" ist **stale** (Riss, nicht geglättet) — der
  Eigentümer liefert die Vollschicht ohne Declustering-Flag (198, alle M≥7).
- **Blockade:** kein Descriptor/Konsument nennt `erbq_mainshock`
- **Braucht:** `erbq_mainshock` an die ETA-Form (`EVENT_FLOOR=2`) verdrahten —
  Rivers Natur (siehe `## An river`).

### Probes-Wanderung — Kandidaten (Riss, gemessen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05 via general) `solar_causal_graph` +
  `signal_cone_audit` **tragen** ihre Quellen bereits — `f107_penticton`
  (`sources.φ:24773`) + `Lya1216` (`:24782`) sind seit 2026-10-04 registriert
  (`2deb23292`), die Proben laden sie; die river-93-Behauptung „brauchen" ist
  stale. **laic** = kompiliertes TE-Fenster-Artefakt, kein physisches SI-Feld →
  `descoped`. **trishuli_gauge** = DHM-Pegel (m, real, keyless) → Registratur-Pflicht,
  aber kein Parser-Arm. **Riss:** `goes_euvs` CDN-Tag `ssd.jpl.nasa.gov` (Probe,
  `solar_causal_graph_probe.rs:15`) ≠ Register `ncei.noaa.gov` (`sources.φ:24775`).
- **Blockade:** trishuli kein Parser-Arm; goes_euvs Tag-Riss
- **Braucht:** trishuli `format`/HTML-Parser-Arm (sonst `blocked parser-def`);
  goes_euvs Tag-Riss an River/Rat.

### em-Apertur `(1+z)⁻²` — Quellen-Identität (Riss)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05 via grind-flash) Die sieben genannten z-tragenden
  em-Felder: `:7088`/`:9681`/`:9692` NED, `:9579` PSZ2, `:1183` TNS, `:17694`
  sncat, `:17705` swiftgrb — sechs halten; **Riss 1:** `:1183` TNS trägt `τ 3600`
  bei Block-`ttl 3600` und der `tns_compiler` zielt auf die API, nicht die
  statische CSV (unverifiziert); **Riss 2:** die Apertur erreicht **≥25** em-Felder
  mit `z`, nicht sieben (`sources.φ:8814-8820` bestätigt).
- **Blockade:** keine (Messung); `field z`-Anwendung (`spatial.rs:718`) ist Rats-Entscheid
- **Braucht:** für `:1183` keyed-POST der statischen CSV messen oder `τ` aus
  Prozesswissen; den ≥25-Felder-Riss an River/Rat tragen.

### iEEG 4D-Anker — BIDS-Sidecar (Elektroden)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `https://www.ieeg.org/services` wieder 200
- **Lage:** (gemessen 2026-10-05 via grind-flash) `ieeg` nicht in `sources.φ`;
  `blocked_sources.φ:154-156` `pending`; Arm `ieeg_compiler.rs` (REST `/services`,
  **keine** Koordinaten); `IEEG_USER`/`IEEG_PASS` vorhanden; Server 503; kein
  Datensatz mit echten MNI-Koordinaten.
- **Blockade:** Server 503; kein `SourceConfig`-Sidecar-URL-Feld
- **Braucht:** `SourceConfig`-Sidecar-Feld + Arm, der `*_electrodes.tsv` +
  `*_coordsystem.json` lädt → `edf_emit_channels_with_electrodes`;
  `Position::Electrode`→ICRS (`channels.rs:1298`); Re-Dispatch bei Server 200.

### absorption-/advection-Slots — erzwungene `0.0` statt `absent` (Fabrikation; Rat: Presence-Bit)
- **Status:** eigen | **Bindung:** eigen (Archivar) + An river (membrane)
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05) `parse_field_config` (`src/archivar/parse.rs:1603-1619`)
  liest `absorption`/`advection` als **pflichtige** f64 (9-Token-Form kodifiziert in
  `phi/pipeline/prompt.φ:18`, `docs/SOURCE_PORT.md:189`); das Format kennt **kein `absent`**,
  darum tragen alle 9059 field-erzeugenden Direktiven `0.0 0.0` — nicht von einem
  gemessenen Nullwert unterscheidbar (`unwrap_or(0.0)` in anderem Gewand). Der Record ist
  fixed-stride 26 × f64, die Slots **müssen** gefüllt werden; die GPU liest sie real
  (`shaders.rs:187/213` `force_absorption(ft, tm.w, d)`, `:129` `propagation_v`
  `ft==7 && advection > 0.0`). `flat_propagation_speed` (`membrane.rs:488-492`)
  substituiert bei `advection = 0.0` `ADVECTIVE_BASE_SPEED = 1.0` (`:424`) — eine
  erfundene Geschwindigkeit; Test `:765-768` zementiert sie. `FieldConfig.absorption: f64`
  (`types.rs:296`).
- **Blockade:** keine (Archivar-Seite); membrane.rs ist Rivers Pfad
- **Braucht:** **Rat-Verdikt (2026-10-05, Rat + research-max + `--exa`): Träger ist ein
  Presence-Bit, kein Sentinel.** Bit-Feld im bestehenden Presence-Slot (Slot 26,
  `meta[14]`, Byte 200): Bit 0 = Phase (Bestand), Bit 1 = Absorption, Bit 2 = Advektion;
  Wert-Slots tragen Pad `0.0`, der Leser liest das Bit, nie den Pad — keine
  Wire-Längenänderung. Gemessene Präzedenz: CF/`_FillValue` liegt außerhalb `valid_range`
  und **out-of-band** (der fixed-stride Frame hat diesen Kanal nicht); FITS §10.2.2
  Integer-Maske, CDF VXR-Written-Index, GHCN Flag-Spalten, NumPy masked arrays, HDF5
  `H5Pfill_value_defined` — alle setzen die Presence neben den Wert, wo ein Sentinel
  nicht trägt.
  **Online-Stimmen (Browser-Bridge, 2026-10-05):** **Claude (Sonnet 5.5)** empfiehlt
  **Sentinel (A)** mit Bedingungen (beide Größen ≥0 → negativer Wert kollisionsfrei;
  Sentinel einmal in der Spec deklariert, nicht pro Record; Leser prüft `< 0` vor jeder
  Arithmetik) und nennt (B) die schwächere Wahl (alle 26 Slots belegt; Bitmuster per
  Reinterpretation könne NaN/Denormal erzeugen und werde bei Skalierung/Interpolation
  still zerstört). **GLM-5.3 (Deep Think Max)** empfiehlt **(C): ein eigener Masken-/
  Flag-Slot neben den 26 floats** (z. B. uint32, 26 Bits) — das B-Konzept, aber nicht im
  Messwert-Slot versteckt; Muster: CF flag variables, NASA/ESA quality flags,
  GRIB-Bitmaps, CCSDS-Validitätsbits; Sentinel (A) nennt GLM ein Magic-Value-Anti-Pattern
  (kollabiert mehrere Zustände, überlebt Mittelung/Skalierung ungeprüft), (B) im
  Messwert-Slot sei Typmissbrauch. **Kimi K3 (via everask, 2026-10-05):** empfiehlt
  **Presence-Bit (B)**, möglichst in einem **eigenen Integer-Feld**, nicht in einem der
  26 f64-Slots (ein Bitmuster im float64-Slot zerstört die „Slot = physikalische Größe"-
  Semantik, exakt bis 2⁵³, aber ein Hack); (A) nur akzeptabel bei Header/Schema-Deklaration
  außerhalb `valid_range`. **tryingopen/Kimi K3** = out of API credit (Retry bestätigt);
  **kimi.ai/Kimi 2.6** = Gratis-Kontingent aufgebraucht (Reset 10-25).
  **Klärung (2026-10-05, Code + Wissenschaft):** Slot 26 `presence` ist bereits der
  **dedizierte Flag-Kanal** (`archivar-mathematikerin.md:53-55`; GPU liest
  `props[id*4+3].z`→`pb.z`, Test `> 0.5` in `shaders.rs:387`) — **kein Messwert-Slot**.
  GLM/Kimis Einwand („Bit in einem der 26 f64-Slots") trifft einen Messwert-Slot und damit
  nicht den Vorschlag des Rats; ihr „eigenes Integer-Feld" **ist** Slot 26. Wissenschaft:
  CF §3.5 Flags + CF-Ticket #26 (Bitfelder) → **`flag_masks`: ein dediziertes Flag-Feld
  trägt mehrere Bits** (`--exa`, cfconventions.org ch03s05; GRIB-Bitmap, CCSDS/PDS4-Qualität
  ebenso). **Auflösung:** Bitmaske in Slot 26 (Phase Bit 0, Absorption Bit 1, Advektion
  Bit 2) — keine Längenänderung, kein 27. Feld; Claudes Sentinel bleibt nur deklarierter
  Fallback. Rest: der Phase-Test `> 0.5` wird ein Maskentest.
  Archivar-Brücke gebaut (2026-10-05): 7-Token-`field`
  parst absent, `SLOT_ABSENT = -1.0` + Accessoren (`types.rs:308`), Test
  `parse.rs::field_without_absorption_advection_declares_absent_not_zero`; `cargo check
  --tests` 0/0 — `SLOT_ABSENT` ist CPU-Brücke und reitet **nie** den Draht.
  Bau (nächster Schritt): (1) `PRESENCE_FLAG_{PHASE,ABSORPTION,ADVECTION} = 1/2/4` benannt,
  nie nacktes `4.0`; (2) Archivar schreibt Flag + Pad 0.0; (3) Mathematikerin/WGSL liest
  das Bit vor `tm.w`/`fm.x` und überspringt den Term bei 0; (4) `ADVECTIVE_BASE_SPEED = 1.0`
  fällt (`membrane.rs:424/491/767/789`, `PROPAGATION_SPEED[7]`); (5) Migration der 9059
  Zeilen: Standard `absent`, eine Zahl bleibt nur bei Neu-Messung (Register-Akt, kein `sed`);
  (6) Vertragstext in `docs/concepts/archivar-mathematikerin.md`.

### Field_te_query-Kanalquellen — Ernte-Arme
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05 via grind-flash) **PDO/AMO/Oulu sind `declined`**
  (`declined_sources.φ:2840`/`:3092` `aggregate-index` positionslos; `:5380`
  `superseded`); NINO3.4 trägt live `ersstv5_nino34` (`sources.φ:18536`); SOI ist
  positionslos und `thermal 1` fällt das Physik-Gate → **keine** neue Zeile.
  Die dafür gebauten Arme wurden wieder entfernt (Freigabe mit Messung). **IERS
  EOP LOD** ist registriert (siehe oben). Offen bleiben nur echte Parser-Gaps:
  **NOAA GODAS** und **GISTEMP Vulkan-AOD** (netCDF/OPeNDAP → `nc4.rs`/`opendap.rs`);
  **BOM RMM** Host-Block.
- **Blockade:** netCDF/OPeNDAP-Arm; BOM RMM §95a-Frage
- **Braucht:** netCDF-Arm (Substrat `src/archivar/nc4.rs`); BOM RMM
  `proton-wg.sh suggest www.bom.gov.au` = Operator-Wort.

## Träger (Prosa, eigene)

- `docs/surveys/survey-2026-10-03-medizinische-datenquellen.md` (`class: survey`, Header-sha `feb28078…`) — Pool disponiert (2 admit / 3 hold / ~97 declined).
- `docs/specs/sources-v2-spec.md` (`class: concept`, Header-sha `11c6db3c…`) — `range`-Zeile trägt den Rat-Befund (kein Wire-Slot).
- `docs/surveys/survey-raetsel-bestand.md` (`class: survey`, Header-sha `524d61dc…`).
- `docs/blatt/blatt-pioneer-floor-falsifikation.md` (`class: sheet`, Header-sha `5bb1696b…`).
- `docs/concepts/kybernetische-astrophysik.md` (`class: concept`).
- `docs/concepts/tools-map.md` (`class: concept`).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (`class: survey`).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` (`class: survey`, Header-sha `ac672e3e…`).

## An mycelium

Origin: mountain folge236.

- **FMI-IMAGE-Magnetometer NUR** — Arm + Register gebaut (`sources.φ:27042-27048`,
  `image_mag_compiler.rs`). Braucht: `image-cdn`-Workflow (Muster `fmi-gic-cdn.yml`)
  + `gh workflow run`; Asset `fmi_image_mag_nur.bin` auf Release `space.fmi.fi`.
- **IERS EOP C04 LOD** — Arm + Register gebaut (`sources.φ:27050-27053`,
  `iers_eop_c04_compiler.rs`). Braucht: CDN-Manifestation.
- **gbco axis-value Serie** — `gbco_series_compiler.rs` gebaut. Braucht:
  Manifestation `gbco_elevation_gebco_{2023,2026}.txt` auf `data.ceda.ac.uk-gebco`.
- **EMM (Rat-Verdikt: Bildquelle ohne `field`):** `emm_sdc_compiler.rs` von
  mycelium 232c committet; offen bleibt der `emm-sdc-cdn.yml`-Angleiche +
  Frame-Bundle-Arm + `format emm_exi_l2a`-Block.
- **superdarn-Tag** auf `pending` (`wartend.φ:8`); RST-Ernte über Globus offen.

## An river

Origin: mountain folge236.

- **Enclosure-Riss:** Die Span-Probe bricht Icarus (1.076×) und Phaethon (1.546×)
  über `64·ttl` (`spatial.rs:863`); das Bound muss auf ein Per-Body-Spannenmaximum
  wandern. Test `src/archivar/tests.rs:3336`.
- **main_flow:** der Loader-Token `"fmi_image_mag"` wurde in `src/archivar/
  main_flow.rs:4118` ergänzt (mechanisch, für die IMAGE-Ladebarkeit) — dein Pfad,
  bitte prüfen/übernehmen.
- **declustered Mainshock:** `erbq_mainshock` (`witnesses.φ:180-184`, 153/198) ist
  unverdrahtet an die ETA-Form; Compiler `erbq_mainshock_compiler.rs` reproduziert es.
- **em-Apertur-Riss:** Apertur erreicht ≥25 em-Felder mit `z` (`sources.φ:8814-8820`),
  nicht die sieben aus river-folge93 §Zeugen; beide Linien benannt, nicht geglättet.
- **goes_euvs Tag-Riss:** Probe `ssd.jpl.nasa.gov/goes_euvs.bin` vs Register
  `ncei.noaa.gov/goes_euvs.bin` (`sources.φ:24775`).
- **`ADVECTIVE_BASE_SPEED = 1.0` — erfundener Default:** `flat_propagation_speed`
  (`membrane.rs:488-492`) gibt bei `advection = 0.0` `Some(1.0)` statt absent;
  das Register kann Advektion nicht als `absent` deklarieren (`parse.rs:1603-1619`).
  Rat-Verdikt 2026-10-05: **Presence-Bit** (Bit 2, Slot 26/`meta[14]`) statt Sentinel;
  die `1.0`-Substitution fällt, abwesende Advektion = kein Term, gemessene 0 = Speed 0.
  Bitte die Bit-Lesung in `shaders.rs`/`membrane.rs` bauen (Test `:765-768`).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02,
  river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate
  erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent,
  nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut,
  `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer
  Kovariat-Varianz. Der private Wort-Laut nur im privaten
  `state/operator-gespraeche/`.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und
Push. `/consent` ist der session-weite Consent, nie das Commit-Wort.
