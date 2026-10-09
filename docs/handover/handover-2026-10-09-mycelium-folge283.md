<!--
  title: Handover — Mycelium-Folge 283 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. Der Rust-Refresh-Arm ist am ersten Lauf verifiziert (sources-refresh 37970858064 success, data-Snapshot 415a14b; sources-README auf den Rust-Arm gezogen). PETREL19-CDN success (37966130284), der pending-Eintrag geschlossen. Mountains dropped-gate-Verdikt gelandet (canonical_point_key = kurzer Namenskopf, Baseline neu gezogen); die zwei tools-build-Fehler der 282 geheilt. Vier Kontraktfragen (USGS-Extract, Keogramm-Form, terms-Granularität, DTM-quantity) durch archive_search --all → Rat → UI/Open-Weight gefahren; Vorlagen registriert, drei ungemittelte Risse benannt.
  class: handover
  date: 2026-10-09
  sha256: 15ca32a297dd8406b189ab9783f6f956e6a49470325e81b3da26125453149b23
  status: live
-->
# Handover — Mycelium-Folge 283 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge282.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`.

## Burn: open 0.0000 · close 0.1086 · cap 0.5 — Grund: der Rust-Refresh-Arm ist am ersten `sources-refresh`-Lauf verifiziert (`37970858064` success, `data`-Snapshot `415a14b`), sources-README auf den Rust-Arm gezogen; PETREL19-CDN success (`37966130284`), `pending`-Eintrag `blocked_sources.φ` geschlossen; dropped-gate-Verdikt (287) gefaltet; **vier Kontraktfragen durch die Kette `archive_search --all` → Rat → UI/Open-Weight gefahren** (5 UI-Seats); `register_lookup --fired/--stale`/`open_points_check` = 0 · deepseek-flash, kein pro/max · Session-Kosten per-session via `session_burn` (Session „Mycelium-Linie starten und Stehenden Pass sch…"); Fenster-Total $0.9884 (23 Sessions).

## Operator-Wort-Register

- „das müsst ihr doch unter euch klären" | 2026-10-09 | Quelle: mycelium-282. **Konsequenz:** die Linien-Zuordnung eines Artefakts (wer das `sources_refresh`-Bin gebaut hat) wird unter den Linien geklärt (Commit-/Register-Spur), nie dem Operator vorgelegt. Kein Consent-Stopp für Bekanntes.
- „welche punkte können mit archive search all, rat, und den frontier ui und openweight chats geklärt werden?" | 2026-10-09 | Quelle: mycelium-283. **Konsequenz:** die vier Kontraktfragen (USGS-`Extract`, Keogramm-Form, `terms`-Granularität, DTM-`quantity`) werden durch die Kette `archive_search --all` → Rat → UI/Open-Weight geführt.
- „ja bitte" | 2026-10-09 | Quelle: mycelium-283. **Konsequenz:** Ausführung der Kette (Recherche + Rat + UI-Runde) freigegeben; Runde gefahren, Ergebnis als Vorlage registriert.
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-09-mycelium-folge281.md` §Operator-Wort-Register (und folge280) — gefaltet, nicht kopiert | 2026-10-09 | Quelle: mycelium-283.

## Offen — eigen

### CI — Tip-`subset`, `hips-png-cdn` und tools-build
- **Status:** wartend | **Bindung:** eigen (CI-Infra)
- **Trigger:** `state/zustand/ci-gate.φ` wird am Tip rot; `hips-png-cdn` `37932098229` endet
- **Lage:** (gemessen 2026-10-09T18:35Z via `ci_manage`) `hips-png-cdn` `37932098229` **in_progress seit 12:45Z** (~5,8 h, SHA `eaf1337fd`). Die zwei tools-build-Fehler der 282 sind **geheilt**: `channels: Vec::new()` steht (`tools/utils/src/bin/volume_builder.rs:367`), `tools-build 37972383719` = success an `588735e39`. Am Tip `81b6db7cc`: `tools-build 37972593811` in_progress, `register-dropped 37972593839` success, `register-coverage 37972593906` success, `ci-gate 37972593903` queued.
- **Blockade:** Single-Runner-Stau.
- **Braucht:** nächste Pass-Runde `ci_manage status`; falls `37932098229` gemessen hungert → cancelnden Watchdog-Lauf abwarten.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` (hips-png-cdn) Abschluss
- **Lage:** (gemessen 2026-10-09 via `ci_manage`) **in_progress** seit 12:45Z; der konkurrierende Alt-Lauf `37853161709` wurde in Mycelium-281 gecancelt. `ledger.φ:110` `ausstehend`.
- **Blockade:** keiner.
- **Braucht:** bei success `ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo + Release-Body-Lizenz
- **Status:** wartend | **Bindung:** eigen (Manifestation) · auf Mountain-`terms`
- **Trigger:** Mountains `terms`-Format-Verdikt (pro Quelle vs. netloc)
- **Lage:** (gemessen 2026-10-09) `.github/workflows/sources-repo-licence.yml` ruft `sources_repo_license`; Parser konsistent (`sources_repo_license.rs:115`). Der Release-Body-Arm steht; `LICENSE` wird nur nach `/tmp/licence` erzeugt, nicht ins Repo geschrieben. **Rat+UI-Vorlage:** Speicherung **pro Quelle** (Truth), netloc-Rollup **berechnet, nie gespeichert**, inhomogener Host → `mixed`. **UI-Riss (4/5):** die Ableitung `532 = 1359 − 827` subtrahiert über zuvor als *verschieden* deklarierte Populationen — ohne belegte Subset-/Kommensurabilitäts-Relation ist die Differenz undefiniert. Entweder Teilmengen-Beziehung messen (827 ⊆ 1359?) oder beide Zählungen mit Definition führen und die 532 **nicht** als Gap ableiten. UI-Stimmen: alle fünf.
- **Blockade:** Mountains Register-Verdikt; die 532-Differenz ist als **Riss** zu führen (nicht gemittelt, nicht als Gap behauptet), bis die Relatio gemessen ist.
- **Braucht:** Mountain `terms`-Format-Verdikt (pro Quelle) → dann `LICENSE` aus `/tmp/licence` ins Repo verdrahten; Riss-Messung `license_census` vs. `sources_repo_license` (Populationen).

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ:86`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung) · auf mountain
- **Trigger:** Mountains `inpe-big-stac`-Compiler/Arm (`blocked_sources.φ:65`)
- **Lage:** (gemessen 2026-10-09, mountain-282 `55d8bb99c`) 79 Sammlungen; STAC+tiff+netcdf+grib2-Arme stehen; offen ist der Sammlung→Feld-Compiler + Lizenz je Sammlung.
- **Blockade:** Mountains Arm-Compiler (parser-def).
- **Braucht:** Compiler steht → Ernte-Verdrahtung (Workflow/`sources.φ`-Zeilen) durch Mycelium.

### Gegen-Audit Quellen-Delta + Manifestation der neuen Routen
- **Status:** wartend | **Bindung:** eigen (Manifestation) · auf mountains Parser-Arme
- **Trigger:** je Route der Mountain-Arm (`blocked parser-def`)
- **Lage:** (gemessen 2026-10-09) die 5 Substorm-Blöcke `phi/sources.φ:19663-19699` vollständig; übrige `gap`-Träger (bc-mpo-more, tracking-doppler, mariner-rst, dmap-map-grid, kaguya-lrs, inpe-big-stac, hi-21cm, cmb-lambda, solar-vso, laic-cssdc, particle-cern, blinkverse-frb) warten auf Mountain-Arm.
- **Blockade:** je Route der fehlende Mountain-Parser.
- **Braucht:** Mountain-Arm je Delta-Route → dann `url`/`origin`/`compiler`/Tags + Workflow (Mycelium).

### ShadowCam — Format-Arm fehlt (Admission ja)
- **Status:** wartend | **Bindung:** eigen (Bau) · operator (neuer Wire-Slot)
- **Trigger:** Kontrakt-Akt für den Höhen-`quantity`/Wire-Slot; danach Format-Arm
- **Lage:** (gemessen 2026-10-09) `pds.shadowcam.im-ldi.com/derived/` HTTP 200; DTM `.cub` (ISIS) + `_cog.tif` (COG) + PDS4-XML, **kein `.fits`**; `omegaflow::archivar::tiff::parse_tiff` existiert. **Rat+UI-Vorlage:** PDS4-XML als Label (`NAME/UNIT/SCALING_FACTOR/OFFSET/Radius`), Pixel via `parse_tiff` (COG); `val` = Höhe in m relativ zu benanntem `r_ref` (LOLA: `HEIGHT=DN*SF`, `OFFSET=1737400.`, A/B/C=1737.4 km; `lunar radius`), Metadaten reisen im Label. `.cub` nur wo kein COG. **UI-Riss (2/5, Qwen + Duck/Haiku):** ein *offener* Wire-Slot verletzt den strikten 26×f64-Kontrakt; der Slot muss vor Ingestion explizit zugeordnet sein (kein `_ => 0`, kein offener Platzhalter). **Rat-Riss:** ein Höhenraster ist kein 26×f64-Record; `force.rs:19` kennt kein `height`/`length`, kein `QuantityKind`. Bis dahin als Referenzzeile wie das Keogramm führen.
- **Blockade:** der Kontrakt-Akt (neuer `QuantityKind`/Wire-Slot) — Operator/Rat; `parse_tiff`-GeoTIFF-Geo-Key-Fähigkeit + Nodata/Offset ungemessen.
- **Braucht:** Operator/Rat-Verdikt über den Slot → dann `shadowcam_compiler.rs` (PDS4-Label + COG) + `sources.φ`-Zeile.

### PDS-PPI — Zuordnung gemessen, `sources.φ`-Zeile offen
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Register/field/terms)
- **Trigger:** Mountains `field`/`terms`-Messung; dann `sources.φ`-Block
- **Lage:** (gemessen 2026-10-09) `pds_ppi_compiler.rs` (`115cf1657`, EPN-TAP) ist der Enumerator; `.github/workflows/pds-ppi-cdn.yml` steht. Family unbounded, kein Manifest, `pds4_fixed_width` braucht `field`-Zeilen.
- **Blockade:** Mountain-`field`/`terms` je Sammlung.
- **Braucht:** Mountain misst `field`/`terms`/`ttl` → dann `sources.φ`-Block.

### USGS-geomag E-Feld — Reader-Arm fehlt (`blocked_sources.φ:100`)
- **Status:** wartend | **Bindung:** eigen (Erhebung) · mountain (Bau/Kontrakt)
- **Trigger:** Bau des `Extract::GeomagParallel`-Arms (Vorlage steht); danach `sources.φ`-Zeile
- **Lage:** (gemessen 2026-10-09) `geomag.usgs.gov/ws/data/?id=BOU&elements=E-E,E-N&format=json` HTTP 206; zwei parallele Top-Level-Arrays `times[]` + `values[].values[]`. **Rat+UI-Vorlage (2026-10-09):** `Extract::GeomagParallel { times, values, id }`, ein `(Channel,FieldConfig)` je `values[i]`, Name/Unit aus `values[i].id` (`usgs_geomag::COLUMNS`, `usgs_geomag.rs:8/10`); Längen-Divergenz → `Riss` mit beiden Längen als Zeugen, nie Pad/Truncate; generisches `ZipParallel` erst bei 2. Parallelklasse. UI-Runde 5/5 OK. **Riss (ungemittelt):** `times.len == values[i].len` ist nirgends garantiert; `ttl` ungemessen → `pending`.
- **Blockade:** der Arm-Bau (`src/archivar/parse.rs`/`extract.rs:3296`, Consumer-Typ steht).
- **Braucht:** Arm bauen; danach `sources.φ`-Zeile/Workflow (Mycelium) + Mountain `field`/`terms`/`ttl`.

### Keogramm-Quelle als Vision-Asset
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** die konkrete Keogramm-URL-Messung `archive_search --sniff <keogram-url>` (noch offen) → Referenzzeile in `phi/sources.φ`
- **Lage:** (gemessen 2026-10-09) `space.fmi.fi/MIRACLE/ASC/ASC_keograms/…` liefert ABK-Keogramme (`206 image/jpeg`); Wire-Feld-Pfad descoped; `keogram.rs` + `keogram_compiler.rs` stehen. **Rat+UI-Vorlage:** **Hybrid C** — Referenzzeile in `phi/sources.φ` mit `origin` + `format image/jpeg` + sha256 via `archive_search --sniff`, **kein** eigenes Kanon-Register, **kein** Compiler; eigenes Bild-Register erst bei ≥2 Bildklassen (dann Kanon-Akt Operator/Rat + `phi/canon.φ`). UI-Runde 5/5 OK. **Riss (ungemittelt):** eine zweite Kanon-Objektklasse ist nicht existent/nicht gemessen.
- **Blockade:** keiner.
- **Braucht:** Referenzzeile + sha256 (`--sniff`) in `sources.φ`; die CDN-Manifestation der Referenz (Mycelium).

## Rat + UI — Kontraktvorlagen (2026-10-09)

Kette gefahren: `archive_search --all` (4 Fragen; Texte `/tmp/opencode/research-{usgs,keogramm,license,shadowcam}.txt`) → **Rat** (5 Stimmen, flash) → **UI-Runde** (Claude · Duck.ai/Claude-Haiku · DeepSeek Chat · Qwen · DeepSeek V4 Pro via `open-weight-ui`; Tabs danach geschlossen, Lock gelöscht). Nicht geöffnet diese Runde: Z.ai/GLM · Kimi · MiMo · Gemini AI Studio · Mistral · Lumo (Roster-Rest, `pending`).

| Frage | Vorlage (Rat) | UI |
|---|---|---|
| USGS-`Extract` | `GeomagParallel {times,values,id}`, ein `(Channel,FieldConfig)` je `values[i]`, Divergenz → `Riss` | **5/5 OK** |
| Keogramm-Form | Referenzzeile (`origin`+sha256), kein Kanon-Register, kein Compiler | **5/5 OK** |
| `terms`-Basis | pro Quelle speichern; netloc-Rollup berechnet; `mixed` | **4/5 RISS:** `532 = 1359−827` über getrennte Populationen |
| DTM-`quantity` | PDS4-Label + COG-Pixel; Höhe vs. benanntem `r_ref` | **2/5 RISS:** offener Slot verletzt 26×f64 |

**Ungemittelte Risse (stehen, werden nicht geglättet):** (a) die 532-Differenz ohne belegte Subset-Relation; (b) fehlender Höhen-Wire-Slot/`QuantityKind`; (c) zweite Bildklasse nicht existent; (d) `times.len == values[i].len` nirgends garantiert.

## An mountain

Origin: mycelium-283 (2026-10-09) — bittet um die Register-/Tooling-Verdikte, die allein Mountain schreibt:

- **`terms`-Format:** `phi/sources.φ` trägt `terms <SPDX> <url>` je Block. Vorlage: **pro Quelle** speichern (Truth), netloc nur berechnet, inhomogener Host → `mixed`. Bitte das Format-Verdikt (netloc-keyed vs. pro-Quelle). Der Riss `license_census` no-terms **827** vs. `sources_repo_license` **1359** ist ein **Populations-Riss**, kein Mittelwert: Teilmengen-Relation (827 ⊆ 1359?) messen oder beide Zählungen mit Definition führen (`tools/register/src/bin/license_census.rs:221` vs. `sources_repo_license.rs:150`).
- **USGS-geomag `field`/`terms`/`ttl`:** nach dem Bau des `GeomagParallel`-Arms braucht `sources.φ` die Zeile; `ttl` ungemessen → `pending`. Vorlage steht (Rat+UI).
- **DTM-Wire-Slot:** der neue `quantity`/Slot ist ein **Kontrakt-Akt** (Operator/Rat), nicht Mountains Registerzeile; Mountain liefert erst nach dem Slot-Verdikt `field`/`terms`.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
- **Nachtrag 2026-10-08:** die Route `https://vt.superdarn.org/data-download` ist eingeloggt und erreichbar (gemessen). Der **Download-Akt bleibt die Operator-Hand**.
- **Nachtrag 2026-10-09:** mountain-283 meldet Globus-Credentials stehen — **Riss** zum Operator-Wort „warte bis zur glasfase". Das Wort gilt: der Re-Submit bleibt bis Glasfaser vertagt (**LOCK**); kein Maschinen-Akt. `wartend.φ:8` → `superdarn-globus-map`. Transfer-Task `0f2819ca…` **FAILED** `EXPIRED`.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
