<!--
  title: Handover — Mountain-Folge 343 (2026-10-14)
  session: Mountain-Linie — ein Pass: warum Zeugen keine Lizenzen tragen (gemessen), `ingest-terms-absent`-Gate gebaut
  class: handover
  date: 2026-10-14
  sha256: df0fdeb405b6ef2c81644e07b6420c742513fff0273d7461547e3d90d4d82797
  status: live
-->
# Handover — Mountain-Folge 343 (2026-10-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert (`state/zustand/standing-pass.md`), nie kopiert. Diese
Session konsumierte `handover-2026-10-14-mountain-folge342.md` (→
`docs/handover/archiv/`) und faltete die adressierten Blöcke (future-folge253,
mycelium-folge345).

## Burn: open 0.00 · close 0.31 · cap 0.40 Grund: ein Pass — vier Architektur-Fragen mit voller Recherche/Rat/Roster-Runde (10 Seats gestellt) + zwei Taucher-Wellen + CDF-Reader-Bau + Koordinaten-/Höhenmessung.

Gemessen `session_burn` (opencode.db, Session „Warum Zeugen-Blöcke keine Lizenzen tragen", deepseek-flash).

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

## Architektur — Recherche · Rat · Roster (2026-10-14)

**Recherche (gemessen, drei Taucher).**
- *ICGEM/XGM2019e:* `.gfc`-Header trägt kein `epoch`/`ref_epoch`; DataCite `10.5880/ICGEM.2019.007`, GFZ-Landing, Zingerle et al. 2020 nennen kein Referenzjahr — nur Publikationsjahr 2019/2020. Statisches Kombinationsfeld (GOCO06s + 15′-Bodengitter). Wire-`response_epoch` = max Record-`SLOT_EPOCH` (`gfc.rs:101`); der Compiler verlangt Header- oder `--epoch`, sonst Verweigerung. Präzedenz: statische Kataloge deklarieren eine Konstante im Format-Modul (`viking_grav` 1976, `fermi_4lac` 2008-08-04); `catalog_epoch` nur für Sternfelder.
- *Venus:* 15 Kacheln `venus_{1988|2012|2015|2017|2020}_north_{inc|ocp|scp}.img`, je 960 000 000 B, `Array_2D_Map` 12000×20000 IEEE754LSBSingle, Sinusoidal 1000 m/pixel, R=6051 km, DOI `10.17189/1522443`. Einheit nur im Label-`<comment>`: `inc` = Grad, `ocp`/`scp` = dimensionslos. Kein `format`-Compiler; Arm `pds4_img.rs` reduziert auf 12000 Zeilenmittel. CC0-1.0 **nicht** im Label (nur DOI).
- *Athabasca:* die 64 082 658-B-CDF ist **CDF-komprimiert** — die Variablennamen sind nicht als ASCII messbar; kein CDF-Reader-Bin. Variablennamen nur als Behauptung aus `pyspedas` (`ch1`,`ch2`,`freq_vlf`). Station ATH gemessen **54.60 N, 246.36 E**, Höhe absent.
- *UCAR-FPI:* Live-URL antwortet 200, aber der Body ist das **E-Mail-Registrierungsformular** (Login/Cookie nötig); Verzeichnis 404; nur 2023+2024 je eine Athabasca-Datei; **kein** `ucar_fpi`-Compiler. Athabasca-Standort co-located ≈ 54.7 N, 246.7 O, ~500 m (FPI-eigene Koordinate ungemessen).

**Rat (fünf Stimmen).** F1 = **(b)** (statt/absent bis eine physikalische Referenzepoche gemessen ist; nie 2019). F2 = **eine Quelle, ein Block, Einheit am Feld** (`inc:deg`, `ocp`/`scp:1`), Vorlage über die Jahres-Liste; Riss: bände die Grammatik die Einheit nur am Block, müssten es drei Blöcke sein. F3 = **C-1** (`field ktemp.<i>` Level im Schlüssel, `alt tpgpaltitude km` Höhe als `Position.alt`; HAPI-Arm muss `size` expandieren und `param.N` spaltenweise adressieren); C-2 und `freq`/`bin_width` verworfen. F4 = **(b) Compiler+fetch-einmal, aber CDN durch nicht-offene Lizenz blockiert → `local-only`/`blocked account`**; Live verweigert; Registrierung = per-act consent (Operator-Hand).

**Roster (10 Seats gestellt).** Geantwortet mit den vier Fragen:
| Seat | F1 | F2 | F3 | F4 |
|---|---|---|---|---|
| Rat | (b) | 1 Block, Einheit am Feld | C-1 dotted | compiler, aber local-only |
| Qwen | (b) | 3 Blöcke + 15 explizit | dotted | compiler+CDN |
| Claude Sonnet 5.5 | (b) (c als Notausgang) | 1 Block, Einheit am Feld, Vorlage | dotted | live-fetch, kein CDN |
| MiniMax M3 | (b) | 3 Blöcke + Vorlage | dotted | live/credentialed, kein CDN |
| Lumo 2.0 Max | (a) | 3 Blöcke + Vorlage | alias | CDN-Asset |

Nicht-Antwort (gemessen): Z.ai/GLM **kam nicht zurück** (Deep Think lief auf einer fremden Frage weiter, `pending`); Duck.ai · DeepSeek · AI Studio · Inkling zeigten den Alt-Verlauf (Nachricht erreichte den Composer, keine neue Antwort).

**Konvergenz/Riss.** F1 (b) und F3 (C-1/dotted) **konvergieren** (Rat + Qwen/Claude/MiniMax); Lumo dissentierend (a / alias). F2 **Riss**: ein Block mit Einheit am Feld (Rat/Claude) gegen drei Blöcke (Qwen/MiniMax/Lumo) — die Grammatik trägt die Einheit je `field` (parse.rs:1642), daher gilt der Rat; der Riss bleibt benannt. F4 **Riss**: live vs. CDN, aufgelöst durch die **nicht-offene Lizenz** → kein CDN (`local-only`, Operator-Wort 2026-10-13).

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
- **Braucht:** Rat ergangen: **eine Quelle, ein Block, Einheit am Feld** (`inc`=Grad, `ocp`/`scp`=1), **Vorlage** über die Jahres-Liste; Compiler per Kachel; CDN nur für das reduzierte Asset (~2,5 MB × 15), Roh 13,41 GiB bleiben am PDS-Ursprung; Lizenz am DOI messen. **Riss:** drei Blöcke dissentieren (Qwen/MiniMax/Lumo) — grammatisch trägt die Einheit je `field`.
- **Riss:** CC0-1.0 steht **nicht** im Label (nur die DOI); die Lizenz ist am DOI-Landing zu messen, ehe der Block die `terms`-Zeile erhält.

### ICGEM gfc — Override gebaut, Modell-Epoche ungemessen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Rat-Verdikt zur Epoche einer statischen Quelle / ein gemessenes Modell-Epoch
- **Lage:** (gemessen 2026-10-14) `--epoch <ISO-8601>`-Override in `gfc_compiler.rs`
  (`a650b1327`; Header gewinnt, sonst Override, sonst Verweigerung). **Riss 1:** kein
  Referenz-Epoch dokumentiert (`.gfc`-Header, DataCite `10.5880/ICGEM.2019.007`, GFZ-Landing,
  Zingerle et al. 2020 tragen keins). **Riss 2:** volles Modell ≈ 12,8 GB > 2-GiB-Release-Deckel
  (`cdn.rs:230`); zulässiger Cap D=3211.
- **Blockade:** der Wire-Epoch ist Pflicht; ein statisches Feld hat physikalisch vielleicht keinen.
- **Braucht:** Rat ist ergangen (**b**: keine erfundene Epoche). Nächster Schritt: die Referenzepoche des Langwellen-Elterns **GOCO06s** aus dessen eigener Quelle messen; fehlt sie, bleibt der Block ungeschrieben (kein `format gfc` ohne Epoch). Danach Block `url …/icgem.gfz.de/gfc.bin`, `terms CC-BY-4.0 https://doi.org/10.5880/ICGEM.2019.007`, `format gfc`, `at earth`, 4 Felder.
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
- **Blockade:** der `format cdf`-Arm (`extract.rs:5047`) nutzt nur `cdf25` (CDF 2.5/2.6); die ath-Datei ist CDF **3.8.0, whole-file gzip-komprimiert** — dieser Pfad ist im Arm noch nicht eingezogen.
- **Braucht:** Reader **gebaut** (`tools/measure/src/bin/cdf_vars.rs`, `cargo build -p omegaflow-measure --bin cdf_vars` 0/0, `rustfmt` sauber): die Datei trägt Magic `0xcdf30001`, Bytes 4–7 `0xcccc0001` (→ whole-file-Kompression), gzip-Strom ab Offset 40, `cType 5`; der Baum hat bereits den vollen CDF-3.0-Leser `src/archivar/cdf.rs` (`CdfFile::parse`), der nur an `WholeFileCompressed` scheitert. **Variablen gemessen:** `epoch_vlf` (TT2000), `ch1`, `ch2`, `freq_vlf` (REAL4 skalar), `label_freq_vlf` (CHAR 5), `amplitude_cal_vlf_ch1`, `amplitude_cal_vlf_ch2`. **Nächster Schritt:** den v3+Ganzdatei-Dekompression-Pfad in den `format cdf`-Arm ziehen (statt `cdf25`), dann Block: `url …/isee_vlf_ath_<YYYYMMDDHH>_v01.cdf`, `terms erg-sc-rules-of-the-road https://ergsc.isee.nagoya-u.ac.jp/data_info/erg.shtml.en`, `format cdf`, `witness …`, `field` je Kanal, `force em`, `on earth 54.60 246.36 693`.
- **Koordinate/Höhe (gemessen 2026-10-14):** die ISEE-Kopftabelle **und** Shiokawa et al. 2017 Table 1 nennen **54.60 / 246.36 = AUGO II**, Offizialstandort **54.60282 / −113.64442, 693 m** (Wayback `augo.athabascau.ca/location`); die ISEE-Beschreibung nennt dagegen **54.7 / 246.7 = AUGO I** (Campus, Site-Log 54.71340/−113.31308, ellipsoidal 551,93 m). Die exakte Loop-Position ist in keiner Quelle genauer als 0,1° publiziert. **Höhe:** AUGO II offiziell **693 m** (deckt sich mit GEBCO 693 m); die drei 30/90-m-DEMs geben ~700 m; **NRCan HRDEM 1 m (~0,3 m) trägt an AUGO II keine Deckung** (nur am Campus AUGO I: DSM 571 m). Antenne über Grund **5,3 m** (Shiokawa 2017, HR-130C). → Stationshöhe **693 m** (dokumentiert), Instrument ≈ **698 m**. **Riss:** welches AUGO die Loop trägt (Tabelle/Paper = II, Beschreibung = I) — die Koordinate entscheidet die Höhe (~693 m vs. ~571 m).

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
- **Braucht:** Rat ergangen: **Compiler + fetch-einmal**, aber **CDN durch die nicht-offene Lizenz blockiert → `local-only`** (Arbeitskopie unter `data/`), kein `phi`-`url`/`compiler`; die Registrierung ist per-act consent (Operator-Hand, pending). Offen: die **FPI-eigene** Athabasca-Instrumentenkoordinate messen (die Co-Location ≈ 54.7 N/246.7 O ist nicht die Instrumentenkoordinate); die zwei `field`-Zeilen `fpi_mer_wind_ms`/`fpi_zon_wind_ms` stehen bereit.

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
- **Braucht:** Rat ergangen: **C-1** — `field ktemp.<i> …` (dotted Level im Schlüssel), neue Direktive `alt tpgpaltitude km` (Höhe als `Position.alt`); der HAPI-Arm muss `size` in dotted-Indices expandieren und `param.N` spaltenweise adressieren (heute fehlt der Arm). Block `terms saber-rules-of-the-road <url>`.

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
- **„Athabasca kannst du nicht mit unsrenen geodäsie daten und den bauplänen exakt die höhe feststellen? kein CDF-Reader-Bin vorhanden müssen wir den bauen? Login = Operator-Hand. dann öffne die seite"** | 2026-10-14 | (mountain 343 — Höhe ≈700 m gemessen; CDF-v3-Reader nötig; UCAR-Registrierungsseite geöffnet).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. In diesem
Atom (folge343) committet (HEAD == origin/main): `169d5ab10` (Gate + Fixture + Test) ·
diese Übergabe.
