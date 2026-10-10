<!--
  title: Handover — Mycelium-Folge 295 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. cdn-manifest `crate`-Feld; viking-text + weberin-verdicts migriert; startup_failure-Fix (66654392f) gemessen bestätigt.
  class: handover
  date: 2026-10-10
  sha256: 5bca206866ae9fd59238c2d63703062863e9aa4d62cc105584509127fdf45a88
  status: live
-->
# Handover — Mycelium-Folge 295 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge294.md` (→ `archiv/`).

**Faltung `## An mycelium`** (mountain-299): GIC-Ground-Truth in CI dispatched (s. u.);
`cmb-cdn`-Timeout ≥360 gesetzt; `giro-fastchar-cdn`-Re-Lauf `38038736681` **success**
(mountain-299-Wunsch erfüllt) — der `## An mycelium`-Block ist damit gefaltet. PDS-PPI
bleibt mountain.

## Burn: open 0.0000 · close 0.0617 · cap 0.5 — Grund: Meta-Pass + cdn-manifest-`crate` + 2 Aufrufer · deepseek-flash, kein pro/max (gemessen `session_burn`: 22 Sessions $0.9852 → 23 Sessions $1.0469).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 295; fortgeschrieben aus 294) |
| „archive search all und roster recherche … wie wir unsere github, CI und CDN abläufe optimieren können, denkt gross … analyse auch welche kostenlosen externen möglichkeiten z.b. um GPU runs oder läufe zu machen bestehen … auch die kommunikation zw. opencode und github, CI, CDN" | 2026-10-10 | Operator (Session, Mycelium 293) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge293.md` §Operator-Wort-Register (fetchbar via `git show HEAD:archiv/…`) | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Konsolidierung — `cdn-manifest.yml` (11 Aufrufer; `crate`-Feld gebaut)
- **Status:** eigen (Fortsetzung) | **Bindung:** eigen
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) `cdn-manifest.yml` trägt nun ein optionales
  `crate`-Feld (Default `omegaflow-harvest`), damit auch Compiler aus
  `omegaflow-measure`/`-science` als dünne Aufrufer laufen. **2 weitere migriert:**
  `viking-text-cdn` (harvest, Schedule `17 4 1 * *` erhalten) und
  `weberin-verdicts-cdn` (`crate: omegaflow-measure`, `--out data/weberin_verdicts.bin`).
  Damit **11** dünne Aufrufer. Der frühere 294-Fix (`66654392f`) ist **bestätigt**:
  von 8 re-dispatchen `startup_failure`-Läufen **7 success** (frb-chime · wqp ·
  eea-noise · hadisst · impc-roti · dsn · planetary-odf) + `register-coverage`
  success; `lro-trk` `in_progress`; `ci-gate` `queued`.
- **Blockade:** keine.
- **Braucht:** der **exakt-formgleiche** Rest ist schmal (Filter: genau
  `checkout@v7`+`setup-rust-toolchain@v1`+**ein** `cargo run …--ci-mode`, kein
  `inputs:`/`idempotence`/`gh release`/`gh_issue_once`/`mkdir`/`curl`/`sha256sum`/
  `set -euo`): **5 `*-cdn`-Dateien** — `pioneer-doppler` (`pioneer_doppler_compiler`
  **nicht** in `phi/sources.φ` als compiler registriert → erst messen),
  `voyager-merged` (**zwei** Assets voyager1+2_merged.bin, ein Compiler-Lauf →
  Mehr-Asset-Form), `xp-pilot` (`gaia_xp_compiler`, Asset-Zeile nicht gefunden).
  Die verbleibenden 339 `*-cdn` tragen Zusatzsteps (Health-Wrapper/args/inputs) und
  brauchen je Datei-Judgment, kein Blind-Batch.

### CI — `te_ground_truth` (mountain-299 adressiert, dispatched)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `te-bias-n`-Lauf `38038722712` Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view`) Lauf `38038722712` auf
  `head_sha ea4025b28` (dem Commit, der `te_ground_truth` aufnahm) `in_progress`,
  Step „Run the TE bias-vs-n probe" — der Punkt ist **dispatched**, nicht mehr offen
  zu werfen.
- **Blockade:** keine.
- **Braucht:** Abschluss → Artefakt `te-bias-n` trägt den Ground-Truth-Abschnitt;
  **scalar-KDE-Arm bleibt benannter Riss** (mountain-299).

### Manifestation — SPT-3G D1 `cmap`-Block (cmb-cdn-Re-Lauf)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** `cmb-cdn`-Lauf `38032912687` Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view`) `in_progress`, HEAD
  `df5f0495`, Step „Compile the SPT-3G D1 CMB map once and upload to CDN";
  `timeout-minutes` 240 → **360** (`[self-hosted, Linux]`). `phi/sources.φ`-SPT-`terms`
  = `CC0-1.0`.
- **Blockade:** Laufdauer (7,87-GB-Tarball).
- **Braucht:** Abschluss → bei success `sha256` in `phi/sources.φ`; bei timeout den
  360-min-Lauf neu dispatchen.

### CI-Hygiene — `matrix-rotor` GitHub-Präemption
- **Status:** wartend | **Bindung:** eigen (Workflow, ggf. linie:river)
- **Trigger:** nächster `matrix-rotor`-Lauf
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 38030708894`) `38030708894` rot,
  **zweimal** (attempt 1+2): „The runner has received a shutdown signal" →
  `rotor slice ended rc=137` nach ~4–7 min. Sender = GitHub-Infrastruktur
  (hosted-Runner-Präemption); Watchdog-Re-Run verbraucht.
- **Blockade:** der lange `rotor slice` (~5 h) auf gehosteten Runnern wird präemptiert.
- **Braucht:** Präemptions-Mitigation — gecheckpointete Kürzere Slices (State alle 120 s
  liegt vor) oder dauerhafter self-hosted Runner; Survey-Säule A.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view`) Workflow `hips-png-cdn`,
  `in_progress`, `attempt 1`, `updated_at` `2026-10-10T07:40:02Z`; `phi/pipeline/ledger.φ:110` `ausstehend`.
- **Blockade:** Laufdauer.
- **Braucht:** Abschluss → bei success `phi/pipeline/ledger.φ:110` → `disponiert` + CDN-Asset prüfen.

### Pipeline — `hips-png-cdn` schedule (Kadenz-Fix)
- **Status:** wartend | **Bindung:** eigen (Workflow)
- **Trigger:** nächster `schedule`-Lauf (`'37 3 * * *'`)
- **Lage:** (gemessen 2026-10-10) `hips-png-cdn.yml:5` = `'37 3 * * *'` (täglich).
- **Blockade:** keine.
- **Braucht:** der laufende `37932098229` (07:40Z) ist bereits ein Kadenz-Beleg.

### Architektur — GitHub/CI/CDN-Optimierung (Survey + Rat + UI-Runde)
- **Status:** eigen (Umsetzung) | **Bindung:** eigen · Teile linie:mountain/river
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) Survey `docs/surveys/survey-2026-10-10-github-ci-cdn-optimierung.md`,
  Säulen A–G; Runde `state/stimmen/2026-10-10_mycelium_ci-cdn-roster.md`. Maßnahme 2
  (Lauf-Hygiene) `b1b2bd37f`; Maßnahme 1 in Arbeit (s. Konsolidierung).
- **Blockade:** je eigener begrenzter Dispatch (ein Schritt je Atom).
- **Braucht:** die verbleibenden Maßnahmen flash-first: (3) nextest-Sharding auf
  `ubuntu-24.04-arm` + rust-cache/sccache · (4) R2-CDN (`operator-gebunden`) ·
  (5) OIDC+R2-Worker-Verifier + `external-state`-Rate-Zeile · (6) Free-GPU-Probe — `pending`, hinten.

### Speicher — 1,76 TB Bulk vs. R2-10-GB
- **Status:** eigen (Dokumentation/Architektur) | **Bindung:** eigen
- **Trigger:** Entlastung/Rebalancing nötig
- **Lage:** (gemessen 2026-10-10 via GitHub-API) `omegaflow/sources` = **344 Releases /
  1 763 GB**; Release-Assets ohne Gesamt-/Bandbreitenlimit → Bulk bleibt **$0**.
  R2 **10 GB frei** trägt nur Manifeste/Indizes. Voller S3-Store: R2 ≈$26/mo ·
  R2-IA ≈$17/mo · B2 ≈$12/mo.
- **Blockade:** keine.
- **Braucht:** Architektur-Wort, ob R2 als Hot-Tier kommt (sonst weglassen) — Survey-Säule B.

### Zweite CI-Lane — self-hosted t420 (installiert, verifiziert)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Bedarf / Media-Session (regelt sich selbst)
- **Lage:** (gemessen 2026-10-10) 20 Workflows auf `runs-on: [self-hosted, Linux]`;
  Throttle aktiv (Live-CPU `CPUQuotaPerSecUSec=1.5s`, Ingress-Cap 20 Mbit auf `ifb0`,
  `runner-throttle-adaptive.service` active). Konzept `docs/concepts/self-hosted-runner.md`.
- **Blockade:** keine.
- **Braucht:** nichts — tunen via `/etc/default/runner-throttle`. Offen (`pending`):
  echtes per-Gerät-QoS am Router für den getrennten Media-PC.

### MCP — lokale no-leak-Server (Autonomie-Kandidat)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) Survey Säule F: `local` (stdio) no-leak-Fit
  `filesystem`·`git`·`memory`·`sequential-thinking`·`time`; GitHub-MCP nur readonly.
- **Blockade:** keine.
- **Braucht:** Architektur-Wort (Rat) für den MCP-`block` in `opencode.json`, dann
  begrenzter Dispatch — nach der Konsolidierung.

### Rat Runde 2 — CI/CDN-Architektur (Verdikt + Roster)
- **Status:** eigen (Umsetzung) | **Bindung:** eigen
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) Verdikt (1) `workflow_call` nach Identität; (2) Bulk
  GitHub-Release-Assets, R2 Hot-Tier `pending`; (3) zweite Lane; (4) MCP Docker-Gateway;
  (5) AI-in-CI nur runner-lokal; (6) Sequenz. Roster GLM 5.3 Flash bestätigt.
- **Blockade:** keine.
- **Braucht:** Test-Suite-Dedup als eigener begrenzter Dispatch (schlägt Workflow-Merging
  zeitlich, Roster-Befund).

### cmb-cdn (mountain-299 adressiert)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** s. „Manifestation SPT-3G D1" (derselbe Lauf).

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
- **Meta:** der Stehende Pass trägt die CI-Tafel und den Postfach-/Register-Stand.
