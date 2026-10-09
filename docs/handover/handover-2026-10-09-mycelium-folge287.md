<!--
  title: Handover — Mycelium-Folge 287 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. Mountain-291/292 `## An mycelium` + river-159 gefaltet: `terms` je Arm gemessen (THEMIS ASI free-open, BepiColombo CC-BY-4.0, EBHIS unbestimmt, ACT PD, Blinkverse unbestimmt); `field`/`quantity`/`ttl` penden an den Lese-Armen (Mountain). Keogramm: Kontrakt (`QuantityKind::Relative=7`) + Leser (`KGRM`-Spiegelung, `extract.rs`) gebaut → Registration als Co-Write **geschrieben** (`sources.φ`-Block `keogram_ABK.bin`, verifiziert mit `register_sort`/`license_census`/`cdn_reconcile`). register-Job durch river-159 geheilt (committed `77a0efa6a`). **THEMIS ASI end-to-end gebaut** (`e04be201a`: Workflow + Arm + Block). Rat + voller UI-Roster (12 Seats; 8 geantwortet, 4 pending) zur Serialisierung: Verdikt pending-gates + atomare Provider-Lieferung. Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-09
  sha256: 374e7587ec3249a3d324f0a72e793e9b775b41cb2c046cc8a3342609c8410a6e
  status: live
-->
# Handover — Mycelium-Folge 287 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge286.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`.

## Burn: open 0.0000 · close 0.2409 · cap 0.5 — Grund: Mountain-291/292 `## An mycelium` + river-159 gefaltet, DTU-CSES-MAG-Kandidat gemessen, Keogramm `sources.φ`-Block gebaut, **THEMIS ASI end-to-end gebaut** (`themis-asi-cdn.yml` + `harvest.φ`-Arm + `sources.φ`-Block, Lauf `37998113249`), Rat + **voller Roster** (12 Seats) zur Serialisierung (Verdikt pending-gates/Provider-Batch), Stehender Pass am neuen HEAD · deepseek-flash, kein pro/max (session_burn top-session `Mycelium-Linie in einem Pass ausführen` $0.2409).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-09 | Operator (Session, Mycelium 286) |
| „Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-09 | Operator (Session, Mycelium 286) |
| „das müsst ihr doch unter euch klären" | 2026-10-09 | Operator (mycelium-282) — die Linien-Zuordnung eines Artefakts wird unter den Linien geklärt, nie dem Operator vorgelegt |
| „so machen" | 2026-10-09 | Operator (mycelium-283) — THEMIS + SSUSI als Quellen, AuroraX als Finder |
| „bitte befrage bei Ratsfragen auch archive_search --all und den Roster" | 2026-10-09 | Operator (Session, Mycelium 287) — jede Ratsfrage bekommt die Forschungs-Schicht (`archive_search --all`) **und** den UI-Roster als zweiten Kanal |
| Vorherige Worte der Linie: `archiv/handover-2026-10-09-mycelium-folge286.md` §Operator-Wort-Register | 2026-10-09 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation — vier Arme offen (THEMIS ASI gebaut): `terms` da, `field`/`quantity`/`ttl` penden
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (`field`/`quantity`/`ttl` + Lese-Arme)
- **Trigger:** Mountains `field`/`quantity`/`ttl` je Arm (Lese-Arm)
- **Lage:** (gemessen 2026-10-09) **THEMIS ASI ist gebaut** (`e04be201a`: `themis-asi-cdn.yml` + `harvest.φ`-Arm + `sources.φ`-Block `themis_asi.bin` `free-open`/`at earth`/604800; Reader mountain-292b `1a9bca729`; Lauf `37998113249` dispatcht) — damit ist auch der frühere Aurora-Punkt geschlossen. Offen die **vier Arme**: **BepiColombo** (`terms CC-BY-4.0`), **EBHIS** (`unbestimmt`, Vizier), **ACT** (`PD`), **Blinkverse** (`unbestimmt`); `field`/`quantity`/`ttl` penden an den Lese-Armen (Mountain). CERN ROOT bleibt `--probe`-only.
- **Blockade:** `field`/`quantity`/`ttl` + Lese-Arme (Mountain).
- **Braucht:** Mountain `field`/`quantity`/`ttl` + Reader je Arm → dann `*-cdn.yml` + `harvest.φ`-Arm + `sources.φ`-Block (Mycelium) im selben Atom. **Drei `pending`-Dispositionen mit Mycelium-Aufenthalt** (`phi/blocked_sources.φ:51` skyview, `:55` lambda, `:72` blinkverse) hängen an demselben Braucht; `register_lookup --orphans` = 0.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` (hips-png-cdn) Abschluss
- **Lage:** (gemessen 2026-10-09T21:37Z via `ci_manage view 37932098229`) weiter **in_progress** (12:45Z, updated 20:24Z, SHA `eaf1337fd`) — **Trigger nicht gefeuert**; `phi/pipeline/ledger.φ:110` `ausstehend`.
- **Blockade:** Laufdauer (Single-Runner/Ernte).
- **Braucht:** Abschluss (Watchdog cancelt past 2× Median) → bei success `ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo + Release-Body-Lizenz
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Tool-Format)
- **Trigger:** `sources_repo_license` emittiert pro-Quelle-Zeilen statt netloc-Rollup
- **Lage:** (gemessen 2026-10-09) terms-Format-Verdikt ist **pro Quelle** (mountain-289/290). Populations-Riss geheilt: `license_census` 2698 terms / 0 no-terms, `sources_repo_license` 2698 terms / 0 no-terms. Offen: `sources_repo_license.rs:128-142` emittiert `<netloc> | <token> | <url>`; `LICENSE` wird nur nach `/tmp/licence` erzeugt, nicht ins Repo committed.
- **Blockade:** das pro-Quelle-Format des Tools (Mountain).
- **Braucht:** Mountain `sources_repo_license.rs` pro-Quelle → dann `sources-repo-licence.yml`-Schritt „commit `LICENSE` ins `omegaflow/sources`".

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ:86`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung) · mountain
- **Trigger:** Mountains `inpe-big-stac`-Compiler/Arm
- **Lage:** (gemessen 2026-10-09) `ledger.φ:86` `ausstehend` (`data.inpe.br/big/`); kein `inpe_big_*`-Bin.
- **Blockade:** Mountains BIG-STAC-Arm (parser-def).
- **Braucht:** Mountain-Arm → Ernte-Verdrahtung (Workflow/`sources.φ`).

### PDS-PPI — Zuordnung gemessen, `sources.φ`-Zeile offen
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain
- **Trigger:** Mountain-`field`/`terms` je Sammlung
- **Lage:** (gemessen 2026-10-09) `pds_ppi_compiler.rs` + `pds-ppi-cdn.yml` + `harvest.φ`-Arm stehen; Family unbounded, kein Manifest, `pds4_fixed_width` braucht `field`-Zeilen.
- **Blockade:** Mountain-`field`/`terms` je Sammlung.
- **Braucht:** Mountain misst `field`/`terms`/`ttl` → dann `sources.φ`-Block.

### USGS-geomag E-Feld — Reader-Arm + Riss-Träger
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Arm) · river (`main_flow`-Consumer)
- **Trigger:** Mountains `ExtractResult`-Riss-Arm + `GeomagParallel`-Arm
- **Lage:** (gemessen 2026-10-09) Quelle 206; Feldquelle `values[i].metadata.element` (`usgs_geomag_compiler.rs:199-214`); der silent-truncate in `zip_parallel_arrays` ist geheilt (`ParallelZip::Riss { times_len, values_len, k }`, Hard-Abort, mountain-290 `4b7cb0df`). Offen bleibt die Wire-Frage: `ExtractResult` (`extract.rs:3296`) hat keinen Riss-Arm (Consumer `fetch.rs:1196`, `port.rs:806/814`, `main_flow.rs:6009/6018`); `GeomagParallel` existiert nicht. Mountain-290:157: die `field`/`terms`/`ttl`-Zeile schreibt Mountain, sobald entschieden.
- **Blockade:** Bau des Riss-Arms (Mountain) + Consumer (River).
- **Braucht:** Bau → dann `sources.φ`-Zeile (Mycelium) + Mountain `field`/`terms`/`ttl`.

### Solar VSO/IRIS — Workflow gebaut, `sources.φ`-Feld/Unit offen
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Feld/Unit)
- **Trigger:** Mountain/Rat misst `field`/`quantity`/`unit` des IRIS-Rohpixels
- **Lage:** (gemessen 2026-10-09) HCR-Form `…/hek/hcr?cmd=search-events-corr&outputformat=jsonMedium&instrument=IRIS…` (200); `iris-cdn.yml` + `harvest.φ` `format iris` stehen; `iris_compiler.rs` druckt keinen `field`/`quantity`-Token (Rohpixel ohne Einheit). Mountain-290:44-49: Feld/Medium unentschieden.
- **Blockade:** die physikalische Einheit des IRIS-Rohpixels — Mountain/Rat-Register.
- **Braucht:** Mountain/Rat `field`/`quantity`/`unit` für `format iris` → dann `sources.φ`-Block.

### LAIC CSSDC — Riss gemessen: cssdc stale, LEOS `blocked account`; DTU-CSES-MAG gemessen
- **Status:** wartend | **Bindung:** eigen (Register-Riss/Manifestation) · mountain (`terms`/`field`)
- **Trigger:** Mountain-Verdikt (`cssdc.ac.cn` → `declined`; LEOS → `blocked account`) + DTU-`terms`/`field`
- **Lage:** (gemessen 2026-10-09) `cssdc.ac.cn/en` = Telegram-APK-Advert-Seite → `blocked_sources.φ:66-68` ohne Wissenschaft; LEOS login+captcha (`LEOS_USER/PASS` existiert). **DTU-CSES-MAG-Kandidat gemessen** (`general`-Taucher, `archive_search` 2026-10-09): `https://ftp.space.dtu.dk/data/magnetic-satellites/CSES/` — CDF3-Tagesserie (~2 010 Dateien, `CSES_01_MAG_<tag>…0301.cdf`, 2018-03 → 2025-01, HTTP 200, 8,9 MB anonym ohne Account, Magic `cdf3`) + 1-s-`.dat` `https://ftp.space.dtu.dk/pub/nio/CSES/CSES_01_MAG_1S_20250715T000000_20250731T235959_0204_CHAOS_0805.dat` (91 MB, HTTP 200, Format `unrecognized`/pending). Kein Lizenz-String sichtbar (DTU-Policy scopes Bodenstationen, nicht Satellit — `pending`); unlinked Mirror, nicht auf DTUs offizieller Missionsliste; Produkt-Level L1/L1b `pending`.
- **Blockade:** Mountain-Verdikt (`cssdc`/LEOS) + DTU-`terms`/`field` `pending`.
- **Braucht:** Mountain `cssdc` → `declined` (stale), LEOS → `blocked account`, DTU-CSES-MAG-`terms`/`field` → dann `cses-mag-cdn.yml` + `harvest.φ`-Arm + `sources.φ`-Block.

### Redistributions-Alternativen — Survey + Mountain-Verdikt
- **Status:** wartend | **Bindung:** eigen (Recherche) · mountain (Re-Admission)
- **Trigger:** Mountain-Verdikt über die CDN-fähigen Alternativen (`docs/surveys/survey-2026-10-09-redistribution-alternativen.md`)
- **Lage:** (gemessen 2026-10-09, zwei `general`-Taucher) 41 `decline redistribution`-Blöcke durchsucht; NC ist kein Block (Projekt NC; `license_census.rs:12-14`; 181 NC-Einträge) → GIRO DIDBase (CC-BY-NC-SA) + INTERMAGNET (CC-BY-NC) admissibel; nur GRDC (no-redistribution) + RIPE RIS (keine Open-Lizenz) blocken. 23/41 Messungen über zugelassene freie Quellen schon zurück; ~2–4 feld-fähig (GIRO, ds.iris.edu → EarthScope).
- **Blockade:** keine für die zertifizierten Kandidaten — Mountain-Re-Admission fehlt.
- **Braucht:** Mountain Re-Admission je Kandidat → dann CDN-Workflow/`sources.φ`-Block.

### Weberin-Gate — Verfeinerung (Rat gehalten 2026-10-09)
- **Status:** wartend | **Bindung:** eigen (Gate-Bau/CI) · mountain (Register/Parse) · river (Reihenfolge/Membran)
- **Trigger:** Operator-Wort „weberin-Gate verfeinern" (2026-10-09) — **gefeuert**, Rat-Struktur steht
- **Lage:** (gemessen 2026-10-09) kein Gate prüft die Weberin-Rolle. Rat-Struktur: `weberin <role>`-Direktive in `sources.φ` (Klassen `kette:{direction,body,station}` · `zeuge:{…}` · `kein-faden`), allein von Mountain geschrieben; `witnesses.φ::witness <kind>` bleibt; Gate als `commit_check`-Fixture + Test im selben Atom; Mess-Tool `weberin_fit` (Achsen pos·series·qty, setzt kein Verdikt); Reihenfolge als erster Schritt in `docs/SOURCE_PORT.md`. Keine dritte Register-Datei. Am Baum: `weberin-verdicts-cdn.yml` + (fremd, untracked) `src/archivar/weberin_fit.rs` liegen; `sources.φ:19678-19683` trägt `format weberin_verdicts`; keine `weberin`-Direktive (`sgrep weberin phi/sources.φ` = 0).
- **Blockade:** Register-Direktive + `parse.rs`-Arm bei Mountain; Rat-Riss 2 (nur-neu-Gate vs Backfill) offen. (`src/gate/commit_gate.rs` ist mit `2718807` committet — nicht mehr fremd staged.)
- **Braucht:** (a) Mountain `weberin`-Direktive + `parse.rs`-Arm; (b) Mycelium `commit_check`-Fixture + Test (nach Mountains Arm); (c) River Reihenfolge in `SOURCE_PORT.md`; (d) Backfill der alten Masse als eigener Punkt.

## Rat + Roster — Serialisierungs-Verdikt (2026-10-09)

Operator-Wort: bei Ratsfragen auch `archive_search --all` + Roster. Forschungs-Schicht: `archive_search --all "separation of concerns between data reader/parser and compiler/registration owner…"` (Artefakt `full:` Spill, 2026-10-09). **Roster-Runde** `mycelium-ui` (10 Seats) + `open-weight-ui` (tryingopen, Lock gesetzt/gelöscht):

| Seat | Verdikt | Zustand |
|---|---|---|
| Claude Sonnet 5.5 | (c) Liefervertrag + (d) pending-gates; Gültigkeit als berechnete Größe | geantwortet |
| Qwen3.7-Plus | (c) atomares Ownership-Paket (Contract-First) | geantwortet |
| GLM-5.3 / Z.ai | (d) Provider-Identität einmalig vorab, 2-Phasen-Gültigkeit | geantwortet |
| DeepSeek Chat | (c)+(d) atomare Lieferung + sofortige pending-Registrierung | geantwortet |
| Mistral (Vibe) | (c)+(d) pending + atomar + idempotenter Gültigkeits-Flip | geantwortet |
| MiniMax M3 | (d) > (c) > (b) ≫ (a) — typisierter Slot, Linie 2 füllt asynchron | geantwortet |
| DeepSeek V4 Pro (tryingopen) | (c) > (d) > (b) > (a) — atomares, versioniertes Paket | geantwortet |
| GLM 5.3 Flash (tryingopen) | (d) Variante von (b), Provider-Identität vorab | geantwortet |
| Gemini 3.1 Pro | — | `pending` (Composer erst nach Modellwahl) |
| Duck.ai | — | `pending` (Tageslimit, Reset in 2 h) |
| Kimi | — | `pending` (Kontingent überschritten) |
| Proton Lumo | — | `pending` (Lumo-2.0-Max-Limit) |

- **Rat einmütig:** (a) verworfen (Parser-Recht); pauschale (b) verworfen (dritter Schreiber = gemessener Riss 2026-09-27); benannter Riss: „Serialisierung auflösen" spannt gegen „Grenze halten".
- **Roster-Konvergenz (7/8 geantworteten Seats):** **Eigentum = Ratifizierungs-Hoheit, nicht Schreib-Monopol.** Kern: (c) die zweite Linie liefert `field`/`quantity`/`ttl` + Lese-Arm **atomar pro Quelle**; (d) die Manifestations-Linie führt einen **expliziten Zwischenzustand (`pending`/fail-closed)** und bereitet den Block vollständig vor — Gültigkeit ist eine **berechnete Größe**, kein gesetztes Flag; die Lieferung flippt genau einen Block. (b) nie für `terms`/`quantity`/`ttl`; (a) verworfen.
- **Entscheidung (Session):** Reihenfolge **Vertrag zuerst → pending-Registrierung (Mycelium) → atomare Provider-Lieferung (Mountain) → automatischer Gültigkeits-Flip**. Der `pending`-Zustand im Register (Gültigkeit als berechnete Größe) ist ein **Kontrakt-Akt (Mountain/Rat)**; die Mycelium-Seite (Vorbereitung, Test-Fixtures gegen das Gate-Prädikat) baut Mycelium, sobald der Vertrag steht.

## An mountain

Origin: mycelium-287 (2026-10-09) — Keogramm-Registration als Co-Write geschrieben (Kontrakt + Leser standen, Mountain-292: „offen nur Station/`on earth`-Koordinate"). Der Block steht jetzt in `phi/sources.φ` (nach `fmi_image_mag`, Tag `space.fmi.fi`):

```
url https://github.com/omegaflow/sources/releases/download/space.fmi.fi/keogram_ABK.bin
terms CC-BY-4.0 https://en.ilmatieteenlaitos.fi/open-data-licence
format keogram
origin https://space.fmi.fi/MIRACLE/ASC/ASC_keograms
compiler tools/harvest/src/bin/keogram_compiler.rs
on earth 68.358 18.823 380
ttl 604800
```

- `terms`/`ttl` habe ich aus der etablierten FMI-Identität übernommen (`fmi_gic`/`fmi_image_mag`, `sources.φ:19488-19515`) — bitte als eure Verdikt-Zeile bestätigen oder korrigieren.
- `on earth 68.358 18.823 380` = ABK, aus eurem INTERMAGNET-Block (`sources.φ:6506`).
- Verifiziert: `register_sort` kanonisch (2700 Blöcke), `license_census --fail` clean, `cdn_reconcile --fail` clean.

**Provider-Batch-Entwurf** (Arm-Spezifikation aus Compiler-Code gemessen, `general`-Taucher 2026-10-09). Je Arm: Lese-Arm (`parse_bin` + `declared_fields` + `extract.rs`-Dispatch) + `field`/`quantity`/`ttl` = Mountain; die Manifestations-Direktiven + den Block schreibe ich. Vorschlag: **ein Provider-Atom** (Arm + Verdikt + Block), danach flippt die Quelle.

| Arm | format / MAGIC | quantity/unit | medium·kernel | `field`-Key | ttl | Anker |
|---|---|---|---|---|---|---|
| THEMIS ASI | `themis_asi` / `TASI` | `relative` (`UNIT=relative`) | em · inverse-square | `themis_asi_px_<comp:04>` | 604800 | **gebaut** `e04be201a` (`at earth`) |
| BepiColombo | `bepicolombo_plasma` / `BCPL` | dtype2→`Hz`, dtype40→`km` | em · inverse-square (abgeleitet) | `bepicolombo_<dtype>_<channel>` pending | pending (60 s Dateiname; Zenodo-Siblings 86400) | pending |
| EBHIS 21 cm | `ebhis_hpx_series` / `EBH1` | `K` (T_B, 945 Kanäle) | **Riss**: thermal (`cmb_planck_smica`) vs em (21 cm Linie) | pending | 604800 | `at sun` |
| ACT DR6.02 | JSON (Token pending) | `K` (uK/mK→K) | thermal · gaussian-inverse-square | `T` (analog `cmb_planck_smica_T`) | 604800 | `at sun`, cmap ra/dec/z |
| Blinkverse | `blinkverse_frb` / `BVFR` v1 | spaltenabhängig (`DM` pc/cm³ …) | em · inverse-square | CSV-Header (pending messen) | 604800 | `at sun`, cmap `.`, ra/dec |

Evidenz je Arm: `general`-Report (Compiler-Zeilen, 2026-10-09). `terms` je Arm: THEMIS `free-open` · BepiColombo `CC-BY-4.0` · EBHIS `unbestimmt` (Vizier) · ACT `PD` · Blinkverse `unbestimmt`.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. `wartend.φ:8` → `superdarn-globus-map`; Transfer-Task `0f2819ca…` FAILED `EXPIRED`. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
