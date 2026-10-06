<!--
  title: Handover — Mycelium-Folge 239 (2026-10-06)
  session: Mycelium-Linie — Meta-Pass: Exposom-x-Homes gemessen, adressierte Blöcke gefaltet, Stehender Pass
  class: handover
  date: 2026-10-06
  sha256: 0d61f6ddf4efc469faa6443de93b7b54f947f43836a901faffa75e0e4690b6a8
  status: live
-->
# Handover — Mycelium-Folge 239 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-06-mycelium-folge238.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0272

`session_burn`: diese Session ~**$0.0272** (line „Mycelium-Linie in einem Pass starten" $0.0211
+ grind-flash $0.0061). Rolling-Fenster (25 Sessions) bei Schluss **$0.2843** (ältere Sessions
rollen aus dem Fenster). `bin/.tools_ensure archive_search|sgrep`: frisch (keine Meldung).

## Operator-Wort-Register

- Wort | 2026-10-06 | „Genau ein Zulassungskriterium (Presence-Hülle) und ein deklarierter Beobachter je Messung; ein Body-Name, den der Code wählt, ist der Bias" | Quelle: Operator-Session 2026-10-06 — als Regel in `AGENTS.md` `## Block Universe Physics`.
- Wort | 2026-10-06 | JAXA-G-Portal-Bestellungen (`download_limit=1` je, Fenster `2026/01/01`); die Abholung (fetch) ist der Vollzug desselben Worts | Quelle: Operator-Session 2026-10-06.

## Offen — eigen

### JAXA G-Portal — Abholung (Reihe queued)
- **Status:** wartend
- **Trigger:** `jaxa-gportal-cdn`-Reihen-Ausgang → `ci_manage list` (kein Polling)
- **Lage:** (gemessen 2026-10-06T09:17 via `ci_manage list`) Fix committet (`jaxa-gportal-cdn.yml:3-5` Gruppe um `${{ github.event.inputs.dataset }}`); die 11 Re-Dispatch-Läufe `37441247309` + `37441267585`–`37441304379` stehen **queued** im Runner-Stau.
- **Blockade:** Runner-Kapazität (Stau).
- **Braucht:** Reihen-Ausgang; Reader-Feld-Verdikt je Produkt (Mountain).

### ps1-cdn — Reparatur-Lauf dispatched
- **Status:** wartend
- **Trigger:** `ps1-cdn`-Lauf-Ausgang → `ci_manage view <id>`
- **Lage:** (gemessen 2026-10-06T09:17 via `ci_manage view 37441624456`) **queued**; Alt-Lauf `37355972362 @410d9f2b` = failure, Grund `unread` (Jobs `cancelled`, `ci_manage log` 404).
- **Blockade:** Runner-Kapazität.
- **Braucht:** `ci_manage view 37441624456`.

### Exposom-Quellenmatrix — Matrix-Lauf-Workflow
- **Status:** eigen
- **Trigger:** Sources-Zeilen je pending Domäne + `.te`-Descriptor + Workflow-YAML → `gh workflow run`
- **Lage:** (gemessen 2026-10-06) öffentlich `docs/surveys/survey-2026-10-04-exposom-matrix.md` (12 Domänen); 4 Kern-x-Serien erreichbar/registriert (OpenAQ 206, Open-Meteo 200, NASA POWER 206, OMNIWeb 200); 2 Arme gebaut (WQP + EEA-noise, `mycelium-folge231:70-75`). **x-Home-Messung (grind-flash, 2026-10-06):** 9 pending Domänen gemessen — 3 netloc-registriert über abweichenden origin (Wasser `waterqualitydata.us` → `phi/sources.φ:18008`; Lärm `noise.eea.europa.eu` → `:9403`; Grünraum `lpdaac.usgs.gov` → `:9337`), 6 **unregistriert** und direkt 200 (Licht `blackmarble.gsfc.nasa.gov`; Pollen `ads.atmosphere.copernicus.eu`; gebaute Umwelt `ghsl.jrc.ec.europa.eu`; Ernährung `ers.usda.gov/data-products/food-access-research-atlas/`; Arbeit `onetonline.org`; Chemikalien `exposome-explorer.iarc.fr`). Descriptor-Form `phi/pipeline/descriptors/solar_seconds_matrix.te`, Parser `field_te_query.rs:580-684`.
- **Blockade:** die y-Serien (Gesundheits-Outcome) der Matrix-Klassen sind unregistriert; eine Aufnahme braucht das Mountain-Verdikt + die Mycelium-Manifestations-Direktive.
- **Braucht:** je pending Domäne die Sources-Zeile (Verdikt + `url`/`origin`/`compiler`); dann `.te` je Klasse + `.github/workflows/exposom-matrix-te.yml`.

### Träger der zwei Surveys mit offenem Marker
- **Status:** eigen
- **Trigger:** Marker-Schließung
- **Lage:** (gemessen 2026-10-06, `register_lookup --orphan-docs` = 1 mit diesem Träger) `docs/surveys/survey-2026-09-03-orphan-verdicts.md`: Step 5 „CDN-kanonisch" offen (`:100-126`) — jedes `*-cdn.yml` soll seine Release-Menge an `phi/sources.φ` binden statt eigener Tag-Sätze (Familien-Identität ins Register, Jahr-/Slab-Menge Laufzeit-Ableitung); `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md`: `## Offen / Befunde` (`:96`), Byte-Messung je Holding, Registry-first.
- **Blockade:** keine (Schritt ist die Messung).
- **Braucht:** Step-5-Bindung je `*-cdn.yml`; Byte-Messung je Holding.

### Register-Träger `ledger.φ:2`/`:6` — Port-Runner verloren
- **Status:** blockiert
- **Trigger:** Port-Runner im Baum
- **Lage:** (gemessen 2026-10-04) `ledger.φ:2` = 825 Blöcke, `:6` = 63; `phi/pipeline/stage/*` leer; der Ausführer war ein nie committeter Working-Tree-Bin; nur der Motor `src/archivar/port.rs`.
- **Blockade:** Port-Runner verloren.
- **Braucht:** Port-Runner als Bin rekonstruieren/committen (Konverter-Spec = Mountain).

### `phi/blocked_sources.φ` — Mycelium-Klasse
- **Status:** je eigen
- **Trigger:** Arm-Bau/Manifestation je Eintrag
- **Lage:** (gemessen 2026-10-06) ExoMars TGO ACS / Viking gravity / Hayabusa LIDAR / Phobos-2 KRFM — Workflows dispatched, queued; EUMETSAT MTG-LI — `37437023048` queued; Chandrayaan-1 Mini-RF — **blockiert** (`pds3_img` ohne Feld-Arm); Tianwen-1 MoRIC / ShadowCam / JAXA G-Portal — Sample/Record-Download = Operator-Hand; `:78` SuperDARN — LOCK.
- **Blockade:** Chandrayaan-`pds3_img`-Arm (Mountain); Sample-/Record-Downloads (Operator/per-act).
- **Braucht:** `pds3_img`-Feld-Arm (Mountain); Consent für Record-Downloads (Operator/per-act).

### iEEG-Ernte — Backend 503
- **Status:** wartend
- **Trigger:** iEEG-Backend erholt sich → `gh workflow run ieeg-cdn.yml -f dataset=09_14_limbic_seizure_374`
- **Lage:** (gemessen 2026-10-06) `phi/sources.φ:3584` bindet `www.ieeg.org` (Mountain); `ieeg_compiler.rs:12` `NETLOC = "www.ieeg.org"`.
- **Blockade:** Backend 503.
- **Braucht:** Re-Dispatch bei Kapazität.

### Membran-Assets — `dr3_stars.bin` / `ephemeris_de440_*`
- **Status:** wartend
- **Trigger:** Assets am CDN → `archive_search --sniff <url>`
- **Lage:** (gemessen 2026-10-05/06) `membrane.html` hängt bei „anchoring bodies…"; `/dr3_stars.bin` und `/ephemeris_de440_{earth,moon,sun}.bin` kommen nicht durch; river-97/98/101/102 adressiert. Ohne Remanifestation bleibt der Anker lokal (Register-Schuld).
- **Blockade:** `de_compiler`-Ausgang + `dr3_stars.bin` nicht remanifestiert.
- **Braucht:** DE440-Assets bauen (`de_compiler`) + `dr3_stars.bin` → CI/CDN, `pages-deploy.yml` same-origin, `--verdict`/`--sniff`.

### Rand ohne Rubin — Fink-Cutout-/FP-Manifestation
- **Status:** wartend
- **Trigger:** Mountains Fink-Admission im Baum → `url`/`origin`/`compiler`/Tags setzen
- **Lage:** (gemessen via future-182 addressed block, 2026-10-06) die geharvesteten FP-Assets brauchen die Manifestations-Direktiven neben Mountains Fink-Admission; verwandte bestehende Quelle ALeRCE ZTF (`phi/sources.φ:561`).
- **Blockade:** Fink-Admission (Mountain).
- **Braucht:** `phi/sources.φ`-Direktiven (Mycelium) nach Admission.

### Self-hosted Runner — Operator richtet 2011er PC ein
- **Status:** wartend
- **Trigger:** Operator-Wort 2026-10-06 (Maschine läuft) → Runner-Registrierung + Routing
- **Lage:** (gemessen 2026-10-06 via sensory-239 addressed block) Operator-Wort 2026-10-06: „ok ich schaue ob ich einen gaming pc von 2011 zum laufen bekomme und da linux mint drauf installiere um ihn als runner laufen zu lassen". Präzedenz `t420` (sensory-233 → mycelium-folge233).
- **Blockade:** Maschine noch nicht registriert (Operator).
- **Braucht:** Runner-Registrierung (Labels) + Routing-Entscheidung für die `hyperscanning-te`-Screen-Jobs.

## An mountain  ·  PRIO

Origin: mycelium-folge239. **Routed — nicht-eigen:**

- **`ci-gate` rot am Baum (gemessen 2026-10-06, HEAD `e4bf4a53e`):** `src/archivar/mtg_li.rs:329:12` clippy `neg_cmp_op_on_partial_ord` (`if !(value > 0.0)`) + `:343:1` `items after a test module` → `cargo check`/clippy mit `-D warnings` kompiliert lib + lib test nicht; Herkunft `17d626ef7` (mountain 239). Der Fehler steht am aktuellen HEAD unverändert. **Braucht:** `!(value > 0.0)` → `value <= 0.0`, Test-Modul ans Dateiende.
- **JAXA G-Portal — Reader-Feld-Verdikt:** `sources.φ:9512` trägt keine `field`-Zeile; Produktpriorität AMSR2 L2 SST/Wind/SMC, GPM-L2 Regen, SGLI. Braucht: Feld-Verdikt je Produkt.
- **Chandrayaan-1 Mini-RF:** `sources.φ:10019` `pds3_img`-Block ohne `field`-Zeile; fehlender `pds3_img`-Feld-/Slug-Arm (`sources.φ:17053-17055`). Braucht: `pds3_img`-Feld-Arm + force/unit-Verdikt.
- **gistemp_aod550 / godas_pottmp:** godas `37434149825` = success; gistemp `37434146422` → Stand via `ci_manage view`; danach `sha256`-Nachzug (`1ba4e901…`/`c2448a10…`) in `sources.φ`.
- **GOES-18 ABI:** `goes18-cdn.yml` gebaut, `37434142761`; `goes_abi`-Workflow-Angleich prüfen.
- **AGrav/CEEIN-Entries** in `blocked_sources.φ` — gegen den Baum re-measured; release oder descope.

## An river

Origin: mycelium-folge239. **Routed — nicht-eigen:**

- **DE440-`.bin`-Remanifestation:** `ephemeris_de440_{earth,moon,sun}.bin` nach `de_compiler`-GM-Landung über die CI zur CDN; `pages-deploy.yml` stagt sie same-origin. Checkmark: Browser-Re-Messung `nearCount(<1e13 m) > 0`.
- **`static/membrane.html:43` BODIES-Handkopie** (`["earth","moon","sun"]`) → Build-Time-Manifest aus der Hüllen-Pipeline (Rivers Agnosis-Rest, kein Mycelium-Akt).

## An sensory

Origin: mycelium-folge239.
- **paper-check `37440563245 @bfb010ead` = failure, gemessener Grund:** `hyperscanning-te-method` Titel 93 Zeichen > 75 (`ci_manage log 37440563245`: `hyperscanning-te-method 93/93/long`). Braucht: Titel ≤75 kürzen.
- `docs/paper/hyperscanning-te-preregistration.md` — Träger in deiner Übergabe oder `descoped` (Zensus `register_lookup --orphan-docs`).

## An future

Origin: mycelium-folge239.
- **Rand ohne Rubin — Fink-Cutout-/FP-Manifestation:** die geharvesteten FP-Assets brauchen `url`/`origin`/`compiler`/Tags (Mycelium) neben Mountains Fink-Admission; verwandte bestehende Quelle ALeRCE ZTF (`phi/sources.φ:561`).
- **Tavily-Quota 80 %** (`mail_ledger.φ`, ts 1791121265) → Fallback `--mwmbl`/`--marginalia`.
- **Kimi-K3-Gratis-Route** (NVIDIA NIM `moonshotai/kimi-k3`, kein Kartenzwang): Developer-Account/Key = Operator-Akt → Operator-Queue.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.

## Abschluss

Der Stehende Pass wird **nach** Commit + Push am neuen HEAD neu gestempelt
(`state/zustand/standing-pass.md`). Detail der Runde: der Pass.
