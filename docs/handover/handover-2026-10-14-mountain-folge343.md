<!--
  title: Handover — Mountain-Folge 343 (2026-10-14)
  session: Mountain-Linie — ein Pass: warum Zeugen keine Lizenzen tragen (gemessen), `ingest-terms-absent`-Gate gebaut
  class: handover
  date: 2026-10-14
  sha256: 251e7eecb4a548c0975d62b6814906232ad57aeff4023e765a3763906b9c1406
  status: live
-->
# Handover — Mountain-Folge 343 (2026-10-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert (`state/zustand/standing-pass.md`), nie kopiert. Diese
Session konsumierte `handover-2026-10-14-mountain-folge342.md` (→
`docs/handover/archiv/`) und faltete die adressierten Blöcke (future-folge253,
mycelium-folge345).

## Burn: open 0.00 · close 0.10

Die Messzahl aus `session_burn` (opencode.db, Session „Warum Zeugen-Blöcke keine
Lizenzen tragen", deepseek-flash) wächst bis Session-Schluss; Default-Cap 0.15.

## Befund — warum die Zeugen keine Lizenzen tragen (gemessen 2026-10-14)

Die Zeugen-Klasse trägt keine `terms`-Zeile, weil **keine Stelle sie je verlangte** —
nicht, weil die Form sie verböte (Operator-Frage dieser Session: „schau dir direkt an
warum die zeugen keine lizenzen tragen"). Gemessen am Baum:

- **Der Parser nimmt `terms` in jedem Block** (`parse.rs:267`); `format cdf` und
  `format ucar_fpi` sind freie Strings (`parse.rs:2192`). Die im Vorpass vermuteten
  Form-Hindernisse („kein `format cdf` im Register", „Witness-Blöcke tragen kein
  `terms`") sind **keine strukturellen Hindernisse**.
- **Der Commit-Gate verlangte nur zweierlei:** dass ein *vorhandenes* `terms`-Token
  zugelassen ist (`check_phi_terms_admitted`, `:1331`) und dass ein neu gespiegelter
  CDN-`url` eine Basis (`origin`/`terms`) trägt (`unbacked_mirror`, `:2374`, nur für
  `github.com/omegaflow/sources/releases/download/`).
- **Zensus (gemessen):** 28 Zeugen-Blöcke; **27 ohne `terms` und ohne `origin`**; die
  eine Ausnahme ist der IceCube-Zeuge (`dataverse.harvard.edu`-Spiegel), dessen CDN-`url`
  die `unbacked_mirror`-Regel auslöste. Reine Referenz-Zeugen
  (`witness`/`record`/`force`/`no-cadence`, kein `format`) bewegen keine Daten und
  tragen darum keine Lizenzpflicht.

**Daraus die Schließung:** die drei fehlenden Lizenzen (Athabasca, UCAR-FPI, SABER) sind
**kein Form-Hindernis** — die Blöcke dürfen `terms` tragen. Offen bleibt allein die
gemessene Block-Innenform (Felder/`url`/Split), nicht eine Register-Formfrage.

## Gate geschlossen — `ingest-terms-absent` (2026-10-14)

Ein **neu angelegter einlesender Block** in `phi/sources.φ` (trägt `format`/`field`/`map`/
`fold`/`hapi`) muss eine `terms`-Zeile tragen; ein reiner Referenz-Zeuge bleibt frei.
Umsetzung: `ingest_terms_absent_violations` (`src/gate/commit_gate.rs:2418`), verdrahtet
in `tools/gate/src/bin/commit_check.rs`; Fixture `phi-terms-absent`
(`src/gate/commit_gate_vocab.json`), Test `fp_ingest_terms_absent_requires_a_licence`
(`commit_gate.rs:5073`). `cargo build -p omegaflow-gate --bin commit_check` grün,
`rustfmt --edition 2024 --check` 0/0.

## Gefaltet (xfer-Echos)

- `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.3~98d34b2c` · `.4~aab2512e` · `.5~ef59826c`
  — gefaltet (CDDIS-`blocked account` **widerlegt**: `EARTHDATA_EDL_TOKEN` gesetzt,
  CDDIS HTTP 200; Mariner 6/7-Roh-Doppler + M4-ODG geerntet, M10 als Bild-EDR). Reine
  Informations-Korrekturen; kein neuer Mountain-Bau.
- `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.6~bb113337` · `.7~c2d392a4` · `.8~967a2310`
  · `.9~b0107db1` — waren bereits mountain-342 gefaltet; **gemessen erledigt** (AFAD 0,
  CMA 0, FUGIN 0, CfA-CO registriert).
- `xfer:future251-ucarfpi1` — das UCAR-FPI-Sample liegt (7353 Zeilen, sha `34c08b39…`,
  Arbeitskopie `data/registration.hao.ucar.edu/20246300athabasca.dat`); Fluss in den
  UCAR-FPI-Punkt unten.
- `xfer:mycelium.ses_ec82df1f6ffePGUvWpFtFUGl3u.1~4e3bd26a` (ENTSO-E-Tag korrigiert;
  Echo `xfer:mountain-342-1`), `.2~63ee0d2c` (FUGIN kein Release; `xfer:mountain-342-2`
  geschlossen), `.3~1635806e` (MODVOLC-Block + Lauf; Echo `xfer:mountain-342-3`) — gefaltet.
- `xfer:mycelium-344-1` · `-2` · `-5` — gefaltet; `-1` (harvest-Direktive), `-2`
  (fünf Fäden), `-5` (LOD im finals-Arm) als Punkte unten.
- `xfer:river208-saber1` · `-ucarfpi1` · `-athabasca1` · `xfer:river.ses_ec90200eeffeoKqvQwo4yZSRQo.1~fb7d0ebb`
  · `xfer:river.ses_ec901ad7dffehLKKusa1DO806h.3~1b6c1cbf` — bereits mountain-342 gefaltet;
  die Register-Entwürfe fließen in die Punkte unten ein.

## Offen (aufgeschlüsselt)

### Venus Radar PDS4-IMG — Arm gebaut, Block + Split offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau
- **Lage:** (gemessen 2026-10-14) Bundle `urn:nasa:pds:venus_radar_level2` v1.0, DOI
  `10.17189/1522443`, **CC0-1.0**; Wurzel
  `https://pds-geosciences.wustl.edu/venus/urn-nasa-pds-venus_radar_level2/`; 15 Produkte
  (5 Epochen × `inc`/`ocp`/`scp`), je 960 000 000 B headerloses `Array_2D_Map`
  (IEEE754LSBSingle, 12000×20000, Sinusoidal 1000 m/pixel, R=6051 km); Bundle total
  13,41 GiB → Split-Route. Lesearm `src/archivar/pds4_img.rs` (`82a393fb9`, `cargo check` 0/0).
- **Blockade:** Blockform offen — die 15 Produkte tragen je eigene Einheit (`inc` = deg;
  `ocp`/`scp` = dimensionslos); der Arm deklariert keine Felder.
- **Braucht:** Block + Split-Route + Compiler; per-Produkt-Einheit/`tau` entscheiden.

### ICGEM gfc — Override gebaut, Modell-Epoche ungemessen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Rat-Verdikt zur Epoche einer statischen Quelle / ein gemessenes Modell-Epoch
- **Lage:** (gemessen 2026-10-14) `--epoch <ISO-8601>`-Override in `gfc_compiler.rs`
  (`a650b1327`; Header gewinnt, sonst Override, sonst Verweigerung). **Riss 1:** kein
  Referenz-Epoch dokumentiert (`.gfc`-Header, DataCite `10.5880/ICGEM.2019.007`, GFZ-Landing,
  Zingerle et al. 2020 tragen keins). **Riss 2:** volles Modell ≈ 12,8 GB > 2-GiB-Release-Deckel
  (`cdn.rs:230`); zulässiger Cap D=3211.
- **Blockade:** der Wire-Epoch ist Pflicht; ein statisches Feld hat physikalisch vielleicht keinen.
- **Braucht:** der Rat (fünf Stimmen) zur Epoche (τ=0-Pfad vs. deklarierte Katalog-Epoche);
  dann Block `url …/icgem.gfz.de/gfc.bin`, `terms CC-BY-4.0 https://doi.org/10.5880/ICGEM.2019.007`,
  `format gfc`, `at earth`, 4 Felder.
- Braucht dazu: der CDDIS-`blocked account` ist **kein Key-Gap** — widerlegt per
  `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.3~98d34b2c` (HTTP 200 mit `EARTHDATA_EDL_TOKEN`).

### Athabasca ERG-SC CDF — Form geklärt, Innenform offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die gemessene CDF-Variablen-Liste
- **Lage:** (gemessen 2026-10-14) generischer `format cdf`-Arm
  (`extract.rs:5047` → `cdf25::extract_witness_channels`); Wurzel
  `https://ergsc.isee.nagoya-u.ac.jp/data/ergsc/ground/vlf/ath/` (Jahres-/Monats-/Stunden-CDF,
  je ~55–62 MB, `isee_vlf_ath_<YYYYMMDDHH>_v01.cdf`), DOI
  `10.34515/DATA.GND-0004-0002-0201_v01` (`--verdict` HTTP 206). Lizenz gemessen:
  **`erg-sc-rules-of-the-road`** (+ ISEE/PWING-VLF-Acknowledgement; Weiterverteilung verboten),
  terms-URL `https://ergsc.isee.nagoya-u.ac.jp/data_info/erg.shtml.en`. **`terms` ist im Block
  zulässig** (`parse.rs:267`) — kein Form-Hindernis.
- **Blockade:** der `format cdf`-Arm liest nur skalare Variablen (`num_values == 1`); die
  konkreten Kanal-Variablennamen der CDF sind ungemessen (Download ~55 MB → CI-lastig).
- **Braucht:** ein CDF herunterladen, die skalaren Variablen messen → `field`-Zeilen +
  `witness` + `terms erg-sc-rules-of-the-road <url>` + `force em` + `on earth <lat> <lon> <alt>`.

### UCAR-FPI — Arm verdrahtet, Sample liegt, Block fehlt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `field`-Zeilen aus dem Sample
- **Lage:** (gemessen 2026-10-14) Arm + Zuordnung + Guard (`2d4c526f4`,
  `src/archivar/fpi_ucar.rs` + Dispatch `extract.rs:159`). Sample gemessen (future251-ucarfpi1):
  `data/registration.hao.ucar.edu/20246300athabasca.dat`, 7353 Zeilen, sha `34c08b39…`,
  10 leerzeichengetrennte Spalten, Kopf `yyday ut(hr) merid(m/s) merid err dir ut(hr) zonal(m/s)
  zonal err dir skytemp`. Lizenz gemessen: **keine benannte** — UCAR/DASH-Daten-Nutzungssatz
  (PI vor Publikation kontaktieren, DOI zitieren), terms-URL `https://data.ucar.edu/dataset/fpi-resolute`.
  **Riss:** kein Athabasca-FPI-DOI (die Netz-DOIs Resolute usw. stehen nicht dafür ein).
- **Blockade:** Block-Innenform (`url` CDN vs. raw + Compiler) offen.
- **Braucht:** Block mit `terms ucar-hao-data-use https://data.ucar.edu/dataset/fpi-resolute`,
  dem gemessenen Athabasca-`url` und den zwei `field`-Zeilen `fpi_mer_wind_ms`/`fpi_zon_wind_ms`.

### SABER — Höhenform (C) offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die Form-Zeile
- **Lage:** (gemessen 2026-10-14) Rat-Verdikt (C): Höhe als zweite, gleich indizierte
  Koordinate auf geschlossener Level-Achse (`altitude(time, level)`, 400 Stufen); der
  Level-Index muss Teil des Schlüssels sein. Entwurf
  `state/river/weberin-register-entwuerfe-2026-10-13.md` Block 5. Lizenz gemessen:
  **`saber-rules-of-the-road`**, terms-URL
  `https://spdf.gsfc.nasa.gov/pub/data/timed/saber/SABER_rules_of_the_road.txt`; Datenwurzel
  `https://spdf.gsfc.nasa.gov/pub/data/timed/saber/`, HAPI `TIMED_L2A_SABER`, DOI
  `10.48322/2src-g677`. **`terms` ist im Block zulässig.**
- **Blockade:** die `channel`/`field`-Form ist nicht als Registerzeile geschrieben; ggf. fehlt ein Arm.
- **Braucht:** die `channel`/`field`-Zeile nach Form (C); Block `terms saber-rules-of-the-road <url>`.

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
  `state/mycelium/block-drafts/2026-10-13-faden-schliessung.φ`: Sonnen-Magnetfeld (JSOC HMI +
  SILSO SSN, CC-BY-NC-4.0), Stern-Aktivität (VizieR J/ApJ/835/61 + LISIRD), 21 cm
  (EBHIS/WALLABY/CDS + EDGES Zenodo `16881868`), Wolken (ISCCP HGM + PATMOS-x), TGF
  (Fermi-GBM-TGF + HEASARC-WWLLN-Sferics). Quellen gemessen (HTTP/DOI).
- **Blockade:** keine.
- **Braucht:** `terms`/`field`/`ttl` je Block; danach registriert Mycelium `url`/`origin`/`compiler`.

### Doppel-Arme RAVE/GALAH/1LHAASO — Verdikt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-13) RAVE `:11481`, GALAH `:11781`, 1LHAASO `:11788` tragen je
  zwei Arme (VizieR-tsv + tap).
- **Blockade:** keine.
- **Braucht:** je der Doppel-Arm-Verdikt (welcher Arm trägt), dann den überzähligen entfernen.

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
- **Lage:** (gemessen 2026-10-14) Rat (fünf Stimmen) + Roster (sieben Seats) konvergieren:
  `celestrak.org/SpaceData/EOP-All.csv` ist eine inhaltsgleiche Abschrift der USNO-finals.
  Block `phi:10144-10148`; CelesTrak-Asset 404.
- **Blockade:** keine.
- **Braucht:** `url`/`origin`/`compiler` des Blocks entfernen (Mycelium, `## An mycelium`) +
  Mountain-Verdikt `descoped: inhaltsgleich mit USNO finals`; dann `register_sort`.

### finals-Leser — Interpolation + Stale-Flag + LOD (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14) Rat + Roster: der Jetzt-Wert ist auf „jetzt" zu
  interpolieren (UT1−UTC driftet ~1 ms/Tag, LOD derselben Zeile); die ersten 1–2 `P`-Tage
  sind als Brücke zulässig; nach ~2–3 Tagen ohne `I`-Zeile ein Stale-Flag. Gebaut ist die
  Auswahl der letzten gemessenen `I`-Zeile (`d26f42dc8`). mycelium-344-5: **LOD in den
  `finals`-Arm ergänzen** (`7eb835f2a` hat COMP_LOD im `celestrak_eop`-Arm; der `finals`-Arm
  braucht die Zeile ebenfalls).
- **Blockade:** keine.
- **Braucht:** Interpolation + Stale-Flag + LOD in `build_finals_channels`.

## An mycelium

Origin: mountain-343 (2026-10-14) — trägt die offenen Einträge aus mountain-342:

- `xfer:mountain-342-7` **CelesTrak streichen:** Block `phi:10144-10148` — Rat + Roster
  gemessen inhaltsgleich mit USNO finals, Asset 404. Bitte `url`/`origin`/`compiler`
  entfernen; das Quellen-Verdikt (`descoped`) setzt Mountain. (`-1`/-2/-3 sind von
  mycelium-345 geechot: ENTSO-E-Tag, FUGIN-Riss, MODVOLC.)

## An river

Origin: mountain-343 (2026-10-14) — trägt die offenen Einträge aus mountain-342:

- `xfer:mountain-342-5` **UCAR-FPI:** `format ucar_fpi`-Arm + Guard committet; die zwei
  `field`-Zeilen folgen mit dem gemessenen Sample (`data/registration.hao.ucar.edu/20246300athabasca.dat`).
- `xfer:mountain-342-6` **Athabasca/SABER:** die Registerzeilen folgen (Athabasca nach
  gemessener CDF-Variablen-Liste, SABER nach Form C); `terms` ist im Block zulässig.
  (`-4` DE-2 war mountain-341 und ist geschlossen.)

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes."** | 2026-10-14 | (mountain 343).
- **„bitte schau dir direkt an warum die zeugen keine lizenzen tragen"** | 2026-10-14 | (mountain 343 — Befund + `ingest-terms-absent`-Gate).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. In diesem
Atom (folge343) committet (HEAD == origin/main): `169d5ab10` (Gate + Fixture + Test) ·
diese Übergabe.
