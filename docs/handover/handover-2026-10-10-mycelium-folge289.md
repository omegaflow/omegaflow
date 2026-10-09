<!--
  title: Handover — Mycelium-Folge 289 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. Reibungsschnitt: Blinkverse-CDN gebaut, `sources-repo-licence`-LICENSE-Schritt geschrieben, INPE-BIG (`ledger.φ:86`) disponiert; die mountain-gebundenen Punkte aus `Offen — eigen` entfernt (`## An mountain`) — Mycelium wartet nicht mehr auf fremde Akte. Stehender Pass am HEAD.
  class: handover
  date: 2026-10-10
  sha256: 618b6dda566b3d10734000deebf1af920acd4e40355162a6e703c76290637290
  status: live
-->
# Handover — Mycelium-Folge 289 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge288.md` (→ `archiv/`).

**Reibungsschnitt (Operator-Wort 2026-10-10 „beseitige die Reibung").** `Offen —
eigen` trägt nur Punkte, deren *nächster Schritt* Myceliums Akt ist. Ein Punkt,
dessen nächster Schritt ein fremder Akt ist (Mountain-Verdikt, Mountain-Parser,
Rat-Frage), ist **kein** Mycelium-Punkt: er liegt als kopierbarer Schritt in
`## An mountain` (bzw. wird vom Register-Owner-Tag getragen) — Mycelium hält ihn
nicht als Warten. Ein bereits dispatchter CI-Lauf ist ein *Flugzustand*, kein
Punkt; die Punkte unten sind die Läufe selbst, deren Abschluss Mycelium prüft.

## Burn: open 0.0000 · close 0.1016 · cap 0.5 — Grund: Blinkverse-CDN (Workflow + `format blinkverse_frb`-Block) gebaut, `sources-repo-licence`-LICENSE-Schritt geschrieben, INPE-BIG disponiert, foreign-bound Punkte aus der eigenen Liste in `## An mountain` verlagert · deepseek-flash, kein pro/max (gemessen `session_burn` @Schluss).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „ich kappiere es nicht warum seit ihr so krass voneinander abhängig, das ist krasse reibung — beseitige die reibung" | 2026-10-10 | Operator (Session, Mycelium 289) |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 289) |
| „bauen vor registrieren" | 2026-10-10 | Operator (Session, Mycelium 288) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge288.md` §Operator-Wort-Register | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation — Blinkverse-CDN gebaut, erster Lauf offen
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** erster `blinkverse-cdn`-Lauf `38005813723` (queued) — sha256/Asset
- **Lage:** (gemessen 2026-10-10) mountain-294 `b385a7ea5` baute den Dispatch (`extract.rs:213/3513/3783`, `blinkverse.rs`). Gebaut: `.github/workflows/blinkverse-cdn.yml`; Register-Block `format blinkverse_frb` in `phi/sources.φ` (`url …/blinkverse.zero2x.org/blinkverse_frb.bin`, `origin …/type/FRB_SOURCE/download`, `terms unknown`, `at sun`, `ttl 604800`, Feldzeilen `DM`/`DM_ne2001`/`DM_ymw16` → `frb_blinkverse_dm_*_pccm3`). CSV-Header gemessen (`curl`, 517 KB): `Source,Telescope,Repeater,RA,RA_err,Dec,Dec_err,GL,GB,DM,DM_ne2001,DM_ymw16,Reference`. `register_sort` kanonisch (2708).
- **Blockade:** keine (der Arm steht).
- **Braucht:** Lauf-Ergebnis (`ci_manage log 38005813723`) → `sha256` in den Block.

### Manifestation — LICENSE im `omegaflow/sources`-Repo (Schritt gebaut, Lauf offen)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** `sources-repo-licence`-Lauf `38005816019` (pending, dispatcht)
- **Lage:** (gemessen 2026-10-10) `sources-repo-licence.yml` erzeugt `/tmp/licence/{LICENSE,README.md}`; neuer Schritt „Commit the aggregate LICENSE into omegaflow/sources" klont `omegaflow/sources`, kopiert die zwei Dateien, committet+pusht nur bei Änderung. `sources_repo_license` emittiert `<source-url> | <terms-token> | <terms-url>` (mountain-294 `b385a7ea5`).
- **Blockade:** keine.
- **Braucht:** Lauf-Ergebnis (`ci_manage log 38005816019`) → `LICENSE` steht im `omegaflow/sources`-Repo.

### Manifestation — SSB-Feld-Asset (river-160): CI/CDN + Register-Zeile gebaut
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** erster `ssb-field-cdn`-Lauf `38005007273` (queued)
- **Lage:** (gemessen 2026-10-10) `tools/measure/src/bin/ssb_field_bake.rs` (river-160) backt die Enclosure-Query am SSB (26×f64 LE, `--ci-mode`). Gebaut: `.github/workflows/ssb-field-cdn.yml` + Register-Block `format ssb_field` in `phi/sources.φ` (`url …/download/ssb/ssb_field.bin`, `terms CC-BY-NC-3.0-IGO`, `compiler …/ssb_field_bake.rs`, `at sun`, `ttl 604800`).
- **Blockade:** sha256 erst nach dem Lauf.
- **Braucht:** Lauf-Ergebnis → `sha256` in den Block (die `terms`-Herkunft CC-BY-NC-3.0-IGO stammt vom geerbten `catalog_tycho`; Abweichung = Riss).

### Manifestation — SPT-3G D1 `cmap`-Block geschrieben (Mountain-294 lieferte die Messung)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** erster `cmb-cdn`-SPT-Lauf `38005262361` (queued)
- **Lage:** (gemessen 2026-10-10) Mountain-294: SPT-3G D1 tar `…/SPT/spt_3g_d1/d1_midell_tqu_healpix/real_data_maps/full_maps_d1.tar.bz2` (HTTP 206, 7,87 GB), Member-Template `full_{095,150,220}ghz.fits`, tar-interner Prefix `pending`. Register-Block `cmb_spt_d1_n64.json` in `phi/sources.φ` (`url …/download/lambda.gsfc.nasa.gov/cmb_spt_d1_n64.json`, `origin` tar, `compiler …/cmb_planck_compiler.rs`, `cmap .`, `at sun`, `ttl 604800`, `field T cmb_spt_d1_T …`, `terms unknown`); `cmb-cdn.yml` trägt den SPT-Schritt; Upload-Host im Compiler auf `lambda.gsfc.nasa.gov` korrigiert.
- **Blockade:** tar-interner Member-Prefix `pending` (kann den Lauf scheitern lassen).
- **Braucht:** Lauf-Ergebnis; Member-Prefix als kopierbarer Schritt in `## An mountain`.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` (hips-png-cdn) Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view 37932098229`) **queued** — **Trigger nicht gefeuert**; `phi/pipeline/ledger.φ:110` `ausstehend`. Zwei `hips-png-cdn`-Läufe in_progress (`37991944245` seit 21:11Z, `37959785850` seit 16:31Z).
- **Blockade:** Laufdauer auf dem einzigen Heavy-Runner (`t420`, busy seit 16:31Z).
- **Braucht:** Abschluss → bei success `ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

## An mountain

Origin: mycelium-289 (2026-10-10) — Reibungsschnitt (Operator-Wort). Diese Punkte haben als *nächsten Schritt* einen Mountain-Akt; die Messung steht, der Schritt ist kopierbar. Sie liegen nicht mehr in Myceliums `Offen — eigen` (Mycelium wartet nicht); Mountain faltet sie per `register_lookup --addressed mountain`.

- **Keogramm ABK** — `keogram-cdn 37995952959` rot (attempt 2): `ABK.2610/ABK_261008.jpg` 404. FMI endet `ABK.2604` (April 2026), SGO `www.sgo.fi/pub_asc/emCCD_ABK/emCCD_ABK_2026{08,09,10}` leer, jüngste Nacht **2026-04-21**. Schritt: Disposition in `declined_sources.φ`/`dead_sources.φ` (`www.sgo.fi/pub_asc/emCCD_ABK/` → `declined`; `space.fmi.fi` ABK → `pending`); danach `keogram-cdn.yml`/`sources.φ` anpassen (Mycelium, folgt dem Verdikt).
- **PDS-PPI `quantity`** — Rat-Verdikt: `field`-Force für die generische PDS4-Familie descoped; `quantity` allein aus der gemessenen Label-Einheit je Spalte, ohne Einheit `pending`. Schritt: `pds_ppi_compiler.rs`-Build-Schritt `quantity` je Spalte (ein Hunk liegt bereits im Baum).
- **IRIS `field`/`quantity`/`unit`** — `iris_compiler.rs` druckt keinen Feld-Token (Rohpixel ohne Einheit; BUNIT `count` gemessen, mountain-292f). Schritt: Mountain/Rat-Verdikt des Rohpixel-Felds → dann `sources.φ`-Block (Mycelium).
- **cssdc/LEOS/DTU-CSES-MAG** — `cssdc.ac.cn/en` = Telegram-Advert (kein Sci); LEOS = login+captcha (`LEOS_USER/PASS` existiert); DTU-CSES-MAG `https://ftp.space.dtu.dk/data/magnetic-satellites/CSES/` (HTTP 200, CDF3 + `.dat`) ohne Lizenz-String. Schritt: `cssdc.ac.cn` → `declined`; LEOS → `blocked account`; DTU `terms`/`field` → dann `cses-mag-cdn.yml` + `harvest.φ` + Block (Mycelium).
- **Redistributions-Alternativen** — `survey-2026-10-09-redistribution-alternativen.md`: GIRO DIDBase (CC-BY-NC-SA) + INTERMAGNET (CC-BY-NC) admissibel; nur GRDC + RIPE RIS blocken. Schritt: Re-Admission je Kandidat → dann CDN-Workflow/Block (Mycelium).
- **Weberin-Direktive** — Rat-Struktur steht; Parse-Arm `parse.rs:255` + `types.rs:527` stehen; keine `weberin`-Direktive, weil keine Stations-Serien-Quelle existiert. Schritt: erste Quelle registrieren → Direktive schreiben; Rat-Riss 2 (nur-neu-Gate vs Backfill) entscheiden. Myceliums `commit_check`-Fixture folgt nach dem Arm.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
- **CI-Reibung (gemessen):** ~45 Läufe queued; **ein** Heavy-Runner `[self-hosted, Linux]` (`t420`, busy seit 16:31Z), `demeter-residential` (online, idle) trägt nur den eigenen Label. Der Stau ist Runner-Kapazität, nicht Register-Warten — Operator-Entscheid (zweiter Heavy-Runner oder `demeter-residential` in den Pool).
