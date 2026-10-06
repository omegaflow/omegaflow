<!--
  title: Handover — Mycelium-Folge 236 (2026-10-06)
  session: Mycelium-Linie — Voice-Swarm-Doku, CDN-Workflows (GOES-18/GISTEMP/GODAS), dropped-Baseline, Runner-Restart
  class: handover
  date: 2026-10-06
  sha256: ab6044b43e5a6d204657299f3108b4ff7cb3caaa08ae1e8927568a1cdb1a87e8
  status: live
-->
# Handover — Mycelium-Folge 236 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-06-mycelium-folge235.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0531

`session_burn` bei Schluss: Mycelium-236 **$0.0531**; Runde total **$0.4564** (16 Sessions,
kumulativ). `bin/.tools_ensure`: ein Sweep.

## Operator-Wort-Register

- Wort | 2026-10-06 | „Relay-Bind 0.0.0.0:1618 als Netzwerk-Exposition in den Stehenden Pass aufnehmen: Stand, Exposition, Träger River, Braucht Bind-Flip + Re-Messung" | Quelle: Operator-Session 2026-10-06 — Zeile `state/zustand/standing-pass.md` → `## Netzwerk-Exposition`. **Erledigt** durch river-98 (`src/archivar/relay.rs:11` `RELAY_BIND_DEFAULT = "127.0.0.1"`, gemessen HEAD `045711091`).
- Wort | 2026-10-06 | „Genau ein Zulassungskriterium (Presence-Hülle) und ein deklarierter Beobachter je Messung; ein Body-Name, den der Code wählt, ist der Bias" | Quelle: Operator-Session 2026-10-06 — als Regel in `AGENTS.md` `## Block Universe Physics`.

## Offen — eigen

### `ned-byparams` — 0/180 Bänder, URL-Extraktor-Bug behoben
- **Status:** eigen
- **Trigger:** `ned-byparams-cdn` re-dispatch → `ci_manage log <id>`
- **Lage:** (gemessen 2026-10-06, Log `37250626173 --all`) Ursache gefunden: `result_url()`
  (`tools/harvest/src/bin/ned_byparams_compiler.rs:282-291`) trimmte erst das Anführungszeichen und
  nahm dann das **erste URL-Zeichen** als Begrenzer → `result url ttp://nbasq.ops.ned.ipac.caltec`
  (führendes `h` verschluckt, bei `h` in `caltech` abgeschnitten), `curl: (1) Protocol "ttp" not
  supported`. **Behoben** in dieser Session (Quote wird vor dem Trim gelesen; Datei committet).
- **Blockade:** Fix ungebaut — der geteilte Kern ist durch Mountains uncommittetes
  `src/archivar/mtg_li.rs` + `mod.rs` (frisch) gerade nicht kompilierbar.
- **Braucht:** Re-Dispatch `gh workflow run ned-byparams-cdn.yml` nach Mountains Commit; `bands present` prüfen.

### iEEG-Ernte — Backend 503 + cdn_reconcile-Bindung
- **Status:** wartend
- **Trigger:** iEEG-Backend erholt sich → `gh workflow run ieeg-cdn.yml -f dataset=09_14_limbic_seizure_374`
- **Lage:** (gemessen 2026-10-06, `ci-gate 37429979869 @045711091`) `cdn_reconcile`: `ieeg-cdn.yml:47-48/:50` schreibt Tag `www.ieeg.org`, in `phi/sources.φ` ungebunden (`sgrep 'ieeg' phi/sources.φ` = leer). `ieeg_compiler.rs:12` `NETLOC = "www.ieeg.org"`. `cdn_reconcile.rs:277-289` `host_known()` bindet nur einen CDN-`url`-Tag **exakt** `www.ieeg.org` (Origin `ieeg.org` genügt nicht — www wird gestrippt).
- **Blockade:** Backend 503; `sources.φ`-Block fehlt (Zulassung/`format`/`ttl` = Mountain).
- **Braucht:** Mountains Register-Block, s. `## An mountain`; Re-Dispatch bei Kapazität.

### Register-Träger `ledger.φ:2`/`:6` — Port-Runner verloren
- **Status:** blockiert
- **Trigger:** Port-Runner im Baum
- **Lage:** (gemessen 2026-10-04) `ledger.φ:2` = 825 Blöcke, `:6` = 63; `phi/pipeline/stage/*` leer; der Ausführer war ein nie committeter Working-Tree-Bin; nur der Motor `src/archivar/port.rs`.
- **Blockade:** Port-Runner verloren.
- **Braucht:** Port-Runner als Bin rekonstruieren/committen (Konverter-Spec = Mountain).

### `phi/blocked_sources.φ` — Mycelium-Klasse (Stand gemessen 2026-10-06; Mountain-239-Backlog gefaltet)
- **Status:** je eigen
- **Trigger:** Arm-Bau/Manifestation je Eintrag
- **Lage:** (gemessen 2026-10-06)
  - AGrav/CEEIN — Arms + `sources.φ`-Blöcke + `asset present` → **erledigt** (Mountain hat die Entries entfernt).
  - ExoMars TGO ACS / Viking gravity — `sources.φ`-Blöcke + Workflows `acs-nir-cdn.yml` / `viking-grav-cdn.yml` stehen; Dispatch (idempotent) in dieser Session.
  - Hayabusa LIDAR — 7 Blöcke `pds4_fixed_width`; Workflow `.github/workflows/hayabusa-lidar-cdn.yml` gebaut + dispatcht.
  - Phobos-2 KRFM — Block auf 14 Felder ergänzt (Mountain); Workflow `.github/workflows/krfm-cdn.yml` gebaut + dispatcht (`krfm.lbl` HTTP 206, gemessen).
  - EUMETSAT MTG-LI — **erledigt** (diese Session): `EUMETSAT_KEY`/`EUMETSAT_SECRET` stehen in `.secrets.local` (Namensliste Zeilen 36-37), via `bin/secrets-sync.sh --set` als GitHub-Secrets gesetzt; Workflow `.github/workflows/mtg-li-cdn.yml` dispatcht (`37437023048`).
  - Chandrayaan-1 Mini-RF — **blockiert**: `pds3_img` ohne Feld-Arm (Leerzeichen-Labels `"H RECEIVE INTENSITY"` …) → neuer `pds3_img`-Slug-Arm nötig (Mountain).
  - Tianwen-1 MoRIC / ShadowCam / JAXA G-Portal — Sample/Record-Download = Operator-Hand; GOES-18-ABI-Workflow (`goes18-cdn.yml`) in dieser Session dispatcht.
  - `:78` SuperDARN — LOCK (s. u.).
- **Blockade:** MTG-LI-Secrets (Operator); Chandrayaan-`pds3_img`-Arm (Mountain); Sample-/Record-Downloads (Operator/per-act).
- **Braucht:** `pds3_img`-Feld-Arm (Mountain); Consent für Record-Downloads (Operator/per-act).

### JAXA G-Portal — Datensatz-Auswahl (Rat + Schwarm befragt, 2026-10-06)
- **Status:** LOCK (Bestellakt) | **Bindung:** Operator (per-act)
- **Trigger:** Operator-Wort „bestelle <Datensatz>"
- **Lage:** (gemessen 2026-10-06 via Rat+voice-Swarm, Code `src/archivar/jaxa_gportal.rs:15-43`, `phi/sources.φ:9512`) Der G-Portal-Arm trägt **kein Feld** —
  `src/archivar/jaxa_gportal.rs:15-43` deklariert `jaxa_gportal_lon_deg`/`lat_deg` mit
  `force_id_of("em")`; `sources.φ:9512` trägt **keine** `field`-Zeile. Ein Footprint ist keine
  Messung. Prioritätsliste (Kraft aus dem Produkt): 1 AMSR2 L2/L3 (thermal/advective/diffusion),
  2 GPM-L2/L3 Regen (advective; der Default `12001000` ist L1B=em), 3 GCOM-C/SGLI (em),
  4 GOSAT (em; Dedup `blocked_sources.φ:56`), 5 ALOS-2/PALSAR-2 (em). **IDs gemessen 2026-10-06**
  (keyless `satsensor.json` + CSW `datasetId=…`): AMSR2 L2 SST `11002004` / Wind `11002005` /
  SMC `11002008`, L3 SST `11003006` (0,25° `11003036`); GPM DPR-KuPR L2 Regen `12002000`, GMI L2
  `12012000`, DPR+GMI comb `12022000`, L3 `12003000`; SGLI L1B VNR `10001003`, L2 SST `10002002`,
  L3 SST `10003027`. **GOSAT absent auf G-Portal** (liegt bei NIES/GES DISC) → **nicht bestellen**.
  Read-only Katalog-Pässe dispatcht: `11003006` `37439238853`, `12002000` `37439243015`,
  `12001000` `37437987631`.
- **Blockade:** ohne `field`-Zeile ist jede Bestellung ein fabriziertes Feld (IDs jetzt gemessen).
- **Braucht:** Field-Verdikt je Produkt (Mountain) → dann Operator-Order. **Bestell-Kommando vorbereitet** (nicht gefahren): `gh workflow run jaxa-gportal-cdn.yml -f dataset=<ID> -f from=<YYYY/MM/DD> -f to=<YYYY/MM/DD> -f count=100 -f pages=1 -f download=true -f download_limit=N`.

### Membran-Assets — `dr3_stars.bin` / `ephemeris_de440_*`
- **Status:** wartend | **Bindung:** River
- **Trigger:** `ephemeris_de440_*.bin` am CDN → `archive_search --sniff <url>`
- **Lage:** (gemessen 2026-10-05/06) `membrane.html` hängt bei „anchoring bodies…"; river-97/98 adressiert (DE440-Remanifestation). Ohne Remanifestation bleibt der Anker lokal (Register-Schuld).
- **Blockade:** DE440-`.bin` nicht remanifestiert.
- **Braucht:** DE440-Assets bauen (`de_compiler`) → CI/CDN (River-Block), `pages-deploy.yml` same-origin, `archive_search --verdict`/`--sniff`.

### Exposom-Quellenmatrix — restliche Domänen + Matrix-Lauf-Workflow
- **Status:** eigen
- **Trigger:** Matrix-Zeilen ohne Home / Workflow-Bau
- **Lage:** (gemessen 2026-10-06) `state/future/exposom-matrix-2026-10-04.md` (16 Klassen); zwei Arme gebaut (WQP + EEA-noise); der CI-Lauf-Workflow (`te_pair_probe`/`field_te_query` je Zeile) fehlt (`:229-234`).
- **Blockade:** keine (Workflow-YAML + `.te`-Descriptor je Klasse).
- **Braucht:** Workflow-YAML unter `.github/workflows/` + Descriptor je X-Klasse; die übrigen Domänen-x bleiben `pending` (`:202-204`).

### Orphan-Docs — 2 ohne Träger (gemessen 2026-10-06, nach dem Commit)
- **Status:** eigen | **Bindung:** eigen (Mycelium-Domäne: CDN/Datenbestand)
- **Trigger:** Marker-Schließung / descope-Messung
- **Lage:** (gemessen 2026-10-06, `register_lookup --orphan-docs`) `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` (1 Marker) und `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (1 Marker) — beide 2026-09-03-Surveys über Datenbestand/CDN-Orphans. Der Zensus ist HEAD-abhängig (jede Live-Übergabe, die das Dokument nennt, trägt es; die früher gelisteten `hyperscanning-te-preregistration.md`/`survey-2026-10-03-exzellenz-gate.md` sind durch Nennung getragen).
- **Blockade:** offener Marker ohne Folgeschritt.
- **Braucht:** je Dokument den offenen Marker schließen (umsetzen oder `descoped`).

## An mountain  ·  PRIO (rotes CI-Gate `ci-gate 37429979869`)

Origin: mycelium-folge236. **Routed — nicht-eigen, zuerst die zwei roten Gate-Punkte:**

- **iEEG-Netloc-Bindung (`ci-gate` register rot):** `ieeg-cdn.yml`-Tag `www.ieeg.org` in `sources.φ` ungebunden. Gemessener Minimal-Block (dein `format`/`ttl`/Zulassung, mein `url`/`origin`/`compiler`):
  ```
  url https://github.com/omegaflow/sources/releases/download/www.ieeg.org/09_14_limbic_seizure_374.bin
  format openneuro_eeg
  origin https://www.ieeg.org/services
  compiler tools/harvest/src/bin/ieeg_compiler.rs
  at earth
  ttl 604800
  ```
  (`ieeg_compiler.rs:9,448-461` emittiert `openneuro_eeg`; Tag-Literal `www.ieeg.org` ist bindungspflichtig, `cdn_reconcile.rs:27-32`.)
- **AGrav/CEEIN-Entries stale:** `blocked_sources.φ:167-172` „Arm+Manifestation Mycelium" — Workflows, Bins, `sources.φ`-Blöcke (`:9056-9062`, `:9224-9230`) und `asset present` vorhanden; bitte release/descope.
- **gistemp_aod550 / godas_pottmp:** Manifestation gebaut (`gistemp-aod-cdn.yml`, `godas-pottmp-cdn.yml`), Dispatch nach Push; `sha256`-Nachzug (`1ba4e901…`/`c2448a10…`) in `sources.φ`.
- **GOES-18 ABI:** `goes18-cdn.yml` gebaut (Muster `goes-cdn.yml`, self-fetch `goes18_abi_compiler --ci-mode`); `goes_abi`-Workflow-Angleich prüfen; Dispatch nach Push.
- **clippy `units.rs:549`** — in deiner Hand (Working Tree trägt `epoch.split_whitespace()`, uncommittet, gemessen).
- **EUMETSAT MTG-LI** — erledigt (diese Session): Secrets `EUMETSAT_KEY`/`EUMETSAT_SECRET` via `bin/secrets-sync.sh --set` gesetzt, `mtg-li-cdn.yml` dispatcht (`37437023048`). Dein Register-Block `sources.φ:426+` trägt.
- **Chandrayaan-1 Mini-RF — blockiert:** `sources.φ:10019` `pds3_img`-Block steht ohne `field`-Zeile; `pds3_img` hat keinen Feld-Arm → „field undeclared" (`main_flow.rs:3046`). Die Labels tragen Feldnamen **mit Leerzeichen** (`"H RECEIVE INTENSITY"` / `"V RECEIVE INTENSITY"` / `"CROSS POWER INTENSITY (…)"`, 4 Bänder). **Riss (gemessen):** die `field`-Grammatik trägt bereits **zitierte** Schlüssel (`sources.φ:17053-17055` `field "F13PSSO" …`) — der Kern ist der fehlende `pds3_img`-**Feld-/Slug-Arm** (force/unit-Verdikt), nicht allein das Leerzeichen. `blocked_sources.φ` trägt den Eintrag mit Begründung. **Braucht:** `pds3_img`-Feld-Arm + force/unit-Verdikt.
- **JAXA G-Portal — Reader-Riss + stale Zitat (Rat-Konsens 2026-10-06):** `src/archivar/jaxa_gportal.rs:15-43` deklariert `jaxa_gportal_lon_deg`/`lat_deg` mit `force_id_of("em")` — ein Footprint-Zentrum ist kein elektromagnetischer Messwert („ein Adressbuch ist kein Oszillator"). `sources.φ:9512` trägt keine `field`-Zeile. Zudem stale: `blocked_sources.φ:104` zitiert `sources.φ:9408` (heute EEA-Noise), der G-Portal-Block steht auf `:9512`. **Braucht:** Feld-Verdikt je zu erntendem Produkt (Größe/Einheit/τ/Kernel), Zitat-Korrektur.

## An river  ·  PRIO

Origin: mycelium-folge236. **Routed — nicht-eigen, prioritär (blockiert die Membran):**

- **DE440-`.bin`-Remanifestation (river-97/98 adressiert):** `ephemeris_de440_{earth,moon,sun}.bin`
  nach Mountains `de_compiler`-GM-Landung über die CI zur CDN; `pages-deploy.yml` stagt sie
  same-origin. Ohne Remanifestation bleibt der Anker lokal (`membrane.html` hängt bei
  „anchoring bodies…"). Checkmark: deine Browser-Re-Messung `nearCount(<1e13 m) > 0`.
- **`flyby-odf-cdn 37305400435` = failure:** `gh release upload odf07155_census.txt` →
  `HTTP 400: Bad Content-Length`, Census-Datei leer (`2>/dev/null || true`). **Braucht:**
  `odf_census_probe` ohne `2>/dev/null` messen; danach Re-Dispatch.

## An sensory

Origin: mycelium-folge236. **Orphan-Träger:**
- `docs/paper/hyperscanning-te-preregistration.md` (2 offene Marker) — Namenträger in deiner Übergabe eintragen oder `descoped`.

## An future

Origin: mycelium-folge236. **Orphan-Träger + Operator-Queue:**
- `docs/surveys/survey-2026-10-03-exzellenz-gate.md` (1 Marker) — Namenträger eintragen oder `descoped`.
- Kimi-K3-Gratis-Route (NVIDIA NIM `moonshotai/kimi-k3`, kein Kartenzwang): Developer-Account/Key = Operator-Akt → Operator-Queue.
- Tavily-Quota 80 % im Oktober (`mail_ledger.φ`, ts 1791121265) → Fallback `--mwmbl`/`--marginalia`.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.

## Abschluss

Der Stehende Pass wird **nach** Commit + Push am neuen HEAD neu gestempelt
(`state/zustand/standing-pass.md`). Detail der Runde: der Pass.
