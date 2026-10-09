<!--
  title: Handover — Mycelium-Folge 273 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. Adressierten Block mountain-281 gefaltet; SuperMAG-Index-Endpoint gemessen und archive_search-Mess-Arm gebaut; hdf5-real-granule-401 gemessen.
  class: handover
  date: 2026-10-09
  sha256: 1549162fd04b606e35f5c7e4f5f6f15d025945e042e1a090c040ce86ea088939
  status: live
-->
# Handover — Mycelium-Folge 273 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge272.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`; der Block `mountain-281` ist in
diesem Atom gefaltet.

## Burn: open 0.0026 · close 0.0706 · cap 0.5 — Grund: Meta-Pass „Mycelium-Linie in einem Pass starten" (`session_burn`, Session-Figur; kein pro/max, keine Sub-Agenten).

## Operator-Wort-Register

- „auth ist kein ausschlusskriterium nur kommerziell" | 2026-10-08 | Quelle: mycelium-269.
- „in sources nur APIs mit Kräften" | 2026-10-08 | Quelle: mycelium-269.
- „auf meinem XPS13 dürfen sie auf keinen Fall laufen" | 2026-10-08 | Quelle: mycelium-269 (`subset` auf `t420`).
- „VT SuperDArn ist eingeloggt" | 2026-10-08 | Quelle: mycelium-269.
- „consensus/perplexity als descoped streichen" | 2026-10-09 | Quelle: mycelium-272. **Descoped-Befund:** die zwei Remote-MCP-Einträge (`mcp.consensus.app`, `api.perplexity.ai`) aus `opencode.json` entfernt; der `archive_search`-Arm trägt die Route.
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-09-mycelium-folge272.md` §Operator-Wort-Register — gefaltet, nicht kopiert | 2026-10-09 | Quelle: mycelium-272.

## Offen — eigen

### HadISST-CDN — Compiler-dim-Toleranz geheilt, Lauf in der Queue
- **Status:** wartend | **Bindung:** eigen (CI-Dispatch)
- **Trigger:** grüner `hadisst-cdn.yml`-Lauf am Fix-HEAD
- **Lage:** (gemessen 2026-10-09 via `ci_manage log 37853405276`) Lauf `37853405276` an `4ee4428ab` **failure**: `hadisst_compiler: …HadISST_sst.nc.gz: sst dims ["time","latitude","longitude"] are not ["time","lat","lon"]`. Toleranz in `tools/harvest/src/bin/hadisst_compiler.rs` geheilt (`matches!(…, "lat"|"latitude")`), `cargo check` grün, `hadisst-cdn.yml` neu dispatcht (`37891967856`/`37891979990`, `cc9f4cf7c`), beide **queued** (t420-Stau). CDN-Release `metoffice.gov.uk` HTTP 404 (gemessen via `gh api …/releases/tags/metoffice.gov.uk`).
- **Blockade:** der geteilte `t420`-Runner.
- **Braucht:** `ci_manage view <id>` nach Abschluss → `gh api repos/omegaflow/sources/releases/tags/metoffice.gov.uk`; Asset-sha via `archive_search --sniff`.

### `ci-gate` Per-SHA-Verdikt — Mechanik steht, Dateninvariante bei Mountain
- **Status:** wartend | **Bindung:** eigen (CI-Config) · mountain (Register)
- **Trigger:** Operator/Rat-Wort zum **Ort** des Registers (getracktes Register vs. lokaler Zustands-Speicher)
- **Lage:** (gemessen 2026-10-08; mountain-281 gefaltet 2026-10-09) Branch-Protection gesetzt (`main` + Pflicht-Check `subset`, API `branches/main/protection`); `ci-gate.yml:28` `group: ci-gate-${{ github.sha }}`, `subset` läuft auf `[self-hosted, Linux]` (`t420`). Rat + 3 UI-Seats einhellig: Per-SHA-Gruppe ist Mechanik, der dauerhafte Verdikt muss Dateninvariante werden. Mountain-281: der Verdikt ist die totale Funktion `SHA → {grün,rot,pending}`, Default `pending`.
- **Blockade:** der **Ort** des Registers — Operator/Rat-Wort.
- **Braucht:** Operator/Rat-Wort; danach baut Mountain Register + SHA-Abfrage, Mycelium den `ci-check`-Push-Ausbau.

### `ci-gate` clippy-Suite — in Mycelium-272 geheilt, CI-Verifikation hängt in der Queue
- **Status:** wartend | **Bindung:** eigen (CI)
- **Trigger:** grüner `ci-gate`-Lauf an `fa7b1144f` (oder `19cbb7fc7`/`974466552`)
- **Lage:** (gemessen 2026-10-09) der Lauf an `30e84819f` war **failure** — clippy `-D warnings`, 10 Lints in `src/archivar`; in Mycelium-272 geheilt (`974466552`, `ChannelQuery`-Kontextstruktur + positive Vergleiche + `?` + `impl Default for ReceiverAperture`). Die `ci-gate`-Läufe an `974466552`/`19cbb7fc7`/`fa7b1144f` (`37892705371`/`37892764277`/`37895712548`) stehen **queued** (t420 tief gestaut); `ci_manage view` zum Pass-Zeitpunkt bestätigt `queued`.
- **Blockade:** der geteilte `t420`-Runner — kein abgeschlossener Lauf am Fix-HEAD.
- **Braucht:** `ci_manage view <id>` am Fix-HEAD nach Abschluss; bei rot die benannte Stelle.

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend | **Bindung:** eigen (Manifestation) · blockiert auf Mountain-`terms`
- **Trigger:** Mountains `terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-09; mountain-folge281 meldet 1086 neue `terms PD <url>`-Zeilen, `license_census` terms 1257 · no-terms 1164; Rest-Sweep folgt) `LICENSE`/`README` im `omegaflow/sources`-Repo absent (HTTP 404 raw, 2026-10-07). Ein Generator-Bin existiert noch nicht (`license_census` misst nur).
- **Blockade:** die `terms`-Zeilen (Mountain-Pen, Rest-Sweep).
- **Braucht:** die vollständigen `terms`-Zeilen + ein `sources_repo_license`-Generator-Bin (Manifestation); dann erzeugt Mycelium `LICENSE`/`README`.

### Pipeline — INPE-BIG-Kandidat (`phi/pipeline/ledger.φ`)
- **Status:** wartend | **Bindung:** eigen (Ernte-Verdrahtung) · auf mountain
- **Trigger:** Mountains Zulassungs-/Dispositions-Verdikt (`docs/handover/archiv/handover-2026-10-09-mountain-folge280.md`)
- **Lage:** (gemessen 2026-10-07) die 5 Alt-Einträge auf `disponiert`; neu `https://data.inpe.br/big/` (STAC/GeoTIFF, em; 2026-10-07 HTTP 200, 192329 B) als eigener Kandidat.
- **Blockade:** Mountains Zulassung.
- **Braucht:** Mountains Dispositions-Verdikt; dann Ernte-Verdrahtung.

### Gegen-Audit — Quellen-Delta + Re-Audit (`survey-2026-10-08-open-sources-delta.md`)
- **Status:** wartend | **Bindung:** eigen (Recherche) → mountain (Admission)
- **Trigger:** Mountains Admission (`docs/handover/archiv/handover-2026-10-09-mountain-folge280.md`)
- **Lage:** (gemessen 2026-10-08) Quellen-Delta (HI/CMB/Solar/LAIC/FRB/Teilchen) unregistriert; LEOS-Riss: `blocked_sources.φ` descoped (Captcha) vs. Survey-Messung 206 (user-gated) — stale Verdikt.
- **Blockade:** Mountain-Admission + Mycelium-Manifestation.
- **Braucht:** Mountain-Verdikt (inkl. LEOS-Reopen); Manifestation der neuen Routen nach Admission.

### Manifestation der neuen Routen (from future-199/200)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** Mountains Zulassungs-Verdikt (`docs/handover/archiv/handover-2026-10-09-mountain-folge280.md`)
- **Lage:** (gemessen 2026-10-08, `register_lookup --addressed mycelium`) THEMIS-HAPI/CDAWeb, ROTI-DLR-`latest`, SuperDARN-Plots + Zenodo-CPCP harren der Manifestations-Direktiven (`url`/`origin`/`compiler`/Tags). mountain-279/`2328b58a2` hat THEMIS (`H/E/Z`) + `cluster_ka` (asu-tsv, live VizieR `sources.φ:19486`) + SuperDARN-CPCP-NC registriert — die live-Routen tragen bereits `url`/`format`, kein CDN-Asset.
- **Blockade:** Mountains Verdikt zu den übrigen Zeilen.
- **Braucht:** die Manifestations-Direktiven schreibt Mycelium, sobald Mountain die Zeilen gebaut hat.

### Pipeline `phi/pipeline/ledger.φ` `ausstehend` (owner mycelium) — Klassen-Träger
- **Status:** eigen (Ernte-Verdrahtung) | **Bindung:** eigen → river (GIC §A–E)
- **Trigger:** —
- **Lage:** (gemessen 2026-10-09; mountain-279 `bd0e34fcf` bewegte 20 Einträge in `ausstehend`, 11 `parser-def` re-taggt) die verbleibenden `ausstehend`-Kandidaten tragen Compiler + Workflow je Eintrag; offen ist das Feld-Verdikt / der fehlende Arm (Lunar/Mars/Portal- und GIC-Reihe §A–E). `impc_roti`: der Compiler steht (`sources.φ:1642-1648`, `bd0e34fcf`) — die Ledger-Notiz `ledger.φ:132` „impc_roti_compiler.rs fehlt" ist **stale**; die Manifestation fehlte (HTTP 404), in Mycelium-272 verdrahtet (`impc-roti-cdn.yml` + harvest.φ-Arm, dispatcht).
- **Blockade:** je Eintrag das Feld-Verdikt der Feder (Mountain register) oder der fehlende Parser-Arm.
- **Braucht:** je Eintrag Ernte-Verdrahtung (Mycelium); die GIC-Reihe §A–E ist Rivers GIC-Deskriptor-Arbeit.

### `canonical_point_key` erzeugt Ganzzeilen-Schlüssel → dropped-gate-Baseline driftet
- **Status:** eigen (Register-Tooling) | **Bindung:** eigen
- **Trigger:** der nächste Handover-Rotations-/Wachstums-Drop
- **Lage:** (gemessen 2026-10-08; river-139 bestätigt) `canonical_point_key` (`register_lookup.rs:2417`) verschlüsselt **alle** `point_key_tokens` einer Prosa-Zeile (kein `match_prefix`, min(6)); eine umformulierte/gewachsene Zeile liefert einen neuen Schlüssel → die `dropped-gate`-Baseline (NAMENS-Basis) muss nach jeder Rotation nachgezogen werden. 270 hat die Baseline gegen die gemessenen Drop-Namen nachgezogen.
- **Blockade:** kein stabiler Namensraum; eine echte Heilung (explizites `**ID:**` bevorzugen, Prosa-Fragmente verwerfen) würde die 927-Altschüssel invalidieren.
- **Braucht:** Verdikt (Mountain register tooling), ob `canonical_point_key` auf kurze Namens-Köpfe begrenzt wird (Alt-Baseline dann einmalig neu erzeugen) und ob `derive_carriers` auch `archiv/` liest.

### SuperMAG SME/SMU/SML-Index — Mess-Arm steht, Compiler-Arm offen (`blocked_sources.φ:117`)
- **Status:** eigen (Ernte-Verdrahtung) | **Bindung:** eigen · mountain (Datenkontrakt)
- **Trigger:** Mountain-Register-Entscheid zum Index-Träger (neue `COMP_SMG`-Codes vs. eigener Format-Name)
- **Lage:** (gemessen 2026-10-09) Der Index-Endpoint ist gemessen: `https://supermag.jhuapl.edu/services/indices.php?logon=…&start=…&end=…&fmt=json&indices=all` liefert ein JSON-Array `[{tval, SME, SML, SMU, …}]` (Testlauf 2024-05-10, 10 Zeilen; Konto `omegaflow` live). Der `archive_search`-Mess-Arm ist gebaut (`tools/utils/src/bin/archive_search/supermag.rs`: `index=<all>`-Token, `parse_indices`/`indices_url`, 4 Tests; `cargo build -p omegaflow-utils --bin archive_search` grün) — `archive_search --supermag "index=all start=… end=…"` liest SME/SML/SMU. Der Magnetik-Arm `supermag_1m` steht (`phi/sources.φ:18909-18920`, `tools/harvest/src/bin/supermag_compiler.rs`). Offen: der Compiler-Index-Arm + `quantity`-Zeilen in `sources.φ`; Index = quantity, nie em (Rat + Science 2026-10-08). future-204.
- **Blockade:** die Register-Form des Index-Trägers — neue `COMP_SMG`-Codes 7-9 im `supermag_1m`-Kontrakt (berührt `src/archivar/geo.rs::COMP_SMG_MAX`, `comp_max`, `geo_series_component_name`, Tests `tests.rs:11078`) oder ein eigener `supermag_index`-Format-Name. Datenkontrakt = Mountain.
- **Braucht:** Mountain-Register-Wort; dann `supermag_compiler.rs` um den Index-Arm erweitern und die `quantity`-Zeilen registrieren.

## An mountain

Origin: mycelium-273.

- **SuperMAG-Index-Träger-Entscheid.** Der Index-Endpoint ist gemessen (`services/indices.php?...&indices=all` → `[{tval,SME,SML,SMU,…}]`). Für den Compiler-Arm ist die Register-Form des Trägers offen: neue `COMP_SMG`-Codes 7-9 im `supermag_1m`-Kontrakt (dann `COMP_SMG_MAX`, `comp_max`, `geo_series_component_name` + Test `tests.rs:11078`) oder ein eigener `supermag_index`-Format-Name. Datenkontrakt = dein Pen.
- **HadISST-Compiler geheilt (aus mycelium-272, gefaltet).** `hadisst_compiler.rs` prüfte die sst-Dim-Namen exakt `["time","lat","lon"]`; die Met-Office-Datei trägt CF-Namen `latitude`/`longitude`. Toleranz ergänzt, `cargo check` grün.
- **`ci-gate` clippy — geheilt in `974466552`.** Die 9 Lints in `src/archivar` plus `types.rs:313` (River) sind geheilt; die vier 8/7-Builder nehmen `ChannelQuery { … }`. Bitte gegenlesen.
- **`impc_roti`-Manifestation verdrahtet.** `harvest.φ`-Arm + `.github/workflows/impc-roti-cdn.yml` ergänzt (dispatcht `37895723379`). Die Ledger-Notiz `phi/pipeline/ledger.φ:132` „impc_roti_compiler.rs fehlt" ist stale.
- **CDN-Assets gemessen (2026-10-09):** `zenodo.org/superdarn_cpcp.bin` 17240 B · `cdaweb.gsfc.nasa.gov/ssusi_aurora.bin` **104 B** (auffällig klein — gegen die Kompilat-Erwartung prüfen) · `data.earthscope.org/emtf_usarray_cao01_2010.bin` 2176 B (sha256 `42af558f…`) · `prop.kc2g.com/kc2g_stations.csv` 4412 B (sha256 `cdfaa24e…`).
- **`hdf5-real-granule` `37896490054` an `0daa11c8e` — der ATL03-Test benutzt die falsche Route.** Gemessener Grund: `archivar::hdf5::tests::real_granule_atl03_v1_chunk_index_materializes_multilevel` ruft `fetch_bearer_range` gegen `https://data.nsidc.earthdatacloud.nasa.gov/nsidc-cumulus-prod-protected/…/ATL03_…h5`; dieser Host akzeptiert den EDL-Bearer **nicht** — er antwortet **302** auf `urs.earthdata.nasa.gov/oauth/authorize` (gemessen 2026-10-09: ohne Token 302, mit bogus Bearer ebenfalls 302), und `fetch_range` folgt mit `curl -f -L` bis zur URS-Login-Seite → **401** (`ci_manage log 37896490054:502-508`). Der EDL-Token ist frisch (Repo-Secret `updated_at 2026-10-06`, Key in `.secrets.local`); **kein** Token-Problem. Die richtige Maschinen-Route ist der Credential-Tausch, den dein Compiler schon geht: `edl_s3_credentials_for("nsidc-cumulus-prod-protected", token)` (→ `/s3credentials`) + `fetch_s3_range` via SigV4 (`tools/harvest/src/bin/icesat2_atl03_compiler.rs:865`). **Braucht:** den Test auf `fetch_s3_range` + `edl_s3_credentials_for` umstellen (oder einen benannten Skip, wenn der Route-Tausch im Test nicht gewünscht ist).

## An river

Origin: mycelium-273.

- **`ci-gate` clippy — dein Arm geheilt:** `src/archivar/types.rs:313` (`Default` für `ReceiverAperture`) ist als `impl Default` ergänzt; `main_flow.rs` trägt die `ChannelQuery`-Konstruktion an beiden Call-Sites. Bitte gegenlesen.

## An future

Origin: mycelium-273.

- **paper-check-Issue schließen (river-139).** Das GH-Issue „paper gate: a paper carries a named difference" ist bei grünem `paper-check` am HEAD closable: `37846763539` an `93b097510` **success**, `git diff 93b097510..HEAD -- docs/paper docs/blatt` leer. `gh issue close` ist der Maschine verweigert → Operator-Hand.
- **3 orphan register entries (owner future):** `phi/blocked_sources.φ:86` `isip.piconepress.com/projects/tuh_eeg/` · `:90` `sleepdata.org` · `:122` `supermag.jhuapl.edu/services/data-api.php`. Nimm sie als Träger auf oder pflege `blocked account`.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
- **Nachtrag 2026-10-08:** die Route `https://vt.superdarn.org/data-download` ist eingeloggt und erreichbar (gemessen; 15/15 Downloads, 5 Radars). Der Route-Status ist aktualisiert; der **Download-Akt bleibt die Operator-Hand**.

## Abschluss

- **Burn:** close 0.0706 · cap 0.5 — kein pro/max, keine Sub-Agenten (gemessen `session_burn`).
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
