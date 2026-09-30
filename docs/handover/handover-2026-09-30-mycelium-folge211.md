<!--
  title: Handover — Mycelium-Folge 211 (2026-09-30)
  session: Mycelium-Folge 211
  class: handover
  date: 2026-09-30
  sha256: e55273068eb4b9c87da1e56865de82d63ca45a0802961b6ce9aed7895dfac66d
  status: live
-->
# Handover — Mycelium-Folge 211 (2026-09-30)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-09-30-mycelium-folge210.md` (→ `archiv/`).

## Operator-Wort-Register

- Wort | 2026-09-30 | „vorbestehend ist verboten mein wort" | Quelle: mountain-209 (`## An mycelium`) — keine Ausnahme für vorbestehende Register-Verstöße in `phi/`; jede `note`-Zeile ≤ 256 Zeichen, keine `#`-Kommentarzeilen in den gated Registern.
- Wort | 2026-09-30 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt)." | Quelle: Mycelium-Session 211; session-weiter Consent (Delegation), **nicht** das Commit-Wort.
- Wort | 2026-09-30 | „es DARF nichts machbares in der nächsten session landen das ist ein befehl" | Quelle: Mycelium-Session 210.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.

## Haus (die vier Orte) — gemessen 2026-09-30

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02).
- `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep` ohne `--all` über den gitignorierten Baum.

## Offen (aufgeschlüsselt)

### CI-Rot-Stand — gemessen, ein Gate-Rot
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende der offenen Läufe
- **Lage:** (gemessen 2026-09-30 via `ci_manage jobs`/`log`, Taucher-Bericht) **success:** `frame-registry 36703933828`, `ersstv5-cdn 36710561770` (403 überwunden), `occultation-cdn 36701698095`, `register-dropped 36701691329`, `register-coverage 36701691297`, `tools-build 36701720094`. **in_progress/queued (unread):** `nvss-cdn 36703925283`, `first14-cdn 36703929656`, `kernel-flatten 36703938769` (bodies), `noaa-cdo-cdn 36708582291`, `ci-check 36701691301`. **rot:** `pds4-binary-cdn 36701710777` (Arm-Riss → Mountain), **`ci-gate 36701691418`** (dropped-gate, s. u.). **cancelled (überholt):** `pds4-fits-cdn`/`pds3-img-cdn`/`pds3-binary-cdn`.
- **Blockade:** keine
- **Braucht:** offene Läufe lesen (`ci_manage jobs <id>`); kein Polling.

### `ci-gate` dropped-gate — rot (Baseline-Bump fällig)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `/commit` (annehmender Commit)
- **Lage:** (gemessen 2026-09-30 via `ci_manage log 36701691418`) `dropped-gate: baseline 1157 | current 1332 | delta 175` @`67d36d6c0`; `docs/zustand/dropped-baseline.md:16` = `dropped-baseline 1157` (Mycelium-209). **Riss:** der Stehende Pass (folge210) nannte „1067 < 1157 grün" — die lokale Zahl (Arbeitsbaum) weicht von der CI-Zahl (committed state) ab; die lokale 1045 ist niedriger, weil die **uncommitteten** Übergaben der anderen Linien (mountain-211, river-71 …) Punkte weiter tragen. Die Gate-Zahl ist CI-only.
- **Blockade:** keine
- **Braucht:** Mycelium committet **als letzter** — erst am eigenen HEAD (nach den anderen Commits) `ci-gate` lesen; die committed-state-Zahl fällt dann. Nur bei weiterhin Delta > 0 die Baseline bumpen (`dropped-baseline.md:13-14`); der Register-Träger-Index (−22) senkt zusätzlich.

### CDN-Manifestationen — Halley/Itokawa void
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `kernel-flatten 36703938769`
- **Lage:** (gemessen 2026-09-30 via `archive_search --verdict`) `ephemeris_halley.bin` (Tag `ssd.jpl.nasa.gov-horizons`) **404 void**; `ephemeris_itokawa.bin` (Tag `ssd.jpl.nasa.gov-ephemeris`) **404 void**; `dr3_stars.bin` 206, `ersstv5_nino34.bin` 206, `spectra.bin` 206 (vorhanden). `halley` steht im Compiler (`tools/harvest/src/bin/horizons_compiler.rs:655`), **nicht** in der `FLYBYS`-Liste (`:13-29`); `kernel-flatten.yml:112` fährt `horizons_compiler --ci-mode` (Default-Liste).
- **Blockade:** CDN-Upload der beiden Assets fehlte bisher
- **Braucht:** `kernel-flatten`-Lauf-Ende lesen; bleibt das Asset void, die Workflow-Direktive (Default-Körperliste) prüfen.

### Neue Compiler-Bins — Arm-Risse
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Arm-Erweiterung (`## An mountain`)
- **Lage:** (gemessen 2026-09-30) vier Bins gebaut (`pds4_fits`/`pds4_binary`/`pds3_binary`/`pds3_img`) + fünf `*-cdn`-Workflows; die Arme decken die echten Daten nicht: `pds4_binary` nur `Table_Binary`-Route (ExoMars `data_raw` → 0 honored, `36701710777` rot); `pds3_binary` nur int 1/2/4/8; `pds3_img` kein `PC_REAL`; `pds4_fits` erstes `IMAGE`-HDU.
- **Blockade:** Parser-Arm-Lücken
- **Braucht:** `## An mountain` (Arm-Erweiterung + ttl/frame).

### Legacy-CDN Re-Manifest — tapvizier erreichbar
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `nvss-cdn 36703925283` / `first14-cdn 36703929656`
- **Lage:** (gemessen 2026-09-30 via `archive_search --verdict`) `tapvizier.cds.unistra.fr/TAPVizieR/tap/capabilities` 200 — nicht void; beide Workflows dispatcht.
- **Blockade:** voriger async-void möglicherweise überholt
- **Braucht:** Lauf-Ende lesen; bei rot `ci_manage log`.

### CDN-Manifestationen pds3/pds4/dr3_stars
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-`ttl`/`frame` für die neuen Formate
- **Lage:** (gemessen 2026-09-30 via `sread`/`sgrep phi/sources.φ`) `dr3_stars` url `:10310` (200), `spectral` `:2416` (200); für die neuen `pds3`/`pds4`-Compiler-Formate tragen `sources.φ` die `url`-Zeilen noch nicht (`STAR_CATALOG_COUNT spatial.rs:8 = 1_704_587`).
- **Blockade:** Quelle/Arm ungeklärt
- **Braucht:** je Format die Daten-URL messen + `url`/`format`/`compiler`/Tag setzen (Mycelium); `ttl`/`frame` → Mountain.

### gosat-cdn / hips-png-cdn — CDN-Release-Upload
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende
- **Lage:** (gemessen 2026-09-30 via `ci_manage`) `gosat-cdn 36283215548` rot: `returned void` (Server-Resultat leer, alle Splits) → Mountain-Quelle; `hips-png-cdn` Shards (4,0)/(5,0) rot: `CDN upload returned void` (Kacheln 3072/3072 + 10000/10000 entschieden, 0 absent), teils Norder-Shards laufend; Re-Dispatch für MoRIC.
- **Blockade:** CDN-Release-Erstellung
- **Braucht:** Lauf-Ende lesen; CDN-Release-Ursache (`gh release`-Cap/Rechte); **Re-Dispatch** — der registrierte Fix (Empty/Overflow getrennt, 2026-09-27) wartet auf einen neuen Lauf, `GWT3F_L1B` liefert aktuell 0 Treffer.

### USGS-comcat Manifestation (mountain-folge211)
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Commit `tools/harvest/src/bin/usgs_comcat_compiler.rs` + Mountain-Register-Feder
- **Lage:** (gemessen 2026-09-30, Taucher) Workflow `.github/workflows/usgs-comcat-cdn.yml` **angelegt** (Bin `usgs_comcat_compiler`, `--ci-mode` `:207`, Tag `earthquake.usgs.gov`, Asset `usgs_comcat_m45.bin`). Quelle `url https://earthquake.usgs.gov/fdsnws/event/1/query`. **Riss:** der Compiler ist uncommittet (`?? tools/harvest/src/bin/usgs_comcat_compiler.rs`, Mountain) → CI-`checkout` trägt ihn noch nicht.
- **Blockade:** `usgs_comcat_compiler.rs` uncommittet (Mountain); `phi/sources.φ` fremd-modifiziert
- **Braucht:** `url`/`format`-Zeile in `sources.φ` setzen, sobald Mountain Compiler + Register-Feder committet hat.

### `tao-wnd-cdn.yml` — volles Fenster (mountain-folge211)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Dispatch nach Release-Asset-Löschung
- **Lage:** (gemessen 2026-09-30, Taucher) `tao-wnd-cdn.yml:35-36` **korrigiert** auf `d_end=heute` / `d_start=1977-11-06T00:00:00Z` (deckungsgleich mit `tao_wnd_compiler.rs:5`; das Fenster saß allein in den zwei `date`-Zeilen).
- **Blockade:** keiner
- **Braucht:** Release-Asset `coastwatch...`/`tao` vor dem Dispatch löschen, dann Re-Dispatch.

### `pds4-binary-cdn.yml` — Quellenzuordnung (mountain-folge211)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Dispatch `pds4-binary-cdn` nach Compiler-Commit
- **Lage:** (gemessen 2026-09-30, Taucher) der Arbeitsbaum-`pds4_binary_compiler.rs` routet `Table_Character` bereits selbst auf `pds4_fixed_width_*` (uncommittet); der Workflow `pds4-binary-cdn.yml:25-32` **korrigiert**: Idempotenz-Grep auf `^pds4_(binary|fixed_width)_` erweitert. `--bin pds4_binary_compiler` bewusst unverändert (der `pds4_fixed_width_compiler` ist Hayabusa-fixiert).
- **Blockade:** `pds4_binary_compiler.rs` uncommittet (Mountain)
- **Braucht:** Re-Dispatch, sobald der Compiler-Routing-Stand committet ist.

### CDSE-CCM — STAC-Arm steht, Asset-Kantenschritt
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `stac_asset_fetch --asset`-Messung
- **Lage:** (gemessen 2026-09-30) Mountain: `src/archivar/stac.rs` + `tools/harvest/src/bin/stac_asset_fetch.rs` gebaut; `/stac/collections` 200 → `ccm-optical`/`ccm-sar`/`ccm-thermal-lst-hr`/`-mr`/`ccm-hyperspectral-ref-hr`; `--items .../collections/ccm-optical/items` 200 → 10 Assets (OData `$value`, `application/zip`); Admission `phi/blocked_sources.φ` pending.
- **Blockade:** `--asset` lädt den ganzen Body in Memory (PHR-Produkt ~100–500 MB) — ein lokaler Voll-Download ist eine schwere Handlung
- **Braucht:** CI-Probe `.github/workflows/cdse-stac-probe.yml` (dispatch nach Push) — listet CCM + lädt ein Asset (`--asset`, `secrets.CDSE_TOKEN`), Größe+sha256 im Log; danach Mountain `ttl`/`frame` + `url`/`format`. **Riss:** `CDSE_TOKEN` steht nicht im `secrets-sync`-Trockenlauf → falls der Abruf Auth braucht, ist das Setzen ein Operator-Akt.

### future-155/156 Sources-Zeilen — Aufnahme-Regel
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-`ttl`/`frame`-Zeilen (`phi/sources.φ`)
- **Lage:** (gemessen 2026-09-30) `src/archivar/parse.rs:80-85` verlangt `ttl>0`/`no-cadence` **und** `frame`/`extract`; ShadowCam/ESA-PSA-TAP/Chang'e-MRM register-reif bis auf die Mountain-Feder.
- **Blockade:** Mountain-`ttl`/`frame`
- **Braucht:** `## An mountain`.

### kuprat Family-Tag — Phantom
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`phi/witnesses.φ`)
- **Lage:** (gemessen 2026-09-30) kein Compiler setzt `tag kuprat`; die vier Kanäle liegen unter gemessenen Tags (`crystallography.net`/`srdata.nist.gov`), admitiert als Substance-Witnesses (`witnesses.φ:120-142`).
- **Blockade:** kein `kuprat`-Host
- **Braucht:** `## An mountain`.

### NOAA-NCDC-CDO-Token
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `noaa-cdo-cdn 36708582291`
- **Lage:** (gemessen 2026-09-30) `NOAA_CDO_TOKEN` **vorhanden** — `bin/secrets-sync.sh` (Trockenlauf, **Namen only**, 0 Cloud) nennt `would NOAA_CDO_TOKEN`; `docs/specs/ref-auth-apis.md:57` führt ihn als vorhanden (2026-08-14); `phi/sources.φ:1117` `header token {NOAA_CDO_TOKEN}`. Token-authentifizierte CDO-Messung dispatcht (`noaa-cdo-cdn 36708582291`, CI liest den Token intern).
- **Blockade:** keine
- **Braucht:** Lauf-Ende lesen (`ci_manage jobs 36708582291`); bei rot Träger. Der Wert wird nie in den Transcript geschrieben (Werkzeug liest intern).

### hinet-cdn Readiness
- **Status:** wartend | **Bindung:** eigen ← operator
- **Trigger:** Operator-Wort Proton-Exit, oder Route trägt
- **Lage:** (gemessen 2026-09-30) `hinetwww11.bosai.go.jp` direct 403; Wayback nur 2013.
- **Blockade:** 403 direkt
- **Braucht:** Operator-Wort für den Proton-Exit (Geo-Umgehung) oder Re-Dispatch bei Readiness.

### KARI KPDS (Danuri/KPLO) — kein Maschinen-Endpunkt
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Verdikt `## An mountain` (descope oder neuer Endpunkt)
- **Lage:** (gemessen 2026-09-30 via `--playwright`) `www.kari.re.kr/kpds/` 200, gerendert nur Login/Search/`mailto:` — kein maschinenlesbarer Endpunkt (0 honored).
- **Blockade:** kein Daten-Endpoint
- **Braucht:** `## An mountain` (descopen mit Befund oder auf Portal-Antwort warten).

### te-gate #112/#13/#43 — River
- **Status:** wartend | **Bindung:** eigen ← river
- **Trigger:** `te-gate`-Lauf
- **Lage:** (gemessen 2026-09-30) `#112` rot bestätigt; river70 trägt die Membran-FPR auf 128 Trials (Rat). `#13` (n=1000 FPR) + `#43` (measure-gates) bleiben.
- **Blockade:** keine
- **Braucht:** `## An river`.

### Quellenseitige Waits (`state/zustand/wartend.φ`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort/Readiness
- **Lage:** (gemessen 2026-09-30 via `mail_ledger.φ`) keine neue Antwort von NSSDCA/JPL; Asmar (mail 191) verwies auf die Paper-Autoren (Antwort 192), drei ODF-Anfragen raus (193–195). SAMPLE_CONTACT-Antwort (181/184) liegt vor.
- **Blockade:** Antwort
- **Braucht:** Postfach weiter beobachten (Aufnehmer mycelium).

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 · 2026-10-19 · 2026-12-02 · 2026-12-03 · 2027-04-01
- **Trigger:** `superdarn-af68c4f1` · `emodnet-hfr` · `noirlab-gaia-dr4` · `europa-clipper` · `bepicolombo-more`
- **Lage:** (gemessen 2026-09-30 via `state/zustand/wartend.φ`) Wiedervorlage, Aufnehmer mycelium.
- **Blockade:** Termin
- **Braucht:** `archive_search --verdict <url>` beim jeweiligen Datum.

### GitHub-Issues — Zensus
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Issues-Zensus (`gh issue list`)
- **Lage:** (gemessen 2026-09-30) `ci_manage list` listet Runs, keine Issues; `gh issue list` ist verweigert (Permission-Map). Zuletzt bekannt (`/tmp/opencode/issues.json`, 2026-09-29): `#98` paper gate, `#81` clippy, `#80` Anomaly, `#71` bodies void, `#67` de441, `#60/#58` recheck/cargo-test, `#53` flatten.
- **Blockade:** `gh issue` nicht erlaubt
- **Braucht:** `gh issue list` in einer Rolle, die es trägt, oder eine kanonische Issue-Leseform.

### `register_lookup --dropped` — Rest-Gap (Register-Träger-Index gebaut)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** keiner (descoped, gemessen)
- **Lage:** (gemessen 2026-09-30 via `./target/debug/register_lookup --dropped --count`) Register-Träger-Index gebaut (`tools/register/src/bin/register_lookup.rs` `load_register_content_index`: Wort-Index gegen `phi/*.φ` + `phi/pipeline/*.φ` + `state/zustand/wartend.φ`): **1067 → 1045** (−22). Gate grün (< Baseline). Debug-Laufzeit ~60 s; das Gate (`register-dropped.yml:20`) baut **release** → schnell. Ein Lowercase-Fix im Content-Index-`offer_word` glättete auf **145** (Falsch-Positive über deutsche Groß-Nomen in Commit-Prosa) und wurde **verworfen** — der Riss ist benannt, nicht geglättet.
- **Blockade:** keine
- **Braucht:** kein Schritt — die Messung ist der Befund.

### ersstv5 403 — Diagnose (gebaut)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ersstv5-cdn`-Lauf
- **Lage:** (gemessen 2026-09-30) Diagnose-Step in `.github/workflows/ersstv5-cdn.yml` ergänzt (spiegelt `ersstv5_compiler.rs:12-17`; `curl -g -D -` + Body-Größe, `continue-on-error`), damit der runner-spezifische 403 header-belegt gelesen wird; lokal 200 (14 999 659 B); `ersstv5_nino34.bin` 206 vorhanden.
- **Blockade:** keine
- **Braucht:** Lauf-/Dispatch-Ergebnis lesen (`ci_manage log <id>`).

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383` §14.4)
- **Lage:** (gemessen 2026-09-30) kein Producer-Bin/Register/Wf
- **Blockade:** Producer fehlt
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag ändert den Zustand.

### Trägerlose Dokumente
- **Status:** wartend | **Bindung:** eigen ← river/future (je ein Dokument)
- **Trigger:** nächster `register_lookup --orphan-docs`-Pass
- **Lage:** (gemessen 2026-09-30 via `register_lookup --orphan-docs` + `sread docs/concepts/tools-map.md:310-347`) **Mycelium-eigen:** `docs/concepts/exzellenz-konzept.md` (2), `docs/concepts/kybernetische-astrophysik.md` (1), `docs/surveys/survey-2026-09-03-orphan-verdicts.md` (1), `docs/concepts/tools-map.md` (2 — `:310-313` Chrome-DevTools-MCP gebunden, offen bleibt allein das Operator-Wort Debugger-Rechte am live Chrome; `:329-347` Pfad-4 `opencode-chromium` registriert, gemessen). **Fremd (nicht eigene Feder):** `docs/surveys/survey-messpunkt-verteilung.md` (3) → River; `docs/auftrag/auftrag-gic-einreichung.md` → Future (Übergabe privat).
- **Blockade:** Prosa-Marker; teils echte Messgrenzen (getragen, nicht geglättet)
- **Braucht:** die vier Mycelium-Dokumente mit einem Träger benennen oder descope-annotieren; River + Future je eigenes.

## An mountain

Origin: mycelium-folge211 (Mountain-210-Block gefaltet; Sender entfernt beim nächsten Pass).

- **Parser-Arm-Lücken (blockieren die neuen Compiler-Bins).** `src/archivar/pds4_binary.rs`
  deckt nur `Table_Binary` — ExoMars-ACS `data_raw` hat keines (`36701710777` rot, 0 honored).
  `pds3_binary.rs` nur int 1/2/4/8; `pds3_img.rs::byte_order_of` kein `PC_REAL`;
  `pds4_fits.rs` nimmt das erste `IMAGE`-HDU. Bins:
  `tools/harvest/src/bin/pds3_binary_compiler.rs`, `pds3_img_compiler.rs`,
  `pds4_fits_compiler.rs`, `pds4_binary_compiler.rs`.
- **ttl/frame für die drei register-reifen Endpunkte** (ShadowCam `at moon`+`ttl 604800`,
  ESA PSA TAP `ttl 604800`+`field`/`at`, Chang'e MRM `no-cadence`) — dann setzt Mycelium `url`+`format`.
- **CDSE-CCM Admission:** `https://catalogue.dataspace.copernicus.eu/stac/collections` (200, 53309 B) — neuer STAC-Arm + ttl/frame.
- **`gosat-cdn` / `hips-png-cdn` „returned void":** `gosat-cdn` liefert leeres Suchergebnis über alle Splits (Source-Seite); `hips-png-cdn` Shards (4,0)/(5,0) `CDN upload returned void` (Kacheln 3072/3072 + 10000/10000 entschieden, 0 absent) — Release-Upload-Seite.
- **`kuprat` Family-Tag:** kein Compiler setzt `tag kuprat`; die vier Kanäle sind als Substance-Witnesses admitiert (`witnesses.φ:120-142`), liegen unter den Tags `crystallography.net`/`srdata.nist.gov` — kein `kuprat`-Host.
- **KARI KPDS:** kein maschinenlesbarer Endpunkt (0 honored) — descopen mit Befund oder auf Portal-Antwort.
- **DAS2 Iowa (`blocked_sources.φ:374`):** `src/archivar/port.rs:1369` `hapi_draft_fields_csv` ist ein Register-Draft-Textgenerator, kein Datenleser; ein Compiler braucht einen echten Reader (Vorbild `omni_hro_compiler.rs`).
- **`kernel-flatten` `spk_split`-Ordnungsfehler** (Kern): „segment data begins before address … summary arrived after its data" → `curl: (23)` (`ci_manage log 36639779027`).
- **Riss:** `blocked_sources.φ:443` (`gap html-parser-arm`) widerspricht `extract.rs:2689`.

## An river

Origin: mycelium-folge211.

- **`te-gate #112`** rot bestätigt; **`#13`** (n=1000 FPR) + **`#43`** (measure-gates) bleiben. river70 trägt die Membran-FPR auf `MEMBRANE_FPR_TRIALS = 128` (Rat) — bitte an `#112`/`#13` weiter. Braucht River-Entscheid Phase vs. Arx-Switch. *(Transport-Riss geheilt: der Punkt stand in folge210 nur in der Offen-Liste `:132-138` + im Issues-Zitat `:160` + Stehender-Pass, **nicht** im `## An river`-Block — `--addressed river` lieferte ihn nicht; das DROPPED-Register führte ihn als mycelium-207→208 gefallen. Jetzt im Block: beim Falten aus diesem Zitat aufnehmen.)*
- **`ci-check` cargo-test GPU:** 4 Ausfälle (`te_gpu_crosscheck`, `te_wgsl_validates_offline`, `gate_wgsl_horizon_and_ksg_parity`, `volume_probe_parity_masked_corner_and_plain`) — WGSL `find_cross_mi_lag`-Arität; river70 hat `shaders.rs`/`te.rs` geheilt, neuer Lauf bestätigt.
- **`open_points_check`-Test `extracts_bold_keywords` geheilt** (river-folge71): `first_bold_words` (`tools/register/src/bin/open_points_check.rs:553`) splittet jetzt auf non-alphanumerisch — der Test ist die Spezifikation, die `-`-Ausnahme der Bug. `cargo build -p omegaflow-register --bin open_points_check` 0/0.

## An future

Origin: mycelium-folge211.

- **Secret-Rotation** — GitHub-PATs `omegaflow-read`/`omegaflow-ci-write` regeneriert (mail 200/201) → `http_401`-Residuum beobachten.
- **NSE-Redistribution + Dank** — Reply `state/mail/[redacted].md` wartet.
- **ENSO-Zuschnitt** — `ersstv5-cdn` success; offen nur der Blatt-Zuschnitt (NINO3.4).
- **`gic-causal-driver.md` DOI-Minting** — DOIs `pending`; erst nach Einfrieren.
- **NCIS Research Grant** (mail 196) + LCO/KARI/Helmholtz-Konto-Aktivierungen (mail 188/189/206) — neu eingegangene Konto-/Grant-Vorgänge für die Operator-Queue.
- **`docs/auftrag/auftrag-gic-einreichung.md`** (sensory-folge213) — Future-eigenes Dokument, 1 offener Marker, trägerlos; braucht einen öffentlichen Träger in Futures Übergabe oder descope-annotieren (0 honored).
- **ODF-Encounter-Konsument** — Mountain hat `src/archivar/flyby_encounters.rs` (Earth-Flyby-Epochen) gebaut; der Konsument wartet auf die request-only DSN-Rohdaten (Operator-Anfrage / Paper-Autoren, Asmar-Verweis mail 191).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
