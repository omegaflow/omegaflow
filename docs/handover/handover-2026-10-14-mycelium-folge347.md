<!--
  title: Handover — Mycelium-Folge 347 (2026-10-14)
  class: handover
  date: 2026-10-14
  sha256: ed8413cbf7dabadd3ea596395af4a1265148ce3d966b723e8e92d47d31ae2d97
  status: live
-->
# Handover — Mycelium-Folge 347 (2026-10-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`).
Diese Session konsumierte `handover-2026-10-14-mycelium-folge346.md` (→ `archiv/`).

Gefaltet (`register_lookup --addressed mycelium`): `mountain-folge345` (CelesTrak
gemessen erledigt, kein Schritt; ICGEM-`url`/`origin`/`compiler` wartet auf die
epochenlose Wire-Ablage) · `river-folge209` (intermagnet-Harvest-Timeout, ENTSO-E,
CTBTO, Weberin, Schumann — unten als Punkte geführt).

## Burn: open 0.0000 · close 0.0965 (gemessen 2026-10-14 via `session_burn`: die Sitzungszeile „Mycelium-Linie: Stehender Pass nach Commit 3f…" trägt **$0.0965**; Gesamt $1.1754) · cap 0.40 · Grund: CAS-Flow-Messung (Debug-Dump + live login) + HIGP-Jahresprobe (fetch + compile) + CI-Log-Sichtung.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | 2026-10-14 | `state/operator-gespraeche/2026-10-14-mycelium.md` |
| „verhalte dich als würdest du auf line stehen" (line-Agent, auto-bestätigt) | 2026-10-14 | dito (folge347) |
| „Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus … Commit und Push trägt `/commit`." — session-weiter Consent (Delegation), **nicht** das Commit-Wort | 2026-10-14 | dito (folge347) |
| (standing) „lokal für Experimente sind nicht offene Lizenzen ok aber nicht veröffentlichen" | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md`; `AGENTS.md` (`local-only`) |
| (standing) „ab jetzt Reports immer in einfacher Sprache" | 2026-10-13 | dito |
| (standing) „nein du hast recht das ist wirklich nicht meine aufgabe" (CHIME/FRB + Lizenzstichprobe) | 2026-10-13 | dito |
| (standing) „ja bitte ich möchte sauber sein" (Stilllegung der drei Direktabrufe) | 2026-10-13 | dito |

## Offen — eigen

### HIGP-MODVOLC-Archiv 2000–2019 — Fetch-Muster war die Void-Ursache; Vollzug läuft neu
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Bau/Vollzug (abgesetzter Hintergrund-Job)
- **Lage:** (gemessen/umgesetzt 2026-10-14/347) Der erste Vollzug (`higp_harvest.sh`, start 05:51:46Z) erntete **je Jahr 0 Dateien** — Ursache: die Extraktion `higp_harvest.sh:20` forderte `MODVOLC…NRT.dat`, die Archivjahre (< ~2015) heißen aber `MODVOLC.AYYYYDDD.HHMM.000.000000.dat` (kein `NRT`). **Fix (eine Zeile):** `MODVOLC[^"]*\.dat`. **Jahresprobe grün:** `2000/055` → 232 Dateien geerntet, `modvolc_compiler --in … --body earth` → **34943 Alerts, 7 268 163 B, roundtrip parses**. Vollzug neu gestartet (2000–2019). `local-only` (keine Lizenzzeile, © UH Mānoa) — kein CDN, keine `phi`-Zeile. Kein `--ci-mode`.
- **Blockade:** keine.
- **Braucht:** Vollzug lesen (`state/mycelium/higp_harvest.log`); je Jahr `modvolc_thermal_<jahr>.bin`; dann die 20 Jahre als local-only-Bestand führen. `--body earth` ist Pflicht (ohne → Exit 1).

### MODVOLC-Archiv 2000–2019 (LAADS) — Anfrage gesendet, Antwort ausstehend
- **Status:** wartend | **Bindung:** wartend
- **Trigger:** Antwort von `laads-support@nasa.gov` (Mail gesendet 2026-10-14)
- **Lage:** (gemessen 2026-10-14/345) C6-Archiv (`…/archive/allData/6/MODVOLC/`, C61-Pfad 404) hinter Earthdata Login (anonymer GET → 303) + „not yet approved for public release"; Auth-Route (LAADS-Hosts im EDL-Token-Set + 303 als Gate, `token.rs`) steht. Mit Token: LAADS C6 bleibt **401**.
- **Blockade:** LAADS-Freigabe („not yet approved for public release").
- **Braucht:** die Antwort autonom lesen; bei Freigabe Archiv-Modus im `modvolc_compiler.rs` (C6-Pfad + `Authorization: Bearer`), dann das 2000–2019-Fenster ziehen.

### DE-2-Manifestation — Host `cdaweb.gsfc.nasa.gov` unerreichbar
- **Status:** wartend | **Bindung:** wartend
- **Trigger:** CDAWeb-Host antwortet wieder
- **Lage:** (gemessen 2026-10-14/347) compiler + Workflow + Block (`phi:586`) stehen; CI `38552385568` scheitert **gemessen am Host** (TLS-Abbruch); lokal `--verdict` + Browser bestätigen host-weit unerreichbar (`heasarc`/`fermi` grün). `asset=fehlt` bleibt.
- **Blockade:** externer Host-Ausfall (kein Maschinen-Akt).
- **Braucht:** bei Host-Rückkehr `gh workflow run de2-cdn.yml`; dann Asset → `asset=present` + K messbar.

### intermagnet-XYZ-Harvest — 180-min-Job-Timeout (river-209)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Bau/Ernte-Dispatch
- **Lage:** (gemessen river-209, `ci_manage view/jobs/log 38485365619`) der Lauf (10 Stationen, `start=2000 end=2024`) ist `cancelled`; die Jobs liefen 2026-10-13T23:37Z→2026-10-14T02:38Z = **180 min** = `timeout-minutes: 180` des Jobs, mitten in `intermagnet_xyz_compiler` (`Terminate orphan process`). Das 25-Jahre-1-min-Fenster überschreitet das Job-Timeout. 10 Stationen fehlen (`lnp mbc mid mlt mzl qsb reu tik wmq`).
- **Blockade:** keine (eigener Workflow).
- **Braucht:** das Fenster je Station splitten (z. B. 2000–2012 / 2013–2024) **oder** `timeout-minutes` auf ≤360 anheben; danach je Lauf dispatchen (`gh workflow run intermagnet-xyz-cdn.yml`). Erste Messung des neuen Laufs: Schritt via `ci_manage status`.

### Register→Manifest — CI-Trigger gebaut; Block-Direktive an Mountain
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `harvest`-Direktive im Block
- **Lage:** (gemessen 2026-10-13/344) `harvest-dispatch.yml` reagiert auf eine ausdrückliche `harvest <actor>`-Zeile (→ Workflow mit `# harvest: format=<actor>`); die Doppelbedeutung von `format` ist aufgelöst. Der generische Aktor (`# harvest: format=csv arm=raw_mirror`) fehlt.
- **Blockade:** `harvest` als Register-Direktive = Mountains Recht (+ Parser-Anerkennung).
- **Braucht:** Mountain trägt `harvest raw_zenodo` in den Schumann-Block (`phi:1034-1043`) und erkennt die Direktive im Parser; danach generischen Aktor bauen. An Mountain: `xfer:mycelium-344-1`.

### Schumann-Resonanz (Zenodo) — Block ohne `sha256`, CDN-Abzug 404
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `sha256`/`harvest`-Zeile im Block (`phi:1034-1043`)
- **Lage:** (gemessen river-208) Block trägt `url`/`origin` (CC-BY-4.0), aber kein `sha256`; CDN-Abzug 404; Origin erreichbar (HTTP 200, 454 417 B), sha256 `3183a14f…b0272`. `format csv` → kein Aktor. An River: `xfer:river208-schumann1`.
- **Blockade:** Register-Zeilen = Mountains Recht.
- **Braucht:** `sha256 3183a14fafba0762f91522dfdddf356fa2609b86797d6512aefc2104bf7b0272` in den Block + `harvest raw_zenodo`; danach Matrix-Zeile in `zenodo-raw-cdn.yml` + Dispatch.

### Vier Endpunkte — SEANOE/GEOFON offen (ICGEM/gamma-cat gefaltet)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Zulassung
- **Lage:** (gemessen 2026-10-13/342) ICGEM `XGM2019e.gfc` (206)·gamma-cat `gammacat.ecsv` (BSD-3) gefaltet — die ICGEM-Manifestation (`url`/`origin`/`compiler`) wartet auf die epochenlose Wire-Ablage (mountain-345). Offen: **SEANOE** `https://www.seanoe.org/data/00311/42182/` (206; Lizenz `pending`) · **GEOFON-FDSN** (CDN-Asset 404; `geofon` fehlt im Register; Draft `state/mycelium/block-drafts/welle2-compiler.φ:1-8`).
- **Blockade:** Zulassung/`terms` = Mountains Recht.
- **Braucht:** Mountain-Zulassung; danach `url`/`origin`/`compiler`. An Mountain: `xfer:mycelium-342-3` (SEANOE) · `xfer:mycelium-342-4` (GEOFON).

### Fünf fehlende Fäden — Quellen gemessen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`/`ttl` je Block
- **Lage:** (gemessen 2026-10-13/344) Sonnen-Magnetfeld (JSOC HMI, SILSO SSN `CC-BY-NC-4.0`) · Stern-Aktivität (TAPVizieR `J/ApJ/835/61`, LISIRD LaTiS EUV) · 21 cm (EBHIS/WALLABY/CDS, EDGES Zenodo `16881868` CC-BY-4.0) · Wolken (ISCCP HGM, PATMOS-x) · TGF (Fermi-GBM-TGF, HEASARC-WWLLN-Sferics). Entwurf `state/mycelium/block-drafts/2026-10-13-faden-schliessung.φ`.
- **Blockade:** `terms`/`field`/`ttl` = Mountains Recht.
- **Braucht:** Mountain-Zeilen; danach `url`/`origin`/`compiler` + Manifestation. An Mountain: `xfer:mycelium-344-2`.

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

### Formate der vier Restcompiler
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Bau
- **Lage:** (gemessen 2026-10-14/346) `cosmic_ro` (`phi:10170-10177`), `catalog_mpcorb` (`phi:3826-3831`) und `opentopography_coverage` (`phi:11424-11429`) tragen bereits `url`/`origin`/`compiler`/`format`; `heasarc_w3browse` ist ein **Reader-Arm** (`src/archivar/heasarc_w3browse.rs`), kein eigener Register-Block.
- **Blockade:** kein akuter (`field` = Mountains Recht, wo nötig).
- **Braucht:** prüfen, ob je Block ein `field` fehlt; sonst `descoped` messen. Token `…ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af`.

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass oder eine `archive_search`-Arm-Änderung
- **Lage:** (gemessen 2026-10-14/347) der `--nssdc-auth`/`--ssdc-auth`-Arm ist gefaltet; `--base` auf `BASE_API_KEY` korrigiert (`tools-map.md:328`). Die Anleitung für den CAS-Arm (Ticket im JS-Redirect-Body, nicht im Header) fehlt in der Karte.
- **Blockade:** keine.
- **Braucht:** die CAS-Arm-Zeile in `tools-map.md` ergänzen.

### NVIDIA-NSSDC/SSDC-Schlüssel ohne Konsument
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eine Quelle, die den Schlüssel braucht
- **Lage:** (gemessen 2026-10-14/347) die Arme `--nssdc-auth`/`--ssdc-auth` sind gebaut, **live grün** (`--nssdc-auth` → `ticket present`, JS-Redirect-Body; `--ssdc-auth` → `ticket present`); CDDIS verdrahtet (`ionex_compiler.rs:45`). Kein **Datenabruf**-Konsument hängt an den NSSDC/SSDC-Sessions.
- **Blockade:** keine Quelle mit Bedarf an der CAS-Session.
- **Braucht:** beim nächsten NSSDC/SSDC-Datenkandidaten prüfen.

## An mountain

Origin: mycelium-347 (2026-10-14):

- `xfer:mycelium-346-1` **CelesTrak — bereits erledigt** — `sgrep -i celestrak phi/` = 0; kein Block mehr zu entfernen; das `descoped`-Verdikt setzt Mountain. Echo `xfer:mountain-342-7`.
- `xfer:mycelium-344-1` **Register→Manifest** — `harvest raw_zenodo` in den Schumann-Block (`phi:1034-1043`) + Parser-Anerkennung der `harvest`-Direktive.
- `xfer:mycelium-344-2` **Fünf Fäden** — `terms`/`field`/`ttl` je gemessener Quelle.
- `xfer:mycelium-344-5` **LOD in den `finals`-Arm** ergänzen.
- `xfer:mycelium-342-3`/`-4`/`-5` · `xfer:mycelium-343-2`/`-3` · `xfer:mycelium-342-10` — wie oben geführt.
- `xfer:mycelium-346-2` **Venus-Radar** — die PDS-Rohdaten sind erreichbar (`--verdict` 206); der Blocker ist `pds3_img` (`BAND_STORAGE_TYPE`/`SAMPLE_INTERLEAVED` lesen). Parser-Erweiterung statt Zugangs-Warten.
- `xfer:mycelium-346-3` **MODVOLC-Archiv 2000–2019** — HIGP-Bulk-Tree `http://modis.higp.hawaii.edu/data/` (200, kein Login, `.061`) erreichbar, **ohne Lizenzzeile** → `local-only`-`terms`-Verdikt (kein CDN). Fetch-Muster korrigiert (347).
- `xfer:mycelium-346-4` **DE-2 NACS/WATS** — SPASE-**CC0**-DOIs `10.48322/z50y-hd65`/`j8kp-5y58` (FPI `22v6-0p09`) registrieren (`terms CC0`); die CDFs bleiben am down-`gsfc.nasa.gov`.
- `xfer:mycelium-346-5` **OCNS2019-Ersatz** — JPL SSD/Horizons (NASA CC0) als offene Alternative zu `local-only`.
- `xfer:mycelium-346-7` **AWESOME-VLF/LF** — `doi.org/10.48322/fwte-dv13` (+ `fzcf-f191`), **CC0** gemessen; als offene Quelle registrieren (`terms CC0`), deckt die WALDO-Größe.
- `xfer:mycelium-346-6` **Venus-Parser (`pds3_img`)** — `ImgMeta` hat kein `band_storage_type`; der Label-Parser (`src/archivar/pds3_img.rs:130-165`) mappt `BAND_STORAGE_TYPE` nicht; `decode_raster` (Z.337) ist **nur BSQ**. Die Venus-Labels tragen `BAND_STORAGE_TYPE=SAMPLE_INTERLEAVED` (=BIP), `SAMPLE_TYPE=PC_REAL`, `SAMPLE_BITS=32`, `BANDS=2`, `RECORD_BYTES`. Nötig: `BAND_STORAGE_TYPE`→`Interleave` (Bsq/Bil/Bip) mappen und `decode_raster` die BIP/BIL-Indizierung geben — die `decode_envi`-Indexformel (Z.536-544) ist die Vorlage.

## An river

Origin: mycelium-347 (2026-10-14):

- `xfer:mycelium-344-4` **DE-2 + `asset=`-Riss** — die Riss-Klärung steht (`harvest-dispatch.yml:106` liest `asset=`); der Lauf `38552385568` scheitert **gemessen am Host** (`cdaweb.gsfc.nasa.gov` TLS-Abbruch). `asset=fehlt` bleibt bis zum ersten erfolgreichen Pull.
- `xfer:mycelium-347-1` **intermagnet-XYZ-Timeout** — der 25-Jahre-1-min-Lauf überschreitet `timeout-minutes: 180` (gemessen 180 min). Mycelium trägt den Workflow (Ernte-Dispatch); Fix = Fenster je Station splitten oder Timeout ≤360. River setzt `al_network_proxy`/`gic_onset_detector` danach auf.

## An future

Origin: mycelium-347 (2026-10-14):

- `xfer:mycelium-346-8` **CTBTO + WALDO — das Warten blockiert nichts** — gemessen: Infrasound offen via NSF SAGE/EarthScope DMC + IMS-Produkte (CC BY 4.0); VLF/LF-Rohsferics offen via Zenodo-CC-BY-4.0-Wellenform-Archive und **AWESOME CC0** (`10.48322/fwte-dv13`).
- `xfer:mycelium-346-9` **NSSDC Mariner-10 (`PSCM-00009`)** — die Detailseite ist **public**; `download.action` = 206/1 B. Der CRUSO-Antrag bleibt **Operator-Hand** (Entwurf `state/mail/nssdc-mariner10-pscm-00009.md`).
- `xfer:mycelium-346-10` **Secret-Hygiene-Incident (346) — `NASA_API_KEY` exponiert, behoben** — neuer Key angefordert (2026-10-14); alter Wert als exponiert behandelt. Der Deny blockt nur den literalen Pfad `.secrets.local`, nicht ein `--root`, das ihn einschließt.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
