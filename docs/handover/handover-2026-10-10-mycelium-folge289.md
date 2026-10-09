<!--
  title: Handover — Mycelium-Folge 289 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. Blinkverse-CDN (Workflow + `sources.φ`-Block) gebaut; `sources-repo-licence`-LICENSE-Commit-Schritt geschrieben; INPE-BIG (`ledger.φ:86`) disponiert; die adressierten Blöcke future-211 + mountain-294 gefaltet. Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-10
  sha256: a051a8d88abffa1c7e751fbe1e996d8d53611d21122166f9cecbf5f598b09a72
  status: live
-->
# Handover — Mycelium-Folge 289 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge288.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`.

## Burn: open 0.0000 · close 0.0611 · cap 0.5 — Grund: Blinkverse-CDN (Workflow + `format blinkverse_frb`-Block) gebaut, `sources-repo-licence`-LICENSE-Schritt geschrieben, INPE-BIG disponiert, future-211/mountain-294 gefaltet · deepseek-flash, kein pro/max (gemessen `session_burn` @Schluss).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 289) |
| „bauen vor registrieren" | 2026-10-10 | Operator (Session, Mycelium 288) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge288.md` §Operator-Wort-Register | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation — Blinkverse-CDN gebaut, erster Lauf offen
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** erster `blinkverse-cdn`-Lauf (sha256/Asset)
- **Lage:** (gemessen 2026-10-10) mountain-294 `b385a7ea5` baute den Dispatch (`extract.rs:213/3513/3783`, `blinkverse.rs`). Geschrieben: `.github/workflows/blinkverse-cdn.yml` (fetch → `blinkverse_compiler --url …/type/FRB_SOURCE/download --ci-mode`); Register-Block `format blinkverse_frb` in `phi/sources.φ` (`url …/blinkverse.zero2x.org/blinkverse_frb.bin`, `origin …/type/FRB_SOURCE/download`, `terms unknown`, `at sun`, `ttl 604800`, Feldzeilen `DM`/`DM_ne2001`/`DM_ymw16` → `frb_blinkverse_dm_*_pccm3`). CSV-Header gemessen (`curl`, 517 KB): `Source,Telescope,Repeater,RA,RA_err,Dec,Dec_err,GL,GB,DM,DM_ne2001,DM_ymw16,Reference`; `RA`/`Dec` → Position, nur die drei DM-Spalten als Kanäle (RA_err/Dec_err Einheit ungemessen → kein Feld). `register_sort` kanonisch (2708).
- **Blockade:** keine (der Arm steht).
- **Braucht:** ersten `blinkverse-cdn`-Lauf abwarten → `sha256` in den Block.

### Manifestation — LICENSE im `omegaflow/sources`-Repo (Schritt gebaut, Lauf offen)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** `sources-repo-licence`-Lauf am neuen HEAD (Push auf `phi/sources.φ` feuert den Workflow)
- **Lage:** (gemessen 2026-10-10) `sources-repo-licence.yml` erzeugt `/tmp/licence/{LICENSE,README.md}`; neuer Schritt „Commit the aggregate LICENSE into omegaflow/sources" klont `omegaflow/sources`, kopiert die zwei Dateien, committet+pusht nur bei Änderung (`sources-refresh.yml`-Muster). `sources_repo_license` emittiert `<source-url> | <terms-token> | <terms-url>` (mountain-294 `b385a7ea5`); `license_census` 2708.
- **Blockade:** keine.
- **Braucht:** Lauf-Ergebnis (`ci_manage log <id>`) → `LICENSE` steht im `omegaflow/sources`-Repo.

### Manifestation — Keogramm ABK: Quelle steht seit 2026-04-21 (Riss gemessen)
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Quellen-Verdikt)
- **Trigger:** Mountain-Verdikt zur ABK-Keogramm-Quelle (re-point SGO oder `declined`)
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 37995952959` + `general`-Taucher) `keogram-cdn` Lauf `37995952959 @9347c3fc4` **rot**: `fetch_bytes … ABK.2610/ABK_261008.jpg curl: (22) … 404`. Das FMI-MIRACLE-Archiv endet `ABK.2604` (April 2026; `2605`/`2606` leer, kein `2607`–`2610`). SGO (`www.sgo.fi/pub_asc/emCCD_ABK/…`) trägt 2026-08/09/10, aber **leer**; `iXon/ABK_latest.jpg` Last-Modified 2026-04-22. **Jüngste vorhandene ABK-Nacht: 2026-04-21** (HTTP 200) — die ABK-Quelle liefert seit 2026-04-21 nichts, der Vorgestern-Default ist dauerhaft 404.
- **Blockade:** ABK-Kamera liefert seit 2026-04-21 nichts (an FMI **und** SGO).
- **Braucht:** Mountain-Verdikt (SGO-Live gibt es nicht → `declined`/`pending`) → dann `keogram-cdn.yml`/`sources.φ` anpassen.

### Manifestation — SSB-Feld-Asset: CI/CDN + Register-Zeile gebaut (river-160)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** erster `ssb-field-cdn`-Lauf (sha256 erst dann messbar)
- **Lage:** (gemessen 2026-10-10) `tools/measure/src/bin/ssb_field_bake.rs` (river-160) backt die Enclosure-Query am SSB (26×f64 LE, `--ci-mode`). Gebaut: `.github/workflows/ssb-field-cdn.yml` + Register-Block `format ssb_field` in `phi/sources.φ` (`url …/download/ssb/ssb_field.bin`, `terms CC-BY-NC-3.0-IGO`, `compiler …/ssb_field_bake.rs`, `at sun`, `ttl 604800`). `register_sort` kanonisch (2708).
- **Blockade:** sha256 + `terms`/`ttl`-Bestätigung erst nach dem ersten Lauf / durch Mountain.
- **Braucht:** erster Lauf (`ssb-field-cdn 38005007273` queued) → `sha256` in den Block; Mountain bestätigt `terms`/`ttl`.

### Manifestation — SPT-3G D1 `cmap`-Block geschrieben (Mountain-294 lieferte die Messung)
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (terms/Member)
- **Trigger:** erster `cmb-cdn`-SPT-Lauf (sha256 + Member-Prefix)
- **Lage:** (gemessen 2026-10-10) Mountain-294: SPT-3G D1 tar `…/SPT/spt_3g_d1/d1_midell_tqu_healpix/real_data_maps/full_maps_d1.tar.bz2` (HTTP 206, 7,87 GB), Member-Template `full_{095,150,220}ghz.fits`, tar-interner Prefix `pending`. Der Register-Block `cmb_spt_d1_n64.json` steht in `phi/sources.φ` (`url …/download/lambda.gsfc.nasa.gov/cmb_spt_d1_n64.json`, `origin` tar, `compiler tools/harvest/src/bin/cmb_planck_compiler.rs`, `cmap .`, `at sun`, `ttl 604800`, `field T cmb_spt_d1_T …`, `terms unknown` pending); `cmb-cdn.yml` trägt den SPT-Schritt (`--url <tar> --nside 64 --ci-mode`); Upload-Host im Compiler auf `lambda.gsfc.nasa.gov` korrigiert.
- **Blockade:** tar-interner Member-Prefix + SPT-Datenlizenz `pending` (Mountain).
- **Braucht:** Member-Prefix + `terms`-Verdikt (Mountain) → erster Lauf (`cmb-cdn 38005262361` queued) → `sha256` in den Block.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` (hips-png-cdn) Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view 37932098229`) **queued** (created 2026-10-09T12:45Z, updated 22:47Z, SHA `eaf1337fd`) — **Trigger nicht gefeuert**; `phi/pipeline/ledger.φ:110` `ausstehend`.
- **Blockade:** Laufdauer (Single-Runner/Ernte).
- **Braucht:** Abschluss (Watchdog cancelt past 2× Median) → bei success `ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

### PDS-PPI — Force descoped, `quantity` aus Label-Einheit
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (pro Spalte)
- **Trigger:** Mountain-Build-Schritt `quantity` je Spalte
- **Lage:** (gemessen 2026-10-10) Rat-Verdikt (mountain-294): `field`-Force für die generische PDS4-Familie **descoped**; `quantity` **allein aus der gemessenen Label-Einheit** pro Spalte; Spalten ohne Label-Einheit `pending`. `pds_ppi_compiler.rs` + `pds-ppi-cdn.yml` + `harvest.φ`-Arm stehen.
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
- **Lage:** (gemessen 2026-10-10, mountain-294) kein Gate prüft die Weberin-Rolle. Rat-Struktur: `weberin <role>`-Direktive in `sources.φ` (Klassen `kette:{direction,body,station}` · `zeuge:{…}` · `kein-faden`), allein von Mountain geschrieben; `witnesses.φ::witness <kind>` bleibt; Gate als `commit_check`-Fixture + Test im selben Atom; Mess-Tool `weberin_fit` (Achsen pos·series·qty, setzt kein Verdikt). Am Baum: `parse.rs:255` (Parse-Arm) + `types.rs:527` (`SourceConfig.weberin_role`) stehen; `sources.φ` trägt `format weberin_verdicts`; keine `weberin`-Direktive, weil **keine Stations-Serien-Quelle existiert** — Mountain schreibt die Direktive, sobald eine registriert ist.
- **Blockade:** die erste Stations-Serien-Quelle (für die Direktive); Rat-Riss 2 (nur-neu-Gate vs Backfill) offen.
- **Braucht:** (a) Mountain `weberin`-Direktive sobald eine Quelle steht; (b) Mycelium `commit_check`-Fixture + Test gegen das Gate-Prädikat (Arm steht); (c) River Reihenfolge in `SOURCE_PORT.md`; (d) Backfill der alten Masse als eigener Punkt.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
