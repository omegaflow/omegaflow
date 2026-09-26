<!--
  title: Handover — Mountain-Folge 171 (Stand 2026-09-26)
  session: Mountain-Folge 171
  class: handover
  date: 2026-09-26
  sha256: 7980dc1c33db8249db16a878b6de897b7db58eafe4a0ed2586bed772c0801faa
  status: live
-->
# Handover — Mountain-Folge 171 (2026-09-26)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert; git trägt, was gemacht wurde. Keine Rangfolge — die offenen Punkte
werden parallel von Agenten abgearbeitet; `blockiert`/`wartend` werden benannt,
nie dispatcht. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen Hunks,
pfad-begrenzter Commit. Jeder Punkt aufgeschlüsselt: Trigger / Lage / Blockade /
Braucht.

## Stehender Pass

- **Postfach:** (gemessen 2026-09-26) `state/mail/mail_ledger.φ` existiert jetzt
  (jüngster Eintrag 2026-09-14, kein neuer Mountain-Eingang) — folge170s „absent"
  war stale.
- **CI-Status am HEAD:** (gemessen 2026-09-26 via `ci_manage`) letzter
  abgeschlossener `ci-check` `36250412521` (HEAD `0b182a7a`) **failure**; neuer
  Lauf `36256060965` in_progress. Vor `/commit`+Push ungemessen für den neuen HEAD.
- **Artefakt-Frische (`DUE`):** mit dem Push neu zu messen; `gh workflow run`
  für die betroffenen `*-cdn` + `tools-build` erst nach dem Commit (neue Workflows
  404en vor dem Push).

## Offen (aufgeschlüsselt)

### P2 Atom D — uvfits-Reader gebaut, Aa–Ap-Vollauf + CDN offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `eht-uvfits-cdn`-Lauf (Workflow neu, braucht Commit+Push).
- **Lage:** (gemessen 2026-09-26 via grind-max) uvfits-Reader gebaut:
  `src/archivar/uvfits.rs` (FITS-IDI: `baseline_rows` ANNAME-Setmatch,
  `fringe_rate_hz`, `beat_open` Gate `df·dt<0.5`, `beat_rows`→`TnfPhaseRow`),
  `tools/harvest/src/bin/eht_uvfits_compiler.rs`, `extract.rs`-Arm `eht_uvfits`,
  `fits.rs::parse_lenient`, vocab-Fixture; `cargo check` 0/0. Real gemessen:
  BINTABLE, INTTIM 0.4 s, REF_FREQ 228.16 GHz, CHAN_BW 500 kHz; **AP–AZ
  df = 3.242e-1 Hz, Gate OPEN (df·dt = 0.1297)**. ALMA (`Aa`) liegt in einem
  späteren `.FITS`-Member (Reader iteriert alle Member).
- **Blockade:** keine.
- **Braucht:** CI-Workflow `eht-uvfits-cdn.yml` bauen (Download + `--verify`
  sha `ebff01cc…` + `--run`), Quelle in `phi/sources.φ` registrieren, dann
  `gh workflow run eht-uvfits-cdn.yml`; danach die `Aa`–`Ap`-df aus dem Vollauf.
  (Eigener Hunk im geteilten `phi/sources.φ`.)

### P3 Harvest-Assets — HAMQSL + OGIMET geschlossen, NOHRSC offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `nohrsc_snowfall-cdn` `36259470458`.
- **Lage:** (gemessen 2026-09-26 via grind-flash) HAMQSL Asset 200
  (`sha256 872bcbd8…`, magic `HSL1`) → `phi/harvest.φ:95` geschlossen; OGIMET
  Asset 200 (1688 B, `sha256 c66e905e…`) → `phi/harvest.φ:164` geschlossen.
  NOHRSC Lauf `36242617755` failure (E0583, vor dem Fix); Asset
  `nohrsc_snowfall.bin` 404; neu dispatcht `36259470458`.
- **Blockade:** keine.
- **Braucht:** Lauf `36259470458` lesen; bei success `phi/harvest.φ:145`
  schließen.

### P4 gll.rss ODR-Shard
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `gll-rss-odr-cdn`.
- **Lage:** (gemessen 2026-09-26 via grind-flash) Lauf `36242358527` failure
  (E0583); `gll_rss_odr.bin` = 404.
- **Blockade:** keine.
- **Braucht:** Lauf `36260734404` lesen; nach Manifestation den sha in den Block
  `phi/sources.φ:8431` nachtragen.

### P5 CI-Verify TAP/Parquet/GRIB
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner `ci-check` am HEAD.
- **Lage:** (gemessen 2026-09-26 via `ci_manage`) `ci-check` `36250412521`
  (HEAD `0b182a7a`) failure an **fremden** Lints (format `skydirection.rs:200`;
  clippy `extract.rs`×4, `main_flow.rs`×2, `spatial.rs`, `uws.rs`, `te.rs:4306`,
  `archivar/tests.rs:6181`) + Mountain-Tests am Vor-Fix-SHA
  (`grib2::complex_packing_two_groups`, `ionocal::*`, `parquet::testkit::*`).
  `epoch obs_time mjd` (ogle) vorhanden `phi/sources.φ:9524`.
- **Blockade:** fremde format/clippy-Lints (sensory `skydirection.rs`/`te.rs`;
  mycelium `extract.rs`/`uws.rs`/`archivar/tests.rs`; river
  `main_flow.rs`/`spatial.rs`).
- **Braucht:** nach `/commit`+Push den `ci-check` lesen; Mountain-Tests erneut
  prüfen (die lokalen Lint-Fixes sind noch uncommittet).

### P1 Planck-SZ — Register-Akt angewendet, Manifestation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `planck-psz2-cdn`-Lauf (Workflow neu, braucht Commit+Push).
- **Lage:** (gemessen 2026-09-26 via grind-flash) die Block-Datei
  `/tmp/opencode/planck-register-block.φ` ist **absent** → Block aus gemessenen
  Fakten + Parser-Vertrag rekonstruiert. `cmap`-Vertrag in `src/archivar/parse.rs`
  verifiziert (`at sun`, `cmap .`, `z z` gültig); **Korrektur:** ohne `field`
  null Kanäle → `field snr planck_psz2_snr inverse-square em 1 604800 0.0 0.0`
  ergänzt. Neuer Block `phi/sources.φ` @ ~8078; `phi/blocked_sources.φ:331-333`
  → `released`. CI-Workflow `.github/workflows/planck-psz2-cdn.yml` gebaut;
  `url`/Dateiname `planck_psz2_sz_mmf3.json` stimmt mit dem Compiler-`--out`.
- **Blockade:** Workflow nicht auf dem default branch (`gh workflow run` → 404).
- **Braucht:** nach `/commit`+Push `gh workflow run planck-psz2-cdn.yml`; dann
  das Asset `--sniff`/sha256 messen und in den Block nachtragen (die abgeleiteten
  Werte CDN-Zielname/ttl sind erst dann gemessen).

### P8 S3-Scheme — lp-prod gemessen frei
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-26 via grind-flash) kein Register-Eintrag. Die
  SigV4+STS-Route (`src/archivar/range.rs:236/255`) mit `EARTHDATA_EDL_TOKEN`
  (`.secrets.local`) gemessen: `gedi_l2a_compiler --cmr …` → Range-GET auf
  `s3://lp-prod-protected` **freigeschaltet** (Granule 263 MB → 2473 records,
  158280 B, roundtrip parses). Der alte 401 ist abgelöst. ListBucket bleibt 403
  (Temp-Creds scoped auf GetObject).
- **Blockade:** keine.
- **Braucht:** `nsidc-cumulus-prod-protected` (ICESat-2) und den SWOT/PODAAC-Bucket
  je mit einem Range-GET über denselben Compiler-`--cmr`-Pfad messen und als frei
  bestätigen; der `declined_sources.φ:2455`-Note-Wortlaut bleibt unberührt.

### P6 `epochrange` Wire-Slot — descoped (Rat)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** Wiederöffnung — eine Ernte trägt eine Start/Stopp-MJD.
- **Lage:** (gemessen 2026-09-26 via Rat + `sources.φ`-Scan) **descoped mit
  Befund:** kein `tstart/tstop/range/exposure` in `phi/sources.φ` (nur spektrales
  `bin_width_hz`); kein Konsument liest eine Breite (WGSL/ω() unberührt); der
  `r_eq`-Pad ist ehrliche Abwesenheit (Atom 7), und ein nacktes `0.0` kollabiert
  „gemessener Punkt"/„nie gemessen" (Kontrakt verbietet). Der Entwurf zitierte
  einen falschen Pfad (mathematikerin statt archivar) — die Drahtschreiber-Datei
  ist `src/archivar/spatial.rs`, `relay.rs`, Tuple `types.rs:342`. **Externe
  Stimmen bestätigen** (gemessen 2026-09-26, API-Voice-Runner): `glm-4.5-flash`
  + `gemini-2.5-flash` tragen das `descoped`, keine nennt eine gemessene
  Gegendquelle; Rohantworten `state/stimmen/2026-09-26_1959*_prompt-epochrange-p6.json`;
  mistral + `glm-4.7-flash` = 429 (Quota).
- **Blockade:** keine.
- **Braucht:** kein Bau. Sobald eine Ernte eine Start/Stopp-MJD trägt, wird der
  Slot im selben Atom wie der Parser geboren (Wire v10, mit Presence-Bit).

### P7 DevTools-MCP — Brücke steht, MCP-Timeout ungemessen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `chrome-devtools_list_pages` erneut prüfen.
- **Lage:** (gemessen 2026-09-26) der Extension-Token ist gesetzt; der
  `bad_token`-Riss (`opencode.log:721848`: Broker erwartete 48 Hex, Extension
  sandte 34) ist gelöst — `browser_targets` liefert **1 Ziel**
  (`d5c512d7-c66a-4ef4-916f-146da21ec8a8`). Das MCP `-32001` blieb in diesem
  Zustand **ungemessen** → `pending`.
- **Blockade:** keine.
- **Braucht:** `chrome-devtools_list_pages` / eine `--autoConnect`-Prüfung; bei
  Timeout ist die MCP-Seite (`npx chrome-devtools-mcp@1.9.0`) der Rest.

## Operator-Wort-Register

- **Wort:** P6 — den Rat zur `epochrange`-Wire-Frage hören | 2026-09-26 | Operator (Session). Ergebnis: Rat `descoped` mit Befund.
- **Wort:** P7 — „ja, neues Addon installiert, Token passt nicht" | 2026-09-26 | Operator (Session).
- **Wort:** P8 — „ja" (S3-Route freischalten) | 2026-09-26 | Operator (Session).
- **Wort:** P7 — „Token ist gesetzt" | 2026-09-26 | Operator (Session).
- **Wort:** P6 — externe Stimmen über den Voice-Runner fragen (nicht chatgpt.com; die Reviewer stehen in `state/zai-export/freie-llm-zugaenge-2026-09-25.md`) | 2026-09-26 | Operator (Session).
- **Wort:** Session-Consent „alles bis zur Kante nicht verschleppen" (Delegation, nicht Commit-Wort) | 2026-09-26 | Operator (Session).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
