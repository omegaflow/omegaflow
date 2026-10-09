<!--
  title: Handover — Mycelium-Folge 288 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. de441/de442 `sha256` aus dem GitHub-Release-Digest registriert (erste Manifest-Läufe, Wait `de441-de442-sha` geschlossen); future-211-Block (sources-refresh) gefaltet — Workflow war bereits in mycelium-282 verdrahtet (`ba59976da`/`b46c9e514`, Rust-Bin `d72710803`), kein offener Punkt. Register kanonisch (2705 Blöcke), license_census/cdn_reconcile clean. Nachtrag nach Operator-Wort „fixen": die CI-Roh-API-Lehre (`conclusion=` unwirksam → `status=failure`, `docs/concepts/tools-map.md`) und der zuvor übersehene rote `keogram-cdn`-Lauf (`ABK`-Quelle steht seit 2026-04-21) sind gemessen + eingetragen. `harvest-dispatch`-Rot geheilt: `phi/harvest.φ` Block-Ordnung (75/0, `harvest_reg --check`); `path_reference_scan .` clean (3427/0/0). Folgefaltung (river-160 + Mountain-294): SSB-Feld-Asset-CI/CDN + `sources.φ`-Block gebaut (`.github/workflows/ssb-field-cdn.yml`, `format ssb_field`); Blinkverse-Reader steht (Mountain `b385a7ea5`), Register offen; INPE-BIG (gedeckt) + USGS (kein Arm) geschlossen; LICENSE/PDS-PPI/Weberin fortgeschrieben. Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-10
  sha256: d114d125d3514b77e9a69b832cd77081162a614c813a1c177ef526ac3f0f21de
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

## Burn: open 0.0000 · close 0.1885 · cap 0.5 — Grund: de441/de442 `sha256` registriert (6 Blöcke), future-211-Block gefaltet (sources-refresh, bereits verdrahtet → kein offener Punkt), Register kanonisch 2706 Blöcke, license_census/cdn_reconcile clean, `harvest.φ` sortiert, CI-Roh-API-Lehre, SSB-Feld-Asset (Workflow + Register-Block) gebaut, river-160/Mountain-294 gefaltet · deepseek-flash, kein pro/max (gemessen `session_burn` @Schluss; Fenster 25 Sessions total $1.4627, Session `Mycelium-Linie starten: Stehender Pass`).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „bauen vor registrieren" | 2026-10-10 | Operator (Session, Mycelium 288) — Name/Atom der Session |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 288) |
| „Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-10 | Operator (Session, Mycelium 288) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-09-mycelium-folge287.md` §Operator-Wort-Register | 2026-10-09 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation — sechs Arme registriert; Blinkverse-Reader steht, Register offen
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Mountain-`b385a7ea5` (Blinkverse-Dispatch in `extract.rs`) — **gefeuert**
- **Lage:** (gemessen 2026-10-10) Registriert: Keogramm, THEMIS ASI (`e04be201a`), EBHIS + BepiColombo (`05924d088`/`4bbb836dc`), ACT (`4570ee30e`), IRIS (`a637c5672`). **Blinkverse-Dispatch gebaut** (mountain-294 `b385a7ea5`): `format blinkverse`/`blinkverse_frb` in `extract.rs` (`verify_records:213`, `extract_raw:3783`, Helfer `blinkverse_frb_channels:3513`); eine `field`-Zeile mit CSV-Spaltenname macht genau diese Spalte sichtbar, `RA`/`Dec` → Position (Einheitsvektor), `None`-Zelle ⇒ kein Kanal (nie 0.0). Der Block (`url`/`origin`/`compiler`/`format blinkverse_frb` + `field DM …`) + `blinkverse-cdn.yml` sind noch **nicht** geschrieben.
- **Blockade:** keine (der Mountain-Arm steht).
- **Braucht:** `blinkverse-cdn.yml` (Build/Upload des `blinkverse_compiler`-Bins) + `sources.φ`-Block (`format blinkverse_frb`, `field DM …`) → `register_sort`/`license_census`/`cdn_reconcile`.

### Manifestation — Keogramm ABK: Quelle steht seit 2026-04-21 (Riss gemessen)
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Quellen-Verdikt)
- **Trigger:** Mountain-Verdikt zur ABK-Keogramm-Quelle (re-point SGO oder `declined`)
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 37995952959` + `general`-Taucher) `keogram-cdn` Lauf `37995952959 @9347c3fc4` **rot**: `fetch_bytes … ABK.2610/ABK_261008.jpg curl: (22) … 404`. Das FMI-MIRACLE-Archiv endet `ABK.2604` (April 2026; `2605`/`2606` existieren leer, kein `2607`–`2610`). SGO (`www.sgo.fi/pub_asc/emCCD_ABK/emCCD_ABK_{YYYY}/emCCD_ABK_{YYYYMM}/ABK_{YYMMDD}/ABK_{YYMMDD}.jpg`) trägt die Monats-/Tagesverzeichnisse 2026-08/09/10, aber **leer**; `iXon/ABK_latest.jpg` Last-Modified 2026-04-22. **Jüngste vorhandene ABK-Nacht: 2026-04-21** (`…/emCCD_ABK_202604/ABK_260421/ABK_260421.jpg`, HTTP 200). Die ABK-Quelle liefert seit 2026-04-21 keine Daten — der Vorgestern-Default ist dauerhaft 404.
- **Blockade:** ABK-Kamera liefert seit 2026-04-21 nichts (an FMI **und** SGO).
- **Braucht:** Mountain-Verdikt (SGO-Live gibt es nicht → `declined`/`pending`) → dann `keogram-cdn.yml`/`sources.φ` anpassen.

### Manifestation — SSB-Feld-Asset: CI/CDN + Register-Zeile gebaut (river-160)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** erster `ssb-field-cdn`-Lauf (sha256 erst dann messbar)
- **Lage:** (gemessen 2026-10-10) `tools/measure/src/bin/ssb_field_bake.rs` (river-160) backt die Enclosure-Query am SSB (26×f64 LE, `--ci-mode`). Gebaut: `.github/workflows/ssb-field-cdn.yml` (holt `dr3_stars.bin` vom `ssd.jpl.nasa.gov-gaia`-CDN → `--ci-mode` → `gh release upload ssb out/ssb_field.bin`) + `phi/sources.φ:19664`-Block `format ssb_field` (`url …/download/ssb/ssb_field.bin`, `terms CC-BY-NC-3.0-IGO` vom `catalog_tycho` geerbt, `compiler …/ssb_field_bake.rs`, `at sun`, `ttl 604800`). `register_sort` kanonisch (2706), `license_census --fail`/`cdn_reconcile --fail` clean, `harvest_reg` 75/0.
- **Blockade:** sha256 + `terms`/`ttl`-Bestätigung erst nach dem ersten Lauf / durch Mountain.
- **Braucht:** erster Lauf → `sha256` in den Block; Mountain bestätigt `terms`/`ttl`.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` (hips-png-cdn) Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view 37932098229`) **queued** (created 2026-10-09T12:45Z, updated 22:47Z, SHA `eaf1337fd`) — **Trigger nicht gefeuert**; `phi/pipeline/ledger.φ:110` `ausstehend`.
- **Blockade:** Laufdauer (Single-Runner/Ernte).
- **Braucht:** Abschluss (Watchdog cancelt past 2× Median) → bei success `ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo + Release-Body-Lizenz
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** `sources_repo_license` pro-Quelle (mountain-294 `b385a7ea5`) — **gefeuert**
- **Lage:** (gemessen 2026-10-10) `sources_repo_license` emittiert jetzt `<source-url> | <terms-token> | <terms-url>` (eine Zeile je `url`-Block, stabil sortiert; mountain-294 `b385a7ea5`); `license_census` 2706/0. Offen: der `sources-repo-licence.yml`-Schritt, der `LICENSE` ins `omegaflow/sources`-Repo committed.
- **Blockade:** keine.
- **Braucht:** `sources-repo-licence.yml`-Schritt „commit `LICENSE` ins `omegaflow/sources`".

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ:86`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung)
- **Trigger:** `inpe_stac_compiler --collection <id>` (mountain-294 `4f85c7311`) — **gefeuert, gedeckt**
- **Lage:** (gemessen 2026-10-10) mountain-294: `data.inpe.br/big/` ist ein WordPress-**Portal** (HTTP 200, HTML, kein STAC); die Daten liegen am **INPE STAC Server** `https://data.inpe.br/bdc/stac/v1` (HTTP 200, **79 Collections**), und `inpe_stac_compiler` trägt bereits `--collection <id>` (default `samet_daily-1`, `4f85c7311`). `ledger.φ:86-88` ist damit durch den bestehenden Arm gedeckt — **kein `inpe_big_*`-Arm nötig**.
- **Blockade:** keine; `ledger.φ`-Disposition + der konkrete BDC-Collection-Block ist die offene Ernte-Verdrahtung.
- **Braucht:** BDC-Collection wählen → `ledger.φ:86` disponieren + `sources.φ`-Block (`format inpe_stac_samet_daily`-Familie).

### PDS-PPI — Force descoped, `quantity` aus Label-Einheit
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (pro Spalte)
- **Trigger:** Mountain-Build-Schritt `quantity` je Spalte
- **Lage:** (gemessen 2026-10-10) Rat-Verdikt: `field`-Force für die generische PDS4-Familie **descoped**; `quantity` **allein aus der gemessenen Label-Einheit** pro Spalte; Spalten ohne Label-Einheit `pending`. `pds_ppi_compiler.rs` + `pds-ppi-cdn.yml` + `harvest.φ`-Arm stehen.
- **Blockade:** Mountain-Build-Schritt (pro Spalte) — siehe Mountain-Offen.
- **Braucht:** Mountain `quantity` je Spalte → dann `sources.φ`-Block.

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

### Weberin-Gate — Fixture folgt dem Arm (Rat gehalten 2026-10-09)
- **Status:** wartend | **Bindung:** eigen (Gate-Bau/CI) · mountain (Register/Parse) · river (Reihenfolge/Membran)
- **Trigger:** Operator-Wort „weberin-Gate verfeinern" (2026-10-09) — **gefeuert**, Rat-Struktur steht; Mountain-Arm steht
- **Lage:** (gemessen 2026-10-10, mountain-294) kein Gate prüft die Weberin-Rolle. Rat-Struktur: `weberin <role>`-Direktive in `sources.φ` (Klassen `kette:{direction,body,station}` · `zeuge:{…}` · `kein-faden`), allein von Mountain geschrieben; `witnesses.φ::witness <kind>` bleibt; Gate als `commit_check`-Fixture + Test im selben Atom; Mess-Tool `weberin_fit` (Achsen pos·series·qty, setzt kein Verdikt). Am Baum: `parse.rs:255` (Parse-Arm) + `types.rs:527` (`SourceConfig.weberin_role`) stehen (mountain-294); `sources.φ:19678` trägt `format weberin_verdicts`; keine `weberin`-Direktive, weil **keine Stations-Serien-Quelle existiert** — Mountain schreibt die Direktive, sobald eine registriert ist.
- **Blockade:** die erste Stations-Serien-Quelle (für die Direktive); Rat-Riss 2 (nur-neu-Gate vs Backfill) offen.
- **Braucht:** (a) Mountain `weberin`-Direktive sobald eine Quelle steht; (b) Mycelium `commit_check`-Fixture + Test gegen das Gate-Prädikat (Arm steht); (c) River Reihenfolge in `SOURCE_PORT.md`; (d) Backfill der alten Masse als eigener Punkt.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
