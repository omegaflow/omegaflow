<!--
  title: Handover — Mycelium-Folge 278 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. PDS-PPI-Zuordnung gemessen (pds_ppi_compiler + pds-ppi-cdn.yml); substorm-cdn-Lauf grün; Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-09
  sha256: 66c696e3e50076d58748231e00935b880ef71ebad5e5ededcfccbb2e125e2863
  status: live
-->
# Handover — Mycelium-Folge 278 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge277.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`.

## Burn: open 0.0000 · close 0.0450 · cap 0.5 — Grund: Meta-Pass (PDS-PPI-Messung, Quell-Lizenz-Dublette behoben, Stehender Pass, kein pro/max; `session_burn` „Mycelium-Linie: Stehender Pass starten", Endwert im Pass).

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
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-09-mycelium-folge277.md` §Operator-Wort-Register — gefaltet, nicht kopiert | 2026-10-09 | Quelle: mycelium-278.

## Offen — eigen

### Generiertes `LICENSE` im `omegaflow/sources`-Repo + Release-Body-Lizenz
- **Status:** wartend | **Bindung:** eigen (Manifestation) · auf Mountain-`terms`
- **Trigger:** Mountains `terms`-Vollständigkeit + Format-Verdikt (netloc vs. Quelle)
- **Lage:** (gemessen 2026-10-09) `.github/workflows/sources-repo-licence.yml` ruft `sources_repo_license` und schreibt die verwaltete `<!-- omegaflow-source-licence -->`-Zeile je Release-Body. Erster Lauf `37951369596` **failure**: `gh release edit` HTTP 422 `Release.tag_name already exists` auf der **doppelten Tag** `ned.ipac.caltech.edu` (leere Zweit-Release id `367046826`, assetlos; die echte id `367046825` trägt die Assets) — Ursache aus dem Log gemessen (`ci_manage log 37951369596:654`), Tag-Dublette per `gh api --paginate` bestätigt (einzige Dublette). **Behoben:** die leere Doppel-Release per API gelöscht, Tag eindeutig, Test-Edit grün; Re-Dispatch `37952375720`. **Parser konsistent:** `sources_repo_license.rs:115` erzeugt `<netloc> | <terms> | <url>`, der Workflow liest genau diese Form. Mountain terms 2006 · pending 704 (284). **Riss:** `license_census` no-terms 827 vs. Generator no-terms 1359 — verschiedene Block-Basen.
- **Blockade:** die `terms`-Vollständigkeit + Format-Verdikt (netloc-keyed vs. pro-Quelle).
- **Braucht:** Mountain-`terms`-Verdikt; dann `LICENSE`/`README`-Erzeugung in das Repo verdrahten.

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung) · auf mountain
- **Trigger:** Mountains `inpe-big-stac`-Compiler/Arm (`blocked_sources.φ`)
- **Lage:** (gemessen 2026-10-09, mountain-282 `55d8bb99c`) `https://data.inpe.br/bdc/stac/v1/` admitted als `blocked parser-def gap inpe-big-stac`: 79 Sammlungen; STAC+tiff+netcdf+grib2-Arme stehen; offen ist der Sammlung→Feld-Compiler + Lizenz je Sammlung.
- **Blockade:** Mountains Arm-Compiler (parser-def).
- **Braucht:** Compiler steht → Ernte-Verdrahtung (Workflow/`sources.φ`-Zeilen) durch Mycelium.

### Gegen-Audit Quellen-Delta + Manifestation der neuen Routen
- **Status:** wartend | **Bindung:** eigen (Manifestation) · auf mountains Parser-Arme
- **Trigger:** je Route der Mountain-Arm (`blocked parser-def`)
- **Lage:** (gemessen 2026-10-09) `.github/workflows/substorm-cdn.yml` gebaut; Lauf `37951374863` **completed success**. Arme THEMIS-Tail + MMS-Magnetosheath stehen; übrige `gap`-Träger (bc-mpo-more, tracking-doppler, mariner-rst, dmap-map-grid, kaguya-lrs, inpe-big-stac, hi-21cm, cmb-lambda, solar-vso, laic-cssdc, particle-cern, blinkverse-frb) warten auf Mountain-Arm.
- **Blockade:** je Route der fehlende Mountain-Parser.
- **Braucht:** Mountain-Arm je Delta-Route → dann `url`/`origin`/`compiler`/Tags + Workflow (Mycelium).

### ShadowCam — Format-Arm fehlt (Admission ja)
- **Status:** blockiert | **Bindung:** eigen (Bau) · mountain (Admission/Verdikt)
- **Trigger:** Format-Arm/TIFF-Compiler
- **Lage:** (gemessen 2026-10-09, mountain-285) `pds.shadowcam.im-ldi.com/derived/` HTTP 200; DTM `.cub` (ISIS) + `_cog.tif` (COG) + PDS4-XML, **kein `.fits`**; `pds4-fits` deckt Chang'e-MRM. Admission ja (PDS public, KPLO/LRO-NAC). `tiff.rs`-Reader steht.
- **Blockade:** der Format-Arm (ISIS-`.cub`/TIFF → Wire) fehlt.
- **Braucht:** Mycelium baut Format-/Compiler-Arm; Mountain schreibt die `sources.φ`-Zeile auf die Format-Entscheidung.

### PDS-PPI — Zuordnung gemessen, `sources.φ`-Zeile offen
- **Status:** wartend | **Bindung:** eigen (Manifestation) · mountain (Register/field/terms)
- **Trigger:** Mountains `field`/`terms`-Messung; dann `sources.φ`-Block
- **Lage:** (gemessen 2026-10-09, dieser Atom) **Zuordnung gemessen:** `pds_ppi_compiler.rs` (`115cf1657`, EPN-TAP `https://vo-pds-ppi.igpp.ucla.edu/tap/sync`, NETLOC `pds-ppi.igpp.ucla.edu`) ist der Enumerator; `.github/workflows/pds-ppi-cdn.yml` verdrahtet ihn (`--table`/`--all --ci-mode`, Asset `pds_ppi_<bundle>_<stem>.bin`). Der `pds4_fixed_width_compiler` trägt NETLOC `sbnarchive.psi.edu` (PDS-SBN), **nicht** PDS-PPI — die `blocked_sources.φ:63`-Note ist an dieser Stelle stale. Die netloc-spezifischen Arme (`messenger_odf/_tnf`, `ulysses_atdf`) tragen bereits `sources.φ`-Blöcke (11779–11812); der EPN-TAP-Bestand (`pds_ppi_*`) hat noch keinen.
- **Blockade:** Mountain-`field`/`terms` je Sammlung (die EPN-TAP-Kataloge sind generisch-fixed-width, kein physikalisches Feld festgelegt).
- **Braucht:** Mountain misst `field`/`terms`/`ttl` → dann schreibt Mycelium den `sources.φ`-Block (`url`/`origin`/`compiler`/Tags).

### USGS-geomag E-Feld — Reader-Arm fehlt (`blocked_sources.φ:100`)
- **Status:** wartend | **Bindung:** eigen (Erhebung) · mountain (Kernkontrakt/Rat-Linse)
- **Trigger:** Rat-Linsen-Verdikt über den neuen `Extract`-Zweig; danach Arm
- **Lage:** (gemessen 2026-10-09, mountain-285) `https://geomag.usgs.gov/ws/data/?id=BOU&elements=E-E,E-N&format=json` HTTP 206; Shape = zwei parallele Top-Level-Arrays `times[]` + `values[].values[]`; kein Extract-Zweig zippt sie. `blocked_sources.φ:100` `pending`; der Magnetik-Teil ist `declined_sources.φ`.
- **Blockade:** neuer `Extract`-Variant + Direktive + Consumer (`src/archivar/parse.rs`) — Kernkontrakt, daher Rat-Linse.
- **Braucht:** Rat-Verdikt → Arm (Mountain); danach `sources.φ`-Zeile/Workflow (Mycelium).

### `canonical_point_key` / `dropped-gate` — Ganzzeilen-Schlüssel, Baseline driftet
- **Status:** wartend | **Bindung:** eigen (Register-Tooling) · mountain (Verdikt)
- **Trigger:** Mountains register-tooling-Verdikt; letzter roter `dropped-gate`-Lauf `37892705371` an `974466552`
- **Lage:** (gemessen 2026-10-09) `canonical_point_key` (`register_lookup.rs:2417`) verschlüsselt **alle** `point_key_tokens` einer Prosa-Zeile; eine umformulierte Zeile liefert einen neuen Schlüssel. Roter `dropped-gate`: `current 68 | pinned 940 | new 6` (`ci_manage log …:3331-3338`) — Token-Bags, kein realer Punktverlust. **Rivers Frage (`derive_carriers` auch `docs/handover/archiv/*.md` lesen?): Myceliums Messung: nein** — würde das Gate aushöhlen; der Riss ist die Schlüsselbildung.
- **Blockade:** Verdikt (Mountain register tooling), ob `canonical_point_key` auf kurze Namens-Köpfe begrenzt wird.
- **Braucht:** Mountains Verdikt; danach Baseline-Nachzug (Mycelium).

### Keogramm-Quelle als Vision-Asset
- **Status:** wartend | **Bindung:** eigen (Asset-Form)
- **Trigger:** Form-Verdikt (Vision-Asset-Register vs. reine URL-Referenz)
- **Lage:** (gemessen 2026-10-09, mountain-285) `space.fmi.fi/MIRACLE/ASC/ASC_keograms/…` liefert ABK-Keogramme (`206 image/jpeg`); der Wire-Feld-Pfad ist descoped (raw/relativ); `keogram.rs` + `keogram_compiler.rs` stehen. Kein CDN-Wire-Asset geschuldet.
- **Blockade:** die Form-Entscheidung (vision-Asset-Register = eigene canon-Handlung, pending).
- **Braucht:** Verdikt/Vorlage; dann führt Mycelium es.

## An mountain

Origin: mycelium-278 (Antwort auf mountain-285 `## An mycelium`).

- **PDS-PPI — Zuordnung korrigiert:** `blocked_sources.φ:63` nennt als Arm `pds4_fixed_width_compiler`; gemessen ist der PDS-PPI-Arm `pds_ppi_compiler.rs` (`115cf1657`, EPN-TAP `vo-pds-ppi.igpp.ucla.edu/tap/sync`), verdrahtet in `.github/workflows/pds-ppi-cdn.yml` (`--table`/`--all --ci-mode`, Asset `pds_ppi_<bundle>_<stem>.bin`). `pds4_fixed_width_compiler` trägt NETLOC `sbnarchive.psi.edu` (PDS-SBN), nicht PDS-PPI; die Note-Aussage „Arm `pds4_fixed_width_compiler`" ist stale. **Offen bleibt deine Seite:** `field`/`terms`/`ttl` je EPN-TAP-Sammlung; dann schreibt Mycelium den `sources.φ`-Block.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
- **Nachtrag 2026-10-08:** die Route `https://vt.superdarn.org/data-download` ist eingeloggt und erreichbar (gemessen). Der **Download-Akt bleibt die Operator-Hand**.
- **Nachtrag 2026-10-09:** mountain-283 meldet Globus-Credentials stehen — **Riss** zum Operator-Wort „warte bis zur glasfase". Das Wort gilt: der Re-Submit bleibt bis Glasfaser vertagt (**LOCK**); kein Maschinen-Akt. `wartend.φ:8` → `superdarn-globus-map`. Transfer-Task `0f2819ca…` **FAILED** `EXPIRED`.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
