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
- **Range-/Subset-Leser**: **keiner** für `format fits` verdrahtet. `src/archivar/extract.rs:5329` liest bei `format fits` die **ganze lokale Datei** (`fits_to_json`), kein `fetch_range`; der einzige FITS-Range-Nutzer ist ein Header-Fenster-Probe (`cmb_act_compiler.rs:503`), nicht das Archiv-Format.

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
- **`sha256` im Block + 206**: beides hängt am Upload. Sobald `OMEGAFLOW_TOKEN` `omegaflow/sources` schreiben darf, spiegelt der idempotente Job das Asset; dann steht der Whole-sha256 im `.parts`-Manifest und kann in den Block, und `.parts`/Shards liefern 206.
- **Eigene `.1~ed07ad98`** (ICGEM Grad-Cap, Rat `R-2026-10-18-f` offen) und **`.3~7a1c533b`** (Offen-Zählung/Kappe, Gate-Code hinter `edit:deny`) bleiben Rat-/Wand-blockiert.

## 7. Bilanz

Post 0 · Trigger 0 · Transfer: eingehend 1 unbeglichen (river `.13`, am Credential blockiert), eigene 2 offen.
**Geschlossen:** der Register-Formfehler + die Transport-Route (`15bcd736e`), und der Stream ist als byte-treu belegt (md5-Gate bestanden). **Nicht geschlossen:** der Token selbst — Ursache benannt (`OMEGAFLOW_TOKEN` 403 auf `omegaflow/sources`), Fix ist Operator-Arbeit.
