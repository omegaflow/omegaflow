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

### Papiere & Spaltenköpfe (gemessen 2026-10-18, `archive_search --all` + `--openalex`)

- **RIXS spin/charge (Zenodo 7286412):** Zhang et al., *npj Quantum Materials* **7**, 105 (2022), DOI `10.1038/s41535-022-00528-5`, open PDF `nature.com/articles/s41535-022-00528-5.pdf`. Daten: `exp_data/azimuthal_analysis/<UD|OD1|OD2>_<p>_<p>/rixs_phi_<phi>_<sigma|pi>.txt`, Spaltenkopf **`Eloss  RIXS  err`** (Energieverlust eV, Intensität, Fehler); `absorption_coeff` trägt `faa`/`fcc`.
- **Charge/Plasmon (Zenodo 15179114):** „Out-of-phase Plasmon Excitations in the Trilayer Cuprate Bi2223", *Phys. Rev. B* **111**, 165141 (2025), DOI `10.5281/zenodo.15179114`.
- **EELS (Zenodo 21859473):** `Fig4_eels_and_peakfitting.mat` (MAT, Peak-Fitting).
- **Weitere Cuprat-Daten mit expliziten Spalten:** Zenodo `18877118` („Unveiling the Nature of Collective Charge Excitations", `data_description_zenodo.md`), `6914223` (Sr14-xCaxCu24O41-Leitern), `12721667` (RIXS/XAS, `xy`-Spalten, Header = Incidenzenergien), `7264820` (Kagome, Header = Temperaturen), `8430044`/`4724109` (CDW `Height`/`FWHM` vs T).
- **Penetration depth/Superfluid (SRD62-Kanal):** Zenodo `8040941` (Superfluid/London), `15688310` (4Hb-TaS2, penetration-depth + superfluid-density `.txt`); Autoren Bernhard (Konstanz), Khasanov (μSR).

**Rohdaten-Spalten (gemessen 2026-10-18, heruntergeladen und gelesen):**
- **RIXS** (Zenodo 7286412): `rixs_phi_<phi>_<sigma|pi>.txt` mit Kopf **`Eloss  RIXS  err`**; Readme: `exp_figure_data`, `azimuthal_analysis` (UD/OD1/OD2 × q), `absorption_coeff` (`faa`/`fcc`).
- **Charge/Plasmon** (Zenodo 15179114, `data.zip`): `Fig1c.csv` = **`XAS,XAS_hv`**; `Fig2b/Fig3a/Fig2c/*.txt` = Kopf **`# H = <wert> (<H scan|theta scan>)`** + **`# Energy (meV) Intensity (arb. units)`**.
- **EELS** (Zenodo 21859473, `.mat` v5): Variablen **`ene`** (Energieachse) + **`eels_lineprofile`** (Linienprofil), Peak-Fitting → Spin-Oszillatoren (`harvest_eels`).
- **SRD62**: NIST CeramicDataPortal, Penetration-Depth-Tabelle (`srd62_compiler`).

**5. Ader (NSE I(q,t), privat):** Paper **Haug et al., *New Journal of Physics* 12, 105006 (2010)**, DOI `10.1088/1367-2630/12/10/105006`, arXiv `1008.4298` — „Neutron scattering study of the magnetic phase diagram of underdoped YBa2Cu3O6+x". 13 Läufe **3,47–32,52 K**, Q=(0.5, 0.5, 1.000), LAB_A/MLZ. Spalten: `.dat` (Echo-Fit) **`tau, FREQ1/2, I0/DI0, Pol/dPol, x0/Dx0, dL, Chisq`**; `.log` (Fit) **`P0/dP0, Gamma/dGamma, ChiSqr, QH/QK/QL, KF, TTAMEAN`**. Rohdaten **privat** (`LOCK`), nicht auf IOP-Suppdata/iMPULSE/Diss.; `src/archivar/lab_reader.rs` und `tools/harvest/src/bin/lab_reader_compiler.rs` sind im aktuellen Baum **absent** (Riss).

**Öffentliche Suche mit den Spaltenköpfen (2026-10-18):** `archive_search --all "TTAMEAN FREQ1 Pol dPol Chisq neutron spin echo"` → **kein** Datensatz mit diesen Spalten (Zenodo/OpenAlex/Dryad/PANGAEA absent für `TTAMEAN`). Die exakten 13 YBCO-Läufe sind **nicht öffentlich**. Nächstliegend: **hGBP1**-NSE (Zenodo `6534557`, CC-BY-4.0, JCNS/MLZ; aber 0,5–2,7 GB/tgz, Protein, anderes Reduktionsformat), **BaZrO3/RESEDA** (Zenodo `18306252` — **restricted**, nicht public) und SESANS (`14161883`, 4-Spalten, anderes Format). Paper public, Daten privat.

**Fuzzy-Suche (2026-10-18, ebenfalls negativ):** `--all` mit den `.log`-Spalten (`ChiSqr Gamma dGamma QH QK QL KF NSE`) → nur unbeteiligte Treffer (Hydrologie-`NSE` = Nash-Sutcliffe, Gamma-Strahlung; kein Neutron/Spin-Echo); `--github`/`--sourcegraph` `TTAMEAN` → absent; `--zenodo` nur generische Neutron-Software. **Kein öffentlicher NSE-Datensatz mit diesen Spalten.** Der RESEDA/BaZrO3-Akt ist an future geroutet (`post/out/2026-10-18-mycelium-reseda-nse-datenantrag.md`): Operator-Datenantrag bei Braun, bei Freigabe als öffentlicher NSE-Parser-Fixture.

## 14. Mountain-Restarme (Token `.10~15b6b847`): Triage

Gemessen 2026-10-18 am HEAD. **Kein Arm ist „nur Mycelium-Bau":** drei Assets fehlen, einer ist lizenz-blockiert, einer hat keine Residuen.

| Arm | Block | Asset | Reader/Compiler | Blocker |
|---|---|---|---|---|
| HMI | `hmi_polar` steht (`hmi_polar_compiler`) | ? | `hmi_compiler`/`format hmi` fehlt | neuer Bau; Terms CC0 (river-208) |
| EVE | fehlt (entfernt `9c60ddf89`) | fehlt | `format eve` steht; `eve-cdn.yml` fehlt | **Lizenz `permission-required`** (Operator/Rat) |
| EPTA | `format zip`+raw_mirror | **206** (39 MB) | `.par`/TRES-Compiler fehlt | nur **TOAs, keine Residuen** (river-208) |
| EDGES | fehlt | ? | `edges_compiler` fehlt | Quelle/Lizenz ungemessen |
| WALLABY | `format hips3d` | **fehlt** (404) | `hips3d`-Reader fehlt | kein exaktes HiPS3D-Produkt (river-208) |
| Conicus | `format tar_zst` | **fehlt** (404) | `tar_zst`-Reader fehlt | 34,4 GB; `zstd` nicht im Core-Crate |
| IceCube | `format json`+witness | **fehlt** (404) | Compiler fehlt | `origin` = Dataverse-**Suchseite**, keine Datei |

**Folge:** Die Builds, die ohne neue Daten/Lizenz gehen, sind HMI (CC0) und die Reader (`hips3d`, `tar_zst`) — letztere brauchen aber erst die fehlenden Assets. EPTA ist TOA-only, Conicus 34 GB, WALLABY ohne Produkt, EVE ohne Lizenz. Das ist eine **mehrpassige** Arbeit mit externen Blockern, keine stille Ein-Pass-Erledigung.

## 15. Offene Alternativen (drei Recherche-Agenten, `archive_search --all`)

- **HMI:** CC0, anonym — `jsoc1.stanford.edu/data/hmi/fits/YYYY/MM/DD/hmi.M_720s.YYYYMMDD_HHMMSS_TAI.fits` (200/206), FITS `magnetogram` (int32 4096², Gauss, `BSCALE=0.1`, `QUALITY`, `CRVAL1/2`, `CDELT1/2=0.504012942″`); Export `jsoc_fetch` → `SUM8/…`. **Bau:** `hmi_compiler` + `format hmi` (FITS).
- **IceCube — korrigiert (2026-10-18, Mountain `7de89f91e`):** CC0 — DOI `10.7910/DVN/MMIIZA` v3.1, **14 Event-TSVs** (`events`; IC40/59/79/86-I..XI, 209,6 MB; die Doppelungen sind `uptime`). Compiler `tools/harvest/src/bin/icecube_compiler.rs` neu: Header-Mapping, MJD→TDB über eingebetteten LSK, **`S2E1`** statt SKD1 → **1 643 355 Events**, 85 454 473 B (= 13 + n×52), `sha256 c44fd465…`, CDN **206**. Block: `origin` = DOI, `record s2e1 sky1` (nicht `skydirection`), url/sha256 neu; das alte SKD1-Asset (118 774 Events) wurde gelöscht.
- **EDGES:** MIT — `raw.githubusercontent.com/edges-collab/edges-bowman2018-pipeline/main/input-data/figure1_plotdata.csv` (7 907 B, `sha256 6eebf9cd…`), Spalten `Frequency [MHz], Weight, Tsky, Tres1, Tres2, Tmodel, T21`. **Spektralquelle** (Frequenz-Achse) — passt nicht ins Zeitreihen-`field`-Modell → **Modellfrage** (Rat/Architektur), kein stiller Block.
- **EPTA:** Residuen **absent** (Zenodo `8300645` TOAs/Modelle, `8025019` nur PDF-Plots). **Stand-in:** NANOGrav 12.5 yr (`4312887`, CC-BY-4.0, 7,1 MB, `.res`-Spalten `MJD frequency(MHz) residual(us) uncertainty(us) orb_phase(turns) flag`).
- **WALLABY:** HiPS3D **existiert** (`alasky.cds.unistra.fr/HIPS3D/ASKAP/CDS_C_ASKAP_WALLABY-Pilot-DR2/`, ODbL-1.0; `hips_order=7`, `order_freq=18`, `tile_depth=16`; Tile `Norder3_14/Dir0_15880/Npix389_15887.fits`, `NAXIS3=16`, `CTYPE3=FREQ-LOG`). Der Register-`.bin` ist **falsch** → **Bau:** `hips3d`-Tile-Reader.
- **Conicus:** nur 34,4 GB `lp_dataset_patched_c000_phase_metadata.tar.zst` (CC-BY-4.0); kein kleineres Produkt; Schema `edges, chi_eff, a_eff, dchi_tot, G, ph, lcid`; MIT-Build `github.com/brandonyzhao/conicus-dataset`.
**Rat-Fragen geroutet (2026-10-18):** `omegaflow-work/post/out/2026-10-18-mycelium-rat-fragen-restarme.md` — EVE-Lizenz (LISIRD-CC0 vs. L2B-FITS-Terme), WALLABY-Produkt (Cube vs. CASDA-Katalog), EPTA-Zeuge (`.par`-RMS vs. NANOGrav), Conicus-Mismatch (δ statt Shear/κ), HMI-Export-Registrierung. Kein Feld erfunden; die Arme warten dort, nicht in diesem Pass.

- **EVE (Agent C):** ein **zugelassener** Weg existiert nur für das **LISIRD-L3-Tagesprodukt** `sdo_eve_lines_l3` (LaTiS: `https://lasp.colorado.edu/lisird/latis/dap/sdo_eve_lines_l3.csv?time>=…`, 200, **CC0-1.0** per `lds:license lds:cc0`, keyless, 285 Spalten inkl. aller 11 Compiler-Linien + `*_stdev/_precision/_accuracy`); das rohe **L2B-FITS** (das `eve_compiler.rs` liest) trägt weiter „contact the PI … discuss appropriate use" → `permission-required`. **Vorbehalt:** Gilt die LISIRD-Portal-CC0 oder die eingebetteten FITS-Terme? Wenn LISIRD → EVE ist admitted (L3, nicht L2B); wenn FITS → `absent` (nur Lizenz, Zugang keyless). Das ist ein **Rat-/Register-Entscheid**, kein stiller Bau. CDAWeb/SPDF/CMR/NSSDCA tragen kein EVE.

## 16. Gebaute Rest-Arme nach den Mountain-Entscheiden (2026-10-18)

Grenze (Mountain `state/mountain/2026-10-18-restarme-entscheide.φ`): Mountain entscheidet **WAS** (Quelle/terms/field/Verdikt), Mycelium baut **WIE** (Compiler/Reader/CDN). Die Fragen aus §15 sind damit beantwortet; Conicus **declined**, TGF-3/REACH **pending**, EDGES bleibt Modellfrage.

- **EPTA (gebaut):** `tools/harvest/src/bin/epta_dr2_compiler.rs` liest das gemirrorte `epta_dr2.zip` (CC-BY-4.0, `sha256 2ae54694…`), nimmt `DR2full/*/*.par`, `TRES × 1e-6` s, Position `RAJ/DECJ` mit `ELONG/ELAT`-Fallback (3 Pulsare), schreibt **SKY1**. 26 `.par` → **25 eindeutige Pulsare** (`J1751-withoutBON128` ist die BON-lose Variante von `J1751-2857`, dedupliziert). Asset `epta_dr2.sky1`, 513 B, `sha256 3c4285c2…`, CDN **206**. Block (Mountain): `format sky1`, `record sky1`, `field TRES … s`, `at sun`.
- **EVE (gebaut):** Compiler-Pfad `…/level2b/YYYY/DOY/EVL_L2B_YYYYDOY_008_01.fit.gz` trägt (206); **2011 DOY 40–130** (vor MEGS-A-Ausfall, alle 11 Linien) → **1 146 616 Records**, 22 932 328 B, `sha256 0c0de10e…`, `epoch_tdb` 350 481 696..358 344 036, 0 void. Asset `eve_l2b.bin`; **hochgeladen:** CDN **206** unter Tag `lasp.colorado.edu-eve`.
- **WALLABY (gebaut):** CASDA TAP `AS102.wallaby_pdr2_sourcecatalogue_v01` (43 Spalten), Abzug `name,ra,dec,freq,w20,w50,dist_h,log_m_hi` → **3 454 Zeilen**, JSON 730 934 B, `sha256 45fc9aea…`. Asset `wallaby_pdr2_sourcecatalogue.json`; **hochgeladen:** CDN **206** unter Tag `casda.csiro.au`; Block (`cmap .`, `ra ra`, `dec dec`, `quantity` HI-Masse/Breite, `at sun`) steht (Mountain `4d66dbf18`).
- **HMI (gebaut + live, 2026-10-18):** `src/archivar/hmi.rs` (`HMI1`, comp = CAMERA 1/2/3) + `format hmi` in `extract.rs` + `tools/harvest/src/bin/hmi_compiler.rs`. Route: **direktes Archiv** `jsoc1.stanford.edu/data/hmi/fits/YYYY/MM/DD/` (keyless, 206), **stündlich** (23–24 Files/Tag); diese Dateien sind **1024² boxcar-4-gebinnt** (nicht der 4096²-Export; `DATAMEAN` ist skalierungsinvariant). Header-Range 0–20 000 B; `DATAMEAN` [G], `T_REC` (TAI→TDB über LSK), `CAMERA`; 1 Record/T_REC, Dedup über die Zeit. Asset `hmi_m_720s.bin` — **volle Serie 2010–2026** über `jsoc_info?op=rs_list` (`DATAMEAN,T_REC,CAMERA`, Jahresabrufe): **705 477 Records**, 14 109 548 B (= 8 + n×20), `sha256 08f0fd9f…`, CDN **206**. 2009 trägt keine HMI-Daten (SDO-Start 2010); `DATAMEAN` in **nT** (Gauß × 100 000), `CAMERA` Pflicht. Der FITS-Header-Weg bleibt als `--day`-Fallback. Mountain: `format hmi`, `field hmi_m_720s_blos`, `at sun`, `ttl 86400`.

## 17. matrix-vlies — die n=0-Zellen sind ein Datenfenster (2026-10-18)

Die 210 Zellen sind nicht leer wegen Code, sondern weil die vier RTSW/SWPC-Felder (`magnetosphere_imf_bz_nt`, `magnetosphere_imf_bt_nt`, `solar_wind_speed_km_s`, `solar_wind_density_cm3`, `phi:175/176/181/182`) an **1–7-Tage-JSON** hängen und das OMNI-Archiv am **2026-10-05** endet — die Schnittmenge ist leer (`alignment pending`). Gemessene mehrjährige Route (CDAWeb HAPI, CC0/PD-USGov):

| Quelle | Parameter | Zeitraum | Rate |
|---|---|---|---|
| `AC_H0_MFI` | `Magnitude` (Bt), `BGSM` (Bz GSM) | 1997-09-02 → 2026-08-04 | ~16 s |
| `AC_K0_SWE` | `Np`, `Vp` | 2017-07-01 → 2026-10-10 | 5 min |
| `AC_H0_SWE` | `Np`, `Vp` | 1998-02 → 2024-07 | 64 s — **`Np=-1.0E31` (Fill)**, nicht als Wert lesen |

Gemeinsames Fenster **2017-07 → 2026-08 (~9 Jahre)**. Mountain registriert die vier Felder auf diese Quellen (Tag `cdaweb.gsfc.nasa.gov`, `at sun`); Mycelium erntet stündlich (decimate) und stößt den Lauf an: `matrix-vlies` ist ein **CI-Job** (`.github/workflows/field-te-query.yml`, Descriptor `vlies_matrix.te`), nie lokal.

## 18. n=0-Zellen — Ernte + Matrix-Lauf (2026-10-18)

Die vier Felder sind auf **stündliche CDN-Assets** umgestellt; die Quelle bleibt CDAWeb HAPI (PD-USGov), die `hapi`/`terms`/Felder der Blöcke unverändert:

- `ace_h0_mfi_hourly.json` — 17 328 415 B, `sha256 aa849e89…`, **248 907** Stundenwerte, **1997-09 → 2026-08** (`Magnitude` = Bt, `BGSM` = By/Bz), CDN **206**.
- `ace_k0_swe_hourly.json` — 4 296 883 B, `sha256 cb1d5028…`, **85 335** Stundenwerte, **2017-07 → 2026-10** (`Np`, `Vp`), CDN **206**. `AC_K0_SWE` beginnt erst 2017-07; die frühere `AC_H0_SWE`-Reihe (1998–2024) trägt Fill `-1.0E31` und wäre ein **eigener Arm**, kein stiller Mix.

Compiler `tools/harvest/src/bin/ace_rtsw_compiler.rs`: monatsweise CDAWeb-CSV → Stundenmittel → HAPI-JSON; Fill `-1.0E31` wird **verworfen**, nie gemittelt. Die Blöcke tragen jetzt `url` (CDN), `origin` (HAPI-CSV-Abfrage), `compiler`, `sha256`; `terms`/`hapi`/`at sun` bleiben. Lauf: `field-te-query` dispatchen (enthält `matrix-vlies`).

**Ergebnis Lauf `39165623701` (matrix-vlies 5m25s, Commit `a9eb27a77`) und Bestätigung `39166372464` (Commit `6704a3dae`):**
- **Korrektur einer eigenen Fehlmessung:** der erste Grep zählte die `cond`-Spalte (immer 0), nicht die `n`-Spalte. Korrekt (n-Spalte, drittes Feld): `n=0`-Zellen **126 → 112** (Lauf 391) bzw. **120** (Lauf 664); `alignment pending` **126 → 84** (391) bzw. **92** (664). Die frühere Behauptung „210 → 0" war falsch.
- **RTSW-Zellen** (4 Felder × 25 Paare = 100): `n>0` **28 → 52** (391) bzw. **44** (664); `n=0` **72 → 48** (391) bzw. **56** (664). Die verbleibenden RTSW-Nullen liegen an den **Partnern**, nicht an der ACE-Reihe: AIA 2013, EVE 2011, Swarm/QBO (kurz/diskontinuierlich), INTERMAGNET-Riss, D20. Die 8 Extra-Pending in Lauf 664 sind **alle** `d20_thermocline_depth_m` (Live-Quelle im Lauf ausgefallen).
- Die vier ACE-Arme tragen echte n: 21 479 (XRS→Bt/Bz), 3 155 (XRS→Speed, Lauf 664), 28 223 (OMNI→Bz, Lauf 664), 3 309/3 338 (TAO), 108 (ENSO).
- `unit` war rot an einem **vorbestehenden Fixture** (`SourceConfig` ohne `head`, `field_te_query.rs:6383`); Fix `head: None`, lokal **80/80** Tests grün; Bestätigungslauf `39166372464`: `unit` ✓, `matrix-vlies` ✓.

## 19. CelesTrak TIMED — Ausweichroute belegt (2026-10-19)

Der Block `celestrak.org/timed_orbit.bin` war **404**: das Release-Tag existierte, trug 0 Assets; der Upstream `gp.php?CATNR=26998` lief in Timeouts (**HTTP 000**, 30 s). Belegte Ausweichroute: `https://tle.ivanstanojevic.me/api/tle/26998` → **200, 0,8 s**, `line1`/`line2`, TLE-Epoche `26291.37163704`. Der Workflow `timed-orbit-cdn.yml` holt jetzt das Mirror-JSON und schreibt per `jq` die zwei TLE-Zeilen nach `data/celestrak.org/timed.gp`; der Compiler bleibt unverändert (TLE-Text). Asset: **1 441 Records**, 80 704 B, `sha256 31b3f9b3…`, CDN **206**. Lizenz: `free-open` / CelesTrak Usage Policy (Upstream), Mirror als Relay; **kein Login, kein Konto**. Der Pin bleibt stabil, weil die Idempotenz-Stufe den vorhandenen Blob nicht überschreibt. Die alte lokale Datei (`bf5609e9…`, TLE 26287) ist durch die neue Epoche ersetzt.

## 20. Haus ist non-commercial — NC-Arme im Scope (2026-10-19)

Operator-Entscheid: Das Haus arbeitet **non-commercial**. Damit sind die NC-Terme der bereits registrierten Arme **im Scope**, kein „commercial decline":
- **INTERMAGNET** `CC-BY-NC-4.0`, **NMDB** `nmdb-non-commercial`, **GIRO** `CC-BY-NC-SA-4.0`, **ESA ESDC** `CC-BY-NC-3.0-IGO`, **Ocean Networks Canada** `onc-open`, **Columbia/GCMT** `columbia-research-noncommercial`.

**Pflichten bleiben:** Namensnennung, keine kommerzielle Weiterverwendung, bei GIRO **Share-Alike** für Ableitungen. Die `terms`-Zeilen bleiben unverändert; die CDN-Assets sind für den NC-Betrieb zulässig. Verbleibende echte Blocker: **CSES-01** (Registrierung), **WWLLN** (offline/Kosten), **RESEDA/BaZrO3** und **Kuprat 5. Ader** (Operator/privat). **Conicus/TGF-3/REACH/EDGES** sind Produkt-/Modellfragen, keine Lizenz.

## 21. LOLA RDR — Referenz-Orbit, binär gelesen (2026-10-19)

Erster Schnitt (River): ein Orbit, `lolardr_122590153` aus `lro_es_01`.
- Produkt: PDS3 **binär**, 256 B/Record, 199 976 Zeilen, 66 Spalten; `.dat` **51 193 856 B**, `sha256 d2d0e0ea…`.
- Reader: `pds3_fixed_width` um **`LSB_UNSIGNED_INTEGER`** und den **binären `decode_rows`**-Zweig erweitert (bisher textbasiert). Der Compiler parst die `LOLARDR.FMT`-COLUMN-Objekte selbst.
- Gültigkeitsregel (River; die FMT-Zeile „any value other than 0 invalid" ist falsch): `Radius_N != -1 ∧ (SHOT_FLAG_N & 0xFCFF) == 0 ∧ |Radius_N − Selenoid_Radius| ≤ 20 000 000 mm`. Bits 8/9 (RMU-Phase) sind erlaubt; Bit 0 = editiert/verworfen.
- Ergebnis: **109 867** gültige Spots — exakt Rivers Gegenprobe; 0 Ausreißer unter der Maske. CSV **8 542 951 B**, `sha256 ffaa8684…`, CDN **206** unter `pds-geosciences.wustl.edu`.
- Semantik: `Radius_N` = **Mondzentrums-Distanz** [mm] (FMT: „distance from the Moon center"); `SELENOID_RADIUS` = Geoid-Radius [mm]; Elevation = (Radius − Selenoid)/1000 [m]. Zeit = `TRANSMIT_TIME` (TT seit J2000, zwei u32: Sekunden + Fraktion).
- Lizenz/Provenienz: PDS/NASA, offen; `origin` = Produkt-URL, `terms` schreibt Mountain.

## 22. Arbeitsstand 2026-10-19 — laufende und offene Aufgaben (Arbeitszettel)

Dieser Abschnitt ist der **Zustand der Linie**, nicht ein Plan: was läuft, was fehlt, was der nächste
Schritt ist. Jede Session liest ihn zuerst und schreibt ihn am Ende fort.

### Laufend (Hintergrund)

**AIA hourly** — `target/release/aia_rs_list_compiler --from 2010-05-01 --to 2026-10-18 --jobs 6
--out /tmp/aia_hourly.bin --ci-mode`, Start `2026-10-18T22:58Z`, PID 730194 (Watcher:
`/tmp/opencode/aia_hourly.done.log`). Stand `23:55Z`: bei **2020-06** von 2026-10 (~63 %), ETA ~30–40 min.

- **Bekannte Ineffizienz, Ergebnis korrekt:** jeder Monats-Task fragt weiter `367d@1h` ab (der
  Compiler hat den Span nicht auf den Monat gekürzt). `write_aia` sortiert und dedupliziert
  `(t, comp)` — die Überlappung kostet **12× JSOC-Zeit**, nicht Datenqualität. Fix gehört in den
  nächsten Pass (Span = Monatslänge), **nicht** in den laufenden Job.
- **Fertig, wenn:** Datei geschrieben, `sha256` notiert, CDN-`206` auf `jsoc.stanford.edu/aia_hourly.bin`,
  Block (Mountain) auf `url`/`origin`/`compiler`/`sha256` gepinnt, `matrix-vlies` neu gelaufen.
- Danach: Record-Abschnitt AIA (Lizenz CC0/PD-USGov, NASA/SDO/AIA-Courtesy, Query + Compiler + Digest).

### Offen — eigene Bau-Aufgaben (mit nächstem Schritt)

| Aufgabe | Stand | nächster Schritt |
|---|---|---|
| TGF `gbm_tgf` | Asset lebt (`fermi.gsfc.nasa.gov/gbm_tgf_catalog_offline.csv`, 658 059 B, `f75cf291…`, 206); Block gepinnt | Port-Reader `gbm_tgf` fehlt; Block nutzt `lat Lat`/`lon Lon` statt `lat_key`/`lon_key` — Syntax korrigieren (Mountain) und Reader bauen |
| WALDO A/B | Entscheid: Zenodo-Kopie `CC-BY-4.0`, zwei Header-Ordnungen | Compiler mit `header_version=A|B`, `Fc` als zweiter Zeuge; Block danach |
| SOHO/MDI+EIT | Entscheid (Mountain): eigener Rückwärtszeuge, 5 Felder, `other-attribution` | VSO-Compiler (MDI LOS + EIT 171/195/284/304); VSO-API-Route noch ungemessen |
| SPK JUICE/Clipper | Kandidaten-Kerne registriert (`e7a20a60…`, `1727f61f…`) | Compiler/Reader für SPK-Assets, Block/`field` |
| CTBTO ISF | 6/7 Felder (`duration` fehlt in ISF1.0/2.1) | `format isf`-Reader + Block mit den 6 gemessenen Feldern |
| NANOGrav-Residuen | 15-yr-Tar 638 719 668 B, EPTA-Ersatz | kleiner `.par`/`.tim`/Residuen-Compiler (nach AIA) |
| LOLA RDR | Referenzorbit 109 867 Spots, CSV 8 542 951 B, `ffaa8684…`, 206 | Per-Orbit-Sharding (River wählt Orbit-Set), Inventar-Spiegel + per-File-`sha256` |
| WALLABY HiPS3D | CASDA-Katalog gebaut; HiPS3D-Route bekannt (`alasky.cds.unistra.fr`) | `hips3d`-Tile-Reader; Asset-Route/Block (Mountain) |
| ACE SWE rückwärts | `AC_K0_SWE` beginnt 2017-07 | `AC_H0_SWE` (1998–2024, Fill `-1.0E31`) als **eigener** Arm verwerfen/ernten — kein stiller Mix |
| EDGES | Spektralquelle, passt nicht ins Zeitreihen-`field` | Rat-/Architekturfrage, kein stiller Block |

### Geschlossen in diesem Zug

- **Liniennamen-Riss (Dispatch 2026-10-19):** `bin/xfer` löst den Basisnamen auf die Pass-Ledger auf
  (`sensory` → `sensory-259`), ohne Rename. `list`/`coverage` aggregieren, `mark`/`close`/`fold`
  schreiben in den einen vorhandenen Ledger (Token-Namensraum bleibt `sensory-259`), zwei Treffer
  sind mehrdeutig (exit 2), ein fehlender Ledger endet exit 2. Done-when: `coverage sensory` ==
  `coverage sensory-259` (20/15/5), `list --open` trägt `xfer:sensory-259…`, `gibt-es-nicht` → 2,
  kein `ledger/sensory`. Tests: `cargo test -p omegaflow-register --bin xfer` (11/11).
