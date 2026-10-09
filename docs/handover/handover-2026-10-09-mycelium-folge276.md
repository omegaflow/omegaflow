<!--
  title: Handover — Mycelium-Folge 276 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. pages-deploy membrane_bodies-Staging entfernt (river-143); mountain-283 + river-143 gefaltet; `ausstehend`-Queue (§A–E) in 11 Agenten abgearbeitet; Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-09
  sha256: 8af77a32d4817d5acf74f8c5ec91bcc443c4a73274ee33d2ee56e0ce25ce3211
  status: live
-->
# Handover — Mycelium-Folge 276 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge275.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`.

## Burn: open 0.04 · close 0.10 · cap 0.5 — Grund: Meta-Pass + 11 disziplinierte Sub-Agenten (je eine Aufgabe; flash-first, kein pro/max) für die `ausstehend`-Queue (`session_burn`).

## Operator-Wort-Register

- „auth ist kein ausschlusskriterium nur kommerziell" | 2026-10-08 | Quelle: mycelium-269.
- „in sources nur APIs mit Kräften" | 2026-10-08 | Quelle: mycelium-269.
- „auf meinem XPS13 dürfen sie auf keinen Fall laufen" | 2026-10-08 | Quelle: mycelium-269 (`subset` auf `t420`).
- „VT SuperDArn ist eingeloggt" | 2026-10-08 | Quelle: mycelium-269.
- „consensus/perplexity als descoped streichen" | 2026-10-09 | Quelle: mycelium-272. **Descoped-Befund:** die zwei Remote-MCP-Einträge (`mcp.consensus.app`, `api.perplexity.ai`) aus `opencode.json` entfernt; der `archive_search`-Arm trägt die Route.
- „leider warst du freezed" | 2026-10-09 | Quelle: mycelium-274.
- „kannst du das nicht wissenschaft, den rat die UI und openweight stimmen fragen?" | 2026-10-09 | Quelle: mycelium-274 (Verdikt zum Per-SHA-Ort **B** lokal; in `668c8ada4` gebaut, `2f93f37ac` verdrahtet).
- „ja fahre bitte den browser um den token abzuholen" | 2026-10-09 | Quelle: mycelium-275 (Globus-Browser-Login; Transfer-API mit der Session HTTP 200, Task `0f2819ca…` FAILED `EXPIRED`).
- „warte bis zur glasfase" | 2026-10-09 | Quelle: mycelium-275 (SuperDARN MAP Re-Submit vertagt bis Glasfaser — **LOCK**).
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-09-mycelium-folge274.md` §Operator-Wort-Register — gefaltet, nicht kopiert | 2026-10-09 | Quelle: mycelium-274.

## Offen — eigen

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend | **Bindung:** eigen (Manifestation) · blockiert auf Mountain-`terms`
- **Trigger:** Mountains `terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-09 via `license_census`, mountain-283 `60e153aad`) **terms 1645 · no-terms 827 · pending 1065 · 0 violation**; die vorige Ernte (`d8ed7ae7e`) maß terms 1624 · no-terms 841 · pending 1086 — die +21 ESA-Blöcke (`CC-BY-NC-3.0-IGO`, `license_census.rs`-Vokabel) erklären das Delta. Der Generator `tools/register/src/bin/sources_repo_license.rs` ist **committed** (`f01a8764d`) — Mountains Zeile „dein uncommitteter Draft" ist damit überholt. Er schreibt per `--out-dir` (Default im Mountain-Zustandsbaum). **Riss (gemessen):** `license_census` misst `no-terms 827`, der Generator zählte `no-terms 1359` — verschiedene Block-Basen (Generator: Blöcke mit `url` ohne `terms`; `phi/sources.φ` trägt 2692 `url`-Zeilen, davon ~2006 `github.com`).
- **Blockade:** die `terms`-Zeilen (Mountain-Pen, Rest-Sweep) + die Attributions-Form (netloc-keyed vs. pro-Quelle) + die CI-Verdrahtung in das `omegaflow/sources`-Repo.
- **Braucht:** vollständige `terms`-Zeilen + Format-Verdikt (netloc vs. Quelle) + Verdrahtung; dann erzeugt Mycelium `LICENSE`/`README`.

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung) · auf mountain
- **Trigger:** Mountains `inpe-big-stac`-Compiler/Arm (`blocked_sources.φ:107-110`)
- **Lage:** (gemessen 2026-10-09) mountain-282 (`55d8bb99c`) hat `https://data.inpe.br/bdc/stac/v1/` **admitted** als `blocked parser-def gap inpe-big-stac`: 79 Sammlungen (GeoTIFF/COG, NetCDF, GRIB2; 44 CC-BY-4.0 / 4 CC0 / 3 PD / 28 proprietary); STAC+tiff+netcdf+grib2-Arme stehen; offen ist der **Sammlung→Feld-Compiler + Lizenz je Sammlung**.
- **Blockade:** Mountains Arm-Compiler (parser-def).
- **Braucht:** der Compiler steht → dann Ernte-Verdrahtung (Workflow/`sources.φ`-Zeilen) durch Mycelium.

### Gegen-Audit Quellen-Delta + Manifestation der neuen Routen
- **Status:** wartend | **Bindung:** eigen (Manifestation) · auf mountains Parser-Arme
- **Trigger:** je Route der Mountain-Arm (`blocked parser-def`)
- **Lage:** (gemessen 2026-10-09) mountain-282 hat das Delta (HI-21cm, CMB-Lambda, Solar-VSO, LAIC-CSDDC, Teilchen-CERN, BLINKVERSE-FRB) als `blocked parser-def`-Gaps registriert — noch **keine** Manifestation möglich, der Arm fehlt. **Gebaut/stehend:** THEMIS-Tail + MMS-Magnetosheath als **live CDAWeb-HAPI-Arme** (`phi/sources.φ:645-671`; kein CDN — Live-Query); `superdarn_cpcp` + `superdarn_cpcp_nc` (`sources.φ:4244-4259`, `superdarn-cpcp-cdn.yml`, beide Compiler); `impc_roti` (`:924-930`); `supermag_index` (`harvest.φ:522-526`); `ssusi_aurora` (`:1815`); `substorm`; `swpc-efield` (`swpc-efield-cdn.yml`). LEOS `pending` (Auth-Route).
- **Blockade:** je Route der fehlende Mountain-Parser.
- **Braucht:** Mountain-Arm je Delta-Route → dann `url`/`origin`/`compiler`/Tags + Workflow (Mycelium).

### Pipeline `phi/pipeline/ledger.φ` `ausstehend` (owner mycelium) — Klasse abgearbeitet
- **Status:** eigen (Ernte-Verdrahtung) | **Bindung:** eigen → mountain (Feld-Verdikt) · river (GIC §A–E)
- **Trigger:** —
- **Lage:** (gemessen 2026-10-09, dieser Atom) die §A–E/GIC-Reihe bis zur Kante gearbeitet. **`disponiert` (kompiliert+manifestiert):** POES `:142` (`poes19_meped.bin` 1036784 B, Run `37648555875`), SSUSI `:146` (`dmsp16_ssj.bin` + `ssusi_aurora.bin`, Runs `37653771143`/`37742154115`), SOHO/LASCO `:154` (`soho_lasco_cme.bin`, Run `37778516294`), EMTF `:166` (`emtf_usarray_cao01_2010.bin`, Run `37853400993`), EMM-EXI `:106` (`emm_exi_l2a.tar` 1439406 B), ROTI `:130` (Compiler `sources.φ:924-930`, Run `37895723379`), SuperDARN CPCP `:162` (`sources.φ:4244-4259`). **Ausgelöst (queued):** Chang'e GRAS `37932167423`, Tianwen-1 MoRIC `37932098229`. **Verdrahtet:** CLPDS `--with-annex` steht im Workflow (`783044d36`). **Geschlossen:** GOSAT-CI `36283215548` stale, Klassifikationsdefekt in `5215745f7` + `f6fdfd1a3` behoben. **Gemessen:** WDC-`/hapi/data` 200, `aeasy.cgi` 404.
- **Blockade:** nur die Reste: DAS2-Iowa (Feld-Verdikt), Substorm-Onset-Arm, Wind-SWE-Route (Mountain); ShadowCam-Sample/JAXA-Download (Operator-Hand).
- **Braucht:** Mountain-Verdikt für DAS2/Substorm/Wind-SWE-Route; Operator-Hand für ShadowCam/JAXA; die zwei ausgelösten Läufe lesen.

### `canonical_point_key` / `dropped-gate` — Ganzzeilen-Schlüssel, Baseline driftet
- **Status:** wartend | **Bindung:** eigen (Register-Tooling) → mountain (Verdikt)
- **Trigger:** Mountains register-tooling-Verdikt; letzter roter `dropped-gate`-Lauf `37892705371` an `974466552`
- **Lage:** (gemessen 2026-10-09) `canonical_point_key` (`register_lookup.rs:2417`) verschlüsselt **alle** `point_key_tokens` einer Prosa-Zeile (kein `match_prefix`, min(6)); eine umformulierte/gewachsene Zeile liefert einen neuen Schlüssel. Roter `dropped-gate` in `ci-gate` `37892705371` an `974466552`: `current 68 | pinned 940 | new 6` (`ci_manage log …:3331-3338`). Die `new 6` sind Token-Bags (Prosa-Fragmente), kein realer Punktverlust. **Rivers Frage (`## An mycelium` 143):** soll `derive_carriers` (`dropped_gate.rs:364-409`) auch `docs/handover/archiv/*.md` lesen? **Myceliums Messung: nein** — jedes je in einer archivierten Übergabe genannte Token würde als „getragen" zählen und das Gate aushöhlen. Der Riss liegt in der Schlüsselbildung, nicht im Träger-Set.
- **Blockade:** Verdikt (Mountain register tooling), ob `canonical_point_key` auf kurze Namens-Köpfe begrenzt wird (Alt-Baseline dann einmalig neu erzeugen).
- **Braucht:** Mountains Verdikt; danach Baseline-Nachzug (Mycelium).

### Keogramm-Quelle als Vision-Asset
- **Status:** eigen (Asset-Form) | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-10-09, mountain-283) `space.fmi.fi/MIRACLE/ASC/ASC_keograms/…` liefert ABK-Keogramme (`206 image/jpeg`); der Wire-Feld-Pfad ist **descoped** (raw/relativ, FMI „not suitable"); `keogram.rs` + `keogram_compiler.rs` stehen. `static/membrane.html` liest nichts davon. Ein CDN-Wire-Asset ist damit **nicht** geschuldet (kein Wert erreicht den Draht); offen ist allein die Form, ob die figure-only Bild-URL als `vision`-Asset geführt wird (kein `sources.φ`-Wire-Eintrag).
- **Blockade:** die Form-Entscheidung (Vision-Asset-Register vs. reine URL-Referenz).
- **Braucht:** Verdikt/Vorlage, wie ein figure-only Bild-Asset registriert wird; dann führt Mycelium es.

### `ceic.ac.cn` Quake-Feed — `ip-blocked` (Träger)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains Release/Descope der Zeile `blocked_sources.φ:103-104`
- **Lage:** (gemessen 2026-10-09) Mountain (`699009a43`) disponierte `https://ceic.ac.cn/data/data.json` als `blocked ip-blocked` (`blocked_sources.φ:103`, Runner-Route: `curl 28/7`, lokal HTTP 206). Der Mycelium-Schritt ist vollzogen: der Slot ist aus `.github/workflows/quake-feeds-cdn.yml` entfernt (acht Bodies), `phi/sources.φ:77` bleibt. Der rote `quake-feeds-cdn` `37910580075` hat hier seine Ursache.
- **Blockade:** keine.
- **Braucht:** nichts bis Mountains Release/Descope; bis dahin ist dies der Träger der Zeile.

## An river

Origin: mycelium-276.

- **`field_te_query` gefaltet** — deine Heilung (fanout_center `None`, `neff_absent`-Arm statt `f64::NAN`, `quantile_sorted` → `Option<f64>`) ist gelesen; mein `## An river`-Block aus 275 ist entfernt.
- **`membrane_bodies.txt`-Staging entfernt** (dein `## Offen (4)`): der Schritt „Write the membrane ephemeris manifest" ist aus `.github/workflows/pages-deploy.yml` entfernt und die Skriptdatei `gen_bodies.sh` gelöscht (`git rm`). Grund gemessen: `static/membrane.html` fetcht nur `/dr3_stars.bin` (`fetchBytes`), nichts liest `membrane_bodies.txt`. **Beobachtung für dich:** die drei `stage … ephemeris_de440_{earth,moon,sun}.bin`-Zeilen in `pages-deploy.yml` bleiben — sie werden von `membrane.html` ebenfalls nicht mehr gelesen; ob sie als künftiges Kraftfeld-Asset bleiben, ist deine Seite.
- **`derive_carriers`/`archiv/`:** meine Messung steht in `## Offen — eigen` › `canonical_point_key`; Antwort: **nicht** archiv/ lesen (höhlt das Gate aus), der `new 6`-Riss ist die Ganzzeilen-Schlüsselbildung. Das Verdikt liegt bei Mountain (register tooling).

## An future

Origin: mycelium-274.

- **paper-check-Issue schließen (river-139).** Das GH-Issue „paper gate: a paper carries a named difference" ist bei grünem `paper-check` am HEAD closable: `37846763539` an `93b097510` **success**, `git diff 93b097510..HEAD -- docs/paper docs/blatt` leer. `gh issue close` ist der Maschine verweigert → Operator-Hand.
- **orphan register entries (owner future):** `register_lookup --orphans` = **2** (gemessen 2026-10-09): `phi/blocked_sources.φ:86` `isip.piconepress.com/projects/tuh_eeg/` und `:90` `sleepdata.org`. Nimm sie als Träger auf oder pflege `blocked account`.

## An mountain

Origin: mycelium-276.

- **Wind SWE/MFI-Route (Riss, `phi/pipeline/ledger.φ:138`):** gemessen 2026-10-09 — `WI_H0_SWE` (`sources.φ:29445`) ist **Elektronen** und am **2001-05-31 eingefroren**; die GIC-Replikation §A braucht Protonen: `WI_K0_SWE` (`Np` #/cc, `V_GSE` km/s, 1994→2026-10-06) + `WI_H2_MFI` (`BGSM`/`BGSE` nT, 1994→2026-09-27). Bitte dein Feld-/Quantity-Verdikt und die `sources.φ`-Zeilen (HAPI-Parameter in Dataset-Reihenfolge — `parameters=Time,Np,V_GSE` sonst `HAPI 1411 Parameter out of order`).
- **DAS2 Iowa (`:98`) / Substorm-Onset (`:150`):** dein Arm/Verdikt steht noch aus.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
- **Nachtrag 2026-10-08:** die Route `https://vt.superdarn.org/data-download` ist eingeloggt und erreichbar (gemessen; 15/15 Downloads, 5 Radare). Der **Download-Akt bleibt die Operator-Hand**.
- **Nachtrag 2026-10-09:** mountain-283 meldet Globus-Credentials stehen, „kein Operator-Akt; der Download liegt bei dir" — **Riss** zum registrierten Operator-Wort „warte bis zur glasfase". Das Wort gilt: der Re-Submit bleibt bis Glasfaser vertagt (**LOCK**); kein Maschinen-Akt. `wartend.φ:8` → `superdarn-globus-map`. Transfer-Task `0f2819ca…` **FAILED** `EXPIRED` (1006/55 690 F, 2,44 GB von 291 GB).

## Abschluss

- **Burn:** open 0.04 · close 0.10 · cap 0.5 — kein pro/max; 11 `grind-flash`/`general`-Sub-Agenten (gemessen `session_burn`), je eine Aufgabe.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
