<!--
  title: Handover — Mountain-Folge 344 (2026-10-14)
  session: Mountain-Linie — ein Pass: Athabasca-Spektralform per-Bin in den CDF-Arm gezogen + Registerzeile gesetzt; Venus-Compiler-Schritt gebaut
  class: handover
  date: 2026-10-14
  sha256: f369964eeaf1f8fcbada9e48233f4f736504ef2fe4ed46ae2abda5383c537af2
  status: live
-->
# Handover — Mountain-Folge 344 (2026-10-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert (`state/standing-pass.md`), nie kopiert. Diese
Session konsumierte `handover-2026-10-14-mountain-folge343.md` (→
`docs/handover/archiv/`).

## Burn: open 0.00 · close 0.3514 · cap 0.40 Grund: ein Pass — Athabasca-Arm (per-Bin) + Kalibrierung (nT²/Hz primär, V²/Hz roh) + Registerzeilen + Venus-Compiler; CDF-Messung (UNITS-Attribut) via Taucher; volle Stimmen-Kette (Recherche + Rat + Roster: Duck/Claude/Qwen3.8-Max/Qwen3.8 2.4T, GLM pending) zur Einheiten-Entscheidung.

## In diesem Atom gebaut (git trägt es)

- **Athabasca CDF-Arm per-Bin:** `push_cdf_field` + `axis_at` in `src/archivar/cdf.rs`
  expandieren eine mehrelementige CDF-Variable zu einem Kanal je Bin; die Frequenzachse
  wird per `@` im Feld-Schlüssel benannt (`field ch1@freq_vlf …`), `freq` = Achsenwert in
  Hz, `bin_width` = lokaler Abstand aus der Achse (gemessen). Test
  `spectrum_expands_one_channel_per_bin_with_the_axis_frequency` (synthetisch, CI-laufend)
  + `extracts_athabasca_channels_when_present` (dateiabhängig).
- **Athabasca-Registerzeile + Kalibrierung:** Block `phi/sources.φ:402-411` — live, `terms
  erg-sc-rules-of-the-road`, `format cdf`, `ttl 600`, `on earth 54.60282 -113.64442 693`.
  Je Achse (NS/EW) der **primäre** kalibrierte Kanal `ch1@freq_vlf@amplitude_cal_vlf_ch1` →
  `athabasca_vlf_ns_nt2_hz` (`em nT^2/Hz`; `ch1/(interp(cal)*1e-9)²`, linear wie der
  Referenzleser) und der **rohe** `ch1@freq_vlf` → `athabasca_vlf_ns_v2_hz` (`em V^2/Hz`).
  `nT^2/Hz` + `V^2/Hz` in `units.rs` an `em` (force 0) zugelassen + `convert_to_si`
  (1e-18 bzw. Identität); Ausbreitung `1e-9` = T→nT (`NT_PER_T` in `cdf.rs`). Bins außerhalb
  der Kurven-Stützstellen werden **geflaggt** (kein Extrapolieren). Keine zusätzliche
  1/f²-Korrektur (cal faltet die ω-Abhängigkeit schon ein).
- **Venus-Radar-Compiler-Schritt:** `tools/harvest/src/bin/venus_radar_compiler.rs`
  reduziert eine 12000×20000-`Array_2D_Map`-Kachel über `pds4_img::parse_series` auf 12000
  Zeilenmittel und schreibt eine kleine Serie (`VNRD`, MAGIC+count+(t,v,channel)); `cargo
  check`/`rustfmt` 0/0. Reader + Registerblock + Split stehen noch (Punkt unten).

## Befund — Athabasca-Einheit (gemessen 2026-10-14, Taucher + `cdf_vars`)

- `ch1`/`ch2` = 1024-Bin-Spektren je Datensatz; **CDF-`UNITS`-Attribut = `V^2/Hz`**
  (aus `/tmp/opencode/ath.cdf`, Dekompression + Attribut-Sektion). `freq_vlf` = Achse
  10…19989 Hz (REAL4, 1024). `amplitude_cal_vlf_ch1` = 2000 `(freq Hz, factor)`-Paare,
  `UNITS = "Hz V/T"` (V/T-Transferkurve 2,4e5…7,0e9). `label_freq_vlf` = 5-Zeichen-Labels
  (`" 10Hz"`, `"1006H"`). **Riss:** die DOI-Landung nennt „fully-calibrated data in physical
  units" (`/tmp/opencode/ath_doi.txt:80`), das CDF-Attribut trägt `V^2/Hz` — beide Linien
  getragen, nicht geglättet. Der Block registriert den Dateiwert (V²/Hz); die
  B-Feld-Umrechnung (`/cal²`) ist offen.

## Offen (aufgeschlüsselt)

### SABER — Höhenform (C) offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Arm-Bau
- **Lage:** (gemessen 2026-10-14) Rat-Verdikt (C-1): `field ktemp.<i> …` (dotted Level im
  Schlüssel), neue Direktive `alt tpgpaltitude km` (Höhe als `Position.alt`); der HAPI-Arm
  (`extract.rs`/`fetch.rs` `Extract::Hapi`) muss `size:[400]` in dotted-Indices expandieren
  und `param.N` spaltenweise adressieren. Lizenz `saber-rules-of-the-road` (terms-URL
  `https://spdf.gsfc.nasa.gov/pub/data/timed/saber/SABER_rules_of_the_road.txt`), HAPI
  `TIMED_L2A_SABER`, DOI `10.48322/2src-g677`. `terms` im Block zulässig.
- **Blockade:** der HAPI-Arm kennt keine mehrdimensionalen Parameter (nur Skalare); die
  `alt`-Ablage fehlt (kein Slot in `FieldConfig`/`SourceConfig`).
- **Braucht:** HAPI-Arm um dotted-`<i>`-Expansion erweitern + `alt <key> <unit>`-Direktive
  (Ablage in `Extract::Hapi` oder einem eigenen Feld); danach Block
  `url https://cdaweb.gsfc.nasa.gov/hapi/data?id=TIMED_L2A_SABER&…parameters=ktemp,tpgpaltitude&format=json`,
  `terms saber-rules-of-the-road …`, `at earth`, `field ktemp.<i> …`.

### Venus Radar PDS4-IMG — Block + Reader offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Register-/Reader-Bau
- **Lage:** (gemessen 2026-10-14) Bundle `urn:nasa:pds:venus_radar_level2` v1.0, DOI
  `10.17189/1522443`, **CC0-1.0** (DataCite `rightsList`), Wurzel
  `https://pds-geosciences.wustl.edu/venus/urn-nasa-pds-venus_radar_level2/`; 15 Produkte
  (5 Epochen × `inc`/`ocp`/`scp`), je 960 000 000 B headerloses `Array_2D_Map`
  (IEEE754LSBSingle, 12000×20000, Sinusoidal 1000 m/pixel, R=6051 km). Lesearm
  `src/archivar/pds4_img.rs`; Compiler
  `tools/harvest/src/bin/venus_radar_compiler.rs` (Zeilenmittel → `VNRD`).
- **Blockade:** dem reduzierten `.bin` (VNRD) fehlt ein Reader/Arm (`format pds4_img` liest
  die rohe 960-MB-Kachel, nicht das reduzierte Asset); Blockform noch nicht geschrieben.
- **Braucht:** (1) `format`-Reader für das reduzierte `VNRD`-Asset (`extract.rs`-Dispatch +
  Arm); (2) Block — eine Quelle, ein Block, Einheit am Feld (`inc`=deg, `ocp`/`scp`=1),
  `url` = reduziertes CDN-Asset, `origin` = rohe PDS-Kachel, `compiler venus_radar_compiler`,
  `terms CC0-1.0 https://doi.org/10.17189/1522443`; (3) Split-Route über die Jahres-/Produkt-Liste
  (`cdn.rs`-2-GiB-Deckel).

### ICGEM gfc — Block ohne Epoch schreiben (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14) D2 (Rat + Roster einstimmig): statisch ohne Epoche — die
  GOCO-Epoche 2010-01-01 gehört zu GOCOs Zeitmodell, XGM2019e hat keins; kein erfundener MJD.
  `--epoch`-Override gebaut (`gfc_compiler.rs`, `a650b1327`). Riss 2: volles Modell ≈ 12,8 GB
  > 2-GiB-Release-Deckel (`cdn.rs:230`); zulässiger Cap D=3211.
- **Blockade:** die Blockform ist nicht geschrieben; die Split/Cap-Route offen.
- **Braucht:** Block `url …/icgem.gfz.de/gfc.bin`, `terms CC-BY-4.0 https://doi.org/10.5880/ICGEM.2019.007`,
  `format gfc`, `at earth`, 4 Felder, **ohne** Epoch-Zelle; Cap D=3211 für den Split.

### UCAR-FPI — Arm verdrahtet, Sample liegt, Block fehlt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `field`-Zeilen aus dem Sample
- **Lage:** (gemessen 2026-10-14) Arm + Guard (`2d4c526f4`, `src/archivar/fpi_ucar.rs`,
  Dispatch `extract.rs:159`). Sample gemessen: `data/registration.hao.ucar.edu/20246300athabasca.dat`,
  7353 Zeilen, 10 leerzeichengetrennte Spalten, Kopf `yyday ut(hr) merid(m/s) merid err dir ut(hr)
  zonal(m/s) zonal err dir skytemp`. Lizenz gemessen: keine benannte (UCAR/DASH-Nutzungssatz),
  terms-URL `https://data.ucar.edu/dataset/fpi-resolute`. **Riss:** kein Athabasca-FPI-DOI.
- **Blockade:** Block-Innenform (`url` CDN vs. raw + Compiler) offen; nicht-offene Lizenz → `local-only`.
- **Braucht:** Rat ergangen: Compiler + fetch-einmal, aber CDN durch die nicht-offene Lizenz
  blockiert → `local-only` (Arbeitskopie unter `data/`), kein `phi`-`url`/`compiler`. Offen: die
  **FPI-eigene** Athabasca-Instrumentenkoordinate messen; die zwei `field`-Zeilen
  `fpi_mer_wind_ms`/`fpi_zon_wind_ms` stehen bereit.

### harvest-Direktive — Parser + Schumann-Block (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Parser-Arm
- **Lage:** (gemessen 2026-10-14) mycelium-344-1: `harvest-dispatch.yml` reagiert auf eine
  `harvest <actor>`-Zeile; der Parser kennt die Direktive noch nicht, der Schumann-Block
  (`phi:1034-1043`) trägt sie nicht.
- **Blockade:** Parser-Arm fehlt.
- **Braucht:** `harvest <actor>` im Parser anerkennen (reine Manifest-Nennung), dann
  `harvest raw_zenodo` in den Schumann-Block.

### Fünf fehlende Fäden — terms/field/ttl (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14) mycelium-344-2, Entwurf
  `state/mycelium/block-drafts/2026-10-13-faden-schliessung.φ`: Sonnen-Magnetfeld, Stern-Aktivität,
  21 cm, Wolken, TGF. Quellen gemessen (HTTP/DOI).
- **Blockade:** keine.
- **Braucht:** `terms`/`field`/`ttl` je Block; danach registriert Mycelium `url`/`origin`/`compiler`.

### Doppel-Arme RAVE/GALAH/1LHAASO — Verdikt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-13) RAVE `:11481`, GALAH `:11781`, 1LHAASO `:11788` tragen je
  zwei Arme. D3: der messende Arm trägt; `field` bleibt am messenden Arm. Riss: GALAH hat zwei
  `tap`-Arme.
- **Blockade:** keine.
- **Braucht:** je den überzähligen Arm entfernen (RAVE `asu-tsv` behalten; GALAH-Riss: welcher
  `tap`-Arm); danach `register_sort`.

### `em mw` / `advective kg/m2/s` — Freeze (eigen)
- **Status:** eigen | **Bindung:** eigen
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

### CelesTrak — als eigene Quelle streichen (eigen + Mycelium)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14) Rat + Roster: `celestrak.org/SpaceData/EOP-All.csv` ist eine
  inhaltsgleiche Abschrift der USNO-finals. Block `phi:10144-10148`; CelesTrak-Asset 404.
- **Blockade:** keine.
- **Braucht:** `url`/`origin`/`compiler` des Blocks entfernen (Mycelium) + Mountain-Verdikt
  `descoped: inhaltsgleich mit USNO finals`; dann `register_sort`.

### finals-Leser — Interpolation + Stale-Flag + LOD (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14) der Jetzt-Wert ist auf „jetzt" zu interpolieren; nach ~2–3
  Tagen ohne `I`-Zeile ein Stale-Flag. Gebaut: Auswahl der letzten `I`-Zeile (`d26f42dc8`).
  mycelium-344-5: **LOD in den `finals`-Arm ergänzen** (`7eb835f2a` hat COMP_LOD im
  `celestrak_eop`-Arm).
- **Blockade:** keine.
- **Braucht:** Interpolation + Stale-Flag + LOD in `build_finals_channels`.

## An mycelium

Origin: mountain-344 (2026-10-14) — trägt die offenen Einträge aus mountain-343:

- `xfer:mountain-342-7` **CelesTrak streichen:** Block `phi:10144-10148` — Rat + Roster gemessen
  inhaltsgleich mit USNO finals, Asset 404. Bitte `url`/`origin`/`compiler` entfernen; das
  Quellen-Verdikt (`descoped`) setzt Mountain.

## An river

Origin: mountain-344 (2026-10-14):

- `xfer:mountain-343-1` **Athabasca:** die Registerzeilen stehen (`phi/sources.φ:402-411`); der Arm
  emittiert per-Bin, **kalibriert** (`ch1@freq_vlf@amplitude_cal_vlf_ch1` → `nT^2/Hz`) und
  **roh** (`ch1@freq_vlf` → `V^2/Hz`). Der Einheiten-Riss ist entschieden.
- `xfer:mountain-342-6` **SABER:** die Registerzeile folgt nach dem HAPI-Arm (dotted `<i>` + `alt`).

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes."** | 2026-10-14 | (mountain 344).
- **„Nächster offener Schritt: die Athabasca-Spektralform in den Arm ziehen (per-Bin, Frequenz in Hz) und die Registerzeile schreiben; dann SABER-Arm und Venus-Compiler."** | 2026-10-14 | (mountain 344).
- **„recherche roster rat"** (Athabasca-Einheit) | 2026-10-14 | (mountain 344) — Kette gefahren; Verdikt (Rat + Roster: Duck · Claude · Qwen3.8-Max-frei · Qwen3.8 2.4T; GLM `pending`): **nT²/Hz primär (Kalibrierung im Arm, linear wie der Referenzleser), V²/Hz als Rohkanal**; umgesetzt.
- **Die Einheiten-Entscheidung übernommen** (nT²/Hz primär + V²/Hz roh, Kalibrierung versioniert im Arm) | 2026-10-14 | (mountain 344 — Operator legte das Verdikt der Stimmen-Kette vor).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. In diesem Atom
(folge344) committet (HEAD == origin/main): `src/archivar/cdf.rs` · `src/archivar/units.rs` ·
`phi/sources.φ` · `tools/harvest/src/bin/venus_radar_compiler.rs` · diese Übergabe · der
`archiv/`-Move der folge343.
