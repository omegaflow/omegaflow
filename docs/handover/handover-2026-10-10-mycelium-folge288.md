<!--
  title: Handover — Mycelium-Folge 288 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. de441/de442 `sha256` aus dem GitHub-Release-Digest registriert (erste Manifest-Läufe, Wait `de441-de442-sha` geschlossen); future-211-Block (sources-refresh) gefaltet — Workflow war bereits in mycelium-282 verdrahtet (`ba59976da`/`b46c9e514`, Rust-Bin `d72710803`), kein offener Punkt. Register kanonisch (2705 Blöcke), license_census/cdn_reconcile clean. Nachtrag nach Operator-Wort „fixen": die CI-Roh-API-Lehre (`conclusion=` unwirksam → `status=failure`, `docs/concepts/tools-map.md`) und der zuvor übersehene rote `keogram-cdn`-Lauf (`ABK`-Quelle steht seit 2026-04-21) sind gemessen + eingetragen. Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-10
  sha256: 1a520dd9fff2951f727063ccc356fadaa6748a65fa43f16266202747688b2d2c
  status: live
-->
# Handover — Mycelium-Folge 288 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge287.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`.

**CI-Messlehre (2026-10-10, Randbefund-Fix):** der GitHub-Roh-API-Filter `conclusion=` auf `…/actions/runs` ist unwirksam (wird still ignoriert; `?status=completed&conclusion=failure` lieferte `cancelled`-Läufe und zählte fälschlich 0 rote) — korrekt ist `status=failure`/`status=cancelled`/`status=success` (oder client-seitig `jaq 'select(.conclusion=="failure")'`); kanonischer Leser bleibt `ci_manage`. Eingetragen in `docs/concepts/tools-map.md` (CI-Roh-API). Der zuvor übersehene rote Lauf `keogram-cdn 37995952959` ist damit gemessen (siehe Keogramm-Punkt).

## Burn: open 0.0000 · close 0.0504 · cap 0.5 — Grund: de441/de442 `sha256` registriert (6 Blöcke), future-211-Block gefaltet (sources-refresh, bereits verdrahtet → kein offener Punkt), Register kanonisch 2705 Blöcke, license_census/cdn_reconcile clean · deepseek-flash, kein pro/max (gemessen `session_burn` @Schluss; Fenster 9 Sessions total $0.7817, Session `Mycelium-Linie starten: Stehender Pass`).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „bauen vor registrieren" | 2026-10-10 | Operator (Session, Mycelium 288) — Name/Atom der Session |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 288) |
| „Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-10 | Operator (Session, Mycelium 288) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-09-mycelium-folge287.md` §Operator-Wort-Register | 2026-10-09 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation — sechs Arme registriert; nur Blinkverse offen
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Archivar-Leser)
- **Trigger:** Blinkverse-Katalog-Pfad in `src/archivar/extract.rs` gebaut
- **Lage:** (gemessen 2026-10-10 via `ci_manage status`) HEAD `53c1aacd2`. Registriert: Keogramm, THEMIS ASI (`e04be201a`), EBHIS + BepiColombo (`05924d088`/`4bbb836dc`), ACT (`4570ee30e`), IRIS (`a637c5672`). Die sechs CDN-Läufe (Single-Runner-Stau): `37995952959` keogram **rot** (siehe Keogramm-Punkt) · `37998113249` themis-asi queue · `38000948001` ebhis queue · `38000952599` bepicolombo queue · `38001184677` act (`in_progress`) · `38001780912` iris queue. Offen: Blinkverse — Reader `src/archivar/blinkverse.rs` steht, aber der benannte Katalog-Pfad (Tabelle ohne `t`) fehlt in `src/archivar/extract.rs` (`sgrep -i blinkverse src/archivar` = reader+mod, kein dispatch).
- **Blockade:** der Blinkverse-Dispatch in `extract.rs` (Archivar = Mountain) fehlt; die Feld-/Anker-Zuordnung (ra/dec, cmap, DM-Spalte) ist Mountain/Rat-Register.
- **Braucht:** Blinkverse-Katalog-Pfad in `extract.rs` → dann `blinkverse-cdn.yml` + `sources.φ`-Block.

### Manifestation — Keogramm ABK: Quelle steht seit 2026-04-21 (Riss gemessen)
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Quellen-Verdikt)
- **Trigger:** Mountain-Verdikt zur ABK-Keogramm-Quelle (re-point SGO oder `declined`)
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 37995952959` + `general`-Taucher) `keogram-cdn` Lauf `37995952959 @9347c3fc4` **rot**: `fetch_bytes … ABK.2610/ABK_261008.jpg curl: (22) … 404`. Das FMI-MIRACLE-Archiv endet `ABK.2604` (April 2026; `2605`/`2606` existieren leer, kein `2607`–`2610`). SGO (`www.sgo.fi/pub_asc/emCCD_ABK/emCCD_ABK_{YYYY}/emCCD_ABK_{YYYYMM}/ABK_{YYMMDD}/ABK_{YYMMDD}.jpg`) trägt die Monats-/Tagesverzeichnisse 2026-08/09/10, aber **leer**; `iXon/ABK_latest.jpg` Last-Modified 2026-04-22. **Jüngste vorhandene ABK-Nacht: 2026-04-21** (`…/emCCD_ABK_202604/ABK_260421/ABK_260421.jpg`, HTTP 200). Die ABK-Quelle liefert seit 2026-04-21 keine Daten — der Vorgestern-Default ist dauerhaft 404.
- **Blockade:** ABK-Kamera liefert seit 2026-04-21 nichts (an FMI **und** SGO).
- **Braucht:** Mountain-Verdikt (SGO-Live gibt es nicht → `declined`/`pending`) → dann `keogram-cdn.yml`/`sources.φ` anpassen.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` (hips-png-cdn) Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view 37932098229`) **queued** (created 2026-10-09T12:45Z, updated 22:47Z, SHA `eaf1337fd`) — **Trigger nicht gefeuert**; `phi/pipeline/ledger.φ:110` `ausstehend`.
- **Blockade:** Laufdauer (Single-Runner/Ernte).
- **Braucht:** Abschluss (Watchdog cancelt past 2× Median) → bei success `ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo + Release-Body-Lizenz
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Tool-Format)
- **Trigger:** `sources_repo_license` emittiert pro-Quelle-Zeilen statt netloc-Rollup
- **Lage:** (gemessen 2026-10-09) terms-Format-Verdikt ist **pro Quelle** (mountain-289/290). Populations-Riss geheilt: `license_census` 2698 terms / 0 no-terms (2026-10-10: 2705/0), `sources_repo_license` 2698 terms / 0 no-terms. Offen: `sources_repo_license.rs:128-142` emittiert `<netloc> | <token> | <url>`; `LICENSE` wird nur nach `/tmp/licence` erzeugt, nicht ins Repo committed.
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
- **Trigger:** Mountain/Rat misst `field`/`quantity`/`unit` des IRIS-Rohpixels
- **Lage:** (gemessen 2026-10-09) HCR-Form `…/hek/hcr?cmd=search-events-corr&outputformat=jsonMedium&instrument=IRIS…` (200); `iris-cdn.yml` + `harvest.φ` `format iris` stehen; `iris_compiler.rs` druckt keinen `field`/`quantity`-Token (Rohpixel ohne Einheit, mountain-292f misst BUNIT `count`).
- **Blockade:** die physikalische Einheit des IRIS-Rohpixels — Mountain/Rat-Register.
- **Braucht:** Mountain/Rat `field`/`quantity`/`unit` für `format iris` → dann `sources.φ`-Block.

### LAIC CSSDC — Riss gemessen: cssdc stale, LEOS `blocked account`; DTU-CSES-MAG gemessen
- **Status:** wartend | **Bindung:** eigen (Register-Riss/Manifestation) · mountain (`terms`/`field`)
- **Trigger:** Mountain-Verdikt (`cssdc.ac.cn` → `declined`; LEOS → `blocked account`) + DTU-`terms`/`field`
- **Lage:** (gemessen 2026-10-09) `cssdc.ac.cn/en` = Telegram-APK-Advert-Seite → `blocked_sources.φ:66-68` ohne Wissenschaft; LEOS login+captcha (`LEOS_USER/PASS` existiert). **DTU-CSES-MAG-Kandidat gemessen** (`general`-Taucher, `archive_search` 2026-10-09): `https://ftp.space.dtu.dk/data/magnetic-satellites/CSES/` — CDF3-Tagesserie (~2 010 Dateien, HTTP 200, 8,9 MB anonym) + 1-s-`.dat` (91 MB, HTTP 200). Kein Lizenz-String sichtbar (DTU-Policy scopes Bodenstationen, nicht Satellit — `pending`); Produkt-Level L1/L1b `pending`.
- **Blockade:** Mountain-Verdikt (`cssdc`/LEOS) + DTU-`terms`/`field` `pending`.
- **Braucht:** Mountain `cssdc` → `declined` (stale), LEOS → `blocked account`, DTU-CSES-MAG-`terms`/`field` → dann `cses-mag-cdn.yml` + `harvest.φ`-Arm + `sources.φ`-Block.

### Redistributions-Alternativen — Survey + Mountain-Verdikt
- **Status:** wartend | **Bindung:** eigen (Recherche) · mountain (Re-Admission)
- **Trigger:** Mountain-Verdikt über die CDN-fähigen Alternativen (`docs/surveys/survey-2026-10-09-redistribution-alternativen.md`)
- **Lage:** (gemessen 2026-10-09, zwei `general`-Taucher) 41 `decline redistribution`-Blöcke durchsucht; NC ist kein Block (Projekt NC); GIRO DIDBase (CC-BY-NC-SA) + INTERMAGNET (CC-BY-NC) admissibel; nur GRDC + RIPE RIS blocken. 23/41 Messungen über zugelassene freie Quellen schon zurück; ~2–4 feld-fähig (GIRO, ds.iris.edu → EarthScope).
- **Blockade:** keine für die zertifizierten Kandidaten — Mountain-Re-Admission fehlt.
- **Braucht:** Mountain Re-Admission je Kandidat → dann CDN-Workflow/`sources.φ`-Block.

### Weberin-Gate — Verfeinerung (Rat gehalten 2026-10-09)
- **Status:** wartend | **Bindung:** eigen (Gate-Bau/CI) · mountain (Register/Parse) · river (Reihenfolge/Membran)
- **Trigger:** Operator-Wort „weberin-Gate verfeinern" (2026-10-09) — **gefeuert**, Rat-Struktur steht
- **Lage:** (gemessen 2026-10-10) kein Gate prüft die Weberin-Rolle. Rat-Struktur: `weberin <role>`-Direktive in `sources.φ` (Klassen `kette:{direction,body,station}` · `zeuge:{…}` · `kein-faden`), allein von Mountain geschrieben; `witnesses.φ::witness <kind>` bleibt; Gate als `commit_check`-Fixture + Test im selben Atom; Mess-Tool `weberin_fit` (Achsen pos·series·qty, setzt kein Verdikt); Reihenfolge als erster Schritt in `docs/SOURCE_PORT.md`. Am Baum: `weberin-verdicts-cdn.yml` + (fremd, untracked) `src/archivar/weberin_fit.rs`; `sources.φ:19678-19683` trägt `format weberin_verdicts`; keine `weberin`-Direktive (`sgrep weberin phi/sources.φ` = 0).
- **Blockade:** Register-Direktive + `parse.rs`-Arm bei Mountain; Rat-Riss 2 (nur-neu-Gate vs Backfill) offen.
- **Braucht:** (a) Mountain `weberin`-Direktive + `parse.rs`-Arm; (b) Mycelium `commit_check`-Fixture + Test (nach Mountains Arm); (c) River Reihenfolge in `SOURCE_PORT.md`; (d) Backfill der alten Masse als eigener Punkt.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
