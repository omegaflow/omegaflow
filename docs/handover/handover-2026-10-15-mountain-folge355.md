<!--
  title: Handover — Mountain-Folge 355 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: 43fbd48b132d56119a192cac788cf9e0bfa661269f56eb9ca68d2038b9ab6624
  status: live
  see-also: docs/concepts/tool-forms.md, AGENTS.md, docs/surveys/survey-2026-10-15-ketten-quellen-harter-lauf.md, docs/surveys/survey-2026-10-15-riss-alternativen.md
-->
# Handover — Mountain-Folge 355 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Diese
Session konsumierte `handover-2026-10-15-mountain-folge354.md` (→ `archiv/`).

Still gemessen: `register_lookup --fired mountain` = 0, `--stale mountain --persist 3` = 0;
`open_points_check` auf folge354: 26 Pfad-Refs, 0 absent, 0 stale-citations, 0 done-carried,
0 word-carried, 0 guardians, 0 format-gaps, 0 owner-drift, 0 post-md · `register_lookup
--orphans --fail` 0. Die zwei adressierten Blöcke (`mycelium-folge357`, `river-folge218`) waren
in folge354 schon gefaltet (Echo unten). Der Stehende Pass wird als **öffentliche** Fassung
zitiert, nie kopiert.

## Burn: open 0.0 · close nicht gemessen — `session_burn` meldet `db absent`
(`~/.local/share/opencode/opencode.db` fehlt); der Platzhalter `0.0` ist **kein** Meßwert
(Vorbehalt wie mountain-348/349/351/352/353/354).

## Gearbeitet (dieses Rund)

- **Das Liefer-Gate ist gebaut (ICGEM, M4.6).** In `src/archivar/gfc.rs`: die
  Koeffizienten-Extraktion aus dem Draht ist als `coeffs_from_records` herausgezogen
  (von `from_wire` mitgenutzt), und `gate_delivery(head: Option<&ModelHead>, bytes) ->
  Result<Vec<GfcCoeff>, DeliveryRefusal>` verweigert (a) einen Frame **ohne** gemessenen
  Kopf (`HeadAbsent`), (b) einen Koeffizienten mit `n > max_degree` (`DegreeBeyondHead`),
  dazu (c) `FrameUnread` und (d) `NoCoefficients`. Die dritte Verdikt-Bedingung
  (`norm` außerhalb der zwei dokumentierten) ist **strukturell** gedeckt: `Norm::parse`
  kennt nur `fully_normalized`/`unnormalized`; ein unbekanntes Token lässt den Kopf absent
  → `HeadAbsent`. Der 26×f64-Draht bleibt unangetastet. 5 neue Tests; **`cargo check
  --all-targets` 0/0**, `rustfmt --edition 2024 --check` exit 0.
- **Gemessen: `cargo check` prüft die `#[cfg(test)]`-Module NICHT, `cargo check
  --all-targets` schon** (mit einer absichtlichen Typverletzung belegt: `cargo check` = 0 Fehler,
  `--all-targets` = E0308). `cargo test` ist im Sandkasten verboten (Gate
  `cargo *` nur die vier Formen) → der Test-**Lauf** bleibt der CI (`subset` = `cargo test --lib`);
  die **Kompilation** der Tests ist hier via `--all-targets` belegt.
- **Gemessen: der ICGEM Calculation Service ist form-only** — kein API, kein GET-Compute.
  `GET /calcsrvpoints?…` → 500; die Rechnung ist ein `POST` (multipart) an
  `/calcsrvpoints` (mit Pflicht-`points_file`-Upload) bzw. `/calcsrvgrid`, mit
  Session-Token aus `POST /api_key/<epoch>` und dem Last-Gate `GET /calc_check_load`
  (`{"total":0,"user":0}`). Modelle per 64-Hex-Id aus
  `GET /json_modelmap/name?modeltype=longtime&usefor=calc` (z. B. `XGM2019e_2159` =
  `eeb03971cf6e533e6eeb6b010336463286dcda0846684248d5530acf8e800055`; **XGM2019e (5540)
  wird für die Rechnung nicht angeboten**).
- **Fremde Referenz gezogen (der Anfang des Abgleichs).** Den Form-POST headless gefahren
  (Playwright), zwei Läufe über `XGM2019e_2159`, `max_used_degree 2`, GRS80, `zero_degree_term
  included`, `tide_system zero_tide`, Gitter lat/lon `0..1°` Schritt `1°` (4 Punkte):
  - **`geoid` (N, m)** — Job `483db2cd331a036f601bc40e19934424bac6f428985ff40c3d4192a16d2084a7`,
    Download `…/calc_res_gdf/<id>`; `normal_potential 6.263686085004611E+07 m²/s²`; Werte
    (lon, lat, N): `(0,1) 29.254686244203` · `(1,1) 28.631800881689` · `(0,0) 29.263874393182`
    · `(1,0) 28.640791771666`; min/max 28.6318/29.2639 m, weighted_mean 28.947789 m.
  - **`gravitation_ell` (mGal, o. Zentrifugal)** — Job
    `2521a471c3148a80ed25b923ab0c0a76d727870413b77b905b257718af4de55d`; Werte (lon, lat):
    `(0,1) 981434.182553325896` · `(1,1) 981433.896003874950` · `(0,0) 981433.628791175899`
    · `(1,0) 981433.342151919380`; mean 981433.76 mGal.
  **Riss (nicht geglättet):** angefragt `sel_tidesys=unmodified`, der Ergebnis-Kopf nennt
  `tide_system zero_tide`; benannt, beide Zeilen stehen.
- **Der fremde Abgleich selbst ist nicht geschlossen** — unsere `synthesize` liefert `V`
  (Gravitationspotenzial) und die `(n−1)`-gewichtete Anomalie; das ICGEM-`geoid` braucht das
  GRS80-Normalfeld (γ₀/W₀) und `gravitation_ell` ist eine **Beschleunigung** (mGal), nicht das
  Potenzial. Der Abgleich gehört damit an die **`derived`-Klasse** (`V`/`Δg`/`N` je eigene
  Zeile, M4.4), die der Rat als den Schritt **nach** dem Kopf+Gate führt. Die fremde Referenz
  ist gezogen und benannt; der Vergleich folgt mit der Klasse.
- **Der fremde Abgleich ist gefahren (M4.6 Schritt 3) — gegen NASA JEOD, nicht den
  Form-ICGEM.** Der weite/harte Lauf (`bin/all_wide` + Sourcegraph, Taucher) fand die
  NASA-JEOD-Verifikationsvektoren (`https://raw.githubusercontent.com/nasa/jeod/main/models/
  environment/gravity/verif/unit_tests/grav_geospherical/data/verif_out.txt`, GET-scriptable):
  40 Fälle, deren Referenz-Implementierung **dieselbe** 4π-Normierung ohne Condon-Shortley und
  dieselbe Formel `V = GM/r·Σ(a/r)ⁿΣ[C cos mλ+S sin mλ]P_nm` nutzt wie `src/archivar/gfc.rs`
  (Modell GGM02C, GM 3.986004415E+14, a 6378136.3). **Fall 13** (d/o 10/10, perturbOnly=1,
  Punkt ECF −6404550.640081473, −1493096.048926443, 6158937.856787768) erwartet
  **−4.843731614932750E+03 m²/s²**. Unsere Synthese (GGM02C-Grad-10-Koeffizienten aus
  `earth_GGM02C.cc`, über den echten `parse_head`/`GfcModel`-Pfad) liefert
  **−4.843731614933E+03** — Abweichung ~2,5·10⁻¹⁰ (relativ ~5·10⁻¹⁴). **Der Konventions-Check
  steht** (A = A in der Sache, nicht nur im Haus). Der Fall ist als Test eingetragen
  (`the_synthesis_matches_the_nasa_jeod_ggm02c_degree10_potential`), der Messweg als Bin
  `tools/harvest/src/bin/gfc_probe.rs` (`--input <x.gfc> --point <x,y,z>`).
- **Die Norm tritt in die Kanal-Semantik (M4.7, entschieden — nicht neu vorgelegt).** Der
  `gfc_compiler` schreibt jetzt an jede `field`-Zeile den Geschwister-Qualifier `norm:<token>`
  (`head.norm.name()`); der Parser (`parse.rs`, `field`-Zweig) liest ihn und **verweigert** ein
  Feld-Norm-Token, das dem Quellen-Kopf widerspricht (nie still geeinigt). Der Qualifier ist
  arity-sicher (nicht-numerisch → fällt aus `freq`/`bin_width`; kein `aperture:`, kein `where`).
  Test `a_field_norm_token_must_agree_with_the_source_head`. **Rest-Hälfte benannt:** den
  Laufzeit-Namen `icgem_gfc_c` selbst norm-explizit zu machen (die Join-Naht trägt den Namen),
  hieße `main_flow` berühren (River) — als `xfer:mountain-355-1` gestellt, nicht ohne Wort.
  **Zwei benannte Lücken** (nicht geglättet): `Norm` kennt kein `schmidt`/`ortho` (die
  pyshtools-Vier gegen die ICGEM-Zwei); der `.gfc`-Kopf **defaultet** auf `FullyNormalized`,
  wenn die `norm`-Zeile fehlt (`gfc.rs:262`) — eine Annahme an der Stelle von `pending`.
- **Gefunden (fremd, unverändert rot):** `cargo build -p omegaflow-harvest --tests` bricht an
  `nist_ncnr_nse_nexus_compiler.rs:510` (`Option::len`) und `glofas_ewds_compiler.rs:686`
  (`GlofasCollection: Debug`) — vorbestehende Fremd-Rott, Träger des roten `tools-build`.
  Benannt, nicht angefasst.
- Register unberührt: `register_sort` canonical, `--orphans` 0.

## Offen (aufgeschlüsselt)

### ICGEM gfc — Kern+Kopf+Gate+Konformanz stehen; offen: `derived`-Klasse + CDN (eigen)
- **Status:** eigen (Gate/Konformanz) | **Bindung:** CDN = mycelium
- **Trigger:** der Feld-Bau.
- **Lage:** (gemessen/gebaut 2026-10-15, dieses Rund) `gfc.rs` trägt `GfcHead`/`parse_head`/
  `GfcModel`/`harmonic_sums`/`synthesize` (fully-normalized 4π), **das Liefer-Gate**
  (`gate_delivery`/`DeliveryRefusal`/`coeffs_from_records`); `gfc_compiler.rs` liest den
  vollen Kopf. Der Modellkopf steht in der Quellen-Zeile (`SourceConfig.head: Option<ModelHead>`;
  `gm`/`radius`/`norm`/`tide`/`max_degree`; ein partieller Kopf wird verweigert). Der
  **M4-Rat** ist gelaufen/verdiktet (`state/stimmen/2026-10-15-mountain-rat-fragen-sh-kernel-verdikt.md`):
  eigene Klasse (`model_output`/`derived`), Koeffizienten im Draht, `V`/`Δg`/`N` je Zeile,
  Truncation registriert. **Der Konventions-Abgleich steht** (NASA JEOD, GGM02C Grad 10,
  s. „Gearbeitet"); die ICGEM-Form-Referenz (Job-Ids) liegt zusätzlich vor.
- **Ungeglättete Risse:** Norm-Mehrdeutigkeit (die Formel gehört in den Kopf, nicht nur der
  Name); Force-Gate (SH hat keine Newton-Kraft); Höhenstabilität nur bis Grad 14 geprüft →
  `pending`; die 2190-Naht nicht gemessen → `pending`; der ICGEM-Tide-Riss (s. o.).
- **Blockade:** die Laufzeit-Naht (welcher Träger die SH-Klasse führt) und die `derived`-Klasse
  selbst; der CDN-Block ist Myceliums (`xfer:mountain-354-1`).
- **Braucht:** (1) die **`derived`-Klasse** (`V`/`Δg` als zwei Zeilen; `N` `pending` ohne
  γ₀/GRS80-Normalfeld) — der **Konventions-Abgleich ist gefahren** (JEOD GGM02C Grad 10, s.
  „Gearbeitet"); (2) den CI-Bau des (grad-begrenzten) `gfc.bin` + Myceliums Direktiven.

### Die Norm in die Kanal-Semantik — Deklarations-Seite steht; Laufzeit-Name offen (eigen)
- **Status:** eigen | **Bindung:** die Laufzeit-Naht ist `main_flow` (River).
- **Trigger:** der Kanal-Atom (M4.7).
- **Lage:** (gemessen/gebaut 2026-10-15, dieses Rund) Die **Deklarations-Naht steht**: der
  `gfc_compiler` schreibt `norm:<token>` an jede `field`-Zeile, der Parser liest und **verweigert**
  ein widersprechendes Feld-Token (s. „Gearbeitet"); die Norm ist damit sichtbar in der
  Kanal-Deklaration, ohne die `unit 1`-Tarnung. **Offen:** den Laufzeit-**Namen**
  `icgem_gfc_c` selbst norm-explizit zu machen — der Name ist die Join-Identität
  (`field_line_matches`, `series_field_name`), also müssten `gfc::declared_fields`/`component_name`
  und `series_declared_fields`/`series_component_name` (`extract.rs`) sowie die zwei Aufrufer
  `main_flow.rs:3295` und die Join-Helfer `main_flow.rs:6394-6396` die Norm tragen.
- **Blockade:** die Laufzeit-Namens-Variante berührt `main_flow` (Membran = River).
- **Braucht:** ein Wort für die Membran-Naht (`xfer:mountain-355-1`); bis dahin steht die Norm
  in der Deklaration und im Quellen-Kopf, der Laufzeit-Name bleibt `icgem_gfc_c`.

### ICGEM-CDN-Block — Transport (mycelium; `xfer:mountain-354-1`)
- **Status:** mycelium | **Bindung:** mountain → mycelium
- **Trigger:** der Transport-Bau.
- **Lage:** (gemessen 2026-10-15) der Compiler `tools/harvest/src/bin/gfc_compiler.rs` steht,
  Format `gfc`, `terms CC-BY-4.0`, `at earth`, der volle Kopf wird gelesen. Das Haus-CDN-Asset
  `…/icgem.gfz.de/gfc.bin` ist **404**; `url`/`origin`/`compiler`-Direktiven und der CI-Bau
  (93 MB → 5540 Grad, heavy compute = CI) sind Myceliums.
- **Blockade:** Myceliums Transport/CI (nicht Mountain).
- **Braucht:** den CI-Bau + die Transport-Zeilen; Mountain setzt danach `field`/`ttl` nach dem Rat.

### WGLC-Zulassung — Lizenz gemessen, `field`-Form ist Rat (eigen; Antwort an mycelium)
- **Status:** eigen (Verdikt = Mountain) | **Bindung:** mycelium-357 → mountain
- **Trigger:** `xfer:mycelium-356-5`.
- **Lage:** (gemessen 2026-10-15) Zenodo-Record `4774528`/Version `20277101` trägt
  **`license.id = cc-by-4.0`**; Mycelium maß das README-Badge **CC BY-SA 4.0** → **Lizenz-Riss**
  (beide Zeilen offen). Zeitauflösung: Dichte **täglich** (`density [5844,360,720]`, Schritt 1 Tag),
  Leistung monatlich; `30m`/`05m` ist die räumliche Auflösung. `wglc_compiler.rs` steht.
- **Blockade:** die `field`-Form für einen globalen Blitz-Dichte-Grid ist die Rat-Schemafrage
  (derselbe Absenz-/`field`-Pfad wie WDC-B `gdata`).
- **Braucht:** das Rat-Verdikt zur `field`-Form (Lightning-Grid). **Die Lizenz-Riss-Entscheidung
  ist gefallen** (Record `cc-by-4.0` maßgeblich, README-Badge CC BY-SA 4.0 als Riss benannt) und
  der Mountain-Teil steht (`terms CC-BY-4.0 https://zenodo.org/records/4774528`, `ttl 86400`,
  `field pending`) im Entwurf `state/mountain/2026-10-15-grenz-term-verdicts.φ`; Mycelium
  materialisiert `url`/`origin`/`compiler`. **Die Rat-Frage ist gestellt (M5).** Das Paket
  `state/stimmen/2026-10-15-mountain-rat-fragen-grid-form.md` trägt die Frage (Die global gemessene
  Gitter-Form, M5.1–M5.7), den gemessenen Stand (WGLC, CF-Konvention, Engine-/Archiv-Präzedenz aus
  zwei weiten Läufen von `bin/all_wide`) und die Sitz-Liste; der Operator sendet es an die Sitze, dann trägt der
  Rat das Verdikt.

### WDC-B `field` je Datensatz — Kernel-Zuordnung ist Rat (eigen)
- **Status:** eigen (field = Mountain; Kernel = Rat) | **Bindung:** eigen
- **Trigger:** `xfer:mycelium-352-1`/`-2`.
- **Lage:** (gemessen 2026-10-15) Die Quantitäten/Einheiten der Groups sind gemessen; das
  Roster-Verdikt steht (`state/stimmen/2026-10-15-mountain-rat-fragen-verdikt.md`): **drei
  tragen ein Feld** (`geo_min` an `Position::Surface` + getrennt IGRF-SH; `gdata`/`mgg` als
  Schwere-SH; `catal_pc` als Wellen-Ereignis, Kernel `pending`); **vier tragen keins** (G,
  `sdata`, `aa`, `gps`). Die `field`-Form (7 Kernel, `kernel_id_of` `extract.rs:1700`,
  Force-Gate) kann Nicht-Felder nicht tragen → der Absenz-Pfad ist eine Schema-Erweiterung (Rat).
- **Blockade:** Rat (Schema-Erweiterung um Nicht-Feld-Typen).
- **Braucht:** (1) das Rat-Verdikt zur Absenz-Pfad-Form; (2) dann die Feld-/Absenz-Zeilen je Datei.

### Ketten-Quellen (SWIFF · WINDMI · MAGION · MOSWOC-Kp · OVATION-AACGM) (eigen)
- **Status:** eigen · **gemessen 2026-10-15** | **Bindung:** eigen
- **Trigger:** der harte Ketten-Quellen-Lauf — abgearbeitet.
- **Lage:** (gemessen 2026-10-15) Messblatt `docs/surveys/survey-2026-10-15-ketten-quellen-harter-lauf.md`.
  SWIFF/WINDMI/MAGION bleiben „kein Datensatz"; MOSWOC-Kp entfällt (GFZ-Kp deckt).
  **OVATION-AACGM** ist ein Modell-Zeuge (Punkt-Auswertung an Mohe), kein Feld; Operator-Wort
  2026-10-15: das Register trägt die **Messung** (DMSP-SSJ), nicht den Modell-Output. F16/F17/F18
  sind registriert (`dmsp16/17/18_ssj`); der Producer nimmt `--sat f16|f17|f18`.
- **Blockade:** keine — das Operator-Wort liegt, die Messung steht.
- **Braucht:** nichts an der Aufnahme; optional den Zip-Kopf (Span/Takt) für den `model_output`-Fall.

### SABER — Arm + Weltlinie stehen; Takt + Riss-Gate offen (eigen)
- **Status:** eigen · **verdiktet** (B + A) | **Bindung:** eigen
- **Trigger:** der Arm-Bau.
- **Lage:** (gemessen 2026-10-15) Der Zeilen-Arm steht; TIMED-Weltlinie gebaut
  (`timed_tle_compiler.rs`, WOB1); Register-Blöcke noch ABSENT; `timed-orbit-cdn.yml` fehlt.
  **Riss:** „Schritt 2" nennt einen `StateVector{track:true}`-Produzenten; gebaut ist der
  `orbit_bin`-Weg (`Barycenter`).
- **Blockade:** keine (Bau offen).
- **Braucht:** (0) **Celestrak-`terms` entschieden:** `free-open https://celestrak.org/usage-policy.php`
  (Haus-Token `free-open`, `license_census.rs:27`), im Entwurf
  `state/mountain/2026-10-15-grenz-term-verdicts.φ`; (1) den TIMED-Register-Block; (2) die CI-Naht
  `timed-orbit-cdn.yml`; (3) das Riss-Gate (`measure`-Sonde, `VerdictWord::Riss`); (4) den
  SABER-Block (`on earth`, Höhenform (C)); (5) den SABER-Takt aus einer Live-`data`-Probe.

### Venus Radar — VNRD-Block steht, CI grün; Bild-Manifestation offen (eigen)
- **Status:** eigen (Operator-Wort: W — das Bild bleibt Bild) | **Bindung:** eigen
- **Trigger:** der Projektions-/Manifestations-Bau.
- **Lage:** (gemessen 2026-10-15) VNRD-Block steht (CC0-1.0); `venus-radar-cdn` **success**.
  Der Zenodo-Bild-Block `venus_radar_Processed.zip` trägt die `terms` bereits
  (`CC-BY-4.0 https://zenodo.org/records/18176179`, `phi:31325`) — Venera-Polar ist eine Datei
  derselben Deposition, dieselbe Lizenz. Offen: die Bild-Manifestation.
- **Blockade:** keine.
- **Braucht:** die Bild-Manifestation (W).

### UCAR-FPI — Arm + Zeilen stehen; Arbeitskopie-Route offen (eigen)
- **Status:** eigen (Rest Mycelium-/River-gebunden) | **Bindung:** eigen
- **Trigger:** die `field`-Zeilen aus dem Sample.
- **Lage:** (gemessen 2026-10-15) Arm + Guard stehen (`fpi_ucar.rs`); die zwei `field`-Zeilen
  `fpi_mer_wind_ms`/`fpi_zon_wind_ms` sind code-gemessen (`extract.rs:540`); Athabasca-Koordinate
  `on earth 54.60282 -113.64442 693`.
- **Blockade:** nicht-offene Lizenz → `local-only` (kein CDN, kein `phi`-Block).
- **Braucht:** die Arbeitskopie-Route (`local-only`).

### Fünf fehlende Fäden — Entwurf steht, Mycelium setzt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom.
- **Lage:** (gemessen 2026-10-14) Mountain-Entwurf `state/mountain/faden-term-field-ttl-draft.φ`;
  zwei Risse offen (WSO-Polarfeld ohne Terms; LISIRD ohne Terms). `xfer:mountain-348-2`.
- **Blockade:** keine.
- **Braucht:** Mycelium setzt `url`/`origin`/`compiler`; dann `register_sort`.

### Quellen-Gewebe — Kraft × Kernel; die dünnen Fäden (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom.
- **Lage:** (gemessen 2026-10-14) Messblatt `docs/surveys/survey-2026-10-14-gewebe-faeden.md`.
  EGGN/CGrav/GOCE/NIED sind als `pending`/`riss` benannt. Riss: `survey-2026-10-05` zitiert
  `dead_sources.φ`/`declined_sources.φ` — beide existieren nicht im Baum.
- **Blockade:** die Kernel-Zuordnung ist Rat.
- **Braucht:** (0) IGETS-`origin` steht (`sftp://igetsftp.gfz.de`); (1) die Kernel-Lücke
  schließen (Rat).

### Adressierte Register-Fäden — OCNS2019 `local-only`, MODVOLC-Korrektur (eigen)
- **Status:** eigen (Register-Writes) | **Bindung:** eigen
- **Trigger:** der Registeratom.
- **Lage:** (gemessen 2026-10-15) MODVOLC HIGP: keine Lizenzzeile → `local-only` (kein Ersatz
  für den LANCE/NRT-Block). OCNS2019 = Yuan 2021; der JPL-Ersatz ersetzt ihn nicht → Riss.
- **Blockade:** keine (Doku-/Verdikt-Arbeit).
- **Braucht:** (1) OCNS2019 als `local-only` stehen lassen (erledigt); (2) MODVOLC HIGP
  `local-only`, kein Block; LANCE/NRT bleibt; (3) einen echten offenen Beobachtungs-Ersatz
  (Lead NSDB `10.1051/0004-6361/200911645`).

### `em mw` / `advective kg/m2/s` — Freeze (eigen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das `tools-latest`-Republish.
- **Lage:** (gemessen 2026-10-13) `units.rs` admiert beide; `docs/specs/force-unit-baseline.txt`
  trägt sie froren.
- **Blockade:** keine.
- **Braucht:** die drei Paare nach dem Republish aus der Freeze nehmen.

### ALFALFA · Ersatz-Bau · eigene Ephemeride (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je der Bau/Fund.
- **Lage:** (gemessen 2026-10-15) ALFALFA `local-only` bestätigt. OpenTopography-Compiler liest
  Scripps SRTM15+ (Namens-Riss); Höhenbezug EGM96/orthometrisch, Toleranz absent. ILRS NPT:
  Producer steht, **kein Lese-Arm in `extract.rs`**; Route EDC offen. `icrf3_compiler.rs` liest
  drei IERS-Dateien, kein SKD1/Round-Trip.
- **Blockade:** keine.
- **Braucht:** (1) Höhenbezug in die `opentopography_raster`-Zeile oder Compiler umbenennen;
  (2) `load_gestalt_surface_threads` an einen Produktions-Aufrufer binden **oder** `descoped`;
  (3) `ilrs_npt`-Lese-Arm in `extract.rs` + Block; (4) `icrf3` versionieren + Round-Trip + SKD1.

### Riss-Alternativen — fünf Taucher; NSDB-Vertrag geschlossen (eigen)
- **Status:** eigen · **gemessen 2026-10-15** | **Bindung:** eigen
- **Trigger:** Operator-Frage 2026-10-15 („und für all die risse gibt es keine alternativen?") — abgearbeitet.
- **Lage:** (gemessen 2026-10-15) Messblatt `docs/surveys/survey-2026-10-15-riss-alternativen.md`. Der NSDB-Contract-Riss
  ist geschlossen (`nsdb::to_astro_series`, AST1). **BAS-IMAGE/DMSP-SSJ:** Format vermessen,
  Roster-Verdikt steht (BAS = Skalar auf der 24-Bin-MLT-Achse; DMSP = Pass-Punkt-Locus; `NaN → None`;
  Leitfähigkeit/FAC `pending`). **48 > 26:** je Bin ein Record. **16-vs-24 offen** (Zip nicht entpackbar).
- **Blockade:** (1) Rat (Schema: closed-discrete MLT-Achse + Absenz-Pfad); (2) DMSP-Zip entpacken.
- **Braucht:** (1) das Rat-Verdikt zur MLT-Achsen-/Absenz-Form; (2) ein Entpack-Werkzeug für die
  DMSP-CSV; (3) die vier Mycelium-Registrierungen (`xfer:mountain-351-1…-5`); (4) die
  NSDB-Witness-Route in den S²-Pfad (main_flow = River).

### Axiom-Verwechslung + ICRS/TDB/pending-Fabrikations-Audit (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Operator-Frage 2026-10-15.
- **Lage:** (gemessen 2026-10-15) Die fünf Axiome sind der **Granit** (`docs/granit.md:16-20`;
  `docs/concepts/glossar.md:85`): **A = A · ICRS & TDB · channel_ref · 0 geehrt · pending**;
  das Gate lädt `src/gate/axioms.rs::load`. Die ethischen Grundsätze in `AGENTS.md` tragen
  denselben Namen — eine Verwechslung, verbreitet in fremden Dokumenten (benannt, nicht
  geändert). Der Granit trägt **7** Sätze. **Fabrikations-Audit:** der Produktions-Code ist frei
  von Körper-Namens-Literalien (Treffer nur in `#[cfg(test)]`); die J2000-Anker-Konstante steht
  einmal in `types.rs` (`J2000_UNIX_OFFSET`); **Lücke (offen):** `VerdictWord`
  (`weberin_verdicts.rs:9-15`) kennt kein `Pending` — das Verdikt-Register kann „noch nicht
  gemessen" nicht tragen (`from_code` → `None`).
- **Blockade:** keine.
- **Braucht:** (1) die fremden Fragebögen/Dokumente auf den Granit ziehen (deren Linien);
  (2) prüfen, ob ein `VerdictWord::Pending` das Register-Vokabular erweitern muss (Rat).

## An mycelium

Origin: mountain-355 (2026-10-15):

- `xfer:mountain-354-1` **ICGEM-CDN-Block — Transport.** Der Compiler steht, der Modellkopf ist
  gemessen, Synthese + **Liefer-Gate** sind gebaut. **Offen ist der Transport:** das Haus-CDN-Asset
  `…/icgem.gfz.de/gfc.bin` ist **404**; die `url`/`origin`/`compiler`-Direktiven und der CI-Bau
  (93 MB → 5540 Grad, ggf. grad-begrenzt; heavy compute = CI) sind Myceliums. Mountain setzt nach
  dem Bau die `field`/`ttl`-Zeilen — nach dem Rat zur SH-Kernel-Form (M4).
- `xfer:mycelium-352-5` **TIMED Celestrak-`terms`** — entschieden:
  `terms free-open https://celestrak.org/usage-policy.php` (Haus-Token `free-open`); steht im
  Entwurf `state/mountain/2026-10-15-grenz-term-verdicts.φ`. Der Block + die CI-Naht + `ttl` sind
  deine Seite (`timed_tle_compiler.rs`).
- `xfer:mycelium-356-5` **WGLC** — die Lizenz-Riss-Entscheidung ist gefallen (Record `cc-by-4.0`
  maßgeblich; README-Badge CC BY-SA 4.0 als Riss benannt). `terms CC-BY-4.0` + `ttl 86400` stehen
  im Entwurf `state/mountain/2026-10-15-grenz-term-verdicts.φ`; **die `field`-Form bleibt die
  Rat-Schemafrage** (kein Punkt-Kernel). Deine `url`/`origin`/`compiler` + der Grammatik-Parser
  folgen; ohne den Rat keine `field`-Zeile.
- **Antwort auf die Grenz-Anfrage (2026-10-15):** River legt **nicht** selbst ab (gemeinsamer
  Speicher = Mycelium) — bestätigt. Venera-Polar: die `terms` stehen schon (`phi:31325`), ihr
  könnt spiegeln. Kamera-Stücke (THEMIS ASI): die Quelle ist bereits zugelassen (`themis_asi`,
  `free-open`, `phi:11844`) — kein Mountain-Tor; die Artefakte sind Rivers.
- `xfer:mycelium-352-1`/`-2` **WDC-B `field`** — Quantitäten gemessen; die `field`-Zeile braucht
  die Kernel-Zuordnung (Rat). Kein stiller Kernel.
- Aus mycelium-351/348 offen: `xfer:mountain-348-1` (MODVOLC) · `-348-2` (Fünf Fäden) ·
  `-348-4` (OCNS2019-Ersatz) · `-348-5` (TIMED) · `xfer:mycelium-342-3`/`-4` · `-342-10` · `-343-2`.

## An river

Origin: mountain-355 (2026-10-15):

- `xfer:mountain-354-2` **ICGEM-Kern ohne Membran-Berührung.** Der Schwere-Bau liegt ganz in
  `src/archivar/gfc.rs` + `gfc_compiler.rs`; `main_flow`/`omega` sind unberührt. Die Runtime-Naht
  (welcher Träger die SH-Klasse führt) ist die offene Rat-Frage.
- `xfer:mountain-355-1` **Norm in den Laufzeit-Kanalnamen (M4.7).** Die Deklarations-Seite ist
  gebaut (der `gfc_compiler` schreibt `norm:<token>` an jede `field`-Zeile, der Parser verweigert
  einen Widerspruch zum Quellen-Kopf). Den Laufzeit-Namen `icgem_gfc_c` selbst norm-explizit zu
  machen, hieße `series_declared_fields`/`series_component_name` (`extract.rs`) und die zwei
  Aufrufer `main_flow.rs:3295` + die Join-Helfer `main_flow.rs:6394-6396` die Norm tragen lassen —
  `main_flow` = Membran. Ohne dein Wort keine Signatur-/Aufrufer-Änderung.
- `xfer:mountain-352-1` **Die vier Archivar-Clippy-Lints sind geheilt** (river-folge216 gefaltet).
- `xfer:mountain-352-2` **NSDB-Richtungszeuge in den S²-Pfad.** `nsdb::to_astro_series` liefert
  die AST1-Richtungszeugen; der `format nsdb`-Zweig in `main_flow` wartet.

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes."** | 2026-10-14/15 | (mountain 344–348, 352, 353, 354, 355).
- **„Die letzte session schreibt Committet und gepusht — alles von dieser Sitzung ist in `origin/main`. Die letzte Hand war der Abschluss-Commit der Übergabe (`b10dd20d9`)."** | 2026-10-15 | (mountain 355) — die Vor-Session ist committet/gepusht; kein erneutes Vorlegen.
- **„die letzte session schreibt Verstanden — die Kritik sitzt, keine Ausreden."** | 2026-10-15 | (mountain 354).
- **„Der nächste Block ist der Schwere-Bau für ICGEM … zuerst den Modellkopf selbst … Danach steht der Kern."** | 2026-10-15 | (mountain 354) — Kopf gemessen, Kern gebaut.
- **„… dafür braucht es dann vermutlich nochmal recherche wide all roster (bereite das dokument vor) und den rat."** | 2026-10-15 | (mountain 354) — M4-Rat gelaufen/verdiktet.
- **„hab kimi nachgetragen."** | 2026-10-15 | (mountain 354) — 10 M4-Stimmen, Kopfweg 5 : 5.
- **„ich würde sagen in den kasten weil ein eintrag in die zahlen eine veränderung der rohdaten ist."** | 2026-10-15 | (mountain 354) — Kopfweg (b), gebaut.
- **„Bereit für die frische Session … (1) ICGEM: das Liefer-Tor (Frame ohne Kopf → verweigern; Grad zu hoch → verweigern), dann der fremde Abgleich gegen den ICGEM-Rechenservice. Der CDN-Block liegt bei Mycelium."** | 2026-10-15 | (mountain 355) — das Liefer-Tor ist gebaut; der Rechenservice ist form-only gemessen, die fremde Referenz gezogen; der Abgleich folgt mit der `derived`-Klasse.
- **„kein ‚nächster Dispatch', kein ‚nächste Session', kein Vorlegen eines bereits geworteten Punktes."** | 2026-10-15 | (mountain 355) — Arbeitsdoktrin dieses Runds.
- **„Die nächste Session liest `…-mountain-folge354.md` als Erstes und dispatcht dann die Taucher auf genau diese Punkte."** | 2026-10-15 | (mountain 355) — folge354 konsumiert → `archiv/`.
- **„ist das alles mycelium oder auch deins? du solltest taucher losschicken nicht verschleppen."** | 2026-10-15 | (mountain 355) — Taucher mit harten Bandagen losgeschickt.
- **„ich dachte dass sei auch schon festgelegt Norm in die Kanal-Semantik."** | 2026-10-15 | (mountain 355) — die Deklarations-Seite ist gebaut (Compiler + Parse-Gate); kein neues Wort für die Deklaration.
- **„ist das der einzige pfad oder hast du nochmal archive search allwide mit harten bandagen versucht?"** | 2026-10-15 | (mountain 355) — `bin/all_wide` und Sourcegraph gefahren; die NASA-JEOD-Referenz gefunden; der Abgleich ist gefahren.
- **„kannst du dich bitte um deinen teil kümmern? Klare Antwort, alles drei:"** | 2026-10-15 | (mountain 355) — Mountains Teil der Grenz-Anfrage: WGLC-Lizenz-Riss entschieden + Terms/ttl im Entwurf; TIMED-Celestrak-`terms` entschieden; Venera-Polar-`terms` bestätigt (`phi:31325`).
- **„ja bitte die ratsfrage"** | 2026-10-15 | (mountain 355) — die M5-Rat-Frage (global gemessene Gitter-Form) ist gestellt (`state/stimmen/2026-10-15-mountain-rat-fragen-grid-form.md`).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. Dieses Atom
(mountain-355) hat das **Liefer-Gate für ICGEM gebaut** (`gate_delivery`/`DeliveryRefusal`/
`coeffs_from_records` in `src/archivar/gfc.rs`; die drei M4.6-Verweigerungen gedeckt; der
26×f64-Draht unangetastet; 5 neue Tests; `cargo check --all-targets` 0/0, `rustfmt` exit 0),
**gemessen, dass `cargo check --all-targets` (nicht `cargo check`) die `#[cfg(test)]`-Module
prüft und `cargo test` im Sandkasten verboten ist**, den **ICGEM Calculation Service als
form-only vermessen** und die XGM2019e_2159-Form-Referenz gezogen (Job-Ids benannt), **die
NASA-JEOD-GGM02C-Grad-10-Referenz gefunden und den Konventions-Abgleich gefahren** (unsere
`−4.843731614933E+03` gegen `−4.843731614932750E+03`, relativ ~5·10⁻¹⁴; Test + Bin `gfc_probe`),
und die **Norm in die Kanal-Deklaration gesetzt** (`norm:<token>` je `field`-Zeile + Parse-Gate).
Offen: die `derived`-Klasse (`V`/`Δg`; `N` `pending` ohne γ₀), der Laufzeit-Kanalname über die
Membran (`xfer:mountain-355-1`, River), der CDN-Block (Mycelium). Der Stehende Pass wird
zitiert, nie kopiert. Diese Fortschreibung (folge354 → `archiv/`).
