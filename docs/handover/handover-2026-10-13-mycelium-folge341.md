<!--
  title: Handover — Mycelium-Folge 341 (2026-10-13)
  class: handover
  date: 2026-10-13
  sha256: 2260bd5716f606d2eb1a978f145852160335bb665718db98c0f366687092182c
  status: live
-->
# Handover — Mycelium-Folge 341 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-13-mycelium-folge340.md` (→ `archiv/`).

## Burn: open 0.00 · close 0.0393 — gemessen `opencode.db` beim Handover-Schreiben (offen; wächst bis Session-Schluss); deepseek-flash, kein pro/max.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „hast du die ttls gewürfelt?" | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md` |
| „mache weiter" (weitergetragener folge337-Bericht) | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md` |
| „ja und den rest bauen schick einfach agenten" | 2026-10-12 | `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass … /commit (Push), danach den frischen Pass" | 2026-10-12/13 | session-weiter Delegations-Consent |
| „ab jetzt Reports immer in einfacher Sprache; Maschinensprache untereinander ok" | 2026-10-13 | `state/operator-gespraeche/2026-10-13-mycelium.md`; `AGENTS.md` (Language doctrine) |

## Offen — eigen

### Ersatzquellen-Registerblöcke — Format der 4 Restcompiler (Klassen-Träger)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** ein Format-Name je Restcompiler von Mountain
- **Lage:** (gemessen 2026-10-13/339) die 4 Compiler `heasarc_w3browse`/`jpl_sbdb`/`gesdisc_gnssro`/`opentopography_raster` printen keinen vollständigen Block (kein `FORMAT`/`COMPILER`-const, `--out` Pflicht); `opentopography_raster` kollidiert mit dem vorhandenen `SRTM15_V2.6`-Block (`phi:29896`). An Mountain getragen: `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af`.
- **Blockade:** Format/Format-Name ist Mountains Recht.
- **Braucht:** Mountain typisiert die 4 Restcompiler + registriert sie.

### LHAASO/VizieR `J/ApJS/271/25` (Ersatz für `casdc`)
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `at`/`field` + das CI-Fetch-Wiring
- **Lage:** (gemessen 2026-10-13/338) Block `phi/sources.φ:10799-10802`; Compiler `cargo build -p omegaflow-harvest --bin lhaaso_compiler` grün; liest ASU-TSV. An Mountain: `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.2~c28e0b91`.
- **Blockade:** `at`/`field` = Mountains Recht; kein CI-Workflow ruft `lhaaso_compiler`.
- **Braucht:** Mountain `at sun`/`field`; danach das Fetch-Wiring.

### `register_sort` — Kanonisierung fehlt (`--write`)
- **Status:** eigen (Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains Lauf `register_sort --write phi/sources.φ`
- **Lage:** (gemessen 2026-10-13/340) ttl-Riss geschlossen (`544b136f3`); allein die Ordnung — exit 1, 4 ttl-order + 22 url-order über 2591 Blöcke.
- **Blockade:** `phi/sources.φ` ist Mountains Register; `--write` ist Ganz-Datei-Eingriff.
- **Braucht:** Mountain fährt `register_sort --write` in einem ruhigen Atom.

### OCNS2019 — Ersatzblock für `geoazur`/APDB — Mycelium-Teil gebaut
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** Mountains `terms` + `field`
- **Lage:** (gemessen 2026-10-13/341) **Mycelium-Teil steht** als Entwurf `state/mycelium/block-drafts/welle3-rest-nuts.φ`: `url .../paperdata.china-vo.org/ocns2019.zip`, `origin http://paperdata.china-vo.org/yuanye/ocns2019.zip`, `format zip`, `at sun`, `no-cadence` (OCNS2019 = 24 996 Positionen, 11 Neptun-Monde, 1847–2019, ICRS/TT; DOI `10.12149/101048`; Host nur via Wayback). `terms` (China-VO PaperData) + `field`-Mapping (Neptun-Astrometrie) fehlen → bezweckt. Gefaltet: `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.10~7d5c9163`.
- **Blockade:** `terms`/`field` = Mountains Recht; Host nur über Wayback (sha `7b7ad66b…`).
- **Braucht:** Mountain setzt `terms` + `field`; dann Fold + CI-Manifestation.

### BGR-Infraschall-Ersatzquellen (river-205) — Endpunkt pending
- **Status:** eigen (Rest Mountain-gebunden) | **Bindung:** eigen
- **Trigger:** ein gemessener Datei-Endpunkt (Format/Klasse)
- **Lage:** (gemessen river-205, 2026-10-13) DOIs `10.25928/bgrseis_mbhf-ifsd` (CC BY-NC-SA 4.0) + `10.25928/bgrseis_hist-ifsd` (CC BY 4.0); `gdk.gdi-de.org` HTTP 200, `download.bgr.de` direkt 403. Entwurf-Notiz in `welle3-rest-nuts.φ`; **kein `format`/`compiler`/Datei-URL gemessen** (Verifikations-Läufe in diesem Atom im Timeout) → `pending`, nie geraten. Tokens `xfer:river205-1` / `xfer:river205-2`.
- **Blockade:** der konkrete netCDF-Datei-Endpunkt ist nicht gemessen.
- **Braucht:** der Datei-Endpunkt gemessen (`archive_search --verdict` auf die DOI-Landing + gdk-Katalog), dann Block + Compiler.

### Lizenzklare Alternativen — Register-/Compiler-Swap (future-248)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je Alternative ein Compiler + Block gebaut
- **Lage:** (gemessen 2026-10-13/341) Bau-Liste `state/mycelium/alternativen-endpunkte-2026-10-13.md` (~20 `block`, ~30 `compiler`, Rest-Nüsse). Wellen 1+2 (43 Blöcke) in folge340 eingefaltet. **CDN-Orphan-Takedown in folge341 erledigt:** 36 Orphan-Releases von `omegaflow/sources` offline genommen (377→341) — `services1..9.arcgis.com`/`services-ap1`, `minorplanetcenter.net`, `api.lsst.fink-portal.org`, `jades.herts.ac.uk`, `maxi.riken.jp`, `clpds.bao.ac.cn`, `data.kasi.re.kr`, `ceein.infp.ro`, `blinkverse.zero2x.org`, `babamul.caltech.edu`, `blpd0.ssl.berkeley.edu`, `hinetwww11.bosai.go.jp`, `igetsftp.gfz.de`, `modis.higp.hawaii.edu`, `opendata.magic.pic.es`, `padc-tap-rcsed.obspm.fr`, `voparis-tap-astro-m.obspm.fr`, `vo.lmd.jussieu.fr`, `media.githubusercontent.com`, `argovis-api.colorado.edu`, `lasp.colorado.edu`(+`-eve`), `api.alerce.online`, `resonanceone.app`, `sheffieldcitycouncil.cloud.esriuk.com`, `www.hou.usra.edu`, `api.wolfx.jp`. **Gehalten (Register trägt Blöcke, kein Orphan):** `raw.githubusercontent.com` (fieldtrip GPL-3.0 `standard_1005.elc`/`standard_bem.mat`, PETREL19, VERITAS) · `services.arcgis.com` (GOES_GLM_Bolides, Environment-Canada-Wind, EEA-Noise, AHN4). **Riss:** Future-249 meldete „jeder fragliche Tag steht auf 0 Assets"; gemessen standen 36 Releases, teils mit Assets (media=2, argovis=2, lasp=5+1, wolfx=7, hou.usra=1, resonanceone=1, sheffield=1, alerce=1). Tokens `xfer:mountain-336-1` (Takedown, erledigt) · `xfer:mountain-336-2` (Manifestation folgt) · `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.1~8f771362`.
- **Blockade:** `url`/`origin`/`compiler` = Mycelium, `terms`/`field`/`ttl` = Mountain.
- **Braucht:** je Alternative ein bounded step (Block bauen, dann Mountains `terms`/`field`); die Rest-Nüsse (VizieR II/288 · III/279 · J/ApJS/257/59 Subtabellen-ID; ICGEM `getmodel`-Hash; gamma-cat Einzeldatei).

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass oder eine `archive_search`-Arm-Änderung
- **Lage:** (gemessen 2026-10-13/340) auf den `archive_search --help`-Stand gesynct. Ruht.
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben.

## An mountain

Origin: mycelium-341 (2026-10-13):

- `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af` **Format-Name für die 4 Restcompiler** `heasarc_w3browse`/`jpl_sbdb`/`gesdisc_gnssro`/`opentopography_raster`.
- `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.2~c28e0b91` **`at`/`field` für den LHAASO/VizieR-Block + CI-Fetch-Wiring** (`phi/sources.φ:10799-10802`).
- `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.2~98652e66` **Ersatzquellen-Register + Risse** (MAXI/OSC/IGETS/MODVOLC/DES/ALFALFA/LoTSS/TeVCat) — Verdikt offen.
- `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.3~7d22b061` **Welle 3: 2 Schranken** — HiPS-HEALPix-WCS-Arm in `src/archivar/fits.rs` nötig, 4LAC als eigene Quelle oder gestrichen?
- **`register_sort` Kanonisierung:** `register_sort --write phi/sources.φ` (4 ttl-order + 22 url-order). `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.3~8eff6a4e`.
- **CDN-Orphan-Takedown erledigt (folge341)** — 36 Releases offline; die zwei Register-tragenden (`raw.githubusercontent.com`, `services.arcgis.com`) gehalten. `xfer:mountain-336-1`.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
