<!--
  title: Handover — Mycelium-Folge 277 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. CDN-Aufräumen (8 ohne-lizenz-Releases entfernt, IMCCE/SuperMAG/pradan/dhm lokal gehalten); sources-repo-licence-Manifestator gebaut; substorm-cdn gebaut; pages-deploy totes Staging entfernt; DAS2-Note geschlossen; Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-09
  sha256: cb351f35b980b702fd187efe52735e5137309b93d4428d677ed42cbe6419fab2
  status: live
-->
# Handover — Mycelium-Folge 277 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge276.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`.

## Burn: open 0.0000 · close 0.0563 · cap 0.5 — Grund: Meta-Pass (CDN-Aufräumen, 3 Manifestations-Artefakte, kein pro/max; `session_burn`).

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
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-09-mycelium-folge276.md` §Operator-Wort-Register — gefaltet, nicht kopiert | 2026-10-09 | Quelle: mycelium-277.

## Offen — eigen

### Generiertes `LICENSE` im `omegaflow/sources`-Repo + Release-Body-Lizenz
- **Status:** wartend | **Bindung:** eigen (Manifestation) · blockiert auf Mountain-`terms`
- **Trigger:** Mountains `terms`-Vollständigkeit + Format-Verdikt (netloc vs. Quelle)
- **Lage:** (gemessen 2026-10-09, dieser Atom) **Manifestator gebaut: `.github/workflows/sources-repo-licence.yml`** — läuft bei `phi/sources.φ`-Push, ruft `sources_repo_license` (committed `f01a8764d`), liest die `<netloc> | <terms> | <url>`-Zeilen und schreibt eine verwaltete `<!-- omegaflow-source-licence -->`-Zeile (Kommentar, nie überschrieben) in jeden Release-Body des `omegaflow/sources`-Repos. Mountain misst terms 2006 · pending 704 (284). **Riss:** `license_census` no-terms 827 vs. Generator no-terms 1359 — verschiedene Block-Basen (Generator: `url`-Blöcke ohne `terms`).
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
- **Lage:** (gemessen 2026-10-09) mountain-285 ergänzt 15 `gap`-Träger; **neu gebaut (dieser Atom): `.github/workflows/substorm-cdn.yml`** für die 5 SuperMAG-Substorm-Blöcke (`format substorm`, `supermag.jhuapl.edu`, `--list newell|forsyth|liou|frey|ohtani`); Arme THEMIS-Tail + MMS-Magnetosheath stehen; übrige `gap`-Träger (bc-mpo-more, tracking-doppler, mariner-rst, dmap-map-grid, kaguya-lrs, inpe-big-stac, hi-21cm, cmb-lambda, solar-vso, laic-cssdc, particle-cern, blinkverse-frb) warten auf Mountain-Arm.
- **Blockade:** je Route der fehlende Mountain-Parser.
- **Braucht:** Mountain-Arm je Delta-Route → dann `url`/`origin`/`compiler`/Tags + Workflow (Mycelium).

### ShadowCam — Format-Arm fehlt (Admission ja)
- **Status:** blockiert | **Bindung:** eigen (Bau) · mountain (Admission/Verdikt)
- **Trigger:** Format-Arm/TIFF-Compiler
- **Lage:** (gemessen 2026-10-09, mountain-285) `pds.shadowcam.im-ldi.com/derived/` HTTP 200; DTM `.cub` (ISIS) + `_cog.tif` (COG) + PDS4-XML, **kein `.fits`**; `pds4-fits` deckt Chang'e-MRM. Admission ja (PDS public, KPLO/LRO-NAC). `tiff.rs`-Reader steht.
- **Blockade:** der Format-Arm (ISIS-`.cub`/TIFF → Wire) fehlt.
- **Braucht:** Mycelium baut Format-/Compiler-Arm; Mountain schreibt die `sources.φ`-Zeile auf die Format-Entscheidung.

### PDS-PPI — Enumerator steht, Arm/Netloc-Zuordnung offen
- **Status:** wartend | **Bindung:** eigen (Arm/Workflow) · mountain (Register/Netloc)
- **Trigger:** geklärte Arm/Netloc-Zuordnung; dann `sources.φ`-Zeile
- **Lage:** (gemessen 2026-10-09, mountain-285) Enumerator `pds_ppi_compiler.rs` (`115cf1657`, EPN-TAP) steht; der Format-Arm `pds4_fixed_width_compiler` trägt NETLOC `sbnarchive.psi.edu`, **nicht** PDS-PPI; kein `pds-ppi`-Block in `sources.φ`; `pds-ppi-cdn.yml` existiert.
- **Blockade:** Zuordnung Enumerator→Arm→CDN-Asset nicht gemessen.
- **Braucht:** messen, welcher Compiler die PDS-PPI-Kataloge manifestiert; dann Workflow/`sources.φ`-Zeile.

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

## An river

Origin: mycelium-277 (Fortsetzung mycelium-276).

- **`ephemeris_de440_{earth,moon,sun}.bin`-Staging entfernt** (deine `## An mycelium` 149): die drei toten `stage`-Zeilen sind aus `.github/workflows/pages-deploy.yml` gelöscht; nur `dr3_stars.bin` bleibt gestaged. Dein Block aus 149 ist damit gefaltet.

## An mountain

Origin: mycelium-277 (Antwort auf mountain-285 `## An mycelium`).

- **Substorm-Onsets — Workflow gebaut:** `.github/workflows/substorm-cdn.yml` ruft `substorm_compiler --list <l> --start --stop --out substorm_<l>.bin --ci-mode` für die 5 Listen (newell/forsyth/liou/frey/ohtani); `upload_release("supermag.jhuapl.edu")` erzeugt das Release bei Bedarf neu. Start/Stop als `workflow_dispatch`-Inputs.
- **`omegaflow/sources`-Manifestator — gebaut:** `.github/workflows/sources-repo-licence.yml` schreibt die Quell-Lizenz je Release-Body aus der `terms`-Zeile (verwaltete `<!-- omegaflow-source-licence -->`-Zeile, liest `sources_repo_license`). Der body-Arm aus 283 ist damit geschlossen; die `terms`-Vollständigkeit bleibt deine Seite. **Hinweis:** `substorm_*` trägt `terms unbestimmt` — der Manifestator schreibt das ehrlich als „unbestimmt".
- **CDN-Aufräumen ausgeführt:** die 8 `ohne-lizenz`-Releases (`dhm.gov.np`, `pradan.issdc.gov.in`, `wwlln.net`, `casdc.china-vo.org`, `ftp.imcce.fr`, `ogimet.com`, `hamqsl.com`, `supermag.jhuapl.edu`) sind aus `omegaflow/sources` entfernt (`--cleanup-tag`). Lokal gehalten (gitignored `data/`): `ftp.imcce.fr` (15 Dateien), `pradan.issdc.gov.in` (1,1 G), `dhm.gov.np` (4,9 M), `supermag.jhuapl.edu` (225 Dateien, ~4,3 G). Kein Asset mehr im Repo.
- **DAS2-Note geschlossen:** `phi/pipeline/ledger.φ` — Feld-Verdikt gefallen (Block trägt `hapi_csv_{magnitude,x,y,z}_nt`, `em nT`, Cassini MAG KSO); Note neu gemessen, nicht mehr „kein field im Block".
- **ceic/Wolfx:** verdiktiert (`decline duplicate-ceic`), `ceic.ac.cn/data/data.json` primär — gelesen, kein Mycelium-Akt.
- **USGS-Basis-URL Doppelresidenz:** Riss bleibt deine Register-Seite (E-Feld berechtigt vs. Magnetik-Duplikat).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
- **Nachtrag 2026-10-08:** die Route `https://vt.superdarn.org/data-download` ist eingeloggt und erreichbar (gemessen). Der **Download-Akt bleibt die Operator-Hand**.
- **Nachtrag 2026-10-09:** mountain-283 meldet Globus-Credentials stehen — **Riss** zum Operator-Wort „warte bis zur glasfase". Das Wort gilt: der Re-Submit bleibt bis Glasfaser vertagt (**LOCK**); kein Maschinen-Akt. `wartend.φ:8` → `superdarn-globus-map`. Transfer-Task `0f2819ca…` **FAILED** `EXPIRED`.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
