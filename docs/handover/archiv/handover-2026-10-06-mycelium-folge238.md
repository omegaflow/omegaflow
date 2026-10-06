<!--
  title: Handover — Mycelium-Folge 238 (2026-10-06)
  session: Mycelium-Linie — Meta-Pass: ps1-cdn-Re-Dispatch, Stehender Pass, Runde
  class: handover
  date: 2026-10-06
  sha256: aeacc9d34386b888cb87cb0a2dc3f8ca476b91aef478342caf792b574beab4c0
  status: live
-->
# Handover — Mycelium-Folge 238 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-06-mycelium-folge237.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0267

`session_burn` bei Schluss: Mycelium-238 **$0.0267** (Runde total $0.3267, 24 Sessions); `bin/.tools_ensure archive_search|sgrep`: frisch.

## Operator-Wort-Register

- Wort | 2026-10-06 | „Genau ein Zulassungskriterium (Presence-Hülle) und ein deklarierter Beobachter je Messung; ein Body-Name, den der Code wählt, ist der Bias" | Quelle: Operator-Session 2026-10-06 — als Regel in `AGENTS.md` `## Block Universe Physics`.
- Wort | 2026-10-06 | JAXA-G-Portal-Bestellungen (`download_limit=1` je, Fenster `2026/01/01`); die Abholung (fetch) ist der Vollzug desselben Worts | Quelle: Operator-Session 2026-10-06.

## Offen — eigen

### JAXA G-Portal — Abholung kollabierte an der Concurrency (Fix committet)
- **Status:** wartend
- **Trigger:** `jaxa-gportal-cdn`-Reihen-Ausgang → `ci_manage list` (kein Polling)
- **Lage:** (gemessen 2026-10-06T09:11) Fix committet (`jaxa-gportal-cdn.yml:3-5` Gruppe um `${{ github.event.inputs.dataset }}`); die 11 Re-Dispatch-Läufe `37441247309` + `37441267585`–`37441304379` stehen **queued** im Runner-Stau, `37440399851` pending.
- **Blockade:** keine (Fix + Re-Dispatch gebaut).
- **Braucht:** Reihen-Ausgang im nächsten Pass; Reader-Feld-Verdikt je Produkt (Mountain) für ein geerntetes Produkt.

### ned-byparams — URL-Extraktor behoben, Lauf misst
- **Status:** wartend
- **Trigger:** `ned-byparams-cdn`-Lauf → `bands present` prüfen
- **Lage:** (gemessen 2026-10-06T09:14) `37441255115` **in_progress**; der behobene URL-Extraktor (`tools/harvest/src/bin/ned_byparams_compiler.rs:282-291`, Quote vor dem Trim gelesen) kompiliert (HEAD).
- **Blockade:** keine.
- **Braucht:** `ci_manage log 37441255115` → `bands present`.

### ps1-cdn — Reparatur-Lauf dispatched
- **Status:** wartend
- **Trigger:** `ps1-cdn`-Lauf-Ausgang → `ci_manage view <id>`
- **Lage:** (gemessen 2026-10-06) Alt-Lauf `37355972362 @410d9f2b` = failure, Grund `unread` (Jobs `cancelled`, `ci_manage log` liefert 404); Re-Dispatch `37441624456` (diese Session).
- **Blockade:** keine.
- **Braucht:** `ci_manage view 37441624456`.

### Exposom-Quellenmatrix — Matrix-Lauf-Workflow
- **Status:** eigen
- **Trigger:** Descriptor + Workflow-YAML gebaut → `gh workflow run`
- **Lage:** (gemessen 2026-10-06) öffentlich `docs/surveys/survey-2026-10-04-exposom-matrix.md` (12 Domänen); 4 Kern-x-Serien erreichbar/registriert (OpenAQ 206, Open-Meteo 200, NASA POWER 206, OMNIWeb 200), 8 x-Homes `pending`; 2 Arme gebaut (WQP + EEA-noise, `mycelium-folge231:70-75`); Descriptor-Form `phi/pipeline/descriptors/solar_seconds_matrix.te`, Parser `tools/measure/src/bin/field_te_query.rs:580-684`. `.github/workflows/exposom-matrix-te.yml` + `.te` je Klasse fehlen.
- **Blockade:** die x-Homes der übrigen Domänen (Wasser, Lärm, Licht, Pollen, Grünraum, gebaute Umwelt, Ernährung, Arbeit, Chemikalien) tragen keine gemessene URL/`phi/sources.φ`-Registrierung — ein Descriptor braucht echte driver/target-Namen.
- **Braucht:** je pending Domäne die x-Home messen (`archive_search --verdict <url>`) + `phi/sources.φ`-Zeile; dann `.github/workflows/exposom-matrix-te.yml` + `.te` je Klasse.

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
- **Status:** wartend (Bindung River)
- **Trigger:** `ephemeris_de440_*.bin` am CDN → `archive_search --sniff <url>`
- **Lage:** (gemessen 2026-10-05/06) `membrane.html` hängt bei „anchoring bodies…"; river-97/98/101/102 adressiert (DE440-Remanifestation). Ohne Remanifestation bleibt der Anker lokal (Register-Schuld).
- **Blockade:** DE440-`.bin` nicht remanifestiert.
- **Braucht:** DE440-Assets bauen (`de_compiler`) → CI/CDN, `pages-deploy.yml` same-origin, `--verdict`/`--sniff`.

## An mountain  ·  PRIO

Origin: mycelium-folge238. **Routed — nicht-eigen:**

- **`ci-gate 37441410996 @f127899f7` = failure — blockiert den Baum (gemessen 2026-10-06):** `src/archivar/mtg_li.rs:329:12` clippy `neg_cmp_op_on_partial_ord` (`if !(value > 0.0)`) + `:343:1` `items after a test module` → `cargo check`/clippy mit `-D warnings` kompiliert lib + lib test nicht. Herkunft `17d626ef7` (mountain 239). Jeder Push scheitert an ci-gate, bis das steht. **Braucht:** `!(value > 0.0)` → `value <= 0.0` (bzw. `partial_cmp`), Test-Modul ans Dateiende.
- **JAXA G-Portal — Reader-Feld-Verdikt:** `src/archivar/jaxa_gportal.rs` wurde in `mountain-240` gelöscht („catalog footprint is no field"). Für ein geerntetes Produkt braucht es das Feld-Verdikt (Größe/Einheit/τ/Kernel) je Produkt + Reader; `sources.φ:9512` trägt keine `field`-Zeile. Priorität aus dem Produkt: AMSR2 L2 SST/Wind/SMC, GPM-L2 Regen, SGLI. Braucht: Feld-Verdikt.
- **Chandrayaan-1 Mini-RF:** `sources.φ:10019` `pds3_img`-Block ohne `field`-Zeile; Kern ist der fehlende `pds3_img`-**Feld-/Slug-Arm** (die `field`-Grammatik trägt zitierte Schlüssel, `sources.φ:17053-17055`). Braucht: `pds3_img`-Feld-Arm + force/unit-Verdikt.
- **gistemp_aod550 / godas_pottmp:** godas-Lauf `37434149825` = success; gistemp `37434146422` queued. Danach `sha256`-Nachzug (`1ba4e901…`/`c2448a10…`) in `sources.φ`.
- **GOES-18 ABI:** `goes18-cdn.yml` gebaut, `37434142761` queued; `goes_abi`-Workflow-Angleich prüfen.
- **AGrav/CEEIN-Entries** in `blocked_sources.φ` — gegen den Baum re-measured (Workflows/Bins/`sources.φ`-Blöcke standen 2026-10-06); release oder descope.

## An river

Origin: mycelium-folge238. **Routed — nicht-eigen:**

- **DE440-`.bin`-Remanifestation:** `ephemeris_de440_{earth,moon,sun}.bin` nach `de_compiler`-GM-Landung über die CI zur CDN; `pages-deploy.yml` stagt sie same-origin. Checkmark: Browser-Re-Messung `nearCount(<1e13 m) > 0`.
- **`static/membrane.html:43` BODIES-Handkopie** (`["earth","moon","sun"]`) → Build-Time-Manifest aus der Hüllen-Pipeline (Rivers Agnosis-Rest, kein Mycelium-Akt).

## An sensory

Origin: mycelium-folge238.
- **paper-check `37440563245 @bfb010ead` = failure, gemessener Grund:** `hyperscanning-te-method` Titel 93 Zeichen > 75 (`ci_manage log 37440563245`: `hyperscanning-te-method 93/93/long`). Braucht: Titel ≤75 kürzen (sensory 238 editierte das Papier).
- `docs/paper/hyperscanning-te-preregistration.md` — Träger in deiner Übergabe oder `descoped` (Zensus `register_lookup --orphan-docs`).

## An future

Origin: mycelium-folge238.
- **Rand ohne Rubin — Fink-Cutout-/FP-Manifestation:** die geharvesteten FP-Assets brauchen `url`/`origin`/`compiler`/Tags (Mycelium) neben Mountains Fink-Admission; verwandte bestehende Quelle ALeRCE ZTF (`phi/sources.φ:561`).
- **Tavily-Quota 80 %** (`mail_ledger.φ`, ts 1791121265) → Fallback `--mwmbl`/`--marginalia`.
- **Kimi-K3-Gratis-Route** (NVIDIA NIM `moonshotai/kimi-k3`, kein Kartenzwang): Developer-Account/Key = Operator-Akt → Operator-Queue.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.

## Abschluss

Der Stehende Pass wird **nach** Commit + Push am neuen HEAD neu gestempelt
(`state/zustand/standing-pass.md`). Detail der Runde: der Pass.
