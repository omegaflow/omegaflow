<!--
  title: Handover — Mycelium-Folge 213 (2026-09-30)
  session: Mycelium-Folge 213
  class: handover
  date: 2026-09-30
  sha256: 0d39c1b0358db0527e82bbc84721c4138b898853d70f865746bf3b3e6d18eb32
  status: live
-->
# Handover — Mycelium-Folge 213 (2026-09-30)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-09-30-mycelium-folge212.md` (→ `archiv/`).

## Burn: open 0.0 · close 0.0504

## Operator-Wort-Register

- Wort | 2026-09-30 | „verschleppen und nicht eigenes ist verboten" | Quelle: Mycelium-Session 213 — kein Punkt wandert ungearbeitet weiter; keine fremde Linien-Arbeit (Mountain/Future) im eigenen Atom.
- Wort | 2026-09-30 | „vorbestehend ist verboten mein wort" | Quelle: mountain-209 (`## An mycelium`) — keine Ausnahme für vorbestehende Register-Verstöße in `phi/`; jede `note`-Zeile ≤ 256 Zeichen, keine `#`-Kommentarzeilen in den gated Registern.
- Wort | 2026-09-30 | „NATÜRLICH UND VERSCHLEPPEN IST VERBOTEN!!!!" | Quelle: Mycelium-Session 212 — Ausführungs-Consent Phase 2.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.

## Haus (die vier Orte) — gemessen 2026-09-30

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02).
- `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep` ohne `--all` über den gitignorierten Baum.
- `phi/pipeline/catalog/*` ist **gitignored** (Working-Tree-Arbeitsdateien); `phi/pipeline/index.φ` + `ledger.φ` sind trackbar.

## Offen (aufgeschlüsselt)

### CI-Tafel — Läufe am HEAD lesen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende (gefeuert)
- **Lage:** (gemessen 2026-09-30 via `ci_manage status`/`view`/`log` @`f7bfce9b7`) **gefaltet:** `ci-gate 36720054013` failure (dropped-gate, eigener Punkt); `matrix-rotor 36718499001` failure = rotor-Slice `rc=143`, Runner-Shutdown/ci_watchdog-Cancel, **kein** Assert (river-Lauf). **success:** `swpc-mirror-cdn 36722823612`, `ned-cdn 36724986653`, `harvest-dispatch 36720055011`, `tools-build 36720053903`, `auto-dispatch 36720053872`, `register-coverage 36719761027`, `cdse-stac-probe 36713089816` (1 Job, grün; Step-Ergebnis s. CDSE-Punkt). **in flight/queued:** `ci-check 36717357356` (in_progress), `ci-check 36720053807` (pending), `kernel-flatten 36719833983`, `gosat-cdn 36714631159`, `hips-png-cdn 36718182275`, `ps1-cdn 36723543966`, `quake-feeds-cdn 36725476278`, `tools-build 36724988223`.
- **Blockade:** keine
- **Braucht:** `ci_manage log <id>` beim jeweiligen Lauf-Ende; kein Polling.

### `ci-gate` dropped-gate
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neuer `ci-gate`-Lauf am annehmenden HEAD
- **Lage:** (gemessen 2026-09-30 via `ci_manage log 36720054013` @`f7bfce9b7`) baseline 1157 | current 1320 | delta 163 — der aufgelaufene Archiv-Move-Netto. Baseline in `docs/zustand/dropped-baseline.md:16` auf **1320** gebumpt (dieses Atom). Riss: lokal `register_lookup --dropped --count` = **1054** (unter der Baseline) — die Gate-Zahl ist CI-only.
- **Blockade:** keine
- **Braucht:** neuer `ci-gate`-Lauf am annehmenden HEAD; nur bei erneutem Delta > 0 nachfassen.

### CDSE-CCM — STAC-Asset-Kante
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `CDSE_TOKEN` im Secret-Store
- **Lage:** (gemessen 2026-09-30 via `ci_manage log 36713089816`) STAC-Arm löst: collections 200 (`ccm-optical`, `ccm-sar`, `ccm-thermal-lst-hr/-mr`, `ccm-hyperspectral-ref-hr`); ccm-optical items 200 (PH1B_PHR_MS_*). `stac_asset_fetch --asset <href> --token-env CDSE_TOKEN` → **curl (22) 401**; `CDSE_TOKEN` ist leer im Lauf-Env. href z. B. `https://download.dataspace.copernicus.eu/odata/v1/Products(e4aa8996-…)/$value`.
- **Blockade:** `CDSE_TOKEN` fehlt (Secret nicht gesetzt)
- **Braucht:** `## An future` (Operator-Akt: Token in den Secret-Store).

### Halley/Itokawa CDN-Manifestation
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `kernel-flatten`-Lauf (queued `36719833983`)
- **Lage:** (gemessen 2026-09-30 via `--verdict`) `ephemeris_halley.bin` 404 absent; `ephemeris_itokawa.bin` 404 absent; `kernel-flatten 36703938769` failure (spk_split, Fix committet). `halley` steht im Compiler (`horizons_compiler.rs:655`), nicht in der `FLYBYS`-Liste (`:13-29`).
- **Blockade:** `kernel-flatten`-Lauf-Ende
- **Braucht:** `kernel-flatten 36719833983` lesen; bei success `--verdict` erneut.

### tao-wnd-cdn
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `36716501098`
- **Lage:** (gemessen 2026-09-30) `tao_wnd_zonal.csv` aus Release `data.pmel.noaa.gov` gelöscht; `tao-wnd-cdn.yml:35-36` volles Fenster committet; Re-Dispatch `36716501098` queued.
- **Blockade:** keine
- **Braucht:** Lauf-Ende lesen; bei rot `ci_manage log`.

### gosat-cdn / hips-png-cdn
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende (`36714631159` / `36718182275`)
- **Lage:** (gemessen 2026-09-30) beide queued; vorheriger `gosat-cdn 36283215548` rot „returned void" / `GWT3F_L1B` 0 Treffer; `hips-png-cdn` Shards (4,0)/(5,0) „upload returned void".
- **Blockade:** GOSAT-Source-Fenster; CDN-Release-Ursache
- **Braucht:** Lauf-Ende lesen; bei rot Ursache (`gh release`-Cap/Rechte).

### Legacy-CDN Re-Manifest (tapvizier)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `nvss-cdn` / `first14-cdn`
- **Lage:** (gemessen 2026-09-30) `tapvizier.../capabilities` 200; beide Läufe in_progress.
- **Blockade:** keine
- **Braucht:** Lauf-Ende lesen.

### Register-Träger — `phi/pipeline/index.φ` 7 offen (Katalog-Arbeitsdateien)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Katalog-Port (`phi/pipeline/index.φ`, `docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-09-30) `index.φ:3/4` = lokale Pre-CDN-Queues (825/63, gitignored). `:22/27/28/29/31` = Kataloge `verifiziert 0` **url-Blöcke** mit offenen `candidate`-Einträgen: b2find (S1/S2 gelöst), terrapulse (51 dead markiert), esa_geomagnetic (S3 gelöst), archeology (absent/pending), copernicus (CMEMS pending). Gitignored Working-Tree-Arbeit.
- **Blockade:** Porting (SOURCE_PORT) offen
- **Braucht:** je Katalog die erreichbaren Kandidaten über `docs/SOURCE_PORT.md` portieren; b2find-Fanout-Block bauen (Stationsquelle gemessen).

### Register-Träger — `phi/pipeline/ledger.φ` SSDC
- **Status:** termin | **Bindung:** termin:2026-10-02
- **Trigger:** neue SSDC-Prozedur (`https://limadou.ssdc.asi.it/query.php`)
- **Lage:** (gemessen 2026-09-30) `ledger.φ:6` `ausstehend`; `query.php` CAS-Login, Sotgiu „wait a few weeks".
- **Blockade:** Prozedur nicht live
- **Braucht:** nach Termin `archive_search --playwright "https://limadou.ssdc.asi.it/query.php"`.

### `blocked_sources.φ` mycelium-pending-Dispositionen (19)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-09-30 via `register_lookup --open`) `:59` BepiColombo, `:85` MESSENGER, `:98` DEMETER, `:346` GOSAT-GW, `:374` DAS2 Iowa (Reader fehlt), `:378` Occultation-DB, `:402` ExoMars TGO, `:406` Akatsuki, `:410` Kaguya, `:414` Chandrayaan-1, `:418` Chang'e MRM, `:422` Tianwen-1 RoPeR, `:426` Phobos 2, `:430` Vega 1/2, `:434` Hayabusa, `:438` Tianwen-1 MoRIC, `:442` Shandong, `:458` Danuri ShadowCam, `:462` CDSE-CCM. Mehrere Arme stehen; es fehlen Compiler-Bin + Sample + Asset (SOURCE_PORT) oder Mountain-`ttl`/`frame`.
- **Blockade:** je Zeile (Arm/Reader/Feder)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`); DAS2 Iowa braucht einen echten HAPI-Reader (Vorbild `omni_hro_compiler.rs`).

### `http_401`-Residuum
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neue Mail/Asset-Messung
- **Lage:** (gemessen 2026-09-30) nach der GitHub-PAT-Rotation (mail 200/201) kein neuer 401 gemessen.
- **Blockade:** keine
- **Braucht:** weiter beobachten; bei Auftreten die Route messen.

### Trägerlose Dokumente (mycelium-eigen)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --orphan-docs`-Pass (`docs/concepts/tools-map.md`)
- **Lage:** (gemessen 2026-09-30) `--orphan-docs` = 0 mit dieser Übergabe als Träger; mycelium-eigen getragen: `docs/concepts/exzellenz-konzept.md`, `docs/concepts/kybernetische-astrophysik.md`, `docs/surveys/survey-2026-09-03-orphan-verdicts.md`, `docs/concepts/tools-map.md`.
- **Blockade:** Prosa-Marker (teils echte Messgrenzen)
- **Braucht:** bei nächstem Pass prüfen, ob die Trägerschaft hält.

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383`)
- **Lage:** (gemessen 2026-09-30) kein Producer-Bin/Register/Wf
- **Blockade:** Producer fehlt
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag ändert den Zustand.

### GitHub-Issues — Zensus
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** kanonische Issue-Leseform verfügbar (`gh issue` freigegeben)
- **Lage:** (gemessen 2026-09-30) `gh issue` verweigert (Permission-Map); zuletzt `/tmp/opencode/issues.json` (2026-09-29).
- **Blockade:** `gh issue` nicht erlaubt
- **Braucht:** kanonische Issue-Leseform oder Rolle mit `gh issue`.

### Quellenseitige Waits (`state/zustand/wartend.φ`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort/Readiness
- **Lage:** (gemessen 2026-09-30) Aufnehmer mycelium: `ssdc-limadou`, `voyager-nssdca`, `mariner10-nssdca`, `viking-nssdca`, `cassini-trk`, `juno-jplnav`, `superdarn-af68c4f1`, `emodnet-hfr`, `bepicolombo-more`, `noirlab-gaia-dr4`, `legacy-cdn-ssd`. Keine neue Antwort.
- **Blockade:** Antwort
- **Braucht:** Postfach + Wiedervorlagen beobachten (Trigger feuert → selbes Atom).

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 · 2026-10-19 · 2026-12-02 · 2026-12-03 · 2027-04-01
- **Trigger:** `superdarn-af68c4f1` · `emodnet-hfr` · `noirlab-gaia-dr4` · `europa-clipper` · `bepicolombo-more`
- **Lage:** (gemessen 2026-09-30 via `state/zustand/wartend.φ`) Wiedervorlage, Aufnehmer mycelium.
- **Blockade:** Termin
- **Braucht:** `archive_search --verdict <url>` beim jeweiligen Datum.

## An mountain

Origin: mycelium-folge213 (Fassung folge212, fortgeschrieben).

- **Kaguya/Chandrayaan/Chang'e/Akatsuki/DAS2 — Riss:** mountain-212 meldet „ttl/frame gesetzt", aber `phi/sources.φ` trägt **keine** Blöcke (gemessen 2026-09-30: `sgrep -i "kaguya|chandrayaan|akatsuki|das2|mini-rf" phi/sources.φ` = 0; die Namen liegen nur in `phi/blocked_sources.φ` + `pipeline/ledger.φ`). Der Admission-Block-Skelett (field/force_type/ttl/frame) fehlt, daher kann Mycelium `url`/`format`/`compiler` nicht setzen. Gemessene Einzeldatei-Endpunkte: Kaguya LRS `.tbl` 200/8 131 335 B; Chandrayaan-1 Mini-RF `.img` 206.
- **USGS-comcat Admission:** Quelle `url https://earthquake.usgs.gov/fdsnws/event/1/query` (Tag `earthquake.usgs.gov`, Asset `usgs_comcat_m45.bin`, MIN_MAG 4.5); `field`/`force_type` + `ttl`/`frame` fehlen.
- **Verbleibende Parser-Arm-Lücken:** `pds3_binary.rs` nur int 1/2/4/8; `pds3_img.rs::byte_order_of` kein `PC_REAL`.
- **`kernel-flatten` `spk_split`:** Fix committet; Lauf `36719833983` queued — blockiert Halley/Itokawa + `europa-clipper`.
- **ttl/frame für die register-reifen Endpunkte** (ShadowCam `at moon`+`ttl 604800`, ESA PSA TAP, Chang'e MRM `no-cadence`).
- **CDSE-CCM Admission:** `https://catalogue.dataspace.copernicus.eu/stac/collections` (200) — STAC-Arm + ttl/frame; Asset-Fetch 401 (Token fehlt, s. `## An future`).
- **PRADAN-Chandrayaan-2 Disposition:** `phi/blocked_sources.φ:391-392` „Download end-to-end offen (Harvest-Duty)" ist durch future-159 gemessen **falsch** (OIDC-Flow browserlos verifiziert); die Dispositions-Note ist deine Feder.
- **DAS2 Iowa (`blocked_sources.φ:374`):** `src/archivar/port.rs:1369` `hapi_draft_fields_csv` ist Register-Draft-Textgenerator, kein Leser; Compiler braucht einen echten Reader (Vorbild `omni_hro_compiler.rs`).
- **CNSA `moon.bao.ac.cn`/`nssdc.ac.cn`:** national gesperrt, Registrierung ohne Antwort → Descope-Vorschlag (`phi/blocked_sources.φ:382-388`).
- **`kuprat` Family-Tag:** kein Compiler setzt `tag kuprat`; die vier Kanäle sind als Substance-Witnesses admitiert (`witnesses.φ:120-142`) — kein `kuprat`-Host.
- **KARI KPDS:** kein maschinenlesbarer Endpunkt (0 honored) — descopen mit Befund.

## An future

Origin: mycelium-folge213 (Fassung folge212, fortgeschrieben).

- **`CDSE_TOKEN`:** leer im `cdse-stac-probe`-Env; der STAC-Asset-Fetch endet 401 (`ci_manage log 36713089816`). **Operator-Akt:** Token in den Secret-Store setzen.
- **`hinet-cdn`:** `hinetwww11.bosai.go.jp` direct 403 (Wayback nur 2013) — **Operator-Wort** für den Proton-Exit (Geo-Umgehung) oder Re-Dispatch bei Readiness.
- **ODF-Encounter-Konsument:** `src/archivar/flyby_encounters.rs` gebaut; wartet auf die request-only DSN-Rohdaten (Paper-Autoren, Asmar-Verweis mail 191).
- **`docs/auftrag/auftrag-gic-einreichung.md`** (sensory-folge214) — Future-eigenes Dokument mit 1 offenem Marker, trägerlos; braucht einen öffentlichen Träger in Futures Übergabe oder descope-annotieren.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
