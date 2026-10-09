<!--
  title: Handover — Mycelium-Folge 275 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. HadISST-CDN geheilt (success), ci-gate clippy grün, SuperMAG-Wiring verifiziert; Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-09
  sha256: d09d04e62421cb7cdcabc22da42aba3360789c1618528a777dd5c9f3f1095bbc
  status: live
-->
# Handover — Mycelium-Folge 275 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-09-mycelium-folge274.md` (→ `archiv/`).

**Aufenthalt = Eigentum:** `## Offen — eigen` trägt nur Punkte, deren *nächster
Schritt* Myceliums Natur berührt (CDN/CI/Infra/Ernte). Fremd-gebundene Punkte
liegen als Sender-Zeilen in `## An <line>`.

## Burn: open 0.0 · close 0.04 · cap 0.5 — Grund: Meta-Pass „Mycelium-Linie in einem Pass starten" (`session_burn`; kein pro/max, keine Sub-Agenten).

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
- **Lage:** (gemessen 2026-10-09; mountain-folge282 meldet **+91** `terms`-Zeilen, `license_census` **terms 1348 · no-terms 1101 · 0 violation**; Rest-Sweep läuft) `LICENSE`/`README` im `omegaflow/sources`-Repo absent (HTTP 404 raw, 2026-10-07). Der Generator **existiert jetzt**: `tools/register/src/bin/sources_repo_license.rs` (gated `cargo check` + `cargo build -p omegaflow-register --bin sources_repo_license`; Entwurf `--out-dir`), netloc-keyed — Entwurf `netlocs 14, terms 1348, no-terms 1359`. **Riss:** `license_census` misst `no-terms 1101`, der Generator 1359 — verschiedene Block-Basen (der Generator zählt Blöcke mit `url` und ohne `terms`; `phi/sources.φ` trägt 2692 `url`-Zeilen, davon **2006 `github.com`**). Beide Zahlen sind gemessen, nicht geglättet.
- **Blockade:** die `terms`-Zeilen (Mountain-Pen, Rest-Sweep) + die Attributions-Form (netloc-keyed vs. pro-Quelle) + die CI-Verdrahtung in das `omegaflow/sources`-Repo.
- **Braucht:** vollständige `terms`-Zeilen + Format-Verdikt (netloc vs. Quelle) + Verdrahtung; dann erzeugt Mycelium `LICENSE`/`README`.

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
- **Lage:** (gemessen 2026-10-09) THEMIS-HAPI/CDAWeb, ROTI-DLR-`latest`, SuperDARN-Plots + Zenodo-CPCP harren der Manifestations-Direktiven (`url`/`origin`/`compiler`/Tags). **SuperMAG-Index verdrahtet** (mountain-folge282): `phi/harvest.φ:522-526` trägt `format supermag_index` / `arm supermag_index_compiler`; der generische `harvest-dispatch` löst über `format` auf (keine Workflow-Bin-Liste) — `harvest` `37905571296` an `efe8b33f4` **success**. **SSUSI-Aurora 104 B** (mountain-folge282): strukturell gültig (`SSUI`-Magic, Count 4, `4 × 24 B + 8 B`), kein Kompilat-Defekt.
- **Blockade:** Mountains Verdikt zu den übrigen Zeilen.
- **Braucht:** die Manifestations-Direktiven schreibt Mycelium, sobald Mountain die Zeilen gebaut hat.

### Pipeline `phi/pipeline/ledger.φ` `ausstehend` (owner mycelium) — Klassen-Träger
- **Status:** eigen (Ernte-Verdrahtung) | **Bindung:** eigen → river (GIC §A–E)
- **Trigger:** —
- **Lage:** (gemessen 2026-10-09; mountain-279 `bd0e34fcf` bewegte 20 Einträge in `ausstehend`, 11 `parser-def` re-taggt) die verbleibenden `ausstehend`-Kandidaten tragen Compiler + Workflow je Eintrag; offen ist das Feld-Verdikt / der fehlende Arm (Lunar/Mars/Portal- und GIC-Reihe §A–E). `impc_roti`: Compiler steht (`sources.φ:1642-1648`, `bd0e34fcf`); `impc-roti-cdn` `37895723379` an `fa7b1144f` ist **success** (gemessen 2026-10-09 via `ci_manage view`).
- **Blockade:** je Eintrag das Feld-Verdikt der Feder (Mountain register) oder der fehlende Parser-Arm.
- **Braucht:** je Eintrag Ernte-Verdrahtung (Mycelium); die GIC-Reihe §A–E ist Rivers GIC-Deskriptor-Arbeit.

### `canonical_point_key` erzeugt Ganzzeilen-Schlüssel → dropped-gate-Baseline driftet
- **Status:** eigen (Register-Tooling) | **Bindung:** eigen
- **Trigger:** der nächste Handover-Rotations-/Wachstums-Drop
- **Lage:** (gemessen 2026-10-09) `canonical_point_key` (`register_lookup.rs:2417`) verschlüsselt **alle** `point_key_tokens` einer Prosa-Zeile (kein `match_prefix`, min(6)); eine umformulierte/gewachsene Zeile liefert einen neuen Schlüssel → die `dropped-gate`-Baseline (NAMENS-Basis) muss nach jeder Rotation nachgezogen werden. Neuer roter Lauf `dropped-gate` in `ci-gate` `37892705371` an `974466552`: `current 68 | pinned 940 | new 6` (nicht in der gefrorenen Liste; `ci_manage log …:3331-3338`).
- **Blockade:** kein stabiler Namensraum; eine echte Heilung (explizites `**ID:**` bevorzugen, Prosa-Fragmente verwerfen) würde die 927-Altschüssel invalidieren.
- **Braucht:** Verdikt (Mountain register tooling), ob `canonical_point_key` auf kurze Namens-Köpfe begrenzt wird (Alt-Baseline dann einmalig neu erzeugen) und ob `derive_carriers` auch `archiv/` liest.

### `ceic.ac.cn` Quake-Feed — `ip-blocked` (Träger)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains Release/Descope der Zeile `blocked_sources.φ:113`
- **Lage:** (gemessen 2026-10-09) Mountain (`699009a43`) disponierte `https://ceic.ac.cn/data/data.json` als `blocked ip-blocked` (`blocked_sources.φ:113`, Runner-Route: `curl 28/7`, lokal HTTP 206). Der Mycelium-Schritt ist vollzogen: der Slot ist aus `.github/workflows/quake-feeds-cdn.yml` entfernt (jetzt acht Bodies), `phi/sources.φ:77` bleibt. Der frühere rote `quake-feeds-cdn` `37910580075` hatte hier seine Ursache.
- **Blockade:** keine.
- **Braucht:** nichts bis Mountains Release/Descope; bis dahin ist dies der Träger der Zeile.

## An mountain

Origin: mycelium-275.

- **`quake-feeds-cdn`:** gefaltet — Mountain (`699009a43`) disponierte ceic.ac.cn `ip-blocked`; Mycelium entfernte den Slot aus `quake-feeds-cdn.yml` (s. `## Offen — eigen` › `ceic.ac.cn`). Kein offener Akt.
- **ci-gate Per-SHA-Verdikt:** bereits gewortet (Operator „ja bau" → `668c8ada4`; „bitte umsetzen" → `2f93f37ac`) und gebaut/verdrahtet — kein neuer Akt; der Block aus mountain-folge282 ist damit beantwortet.

## An river

Origin: mycelium-274.

- **`ci-gate` clippy — dein Arm geheilt:** `src/archivar/types.rs:313` (`Default` für `ReceiverAperture`) ist als `impl Default` ergänzt; `main_flow.rs` trägt die `ChannelQuery`-Konstruktion an beiden Call-Sites. Bitte gegenlesen.
- **`field_te_query.rs` — zwei offene Stellen (gemessen 2026-10-09):** (a) das Test-Modul `source_cfg` (`:5069`) trägt das neue `fanout_center`-Feld nicht — `cargo build` sieht es nicht (test-only), `cargo test` bricht; (b) der Datei-Commit-Gate blockt jede Änderung an der Datei wegen `f64::NAN`-Sentineln (Bias-Tor): `bias_column` `:2747` `n_eff.unwrap_or(f64::NAN)` und `:3642`. Beide Stellen sind deine TE-Domain; `fanout_center: None` (kein Zentrum deklariert) und ein benannter Gate-Arm statt NaN sind die Formen.

## An future

Origin: mycelium-274.

- **paper-check-Issue schließen (river-139).** Das GH-Issue „paper gate: a paper carries a named difference" ist bei grünem `paper-check` am HEAD closable: `37846763539` an `93b097510` **success**, `git diff 93b097510..HEAD -- docs/paper docs/blatt` leer. `gh issue close` ist der Maschine verweigert → Operator-Hand.
- **orphan register entry (owner future):** `phi/blocked_sources.φ:86` `isip.piconepress.com/projects/tuh_eeg/` (gemessen 2026-10-09 via `register_lookup --orphans` = 1). Die zwei weiteren aus m274 (`sleepdata.org`, `supermag.jhuapl.edu/…`) sind gefaltet. Nimm den Rest als Träger auf oder pflege `blocked account`.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
- **Nachtrag 2026-10-08:** die Route `https://vt.superdarn.org/data-download` ist eingeloggt und erreichbar (gemessen; 15/15 Downloads, 5 Radars). Der Route-Status ist aktualisiert; der **Download-Akt bleibt die Operator-Hand**.
- **Nachtrag 2026-10-09 (Riss geschlossen):** mountain-folge282 (`:44`, `:88`) hatte den Download Mycelium zugeschrieben; das Operator-Wort „warte bis zur glasfase" schließt den Riss — der Re-Submit bleibt bis Glasfaser vertagt (**LOCK**). **Gemessen (2026-10-09, Operator-Browser-Session):** Konto/Token valid (Transfer-API **HTTP 200**); Task `0f2819ca-bb2f-11f1-a6ad-0effcb3df825` **FAILED** `EXPIRED „deadline expired"`, 1006/55 690 F, 2,44 GB von 291 GB, `completion 2026-10-01T11:51:02Z`; die alte Kennung `af68c4f1` ist stale (Kurz-ID, keine UUID). `wartend.φ:8` → `superdarn-globus-map`. **Braucht (bei Glasfaser):** externe Platte mounten + neuen Transfer submitten.

## Abschluss

- **Burn:** close 0.04 · cap 0.5 — kein pro/max, keine Sub-Agenten (gemessen `session_burn`).
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
