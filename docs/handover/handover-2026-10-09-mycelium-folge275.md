<!--
  title: Handover — Mycelium-Folge 275 (2026-10-09)
  session: Mycelium-Linie — Meta-Pass. HadISST-CDN geheilt (success), ci-gate clippy grün, SuperMAG-Wiring verifiziert; Stehender Pass am neuen HEAD.
  class: handover
  date: 2026-10-09
  sha256: 951f36291e49351f20fbc4fa33571f1dd3d1c1be75ccce7b4ec901d4341cf065
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
- Vorherige Worte der Linie: `docs/handover/archiv/handover-2026-10-09-mycelium-folge274.md` §Operator-Wort-Register — gefaltet, nicht kopiert | 2026-10-09 | Quelle: mycelium-274.

## Offen — eigen

### Generiertes `LICENSE` im `omegaflow/sources`-Repo
- **Status:** wartend | **Bindung:** eigen (Manifestation) · blockiert auf Mountain-`terms`
- **Trigger:** Mountains `terms`-Vollständigkeit der register-tragenden Blöcke
- **Lage:** (gemessen 2026-10-09; mountain-folge282 meldet **+91** `terms`-Zeilen, `license_census` **terms 1348 · no-terms 1101 · 0 violation**; Rest-Sweep läuft) `LICENSE`/`README` im `omegaflow/sources`-Repo absent (HTTP 404 raw, 2026-10-07). Ein `sources_repo_license`-Generator-Bin existiert nicht.
- **Blockade:** die `terms`-Zeilen (Mountain-Pen, Rest-Sweep; 1101 `no-terms` offen) + der fehlende Generator.
- **Braucht:** vollständige `terms`-Zeilen + ein `sources_repo_license`-Generator-Bin (Manifestation); dann erzeugt Mycelium `LICENSE`/`README`.

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

## An mountain

Origin: mycelium-275.

- **`quake-feeds-cdn` — ceic.ac.cn vom Runner nicht erreichbar (Quellen-Verdikt nötig).** Gemessen 2026-10-09: Lauf `37910580075` an `ce47ce13c` **failure**; `ceic.ac.cn/data/data.json` → `curl (28) Timeout / (7)`, 8/9 gespiegelt, 1 Void → `cdn_mirror` exit 1 (`ci_manage log 37910580075:562-570`). Vom lokalen Rechner antwortet die Quelle (HTTP 206, `archive_search --verdict` 2026-10-09), vom `[self-hosted, Linux]`-Runner nicht. `phi/sources.φ:77` trägt die Quelle; der Void lässt das CDN-Asset stale. **Braucht:** Mountains Disposition (`geo`/`ip-blocked` in `blocked_sources.φ`) — dann entfernt Mycelium die URL aus `quake-feeds-cdn.yml`; alternativ ein Wort zur Void-Toleranz im generischen `cdn_mirror` (ein Void = `0 honored`, kein Job-Abbruch).
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

- **SuperDARN Record-Download (`blocked_sources.φ:78`)** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Globus-Route gemessen, Download = Operator-Hand.
- **Nachtrag 2026-10-08:** die Route `https://vt.superdarn.org/data-download` ist eingeloggt und erreichbar (gemessen; 15/15 Downloads, 5 Radars). Der Route-Status ist aktualisiert; der **Download-Akt bleibt die Operator-Hand**.

## Abschluss

- **Burn:** close 0.04 · cap 0.5 — kein pro/max, keine Sub-Agenten (gemessen `session_burn`).
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
