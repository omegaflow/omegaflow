<!--
  title: Handover — Mountain-Folge 352 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: 66a90905039dae2afad9efaa2d43f10652d4ade17950627a23e1614327a19756
  status: live
  see-also: docs/concepts/tool-forms.md, AGENTS.md, docs/surveys/survey-2026-10-15-ketten-quellen-harter-lauf.md, docs/surveys/survey-2026-10-15-riss-alternativen.md
-->
# Handover — Mountain-Folge 352 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Diese
Session konsumierte `handover-2026-10-15-mountain-folge351.md` (→ `archiv/`).

Still gemessen: `register_lookup --fired mountain` = 0, `--stale mountain --persist 3` = 0;
`open_points_check` auf folge351: 2 ABSENT-Refs (die zwei weite-Leiter-Floskeln in
Prosa, kein Pfad-Block) · 0 stale. Gefaltet (`register_lookup --addressed mountain`):
`mycelium-folge355` und `river-folge216` (s. „Gearbeitet"). Der Stehende Pass wird als
**öffentliche** Fassung zitiert, nie kopiert.

## Burn: open 0.0 · close nicht gemessen — `session_burn` meldet `db absent`
(`~/.local/share/opencode/opencode.db` fehlt); der Platzhalter `0.0` ist **kein** Meßwert
(Vorbehalt wie mountain-348/349/351).

## Gearbeitet (dieses Rund)

- **NSDB-Vertrag geschlossen — die fehlerlose Richtungs-Serie trägt jetzt im AST1.**
  Der Operator-Wunsch „… und mach das bitte fertig" ist am letzten Meter abgearbeitet:
  `src/archivar/astrometry_series.rs` `sample_is_a_measurement` nimmt Fehler in `[0,∞)` an —
  die `0.0` in den zwei Fehler-Slots ist das sanktionierte **feste-Stride-Absenz-Maß** (ein
  Fehler ist strikt positiv, `0.0` kann kein gemessener Fehler sein → eindeutig absent),
  gelesen als `None` durch `astro_sample_sigma_rad` (der flache Winkelkern, ein bereits
  gefälltes Verdikt, kein neuer Kernel). Negativ/NaN wird weiter abgewiesen. Der Riss ist
  damit **aufgelöst** (kein Fabrikat, keine erfundenen Fehler).
- **`src/archivar/nsdb.rs::to_astro_series`** — die Beobachtung als AST1-Richtungszeuge:
  RA/Dec falten über die Schaltsekundentabelle auf die TDB-Achse, die Fehler bleiben absent.
  Zwei Tests ergänzt (Absenz-Roundtrip im AST1; die NSDB→AST1-Faltung). `cargo check` 0/0,
  `rustfmt --edition 2024 --check` exit 0.
- **Die vier Archivar-Clippy-Lints geheilt (river-folge216 gefaltet).** `hapi_emit_row`
  (`extract.rs`) trägt seine zwölf Argumente jetzt als `HapiRowCtx` (sechs Parameter);
  `tle.rs:90` nutzt `(0.0..1.0).contains(&ecco)`; `tle.rs:147/:341` schreiben den NaN-Fang
  explizit (`v.is_nan() || v <= 0.0`) statt `!(v > 0.0)`. `cargo check` 0/0.
- **Schwester-Host-`terms` gesetzt** (`xfer:mycelium-352-3`): `rjes.wdcb.ru` `terms CC-BY-4.0`,
  `usd.wdcb.ru` `terms CC-BY-NC-4.0`, `www.wdcb.ru/sep/` `terms wdcb-sep-policy
  …/policy.html`. `esdb.wdcb.ru` (keine eigene Terms-Seite) und `stp.wdcb.ru` (Apache-Wurzel,
  kein WDC-Inhalt) bleiben ohne `terms`-Zeile — benannte Absenz, kein erfundenes Token.
  Verifiziert: `register_sort phi/sources.φ` **canonical** (2 805 Blöcke) ·
  `cdn_reconcile --fail` clean (334 Hosts) · `register_lookup --orphans --fail` 0.
- **Roster-/Rat-Fragebogen erstellt** (`state/stimmen/2026-10-15-mountain-rat-fragen.md`,
  Operator-Wort 2026-10-15): die drei offenen Kraft-/Kernel-Fragen (M1 WDC-B-Messgrößen,
  M2 Aurora-Grenzbreite, M3 OVATION-AACGM) mit den **fünf Axiomen + fünf Stimmen**, der gefahrenen
  **weiten Leiter** (`bin/all_wide`, 7 Spills) und der Sitzliste. Der Operator reicht das Paket je
  Sitz ein; das Verdikt trägt der Rat.
- **Aurora-Grenzen vermessen (Taucher).** Vollständige Format-Spec für **BAS IMAGE SI12
  Boundaries** (`ramadda.nerc-bas.ac.uk`, OGL v3.0; 30 Monats-`.txt`, 50 Spalten:
  Zeit, Instrument, 24 equatorwärtige + 24 polwärtige AACGM-Breiten je 1-h-MLT-Bin, `NaN` =
  fehlend; sha256 `ceb8d697…`) und **DMSP SSJ FOM-Boundaries** Zenodo `10.5281/zenodo.3373812`
  (CC-BY-4.0; `fom_boundaries_F16_F17_F18_2010_2014.zip`, sha256 `aae69e9a…`; 16-Spalten-CSV,
  eine Zeile = ein Polar-Pass EQ1/PO1/PO2/EQ2). **Der Bau der Lese-Arme wartet auf die
  Feld-/Kernel-Zuordnung einer Grenzbreite (Rat)** — eine Grenzbreite ist ein geometrischer
  Ort, kein Kraftwert; `declared_fields` braucht `FieldConfig`/Kernel. Spec liegt, Block
  bleibt ungeschrieben.

## Offen (aufgeschlüsselt)

Gefaltet (Echo in diesem Block): `xfer:mycelium-352-1` · `xfer:mycelium-352-2` ·
`xfer:mycelium-352-3` · `xfer:mycelium-352-5` · `xfer:mountain-348-1` · `xfer:mountain-348-2` ·
`xfer:mountain-348-4` · `xfer:mountain-348-5` · `xfer:mycelium-342-3` · `xfer:mycelium-342-4` ·
`xfer:mycelium-342-10` · `xfer:mycelium-343-2` · `xfer:mycelium-352-1`/`-2` (field).

### WDC-B `field` je Datensatz — Kernel-Zuordnung ist Rat (eigen)
- **Status:** eigen (field = Mountain; Kernel = Rat) | **Bindung:** eigen
- **Trigger:** `xfer:mycelium-352-1`/`-2` (die 179 Wayback- und 7 lebenden `sep`-Blöcke).
- **Lage:** (gemessen 2026-10-15) Die Quantitäten/Einheiten der Groups sind gemessen
  (G CGS · 1-min-Geomagnetik nT · Freiluft-Schwere mGal+Geoid m · Herdparameter · aa-Index nT ·
  Pc1-Pulsationen · GNSS-RINEX). Eine `field`-Zeile braucht aber **Kernel/Force/Unit** — die
  Kernel-Zuordnung einer Quelle ist laut Haus **keine Pro-Solo-Entscheidung** (Force-/
  Kernel-Kontrakt → Rat).
- **Blockade:** Rat (Kernel-Zuordnung).
- **Braucht:** das Rat-Verdikt je Group (Kernel/Force); dann die `field`-Zeilen.

### Ketten-Quellen (SWIFF · WINDMI · MAGION · MOSWOC-Kp · OVATION-AACGM) (eigen)
- **Status:** eigen · **gemessen 2026-10-15** | **Bindung:** eigen
- **Trigger:** der harte Ketten-Quellen-Lauf (Operator-Wort 2026-10-15) — abgearbeitet.
- **Lage:** (gemessen 2026-10-15) volles Messblatt `docs/surveys/survey-2026-10-15-ketten-quellen-harter-lauf.md`.
  SWIFF/WINDMI/MAGION bleiben als **kein Datensatz** benannt. MOSWOC-Kp-Zugang entfällt
  (GFZ-Kp deckt den Bedarf). OVATION-AACGM (Zenodo `19651247`, CC-BY-4.0, eine Station Mohe)
  wartet auf die **Register-/Feld-/Kernel-/Format-Entscheidung (Rat)**.
- **Blockade:** Rat (OVATION).
- **Braucht:** das Rat-Verdikt zu OVATION-AACGM (Feld/Kernel/Format).

### Epochen-Absenz — Umsetzung trägt der Modellart-Unterscheider (eigen)
- **Status:** eigen · **verdiktet** (Operator-Wort: direkt der Unterscheider) | **Bindung:** eigen
- **Trigger:** keine — die Umsetzung trägt der Modellart-Unterscheider.
- **Lage:** (gemessen 2026-10-15) Die Bit-Brücke `PRESENCE_FLAG_EPOCH` ist verworfen; der Riss der Leselinien bleibt
  Zeuge, bis der Unterscheider ihn löst.
- **Blockade:** keine.
- **Braucht:** nichts an dieser Stelle.

### Modellart-Unterscheider — gebaut (eigen)
- **Status:** eigen · **Rat-Runde vollständig, Verdikt gebunden (Achse 1)** | **Bindung:** eigen
- **Trigger:** die ICGEM-Manifestation.
- **Lage:** (gemessen 2026-10-15) `ModelKind` + `source_model_kind` + `SourceConfig.model_kind` (`types.rs`),
  Parser-Ableitung, `phi`-Direktive `model_kind static|time_variable`; die Maschinen-Naht
  trägt `Option<ModelKind>`; `record` weist einen `Static`-Batch ab. `cargo check` 0/0.
- **Blockade:** keine.
- **Braucht:** nichts — die `model_kind`-Direktive landet mit der ICGEM-Manifestation.

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

### ICGEM gfc — Rezept + Site gemessen; wartet auf die Epochen-Ablage (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Feld-Bau.
- **Lage:** (gemessen 2026-10-15) `gfc.rs` parst den 26×f64-Draht ohne GM/radius/norm/tide; die Kugelflächen-Synthese
  fehlt; Haus-CDN-Asset `gfc.bin` = 404. DOI CC-BY-4.0 bestätigt.
- **Blockade:** die Epochen-Ablage (Modellart-Unterscheider).
- **Braucht:** (1) GM/radius/norm/tide an den Auswerter; (2) die Kugelflächen-Synthese; (3) den Block.

### UCAR-FPI — Arm + Zeilen stehen; Manifestation wartet auf die `field`-Zeilen (eigen)
- **Status:** eigen (Rest Mycelium-/River-gebunden) | **Bindung:** eigen
- **Trigger:** die `field`-Zeilen aus dem Sample.
- **Lage:** (gemessen 2026-10-15) Arm + Guard stehen (`fpi_ucar.rs`); die zwei `field`-Zeilen `fpi_mer_wind_ms`/
  `fpi_zon_wind_ms` sind code-gemessen; Athabasca-Koordinate `on earth 54.60282 -113.64442 693`.
- **Blockade:** nicht-offene Lizenz → `local-only` (kein CDN).
- **Braucht:** den UCAR-FPI-Block + die zwei `field`-Zeilen + die Arbeitskopie-Route.

### Fünf fehlende Fäden — Entwurf steht, Mycelium setzt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom.
- **Lage:** (gemessen 2026-10-14) Mountain-Entwurf `state/mountain/faden-term-field-ttl-draft.φ`; zwei Risse offen
  (WSO-Polarfeld ohne Terms; LISIRD ohne Terms). `xfer:mountain-348-2`.
- **Blockade:** keine.
- **Braucht:** Mycelium setzt `url`/`origin`/`compiler`; dann `register_sort`.

### Taucherstaffel 347 — überführt (descoped)
- **Status:** descoped (überführt) | **Bindung:** eigen
- **Trigger:** keiner.
- **Lage:** (gemessen 2026-10-15) auf „Venus Radar", „SABER" und „Fünf fehlende Fäden" übergegangen; kein eigener
  offener Gegenstand.
- **Blockade:** keine.
- **Braucht:** nichts an dieser Stelle.

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
- **Braucht:** (1) OCNS2019 als `local-only` stehen lassen; (2) die MODVOLC-Forderung auf
  „HIGP `local-only`, kein Block; LANCE/NRT bleibt" korrigieren; (3) ein echter offener
  Beobachtungs-Ersatz (Lead NSDB `10.1051/0004-6361/200911645`).

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
  vermessen (s. „Gearbeitet"); der Bau wartet auf die **Feld-/Kernel-Zuordnung (Rat)**.
- **Blockade:** Rat (Aurora-Grenzbreite als Feld/Kernel); die vier Registrierungen sind
  Mycelium (`xfer:mountain-351-1…-5`).
- **Braucht:** (1) das Rat-Verdikt zur Feld-/Kernel-Zuordnung einer Grenzbreite; (2) die vier
  Mycelium-Registrierungen; (3) die NSDB-Witness-Route in den S²-Pfad (main_flow = River) —
  `nsdb::to_astro_series` liefert die AST1-Richtungszeugen.

## An mycelium

Origin: mountain-352 (2026-10-15):

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

## An river

Origin: mountain-352 (2026-10-15):

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

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes."** | 2026-10-14 | (mountain 344–348, 352).
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

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. Dieses Atom
(mountain-352) hat den NSDB-Vertrag geschlossen (AST1 trägt die fehlerlose Richtungs-Serie;
`nsdb::to_astro_series`; Tests), die vier Archivar-Clippy-Lints geheilt (river-folge216),
die Schwester-Host-`terms` gesetzt (WDC-B, `xfer:mycelium-352-3`) und die Aurora-Grenz-Spec
per Taucher vermessen (BAS-IMAGE/DMSP-SSJ; der Bau wartet auf die Feld-/Kernel-Zuordnung,
Rat). Die adressierten Blöcke `mycelium-folge355` und `river-folge216` sind gefaltet.
Diese Fortschreibung (folge351 → `archiv/`).
