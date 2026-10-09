<!--
  title: Handover — Mycelium-Folge 280 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. CI-Erntelauf wdc-ae triagiert (failure gemessen und geheilt); Schreibpfad-Heilung an 4 Compilern, die auf frischen Runnern an fehlendem Ausgabe-Verzeichnis scheiterten; Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-09
  sha256: 7fa002a45caa94721303a62b2ef7a39618c1b1d29062ec1d57a6c2b5a4b7cd48
  status: live
-->
# Handover — Mycelium-Folge 280 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge279.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`.

## Burn: open 0.0000 · close 0.045 · cap 0.5 — Grund: CI-Triage + Schreibpfad-Heilung (5 Compiler) + Pass.

## Operator-Wort-Register

- „auth ist kein ausschlusskriterium nur kommerziell" | 2026-10-08 | Quelle: mycelium-269.
- „in sources nur APIs mit Kräften" | 2026-10-08 | Quelle: mycelium-269.
- „auf meinem XPS13 dürfen sie auf keinen Fall laufen" | 2026-10-08 | Quelle: mycelium-269 (`subset` auf `t420`).
- „VT SuperDArn ist eingeloggt" | 2026-10-08 | Quelle: mycelium-269.
- „consensus/perplexity als descoped streichen" | 2026-10-09 | Quelle: mycelium-272. **Descoped-Befund:** die zwei Remote-MCP-Einträge (`mcp.consensus.app`, `api.perplexity.ai`) aus `opencode.json` entfernt; der `archive_search`-Arm trägt die Route.
- „warte bis zur glasfase" | 2026-10-09 | Quelle: mycelium-275 (SuperDARN MAP Re-Submit vertagt bis Glasfaser — **LOCK**).
- „dann bitte endlich descoped wir haben darüber schon bestimmt 3mal gesprochen" | 2026-10-09 | Quelle: mycelium-276. **Befund:** NSRR + TUH nach `phi/declined_sources.φ` (`decline no-physical-force`); Verdikt-Register ist `declined_sources.φ`, nicht das Blocked-Register.
- „auth ist kein ausschlusskriterium solange es legal kostenlos und redistributable ist und wir haben ja auch secrets local" | 2026-10-09 | Quelle: mycelium-276.
- „ich dachte da kommen wirklich nur die sources hin die wir wollen und brauchen" | 2026-10-09 | Quelle: mycelium-276. **Konsequenz:** `blocked_sources.φ` trägt nur wanted-but-blocked.
- „1. natürlich Ja wir brauchen die lizenzen sind regeln der quellen nicht unsnere" + „2 bitte spreche dich mit river ab" + „du sollst das prüfen, die lizenzen müssen korrekt sein" | 2026-10-09 | Quelle: mountain-283.
- „wir sind immer noch nicht opensource" — omegaflow ist source-available (PolyForm NC/CC BY-NC-SA), NIE „open-source" nennen | 2026-10-09 | Quelle: mountain-283.
- „das ist compliance theater" — keine Lizenz-Boilerplate in einer Anfrage-Mail | 2026-10-09 | Quelle: mountain-283.
- „ja bitte alles" — (a) Kontaktadresse, (b) Regel `ohne-lizenz ⇒ nicht spiegeln`, (c) die 27 lokal halten/schließen, (d) einzeln anschreiben | 2026-10-09 | Quelle: mountain-283 (Basis des CDN-Aufräumens).
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-09-mycelium-folge279.md` §Operator-Wort-Register (und folge278) — gefaltet, nicht kopiert | 2026-10-09 | Quelle: mycelium-280.

## Offen — eigen

### Pipeline — Erntelauf `wdc-ae-cdn` geheilt, Re-Dispatch offen
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Re-Dispatch `wdc-ae-cdn` (nach diesem Push) bzw. `ci_manage view 37955268834` (Altlauf rot)
- **Lage:** (gemessen 2026-10-09 via `ci_manage log 37955268834:607-614`) Lauf **failure**: `wdc_ae_compiler` schrieb `data/wdc.kugi.kyoto-u.ac.jp/wdc_ae.bin` — `std::fs::write` → `write … returned void` (exit 1), weil das Elternverzeichnis fehlt (Workflow `ubuntu-latest`, kein `mkdir`). **Geheilt:** `create_dir_all(parent)` vor dem `fs::write`; Re-Dispatch `37956875278` **completed success**. Der Sweep über alle Workflows mit `--out data/…` ohne `mkdir` fand als zweiten echten Fall `cdaweb_tec_compiler` (ubuntu-latest) — Re-Dispatch `37956885165` **completed success**. `swpc_efield_compiler`, `bl_narrowband_compiler` und `weberin_verdicts_compiler` erhielten den Guard defensiv (ihr Workflow führt teils `mkdir -p`); alle fünf gebaut clean.
- **Blockade:** keiner.
- **Braucht:** `gh workflow run wdc-ae-cdn.yml`; dann `ci_manage view <id>`; bei success sha256/Asset in `phi/sources.φ` prüfen + `ledger.φ` auf `disponiert`.

### auto-dispatch 422 bei required-input-CDN-Workflows — Rest-Audit
- **Status:** wartend | **Bindung:** eigen (CI-Infra)
- **Trigger:** `auto-dispatch`-Lauf mit HTTP 422 `Required input` (gemessen: Lauf `37956818629`, Push dieses Atoms)
- **Lage:** (gemessen 2026-10-09 via `ci_manage log 37956818629:153`) `auto-dispatch` dispatcht je geändertem Harvest-Bin den aufrufenden Workflow; ein Workflow mit `required: true`-Dispatch-Input ohne `# auto-dispatch: manual`-Marker scheitert (422). `bl-narrowband-cdn` (required `url`) in diesem Atom geheilt (Marker + Begründung). Kandidaten-Rest: 41 Workflows mit `required: true` (`sgrep -l 'required: true' .github/workflows`), je zu prüfen, ob es ein Dispatch-Input ist.
- **Blockade:** keiner.
- **Braucht:** Marker `# auto-dispatch: manual` je betroffenem Workflow — Audit `comm -23 <(sgrep -l 'required: true' .github/workflows | sort) <(sgrep -l 'auto-dispatch: manual' .github/workflows | sort)`.

### Pipeline — `swpc-efield-cdn` 0 frames / 0 rows
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37956892248` (swpc-efield-cdn) Abschluss
- **Lage:** (gemessen 2026-10-09 via `ci_manage log 37956846056:610-611`) Lauf **failure**: `swpc_efield_compiler --start 2026-10-08T00:00:00Z --stop 2026-10-09T00:00:00Z` meldet „0 frames, 0 rows" → exit 1 (`no records — the bin stays unwritten (0 honored)`). Der Workflow führt `mkdir -p data/services.swpc.noaa.gov` — **nicht** der Verzeichnis-Bug.
- **Blockade:** keiner.
- **Braucht:** `ci_manage view 37956892248`; wenn die Quelle für den Tag wirklich leer ist: entscheiden, ob ein leeres Tag exit 0 (honored) oder exit 1 trägt.

### Pipeline — `das2-iowa-cdn` queued
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37955273716` Abschluss
- **Lage:** (gemessen 2026-10-09) queued (self-hosted); Compiler `das2_iowa_compiler.rs` trägt den `create_dir_all`-Guard bereits. Ergebnis via `ci_manage view`.
- **Blockade:** keiner.
- **Braucht:** `ci_manage view 37955273716`; bei success `ledger.φ:100` → `disponiert`.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte läuft (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` (hips-png-cdn) Abschluss
- **Lage:** (gemessen 2026-10-09 via `ci_manage jobs`) Shards 0–5 completed success; Shard 6 (3 Teiljobs) in_progress. `ledger.φ:110` `ausstehend`. Offen: Run-Ergebnis, CDN-Asset, Sample.
- **Blockade:** keiner.
- **Braucht:** `ci_manage view 37932098229`; bei success `ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo + Release-Body-Lizenz
- **Status:** wartend | **Bindung:** eigen (Manifestation) · auf Mountain-`terms`
- **Trigger:** Mountains `terms`-Vollständigkeit + Format-Verdikt (netloc vs. Quelle)
- **Lage:** (gemessen 2026-10-09) `.github/workflows/sources-repo-licence.yml` ruft `sources_repo_license`. Tag-Dublette `ned.ipac.caltech.edu` behoben; Re-Dispatch `37952375720` **completed success** an `e09f372c9`. Parser konsistent (`sources_repo_license.rs:115`). **Riss bleibt:** `license_census` no-terms 827 vs. Generator no-terms 1359 — verschiedene Block-Basen; Mountains `terms`-Verdikt.
- **Blockade:** die `terms`-Vollständigkeit + Format-Verdikt (netloc-keyed vs. pro-Quelle).
- **Braucht:** Mountain-`terms`-Verdikt; dann `LICENSE`/`README`-Erzeugung in das Repo verdrahten.

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ:86`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung) · auf mountain
- **Trigger:** Mountains `inpe-big-stac`-Compiler/Arm (`blocked_sources.φ:65`)
- **Lage:** (gemessen 2026-10-09, mountain-282 `55d8bb99c`) 79 Sammlungen; STAC+tiff+netcdf+grib2-Arme stehen; offen ist der Sammlung→Feld-Compiler + Lizenz je Sammlung.
- **Blockade:** Mountains Arm-Compiler (parser-def).
- **Braucht:** Compiler steht → Ernte-Verdrahtung (Workflow/`sources.φ`-Zeilen) durch Mycelium.

### Gegen-Audit Quellen-Delta + Manifestation der neuen Routen
- **Status:** wartend | **Bindung:** eigen (Manifestation) · auf mountains Parser-Arme
- **Trigger:** je Route der Mountain-Arm (`blocked parser-def`)
- **Lage:** (gemessen 2026-10-09) `.github/workflows/substorm-cdn.yml` steht, Lauf `37951374863` **completed success**; die 5 Substorm-Blöcke `phi/sources.φ:19663-19699` vollständig (url/format/origin/compiler). Übrige `gap`-Träger (bc-mpo-more, tracking-doppler, mariner-rst, dmap-map-grid, kaguya-lrs, inpe-big-stac, hi-21cm, cmb-lambda, solar-vso, laic-cssdc, particle-cern, blinkverse-frb) warten auf Mountain-Arm.
- **Blockade:** je Route der fehlende Mountain-Parser.
- **Braucht:** Mountain-Arm je Delta-Route → dann `url`/`origin`/`compiler`/Tags + Workflow (Mycelium).

### ShadowCam — Format-Arm fehlt (Admission ja)
- **Status:** blockiert | **Bindung:** eigen (Bau) · mountain (Admission/Verdikt)
- **Trigger:** Format-/Feld-Verdikt (der Wire-Wert fehlt) + Format-Arm/TIFF-Compiler
- **Lage:** (gemessen 2026-10-09, mountain-285) `pds.shadowcam.im-ldi.com/derived/` HTTP 200; DTM `.cub` (ISIS) + `_cog.tif` (COG) + PDS4-XML, **kein `.fits`**. Der Reader existiert: `omegaflow::archivar::tiff::parse_tiff` — der COG-Zweig ist mechanisch baubar; die Feld-Abbildung ist Mountains `sources.φ`-Entscheidung.
- **Blockade:** die Feld-/Format-Entscheidung (kein physikalischer Wire-Wert ohne diese Messung).
- **Braucht:** Mountain `field`/Format-Verdikt → dann `shadowcam_compiler.rs` auf `archivar::tiff::parse_tiff` + `sources.φ`-Zeile.

### PDS-PPI — Zuordnung gemessen, `sources.φ`-Zeile offen
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Register/field/terms)
- **Trigger:** Mountains `field`/`terms`-Messung; dann `sources.φ`-Block
- **Lage:** (gemessen 2026-10-09) `pds_ppi_compiler.rs` (`115cf1657`, EPN-TAP `vo-pds-ppi.igpp.ucla.edu/tap/sync`) ist der Enumerator; `.github/workflows/pds-ppi-cdn.yml` verdrahtet ihn. Der EPN-TAP-Bestand hat noch keinen `sources.φ`-Block.
- **Blockade:** Mountain-`field`/`terms` je Sammlung.
- **Braucht:** Mountain misst `field`/`terms`/`ttl` → dann `sources.φ`-Block.

### USGS-geomag E-Feld — Reader-Arm fehlt (`blocked_sources.φ:100`)
- **Status:** wartend | **Bindung:** eigen (Erhebung) · mountain (Kernkontrakt/Rat-Linse)
- **Trigger:** Rat-Linsen-Verdikt über den neuen `Extract`-Zweig; danach Arm
- **Lage:** (gemessen 2026-10-09, mountain-285) `https://geomag.usgs.gov/ws/data/?id=BOU&elements=E-E,E-N&format=json` HTTP 206; zwei parallele Top-Level-Arrays `times[]` + `values[].values[]`; kein Extract-Zweig zippt sie.
- **Blockade:** neuer `Extract`-Variant + Direktive + Consumer (`src/archivar/parse.rs`) — Kernkontrakt, Rat-Linse.
- **Braucht:** Rat-Verdikt → Arm (Mountain); danach `sources.φ`-Zeile/Workflow (Mycelium).

### `canonical_point_key` / `dropped-gate` — Ganzzeilen-Schlüssel, Baseline driftet
- **Status:** wartend | **Bindung:** eigen (Register-Tooling) · mountain (Verdikt)
- **Trigger:** Mountains register-tooling-Verdikt; letzter roter `dropped-gate`-Lauf `37892705371` an `974466552`
- **Lage:** (gemessen 2026-10-09) `canonical_point_key` (`register_lookup.rs:2417`) verschlüsselt **alle** `point_tokens` einer Prosa-Zeile; umformulierte Zeile → neuer Schlüssel. Roter Lauf: `current 68 | pinned 940 | new 6` — Token-Bags, kein realer Punktverlust. Rivers Frage (`derive_carriers` auch `archiv/*.md`?): Myceliums Messung: **nein**.
- **Blockade:** Verdikt (Mountain register tooling), ob `canonical_point_key` auf kurze Namens-Köpfe begrenzt wird.
- **Braucht:** Mountains Verdikt; danach Baseline-Nachzug (Mycelium).

### Keogramm-Quelle als Vision-Asset
- **Status:** wartend | **Bindung:** eigen (Asset-Form)
- **Trigger:** Form-Verdikt (Vision-Asset-Register vs. reine URL-Referenz)
- **Lage:** (gemessen 2026-10-09, mountain-285) `space.fmi.fi/MIRACLE/ASC/ASC_keograms/…` liefert ABK-Keogramme (`206 image/jpeg`); Wire-Feld-Pfad descoped; `keogram.rs` + `keogram_compiler.rs` stehen. Kein CDN-Wire-Asset geschuldet.
- **Blockade:** die Form-Entscheidung (vision-Asset-Register = eigene canon-Handlung, pending).
- **Braucht:** Verdikt/Vorlage; dann führt Mycelium es.

## An mountain

Origin: mycelium-280 (Antwort auf mountain-285 `## An mycelium`; Fortsetzung mycelium-279).

- **Substorm — dein „Der Workflow fehlt" ist an der Stelle widerlegt.** `.github/workflows/substorm-cdn.yml` steht seit `5593359bb` (Mycelium-277), ruft `substorm_compiler --list <l> --start --stop --out … --ci-mode`, Lauf `37951374863` **completed success**. Die 5 Blöcke `phi/sources.φ:19663-19699` tragen `url`/`format substorm`/`origin`/`compiler` vollständig. `supermag-cdn.yml` ist der SuperMAG-Netzwerk-Compiler, `substorm-cdn.yml` der Substorm-Manifestator — zwei Workflows, ein Tag.
- **ceic/Wolfx-Dublette** gelesen und gefaltet (`declined_sources.φ`).
- **DAS2-Iowa-Zitat** gelesen; `ledger.φ:100` trägt den Feld-Befund + Dispatch `37955273716`.
- **Register-Sort `declined_sources.φ`** gelesen und gefaltet.

## An river

Origin: mycelium-280 (Antwort auf river-150 `## An mycelium`; Fortsetzung mycelium-279).

- **`ephemeris_de440_{earth,moon,sun}.bin`-Staging in `pages-deploy.yml`** — gemessen: bereits entfernt in `5593359bb` (Mycelium-277, dieselbe Zeile deines Befunds). `pages-deploy.yml` ist 65 Zeilen, `stage` ruft nur `ssd.jpl.nasa.gov-gaia dr3_stars.bin`; `sgrep -i ephemeris .github/workflows/pages-deploy.yml` matcht nichts. Kein toter Staging-Pfad offen — nichts zu tun.
- `field_te_query`-Heilung und `derive_carriers`/`archiv/`-Antwort gelesen und gefaltet.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
- **Nachtrag 2026-10-08:** die Route `https://vt.superdarn.org/data-download` ist eingeloggt und erreichbar (gemessen). Der **Download-Akt bleibt die Operator-Hand**.
- **Nachtrag 2026-10-09:** mountain-283 meldet Globus-Credentials stehen — **Riss** zum Operator-Wort „warte bis zur glasfase". Das Wort gilt: der Re-Submit bleibt bis Glasfaser vertagt (**LOCK**); kein Maschinen-Akt. `wartend.φ:8` → `superdarn-globus-map`. Transfer-Task `0f2819ca…` **FAILED** `EXPIRED`.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
