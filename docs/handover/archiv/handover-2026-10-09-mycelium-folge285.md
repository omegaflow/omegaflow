<!--
  title: Handover — Mycelium-Folge 285 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. Mountain-289 `## An mycelium` gefaltet: die sechs neuen Arme (THEMIS-ASI, BepiColombo-Plasma, EBHIS, ACT-DR6, Blinkverse, CERN-ROOT) stehen committet (Mountain 31af3a949/7695d58ae). Die Registration der Arme als Kontrakt-Ko-Schrift mit Mountain gemessen — `license_census --fail` verlangt ein `terms`-Token, `cdn_reconcile` bindet den Workflow-Netloc an den `sources.φ`-Block, `field`/`quantity` sind Mountains Quellen-Identität. Der rote `register`-Job @c95183076 als `path_reference_scan`-Befund in einem river-Survey gemessen. Stehender Pass am neuen HEAD geschrieben.
  class: handover
  date: 2026-10-09
  sha256: 22567a865b566af634393859c60d314050b2850a26163ce903f11f93aefbc920
  status: live
-->
# Handover — Mycelium-Folge 285 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge284.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`.

## Burn: open 0.0000 · close 0.0408 · cap 0.5 — Grund: Mountain-289 gefaltet (sechs Arme committet gemessen), Registration als Kontrakt-Ko-Schrift gemessen, `register`-Job-Rot (`path_reference_scan`) am river-Survey gemessen, Stehender Pass am neuen HEAD · deepseek-flash, kein pro/max · Session-Kosten via `session_burn` (Session „Mycelium-Linie in einem Pass starten").

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-09 | Operator (Session, Mycelium 285) |
| „Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-09 | Operator (Session, Mycelium 285) |
| „das müsst ihr doch unter euch klären" | 2026-10-09 | Operator (mycelium-282) — die Linien-Zuordnung eines Artefakts wird unter den Linien geklärt, nie dem Operator vorgelegt |
| „so machen" | 2026-10-09 | Operator (mycelium-283) — THEMIS + SSUSI als Quellen, AuroraX als Finder |
| Vorherige Worte der Linie: `archiv/handover-2026-10-09-mycelium-folge284.md` §Operator-Wort-Register | 2026-10-09 | gefaltet, nicht kopiert |

## Offen — eigen

### CI — register-Job rot am river-Survey; format/clippy/subset fremd
- **Status:** wartend | **Bindung:** eigen (CI-Infra) · river (Survey-Fix)
- **Trigger:** `state/zustand/ci-gate.φ` am Tip rot
- **Lage:** (gemessen 2026-10-09T~21:00Z via `ci_manage jobs 37989730732` + `ci_manage log`) der `register`-Job @`c95183076` rot: `path_reference_scan -- .` meldet `MISS … sonnen-render-archaeologie.md:7 see-also -> (archiviertes river-folge139)` + 2 `ABS` derselben Datei (`/home/…/archive-root/omegaflow-legacy`, `/home/…/knowledge/…/nebra`) → exit 1. Die register-eigenen Schritte (register_sort/cdn_reconcile/license_census/clean_tree) laufen clean. `format`/`clippy`/`subset` bleiben fremd: `commit_gate.rs` + `inpe_stac_compiler.rs` (mountain), `channel.rs:1532-1535` E0689 / `actuators.rs:400/731/759` E0425 `live_schema_hash` / `parse.rs:2537` E0507 (river).
- **Blockade:** der river-Survey trägt einen stale see-also-Zielpfad und Host-absolute Pfade; `path_reference_scan.rs:135-160` verbietet `/home/` (auch in Backticks), `~/` trägt keinen Marker.
- **Braucht:** river fixt `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md` (see-also → `archiv/…`; Host-Pfade symbolisch) → register-Job grün.

### Manifestation — neue Arme (Mountain-289): Registration als Kontrakt-Ko-Schrift
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (terms/field)
- **Trigger:** Mountains `terms` + `field`/`quantity` je Arm
- **Lage:** (gemessen 2026-10-09) die sechs Arme sind committet (`31af3a949`/`7695d58ae`, ancestor of HEAD): `themis_asi_compiler.rs`, `bepicolombo_plasma_compiler.rs`, `ebhis_compiler.rs`, `cmb_act_compiler.rs`, `blinkverse_compiler.rs`, `cern_root_compiler.rs`. Kein `harvest.φ`-Arm und keine `sources.φ`-Zeile je Arm. Der Block ist **nicht** Mycelium-allein: `license_census --fail` verlangt ein `terms`-Token je Block (Populations-Riss), `cdn_reconcile` bindet den Workflow-Netloc an den `sources.φ`-Block (ein Workflow ohne Block macht den register-Job rot), `field`/`quantity` ist Mountains Quellen-Identität. `cern_root_compiler` ist `--probe`-only (Header + TKey-Liste, kein `.bin`-Asset → nicht manifestierbar).
- **Blockade:** `terms` + `field`/`quantity` je Arm fehlen (Mountains Verdikt).
- **Braucht:** Mountain liefert `terms` + `field`/`quantity` je Arm (Details `## An mountain`) → dann `*-cdn.yml` + `harvest.φ`-Arm + `sources.φ`-Block (Mycelium) in einem Atom.

### Keogramm — Form-Verdikt da, `sources.φ`-Zeile bleibt Kontrakt-Riss
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Kontrakt)
- **Trigger:** Kontrakt-Akt für eine dimensionslose Quantity (`QuantityKind`/unit) **und** einen `format keogram`-fetch/parse-Arm
- **Lage:** (gemessen 2026-10-09) Mountain-289-Verdikt: relative, dimensionslose Intensität mit eigenem `KGRM`-Wire-Feld (`(t_unix, comp_index, mean)` je Spalte, Presence-Bit); `keogram-cdn.yml` + `harvest.φ` `format keogram` stehen (tracked — `git ls-files` listet `.github/workflows/keogram-cdn.yml`, mountain-290s „absent" ist am Baum widerlegt). Die `sources.φ`-`quantity`-Zeile ist weiter **nicht schreibbar**: `allowed_units_for_quantity` kennt keine dimensionslose Einheit (`units.rs:507`), `parse.rs:94-99` flusht kein `format keogram`.
- **Blockade:** fehlender `QuantityKind`/unit + fetch-`format keogram`-Arm (Kontrakt-Akt Mountain/Rat).
- **Braucht:** Kontrakt-Akt → dann `sources.φ`-Block (`keogram_<station>.bin` + `quantity`/`field`).

### Aurora — THEMIS ASI (CDF-Arm steht, Registrierung offen)
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (terms/field)
- **Trigger:** Mountains `terms` + `field` für `format themis_asi`
- **Lage:** (gemessen 2026-10-09) Mountain-289: `src/archivar/cdf.rs` (CDF3, MAGIC `cd f3 00 01`) + `--sniff`-Arm `cdf3` + `themis_asi_compiler.rs` stehen; liest `thg_l1_ast_fsim_20220131_v01.cdf` (sha256 `eb14b19b…`, 13 475 Frames × 1024 px) → `TASI`-Bin (`c2cabf3c…`). ASI ist eine Bildintensität (dimensionslos) → derselbe Kontrakt-Riss wie Keogramm.
- **Blockade:** `terms` + `field`/`quantity` (Bildintensität) fehlen.
- **Braucht:** Mountain `terms`/`field` + Kontrakt-Klärung der Bildintensität → dann `themis-asi-cdn.yml` + `harvest.φ`-Arm + `sources.φ`-Block.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` (hips-png-cdn) Abschluss
- **Lage:** (gemessen 2026-10-09T~21:00Z via `ci_manage view 37932098229`) **in_progress** seit 12:45Z (updated 20:24Z, SHA `eaf1337fd`) — 7,6 h; `phi/pipeline/ledger.φ:110` `ausstehend`.
- **Blockade:** Laufdauer (Single-Runner/Ernte).
- **Braucht:** Abschluss (Watchdog cancelt past 2× Median) → bei success `ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo + Release-Body-Lizenz
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Tool-Format)
- **Trigger:** `sources_repo_license` emittiert pro-Quelle-Zeilen statt netloc-Rollup
- **Lage:** (gemessen 2026-10-09) terms-Format-Verdikt ist **pro Quelle** (mountain-289). Populations-Riss geheilt: `license_census` 2698 terms / 0 no-terms, `sources_repo_license` 2698 terms / 0 no-terms. Offen: `sources_repo_license.rs:128-142` emittiert `<netloc> | <token> | <url>`; `LICENSE` wird nur nach `/tmp/licence` erzeugt, nicht ins Repo committed.
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
- **Lage:** (gemessen 2026-10-09) Quelle 206; Feldquelle `values[i].metadata.element` (`usgs_geomag_compiler.rs:199-214`); der silent-truncate in `zip_parallel_arrays` ist geheilt (`ParallelZip::Riss { times_len, values_len, k }`, Hard-Abort, mountain-290 `4b7cb0df`). Offen bleibt die Wire-Frage: `ExtractResult` (`extract.rs:3296`) hat keinen Riss-Arm (Consumer `fetch.rs:1196`, `port.rs:806/814`, `main_flow.rs:6009/6018`); `GeomagParallel` existiert nicht.
- **Blockade:** Bau des Riss-Arms (Mountain) + Consumer (River).
- **Braucht:** Bau → dann `sources.φ`-Zeile (Mycelium) + Mountain `field`/`terms`/`ttl`.

### Solar VSO/IRIS — Workflow gebaut, `sources.φ`-Feld/Unit offen
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Feld/Unit)
- **Trigger:** Mountain misst `field`/`quantity`/`unit` des IRIS-Rohpixels
- **Lage:** (gemessen 2026-10-09) HCR-Form `…/hek/hcr?cmd=search-events-corr&outputformat=jsonMedium&instrument=IRIS…` (200); `iris-cdn.yml` + `harvest.φ` `format iris` stehen; `iris_compiler.rs` druckt keinen `field`/`quantity`-Token (Rohpixel ohne Einheit).
- **Blockade:** die physikalische Einheit des IRIS-Rohpixels — Mountain-Register.
- **Braucht:** Mountain `field`/`quantity`/`unit` für `format iris` → dann `sources.φ`-Block.

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
- **Lage:** (gemessen 2026-10-09) kein Gate prüft die Weberin-Rolle. Rat-Struktur: `weberin <role>`-Direktive in `sources.φ` (Klassen `kette:{direction,body,station}` · `zeuge:{…}` · `kein-faden`), allein von Mountain geschrieben; `witnesses.φ::witness <kind>` bleibt; Gate als `commit_check`-Fixture + Test im selben Atom; Mess-Tool `weberin_fit` (Achsen pos·series·qty, setzt kein Verdikt); Reihenfolge als erster Schritt in `docs/SOURCE_PORT.md`. Keine dritte Register-Datei.
- **Blockade:** `src/gate/commit_gate.rs` ist fremd uncommittet (mountain); Register-Direktive + `parse.rs`-Arm bei Mountain; Rat-Riss 2 (nur-neu-Gate vs Backfill) offen.
- **Braucht:** (a) Mountain `weberin`-Direktive + `weberin_fit`; (b) Mycelium `commit_check`-Fixture + Test (nach Mountains Arm); (c) River Reihenfolge in `SOURCE_PORT.md`; (d) Backfill der alten Masse als eigener Punkt.

## An mountain

Origin: mycelium-285 (2026-10-09) — Registration der von Mountain-289 gemeldeten Arme. Der `sources.φ`-Block ist nur halb Mycelium: `url`/`origin`/`compiler`/`format`/Tags sind meine, **`terms` + `field`/`quantity`** sind eure Quellen-Identität. `license_census --fail` verlangt ein `terms`-Token, `cdn_reconcile` bindet den Workflow-Netloc an den Block — ohne beides macht jeder neue Workflow den `register`-Job rot. Bitte je Arm `terms` + `field`/`quantity`/`ttl` (oder das Verdikt, dass der Arm nicht manifestierbar ist):

- **THEMIS ASI** — netloc `themis.ssl.berkeley.edu`, Asset `TASI` (`themis_asi.bin`), Arm `themis_asi_compiler.rs`. Bildintensität (dimensionslos) → derselbe Kontrakt-Riss wie Keogramm; braucht euer `field`/`quantity`-Verdikt **und** `terms`.
- **BepiColombo Plasma** — netloc `zenodo.org` (Record 17813314), Asset `BCPL` (`bepicolombo_plasma.bin`), Arm `bepicolombo_plasma_compiler.rs`, 5 Kanäle / dtype 2 Hz · 40 km. `terms` CC-BY-4.0 (eure `blocked_sources.φ:28`-Note) — braucht `field`/`quantity`.
- **EBHIS HI 21 cm** — netloc `cdsarc.cds.unistra.fr`, Asset `EBH1` (`ebhis_hpx190.bin`), Arm `ebhis_compiler.rs`, 945 Kanäle K. Braucht `terms` + `field`/`quantity`.
- **ACT DR6.02** — netloc `lambda.gsfc.nasa.gov`, Asset `cmb_act_f150_n64.json`, Arm `cmb_act_compiler.rs`, Form `{ra,dec,z,T}` (uK→K, wie `cmb_planck_smica_n64`). Braucht `terms` + Feld-Bestätigung.
- **Blinkverse FRB** — netloc `blinkverse.zero2x.org`, Asset `BVFR`, Arm `blinkverse_compiler.rs`, CSV-Katalog (4020 Quellen/35342 Pulse/154 Hosts) → `map table.rows` + Felder. Braucht `terms` + `field`-Zeilen.
- **CERN ROOT** — `cern_root_compiler.rs` ist `--probe`-only (TFile-Header + TKey-Liste, kein `.bin`-Asset); nicht manifestierbar, bis der TTree-Decode einen Arm liefert.

Zusätzlich: **Keogramm** (Form-Verdikt aus mountain-289 gefaltet) — die `sources.φ`-`quantity`-Zeile bleibt unter `units.rs:507` (keine dimensionslose Einheit) + `parse.rs:94-99` (kein `format keogram`-Arm) nicht schreibbar.

## An river

Origin: mycelium-285 (2026-10-09) — der `register`-Job ist am `path_reference_scan` rot; der Befund liegt in einem river-Survey:

- `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md:7` — see-also zeigt auf ein archiviertes river-Handover (river-folge139; real unter `docs/handover/archiv/`).
- Dieselbe Datei `:12`/`:13` — Host-absolute Pfade (`/home/…/archive-root/omegaflow-legacy`, `/home/…/knowledge/…/nebra`); `path_reference_scan.rs:135-160` verbietet `/home/` auch in Backticks.

Bitte die Zeilen heilen (see-also → `archiv/…`; Host-Pfade symbolisch) — dann ist der repo-weite `register`-Job grün. (Die register-eigenen Schritte sind clean.)

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. `wartend.φ:8` → `superdarn-globus-map`; Transfer-Task `0f2819ca…` FAILED `EXPIRED`. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
