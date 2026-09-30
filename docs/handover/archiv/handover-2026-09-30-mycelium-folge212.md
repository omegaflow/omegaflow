<!--
  title: Handover — Mycelium-Folge 212 (2026-09-30)
  session: Mycelium-Folge 212
  class: handover
  date: 2026-09-30
  sha256: ff19825aebe9f9c8257f14c72e66edb08a7a9e0fb75cddbcbb325bfe9f35d240
  status: live
-->
# Handover — Mycelium-Folge 212 (2026-09-30)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Kein Standard-Pass: es gilt der **Stehende Pass**
(`state/zustand/standing-pass.md`, zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-09-30-mycelium-folge211.md` (→ `archiv/`).

## Burn: open 0.0 · close 0.0709

## Operator-Wort-Register

- Wort | 2026-09-30 | „vorbestehend ist verboten mein wort" | Quelle: mountain-209 (`## An mycelium`) — keine Ausnahme für vorbestehende Register-Verstöße in `phi/`; jede `note`-Zeile ≤ 256 Zeichen, keine `#`-Kommentarzeilen in den gated Registern.
- Wort | 2026-09-30 | „NATÜRLICH UND VERSCHLEPPEN IST VERBOTEN!!!!" | Quelle: Mycelium-Session 212 — Ausführungs-Consent Phase 2; jeder machbare Punkt wird im nennenden Atom gearbeitet, nichts wandert in die nächste Session.
- Wort | 2026-09-30 | „bitte wirklich bis zur kante umsetzen nicht nur wieder messen und verschleppen" | Quelle: Mycelium-Session 209.

## Haus (die vier Orte) — gemessen 2026-09-30

- `omegaflow` = `$HOME/projects/omegaflow` (+ privates Schwester-Repo `state/`, Remote `omegaflow/personal`).
- `omegaflow-legacy` = `archive-root/omegaflow-legacy` (+ backup-2026-09-02).
- `temp` = `/tmp/opencode`; `archive` = `archive-root` (+ `~/backup/archive/omegaflow`).
- Linien-Preset: `state/mycelium/archive-search-preset.txt` — `--root .github --root tools --root phi --root state --root docs/handover --root docs/concepts`.
- `state/` wird mit `archive_search <kw> --root state` vermessen, **nie** `sgrep` ohne `--all` über den gitignorierten Baum.
- `phi/pipeline/catalog/*` ist **gitignored** (Working-Tree-Arbeitsdateien); `phi/pipeline/index.φ` + `ledger.φ` sind trackbar.

## Erledigt in diesem Atom (nicht mehr getragen)

- **NOAA-CDO-Token** — `noaa-cdo-cdn 36708582291` success; Asset `noaa_cdo_ghcnd_tmax.bin` gemessen 200, 6952 B, sha256 `43a78051…`.
- **ersstv5 403-Diagnose** — `ersstv5-cdn 36714635786` **success** (Diagnose-Step trug).
- **pds4-binary Arm-Riss** — `pds4-binary-cdn` success (Arm routet `Table_Character`); ebenso `pds4-fixed-width-cdn` success, `pds4-fits-cdn` queued.
- **Katalog-Befunde eingetragen** (Working Tree, gitignored): b2find S1 (INTERMAGNET-Stationsliste) gelöst über `GINServices?Request=GetCapabilities&format=json` (200, 29241 B, IagaCode/Latitude/Longitude); terrapulse `hourly_precip-3240`-Familie (51) als `dead` (404); esa_geomagnetic S3 gelöst; archeology absent/pending-Block; copernicus Recheck.
- **Re-Dispatches gefeuert:** gosat, hips-png, tao-wnd (Asset `tao_wnd_zonal.csv` gelöscht), pds4-binary, usgs-comcat, ersstv5.

## Offen (aufgeschlüsselt)

### CI-Tafel — Läufe am HEAD lesen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende
- **Lage:** (gemessen 2026-09-30 via `ci_manage list`/`view`) **success:** `pds4-binary-cdn 36716488328`, `pds4-fixed-width-cdn 36716496970`, `ersstv5-cdn`, `paper-check 36715791327`, `frame-registry`. **queued/unread:** `ci-gate 36717357302`, `ci-check 36717357356`, `register-coverage 36717357314`, `gosat-cdn 36714631159`, `usgs-comcat-cdn 36714687505`, `tao-wnd-cdn 36716501098`, `hips-png-cdn 36718182275`, `pds4-fits-cdn 36716492698`, `cdn-health 36716604525`, `matrix-rotor 36718499001`, `nvss-cdn`/`first14-cdn` (in_progress), `kernel-flatten 36703938769` **failure**. Viele `cancelled` = rolling auto-dispatch überholt.
- **Blockade:** keine
- **Braucht:** `ci_manage jobs <id>` / `ci_manage log <id>`; kein Polling.

### `ci-gate` dropped-gate
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `/commit` (annehmender Commit)
- **Lage:** (gemessen 2026-09-30) rot @`67d36d6c0` `dropped-gate: baseline 1157 | current 1332 | delta 175`; `--dropped --count` lokal = **1075**. Baseline `docs/zustand/dropped-baseline.md:16` = 1157. Der neue `ci-gate 36717357302` @`f4f271277` ist queued.
- **Blockade:** keine
- **Braucht:** `ci-gate` am neuen HEAD lesen; nur bei weiterhin Delta > 0 die Baseline bumpen (`dropped-baseline.md:13-14`).

### Halley/Itokawa CDN-Manifestation
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** `kernel-flatten` Lauf (rot)
- **Lage:** (gemessen 2026-09-30 via `--verdict`) `ephemeris_halley.bin` 404 absent; `ephemeris_itokawa.bin` 404 absent; `kernel-flatten 36703938769` **failure**. `halley` steht im Compiler (`horizons_compiler.rs:655`), **nicht** in der `FLYBYS`-Liste (`:13-29`); `kernel-flatten.yml:112` fährt die Default-Liste.
- **Blockade:** `kernel-flatten` rot (`spk_split`-Ordnungsfehler, s. `## An mountain`)
- **Braucht:** `## An mountain` (`spk_split`), dann Re-Dispatch `kernel-flatten` + `--verdict`.

### Neue Compiler-Bins — verbleibende Arm-Risse
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Arm-Erweiterung (`src/archivar/pds3_binary.rs`, `pds3_img.rs`)
- **Lage:** (gemessen 2026-09-30) `pds4_binary`/`pds4_fixed_width`/`pds4_fits` laufen (Arme geheilt); offen bleiben `pds3_binary` (nur int 1/2/4/8), `pds3_img` (kein `PC_REAL`).
- **Blockade:** Parser-Arm-Lücken
- **Braucht:** `## An mountain`.

### CDN-Manifestationen pds3/pds4/dr3_stars
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-`ttl`/`frame` für die neuen Formate
- **Lage:** (gemessen 2026-09-30) `dr3_stars` url `:10310` (200), `spectral` `:2416` (200); für `pds3`/`pds4`-Formate tragen `sources.φ` die `url`-Zeilen noch nicht.
- **Blockade:** Quelle/Arm ungeklärt
- **Braucht:** je Format Daten-URL messen + `url`/`format`/`compiler`/Tag (Mycelium); `ttl`/`frame` → Mountain.

### USGS-comcat Manifestation
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`phi/sources.φ`-Feder)
- **Lage:** (gemessen 2026-09-30) `usgs_comcat_compiler.rs` committet (`49b18dd6f`), Workflow committet, `usgs-comcat-cdn 36714687505` queued; Tag `earthquake.usgs.gov`, Asset `usgs_comcat_m45.bin`, `BASE=https://earthquake.usgs.gov/fdsnws/event/1/query` (MIN_MAG 4.5). `phi/sources.φ` trägt **keine** comcat-`url`-Zeile.
- **Blockade:** Mountain-Admission (field/force_type) fehlt — `url`/`format`/`compiler` (Mycelium) brauchen den Block-Skelett
- **Braucht:** `## An mountain` (Admission-Feder); danach Transport-Zeilen + Lauf-Ergebnis.

### tao-wnd-cdn
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `36716501098`
- **Lage:** (gemessen 2026-09-30) `tao_wnd_zonal.csv` aus Release `data.pmel.noaa.gov` **gelöscht**; `tao-wnd-cdn.yml:35-36` volles Fenster committet; Re-Dispatch `36716501098` queued.
- **Blockade:** keine
- **Braucht:** Lauf-Ende lesen; bei rot `ci_manage log`.

### gosat-cdn / hips-png-cdn
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende
- **Lage:** (gemessen 2026-09-30) `gosat-cdn 36714631159` + `hips-png-cdn 36718182275` neu dispatcht (queued); vorheriger `gosat-cdn 36283215548` rot „returned void" / `GWT3F_L1B` 0 Treffer; `hips-png-cdn` Shards (4,0)/(5,0) „upload returned void".
- **Blockade:** GOSAT-Source-Fenster; CDN-Release-Ursache
- **Braucht:** Lauf-Ende lesen; bei rot Ursache (`gh release`-Cap/Rechte).

### Legacy-CDN Re-Manifest (tapvizier)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Lauf-Ende `nvss-cdn` / `first14-cdn`
- **Lage:** (gemessen 2026-09-30) `tapvizier.../capabilities` 200; beide Läufe in_progress.
- **Blockade:** keine
- **Braucht:** Lauf-Ende lesen.

### CDSE-CCM — STAC-Asset-Kante
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `cdse-stac-probe` Lauf-Ende
- **Lage:** (gemessen 2026-09-30) Arm `stac_asset_fetch` steht; Collections/Items 200; Probe `36713089816` queued; `--asset`-Step `continue-on-error` mit `CDSE_TOKEN`.
- **Blockade:** `CDSE_TOKEN` nicht im `secrets-sync`-Trockenlauf (Riss)
- **Braucht:** Probe-Log lesen → Größe+sha256; bei Auth-Fehler `CDSE_TOKEN` = Operator-Akt (s. `## An future`).

### future-155/156 Sources-Zeilen
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-`ttl`/`frame`
- **Lage:** (gemessen 2026-09-30) `src/archivar/parse.rs:80-85` verlangt `ttl>0`/`no-cadence` **und** `frame`/`extract`; ShadowCam/ESA-PSA-TAP/Chang'e-MRM register-reif bis auf die Feder.
- **Blockade:** Mountain-Feder
- **Braucht:** `## An mountain`.

### kuprat Family-Tag
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Admission (`phi/witnesses.φ`)
- **Lage:** (gemessen 2026-09-30) kein Compiler setzt `tag kuprat`; die vier Kanäle liegen unter `crystallography.net`/`srdata.nist.gov`, admitiert (`witnesses.φ:120-142`).
- **Blockade:** kein `kuprat`-Host
- **Braucht:** `## An mountain`.

### KARI KPDS — kein Maschinen-Endpunkt
- **Status:** wartend | **Bindung:** eigen ← mountain
- **Trigger:** Mountain-Verdikt (`## An mountain`)
- **Lage:** (gemessen 2026-09-30 via `--playwright`) `kari.re.kr/kpds/` nur Login/Search/`mailto:` (0 honored).
- **Blockade:** kein Endpoint
- **Braucht:** `## An mountain` (descopen mit Befund).

### hinet-cdn Readiness
- **Status:** wartend | **Bindung:** eigen ← operator
- **Trigger:** Operator-Wort Proton-Exit oder Route trägt
- **Lage:** (gemessen 2026-09-30) `hinetwww11.bosai.go.jp` direct 403; Wayback nur 2013.
- **Blockade:** 403 direct
- **Braucht:** `## An future` (Operator-Wort Geo-Umgehung).

### Register-Träger — `phi/pipeline/index.φ` 7 offen (Katalog-Arbeitsdateien)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Katalog-Port (`phi/pipeline/index.φ`, `docs/SOURCE_PORT.md`)
- **Lage:** (gemessen 2026-09-30) `index.φ:3/4` = lokale Pre-CDN-Queues (825/63, gitignored). `:22/27/28/29/31` = Kataloge `verifiziert 0` **url-Blöcke**, aber mit offenen `candidate`-Einträgen: b2find (S1/S2 gelöst), terrapulse (51 dead markiert, Kandidaten erreichbar), esa_geomagnetic (S3 gelöst), archeology (absent/pending markiert), copernicus (CMEMS pending). Die Dateien sind gitignored Working-Tree-Arbeit.
- **Blockade:** Porting (SOURCE_PORT) offen
- **Braucht:** je Katalog die erreichbaren Kandidaten über `docs/SOURCE_PORT.md` portieren; b2find-Fanout-Block bauen (Stationsquelle gemessen).

### Register-Träger — `phi/pipeline/ledger.φ` SSDC
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** neue SSDC-Prozedur (`https://limadou.ssdc.asi.it/query.php`, Termin 2026-10-02)
- **Lage:** (gemessen 2026-09-30) `ledger.φ:6` `ausstehend`; `query.php` CAS-Login, Sotgiu „wait a few weeks".
- **Blockade:** Prozedur nicht live
- **Braucht:** nach Termin 2026-10-02 `archive_search --playwright "https://limadou.ssdc.asi.it/query.php"`.

### `blocked_sources.φ` mycelium-pending-Dispositionen (19)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** je Zeile (`phi/blocked_sources.φ`)
- **Lage:** (gemessen 2026-09-30 via `register_lookup --open`) `:59` BepiColombo, `:85` MESSENGER, `:98` DEMETER, `:346` GOSAT-GW, `:374` DAS2 Iowa (Reader fehlt), `:378` Occultation-DB, `:402` ExoMars TGO, `:406` Akatsuki, `:410` Kaguya, `:414` Chandrayaan-1, `:418` Chang'e MRM, `:422` Tianwen-1 RoPeR, `:426` Phobos 2, `:430` Vega 1/2, `:434` Hayabusa, `:438` Tianwen-1 MoRIC, `:442` Shandong, `:458` Danuri ShadowCam, `:462` CDSE-CCM. Mehrere dieser Arme stehen; es fehlen Compiler-Bin + Sample + Asset (SOURCE_PORT) oder Mountain-`ttl`/`frame`.
- **Blockade:** je Zeile (Arm/Reader/Feder)
- **Braucht:** je Zeile den nächsten Port-Schritt (`docs/SOURCE_PORT.md`); DAS2 Iowa braucht einen echten HAPI-Reader (Vorbild `omni_hro_compiler.rs`), nicht `hapi_draft_fields_csv`.

### Trägerlose Dokumente
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `register_lookup --orphan-docs`-Pass
- **Lage:** (gemessen 2026-09-30) `--orphan-docs` = 0 mit diesem Handover als Träger. Mycelium-eigen getragen: `docs/concepts/exzellenz-konzept.md`, `docs/concepts/kybernetische-astrophysik.md`, `docs/surveys/survey-2026-09-03-orphan-verdicts.md`, `docs/concepts/tools-map.md`.
- **Blockade:** Prosa-Marker (teils echte Messgrenzen)
- **Braucht:** bei nächstem Pass prüfen, ob die Trägerschaft hält; Marker mit echtem nächsten Schritt versehen oder descope-annotieren.

### D5-Orphan-Residuum
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Asset-Producer des Röhren-Feldes (`docs/concepts/zeugnis.md:383`)
- **Lage:** (gemessen 2026-09-30) kein Producer-Bin/Register/Wf
- **Blockade:** Producer fehlt
- **Braucht:** kein Schritt zur Kante — erst ein Bau-Auftrag ändert den Zustand.

### GitHub-Issues — Zensus
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster Issues-Zensus (`gh issue list`)
- **Lage:** (gemessen 2026-09-30) `gh issue` verweigert (Permission-Map); zuletzt bekannt `/tmp/opencode/issues.json` (2026-09-29).
- **Blockade:** `gh issue` nicht erlaubt
- **Braucht:** kanonische Issue-Leseform oder Rolle mit `gh issue`.

### Quellenseitige Waits (`state/zustand/wartend.φ`)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Antwort/Readiness
- **Lage:** (gemessen 2026-09-30) Aufnehmer mycelium: `ssdc-limadou`, `voyager-nssdca`, `mariner10-nssdca`, `viking-nssdca`, `cassini-trk`, `juno-jplnav`, `superdarn-af68c4f1`, `emodnet-hfr`, `bepicolombo-more`, `noirlab-gaia-dr4`, `legacy-cdn-ssd`. Keine neue Antwort.
- **Blockade:** Antwort
- **Braucht:** Postfach + Wiedervorlagen beobachten (Trigger feuert → selbes Atom).

### termin-Punkte — Wiedervorlage
- **Status:** termin | **Bindung:** termin:2026-10-02 · 2026-10-19 · 2026-12-02 · 2026-12-03 · 2027-04-01
- **Trigger:** `superdarn-af68c4f1` · `emodnet-hfr` · `noirlab-gaia-dr4` · `europa-clipper` · `bepicolombo-more`
- **Lage:** (gemessen 2026-09-30 via `state/zustand/wartend.φ`) Wiedervorlage, Aufnehmer mycelium.
- **Blockade:** Termin
- **Braucht:** `archive_search --verdict <url>` beim jeweiligen Datum.

## An mountain

Origin: mycelium-folge212 (mountain-211-Block gefaltet; Sender entfernt beim nächsten Pass).

- **`kernel-flatten` `spk_split`-Ordnungsfehler** (Kern): „segment data begins before address … summary arrived after its data" → `curl: (23)` (`ci_manage log 36639779027`). Blockiert Halley/Itokawa + `europa-clipper`.
- **Verbleibende Parser-Arm-Lücken:** `pds3_binary.rs` nur int 1/2/4/8; `pds3_img.rs::byte_order_of` kein `PC_REAL`. (`pds4_binary`/`pds4_fixed_width`/`pds4_fits` laufen jetzt.)
- **USGS-comcat Admission:** Quelle `url https://earthquake.usgs.gov/fdsnws/event/1/query` (Tag `earthquake.usgs.gov`, Asset `usgs_comcat_m45.bin`); `field`/`force_type` + `ttl`/`frame` fehlen — dann setzt Mycelium `url`/`format`/`compiler`.
- **ttl/frame für die drei register-reifen Endpunkte** (ShadowCam `at moon`+`ttl 604800`, ESA PSA TAP, Chang'e MRM `no-cadence`) — dann `url`+`format`.
- **CDSE-CCM Admission:** `https://catalogue.dataspace.copernicus.eu/stac/collections` (200) — STAC-Arm + ttl/frame.
- **DAS2 Iowa (`blocked_sources.φ:374`):** `src/archivar/port.rs:1369` `hapi_draft_fields_csv` ist Register-Draft-Textgenerator, kein Leser; Compiler braucht echten Reader (Vorbild `omni_hro_compiler.rs`).
- **`gosat-cdn` „returned void":** leeres Suchergebnis über alle Splits (Source-Seite) — `GWT3F_L1B` 0 Treffer.
- **`kuprat` Family-Tag:** kein Compiler setzt `tag kuprat`; die vier Kanäle sind als Substance-Witnesses admitiert (`witnesses.φ:120-142`) — kein `kuprat`-Host.
- **KARI KPDS:** kein maschinenlesbarer Endpunkt (0 honored) — descopen mit Befund.

## An river

Origin: mycelium-folge212.

- **`ci-check` cargo-test GPU + `extracts_bold_keywords`:** river-71 meldet geheilt (`first_bold_words` splittet auf non-alphanumerisch); der neue `ci-check 36717357356` @`f4f271277` ist queued — bestätigt beim Lauf-Ende.
- **`te-gate #112`** rot bestätigt; **`#13`** (n=1000 FPR) + **`#43`** (measure-gates) offen; river70 trägt die Membran-FPR auf 128 Trials.

## An future

Origin: mycelium-folge212.

- **`hinet-cdn`:** `hinetwww11.bosai.go.jp` direct 403 (Wayback nur 2013) — **Operator-Wort** für den Proton-Exit (Geo-Umgehung) oder Re-Dispatch bei Readiness.
- **`CDSE_TOKEN`:** steht nicht im `secrets-sync`-Trockenlauf; falls der `cdse-stac-probe`-Asset-Abruf Auth braucht, ist das Setzen ein **Operator-Akt**.
- **ODF-Encounter-Konsument:** `src/archivar/flyby_encounters.rs` (Earth-Flyby-Epochen) gebaut; wartet auf die request-only DSN-Rohdaten (Paper-Autoren, Asmar-Verweis mail 191).
- **`docs/auftrag/auftrag-gic-einreichung.md`** (sensory-folge213) — Future-eigenes Dokument mit 1 offenem Marker, trägerlos; braucht einen öffentlichen Träger in Futures Übergabe oder descope-annotieren.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
