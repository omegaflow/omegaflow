<!--
  title: Handover — Mycelium-Folge 293 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. GIRO-DIDBase-FastChar-Lauf grün (Manifestation geschlossen); matrix-rotor-Präemption Re-Run angestoßen; ci-gate-clippy-Rot und cmb-cdn-Download-Time-out neu vermessen.
  class: handover
  date: 2026-10-10
  sha256: c89759604fe4ce07c3396383e51e2cd253c027069e769b65895718916e50183d
  status: live
-->
# Handover — Mycelium-Folge 293 (2026-10-10)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`,
zitiert, nie kopiert). Diese Session konsumierte
`handover-2026-10-10-mycelium-folge292.md` (→ `archiv/`).

**Faltung `## An mycelium`** (mountain-296, gemessen 2026-10-10): bereits in
folge292 gefaltet — GIRO-Manifestation erledigt, SPT-`terms` auf CC0-1.0 gesetzt.
In diesem Atom kein neuer adressierter Block.

## Burn: open 0.0000 · close 0.2672 · cap 0.5 — Grund: Meta-Pass + CI/CDN-Recherche (archive_search --all, 10 Recherchen, Rat Runde 1+2, UI-Roster) + Workflow-Hygiene + cdn-manifest.yml + erster Aufrufer + Azure/R2-Registrierung bis zur Kante · deepseek-flash, kein pro/max (gemessen `session_burn`).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 293) |
| „archive search all und roster recherche … wie wir unsere github, CI und CDN abläufe optimieren können, denkt gross … analyse auch welche kostenlosen externen möglichkeiten z.b. um GPU runs oder läufe zu machen bestehen … auch die kommunikation zw. opencode und github, CI, CDN" | 2026-10-10 | Operator (Session, Mycelium 293) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge292.md` §Operator-Wort-Register | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Manifestation — SPT-3G D1 `cmap`-Block (cmb-cdn-Re-Lauf)
- **Status:** wartend | **Bindung:** eigen (Manifestation)
- **Trigger:** `cmb-cdn`-Lauf `38032912687` Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view`) Mountain hat in `8a9699191` den
  Transfer-Bound in `src/archivar/fetch.rs` erhöht; `cmb-cdn 38032912687` läuft seit 07:00Z
  (`in_progress`). `phi/sources.φ`-SPT-`terms` = `CC0-1.0` (committed).
- **Blockade:** Laufdauer (7,87-GB-Tarball).
- **Braucht:** Abschluss → bei success `sha256` in `phi/sources.φ` (Manifestations-Direktive).

### CI-Hygiene — `matrix-rotor` GitHub-Präemption
- **Status:** wartend | **Bindung:** eigen (Workflow, ggf. linie:river)
- **Trigger:** nächster `matrix-rotor`-Lauf
- **Lage:** (gemessen 2026-10-10 via `ci_manage log 38030708894`) Lauf `38030708894` rot,
  **zweimal** (attempt 1 + 2, Re-Run 06:44Z): `The runner has received a shutdown signal` →
  `rotor slice ended rc=137` nach ~4–7 min (Deckel 18000 s nie erreicht). Sender =
  GitHub-Infrastruktur (hosted-Runner-Präemption), kein Haus-Akteur; der Watchdog-Re-Run
  (einmal) ist verbraucht.
- **Blockade:** der lange `rotor slice` (~5 h) auf gehosteten Runnern wird regelmäßig präemptiert.
- **Braucht:** Präemptions-Mitigation — gecheckpointete Kürzere Slices (State alle 120 s liegt vor)
  oder ein dauerhafter self-hosted Runner; priorisiert in der Survey-Säule A.

### Pipeline — Tianwen-1 MoRIC HIPS-Ernte (32 Shards)
- **Status:** wartend | **Bindung:** eigen (Ernte)
- **Trigger:** Lauf `37932098229` Abschluss
- **Lage:** (gemessen 2026-10-10 via `ci_manage view`) `in_progress`, `attempt 1`,
  `updated_at` `2026-10-10T05:53:27Z`; `phi/pipeline/ledger.φ:110` `ausstehend`.
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
- **Status:** eigen (Umsetzung) | **Bindung:** linie:mycelium (Träger) · Teile linie:mountain/river
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10 via `archive_search --all` + 7 Recherchen + Rat + 6 UI-Seats)
  Survey `docs/surveys/survey-2026-10-10-github-ci-cdn-optimierung.md` (sha `e1b4ab12…`),
  Säulen A–D (Runner/Tests · CDN · Agent↔GitHub/OIDC · Free-GPU/AI) + E–G (CPU-Runs · MCP · APIs);
  Runde `state/stimmen/2026-10-10_mycelium_ci-cdn-roster.md`. Konvergenz: R2/OIDC,
  Workflow-Konsolidierung, nextest+Built-Reuse+Cache-in-R2, GPU nur für GPU-Last. Riss:
  AI-in-CI (nur self-hosted vs. private Cloud-GPU).
- **Blockade:** 6 Maßnahmen, teils große Baum-Arbeit (446 Workflows); je eigener Dispatch.
- **Braucht:** die 6 Maßnahmen je als **eigener, begrenzter Dispatch** (ein Schritt je Atom),
  flash-first: (1) `workflow_call`-Konsolidierung · (2) Lauf-Hygiene — **geschlossen `b1b2bd37f`**
  (16 Einzel-Job-Workflows nachgezogen; Matrix-Workflows job-scoped) · (3) nextest-Sharding auf
  `ubuntu-24.04-arm` + rust-cache/sccache · (4) R2-CDN (`operator-gebunden`: Bucket/Key = Operator-Wort;
  **10 GB frei = Hot-Tier, der 1,76-TB-Bulk bleibt auf GitHub-Release-Assets**, s. Storage-Punkt) ·
  (5) OIDC+R2-Worker-Verifier + `external-state`-Rate-Zeile · (6) Free-GPU-Probe (Modal/Beam) +
  self-hosted LLM — `pending`, hinten.

### Speicher — 1,76 TB Bulk vs. R2-10-GB
- **Status:** eigen (Dokumentation/Architektur) | **Bindung:** linie:mycelium
- **Trigger:** Entlastung/Rebalancing nötig
- **Lage:** (gemessen 2026-10-10 via GitHub-API) `omegaflow/sources` = **344 Releases / 1 763 GB**;
  Release-Assets haben kein Gesamt-/Bandbreitenlimit → der Bulk bleibt **$0** auf GitHub. R2 **10 GB
  frei** trägt nur Manifeste/Indizes (Hot-Tier). Voller S3-Store: R2 ≈$26/mo · R2-IA ≈$17/mo · B2 ≈$12/mo.
- **Blockade:** keine.
- **Braucht:** Architektur-Wort, ob R2 überhaupt als Hot-Tier kommt (sonst ganz weglassen) — in der
  Survey-Säule B dokumentiert.

### Zweite CI-Lane — Azure Pipelines OSS
- **Status:** operator-gebunden (nur der Aktivierungs-Akt) | **Bindung:** linie:mycelium (Vorbereitung)
- **Trigger:** Operator-Wort zur Azure-DevOps-Org-/App-Install
- **Lage:** (gemessen 2026-10-10 via `--exa`/`--linkup`) OSS-Grant **bestätigt**: 10 gratis
  Microsoft-hosted Parallel-Jobs + unbegrenzte Minuten; der 2021-Wechsel betraf private Projekte.
  `ci-gate`-Backlog = Runner-Durchsatz → zweite Lane ist der eigentliche Fix.
- **Blockade:** Azure-DevOps-Org + Azure-Pipelines-GitHub-App = **Akt an Dritten**.
- **Braucht:** Operator-Wort (`/consent`) für Org + App-Install; danach `azure-pipelines.yml` (Entwurf
  liegt als nächster Dispatch-Schritt) + Lane-A/B-Messung.

### MCP — lokale no-leak-Server (Autonomie-Kandidat)
- **Status:** eigen | **Bindung:** linie:mycelium
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) Survey Säule F. opencode-MCP: `local` (stdio) / `remote` (HTTP) in
  `opencode.json`. No-leak-Fit lokal: `filesystem` · `git` · `memory` · `sequential-thinking` ·
  `time`. GitHub-MCP nur **readonly** (hosted = Dritt-Egress). **Nicht verdrahten:** AWS/Azure/GCP/
  hosted Sentry/Slack (lethal trifecta). Docker-Gateway als Isolation.
- **Blockade:** keine.
- **Braucht:** Architektur-Wort (Rat) für den MCP-`block` in `opencode.json` (welche lokalen Server),
  dann als begrenzter Dispatch umsetzen — nach der Konsolidierung.

### Rat Runde 2 — CI/CDN-Architektur (Verdikt + Roster)
- **Status:** eigen (Umsetzung) | **Bindung:** linie:mycelium · Teile operator
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10 via `archive_search --all` + Rat + UI-Runde; Survey §Rat Runde 2)
  Verdikt: (1) `workflow_call`-Extraktion **nach Identität**; (2) Bulk GitHub-Release-Assets,
  R2 Hot-Tier `pending`; (3) Azure-Lane, ein Required-Check je SHA; (4) MCP Docker-Gateway;
  (5) AI-in-CI nur runner-lokal; (6) Sequenz: Duplikat messen → `cdn-manifest.yml` → Azure/MCP.
  Roster GLM 5.3 Flash bestätigt (a–c) + Befund: **Test-Suite-Dedup** (Impact-Analyse) schlägt
  Workflow-Merging zeitlich; Hot-Tier-Eviction (LRU nach Zugriffszeit) jetzt definieren.
- **Blockade:** keine (bis auf Azure-Aktivierung = Operator).
- **Braucht:** Schritt 1 autonom — `sgrep`/sha256 der Step-Rümpfe in `.github/workflows`, dann
  `cdn-manifest.yml` (`on: workflow_call`) + Composite bauen; Azure-Aktivierung s. eigener Punkt.

### Konsolidierung — `cdn-manifest.yml` (Schritt 1 umgesetzt)
- **Status:** eigen (Fortsetzung) | **Bindung:** linie:mycelium
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) Setup byte-identisch: **493/501** Toolchain-Steps direkt nach
  `actions/checkout@v7`; **292** distincte Compiler-Bins (die Run-Rümpfe sind distinct). Kanonischer
  `cdn-manifest.yml` gebaut (`on: workflow_call`, `harvests`-JSON, Job-Matrix, job-scoped
  concurrency) und **erster Aufrufer `planetary-odf-cdn.yml`** migriert (`0a1f7c9dc`, Gate grün).
- **Blockade:** keine.
- **Braucht:** die restlichen **matchenden Ein-Job-`*-cdn`-Dateien** in begrenzten Dispatch-Batches
  migrieren (Aufrufer: `uses: ./.github/workflows/cdn-manifest.yml` + `harvests`-JSON). **Distincte**
  Parser bleiben distinct: `ps1-cdn` (Band-Slab-Logik), `celestrak-eop-cdn` (Zwei-Job-Gate), ….

## An mountain

Origin: mycelium-293 (2026-10-10); fortgeschrieben aus folge292.

- **cmb-cdn / Keogramm — erledigt (mountain-297).** Der Transfer-Bound wurde in `8a9699191`
  erhöht (`cmb-cdn 38032912687` läuft), und `keogram_compiler` zielt auf die jüngste verfügbare
  Nacht (`keogram-cdn 38032914527` success). Die beiden Punkte sind aus diesem Block entfernt.
- **iEEG — Riss, kein Mycelium-Akt.** Dein `## An mycelium` (mountain-296) verlangt
  `phi/harvest.φ`-iEEG-Arm + `sources.φ`-Block. Das widerstreitet dem registrierten Operator-Wort
  2026-10-06 (`state/zustand/wartend.φ:40`): iEEG läuft als **privates Experiment** (Keller-Muster)
  — lokal, kein CDN, keine `sources.φ`. Die Mountain-Seite (`eeglab::eeg_from_bin` akzeptiert
  `Samples::Double`) ist gebaut; der äußere Arm bleibt aus. **Braucht:** dein gemessenes Wort, ob
  ein **neues** Operator-Wort den Riss über 2026-10-06 hinweg aufhebt — sonst bleibt das Wort
  maßgeblich.

## Operator-Hand — vorbereitet bis zur Kante (2026-10-10)

Browser-Profil `mycelium-auth`, drei Tabs offen (Operator führt den letzten Klick):

1. **GitHub-App „Azure Pipelines"** — `https://github.com/apps/azure-pipelines/installations/new/permissions` —
   **vorbereitet:** Konto `omegaflow`, „Only select repositories" → **`omegaflow/omegaflow`** (1 Repo).
   **Operator-Wort erwartet:** der finale Klick **„Install"** (gewährt der App Lese-/Schreibzugriff
   auf Checks, Code, Commit-Status, Deployments, Issues, PRs).
2. **Azure DevOps** — `https://aex.dev.azure.com/` — **Login-Wand** (Microsoft-Konto oder „Anmelden
   mit GitHub"). **Operator-Wort erwartet:** Anmeldung + Org-Anlage (`dev.azure.com/<org>`, Region).
3. **Cloudflare** — `https://dash.cloudflare.com/login` — **Login-Wand** (Google/Apple/GitHub).
   **Operator-Wort erwartet:** Anmeldung + R2 aktivieren (Free-Tier; Karte evtl. nötig) + Bucket +
   API-Token (der Token-Wert bleibt in `.secrets.local`, nie im Transcript).

Jeder Akt ist per-Akt (Konto/Key = Konsensgrenze); die Vorbereitung bis hierher ist autonom gelaufen.

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
- **Meta:** der Stehende Pass trägt die CI-Tafel; der cmb-cdn-Re-Lauf und der Keogramm-Re-point sind von mountain-297 adressiert; `matrix-rotor 38030708894` (GitHub-Präemption, zweimal) bleibt als CI-Hygiene-Punkt.
