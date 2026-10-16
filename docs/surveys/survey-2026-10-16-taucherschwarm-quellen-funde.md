<!--
  title: Survey — Taucherschwarm 2026-10-16: Quellen-Funde zu offenen Punkten
  class: survey
  date: 2026-10-16
  sha256: 37c3e96f5ed6ce04ed48098eb20a2fcb160f3bccfab8fc17ed9785c49356e8d1
  status: done
-->
# Survey — Taucherschwarm 2026-10-16: Quellen-Funde zu offenen Punkten

**Frage.** Zu welchen offenen Punkten aus `survey-2026-10-16-gap-zensus-riss-pending.md`
findet ein Taucherschwarm (Baum + Netz, `archive_search`) eine **Antwort**?

**Status der Einträge.** Jede Zeile ist ein **Taucher-Claim** (taucher-gemessen, HTTP/`rg`-belegt) —
**nicht** registriert; die Session-Gegenprobe steht vor jedem Register-Schreibakt aus. Nichts
geglättet; Risse sind benannt.

## 1 · CDN-404 (Transport-Riss) — Faden 4
- **Gemessen:** drei registrierte CDN-`url` = **HTTP 404** — `phi/sources.φ:30641` (conicus3d),
  `:31226` (KiDS DR4 bright), `:31237` (KiDS DR4.1 WL); Ursache: **keine Matrix-Einträge** in
  `.github/workflows/zenodo-raw-cdn.yml`, keine `sha256`.
- **Alternative (HTTP 206 gemessen):** die `origin`-Zenodo-Upstreams
  `zenodo.org/api/records/{18776222,17086764,16366035}/files/…/content` und Leiden
  `kids.strw.leidenuniv.nl/DR4/data_files/…`.
- **Zu korrigieren:** der **Mirror-Vertrag** (3 Matrix-Einträge), **nicht** die `url`-Zeile. Zusatz:
  `format tar_zst` hat **keinen Reader** (`fetch.rs`) und ein Fingerprint fehlt → **Mycelium/Code**.
- **Riss (Nebenfund):** `sources.φ:31226` (KiDS-Block) trägt einen **NOIRLab-LS-Feldnamen** —
  mögliche Copy-Paste-Divergenz, eigener Prüfpunkt.

## 2 · GALAH-Block (Riss B) — Faden 4
- **Gemessen am Datacentral-TAP** (`https://datacentral.org.au/vo/tap`): `galah_dr3.main_star` trägt
  **`ra`/`dec`** (NICHT `ra_dr2`/`dec_dr2`; Gegenprobe `ra_dr2` → HTTP 400 „Unknown column").
- **Präzisierung:** die registrierte **URL ist schon korrekt** (`ra,dec,…`, HTTP 200); der Fehler sitzt
  **nur in den `cmap`-Zeilen** `phi/sources.φ:9943-9944` → korrekt: **`ra ra` / `dec dec`**. URL ≠
  Fehler → **interner Divergenz-Riss**, nicht glätten.

## 3 · EVE (Riss A) — Faden 4
- **Bestätigt:** kein `eve_*`-Block, kein `format eve` in `phi/sources.φ` (der Kanal ist im Code
  verdrahtet: `src/archivar/eve.rs` MAGIC `EVL1`, `eve_compiler.rs`).
- **Quelle (live gemessen):** LASP EVE L2-Stundenraster
  `lasp.colorado.edu/eve/data_access/evewebdata/products/level2/{YYYY}/{DDD}/EVL_L2_…fit.gz`
  (der Compiler-Pfad stimmt exakt); LISIRD-HAPI **`sdo_eve_ssi_1nm_l3`** — **License CC0-1.0**
  gemessen (2010-04-30 … 2026-10-15).
- **Zu registrieren:** Vorschlag `url` (CDN `lasp.colorado.edu-eve`) · `terms CC0-1.0` ·
  `format eve` · `origin …/level2/` · `compiler tools/harvest/src/bin/eve_compiler.rs` · `at sun` +
  11 `field eve_<λ>_line_irradiance`. Junktim: `url/origin/compiler` = **Mycelium**,
  `terms/field/ttl` = **Mountain**; CI-Vorbedingung `eve-cdn.yml` + Tag.
- **Riss (Doku vs. Baum):** `docs/paper/solar-seconds-matrix.md:22` behauptet, `eve_lines_2011.bin`
  stehe in `phi/sources.φ` — der Baum sagt **nein**. **Namens-Risse:** Survey `sdo_eve_ssi_l3` → real
  `sdo_eve_ssi_1nm_l3`; `…/evewebdata/…` vs. `…/eve_data/…`.

## 4 · 21 cm / EDGES (Faden 6 — „nie registriert")
- **Nicht in `phi`** (`loco.lab`/`galileo.sese`/`bowman`/`saras` = 0; `edges` = nur `phi:12236` =
  DECam-g-Band). → **Die Lücke liegt in phi, nicht in der Welt** — bestätigt.
- **Beste offene Quelle — EDGES** (Bowman+2018, `10.1038/nature25792`): DR1 reduziert
  `https://loco.lab.asu.edu/download/790/` (200/206, sha256, CSV `…,e:T21[K]`); Kalibration
  Zenodo `10.5281/zenodo.18091240` (**cc-by-4.0**); Roh `galileo.sese.asu.edu/edges-data/`
  (~150 `.gsh5`, 1,2–3,6 GB ⇒ Shard). **SARAS 3/LEDA = kein Fund**, **REACH `pending`**.
- **Zu registrieren:** Vorschlag `format csv` + `field brightness_temperature … T21` +
  `compiler edges_compiler.rs` (K-nativ); Zweit-Asset Zenodo `cc-by-4.0` (Shard).

## 5 · WALLABY (zweiter Riss, Faden 6)
- **Der Block** (`phi:14519` → real heute **`phi:14569–14576`**) registriert nur das **CDS-HiPS3D-
  Produkt** (`terms ODbL-1.0`, `format hips3d`), nicht den Survey; `format hips3d` hat **keinen
  Reader** (`rg hips3d` in `src/`+`tools/` = 0); `field`/`compiler`/`sha256` fehlen.
- **Echter Datensatz:** CASDA/CSIRO TAP `https://casda.csiro.au/casda_vo_tools/tap`, collection
  *„ASKAP Pilot Survey for WALLABY"* (44 987 Zeilen); DOIs `10.25919/hg66-4v60` (+ `…qw7w-tn96`,
  `…47tr-k441`), alle **CC-BY-4.0**; Paper Murugeshan 2024 `10.1017/pasa.2024.91`.
- **Riss-Verschärfung:** `ODbL-1.0` = **Visualisierer**-Lizenz (CDS/Unistra), die **Daten** =
  **CC-BY-4.0** — der Block zitiert den Visualisierer, nicht die Quelle.

## 6 · TGF-Katalog (Roberts)
- **Kein Journal-Artikel, keine DOI.** Real: **Vortrag** „The 3rd Fermi-GBM TGF Catalog",
  **Briggs** et al., 11th Fermi Symposium (2024-09-12); Slides-PDF
  `fermi.gsfc.nasa.gov/…/thu/Briggs_TGFs_Catalog.pdf` (200, 2 117 507 B). Inhalt: **~17 000** TGFs,
  11 a, **4280** mit WWLLN.
- **Riss (Survey):** Autorenzuordnung „Roberts" falsch → **Briggs** (Vortragender), Roberts Co-Autor;
  Zahl **~17 000**, nicht 15 000.
- **Datensatz-URL bleibt `pending`** — Slides sagen „available at FSSC in ~December [2024]"; Stand
  **2026-10-16** trägt das FSSC weiterhin nur den **2. Katalog** (Roberts 2018, `10.1029/2017JA024837`,
  5 CSVs live). Nicht glätten → offener Punkt.
- **Disambiguierung:** „3rd **AGILE** TGF Catalog" (Ursi 2020, `10.1029/2019jd031985/.986`) ≠ GBM.

## Synthese — welche Gaps jetzt eine Antwort haben
| Offener Punkt | Antwort gefunden? | Wer handelt |
|---|---|---|
| CDN-404 (3 urls) | ja — Alternativen + Ursache | Mycelium/Code (Matrix + `tar_zst`-Reader) |
| GALAH-Block (Riss B) | ja — `cmap` `ra ra`/`dec dec` | Mountain (Register-Text) |
| EVE (Riss A) | ja — Quelle + Block-Entwurf | Mycelium + Mountain |
| 21 cm / EDGES | ja — Quelle + Block-Entwurf | Mycelium + Mountain |
| WALLABY | ja — CASDA-DOI + Lizenz-Riss | Mycelium + Mountain |
| TGF-Katalog | **teil** — Referenz korrigiert; Datensatz bleibt `pending` | Future (Operator/FSSC) |

**Was bleibt offen:** der TGF-**Datensatz** (FSSC versprochen ~12/2024, noch absent); der
`tar_zst`/`hips3d`-Reader; die Doku-vs-Baum-Divergenzen (EVE solar-seconds-matrix; Survey-Zeilen
`phi:14519`, `phi:9898`).

**Grenze (ehrlich).** Alle Einträge sind **Taucher-Claims** — vor jedem Register-Schreibakt prüft die
Session am Baum nach (house-rules: ein Taucher-Befund ist ein Claim, keine Messung). Netz-Zugänge
sind HTTP-gemessen (200/206/404); „kein Fund" heißt **nicht** „existiert nicht". Architektur/Register-
Form → Rat/Mountain.

*Gemessen 2026-10-16 (Sensory; 6 Taucher). Quellen: `archive_search` (Baum + Netz), `phi/sources.φ`,
Datacentral-TAP, CASDA-TAP, LASP-HAPI, FSSC/Wayback.*
