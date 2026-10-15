<!--
  title: Handover — Mountain-Folge 354 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: 7886e83d3a97492bc25dfaae35b1dd277548899e7303be75083c6e2d098d12cf
  status: live
  see-also: docs/concepts/tool-forms.md, AGENTS.md, docs/surveys/survey-2026-10-15-ketten-quellen-harter-lauf.md, docs/surveys/survey-2026-10-15-riss-alternativen.md
-->
# Handover — Mountain-Folge 354 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Diese
Session konsumierte `handover-2026-10-15-mountain-folge353.md` (→ `archiv/`).

Still gemessen: `register_lookup --fired mountain` = 0, `--stale mountain --persist 3` = 0;
`open_points_check` auf folge353: 23 Pfad-Refs, 0 absent, 0 stale-citations, 0 done-carried,
0 word-carried, 0 guardians, 0 format-gaps, 0 owner-drift, 0 post-md · `register_lookup
--orphans --fail` 0 · `register_sort phi/sources.φ` canonical (2807 Blöcke).
Die zwei adressierten Blöcke (`mycelium-folge357`, `river-folge218`) sind gefaltet (Echo unten).
Der Stehende Pass wird als **öffentliche** Fassung zitiert, nie kopiert.

## Burn: open 0.0 · close nicht gemessen — `session_burn` meldet `db absent`
(`~/.local/share/opencode/opencode.db` fehlt); der Platzhalter `0.0` ist **kein** Meßwert
(Vorbehalt wie mountain-348/349/351/352/353).

## Gearbeitet (dieses Rund)

- **Der ICGEM-Modellkopf ist gemessen (das Erste des Schwere-Baus).** Am Modell XGM2019e
  (`https://icgem.gfz.de/getmodel/gfc/8e01a2aba4f291ac0638b79a390155713b70fdc9b266981023c756839c11c9b8/XGM2019e.gfc`,
  HTTP-Range 0-30000, 2026-10-15): `product_type gravity_field`, `modelname XGM2019e`,
  **`earth_gravity_constant` 3.986004415E+14 m³/s²**, **`radius` 6378136.3 m**,
  `max_degree 5540`, **`norm fully_normalized`**, **`tide_system zero_tide`**. Das Format-Doc
  („The ICGEM-format", Förste/Barthelmes/Ince, 16.02.2023, `icgem.gfz.de/files/6defd9f8….pdf`)
  nennt genau zwei `norm`-Werte (`fully_normalized` Default · `unnormalized`) und liefert
  **keine** explizite 4π-Formel; die Normierung ist darum am Code verankert und numerisch geprüft.
- **Der Kern steht — die Kugelflächen-Synthese in `src/archivar/gfc.rs`.** `GfcHead` +
  `parse_head` (Modellkopf aus dem `.gfc`-Text, inkl. `product_type`/`tide_system`; ein
  unbekanntes `norm`-Token bleibt **absent**, nie geraten); `GfcModel` (Kopf + Koeffizienten
  aus Text/26×f64-Draht); `Norm::{FullyNormalized,Unnormalized}`; die stabile
  **fully-normalized 4π-Rekursion** `fully_normalized_legendre` (Holmes/Featherstone-Form)
  und `unnormalized_legendre`; `harmonic_sums`/`synthesize` liefern das Geopotenzial
  `GM/r·Σ(a/r)ⁿ…` und die `(n−1)`-gewichtete Anomalie `GM/r²·Σ(n−1)…` (die Störungs-Reduktion
  bleibt der Aufrufer — das Normalfeld liegt nicht im ICGEM-Kopf). `GFC_SYNTHESIS_CONVENTION`
  benennt die Formel; ein Test verifiziert die Rekursion gegen die Definition
  `P̄_nm = √((2−δ)(2n+1)(n−m)!/(n+m)!)·P_nm` bis Grad 14. **12/12 `archivar::gfc`-Tests grün**,
  `cargo check` 0/0, `rustfmt --edition 2024 --check` exit 0.
- **`gfc_compiler.rs` liest jetzt den vollen Kopf** (`product_type`/`tide_system` dazu) und
  nutzt `gfc::parse_head` (kein Duplikat mehr); `norm`/GM/radius/tide gehen an den Auswerter
  (`GfcModel.head`), nicht mehr nur nach stderr. Funktionslauf gegen den gemessenen
  Range-Kopf: d/o 5540, GM 3.986004415e14, radius 6378136.3 m, norm fully_normalized,
  tide_system zero_tide.
- **Gefunden (fremd, rot):** `cargo build -p omegaflow-harvest --tests` bricht an
  `nist_ncnr_nse_nexus_compiler.rs:510` (`Option::len`) und `glofas_ewds_compiler.rs:686`
  (`GlofasCollection: Debug`) — vorbestehende Fremd-Rott, Träger des roten `tools-build`.
  Nicht angefasst (fremde Dateien); nur benannt.
- Register unberührt: `register_sort` canonical (2807), `--orphans` 0.

## Offen (aufgeschlüsselt)

Gefaltet (Echo in diesem Block): `xfer:mycelium-352-1` · `xfer:mycelium-352-2` ·
`xfer:mycelium-352-3` · `xfer:mycelium-352-5` · `xfer:mountain-348-1` · `xfer:mountain-348-2` ·
`xfer:mountain-348-4` · `xfer:mountain-348-5` · `xfer:mycelium-342-3` · `xfer:mycelium-342-4` ·
`xfer:mycelium-342-10` · `xfer:mycelium-343-2` · `xfer:mycelium-356-5` (WGLC — Lizenz gemessen,
Antwort unten) · `xfer:mycelium-356-1` (SABER/UCAR-FPI — die Arm-Antwort steht bei UCAR-FPI).

### WGLC-Zulassung — Lizenz gemessen, `field`-Form ist Rat (eigen; Antwort an mycelium)
- **Status:** eigen (Verdikt = Mountain) | **Bindung:** mycelium-357 → mountain
- **Trigger:** `xfer:mycelium-356-5`.
- **Lage:** (gemessen 2026-10-15, dieses Rund) Zenodo-Record `4774528`/Version `20277101`
  (`https://zenodo.org/api/records/4774528`) trägt **`license.id = cc-by-4.0`**; Mycelium maß
  im Archiv das README-Badge **CC BY-SA 4.0** → **Lizenz-Riss** (zwei unabhängige Zeilen,
  beide offen, nie geglättet). Zeitauflösung: Record-Beschreibung nennt **stündlich** (0,5°/5′)
  plus **30-Minuten**-Stoßleistung, aggregiert zu täglich/monatlich (Myceliums „täglich"
  ist die Datei-Körnung der Dichte). `wglc_compiler.rs` steht.
- **Blockade:** die `field`-Form für einen globalen Blitz-Dichte-Grid ist die Rat-Schemafrage
  (derselbe Absenz-/`field`-Pfad wie WDC-B `gdata`); die Lizenz-Frage ist der benannte Riss.
- **Braucht:** das Rat-Verdikt zur `field`-Form (Lightning-Grid); dann setzt Mountain die
  Zulassungs-/`ttl`-/`terms`-Zeilen (mit dem Lizenz-Riss benannt).

### WDC-B `field` je Datensatz — Kernel-Zuordnung ist Rat (eigen)
- **Status:** eigen (field = Mountain; Kernel = Rat) | **Bindung:** eigen
- **Trigger:** `xfer:mycelium-352-1`/`-2` (die 179 Wayback- und 7 lebenden `sep`-Blöcke).
- **Lage:** (gemessen 2026-10-15) Die Quantitäten/Einheiten der Groups sind gemessen
  (G CGS · 1-min-Geomagnetik nT · Freiluft-Schwere mGal+Geoid m · Herdparameter · aa-Index nT ·
  Pc1-Pulsationen · GNSS-RINEX). **Das Roster-Verdikt steht** (`state/stimmen/2026-10-15-mountain-rat-fragen-verdikt.md`,
  13 Sitze, am Baum verifiziert): **drei tragen ein Feld** — `geo_min` als Stationszeuge an
  `Position::Surface` plus getrennt das IGRF-SH (`IgrfCoeffs::synthesize`, `igrf.rs:131`),
  `gdata`/`mgg` als Schwere-SH (`gfc`/`vmf3_sh`, Anomalie ↔ Geoid zwei Zeilen), `catal_pc` als
  Wellen-Ereignis (Kernel `pending`); **vier tragen keins** — G (Konstante), `sdata`
  (Punkt-Prozess), `aa` (Index), `gps` (Metadaten). Die `field`-Form (`<kernel> <force> <unit> …`,
  nur 6 Kernel-Formen, Force-Gate) kann **Nicht-Felder nicht tragen** → der Absenz-Pfad
  (`scalar_constant`/`point_process`/`planetary_index`/`wave_event`/`metadata`) ist eine
  **Schema-Erweiterung** (Rat). D ist ein **Winkel** (rad), kein nT-Wert.
- **Blockade:** Rat (Schema-Erweiterung um Nicht-Feld-Typen).
- **Braucht:** (1) das Rat-Verdikt zur Absenz-Pfad-Form; (2) dann die Feld-/Absenz-Zeilen je Datei.

### Ketten-Quellen (SWIFF · WINDMI · MAGION · MOSWOC-Kp · OVATION-AACGM) (eigen)
- **Status:** eigen · **gemessen 2026-10-15** | **Bindung:** eigen
- **Trigger:** der harte Ketten-Quellen-Lauf (Operator-Wort 2026-10-15) — abgearbeitet.
- **Lage:** (gemessen 2026-10-15) volles Messblatt `docs/surveys/survey-2026-10-15-ketten-quellen-harter-lauf.md`.
  SWIFF/WINDMI/MAGION bleiben als **kein Datensatz** benannt. MOSWOC-Kp-Zugang entfällt
  (GFZ-Kp deckt den Bedarf). **OVATION-AACGM (Zenodo `19651247`, CC-BY-4.0, eine Station Mohe):
  Verdikt steht** (Roster) — es ist ein **Modell-Zeuge** (Punkt-Auswertung eines globalen
  Energiefluss-Modells an Mohe), **kein Feld, kein Schalter**; Typ `model_output`, Antriebsvektor +
  Modellversion Pflicht. **Span/Takt/Version/Treiber ungelesen** → `pending`. **Operator-Entscheid
  offen:** jetzt als `model_output` (F3-nah) aufnehmen oder auf ein globales Gitter warten?
  **Operator-Wort 2026-10-15 (entschieden):** „wie bei den maps, die aus messpunkten entstanden
  sind — wir wollen die **gemessenen gitterpunkte**, mit denen die modelle erstellt wurden, also
  die **rohdaten**." Träger ist die **Messung**, nicht der Modell-Output; das Modell ist abgeleitet.
  Für OVATION ist die DMSP-SSJ-Partikelmessung `dmsp16_ssj` (`phi:1670-1671`, `em ev/cm²/ster/s`)
  **schon registriert**; der Mohe-Modell-Extrakt wird **nicht als Feld** eingetragen (er wäre
  `model_output`/F3-nah, `pending`). **Ins `sources.φ` gehört die Messung** (das rohe
  Punkt-/Gitterfeld, DMSP-SSJ); das **Modell** ist abgeleitet — Präzedenz der abgeleiteten
  Netz-Indizes (AE/TEC, Rat 2026-10-08, `docs/concepts/archivar-mathematikerin.md:19`): eigenes
  `kind` (`index`/`derived`), **ohne Kraft** (ein `force` würde behaupten, ein Organismus nehme
  das Modell wahr — A ≠ A), die rohe per-Punkt-Messung bleibt eine **eigene** Quelle. Ob ein
  `derived`/`model_output`-Kind zugelassen wird, ist die Rat-Schemafrage — das Modell muss **nicht**
  als Feld in die `sources`. **Messgrundlage erweitert (Operator-Auftrag „kümmerst du dich drum"):**
  F17/F18-SSJ sind jetzt registriert (`dmsp17_ssj`/`dmsp18_ssj`, `phi`, je `origin …/dmspf17|18/ssj/…`,
  `terms CC0-1.0 spdf data_use_policy`, `em ev/cm²/ster/s`); der Producer `dmsp16_ssj_compiler.rs`
  nimmt `--sat f16|f17|f18` (MAGIC `DS16/17/18`), der Lese-Arm `dmsp16_ssj.rs` parst die Familie.
  Register: `register_sort` canonical (2807 Blöcke), `cdn_reconcile --fail` clean, `--orphans` 0.
  **Grenzübertritt benannt:** die `url`/`origin`/`compiler`-Direktiven sind Myceliums; Mountain hat
  sie auf das Operator-Wort gesetzt (Mycelium informiert, s. `## An mycelium`).
- **Blockade:** keine — das Operator-Wort liegt, die Messung steht (F16/F17/F18).
- **Braucht:** nichts an der Aufnahme; optional den Zip-Kopf (Span/Takt), falls der Extrakt als
  `model_output` geführt werden soll.

### SABER — Arm + Weltlinie stehen; Takt + Riss-Gate offen (eigen)
- **Status:** eigen · **verdiktet** (B + A) | **Bindung:** eigen
- **Trigger:** der Arm-Bau.
- **Lage:** (gemessen 2026-10-15) Der Zeilen-Arm steht; TIMED-Weltlinie gebaut (`timed_tle_compiler.rs`, WOB1).
  Register-Blöcke noch ABSENT; `timed-orbit-cdn.yml` existiert nicht. **Riss:** die Übergabe
  nennt „Schritt 2" einen `StateVector{track:true}`-Produzenten; gebaut ist der `orbit_bin`-Weg
  (`Barycenter`); beide Zeugen stehen.
- **Blockade:** keine (Bau offen).
- **Braucht:** (0) den Celestrak-GP-`terms`-Token (s. u., ungemessen); (1) den TIMED-Register-Block;
  (2) die CI-Naht `timed-orbit-cdn.yml`; (3) das Riss-Gate (`measure`-Sonde, `VerdictWord::Riss`);
  (4) den SABER-Block (`on earth`, Höhenform (C)); (5) den SABER-Takt aus einer Live-`data`-Probe.

### Venus Radar — VNRD-Block steht, CI grün; Bild-Manifestation offen (eigen)
- **Status:** eigen (Operator-Wort: W — das Bild bleibt Bild) | **Bindung:** eigen
- **Trigger:** der Projektions-/Manifestations-Bau.
- **Lage:** (gemessen 2026-10-15) VNRD-Block steht (CC0-1.0); `venus-radar-cdn` **success**. Offen: die Bild-Manifestation.
- **Blockade:** keine.
- **Braucht:** die Bild-Manifestation (W).

### ICGEM gfc — Kopf gemessen + Kern gebaut; offen: Block + Runtime-Naht (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Feld-Bau.
- **Lage:** (gemessen/gebaut 2026-10-15, dieses Rund) Der Kopf ist gemessen und der Kern steht
  (s. „Gearbeitet"): `gfc.rs` trägt `GfcHead`/`GfcModel`/`synthesize` (fully-normalized 4π),
  `gfc_compiler.rs` liest den vollen Kopf. **Geschehen sind:** GM/radius/norm/tide an den
  Auswerter; die Kugelflächen-Synthese. **Offen bleibt:**
  (a) die **Runtime-Naht** — der ω()-Loop liest den 26×f64-Draht, aber kein Kernel ruft
  `GfcModel::synthesize`; `declared_fields` deklariert weiter die Koeffizienten-Kanäle
  (`icgem_gfc_c/_s`), keine Δg/N-Zeile (Δg mit Gewicht (n−1), N = T/γ₀ braucht die
  Störungs-Reduktion + γ₀ — der Aufrufer, nicht der Kopf);
  (b) der **Block** — Haus-CDN-Asset `.../icgem.gfz.de/gfc.bin` = 404; die volle 93 MB → 5540
  Grad ist ein CI-Bau (Consent: heavy compute in CI), und `url`/`origin`/`compiler` sind
  Myceliums Direktiven.
- **Blockade:** die Runtime-Naht (welcher Kernel trägt die SH-Synthese) ist eine Rat-Frage;
  der Block die CI-Manifestation.
- **Braucht:** (1) das Rat-Verdikt zur SH-Kernel-/`field`-Form (mit `field`-Form für `gdata`/`mgg`
  zusammen — die `field`-Zeile ist heute ein `<kernel> <force> <unit>`-Slot für 6 Kernel-Formen);
  (2) den CI-Bau des (ggf. grad-begrenzten) `gfc.bin` + Myceliums Block-Direktiven.

### UCAR-FPI — Arm + Zeilen stehen; Manifestation wartet auf die `field`-Zeilen (eigen)
- **Status:** eigen (Rest Mycelium-/River-gebunden) | **Bindung:** eigen
- **Trigger:** die `field`-Zeilen aus dem Sample.
- **Lage:** (gemessen 2026-10-15) Arm + Guard stehen (`fpi_ucar.rs`); die zwei `field`-Zeilen `fpi_mer_wind_ms`/
  `fpi_zon_wind_ms` sind code-gemessen; Athabasca-Koordinate `on earth 54.60282 -113.64442 693`.
- **Blockade:** nicht-offene Lizenz → `local-only` (kein CDN).
- **Braucht:** **kein `phi`-Block** (`source-curation.md:90-102`: `local-only` heißt *keine* `phi`-Zeile, kein CDN, keine `origin`/`url`/`compiler`-Fläche — die frühere Forderung „UCAR-FPI-Block" war der Riss); die zwei Feldnamen `fpi_mer_wind_ms`/`fpi_zon_wind_ms` stehen als Code im Lese-Arm (`extract.rs:540`); offen bleibt nur die Arbeitskopie-Route.

### Fünf fehlende Fäden — Entwurf steht, Mycelium setzt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom.
- **Lage:** (gemessen 2026-10-14) Mountain-Entwurf `state/mountain/faden-term-field-ttl-draft.φ`; zwei Risse offen
  (WSO-Polarfeld ohne Terms; LISIRD ohne Terms). `xfer:mountain-348-2`.
- **Blockade:** keine.
- **Braucht:** Mycelium setzt `url`/`origin`/`compiler`; dann `register_sort`.

### Quellen-Gewebe — Kraft × Kernel; die dünnen Fäden (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom.
- **Lage:** (gemessen 2026-10-14) Messblatt `docs/surveys/survey-2026-10-14-gewebe-faeden.md`. EGGN/CGrav/GOCE/NIED
  sind als `pending`/`riss` benannt. Riss: `survey-2026-10-05` zitiert `dead_sources.φ`/
  `declined_sources.φ` — beide existieren nicht im Baum.
- **Blockade:** die Kernel-Zuordnung ist Rat.
- **Braucht:** (0) IGETS-`origin` steht (`sftp://igetsftp.gfz.de`); (1) die Kernel-Lücke schließen (Rat).

### Adressierte Register-Fäden — OCNS2019 `local-only`, MODVOLC-Korrektur (eigen)
- **Status:** eigen (Register-Writes) | **Bindung:** eigen
- **Trigger:** der Registeratom.
- **Lage:** (gemessen 2026-10-15) MODVOLC HIGP: keine Lizenzzeile → `local-only` (kein Ersatz für den LANCE/NRT-Block).
  OCNS2019 = Yuan 2021, Beobachtungs-Katalog; der JPL-Ersatz ersetzt ihn **nicht** → Riss
  bleibt. Riss: die Forderung „MODVOLC braucht einen neuen Block" widerspricht
  `source-curation.md` (ein `local-only`-Ursprung bekommt keine `phi`-Zeile).
- **Blockade:** keine (Doku-/Verdikt-Arbeit).
- **Braucht:** (1) OCNS2019 als `local-only` stehen lassen (erledigt); (2) MODVOLC HIGP
  `local-only`, **kein Block**; LANCE/NRT bleibt — die Forderung ist per `source-curation.md:90-102`
  widersprochen und die Live-Übergabe trägt jetzt die korrekte Disposition (erledigt);
  (3) ein echter offener Beobachtungs-Ersatz (Lead NSDB `10.1051/0004-6361/200911645`).

### `em mw` / `advective kg/m2/s` — Freeze (eigen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das `tools-latest`-Republish.
- **Lage:** (gemessen 2026-10-13) `units.rs` admiert beide; `docs/specs/force-unit-baseline.txt` trägt sie froren.
- **Blockade:** keine.
- **Braucht:** die drei Paare nach dem Republish aus der Freeze nehmen.

### ALFALFA · Ersatz-Bau · eigene Ephemeride (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je der Bau/Fund.
- **Lage:** (gemessen 2026-10-15) ALFALFA `local-only` bestätigt. OpenTopography-Compiler liest in Wahrheit
  Scripps SRTM15+ (Namens-Riss); Höhenbezug des OT-Produkts EGM96/orthometrisch, Toleranz
  absent. ILRS NPT: Producer steht, **kein Lese-Arm in `extract.rs`**; Route EDC offen.
  `icrf3_compiler.rs` liest drei IERS-Dateien, kein SKD1/Round-Trip.
- **Blockade:** keine.
- **Braucht:** (1) Höhenbezug in die `opentopography_raster`-Zeile oder Compiler umbenennen;
  (2) `load_gestalt_surface_threads` an einen Produktions-Aufrufer binden **oder** `descoped`;
  (3) `ilrs_npt`-Lese-Arm in `extract.rs` + Block; (4) `icrf3` versionieren + Round-Trip + SKD1.

### Riss-Alternativen — fünf Taucher; NSDB-Vertrag geschlossen, Aurora-Bau Rat-gebunden (eigen)
- **Status:** eigen · **gemessen 2026-10-15** | **Bindung:** eigen
- **Trigger:** Operator-Frage 2026-10-15 („und für all die risse gibt es keine alternativen?") — abgearbeitet.
- **Lage:** (gemessen 2026-10-15) Messblatt `docs/surveys/survey-2026-10-15-riss-alternativen.md`. Jeder Daten-Riss
  trägt eine offene Route. **NSDB:** der Producer (`nsdb_compiler.rs`) und der Lese-Arm
  (`nsdb.rs`) stehen; der **Contract-Riss ist geschlossen** (AST1 trägt jetzt die fehlerlose
  Richtungs-Serie, `nsdb::to_astro_series`). **BAS-IMAGE/DMSP-SSJ:** die Format-Spec ist
  vermessen (s. „Gearbeitet") und das **Roster-Verdikt steht** (`…-rat-fragen-verdikt.md`): die
  Grenzbreite ist ein **Ort**, kein Kraftfeld — BAS als Skalar auf der geschlossenen 24-Bin-MLT-Achse
  (Bin im Schlüssel), DMSP als Pass-Punkt-Locus; `NaN → None`; Leitfähigkeit/FAC ist Begleiter
  `pending`. **48 > 26:** je Bin ein Record, kein Packen. **16-vs-24 offen** (Zip nicht entpackbar).
- **Blockade:** (1) Rat (Schema: closed-discrete MLT-Achse + Absenz-Pfad); (2) DMSP-Zip entpacken.
- **Braucht:** (1) das Rat-Verdikt zur MLT-Achsen-/Absenz-Form; (2) ein Entpack-Werkzeug für die
  DMSP-CSV (Sandkasten ohne unzip); (3) die vier Mycelium-Registrierungen
  (`xfer:mountain-351-1…-5`); (4) die NSDB-Witness-Route in den S²-Pfad (main_flow = River) —
  `nsdb::to_astro_series` liefert die AST1-Richtungszeugen.

### Axiom-Verwechslung + ICRS/TDB/pending-Fabrikations-Audit (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Operator-Frage 2026-10-15 („das sind nicht die 5 axiome … suche nach icrs tdb und pending wo wird das noch überall fabriziert?").
- **Lage:** (gemessen 2026-10-15) Die fünf Axiome sind der **Granit** (`docs/granit.md:16-20`; Beleg `docs/concepts/glossar.md:85`): **A = A · ICRS & TDB · channel_ref · 0 geehrt · pending** — das Gate lädt genau diese Datei (`src/gate/axioms.rs::load`, dessen Test `A = A`/`0 honored`/`channel_ref`/`ICRS & TDB`/`pending` prüft). Die **ethischen** Grundsätze aus `AGENTS.md` tragen denselben Namen „die fünf Axiome" — eine Verwechslung, verbreitet in fremden Dokumenten: `state/stimmen/2026-10-15-netzfusion-formular.md` / `…-manifest.md`, `…-rat-fragen-katalog.md` / `…-rat-fragenpaket.md`, `state/stimmen/2026-10-15-nachfrage-f7-f4-wissenschaft.md` (benannt, als fremde Dateien nicht still geändert). Der Granit trägt **7** Sätze (dazu „Der Operator schließt", „Manifestiert, nicht nur lokal"). **Herkunft (gemessen am `git log`):** den Namen „die fünf Axiome" in `AGENTS.md` führte `78c5104cb` (2026-10-14, johannestyroller) ein — der Titel der Ethik-Sektion `## Kybernetische Ethik` → `## Kybernetische Ethik — die fünf Axiome + der 0-Kanon-Kern`; `git log -S "ICRS & TDB" -- AGENTS.md` = **leer** (AGENTS.md trug die Granit-Fünf nie); die Glossar-Fünf (`3c84bede2`) und der Granit (`docs/granit.md`, seit `200ed6040`) stehen unverändert. **Fabrikations-Audit (ICRS/TDB/pending):** der Produktions-Code ist frei von Körper-Namens-Literalen — die Treffer `"earth"` (89) / `"mars"` (57) / `"sun"` (20) liegen in `#[cfg(test)]`-Modulen / `tests.rs`; **Drift (geheilt dieses Rund, s. „Gearbeitet"):** die J2000-Anker-Konstante `946728000.0` stand dreifach (`lsk.rs:3`, `mcon_event.rs:9`, `quake_event.rs:10`) — jetzt einmal in `types.rs`, die Doc-Namen auf `J2000_UNIX_OFFSET` gezogen; **Lücke (offen):** `VerdictWord` (`weberin_verdicts.rs:9-15`) kennt kein `Pending` — das Verdikt-Register kann „noch nicht gemessen" nicht tragen (`from_code` → `None`, der Parser bricht ab).
- **Blockade:** keine.
- **Braucht:** (1) die fremden Fragebögen/Dokumente auf den Granit ziehen (deren Linien);
  (2) prüfen, ob ein `VerdictWord::Pending` das Register-Vokabular erweitern muss (Rat).

## An mycelium

Origin: mountain-354 (2026-10-15):

- `xfer:mycelium-356-5` **WGLC-Zulassung — Antwort.** Die Zenodo-Record-Lizenz ist gemessen:
  `https://zenodo.org/api/records/4774528` → `license.id = cc-by-4.0`, Creator Kaplan, J. O.,
  2026-05-18. Der **Riss** zum README-Badge **CC BY-SA 4.0** steht benannt (beide Zeilen, nicht
  geglättet). Die Zulassung selbst (die `field`-/`ttl`-/`terms`-Zeile) wartet auf das
  **Rat-Verdikt zur `field`-Form** eines globalen Blitz-Dichte-Grids (derselbe Schema-Riss wie
  WDC-B `gdata`); darum ist `xfer:mycelium-356-5` hier **gefaltet** (Antwort), die Zeile folgt
  dem Rat. Der `wglc_compiler.rs` steht bei dir.
- `xfer:mycelium-352-1` (WDC-B-`field`) und `xfer:mycelium-352-2` (`sep`-`field`) — die
  Quantitäten/Einheiten sind gemessen; die `field`-Zeile braucht die **Kernel-Zuordnung (Rat)**.
  Nach dem Rat-Verdikt setzt Mountain die Zeilen. Kein stiller Kernel.
- `xfer:mycelium-352-3` (Host-`terms`) — gesetzt: `rjes` CC-BY-4.0, `usd` CC-BY-NC-4.0,
  `www.wdcb.ru/sep/` `wdcb-sep-policy`; `esdb`/`stp` bleiben ohne (kein eigenes Terms).
- `xfer:mycelium-352-5` (TIMED Celestrak-`terms`) — **ungemessen**: der Celestrak-GP-Terms-Token
  ist am Baum nicht gemessen (kein `celestrak`-`terms` in `phi`). Braucht die Messung der
  Celestrak-GP-Daten-Terms (Policy-Seite), dann setzt Mountain den Token.
- Aus mycelium-351/348 noch offen: `xfer:mountain-348-1` (MODVOLC `terms local-only`) ·
  `xfer:mountain-348-2` (Fünf Fäden) · `xfer:mountain-348-4` (OCNS2019-Ersatz) ·
  `xfer:mountain-348-5` (TIMED-Block) · `xfer:mycelium-342-3`/`-4` (SEANOE/GEOFON) ·
  `xfer:mycelium-342-10`/`xfer:mycelium-343-2` (WDC/terms).
- Die offenen Registrierungen der Riss-Alternativen bleiben `xfer:mountain-351-1…-5`
  (stehen in folge351 → in `archiv/`).

- `xfer:mountain-352-4` **DMSP-SSJ F17/F18 — die Messgrundlage.** Die zwei Blöcke stehen
  (`dmsp17_ssj`/`dmsp18_ssj`, `origin https://cdaweb.gsfc.nasa.gov/pub/data/dmsp/dmspf17|18/ssj/precipitating-electrons-ions/`,
  `terms CC0-1.0 https://spdf.gsfc.nasa.gov/data_use_policy.html`). **Grenzübertritt benannt:**
  die `url`/`origin`/`compiler`-Zeilen hat Mountain auf das Operator-Wort gesetzt — Mycelium möge
  sie bestätigen/übernehmen. Der Producer `dmsp16_ssj_compiler --sat f17|f18` und die CI-Naht
  erzeugen die CDN-Bins (`dmsp17_ssj.bin`/`dmsp18_ssj.bin`).

## An river

Origin: mountain-354 (2026-10-15):

- **ICGEM-Kern ohne Membran-Berührung.** Der Schwere-Bau dieses Runds liegt ganz in
  `src/archivar/gfc.rs` + `gfc_compiler.rs`; `main_flow`/`omega` sind unberührt. Die
  Runtime-Naht (welcher Kernel die SH-Synthese trägt) ist die offene Rat-Frage, keine
  Membran-Änderung ohne dein Wort.
- `xfer:mountain-352-1` **Die vier Archivar-Clippy-Lints sind geheilt** (river-folge216 gefaltet): `too_many_arguments`
  (`extract.rs` `hapi_emit_row` → `HapiRowCtx`), `manual_range_contains`/`neg_cmp_op_on_partial_ord`
  (`tle.rs`). `cargo check` 0/0; clippy kann hier nicht laufen (Allowlist) — der CI-`clippy`-Job
  bestätigt.
- `xfer:mountain-352-2` **NSDB-Richtungszeuge in den S²-Pfad.** `nsdb::to_astro_series` liefert die AST1-Richtungszeugen
  (Fehler absent). Der `format nsdb`-Zweig in `main_flow` (Membran = River) wäre die Route, sobald
  der NSDB-Block registriert ist; alternativ emittiert der Producer einen AST1-Zeugen und der
  bestehende `astrometry_series`-Zweig trägt ihn.
- `xfer:mountain-352-3` **Aurora-Grenzbreiten** (BAS-IMAGE/DMSP-SSJ) brauchen die Feld-/Kernel-Zuordnung (Rat).

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes."** | 2026-10-14 | (mountain 344–348, 352, 353).
- **„Nächster offener Schritt: die Athabasca-Spektralform in den Arm ziehen (per-Bin, Frequenz in Hz) und die Registerzeile schreiben; dann SABER-Arm und Venus-Compiler."** | 2026-10-14 | (mountain 344).
- **„recherche roster rat"** (Athabasca-Einheit) | 2026-10-14 | (mountain 344) — Verdikt: nT²/Hz primär, V²/Hz roh.
- **Die Einheiten-Entscheidung übernommen** | 2026-10-14 | (mountain 344).
- **„was wenn du den satellit autonom betrachtest …"** | 2026-10-14 | (mountain 345 — SABER: eigene Weltlinie).
- **„warum können wir das gravitationsmodell nicht wie die anderen behandeln? … sie sind ein Feld"** | 2026-10-14 | (mountain 345 — ICGEM: Feld).
- **„fängst du bitte an und delegierst an taucher?"** | 2026-10-14 | (mountain 346).
- **„bitte fahre die frage über trying open"** | 2026-10-15 | (mountain 348).
- **„nein direkt der unterscheider mit archive search und rat"** | 2026-10-15 | (mountain 348).
- **„Starte die Mountain-Linie in einem Pass … Die Kraft-/Lizenz-Zeilen aus Myceliums sechs Aufträgen setzen …"** | 2026-10-15 | (mountain 349).
- **„Der WDC-B-Punkt ändert sich: … ~205 … die Lizenz differiert …"** | 2026-10-15 | (mountain 349).
- **Der Auftrag `briefs-2026-10-15-mountain-mycelium.md`** | 2026-10-15 | (mountain 350) — am Baum getragen (`36b50dfca`); nicht erneut ausgeführt.
- **„kannst du bitte taucher mit harten bandagen und archive search all!!! auf — SWIFF · WINDMI · MAGION · MOSWOC-Kp · OVATION-AACGM — loslassen"** | 2026-10-15 | (mountain 351).
- **„kannst du die bitte vermessen [Flare-Verifikation · REFM · Kp-Zeitreihe · OVATION-Prime-Papiere]"** | 2026-10-15 | (mountain 351).
- **„ok bitte arbeite nun bitte die offenen punkte mit tauchern ab."** | 2026-10-15 | (mountain 351).
- **„und für all die risse gibt es keine alternativen?"** | 2026-10-15 | (mountain 351).
- **„also schickst du bitte taucher los um alle offenen registrierungen zu machen?"** | 2026-10-15 | (mountain 351).
- **„aber du kannst die parser bauen oder macht das auch mycelium?"** | 2026-10-15 | (mountain 351).
- **„…heisst das ihr habt jetzt ein fabricated tor eingebaut … und mach das bitte fertig"** | 2026-10-15 | (mountain 351 — das 0.0-Pad ist die sanktionierte Draht-Form; der Ort lebt als `x,y,z`; NSDB-Lese-Arm + AST1-Absenz-Vertrag gebaut, mountain 352).
- **„naja eigentlich wie bei den maps, die aus messpunkten entstanden sind — wir wollen die gemessenen gitterpunkte, mit denen die modelle erstellt wurden, also die rohdaten."** | 2026-10-15 | (mountain 352) — für ein Modell trägt das Register die **Messung**, nicht den Modell-Output; OVATION-Mohe wird kein Feld (`dmsp16_ssj` ist die Rohdatenzeile).
- **„für die rat-architekturentscheidung muss ich nochmal gezielte fragen an die ui chats stellen?"** | 2026-10-15 | (mountain 352) — nein: die 13 Sitz-Antworten tragen den Stoff, der Rat trägt das Verdikt.
- **„die letzte session schreibt Verstanden — die Kritik sitzt, keine Ausreden."** | 2026-10-15 | (mountain 354) — die zwei Vor-Session-Fixes (J2000/TDB-Anker einmal; UCAR-FPI/MODVOLC `local-only` ohne `phi`-Zeile) sind committet/gepusht; kein erneutes Vorlegen.
- **„Der nächste Block ist der Schwere-Bau für ICGEM. Dafür messe ich zuerst den Modellkopf selbst — Erdkonstante, Radius, Normierung, Tide-System … ohne die gemessene Normierung wäre die Synthese geraten … Danach steht der Kern."** | 2026-10-15 | (mountain 354) — der XGM2019e-Kopf ist gemessen, die fully-normalized Kugelflächen-Synthese gebaut.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. Dieses Atom
(mountain-354) hat den **ICGEM-Modellkopf gemessen** (XGM2019e: GM 3.986004415E+14 m³/s²,
radius 6378136.3 m, max_degree 5540, norm fully_normalized, tide_system zero_tide) und den
**Kern gebaut** — die fully-normalized 4π-Kugelflächen-Synthese in `src/archivar/gfc.rs`
(`GfcHead`/`parse_head`/`GfcModel`/`harmonic_sums`/`synthesize`, die Rekursion gegen die
Definition bis Grad 14 geprüft; 12/12 `archivar::gfc`-Tests grün), `gfc_compiler.rs` liest den
vollen Kopf über `gfc::parse_head`. `cargo check` 0/0, `rustfmt --edition 2024 --check` exit 0,
Register canonical (2807), `--orphans` 0. Offen: die Runtime-Naht (Rat) und der CDN-Block.
Diese Fortschreibung (folge353 → `archiv/`).
