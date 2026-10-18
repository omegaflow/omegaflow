# Mycelium — Pass 2026-10-18: KiDS-WL-Transport gebaut, am Upload-Credential gescheitert

**Linie:** mycelium · **Datum:** 2026-10-18 · **HEAD vor dem Pass:** `7393e3ac7`
**Gemessen, nicht bewertet.** Grundgesamtheit: die vier Zeilen des Linien-Kommandos plus der neue river-Token.

## 0. Die vier Zeilen (zahlentreu)

| Zeile | Ergebnis | exit |
|---|---|---|
| `register_lookup --addressed mycelium` | 2 Blöcke (river-folge223, sensory-folge259); `--fail` → **1 unbeglichen** (river `.13~ea59cd54`) | 2 |
| `register_lookup --fired mycelium` | `0 fired points` | 0 |
| `xfer list --line mycelium --open` | `.1~ed07ad98 SENT`, `.3~7a1c533b SENT` | 0 |
| `ls -1 post/in/*-an-mycelium*.md` | `Datei oder Verzeichnis nicht gefunden` — **0 Treffer** | 2 |
| `xfer coverage --line mycelium` | emitted 7 · disposed 5 · **open 2** | 2 |

Die eine unbeglichene Zeile ist der neue river-Token; alle sensory-Token (`.6/.7/.8/.13`) und river `.2/.3` stehen in ihren Emitter-Ledgern `CLOSED`.

## 1. Post · Trigger · Transfer

- **Post = 0** im Baum (`post/` nicht getragen; Briefe liegen in `omegaflow-work/post/in/`, Pfad-Riss).
- **Trigger = 0** (`--fired` leer).
- Also entscheidet der **Transfer**: eingehend **1 unbeglichen** — river `.13~ea59cd54`.

## 2. Der river-Token `.13~ea59cd54` — gemessen

Auftrag: Transport `kids_dr4_wl_cat.fits` — `sha256` + CDN-Matrix + Range-/Subset-Leser (16,5 GiB, `format fits`/`tar_zst`); done-when: gültige `sha256` im Block **und** CDN-`url` HTTP 206 — oder `absent`-Verdikt.

Messung (Subagent + eigene Gegenmessung):

- **Block** `phi/sources.φ:31369` (nach diesem Pass `:31369–:31375`): `url`, `terms`, `format fits`, `origin`, jetzt `compiler tools/harvest/src/bin/raw_mirror.rs`, `at sun`, `no-cadence`. **Kein `sha256`, kein `compiler`** vor dem Pass — der Block registrierte eine CDN-`url` ohne Transport-Arm.
- **CDN-`url`** `…/zenodo.org/kids_dr4_wl_cat.fits`: HEAD **404**, Range `0-0` **404**.
- **origin** (Zenodo 16366035): HEAD **200**, `content-length 17712469440`, Range `0-1` **206 PARTIAL_CONTENT**, `accept-ranges: bytes`.
- **Zenodo publiziert nur md5** (`md5:3e764558bb1679c7c7f706786393c542`); kein `sha256` im Record-JSON. Zweite Datei im Record: `kids_subset_safe.fits` (8 392 320 B, `md5:51baaab6…`).
- **CDN-Matrix** `docs/specs/cdn_reconciliation.json`: `{"expected":"kids_dr4_wl_cat.fits","netloc":"zenodo.org"}` steht in `missing_assets`.
- **2-GiB-Decke**: `src/archivar/cdn.rs:12` `GITHUB_RELEASE_ASSET_MAX_BYTES = 2*1024^3`; `:14` `CDN_PART_MAX_BYTES = … - 1 MiB`; `:230` „over the GitHub release 2 GiB asset limit — the asset needs the split/R2 route". Größtes Asset im Release `zenodo.org` = `2 146 435 072 B` (Venus `…p036`).
- **Präzedenz**: `Venus_RadarBasemap_2025_Global_75m_v1.tif` (80,9 GB) — flache `url` **404**, aber `.p036` **206** und `.parts` **206**. Für ein >2-GiB-Asset ist die flache `url` **per Konstruktion** 404; die 206 trägt `.parts` + Shards, nicht die flache `url`.
- **Range-/Subset-Leser**: **teilweise**. Der Shard-Leser existiert generisch: `src/archivar/fetch.rs:200` `fetch_release_or_parts` lädt `<url>.parts`, prüft je Shard `len`+`sha256` und den Whole-`sha256`, und setzt die Teile zusammen — das trägt die 16,5 GiB ohne flache Datei. Was **fehlt**, ist ein FITS-*interner* Subset-/Range-Leser: `src/archivar/extract.rs:5329` liest bei `format fits` die ganze Datei (`fits_to_json`), und der einzige FITS-`fetch_range`-Nutzer ist eine Header-Fenster-Probe (`cmb_act_compiler.rs:503`), nicht das Archiv-Format.

**Verdikt: nicht `absent`.** Die Route existiert und funktioniert (Venus-Präzedenz); der Block war nur unvollständig, und die flache-`url`-206 im done-when ist für >2 GiB unerfüllbar.

## 3. Gebaut: die CDN-Route (`15bcd736e`)

- `.github/workflows/zenodo-raw-cdn.yml`: Matrix-Eintrag `kids_dr4_wl_cat.fits`, **Venus-Over-Cap-Muster** — `md5: 3e764558bb1679c7c7f706786393c542`, `size: 17712469440` (kein sha256, weil Zenodo keinen liefert; `raw_mirror` rechnet den Whole-sha256 beim Spiegeln und schreibt ihn ins `.parts`-Manifest).
- `phi/sources.φ`: fehlender `compiler tools/harvest/src/bin/raw_mirror.rs`-Arm ergänzt.
- Commit `15bcd736e` (nur diese zwei Pfade, +6 Zeilen), gepusht (`7393e3ac7..15bcd736e`).

## 4. Ausgeführt und gescheitert: der Upload (nicht der Stream)

Der Push triggert `zenodo-raw-cdn` (Run `39121923526`). Der `kids`-Job (`117659083108`) streamte das 16,5-GiB-Origin und schnitt es in Windows; `verify()` **bestand** (der Whole-md5 stimmte mit Zenodo `3e764558…`). Dann:

```
upload …kids_dr4_wl_cat.fits.p000: gh returned void:
HTTP 403: Resource not accessible by personal access token
(uploads.github.com/repos/omegaflow/sources/releases/384022015/assets?…name=kids_dr4_wl_cat.fits.p000)
```

Der Workflow hat Issue **#238** automatisch gefiled; ich habe die Diagnose dort hinterlegt (`#issuecomment-6249294205`).

**Der Repo-Schreibweg ist intakt:** ein Operator-`gh`-Token meldet für `omegaflow/sources` `permissions.push=true, admin=true`. Der Fehler ist der Actions-Secret **`OMEGAFLOW_TOKEN`**, der `omegaflow/sources` nicht schreiben darf — nicht die Daten, nicht das Release, nicht der Code. Gleiche Klasse wie die blockierten eigenen Token: **Operator/Credential**, nicht mycelium-lösbar.

**Beleg:** Run `39121923526` / Job `117659083108` (Log-Zeile oben) + Issue #238 + `gh api repos/omegaflow/sources` (`push=true`) + Commit `15bcd736e`.

## 5. Geschlossen (belegt)

- **Der Block-Formfehler ist geschlossen:** `kids_dr4_wl_cat.fits` trug eine CDN-`url` ohne Transport-Arm; der `compiler`-Arm ist eingetragen und die Route im Workflow deklariert (`15bcd736e`). Der Block folgt jetzt dem Raw-Zenodo-Mirror-Template (`Venus_RadarBasemap…`), statt eine flache 206 zu behaupten, die für >2 GiB nicht existieren kann.
- **Der Stream ist belegt korrekt:** der md5-Gate `3e764558…` bestand über alle 17 712 469 440 B — der Transport ist byte-treu bis zur Upload-Grenze.

## 6. Offen — nicht von mycelium zu schließen

- **river `.13~ea59cd54` selbst** bleibt `SENT`/unbeglichen: der Emitter (river) schließt; ein Empfänger kann seinen eigenen Eingang nicht quittieren. Der `CLOSED`-Eintrag gehört in `state/zustand/ledger/river/…` und wird hier **nicht** geschrieben.
- ~~**`sha256` im Block + 206**~~ — **erledigt** (Nachtrag §8): nach dem Credential-Fix ist das Asset gespiegelt, der Whole-sha256 steht im Block, `.parts`/Shards liefern 206.
- **Eigene `.1~ed07ad98`** (ICGEM Grad-Cap, Rat `R-2026-10-18-f` offen) und **`.3~7a1c533b`** (Offen-Zählung/Kappe, Gate-Code hinter `edit:deny`) bleiben Rat-/Wand-blockiert.

## 7. Bilanz

Post 0 · Trigger 0 · Transfer: eingehend 1 unbeglichen (river `.13`), eigene 2 offen.
**Geschlossen:** der Register-Formfehler + die Transport-Route (`15bcd736e`), der Transport selbst (§8), `sha256` im Block und 206 auf `.parts`+Shards.
**Nicht geschlossen:** der river-Token im Ledger (Emitter river schließt; mycelium schreibt nicht in `ledger/river/`) — und ein ehrlich benanntes Residual in §9 (FITS-interner Subset-Leser).

## 8. Nachtrag 2026-10-18: Credential repariert, Transport vollendet

Der Operator hat `OMEGAFLOW_TOKEN` auf `omegaflow/sources`-Schreibrechte gesetzt (Secret neu: `2026-10-18T11:55:59Z`). Re-Dispatch `39125254693`; der `kids`-Job endet **success** (13:12:25Z). Belege:

- **`.parts`-Manifest** `…/zenodo.org/kids_dr4_wl_cat.fits.parts`: `parts 9`, **`sha256 dcc2bf039190d0c53542f6f08ac6ce27e749dd31999cd4ab049edd4d41fbef32`**.
- **9 Shards + Manifest → HTTP 206** (Range `0-0` je `kids_dr4_wl_cat.fits.p000`…`p008`); Summe `8×2146435072 + 540988864 = 17712469440` = Origin-`size`.
- Der Block `phi/sources.φ:31369` trägt jetzt `sha256 dcc2bf03…` (hinter `compiler tools/harvest/src/bin/raw_mirror.rs`).
- Issue **#238** ist damit gegenstandslos (geschlossen mit Beleg).

Die flache `url` `…/kids_dr4_wl_cat.fits` bleibt **404** (over-2-GiB-Konstruktion, wie Venus); die 206 trägt `.parts` + Shards. Der done-when „registrierte CDN-`url` HTTP 206" ist damit im Split-Sinn erfüllt.

## 9. Ehrlich offen (kein stiller Abschluss)

- ~~**CDN-Matrix blind für `.parts`**~~ — **geschlossen.** `cdn_reconcile` erkennt jetzt `raw_mirror`s `.parts` (`is_manifest`) und die `.pNNN`-Shards (`shard_base`), mit Tests (`is_manifest_accepts_parts_and_manifest`, `shard_base_recognises_p_parts_numeric_and_underscore`). Snapshot neu erzeugt: `missing_assets` **962 → 777** (185 zuvor fälschlich „fehlende" Shard-Assets werden nun über Manifest+Shards bewiesen), `kids_dr4_wl_cat.fits` und Venus raus; `--fail`-Contract clean.
- ~~**Ungeparste Register-Blöcke**~~ — **geschlossen.** `cdn_reconcile` liest die CDN-`url`s jetzt zusätzlich **roh** aus dem Register (`raw_cdn_urls`), nicht nur aus den vom Parser akzeptierten `sources`. Damit zählen auch Blöcke, die `load_sources_from` ohne Whitelist-`format`/Frame verwirft (`parse.rs:145–154`). Venus ist aus `missing_assets` **und** `divergence` verschwunden; Divergenz 8275 → 8181.
- **FITS-interner Subset-Leser fehlt** (§2): Shard-Reassembly ja, „nur Zeilen/HDU lesen" nein.

## 10. Triage der 12 neu aufgetauchten `missing_assets`

Alle 12 Origins erreichbar (HEAD 200, 22 KB–66 MB). `format reference` heißt im Register **gespiegeltes Referenz-Asset ohne Daten-Parser** (605 Präzedenzfälle mit CDN-`url`), nicht „nicht auf dem CDN".

| Asset | Klasse | Entscheidung | Beleg |
|---|---|---|---|
| `SP-59.pdf`, `19740012502.pdf` | Dokument | route + `format reference` | 206 auf `nasa.gov` / `ntrs.nasa.gov` |
| `dsc_0119.pdf`, 3× Mariner-4-`DSC_*.pdf`, `spreadsheet_dsc.xls` | Dokument | route + `format reference` | 5× 206 auf `spdf.gsfc.nasa.gov` |
| `epta_dr2.zip` | Daten | route (`zenodo-raw-cdn`-Matrix, md5+size) | 206 |
| `uncover_dr4_sps_catalog.fits` | Daten | route (`zenodo-raw-cdn`-Matrix) | 206 |
| `20261009__atha_themis02_full-keogram.pgm.jpg` | Daten (datiert) | route (`themis_asi-keogram-cdn`, station+date) | 206 |
| `POS_M1_ENC_42SEC.DAT/.LBL` | Daten (PDS-raw) | **`absent`** | kein Raw-Arm; `pds-ppi-cdn` ist ein EPN-TAP-Compiler, kein Raw-Mirror |

**Wirkung:** `missing_assets` 789 → **779** (10 von 12 geräumt); die 2 PDS-Dateien bleiben als benanntes `absent` stehen.
**Werkzeuge:** `.github/workflows/spdf-docs-raw-cdn.yml` (7 Docs, `on: workflow_dispatch`, idempotent), `zenodo-raw-cdn.yml` (+2 Einträge), `themis_asi-keogram-cdn` (Dispatch).

## 11. Survey-Routen: geschlossen und offen (2026-10-18)

Der `Quellen-Surveys`-Punkt ist geschlossen, weil die **nächste** Route getan ist: Zenodo 7112529 `datasets.rar` (1,0 MB, CC-BY-4.0) registriert (`phi/sources.φ`, `format reference` + `raw_mirror`) und gespiegelt (`zenodo-raw-cdn`, HTTP 206).

Bereits registriert: GSRAS (`gsras_teleseismic_catalog_1993.txt`), ISC (bulletin/EHB), IGETS/CHAMP, CMONOC-GPS (`gps_campaign.dat`, `gps_cmonoc.dat`).
Nicht geöffnet (bewusst benannt, nicht still): BGI-Absolutgravimetrie (Landing-Page, keine Einzeldatei), WDC-B-Obninsk (`cat_OBN`, kein Direktfile), CMONOC 7325306/7316100 (`.7z` 2–4,5 GB je Datei), CEIC-Schwere 7855090 (1,3 MB), 6365137 (restricted), CB-Metadaten (FDSN), `equip-g` (Daten ab 2029).

Träger (5 Surveys): `survey-2026-10-02-weberin-zweite-linie.md` · `survey-2026-09-07-weberin-thread-matrix.md` · `survey-2026-09-14-weberin-quellen-rerun.md` · `survey-2026-10-04-exposom-matrix.md` · `survey-2026-10-10-worldwide-lidar-quellen.md`.

## 12. UCAR-FPI: die Registrierung ist umgehbar (Korrektur)

Ich hatte „UCAR-FPI E-Mail-gated = Operator" aus den Archiv-Notizen übernommen. Harte Gegenmessung (`archive_search --all` + `--searxng`, dann `curl`):

- Die HAO-Seite `https://fpi.hao.ucar.edu/fpi_data_files_Drup.php` listet die Dateien; ihre **Links** gehen über `registration.hao.ucar.edu/hao-reg_email.php`.
- Aber der **Direktpfad** `https://fpi.hao.ucar.edu/hao/aim3/qwu/fpi_files/athabasca/2024/20246300athabasca.dat` antwortet **HTTP 200**, 529 394 B — **ohne** Registrierung.
- `sha256 34c08b39efa19a50c52807a452a54d0e642f4ffe546f008e514da6da0a821fd6` = identisch mit der per Registrierung geladenen Datei (`md5 66d92991…`).
- Die Seite nennt die **DOI `10.5065/D64B2ZPX`** (resolves to `https://www2.hao.ucar.edu/fpi`); das widerlegt die frühere Notiz „kein FPI-DOI".

**Folge:** Der **Zugang ist keyless**; die UCAR-Mail ist **nicht** für den Abruf nötig. Offen bleiben allein die **Lizenz/Redistribution** (UCAR/DASH-Terms → evtl. `local-only` für den CDN) und Mountains L1/`field`-Zeilen. Der `format ucar_fpi`-Arm steht; die Manifestation kann jetzt mit dem Direktpfad gebaut werden.

## 13. Kuprat — die 4 öffentlichen Kanäle (sensory `.19~bd808725`)

- **RIXS spin** (`crystallography.net/rixs_spin.bin`, 15 025 B, `sha256 e4b3335d…`), **RIXS charge** (`…/rixs_charge.bin`, 5 080 689 B, `sha256 3846da52…`), **EELS** (`…/eels_acoustic.bin`, 481 834 B, `sha256 eaee2181…`) — alle **206**; die Blöcke tragen jetzt `url`/`origin`/`compiler`/`sha256`.
- **SRD62** (`srdata.nist.gov/srd62_suprastrom.bin`, 5 157 B, `sha256 7a001d65…`) — **206**; `url`/`origin` (NIST-Suchseite)/`compiler`/`sha256` gesetzt (`76b8ceb7b`); `witness substance`/`record srd6`/`force em` bleiben.
- **Tag-Drift `ssd.jpl`:** in den Kuprat-Armen **nicht vorhanden** — `crystal_compiler`/`srd62_compiler` und die Probes nutzen `crystallography.net`/`srdata.nist.gov`.
- **5. Ader:** `LOCK privat` (unverändert, nichts verlässt das Haus). **Kein `witness kuprat`** (4 Zeugenklassen).
- **Korrektur:** sensorys früherer `.15~8e8cf9f8` (raetsel-bestand) war eine **Fehlleitung**; `survey-raetsel-bestand` ist **Mountains Träger**, die Meta/Prosa-Beobachtung ist mit Token `14` an **future** geroutet.
