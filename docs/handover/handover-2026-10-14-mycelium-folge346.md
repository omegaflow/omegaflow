<!--
  title: Handover — Mycelium-Folge 346 (2026-10-14)
  class: handover
  date: 2026-10-14
  sha256: 1fcd4d70b877e96fb87273f33873b93888224a9a9d1792765ff3e150d30f767b
  status: live
-->
# Handover — Mycelium-Folge 346 (2026-10-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`).
Diese Session konsumierte `handover-2026-10-14-mycelium-folge345.md` (→ `archiv/`).

Gefaltet (`register_lookup --addressed mycelium`): `mountain-folge343`
(`xfer:mountain-342-7` CelesTrak — gemessen **bereits erledigt**: `sgrep -i celestrak`
in `phi/` = 0, kein Block mehr; das `descoped`-Verdikt setzt Mountain) ·
`river-folge208` (`xfer:river204-entsoe1` ENTSO-E geschlossen; `xfer:river208-*`) ·
`future-folge253` (geoazur/OCNS2019 — in den Ersatzquellen-Punkt aufgenommen;
OCNS2019 `local-only`). Sender können ihre Blöcke streichen.

## Burn: open 0.0000 · close 0.0556 (gemessen 2026-10-14: `session_burn`-Gesamtdelta 2.5823 → 2.6379 während dieser Session — nicht voll isoliert, das DB-Fenster verschob sich einmal) · cap 0.40 · Grund: der eine gefeuerte Punkt (MODVOLC-Lauf) + CI-Log-Triage (Default 0.15).

## Eigener CI-Befund dieser Runde (gemessen aus dem Job-Log)

- `modvolc-cdn` `38491452658` (attempt 2) **failure** — `ci_manage log`: `curl: (23)
  Failure writing output to destination`. **Ursache:** die Granulen-Extraktion
  `grep -oE 'MODVOLC[^"<> ]*\.NRT\.dat'` griff auch das href-Fragment der Autoindex-Seite
  (`…/allData/61/MODVOLC/2026/286/MODVOLC…NRT.dat`, enthält `/`), so schrieb
  `curl -o "gran/$name"` in ein **nicht vorhandenes Unterverzeichnis** → curl 23 nach dem
  ~32-min-Download der ~5000 Granulen. **Gegenmessung lokal:** der Recent-Autoindex liefert
  HTTP 200 (2 779 324 B); sowohl `Recent/<short>` als auch die href-API-v2-URL antworten 200.
  **Fix (eigen, umgesetzt):** `.github/workflows/modvolc-cdn.yml:59-66` extrahiert jetzt
  `href="[^"]*/MODVOLC[^"]*\.NRT\.dat"` (absolute URL) und lädt aus `${url##*/}`.
  Die anderen roten Läufe trägt der Stehende Pass (dort jetzt gemessen: dsn-cdn, quake-feeds-cdn).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes." | 2026-10-14 | `state/operator-gespraeche/2026-10-14-mycelium.md` |
| (standing) „lokal für Experimente sind nicht offene Lizenzen ok aber nicht veröffentlichen" | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md`; `AGENTS.md` (`local-only`) |
| (standing) „ab jetzt Reports immer in einfacher Sprache" | 2026-10-13 | dito |
| (standing) „nein du hast recht das ist wirklich nicht meine aufgabe" (CHIME/FRB + Lizenzstichprobe) | 2026-10-13 | dito |
| (standing) „ja bitte ich möchte sauber sein" (Stilllegung der drei Direktabrufe) | 2026-10-13 | dito |

## Offen — eigen

### MODVOLC — Lauf scheiterte an der Granulen-Extraktion; Fix umgesetzt
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** kein — Re-Dispatch nach Push
- **Lage:** (gemessen/umgesetzt 2026-10-14/346) Lauf `38491452658` failure, Ursache `curl (23)` wie oben; Extraktion auf den Autoindex-`href` umgestellt (`modvolc-cdn.yml:59-66`).
- **Blockade:** keine.
- **Braucht:** nach dem Commit `gh workflow run modvolc-cdn.yml`; dann `asset=present` bestätigen. Riss bleibt: LANCE nur NRT (`Recent/`).

### MODVOLC-Archiv 2000–2019 — LAADS-Anfrage gesendet, Antwort ausstehend
- **Status:** wartend | **Bindung:** wartend
- **Trigger:** Antwort von `laads-support@nasa.gov` (Mail gesendet 2026-10-14)
- **Lage:** (gemessen 2026-10-14/345) C6-Archiv (`…/archive/allData/6/MODVOLC/`, C61-Pfad 404) hinter Earthdata Login (anonymer GET → 303) + „not yet approved for public release"; Zugangsanfrage vom Operator gesendet; Auth-Route (LAADS-Hosts im EDL-Token-Set + 303 als Gate, `token.rs`) steht.
- **Blockade:** LAADS-Freigabe („not yet approved for public release").
- **Braucht:** die Antwort autonom lesen; bei Freigabe Archiv-Modus im `modvolc_compiler.rs` (C6-Pfad + `Authorization: Bearer`), dann das 2000–2019-Fenster ziehen.

### DE-2-Manifestation — Host `cdaweb.gsfc.nasa.gov` unerreichbar
- **Status:** wartend | **Bindung:** wartend
- **Trigger:** CDAWeb-Host antwortet wieder
- **Lage:** (gemessen 2026-10-14/345) compiler + Workflow + Block (`phi:586`) stehen; CI `38489787692` scheitert **gemessen am Host** (TLS `35`/`52`/`56`); lokal `--verdict` + Browser bestätigen host-weit unerreichbar (`heasarc`/`fermi` grün). `asset=fehlt` bleibt.
- **Blockade:** externer Host-Ausfall (kein Maschinen-Akt).
- **Braucht:** bei Host-Rückkehr `gh workflow run de2-cdn.yml`; dann Asset → `asset=present` + K messbar.

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
- **Lage:** (gemessen 2026-10-13/342) ICGEM `XGM2019e.gfc` (206)·gamma-cat `gammacat.ecsv` (BSD-3) gefaltet. Offen: **SEANOE** `https://www.seanoe.org/data/00311/42182/` (206; Lizenz `pending`) · **GEOFON-FDSN** (CDN-Asset 404; `geofon` fehlt im Register; Draft `state/mycelium/block-drafts/welle2-compiler.φ:1-8`).
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
- **Lage:** (gemessen river-204/208) `url`/`origin`/`compiler` der drei Zeugen (SABER `TIMED_L2A_SABER` keyless; UCAR-FPI E-Mail-gated = Operator; Athabasca ERG-SC keyless) sind Mycelium; `weberin-verdicts-cdn.yml` existiert.
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

Origin: mycelium-346 (2026-10-14):

- `xfer:mycelium-346-1` **CelesTrak — bereits erledigt** — `sgrep -i celestrak phi/` = 0; kein Block mehr zu entfernen; das `descoped`-Verdikt setzt Mountain. Echo `xfer:mountain-342-7`.
- `xfer:mycelium-344-1` **Register→Manifest** — `harvest raw_zenodo` in den Schumann-Block (`phi:1034-1043`) + Parser-Anerkennung der `harvest`-Direktive.
- `xfer:mycelium-344-2` **Fünf Fäden** — `terms`/`field`/`ttl` je gemessener Quelle.
- `xfer:mycelium-344-5` **LOD in den `finals`-Arm** ergänzen.
- `xfer:mycelium-342-3`/`-4`/`-5` · `xfer:mycelium-343-2`/`-3` · `xfer:mycelium-342-10` — wie oben geführt.

## An river

Origin: mycelium-346 (2026-10-14):

- `xfer:mycelium-344-4` **DE-2 + `asset=`-Riss** — die Riss-Klärung steht (`harvest-dispatch.yml:106` liest `asset=`); der Lauf `38489787692` scheitert **gemessen am Host** (`cdaweb.gsfc.nasa.gov` TLS-Abbruch; lokal `--verdict` bestätigt host-spezifisch). `asset=fehlt` bleibt bis zum ersten erfolgreichen Pull.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
