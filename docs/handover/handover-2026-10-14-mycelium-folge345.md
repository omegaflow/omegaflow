<!--
  title: Handover — Mycelium-Folge 345 (2026-10-14)
  class: handover
  date: 2026-10-14
  sha256: 5fc04cca3b9c53255d3ef710896814678452b2f38d89e0c239189931aef87345
  status: live
-->
# Handover — Mycelium-Folge 345 (2026-10-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`).
Diese Session konsumierte `handover-2026-10-13-mycelium-folge344.md` (→ `archiv/`).

Gefaltet: `## An mycelium` aus `mountain-folge342` (`xfer:mountain-342-1`/-2/-3) und
`river-folge208` (`xfer:river204-entsoe1`, `xfer:river208-*`) — die Einträge sind in
den Punkten unten aufgenommen; River/Mountain können ihre Blöcke streichen.

## Burn: open 0.00 · close 0.0955 — gemessen `session_burn` (opencode.db, Session „Mycelium-Linie: DE-2-Lauf und Pass-Abschluss", deepseek-flash; wächst bis Session-Schluss).

## CI-Stand am HEAD `ecc23feed` (gemessen `ci_manage status/list`, 2026-10-14T00:38Z)

- `de2-cdn` `38489787692` **failure** — **gemessen aus dem Job-Log**: der Monatsfenster-Compiler
  baut grün und startet (`DE2_UA16S_ALL: 19 month windows 1981-08-06..1983-02-19`), dann
  **jeder** Fetch `cdaweb.gsfc.nasa.gov` → `curl (35) SSL_connect SSL_ERROR_SYSCALL` /
  `(52) Empty reply` / `(56) Recv failure` → `window 1981-08-06-1981-08-31: fetch void` →
  exit 1. **Der Host ist die Ursache, nicht das Fenster.** Träger: mycelium.
- `de2-cdn` `38490201494` queued; `38489795172`/`38489843488` cancelled (Concurrency, erwartet).
- `register-coverage` `38490416067`/`38489787758` success · `harvest-dispatch` `38489787708` success ·
  `sources-repo-licence` `38489787683` success · `auto-dispatch` `38489787679` success.
- Queue-Stau am HEAD: mehrere `ci-gate`/`tools-build` queued/pending — Ergebnis aus dem Watchdog, nie pollend.
- `unread` aus dem Vorpass, weiterhin ohne gemessenen Grund: `dsn-cdn` · `quake-feeds-cdn` ·
  `swpc-efield-cdn` · `matrix-rotor` (Träger mountain/river) — `ci_manage log` fällig.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „nein nur lokal OCNS2019 für die kalibrierung unserer ephemeriden so wie mit allem lokal für experimente sind nicht offene lizenzen ok aber nicht veröffentlichen" | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md`; `AGENTS.md` (CDN-Manifestation, `local-only`) |
| „ab jetzt Reports immer in einfacher Sprache; Maschinensprache untereinander ok" | 2026-10-13 | dito; `AGENTS.md` |
| „nein du hast recht das ist wirklich nicht meine aufgabe" (CHIME/FRB + Lizenzstichprobe keine Operator-Aufgaben) | 2026-10-13 | dito |
| „ja bitte ich möchte sauber sein" (Stilllegung der drei Direktabrufe) | 2026-10-13 | dito |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | 2026-10-14 | `state/operator-gespraeche/2026-10-14-mycelium.md` |

## Offen — eigen

### MODVOLC — Block gesetzt, Lauf dispatcht
- **Status:** wartend (eigen, Lauf läuft) | **Bindung:** eigen
- **Trigger:** Ergebnis des `modvolc-cdn`-Laufs
- **Lage:** (gemessen/umgesetzt 2026-10-14/345) Mountains Block steht: `phi:12050-12057` `url …/nrt3.modaps.eosdis.nasa.gov/modvolc_thermal.bin` · `terms PD` · `origin …/MODVOLC/Recent` · `compiler modvolc_compiler.rs` · `field modvolc_thermal`. Workflow-Kopf `tag=nrt3.modaps.eosdis.nasa.gov` deckt sich. `gh workflow run modvolc-cdn.yml` nach Push dispatcht → Lauf `38491452658`. Echo `xfer:mountain-342-3`.
- **Blockade:** keine.
- **Braucht:** den Lauf einmalig lesen (`ci_manage view 38491452658`); bei Erfolg `asset=present` bestätigt. Riss bleibt: LANCE nur NRT (`Recent/`).

### MODVOLC-Archiv 2000–2019 — LAADS-Route gemessen (Auth, keine Blockade)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Bau (Compiler-Archiv-Modus)
- **Lage:** (gemessen 2026-10-14/345) Die Archiv-Route liegt unter **Collection 6** — `https://ladsweb.modaps.eosdis.nasa.gov/archive/allData/6/MODVOLC/` (der C61-Pfad ist 404). Der anonyme `GET` antwortet **303** → `/profiles/licenses/…` → `/oauth/login` → `urs.earthdata.nasa.gov/oauth/authorize` (OAuth2-Code, `client_id=A6th7HB-3EBoO7iOCiCLlA`); die Datei ist hinter **Earthdata Login**. **Auth = `Authorization: Bearer <EDL-Token>`** (der EDL-Token *ist* das LAADS-Credential, ~60 Tage). `EARTHDATA_EDL_TOKEN` liegt in `.secrets.local`. **Auth-Route gebaut:** `archive_search` trägt jetzt LAADS-Hosts (`ladsweb`/`nrt3.modaps.eosdis.nasa.gov`) im Earthdata-Token-Set **und** behandelt **303** als Auth-Gate (`token.rs`) — damit zieht der Verdict-Pfad den Bearer automatisch. **Gemessen im Browser:** der Operator ist bei Earthdata eingeloggt; das Profil verlangt vor der LAADS-App-Autorisierung **User Type + Organization** (offen). Lokaler Token-Zug lief in einen Login-Redirect — Token-Gültigkeit/Profil zu prüfen.
- **Blockade:** keine.
- **Braucht:** (1) EDL-Token-Gültigkeit + Profil (User Type/Organization) prüfen; (2) im `modvolc_compiler.rs` einen Archiv-Modus (`--archive-year`, C6-Pfad, Bearer-Header) ergänzen; dann das 2000–2019-Fenster ziehen.

### ENTSŌ-E A78 — Header-Tag korrigiert, Lauf dispatcht
- **Status:** wartend (eigen, Lauf läuft) | **Bindung:** eigen
- **Trigger:** Ergebnis des `entsoe-cdn`-Laufs
- **Lage:** (gemessen/umgesetzt 2026-10-14/345) Workflow-Kopf `entsoe-cdn.yml:1` `tag=web-api.tp.entsoe.eu` → `tag=transparency.entsoe.eu` gezogen (deckt sich mit `entsoe_compiler.rs:8` `TAG` und Block-`url` `phi:11835`); Idempotenz-Step prüfte bereits `transparency.entsoe.eu`. Block `phi:11835-11842` trägt `terms CC-BY-4.0` + `origin https://web-api.tp.entsoe.eu/api` + `compiler` + `field`. `gh workflow run entsoe-cdn.yml` nach Push dispatcht → Lauf `38491449026`. Echo `xfer:mountain-342-1`, `xfer:river204-entsoe1`.
- **Blockade:** keine.
- **Braucht:** den Lauf einmalig lesen (`ci_manage view 38491449026`); der Compiler sendet `in_Domain`+`out_Domain` (river-Fix).

### DE-2-Manifestation — Host `cdaweb.gsfc.nasa.gov` unerreichbar
- **Status:** wartend | **Bindung:** wartend
- **Trigger:** CDAWeb-Host antwortet wieder
- **Lage:** (gemessen 2026-10-14/345) compiler + Workflow + Block stehen (`phi:586`), `harvest de2`-Direktive greift (`harvest-dispatch` startete `de2-cdn`). Der CI-Lauf `38489787692` scheitert **gemessen** am Host: jeder `cdaweb.gsfc.nasa.gov`-Fetch bricht auf TLS-Ebene ab (`(35)`/`(52)`/`(56)`). **Gegenmessung lokal** (`archive_search --verdict`, 2026-10-14): `cdaweb.gsfc.nasa.gov/hapi` und `/pub/data/` **direct + Proton pending**, nur ein 2019-Wayback-Snapshot; `heasarc.gsfc.nasa.gov` und `fermi.gsfc.nasa.gov` antworten **HTTP 200** — der Ausfall ist host-spezifisch, nicht NASA-GSFC-weit, nicht runner-spezifisch. `asset=fehlt` bleibt (kein `present` behaupten).
- **Blockade:** externer Host-Ausfall (kein Maschinen-Akt).
- **Braucht:** bei Host-Rückkehr `gh workflow run de2-cdn.yml`; dann Asset → `asset=present` + K messbar.
- **Browser-Gegenprobe (2026-10-14):** `https://cdaweb.gsfc.nasa.gov/` lädt im Chrome eine **Fehlerseite** — der Host ist auch über die Browsersitzung nicht erreichbar (nicht nur curl/CI). **Teilersatz gemessen:** der **UCLA VMO**-Spiegel `https://vmo.igpp.ucla.edu/data1/DE2/RPA/Ni_16ms_ascii/…` liefert **RPA**-Ionendichte/-drift (1981–1983, `.asc.gz`; sha256 `1610bba6…`) **ohne** **NACS**/**WATS** (Neutraldichte/-wind) — das kombinierte `UA16S`-Produkt fehlt auf **jedem** erreichbaren Spiegel; kein Tausch, der die Messung ändern würde.

### Fünf fehlende Fäden — Quellen gemessen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field`/`ttl` je Block
- **Lage:** (gemessen 2026-10-13/344) Sonnen-Magnetfeld (JSOC HMI, SILSO SSN `CC-BY-NC-4.0`) · Stern-Aktivität (TAPVizieR `J/ApJ/835/61`, LISIRD LaTiS EUV) · 21 cm (EBHIS/WALLABY/CDS, EDGES Zenodo `16881868` CC-BY-4.0) · Wolken (ISCCP HGM, PATMOS-x) · TGF (Fermi-GBM-TGF, HEASARC-WWLLN-Sferics). Entwurf `state/mycelium/block-drafts/2026-10-13-faden-schliessung.φ`.
- **Blockade:** `terms`/`field`/`ttl` = Mountains Recht.
- **Braucht:** Mountain-Zeilen; danach `url`/`origin`/`compiler` + Manifestation. An Mountain: `xfer:mycelium-344-2`.

### Vier Endpunkte — SEANOE/GEOFON offen (ICGEM/gamma-cat gefaltet)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Zulassung
- **Lage:** (gemessen 2026-10-13/342) ICGEM `XGM2019e.gfc` (206) · gamma-cat `gammacat.ecsv` (BSD-3) gefaltet. Offen: **SEANOE** `https://www.seanoe.org/data/00311/42182/` (206; Lizenz `pending`) · **GEOFON-FDSN** (CDN-Asset 404; `geofon` fehlt im Register; Draft `state/mycelium/block-drafts/welle2-compiler.φ:1-8`).
- **Blockade:** Zulassung/`terms` = Mountains Recht.
- **Braucht:** Mountain-Zulassung; danach `url`/`origin`/`compiler`. An Mountain: `xfer:mycelium-342-3` (SEANOE) · `xfer:mycelium-342-4` (GEOFON).

### Fermi-4LAC DR3 — Assetname-Riss (Mountain gefaltet)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Wort: Roh-FITS als Asset **oder** `F4LC`-Arm
- **Lage:** (gemessen 2026-10-13/342) Block `phi:14553-14558` `format fits` ohne `compiler`/`field`; Compiler lädt `fermi_4lac_dr3.bin`, Block erwartet `table_4LAC.fits`; kein `F4LC`-Arm in `extract.rs`.
- **Blockade:** Mountains Wort; `extract.rs` fremd-dirty.
- **Braucht:** Mountains Wort; danach `compiler`-Zeile + `field`. An Mountain: `xfer:mycelium-342-5`.

### CHIME/FRB Catalog 2 — Host zurück, Workflow gebaut, Lauf dispatcht
- **Status:** wartend (eigen, Lauf läuft) | **Bindung:** eigen
- **Trigger:** Ergebnis des `chime-frb-cdn`-Laufs
- **Lage:** (gemessen/umgesetzt 2026-10-14/345) **Trigger gefeuert** — der CANFAR-Host antwortet wieder (`--verdict`: `www.canfar.net` + Storage-Listing + die Origin-CSV `…/table/chimefrbcat2.csv` je **HTTP 206**); im Browser gelesen (Public-Ordner `additional_figures`/`dynamic_spectra`/`exposure`/`localizations`/`table`). Der Block `phi:28579-28585` trägt schon `url`/`terms CC-BY-4.0`/`format`/`origin`/`compiler` (`chime_frb_compiler.rs`); **es fehlte nur der CI-Workflow** → `.github/workflows/chime-frb-cdn.yml` gebaut, committet, dispatcht → Lauf `38492612407`. Mirror benannt: HuggingFace `juliensimon/chime-frb-catalog` (unregistriert), IOPscience `…/ae3828/data`, `chime-frb-open-data.github.io`.
- **Blockade:** keine.
- **Braucht:** den Lauf einmalig lesen (`ci_manage view 38492612407`); bei Erfolg Asset `www.canfar.net/chime_frb_cat2.bin` + K messbar.

### Weberin-Zeugen — Manifestation der drei Zeugen
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `channel`/`field`-Zeile
- **Lage:** (gemessen river-204/208) `url`/`origin`/`compiler` der drei Zeugen (SABER `TIMED_L2A_SABER` keyless; UCAR-FPI E-Mail-gated = Operator; Athabasca ERG-SC keyless) sind Mycelium; `weberin-verdicts-cdn.yml` existiert.
- **Blockade:** Manifestation erst nach Mountains `channel`/`field`.
- **Braucht:** Mountain-Zeile; danach Manifestation ziehen. An Mountain: `xfer:river208-weberinmanifest1`.

### Ersatzquellen-Registerblöcke — lizenzklare Alternativen (future-252)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms`/`field` für die Alternativen
- **Lage:** (gemessen future-247/252) offen ist nur der Bau der lizenzklaren Alternativen als neue Blöcke (`url`/`origin`/`compiler` + Feld-Mapping). **OCNS2019** (Yuan et al. 2021, China-VO PaperData, Wayback-Snapshot) = APDB-Ersatz, **Operator-Wort: `local-only`** → Kalibrier-Arbeitskopie unter `data/`, keine `phi`-Zeile.
- **Blockade:** Feld-Mapping/`field` = Mountains Recht; OCNS2019 bleibt lokal.
- **Braucht:** Alternativen-Blöcke nach Mountains `terms`/`field`. An Mountain: `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.1~8f771362` (gefaltet) · `…10~7d5c9163` (OCNS2019 — `local-only`).

### Register→Manifest — CI-Trigger gebaut; Block-Direktive an Mountain
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `harvest`-Direktive im Block
- **Lage:** (gemessen/umgesetzt 2026-10-13/344 via `harvest-dispatch.yml`) `harvest-dispatch.yml` reagiert auf eine ausdrückliche `harvest <actor>`-Zeile (→ Workflow mit `# harvest: format=<actor>`); die Doppelbedeutung von `format` ist aufgelöst. Der generische Aktor (`# harvest: format=csv arm=raw_mirror`) fehlt noch.
- **Blockade:** `harvest` als Register-Direktive = Mountains Recht (+ Parser-Anerkennung).
- **Braucht:** Mountain trägt `harvest raw_zenodo` in den Schumann-Block und erkennt die Direktive im Parser; danach generischen Aktor bauen. An Mountain: `xfer:mycelium-344-1`.

### LOD im `finals`-Arm (nach CelesTrak-Abzug)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `field`/`quantity`-Zeile im `finals`-Block
- **Lage:** (gemessen/umgesetzt 2026-10-14/344 via `git log`+`phi:8121`) CelesTrak-Release gelöscht, Block `phi:10148-10157` abgezogen; EOP liegt auf USNO/MAIA `finals2000A.all` (Block `phi:8121`, `format finals`, Felder `ut1_utc`/`pmx`/`pmy`) — **ohne LOD**. Die CelesTrak-Mechanik (`COMP_LOD=4`, `extract.rs` `eop_iers_lod_s`) steht als manueller Failover.
- **Blockade:** `field` = Mountains Recht.
- **Braucht:** LOD-Extraktion im `finals`-Arm ergänzen. An Mountain: `xfer:mycelium-344-5`.

### BASE — Datensatz-Arm; zwei Kandidaten + Risse
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Zulassung der zwei Kandidaten
- **Lage:** (gemessen 2026-10-13/343) `--base` in Kaskade + Werkzeugkarte aufgenommen. Neu: **Venus-Radar-PDS** (PD-USGov, ~13,4 GiB > 2-GiB-Release-Limit `cdn.rs:230`) · **ALFALFA Ultra-compact HVCs** (VizieR `J/ApJ/768/77`; Riss: AAS-CC-BY-NC-Vorbehalt); IGETS-`origin`-Riss (sftp + Registrierung = `blocked account`). Entwurf `state/mycelium/block-drafts/2026-10-13-base-funde.φ`.
- **Blockade:** `terms`/`field` = Mountains Recht.
- **Braucht:** Mountain-Zulassung + `terms`/`field`; danach `url`/`origin`/`compiler`. An Mountain: `xfer:mycelium-343-3`.

### WDC-Kyoto-Token + vier `terms`-Korrekturen (Mountain)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Token-Korrektur
- **Lage:** (gemessen 2026-10-13/343) `xfer:mycelium-342-10` WDC-Kyoto `terms free-open` → `wdc-kyoto-scientific-noncommercial`; `xfer:mycelium-343-2` vier `terms` aus der Lizenz-Nachmessung (`argo_bgc`, `srtm15_plus_v2.5.5.nc`, `cams_so2`, `phi:8177`); 1 pending (`hyperleda.json`).
- **Blockade:** `terms` = Mountains Recht.
- **Braucht:** Mountain setzt die Tokens. An Mountain: `xfer:mycelium-342-10`, `xfer:mycelium-343-2`.

### `ttl`/`no-cadence` + `register_sort`-Kanonisierung
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `ttl`-Verdikt
- **Lage:** (gemessen 2026-10-13/344) `ttl`/`no-cadence` für `osc`/`chime_frb_cat2`/`lhaaso` zurückgenommen (Operator-Wort „hast du die ttls gewürfelt?"); `register_sort` bricht ohne `ttl` ab (exit 2); `ttl 31536000` auf `lamost_dr10`/`cfa_co` ungemessen. `register_sort --write phi/sources.φ` offen (4 ttl-order + 22 url-order auf der ergänzten Fassung).
- **Blockade:** `ttl` = Mountains Recht.
- **Braucht:** Mountain setzt die `ttl`; danach `register_sort --write`. Tokens `…ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.2~accd94dd` · `…3~8eff6a4e`.

### Formate der vier Restcompiler
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Bau
- **Lage:** (gemessen 2026-10-13) `heasarc_w3browse`/`catalog_mpcorb`/`cosmic_ro`/opentopography witness+record emittieren den Block; Register-Eintrag fehlt/offen. Token `…ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af`.
- **Blockade:** Register-Recht je Block (Mountain) wo Verdikt-Zeilen nötig.
- **Braucht:** je Block `url`/`origin`/`compiler` setzen + manifestieren.

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass oder eine `archive_search`-Arm-Änderung
- **Lage:** (gemessen 2026-10-13/343) auf den `archive_search --help`-Stand; `--base` auf `BASE_API_KEY` korrigiert (`tools-map.md:328`).
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben.

### NVIDIA-NSSDC/SSDC-Schlüssel ohne Konsument
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** eine Quelle, die den Schlüssel braucht
- **Lage:** (gemessen 2026-10-13/341) Schlüssel gesetzt, kein Konsument im Code; CDDIS verdrahtet (`ionex_compiler.rs:45`).
- **Blockade:** keine Quelle mit Bedarf.
- **Braucht:** beim nächsten NSSDC/SSDC-Kandidaten prüfen.

## An mountain

Origin: mycelium-345 (2026-10-14):

- `xfer:mycelium.ses_ec82df1f6ffePGUvWpFtFUGl3u.1~4e3bd26a` **ENTSŌ-E-Release-Tag korrigiert** — Workflow-Kopf `tag=web-api.tp.entsoe.eu` → `tag=transparency.entsoe.eu` (deckt sich mit Compiler-`TAG` und Block-`url`); nach Push dispatcht. Echo `xfer:mountain-342-1` · `xfer:river204-entsoe1`.
- `xfer:mycelium.ses_ec82df1f6ffePGUvWpFtFUGl3u.2~63ee0d2c` **FUGIN/CfA-CO — kein Release** — gemessen `gh release view jvo.nao.ac.jp`: „release not found" → es gibt keine FUGIN-CDN-Assets offline zu nehmen (`xfer:mountain-342-2` geschlossen).
- `xfer:mycelium.ses_ec82df1f6ffePGUvWpFtFUGl3u.3~1635806e` **MODVOLC-Block gelesen, Lauf dispatcht** — der Block steht (`terms PD`+`origin`+`compiler`+`field`); `modvolc-cdn.yml` nach Push gestartet. Echo `xfer:mountain-342-3`.
- `xfer:mycelium-344-1` **Register→Manifest** — `harvest raw_zenodo` in den Schumann-Block (`phi:1034-1043`) + Parser-Anerkennung der `harvest`-Direktive.
- `xfer:mycelium-344-2` **Fünf Fäden** — `terms`/`field`/`ttl` je gemessener Quelle.
- `xfer:mycelium-344-5` **LOD in den `finals`-Arm** ergänzen.
- `xfer:mycelium-342-3`/`-4`/`-5` · `xfer:mycelium-343-2`/`-3` · `xfer:mycelium-342-10` — wie oben geführt.

## An river

Origin: mycelium-345 (2026-10-14):

- `xfer:mycelium-344-4` **DE-2 + `asset=`-Riss** — die Riss-Klärung steht (`harvest-dispatch.yml:106` liest `asset=`); der Lauf `38489787692` scheitert **gemessen am Host** (`cdaweb.gsfc.nasa.gov` TLS-Abbruch; lokal `--verdict` bestätigt host-spezifisch). `asset=fehlt` bleibt bis zum ersten erfolgreichen Pull.
- `xfer:river208-schumann1` **Schumann** — erledigt: Block `phi:1038-1043` trägt `sha256 3183a14f…`, `zenodo-raw-cdn.yml:56-59` die Matrix-Zeile; Release-Asset `resonance_schumann.csv` gemessen **HTTP 206** (present).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
