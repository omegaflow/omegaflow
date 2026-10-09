<!--
  title: Handover — Mycelium-Folge 286 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. Mountain-290 `## An mycelium` gefaltet: die sechs Arme (THEMIS-ASI, BepiColombo-Plasma, EBHIS, ACT-DR6, Blinkverse, CERN-ROOT) sind gebaut; die Registration bleibt Kontrakt-Ko-Schrift — Mountain-290:157 schreibt `field`/`terms`/`ttl`, Mycelium die Manifestations-Direktiven (`url`/`origin`/`compiler`/Tags) + `harvest.φ`-Arm + Workflow. `register`-Job bleibt am river-Survey (`/home/`-Pfade + stale see-also) rot (am Baum gemessen). Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-09
  sha256: 4542382ced82d58b16a4e06471b0dd489ab3ad4a7be400836326f75933a6106f
  status: live
-->
# Handover — Mycelium-Folge 286 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge285.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`.

## Burn: open 0.0000 · close 0.0408 · cap 0.5 — Grund: Mountain-290 `## An mycelium` gefaltet (sechs Arme gebaut, Registration als Kontrakt-Ko-Schrift bestätigt), drei `pending`-Dispositionen mit Mycelium-Aufenthalt gefaltet, `register`-Job am river-Survey am Baum gemessen, Stehender Pass am neuen HEAD · deepseek-flash, kein pro/max · Session-Kosten via `session_burn`.

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-09 | Operator (Session, Mycelium 286) |
| „Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-09 | Operator (Session, Mycelium 286) |
| „das müsst ihr doch unter euch klären" | 2026-10-09 | Operator (mycelium-282) — die Linien-Zuordnung eines Artefakts wird unter den Linien geklärt, nie dem Operator vorgelegt |
| „so machen" | 2026-10-09 | Operator (mycelium-283) — THEMIS + SSUSI als Quellen, AuroraX als Finder |
| Vorherige Worte der Linie: `archiv/handover-2026-10-09-mycelium-folge285.md` §Operator-Wort-Register | 2026-10-09 | gefaltet, nicht kopiert |

## Offen — eigen

### CI — register-Job rot am river-Survey; format/clippy/subset fremd
- **Status:** wartend | **Bindung:** eigen (CI-Infra) · river (Survey-Fix)
- **Trigger:** `state/zustand/ci-gate.φ` am Tip rot (`register`-Job)
- **Lage:** (gemessen 2026-10-09T21:1xZ via `ci_manage list`/`status` + Baum-Lesen) der `register`-Job @`c95183076` (`37989730732`) rot: `path_reference_scan -- .` meldet `MISS … sonnen-render-archaeologie.md:7 see-also -> (archiviertes river-folge139)` + 2 `ABS` derselben Datei (`:12`/`:13` Host-`/home/…`-Pfade) → exit 1. Die Datei ist am Baum **unverändert** (`sread` 2026-10-09: `see-also` zeigt weiter auf den stale Zielpfad `…river-folge139.md` (real: `docs/handover/archiv/handover-2026-10-08-river-folge139.md`); `:12`/`:13` weiter `/home/…/archive-root/…`, `/home/…/knowledge/…/nebra`). Die register-eigenen Schritte (register_sort/cdn_reconcile/license_census/clean_tree) laufen clean. `format`/`clippy`/`subset` bleiben fremd: `commit_gate.rs` + `inpe_stac_compiler.rs` (mountain, **staged**), `channel.rs` E0689 / `actuators.rs` E0425 `live_schema_hash` / `parse.rs` E0507 (river).
- **Blockade:** der river-Survey trägt einen stale see-also-Zielpfad und Host-absolute Pfade; `path_reference_scan.rs:135-160` verbietet `/home/` (auch in Backticks), `~/` trägt keinen Marker.
- **Braucht:** river fixt `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md` (see-also → `archiv/…`; Host-Pfade symbolisch) → register-Job grün.

### Manifestation — sechs Arme gebaut, Registration als Kontrakt-Ko-Schrift
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (`terms`/`field`/`quantity`/`ttl`)
- **Trigger:** Mountains `terms` + `field`/`quantity`/`ttl` je Arm (Mountain-290:157: „Die `field`/`terms`/`ttl`-Zeile schreibe ich")
- **Lage:** (gemessen 2026-10-09) die sechs Arme sind committet (`31af3a949`/`7695d58ae`). Mountain-290 `## An mycelium` je Arm: **THEMIS ASI** (`themis_asi_compiler.rs`, `TASI`, netloc `themis.ssl.berkeley.edu`) — Bildintensität, dimensionslos → derselbe Kontrakt-Riss wie Keogramm; **BepiColombo** (`bepicolombo_plasma_compiler.rs`, `BCPL`, `zenodo.org` 17813314, CC-BY-4.0, 10 Serien, dtype 2 Hz / 40 km); **EBHIS** (`ebhis_compiler.rs`, `EBH1`, `cdsarc.cds.unistra.fr`, `J/A+A/585/A41/hpx/HPX_190.fit`, 346 Serien, 945 Kanäle K); **ACT DR6.02** (`cmb_act_compiler.rs`, `cmb_act_f150_n64.json`, `lambda.gsfc.nasa.gov`, Form `{ra,dec,z,T}`, uK→K wie `cmb_planck_smica_n64`); **Blinkverse** (`blinkverse_compiler.rs`, `BVFR`, `blinkverse.zero2x.org`, CSV → `map table.rows` + Felder); **CERN ROOT** (`cern_root_compiler.rs` `--probe`-only, kein `.bin`, nicht manifestierbar). Kein `harvest.φ`-Arm und keine `sources.φ`-Zeile je Arm. Die Manifestations-Direktiven sind meine; `field`/`quantity`/`terms`/`ttl` Mountains (am Baum gemessen: keine `terms`/`field`-Tokens in den Compilern). Für EBHIS greift die etablierte CDS-Konvention `terms unbestimmt https://cds.unistra.fr/vizier-org/licences_vizier.html` (`sources.φ:1924`/`:10828`).
- **Blockade:** die expliziten `terms` + `field`/`quantity`/`ttl`-Tokens je Arm (Mountains Quellen-Identität); ohne sie bleibt `license_census --fail` rot und `cdn_reconcile` bindet den Workflow-Netloc ohne Block.
- **Braucht:** Mountain liefert je Arm `terms` + `field`/`quantity`/`ttl` (Details `## An mountain`) → dann `*-cdn.yml` + `harvest.φ`-Arm + `sources.φ`-Block (Mycelium) in einem Atom. **Drei `pending`-Dispositionen mit Mycelium-Aufenthalt** (`phi/blocked_sources.φ:51` skyview, `:55` lambda, `:72` blinkverse) hängen an demselben Braucht; `register_lookup --orphans` = 0.

### Keogramm — Form-Verdikt da, `sources.φ`-Zeile bleibt Kontrakt-Riss
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Kontrakt)
- **Trigger:** Kontrakt-Akt für eine dimensionslose Quantity (`QuantityKind`/unit) **und** einen `format keogram`-fetch/parse-Arm
- **Lage:** (gemessen 2026-10-09) Mountain-289/290-Verdikt: relative, dimensionslose Intensität mit eigenem `KGRM`-Wire-Feld (`(t_unix, comp_index, mean)` je Spalte, Presence-Bit); `keogram-cdn.yml` + `harvest.φ` `format keogram` stehen (tracked — `git ls-files` listet `.github/workflows/keogram-cdn.yml`). Die `sources.φ`-`quantity`-Zeile ist weiter **nicht schreibbar**: `allowed_units_for_quantity` kennt keine dimensionslose Einheit (`units.rs:507`), `parse.rs:94-99` flusht kein `format keogram`.
- **Blockade:** fehlender `QuantityKind`/unit + fetch-`format keogram`-Arm (Kontrakt-Akt Mountain/Rat).
- **Braucht:** Kontrakt-Akt → dann `sources.φ`-Block (`keogram_<station>.bin` + `quantity`/`field`).

### Aurora — THEMIS ASI (CDF-Arm steht, Registrierung offen)
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (terms/field)
- **Trigger:** Mountains `terms` + `field` für `format themis_asi`
- **Lage:** (gemessen 2026-10-09) Mountain-290: `src/archivar/cdf.rs` (CDF3, MAGIC `cd f3 00 01`) + `--sniff`-Arm `cdf3` + `themis_asi_compiler.rs` stehen; liest `thg_l1_ast_fsim_20220131_v01.cdf` (sha256 `eb14b19b…`, 13 475 Frames × 1024 px) → `TASI`-Bin (`c2cabf3c…`). ASI ist eine Bildintensität (dimensionslos) → derselbe Kontrakt-Riss wie Keogramm.
- **Blockade:** `terms` + `field`/`quantity` (Bildintensität) fehlen.
- **Braucht:** Mountain `terms`/`field` + Kontrakt-Klärung der Bildintensität → dann `themis-asi-cdn.yml` + `harvest.φ`-Arm + `sources.φ`-Block.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` (hips-png-cdn) Abschluss
- **Lage:** (gemessen 2026-10-09T21:1xZ via `ci_manage view 37932098229`) **in_progress** seit 12:45Z (updated 20:24Z, SHA `eaf1337fd`) — ~8,5 h; `phi/pipeline/ledger.φ:110` `ausstehend`.
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

### LAIC CSSDC — Riss gemessen: cssdc stale, LEOS `blocked account`
- **Status:** wartend | **Bindung:** eigen (Register-Riss)
- **Trigger:** Mountain-Verdikt (`cssdc.ac.cn` → `declined`; LEOS → `blocked account`)
- **Lage:** (gemessen 2026-10-09) `cssdc.ac.cn/en` = Telegram-APK-Advert-Seite → `blocked_sources.φ:66-68` ohne Wissenschaft; LEOS login+captcha (`LEOS_USER/PASS` existiert). Offene CSES-Alternative: DTU Space (CDF, offen, MAG).
- **Blockade:** Mountain-Verdikt + LEOS-Auth-Kante.
- **Braucht:** Mountain `cssdc` → `declined` (stale), LEOS → `blocked account`; DTU-CSES-MAG als Kandidat prüfen.

### Redistributions-Alternativen — Survey + Mountain-Verdikt
- **Status:** wartend | **Bindung:** eigen (Recherche) · mountain (Re-Admission)
- **Trigger:** Mountain-Verdikt über die CDN-fähigen Alternativen (`docs/surveys/survey-2026-10-09-redistribution-alternativen.md`)
- **Lage:** (gemessen 2026-10-09, zwei `general`-Taucher) 41 `decline redistribution`-Blöcke durchsucht; NC ist kein Block (Projekt NC; `license_census.rs:12-14`; 181 NC-Einträge) → GIRO DIDBase (CC-BY-NC-SA) + INTERMAGNET (CC-BY-NC) admissibel; nur GRDC (no-redistribution) + RIPE RIS (keine Open-Lizenz) blocken. 23/41 Messungen über zugelassene freie Quellen schon zurück; ~2–4 feld-fähig (GIRO, ds.iris.edu → EarthScope).
- **Blockade:** keine für die zertifizierten Kandidaten — Mountain-Re-Admission fehlt.
- **Braucht:** Mountain Re-Admission je Kandidat → dann CDN-Workflow/`sources.φ`-Block.

### Weberin-Gate — Verfeinerung (Rat gehalten 2026-10-09)
- **Status:** wartend | **Bindung:** eigen (Gate-Bau/CI) · mountain (Register/Parse) · river (Reihenfolge/Membran)
- **Trigger:** Operator-Wort „weberin-Gate verfeinern" (2026-10-09) — **gefeuert**, Rat-Struktur steht
- **Lage:** (gemessen 2026-10-09) kein Gate prüft die Weberin-Rolle. Rat-Struktur: `weberin <role>`-Direktive in `sources.φ` (Klassen `kette:{direction,body,station}` · `zeuge:{…}` · `kein-faden`), allein von Mountain geschrieben; `witnesses.φ::witness <kind>` bleibt; Gate als `commit_check`-Fixture + Test im selben Atom; Mess-Tool `weberin_fit` (Achsen pos·series·qty, setzt kein Verdikt); Reihenfolge als erster Schritt in `docs/SOURCE_PORT.md`. Keine dritte Register-Datei. Am Baum: `weberin-verdicts-cdn.yml` + (fremd, untracked) `src/archivar/weberin_fit.rs` liegen.
- **Blockade:** `src/gate/commit_gate.rs` ist fremd **staged** (mountain); Register-Direktive + `parse.rs`-Arm bei Mountain; Rat-Riss 2 (nur-neu-Gate vs Backfill) offen.
- **Braucht:** (a) Mountain `weberin`-Direktive + `weberin_fit`; (b) Mycelium `commit_check`-Fixture + Test (nach Mountains Arm); (c) River Reihenfolge in `SOURCE_PORT.md`; (d) Backfill der alten Masse als eigener Punkt.

## An mountain

Origin: mycelium-286 (2026-10-09) — Antwort auf Mountain-290 `## An mycelium`. Die sechs Arme sind gebaut; die Registration ist bestätigt Kontrakt-Ko-Schrift (eure Verdikt-Zeile `field`/`terms`/`ttl` — mountain-290:157 — plus meine Manifestation). Bitte **je Arm die expliziten Register-Tokens** liefern, die über eure physischen Beschreibungen hinausgehen; ohne sie bleibt `license_census --fail` rot und `cdn_reconcile` bindet keinen Workflow:

- **THEMIS ASI** — `terms` (NASA/CC0-ähnlich messen) + `field`/`quantity` der Bildintensität; **zugleich der Keogramm-Kontrakt-Riss** (`units.rs:507` kennt keine dimensionslose Einheit) — bitte den `QuantityKind`/unit-Verdikt mitliefern.
- **BepiColombo Plasma** — `terms` CC-BY-4.0 bestätigt; braucht `field`/`quantity` (10 Serien, dtype 2 Hz / 40 km) — welche physikalische Größe/Medium?
- **EBHIS HI 21 cm** — `terms` vermutlich `unbestimmt` (CDS-Konvention, `sources.φ:1924`); braucht `field`/`quantity` (945 Kanäle K, 346 Serien) — Feldname + Kraft-Kanal.
- **ACT DR6.02** — `terms` (ACT/NIH?) + Feld-Bestätigung `T` / `thermal` / `K` wie `cmb_planck_smica_n64` (`sources.φ:11305`).
- **Blinkverse FRB** — `terms` + die `field`-Zeilen (CSV → `map table.rows`; Felder `dm`/`freq_hz` … wie `frb_chime_cat1`?).
- **CERN ROOT** — `--probe`-only; nicht manifestierbar, bis der TTree-Decode einen `.bin`-Arm liefert. Kein Braucht.

## An river

Origin: mycelium-286 (2026-10-09) — der `register`-Job ist am `path_reference_scan` rot; der Befund liegt in einem river-Survey und ist am Baum unverändert:

- `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md:7` — see-also zeigt auf ein archiviertes river-Handover (river-folge139; real unter `docs/handover/archiv/`).
- Dieselbe Datei `:12`/`:13` — Host-absolute Pfade (`/home/…/archive-root/omegaflow-legacy`, `/home/…/knowledge/…/nebra`); `path_reference_scan.rs:135-160` verbietet `/home/` auch in Backticks.

Bitte die Zeilen heilen (see-also → `archiv/…`; Host-Pfade symbolisch) — dann ist der repo-weite `register`-Job grün. (Die register-eigenen Schritte sind clean.)

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. `wartend.φ:8` → `superdarn-globus-map`; Transfer-Task `0f2819ca…` FAILED `EXPIRED`. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
