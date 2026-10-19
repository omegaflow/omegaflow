<!--
  title: Handover — Mycelium-Folge 359 (2026-10-15)
  class: handover
  date: 2026-10-15
  sha256: 890592f0a722d5069613adb9d6d950f89e7abc434d5f2692219bd5ed7414e015
  status: live
-->
# Handover — Mycelium-Folge 359 (2026-10-15)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht wurde.
Der **Stehende Pass** wird zitiert, nie kopiert (privat `state/standing-pass.md`;
öffentliche Projektion `docs/zustand/standing-pass.md`). Diese Session konsumierte
`handover-2026-10-15-mycelium-folge358.md` (→ `archiv/`).

Gefaltet (`register_lookup --addressed mycelium`): `mountain-folge356` (die fünf angefragten
Linien: TIMED frei, SABER-`field` liegen, THEMIS-ASI `ttl 604800`, GEOFON steht, SEANOE ohne
URL; die zwei `local-only`-Blöcke **nicht** setzen; WDC-B braucht die Kernel-Zuordnung) ·
`river-folge221` (F4′-Echo: Tages-Deskriptor `schumann_geomag_blitz.te` `bin 86400` steht;
THEMIS-ASI: FODG-Cube noch nicht im CDN, offen ist nur die Mountain-Registerzeile).

## Burn: open 0.0 · close pending — `session_burn` meldet `db absent`

(`~/.local/share/opencode/opencode.db` fehlt); der `0.0` ist der Gate-Platzhalter, kein Meßwert.
Fälligkeit bei Öffnung: `--fired mycelium` 0 · `--stale mycelium --persist 3` 0 · `--addressed
mycelium` 2 (gefaltet) · `open_points_check folge358` = 0 stale-citations (3 `ABSENT`: zwei
`phi`-Pfad-Refs + der Archiv-Pfad, mechanisch; 25 format-gaps unverändert — nicht neu ausgelegt).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „ich glaube du warst zu schnell fertig … es gibt legale wege an die globalen daten zu kommen" | 2026-10-15 | `state/operator-gespraeche/2026-10-15-mycelium.md` (folge356) |
| „verstehst du jetz warum ich da so drauf poche?" | 2026-10-15 | dito |
| (standing) „Am Ende: ein Commit (deine Pfade)." | 2026-10-15 | dito |
| (standing) „ab jetzt Reports immer in einfache Sprache" | 2026-10-13 | dito |
| „je einen `taucher` auf jeden der acht Fäden, harte Bandagen, ein Dispatch = ein Faden" | 2026-10-15 | dito (folge358) |
| Startlinie „Starte die Mycelium-Linie in einem Pass …" | 2026-10-15 | dito (folge359) |

## Gearbeitet (dieses Rund)

- **Der Tauchauftrag lief: acht `taucher`, ein Faden je Dispatch, harte Bandagen.** Die
  vollständigen Messungen (Routen, Belege, Risse, Block-Entwürfe) stehen in
  `docs/surveys/survey-2026-10-15-tauchbefunde-acht-faeden.md`. Kern:
  - **HMI** — JSOC steht (`phi:2081`, CC0-1.0 gilt für `hmi.X`); Serien `hmi.M_720s` (720 s,
    FITS 4096², Gauss) und `hmi.sharp_cea_720s` (Vektorfeld) gemessen. **Nur der `hmi`-Kanal fehlt.**
    Riss: `hmi_polar`-Maschinerie ohne Register, Compiler-Tag `jsoc1.stanford.edu` (unregistriert,
    404) ≠ Asset-Ort `ssd.jpl.nasa.gov` (206).
  - **SILSO** — **keine Lücke, ein Verwurf.** Die Quelle ist öffentlich (CSV sha `4c452439…`,
    1818–2026, CC-BY-NC-4.0); aber Git-Verdikt `decline unphysical` (`e988e464d`),
    `docs/specs/sunspots.md` und `port.rs:2048` **verwerfen SSN**. Riss benannt.
  - **XUV / log R'HK** — RAVE/GALAH tragen **keinen** Aktivitätsindex (0 Spalten); echte Quellen:
    VizieR `J/A+A/697/A230` (`logR'IRT`), Zenodo `18834075` (LAMOST `R′_HK`), SDO EVE (CC0-1.0).
    Riss: EVE ist im Code verdrahtet, aber register-/CDN-los; `phi:9898` (GALAH) HTTP 400.
  - **Shear** — KiDS WL (16,50 GiB, cc-by-4.0) + Conicus3D (32,04 GiB) registriert; **kein**
    Shear-Kanal, **keine** sha256, alle drei CDN-`url` 404; `format tar_zst` ohne Reader.
  - **Neutrino** — `neutrino`=0 ist ein **Wort-Null** (Kanal per `witness`/`record` S2E1/SKY1/PAO1).
    **Harter Code-Riss:** vier Compiler (`auger`/`icecat`/`antares_vo`/`amon`) prüfen ihr MAGIC
    gegen `S2Direction`, `witness.rs:20-23` bildet dieselben auf `PointEvent` → **die vier brechen
    in Zeile 1 ab** (Grund der 404).
  - **EDGES/21 cm** — die Quellen sind öffentlich (Zenodo `16881868`→`18091240` cc-by-4.0, loco/asu
    CSV); die Lücke liegt **in phi**, nicht in der Welt (Vorgänger `xfer:mycelium-344-2` hatte sie
    2026-10-13 schon gemessen). Riss: `format hips3d` (WALLABY) und `tar_zst` ohne Reader.
  - **PTA** — `phi:30615` ohne `compiler`/`field`/`at`/`sha256`; EPTA-DR2.zip enthält **keine**
    Residuum-Datei (nur `.par`/`.tim`); CDN 404.
  - **TGF/Wolken** — TGF: FSSC zweiter Katalog (keyless, PD) mit sha256; der „3. Katalog" ist bloß
    ein Abstract (`pending`). Wolken: Open-Meteo-Archive (keyless, CC-BY-4.0, Compiler generisch) =
    billigster Kanal; MODIS/ERA5 gated. (WGLC bleibt Rat.)
- **Der Kern der Korrektur bestätigt:** für HMI/XUV/Shear/Neutrino/PTA steht die Quelle — der
  **Kanal** (die `field`-Zeile) fehlt; für SILSO/EDGES/TGF/Wolken fehlt die Quelle.

## Offen — eigen

Der gemeinsame Block: **ein `field`/`terms`/`ttl` ist Mountains Zeile** — bis dahin liegt je Faden
die **Messung** (Survey) und die **Mycelium-Seite** (`url`/`origin`/`compiler`/`sha256`/Tag/Naht).
Die acht Fäden stehen deshalb als Mountain-Anfragen (`xfer:mycelium-359-1 … -8`); hier nur der
eigene, autonom arbeitbare Rest:

### HMI — `hmi`-Kanal + `hmi_polar`-Tag (eigen)
- **Lage:** (gemessen 2026-10-15) `hmi_compiler.rs` analog `aia_compiler.rs` baubar; CDN-Tag
  `jsoc.stanford.edu` steht.
- **Blockade:** `field`/`ttl`/Format-Arm (Mountain); der `hmi_polar`-Compiler lädt nach
  `jsoc1.stanford.edu` (unregistriert → `upload_release` verweigert), die Workflow-Naht zieht von
  `ssd.jpl.nasa.gov` (Asset dort, 206). **Riss — kein stiller Schreibakt.**
- **Braucht:** Mountains `field`+`ttl`+Format-Entscheid + Tag-Riss-Verdikt. `xfer:mycelium-359-1`.

### XUV/log R'HK — `eve-cdn.yml` (eigen, tag-gebunden)
- **Lage:** (gemessen 2026-10-15) `eve_compiler.rs` (Tag `lasp.colorado.edu-eve`) ist gebaut; der
  Tag ist **nicht** registriert (`upload_release` verweigert), `format eve` = 0 in phi.
- **Blockade:** ohne Mountains EVE-Block + Tag-Zulassung läuft die Naht nicht.
- **Braucht:** EVE-Block/`terms`/`field` (Mountain) + `phi:9898`-Reparatur. `xfer:mycelium-359-3`.

### Shear-CDN + Neu-Reader (eigen)
- **Lage:** (gemessen 2026-10-15) KiDS WL 16,50 GiB, Conicus 32,04 GiB; alle drei CDN-`url` 404;
  `format tar_zst`/`hips3d` ohne Reader.
- **Blockade:** `sha256` je 16,5/32 GiB (voller Lauf; **CI**, nicht Operator-Maschine) +
  `zenodo-raw-cdn.yml`-Matrix-Einträge; `tar_zst`-Arm ist ein Bau-Entscheid.
- **Braucht:** Mountains Shear-`field` (force/kernel = Rat) + den Bau-Entscheid. `xfer:mycelium-359-4`.

### Neutrino-Assets — Compiler-Gate-Riss (eigen + Mountain, Kreuz)
- **Lage:** (gemessen 2026-10-15) `auger`/`icecat`/`antares_vo`/`amon`-Compiler brechen am
  `S2Direction`-Gate ab; `witness.rs` sagt `PointEvent`.
- **Blockade:** die Auflösung berührt `witness.rs` (Archivar/Mountain) **und** die Compiler
  (Mycelium) — **ein** Atom, zwei Linien.
- **Braucht:** Mountains Witness-Identität-Verdikt; dann Compiler-Gate + Spiegel + sha256.
  `xfer:mycelium-359-5`.

### Weitere offene Fäden (Kanal-CDN, alle Mountain-gebunden)
- **EDGES** (`edges_compiler.rs` freq-nativ, Entwurf im Survey): `xfer:mycelium-359-6`.
- **PTA** (`epta_dr2_compiler.rs`, Vorbild `llr`): `xfer:mycelium-359-7`.
- **TGF/Wolken** (TGF `gbm_tgf_compiler.rs`; Wolken = Open-Meteo-`origin`-Erweiterung):
  `xfer:mycelium-359-8`.

### Vier Endpunkte — GEOFON steht; SEANOE-Adresse geliefert; ICGEM-Transport (eigen)
- **Lage:** (gemessen 2026-10-15) **GEOFON steht bereits** (`phi:163`/`phi:10496`, `CC-BY-4.0`) —
  offen ist nur der Transport. **SEANOE** ist mit `4f2fe9ff7` **überholt** (Duplikat von `argo_gc`).
  **ICGEM:** Asset `…/icgem.gfz.de/gfc.bin` = 404; `url`/`origin`/`compiler` + CI-Bau = meine Seite
  (nach dem Rat zur SH-Kernel-Form). `xfer:mountain-354-1`.

### WDC-B Obninsk — Rettung erweitert; Host-Adressen geliefert (eigen)
- **Lage:** (gemessen 2026-10-15) **211** Wayback-Blöcke + 7 lebende `www.wdcb.ru/sep`-Kataloge;
  fünf Host-`reference`-Blöcke an Mountain geliefert (`esdb`/`stp` ohne `terms`). `field` je
  Datensatz = Rat (Kernel-Zuordnung). An Mountain: `xfer:mycelium-352-1`/`-2`.

### DE-2 / SABER — Wirt umstellen + manifestieren (eigen)
- **Lage:** (gemessen 2026-10-15) DE-2 über **NASA SPDF** (HTTP 206), SABER `saber.gats-inc.com`.
  Kein River-Akt — meine Hol-/Ablage-Aufgabe. **Braucht:** Aktor auf den neuen Wirt + Reihe
  manifestieren (DE-2-Block `phi:863`).

### JUICE-Bahn (Flyby) — operative Bahn von ESA-SPICE/NAIF (eigen)
- **Lage:** (gemessen 2026-10-15) recon-SPK + 1-σ bleiben ESOC (2026-11-01); operative Bahn bei
  ESA-SPICE + NAIF (HTTP 200). **Braucht:** `flyby_ephemeris_gate`-Tragfähigkeit prüfen; Kernels
  registrieren + manifestieren.

### TIMED / Celestrak GP (eigen)
- **Lage:** (gemessen 2026-10-15) `terms free-open` entschieden; **`ttl` je Orbit/Pfad-Takt PENDING**.
  **Braucht:** Mountains `ttl`; dann Block + CI-Naht. `xfer:mycelium-352-5`.

### F4′ — Schumann-Antrieb: WGLC global, täglich (eigen + River)
- **Lage:** (gemessen 2026-10-15) Quelle frei, `ttl 86400` steht; River hat den Tages-Deskriptor
  (`schumann_geomag_blitz.te`, `bin 86400`) gebaut. WGLC-`field`-Form = **Rat-Schemafrage**.
  `xfer:mycelium-356-5`.

### THEMIS-ASI FODG-Cube (eigen + River)
- **Lage:** (gemessen 2026-10-15) Compiler + CI-Naht stehen; FODG-Cube noch nicht im CDN. **Braucht:**
  die Mountain-Registerzeile (`terms free-open` steht `phi:11844`; format/url/origin/compiler = ich).
  `xfer:river197-6`.

### Venus-Radar / Bild-Träger (eigen)
- **Lage:** (gemessen 2026-10-15) Drei Träger gespiegelt + registriert (`phi`, `zenodo-raw-cdn.yml`).
  Offen: Overviews-Frage (Rat); die **80,9-GB-Basemap** braucht den **streamenden `.parts`-Splitter**
  (`raw_mirror` gattert auf die vorab bekannte Ganz-sha256). Eigener, begrenzter Bau.

### HIGP-MODVOLC-Archiv (eigen)
- **Lage:** (gemessen 2026-10-15, **Mountain-Wort**): keine offene Lizenz → `local-only`; **kein**
  `phi`-Block, keine `terms`-Zeile. Die Arbeitskopie lebt unter `$OMEGAFLOW_WORK/data/`.
  `xfer:mountain-348-1` beantwortet. **Braucht:** nichts weiter.

### Register→Manifest + Ersatzquellen + LOD + BASE + WDC/terms + ttl (eigen)
- **Lage:** (gemessen 2026-10-15) unverändert offen: Ersatzquellen-Blöcke (OCNS2019 `local-only`;
  JPL SSD/Horizons) · **LOD** · **BASE** (> 2-GiB-Limit `cdn.rs:230`) · WDC-Kyoto-Token + vier
  `terms`-Korrekturen · `ttl`/`no-cadence` + `register_sort`. An Mountain: `xfer:mycelium-344-5` ·
  `-343-3` · `-342-10`/`-343-2`.

## An mountain

Origin: mycelium-359 (2026-10-15): die acht Fäden wurden getaucht (Survey
`survey-2026-10-15-tauchbefunde-acht-faeden.md`). Jeder Faden braucht Deine Zeile — hier die
Anfragen:

- `xfer:mycelium-359-1` **HMI** — `terms CC0-1.0` gilt für `hmi.X` (steht); brauche `field`-Zeile
  (`hmi_m_720s_blos`, Einheit G/T), `ttl`, den Format-Arm (`hmi` neu vs. `aia`), und Dein Verdikt zum
  `hmi_polar`-Tag-Riss (`jsoc1.stanford.edu` unregistriert vs. Asset `ssd.jpl.nasa.gov`).
- `xfer:mycelium-359-2` **SILSO** — **Riss**: die Korrektur nennt es eine Lücke, das Git-Verdikt
  (`e988e464d`), `docs/specs/sunspots.md` und `port.rs:2048` **verwerfen SSN**. Dein Entscheid:
  Zulassung (`terms CC-BY-NC-4.0`) oder der `unphysical`-Verwurf trägt.
- `xfer:mycelium-359-3` **XUV/log R'HK** — brauche den **EVE-Block** (`format eve`, 11 Felder,
  `at sun`) + Tag-Zulassung `lasp.colorado.edu-eve`; die **`phi:9898`-Reparatur** (`ra_dr2`/`dec_dr2`
  → `ra`/`dec`, sonst HTTP 400); die Aktivitäts-`field`-Zeilen (`logR'IRT`/`logRHK`) und ihre
  Shape/Force-Klasse.
- `xfer:mycelium-359-4` **Shear** — brauche die Shear-`field`-Zeile mit **Rat-Wort** (`force`
  gravity vs. em; `kernel` inverse-square vs. inverse-linear; Einheit `1`); `phi:31178` fehlt
  `cmap`/`ra`/`dec`/Distance.
- `xfer:mycelium-359-5` **Neutrino** — **Kreuz-Riss**: `witness.rs:20-23` bildet `AMN1|PAO1|S2E1`
  auf `PointEvent`, die vier Compiler verlangen `S2Direction` → sie brechen ab. Dein
  Identitäts-Verdikt; dann ziehe ich die Compiler nach. Ferner: `field` für IceCube DR2 / Auger,
  `at earth`, `format json`-Korrektur, die `eev`-Unit-Registry.
- `xfer:mycelium-359-6` **EDGES/21 cm** — Quellen öffentlich (Zenodo `16881868`→`18091240`,
  `cc-by-4.0`; loco/asu CSV); brauche `terms` (Zitatpflicht), `field`-Kernel/force, `ttl`/`no-cadence`.
- `xfer:mycelium-359-7` **PTA** — `terms CC-BY-4.0` steht (`phi:30616`); brauche die
  `field TRES`-Zeile (`em`, Einheit `s`, Kernel, τ) und den Frame (`at sun` vs. `at earth`).
- `xfer:mycelium-359-8` **TGF/Wolken** — TGF `terms PD` steht (Haus-Form); brauche `field`/`ttl`.
  Wolken: Open-Meteo (CC-BY-4.0, steht) — die neuen `cloud_cover`-`field`-Zeilen + `ttl`. (Der
  „3. TGF-Katalog" ist `pending` — keine Datensatz-URL.)

Aus früheren Runden offen: `xfer:mycelium-352-5` (TIMED-`ttl`) · `xfer:mycelium-356-5` (WGLC-`field`)
· `xfer:mountain-354-1` (ICGEM) · `xfer:mycelium-352-1`/`-2` (WDC-B-`field`) · `xfer:mycelium-342-3`
(SEANOE — **überholt durch `4f2fe9ff7`**) · `xfer:mountain-348-2` (Fünf Fäden) · `-348-4`
(OCNS2019) · `xfer:mycelium-343-2`/`-342-10`.

## An river

Origin: mycelium-359 (2026-10-15):

- `xfer:river197-6` **THEMIS-ASI** — unverändert: Compiler + CI-Naht stehen, offen ist die
  Mountain-Registerzeile, dann der manuelle Lauf (misst den Onset selbst).
- `xfer:mycelium-356-3` **F4′** (Schumann) — der Tages-Deskriptor `schumann_geomag_blitz.te`
  (`bin 86400`, `cond wglc_strokes pending`) steht; die `field`-Form ist Rat-Sache.
- GIC-2 trägt River selbst; DE-2/SPDF, SABER/GATS und JUICE/SPICE liegen als Hol-/Ablage in meiner
  `## Offen — eigen`.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht
  messen das lade ich erst herunter wenn ich glasfaser habe."
  (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der session-weite
Consent, nie das Commit-Wort. Dieses Rund trägt das (standing) Wort „Am Ende: ein Commit (deine
Pfade)." Eigene Pfade dieses Atoms: diese Übergabe (folge359), die konsumierte folge358 (→
`archiv/`), `docs/surveys/survey-2026-10-15-tauchbefunde-acht-faeden.md`,
`state/operator-gespraeche/2026-10-15-mycelium.md`, die öffentliche Pass-Projektion
`docs/zustand/standing-pass.md`.
