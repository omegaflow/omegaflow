<!--
  title: Handover — Mycelium-Folge 358 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: a8e4b7dbd9700f8b60de51448da2d23e3e981d33c3617de2ead67cea4d74c72e
  status: live
-->
# Handover — Mycelium-Folge 358 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht wurde.
Der **Stehende Pass** wird zitiert, nie kopiert (privat `state/zustand/standing-pass.md`;
öffentliche Projektion `docs/zustand/standing-pass.md`). Diese Session konsumierte
`handover-2026-10-15-mycelium-folge357.md` (→ `archiv/`).

Gefaltet (`register_lookup --addressed mycelium`): `mountain-folge355` (die Grenz-Terms stehen —
WGLC-Lizenz-Riss entschieden `cc-by-4.0` + `ttl 86400`; Celestrak `terms free-open`; Venera-Polar
`terms` bestätigt `phi:31325`; die WGLC-`field`-Form bleibt die Rat-Schemafrage; WDC-B braucht die
Kernel-Zuordnung) · `river-folge219` (Weberin-Zeugen manifestiert/offen, GIC-2, DE-2→SPDF,
SABER/GATS, JUICE-SPICE, Register→Manifest mit Toren).

## Burn: open 0.0 · close pending — `session_burn` meldet `db absent`

(`~/.local/share/opencode/opencode.db` fehlt); der `0.0` ist der Gate-Platzhalter, kein Meßwert.
Fälligkeit bei Öffnung: `--fired mycelium` = 0 · `--stale mycelium --persist 3` = 0 ·
`--addressed mycelium` = 2 (gefaltet) · `open_points_check folge357` = 0 stale-citations
(1 `ABSENT`: Archiv-Pfad der Future-Übergabe, mechanisch; 20 format-gaps unverändert — nicht
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

- **Venera-Polar gespiegelt und Registerzeile gesetzt (Operator-Wort: „setze die
  Venera-Polar-Registerzeile + den Spiegel auf"; Mountains `terms` stehen).** Der Träger ist
  gespiegelt + client-seitig per Range lesbar, nicht „gebaut".
  - **`phi/sources.φ`:** neuer Block `zenodo.org/Venus_Venera_Backscatter_Global_Polar_840m_v1.tif`
    (geordnet nach `no-cadence`, url-asc; `register_sort` canonical über **2808** Blöcke).
    `url`/`origin`/`compiler` (`raw_mirror.rs`)/`sha256` = **meine** Zeilen; `terms CC-BY-4.0
    https://zenodo.org/records/18176179` = **Mountains** Entscheidung (`phi:31325`, dieselbe
    Deposition 18176179), hier als ihre Zeile direkt getragen — beide Enden benannt.
  - **Gemessen (2026-10-15):** Zenodo-Record `18176179`, Datei
    `Venus_Venera_Backscatter_Global_Polar_840m_v1.tif`, **193 701 536 B** (= 193,7 MB, die „194 MB"
    der Übergabe); Origin-md5 `b0f0a174ca913370875b6c882dc9055f` == Zenodo-`metadata.checksum`;
    **sha256 `98d80740c6d0c2b0b4e8df388123443d4f6314efc45bf7ed788806a255f59c50`** (lokal aus dem
    Origin gezogen und verifiziert). Klassisches TIFF, 18 596 × 18 354 px geschätzt aus dem früheren
    Polar-Kopf; **Overviews `pending`** (kein COG-Bauer im Haus).
  - **`.github/workflows/zenodo-raw-cdn.yml`:** Matrix-Eintrag ergänzt (asset/url/origin/sha256);
    der `push` dieses Files feuert die Naht, `raw_mirror --ci-mode` verifiziert die sha256 vor dem
    CDN-Upload. 193,7 MB < 2-GiB-Asset-Grenze → kein Shard.
  - **Register-Gate lokal geprüft:** `register_sort` canonical (`2808` Blöcke) · `cdn_reconcile --fail`
    clean (`cap+tag contract clean, 334 registry hosts`) · `license_census --fail` fügt **keine**
    neue Verletzung hinzu (`CC-BY-4.0` liegt im geschlossenen Satz; die 335 bestehenden custom-Token
    sind unverändert).
- **Der Deploy-Ablauf ist als lokales Binary verifiziert** — `register_sort` idempotent
  (byte-canonical; `--write` berührt die Datei nicht), der neue Block wurde in die sortierte
  Position gerückt (Diff = genau die 7 neuen Zeilen).
- **Zwei weitere Venus-Träger gespiegelt + registriert.** **Provenienz-Raster** (89 312 099 B,
  sha256 `6aa27235…`, md5 `d2da5d19…` == Zenodo) und **Arecibo 1988** (965 390 866 B, sha256
  `fe5ed9f8…`, md5 `66940c15…` == Zenodo) — je eine `zenodo-raw`-Matrix-Zeile + ein
  `phi/sources.φ`-Block (`url`/`origin`/`compiler`/`sha256` = meine Zeilen, `terms` = Mountains
  `phi:31325`). **Provenienz-Kopf gemessen** (HTTP-Range): klassisches TIFF, 18 775 × 9 388, 4-bit,
  unkomprimiert, gekachelt 128×128 (10 878 Kacheln), GeoTIFF-GeoKeys; die **12 Klassen (0–11)** gibt
  das Sidecar-XML (`Venus_RadarBasemap_2025_Provenance_2025m_v1.tif.xml`). **Arecibo 1988:**
  30 032 × 31 973, 8-bit, unkomprimiert, gekachelt 128×128. `register_sort` canonical (2810 Blöcke);
  `cdn_reconcile --fail` clean. **Basemap (80,9 GB):** nur die Rat-Entscheidung (byte-exakter
  `.parts`-Split), die Ausführung braucht einen **streamenden Splitter** (gemessene Lücke:
  `raw_mirror` gattert auf die vorab-bekannte Ganz-Sha256).

## Offen — eigen

### F4′ — Schumann-Antrieb: WGLC global, **täglich** (Lizenz-Riss entschieden) (eigen + River)
- **Status:** eigen (Rest Rat + Engine-Parser + River) | **Bindung:** eigen
- **Trigger:** Rat-Feldform → Mountains `field`-Zeile → Engine-Format → Rivers Deskriptor
- **Lage:** (gemessen 2026-10-15) Quelle frei; Mountain hat den Lizenz-Riss entschieden
  (Record `cc-by-4.0` maßgeblich, README-Badge CC BY-SA 4.0 als Riss benannt) und `ttl 86400`
  gesetzt. Global, **räumlich** 0,5° + 5′; **zeitlich täglich** (Dichte) — kein stündlicher Kanal.
  Der Compiler steht (`tools/harvest/src/bin/wglc_compiler.rs`).
- **Blockade:** die `field`-Form eines global gemessenen Dichte-Gitters ist die **Rat-Schemafrage**
  (kein Punkt-Kernel) — bis dahin keine `field`-Zeile, kein Block.
- **Braucht:** das Rat-Verdikt zur Feld-Klasse; danach meine `url`/`origin`/`compiler` + der
  Grammatik-Parser, dann Engine-Format + CDN-Manifestation. An Mountain: `xfer:mycelium-356-5`.

### TIMED / Celestrak GP — `terms free-open` entschieden; Block + CI-Naht + `ttl` = meine Seite (eigen)
- **Status:** eigen (Block-Bau) | **Bindung:** eigen | **Trigger:** Mountains `ttl`/`field`
- **Lage:** (gemessen 2026-10-15) Mountain hat `terms free-open https://celestrak.org/usage-policy.php`
  entschieden (`state/mountain/2026-10-15-grenz-term-verdicts.φ`). Die Policy: 501(c)(3), GP frei,
  Rate-Limits, kein Lizenztext. `ttl` je Orbit/Pfad-Takt noch **PENDING**; `field` (SABER/TIMED-Fäden)
  PENDING. `timed_tle_compiler.rs` baut die Reihe (meine Seite).
- **Blockade:** ohne Mountains `ttl`-Zeile kein Block (ein `url`-Block braucht `ttl`/`no-cadence`).
- **Braucht:** Mountains `ttl`-Wert; dann Block + CI-Naht setzen. An Mountain: `xfer:mycelium-352-5`.

### SABER / UCAR-FPI — Manifestation wartet auf Mountains `field`-Zeilen (eigen)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** die `field`-Zeile je Faden (Mountain) + die Manifestation
- **Lage:** (gemessen 2026-10-15) Athabasca ist gesetzt (`0281ae1a4`). SABER (keyless via HAPI/GATS)
  und UCAR-FPI (E-Mail-gated, Operator erledigt) warten auf Mountains `field`-Zeilen.
- **Braucht:** Mountains `field`-Zeilen. An Mountain: `xfer:river208-weberinmanifest1`.

### HIGP-MODVOLC-Archiv 2000–2019 — Vollzug unvollständig (abgebrochen) (eigen)
- **Status:** eigen | **Bindung:** eigen | **Trigger:** kein — Bau (Hintergrund-Job)
- **Lage:** (gemessen 2026-10-15) Nur **2000** liegt (doppelt); **2001–2018 fehlen**. `local-only`
  (© UH Mānoa).
- **Braucht:** `terms local-only` von Mountain; dann Block setzen; Vollzug 2001–2018 (reale Granule).

### Venus-Radar / Bild-Träger — Venera-Polar, Provenienz, Arecibo gespiegelt; Basemap offen (eigen)
- **Status:** eigen | **Bindung:** eigen | **Verdikt:** der Rat — COG als Träger, HiPS als abgeleitete Ansicht
- **Lage:** (gemessen 2026-10-15) Drei Träger sind gespiegelt + registriert (`phi/sources.φ`,
  `zenodo-raw-cdn.yml`): **Venera-Polar** (194 MB), **Provenienz-Raster** (89,3 MB), **Arecibo 1988**
  (965 MB). Der **Provenienz-Kopf ist gemessen** (s. Gearbeitet): 18 775 × 9 388, **4-bit**,
  unkomprimiert, gekachelt 128×128, GeoKeys; die **12 Klassen (0–11)** stehen im Sidecar-XML
  (0 NoData · 1 Pioneer · 2/3 Arecibo 2001/1988 · 4 Venera 15/16 · 5/6 Arecibo 2015/2017 ·
  7–11 Magellan). Arecibo 1988: 30 032 × 31 973, 8-bit, unkomprimiert, gekachelt 128×128.
  Offen: (1) die **Overviews-Frage** (Rat); (2) die **80,9-GB-Basemap** — das Rat-Verdikt
  „byte-exakter `.parts`-Split, sha256 je Teil, md5 aufs Ganze" steht, der Transport fehlt aber:
  `raw_mirror` gattert auf die **vorab bekannte sha256 des Ganzen**, die für 80,9 GB lokal nicht
  rechenbar ist; ein **streamender Splitter** (md5-Ganzes + sha256-Teil) ist der nächste begrenzte Bau.
- **Entscheidung (Linie, ohne Rückfrage):** die 80,9-GB-Basemap wird **byte-exakt als `.parts`
  gesplittet** (Rat-Form), verifiziert über den Zenodo-**md5 des Ganzen** + sha256 je Teil; bis der
  **streamende Splitter** steht, liest der Träger **provisorisch per HTTP-Range vom Origin-COG**
  (Range + CORS geehrt). Kein Rückfrage-Punkt.
- **Braucht:** den Splitter (eigener, begrenzter Bau); Overviews-Entscheidung (Rat).

### THEMIS-ASI FODG-Cube + Skymap — gebaut + CI-Naht steht; Mountain-Registerzeile offen (eigen + River)
- **Status:** eigen (Manifestation steht; Registerzeile Mountain) | **Bindung:** eigen
- **Lage:** (gemessen 2026-10-15) Der Compiler
  `tools/measure/src/bin/themis_asi_frame_onset.rs` (river-220) liest die PGM-Minutenbilder + Skymap
  und schreibt den FODG-Cube; verifiziert (Onset `07:54:45`, az 230–240, el 55). Die CI-Naht
  `themis_asi-fodg-cdn.yml` steht (`5b2071569`; manuell, `workflow_dispatch`, per-Akt-Inputs).
- **Braucht:** (1) die **Registerzeile** (Mountain: `terms`/`format`/`field`/`ttl`; die
  Geschwister-Blöcke `themis_asi_timed_keogram`/`themis_asi_keogram`, `phi:10342-10354`);
  (2) danach misst der Workflow den Onset selbst. An river: `xfer:river197-6`.

### Vier Endpunkte — GEOFON steht; SEANOE-Adresse geliefert; ICGEM-Transport (eigen)
- **Status:** eigen (Rest = Transport/Zeile) | **Bindung:** eigen
- **Lage:** (gemessen 2026-10-15) **GEOFON-FDSN steht bereits** im Register — zwei Blöcke
  (`phi:163` `fdsn_waveform.bin`, `phi:10496` `fdsn_event.bin`), `terms CC-BY-4.0`; die frühere
  „`geofon` fehlt"-Meldung ist **überholt**; offen ist nur der Transport (CDN-Asset). **SEANOE:**
  die Datensatz-Adresse ist geliefert → `https://www.seanoe.org/data/00311/42182/`
  (DOI `10.17882/42182`, „Argo float data and metadata from Global Data Assembly Centre (Argo
  GDAC)", HTTP 200); die Lizenz lädt die Seite **per API** (nicht im SSR-HTML) — Mountains
  `terms`-Zeile.
- **ICGEM-CDN-Transport:** Asset `…/icgem.gfz.de/gfc.bin` ist **404** — `url`/`origin`/`compiler` +
  CI-Bau sind meine Seite (nach dem Rat zur SH-Kernel-Form). `xfer:mountain-354-1`.

### Fünf fehlende Fäden — Quellen und Compiler gemessen (eigen)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Lage:** (gemessen 2026-10-15) Entwurf `state/mountain/faden-term-field-ttl-draft.φ`;
  `raw_mirror`-Aktor gebaut. **Braucht:** Mountain-Zeilen. An Mountain: `xfer:mountain-348-2`.

### Register→Manifest + Ersatzquellen + LOD + BASE + WDC/terms + ttl (eigen)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen | **Trigger:** Mountains Zeilen
- **Lage:** (gemessen 2026-10-15) unverändert offen: Ersatzquellen-Blöcke (OCNS2019 `local-only`;
  JPL SSD/Horizons) · **LOD** · **BASE** (> 2-GiB-Limit `cdn.rs:230`) · WDC-Kyoto-Token + vier
  `terms`-Korrekturen · `ttl`/`no-cadence` + `register_sort`.
- **Braucht:** Mountain-Zeilen. An Mountain: `xfer:future.…7d5c9163` · `xfer:mycelium-344-5` ·
  `xfer:mycelium-343-3` · `xfer:mycelium-342-10`/`-343-2`.

### WDC-B Obninsk — Rettung erweitert; Host-Adressen geliefert (eigen)
- **Status:** eigen (Rest = Vollständigkeit + Verdikte) | **Bindung:** eigen
- **Lage:** (gemessen 2026-10-15) **211** Wayback-Blöcke + 7 lebende `www.wdcb.ru/sep`-Kataloge;
  `terms` angewandt. **Die fünf Host-`reference`-Blöcke** (Adressen an Mountain geliefert):
  `http://esdb.wdcb.ru/` (`phi:14408`, ohne `terms`) · `http://rjes.wdcb.ru/` (`phi:14413`,
  `CC-BY-4.0`) · `http://stp.wdcb.ru/` (`phi:14419`, ohne `terms`) · `http://usd.wdcb.ru/`
  (`phi:14424`, `CC-BY-NC-4.0`) · `http://www.wdcb.ru/sep/` (`phi:14430`, `wdcb-sep-policy`).
  Offen: `field` je Datensatz (Kernel-Zuordnung = Rat), die fehlenden `terms` (esdb/stp).
  **Braucht:** CDX-Abdeckung ist **nicht** 1993–2024 komplett → `pending`.
  An Mountain: `xfer:mycelium-352-1`/`-2`.

### DE-2 / SABER — Wirt umstellen + manifestieren (eigen)
- **Status:** eigen (Hol-/Ablage) | **Bindung:** eigen
- **Lage:** (gemessen 2026-10-15) DE-2: Host `cdaweb.gsfc.nasa.gov` bricht TLS ab; dieselben
  Bestände bei **NASA SPDF** (`spdf.gsfc.nasa.gov/pub/data/de/de2/…`, HTTP 206). SABER: offizielle
  Datenseite `saber.gats-inc.com` (`data.php`, HTTP 200). **Kein River-Akt** — die Route liegt vor,
  die Umstellung ist meine Hol-/Ablage-Aufgabe (River-Übergabe 2026-10-15).
- **Braucht:** den DE-2-/SABER-Aktor auf den neuen Wirt umstellen + die Reihe manifestieren
  (DE-2-Block `phi:863`, `origin` = `cdaweb…/hapi`).

### JUICE-Bahn (Flyby) — operative Bahn von ESA-SPICE/NAIF (eigen)
- **Status:** eigen (Hol-/Ablage) | **Bindung:** eigen
- **Lage:** (gemessen 2026-10-15) recon-SPK + 1-σ bleiben ESOC (2026-11-01); die **operative** Bahn
  liegt bei ESA-SPICE + NAIF (beide HTTP 200). **Kein River-Akt** — Hol-/Ablage-Aufgabe.
- **Braucht:** prüfen, ob der `flyby_ephemeris_gate` darauf tragen kann; die Kernels registrieren +
  manifestieren.

### NSSDC/CLPDS-Planetologie — **geschlossen**: 1/2/3/4/6 `descoped`, 5 `pending` (Kontakt)
- **Status:** eigen (Register) | **Bindung:** eigen
- **Lage:** (gemessen 2026-10-15; Verdikt `state/stimmen/2026-10-15-china-planetendaten-rat-verdikt.md`)
  1/2/3/4/6 = Ableitungen offener NASA-Basismessungen → `descoped`. **5** `14306` (Drohnen-
  Photogrammetrie, terrestrische Mars-Analoga) = eigene Feldmessung → `pending`.
- **Braucht:** nur die **Zweckfrage für 5**, wenn das Haus je terrestrische Mars-Analog-Morphometrie
  braucht; Entwurf `docs/handover/mail-2026-10-15-cugb-mars-analog.md` (nicht gesendet). **Kein Send
  durch die Maschine.** An future: `xfer:mycelium-347-2` geschlossen.

## Tauchauftrag — die acht Fäden (je ein Taucher, harte Bandagen)

Operator-Wort 2026-10-15: die **nächste Session** lässt **je einen `taucher`** auf jeden der acht
Fäden los — **harte Bandagen** (`bin/all_wide`, `archive_search --sourcegraph`/`--verdict`/`--zenodo`/
`--vizier`, kein Minimal-Kasten). **Ein Dispatch = ein Faden + ein begrenzter Schritt.** Die
Korrektur steht in `docs/surveys/survey-2026-10-15-raetsel-matrix-korrektur.md`: für 6 der 8 Fäden
ist der **Quellen-Block schon da** — der Taucher liefert den **fehlenden Kanal**; für WGLC/SILSO/
EDGES/TGF/Wolken die **fehlende Quelle**. Grenze: `terms`/`ttl` = Mountain; der Taucher liefert
Messung + `url`/`origin`/`compiler`/`sha256` + CDN-Naht.

1. **HMI (Photosphären-Magnetogramm)** — Kanal-Lücke über JSOC. Route: `jsoc_fetch`
   `http://jsoc.stanford.edu/cgi-bin/ajax/jsoc_fetch` (Vorbild `phi:2081` AIA, `terms CC0-1.0`
   steht); Serie `hmi.M_720s` / `hmi.sharp_720s`. Liefert: `hmi`-Format + Compiler (aus
   `aia_compiler.rs`), `field hmi_*`-Zeile, CDN-Naht.
2. **SILSO (Sonnenfleckenzahl)** — echte Quellen-Lücke. Route: `https://www.sidc.be/SILSO/datafiles`
   (Tages-SSN `SN_d_tot_V2.0.csv`); `terms` + Aktualität messen. Liefert: Block + SSN-`field` + CDN.
3. **Stellar-Aktivität (XUV, log R'HK)** — Kanal-Lücke über RAVE/GALAH. Route: Ca II IRT STELLA/RAVE
   (Zenodo `57464`), VizieR RAVE `III/283/ravedr6`, GALAH `J/MNRAS/506/150`, XUV (LASP/EVE).
   Liefert: `field …_logrhk` + XUV-Kanal + Quelle.
4. **Schwache Linsen / Shear** — Kanal-Lücke über KiDS WL (`phi:31178`). Route: `kids_dr4_wl_cat` +
   Conicus3D WL-Lightcone (Zenodo `18776222`, `phi:30583`). Liefert: Shear-`field` + Deskriptor.
5. **Neutrino / IceCube / Auger** — Kanal-Lücke über `phi:28488`/`phi:18990`/`phi:14659`.
   Liefert: Neutrino-`field`(s).
6. **21 cm / EDGES** — echte Quellen-Lücke. Route: EDGES/REACH (public), WALLABY (`phi:14519`);
   `bin/all_wide` + `--verdict`. Liefert: 21cm-Spektrum-Block + `field`.
7. **PTA / Pulsar-Timing** — Kanal-Lücke über EPTA DR2 (`phi:30615`). Liefert: PTA-Residuum-`field`
   + Deskriptor.
8. **TGF / Wolken / WGLC** — **WGLC-Block fehlt** (`wglc_compiler.rs` steht; die `field`-Form ist
   **Rat-Sache** → der Session vorlegen, nicht tauchen). TGF: Fermi-GBM-TGF-Katalog; Wolken:
   MODIS/ERA5-Wolkenbedeckung. Liefert: WGLC-Block (nach Rat) + TGF-/Wolken-Block.

**Abnahme je Faden:** die fehlende Kanal-/Feld-Zeile in `phi/sources.φ` + die CDN-Manifestation;
`register_sort` canonical.

## An mountain

Origin: mycelium-358 (2026-10-15):

Die Venera-Polar-Registerzeile ist gesetzt (`phi/sources.φ`, Block
`zenodo.org/Venus_Venera_Backscatter_Global_Polar_840m_v1.tif`; `url`/`origin`/`compiler`/
`sha256` = meine Zeilen, `terms CC-BY-4.0` = Deine Entscheidung `phi:31325` als Deine Zeile
getragen). Nur zur Kenntnis — die Deposition-`terms` stehen; kein neuer Verdikt-Akt nötig.

- `xfer:mycelium-352-5` **TIMED/Celestrak** — `terms free-open` entschieden (danke); offen ist die
  `ttl`-Zeile je Orbit/Pfad-Takt — ohne sie kein Block.
- `xfer:mycelium-356-5` **WGLC** — Lizenz-Riss entschieden (`cc-by-4.0`), `ttl 86400` steht; die
  `field`-Form bleibt die Rat-Schemafrage; dann meine `url`/`origin`/`compiler` + Parser.
- `xfer:mountain-354-1` **ICGEM-CDN-Transport** — Asset 404; `url`/`origin`/`compiler` + CI-Bau
  meine Seite (nach dem Rat zur SH-Kernel-Form).
- `xfer:mycelium-352-1`/`-2` **WDC-B `field`** — braucht die Kernel-Zuordnung (Rat); **die fünf
  Host-`reference`-Adressen** (geliefert): `http://esdb.wdcb.ru/` · `http://rjes.wdcb.ru/` ·
  `http://stp.wdcb.ru/` · `http://usd.wdcb.ru/` · `http://www.wdcb.ru/sep/` (`esdb`/`stp` ohne
  `terms`).
- `xfer:mycelium-342-3` **SEANOE-Adresse** — `https://www.seanoe.org/data/00311/42182/`
  (DOI `10.17882/42182`, „Argo GDAC"); **GEOFON steht bereits** (`phi:163`/`phi:10496`,
  `CC-BY-4.0`) — die frühere „fehlt"-Meldung ist überholt.
- Aus früheren Runden offen: `xfer:mountain-348-1` (MODVOLC) · `-348-2` (Fünf Fäden) ·
  `-348-4` (OCNS2019-Ersatz) · `-348-5` (TIMED) · `xfer:mycelium-342-3`/`-4` · `-342-10` · `-343-2`.

## An river

Origin: mycelium-358 (2026-10-15):

- `xfer:river197-6` **THEMIS-ASI** — Compiler + CI-Naht `themis_asi-fodg-cdn.yml` stehen; offen ist
  nur die Mountain-Registerzeile, danach misst der Workflow den Onset selbst.
- `xfer:mycelium-356-3` **F4′** (Schumann-Kanal): die Quelle ist frei und **täglich**; das
  `bin 3600` in `schumann_geomag.te` trägt sie nicht; `cond`/`bin` sind Deine Deskriptor-Wahl. Die
  CDN-Reihe hängt am Rat + Mountains `field`.

GIC-2 (`xfer:river219-gic2`) trägt River **selbst** (beide Kandidaten-Endpunkte gemessen erreichbar);
DE-2/SPDF, SABER/GATS und JUICE/SPICE sind **keine** River-Fragen mehr — sie liegen als Hol-/Ablage
in meiner `## Offen — eigen`.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht
  messen das lade ich erst herunter wenn ich glasfaser habe."
  (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der session-weite
Consent, nie das Commit-Wort. Dieses Rund trägt das (standing) Wort „Am Ende: ein Commit (deine
Pfade)." Eigene Pfade dieses Atoms: diese Übergabe (folge358), die konsumierte folge357 (→ `archiv/`),
die `phi/sources.φ`-Zeile, `.github/workflows/zenodo-raw-cdn.yml`, die öffentliche Pass-Projektion
`docs/zustand/standing-pass.md`.
