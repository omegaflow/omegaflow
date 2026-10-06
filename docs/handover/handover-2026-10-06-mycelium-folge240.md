<!--
  title: Handover — Mycelium-Folge 240 (2026-10-06)
  session: Mycelium-Linie — Meta-Pass: Membran-CDN-Assets gemessen, Träger geschärft, Stehender Pass
  class: handover
  date: 2026-10-06
  sha256: c6198737a7086ec88a9ea683866535792ba9adfafe52438eeea638dcb73bf1df
  status: live
-->
# Handover — Mycelium-Folge 240 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-06-mycelium-folge239.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0255

`session_burn`: diese Session ~**$0.0255** (line „Mycelium-Linie in einem Pass starten").
Rolling-Fenster (37 Sessions) bei Schluss **$0.4343**.
`bin/.tools_ensure archive_search|sgrep|sfetch|smail`: frisch (keine Meldung).

## Operator-Wort-Register

- Wort | 2026-10-06 | „Genau ein Zulassungskriterium (Presence-Hülle) und ein deklarierter Beobachter je Messung; ein Body-Name, den der Code wählt, ist der Bias" | Quelle: Operator-Session 2026-10-06 — als Regel in `AGENTS.md` `## Block Universe Physics`.
- Wort | 2026-10-06 | JAXA-G-Portal-Bestellungen (`download_limit=1` je, Fenster `2026/01/01`); die Abholung (fetch) ist der Vollzug desselben Worts | Quelle: Operator-Session 2026-10-06.

## Offen — eigen

### JAXA G-Portal — Abholung (Reihe queued)
- **Status:** wartend
- **Trigger:** `jaxa-gportal-cdn`-Reihen-Ausgang → `ci_manage list`
- **Lage:** (gemessen 2026-10-06T09:27 via `ci_manage list`) Fix committet (`jaxa-gportal-cdn.yml:3-5`); die Re-Dispatch-Läufe (`37441304379` u. a.) stehen **queued** im Runner-Stau.
- **Blockade:** Runner-Kapazität (Stau).
- **Braucht:** Reihen-Ausgang; Reader-Feld-Verdikt je Produkt (Mountain).

### ps1-cdn — Reparatur-Lauf
- **Status:** wartend
- **Trigger:** `ps1-cdn`-Lauf-Ausgang → `ci_manage view 37441624456`
- **Lage:** (gemessen 2026-10-06T09:27 via `ci_manage list`) `37441624456` **queued**; Alt-Lauf `37355972362 @410d9f2b` = failure, Grund `unread` (Jobs `cancelled`, `ci_manage log` 404).
- **Blockade:** Runner-Kapazität.
- **Braucht:** `ci_manage view 37441624456`.

### Exposom-Quellenmatrix — Matrix-Lauf-Workflow
- **Status:** eigen
- **Trigger:** Sources-Zeilen je pending Domäne + `.te`-Descriptor + Workflow-YAML → `gh workflow run`
- **Lage:** (gemessen 2026-10-06) öffentlich `docs/surveys/survey-2026-10-04-exposom-matrix.md` (12 Domänen); 4 Kern-x-Serien erreichbar/registriert (OpenAQ 206, Open-Meteo 200, NASA POWER 206, OMNIWeb 200); 2 Arme gebaut (WQP + EEA-noise, `mycelium-folge231:70-75`). x-Home-Messung: 3 Domänen netloc-registriert über abweichenden origin (Wasser `waterqualitydata.us` → `phi/sources.φ:18008`; Lärm `noise.eea.europa.eu` → `:9403`; Grünraum `lpdaac.usgs.gov` → `:9337`), 6 **unregistriert** und direkt 200 (Licht `blackmarble.gsfc.nasa.gov`; Pollen `ads.atmosphere.copernicus.eu`; gebaute Umwelt `ghsl.jrc.ec.europa.eu`; Ernährung `ers.usda.gov/data-products/food-access-research-atlas/`; Arbeit `onetonline.org`; Chemikalien `exposome-explorer.iarc.fr`). Descriptor-Form `phi/pipeline/descriptors/solar_seconds_matrix.te`, Parser `field_te_query.rs:580-684`.
- **Blockade:** die y-Serien der Matrix-Klassen sind unregistriert; eine Aufnahme braucht das Mountain-Verdikt + die Mycelium-Manifestations-Direktive.
- **Braucht:** je pending Domäne die Sources-Zeile (Verdikt + `url`/`origin`/`compiler`); dann `.te` je Klasse + `.github/workflows/exposom-matrix-te.yml`.
- **Schwarm-Messung 2026-10-06 (Claims, ungeprüft):** Kandidaten-Endpunkte — Licht `ladsweb.modaps.eosdis.nasa.gov/archive/allData/5000/VNP46A3/` (HDF5, registration), Pollen `ads.atmosphere.copernicus.eu/.../cams-europe-air-quality-forecasts` (NetCDF, registration), GHSL `data.jrc.ec.europa.eu/dataset/jrc-ghsl-10007` (GeoTIFF, open); alle 6 x-Homes erneut `--verdict` HTTP 200. Die freien Schwarm-Routen fielen teils aus (Cloudflare-Tageslimit 10 000 Neuronen; `free tier only within OpenCode`) — Kandidaten sind vor Aufnahme am Baum gegenzumessen.

### Träger `survey-2026-09-03-orphan-verdicts` — Step 5 CDN-kanonisch
- **Status:** eigen
- **Trigger:** je `*-cdn.yml` die Release-Menge aus `phi/sources.φ` lesen
- **Lage:** (gemessen 2026-09-28/2026-10-06) `docs/surveys/survey-2026-09-03-orphan-verdicts.md:103-151` — 13 Netlocs, deren Release aus ≥2 Workflow-Klassen geschrieben wird; kein `*-cdn.yml` liest seine Release-Menge aus `phi/sources.φ` (einzige gemessene Ausnahme `planetary-odf-cdn.yml:37`).
- **Blockade:** keine (Schritt ist die Messung/Bindung).
- **Braucht:** je `*-cdn.yml` die erwartete Release-Menge an `phi/sources.φ` binden (Familien-Identität ins Register; Jahr-/Slab-Menge bleibt Laufzeit-Ableitung).

### Träger `survey-2026-09-03-daten-holdings-inventur` — Ziel-Layout-Migration
- **Status:** eigen
- **Trigger:** Move je Datensatz (Operator-Wort Ziel-Layout steht 2026-09-30) → `du`-Nachmessung
- **Lage:** (gemessen 2026-10-06) Byte-Messung Schritt 2 steht (`:167-183`, 2026-09-30: `archive/knowledge` 29 Gi, `archive-state` 9,7 Gi, Repo-`data` 77 Gi); Registry-first Schritt 3 für die Staging-Kandidaten gemessen erfüllt (Ephemeriden registriert, `omni2_serie.bin` `phi/sources.φ:1420`); auf diesem Host existiert `~/knowledge`/`~/backups` nicht mehr — konsolidiert nach `~/archive/` (2026-10-06 `du`: `archive/knowledge` 29 G, `archive-state` 9,8 G, `archive/archive-root` 974 M).
- **Blockade:** Move/Löschung braucht das Operator-Wort je Datensatz (`0 honored`: nichts löschen ohne Nachbau-Quelle).
- **Braucht:** Schritt 4/5 — Unique-Byte-Move je Holding nach Freigabe.

### Register-Träger `ledger.φ:2`/`:6` — Port-Runner verloren
- **Status:** blockiert
- **Trigger:** Port-Runner im Baum
- **Lage:** (gemessen 2026-10-04) `ledger.φ:2` = 825 Blöcke, `:6` = 63; `phi/pipeline/stage/*` leer; der Ausführer war ein nie committeter Working-Tree-Bin; nur der Motor `src/archivar/port.rs`. **Archäologie 2026-10-06:** kein `port*`-Executor in irgendeinem Ref (`git log --all --name-only -- '*port*'`, 1322 Safety-Refs ab 2026-09-15) — nicht aus git rekonstruierbar; Motor `src/archivar/port.rs` (156 KB) + Protokoll `docs/SOURCE_PORT.md` stehen.
- **Blockade:** Port-Runner verloren (nicht in git).
- **Braucht:** Bin aus der `port.rs`-API rekonstruieren/committen (Konverter-Spec = Mountain).

### `phi/blocked_sources.φ` — Mycelium-Klasse
- **Status:** je eigen
- **Trigger:** Arm-Bau/Manifestation je Eintrag
- **Lage:** (gemessen 2026-10-06) ExoMars TGO ACS / Viking gravity / Hayabusa LIDAR / Phobos-2 KRFM — Workflows dispatched, queued; EUMETSAT MTG-LI — `37437023048` queued; Chandrayaan-1 Mini-RF — **blockiert** (`pds3_img` ohne Feld-Arm); Tianwen-1 MoRIC / ShadowCam / JAXA G-Portal — Sample/Record-Download = Operator-Hand; `:78` SuperDARN — LOCK.
- **Blockade:** Chandrayaan-`pds3_img`-Arm (Mountain); Sample-/Record-Downloads (Operator/per-act).
- **Braucht:** `pds3_img`-Feld-Arm (Mountain); Consent für Record-Downloads (Operator/per-act).

### iEEG-Ernte — Dienst antwortet 500
- **Status:** wartend
- **Trigger:** `www.ieeg.org/services` erholt sich → `gh workflow run ieeg-cdn.yml -f dataset=09_14_limbic_seizure_374`
- **Lage:** (gemessen 2026-10-06) Zugang steht — `IEEG_USER`/`IEEG_PASS` im `.secrets.local` (`bin/secrets_keys`), exportiert `.github/workflows/ieeg-cdn.yml:22-23`, gelesen `ieeg_compiler.rs:211-212`; `phi/sources.φ:3584` bindet `www.ieeg.org` (Mountain). `archive_search --verdict https://www.ieeg.org/services` = **HTTP 500** (Stufe 1+2, 2026-10-06).
- **Blockade:** Dienst 500 (nicht der Zugang).
- **Braucht:** Re-Dispatch bei Erholung.

### Rand ohne Rubin — Fink-Cutout-/FP-Manifestation
- **Status:** wartend
- **Trigger:** Mountains Fink-Admission im Baum → `url`/`origin`/`compiler`/Tags setzen
- **Lage:** (gemessen via future-182 addressed block, 2026-10-06) die geharvesteten FP-Assets brauchen die Manifestations-Direktiven neben Mountains Fink-Admission; verwandte bestehende Quelle ALeRCE ZTF (`phi/sources.φ:561`).
- **Blockade:** Fink-Admission (Mountain).
- **Braucht:** `phi/sources.φ`-Direktiven (Mycelium) nach Admission.

### Self-hosted Runner — Operator richtet 2011er PC ein
- **Status:** operator-gebunden
- **Trigger:** Wort (Operator richtet den 2011er PC ein, 2026-10-06) → Runner-Registrierung + Routing
- **Lage:** (gemessen 2026-10-06) Operator-Wort 2026-10-06: „ok ich schaue ob ich einen gaming pc von 2011 zum laufen bekomme und da linux mint drauf installiere um ihn als runner laufen zu lassen". Mehrere Jobs binden `[self-hosted, Linux]` (u. a. `de44-cdn.yml:18`). Präzedenz `t420` (sensory-233 → mycelium-folge233).
- **Blockade:** Maschine noch nicht registriert (Operator).
- **Braucht:** Runner-Registrierung (Labels) + Routing-Entscheidung für die `hyperscanning-te`-Screen-Jobs.

## An mountain  ·  PRIO

Origin: mycelium-folge240. **Routed — nicht-eigen:**

- **`ci-gate` rot am Baum (gemessen 2026-10-06, HEAD `32e60f52`):** `src/archivar/mtg_li.rs:329:12` clippy `neg_cmp_op_on_partial_ord` (`if !(value > 0.0)`) + `:343:1` `items after a test module` → `cargo check`/clippy mit `-D warnings` kompiliert lib + lib test nicht; Herkunft `17d626ef7` (mountain 239). Der Fehler steht am aktuellen HEAD unverändert (gemessen `sread`). **Braucht:** `!(value > 0.0)` → `value <= 0.0`, Test-Modul ans Dateiende.
- **JAXA G-Portal — Reader-Feld-Verdikt:** `sources.φ:9512` trägt keine `field`-Zeile; Produktpriorität AMSR2 L2 SST/Wind/SMC, GPM-L2 Regen, SGLI. Braucht: Feld-Verdikt je Produkt.
- **Chandrayaan-1 Mini-RF:** `sources.φ:10019` `pds3_img`-Block ohne `field`-Zeile; fehlender `pds3_img`-Feld-/Slug-Arm (`sources.φ:17053-17055`). Braucht: `pds3_img`-Feld-Arm + force/unit-Verdikt.
- **gistemp_aod550 / godas_pottmp:** godas `37434149825` = success; gistemp `37434146422` → Stand via `ci_manage view`; danach `sha256`-Nachzug (`1ba4e901…`/`c2448a10…`) in `sources.φ`.
- **GOES-18 ABI:** `goes18-cdn.yml` gebaut, `37434142761`; `goes_abi`-Workflow-Angleich prüfen.
- **AGrav/CEEIN-Entries** in `blocked_sources.φ` — gegen den Baum re-measured; release oder descope.
- **DHM-Triangulation Dhunche→Bhorle — Messung 2026-10-06 (Mycelium, geroutet):** Fenster `dhm_4657_stage.txt`/`dhm_4661_stage.txt` = 2026-10-06 00:05–10:15 UTC, 62 × 600 s. Dhunche 4657: 2,1661→2,1575 m (−8,6 mm, sd 6,1 mm, fast flach). Bhorle 4661: 4,0730→3,8080 m (−26,5 cm, sd 9,7 cm, Rezession). Kreuzkorrelation r(τ) flach-maximal bei τ≈+10…+80 min (r 0,56–0,61) — von der gemeinsamen Rezession getragen, **kein auflösbarer Laufzeit-Lag**. Open-Meteo-archive am Einzugsgebiet (28,098/85,319): Regen 10-04 (bis 1,8 mm/h), leicht 10-05/06, kein Puls im Fenster. **Befund:** das Paar trägt, aber die Laufzeit braucht ein nasses Anstiegs-Ereignis (Puls an Dhunche, später an Bhorle) und ein längeres Fenster als ~10 h.

## An river

Origin: mycelium-folge240. **Routed — nicht-eigen:**

- **Membran-Assets — CDN-Duty gemessen erfüllt (2026-10-06):** `ephemeris_de440_{earth,moon,sun}.bin` im Release `ssd.jpl.nasa.gov-de` mit den in `pages-deploy.yml:60-62` erwarteten sha256 (`adc990bc…`/`acb42881…`/`9d059db3…`, via `--sniff`); `dr3_stars.bin` `745a3f71…` (`omega_sh sha` über den Volldownload); alle vier **same-origin** `omegaflow.github.io/omegaflow/<name>` HTTP 206 (`--verdict`). Der „anchoring bodies…"-Hang ist kein Manifestations-Defekt — die Rest-Ursache liegt im Client-Pfad (River).
- **`static/membrane.html:43` BODIES-Handkopie** (`["earth","moon","sun"]`) → Build-Time-Manifest aus der Hüllen-Pipeline (Rivers Agnosis-Rest, kein Mycelium-Akt).

## An future

Origin: mycelium-folge240.

- **Tavily-Quota 80 %** (`mail_ledger.φ`, ts 1791121265) → Fallback `--mwmbl`/`--marginalia`.
- **Kimi-K3-Gratis-Route** (NVIDIA NIM `moonshotai/kimi-k3`, kein Kartenzwang): Developer-Account/Key = Operator-Akt → Operator-Queue.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.

## Abschluss

Der Stehende Pass wird **nach** Commit + Push am neuen HEAD neu gestempelt
(`state/zustand/standing-pass.md`). Detail der Runde: der Pass.
