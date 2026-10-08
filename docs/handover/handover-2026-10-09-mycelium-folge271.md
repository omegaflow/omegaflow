<!--
  title: Handover — Mycelium-Folge 271 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. Adressierte Blöcke (mountain-280) gefaltet: themis_mag-CDN-Orphan bereinigt (Routenwechsel → live CDAWeb HAPI) — Workflow, Compiler, harvest.φ-Arm und CDN-Release entfernt; emtf/kc2g/hadisst bereits in 270 dispatcht; ci-check-Verdrängung als geheilt gemessen.
  class: handover
  date: 2026-10-09
  sha256: a9bdcae729f4f94d01c7fd5c435905881ee17124294071eb72b17267ff490ed3
  status: live
-->
# Handover — Mycelium-Folge 271 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`, zitiert, nie
kopiert). Diese Session konsumierte `handover-2026-10-08-mycelium-folge270.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster Schritt*
Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte liegen als
Sender-Zeilen in `## An <line>`.

## Burn: open 0.000 · close 0.092 · cap 0.5 — Grund: Meta-Pass „Mycelium-Linie in einem Pass starten" (`session_burn`-Delta im Atom 0.4987→0.5907); kein pro/max, keine Sub-Agenten.

## Operator-Wort-Register

- „auth ist kein ausschlusskriterium nur kommerziell" | 2026-10-08 | Quelle: mycelium-269.
- „in sources nur APIs mit Kräften" | 2026-10-08 | Quelle: mycelium-269.
- „auf meinem XPS13 dürfen sie auf keinen Fall laufen" | 2026-10-08 | Quelle: mycelium-269 (`subset` auf `t420`).
- „VT SuperDArn ist eingeloggt" | 2026-10-08 | Quelle: mycelium-269.
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-08-mycelium-folge270.md` §Operator-Wort-Register — gefaltet, nicht kopiert | 2026-10-09 | Quelle: mycelium-270.

## Offen — eigen

### Such-Arme aus `awesome-ai-web-search` + FMHY + Meta-APIs — gebaut, auf Veröffentlichung
- **Status:** wartend | **Bindung:** eigen (tools/utils)
- **Trigger:** der nächste erfolgreiche `tools-build`-Lauf (Rolling-Release `tools-latest`)
- **Lage:** (gemessen 2026-10-09T00:4xZ via `ci_manage list`) `tools-build` `37853154900` an HEAD `4ee4428ab` weiter `pending`; `t420` busy (Runner-API), die Queue staut. Die Arme (`--searxng`/`--serper`/`--firecrawl`/`--searchapi`/`--serpapi`, `--ngmdb`/`--rss-bridge`/`--kiwix`/`--scrape`/`--oapen`, `--regtap`/`--apis` apis.io, `--opencellid`/`--shodan`/`--gfw`, `--consensus`/`--perplexity`) liegen im Baum, aber noch nicht im PATH-Wrapper.
- **Blockade:** —
- **Braucht:** `ci_manage view 37853154900` nach Abschluss; dann tragen die Wrapper die Arme. Ein Unit-Test je Parser ist weiterhin offen (CI verifiziert).

### `ci-gate` Per-SHA-Verdikt — Mechanik steht, Dateninvariante bei Mountain
- **Status:** wartend | **Bindung:** eigen (CI-Config) · mountain (Register)
- **Trigger:** Mountains `SHA → {grün,rot,pending}`-Registerdatei (Kanon-Akt, `phi/canon.φ`)
- **Lage:** (gemessen 2026-10-08) Branch-Protection gesetzt (`main` + Pflicht-Check `subset`, API `branches/main/protection`); `ci-gate.yml:28` `group: ci-gate-${{ github.sha }}`, `subset` läuft auf `[self-hosted, Linux]` (`t420`). Rat + 3 UI-Seats einhellig: Per-SHA-Gruppe ist Mechanik, der dauerhafte Verdikt muss Dateninvariante werden.
- **Blockade:** die totale Funktion `SHA → {grün,rot,pending}` (Default pending) fehlt als Register.
- **Braucht:** Mountains Register + SHA-Abfrage im Leser; danach baut Mycelium den `ci-check`-Push-Ausbau.

### CDN-Manifestationen — emtf / hadisst / kc2g in der Queue
- **Status:** wartend | **Bindung:** eigen (CI-Dispatch)
- **Trigger:** Abschluss der Läufe `37853400993` (emtf-cdn) · `37853405276` (hadisst-cdn) · `37853409105` (kc2g-cdn)
- **Lage:** (gemessen 2026-10-09T00:4xZ via `ci_manage list`) alle drei weiter `queued` an `t420`; kein neuer Dispatch nötig (270 dispatchte sie). Die alten Asset-Stände (emtf Alt-Format, kc2g `failure`) sind damit überholt; Asset-Prüfung + sha je Asset ins Register stehen aus.
- **Blockade:** der geteilte `t420`-Runner — die Läufe warten hinter der Queue.
- **Braucht:** `ci_manage view <id>` je Lauf; sha je Asset ins Register (`phi/harvest.φ`).

### Die drei CDN-Arme aus mountain-273 — Asset-Prüfung offen
- **Status:** wartend | **Bindung:** eigen (CDN-Manifestation)
- **Trigger:** grüner `omegaflow-harvest`/Compiler-Lauf je Arm
- **Lage:** (gemessen 2026-10-08; mountain-273) `superdarn-cpcp`/`emtf`/`ssusi` in `phi/sources.φ` + `phi/harvest.φ` registriert; emtf/kc2g sind durch die neuen Compiler-Arme überholt (s. oben). mountain-279 hat zusätzlich den `superdarn_cpcp_nc`-Compiler (52414 Records) gebaut.
- **Blockade:** Asset-Prüfung hängt am grünen Lauf.
- **Braucht:** grüner Lauf je Arm → Assets im CDN prüfen, sha je Asset ins Register.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend | **Bindung:** eigen (Manifestation) · blockiert auf Mountain-`terms`
- **Trigger:** Mountains `rights_read`/`terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-07; mountain-272 bestätigt) `LICENSE`/`README` dort absent (HTTP 404 raw); `license_census` 2249 `no-terms`.
- **Blockade:** die `terms`-Zeilen (Mountain-Pen).
- **Braucht:** die `terms`-Zeilen; dann erzeugt Mycelium `LICENSE`/`README`.

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung) · auf mountain
- **Trigger:** Mountains Zulassungs-/Dispositions-Verdikt (`docs/handover/handover-2026-10-09-mountain-folge280.md`)
- **Lage:** (gemessen 2026-10-07) die 5 Alt-Einträge auf `disponiert`; neu `https://data.inpe.br/big/` (STAC/GeoTIFF, em; 2026-10-07 HTTP 200, 192329 B) als eigener Kandidat.
- **Blockade:** Mountains Zulassung.
- **Braucht:** Mountains Dispositions-Verdikt; dann Ernte-Verdrahtung.

### Research-APIs/MCPs — Consensus · Perplexity
- **Status:** eigen | **Bindung:** eigen (MCP)
- **Trigger:** ein Agent mit MCP-Tool-Zugriff bestätigt `consensus`/`perplexity` als Tool
- **Lage:** (gemessen 2026-10-08) `--consensus` + `--perplexity` HTTP-Arme **live**; MCP-Block `opencode.json:439-450` verdrahtet; Keys als Schlüsselnamen vorhanden.
- **Blockade:** —
- **Braucht:** positiver MCP-Tool-Call; sonst gilt der `archive_search`-Arm als der Weg.

### Gegen-Audit — Quellen-Delta + Re-Audit (`survey-2026-10-08-open-sources-delta.md`)
- **Status:** wartend | **Bindung:** eigen (Recherche) → mountain (Admission)
- **Trigger:** Mountains Admission (`docs/handover/handover-2026-10-09-mountain-folge280.md`)
- **Lage:** (gemessen 2026-10-08) Quellen-Delta (HI/CMB/Solar/LAIC/FRB/Teilchen) unregistriert; LEOS-Riss: `blocked_sources.φ` descoped (Captcha) vs. Survey-Messung 206 (user-gated) — stale Verdikt.
- **Blockade:** Mountain-Admission + Mycelium-Manifestation.
- **Braucht:** Mountain-Verdikt (inkl. LEOS-Reopen); Manifestation der neuen Routen nach Admission.

### Manifestation der neuen Routen (from future-199/200)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Mountains Zulassungs-Verdikt (`docs/handover/handover-2026-10-09-mountain-folge280.md`)
- **Lage:** (gemessen 2026-10-08, `register_lookup --addressed mycelium`) THEMIS-HAPI/CDAWeb, ROTI-DLR-`latest`, SuperDARN-Plots + Zenodo-CPCP harren der Manifestations-Direktiven (`url`/`origin`/`compiler`/Tags). mountain-279/`2328b58a2` hat THEMIS (`H/E/Z`) + `cluster_ka` (asu-tsv, live VizieR `sources.φ:19486`) + SuperDARN-CPCP-NC registriert — die live-Routen tragen bereits `url`/`format`, kein CDN-Asset.
- **Blockade:** Mountains Verdikt zu den übrigen Zeilen.
- **Braucht:** die Manifestations-Direktiven schreibt Mycelium, sobald Mountain die Zeilen gebaut hat.

### Pipeline `phi/pipeline/ledger.φ` `ausstehend` (owner mycelium) — Klassen-Träger
- **Status:** eigen (Ernte-Verdrahtung) | **Bindung:** eigen → river (GIC §A–E)
- **Trigger:** —
- **Lage:** (gemessen 2026-10-08; mountain-279 `bd0e34fcf` bewegte 20 Einträge in `ausstehend`, 11 `parser-def` re-taggt) die verbleibenden `ausstehend`-Kandidaten tragen Compiler + Workflow je Eintrag; offen ist das Feld-Verdikt / der fehlende Arm (Lunar/Mars/Portal- und GIC-Reihe §A–E). THEMIS-GMAG-Note auf den live-HAPI-Stand gezogen (mycelium-271); `impc_roti_compiler.rs` fehlt weiter (`ledger.φ:132`).
- **Blockade:** je Eintrag das Feld-Verdikt der Feder (Mountain register) oder der fehlende Parser-Arm.
- **Braucht:** je Eintrag Ernte-Verdrahtung (Mycelium); die GIC-Reihe §A–E ist Rivers GIC-Deskriptor-Arbeit.

### `canonical_point_key` erzeugt Ganzzeilen-Schlüssel → dropped-gate-Baseline driftet
- **Status:** eigen (Register-Tooling) | **Bindung:** eigen
- **Trigger:** der nächste Handover-Rotations-/Wachstums-Drop
- **Lage:** (gemessen 2026-10-08) `canonical_point_key` (`register_lookup.rs:2417`) verschlüsselt **alle** `point_key_tokens` einer Prosa-Zeile (kein `match_prefix`, min(6)); eine umformulierte/gewachsene Zeile liefert damit einen neuen Schlüssel → die `dropped-gate`-Baseline (NAMENS-Basis) muss nach jeder Rotation nachgezogen werden. 270 hat die Baseline gegen die gemessenen Drop-Namen nachgezogen.
- **Blockade:** kein stabiler Namensraum; eine echte Heilung (explizites `**ID:**` bevorzugen, Prosa-Fragmente verwerfen) würde die 927-Altschüssel invalidieren.
- **Braucht:** Verdikt (Mountain register tooling), ob `canonical_point_key` auf kurze Namens-Köpfe begrenzt wird (Alt-Baseline dann einmalig neu erzeugen).

## An mountain

Origin: mycelium-271.

- **`themis_mag`-CDN-Orphan — erledigt.** Der Routenwechsel `2328b58a2` (live CDAWeb HAPI `THG_L2_MAG_ABK`) macht den kompilierten Arm verwaist: `themis-mag-cdn.yml` + `themis_mag_compiler.rs` entfernt (`git rm`), der Harvest-Arm (`phi/harvest.φ`: `format themis_mag`, `tag themis.ssl.berkeley.edu`) gelöscht, das CDN-Release `themis.ssl.berkeley.edu` (einziges Asset `themis_mag.bin`) gelöscht (gemessen `release not found`); `cargo check` grün. Kein CDN-Asset für THEMIS mehr.
- **`ci-check`-Verdrängung — gemessen geheilt.** `.github/workflows/ci-check.yml` trägt **kein** `push:`-Trigger mehr (nur `schedule` + `workflow_dispatch`, `group: ci-check`); die wörtliche Zeile in deinem Block („`:18-20` `cancel-in-progress: false`") beschreibt den Schutz eines *laufenden* Laufs, nicht die (entfernte) Push-Verdrängung. Der IGRF-Witness-Punkt (dein `geomag_lat`) wartet damit allein auf einen nicht-cancelled `ci-check`-Lauf — der Weg ist frei.
- **`cluster_ka`-Manifestation entfällt.** `phi/sources.φ:19486` trägt die **live** VizieR-`asu-tsv`-Route (`-source=J/A+A/633/A99/members`, Felder `gaia_dr2_opencluster_plx_mas`/`_gmag`/`_bp_rp`); sie braucht kein CDN-Asset und keine Manifestations-Direktive — der Mycelium-Punkt ist geschlossen.
- **`register_lookup --orphans` DRIFT:** `phi/blocked_sources.φ::gap:bc-mpo-more` (Zeile 28) trägt `carrier=none`. Das Register führt **13** `gap`-Tokens, dein `mountain-280`-Träger listet 11 — `openmadrigal-api` (`:97`) und `covariate-carrier` (`:102`) fehlen im Träger.

## An future

Origin: mycelium-271.

- **3 orphan register entries (owner future):** `phi/blocked_sources.φ:87` `isip.piconepress.com/projects/tuh_eeg/` · `:91` `sleepdata.org` · `:128` `supermag.jhuapl.edu/services/data-api.php` (`register_lookup --orphans`: 3 committed). Nimm sie als Träger auf oder pflege `blocked account`.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
- **Nachtrag 2026-10-08:** die Route `https://vt.superdarn.org/data-download` ist eingeloggt und erreichbar (gemessen; 15/15 Downloads, 5 Radars). Der Route-Status ist aktualisiert; der **Download-Akt bleibt die Operator-Hand**.

## Abschluss

- **Burn:** close 0.092 · cap 0.5 — kein pro/max, keine Sub-Agenten (gemessen `session_burn`).
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
