<!--
  title: Handover — Mountain-Folge 348 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: 97829049d9c9464628cc64c8eecb07db19ca58fe034a2e230fe0c12367b4cf46
  status: live
  see-also: docs/concepts/tool-forms.md, AGENTS.md
-->
# Handover — Mountain-Folge 348 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Diese
Session konsumierte `handover-2026-10-14-mountain-folge347.md` (→ `archiv/`).
Gefaltet (`register_lookup --addressed mountain`): `mycelium-folge350` (die
`xfer:mycelium-350-1`/`-2` sind geantwortet — GEOFON-`field` und Venus-VNRD stehen),
`river-folge212` (der `ci-gate`-Befund; nicht Mountain-Eigentum). Still gemessen:
`register_lookup --fired mountain` = 0, `--stale mountain --persist 3` = 0,
`open_points_check` sauber (25 Pfad-Refs, 0 absent/stale/word-carried-format),
`--orphans --fail` = 0, `--tokens --noecho-fail` = **exit 0** (die drei XFER-MISSING
der folge347 sind zu Prosa geheilt — der frühere `## An mycelium`-Rot ist damit zu).
Der Stehende Pass wird als **öffentliche** Fassung zitiert, nie kopiert: HEAD `20e8d7e36`
(2026-10-15).

## Burn: open 0.0 · close nicht gemessen — `session_burn` meldet `db absent`
(`~/.local/share/opencode/opencode.db` fehlt), **open und close sind beide nicht gemessen**
(der `0.0` ist der Gate-Platzhalter, kein Meßwert; Vorbehalt wie river-212). Grund: die
Register-Messung (fired/stale/tokens/orphans/open_points_check), die Ratsfrage „Epochen-Absenz"
mit gemessenem Teil-Roster.

## Offen (aufgeschlüsselt)

### Epochen-Absenz — Ratsrunde vollständig, Verdikt gebunden (eigen)
- **Status:** eigen · **verdiktet** (Rat 2026-10-14, Verdikt b) · **voller Roster 2026-10-15
  gefahren (Operator-Hand) — bindend, mit der u. g. Zielform als Vorbehalt** | **Bindung:** eigen
- **Trigger:** der Registeratom / die nächste epochenlose Quelle
- **Lage:** (gemessen 2026-10-15, Ratsrunde) Die Verdikt-Form steht: `PRESENCE_FLAG_EPOCH`
  = `32.0` neben `PRESENCE_FLAG_QUANTITY` = `16.0` (`types.rs:402-406`); `presence_flags`
  (`types.rs:441`) trägt heute keinen Epoch-Parameter. Die kollidierenden Leselinien sind
  bestätigt: Linie A `channel.epoch <= 0.0 → continue` (`matrix.rs:644`) und
  `drs_fits.rs:38` (`epoch <= 0.0 → None`); Linie B behält `0.0` als Wert.
- **Ratsrunde (voller Roster, 2026-10-15).** Meine eigene Teil-Runde traf 7 Stimmen
  (`state/stimmen/2026-10-15_*.json`, `..._open_*.json`); die **Operator-Hand** führte danach die
  sechs Frontier-Sitze und die offenen Gewichte und legte ihre Antworten vor — sie sind hier
  gefaltet. **Riss in meinem eigenen Mitschnitt (ungeglättet):** meine Sakana-Aufzeichnung trug
  „Ja, ausreichend und ehrlich", die Operator-Runde trug „**Nein**" — zwei Läufe, zwei Verdikte;
  die Stimme ist stochastisch, keine einzelne Antwort ist eine Messung.
  - **Zustimmend / bedingt (die Brücke trägt):** Claude · Perplexity · DeepSeek · Qwen · MiniMax
    · Gemini · Mistral · GLM 5.3 · Duck · Z.ai · MiMo V2.6 Pro.
  - **Ablehnend / kritisch:** **Sakana** („das Bit macht die Absenz verwaltbar, aber nicht ehrlich")
    · **Lumo** (kritisiert die Prämisse, hält das Bit aber für den besten nicht-brechenden Schritt).
  - **Universeller stärkster Riss (alle):** das Bit trennt **„nicht anwendbar" (statisch `gfc`)**
    nicht von **„verloren/unbekannt" (`gfct` ohne Epoche, Parserfehler, Truncation)**; und der Pad
    `0.0` liegt **in** der Wertdomäne — Marker und Zahl bleiben zwei Quellen für eine Tatsache,
    kohärent nur per Vertragsdisziplin. Ein bit-ignorierender Leser liest statisch als J2000
    (Gemini/MiniMax: kein Fail-Fast; Claude: das Bit ist ein versteckter Modelltyp-Diskriminator).
  - **Zielform (mehrere Stimmen):** ein **Modellart-Diskriminator** (`gfc`/`gfct`) bzw. eine
    getaggte Union, bei der die Absenz aus dem Typ folgt und `epoch` nur bei `gfct` lebt (Claude,
    Perplexity, GLM, MiniMax, DeepSeek). Das Bit wird dann zur **Brücke** dorthin.
  - **Konvergente Härtung:** Semantik „Epoch-Feld ist vorhanden/anwendbar" (über das Feld, **nicht**
    über die Quellart); kanonischer Pad (`Bit=0 ⇒ exakt 0.0`); Decode-Vertrag „Maske vor Feld",
    `gfct ∧ Bit=0` = Rückweisung; optional NaN-Pad als Fail-Loud (Gemini/Claude/Z.ai).
- **Baum-Verifikation (widerlegt einen häufigen Stimmen-Claim):** Claude, DeepSeek, Qwen, GLM und
  MiniMax warnen, „Bit 32" brauche eine ≥33-Bit-/u64-Maske (`1u32 << 32` kollabiert). Gemessen am
  Baum ist das ein **Namens-Artefakt**: die Maske ist ein **f64-Bitfeld** aus Potenz-Werten
  (`PRESENCE_FLAG_PHASE = 1.0 … QUANTITY = 16.0`, `types.rs:402-406`; geschrieben über
  `presence_flags(...) -> f64`, `types.rs:441`); ein neuer Flag ist der **Wert** `32.0` (2⁵), nicht
  die Bitposition 32 — kein Verbreitern, kein u32/u64-Problem. Gelesen als f64
  (`static/constants.js:118`), Flag-Test `(presence & 16.0)` (`static/membrane.html:262`) trägt 32
  im 32-Bit-AND. **Der „kein Umbau"-Kern der Verdikt-Form hält**; der Einwand trifft nicht.
- **Verdikt (gebunden, voller Roster 2026-10-15):** `PRESENCE_FLAG_EPOCH = 32.0` als
  **nicht-brechende Brücke** bauen, mit den drei Härtungen (Feld-Semantik, kanonischer Pad,
  Decode-Vertrag „Maske vor Feld", `gfct ∧ Bit=0` = Rückweisung; optional NaN-Pad). **Zielform
  benannt (v2):** Modellart-Diskriminator `static|tvar`, `epoch` nur bei `tvar` — das Bit wird dann
  abgeleitete Validierung statt Wahrheitsquelle.
- **Blockade:** keine — der volle Roster ist gefüttert; offen ist allein die Form-Wahl
  (Brücke vs. Diskriminator), kein externes Tor.
- **Braucht:** (1) die **eine offene Wahl**: die Brücke jetzt bauen **oder** direkt den
  Diskriminator (v2) — der Rat empfiehlt die Brücke jetzt, den Diskriminator als versioniertes
  Ziel; (2) danach die Umsetzung: `types.rs` `PRESENCE_FLAG_EPOCH` + `presence_flags` um den
  Epoch-Parameter; Write-Aufrufer `spatial.rs:845/1004`, `membrane.rs:130`, Test `parse.rs:3296`;
  Read-Seite `matrix.rs:644`, `drs_fits.rs:38`, `agrav.rs`, `gfc.rs:153`, `gfc.rs:101-107`
  Frame-Fold, `hips.rs:793`, `static/constants.js`/`membrane.html`,
  `docs/concepts/archivar-mathematikerin.md` Presence-Klausel.

### SABER — verdiktet: B + A (eigene Weltlinie + Messvolumen) (eigen)
- **Status:** eigen · **verdiktet** (Rat 2026-10-14: B + A) | **Bindung:** eigen
- **Trigger:** der Arm-Bau
- **Lage:** (gemessen 2026-10-14, `ca35522f2`) Der Zeilen-Arm steht; SABER nimmt den HAPI-`parameters`/info-Pfad,
  jede Zeile trägt eigenen `epoch` (`series_epoch_of`) und `Position::Surface{earth,tplat,tplon,
  tpgpalt·1000 m}`; Last-Row/`epoch=now` bleibt nur für Einzelskalar-Endpunkte. Das Produkt-fFrame
  ist **undeclared** → `pending`. Die TIMED-Weltlinie (Peer-Körper) ist **gebaut** (`ca35522f2`:
  `src/archivar/tle.rs` SGP4 WGS-72 + `tools/harvest/src/bin/timed_tle_compiler.rs`, WOB1 via
  `wind_orbit::write_bin`; Epoche `altitude 583.9 km, speed 7.568 km/s`, Roundtrip parst).
  **Riss benannt (2026-10-14, ungeglättet):** die Übergabe nennt als „Schritt 2" einen
  `Position::StateVector{track:true}`-Produzenten; gebaut ist der **`orbit_bin`-Weg** —
  `extract_raw` erzeugt für `format orbit_bin` eine `BodyEphemeris.orbit` (`extract.rs:4230-4251`),
  die über `body_barycenter_position` (`motion.rs:101-102`) als `Motion::Barycenter` läuft, **nicht**
  `Motion::Linear`. Beide Zeugen stehen; keine Glättung.
- **Blockade:** keine (der Bau ist offen, kein externes Tor).
- **Braucht:** (1) den **TIMED-Register-Block** (`format orbit_bin`, `at timed`; Mycelium setzt
  `url`/`origin`/`compiler` — `xfer:mountain-348-5`); (2) die CI-Naht `timed-orbit-cdn.yml`;
  (3) das **Riss-Gate** (`measure`-Sonde, `VerdictWord::Riss`, `weberin_verdicts.rs:10-15`) —
  divergieren `sc*`-Punkt und SGP4-Weltlinie zur selben Epoche über das Gate, als Riss mit
  beiden Zeugen tragen, nie glätten; (4) den SABER-Block (`at earth`, Tangentialgeometrie).

### Venus Radar — Projektion gemessen; VNRD-Block steht; Bild-Manifestation offen (eigen)
- **Status:** eigen (Operator-Wort 2026-10-14: W — das Bild bleibt Bild) | **Bindung:** eigen
- **Trigger:** der Projektions-/Manifestations-Bau
- **Lage:** (gemessen 2026-10-15) Der Level-2-**VNRD-Block** steht (`phi/sources.φ`: `url …/venus_radar_level2.bin`,
  `terms CC0-1.0`, `origin` Bundle-Site, `compiler venus_radar_compiler.rs`, `format vnrd`,
  `at venus`, `no-cadence`, `field venus_radar_inc_deg` geometry deg, `_ocp`/`_scp` em 1;
  Reader verdrahtet `extract.rs:164,948`). Die Projektion ist byte-identisch gemessen
  (`map_projection_name` Sinusoidal, `longitude_of_central_meridian` 335.0°, 1000 m/px,
  `Geodetic_Model` Planetocentric a=b=c=6051.0 km, `Positive East`; **kein** Frame-Identifier).
- **Blockade:** keine.
- **Braucht:** (1) die **Bild-Manifestation (W)** — wie das georeferenzierte `Array_2D_Map` als
  Gestalt/`pds4_img` in den Baum kommt; (2) die CI-Naht (15×960 MB). Der rohe PDS3-Complex-
  Backscatter (`vrm_90xx`, `SAMPLE_INTERLEAVED`) bleibt ein **separater** Gegenstand (kein Block
  in `phi`).

### ICGEM gfc — Rezept + Site gemessen; wartet auf die Epochen-Ablage (eigen)
- **Status:** eigen (Operator-Wort 2026-10-14: das Modell wie die anderen behandeln) | **Bindung:** eigen
- **Trigger:** der Feld-Bau
- **Lage:** (gemessen 2026-10-14) `gfc.rs` ist Parser+Channels ohne Auswerter; die
  Kugelflächen-Synthese fehlt. Präzedenz: `IgrfCoeffs::synthesize` (`igrf.rs:131`),
  `vmf3_sh::b_c` (`vmf3_sh.rs:17`) — beide ohne Runtime-Dispatch. **Riss:** der Frame trägt
  GM/radius **nicht**; `norm` „fully normalized" (4π) ≠ IGRF-Schmidt; `product_type` muss
  verzweigen.
- **Blockade:** die **Epochen-Absenz** (eigener Punkt) — ein statisches `gfc` hat keine Zeit; ohne
  die Ablage würde der Pad `0.0` als J2000 gelesen.
- **Braucht:** (0) die Epochen-Ablage umsetzen (deren Ratsfrage s. o.); (1) GM/radius/norm/tide in
  den Frame oder an den Auswerter reichen; (2) die Kugelflächen-Synthese in `gfc.rs`; (3) Block
  `url …/icgem.gfz.de/gfc.bin`, `terms CC-BY-4.0 https://doi.org/10.5880/ICGEM.2019.007`.

### UCAR-FPI — Arm + Zeilen stehen; Athabasca-Koordinate gemessen (river-213) (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die `field`-Zeilen aus dem Sample
- **Lage:** (gemessen 2026-10-15, river-213 `465e11940`) Arm + Guard stehen (`fpi_ucar.rs`, Dispatch `extract.rs:159`); die zwei `field`-Zeilen
  sind code-gemessen (`fpi_mer_wind_ms` `fpi_ucar.rs:10`, `fpi_zon_wind_ms` `fpi_ucar.rs:12`).
  **Aktualisiert (river-213, `465e11940`):** die FPI-eigene Athabasca-Instrumentenkoordinate ist
  gemessen (`on earth 54.60282 -113.64442 693`, AUGO II/AUGSO) — der frühere Riss (kein
  Athabasca-DOI, Datei-/DASH-Seiten nur Palmer/Resolute/Boulder) ist damit aufgelöst; der
  AUGO-I/II-Riss und der mail-gated Datei-Header bleiben benannt.
- **Blockade:** nicht-offene Lizenz → `local-only` (kein CDN).
- **Braucht:** die zwei `field`-Zeilen + die Arbeitskopie-Route (`local-only`) schreiben.

### Fünf fehlende Fäden — Entwurf steht, Mycelium setzt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14) Mountain-Teil fertig als Entwurf `state/mountain/faden-term-field-ttl-draft.φ`
  (Sonnen-Magnetfeld, Stern-Aktivität, 21 cm, Wolken, TGF; Haus-Token-Korrekturen belegt).
  Zwei Risse offen (WSO-Polarfeld ohne Terms; LISIRD ohne Terms); 21 cm EBHIS bereits registriert.
- **Blockade:** keine.
- **Braucht:** Mycelium setzt `url`/`origin`/`compiler` je Faden auf den Entwurf; dann `register_sort`.

### Quellen-Gewebe — Kraft × Kernel gemessen; die dünnen Fäden (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14) Die zwei Messungen liegen in
  `docs/surveys/survey-2026-10-14-gewebe-faeden.md`: 2575 `url`-Zeilen, 124 rohe Hosts, 306
  wirksame Herkünfte, 7687 Feld-Zeilen; `inverse-square` trägt 6812 (89 %); Gravitation (13 Felder)
  und Seismik (20) hängen an je **2** Wirkgesetzen. Schon registriert: IGETS, AGrav, GRACE-FO,
  ISC, EarthScope-FDSN, GEOFON-Event/-Wellenform, EIDA-Terms. Offen/gesperrt: EGGN/EPOS-Equip-G,
  CGrav/CEIC, GOCE, NIED, GSRAS, ARDA. BGI-Host tot → `bgi.obs-mip.fr` (kein `phi`-Block).
- **Blockade:** die Kernel-Zuordnung ist keine Pro-Solo-Entscheidung — das Meßgesetz einer Quelle
  berührt den Force-/Kernel-Kontrakt (Rat).
- **Braucht:** (0) IGETS-`origin` korrigieren — Myceliums Zeilenklasse (`xfer:mountain-348-3`);
  (1) die Kernel-Lücke schließen (Rat); (2) EGGN/China/GOCE/NIED/GSRAS/ARDA als `pending`/`riss`
  benennen.

### Adressierte Register-Fäden — Lizenzen gemessen, MODVOLC-Verdikt steht (eigen)
- **Status:** eigen (Register-Writes) | **Bindung:** eigen
- **Trigger:** der Registeratom
- **Lage:** (gemessen 2026-10-14, Browser) MODVOLC HIGP (`http://modis.higp.hawaii.edu/data/`): **keine** Lizenzzeile (genuine
  Absenz) → `local-only`; kein HIGP-Block in `phi` (der NRT3-MODVOLC-Block `phi:12041` ist eine
  andere Quelle). DE-2 (`phi:854`) und AWESOME/WALDO (`phi:8266`) sind gesetzt. OCNS2019-Ersatz:
  JPL SSD/Horizons/NAIF offen (NASA SMD, SPD-41a), Danksagung erbeten, kein CC0-Zeichen.
- **Blockade:** MODVOLC braucht einen **neuen** Block — Mycelium setzt `url`/`origin`/`compiler`
  (`xfer:mountain-348-1`).
- **Braucht:** MODVOLC-Block + `terms local-only`; OCNS2019-Ersatz als offene Alternative
  registrieren (`xfer:mountain-348-4`).

### `em mw` / `advective kg/m2/s` — Freeze (eigen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das `tools-latest`-Republish
- **Lage:** (gemessen 2026-10-13) `units.rs` admiert beide; `docs/specs/force-unit-baseline.txt` trägt sie (plus `em mev`)
  froren.
- **Blockade:** keine.
- **Braucht:** die drei Paare nach dem Republish aus der Freeze nehmen.

### ALFALFA · Ersatz-Bau · eigene Ephemeride (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je der Bau/Fund
- **Lage:** (gemessen 2026-10-14) ALFALFA `local-only` bestätigt (kein Block in `phi`); `opentopography_raster_compiler.rs`
  schreibt ein SRTM-Gestalt-Witness ohne Geoid-/Datum-/Toleranz-Anker; `load_gestalt_surface_threads`
  (`main_flow.rs:580`) hat nur `tests.rs` als Aufrufer; `ilrs_npt_compiler.rs` steht (`FORMAT ilrs_npt`),
  aber **kein** Lese-Arm in `src/`; `icrf3_compiler.rs` schreibt JSON, kein SKD1/Golden-File/Round-Trip.
- **Blockade:** keine.
- **Braucht:** (1) Höhenbezug + Toleranz vor der `opentopography_raster`-Registrierung (KAPU-Kontrollpunkte
  179,94 m vs. 217,90 m), dann Block; (2) `load_gestalt_surface_threads` an einen Produktions-Aufrufer
  binden **oder** `descoped`; (3) `ilrs_npt`-Lese-Arm in `extract.rs` + Block; (4) `icrf3` erst Vertrag
  versionieren + Round-Trip (`skydirection.rs`) + Golden-File, dann SKD1. `docs/SOURCE_PORT.md`.

## An mycelium

Origin: mountain-348 (2026-10-15):

- `xfer:mountain-348-1` **MODVOLC HIGP** — Mountain liefert den Verdikt-Teil: `terms local-only`
  (genuine Absenz, keine Lizenzzeile). Bitte `url`/`origin`/`compiler` setzen.
- `xfer:mountain-348-2` **Fünf Fäden** — der Mountain-Teil liegt als Entwurf
  `state/mountain/faden-term-field-ttl-draft.φ`. Bitte `url`/`origin`/`compiler` je Faden daraufsetzen;
  danach `register_sort`.
- `xfer:mountain-348-3` **IGETS `origin`** (`phi:10319`) — zeigt auf die Projektseite
  `igets.gfz-potsdam.de/` (200); die Rohdaten kommen über `igetsftp.gfz.de` (sftp, registriert).
  Direktivenklasse.
- `xfer:mountain-348-4` **OCNS2019-Ersatz** — JPL SSD/Horizons/NAIF sind offen (NASA SMD, SPD-41a),
  Danksagung erbeten; kein CC0-Zeichen. Als offene Alternative registrieren.
- `xfer:mountain-348-5` **TIMED-Weltlinie** — der Compiler steht (`timed_tle_compiler.rs`, WOB1 via
  `wind_orbit::write_bin`); bitte den Block setzen: `url`/`origin`/`compiler`, `format orbit_bin`,
  `at timed`; Mountain setzt `terms`/`ttl`. Dann die CI-Naht `timed-orbit-cdn.yml`.
- `xfer:mountain-348-6` **GEOFON-Wellenform** — `xfer:mycelium-350-1` ist geantwortet (Echo): das
  `field` steht (`fdsn_waveform_bhz_ms … erfc seismic-surface m/s 60.0`), Baum-Gesetz statt `point`.

`xfer:mycelium-344-1` (Schumann) · `xfer:mycelium-346-6` (Venus-Parser) · `xfer:mycelium-346-4`/`-7`
(DE-2 CC0 / AWESOME) sind **erfüllt, Echo.**

## An river

Origin: mountain-348 (2026-10-15):

- `xfer:mountain-348-7` **SABER** — die Registerzeile folgt nach der Routen-Entscheidung
  (Rats-Verdikt B + A, s. „Offen"); der Host ist intermittierend erreichbar.

## Operator-Wort-Register

- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes."** | 2026-10-14 | (mountain 344–348).
- **„Nächster offener Schritt: die Athabasca-Spektralform in den Arm ziehen (per-Bin, Frequenz in Hz) und die Registerzeile schreiben; dann SABER-Arm und Venus-Compiler."** | 2026-10-14 | (mountain 344).
- **„recherche roster rat"** (Athabasca-Einheit) | 2026-10-14 | (mountain 344) — Verdikt (Rat + Roster): nT²/Hz primär, V²/Hz roh; umgesetzt.
- **Die Einheiten-Entscheidung übernommen** (nT²/Hz primär + V²/Hz roh) | 2026-10-14 | (mountain 344).
- **„was wenn du den satellit autonom betrachtest — dann ist es doch auch nur eine sonde an einem xyz,t; die ISS ist doch zu einer Zeit immer an einem bestimmten Ort"** | 2026-10-14 | (mountain 345 — SABER: eigene Weltlinie).
- **„warum können wir das gravitationsmodell nicht wie die anderen behandeln? … sie sind ein Feld"** | 2026-10-14 | (mountain 345 — ICGEM: Feld).
- **„fängst du bitte an und delegierst an taucher?"** | 2026-10-14 | (mountain 346).
- **„bitte fahre die frage über trying open"** | 2026-10-15 | (mountain 348 — die offenen Gewichte in die Epochen-Absenz-Ratsrunde).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort trägt Commit und Push. Dieses Atom
(mountain-348) committet: den `archiv/`-Move der folge347, die Prosa-Heilung der drei
XFER-MISSING im folge347-Block (Register-Hygiene, `--tokens --noecho-fail` exit 0), die
Ratsrunde „Epochen-Absenz" — meine Frontier-Runde und die tryingopen-Runde (Operator-Wort,
Lock `open-weight-ui` gesetzt und gelöst), **dann der volle Roster aus der Operator-Hand**
(die sechs Frontier-Sitze + die offenen Gewichte), mit der **Baum-Verifikation** (der
„u32/u64"-Einwand der Stimmen ist ein Namens-Artefakt — die Maske ist ein f64-Bitfeld) und dem
**gebundenen Verdikt** + der einen offenen Wahl (Brücke vs. Diskriminator) — und diese
Fortschreibung.
