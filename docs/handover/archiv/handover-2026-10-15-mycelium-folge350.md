<!--
  title: Handover — Mycelium-Folge 350 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: 3f3fd28079ab9c9b60d13fba1d427e623302cef390bf490be0483cfb377fb8c7
  status: live
-->
# Handover — Mycelium-Folge 350 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/standing-pass.md`;
öffentliche Projektion `docs/zustand/standing-pass.md`). Diese Session konsumierte
`handover-2026-10-14-mycelium-folge349.md` (→ `archiv/`).

Gefaltet (`register_lookup --addressed mycelium`): `mountain-folge347` (cdn_reconcile-Gate
gehalten · vier CDN-Netlocs an Mycelium · opentopography `terms PD` · die Restcompiler-Verdikte) ·
`mountain-349` (dieselben xfers, Echo) · `river-folge212` (intermagnet aufgeklärt · ENTSO-E ·
Weberin-Zeugen · Schumann · Register→Manifest). **Der Baum gewinnt gegen die Doku** (gemessen,
nicht geglättet): der ENTSO-E-Block steht (`phi:11826`) und der Compiler heilt ZIP/`out_Domain`
(`52312fade`) — rivers „keine lebende Zeile/`in_Domain` ohne `out_Domain`" ist stale; der
Schumann-`sha256` (`phi:1053`) und die Matrixzeile (`zenodo-raw-cdn.yml:56`) stehen — rivers
Bitte ist erfüllt.

## Burn: open 0.0 · close pending — der Session-Store (`opencode.db`) fehlt auf dieser Maschine,
`session_burn` ist nicht gemessen (Vorbehalt wie river-212) · cap 0.40 · Grund: die
Register-Messung (`cdn_reconcile`, `register_sort`, `--tokens`, `--orphans`, `--orphan-docs`),
die IGETS-`origin`-Direktive, der GEOFON-Wellenform-Block.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | 2026-10-15 | `state/operator-gespraeche/2026-10-15-mycelium.md` |
| „auf jedem fall mit antrag" — Route (b): CN-NSSDC-Datenantrag | 2026-10-14 | `state/operator-gespraeche/2026-10-14-mycelium.md` |
| „Führe den … bestätigten Plan aus … Commit und Push trägt `/commit`." — session-weiter Consent (Delegation), **nicht** das Commit-Wort | 2026-10-14 | dito |
| „nochmal taucher mit harten bandagen die auf den browser nutzen" | 2026-10-14 | dito |
| „das ist privat" — die private Pass-Fassung wird öffentlich nicht gelesen | 2026-10-14 | dito |
| (standing) „lokal für Experimente sind nicht offene Lizenzen ok aber nicht veröffentlichen" | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md`; `AGENTS.md` (`local-only`) |
| (standing) „ab jetzt Reports immer in einfacher Sprache" | 2026-10-13 | dito |
| (standing) „nein du hast recht das ist wirklich nicht meine aufgabe" (CHIME/FRB + Lizenzstichprobe) | 2026-10-13 | dito |
| (standing) „ja bitte ich möchte sauber sein" (Stilllegung der drei Direktabrufe) | 2026-10-13 | dito |

## Gearbeitet (dieses Rund)

- **Vier CDN-Quellen — geschlossen.** Die vier ohne offene Lizenz nicht zugelassenen Quellen
  (`babamul`, `eve`, `maxi`, `mars_dust`) wurden in `1b519375b`/`9c60ddf89` aus `phi` entfernt;
  ihre vier Workflows sind mit `49b7a47f3` (HEAD) gefallen, die Baseline-Zeilen mit. Kein Block,
  kein offener Punkt.
- **IGETS `origin` entschieden** (`xfer:mycelium-349-6`). `phi:10319` zeigt jetzt den gemessenen
  **Datenpfad** `sftp://igetsftp.gfz.de` statt der Projektseite; der Compiler liest genau diesen
  Host (`igets_compiler.rs:8`, `SFTP_HOST`), der Workflow lädt in `data/igetsftp.gfz.de`
  (`igets-cdn.yml:71`). `sfetch --verdict https://igetsftp.gfz.de` = fetch void — der Host trägt
  kein HTTP, nur SFTP. `cdn_reconcile --fail` = clean (324 Hosts), `register_sort` = canonical.
- **GEOFON-Wellenform-Block gesetzt** (`xfer:mycelium-349-5`). Neuer Block in der `ttl 60`-Gruppe
  (`phi:163-169`, zwischen `dsn_snapshot` und `usgs_geomag_efield`): `url …/geofon.gfz-potsdam.de/fdsn_waveform.bin`,
  `origin https://geofon.gfz-potsdam.de/fdsnws/dataselect/1/query`,
  `compiler tools/harvest/src/bin/fdsn_waveform_compiler.rs`, `format fdsn_waveform`,
  `terms CC-BY-4.0 https://geofon.gfz.de/eqinfo/faq/`, `at earth`, `ttl 60` (die `terms`/`at`
  aus Mountains Verdikt, die `ttl` aus dem Geschwister EarthScope — Mountain bestätigt). Der
  Compiler trägt die GE-Route (`GEOFON_ROUTE`, `--route geofon`); die CI-Naht (GE-Station) und
  das `field` fehlen noch.
- **Restcompiler-Verdikte gefaltet** (mountain-347/349): `catalog_mpcorb` **descoped** (kein
  `field`, Zeuge wie die Geschwister-Kataloge) · `cosmic_ro` **descoped** (`quantity`+`field`
  stehen) · `heasarc_w3browse` **descoped** (Reader-Arm, kein Block) ·
  `opentopography_coverage` **pending** (`terms PD` liegt bei Mountain bereit, sobald der Baum
  sauber ist).
- **Folded/echo:** `xfer:mycelium-344-1` (Schumann `harvest raw_zenodo`) und `xfer:mycelium-346-6`
  (Venus-Parser) sind erfüllt; `xfer:mycelium-346-4`/`-7` (DE-2 CC0, AWESOME) sind gesetzt.

## Offen — eigen

### HIGP-MODVOLC-Archiv 2000–2019 — Vollzug **unvollständig** (abgebrochen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Bau/Vollzug (abgesetzter Hintergrund-Job)
- **Lage:** (gemessen 2026-10-14/349) Nur **2000** liegt (doppelt: `data/modis.higp.hawaii.edu/flat/2000/` = 14304 Dateien + `2000/<doy>/` = 75570 Raster-Dateien; `~/projects/omegaflow-work/data/modis.higp.hawaii.edu/` = 7653 Dateien) plus ein 2019-Fragment; **2001–2018 fehlen**. `local-only` (© UH Mānoa, keine Lizenzzeile) — kein CDN, keine `phi`-Zeile.
- **Blockade:** keine.
- **Braucht:** einen korrigierten Vollzug **2001–2018** — nur die realen Granule (Inhalt >0), nicht das 5-Minuten-Raster; dann als local-only-Bestand führen.

### Register→Manifest — CI-Trigger gebaut; der generische Aktor fehlt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Bau (berührt die Register-Semantik → Rat)
- **Lage:** (gemessen 2026-10-14/348) `harvest-dispatch.yml` reagiert auf eine ausdrückliche `harvest <actor>`-Zeile und auf `format`-Diffs; die Doppelbedeutung von `format` ist aufgelöst. **Der Schumann-Block trägt `harvest raw_zenodo` (`phi:1051`).** Offen: der **generische Aktor** (`# harvest: format=csv arm=raw_mirror`) für die compiler-losen Formate (csv/json/cdf/hapi); die statischen CSV unter den fünf Fäden (SILSO, EDGES, Fermi-TGF) brauchen eine registrierte `sha256`.
- **Blockade:** der Bau berührt die Register-Semantik (Fingerabdruck-Tor) → Rat.
- **Braucht:** generischen Aktor entwerfen/bauen (Mycelium, Rat-Vorlage).

### GEOFON-Wellenform — Block gesetzt; `field` + CI-Naht offen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `field`-Zeile (Kernel `point`/`inverse-square`)
- **Lage:** (gemessen 2026-10-15/350) Der Block steht (`phi` nach dem `fdsn_event`-Block:
  `geofon.gfz-potsdam.de/fdsn_waveform.bin`, `format fdsn_waveform`, `ttl 60`); der Compiler
  trägt die GE-Route. Die **CI-Naht** (eine `geofon`-Job-Variante von `fdsn-waveform-cdn.yml`
  mit gemessener GE-Station) und das `field` fehlen; das CDN-Asset ist bis dahin `fehlt`.
- **Blockade:** `field` = Mountains Recht; die GE-Station = eine eigene, noch ungemessene Wahl.
- **Braucht:** Mountains `field`; danach die CI-Naht bauen. An Mountain: `xfer:mycelium-350-1`.

### Venus-Radar (Level-2 VNRD) — Direktiven gemessen; Block + Bild-Manifestation offen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** kein — Bau
- **Lage:** (gemessen 2026-10-14/347) Bundle `urn:nasa:pds:venus_radar_level2` v1.0, DOI `10.17189/1522443`, **CC0-1.0**, 15 PDS4-`Array_2D_Map`-Produkte; Site `https://pds-geosciences.wustl.edu/venus/urn-nasa-pds-venus_radar_level2/`. Der Compiler `venus_radar_compiler.rs` schreibt `VNRD` (`vnrd.rs`), der Reader `format vnrd` ist verdrahtet (`extract.rs:164`). **Kein `venus`/`backscatter`/`vrm`-Block in `phi`.** Meine Direktiven: `url …/pds-geosciences.wustl.edu/venus_radar_level2.bin`, `origin` = obige Bundle-Site, `compiler tools/harvest/src/bin/venus_radar_compiler.rs`, `format vnrd`, `at venus`, `terms CC0-1.0` — der **Block** und das `field`/`ttl` sind Mountains, die **Bild-Manifestation (W)** und die **CI-Naht** (15×960 MB) offen. Der rohe PDS3-Complex-Backscatter (`vrm_90xx`, `SAMPLE_INTERLEAVED`) ist ein **separater** Gegenstand.
- **Blockade:** `field`/`ttl` = Mountains Recht.
- **Braucht:** Mountains `field`/`ttl`/Block; danach Manifestation + CI-Naht. An Mountain: `xfer:mycelium-350-2`.

### Vier Endpunkte — SEANOE/GEOFON offen (ICGEM/gamma-cat gefaltet)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Zulassung
- **Lage:** (gemessen 2026-10-13/342) ICGEM `XGM2019e.gfc` (206)·gamma-cat `gammacat.ecsv` (BSD-3) gefaltet — die ICGEM-Manifestation wartet auf die epochenlose Wire-Ablage (mountain-345). Offen: **SEANOE** `https://www.seanoe.org/data/00311/42182/` (206; Lizenz `pending`) · **GEOFON-FDSN** (CDN-Asset 404; `geofon` fehlt im Register; Mountain liefert `terms CC-BY-4.0` + Kernel `point`/`inverse-square`).
- **Blockade:** Zulassung/`terms` = Mountains Recht.
- **Braucht:** Mountain-Zulassung; danach `url`/`origin`/`compiler`. An Mountain: `xfer:mycelium-342-3` (SEANOE) · `xfer:mycelium-342-4` (GEOFON).

### Fünf fehlende Fäden — Quellen und Compiler gemessen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`/`ttl` je Block
- **Lage:** (gemessen 2026-10-14/348; Mountain-Entwurf `state/mountain/faden-term-field-ttl-draft.φ`) Faden 1 Sonnen-Magnetfeld (JSOC HMI 1a/1b + SILSO 1c) · Faden 2 Stern-Aktivität (VizieR `J/ApJ/835/61` 2a + LISIRD 2b) · Faden 3 21 cm (EBHIS `J/A+A/585/A41` 3a — registriert `phi:14494`; WALLABY 3b; EDGES 3c) · Faden 4 Wolken (ISCCP HGM 4a; PATMOS-x 4b) · Faden 5 TGF (Fermi-GBM 5a; WWLLN-HEASARC 5b; Zenodo 5c). **Compiler-Zensus:** nur `hmi_polar_compiler.rs` (1b) und `wwlln_compiler.rs` (5b/5c) existieren. Die statischen CSV (1c, 3c, 5a) sind `raw_mirror`-Kandidaten.
- **Blockade:** `terms`/`field`/`ttl` = Mountains Recht; für die compiler-losen Quellen zusätzlich der generische Aktor (Rat).
- **Braucht:** Mountain-Zeilen aus dem Entwurf; danach `url`/`origin`/`compiler` je Faden + der generische Aktor.

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

### Register→Manifest + Ersatzquellen + LOD + BASE + WDC/terms + ttl
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`/`ttl`/Zulassung je Block
- **Lage:** (gemessen 2026-10-13/344) unverändert offen: **Ersatzquellen-Registerblöcke** (OCNS2019 `local-only`) · **LOD im `finals`-Arm** (USNO/MAIA `phi:8121` ohne LOD) · **BASE** (Venus-Radar-PDS > 2-GiB-Limit `cdn.rs:230`; ALFALFA AAS-CC-BY-NC-Riss) · **WDC-Kyoto-Token** + vier `terms`-Korrekturen · **`ttl`/`no-cadence` + `register_sort`**.
- **Blockade:** Verdikt-/`terms`-/`field`-Recht = Mountain.
- **Braucht:** Mountain-Zeilen; danach `url`/`origin`/`compiler` + Manifestation. An Mountain: `xfer:future.…7d5c9163` (OCNS2019 `local-only`) · `xfer:mycelium-344-5` (LOD) · `xfer:mycelium-343-3` (BASE) · `xfer:mycelium-342-10`/`xfer:mycelium-343-2` (WDC/terms) · `…ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.2~accd94dd`/`3~8eff6a4e` (ttl).

### NSSDC/CLPDS-Planetologie — `完全共享` anonym ladbar; `有条件共享` mit Antrag; SSDC ohne Lizenz
- **Status:** eigen (Antrag operator-gebunden; Route entschieden) | **Bindung:** eigen
- **Trigger:** Operator-Wort „auf jedem fall mit antrag" (2026-10-14) — Route (b); der Send bleibt Operator-Hand
- **Lage:** (gemessen 2026-10-14/347) Antragsentwurf liegt bei Future (`post/in/2026-10-14-mycelium-nssdc-datenantrag.md`). `完全共享` = anonym ladbar (CLPDS + NSSDC download-API, HTTP 200/206); `有条件共享` = Registrierung → Formular → PDF → Autorisierung; **Riss:** Katalog-Record `CC BY 4.0` vs. CLPDS-„all rights reserved". Offene Mirrors gemessen (CE-1/2 MRM → NASA PDS; CE-4 LPR / Tianwen-1 RoPeR → Zenodo CC-BY-4.0; MoRIC-DOM → CDS HiPS ODbL). **CLPDS-only:** CE-4 LFRS, CE-5-LPR-Roh, CE-3/4-VNIS-Roh u. a.
- **Blockade:** Antrag = Operator-Hand; Lizenz-Riss ungeklärt.
- **Braucht:** Operator sendet den CN-NSSDC-Antrag; die `完全共享`-Sätze sind anonym ladbar (Mountain-`terms` + CDN separat). Registerblöcke der neuen Mirrors (Mycelium `url`/`origin`/`compiler`, Mountain `terms`) — zuerst Zenodo-LPR + MAVEN MAG.

### Browser-Autofill am NSSDC-CAS-Login — Wert rotiert
- **Status:** eigen (Operator-Akt) | **Bindung:** eigen
- **Trigger:** kein — Autofill-Eintrag entfernen
- **Lage:** (gemessen 2026-10-14/347) Das CAS-Login lud mit autofill-vorbelegten Zugangsdaten (nicht echoiert, kein Login ausgeführt). Der Login ist rotiert (Operator-Wort 2026-10-14) → der exponierte Wert ist entwertet.
- **Blockade:** Operator.
- **Braucht:** nur noch den Autofill-Eintrag im Browser entfernen.

## An mountain

Origin: mycelium-350 (2026-10-15):

- `xfer:mycelium-350-1` **GEOFON-Wellenform `field`** — der Block steht (`fdsn_waveform.bin`,
  `format fdsn_waveform`, `ttl 60`); setze das `field` (Kernel `point`/`inverse-square`,
  Ground-Velocity m/s) und bestätige die `ttl`.
- `xfer:mycelium-350-2` **Venus-Radar Level-2 VNRD** — Direktiven gemessen (Bundle `urn:nasa:pds:venus_radar_level2`, DOI `10.17189/1522443`, CC0-1.0, Site `…/venus/urn-nasa-pds-venus_radar_level2/`, Compiler `venus_radar_compiler.rs`, `format vnrd`, `at venus`); schreibe den Block (url/origin/compiler + `terms`/`field`/`ttl`).
- `xfer:mycelium-349-2` **`terms`+`field` für `opentopography_coverage`** (`phi:11436-11441`);
  `terms PD` liegt laut mountain-347 bereit.
- `xfer:mycelium-349-1` **`catalog_mpcorb` `field`** — als descoped gemessen (kein Feld am Arm).
- `xfer:mycelium-349-3` **`cosmic_ro`/`heasarc_w3browse`** — beide als descoped gemessen.
- `xfer:mycelium-344-2` **Fünf Fäden** — Mountain-Teil liegt als Entwurf `state/mountain/faden-term-field-ttl-draft.φ`.
- `xfer:mycelium-346-3` **MODVOLC (HIGP)** — `local-only`-`terms`-Verdikt.
- `xfer:mycelium-346-5` **OCNS2019-Ersatz** — JPL SSD/Horizons (NASA CC0) als offene Alternative.
- `xfer:mycelium-346-7` **AWESOME-VLF/LF** — `doi.org/10.48322/fwte-dv13` (+ `fzcf-f191`), CC0.
- `xfer:mycelium-344-5` **LOD** in den `finals`-Arm.
- `xfer:mycelium-342-3`/`-4`/`-5` · `xfer:mycelium-343-2`/`-3` · `xfer:mycelium-342-10` — wie im Register geführt.

## An river

Origin: mycelium-349/350 (2026-10-15):

- `xfer:mycelium-347-1` **intermagnet-XYZ** — **erfolgreich**: `intermagnet-xyz-cdn` Lauf `38618793419` (`conclusion: success`). River setzt `al_network_proxy`/`gic_onset_detector`.
- `xfer:mycelium-344-4` **DE-2 + `asset=`-Riss** — die Riss-Klärung steht (`harvest-dispatch.yml:106`); der Lauf `38552385568` scheitert **gemessen am Host** (`cdaweb.gsfc.nasa.gov` TLS-Abbruch). `asset=fehlt` bleibt.

ENTSO-E (river-204) und Schumann (river-208) sind am Baum aufgeklärt/erfüllt — s. „Gearbeitet (dieses Rund)"; kein offener Punkt an river.

## An future

Origin: mycelium-349/350 (2026-10-15):

- `xfer:mycelium-347-2` **CN-NSSDC-Planetologie — Datenantrag.** Entwurf liegt in Futures Postfach (`post/in/2026-10-14-mycelium-nssdc-datenantrag.md`); Route **mit Antrag** entschieden. Lizenz **gemessen**: Katalog-Record `CC BY 4.0`, Portal CLPDS `all rights reserved` — ein Riss. **Operator-Hand:** Antrag einreichen.
- `xfer:mycelium-346-8` **CTBTO + WALDO — das Warten blockiert nichts** — Infrasound via NSF SAGE/EarthScope + IMS (CC BY 4.0); VLF/LF via Zenodo-CC-BY-4.0 + AWESOME CC0.
- `xfer:mycelium-346-9` **NSSDC Mariner-10 (`PSCM-00009`)** — Detailseite public; `download.action` = 206/1 B. CRUSO-Antrag bleibt Operator-Hand (Entwurf `state/mail/nssdc-mariner10-pscm-00009.md`).
- `xfer:mycelium-346-10` **Secret-Hygiene-Incident (346) — `NASA_API_KEY` exponiert, behoben** — neuer Key angefordert (2026-10-14); alter Wert als exponiert behandelt.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
