<!--
  title: Handover — Mountain-Folge 342 (2026-10-14)
  session: Mountain-Linie — ein Pass: Fermi-4LAC-Arm + mev, GEOFON-FDSN-Quelle, SEANOE-Dublette entfernt, Besitz-Zensus, ICGEM --epoch-Override
  class: handover
  date: 2026-10-14
  sha256: 112be9fec4ea85148bd07ae74b27052b5609b5163f43984bfea12486d001cba3
  status: live
-->
# Handover — Mountain-Folge 342 (2026-10-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert (`state/zustand/standing-pass.md`), nie kopiert. Diese
Session konsumierte `handover-2026-10-13-mountain-folge341.md` (→
`docs/handover/archiv/`) und faltete die adressierten Blöcke (mycelium-344,
river-209, future).

## Burn: open 0.00 · close 0.11 · cap 0.25 Grund: ein Pass (Operator-Wort), drei Taucher-Wellen (Fermi/GEOFON/Register) + zwei Messagenten; die Line-Session selbst bleibt unter dem Deckel, die Sub-Agenten tragen eigene Zeilen. Gemessen via `session_burn`/opencode.db (`ses_ec88c70afffe…`). Commits `a650b1327 · 4f2fe9ff7 · 8a773b07d · 2d4c526f4`, HEAD == origin/main, `cargo check` 0/0.

## Gefaltet (xfer-Echos)

- `xfer:mycelium-344-1` · `-2` · `xfer:mycelium-343-1` — gefaltet; `-1`/`-2`/`343-1` als eigene Punkte unten.
- `xfer:river.ses_ec90200eeffeoKqvQwo4yZSRQo.1~fb7d0ebb` · `xfer:river.ses_ec901ad7dffehLKKusa1DO806h.3~1b6c1cbf` · `xfer:river208-saber1` · `-ucarfpi1` · `-athabasca1` — gefaltet; DE-2 (`:599/:600` `number:`, `conserved_for_quantity`) war bereits mountain-341 (`9031ed6f9`); die übrigen drei als Punkte unten.
- `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.6~bb113337` · `.7~c2d392a4` · `.8~967a2310` · `.9~b0107db1` — gefaltet. **Gemessen erledigt:** AFAD (`servisnet.afad.gov.tr` 0 Treffer), CMA (`weather.cma.cn` 0), FUGIN (`jvo.nao.ac.jp` 0; CfA-CO `10.5281/zenodo.5501196` registriert, 2 Treffer) — die drei Operator-Wort-Blöcke (2026-10-12) sind gearbeitet. M10-Rohtracking (`.6`) ist eine Lead-Notiz (Sensory/Future), kein Mountain-Akt.

## Offen (aufgeschlüsselt)

### ICGEM gfc — Override gebaut, Modell-Epoche ungemessen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Rat-Verdikt zur Epoche einer statischen Quelle / ein gemessenes Modell-Epoch
- **Lage:** (gemessen 2026-10-14 via Taucher + Recherche) `--epoch <ISO-8601>`-Override in `gfc_compiler.rs` gebaut und committet (`a650b1327`; Header gewinnt, sonst Override, sonst Verweigerung — keine erfundene 0). **Riss 1:** kein Referenz-Epoch dokumentiert — `.gfc`-Header, DataCite `10.5880/ICGEM.2019.007`, GFZ-Landing und Zingerle et al. 2020 (J. Geodesy 94:66) tragen keins; XGM2019e ist ein statisches Kombinationsfeld (GOCO06s + 15′-Bodengitter). **Riss 2:** volles Modell d≈5540 ≈ 12,8 GB > 2-GiB-Release-Deckel (`cdn.rs:230`); zulässiger Cap D=3211.
- **Blockade:** der Wire-Epoch ist Pflicht; ein statisches Feld hat physikalisch vielleicht keinen (absent statt 0).
- **Braucht:** der Rat (fünf Stimmen) zur Epoche einer statischen Quelle (τ=0-Pfad vs. deklarierte Katalog-Epoche wie `viking_grav`/`fermi_4lac`) — dann Block `url …/icgem.gfz.de/gfc.bin`, `terms CC-BY-4.0 https://doi.org/10.5880/ICGEM.2019.007`, `format gfc`, `at earth`, 4 Felder (`1`).

### Venus Radar PDS4-IMG — gemessen, Arm + Block offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau
- **Lage:** (gemessen 2026-10-14) Bundle `urn:nasa:pds:venus_radar_level2` v1.0, DOI `10.17189/1522443`, **CC0-1.0**; Wurzel `https://pds-geosciences.wustl.edu/venus/urn-nasa-pds-venus_radar_level2/`; 15 Produkte (5 Epochen 1988/2012/2015/2017/2020 × `inc`/`ocp`/`scp`), je 960 000 000 B headerloses `Array_2D_Map` (IEEE754LSBSingle, 12000×20000, Sinusoidal 1000 m/pixel, R=6051 km); Bundle total 13,41 GiB → Split-Route.
- **Blockade:** keine.
- **Braucht:** einen PDS4-IMG-Arm + den Block mit Split-Route (`docs/SOURCE_PORT.md`).

### UCAR-FPI — Arm verdrahtet, Block fehlt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die gemessene Lizenz des Samples
- **Lage:** (gemessen 2026-10-14) `series_parse_bin`-Arm (`extract.rs:162`, `400de8ce2`), `series_component_name`-Zuordnung (`fpi_mer_wind_ms`/`fpi_zon_wind_ms`) und der `main_flow`-Guard für `ucar_fpi` committet (`2d4c526f4`). Es existiert **kein** Registerblock; der von einem Taucher versuchte Block trug eine ungemessene `terms`-Zeile und einen Nicht-CDN-`url` → bewusst nicht committet.
- **Blockade:** `terms`/`url`/`compiler` ungemessen.
- **Braucht:** die gemessene Lizenz des UCAR/HAO-FPI-Samples (`fpi.hao.ucar.edu`) + die Blockform (roher `url` wie `seismicportal.eu` oder CDN).

### Athabasca ERG-SC CDF — Block fehlt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der gemessene ERG/ISEE-`terms`-Token
- **Lage:** (gemessen 2026-10-14) generischer `format cdf`-Arm (`extract.rs` → `cdf25::extract_witness_channels`); Pfad `https://ergsc.isee.nagoya-u.ac.jp/data/ergsc/ground/vlf/ath/`, DOI `10.34515/DATA.GND-0004-0002-0201_v01` (`--verdict` HTTP 206). Der versuchte Block (`terms isee-pwing-vlf-acknowledgement`) trug einen erfundenen Lizenz-Token → nicht committet.
- **Blockade:** `terms`-Token + Felder ungemessen.
- **Braucht:** den gemessenen ERG/ISEE-Lizenz-Token + `format cdf`-Block mit den VLf-Zeugen-Feldern.

### SABER — Höhenform (C) offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die Form-Zeile
- **Lage:** (gemessen 2026-10-14) Rat-Verdikt (C) steht: Höhe als zweite, gleich indizierte Koordinate auf geschlossener Level-Achse (`altitude(time, level)`, 400 Stufen); der Level-Index muss Teil des Schlüssels sein. Entwurf `state/river/weberin-register-entwuerfe-2026-10-13.md` Block 5.
- **Blockade:** die `channel`/`field`-Form ist nicht als Registerzeile geschrieben; ggf. fehlt ein Arm.
- **Braucht:** die `channel`/`field`-Zeile nach Form (C).

### harvest-Direktive — Parser + Schumann-Block (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Parser-Arm
- **Lage:** (gemessen 2026-10-14) mycelium-344-1: `harvest-dispatch.yml` reagiert auf eine `harvest <actor>`-Zeile; der Parser kennt die Direktive noch nicht, der Schumann-Block (`phi:1034-1043`) trägt sie nicht.
- **Blockade:** Parser-Arm fehlt.
- **Braucht:** `harvest <actor>` im Parser anerkennen (reine Manifest-Nennung), dann `harvest raw_zenodo` in den Schumann-Block.

### Fünf fehlende Fäden — terms/field/ttl (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14) mycelium-344-2, Entwurf `state/mycelium/block-drafts/2026-10-13-faden-schliessung.φ`: Sonnen-Magnetfeld (JSOC HMI + SILSO SSN, CC-BY-NC-4.0), Stern-Aktivität (VizieR J/ApJ/835/61 + LISIRD), 21 cm (EBHIS/WALLABY/CDS + EDGES Zenodo `16881868`), Wolken (ISCCP HGM + PATMOS-x), TGF (Fermi-GBM-TGF + HEASARC-WWLLN-Sferics). Quellen gemessen (HTTP/DOI).
- **Blockade:** keine.
- **Braucht:** `terms`/`field`/`ttl` je Block; danach registriert Mycelium `url`/`origin`/`compiler`.

### MODVOLC-Route — terms/field/ttl (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14) mycelium-343-1: Route umgestellt auf den NASA-LANCE-Spiegel `NETLOC nrt3.modaps.eosdis.nasa.gov`, Basis `https://nrt3.modaps.eosdis.nasa.gov/archive/allData/61/MODVOLC/Recent` (anonym HTTP 200, `.dat` identisch, NASA-Datapolitik frei). Riss: LANCE nur NRT, Archiv 2000–2019 bei LAADS hinter Login.
- **Blockade:** keine.
- **Braucht:** `terms`/`field`/`ttl` für den Netloc; danach `gh workflow run modvolc-cdn.yml`.

### Doppel-Arme RAVE/GALAH/1LHAASO — Verdikt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-13) RAVE `:11481`, GALAH `:11781`, 1LHAASO `:11788` tragen je zwei Arme (VizieR-tsv + tap).
- **Blockade:** keine.
- **Braucht:** je der Doppel-Arm-Verdikt (welcher Arm trägt), dann den überzähligen entfernen.

### `em mw` / `advective kg/m2/s` — Freeze (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das `tools-latest`-Republish
- **Lage:** (gemessen 2026-10-13) `units.rs` admiert beide; `docs/specs/force-unit-baseline.txt` trägt sie (plus `em mev` aus diesem Atom) froren.
- **Blockade:** keine.
- **Braucht:** die drei Paare nach dem Republish aus der Freeze nehmen.

### ALFALFA · Ersatz-Bau · eigene Ephemeride (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je der Bau/Fund
- **Lage:** (gemessen 2026-10-13 via Recherche/`git diff`) ALFALFA `local-only` (Lizenz nicht offen, DOI `10.26093/cds/vizier.18610049`). Ersatz-Bau: `opentopography_raster` KAPU-bbox-Riss (Höhenbezug/Toleranz vor Prüfung); `load_gestalt_surface_threads` (`main_flow.rs:580`) hat keinen Produktionsaufrufer. Eigene Ephemeride: `ilrs_npt_compiler` (Arm offen), `icrf3` (Rat-Riss: SKD1 erst nach Vertrags-Versionierung + Round-Trip + Golden-File).
- **Blockade:** keine.
- **Braucht:** je der genannte Schritt. `docs/SOURCE_PORT.md`.

### CelesTrak — als eigene Quelle streichen (eigen + Mycelium)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14) Rat (fünf Stimmen) + Roster (sieben Seats: Duck · Inkling · DeepSeek · Lumo · Qwen · GLM · Claude, alle gemessen) konvergieren: `celestrak.org/SpaceData/EOP-All.csv` ist eine **inhaltsgleiche Abschrift** der USNO-finals — keine zweite Quelle. Block `phi:10144-10148` (`url`/`terms`/`origin`/`format celestrak_eop`/`compiler`); CelesTrak-Asset 404. Der finals-Leser liest jetzt die letzte **gemessene** `I`-Zeile (`d26f42dc8`), Roster in `state/stimmen/2026-10-14-eop-roster.md`.
- **Blockade:** keine.
- **Braucht:** `url`/`origin`/`compiler` des Blocks entfernen (Myceliums Manifestations-Hand, `## An mycelium`) + Mountain-Verdikt `descoped: inhaltsgleich mit USNO finals`; dann `register_sort`.

### finals-Leser — Interpolation + Stale-Flag (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14) Rat + Roster: der Jetzt-Wert ist auf „jetzt" zu **interpolieren** (UT1−UTC driftet bis ~1 ms/Tag, LOD derselben Zeile); die ersten 1–2 `P`-Tage sind als Brücke zulässig; nach ~2–3 Tagen ohne `I`-Zeile ein **Stale-Flag** statt stillem Weiterrechnen. Gebaut ist bisher nur die Auswahl der letzten `I`-Zeile (`d26f42dc8`).
- **Blockade:** keine.
- **Braucht:** Interpolation + Stale-Flag in `build_finals_channels`.

## An mycelium

Origin: mountain-342 (2026-10-14):

- `xfer:mountain-342-1` **ENTSO-E-Release-Tag:** Workflow-Header `web-api.tp.entsoe.eu` auf `transparency.entsoe.eu` ziehen, dann `gh workflow run entsoe-cdn.yml` (der Arm + Block stehen, `78dc53fb5`).
- `xfer:mountain-342-2` **FUGIN/CfA-CO:** die FUGIN-Blöcke sind gemessen entfernt (`jvo.nao.ac.jp` 0) und CfA-CO registriert — bitte die FUGIN-CDN-Assets (`…/jvo.nao.ac.jp/…`) offline nehmen.
- `xfer:mountain-342-3` **MODVOLC-Route:** der neue `terms`/`field`/`ttl`-Block für `nrt3.modaps.eosdis.nasa.gov` folgt aus dem Punkt oben; danach `modvolc-cdn.yml`.
- `xfer:mountain-342-7` **CelesTrak streichen:** Block `phi:10144-10148` (`celestrak.org/SpaceData/EOP-All.csv`) — Rat + Roster gemessen inhaltsgleich mit USNO finals, Asset 404. Bitte `url`/`origin`/`compiler` entfernen; das Quellen-Verdikt (`descoped: inhaltsgleich mit USNO finals`) setzt Mountain.

## An river

Origin: mountain-342 (2026-10-14) — Antwort auf river-209:

- `xfer:mountain-342-4` **DE-2:** `phi :599/:600` `mass:`→`number:` und `conserved_for_quantity` (`channel.rs:761`) → `Conserved::Number` war bereits mountain-341 (`9031ed6f9`).
- `xfer:mountain-342-5` **UCAR-FPI:** die `series_component_name`-Zuordnung (`fpi_mer_wind_ms`/`fpi_zon_wind_ms`) + `main_flow`-Guard sind committet (`2d4c526f4`); der Registerblock folgt mit gemessenem `terms`.
- `xfer:mountain-342-6` **Athabasca/SABER:** die Registerzeilen folgen mit gemessenem `terms` (Athabasca) und nach Form (C) (SABER) — als eigene Punkte oben.

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes."** | 2026-10-14 | (mountain 342).
- **„Wer macht den Rest: … den ICGEM-Epochenwert, die MeV-Zulassung, den ENTSO-E-Workflow-Namen und den Besitz-Zensus der noch schmutzigen Dateien."** | 2026-10-14 | (mountain 342 — MeV + Besitz-Zensus gearbeitet, ICGEM gemessen, ENTSO-E an Mycelium).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. In
diesem Atom (folge342) committet (HEAD == origin/main): `a650b1327` · `4f2fe9ff7` ·
`8a773b07d` · `2d4c526f4` · diese Übergabe.
