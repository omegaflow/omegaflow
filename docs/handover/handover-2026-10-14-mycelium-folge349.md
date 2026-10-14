<!--
  title: Handover — Mycelium-Folge 349 (2026-10-14)
  class: handover
  date: 2026-10-14
  sha256: 155f73e44ad03dd90f82b769af41b34ad23770eac912f05ca4d6ff091e2d6170
  status: live
-->
# Handover — Mycelium-Folge 349 (2026-10-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
öffentliche Projektion `docs/zustand/standing-pass.md`). Diese Session konsumierte
`handover-2026-10-14-mycelium-folge348.md` (→ `archiv/`).

Gefaltet (`register_lookup --addressed mycelium`): `mountain-folge346` (Venus-Parser erledigt ·
Gewebe-Fäden IGETS/BGI/GEOFON an Mycelium) · `river-folge211` (intermagnet aufgeklärt ·
ENTSO-E-Defekt · CTBTO · Weberin-Zeugen · Schumann · Register→Manifest).

## Burn: open 0.0 · close pending — der Session-Store (`opencode.db`) fehlt auf dieser Maschine, `session_burn` ist nicht gemessen (Vorbehalt wie river-212) · cap 0.40 · Grund: die Register-Messung (`cdn_reconcile`, `--tokens`, `--orphans`, `--orphan-docs`), die HIGP-Bestands-Messung, die Handover-Fortschreibung.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | 2026-10-14 | `state/operator-gespraeche/2026-10-14-mycelium.md` |
| „verhalte dich als würdest du auf line stehen" (line-Agent, auto-bestätigt) | 2026-10-14 | dito |
| „Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus … Commit und Push trägt `/commit`." — session-weiter Consent (Delegation), **nicht** das Commit-Wort | 2026-10-14 | dito |
| „bitte prüfe welche keys wir haben" · „kannst du nicht per browser auf die seiten" | 2026-10-14 | dito |
| „auf jedem fall mit antrag" — **Route (b): CN-NSSDC-Datenantrag absenden** | 2026-10-14 | dito |
| „nochmal taucher mit harten bandagen die auf den browser nutzen" | 2026-10-14 | dito |
| „das ist privat" — die private Pass-Fassung wird öffentlich nicht gelesen | 2026-10-14 | dito |
| (standing) „lokal für Experimente sind nicht offene Lizenzen ok aber nicht veröffentlichen" | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md`; `AGENTS.md` (`local-only`) |
| (standing) „ab jetzt Reports immer in einfacher Sprache" | 2026-10-13 | dito |
| (standing) „nein du hast recht das ist wirklich nicht meine aufgabe" (CHIME/FRB + Lizenzstichprobe) | 2026-10-13 | dito |
| (standing) „ja bitte ich möchte sauber sein" (Stilllegung der drei Direktabrufe) | 2026-10-13 | dito |

## Offen — eigen

### HIGP-MODVOLC-Archiv 2000–2019 — Vollzug **unvollständig** (abgebrochen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Bau/Vollzug (abgesetzter Hintergrund-Job)
- **Lage:** (gemessen 2026-10-14/349) Der Vollzug (2000–2019) ist **unvollständig**; zwei Teilbäume (beide ungetrackt):
  - `data/modis.higp.hawaii.edu/` — `flat/2000/` = **14304 Dateien** (davon **3080 mit Inhalt** >100 B; größte 826327 B — der reale Granul-Satz für 2000) **plus** `2000/<doy>/` = **75570 Dateien** als 5-Minuten-Rasterung (je Tag 288 Slots; 219 davon 21-B-Placeholder — die Route fragte blind jeden Slot ab).
  - `~/projects/omegaflow-work/data/modis.higp.hawaii.edu/` — `2000/` 7653 Dateien (letzte Schreibung `2000/083` 13:00), `2019/` 288, `2001`–`2014` **leer**, `2015`–`2018` fehlen.
  Kurz: **nur 2000** liegt (doppelt, in zwei Strukturen) vor, plus ein 2019-Fragment; **2001–2018 fehlen**. Laufzeit 07:39–13:00. `local-only` (© UH Mānoa, keine Lizenzzeile) — kein CDN, keine `phi`-Zeile. Der LAADS-Weg bleibt **geschlossen** (siehe unten).
- **Blockade:** keine.
- **Braucht:** einen korrigierten Vollzug **2001–2018** — nur die realen Granule (Inhalt >0), nicht das 5-Minuten-Raster; dann als local-only-Bestand führen.

### MODVOLC-Archiv 2000–2019 (LAADS) — abgesagt; HIGP ist die Alternative
- **Status:** descoped (Befund = Eintrag) | **Bindung:** eigen
- **Trigger:** kein — Route gemessen und geschlossen
- **Lage:** (gemessen 2026-10-14/347) LAADS-Antwort: **„this product is not approved for public distribution, so users cannot access this product"** → die LAADS-Route ist **geschlossen**. Alternative: HIGP-Bulk-Tree (`modis.higp.hawaii.edu`, kein Login, `local-only`); für den modernen Rand deckt LANCE `nrt3` (`Recent/`, Workflow `modvolc-cdn.yml`) ab.
- **Blockade:** keine.
- **Braucht:** nichts — der Befund steht; der HIGP-Vollzug (oben) trägt den Bestand.

### DE-2-Manifestation — Host `cdaweb.gsfc.nasa.gov` unerreichbar
- **Status:** wartend | **Bindung:** wartend
- **Trigger:** CDAWeb-Host antwortet wieder
- **Lage:** (gemessen 2026-10-14/347) compiler + Workflow + Block (`phi:586`) stehen; CI `38552385568` scheitert **gemessen am Host** (TLS-Abbruch); lokal `--verdict` + Browser bestätigen host-weit unerreichbar (`heasarc`/`fermi` grün). `asset=fehlt` bleibt.
- **Blockade:** externer Host-Ausfall (kein Maschinen-Akt).
- **Braucht:** bei Host-Rückkehr `gh workflow run de2-cdn.yml`; dann Asset → `asset=present` + K messbar.

### Register→Manifest — CI-Trigger gebaut; der generische Aktor fehlt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Bau
- **Lage:** (gemessen 2026-10-14/348) `harvest-dispatch.yml` reagiert auf eine ausdrückliche `harvest <actor>`-Zeile (→ Workflow mit `# harvest: format=<actor>`) und auf `format`-Diffs; die Doppelbedeutung von `format` ist aufgelöst. **Der Schumann-Block trägt jetzt `harvest raw_zenodo` (`phi:1051`)** — Mountain-Direktive steht (`xfer:mycelium-344-1` erfüllt, Echo). Offen: der **generische Aktor** (`# harvest: format=csv arm=raw_mirror`) für die compiler-losen Formate (csv/json/cdf/hapi) — die statischen CSV unter den fünf Fäden (SILSO, EDGES, Fermi-TGF) sind `raw_mirror`-Kandidaten, brauchen aber eine registrierte `sha256`.
- **Blockade:** keine für die Deklaration; der Bau berührt die Register-Semantik (Fingerabdruck-Tor) → Rat.
- **Braucht:** generischen Aktor entwerfen/bauen (Mycelium, Rat-Vorlage).

### Vier Endpunkte — SEANOE/GEOFON offen (ICGEM/gamma-cat gefaltet)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Zulassung
- **Lage:** (gemessen 2026-10-13/342) ICGEM `XGM2019e.gfc` (206)·gamma-cat `gammacat.ecsv` (BSD-3) gefaltet — die ICGEM-Manifestation (`url`/`origin`/`compiler`) wartet auf die epochenlose Wire-Ablage (mountain-345). Offen: **SEANOE** `https://www.seanoe.org/data/00311/42182/` (206; Lizenz `pending`) · **GEOFON-FDSN** (CDN-Asset 404; `geofon` fehlt im Register; Mountain liefert `terms CC-BY-4.0` + Kernel `point`/`inverse-square`, s. `## An mountain`).
- **Blockade:** Zulassung/`terms` = Mountains Recht.
- **Braucht:** Mountain-Zulassung; danach `url`/`origin`/`compiler`. An Mountain: `xfer:mycelium-342-3` (SEANOE) · `xfer:mycelium-342-4` (GEOFON).

### Fünf fehlende Fäden — Quellen und Compiler gemessen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`/`ttl` je Block
- **Lage:** (gemessen 2026-10-14/348 via Compiler-Zensus `tools/harvest/src/bin`; Mountain-Entwurf `state/mountain/faden-term-field-ttl-draft.φ`) Faden 1 Sonnen-Magnetfeld (JSOC HMI 1a/1b + SILSO 1c) · Faden 2 Stern-Aktivität (VizieR `J/ApJ/835/61` 2a + LISIRD 2b) · Faden 3 21 cm (EBHIS `J/A+A/585/A41` 3a — **bereits registriert** `phi:14494`; WALLABY 3b; EDGES 3c) · Faden 4 Wolken (ISCCP HGM 4a; PATMOS-x 4b) · Faden 5 TGF (Fermi-GBM 5a; WWLLN-HEASARC 5b; Zenodo 5c). **Compiler-Zensus (gemessen):** nur `hmi_polar_compiler.rs` (1b) und `wwlln_compiler.rs` (5b/5c) existieren; für 1a/1c/2a/2b/3b/3c/4a/4b/5a fehlt der Compiler. Die statischen CSV (1c, 3c, 5a) sind `raw_mirror`-Kandidaten. Junktim: `terms`/`field`/`ttl` = Mountain; `url`/`origin`/`compiler` = Mycelium.
- **Blockade:** `terms`/`field`/`ttl` = Mountains Recht; für die compiler-losen Quellen zusätzlich der generische Aktor (Rat).
- **Braucht:** Mountain-Zeilen aus dem Entwurf; danach `url`/`origin`/`compiler` je Faden + der generische Aktor für die statischen CSV.

### Fermi-4LAC DR3 — Assetname-Riss (Mountain gefaltet)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Wort: Roh-FITS als Asset **oder** `F4LC`-Arm
- **Lage:** (gemessen 2026-10-13/342) Block `phi:14553-14558` `format fits` ohne `compiler`/`field`; Compiler lädt `fermi_4lac_dr3.bin`, Block erwartet `table_4LAC.fits`; kein `F4LC`-Arm in `extract.rs`.
- **Blockade:** Mountains Wort; `extract.rs` fremd-dirty.
- **Braucht:** Mountains Wort; danach `compiler`-Zeile + `field`. An Mountain: `xfer:mycelium-342-5`.

### Weberin-Zeugen — Manifestation der drei Zeugen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `channel`/`field`-Zeile
- **Lage:** (gemessen river-209) `url`/`origin`/`compiler` der drei Zeugen (SABER `TIMED_L2A_SABER` keyless; UCAR-FPI E-Mail-gated = Operator; Athabasca ERG-SC keyless) sind Mycelium; `weberin-verdicts-cdn.yml` existiert. Athabasca-`channel`/`field` stehen (`phi:402-411`).
- **Blockade:** Manifestation erst nach Mountains `channel`/`field`.
- **Braucht:** Mountain-Zeile; danach Manifestation ziehen. An Mountain: `xfer:river208-weberinmanifest1`.

### Register→Manifest generischer Aktor + Ersatzquellen + LOD + BASE + WDC/terms + ttl
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`/`ttl`/Zulassung je Block
- **Lage:** (gemessen 2026-10-13/344) unverändert offen: **Ersatzquellen-Registerblöcke** (OCNS2019 `local-only`) · **LOD im `finals`-Arm** (USNO/MAIA `phi:8121` ohne LOD) · **BASE** (Venus-Radar-PDS > 2-GiB-Limit `cdn.rs:230`; ALFALFA AAS-CC-BY-NC-Riss) · **WDC-Kyoto-Token** + vier `terms`-Korrekturen · **`ttl`/`no-cadence` + `register_sort`** (`register_sort` bricht ohne `ttl` ab).
- **Blockade:** Verdikt-/`terms`-/`field`-Recht = Mountain.
- **Braucht:** Mountain-Zeilen; danach `url`/`origin`/`compiler` + Manifestation. An Mountain: `xfer:future.…7d5c9163` (OCNS2019 `local-only`) · `xfer:mycelium-344-5` (LOD) · `xfer:mycelium-343-3` (BASE) · `xfer:mycelium-342-10`/`xfer:mycelium-343-2` (WDC/terms) · `…ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.2~accd94dd`/`3~8eff6a4e` (ttl).

### Formate der vier Restcompiler — gemessen am realen Register
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `field`/`terms` je Block
- **Lage:** (gemessen 2026-10-14/348 am realen Register; die Handover-Zeilennummern von 346 sind verschoben)
  - `cosmic_ro` (`phi:10182-10192`): **vollständig** — `url`/`terms CC-BY-4.0`/`format`/`origin`/`compiler`/`sha256`/`at`/`ttl`/`quantity`/`field`s. Kein Schritt → `descoped`.
  - `catalog_mpcorb` (`phi:3837-3843`): `url`/`terms PD`/`format`/`origin`/`compiler`/`at`/`ttl` — **`field` fehlt**.
  - `opentopography_coverage` (`phi:11436-11441`): `url`/`origin`/`compiler`/`format`/`at`/`ttl` — **`terms` und `field` fehlen**.
  - `heasarc_w3browse`: **kein Register-Block** (Reader-Arm `src/archivar/heasarc_w3browse.rs`).
- **Blockade:** `field`/`terms` = Mountains Recht.
- **Braucht:** Mountain `field` (catalog_mpcorb) + `terms`/`field` (opentopography_coverage); `heasarc_w3browse`/`cosmic_ro` als `descoped` messen.

### Venus-Radar — Parser steht; PDS-Block und Bild-Manifestation fehlen (mountain-346-2)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Bau
- **Lage:** (gemessen 2026-10-14) Der `pds3_img`-Parser liest jetzt `BAND_STORAGE_TYPE` (BSQ/BIL/BIP; Fire mountain-346-6, `xfer:mycelium-346-6` **erledigt**). Die Projektion ist gemessen (Sinusoidal, CM 335°E). Der Zenodo-`Processed.zip`-Block steht (`phi:29393`: `url`/`terms CC-BY-4.0`/`origin`/`compiler raw_mirror`/`sha256`/`no-cadence` — **ohne `format`**); die CDN-Matrixzeile `venus_radar_Processed.zip` steht in `zenodo-raw-cdn.yml:52`. Der **Roh-PDS-Radar-Block** (Venus PDS) und die **Bild-Manifestation (W)** fehlen.
- **Blockade:** `terms` (PD) = Mountain.
- **Braucht:** PDS-Roh-Block (`url`/`origin`/`compiler` = Mycelium; `terms PD` = Mountain) + `format`; dann die Bild-Manifestation.

### NSSDC/CLPDS-Planetologie — `完全共享` anonym ladbar; `有条件共享` mit Antrag; SSDC ohne Lizenz
- **Status:** eigen (Antrag operator-gebunden; Route entschieden) | **Bindung:** eigen
- **Trigger:** Operator-Wort „auf jedem fall mit antrag" (2026-10-14) — Route (b); der Send bleibt Operator-Hand
- **Lage:** (gemessen 2026-10-14/347, drei Browser-Taucher) Antragsentwurf liegt bei Future (`post/in/2026-10-14-mycelium-nssdc-datenantrag.md`, folge349 gelesen, vollständig).
  - **`完全共享` = anonym ladbar, kein Login/Key:** `GET https://archive.nssdc.ac.cn/archives/archives/dataSetBase/download?pathArrayString=/CATALOG/catalog.xls&dataSetID=<id>&webInfo=vsso` → HTTP 200 ZIP. CLPDS ebenso: `GET https://moon.bao.ac.cn/moon-admin/client/science/dataInfo/getAnnexZip/<id>` → `PUBDATA`-URL, `authorization: Bearer undefined`, anonym HTTP 206.
  - **`有条件共享` = Antragsroute (gemessen):** Registrierung (`umt.nssdc.ac.cn/usermanager/register`, Echtname) → Login → Formular `APPLICATION_FORM` → PDF drucken + unterschreiben → hochladen → Autorisierung. `点击下载` ausgeblendet.
  - **Riss (belegt):** Katalog-Record `CC BY 4.0` / `完全共享` vs. CLPDS-„Copyright Notice: without the written permission … no unit or individual may copy, modify, transcribe, disseminate and sell".
  - **SSDC:** keine Lizenz-/Terms-Seite; direkter Download teils ohne Login.
  - **Offene Mirrors (gemessen):** CE-1/2 MRM → NASA PDS (PD, `phi:11137-11155`, CDN 206) · CE-4 LPR → Zenodo `14590856`/`15117915`, Figshare (**CC-BY-4.0**) · CE-3 LPR → Zenodo `3884522` · CE-4 VNIS → Zenodo `3361004` · Tianwen-1 RoPeR → Zenodo `15812343`/`8035493`/`15812357` (CC-BY-4.0, `phi:28689+`) · MoRIC-DOM → CDS HiPS (**ODbL-1.0**) · Mars-Daten → MAVEN MAG (NASA PDS), Zenodo (CC-BY-4.0). **CLPDS-only bleibt:** CE-4 LFRS, CE-5-LPR-Roh, CE-3/4-VNIS-Roh, CE-1/2-CCD-Stereo-Roh, DOM + CE-4/5-Kamera-Roh, Tianwen-1 HiRIC/MMS/MINPA.
- **Blockade:** Antrag = Operator-Hand; Lizenz-Riss ungeklärt.
- **Braucht:** Operator sendet den CN-NSSDC-Antrag; danach Abruf. Die `完全共享`-Sätze sind anonym ladbar — Manifestation (Mountain-`terms` + CDN) separat. **Registerblöcke der neuen Mirrors** (Mycelium `url`/`origin`/`compiler`, Mountain `terms`) — zuerst Zenodo-LPR + MAVEN MAG.

### Browser-Autofill am NSSDC-CAS-Login — Wert rotiert
- **Status:** eigen (Operator-Akt) | **Bindung:** eigen
- **Trigger:** kein — Autofill-Eintrag entfernen
- **Lage:** (gemessen 2026-10-14/347) Das CAS-Login lud mit autofill-vorbelegten Zugangsdaten (nicht echoiert, kein Login ausgeführt). Der Login ist rotiert (Operator-Wort 2026-10-14) → der exponierte Wert ist entwertet.
- **Blockade:** Operator.
- **Braucht:** nur noch den Autofill-Eintrag im Browser entfernen.

## Schlüssel → offene Fäden (gemessen 2026-10-14/347 via `bin/secrets_keys`)

Nur Namen, nie Werte. **A = Schlüssel vorhanden → der Faden ist ein Abruf.**
**B = Schlüssel reicht nicht (Gate bleibt).**

**A:** China-Planetologie (CLPDS `完全共享`) — `CLPDS_USER/PASS` (+ `NSSDC_USER/PASS`, `SSDC_USER/PASS`) · Sonnenmagnetfeld (JSOC HMI) — `JSOC_EMAIL` · 21 cm/Spektren (CDS) — `CDS_API_KEY` · Wolken (ISCCP/PATMOS-x) — `EARTHDATA_EDL_TOKEN`/`NOAA_CDO_TOKEN` · Schumann/Zenodo — `ZENODO_TOKEN` · **CTBTO/WALDO** — `WALDO_USER/PASS` · **Geomagnetik** — `SUPERMAG_USER/PASS` · `KCDC_USER/PASS` · `LEOS_USER/PASS` · `NIED_USER/PASS` · `KASI_DALO_USER/PASS` · ENTSO-E — `ENTSOE_API_TOKEN` · BASE — `BASE_API_KEY` · IGETS — `IGETS_USER/PASS` · Copernicus — `CMEMS_USER/PASS`/`CDS_API_KEY`/`CDSE_USER/PASS` · ESA/JAXA — `ESA_USER/PASS`/`JAXA_GPORTAL_USER/PASS` · CelesTrak — `SPACETRACK_USER/PASS` · KNMI — `KNMI_API_KEY` · Literatur — `NASA_ADS_TOKEN`/`OPENALEX_API_KEY`/`S2_API_KEY`/`UNPAYWALL_EMAIL`/`ORCID_*`.

**B:** LAADS C6 — Token da, aber **nicht freigegeben** (2026-10-14) → geschlossen, Alternative HIGP · China-NSSDC `有条件共享` — **Antrag + Autorisierung** + Lizenz-Riss · SSDC — **keine Lizenz deklariert** · Vier Endpunkte (SEANOE/GEOFON/ICGEM/gamma-cat) — keyless, gated an Mountains `terms` · UCAR-FPI (Weberin-Zeuge) — Download offen (HTTP 200), `format`-Arm/Parser fehlt; UCAR-COSMIC schon registriert · DE-2 — Host down, Ersatzgrößen CHAMP (GFZ CC-BY-4.0), ICON/GRACE (Zenodo CC-BY-4.0) · SuperDARN — Mirror-Route `api.bas.ac.uk/superdarn/mirror/v3/` (`phi:1273`) + `superdarn_rawacf.bin`/`superdarn_fitacf.bin` registriert; **LOCK** nur auf den **Record/Globus-Korpus** (Operator-Wort 2026-09-29).

## An mountain

Origin: mycelium-349 (2026-10-14):

- `xfer:mycelium-344-2` **Fünf Fäden** — Mountain-Teil liegt als Entwurf `state/mountain/faden-term-field-ttl-draft.φ`; die `terms`/`field`/`ttl`-Zeilen setzen.
- `xfer:mycelium-346-3` **MODVOLC-Archiv 2000–2019 (HIGP)** — `local-only`-`terms`-Verdikt (keine Lizenzzeile).
- `xfer:mycelium-346-4` **DE-2 NACS/WATS** — SPASE-**CC0**-DOIs `10.48322/z50y-hd65`/`j8kp-5y58` registrieren (`terms CC0`).
- `xfer:mycelium-346-5` **OCNS2019-Ersatz** — JPL SSD/Horizons (NASA CC0) als offene Alternative.
- `xfer:mycelium-346-7` **AWESOME-VLF/LF** — `doi.org/10.48322/fwte-dv13` (+ `fzcf-f191`), **CC0**; als offene Quelle registrieren.
- `xfer:mycelium-344-5` **LOD in den `finals`-Arm** ergänzen.
- `xfer:mycelium-342-3`/`-4`/`-5` · `xfer:mycelium-343-2`/`-3` · `xfer:mycelium-342-10` — wie im Register geführt.
- `xfer:mycelium-349-1` **`field` für `catalog_mpcorb`** (`phi:3837-3843`).
- `xfer:mycelium-349-2` **`terms`+`field` für `opentopography_coverage`** (`phi:11436-11441`).
- `xfer:mycelium-349-3` **`cosmic_ro`/`heasarc_w3browse` als `descoped` messen**.
- `xfer:mycelium-349-4` **`terms PD` für den Venus-PDS-Roh-Block**.
- `xfer:mycelium-349-5` **GEWEBE: GEOFON-Wellenform** — Mountain liefert `terms CC-BY-4.0` + Kernel `point`/`inverse-square`; Mycelium setzt `url` (CDN-Asset) + `compiler`, dann `register_sort`.
- `xfer:mycelium-349-6` **Gewebe IGETS `origin`** (`phi:10319`) — Projekts**seite** oder **Datenpfad** (`igetsftp.gfz.de`)? Mycelium-Entscheidung offen.

## An river

Origin: mycelium-348/349 (2026-10-14):

- `xfer:mycelium-347-1` **intermagnet-XYZ** — **erfolgreich**: `intermagnet-xyz-cdn` Lauf `38618793419` (`conclusion: success`, head `c7a6c7ca2`). Der Pull steht; River setzt `al_network_proxy`/`gic_onset_detector`.
- `xfer:mycelium-344-4` **DE-2 + `asset=`-Riss** — die Riss-Klärung steht (`harvest-dispatch.yml:106` liest `asset=`); der Lauf `38552385568` scheitert **gemessen am Host** (`cdaweb.gsfc.nasa.gov` TLS-Abbruch). `asset=fehlt` bleibt bis zum ersten erfolgreichen Pull.

## An future

Origin: mycelium-348/349 (2026-10-14):

- `xfer:mycelium-347-2` **CN-NSSDC-Planetologie — Datenantrag.** Entwurf liegt in Futures Postfach (`post/in/2026-10-14-mycelium-nssdc-datenantrag.md`); der Operator hat die Route **mit Antrag** entschieden (2026-10-14). Lizenz **gemessen**: Katalog-Record `CC BY 4.0`, Portal CLPDS `all rights reserved` — ein Riss. **Operator-Hand:** Antrag über das NSSDC-Portal einreichen.
- `xfer:mycelium-346-8` **CTBTO + WALDO — das Warten blockiert nichts** — Infrasound offen via NSF SAGE/EarthScope + IMS (CC BY 4.0); VLF/LF via Zenodo-CC-BY-4.0 + AWESOME CC0.
- `xfer:mycelium-346-9` **NSSDC Mariner-10 (`PSCM-00009`)** — Detailseite public; `download.action` = 206/1 B. CRUSO-Antrag bleibt Operator-Hand (Entwurf `state/mail/nssdc-mariner10-pscm-00009.md`).
- `xfer:mycelium-346-10` **Secret-Hygiene-Incident (346) — `NASA_API_KEY` exponiert, behoben** — neuer Key angefordert (2026-10-14); alter Wert als exponiert behandelt.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
