<!--
  title: Handover — Mycelium-Folge 237 (2026-10-06)
  session: Mycelium-Linie — JAXA-Fetch-Concurrency-Fix, CDN-Läufe, Stehender Pass
  class: handover
  date: 2026-10-06
  sha256: 295d708a574164080e8f83ee2bf4fcf54464a1f86b3f1226400b49540f748b3f
  status: live
-->
# Handover — Mycelium-Folge 237 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-06-mycelium-folge236.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0354

`session_burn` bei Schluss: Mycelium-237 **$0.0354**; `bin/.tools_ensure`: ein Sweep.

## Operator-Wort-Register

- Wort | 2026-10-06 | „Genau ein Zulassungskriterium (Presence-Hülle) und ein deklarierter Beobachter je Messung; ein Body-Name, den der Code wählt, ist der Bias" | Quelle: Operator-Session 2026-10-06 — als Regel in `AGENTS.md` `## Block Universe Physics`.
- Wort | 2026-10-06 | JAXA-G-Portal-Bestellungen (`download_limit=1` je, Fenster `2026/01/01`); die Abholung (fetch) ist der Vollzug desselben Worts | Quelle: Operator-Session 2026-10-06. Der Re-Dispatch dieser Session nach dem Concurrency-Fix läuft darunter.

## Offen — eigen

### JAXA G-Portal — Abholung kollabierte an der Concurrency (Fix committet)
- **Status:** wartend
- **Trigger:** `jaxa-gportal-cdn`-Reihen-Ausgang → `ci_manage list` (kein Polling)
- **Lage:** (gemessen 2026-10-06) Die 11 Abhol-Läufe (`37440358636`–`37440399851`) liefen **10 cancelled + 1 pending**. Ursache gemessen: die `concurrency`-Gruppe war `${{ github.workflow }}-${{ github.ref }}` — alle 11 in **einer** Gruppe; GitHub verwirft bei `cancel-in-progress: false` den jeweils älteren **pending** Lauf, nur der letzte wartet. Fix: Gruppe um `${{ github.event.inputs.dataset }}` erweitert (`.github/workflows/jaxa-gportal-cdn.yml:3-5`), sodass 11 Datensatz-Läufe parallel warten; Re-Dispatch der 11 nach Push.
- **Blockade:** keine (Fix + Re-Dispatch gebaut).
- **Braucht:** Reihen-Ausgang im nächsten Pass; Reader-Feld-Verdikt je Produkt (Mountain) für ein geerntetes Produkt.

### ned-byparams — URL-Extraktor behoben, Re-Dispatch läuft
- **Status:** wartend
- **Trigger:** `ned-byparams-cdn`-Lauf → `bands present` prüfen
- **Lage:** (gemessen 2026-10-06, Log `37437047254 --all`) `band total=180`, `bands present: 0/180` — der behobene URL-Extraktor (`tools/harvest/src/bin/ned_byparams_compiler.rs:282-291`, Quote wird vor dem Trim gelesen) war zum Laufzeitpunkt ungebaut; der geteilte Kern kompiliert jetzt (`cargo check` grün, HEAD `bfb010ead`).
- **Blockade:** keine.
- **Braucht:** `gh workflow run ned-byparams-cdn.yml` (dispatcht); `bands present` im nächsten Pass.

### Exposom-Quellenmatrix — Matrix-Lauf-Workflow
- **Status:** eigen
- **Trigger:** Descriptor + Workflow-YAML gebaut → `gh workflow run`
- **Lage:** (gemessen 2026-10-06) öffentlich `docs/surveys/survey-2026-10-04-exposom-matrix.md`, private Arbeitskopie `state/future/exposom-matrix-2026-10-04.md` (16 Klassen). Descriptor-Form kanonisch `phi/pipeline/descriptors/solar_seconds_matrix.te` (`pair`/`driver`/`target`, `matrix <label> rect|full|upper`, `drivers`/`targets`/`channels`, `fdr bh <q> over matrix`, `expect cells <n>`); `field_te_query` parst sie (`tools/measure/src/bin/field_te_query.rs:580-684`). Zwei Arme gebaut (WQP + EEA-noise, `mycelium-folge231:70-75`).
- **Blockade:** die x-Kanal-Quellen der übrigen Domänen tragen noch keine auflösbare `phi/sources.φ`-Registrierung — ein Descriptor braucht echte driver/target-Namen.
- **Braucht:** je X-Klasse die Quellen registrieren (Register); dann `.github/workflows/exposom-matrix-te.yml` + `.te`-Descriptor je Klasse.

### Träger der zwei Surveys mit offenem Marker
- **Status:** eigen
- **Trigger:** Marker-Schließung
- **Lage:** (gemessen 2026-10-06, `register_lookup --orphan-docs` = 0 mit diesem Träger) `docs/surveys/survey-2026-09-03-orphan-verdicts.md`: Step 5 „CDN-kanonisch" offen (`:100-126`) — jedes `*-cdn.yml` soll seine Release-Menge an `phi/sources.φ` binden statt eigener Tag-Sätze (Familien-Identität ins Register, Jahr-/Slab-Menge Laufzeit-Ableitung); `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md`: `## Offen / Befunde` (`:96`) — Byte-Messung je Holding, Registry-first.
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
- **Lage:** (gemessen 2026-10-06)
  - ExoMars TGO ACS / Viking gravity / Hayabusa LIDAR / Phobos-2 KRFM — `sources.φ`-Blöcke + Workflows stehen; Läufe dispatcht (`37436477618` / `37436483290` / `37436469438` / `37436474006`), im Runner-Stau (queued).
  - EUMETSAT MTG-LI — Workflow + Secrets stehen, `37437023048` queued.
  - Chandrayaan-1 Mini-RF — **blockiert**: `pds3_img` ohne Feld-Arm (Labels mit Leerzeichen) → neuer `pds3_img`-Slug-Arm nötig (Mountain).
  - Tianwen-1 MoRIC / ShadowCam / JAXA G-Portal — Sample/Record-Download = Operator-Hand.
  - `:78` SuperDARN — LOCK (s. u.).
- **Blockade:** Chandrayaan-`pds3_img`-Arm (Mountain); Sample-/Record-Downloads (Operator/per-act).
- **Braucht:** `pds3_img`-Feld-Arm (Mountain); Consent für Record-Downloads (Operator/per-act).

### iEEG-Ernte — Backend 503
- **Status:** wartend
- **Trigger:** iEEG-Backend erholt sich → `gh workflow run ieeg-cdn.yml -f dataset=09_14_limbic_seizure_374`
- **Lage:** (gemessen 2026-10-06) `phi/sources.φ:3584` bindet `www.ieeg.org` jetzt (Mountain) — die `cdn_reconcile`-Bindung ist geschlossen; `ieeg_compiler.rs:12` `NETLOC = "www.ieeg.org"`.
- **Blockade:** Backend 503.
- **Braucht:** Re-Dispatch bei Kapazität.

### Membran-Assets — `dr3_stars.bin` / `ephemeris_de440_*`
- **Status:** wartend (Bindung River)
- **Trigger:** `ephemeris_de440_*.bin` am CDN → `archive_search --sniff <url>`
- **Lage:** (gemessen 2026-10-05/06) `membrane.html` hängt bei „anchoring bodies…"; river-97/98/101 adressiert (DE440-Remanifestation). Ohne Remanifestation bleibt der Anker lokal (Register-Schuld).
- **Blockade:** DE440-`.bin` nicht remanifestiert.
- **Braucht:** DE440-Assets bauen (`de_compiler`) → CI/CDN, `pages-deploy.yml` same-origin, `--verdict`/`--sniff`.

## An mountain  ·  PRIO

Origin: mycelium-folge237. **Routed — nicht-eigen:**

- **JAXA G-Portal — Reader-Feld-Verdikt:** `src/archivar/jaxa_gportal.rs` wurde in `mountain-240` gelöscht („catalog footprint is no field"). Für ein geerntetes Produkt braucht es das Feld-Verdikt (Größe/Einheit/τ/Kernel) je Produkt + Reader; `sources.φ:9512` trägt keine `field`-Zeile. Priorität aus dem Produkt: AMSR2 L2 SST/Wind/SMC, GPM-L2 Regen, SGLI. Braucht: Feld-Verdikt.
- **Chandrayaan-1 Mini-RF:** `sources.φ:10019` `pds3_img`-Block ohne `field`-Zeile; der Kern ist der fehlende `pds3_img`-**Feld-/Slug-Arm** (die `field`-Grammatik trägt bereits zitierte Schlüssel, `sources.φ:17053-17055`) — nicht allein das Leerzeichen. Braucht: `pds3_img`-Feld-Arm + force/unit-Verdikt.
- **gistemp_aod550 / godas_pottmp:** godas-Lauf `37434149825` = success; gistemp `37434146422` queued. Danach `sha256`-Nachzug (`1ba4e901…`/`c2448a10…`) in `sources.φ`.
- **GOES-18 ABI:** `goes18-cdn.yml` gebaut, `37434142761` queued; `goes_abi`-Workflow-Angleich prüfen.
- **AGrav/CEEIN-Entries** in `blocked_sources.φ` — bitte gegen den Baum re-measured (Workflows/Bins/`sources.φ`-Blöcke standen `2026-10-06`); release oder descope.

## An river

Origin: mycelium-folge237. **Routed — nicht-eigen:**

- **DE440-`.bin`-Remanifestation:** `ephemeris_de440_{earth,moon,sun}.bin` nach `de_compiler`-GM-Landung über die CI zur CDN; `pages-deploy.yml` stagt sie same-origin. Checkmark: die Browser-Re-Messung `nearCount(<1e13 m) > 0`.
- **`static/membrane.html:43` BODIES-Handkopie** (`["earth","moon","sun"]`) → Build-Time-Manifest aus der Hüllen-Pipeline (Rivers Agnosis-Rest, kein Mycelium-Akt; die Programm-Hülle ist agnostisch, `AGENTS.md` `## Block Universe Physics`).
- **`flyby-odf-cdn 37305400435` = failure:** `gh release upload odf07155_census.txt` → `HTTP 400: Bad Content-Length`, Census-Datei leer (`2>/dev/null || true`). Braucht: `odf_census_probe` ohne `2>/dev/null` messen; danach Re-Dispatch.

## An sensory

Origin: mycelium-folge237.
- `docs/paper/hyperscanning-te-preregistration.md` — Träger in deiner Übergabe oder `descoped` (Zensus `register_lookup --orphan-docs`).

## An future

Origin: mycelium-folge237.
- **Rand ohne Rubin — Fink-Cutout-/FP-Manifestation:** die geharvesteten FP-Assets brauchen `url`/`origin`/`compiler`/Tags (Mycelium) neben Mountains Fink-Admission; verwandte bestehende Quelle ALeRCE ZTF (`phi/sources.φ:561`). Punkt war in `future-folge183` adressiert (diese Session gefaltet).
- **Tavily-Quota 80 %** (`mail_ledger.φ`, ts 1791121265) → Fallback `--mwmbl`/`--marginalia`.
- **Kimi-K3-Gratis-Route** (NVIDIA NIM `moonshotai/kimi-k3`, kein Kartenzwang): Developer-Account/Key = Operator-Akt → Operator-Queue.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.

## Abschluss

Der Stehende Pass wird **nach** Commit + Push am neuen HEAD neu gestempelt
(`state/zustand/standing-pass.md`). Detail der Runde: der Pass.
