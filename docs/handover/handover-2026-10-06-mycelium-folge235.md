<!--
  title: Handover — Mycelium-Folge 235 (2026-10-06)
  session: Mycelium-Linie — Stehender Pass, Relay-Bind-Exposition, rote CI-Träger am HEAD
  class: handover
  date: 2026-10-06
  sha256: b7a36eb46d28993ac8e6df8d001444f81c16168be431bb522d93af64928debc0
  status: live
-->
# Handover — Mycelium-Folge 235 (2026-10-06)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-05-mycelium-folge234.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0560

`session_burn` bei Schluss: Mycelium-235 („Relay-Bind 0.0.0.0:1618 in Stehenden Pass")
**$0.0560**; Runde total $3.0631 (232 Sessions, kumulativ). `bin/.tools_ensure`: ein Sweep.

## Operator-Wort-Register

- Wort | 2026-10-06 | „Relay-Bind 0.0.0.0:1618 als Netzwerk-Exposition in den Stehenden Pass aufnehmen: Stand, Exposition, Träger River, Braucht Bind-Flip + Re-Messung" | Quelle: Operator-Session 2026-10-06 (Bezug `742a6dbbb`, `survey-2026-10-06-agnostik-llm-verdikt.md`) — Zeile in `state/zustand/standing-pass.md` → `## Netzwerk-Exposition`.
- Wort | 2026-10-06 | „Genau ein Zulassungskriterium (Presence-Hülle) und ein deklarierter Beobachter je Messung; ein Body-Name, den der Code wählt, ist der Bias" | Quelle: Operator-Session 2026-10-06 — als Regel in `AGENTS.md` `## Block Universe Physics` eingetragen.

## Offen — eigen

### `ned-byparams` — 0/180 Bänder, kein Final-Asset
- **Status:** eigen
- **Trigger:** `ned-byparams-cdn` re-dispatch → `ci_manage log <id>`
- **Lage:** (gemessen 2026-10-05) `37250626173` success, aber `bands present: 0/180`; per-Band `result fetch void` (`tools/harvest/src/bin/ned_byparams_compiler.rs:780`).
- **Blockade:** Band-Ergebnis-URL void.
- **Braucht:** einen Band-Lauf mit `--band` und voller Log-Ausgabe; `result_url`/`fetch_body` prüfen.

### iEEG-Ernte — Backend 503 + cdn_reconcile-Bindung offen
- **Status:** wartend
- **Trigger:** iEEG-Backend erholt sich → `gh workflow run ieeg-cdn.yml -f dataset=09_14_limbic_seizure_374`
- **Lage:** (gemessen 2026-10-04) `37235356150` failure, `ieeg: getId http 503`; (gemessen 2026-10-06, `ci-gate 37428430221 @dcc3243f8`) `cdn_reconcile: ieeg-cdn.yml:48/50 writes "www.ieeg.org", not bound in phi/sources.φ` — `sgrep 'ieeg' phi/sources.φ` = kein Block.
- **Blockade:** Backend überlastet; `www.ieeg.org` in `sources.φ` ungebunden.
- **Braucht:** re-dispatch bei Kapazität; die Netloc-Bindung (`www.ieeg.org`) als `url`/`origin`-Zeile — Register-Disposition Mountain (`## An mountain`).

### Register-Träger `ledger.φ:2`/`:6` — Port-Runner verloren
- **Status:** blockiert
- **Trigger:** Port-Runner im Baum
- **Lage:** (gemessen 2026-10-04) `ledger.φ:2` = 825 Blöcke, `:6` = 63; `phi/pipeline/stage/*` leer; der Ausführer war ein nie committeter Working-Tree-Bin; nur der Motor `src/archivar/port.rs`.
- **Blockade:** Port-Runner verloren.
- **Braucht:** Port-Runner als Bin rekonstruieren/committen (Konverter-Spec = Mountain).

### `phi/blocked_sources.φ` — Mycelium-Klasse (Stand gemessen 2026-10-05)
- **Status:** je eigen
- **Trigger:** Arm-Bau/Manifestation je Eintrag (externer Host-Rückkehr oder Mountain-Disposition)
- **Lage:** (gemessen 2026-10-05)
  - `:166` BGI AGrav — `agrav-cdn.yml` existiert (mtime 2026-10-05); Arm/Manifestation prüfen.
  - `:170` C9/CEEIN Infraschall — `ceein-infrasound-cdn.yml` existiert; Arm/Manifestation prüfen.
  - `:118` JAXA G-Portal — `sha256` steht; Record-Download (`add_download.json`/SFTP) offen.
  - `:146` PDS-PPI Kuration offen; `:138`/`:142` externe Hosts down (wartend).
  - `:189` EUMETSAT MTG-LI — API-Route/kein eumetsat-Arm (parser-def `netcdf-arm`, Mountain).
  - `:193` GOES-18 ABI — Arm gebaut (mountain-238); `goes-cdn.yml` deckt goes16/19; Register-Block geschrieben.
- **Blockade:** je Eintrag (Arm-Bau / Mountain-Disposition / externe Hosts).
- **Braucht:** je Eintrag Arm bauen + manifestieren bzw. Download-Route/Kuration (s. `## An mountain`).

### Membran-Assets — `dr3_stars.bin` / `ephemeris_de440_*`
- **Status:** wartend | **Bindung:** River (DE440-Remanifest) + eigen (Asset-Prüfung)
- **Trigger:** `ephemeris_de440_*.bin` am CDN → `archive_search --sniff <url>`
- **Lage:** (gemessen 2026-10-05) `membrane.html` hängt bei „anchoring bodies…"; `/dr3_stars.bin` und `/ephemeris_de440_*.bin` kommen nicht durch. river-97 hat die DE440-Remanifestation als `## An mycelium` adressiert.
- **Blockade:** DE440-`.bin` nicht remanifestiert; Route/Origin offen.
- **Braucht:** DE440-Assets über CI/CDN neu bauen (River-Block), `pages-deploy.yml` same-origin prüfen, `archive_search --verdict`/`--sniff`.

### Exposom-Quellenmatrix — restliche Domänen registrieren (future-181)
- **Status:** eigen
- **Trigger:** Matrix-Zeilen ohne Home
- **Lage:** (gemessen 2026-10-05) zwei Arme gebaut (WQP + EEA-noise, mycelium-folge231:70-75); die übrigen Domänen-x ohne Home fehlen als `sources.φ`-Zeilen + Manifestation.
- **Blockade:** keine.
- **Braucht:** die Matrix (`state/future/exposom-matrix-2026-10-04.md`, 16 Klassen) zeilenweise in `sources.φ` registrieren + Workflow-Dispatch.

### Voice-Swarm-Doku — `free-voices.md` ergänzen (future-181)
- **Status:** eigen
- **Trigger:** Future-181-Bau
- **Lage:** (gemessen 2026-10-05) `voice-swarm.sh` + `voice-roster-core.tsv` (24) + `voice-swarm.md` gebaut; `docs/concepts/free-voices.md` (Mycelium-Doc) trägt den `arch`-Modus, das Synthese-Gate und das Kern-Roster noch nicht.
- **Blockade:** keine.
- **Braucht:** `free-voices.md` um die drei ergänzen.

### CDN-Manifestation der Welle 2026-10-05 — grün, Register-Blöcke ausstehend
- **Status:** wartend (Mountain) | **Bindung:** Mountain (Disposition) + eigen (Manifestation)
- **Trigger:** Mountains Register-Blöcke — gemessen `phi/blocked_sources.φ:74-76` (`pending`)
- **Lage:** (gemessen 2026-10-06, `ci_manage view`) `clpds-cdn 37329649099` **success** (`clpds.bao.ac.cn`), `fmi-gic-cdn 37329656081` **success**, `acs-nir-cdn 37329660510` **success**; `openneuro-cdn 37329643821` (ds004100) **cancelled** — Re-Dispatch nötig.
- **Blockade:** Zulassung/Format/ttl/field fehlen (`phi/blocked_sources.φ:74-76` u. a.).
- **Braucht:** `gh workflow run openneuro-cdn.yml`; nach Mountains Block `url`/`sha256` in `sources.φ` nachziehen.

### Orphan-Docs — 2 ohne Träger
- **Status:** eigen | **Bindung:** eigen (Meta)
- **Trigger:** Owner-Zuordnung
- **Lage:** (gemessen 2026-10-06, `register_lookup --orphan-docs`) `docs/paper/hyperscanning-te-preregistration.md` (2 Marker), `docs/surveys/survey-2026-10-03-exzellenz-gate.md` (1 Marker).
- **Blockade:** Träger-Zuordnung (Owner-Linie).
- **Braucht:** je Dokument einen Namenträger in der Owner-Übergabe (Sensory / Future) oder gemessenes `descoped`.

## An river

Origin: mycelium-folge235. **Routed — nicht-eigen:**

- **Relay-Bind-Exposition (`src/archivar/relay.rs:11` `RELAY_BIND_DEFAULT = "0.0.0.0"`, `PORT_CONST = 1618`):** im Stehenden Pass (`## Netzwerk-Exposition`) als Exposition geführt; der Default stammt aus dem Operator-Wort 2026-09-23 (LAN-Exposition, `survey-2026-09-23-geraete-anbindung-radiatoren.md:278-281,322`). Das neue Operator-Wort 2026-10-06 verlangt **Bind-Flip + Re-Messung**. Relay = dein Membran-Recht.
  **Braucht:** Default auf Loopback flippen (bzw. Env-Zwang dokumentieren) + Re-Messung am neuen HEAD.
- **DE440-`.bin`-Remanifestation (river-97 adressiert):** `ephemeris_de440_{earth,moon,sun}.bin` nach Mountains `de_compiler`-GM-Landung über die CI zur CDN bringen; ohne Remanifestation bleibt der Anker lokal (Register-Schuld). Checkmark: deine Browser-Re-Messung `nearCount(<1e13 m) > 0`.

## An mountain

Origin: mycelium-folge235. **Routed — nicht-eigen:**

- **Clippy rot (`ci-gate 37428430221 @dcc3243f8`, am HEAD `ea47653bf` noch vorhanden):** `src/archivar/units.rs:549` `epoch.trim().split_whitespace()` → `clippy::trim_split_whitespace` (`-D warnings`). **Braucht:** `.trim()` entfernen. Deine Archivar-Domäne.
- **cdn_reconcile:** `ieeg-cdn.yml:48/50` schreibt `www.ieeg.org`, in `phi/sources.φ` ungebunden (`sgrep 'ieeg' phi/sources.φ` = leer). **Braucht:** Register-Bindung (`url`/`origin`/Format) oder Workflow-Netloc angleichen.
- **CDN-Welle 2026-10-05 grün:** `clpds-cdn` (clpds_annex.jsonl), `fmi-gic-cdn`, `acs-nir-cdn` success; neue Assets ohne `sources.φ`-Block. **Braucht:** Zulassung/Format/ttl/field; `url`/`sha256` ziehe ich nach.
- **emm-sdc:** Asset grün (`sha256 6f379edf…`), wartet auf deinen `emm_exi_l2a`-Block.
- **GOES-18 ABI / gistemp_aod550 / godas_pottmp / MTG-LI:** deine `## An mycelium`-Punkte bleiben offen; GOES-18-Arm + Register-Block stehen, Workflow/Manifestation folgt.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.

## Abschluss

Der Stehende Pass wird **nach** Commit + Push am neuen HEAD neu gestempelt
(`state/zustand/standing-pass.md`). Detail der Runde: der Pass.
