<!--
  title: Handover — Mycelium-Folge 281 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. CI-Queue geheilt (zwei in_progress-Geister gecancelt, 5 superseded ci-gate-Runs; Watchdog-Schritt 2b gebaut); auto-dispatch-422-Audit geschlossen; das2-iowa + wdc-ae manifestiert; Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-09
  sha256: 9aa7c8bb528f6811bad35f12321fbea68e20d33bef543cadf17150b582946866
  status: live
-->
# Handover — Mycelium-Folge 281 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge280.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`.

## Burn: open 0.045 · close 0.037 · cap 0.5 — Grund: CI-Queue-Heilung (Ghost-Cancel + Watchdog 2b) + auto-dispatch-Audit + Ledger + Pass.

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
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-09-mycelium-folge280.md` §Operator-Wort-Register (und folge279) — gefaltet, nicht kopiert | 2026-10-09 | Quelle: mycelium-281.

## Offen — eigen

### CI — `subset` nur am Tip; zwei in_progress-Geister geheilt, Watchdog-Aufnehmer gebaut
- **Status:** wartend | **Bindung:** eigen (CI-Infra)
- **Trigger:** neuer `ci-gate`-`subset`-Lauf am Tip, der nie grün wird (gemessen im Register `state/zustand/ci-gate.φ`)
- **Lage:** (gemessen 2026-10-09 17:2xZ) Die t420-Jam-Ursache war nicht der sub-set-Concurrency-Mechanismus, sondern **zwei hängende `in_progress`-Läufe**: `37892764277` (`ci-gate`-`subset`, SHA `19cbb7fc7`, 11 h `in_progress` → `cargo test --lib`) und `37853161709` (`hips-png-cdn`, 4 Shards `in_progress` seit 2026-10-08T22:23) — beide **gecancelt**; dazu 5 superseded queued `ci-gate`-Runs gecancelt (`37966122239`, `37966055969`, `37965985107`, `37965927703`, `37965663000`). **Watchdog-Schritt 2b gebaut:** `bin/ci_watchdog.sh` liest den Tip via `git ls-remote origin refs/heads/main` und cancelt queued `ci-gate`-Runs mit `head_sha != Tip` (per-SHA-Verdikt bleibt `pending`, nie grün). `bash -n` sauber.
- **Blockade:** keiner.
- **Braucht:** nächste Pass-Runde messen, ob der Tip-`subset` am neuen HEAD grün wird (`state/zustand/ci-gate.φ`); **falls er gemessen hungert** → Weg A (`subset` auf `ubuntu-latest`) dem Operator als neues Wort vorlegen.

### Pipeline — `swpc-efield-cdn` Fenster vs. Retention, Verifikation läuft
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37963071058` Abschluss (SHA `3b69fe3b5`)
- **Lage:** (gemessen 2026-10-09 17:2xZ via `gh api …/runs`) Lauf **queued** (created 16:59Z). Fenster-Heilung `3b69fe3b5` (Default `now−12h → now`, Schedule `47 */6 * * *`) steht; die Quellen-1D-Retention (~12 h) ist kürzer als das alte gestern→heute-Fenster.
- **Blockade:** keiner.
- **Braucht:** `ci_manage view 37963071058`; bei success sha256/Asset in `phi/sources.φ` + `ledger.φ` auf `disponiert`.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` (hips-png-cdn) Abschluss
- **Lage:** (gemessen 2026-10-09 17:2xZ via `gh api`) Lauf **queued** seit 12:45Z; der konkurrierende Alt-Lauf `37853161709` (4 `in_progress`-Shards) wurde in diesem Atom gecancelt. `ledger.φ:110` `ausstehend`. Offen: Run-Ergebnis, CDN-Asset, Sample.
- **Blockade:** keiner.
- **Braucht:** `ci_manage view 37932098229`; bei success `ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo + Release-Body-Lizenz
- **Status:** wartend | **Bindung:** eigen (Manifestation) · auf Mountain-`terms`
- **Trigger:** Mountains `terms`-Vollständigkeit + Format-Verdikt (netloc vs. Quelle)
- **Lage:** (gemessen 2026-10-09) `.github/workflows/sources-repo-licence.yml` ruft `sources_repo_license`; Re-Dispatch `37952375720` success an `e09f372c9`. Parser konsistent (`sources_repo_license.rs:115`). **Riss bleibt:** `license_census` no-terms 827 vs. Generator no-terms 1359 — verschiedene Block-Basen; Mountains `terms`-Verdikt.
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
- **Lage:** (gemessen 2026-10-09) `.github/workflows/substorm-cdn.yml` steht, Lauf `37951374863` success; die 5 Substorm-Blöcke `phi/sources.φ:19663-19699` vollständig. Übrige `gap`-Träger (bc-mpo-more, tracking-doppler, mariner-rst, dmap-map-grid, kaguya-lrs, inpe-big-stac, hi-21cm, cmb-lambda, solar-vso, laic-cssdc, particle-cern, blinkverse-frb) warten auf Mountain-Arm.
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

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
- **Nachtrag 2026-10-08:** die Route `https://vt.superdarn.org/data-download` ist eingeloggt und erreichbar (gemessen). Der **Download-Akt bleibt die Operator-Hand**.
- **Nachtrag 2026-10-09:** mountain-283 meldet Globus-Credentials stehen — **Riss** zum Operator-Wort „warte bis zur glasfase". Das Wort gilt: der Re-Submit bleibt bis Glasfaser vertagt (**LOCK**); kein Maschinen-Akt. `wartend.φ:8` → `superdarn-globus-map`. Transfer-Task `0f2819ca…` **FAILED** `EXPIRED`.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
