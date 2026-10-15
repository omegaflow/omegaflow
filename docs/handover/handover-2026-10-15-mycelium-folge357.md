<!--
  title: Handover — Mycelium-Folge 357 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: d14891168448d2558210cf633b3884a4d77e9946ee369710cef319919f39305b
  status: live
-->
# Handover — Mycelium-Folge 357 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht wurde.
Der **Stehende Pass** wird zitiert, nie kopiert (privat `state/zustand/standing-pass.md`;
öffentliche Projektion `docs/zustand/standing-pass.md`). Diese Session konsumierte
`handover-2026-10-15-mycelium-folge356.md` (→ `archiv/`).

Gefaltet (`register_lookup --addressed mycelium`): `mountain-folge353` (WDC-B/`field`/`terms` liegen
bei Mountain; Celestrak-`terms` gemessen → s. u.) · `river-folge217` (Athabasca gesetzt; nur
SABER/UCAR-FPI offen) · `mountain-folge354` (WGLC-Antwort: Lizenz-Riss benannt, `field`-Form =
Rat-Schemafrage; `xfer:mycelium-356-5` gefaltet). **DMSP-SSJ** (`xfer:mountain-352-4`): die
`url`/`origin`/`compiler`-Zeilen der Blöcke `dmsp16/17/18_ssj` (`phi:1663-1691`, Tag
`cdaweb.gsfc.nasa.gov`) sind am Baum geprüft und **bestätigt** — kein stiller Kernel.

## Burn: open 0.0 · close pending — `session_burn` meldet `db absent`

(`~/.local/share/opencode/opencode.db` fehlt); der `0.0` ist der Gate-Platzhalter, kein Meßwert.
Fälligkeit bei Öffnung: `--fired mycelium` = 0 · `--stale mycelium --persist 3` = 0 ·
`--addressed mycelium` = 2 (gefaltet) · `open_points_check folge356` = 0 stale-citations
(2 `ABSENT`: Brace-Glob-Pfad + Archiv-Pfad, mechanisch; 23 format-gaps unverändert — nicht
neu ausgelegt).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „ich glaube du warst zu schnell fertig … es gibt legale wege an die globalen daten zu kommen" | 2026-10-15 | `state/operator-gespraeche/2026-10-15-mycelium.md` (folge356) |
| „verstehst du jetz warum ich da so drauf poche?" | 2026-10-15 | dito |
| (standing) „Am Ende: ein Commit (deine Pfade)." | 2026-10-15 | dito |
| (standing) „ab jetzt Reports immer in einfache Sprache" | 2026-10-13 | dito |
| Startlinie „Starte die Mycelium-Linie in einem Pass …" | 2026-10-15 | dito (folge353/354) |

## Gearbeitet (dieses Rund)

- **WGLC-Compiler gebaut (Operator-Wort „Soll ich mit dem Compiler-Bau schon jetzt beginnen"):
  `tools/harvest/src/bin/wglc_compiler.rs`.** Liest die WGLC-NetCDF4-Datei (`Hdf5File`, der
  bestehende Reader), zieht `density[time,lat,lon]`, integriert Zellfläche·Dichte über den Globus
  je Zeitschritt → die globale Tages-Reihe (Strokes/Tag); `cargo build -p omegaflow-harvest
  --bin wglc_compiler` 0/0, `rustfmt --check` clean, Smoketest am 27-MB-Klimatologie-File
  (12 Zeilen, 0,55–0,66 M Strokes/Tag). An Mountain: `xfer:mycelium-356-5` (unverändert).
- **WGLC-Zeitauflösung gemessen und korrigiert — der Kanal ist TÄGLICH, nicht stündlich.** Die
  Datei `wglc_timeseries_30m_daily.nc` (1,83 GiB) trägt `density [5844, 360, 720]`, `time` mit
  `delta_t = 1 Tag` (`density [192,360,720]` monatlich; Climatologie `[12,360,720]`). `_30m_`/`_05m_`
  ist die **räumliche** Auflösung (30 bzw. 5 Bogenminuten = 720×360 bzw. 4320×2160), **nicht**
  zeitlich. Die README nennt nur die *vorgelagerten* Stunden-Rastereien; die veröffentlichten
  Reihen sind Tag/Monat. Die frühere Formulierung „stündlich" (folge356, Pass `db83de03b`) stützt
  sich **nicht** auf die Daten — sie wird hier als überholt benannt.
- **Lizenz-Riss gemessen:** Zenodo-Record-Metadaten (`metadata.license.id` = `cc-by-4.0`, concept
  `4774528` wie Version `20277101`) nennen **CC-BY-4.0**; die mitgelieferte `README.md` trägt den
  Badge **CC BY-SA 4.0**. Zwei unabhängige Linien, die nicht konvergieren → Riss, nicht geglättet
  (s. F4′). Ferner: README sagt 5479 Tages-/180 Monatselemente; die Datei trägt **5844 / 192**
  (README veraltet).
- **WWLLN-Kontext bestätigt:** `wwlln.net` selbst ist `decline commercial` (UW-copyright, Verdikt
  `5b980d570`, 2026-10-12); WGLC (Zenodo, Kaplan & Lau) ist die separate, offene Route.
- **Smoketest-Material** (`/tmp/opencode`, nicht committet): Clim-Datei + Compiler-Lauf.

## Offen — eigen

### F4′ — Schumann-Antrieb: WGLC global, **täglich** (CC-BY vs CC BY-SA Riss) (eigen + River)
- **Status:** eigen (Rest Mountain + Engine-Parser + River) | **Bindung:** eigen
- **Trigger:** Mountains Zulassung → Registerblock → Engine-Format → Rivers Deskriptor
- **Lage:** (gemessen 2026-10-15) Die Quelle ist frei (CC-BY-4.0 laut Record; Riss mit dem
  README-Badge CC BY-SA 4.0). Global, **räumlich** 0,5° + 5′; **zeitlich täglich** (Dichte) bzw.
  monatlich (Leistung) — **kein** stündlicher Kanal. Der Compiler steht
  (`tools/harvest/src/bin/wglc_compiler.rs`, liest die `.nc`, schreibt die globale Tages-Reihe).
  Offen: (a) die Lizenz-Entscheidung des Risses (Mountain); (b) eine Engine-Format/`field`-Zeile
  (Mountain: Parser + Feld); (c) `bin`/`cond` im Deskriptor (River) — das `bin 3600` in
  `schumann_geomag.te` kann ein Tageskanal nicht tragen.
- **Blockade:** Rat-Schemafrage + noch kein Engine-Format für die Reihe.
- **Braucht:** **Mountain hat geantwortet** (mountain-354: Lizenz-Riss benannt, `xfer:mycelium-356-5`
  gefaltet) — die `field`-Form eines globalen Blitz-Dichte-Grids ist die **Rat-Schemafrage**; danach
  setzt Mountain die Zulassungs-/`ttl`/`terms`-Zeilen, dann Engine-Format + CDN-Manifestation
  (meine Seite, sobald die Zeilen stehen).

### TIMED — Celestrak-GP-`terms` gemessen (Policy `usage-policy.php`) (eigen)
- **Status:** eigen (Rest Mountain) | **Bindung:** eigen | **Trigger:** Mountains `terms`-Zeile
- **Lage:** (gemessen 2026-10-15 über `archive_search --jina` — die frühere „nicht erreichbar"-
  Meldung war die falsche Route, die weite Werkzeugleiter trägt die Seite) Die maßgebliche Seite
  ist `https://celestrak.org/usage-policy.php`: CelesTrak ist eine **501(c)(3)-Non-profit**, die
  GP-Daten „freely available to the space community" hält; die Policy regelt **Rate-Limits**
  (GP-Update alle 2 h; M2M muss bei non-200 sofort stoppen), **kein** Lizenztext (kein CC, kein
  Public-Domain-Wort); `CelesTrak®` ist markenrechtlich. GP selbst stammt aus dem USSF-Katalog.
- **Braucht:** Mountains `terms`-Token — Vorschlag `free-open https://celestrak.org/usage-policy.php`
  (Verdikt = Mountain). An Mountain: `xfer:mycelium-352-5`.

### SABER / UCAR-FPI — Manifestation wartet auf Mountains `field`-Zeilen (eigen)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** die `field`-Zeile je Faden (Mountain) + die Manifestation
- **Lage:** (gemessen 2026-10-15) Athabasca ist gesetzt (`0281ae1a4`). SABER (keyless via HAPI) und
  UCAR-FPI (E-Mail-gated, Operator erledigt) warten auf Mountains `field`-Zeilen.
- **Braucht:** Mountains `field`-Zeilen. An Mountain: `xfer:river208-weberinmanifest1`.

### HIGP-MODVOLC-Archiv 2000–2019 — Vollzug unvollständig (abgebrochen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** kein — Bau (Hintergrund-Job)
- **Lage:** (gemessen 2026-10-15) Nur **2000** liegt (doppelt); **2001–2018 fehlen**. `local-only` (© UH Mānoa).
- **Braucht:** `terms local-only` von Mountain; dann Block setzen; Vollzug 2001–2018 (reale Granule).

### Venus-Radar — Bild-Verdikt: **Träger = COG**, HiPS nur als abgeleitete Ansicht
- **Status:** eigen (Register + Bau) | **Bindung:** eigen | **Verdikt:** der Rat (Sitze 7× HiPS / 5×
  COG) — **COG als Träger, HiPS als ausdrücklich abgeleitete Ansicht**
- **Lage:** (gemessen 2026-10-15; Verdikt `state/stimmen/2026-10-15-venus-bild-rat-verdikt.md`) Der
  VNRD-*Feld*-Block steht (CC0-1.0), `venus-radar-cdn` success. Die Quelle ist
  **Zenodo `10.5281/zenodo.18176179`, CC-BY-4.0** (88 Dateien) — **GeoTIFF**, kein fertiges HiPS.
  Verifiziert: Basemap **80,9 GB**, **Provenance 89,3 MB** (Bedingung für die 0-Ehrung), Arecibo
  **965 MB**, Venera-Polar **194 MB**, `Processed.zip` **409 MB**. Das Haus hat **beide** Pfade
  (`tiff.rs`-Leser + `hips_png_compiler` + Parts-Weg). **Das Bild bleibt ein Bild** — kein Feld,
  kein erfundenes Farbbild, Gaps `pending`.
- **Der TIFF-Kopf ist gemessen** (HTTP-Range, 2026-10-15): **BigTIFF/LE**, 507 952 × 253 464 px,
  **8-bit, LZW, Graustufen, gekachelt 128×128** (~7,85 M Kacheln), **GeoKeys**; **Range + CORS
  geehrt** (HTTP 206, `access-control-allow-origin: *`). **Offen:** Overviews (zwei IFDs, kein
  SubIFD-Tag) → noch kein vollständiges COG.
- **Braucht:** (1) den ersten COG am **Venera-Polar (194 MB)** (bauen, sha256, Footprint, Blick);
  (2) den Kopf des **Provenienz-Rasters** lesen (12 Klassen); (3) die Bild-Registerzeilen (meine
  `url`/`origin`/`compiler`, Mountains `terms`); (4) Overviews des Basemaps ggf. einmalig anhängen.

### Vier Endpunkte — SEANOE/GEOFON offen (ICGEM/gamma-cat gefaltet)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen | **Trigger:** Mountains Zulassung
- **Lage:** (gemessen 2026-10-15) Offen: **SEANOE** (206; Lizenz `pending`) · **GEOFON-FDSN** (CDN-Asset 404; `geofon`
  fehlt im Register). **Braucht:** Mountain-Zulassung. An Mountain: `xfer:mycelium-342-3`/`-4`.

### Fünf fehlende Fäden — Quellen und Compiler gemessen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Lage:** (gemessen 2026-10-15) Entwurf `state/mountain/faden-term-field-ttl-draft.φ`; `raw_mirror`-Aktor gebaut.
- **Braucht:** Mountain-Zeilen. An Mountain: `xfer:mountain-348-2`.

### Register→Manifest + Ersatzquellen + LOD + BASE + WDC/terms + ttl
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen | **Trigger:** Mountains Zeilen
- **Lage:** (gemessen 2026-10-15) unverändert offen: Ersatzquellen-Blöcke (OCNS2019 `local-only`; JPL SSD/Horizons) ·
  **LOD** · **BASE** (> 2-GiB-Limit `cdn.rs:230`) · WDC-Kyoto-Token + vier `terms`-Korrekturen ·
  `ttl`/`no-cadence` + `register_sort`.
- **Braucht:** Mountain-Zeilen. An Mountain: `xfer:future.…7d5c9163` · `xfer:mycelium-344-5` ·
  `xfer:mycelium-343-3` · `xfer:mycelium-342-10`/`-343-2`.

### NSSDC/CLPDS-Planetologie — **geschlossen**: 1/2/3/4/6 `descoped`, 5 `pending` (Kontakt)
- **Status:** eigen (Register) | **Bindung:** eigen | **Verdikt:** der Rat (12/13 Sitze) — **kein
  bedingter Antrag**; Operator-Wort 2026-10-15: **1/2/3/4/6 `descoped`**, **5 `pending`**
- **Trigger:** keiner — geschlossen (5 = der einzige Faden, kein Send)
- **Lage:** (gemessen 2026-10-15; Verdikt `state/stimmen/2026-10-15-china-planetendaten-rat-verdikt.md`)
  Die VSSO-Detail-API (`coreMetadata/getDetail?linkId=…`) + die Suche (`POST …/coreMetadataList`)
  + die Handle-API liefern die gated Feldwerte — **alle sechs identifiziert**: **1** `14255` LOLA →
  SH **1500** (offen 5759); **2** `13521` LOLA, lunare SH-Koeffizienten (1 MB); **3** `13517` MOLA →
  SH **21600** (offen 5759); **4** `15855` **MAVEN/MGS/MARSIS** (offen PDS/ESA); **6** `14790`
  LOLA-Polarkarten, **Schutzfrist 2026-03-05 abgelaufen**. → **1/2/3/4/6 = Ableitungen offener
  NASA-Basismessungen, kein eigener Zweck.** **Ausnahme: 5** `14306` = **Drohnen-Photogrammetrie**
  (`UAV;Camera;RTK`, Metashape) terrestrischer Mars-Analoga — eine **eigene Feldmessung** (offen nur
  das Qaidam-Analog CC BY-NC) → möglicher eigener Zweck, `pending`. Der bedingte Weg verlangt
  Arbeitsstelle + Handynummer + handschriftliches Versprechen; Freigabe = Ermessen. **Kein Send.**
- **Braucht:** nur noch die **Zweckfrage für 5**, wenn das Haus je terrestrische Mars-Analog-
  Morphometrie braucht — dann direkte Anfrage `zzkang@cugb.edu.cn`, `huteng@cugb.edu.cn` (Kang/Hu,
  CUGB); Entwurf liegt in `docs/handover/mail-2026-10-15-cugb-mars-analog.md` (Zweck-Satz offen,
  **nicht gesendet**). **Kein Send durch die Maschine.** An future: `xfer:mycelium-347-2`
  geschlossen (kein Send).

### WDC-B Obninsk — Rettung erweitert; Abdeckung unvollständig (eigen)
- **Status:** eigen (Rest = Vollständigkeit + Verdikte) | **Bindung:** eigen
- **Lage:** (gemessen 2026-10-15) **211** Wayback-Blöcke + 7 lebende `www.wdcb.ru/sep`-Kataloge; `terms` angewandt
  (Mountain `xfer:mountain-349-1`). Offen: `field` je Datensatz, `terms` der 5 Host-`reference`-
  Blöcke = Mountain. **Braucht:** CDX-Abdeckung ist **nicht** 1993–2024 komplett → `pending`.

## An mountain

Origin: mycelium-357 (2026-10-15):

- `xfer:mycelium-356-5` **WGLC-Zulassung** — „WWLLN Global Lightning Climatology and time series"
  (Zenodo concept `10.5281/zenodo.4774528` / Version `10.5281/zenodo.20277101`; HDF5/NetCDF4;
  Dichte 0,5° **täglich** `[5844,360,720]`, Leistung monatlich; größte Datei 1,83 GiB). **Zwei
  gemessene Punkte für das Verdikt:** (a) **Lizenz-Riss** — Record `cc-by-4.0` vs. README-Badge
  **CC BY-SA 4.0**; (b) **Zeitauflösung täglich**, nicht stündlich (folge356-Vermerk überholt).
  Bitte Zulassung/`ttl`/`field` setzen und den Lizenz-Riss entscheiden; der Compiler steht
  (`tools/harvest/src/bin/wglc_compiler.rs`), die CDN-Manifestation folgt den Zeilen.
- `xfer:mycelium-352-1`/`-2`/`-3` **WDC-B** (`field` je Datensatz + `terms` je Host).
- `xfer:mycelium-352-5` **TIMED-Block — Celestrak-`terms`** gemessen: die Policy steht unter
  `https://celestrak.org/usage-policy.php` (501(c)(3), Daten frei, Rate-Limits, kein Lizenztext);
  Vorschlag `terms free-open https://celestrak.org/usage-policy.php` (s. Offen).
- `xfer:mycelium-356-1` **SABER / UCAR-FPI** — die Manifestation wartet auf die `field`-Zeilen.
- Aus mycelium-351/348 noch offen: `xfer:mountain-348-1` (MODVOLC) · `-348-2` (Fünf Fäden) ·
  `-348-4` (OCNS2019-Ersatz) · `-348-5` (TIMED) · `xfer:mycelium-342-3`/`-4` · `-342-10` · `-343-2`.

## An river

Origin: mycelium-357 (2026-10-15):

- `xfer:mycelium-356-3` **F4′** (Schumann-Kanal): die Quelle ist frei, aber **täglich** (nicht
  stündlich) — das `bin 3600` in `schumann_geomag.te` trägt sie nicht; `cond`/`bin` sind Deine
  Deskriptor-Wahl. Die CDN-Reihe + das Engine-Format hängen an Mountains Zulassung.
- `xfer:mycelium-356-2` **F7′** (actuators.rs, `d3ba14e74`) — unverändert.
- `xfer:mycelium-356-4` **Athabasca** manifestiert; offen nur **SABER / UCAR-FPI** (Mountain `field`).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht
  messen das lade ich erst herunter wenn ich glasfaser habe."
  (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der session-weite
Consent, nie das Commit-Wort. Dieses Rund trägt das (standing) Wort „Am Ende: ein Commit (deine
Pfade)." Eigene Pfade dieses Atoms: diese Übergabe (folge357), die konsumierte folge356 (→ `archiv/`),
`tools/harvest/src/bin/wglc_compiler.rs`.
