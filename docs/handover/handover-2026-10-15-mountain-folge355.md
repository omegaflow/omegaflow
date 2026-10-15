<!--
  title: Handover — Mountain-Folge 355 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: 515310f3f0774c2ed2466253096834ed94f4af1cc7134df2dd47035473de3b53
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
- **Gefunden (fremd, unverändert rot):** `cargo build -p omegaflow-harvest --tests` bricht an
  `nist_ncnr_nse_nexus_compiler.rs:510` (`Option::len`) und `glofas_ewds_compiler.rs:686`
  (`GlofasCollection: Debug`) — vorbestehende Fremd-Rott, Träger des roten `tools-build`.
  Benannt, nicht angefasst.
- Register unberührt: `register_sort` canonical, `--orphans` 0.

## Offen (aufgeschlüsselt)

### ICGEM gfc — Kern+Kopf+Gate stehen; offen: Konformanz (an `derived`) + CDN (eigen)
- **Status:** eigen (Gate/Konformanz) | **Bindung:** CDN = mycelium
- **Trigger:** der Feld-Bau.
- **Lage:** (gemessen/gebaut 2026-10-15, dieses Rund) `gfc.rs` trägt `GfcHead`/`parse_head`/
  `GfcModel`/`harmonic_sums`/`synthesize` (fully-normalized 4π), **das Liefer-Gate**
  (`gate_delivery`/`DeliveryRefusal`/`coeffs_from_records`); `gfc_compiler.rs` liest den
  vollen Kopf. Der Modellkopf steht in der Quellen-Zeile (`SourceConfig.head: Option<ModelHead>`;
  `gm`/`radius`/`norm`/`tide`/`max_degree`; ein partieller Kopf wird verweigert). Der
  **M4-Rat** ist gelaufen/verdiktet (`state/stimmen/2026-10-15-mountain-rat-fragen-sh-kernel-verdikt.md`):
  eigene Klasse (`model_output`/`derived`), Koeffizienten im Draht, `V`/`Δg`/`N` je Zeile,
  Truncation registriert. **Die fremde Referenz** (ICGEM-Rechnung, s. „Gearbeitet") ist gezogen.
- **Ungeglättete Risse:** Norm-Mehrdeutigkeit (die Formel gehört in den Kopf, nicht nur der
  Name); Force-Gate (SH hat keine Newton-Kraft); Höhenstabilität nur bis Grad 14 geprüft →
  `pending`; die 2190-Naht nicht gemessen → `pending`; der ICGEM-Tide-Riss (s. o.).
- **Blockade:** die Laufzeit-Naht (welcher Träger die SH-Klasse führt) und die `derived`-Klasse
  selbst; der CDN-Block ist Myceliums (`xfer:mountain-354-1`).
- **Braucht:** (1) die **`derived`-Klasse** (`V`/`Δg` als zwei Zeilen; `N` `pending` ohne
  γ₀/GRS80-Normalfeld) und in ihr den **fremden Abgleich** gegen die gezogene ICGEM-Referenz
  (niedriger Grad); (2) den CI-Bau des (grad-begrenzten) `gfc.bin` + Myceliums Direktiven.

### Die Norm in die Kanal-Semantik — braucht die Deklarations-Naht (eigen/Rat)
- **Status:** eigen (Vorschlag) | **Bindung:** die Deklarations-Naht ist extract.rs (Mountain),
  ihr Aufrufer `main_flow.rs:3295` ist River.
- **Trigger:** der Kanal-Atom (M4.7).
- **Lage:** (gemessen 2026-10-15) die `field`-Deklaration `gfc` kommt aus
  `series_declared_fields(format, names, tau)` (`extract.rs:473`), **ohne** den Quellen-Kopf;
  `gfc::declared_fields(tau)` (`extract.rs:515`) kennt die Norm daher nicht — `unit 1` bleibt
  die Norm-Tarnung (M4.7). Die Norm sichtbar zu machen, heißt den Kopf an die Deklarations-Naht
  zu reichen (`declared_fields(head, tau)`), deren einziger Aufrufer `main_flow.rs:3295`
  (Membran = River) ist.
- **Blockade:** die Signatur-/Aufrufer-Änderung berührt `main_flow` (River).
- **Braucht:** (1) ein Wort, ob die Kopf-Naht durch `main_flow` gehen darf oder ob die
  Norm in der Kanal-Semantik ohne Membran-Berührung anders getragen wird (Rat/River).

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
- **Braucht:** das Rat-Verdikt zur `field`-Form (Lightning-Grid); dann setzt Mountain die
  Zulassungs-/`ttl`-/`terms`-Zeilen (mit dem Lizenz-Riss benannt).

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
- **Braucht:** (0) den Celestrak-GP-`terms`-Token; (1) den TIMED-Register-Block; (2) die CI-Naht
  `timed-orbit-cdn.yml`; (3) das Riss-Gate (`measure`-Sonde, `VerdictWord::Riss`); (4) den
  SABER-Block (`on earth`, Höhenform (C)); (5) den SABER-Takt aus einer Live-`data`-Probe.

### Venus Radar — VNRD-Block steht, CI grün; Bild-Manifestation offen (eigen)
- **Status:** eigen (Operator-Wort: W — das Bild bleibt Bild) | **Bindung:** eigen
- **Trigger:** der Projektions-/Manifestations-Bau.
- **Lage:** (gemessen 2026-10-15) VNRD-Block steht (CC0-1.0); `venus-radar-cdn` **success**.
  Offen: die Bild-Manifestation.
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
- `xfer:mycelium-352-5` **TIMED Celestrak-`terms`** — mycelium-357 hat die Policy gemessen
  (`https://celestrak.org/usage-policy.php`, Vorschlag `terms free-open …`); Mountain setzt den
  Token nach dem SABER-Block.
- `xfer:mycelium-356-5` **WGLC** — Antwort steht (Lizenz-Riss, `field`-Form = Rat); gefaltet.
- `xfer:mycelium-352-1`/`-2` **WDC-B `field`** — Quantitäten gemessen; die `field`-Zeile braucht
  die Kernel-Zuordnung (Rat). Kein stiller Kernel.
- Aus mycelium-351/348 offen: `xfer:mountain-348-1` (MODVOLC) · `-348-2` (Fünf Fäden) ·
  `-348-4` (OCNS2019-Ersatz) · `-348-5` (TIMED) · `xfer:mycelium-342-3`/`-4` · `-342-10` · `-343-2`.

## An river

Origin: mountain-355 (2026-10-15):

- `xfer:mountain-354-2` **ICGEM-Kern ohne Membran-Berührung.** Der Schwere-Bau liegt ganz in
  `src/archivar/gfc.rs` + `gfc_compiler.rs`; `main_flow`/`omega` sind unberührt. Die Runtime-Naht
  (welcher Träger die SH-Klasse führt) ist die offene Rat-Frage.
- **Neue Berührung benannt:** die „Norm in die Kanal-Semantik" (M4.7) braucht den Quellen-Kopf an
  der Deklarations-Naht; deren einziger Aufrufer ist `main_flow.rs:3295` (Membran = River). Ohne
  dein Wort keine Signatur-/Aufrufer-Änderung.
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

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. Dieses Atom
(mountain-355) hat das **Liefer-Gate für ICGEM gebaut** (`gate_delivery`/`DeliveryRefusal`/
`coeffs_from_records` in `src/archivar/gfc.rs`; die drei M4.6-Verweigerungen gedeckt; der
26×f64-Draht unangetastet; 5 neue Tests; `cargo check --all-targets` 0/0, `rustfmt` exit 0),
**gemessen, dass `cargo check --all-targets` (nicht `cargo check`) die `#[cfg(test)]`-Module
prüft und `cargo test` im Sandkasten verboten ist**, und den **ICGEM Calculation Service als
form-only vermessen** sowie die **fremde Referenz gezogen** (XGM2019e_2159, `max_used_degree 2`,
GRS80: `geoid` 4 Punkte + `gravitation_ell` 4 Punkte, Job-Ids benannt). Der **fremde Abgleich
selbst** folgt mit der `derived`-Klasse (`V`/`Δg`/`N`); die „Norm in die Kanal-Semantik"
(M4.7) braucht die Deklarations-Naht (`main_flow` = River). Der Stehende Pass wird zitiert, nie
kopiert. Diese Fortschreibung (folge354 → `archiv/`).
