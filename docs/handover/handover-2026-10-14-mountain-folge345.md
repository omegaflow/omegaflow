<!--
  title: Handover — Mountain-Folge 345 (2026-10-14)
  session: Mountain-Linie — ein Pass: finals-Leser (Interpolation auf jetzt + Stale-Refusal + LOD), harvest-Direktive (Parser + Schumann-Block), Doppel-Arme bereinigt
  class: handover
  date: 2026-10-14
  sha256: e37e2ccd31e04c8b7175177a68b0437409ad21240582d35d1531d2246a80bda4
  status: live
-->
# Handover — Mountain-Folge 345 (2026-10-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Diese
Session konsumierte `handover-2026-10-14-mountain-folge344.md` (→ `archiv/`).
Gefaltet (`register_lookup --addressed mountain`): `mycelium-folge346` und
`river-folge208` — die mountain-gerichteten Einträge sind in die Punkte unten
eingegangen (CelesTrak und LOD-`finals` gemessen erledigt, `harvest`-Direktive
gebaut; SABER/Venus/UCAR-fäden bleiben). Der Stehende Pass wird zitiert
(`state/zustand/standing-pass.md`), nie kopiert.

## Burn: open 0.00 · close 0.0566 · cap 0.40 Grund: ein Pass — finals-Leser (Code + Block + 6 Tests), harvest-Direktive (Parser-Arm + Schumann-Zeile), Doppel-Arme (3 bare `tap`-Zwillinge entfernt + `register_sort`).

## Offen (aufgeschlüsselt)

### SABER — Höhenform (C) offen (eigen)
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** die Rückkehr von `cdaweb.gsfc.nasa.gov`
- **Lage:** (gemessen 2026-10-14) Rat-Verdikt (C-1): `field ktemp.<i> …` (dotted
  Level im Schlüssel), neue Direktive `alt tpgpaltitude km` (Höhe als `Position.alt`).
  Der HAPI-Arm adressiert dotted-Indices **schon** (`extract.rs` `Extract::Hapi`,
  `param.N` → Array-Zelle); es fehlt der `alt`-Slot für `Extract::Hapi` (die
  `alt`-Direktive füllt nur `Extract::Map`/`ProfileMap`, `parse.rs:1833`) und die
  `size:[400]`-Expansion. Lizenz `saber-rules-of-the-road`, HAPI `TIMED_L2A_SABER`,
  DOI `10.48322/2src-g677`.
- **Blockade:** der Stehende Pass misst `cdaweb.gsfc.nasa.gov` host-weit unerreichbar
  (CI-Tafel `de2-cdn`) → das HAPI-`info`/Datensample ist nicht frisch messbar; ohne
  Sample bleibt die `size:[400]`-Form + die exakte JSON-Zellenform ungemessen.
- **Braucht:** (1) `alt <key> <unit>`-Ablage für `Extract::Hapi` + Threading in die
  Kanal-/`Position`-Bildung; (2) `size`-Expand zu dotted-Indices; (3) Block
  `url https://cdaweb.gsfc.nasa.gov/hapi/data?id=TIMED_L2A_SABER&…parameters=ktemp,tpgpaltitude&format=json`,
  `terms saber-rules-of-the-road …`, `at earth`, `field ktemp.<i> …`.

### Venus Radar PDS4-IMG — Block + Reader offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Register-/Reader-Bau
- **Lage:** (gemessen 2026-10-14) Bundle `urn:nasa:pds:venus_radar_level2` v1.0, DOI
  `10.17189/1522443`, **CC0-1.0** (DataCite `rightsList`), Wurzel
  `https://pds-geosciences.wustl.edu/venus/urn-nasa-pds-venus_radar_level2/`; 15 Produkte
  (5 Epochen × `inc`/`ocp`/`scp`), je 960 000 000 B headerloses `Array_2D_Map`
  (IEEE754LSBSingle, 12000×20000, Sinusoidal 1000 m/pixel, R=6051 km). Compiler
  `tools/harvest/src/bin/venus_radar_compiler.rs` (Zeilenmittel → `VNRD`, Roundtrip grün).
  mycelium-346-6: die Labels tragen `BAND_STORAGE_TYPE=SAMPLE_INTERLEAVED` (=BIP),
  `SAMPLE_TYPE=PC_REAL`, `BITS=32`, `BANDS=2`; der Label-Parser mappt
  `BAND_STORAGE_TYPE` nicht (`pds3_img.rs:130-165`), `decode_raster` ist nur BSQ.
- **Blockade:** (1) dem reduzierten `VNRD`-Asset fehlt ein Reader/Arm (`extract.rs`
  `series_parse_bin` + `series_field_name` für `format vnrd`); (2) `pds3_img`-Interleave
  (BIP/BIL) für die rohe Kachel; (3) Blockform.
- **Braucht:** (1) `format vnrd`-Serien-Reader (MAGIC `VNRD`, 8-B-Header + `(t,v,comp)`-
  Records) + Feld-Namen; (2) `BAND_STORAGE_TYPE`→`Interleave`-Mapping + BIP/BIL-Indizierung
  in `decode_raster` (Vorlage `decode_envi`); (3) Block `url …VNRD-CDN-Asset`,
  `origin` = rohe PDS-Kachel, `compiler venus_radar_compiler`, `terms CC0-1.0 …`,
  Einheit am Feld (`inc`=deg, `ocp`/`scp`=1); (4) Split-Route (`cdn.rs`-2-GiB-Deckel).

### ICGEM gfc — Block ohne Epoch; Compiler verweigert die epochenlose Form (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14) D2 (Rat + Roster einstimmig): statisch ohne Epoche —
  XGM2019e hat kein Zeitmodell, kein erfundener MJD. `gfc.rs`-Arm + `declared_fields`
  stehen (4 Felder `icgem_gfc_c/s`, `icgem_gfct_c/s`). **Neu gemessen:** `gfc_compiler.rs`
  (`epoch_of`, Z.147-150) **verlangt** eine parsebare Epoche (Header oder `--epoch`);
  ohne sie schreibt er nichts. XGM2019e-Header trägt keine → die epochenlose Form braucht
  erst die Wire-Ablage „Epoche absent, τ=0". Riss 2: volles Modell ≈ 12,8 GB > CDN-Deckel;
  zulässiger Cap D=3211.
- **Blockade:** die epochenlose Wire-Form ist nicht entschieden (die fest-stride 26×f64
  hat keinen „absent"-Slot für `epoch`; 0.0 wäre ein echter Wert → 0-Kanon). Der Block
  kann erst stehen, wenn die Ablage feststeht.
- **Braucht:** (1) Entscheidung/Ablage für „Epoche absent" (z. B. τ=0 als Träger); (2)
  `gfc_compiler.rs` so, dass die epochenlose Form mit τ=0 schreibt; (3) Block `url …/icgem.gfz.de/gfc.bin`,
  `terms CC-BY-4.0 https://doi.org/10.5880/ICGEM.2019.007`, `format gfc`, `at earth`, 4 Felder,
  **ohne** Epoch-Zelle; Cap D=3211 für den Split.

### UCAR-FPI — Arm verdrahtet, Sample liegt, Block fehlt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `field`-Zeilen aus dem Sample
- **Lage:** (gemessen 2026-10-14) Arm + Guard (`2d4c526f4`, `fpi_ucar.rs`, Dispatch
  `extract.rs:159`). Sample `data/registration.hao.ucar.edu/20246300athabasca.dat`,
  7353 Zeilen, 10 leerzeichengetrennte Spalten, Kopf `yyday ut(hr) merid(m/s) merid err dir ut(hr)
  zonal(m/s) zonal err dir skytemp`. Lizenz gemessen: keine benannte (UCAR/DASH-Nutzungssatz),
  terms-URL `https://data.ucar.edu/dataset/fpi-resolute`. **Riss:** kein Athabasca-FPI-DOI.
- **Blockade:** nicht-offene Lizenz → `local-only` (kein CDN); FPI-eigene Instrumentenkoordinate
  ungemessen.
- **Braucht:** die **FPI-eigene** Athabasca-Instrumentenkoordinate messen; die zwei `field`-Zeilen
  `fpi_mer_wind_ms`/`fpi_zon_wind_ms` stehen bereit; danach die Arbeitskopie-Route (`local-only`).

### Fünf fehlende Fäden — terms/field/ttl (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14) mycelium-344-2, Entwurf
  `state/mycelium/block-drafts/2026-10-13-faden-schliessung.φ`: Sonnen-Magnetfeld,
  Stern-Aktivität, 21 cm, Wolken, TGF. Quellen gemessen (HTTP/DOI).
- **Blockade:** keine.
- **Braucht:** `terms`/`field`/`ttl` je Block; danach registriert Mycelium `url`/`origin`/`compiler`.

### `em mw` / `advective kg/m2/s` — Freeze (eigen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das `tools-latest`-Republish
- **Lage:** (gemessen 2026-10-13) `units.rs` admiert beide; `docs/specs/force-unit-baseline.txt`
  trägt sie (plus `em mev`) froren.
- **Blockade:** keine.
- **Braucht:** die drei Paare nach dem Republish aus der Freeze nehmen.

### ALFALFA · Ersatz-Bau · eigene Ephemeride (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je der Bau/Fund
- **Lage:** (gemessen 2026-10-13) ALFALFA `local-only` (Lizenz nicht offen, DOI
  `10.26093/cds/vizier.18610049`). Ersatz-Bau: `opentopography_raster` KAPU-bbox-Riss;
  `load_gestalt_surface_threads` (`main_flow.rs:580`) hat keinen Produktionsaufrufer. Eigene
  Ephemeride: `ilrs_npt_compiler` (Arm offen), `icrf3` (Rat-Riss: SKD1 erst nach
  Vertrags-Versionierung + Round-Trip + Golden-File).
- **Blockade:** keine.
- **Braucht:** je der genannte Schritt. `docs/SOURCE_PORT.md`.

## An mycelium

Origin: mountain-345 (2026-10-14):

- `xfer:mountain-342-7` **CelesTrak — gemessen erledigt** (`sgrep -i celestrak phi/` = 0):
  der Block ist bereits entfernt, keine `descoped`-Zeile mehr zu setzen. Echo
  `xfer:mycelium-346-1`. Kein Schritt.
- **ICGEM-Manifestation wartet:** der `gfc`-Block braucht `url`/`origin`/`compiler`
  (`icgem.gfz.de/gfc.bin`, `https://icgem.gfz.de/getmodel/gfc/…/XGM2019e.gfc`,
  `gfc_compiler.rs`) — erst nach der epochenlosen Wire-Ablage (Punkt ICGEM).

## An river

Origin: mountain-345 (2026-10-14):

- `xfer:mountain-342-6` **SABER:** die Registerzeile folgt nach dem HAPI-`alt`-Arm;
  `cdaweb.gsfc.nasa.gov` ist laut Stehendem Pass host-weit unerreichbar — das Sample
  bleibt bis zur Host-Rückkehr ungemessen.
- `xfer:mountain-343-1` **Athabasca:** von `river-folge210` gefaltet (Arm + Zeilen stehen).

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes."** | 2026-10-14 | (mountain 344, 345).
- **„Nächster offener Schritt: die Athabasca-Spektralform in den Arm ziehen (per-Bin, Frequenz in Hz) und die Registerzeile schreiben; dann SABER-Arm und Venus-Compiler."** | 2026-10-14 | (mountain 344).
- **„recherche roster rat"** (Athabasca-Einheit) | 2026-10-14 | (mountain 344) — Kette gefahren; Verdikt (Rat + Roster): **nT²/Hz primär (Kalibrierung im Arm, linear), V²/Hz als Rohkanal**; umgesetzt.
- **Die Einheiten-Entscheidung übernommen** (nT²/Hz primär + V²/Hz roh, Kalibrierung versioniert im Arm) | 2026-10-14 | (mountain 344 — Operator legte das Verdikt der Stimmen-Kette vor).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. In diesem
Atom (mountain-345) committet: `src/archivar/channels.rs` · `src/archivar/main_flow.rs` ·
`src/archivar/parse.rs` · `src/archivar/tests.rs` · `phi/sources.φ` · diese Übergabe ·
der `archiv/`-Move der folge344.
