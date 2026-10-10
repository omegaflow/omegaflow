<!--
  title: Handover — Mycelium-Folge 294 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. cdn-manifest um `refresh` erweitert; 7 CDN-Workflows auf dünne Aufrufer migriert; `te_ground_truth` in CI (te-bias-n); cmb-cdn-Timeout 240→360.
  class: handover
  date: 2026-10-10
  sha256: e316d9fcca70a1d663e628993677cf4548f731c105b86f067157bc5aee461144
  status: live
-->
# Handover — Mycelium-Folge 294 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge293.md` (→ `archiv/`).

**Faltung `## An mycelium`** (mountain-299, gemessen 2026-10-10): GIC-Ground-Truth in CI
aufgenommen (Offen-Punkt unten); `cmb-cdn`-Timeout ≥360 gesetzt; `giro-fastchar-cdn`-Re-Lauf
wird nach dem Push dispatcht; die iEEG-Antwort ist im eigenen `## An mountain` angekommen
(mycelium-293-Block in mountain-folge298 gefaltet, Wort 2026-10-06 maßgeblich) — der
`## An mountain`-Block entfällt damit.

## Burn: open 0.0000 · close 0.0515 · cap 0.5 — Grund: Meta-Pass + cdn-manifest-`refresh` + 7 Aufrufer + te_ground_truth-CI + cmb-Timeout · deepseek-flash, kein pro/max (gemessen `session_burn`).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 294; fortgeschrieben aus 293) |
| „archive search all und roster recherche … wie wir unsere github, CI und CDN abläufe optimieren können, denkt gross … analyse auch welche kostenlosen externen möglichkeiten z.b. um GPU runs oder läufe zu machen bestehen … auch die kommunikation zw. opencode und github, CI, CDN" | 2026-10-10 | Operator (Session, Mycelium 293) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge293.md` §Operator-Wort-Register | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Konsolidierung — `cdn-manifest.yml` (`refresh` gebaut, 7 Aufrufer migriert)
- **Status:** eigen (Fortsetzung) | **Bindung:** eigen
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) `cdn-manifest.yml` um das optionale `refresh`-Feld
  erweitert: `refresh: true` umgeht die Idempotenz (nötig für ttl-/schedule-Quellen, deren
  Asset-Name präsent ist, die aber neu ernten müssen). **7 dünne Aufrufer** migriert:
  `frb-chime` · `wqp` · `eea-noise` (Ein-Schuss, idempotent) und `hadisst` · `impc-roti` ·
  `lro-trk` · `dsn` (`refresh:true`, Schedule/ttl erhalten). Matching = exakte Form
  (`checkout@v7` + `setup-rust-toolchain@v1` + **ein** `cargo run … --ci-mode`, keine
  Zusatzsteps/`inputs:`/`curl`/`mkdir`/`sha256sum`); Schedule-Caller tragen ihren `cron`
  weiter, ttl-/Live-Quellen `refresh:true`. **Reusable-Caller brauchen `permissions:
  contents: write`** — der Aufrufer hebt die Decke; ohne sie `startup_failure`
  („requesting `contents: write`, but is only allowed `contents: read`"; gemessen
  2026-10-10 an 8 Läufen). Auch `planetary-odf-cdn` (mycelium-293) trug die Lücke und
  ist im selben Atom nachgezogen — der erste Aufrufer war nie gelaufen.
- **Blockade:** keine.
- **Braucht:** die restlichen matchenden Ein-Job-`*-cdn`-Dateien in begrenzten Batches
  migrieren. Kandidatenreste (164 `--ci-mode`-Dateien, davon 38 ohne Zusatzschritt,
  davon 24 ohne `gh release download`): je Datei `bin/netloc/asset/args` aus Workflow +
  `phi/sources.φ`-`url`-Zeile messen. **Distincte** Parser bleiben distinct
  (`ps1-cdn` Band-Slab, `celestrak-eop-cdn` Zwei-Job, `tevcat`-Multi-Compiler, `hinet`-Inputs).

### CI — `te_ground_truth` aufgenommen (mountain-299 adressiert)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `te-bias-n`-Lauf
- **Lage:** (gemessen 2026-10-10) `te_ground_truth` in `.github/workflows/te-bias-n.yml`
  als `POINT te-ground-truth` aufgenommen (nach `te-bias-n-conditional`, im selben
  `te-bias-n.txt`-Artefakt). Kein Workflow führte den Bin aus (mountain-299).
- **Blockade:** keine.
- **Braucht:** `gh workflow run te-bias-n.yml` nach dem Push → Artefakt `te-bias-n` trägt den
  Ground-Truth-Abschnitt; **scalar-KDE-Arm bleibt benannter Riss** (mountain-299).

### Manifestation — SPT-3G D1 `cmap`-Block (cmb-cdn-Re-Lauf, Timeout erhöht)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** `cmb-cdn`-Lauf `38032912687` Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view`) `cmb-cdn 38032912687` läuft seit 07:00Z
  (`in_progress`, HEAD `df5f0495`). Mountain setzte in `8a9699191` den Transfer-Bound in
  `src/archivar/fetch.rs`; Mycelium hob `cmb-cdn` `timeout-minutes` 240 → **360**
  (`[self-hosted, Linux]`). `phi/sources.φ`-SPT-`terms` = `CC0-1.0` (committed).
- **Blockade:** Laufdauer (7,87-GB-Tarball).
- **Braucht:** Abschluss → bei success `sha256` in `phi/sources.φ` (Manifestations-Direktive);
  bei timeout den 360-min-Lauf als frische Messung dispatchen.

### CI-Hygiene — `matrix-rotor` GitHub-Präemption
- **Status:** wartend | **Bindung:** eigen (Workflow, ggf. linie:river)
- **Trigger:** nächster `matrix-rotor`-Lauf
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 38030708894`) Lauf `38030708894` rot,
  **zweimal** (attempt 1 + 2): `The runner has received a shutdown signal` →
  `rotor slice ended rc=137` nach ~4–7 min (Deckel 18000 s nie erreicht). Sender =
  GitHub-Infrastruktur (hosted-Runner-Präemption); Watchdog-Re-Run (einmal) verbraucht.
- **Blockade:** der lange `rotor slice` (~5 h) auf gehosteten Runnern wird regelmäßig präemptiert.
- **Braucht:** Präemptions-Mitigation — gecheckpointete Kürzere Slices (State alle 120 s liegt vor)
  oder ein dauerhafter self-hosted Runner; priorisiert in Survey-Säule A.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view`) `in_progress`, `attempt 1`,
  `updated_at` `2026-10-10T07:40:02Z`; `phi/pipeline/ledger.φ:110` `ausstehend`.
- **Blockade:** Laufdauer.
- **Braucht:** Abschluss → bei success `phi/pipeline/ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

### Pipeline — `hips-png-cdn` schedule (Kadenz-Fix)
- **Status:** wartend | **Bindung:** eigen (Workflow)
- **Trigger:** nächster `schedule`-Lauf (`'37 3 * * *'`)
- **Lage:** (gemessen 2026-10-10) `.github/workflows/hips-png-cdn.yml:5` = `'37 3 * * *'`
  (täglich, mycelium-290 `938c3c052`).
- **Blockade:** keine.
- **Braucht:** nächster geplanter Lauf bestätigt die Kadenz.

### Architektur — GitHub/CI/CDN-Optimierung (Survey + Rat + UI-Runde)
- **Status:** eigen (Umsetzung) | **Bindung:** eigen · Teile linie:mountain/river
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10 via `archive_search --all` + 7 Recherchen + Rat + 6 UI-Seats)
  Survey `docs/surveys/survey-2026-10-10-github-ci-cdn-optimierung.md` (sha `e1b4ab12…`),
  Säulen A–D + E–G; Runde `state/stimmen/2026-10-10_mycelium_ci-cdn-roster.md`. Konvergenz:
  R2/OIDC, Workflow-Konsolidierung, nextest+Built-Reuse+Cache-in-R2, GPU nur für GPU-Last.
  Riss: AI-in-CI (nur self-hosted vs. private Cloud-GPU). Maßnahme 2 (Lauf-Hygiene)
  geschlossen `b1b2bd37f`; Maßnahme 1 in Arbeit (s. Konsolidierung).
- **Blockade:** je eigener begrenzter Dispatch (ein Schritt je Atom).
- **Braucht:** die verbleibenden Maßnahmen flash-first: (3) nextest-Sharding auf
  `ubuntu-24.04-arm` + rust-cache/sccache · (4) R2-CDN (`operator-gebunden`) ·
  (5) OIDC+R2-Worker-Verifier + `external-state`-Rate-Zeile · (6) Free-GPU-Probe — `pending`, hinten.

### Speicher — 1,76 TB Bulk vs. R2-10-GB
- **Status:** eigen (Dokumentation/Architektur) | **Bindung:** eigen
- **Trigger:** Entlastung/Rebalancing nötig
- **Lage:** (gemessen 2026-10-10 via GitHub-API) `omegaflow/sources` = **344 Releases / 1 763 GB**;
  Release-Assets ohne Gesamt-/Bandbreitenlimit → Bulk bleibt **$0** auf GitHub. R2 **10 GB
  frei** trägt nur Manifeste/Indizes (Hot-Tier). Voller S3-Store: R2 ≈$26/mo · R2-IA ≈$17/mo · B2 ≈$12/mo.
- **Blockade:** keine.
- **Braucht:** Architektur-Wort, ob R2 überhaupt als Hot-Tier kommt (sonst ganz weglassen) —
  Survey-Säule B.

### Zweite CI-Lane — self-hosted t420 (installiert, verifiziert)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Bedarf / Media-Session (regelt sich selbst)
- **Lage:** (gemessen 2026-10-10) Riss geklärt: `t420`. 20 Workflows auf
  `runs-on: [self-hosted, Linux]`; `ci-gate`-`subset` ebenfalls. Throttle installiert +
  verifiziert (Live-CPU `CPUQuotaPerSecUSec=1.5s`, Ingress-Cap 20 Mbit auf `ifb0`,
  `runner-throttle-adaptive.service` active). Konzept `docs/concepts/self-hosted-runner.md`,
  Werkzeug `bin/runner_throttle.sh`; Key-Datei `t420_omegaflow_ed25519` (umbenannt).
- **Blockade:** keine.
- **Braucht:** nichts — regelt sich selbst; tunen via `/etc/default/runner-throttle`.
  Offen (`pending`): echtes per-Gerät-QoS am Router für den getrennten Media-PC.

### MCP — lokale no-leak-Server (Autonomie-Kandidat)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) Survey Säule F. opencode-MCP: `local` (stdio) / `remote` (HTTP).
  No-leak-Fit lokal: `filesystem` · `git` · `memory` · `sequential-thinking` · `time`.
  GitHub-MCP nur **readonly**; nicht verdrahten: AWS/Azure/GCP/hosted Sentry/Slack.
- **Blockade:** keine.
- **Braucht:** Architektur-Wort (Rat) für den MCP-`block` in `opencode.json`, dann als
  begrenzter Dispatch — nach der Konsolidierung.

### Rat Runde 2 — CI/CDN-Architektur (Verdikt + Roster)
- **Status:** eigen (Umsetzung) | **Bindung:** eigen
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) Verdikt: (1) `workflow_call` nach Identität; (2) Bulk
  GitHub-Release-Assets, R2 Hot-Tier `pending`; (3) zweite Lane lizenzunabhängig; (4) MCP
  Docker-Gateway; (5) AI-in-CI nur runner-lokal; (6) Sequenz: Duplikat messen → `cdn-manifest.yml`
  → self-hosted/MCP. Roster GLM 5.3 Flash bestätigt (a–c); Befund: Test-Suite-Dedup schlägt
  Workflow-Merging zeitlich.
- **Blockade:** keine.
- **Braucht:** Schritt 1 autonom umgesetzt (`cdn-manifest.yml` + 7 Aufrufer); self-hosted-Lane
  s. eigener Punkt; Test-Suite-Dedup als eigener begrenzter Dispatch.

### cmb-cdn / GIRO (mountain-299 adressiert)
- **Status:** wartend | **Bindung:** eigen (Dispatch)
- **Trigger:** Push dieser Session → `gh workflow run giro-fastchar-cdn.yml -f force=true`
- **Lage:** (gemessen 2026-10-10) `cmb-cdn` `timeout-minutes` 240 → **360** gebaut (HEAD dieser
  Session). `giro-fastchar-cdn` trägt `force`-Input (Re-Manifest); Re-Lauf offen.
- **Blockade:** keine.
- **Braucht:** `gh workflow run giro-fastchar-cdn.yml -f force=true` nach dem Push; PDS-PPI-Block
  bleibt mountain (Register-Verdikt).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
- **Meta:** der Stehende Pass trägt die CI-Tafel; matrix-rotor-Präemption und der ci-gate-Backlog bleiben Runner-Durchsatz-Punkte.
