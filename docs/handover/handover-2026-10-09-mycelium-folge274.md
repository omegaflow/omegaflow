<!--
  title: Handover — Mycelium-Folge 274 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. HadISST-Compiler-Koordinaten-Lookup (dim-name → var-name) geheilt; Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-09
  sha256: 677058bd142fd004ab96111d933ac9a1b879a9ca48da834ce61da61beb38c010
  status: live
-->
# Handover — Mycelium-Folge 274 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge273.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`.

## Burn: open 0.0 · close 0.2561 · cap 0.5 — Grund: Meta-Pass „Mycelium-Linie in einem Pass starten" (`session_burn`, Session-Figur; kein pro/max, keine Sub-Agenten).

## Operator-Wort-Register

- „auth ist kein ausschlusskriterium nur kommerziell" | 2026-10-08 | Quelle: mycelium-269.
- „in sources nur APIs mit Kräften" | 2026-10-08 | Quelle: mycelium-269.
- „auf meinem XPS13 dürfen sie auf keinen Fall laufen" | 2026-10-08 | Quelle: mycelium-269 (`subset` auf `t420`).
- „VT SuperDArn ist eingeloggt" | 2026-10-08 | Quelle: mycelium-269.
- „consensus/perplexity als descoped streichen" | 2026-10-09 | Quelle: mycelium-272. **Descoped-Befund:** die zwei Remote-MCP-Einträge (`mcp.consensus.app`, `api.perplexity.ai`) aus `opencode.json` entfernt; der `archive_search`-Arm trägt die Route.
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-09-mycelium-folge272.md` §Operator-Wort-Register — gefaltet, nicht kopiert | 2026-10-09 | Quelle: mycelium-272.
- „leider warst du freezed" | 2026-10-09 | Quelle: mycelium-274 (Session nach Freeze fortgesetzt; Pass `44a57459f` war stale → am neuen HEAD `ce47ce13c` neu gemessen).
- „kannst du das nicht wissenschaft, den rat die UI und openweight stimmen fragen?" | 2026-10-09 | Quelle: mycelium-274 (Verdikt zum Per-SHA-Ort: Forschung + Rat + UI/Open-Weight, alle **B**).

## Offen — eigen

### HadISST-CDN — Koordinaten-Lookup geheilt, Lauf in der Queue
- **Status:** wartend | **Bindung:** eigen (CI-Dispatch)
- **Trigger:** grüner `hadisst-cdn`-Lauf am Fix-HEAD
- **Lage:** (gemessen 2026-10-09 via `ci_manage log 37891967856`) Lauf `cc9f4cf7c` **failure**: `hadisst_compiler: …HadISST_sst.nc.gz: lat carries no numeric values` — der Dim-Toleranz-Fix aus 272 matchte `latitude`/`longitude` als **Dim**-Namen, aber `values_numeric(&bytes, "lat")` suchte die Koordinaten-**Variable** literal `lat`; die CF-Datei trägt `latitude`. In `tools/harvest/src/bin/hadisst_compiler.rs` auf `dim_names[0..2]` umgestellt (Dim-Name = Var-Name), `cargo check` grün, `cargo build -p omegaflow-harvest --bin hadisst_compiler` grün.
- **Blockade:** der geteilte `t420`-Runner + die neue Laufzeit.
- **Braucht:** `ci_manage view 37905540612` (an `efe8b33f4`, **queued** gemessen 2026-10-09 via `ci_manage view`) nach Abschluss; bei rot die benannte Stelle aus dem Log.

### `ci-gate` clippy-Suite — in Mycelium-272 geheilt, CI-Verifikation hängt in der Queue
- **Status:** wartend | **Bindung:** eigen (CI)
- **Trigger:** grüner `ci-gate`-Lauf an `fa7b1144f` (oder `19cbb7fc7`/`974466552`)
- **Lage:** (gemessen 2026-10-09 via `ci_manage view`) `37892705371` an `974466552` **in_progress**; `37892764277` (`19cbb7fc7`) und `37895712548` (`fa7b1144f`) **queued** (t420-Stau). Kein abgeschlossener Lauf am Fix-HEAD.
- **Blockade:** der geteilte `t420`-Runner.
- **Braucht:** `ci_manage view <id>` nach Abschluss; bei rot die benannte Stelle.

### `ci-gate` Per-SHA-Verdikt — **gebaut**: lokales Register + SHA-Abfrage
- **Status:** eigen (Watchdog-Verdrahtung) | **Bindung:** eigen
- **Trigger:** der nächste Stehende Pass
- **Lage:** (gemessen 2026-10-09) Ort = **(B) lokal**, riss-frei entschieden (Forschung; Rat einstimmig; UI Duck/Qwen; Open-Weight Nemotron 3 Ultra/DeepSeek V4 Pro/Inkling; Z.ai + Claude pending). **Gebaut:** `tools/utils/src/bin/ci_gate_register.rs` (SHA → `green|red|pending` (Register-Token; deutsch grün/rot/pending) aus `GET /repos/omegaflow/omegaflow/commits/<sha>/check-runs`, decisive Check `subset`; `cargo build -p omegaflow-utils --bin ci_gate_register` grün, 5 Unit-Tests) + Wrapper `bin/ci_gate_register` + Register `state/zustand/ci-gate.φ` in external-state-Zeilenform (`SHA | Verdikt | measured-at (checks) | fällig | Schritt`). Erstlauf für HEAD `1a0a5df8b` = `pending` (alle Checks queued). `ci-check.yml` ist der schwere Nachtlauf, **nicht** der Per-SHA-Gate.
- **Blockade:** keine.
- **Braucht:** `ci_gate_register` je Stehendem Pass (ein Schritt) — dann kein weiterer Bau. Der frühere „`ci-check`-Push-Ausbau" ist **descoped** (Befund: das Register ist lokal; kein CI-Workflow schreibt es; die Attestation ist der `subset`-Check-Run, die Ableitung läuft lokal).

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
- **Lage:** (gemessen 2026-10-09; mountain-279 `bd0e34fcf` bewegte 20 Einträge in `ausstehend`, 11 `parser-def` re-taggt) die verbleibenden `ausstehend`-Kandidaten tragen Compiler + Workflow je Eintrag; offen ist das Feld-Verdikt / der fehlende Arm (Lunar/Mars/Portal- und GIC-Reihe §A–E). `impc_roti`: Compiler steht (`sources.φ:1642-1648`, `bd0e34fcf`); die Manifestation war 404, in Mycelium-272 verdrahtet (`impc-roti-cdn.yml` + harvest.φ-Arm) — `impc-roti-cdn` `37895723379` an `fa7b1144f` ist **success** (gemessen 2026-10-09 via `ci_manage view`).
- **Blockade:** je Eintrag das Feld-Verdikt der Feder (Mountain register) oder der fehlende Parser-Arm.
- **Braucht:** je Eintrag Ernte-Verdrahtung (Mycelium); die GIC-Reihe §A–E ist Rivers GIC-Deskriptor-Arbeit.

### `canonical_point_key` erzeugt Ganzzeilen-Schlüssel → dropped-gate-Baseline driftet
- **Status:** eigen (Register-Tooling) | **Bindung:** eigen
- **Trigger:** der nächste Handover-Rotations-/Wachstums-Drop
- **Lage:** (gemessen 2026-10-08; river-139 bestätigt) `canonical_point_key` (`register_lookup.rs:2417`) verschlüsselt **alle** `point_key_tokens` einer Prosa-Zeile (kein `match_prefix`, min(6)); eine umformulierte/gewachsene Zeile liefert einen neuen Schlüssel → die `dropped-gate`-Baseline (NAMENS-Basis) muss nach jeder Rotation nachgezogen werden. 270 hat die Baseline gegen die gemessenen Drop-Namen nachgezogen.
- **Blockade:** kein stabiler Namensraum; eine echte Heilung (explizites `**ID:**` bevorzugen, Prosa-Fragmente verwerfen) würde die 927-Altschüssel invalidieren.
- **Braucht:** Verdikt (Mountain register tooling), ob `canonical_point_key` auf kurze Namens-Köpfe begrenzt wird (Alt-Baseline dann einmalig neu erzeugen) und ob `derive_carriers` auch `archiv/` liest.

## An river

Origin: mycelium-274.

- **`ci-gate` clippy — dein Arm geheilt:** `src/archivar/types.rs:313` (`Default` für `ReceiverAperture`) ist als `impl Default` ergänzt; `main_flow.rs` trägt die `ChannelQuery`-Konstruktion an beiden Call-Sites. Bitte gegenlesen.
- **`field_te_query.rs` — zwei offene Stellen (gemessen 2026-10-09):** (a) das Test-Modul `source_cfg` (`:5069`) trägt das neue `fanout_center`-Feld nicht — `cargo build` sieht es nicht (test-only), `cargo test` bricht; (b) der Datei-Commit-Gate blockt jede Änderung an der Datei wegen `f64::NAN`-Sentineln (Bias-Tor): `bias_column` `:2747` `n_eff.unwrap_or(f64::NAN)` und `:3642`. Beide Stellen sind deine TE-Domain; `fanout_center: None` (kein Zentrum deklariert) und ein benannter Gate-Arm statt NaN sind die Formen.

## An future

Origin: mycelium-274.

- **paper-check-Issue schließen (river-139).** Das GH-Issue „paper gate: a paper carries a named difference" ist bei grünem `paper-check` am HEAD closable: `37846763539` an `93b097510` **success**, `git diff 93b097510..HEAD -- docs/paper docs/blatt` leer. `gh issue close` ist der Maschine verweigert → Operator-Hand.
- **3 orphan register entries (owner future):** `phi/blocked_sources.φ:86` `isip.piconepress.com/projects/tuh_eeg/` · `:90` `sleepdata.org` · `:122` `supermag.jhuapl.edu/services/data-api.php`. Nimm sie als Träger auf oder pflege `blocked account`.

## LOCK

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
- **Nachtrag 2026-10-08:** die Route `https://vt.superdarn.org/data-download` ist eingeloggt und erreichbar (gemessen; 15/15 Downloads, 5 Radars). Der Route-Status ist aktualisiert; der **Download-Akt bleibt die Operator-Hand**.

## Abschluss

- **Burn:** close 0.2561 · cap 0.5 — kein pro/max, keine Sub-Agenten (gemessen `session_burn`).
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
