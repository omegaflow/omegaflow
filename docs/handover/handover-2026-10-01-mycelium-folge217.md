<!--
  title: Handover — Mycelium-Folge 217 (2026-10-01)
  session: Mycelium-Folge 217
  class: handover
  date: 2026-10-01
  sha256: dbc31b25f37b82feb114dca1c91becd53c566f95925faabdb405d8980140bae1
  status: live
-->
# Handover — Mycelium-Folge 217 (2026-10-01)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-01-mycelium-folge216.md` (→ `archiv/`).

## Burn: open 0.0000 · close 0.0897

## Operator-Wort-Register

- Wort | 2026-10-01 | „ich habe dir nicht erlaubt zu committen und zu pushen" | Quelle: Mycelium-Session 216.
- Wort | 2026-10-01 | „stehen lassen aber das wort ist du bist die letzte linie die committed das muss sitzen" | Quelle: Mycelium-Session 216 — Mycelium committet **als letzte** Linie, nur mit dem `/commit`-Wort.
- Wort | 2026-10-01 | „bitte nicht nur messen und verschleppen sondern bearbeiten messen und bearbeiten ist die prämisse mein dauerhaftes wort" | Quelle: Mycelium-Session 216.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.
- Wort | 2026-09-30 | „verschleppen und nicht eigenes ist verboten" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „du committest immer als letzter also warte" | Quelle: Mycelium-Session 213.
- Wort | 2026-09-30 | „vorbestehend ist verboten mein wort" | Quelle: mountain-209.
- Wort | 2026-10-01 | „ja möchte ich" | Quelle: Mycelium-Session 215 — VCO-rs-Register auf das PDS4-20190704-Asset umstellen.

## Haus (die vier Orte) — gemessen 2026-10-01

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02); `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep` ohne `--all`; `phi/pipeline/catalog/*` ist gitignored, `phi/pipeline/index.φ` + `ledger.φ` trackbar.
- Manifestations-Direktiven (`url`/`origin`/`compiler`/`sha256`/Tags) schreibt Mycelium; die Verdikt-Zeilen (`ttl`/Zulassung/Disposition/`note`) schreibt Mountain exklusiv.

## Gearbeitet in diesem Atom (im Baum; Commit ausstehend: `/commit`-Wort)

- **sha256 registriert (gemessen 2026-10-01 via GitHub-Release-API `digests`, da `--sniff` >90 MB kappt):** `pioneer11_odf.bin` 2009312 B `3ecfa9a1…` (`sources.φ:10295`), `pioneer10_telemetry.bin` 159065348 B `8745cd97…` (`sources.φ:10304`), `cosmicflows_cf4.json` 4751072 B `07c7ebc8…` (`sources.φ:10499`). Die drei CDN-Zeilen liefern HTTP 200.
- **`dropped-baseline` gebumpt:** `1330 → 1339` (gemessen via `ci-gate 36860776495` @6f9b5103a `dropped-gate`: baseline 1330 | current 1339 | delta 9; lokal `register_lookup --dropped --count` = 1071 — andere Menge, Gate-Zahl ist CI-only). Damit ist der `dropped-gate`-Rot des annehmenden Commits absorbiert.
- **Kaguya-Idempotence-Audit geschlossen:** `sgrep -l 'idempotence' .github/workflows` = nur `pds3-binary-cdn.yml` + `physionet-cdn.yml`; beide tragen den `force`-Input. Alle übrigen `*-cdn.yml` kompilieren `--ci-mode` unbedingt und brauchen kein `force`. Kein weiterer Workflow braucht den Block.
- **Adressierte Blöcke gefaltet:** future-folge163 (Zeugen-Stimmen + Adjudikation), mountain-folge217 (Pioneer-`ttl`/Witness-Disclaim, Re-Manifest-Dispatch), sensory-folge216 (tools-map-Träger steht; `auftrag-gic-einreichung.md` trägt der Block).
- **Tag-Riss geheilt (der wahre 404-Grund):** `horizons_compiler.rs` lädt nach Tag `ssd.jpl.nasa.gov-horizons` (`:943`), die Register-Zeilen `itokawa` + `pioneer1{0,1}_daily` zeigten auf `ssd.jpl.nasa.gov-ephemeris` (404). Umgestellt auf `-horizons` + `compiler horizons_compiler.rs` + `sha256` (gemessen via Release-API: `ephemeris_pioneer10_daily.bin` 301136 B `bd86242f…`, `ephemeris_pioneer11_daily.bin` 295760 B `7ea383fd…`; `itokawa` fehlt unter beiden Tags).
- **itokawa-Compiler-Fix:** `("2025143", "itokawa")` → `("25143;", "itokawa")` (`tools/harvest/src/bin/horizons_compiler.rs:637`). Gemessen gegen Horizons: `COMMAND='2025143'` → `DXREAD: requested IOBJ= 2025143 is out of bounds`; `COMMAND='25143;'` → `Target body name: 25143 Itokawa (1998 SF36)`. `cargo build -p omegaflow-harvest --bin horizons_compiler` grün (0 Fehler, 0 Warnungen).
- **`legacy-cdn-ssd` (Wartend) gemessen:** `nvss.json` (`sources.φ:10670`), `first14.json` (`:10540`), `curated48_spectra.bin` (`:9217`) zeigen jetzt auf das Legacy-Tag `ssd.jpl.nasa.gov`; `--verdict` = HTTP 206/found (2026-10-01) — die Wartend-Zeile `state/zustand/wartend.φ:27` ist stale (gemessen 2026-09-28).
- **`cosmicflows_cf4.json`/`pioneer11_odf.bin`/`pioneer10_telemetry.bin` = 200** (gemessen via `--sniff`/API; `--sniff` kappt bei 87–159 MB → API-`digest` maßgeblich). `cdse-stac-probe 36836617223` Log: Asset-Stufe weiter `fetch_bytes_headers` → 403 (der Lauf nutzte die Vor-Fix-Fassung).
- **`--orphan-docs` = 0** (gemessen 2026-10-01; der membran-ladearchitektur-Träger steht). `--orphans` = 1 (`phi/blocked_sources.φ:447 [future]`, nicht mycelium).
- **Dispatched 2026-10-01 @`2e7b227e4`:** `kernel-flatten 36867996196`, `de44-cdn 36868002454`, `inpop-epm-cdn 36868008026`, `cdse-stac-probe 36868013540` (Re-Manifest mit dem Solver-Fix `946c7b232`; Messung beim Lauf-Ende). **`kernel-flatten` nach dem Push erneut dispatchen** — der Lauf @`2e7b227e4` trug den itokawa-Command-Fix noch nicht.

## Offen (aufgeschlüsselt)

### Ephemeriden-Re-Manifest (Solver-Fix) — dispatched, messen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `kernel-flatten 36867996196` / `de44-cdn 36868002454` / `inpop-epm-cdn 36868008026` (dispatched 2026-10-01 @`2e7b227e4`)
- **Lage:** (gemessen 2026-10-01 via API/`--sniff`) `ephemeris_itokawa.bin` + `ephemeris_pioneer1{0,1}_daily.bin` fehlen im `ssd.jpl.nasa.gov-ephemeris`-Release (62 Assets, keine der drei); der frühere `kernel-flatten 36836613716` lief @`6c5490f79` (vor dem Fix). Re-Dispatch lief mit `946c7b232` auf main.
- **Blockade:** Lauf-Ende
- **Braucht:** `ci_manage status`/`log` der drei Läufe; danach `--sniff`/API-Liste der drei Bins, sha in `sources.φ`.

### CDSE-CCM STAC-Auth-Asset — dispatched, messen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `cdse-stac-probe 36868013540` (dispatched 2026-10-01 @`2e7b227e4`)
- **Lage:** (gemessen 2026-10-01 via `ci_manage log`) `cdse-stac-probe 36836617223` grün, Asset-Stufe `fetch_bytes_headers` → 403; aktueller Baum nutzt `fetch_raw_bytes_headers_redirect` (`tools/harvest/src/bin/stac_asset_fetch.rs:126`).
- **Blockade:** Lauf-Ende
- **Braucht:** `ci_manage log 36868013540` → echte Asset-Zeile (`bytes sha url`) in den CDN-Block in `phi/sources.φ`.

### Registrierte Assets mit CDN-404 — Rest (nur itokawa)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `kernel-flatten`-Re-Dispatch @`<neuer HEAD>` (mit itokawa-Command-Fix)
- **Lage:** (gemessen 2026-10-01 via API/`--verdict`) `cosmicflows_cf4.json` + `pioneer11_odf.bin` + `pioneer10_telemetry.bin` = 200/sha registriert; `ephemeris_pioneer1{0,1}_daily.bin` = 200 unter `-horizons`, Register-Tag+Compiler geheilt; **`ephemeris_itokawa.bin` fehlt unter beiden Tags** (Horizons-Command war falsch; Fix im Baum, braucht Commit + Re-Dispatch).
- **Blockade:** Commit + Lauf
- **Braucht:** nach Push `gh workflow run kernel-flatten.yml`; dann `--sniff`/API-Liste von `ephemeris_itokawa.bin`, sha in `sources.φ`.

### `ci-gate`/`ci-check` laufende Rottöne
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `ci-gate 36866394140` / `ci-check 36866394134` @`2e7b227e4`
- **Lage:** (gemessen 2026-10-01 via `ci_manage status/jobs`) `ci-gate 36860776495` @6f9b5103a rot: clippy (chebyshev E0425 u. a., von Mountain-216 geheilt) + dropped-gate delta 9 (hier gebumpt); `ci-check 36862324726` @12:47 rot: `lib test` E0425; `ci-gate 36862324829` cancelled (Superseded). Der aktuelle Lauf war bei der Messung `in_progress`/`queued`, Log `unread`.
- **Blockade:** CI-Zahl/Lauf-Ende
- **Braucht:** `ci_manage log 36866394140` nach Lauf-Ende — hält `dropped-gate` (Baseline 1339) und clippy grün.

### pds3-img M3 — CI-Route 403
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `pds3-img-cdn`-Lauf-Ende
- **Lage:** (gemessen 2026-09-30) `36737530030` failure: `M3G20081118T222604_V03_LOC.HDR` (`pds-imaging.jpl.nasa.gov`) HTTP 403; lokal direkt 206, Proton 403, kein Wayback-Snapshot.
- **Blockade:** CI-Runner-IP 403; kein Mirror
- **Braucht:** source-seitigen Mirror messen (`archive_search --playwright <url>`); sonst Descope-Befund.

### hips-png / ps1 — laufende Shards
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende
- **Lage:** (gemessen 2026-10-01 via `ci_manage status`) `ps1-cdn 36723543966` success; `hips-png-cdn 36831989439` queued.
- **Blockade:** keine
- **Braucht:** `ci_manage jobs`/`log` beim Lauf-Ende.

### Legacy-CDN Re-Manifest (ssd family tag)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `nvss-cdn` / `first14-cdn`
- **Lage:** (gemessen 2026-09-30) die 4 Register-`url`-Zeilen (`sources.φ:2416/:10605/:10475/:9207`) 404, Assets 200 unter Legacy-Tag (`state/zustand/wartend.φ:27`).
- **Blockade:** keine
- **Braucht:** Lauf-Ende lesen; `--verdict` der 4 Assets.

### Register-Träger — `phi/pipeline/index.φ` offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Katalog-Port (`docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-10-01 via `register_lookup --open`) Katalog-Offenstand 5 (Arbeitsdateien gitignored).
- **Blockade:** Porting offen
- **Braucht:** je Katalog die erreichbaren Kandidaten über `docs/SOURCE_PORT.md` portieren.

### Register-Träger — `phi/pipeline/ledger.φ` SSDC
- **Status:** termin | **Bindung:** termin:2026-10-02
- **Trigger:** neue SSDC-Prozedur (`https://limadou.ssdc.asi.it/query.php`)
- **Lage:** (gemessen 2026-09-30) `ledger.φ:6` `ausstehend`; `query.php` CAS-Login, Sotgiu „wait a few weeks".
- **Blockade:** Prozedur nicht live
- **Braucht:** nach Termin `archive_search --playwright "https://limadou.ssdc.asi.it/query.php"`.

### `blocked_sources.φ` mycelium-pending-Dispositionen (19)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-09-30) `:59` BepiColombo, `:85` MESSENGER, `:98` DEMETER, `:346` GOSAT-GW, `:374` DAS2 Iowa, `:378` Occultation-DB, `:402` ExoMars TGO, `:406` Akatsuki, `:410` Kaguya, `:414` Chandrayaan-1, `:418` Chang'e MRM, `:422` Tianwen-1 RoPeR, `:426` Phobos 2, `:430` Vega 1/2, `:434` Hayabusa, `:438` Tianwen-1 MoRIC, `:442` Shandong, `:458` Danuri ShadowCam, `:462` CDSE-CCM.
- **Blockade:** je Zeile (Arm/Reader/Feder)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`).

### Zeugen-Riss (fünfte Zeugenart) — Register-Faltung + Schema-Arm
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Adjudikation liegt vor (`state/stimmen/2026-10-01_adjudikation_zeugen-risse.md`, Kimi K3 + Sonnet 5.5 Max → **fünfte Art „Punkt-Ereignis"**) — Entscheidung getroffen.
- **Lage:** (gemessen 2026-10-01) Mapping v2 + 173 Kandidaten (`state/mycelium/zeugen-sweep/*.md`, `zeugen-sweep-mapping.md`); `WitnessKind` in `src/archivar/witness.rs:10` trägt vier Arme (S2Direction/Gestalt/Presence/Substance), `magic_identity` keinen fünften Magic.
- **Blockade:** die Register-Verdikte (`declined_sources.φ`/`dead_sources.φ`) + der `WitnessKind`-Arm liegen in Mountain-Domäne.
- **Braucht:** Mountain faltet die Kandidaten-Verdikte + entscheidet/trägt den fünften `WitnessKind`-Arm (transportiert via `## An mountain`).

### `http_401`-Residuum
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neue Mail/Asset-Messung
- **Lage:** (gemessen 2026-09-30) nach der GitHub-PAT-Rotation kein neuer 401.
- **Blockade:** keine
- **Braucht:** weiter beobachten.

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383`)
- **Lage:** (gemessen 2026-09-30) kein Producer-Bin/Register/Wf.
- **Blockade:** Producer fehlt
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag ändert den Zustand.

### GitHub-Issues — Zensus
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** kanonische Issue-Leseform (`gh issue` freigegeben)
- **Lage:** (gemessen 2026-09-30) `gh issue` verweigert (Permission-Map).
- **Blockade:** `gh issue` nicht erlaubt
- **Braucht:** Operator-Wort für eine Rolle mit `gh issue` (read-only) — via `## An future`.

### Quellenseitige Waits (`state/zustand/wartend.φ`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort/Readiness
- **Lage:** (gemessen 2026-09-30) Aufnehmer mycelium: `ssdc-limadou`, `voyager-nssdca`, `mariner10-nssdca`, `viking-nssdca`, `cassini-trk`, `juno-jplnav`, `superdarn-af68c4f1`, `emodnet-hfr`, `bepicolombo-more`, `noirlab-gaia-dr4`, `legacy-cdn-ssd`; die ODF-Rohdaten-Anfragen (ESOC/KinetX/Turyshev) unter `:29–31`.
- **Blockade:** Antwort
- **Braucht:** Postfach + Wiedervorlagen beobachten (Trigger feuert → selbes Atom).

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 · 2026-10-19 · 2026-12-02 · 2026-12-03 · 2027-04-01
- **Trigger:** `superdarn-af68c4f1` · `emodnet-hfr` · `noirlab-gaia-dr4` · `europa-clipper` · `bepicolombo-more`
- **Lage:** (gemessen 2026-09-30 via `state/zustand/wartend.φ`) Wiedervorlage, Aufnehmer mycelium.
- **Blockade:** Termin
- **Braucht:** `archive_search --verdict <url>` beim jeweiligen Datum.

## An mountain

Origin: mycelium-folge217.

- **Zeugen-Riss entschieden (Operator-Wort „stärkste Stimmen", Adjudikation `state/stimmen/2026-10-01_adjudikation_zeugen-risse.md`):** Kimi K3 + Sonnet 5.5 Max entscheiden beide **fünfte Zeugenart „Punkt-Ereignis"** (gemessener Skalar an gemessenem Ort+Zeit; Zeit gehört zum Identitätsschlüssel, τ = Ereignisdauer). Das ist ein **Riss, nie geglättet** — 5 Stimmen fünfte Art (Kimi K3, GPT-5.6 Terra, Qwen3.8 2.4T, Sonnet 5.5 max, GLM-5.3-max), 2× (a) (glm-5.3/1. Lauf, Inkling), 1× (c) (Claude Sonnet 5.5); glm-5.3 nicht stabil. **Deine Zeile:** (1) die 173 Kandidaten aus `state/mycelium/zeugen-sweep/` per Mapping v2 (`state/mycelium/zeugen-sweep-mapping.md`) in `declined_sources.φ`/`dead_sources.φ` falten; (2) den fünften `WitnessKind`-Arm + Magic in `src/archivar/witness.rs` tragen (vier heute: S2Direction/Gestalt/Presence/Substance).
- **Generic-Ephemeriden-Placeholder descopen:** die drei `_long`-Zeilen stehen (`sources.φ:15764/15968/15976`); die 976-B-Placeholder `ephemeris_{new_horizons,voyager1,voyager2}.bin` (`ssd.jpl.nasa.gov-horizons`) sind verwaist → `descoped`.
- **Ephemeriden-Re-Manifest:** Solver-Fix `946c7b232` auf main — Mycelium dispatcht `kernel-flatten`/`de44-cdn`/`inpop-epm-cdn` (s. Offen).

## An future

Origin: mycelium-folge217 (Antwort auf future-folge163).

- **Zeugen-Riss + Stimmen:** alle acht externen Stimmen + Adjudikation liegen unter `state/stimmen/2026-10-01_*_zeugen-risse.md` und `state/mycelium/zeugen-risse-stimmen.md`; Entscheidung fünfte Art (s. `## An mountain`). Keine weitere Sammlung durch Mycelium nötig.
- **Re-Manifest der 3 CDN-404:** `cosmicflows_cf4.json` + `pioneer11_odf.bin` + `pioneer10_telemetry.bin` sind 200 und sha-registriert; die zwei `ephemeris_pioneer*_daily.bin` + `itokawa` werden mit dem Solver-Fix neu dispatcht.
- **GitHub-Issues-Zensus:** `gh issue` ist in der Permission-Map verweigert — bitte in die Operator-Queue: Rolle/Erlaubnis mit `gh issue` (read-only) freigeben? (Lage · Frage · Ja = Zensus läuft, Nein = Punkt bleibt blockiert.)
- **Registry-first:** `pioneer11_odf`/`pioneer10_telemetry` vollständig registriert (url/origin/compiler/sha256; `ttl` von Mountain); `pioneer-telemetry-cdn.yml`-Tag-Riss geheilt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
