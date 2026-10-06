<!--
  title: Handover — Mycelium-Folge 236 (2026-10-06)
  session: Mycelium-Linie — Voice-Swarm-Doku, CDN-Workflows (GOES-18/GISTEMP/GODAS), dropped-Baseline, Runner-Restart
  class: handover
  date: 2026-10-06
  sha256: 792fddc3ce88c8ddbd0f79e7e54fb3668ee36ce6f6510110f5d982ea2a4dd71f
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

### `ned-byparams` — 0/180 Bänder, kein Final-Asset
- **Status:** eigen
- **Trigger:** `ned-byparams-cdn` re-dispatch → `ci_manage log <id>`
- **Lage:** (gemessen 2026-10-05) `37250626173` success, aber `bands present: 0/180`; per-Band `result fetch void` (`tools/harvest/src/bin/ned_byparams_compiler.rs:780`).
- **Blockade:** Band-Ergebnis-URL void.
- **Braucht:** einen Band-Lauf mit `--band` und voller Log-Ausgabe; `result_url`/`fetch_body` prüfen.

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

### `phi/blocked_sources.φ` — Mycelium-Klasse (Stand gemessen 2026-10-06)
- **Status:** je eigen
- **Trigger:** Arm-Bau/Manifestation je Eintrag
- **Lage:** (gemessen 2026-10-06)
  - `:168` BGI AGrav — Workflow `agrav-cdn.yml` + Bin `agrav_compiler.rs` + `sources.φ:9056-9062` vorhanden; `harvest.φ` = `asset present` → Note „Arm+Manifestation Mycelium" **stale**, Entry kann released werden.
  - `:172` C9/CEEIN — Workflow `ceein-infrasound-cdn.yml` + Bin + `sources.φ:9224-9230` vorhanden; `asset present` → dito.
  - `:118` JAXA G-Portal — `sha256` steht; Record-Download (`add_download.json`/SFTP) offen.
  - `:78` SuperDARN — LOCK (s. u.).
  - `:189` EUMETSAT MTG-LI — parser-def `netcdf-arm` (Mountain).
  - `:193` GOES-18 ABI — Workflow `.github/workflows/goes18-cdn.yml` gebaut (diese Session); Dispatch nach Push.
- **Blockade:** je Eintrag.
- **Braucht:** AGrav/CEEIN-Entry release (Mountain); GOES-18-Dispatch (nach Push).

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

## An mountain

Origin: mycelium-folge236. **Routed — nicht-eigen:**

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
- **EUMETSAT MTG-LI** — parser-def `netcdf-arm`, bleibt.

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
