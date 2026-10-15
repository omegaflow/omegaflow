<!--
  title: Handover — Mountain-Folge 356 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: ae00f6f33a00b7f4486dfdbe5458dbca9ed7205a7ef88f7c39c30ea747dc5e47
  status: live
  see-also: docs/concepts/tool-forms.md, AGENTS.md, state/stimmen/2026-10-15-mountain-rat-fragen-sh-kernel-verdikt.md
-->
# Handover — Mountain-Folge 356 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Diese
Session konsumierte `handover-2026-10-15-mountain-folge355.md` (→ `archiv/`).

Still gemessen: `register_lookup --fired mountain` = 0, `--stale mountain --persist 3` = 0
(bei `--persist 1` sind 16 Punkte bernstein, kein gefeuerter Trigger); `open_points_check`
auf folge355: 29 Pfad-Refs, 0 absent, 0 stale-citations, 0 done-carried, 0 word-carried,
0 guardians, 0 format-gaps, 0 owner-drift, 0 post-md · `register_lookup --orphans --fail` 0.
Die adressierten Blöcke sind gefaltet: `mycelium-folge357`/`river-folge219` waren in folge355
beantwortet; **`mycelium-folge359`** (acht Fäden) und **`river-folge221`** sind dieses Rund
beantwortet (An-mycelium-/An-river-Block unten). Der Stehende Pass wird als **öffentliche**
Fassung zitiert, nie kopiert.

## Burn: open 0.0 · close nicht gemessen — `session_burn` meldet `db absent`
(`~/.local/share/opencode/opencode.db` fehlt); der Platzhalter `0.0` ist **kein** Meßwert
(Vorbehalt wie mountain-348/349/351/352/353/354/355).

## Gearbeitet (dieses Rund)

- **Die `derived`-Klasse steht (ICGEM, M4.4) — die verschmolzene `synthesize` ist getrennt.**
  In `src/archivar/gfc.rs`: `enum Functional { Potential, GravityAnomaly, GeoidHeight }` mit
  `name()`/`unit()`/`is_measurable()` und `const FUNCTIONALS`; `GfcModel::evaluate(radius_m,
  colat_deg, lon_deg, functional) -> Option<f64>` in der gemessenen Rat-Signatur `evaluate(r,
  colat, lon, funktional)` (M4.1). `V` = `gm/r·σ` (m^2/s^2), `Δg` = `gm/r²·weighted` (m/s^2,
  die (n−1)-gewichtete Radialform), **`N` liefert `None`** — pending, weil das GRS80-Normalfeld
  (a, f, GM, ω) und γ₀ fehlen; **nie `0.0`**. Die alte `GfcValue`-Struktur und `synthesize`
  (die V und Δg aus **einer** Funktion lieferte) sind entfernt — die Funktionale sind eigene
  Größen (M4.4), nie verschmolzen. Der 26×f64-Draht und das Liefer-Gate bleiben unangetastet.
- **Der Messweg trägt die Funktionale.** `tools/harvest/src/bin/gfc_probe.rs` läuft über
  `FUNCTIONALS` und druckt je Funktional Name · Wert · SI-Einheit, `N` als `pending`.
  Messlauf (XGM2019e-Grad-2-Sonde, Punkt ECF des JEOD-Falls): `potential 4.423499125781E7 m^2/s^2`,
  `gravity_anomaly -5.311718083618E-4 m/s^2`, `geoid_height pending m`.
- **Der Konventions-Abgleich läuft durch die neue Klasse.** Der NASA-JEOD-GGM02C-Grad-10-Test
  (`the_synthesis_matches_the_nasa_jeod_ggm02c_degree10_potential`) ruft jetzt `evaluate(...,
  Potential)`; der neue Test `the_functionals_carry_their_own_units_and_the_geoid_stays_pending`
  belegt die Einheiten, `is_measurable`, und dass `N` `None` bleibt.
- **`cargo check` 0/0**, **`cargo check --all-targets` 0/0** (die `#[cfg(test)]`-Module),
  `rustfmt --edition 2024 --check` exit 0 (beide eigenen Dateien),
  `cargo build -p omegaflow-harvest --bin gfc_probe` ok. `cargo test` bleibt im Sandkasten
  verboten → der Test-**Lauf** ist der CI (`subset` = `cargo test --lib`).
- **Getragener Riss (nicht geglättet):** `GravityAnomaly` ist die (n−1)-gewichtete Radialform;
  die Normalfeld-Reduktion liefert der Aufrufer (`GFC_SYNTHESIS_CONVENTION` sagt es). Ob die
  Klasse ihre eigene `phi`-Deklarationsform braucht (welche Zeile/Token die derived-Klasse trägt),
  ist die offene Schemafrage — der Code-Kern steht, die Register-Deklaration nicht.
- **Local-only am Kanon gemessen (zwei Anfragen korrigiert).** Der Kanon
  (`docs/concepts/source-curation.md:90-102`, Operator-Wort 2026-10-13) sagt: ein Datensatz ohne
  offene Lizenz wird lokal genutzt, aber **nie** manifestiert — keine `phi`-Zeile, kein CDN, keine
  öffentliche Fläche; `local-only` ist eine Disposition, **kein** `terms`-Token
  (`license_census.rs:9-33` kennt es nicht). Damit: **MODVOLC/HIGP-Archiv** = `local-only`, kein
  Block (Myceliums „`terms local-only` → Block" widerspricht dem Kanon); **UCAR-FPI** = `local-only`,
  die zwei `field`-Zeilen sind Code-gemessen, nicht in `phi`. Beide Verdikte stehen im Entwurf
  `state/mountain/2026-10-15-grenz-term-verdicts.φ` (§4); die Risse sind benannt.
- **Warum die Geoidhöhe (N) fehlt — gemessen.** `N` braucht das **GRS80-Normalfeld**
  (`GM`, `ω`, `U₀`) und **γ₀** (Somigliana-Normalschwere). Im Baum liegt nur `a`/`1/f` in
  `src/archivar/projection.rs` (für die Karten-Projektion), **kein** Normalpotential und keine
  Normalschwere. Ohne sie ist `W − U₀` nicht gebildet → `N` bleibt ehrlich `pending`, nie eine
  erfundene Null. Der nächste begrenzte Schritt ist ein `normal_field`-Modul (GRS80-Konstanten +
  Somigliana-γ₀) mit Abgleich gegen die ICGEM-Geoid-Referenz (die vier N-Werte aus dem Form-Lauf,
  `docs/handover/archiv/handover-2026-10-15-mountain-folge355.md`), nicht geraten.
- **Die fünf angefragten Mountain-Linien (Operator-Auftrag) — was machbar war.** Verdikte in
  `state/mountain/2026-10-15-grenz-term-verdicts.φ` §5. **TIMED/Celestrak:** der Weltlinien-Compiler
  sampelt mit 60 s (`timed_tle_compiler.rs` `DEFAULT_CADENCE_S`); Haus-Präzedenz `wind_orbit`
  → `terms free-open …celestrak…` + **`ttl 7200`** + `format orbit_bin`; die CelesTrak-Policy
  nennt wörtlich „GP data … once every 2 hours" (Wayback-Messung) — die frühere `wind_orbit`-Konvention
  `86400` ist damit auf 7200 korrigiert.
  **SABER:** Höhenform (C) = Level im Schlüssel (Rat-Verdikt, `…verdikt.md:36`); die zwei
  `field`-Zeilen (K / km) stehen; der Host `cdaweb.gsfc.nasa.gov` bleibt unerreichbar → Block erst
  mit der Rückkehr; der eigene SABER-`terms`-Token ist gemessen: `CC0-1.0` (SPDF-Policy). **THEMIS FODG:**
  dieselben `terms free-open` wie die Geschwister (Quelle zugelassen); `ttl 604800`;
  format/url/origin/compiler = Mycelium. **GEOFON:** schon im Register (`phi:163-166`,
  `phi:10496-10498`, CC-BY-4.0) → keine Arbeit; das 404 ist Myceliums Transport. **SEANOE:** Datensatz
  = Argo GDAC (`10.17882/42182`), Lizenz **`CC-BY-4.0`** gemessen → `terms CC-BY-4.0
  https://creativecommons.org/licenses/by/4.0/`. **WDC-B:** die zwei termlosen Host-Blöcke
  (`esdb`/`stp`) sind auf `terms unknown` gesetzt (keine Lizenzaussage gemessen); `rjes`/`usd`/`www`
  bestätigt.
- **Zwei harte weite Läufe (Taucher) — die offenen Lizenz- und Geodäsie-Fragen gemessen.**
  **WSO Polar Field:** die Nutzung liegt auf `http://wso.stanford.edu/DataPolicy.html` (200) —
  notify + ackcknowledge + Beleg-Exemplar, kein formaler Lizenztext → `terms free-open`. **LISIRD/LASP:**
  keine eigene Terms-Seite; die About trägt eine Citation (DOI `10.25980/L27Z-XD34`), und die
  LASP-Guideline sagt „public data from NASA missions default to CC0 1.0, unless otherwise marked"
  → `terms CC0-1.0` (Rest-Riss: Default, nicht per-Datensatz). **SABER/TIMED:** SPDF-Policy existiert,
  das Haus führt SPDF als CC0-1.0 (wind_orbit `phi:3862`) → `terms CC0-1.0`. **GRS80/Geoid N:**
  die Konstanten und die Normalschwere sind belegt (GeographicLib `GRS80_*`; ICGEM `calc.js` GRS80
  `[a 6378137, 1/f 298.257222101, GM 3.986005e14, ω 7.292115e-5]`; γe `9.7803267715`, γp
  `9.8321863685`), und die geodätisch korrekte N-Bildung steht in SHTOOLS `MakeGeoidGrid.f95`
  (Wieczorek 2007, Gl. 19–20: `cilm2 = GM/r·cilm·(r0/r)^l`, `+ (ωr)²/3 − PotRef`). Damit ist der
  `normal_field`-Bau unten geerdet, nicht geraten.
- **Vier weitere harte weite Läufe (Taucher).** **SEANOE-Argo** (`10.17882/42182`): Lizenz
  **`CC-BY-4.0`** (DataCite-`rightsIdentifier`; SEANOE-REST-API) + Acknowledgment-Pflicht → gesetzt.
  **WDC-Hosts:** `esdb`/`stp` ohne Lizenzaussage → `terms unknown` (GESETZT, `phi:14408/14419`);
  `rjes`/`usd`/`www` bestätigt. **BGS-GIC:** keine Datensatz-Lizenz (© UKRI; nur die WDC-BGS-Seite
  „scientific use without restrictions") → `local-only`/kein Block. **BPA-GIC:** im Register bereits
  `terms PD-USGov` (`phi:337`) — die alte „kein Datensatz"-Zeile ist überholt (Riss benannt).
  **HIGP-MODVOLC:** keine offene Lizenz → `local-only` bestätigt. **CelesTrak:** die Usage-Policy
  nennt wörtlich „GP data … once every 2 hours" → TIMED-`ttl 7200` (86400 korrigiert). **JEOD bis
  Grad 20:** die `verif_out.txt` trägt gelesene Fälle bis Grad 20 (case 33 d/o 20/20 V
  `6.264194249827655E+07`) — ein GET-scriptbarer Zeuge für einen Test **jenseits Grad 10**; die
  2190-Naht bleibt `pending` (kein gemessener 2190-`.gfc`).
- **Die acht Fäden (mycelium-359) verdiktet — Mountain-Teil.** Verdikte in
  `state/mountain/2026-10-15-acht-faeden-verdicts.φ` (Grundlage: `survey-2026-10-15-tauchbefunde-acht-faeden`).
  Gesetzt/entschieden: **HMI** `field … G` + `ttl 86400` + `terms CC0-1.0`, Format `hmi` (eigener Arm);
  **SILSO** der stehende `unphysical`-Verwurf trägt (kein Block; die Quelle-Messung als zweite Zeile
  benannt); **XUV/log R'HK** die zwei Aktivitäts-`field`-Zeilen + EVE-Block, **phi:9898 repariert**
  (`ra_dr2`/`dec_dr2` → `ra`/`dec`, sonst HTTP 400); **Shear** `field e1`/`e2` `gravity`/`inverse-square`/`1`
  + `cmap`/`ra`/`dec`/`z_key`; **Neutrino** Identitäts-Verdikt: PAO1/S2E1/AMN1 → `S2Direction`
  (Code-Fix); Felder + `at earth` + `eev`-Unit; **EDGES** `terms CC-BY-4.0` + `field … K 2628000`;
  **PTA** `field TRES … s` + `at sun`; **TGF/Wolken** die `field`-Zeilen + `ttl`.
- **Zwei Code-Fixes (Mountain).** `src/archivar/witness.rs`: `AMN1`/`PAO1`/`S2E1` von `PointEvent`
  auf `S2Direction` (die vier Record-Compiler fordern es; die widersprechende Tabelle erklärt die
  404 der IceCube/Auger/AMON-Assets) — `ERBQ`/`MCON` bleiben `PointEvent`; Tests angepasst + neuer
  Test `directional_events_are_s2_directions_not_point_events`. `src/archivar/units.rs`: `pev`/`eev`
  in die Kraft-0- und Energie-Listen. `cargo check` 0/0, `--all-targets` 0/0, `rustfmt` exit 0.
- Register berührt: `phi/sources.φ` — zwei `terms`-Zeilen (`esdb`/`stp`) **und** die `phi:9898`-
  Reparatur (eigene Hunks); `register_sort` canonical, `--orphans` 0.

## Offen (aufgeschlüsselt)

### ICGEM gfc — Kern+Kopf+Gate+Konformanz+Funktionalklasse stehen; offen: Register-Deklaration + CDN (eigen)
- **Status:** eigen (Gate/Konformanz/Funktionalklasse) | **Bindung:** CDN = mycelium
- **Trigger:** der Feld-Bau.
- **Lage:** (gemessen/gebaut 2026-10-15) `gfc.rs` trägt `GfcHead`/`parse_head`/`GfcModel`/
  `harmonic_sums`, das **Liefer-Gate** (`gate_delivery`/`DeliveryRefusal`/`coeffs_from_records`)
  und jetzt die **`derived`-Klasse** als benannte Funktionale (`Functional`, `evaluate`); die
  Rat-Signatur M4.1 ist erfüllt (s. „Gearbeitet"). Der Modellkopf steht in der Quellen-Zeile
  (`SourceConfig.head`; `gm`/`radius`/`norm`/`tide`/`max_degree`). Der **M4-Rat** ist verdiktet
  (`state/stimmen/2026-10-15-mountain-rat-fragen-sh-kernel-verdikt.md`): eigene Klasse
  (`derived`), Koeffizienten im Draht, `V`/`Δg`/`N` je Zeile, Truncation registriert. **Der
  Konventions-Abgleich steht** (NASA JEOD, GGM02C Grad 10, relativ ~5·10⁻¹⁴); die ICGEM-Form-Referenz
  (Job-Ids) liegt zusätzlich vor. **Neuer Zeuge (Taucher 2026-10-15):** die JEOD-`verif_out.txt`
  trägt Fälle bis **Grad 20** (GET-scriptbar) — die Konformanz ist damit jenseits Grad 10
  nachprüfbar (Koeffizienten aus `earth_GGM02C.cc` bis 200).
- **Ungeglättete Risse:** Norm-Mehrdeutigkeit (die Formel gehört in den Kopf, nicht nur den Name);
  Force-Gate (SH hat keine Newton-Kraft); Höhenstabilität nur bis Grad 14 geprüft → `pending`;
  die 2190-Naht nicht gemessen → `pending`; der ICGEM-Tide-Riss; `GravityAnomaly` ohne
  Normalfeld-Reduktion (Aufrufer-Sache).
- **Blockade:** (a) die **Register-Deklarationsform** der Funktionalklasse (welche `phi`-Zeile/Token
  die `derived`-Klasse trägt — Schemafrage, nicht Solo); (b) CDN = Mycelium (`xfer:mountain-354-1`).
- **Braucht:** (1) die Register-Deklaration der Funktionalklasse (Arm/Token, nach Rat); (2) den
  CI-Bau des (grad-begrenzten) `gfc.bin` + Myceliums Direktiven; (3) [neu, eigen] einen JEOD-Test
  **Grad 20** (Fall 33/16) über den echten `parse_head`/`GfcModel`-Pfad, als Erweiterung des
  Grad-10-Tests.

### Die Norm in die Kanal-Semantik — Deklarations-Seite steht; Laufzeit-Name offen (eigen)
- **Status:** eigen | **Bindung:** die Laufzeit-Naht ist `main_flow` (River).
- **Trigger:** der Kanal-Atom (M4.7).
- **Lage:** (gemessen/gebaut 2026-10-15) Die **Deklarations-Naht steht**: der `gfc_compiler`
  schreibt `norm:<token>` an jede `field`-Zeile, der Parser (`parse.rs`, `field`-Zweig) liest
  und **verweigert** ein widersprechendes Feld-Token (nie still geeinigt); die Norm ist damit
  sichtbar in der Kanal-Deklaration, ohne `unit 1`-Tarnung. **Offen:** den Laufzeit-**Namen**
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
  `state/stimmen/2026-10-15-mountain-rat-fragen-grid-form.md` trägt die Frage, den gemessenen
  Stand (WGLC, CF-Konvention, Engine-/Archiv-Präzedenz aus zwei weiten Läufen von `bin/all_wide`)
  und die Sitz-Liste; der Operator sendet es an die Sitze, dann trägt der Rat das Verdikt.

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
  (`timed_tle_compiler.rs`, WOB1, Takt 60 s); Register-Blöcke noch ABSENT; `timed-orbit-cdn.yml` fehlt.
  **Riss:** „Schritt 2" nennt einen `StateVector{track:true}`-Produzenten; gebaut ist der
  `orbit_bin`-Weg (`Barycenter`). **Dieses Rund geliefert (§5):** Celestrak-`terms` + `ttl 7200`
  + `format orbit_bin` (TIMED, Präzedenz `wind_orbit`); die zwei SABER-`field`-Zeilen (K/km,
  Höhenform (C) = Level im Schlüssel, Rat) — der Host bleibt unerreichbar, der SABER-`terms`-Token
  pending.
- **Blockade:** keine (Bau offen).
- **Braucht:** (1) den TIMED-Register-Block (Mycelium, `ttl 7200` liegt); (2) die CI-Naht
  `timed-orbit-cdn.yml` (Mycelium); (3) das Riss-Gate (`measure`-Sonde, `VerdictWord::Riss`);
  (4) den SABER-Block, sobald `cdaweb.gsfc.nasa.gov` antwortet (die Feld-Zeilen, `on earth` und
  `terms CC0-1.0` stehen).

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
  `on earth 54.60282 -113.64442 693`. **Verdikt (dieses Rund):** UCAR/DASH-Lizenz ist nicht offen
  → `local-only` nach Kanon (`source-curation.md:90-102`): kein `phi`-Block, die `field`-Zeilen
  bleiben Code. Der Riss mit `quellen-neu-und-riss-ucar.md:19` („Register-Zeile schreiben") ist
  benannt — gegen die eigene Lizenz-Messung fl. 20 und gegen den Kanon.
- **Blockade:** keine (Verdikt steht).
- **Braucht:** nur noch die Arbeitskopie-Route (Myceliums Seite).

### Fünf fehlende Fäden — Entwurf steht, Mycelium setzt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom.
- **Lage:** (gemessen 2026-10-15) Mountain-Entwurf `state/mountain/faden-term-field-ttl-draft.φ`;
  **die zwei Risse sind geschlossen:** WSO-Polarfeld → `terms free-open
  http://wso.stanford.edu/DataPolicy.html` (Taucher); LISIRD/LASP → `terms CC0-1.0` nach LASP-Guideline
  (Rest-Riss: Default „unless otherwise marked"). `xfer:mountain-348-2`.
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
- **Lage:** (gemessen 2026-10-15) MODVOLC HIGP: keine Lizenzzeile → `local-only`, **kein Block**
  (Kanon `source-curation.md:90-102`; LANCE/NRT `phi:12065-12072` terms PD deckt das Register).
  **Myceliums Anfrage „`terms local-only` → Block" widerspricht dem Kanon** — es gibt kein
  `local-only`-`terms`-Token, und ein local-only-Datensatz erhält keinen `phi`-Block; der Riss
  ist benannt (`state/mountain/2026-10-15-grenz-term-verdicts.φ` §4a). OCNS2019 = Yuan 2021;
  der JPL-Ersatz ersetzt ihn nicht → Riss.
- **Blockade:** keine (Doku-/Verdikt-Arbeit).
- **Braucht:** (1) OCNS2019 als `local-only` stehen lassen (erledigt); (2) MODVOLC HIGP
  `local-only`, kein Block (erledigt); (3) einen echten offenen Beobachtungs-Ersatz
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

Origin: mountain-356 (2026-10-15):

- `xfer:mountain-354-1` **ICGEM-CDN-Block — Transport.** Der Compiler steht, der Modellkopf ist
  gemessen, die Funktionalklasse gebaut (V/Δg, N pending). **Offen ist der Transport:** das
  Haus-CDN-Asset `…/icgem.gfz.de/gfc.bin` ist **404**; die `url`/`origin`/`compiler`-Direktiven
  und der CI-Bau (93 MB → 5540 Grad, ggf. grad-begrenzt; heavy compute = CI) sind Myceliums.
  Mountain setzt nach dem Bau die `field`/`ttl`-Zeilen — nach dem Rat zur SH-Kernel-Form (M4).
- `xfer:mycelium-352-5` **TIMED Celestrak — der Block ist frei.** `terms free-open
  https://celestrak.org/usage-policy.php`, **`ttl 7200`** (gemessen: CelesTrak-Policy „GP data …
  once every 2 hours", Wayback; der Weltlinien-Takt ist 60 s im Compiler), `format orbit_bin` — geliefert in
  `state/mountain/2026-10-15-grenz-term-verdicts.φ` §5a. Der Block + die CI-Naht sind deine Seite
  (`timed_tle_compiler.rs`).
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
- **Zwei `local-only`-Anfragen korrigiert (dieses Rund, Kanon `source-curation.md:90-102`).**
  **HIGP-MODVOLC-Archiv** und **UCAR-FPI** tragen keine offene Lizenz → `local-only`: **kein
  `phi`-Block**, **keine** `terms local-only`-Zeile (es gibt kein solches Token), keine öffentliche
  `url`/`origin`/`compiler`-Fläche. Die Arbeitskopie lebt unter `$OMEGAFLOW_WORK/data/` und ist in
  eurer Übergabe benannt (Pfad · Zweck · Lizenzstand). Bitte die zwei Blöcke **nicht** setzen; der
  Riss (eure folge358 fordert „`terms local-only` → Block") ist benannt
  (`state/mountain/2026-10-15-grenz-term-verdicts.φ` §4). `xfer:mountain-348-1` (MODVOLC) ist damit
  beantwortet.
- **Die fünf angefragten Linien — der Stand für euch (§5).** **TIMED:** Block frei
  (`free-open`/`ttl 7200`/`orbit_bin`). **SABER:** die zwei `field`-Zeilen (K/km), `on earth` und
  `terms CC0-1.0` liegen; der Block wartet auf die Rückkehr von `cdaweb.gsfc.nasa.gov`.
  **THEMIS-ASI FODG:** dieselben `terms free-open` wie die Geschwister, `ttl 604800`;
  format/url/origin/compiler = ihr. **GEOFON:** schon im Register (`phi:163-166`, `phi:10496-10498`,
  CC-BY-4.0) — das 404 ist euer Transport. **SEANOE:** Argo GDAC (`10.17882/42182`) →
  `terms CC-BY-4.0 https://creativecommons.org/licenses/by/4.0/`; url/origin/compiler = ihr.
  **WDC-B:** `esdb`/`stp` sind auf `terms unknown` gesetzt (`phi:14408/14419`); `rjes`/`usd`/`www`
  bestätigt (CC-BY-4.0 / CC-BY-NC-4.0 / wdcb-sep-policy).
- **Terms gemessen (Taucher, hart/weit) — die Zeilen können gesetzt werden.** WSO-Polarfeld →
  `terms free-open http://wso.stanford.edu/DataPolicy.html`; LISIRD/LASP (und das SDO-EVE-L2) →
  `terms CC0-1.0` (LASP-Guideline: NASA-Missionsdaten default CC0) — Rest-Riss: Default, nicht
  per-Datensatz; SABER/TIMED → `terms CC0-1.0 https://spdf.gsfc.nasa.gov/data_use_policy.html`;
  SEANOE-Argo → `terms CC-BY-4.0`. **BGS-GIC:** keine Datensatz-Lizenz → `local-only`/kein Block.
  **BPA-GIC:** schon `terms PD-USGov` (`phi:337`) — die alte „kein Datensatz"-Zeile ist überholt.
  Steht in `state/mountain/faden-term-field-ttl-draft.φ` und `…grenz-term-verdicts.φ` §5b/§5d/§6.
- **Die acht Fäden (`xfer:mycelium-359-1…-8`) — Mountain-Antwort.** Vollständig in
  `state/mountain/2026-10-15-acht-faeden-verdicts.φ`. **1 HMI:** `field hmi_m_720s_blos … G` +
  `ttl 86400` + `terms CC0-1.0`; Format `hmi` (eigener Arm); **Riss:** euer `hmi_polar`-Compiler-Tag
  `jsoc1.stanford.edu` ist 404, das Asset liegt auf `ssd.jpl.nasa.gov` → korrigieren. **2 SILSO:**
  der `unphysical`-Verwurf trägt — kein Block (die Open-World-Messung als zweite Zeile benannt).
  **3 XUV/log R'HK:** `field logR_irt`/`logRHK` + EVE-Block; `phi:9898` repariert (`ra`/`dec`).
  **4 Shear:** `field e1`/`e2` `gravity`/`inverse-square`/`1` + `cmap`/`ra`/`dec`/`z_key`; sha256 +
  `tar_zst`-Reader = ihr. **5 Neutrino:** Identität = `S2Direction` gesetzt (`witness.rs`) — eure vier
  Compiler laufen jetzt; Felder + `at earth` + `format json`-Korrektur; `eev` ergänzt. **6 EDGES:**
  `terms CC-BY-4.0 https://zenodo.org/records/18091240` + `field brightness_temperature … K 2628000`;
  loco-CSV ohne SPDX → Zitat-Quelle. **7 PTA:** `field TRES … s` + `at sun`. **8 TGF/Wolken:**
  die `field`-Zeilen + `ttl 2592000`; Open-Meteo-`origin` um `cloud_cover*` erweitern.
  Damit ist `xfer:mycelium-359-1…-8` beantwortet; `url`/`origin`/`compiler`/`sha256`/CDN-Naht = eure Hand.
- Aus mycelium-351/348 offen: `-348-2` (Fünf Fäden) ·
  `-348-4` (OCNS2019-Ersatz) · `-342-10` · `-343-2`.
  (`-348-5`/TIMED, `-342-4`/GEOFON und `-342-3`/SEANOE sind beantwortet.)

## An river

Origin: mountain-356 (2026-10-15):

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

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes."** | 2026-10-14/15 | (mountain 344–348, 352, 353, 354, 355, 356).
- **„Die letzte session schreibt Committet und gepusht — alles von dieser Sitzung ist in `origin/main`. Die letzte Hand war der Abschluss-Commit der Übergabe (`b10dd20d9`)."** | 2026-10-15 | (mountain 355) — die Vor-Session ist committet/gepusht; kein erneutes Vorlegen.
- **„die letzte session schreibt Verstanden — die Kritik sitzt, keine Ausreden."** | 2026-10-15 | (mountain 354).
- **„Der nächste Block ist der Schwere-Bau für ICGEM … zuerst den Modellkopf selbst … Danach steht der Kern."** | 2026-10-15 | (mountain 354) — Kopf gemessen, Kern gebaut.
- **„… dafür braucht es dann vermutlich nochmal recherche wide all roster (bereite das dokument vor) und den rat."** | 2026-10-15 | (mountain 354) — M4-Rat gelaufen/verdiktet.
- **„hab kimi nachgetragen."** | 2026-10-15 | (mountain 354) — 10 M4-Stimmen, Kopfweg 5 : 5.
- **„ich würde sagen in den kasten weil ein eintrag in die zahlen eine veränderung der rohdaten ist."** | 2026-10-15 | (mountain 354) — Kopfweg (b), gebaut.
- **„Bereit für die frische Session … (1) ICGEM: das Liefer-Tor … dann der fremde Abgleich … Der CDN-Block liegt bei Mycelium."** | 2026-10-15 | (mountain 355) — das Liefer-Tor ist gebaut; der Rechenservice ist form-only gemessen, die fremde Referenz gezogen; der Abgleich folgt mit der `derived`-Klasse.
- **„kein ‚nächster Dispatch', kein ‚nächste Session', kein Vorlegen eines bereits geworteten Punktes."** | 2026-10-15 | (mountain 355) — Arbeitsdoktrin dieses Runds.
- **„Die nächste Session liest `…-mountain-folge354.md` als Erstes und dispatcht dann die Taucher auf genau diese Punkte."** | 2026-10-15 | (mountain 355) — folge354 konsumiert → `archiv/`.
- **„ist das alles mycelium oder auch deins? du solltest taucher losschicken nicht verschleppen."** | 2026-10-15 | (mountain 355) — Taucher mit harten Bandagen losgeschickt.
- **„ich dachte dass sei auch schon festgelegt Norm in die Kanal-Semantik."** | 2026-10-15 | (mountain 355) — die Deklarations-Seite ist gebaut (Compiler + Parse-Gate); kein neues Wort für die Deklaration.
- **„ist das der einzige pfad oder hast du nochmal archive search allwide mit harten bandagen versucht?"** | 2026-10-15 | (mountain 355) — `bin/all_wide` und Sourcegraph gefahren; die NASA-JEOD-Referenz gefunden; der Abgleich ist gefahren.
- **„kannst du dich bitte um deinen teil kümmern? Klare Antwort, alles drei:"** | 2026-10-15 | (mountain 355) — Mountains Teil der Grenz-Anfrage: WGLC-Lizenz-Riss entschieden + Terms/ttl im Entwurf; TIMED-Celestrak-`terms` entschieden; Venera-Polar-`terms` bestätigt (`phi:31325`).
- **„ja bitte die ratsfrage"** | 2026-10-15 | (mountain 355) — die M5-Rat-Frage (global gemessene Gitter-Form) ist gestellt (`state/stimmen/2026-10-15-mountain-rat-fragen-grid-form.md`).
- **„ja es ging um WGLC"** | 2026-10-15 | (mountain 355) — Rivers Frage ist die WGLC-`field`-Form; M5 trägt sie (Sender River/F4-Schumann), kein zweites Paket.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. Dieses Atom
(mountain-356) hat die **`derived`-Klasse des ICGEM-Schwerebaus gebaut** (`Functional`:
`Potential`/`GravityAnomaly`/`GeoidHeight` mit SI-Einheiten und `evaluate(r,colat,lon,funktional)`
in der Rat-Signatur M4.1; die verschmolzene `synthesize`/`GfcValue` entfernt; `N` liefert `None`
— pending ohne GRS80-Normalfeld, nie `0.0`), den Messweg `gfc_probe` auf die Funktionale
umgestellt, den JEOD-Grad-10-Abgleich durch die neue Klasse geführt und den Messlauf belegt
(Potential/Δg endlich, N pending). `cargo check` 0/0, `cargo check --all-targets` 0/0,
`rustfmt --edition 2024 --check` exit 0, `cargo build -p omegaflow-harvest --bin gfc_probe` ok.
Offen: die Register-Deklarationsform der Funktionalklasse (Schema), der Laufzeit-Kanalname über
die Membran (`xfer:mountain-355-1`, River), der CDN-Block (Mycelium). Der Stehende Pass wird
zitiert, nie kopiert. Diese Fortschreibung (folge355 → `archiv/`).
