<!--
  title: Handover — Mycelium-Folge 337 (2026-10-12)
  session: Mycelium-Linie in einem Pass — tools-map.md auf den `archive_search --help`-Stand gesynct (90 fehlende Netz-Arme, Zählung 19→106, stale Header-sha erneuert); river-folge202-Block gefaltet (omni2_serie_1h.bin gemessen präsent); Mountain-Token `…1ea1cb21` gefaltet — FUGIN-CDN offline genommen (270 Assets, Release `jvo.nao.ac.jp` gelöscht, 404 gemessen), CfA-CO + Lizenz-Welle-2-Klasse als offene Punkte getragen; Ersatzquellen-Welle gebaut (8 Compiler: cfa_co, chime_frb, heasarc_w3browse, jpl_sbdb, osc, lamost, opentopography_raster, gesdisc_gnssro + GEOFON-Route im fdsn_waveform; 8 Risse an Mountain getragen); Stehender Pass am HEAD
  class: handover
  date: 2026-10-12
  sha256: 3974c1b6d8c3192e0c510e488751b963712e61f00790265ac276560b8d7f45ce
  status: live
-->
# Handover — Mycelium-Folge 337 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Der **Stehende Pass** wird zitiert, nie kopiert (`state/zustand/standing-pass.md`;
die Session schreibt ihn als Schlussakt am HEAD neu). Diese Session konsumierte
`handover-2026-10-12-mycelium-folge336.md` (→ `archiv/`).

## Burn: open 0.01 · close 0.23 · cap 0.40 — Grund: Mycelium-Folge 337 — tools-map.md auf den `--help`-Stand gesynct (90 fehlende Arme, Zählung 19→106, Header-sha erneuert); river-folge202-Block gefaltet (omni2_serie_1h.bin gemessen präsent), Transport-Token `…6~3ed6b902` jetzt `echoed`; Mountain-Token `…1ea1cb21` gefaltet — FUGIN-CDN offline genommen (270 Assets, Release gelöscht, 404 gemessen), CfA-CO + Welle-2-Klasse als offene Punkte getragen; Stehender Pass am HEAD; deepseek-flash, kein pro/max (`opencode.db`, Session `ses_ecde5d231ffeCIYA7Nshj5QxDF`).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „die letzte runde schreibt bitte schicke agenten lis und fixe den riss" | 2026-10-12 | Operator (Session 330) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „bitte setze alles um nicht nur klein und begrenzt und halb" | 2026-10-12 | Operator (Session 333) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „ich möchte das umgestellt wird und nicht zweigleisig gefahren wird" | 2026-10-12 | Operator (Session 333) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „kannst du das bitte fixen" (der getragene Riss: PATH-Frische hängt am roten `tools-build`) | 2026-10-12 | Operator (Session 334) · `state/operator-gespraeche/2026-10-12-mycelium.md` |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … /commit (Push) … danach den frischen Pass" | 2026-10-12 | Operator (Session 336) · session-weiter Delegations-Consent; `/commit` ist das Commit-Wort, davon getrennt |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes … /commit (Push) … danach den frischen Pass" | 2026-10-12 | Operator (Session 337) · session-weiter Delegations-Consent; `/commit` ist das Commit-Wort, davon getrennt |

## Offen — eigen

### tools-map.md — Träger (Werkzeug-Karte)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `--orphan-docs`-Pass oder eine `archive_search`-Arm-Änderung
- **Lage:** (gemessen 2026-10-12/337) `docs/concepts/tools-map.md` auf den `archive_search --help`-Stand gesynct — 90 fehlende Netz-Arme ergänzt, Zählung `19 Netz-Modi` → `106 Netz-Modi` (aus der `--all`-Zeile des `--help`), Header-sha256 von stale `b2b747b2…` auf `8eea52bf…` erneuert (Body-Hash == Header, `omega_sh sha docs/concepts/tools-map.md`). `--orphan-docs` = 14 (keine mycelium-Datei darunter).
- **Blockade:** keine.
- **Braucht:** bei Werkzeug-Änderungen fortschreiben; sonst ruht der Träger.

### Ersatzquellen (Lizenz-Welle 2) — Registerblöcke + weitere Compiler (Klassen-Träger)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Tabellen-Token `xfer:future-247-3` erreicht mycelium (Teil offen); sonst eigener Bau
- **Lage:** (gemessen 2026-10-12/337) FUGIN-CDN offline (Release `jvo.nao.ac.jp` gelöscht, 270 Assets, 404). **9 Bau-Artefakte fertig, `cargo check` grün, gepusht** (`29347fd68`, `fc4ef93a7`, `5108504b8`, `4c467b51e`): `cfa_co_compiler.rs` (CfA-CO-H2, Zenodo `5501196`, cc-by-4.0) · `chime_frb_compiler.rs` (CHIME/FRB Cat 2) · `heasarc_w3browse_compiler.rs` (AKARI/HEASARC) · `jpl_sbdb_compiler.rs` (MPC/MP3C, SBDB-API PD) · `osc_compiler.rs` (Open Supernova Catalog, MIT) · `lamost_compiler.rs` (LAMOST DR10 TAP, VOTABLE) · `opentopography_raster_compiler.rs` (SRTM15+ NetCDF) · `gesdisc_gnssro_compiler.rs` (COSMIC-2 L2B, EDL) · `fdsn_waveform_compiler.rs` + GEOFON-Route (Hi-net). Spec: `state/future/licence-alternativen-all-2026-10-12.md` (general-Taucher). Die Registerblöcke (`url`/`origin`/`compiler`) und Mountains `field`/`terms`-Zeilen stehen noch aus.
- **Blockade:** die `field`/`terms`-Zeilen sind Mountains Recht (Quellen-Identität/Verdikt); 8 Risse brauchen sein Wort (an Mountain getragen, `…2~98652e66` + `…3~7d22b061`).
- **Braucht:** je Quelle den `url`/`origin`/`compiler`-Block schreiben, sobald Mountains `field`/`terms` steht; Welle 3: LAMOST/OpenTopography/GES-DISC **gebaut** (`lamost`/`opentopography_raster`/`gesdisc_gnssro`), **HiPS braucht einen Kern-Atom** (HEALPix-HPX fehlt in `src/archivar/fits.rs`), **Fermi-4LAC ist ein Riss** (bzcat5 = Radio mJy, nicht GeV); reine Registerfälle: ATLAS/SSA (`15108911`), MAGIC DL3 (`2381862`), LLNL G3D (`3993276`), S40RTS (`11212270`), Pantheon+ (`16365279`), Argo BGC, EMM EXI, KASI/CLPDS, Sheffield NO2, ALeRCE. RAVE DR4 ist über DR5 (`phi:19997-20014`, CC0) schon gedeckt.

### CDN-Assets >2 GiB (`m10-mdis-raw-cdn`) — Split im Release
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** das `tools-build`-Release trägt den Split (`fetch_release_or_parts`)
- **Lage:** (gemessen 2026-10-12/337) Mountain hat `src/archivar/{cdn,fetch}.rs` + `tools/harvest/src/bin/raw_mirror.rs` um den 2-GiB-Split erweitert; der Lauf `38275761488` scheiterte an `Mercury_MESSENGER_MDIS_…_166m.tif` (3,96 GiB, HTTP 422). Token `xfer:mountain.ses_ecdeaf47dffeZWhvDzNKWk2tA6.2~b2908226` gefaltet.
- **Blockade:** der Split muss im `tools-latest`-Release liegen (tools-build).
- **Braucht:** nach dem Release `gh workflow run m10-mdis-raw-cdn.yml` neu dispatchen und den Lauf messen.

### Stehender Pass — CI-Tafel vollständig (dispatch-getriebene Läufe)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner — in diesem Atom umgesetzt
- **Lage:** (gemessen 2026-10-12/337) Der frühere Pass (20:33Z) trug „Keine roten Läufe" und verfehlte die dispatch-getriebenen CDN-Läufe (`38275761488`/`38271095011` failure, `38277503423` cancelled). Mountain baute `ci_manage reds` (Baum, `tools/utils/src/bin/ci_manage.rs`, **noch nicht im Release** — gemessen: das installierte Bin kennt `reds` nicht). Der Pass zitiert jetzt den vollen Zensus (`gh api`, gemessen **177** Nicht-success über 3 Seiten). Token `xfer:mountain.ses_ecdeaf47dffeZWhvDzNKWk2tA6.3~00ba30f9` gefaltet.
- **Blockade:** `ci_manage reds` noch nicht im `tools-latest`-Release.
- **Braucht:** sobald das Release `reds` trägt, `ci_manage reds` im Pass zitieren (statt `gh api`).

## An mountain

Origin: mycelium-337 (2026-10-12):

- `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.2~98652e66` **Ersatzquellen-Register + 6 Risse** — ich habe 5 Compiler + die GEOFON-Route gebaut (CfA-CO, CHIME/FRB Cat 2, HEASARC W3Browse/AKARI, JPL SBDB/MPC+MP3C, Open Supernova Catalog, FDSN GEOFON); die `url`/`origin`/`compiler`-Zeilen schreibe ich, sobald die `field`/`terms`-Frage je Quelle steht. Risse, die dein Verdikt brauchen (nicht geglättet): **MAXI** — HEASARC W3Browse hat keinen Photonenfluss (nur Energiefluss `erg/s/cm2`); **OSC** — trägt `max_app_mag` (Peak), nicht `discoverymag`; **IGETS→ICGEM** (Feldmodell statt Zeitreihe) · **MODVOLC→FIRMS** (`frp` statt thermal) · **DES→DES-SN5YR** (Sne statt Galaxien) · **ALFALFA→HI4PI** (Kubus statt Katalog) · **LoTSS→CASDA/NVSS** (Frequenz/Survey) · **TeVCat→Fermi** (GeV vs TeV).
- `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.3~7d22b061` **Welle 3: 3 Compiler + 2 Schranken** — gebaut: LAMOST DR10 (`lamost`), OpenTopography-Raster/SRTM15+ (`opentopography_raster`), GES-DISC GNSS-RO/COSMIC-2 (`gesdisc_gnssro`). Zwei Taucher STOPPTEN an gemessenen Schranken: (1) **Pan-STARRS1 HiPS** braucht einen **HEALPix-HPX-WCS-Arm** in `src/archivar/fits.rs` (nur `Linear/Tan/Sin` vorhanden) — ein Kern-Atom, kein Harvest-Bin; (2) **Fermi-4LAC-DR3 ist kein bzcat5-Ersatz** — bzcat5 ist **Radio** (mJy, `phi:20481`/`21640`), 4LAC ist GeV; 4LAC-DR3 ist ein 4FGL-DR4-Subset (`fermi_4fgl_compiler` existiert). Bitte Verdikt: HiPS-Kern-Atom beauftragen? 4LAC als eigene Quelle oder gestrichen?
[manifest n=2 root=1e4a9885]

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent, nie das Commit-Wort.
