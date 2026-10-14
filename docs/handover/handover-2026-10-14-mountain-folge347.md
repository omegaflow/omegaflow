<!--
  title: Handover — Mountain-Folge 347 (2026-10-14)
  session: Mountain-Linie — ein Pass: der ci-gate-register-Rot geheilt (fünf stale CDN-Workflows entfernt), vier Taucher, dann gebaut — der SABER-Zeilen-Arm + der TIMED-TLE/SGP4-Compiler; opentopography terms gesetzt
  class: handover
  date: 2026-10-14
  sha256: 6cb58b6fdc10f542c34859308f1dec0357e2e7ae53ea892df1446d150a1019c1
  status: live
  see-also: docs/concepts/tool-forms.md, AGENTS.md
-->
# Handover — Mountain-Folge 347 (2026-10-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Diese
Session konsumierte `handover-2026-10-14-mountain-folge346.md` (→ `archiv/`).
Gefaltet (`register_lookup --addressed mountain`): `mycelium-folge348`/`-349` und
`river-folge211`. Still gemessen: `register_lookup --fired mountain` = 0,
`--stale mountain --persist 3` = 0, `open_points_check` sauber (17 Pfad-Refs,
0 absent/stale/word-carried/format-gap). Der Stehende Pass wird als **öffentliche**
Fassung zitiert, nie kopiert: HEAD `6b5aaf2c1` (2026-10-14).

## Burn: open 0.0000 · close nicht gemessen — `session_burn` meldet `db absent` (`~/.local/share/opencode/opencode.db` fehlt), der Wert ist kein Meßwert. Grund: der `cdn_reconcile`-Register-Rot geheilt (fuenf stale CDN-Workflows), drei Format-Lücken der folge346 geschlossen, vier Taucher (CDN-Quellen, SABER-Arm, TIMED/SGP4, kleine Registermessungen), dann gebaut: der SABER-Zeilen-Arm (`extract.rs`) + der TIMED-TLE/SGP4-Compiler (`tle.rs`, `timed_tle_compiler.rs`), `opentopography_coverage`-`terms` gesetzt; kein Rat.

## Offen (aufgeschlüsselt)

### SABER — verdiktet: B + A (eigene Weltlinie + Messvolumen) (eigen)
- **Status:** eigen · **verdiktet** (Rat 2026-10-14: B + A) | **Bindung:** eigen
- **Trigger:** der Arm-Bau
- **Lage:** (gemessen 2026-10-14, Taucher) Das HAPI-`info` (live HTTP 200, 7489 B —
  die Host-Erreichbarkeit ist **intermittierend**, der Cache `/tmp/opencode/saber_info.json`
  ist aktuell absent) nennt neben `ktemp`/`tpgpaltitude` **eine Scan-Mittel-Position**:
  `sclatitudeAVG`/`sclongitudeAVG` (deg) und `scaltitudeAVG` (**km**), dazu
  `tpl…`-Tangentialpunkt-Geometrie. Das Produkt trägt **kein** kartesisches xyz und
  **keine** Geschwindigkeit; Zeit ist `Time` (isotime, UTC). Der Arm heute
  (`extract.rs:6994-7006` family, `:7083-7095` scalar) liest **nur die letzte Zeile**
  (`data.last()`) und stempelt `epoch: now` — die `Time`-Spalte wird verworfen,
  `Position::Source` (unit-Variante, `types.rs:67`) gesetzt. Kein SABER-Block in `phi`.
  Korrigiert: die folge345-Zitation `extract.rs:7013-7025` für `Position::Source` ist
  **stale** (dort steht heute der `param`-Komponentenparse).
- **Präzedenz (gemessen):** eine eigene Weltlinie läuft im Haus **nicht** über
  `Position::StateVector { p, v }`, sondern über eine **Ephemeride**: `at <body>` →
  `Frame::Barycenter` → `Motion::Barycenter` → `body_barycenter_position` (Chebyshev,
  ICRF/J2000, **Meter**, TDB-Sekunden seit J2000), geübt von **Voyager**
  (`phi:27461-27466` Ephemeride + `phi:11603-11613` Datenblock, beide `at voyager1`).
  ISS hat nur die Ephemeriden-Seite, keine Kanäle. `Position::StateVector { p,v }` +
  `Motion::Linear` ist verdrahtet (`channels.rs:1408-1409`, `motion.rs:436-439`), hat aber
  **keinen** Produktionsproduzenten und nie `track: true`.
- **Route B ist NICHT quellen-blockiert — korrigiert (2026-10-14, `archive_search --all`):**
  TIMED ist **NORAD 26998** (2001-055B; Bahn ~615 km, i 74.1°), und **TLEs existieren**
  (CelesTrak/Space-Track; Tracker `in-the-sky.org/spacecraft.php?id=26998`,
  `orbitalradar.com/satellite/26998`, `n2yo.com/satellite/?s=26998`,
  `satellitemap.space/sat/26998`). Eine **TLE/SGP4-eigene Bahn** ist damit baubar
  (Kilometer-Klasse) und legt TIMED als **Peer-Körper** an. Zusätzlich gefunden:
  NTRS `20040082147` „Orbit Determination of the TIMED Mission Using TDRSS Differenced
  One-way Doppler Tracking Data"; `timedsds.jhuapl.edu`; `helio.data.nasa.gov/mission/TIMED`.
  **Was zuerst gemessen wurde (unvollständig):** `--verdict …/naif/TIMED/kernels/spk/` →
  **HTTP 404**; NAIF-Wurzel (200) → kein `TIMED/`; Horizons `COMMAND='TIMED'` → `No matches`.
  Das prüfte nur die **SPK-Güte** (NAIF/Horizons) und übersah die TLE-Stufe — der erste
  „blockiert"-Schluss war eine **unvollständige Messung, nicht die Wahrheit** (A=A).
  **TLE verifiziert (Browser, Wayback 2026-02-22, Status 200; CelesTrak live Timeout):**
  `TIMED` · `1 26998U 01055B 26052.51864582 .00001123 …` ·
  `2 26998 74.0671 107.8683 0002724 294.3335 65.7565 14.94968599313531`
  (i 74.07°, ~14.95 Umläufe/Tag) — die eigene Bahn ist damit belegt.
  **Live-TLE:** CelesTrak direkt **nicht erreichbar** (Browser + curl 60 s Timeout; auch durch
  den Proton-Ausgang TLS-Reset `SSL_ERROR_SYSCALL`, während JPL 200 liefert). **Durch Jina
  geht es:** `archive_search --jina "https://celestrak.org/NORAD/elements/gp.php?CATNR=26998&FORMAT=tle"`
  → die Live-TLE, Epoche `26287.22414614` (= 2026-10-14), identisch mit
  `n2yo.com/satellite/?s=26998` (AFSPC). Der Endpunkt trägt:
  `1 26998U 01055B 26287.22414614 .00001152 …` ·
  `2 26998 74.0694 355.2967 0000920 228.8676 131.2429 14.95511591348618`.
  **Historische Reihe 2002–2026 (gefunden, `--all` + `--exa`):** HuggingFace
  **`juliensimon/space-track-tle-history`** — **CC-BY-4.0**, „complete archive … every tracked
  object in Earth orbit, 1959 to 2026, sourced from Space-Track.org bulk exports", parquet je
  Jahr (`data/tle_2002.parquet … tle_2026.parquet`), Spalten `norad_id, epoch, inclination,
  raan, eccentricity, arg_perigee, mean_anomaly, mean_motion, bstar, intl_designator,
  altitude_km`; TLE = `norad_id 26998`. Daneben CelesTrak `NORAD/archives/` (hier unerreichbar)
  und Space-Track selbst (Login).
- **Exaktheit (Operator-Kriterium):** A ist exakt am **vermessenen Ort** (Limb-Sounder: der
  Tangentialpunkt `tpl…`/`tpgpaltitude` ist die Messstelle, nicht der Satellit) — aber
  scan-gemittelt, ohne Datum, ohne Sub-Scan-Zeit. **B (TLE/SGP4)** gäbe eine kontinuierliche
  Bahn mit `v` über die ganze Mission — **Kilometer-Klasse**, datum-/ICRS-frei — und wäre in
  **Zeit/Dynamik** exakter als A.
  **Ephemeriden-Nutzen:** B legt einen Peer-Körper in `body_ephemerides` (heute 92) an;
  A legt keinen an.
- **Roster (mountain-ui, 12 Frontier-Sitze gestellt, 2026-10-14):** Frage A/B mit der
  vollständigen Recherche. Geantwortet **B**: Claude, Mistral, Qwen, Perplexity,
  Z.ai/GLM-Deep-Think-Max, MiniMax (6); **A**: Lumo, Sakana (2); ohne Antwort: DeepSeek,
  Duck (Frage nicht gepostet), Kimi (Freikontingent erschöpft), Gemini. Der **erste** Anlauf
  (falsche Prämisse „keine Bahn") war 4× A — die Prämisse war der Ausschlag.
- **Rats-Verdikt (2026-10-14): B + A.** Nicht zwei Antworten auf eine Frage, sondern **zwei
  Objekte**: **B** trägt die Plattform (TIMED, NORAD 26998) als **eigene Weltlinie** und
  **93. Peer-Körper**; **A** trägt das **vermessene Volumen je Zeile**. **Riss (entscheidend):**
  ein Limb-Sounder misst am **Tangentialvolumen**, nicht am Satelliten — beide dürfen nicht
  füreinander stehen. **Datum-Riss:** das Produkt-Frame ist **undeclared** → `pending`.
- **Blockade:** keine (der Bau ist offen, kein externes Tor).
- **Braucht:**
  - **SABER-Block:** `at earth` behalten; **je Zeile und je Level** extrahieren — `ktemp` u. a.,
    `epoch` aus `isotime` (UTC→TDB), die **Tangentialgeometrie** `tplatitudeAVG`/
    `tplongitudeAVG`/`tpgpaltitude` über den geteilten Level-Index `<i>`;
    Position = **`Surface { earth, tplat, tplon, tpgpalt (m) }`** (nicht `Position::Source`);
    **Datum `pending`** als Registerzeile; die `sc*`-Spalten sind **nur Querzeuge**, nie Position.
  - **TIMED-Weltlinie (eigener Körper):** body `timed`, NORAD 26998 (2001-055B); zwei Quellen:
    CelesTrak-Live-TLE (kurzes `ttl`) + Space-Track-Historie-Spiegel (CC-BY-4.0). Arm: **SGP4**
    → `StateVector { p, v }` ICRS/J2000, Meter, TDB — **erster Produzent** des verdrahteten,
    bislang unproduzierten `Motion::Linear`; unabhängige Hülle → 93. Peer.
  - **Extrakt-Arm:** `Extract::Hapi` darf für ein Zeilenprodukt **nicht** auf `data.last()`
    kollabieren; SABER nimmt den `parameters`/info-Pfad, jede Zeile liefert Kanäle mit
    **eigenem Zeilen-Epoch** und **eigener Tangentialposition**; Last-Row/`epoch=now` bleibt nur
    für Einzelskalar-Endpunkte. `src/archivar/tests.rs:786` (`Position::Source`-Assertion) auf
    Tangential-`Surface` + Zeilen-Epoch ziehen.
  - **Riss-Gate:** divergieren `sc*`-Punkt und SGP4-Weltlinie zur selben Epoche über das Gate,
    als `VerdictWord::Riss` mit beiden Zeugen tragen — nie glätten.

### Venus Radar — Projektion gemessen; Bild-Manifestation offen (eigen)
- **Status:** eigen (Operator-Wort 2026-10-14: W — das Bild bleibt Bild) | **Bindung:** eigen
- **Trigger:** der Projektions-/Manifestations-Bau
- **Lage:** (gemessen 2026-10-14, Taucher) Bundle `urn:nasa:pds:venus_radar_level2` v1.0,
  DOI `10.17189/1522443`, **CC0-1.0**, 15 Produkte (5 Epochen × `inc`/`ocp`/`scp`).
  Die Level-2-Labels sind PDS4 `Array_2D_Map`, single-band; `pds4_img` braucht keinen
  Interleave-Umbau. Der `SAMPLE_INTERLEAVED`/`BANDS=2`-Befund (mycelium-346-6) gehört zum
  **rohen** PDS3 Complex-Backscatter-Bundle (`vrm_90xx`).
  **Projektion gemessen** (byte-identisch über alle Epochen, aus
  `.../data/venus_1988_north_inc.xml`): `map_projection_name` = **Sinusoidal**,
  `longitude_of_central_meridian` = **335.0 deg**; `pixel_resolution_x/y` = 1000 m/px;
  `upperleft_corner_x/y` = `-10000000.0` / `9359753.174046` m; Bounding-Box
  W 254.256 / E 37.213 / N 80.845 / S -25.0 deg; `Geodetic_Model`: `Planetocentric`,
  a=b=c=`6051.0` km, `Positive East`. **Kein** expliziter Frame-Identifier (absent) —
  der Frame ist nur über Körper-fix + `longitude_direction` konveyiert.
- **Gebaut (mountain-346):** `pds3_img` liest `BAND_STORAGE_TYPE` (BSQ/BIL/BIP);
  `decode_raster` indiziert danach; unbekannter Token → `UnknownInterleave` (kein
  stiller BSQ-Fallback); `pds3_bip_radar_compiler` delegiert an den Kern (Reshuffle weg).
- **Blockade:** keine.
- **Braucht:** (1) die **Bild-Manifestation (W)** wählen — wie das georeferenzierte
  `Array_2D_Map` als Gestalt/`pds4_img` in den Baum kommt; (2) den Block
  (`url`/`origin`/`compiler`/`terms CC0-1.0`, Einheit am Feld); (3) Split-Route erst
  nachgelagert; (4) der native Lauf braucht 15×960 MB (CI). Der D-Compiler
  (`venus_radar_compiler.rs`) und der VNRD-Reader (`vnrd.rs`) bleiben **geparkt**.

### ICGEM gfc — Rezept + Site gemessen; Auswerter wartet auf die Epochen-Ablage (eigen)
- **Status:** eigen (Operator-Wort 2026-10-14: das Modell wie die anderen behandeln) | **Bindung:** eigen
- **Trigger:** der Feld-Bau
- **Lage:** (gemessen 2026-10-14, Taucher) Das `gfc`-Format (ICGEM-Format-2023,
  `https://icgem.gfz.de/home` → Spec-PDF) trägt Header `product_type`, `modelname`,
  `earth_gravity_constant` (GM, m³/s²), `radius` (m), `max_degree`, `errors`,
  optional `norm` (fully normalized) / `tide_system`; Datensätze `gfc` (statische
  C_lm/S_lm) und `gfct` (zeitvariabel). Die **Synthese** ist die Standard-Kugelflächen-
  Entwicklung `V(r,θ,λ) = (GM/r)·Σ_l (a/r)^l ·Σ_m P̄_lm(cosθ)(C_lm cos mλ + S_lm sin mλ)`;
  das Feld = `∇V` (m/s²), Potenzial m²/s². **Präzedenz:** `IgrfCoeffs::synthesize`
  (`igrf.rs:131`), `vmf3_sh::b_c` (`vmf3_sh.rs:17`) + `vmf3.rs`-Produkte — beide aber
  **ohne** Runtime-Dispatch (nur Test bzw. ein Meß-Zeuge im `measure`-Crate). `gfc.rs` ist heute
  Parser+Channels ohne Auswerter; die Header-Konstanten (GM/radius) liest der Compiler
  und **verwirft** sie (nicht im Frame). Empfohlene Site: eine `pub fn` in `src/archivar/gfc.rs`
  (Geschwister von `synthesize`/`b_c`), Eingang Koeffizienten (C/S,n,m) + GM/radius/norm/
  tide_system, plus Dispatch-Erweiterung in `extract.rs`/`main_flow.rs:3111-3226`.
- **Riss:** der Frame trägt GM/radius **nicht** — der Auswerter kann ohne sie nicht
  rechnen (Frame-Header oder Begleitgröße nötig); `norm` „fully normalized" (4π) ≠
  IGRF-Schmidt; `product_type` muss verzweigen (Gravitation vs. Ozeangezeiten); der
  Feldwert ist vektoriell (0c), die Höhenbezugsfläche (0b) und die Bahn↔Feld-Kopplung
  (0a) bleiben Bedingungen.
- **Blockade:** die **Epochen-Absenz** (eigener Punkt) — ein statisches `gfc` hat keine
  Zeit; ohne die Ablage würde der Pad `0.0` als J2000 gelesen.
- **Braucht:** (0) die Epochen-Ablage umsetzen; (1) GM/radius/norm/tide in den Frame
  oder an den Auswerter reichen; (2) die Kugelflächen-Synthese in `gfc.rs`; (3) Block
  `url …/icgem.gfz.de/gfc.bin`, `terms CC-BY-4.0 https://doi.org/10.5880/ICGEM.2019.007`.

### Epochen-Absenz — Rats-Verdikt (b); Umsetzung offen (eigen)
- **Status:** eigen · **verdiktet** (Rat 2026-10-14, Verdikt b) · **nach der Rat-Regel erneut
  zu befragen** (Recherche + voller Roster fehlten) | **Bindung:** eigen
- **Trigger:** der Registeratom / die nächste epochenlose Quelle
- **Lage:** (gemessen 2026-10-14, Taucher + Rat) Zwei Leselinien widersprechen sich an
  **demselben Wert 0.0**: Linie A liest `epoch <= 0.0` als **absent** und überspringt
  (`src/mathematikerin/machines/matrix.rs:644`; `src/archivar/drs_fits.rs:38`); Linie B
  behält `0.0` als Wert (`agrav.rs:49`; `hips.rs:793` Bandindex 0; `gfc.rs:153`).
  Entscheidend: `epoch` ist TDB-Sekunden seit J2000 — **`0.0` ist J2000.0**, ein echter
  Wert; `< 0.0` ist Prä-J2000 (bereits als Wert behandelt, `tests.rs:1115`). Linie A
  ist damit eine **Fabrikation** (sie löscht den Ursprung und die Vorgeschichte).
- **Rats-Verdikt (b):** Absenz **ausdrücklich tragen**, über das schon vorhandene
  Presence-Bit-Mask — neuer `PRESENCE_FLAG_EPOCH` (32.0), **kein** Wire-Umbau (26×f64,
  208 B bleiben). Static `gfc` → Bit aus (Pad 0.0 als *absent* gelesen, Manifestation zur
  Query-Zeit); `gfct` → Bit an (0.0 = J2000 legitim).
- **Offen (Operator-Regel, Commit `94c6eaa10`):** dieses Verdikt fiel **vor** der Regel
  „Rat MUSS gefüttert werden" — nur der Übergabe-Stand, **keine** `archive_search`-Recherche,
  **kein** Roster (`work/ui-seats.md`). Nach der Regel erneut befragen, bevor es bindet.
- **Blockade:** die Umsetzung berührt viele Sites (Write- und Read-Seite) — kein
  Teilumbau möglich ohne die Gegen-Fabrikation (ein Reader-only-Schnitt würde absent als
  J2000 lesen).
- **Braucht:** die Sites des Verdikts: `types.rs` `PRESENCE_FLAG_EPOCH` + `presence_flags`
  (um den Epoch-Parameter erweitern; die 4 Write-Aufrufer `spatial.rs:845/1004`,
  `membrane.rs:130`, Test in `parse.rs:3296`), Read-Seite `matrix.rs:644` (Bit statt
  `<= 0.0`), `drs_fits.rs:38` (nur `!is_finite`), `agrav.rs`-Write (Bit setzen, 0.0
  legal), `gfc.rs:153` (Bit lesen) **und** `gfc.rs:101-107` Frame-Fold (der statische Pad
  darf nicht als Frame-Epoch erscheinen), `hips.rs:793` (Bandindex → Bit an),
  `static/constants.js` (Bit 32 sichtbar machen), `docs/concepts/archivar-mathematikerin.md`
  (Presence-Klausel). **Erst dann** die `gfc`-Manifestation.

### UCAR-FPI — Arm + Zeilen stehen, Primärkoordinate offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `field`-Zeilen aus dem Sample
- **Lage:** (gemessen 2026-10-14) Arm + Guard (`fpi_ucar.rs`, Dispatch `extract.rs:159`).
  Sample `data/registration.hao.ucar.edu/20246300athabasca.dat`, 7353 Zeilen, 10
  leerzeichengetrennte Spalten. Lizenz gemessen: keine benannte (UCAR/DASH-Nutzungssatz),
  terms-URL `https://data.ucar.edu/dataset/fpi-resolute`. **Riss:** kein Athabasca-FPI-DOI.
  Die zwei `field`-Zeilen sind code-gemessen (`fpi_mer_wind_ms` `fpi_ucar.rs:10`,
  `fpi_zon_wind_ms` `fpi_ucar.rs:12`; `extract.rs:534/535`).
- **Blockade:** nicht-offene Lizenz → `local-only` (kein CDN); die FPI-eigene
  Athabasca-Instrumentenkoordinate ist ungemessen (die File-/DASH-Seiten tragen nur
  Palmer/Resolute/Boulder; der einzige Zahlenwert löst auf AUGO auf — Riss, ungeglättet).
- **Braucht:** die **FPI-eigene** Athabasca-Instrumentenkoordinate an der Primärquelle
  (`qwu@ucar.edu` / DASH-Resource-Support) messen; dann die zwei `field`-Zeilen + die
  Arbeitskopie-Route (`local-only`).

### Fünf fehlende Fäden — Entwurf steht, Mycelium setzt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14, Taucher) Mountain-Teil fertig als Entwurf
  `state/mountain/faden-term-field-ttl-draft.φ`: je Faden die gemessene
  `terms`/`field`/`ttl`-Zeile (Sonnen-Magnetfeld, Stern-Aktivität, 21 cm, Wolken, TGF),
  Haus-Token-Korrekturen belegt (`sdo`→`CC0-1.0`, `vizier-open`→`vizier-scientific-free`,
  `heasarc-free`→`PD`, `NOAA-CDR-open`→`PD`, Zenodo→`CC-BY-4.0`). Zwei Risse offen
  (WSO-Polarfeld ohne Terms; LISIRD ohne Terms), 21 cm EBHIS bereits registriert.
- **Blockade:** keine.
- **Braucht:** Mycelium setzt `url`/`origin`/`compiler` je Faden auf den Entwurf; dann
  `register_sort`.

### Quellen-Gewebe — Kraft × Kernel gemessen; die dünnen Fäden (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14, HEAD `fc9f3dba2`; Kreuzprüfung `source_physics_lint`)
  die zwei Messungen am Register — Herkunft × Kraft und Kraft × Kernel — liegen in
  `docs/surveys/survey-2026-10-14-gewebe-faeden.md`: 2575 `url`-Zeilen, 124 rohe Hosts,
  306 wirksame Herkünfte, 7687 Feld-Zeilen; `inverse-square` trägt 6812 (89 %);
  Gravitation (13 Felder) und Seismik (20) hängen an je **2** Wirkgesetzen.
- **Gemessen (2026-10-14, Taucher) — die Quellen sind meist schon da, die Kernel fehlen:**
  - **Gravimetrie ist im Register durchweg `inverse-square`** (IGETS `phi:10317`,
    AGrav `phi:9990`, GRACE-FO `phi:11972`, LLR) — die gesuchten **`point`/`exp.-decay`**
    sind über **jede** gemessene Gravimetrie-Zeile **abwesend**. Die Lücke ist **bestätigt,
    nicht gefüllt**.
  - **Schon registriert:** IGETS, AGrav, GRACE-FO, ISC (`phi:10662`/`12609`),
    EarthScope-FDSN (`phi:174`/`8191`), GEOFON-Event (`phi:10454`), EIDA-Terms (`phi:14525`).
  - **Offen/gesperrt:** EGGN/EPOS-Equip-G (kein Token, `pending`), CGrav/CEIC China
    (Konto-gated, Token ungemessen; re3data „open" ohne Lizenzwert = **Riss**), GOCE
    (ESA offen mit Attribution; exakter SPDX ungemessen, kein `phi`), NIED (Konto-gated,
    Weiterverbreitung untersagt), GSRAS (teils frei/teils bezahlt), ARDA (nicht zuzuordnen).
  - **Bereit, aber unregistriert:** GEOFON-Wellenform (`…/fdsnws/dataselect/1/query` →
    200, kein Key) — keine `phi`-Zeile (nur das Event-Catalogue).
  - **Stale Origins (Register-Korrektur):** IGETS `origin` = Projektseite, echter Endpunkt
    `igetsftp.gfz.de` (sftp, registriert); BGI-Host `www.bgi.omp.obs-mip.fr` tot →
    `bgi.obs-mip.fr`.
  - Kein Host IP-blockiert; `--jina` löste den `206`-Sonden-Quirk.
- **Blockade:** die Kernel-Zuordnung ist keine Pro-Solo-Entscheidung — das Meßgesetz einer
  Quelle berührt den Force-/Kernel-Kontrakt (Rat).
- **Braucht:** (0) IGETS-/BGI-`origin` korrigieren — **Myceliums Zeilenklasse**, an Mycelium
  geroutet (s. „An mycelium"); (1) den GEOFON-Wellenform-Block — `url`/`compiler` Mycelium,
  `terms`/`at`/Kernel liegen bereit; (2) die Kernel-Lücke schließen — Gravimetrie-Kandidaten
  mit eigenem Meßgesetz (`point`/`exp.-decay`) führen **oder** die `inverse-square`-Bindung
  als Verdikt tragen; (3) EGGN/China/GOCE/NIED/GSRAS/ARDA als `pending`/`riss` benennen.

### Adressierte Register-Fäden — Lizenzen gemessen, Verdikte offen (eigen)
- **Status:** eigen (Register-Writes; Messungen jetzt am Baum gegengeprüft) | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14 via `register_lookup --addressed mountain`, Taucher,
  und **direkt im Browser** auf den Seiten — JS-gerenderte Felder, die `archive_search`
  nicht las)
  - **MODVOLC HIGP** (`http://modis.higp.hawaii.edu/data/`): **keine** Lizenzzeile
    (genuine Absenz); nur `© University of Hawaii at Manoa, 2016` + NASA-MODIS-
    Förderhinweis. Kein HIGP-Block in `phi` (der NRT3-MODVOLC-Block `phi:12041` ist eine
    andere Quelle, `terms PD`). → `local-only`.
  - **DE-2 NACS/WATS — gesetzt (mountain-346).** SPASE-`RightsList` (Browser, `spase-metadata.org`
    …/DE2/WATS/PT2S): `RightsIdentifierScheme SPDX`, `RightsIdentifier CC0-1.0`,
    Note „… applicable to all publicly available NASA Heliophysics data products". Der
    `de2`-Block trägt jetzt `terms CC0-1.0` (`phi:854`, vormals `PD`).
  - **AWESOME-VLF/LF — gesetzt (mountain-346).** Browser auf
    `spase-metadata.org/ISWI/NumericalData/AWESOME/VLF/PT0.00001S.html`: JSON-LD
    `"license": "https://cdla.io/permissive-1-0/"`, DATA USAGE POLICY „WALDO data are
    released without restriction, and can be freely analyzed or published" (Danksagung
    erbeten). Exaktes Token **CDLA-Permissive-1.0** (nicht CC0). `phi:8266` (waldo) trägt
    jetzt `terms CDLA-Permissive-1.0 https://doi.org/10.48322/fzcf-f191` (vormals `CC0-1.0`).
  - **OCNS2019-Ersatz (JPL SSD / Horizons / NAIF) — gemessen (Browser).** Die JPL-SSD-Seite
    trägt **keine** Datum-Lizenzzeile (Footer: `URS Clearance: CL#21-4165`, Privacy, Image
    Policy); die Caltech/JPL-Notiz deckt **Dokumente/Bilder**, **nicht** die Daten. Die
    **Daten** fallen unter die NASA-SMD-Politik **SPD-41a**
    (`science.nasa.gov/researchers/science-information-policy/`): offen, kein Embargo.
    **NAIF** (`naif.jpl.nasa.gov/naif/credit.html`) bittet um Nennung, ohne
    Lizenz-Beschränkung. → **offen, Danksagung erbeten** (kein CC0-Zeichen auf der Seite).
- **Blockade:** MODVOLC braucht einen **neuen** Block (kein HIGP-Block in `phi`) — Mycelium
  setzt `url`/`origin`/`compiler`, Mountain das Terms-Verdikt.
- **Braucht:** MODVOLC-Block + `terms local-only`. DE-2 und WALDO sind gesetzt.
- **Für die eigenen Ephemeriden (Operator-Frage):** unsere Bahnen ruhen auf **JPL/NAIF** —
  die DE-Kerne (DE440/441/442 registriert), `horizons_compiler` (Horizons-API) und
  `spacecraft_ephemeris_compiler` (NAIF-SPK). Diese Daten sind **offen** (NASA SMD, SPD-41a),
  die Nennung erbeten — also für Bau und Weitergabe der eigenen Ephemeriden frei; die drei
  Punktquellen (MODVOLC/WALDO/DE-2) tragen dazu nichts bei.

### `em mw` / `advective kg/m2/s` — Freeze (eigen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das `tools-latest`-Republish
- **Lage:** (gemessen 2026-10-13) `units.rs` admiert beide;
  `docs/specs/force-unit-baseline.txt` trägt sie (plus `em mev`) froren.
- **Blockade:** keine.
- **Braucht:** die drei Paare nach dem Republish aus der Freeze nehmen.

### ALFALFA · Ersatz-Bau · eigene Ephemeride (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je der Bau/Fund
- **Lage:** (gemessen 2026-10-14, Taucher) ALFALFA `local-only` bestätigt — kein Block in
  `phi`, DOI `10.26093/cds/vizier.18610049` HTTP 200, ReadMe ohne Lizenzstring.
  Ersatz-Bau: `opentopography_raster_compiler.rs` liest SRTM15+ V2.6 und schreibt ein
  Gestalt-Witness (`SRTM`), **ohne** Geoid-/Datum-/Toleranz-Anker; kein
  `opentopography_raster`-Block in `phi` (nur `opentopography_coverage`, `srtm15_plus_*`).
  `load_gestalt_surface_threads` (`main_flow.rs:580`) — einziger Aufrufer `tests.rs`,
  **kein** Produktionsaufrufer. Eigene Ephemeride: `ilrs_npt_compiler.rs` steht
  (`FORMAT "ilrs_npt"`), aber **kein** Lese-Arm in `src/`; `icrf3_compiler.rs` schreibt
  JSON (Zielpfad absent), **kein** SKD1/Golden-File/Round-Trip.
- **Blockade:** keine.
- **Braucht:** (1) Höhenbezug + Toleranz vor der `opentopography_raster`-Registrierung
  festlegen (KAPU-Kontrollpunkte 179,94 m vs. 217,90 m), dann Block; (2)
  `load_gestalt_surface_threads` an einen Produktions-Aufrufer binden **oder** `descoped`;
  (3) `ilrs_npt`-Lese-Arm in `extract.rs` + Block; (4) `icrf3` erst Vertrag versionieren
  + Round-Trip (`skydirection.rs`) + Golden-File, dann SKD1. `docs/SOURCE_PORT.md`.

### Taucherstaffel 347 — Messungen und nächste Schritte (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau / die Registerfortschreibung
- **Lage:** (gemessen 2026-10-14, vier Taucher, HEAD `88e64f1a7`):
  - **Fuenf CDN-Quellen — geklaert.** Die fünf stale Workflows (`babamul-cdn.yml`,
    `celestrak-eop-cdn.yml`, `eve-cdn.yml`, `mars-dust-cdn.yml`, `maxi-cdn.yml`) sind entfernt
    (CDN-Zeile); `cdn_reconcile --fail` ist clean (324 Hosts). Die vier Quellen waren
    2026-10-13 fuer fehlende offene Lizenz aus `phi` entfernt (`1b519375b`, `9c60ddf89`),
    celestrak.org per EOP-Verdikt (`b138796d4`). Kein Baseline-Eintrag mehr noetig.
  - **`opentopography_coverage`** (`phi:11444-11449`): **kein `field`** (der Compiler schreibt
    nur `shard <href>`-Zeilen, `opentopography_coverage.rs:90-95`); **`terms PD
    https://opentopography.org/usageterms` gesetzt** (HTTP 200, „free of all copyright
    restrictions"). Riss benannt: derselbe Text nennt per-Dataset-Lizenzen (z. B. CC-BY-4.0) —
    das Katalog-Verdikt steht, die Einzel-Datensätze bleiben ein moeglicher Riss. Format-Drift:
    Compiler druckt `format coverage_manifest`, der Block traegt `format opentopography_coverage`.
  - **`catalog_mpcorb`** (`phi:3837-3843`): **kein `field`** (MPCORB-Records = Zeuge, wie
    `catalog_dastcom`/`catalog_dcom5`) → **descoped**. `cosmic_ro` traegt `quantity`+`field`
    (`phi:10190-10192`) → **descoped**; `heasarc_w3browse` ist ein Reader-Arm
    (`src/archivar/heasarc_w3browse.rs`), kein Block → **descoped**.
  - **Venus-Roh-Block fehlt** (`venus`/`backscatter`/`vrm` in `phi` = 0); `venus_radar_compiler.rs`
    schreibt `VNRD` (`vnrd.rs:32-38`), der Reader `format vnrd` ist verdrahtet
    (`extract.rs:164,948`), aber kein Block → **pending** (Mycelium `url`/`origin`/`compiler`,
    Mountain `terms`/`field`). Divergenz ungeglaettet: der Compiler liest **PDS4**
    (`urn:nasa:pds:venus_radar_level2`), der Handover nennt einen `pds3_img`-Blocker —
    `pds3_img.rs:522` mappt `SAMPLE_INTERLEAVED|BIP` seit `4149ee1f6` (2026-10-14), die
    Blocker-Zeile ist stale.
  - **SABER-Arm — GEBAUT (`ca35522f2`).** Die HAPI-Produktform ist live gemessen: `Time`
    isotime + 20 Skalare (u. a. `scaltitudeAVG` km, `tplatitudeAVG`/`tplongitudeAVG` deg) +
    26 `size:[400]`-Arrays (u. a. `ktemp`,`tpgpaltitude`) auf dem geteilten `<i>` + 5
    Nach-Skalare; `fill -999`. Der Arm (`extract.rs`) iteriert jetzt je Zeile statt
    `data.last()`; ein reservierter Ausgabename `@lat`/`@lon`/`@alt` traegt die
    Tangentialposition (kein Enum-Umbau), jede Zeile bekommt `epoch` aus `Time`
    (`series_epoch_of`) und `Position::Surface{earth,tplat,tplon,tpgpalt·1000 m}`. Tests auf
    2 Zeilen × 3 Level = 12 Kanaele + Zeilen-Epoch gezogen; `cargo check` 0/0, rustfmt clean.
    Offen: der Register-Block + die CI-Verifikation (Unit-Tests laufen in CI).
  - **TIMED-Weltlinie — GEBAUT (`ca35522f2`).** `src/archivar/tle.rs` (hand-geschriebenes
    SGP4 WGS-72, TEME→ICRS; die Erdrotation kommt aus der sanktionierten ERA-Funktion
    `earth_rotation_angle_rad`, kein Literal — Gate `remove-bias WP13`) +
    `tools/harvest/src/bin/timed_tle_compiler.rs` (TLE → `WOB1` via `wind_orbit::write_bin`).
    Selbsttest: Epoche `altitude 583.9 km, speed 7.568 km/s`, 1441 Records/24 h, Roundtrip
    parst; SGP4 gegen die Vallado-`00005`-Vektoren im Unit-Test. Offen: Register-Block
    `format orbit_bin`, `at timed` (Mycelium) + CI `timed-orbit-cdn.yml`; Schritt 2
    (`Position::StateVector{track:true}`-Produzent) und das Riss-Gate (`measure`-Sonde,
    `VerdictWord::Riss`, `weberin_verdicts.rs:10-15`).
- **Blockade:** keine.
- **Braucht:** (1) den TIMED-Register-Block + CI; (2) das Riss-Gate; (3) die
  Venus-Roh-Block-Registrierung. `opentopography_coverage`-`terms` ist gesetzt.

## An mycelium

Origin: mountain-347 (2026-10-14):

- **Register-Rot `cdn_reconcile --fail` — geklaert.** Die fünf stale Workflows sind entfernt;
  das Gate ist clean (324 Hosts). Die vier Quellen (`babamul.caltech.edu`, `maxi.riken.jp`,
  `vo.lmd.jussieu.fr`, `lasp.colorado.edu`) waren 2026-10-13 fuer fehlende offene Lizenz aus
  `phi` entfernt (`1b519375b`, `9c60ddf89`), `celestrak.org` per EOP-Verdikt (`b138796d4`).
  Kein Baseline-Eintrag mehr noetig — die Entfernung war die richtige Hand.
- **`opentopography_coverage` `terms PD` gesetzt** (`phi:11445`), Beleg
  `https://opentopography.org/usageterms` (HTTP 200). Die vier `descoped`-Punkte und der
  Venus-Roh-Block sind im Punkt „Taucherstaffel 347" gefuehrt.
- `xfer:mycelium-344-1` (Schumann `harvest raw_zenodo`) — **erfüllt, Echo.**
- `xfer:mycelium-346-6` (Venus-Parser) — **erfüllt, Echo.**
- `xfer:mycelium-346-4`/`-7` (DE-2 CC0, AWESOME) — **terms gesetzt (mountain-346), Echo.**
- `xfer:mycelium-344-2` **Fünf Fäden** — der Mountain-Teil liegt als Entwurf
  `state/mountain/faden-term-field-ttl-draft.φ`. Bitte `url`/`origin`/`compiler` je Faden
  daraufsetzen; danach `register_sort`.
- `xfer:mycelium-346-3` **MODVOLC** — Mountain liefert den Verdikt-Teil: `terms local-only`
  (genuine Absenz, keine Lizenzzeile). Bitte `url`/`origin`/`compiler` setzen.
- `xfer:mycelium-346-5` **OCNS2019-Ersatz** — JPL SSD/Horizons/NAIF sind **offen** (NASA SMD,
  SPD-41a), Danksagung erbeten; kein CC0-Zeichen. Als offene Alternative registrieren.
- **Gewebe-Fäden — `origin`/`url`-Korrekturen + GEOFON-Wellenform (Myceliums Zeilenklasse;
  Mountain schreibt nur `terms`/`field`/`ttl`):**
  - IGETS `origin` (`phi:10319`) zeigt auf die Projektseite `igets.gfz-potsdam.de/` (200);
    die Rohdaten kommen über **`igetsftp.gfz.de`** (sftp, registriert); ISDC `/igets-data-base`
    (200), altes `/igets/` = **404**. Direktivenklasse.
  - Der tote BGI-Host `www.bgi.omp.obs-mip.fr` → **`bgi.obs-mip.fr`** (200): **gemessen —
    kein `bgi`/`obs-mip`-Block in `phi`**; die Taucher-Notiz zielt auf keinen Registereintrag.
    Kein Schreibakt.
  - **GEOFON-Wellenform** ist offen (kein Key), aber ohne `phi`-Block:
    `origin https://geofon.gfz.de/fdsnws/dataselect/1/query` (200, miniSEED/StationXML).
    Mountain liefert den Verdikt-Teil — **`terms CC-BY-4.0`** (Beleg `https://geofon.gfz.de/eqinfo/faq/`),
    `at earth`, Kernel **`point`/`inverse-square`** (Seismik). Bitte `url` (CDN-Asset) +
    `compiler` setzen, dann `register_sort`.

Origin: mycelium-349 (2026-10-14):

- `xfer:mycelium-349-1` **`field` für `catalog_mpcorb`** — gemessen: der Block
  (`phi:3837-3843`) trägt `url`/`terms`/`format`/`origin`/`compiler`/`at`/`ttl`, **kein**
  `field`; die Geschwister-Kataloge `catalog_dastcom` (`phi:3845`) und `catalog_dcom5`
  (`phi:3853`) tragen ebenfalls keins. Ein `field` braucht einen gemessenen Kanalschlüssel
  am Arm; ohne ihn **descoped** (Katalog als Zeuge, nicht als Feld).
- `xfer:mycelium-349-2` **`terms`+`field` für `opentopography_coverage`** — gemessen: der
  Block (`phi:11436-11441`) trägt kein `terms` und kein `field`. Die Terms-Seite ist
  **ungemessen** (die Suche lief in den Timeout) → `terms pending`, kein Verdikt ohne Messung.
- `xfer:mycelium-349-3` **`cosmic_ro`/`heasarc_w3browse` descoped** — gemessen: `cosmic_ro`
  trägt `quantity` (`phi:10190`) + `field` (`phi:10191`) → **descoped**; `heasarc_w3browse`
  ist ein Reader-Arm (`src/archivar/heasarc_w3browse.rs`), kein eigener Register-Block →
  **descoped**.
- `xfer:mycelium-349-4` **Venus-PDS-Roh-Block** — gemessen: **kein** `venus`/`backscatter`-Block
  in `phi` (`archive_search` = 0). Mycelium legt `url`/`origin`/`compiler` an; danach Mountain
  den `terms`-Verdikt-Teil (nach Messung, kein PD ohne Beleg).
- `xfer:mycelium-349-5` **GEOFON-Wellenform** — s. oben (Gewebe-Fäden): `terms CC-BY-4.0`,
  Kernel `point`/`inverse-square`.
- `xfer:mycelium-349-6` **IGETS `origin`** — Mycelium-Entscheidung (Projektseite vs. Datenpfad).

## An river

Origin: mountain-347 (2026-10-14):

- `xfer:mountain-342-6` **SABER:** die Registerzeile folgt nach der Routen-Entscheidung
  (Rats-Verdikt B + A, s. „Offen"); der Host ist intermittierend erreichbar, das Sample
  bleibt bis zur Route ungemessen.
- **UCAR-FPI:** die zwei `field`-Zeilen stehen code-gemessen; offen bleibt allein die
  FPI-eigene Athabasca-Instrumentenkoordinate.

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes."** | 2026-10-14 | (mountain 344, 345, 346, 347).
- **„Nächster offener Schritt: die Athabasca-Spektralform in den Arm ziehen (per-Bin, Frequenz in Hz) und die Registerzeile schreiben; dann SABER-Arm und Venus-Compiler."** | 2026-10-14 | (mountain 344).
- **„recherche roster rat"** (Athabasca-Einheit) | 2026-10-14 | (mountain 344) — Kette gefahren; Verdikt (Rat + Roster): **nT²/Hz primär (Kalibrierung im Arm, linear), V²/Hz als Rohkanal**; umgesetzt.
- **Die Einheiten-Entscheidung übernommen** (nT²/Hz primär + V²/Hz roh, Kalibrierung versioniert im Arm) | 2026-10-14 | (mountain 344 — Operator legte das Verdikt der Stimmen-Kette vor).
- **„was wenn du den satellit autonom betrachtest — dann ist es doch auch nur eine sonde an einem xyz,t; die ISS ist doch zu einer Zeit immer an einem bestimmten Ort"** | 2026-10-14 | (mountain 345 — SABER: eigene Weltlinie, supersediert B2).
- **„warum können wir das gravitationsmodell nicht wie die anderen behandeln? das Erdmagnetfeld-Modell und das Atmosphären-Modell werden an einem Ort ausgewertet und geben dort einen echten Feldwert — sie sind ein Feld"** | 2026-10-14 | (mountain 345 — ICGEM: Feld, supersediert Witness).
- **„fängst du bitte an und delegierst an taucher?"** (die offenen Punkte aufnehmen, Taucher dispatchen) | 2026-10-14 | (mountain 346) — der Taucher-Dispatch scheiterte diesmal an einem Guthaben (`Insufficient Balance`), die Messung wurde selbst gefahren.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. Dieses Atom
(mountain-347) committet: der `archiv/`-Move der folge346 + die Baseline-Zeilen (`802d1834f`,
`88e64f1a7`), die Taucher-Befunde in dieser Übergabe (`01ce52fa6`), die zwei Bauten
(`ca35522f2`: `src/archivar/extract.rs`, `src/archivar/tle.rs`, `src/archivar/tests.rs`,
`src/archivar/mod.rs`, `tools/harvest/src/bin/timed_tle_compiler.rs`),
der `opentopography_coverage`-`terms` in `phi/sources.φ` und diese Fortschreibung.
