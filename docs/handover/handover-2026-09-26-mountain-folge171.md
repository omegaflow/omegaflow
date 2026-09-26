<!--
  title: Handover — Mountain-Folge 171 (Stand 2026-09-26)
  session: Mountain-Folge 171
  class: handover
  date: 2026-09-26
  sha256: 476bf427e1fe06a0af5674a1a0a04d3f5b8d7e037fa16e207189f0b87676e855
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
- **CI-Status am HEAD:** (gemessen 2026-09-26 via `ci_manage`) `ci-check`
  `36261666671` (HEAD `ee64c10f6`) **pending**; letzter abgeschlossener roter
  `36250412521`. Der HEAD läuft weiter (fremde Linien); am jeweils neuen HEAD
  einmal neu lesen.
- **Artefakt-Frische (`DUE`):** mit dem Push neu zu messen; `gh workflow run`
  für die betroffenen `*-cdn` + `tools-build` erst nach dem Commit (neue Workflows
  404en vor dem Push).

## Offen (aufgeschlüsselt)

### P2 Atom D — Reader + Compiler + CDN-Weg gebaut, erste Manifestation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `eht-uvfits-cdn`-Lauf (Workflow neu, braucht Commit+Push).
- **Lage:** (gemessen 2026-09-26 via grind-max/grind-pro/grind-flash) Reader
  `src/archivar/uvfits.rs` (FITS-IDI: `baseline_rows` ANNAME-Setmatch,
  `fringe_rate_hz`, `beat_open` Gate `df·dt<0.5`, `beat_rows`→`TnfPhaseRow`),
  Compiler `tools/harvest/src/bin/eht_uvfits_compiler.rs` (`--verify`/`--run`/
  `--runfits`/`--pair`/`--out`/`--ci-mode` + `upload_release`), `extract.rs`-Arm
  `eht_uvfits`, `fits.rs::parse_lenient`, vocab-Fixture, Workflow
  `.github/workflows/eht-uvfits-cdn.yml`; Quelle in `phi/sources.φ` registriert
  (`format eht_uvfits`, zwei `field`-Komponenten, `at earth`). `cargo check` 0/0.
  Real gemessen: BINTABLE, INTTIM 0.4 s, REF_FREQ 228.16 GHz, CHAN_BW 500 kHz;
  **AP–AZ df = 3.242e-1 Hz, Gate OPEN (df·dt = 0.1297)**. ALMA (`Aa`) liegt in
  einem späteren `.FITS`-Member.
- **Blockade:** erste Manifestation braucht Commit+Push (Workflow noch nicht auf
  dem default branch).
- **Braucht:** nach `/commit`+Push `gh workflow run eht-uvfits-cdn.yml`; danach
  Asset-Größe/sha256 in `phi/sources.φ` nachtragen und die `Aa`–`Ap`-df aus dem
  Vollauf ablesen.

### P3 Harvest-Assets — HAMQSL + OGIMET geschlossen, NOHRSC offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `nohrsc_snowfall-cdn` `36262592887`.
- **Lage:** (gemessen 2026-09-26 via grind-flash + ci_manage) HAMQSL Asset 200
  (`sha256 872bcbd8…`, magic `HSL1`) → `phi/harvest.φ:95` geschlossen; OGIMET
  Asset 200 (1688 B, `sha256 c66e905e…`) → `phi/harvest.φ:164` geschlossen.
  NOHRSC `36242617755` failure (E0583, vor dem Fix); `36259470458` failure (HEAD
  `a0a6a548`, vor dem Push); neu dispatcht `36262592887` (nach `ee64c10f6`).
- **Blockade:** keine.
- **Braucht:** Lauf `36262592887` lesen; bei success `phi/harvest.φ:145`
  schließen.

### P3b RX100-Luminanz — `asset fehlt`-Tag trägt „lokal, kein CDN"
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `rx100_compiler`-CDN-Lauf (keiner existiert).
- **Lage:** (gemessen 2026-09-26 via `register_lookup --open`) `phi/harvest.φ:233`
  ist `[mountain] asset fehlt` (`format rx100_luminance`, Sony RX100 V,
  `src/archivar/rx100.rs`, `rx100_compiler.rs`), doch die eigene note sagt
  „lokal, kein CDN; K-Kalibrierung+Spectral-Proxy pending" — ein Register-Riss
  (der Tag setzt ein CDN-Asset voraus, die note schließt es aus). Der Bau liegt
  bei River (`handover-2026-09-26-river-folge38.md:110`: `K=12.5` ungemessen,
  Band `freq`/`bin_width` nicht verdrahtet).
- **Blockade:** keiner — der Riss ist die Frage: lokale Gerätequelle ohne
  CDN-Asset.
- **Braucht:** `phi/harvest.φ:233`-Status klären — lokale Quelle → Status ≠
  `asset fehlt` (bzw. Eintrag aus dem CDN-Harvest-Register); die Substanz trägt
  River.

### P4 gll.rss ODR-Shard
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `gll-rss-odr-cdn` `36260734404`.
- **Lage:** (gemessen 2026-09-26 via grind-flash) Lauf `36242358527` failure
  (E0583); `gll_rss_odr.bin` = 404. Neu dispatcht `36260734404` → **in_progress**.
- **Blockade:** keine.
- **Braucht:** Lauf `36260734404` lesen; nach Manifestation den sha in den Block
  `phi/sources.φ:8431` nachtragen.

### P5 CI-Verify TAP/Parquet/GRIB
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner `ci-check` am HEAD.
- **Lage:** (gemessen 2026-09-26 via `ci_manage`) jüngster `ci-check` `36261666671`
  (HEAD `ee64c10f6`) **pending**. Der letzte abgeschlossene rote `36250412521`
  (HEAD `0b182a7a`) scheiterte an **fremden** Lints (format `skydirection.rs:200`;
  clippy `extract.rs`×4, `main_flow.rs`×2, `spatial.rs`, `uws.rs`, `te.rs:4306`,
  `archivar/tests.rs:6181`) + Mountain-Tests am Vor-Fix-SHA.
  `epoch obs_time mjd` (ogle) vorhanden `phi/sources.φ:9524`.
- **Blockade:** fremde format/clippy-Lints (sensory `skydirection.rs`/`te.rs`;
  mycelium `extract.rs`/`uws.rs`/`archivar/tests.rs`; river
  `main_flow.rs`/`spatial.rs`).
- **Braucht:** `ci-check` `36261666671` (oder den Lauf am neuen HEAD) **einmal**
  lesen; Mountain-Tests (grib2/ionocal/parquet) am neuen HEAD verifizieren.

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
  Stimmen bestätigen** (gemessen 2026-09-26): `glm-4.5-flash`, `gemini-2.5-flash`
  (API-Voice-Runner) und die UI-Stimme `chat.z.ai` (GLM-5.3-Flash) tragen das
  `descoped`, keine nennt eine gemessene Gegendquelle; Rohantworten
  `state/stimmen/2026-09-26_1959*_prompt-epochrange-p6.json` +
  `state/stimmen/2026-09-26_zai_ui_epochrange-p6.json`; mistral + `glm-4.7-flash`
  = 429 (Quota).
- **Blockade:** keine.
- **Braucht:** kein Bau. Sobald eine Ernte eine Start/Stopp-MJD trägt, wird der
  Slot im selben Atom wie der Parser geboren (Wire v10, mit Presence-Bit).

### P7 DevTools-MCP — Riss gemessen, Config-Hebel gesetzt
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort (opencode-Neustart; die MCP-Config wird beim Prozessstart gelesen).
- **Wort:** „Token ist gesetzt" (Bridge steht); der Neustart-Akt steht beim Operator | 2026-09-26 | Operator (Session).
- **Lage:** (gemessen 2026-09-26 via general) die Brücke steht (Extension-Token
  gesetzt, `browser_targets` liefert 1 Ziel). Der `-32001`-Riss ist **nicht** die
  Ausgabegröße, sondern ein **60 000 ms Wall-Clock-Timeout** (30/55/59,5 s ok;
  60,5 s → `-32001`); Quelle `chrome-devtools-mcp/build/src/third_party/index.js:23985`
  (`DEFAULT_REQUEST_TIMEOUT_MSEC = 60000`); `opencode.json` trug kein `timeout`.
  Hebel gesetzt: `mcp.chrome-devtools.timeout = 180000`.
- **Blockade:** der laufende opencode-Prozess lädt die MCP-Config nicht neu.
- **Braucht:** opencode neu starten, dann die Grenze erneut messen
  (`chrome-devtools_evaluate_script` mit 90-s-Busy-Wait) — läuft er durch, ist
  der Riss zu; sonst nächster Hebel `experimental.mcp_timeout`.

## Benannt — ungemessen (offene Messungen aus diesem Atom)

- **Vier der fünf UI-Chat-Stimmen zum `epochrange`-Befund** — `claude.ai`,
  `arena.ai`, `chat.deepseek.com`, `kimi.ai`. (gemessen 2026-09-26 via general +
  `chrome-devtools`) **`chat.z.ai` (GLM-5.3-Flash) lief** und trägt `descoped`
  (`state/stimmen/2026-09-26_zai_ui_epochrange-p6.json`); die vier anderen stehen
  im `chrome-devtools`-Browser an **Login-/Consent-Wänden**
  (`claude.ai`/new → `/logout?involuntary=1`; `chat.deepseek.com` → `/sign_in`;
  `kimi.ai` Anmelde-Modal; `arena.ai` ToS-Gate Zweit-AGB) — nicht umgangen.
  Braucht: die vier Sessions im `chrome-devtools`-Browser neu anmelden (oder die
  eingeloggten Tabs aus der OpenCode-Bridge-Gruppe, die `chrome-devtools_*`
  nicht adressiert) — Operator-Akt.
- **EHT `Aa`–`Ap`-df** — der Vollauf der 2,1-GB-Module ist ungemessen; gemessen
  ist nur AP–AZ (`df = 3.242e-1 Hz`). Führt P2 (erster `eht-uvfits-cdn`-Lauf).
- **`eht_uvfits`-Registrierung gegen den Lauf** — der Registrierblock `phi/sources.φ`
  (`format eht_uvfits`, zwei `field`, `at earth`) ist bis zur ersten Manifestation
  unbestätigt. Führt P2.
- **P7-Fix-Wirkung** — `mcp.chrome-devtools.timeout = 180000` ist gesetzt; ob die
  Grenze damit über 60 s liegt, ist bis zum opencode-Neustart ungemessen. Führt P7.

## Operator-Wort-Register

- **Wort:** P6 — den Rat zur `epochrange`-Wire-Frage hören | 2026-09-26 | Operator (Session). Ergebnis: Rat `descoped` mit Befund.
- **Wort:** P7 — „ja, neues Addon installiert, Token passt nicht" | 2026-09-26 | Operator (Session).
- **Wort:** P8 — „ja" (S3-Route freischalten) | 2026-09-26 | Operator (Session).
- **Wort:** P7 — „Token ist gesetzt" | 2026-09-26 | Operator (Session).
- **Wort:** P6 — externe Stimmen über den Voice-Runner fragen (nicht chatgpt.com; die Reviewer stehen in `state/zai-export/freie-llm-zugaenge-2026-09-25.md`) | 2026-09-26 | Operator (Session).
- **Wort:** Session-Consent „alles bis zur Kante nicht verschleppen" (Delegation, nicht Commit-Wort) | 2026-09-26 | Operator (Session).
- **Wort:** „können das nicht Agenten machen?" (die Reste P1/P2/P3/P4/P5/P7/P8 an Agenten dispatcht) | 2026-09-26 | Operator (Session).

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
