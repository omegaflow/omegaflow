<!--
  title: Handover — Mycelium-Folge 295 (2026-10-10)
  session: Mycelium-Linie — Meta-Pass. cdn-manifest `crate`-Feld + Failure-Health-Step; 27 Aufrufer migriert; ci-check auf 4-fach-nextest-Shard (ubuntu-24.04-arm); startup_failure-Fix (66654392f) gemessen bestätigt.
  class: handover
  date: 2026-10-10
  sha256: 9be6efe9f939e24bd231278a77ebe84062eaa29c80c3b2b9970f5c144e03b8ee
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

## Burn: open 0.0000 · close 0.0700 · cap 0.5 — Grund: Meta-Pass + cdn-manifest-`crate`/Failure-Health-Step + 27 Aufrufer + ci-check-nextest-Shard · deepseek-flash, kein pro/max (gemessen `session_burn`: 22 Sessions $0.9852 → 29 Sessions $1.3604; Mycelium-295 $0.0699).

## Operator-Wort-Register

| Wort | Datum | Quelle |
| --- | --- | --- |
| „Starte die Mycelium-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-10 | Operator (Session, Mycelium 295; fortgeschrieben aus 294) |
| „bitte öffne die seiten vorausgefüllt" (Maßnahmen 4–6: R2/Modal/Beam) | 2026-10-10 | Operator (Session, Mycelium 295) |
| „archive search all und roster recherche … wie wir unsere github, CI und CDN abläufe optimieren können, denkt gross … analyse auch welche kostenlosen externen möglichkeiten z.b. um GPU runs oder läufe zu machen bestehen … auch die kommunikation zw. opencode und github, CI, CDN" | 2026-10-10 | Operator (Session, Mycelium 293) |
| Vorherige Worte der Linie: `archiv/handover-2026-10-10-mycelium-folge293.md` §Operator-Wort-Register (fetchbar via `git show HEAD:archiv/…`) | 2026-10-10 | gefaltet, nicht kopiert |

## Offen — eigen

### Konsolidierung — `cdn-manifest.yml` (36 Aufrufer; `crate`-Feld + Failure-Health-Step gebaut)
- **Status:** eigen (Fortsetzung) | **Bindung:** eigen
- **Trigger:** nächste Dispatch-Session dieser Linie
- **Lage:** (gemessen 2026-10-10) `cdn-manifest.yml` trägt nun (a) ein optionales
  `crate`-Feld (Default `omegaflow-harvest`), (b) einen **Failure-Health-Step**
  (`gh_issue_once.sh` über den `OMEGAFLOW_TOKEN`-PAT — kein `issues:`-Permission-Grant
  nötig, der PAT trägt). Damit sind **die 25 Health-Wrapper-Caller migrierbar** und
  **migriert** (tap_compiler-Cluster: avo · bzcat5 · cbdata · chandra-csc · corot ·
  eso-harps-rvcat · exoplanets · first14 · frb-a279 · frbcat · gcvs · lmxb · merlin ·
  polarbase · sb9 · sncat · swiftgrb · vsx · wd; dazu cosmicflows · exofop · gaia-sso ·
  pangaea · tess · ztf-fresh). **27 neue dünne Aufrufer** in diesem Atom → **36 total**.
  Zuvor (die Session): `viking-text-cdn` (harvest, Schedule erhalten) + `weberin-verdicts-cdn`
  (`crate: omegaflow-measure`); davor 9. Jede netloc/asset-Kante gegen `phi/sources.φ`
  verifiziert. Der 294-Fix (`66654392f`) ist **bestätigt**: 7/8 re-dispatche
  `startup_failure`-Läufe success; `lro-trk` `in_progress`; `register-coverage` success.
- **Blockade:** keine.
- **Braucht:** der **nicht-exakt-formgleiche** Rest trägt echte Zusatzlogik und bleibt
  distinct: `idempotence`/`gh release create`-Steps (z. B. `qbo-cdn`), `inputs:`-Caller,
  `set -euo`-Mehrzeiler, `curl`/`mkdir`-Kernel-Fetches, Mehr-Asset-in-einem-Lauf
  (`voyager-merged`), zwei-Assets-ein-Compiler (`pioneer-doppler`, dessen vier Assets
  **nicht** in `phi/sources.φ` registriert sind → Mountain-Registrierung), und
  `xp-pilot` (Compiler-Default = `gea.esac.esa.int/xp_spectra.bin`, der Workflow
  überschreibt per `--release-tag dc.g-vo.org` auf einen Pilot → distinct).

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
  (Lauf-Hygiene) `b1b2bd37f`; Maßnahme 1 in Arbeit (s. Konsolidierung). **Maßnahme 3
  (Test-Durchsatz) gebaut:** `ci-check.yml` läuft als **4-fach-Shard-Matrix** auf
  `ubuntu-24.04-arm` über `cargo nextest run --partition count:N/4` (core
  `--features browser_relay`, register+utils, die fünf harvest-Bins); Doctests +
  register_sort/cdn_reconcile/node-Tests bleiben auf Shard 1. Messung (3,5 h → Shard-
  Wall-Clock) nach dem Push-Dispatch.
- **Blockade:** je eigener begrenzter Dispatch (ein Schritt je Atom).
- **Braucht:** die verbleibenden Maßnahmen flash-first: (4) R2-CDN (`operator-gebunden`) ·
  (5) OIDC+R2-Worker-Verifier + `external-state`-Rate-Zeile · (6) Free-GPU-Probe —
  `pending`, hinten. **Offen zu Maßnahme 3:** der ci-check-Shard-Lauf ist zu messen
  (grün 4×?) — `nextest`-Verhalten (doctest-Ausschluss, `--partition`) am Log bestätigen.

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

## An future

Origin: mycelium-295 (2026-10-10).

Operator-gebundene Akte (Survey-Maßnahmen 4–6), bis zur Kante vorbereitet; die Seiten
liegen in der Browser-Gruppe `mycelium-auth` (nicht fokussiert):

- **R2 (Maßnahme 4).** Cloudflare-Login `dash.cloudflare.com` — E-Mail vorausgefüllt
  (`code@omegaflow.space`), Redirect `?to=/:account/r2`. Nach dem Login fehlen: R2
  aktivieren (Plan/„Enable R2") + S3-API-Token anlegen. Vorschlag Bucket-Namen
  `omegaflow`/`omegaflow-sources` (Hot-Tier = Manifeste/Indizes, kein Bulk). Token-ID/Secret
  → `.secrets.local` (nur Schlüsselnamen dort, nie Werte in den Transcript). Der Akt
  (Login + R2 aktivieren + Token erzeugen) = Operator-Hand.
- **R2-Free-Tier (gemessen 2026-10-10, `developers.cloudflare.com/r2/pricing/index.md`):**
  Standard **10 GB-month/Monat** Storage, **1 Mio. Class-A**-Ops, **10 Mio. Class-B**-Ops,
  Egress frei; darüber $0.015/GB-month · $4.50/Mio. Class A · $0.36/Mio. Class B (aufgerundet
  auf die nächste Einheit). Ein Manifest-/Index-Hot-Tier (wenige hundert MB) bleibt also **$0** —
  die Dashboard-Zeile „You will only be charged if you exceed the monthly limits" ist korrekt.
- **OIDC + R2-Worker-Verifier (Maßnahme 5)** hängt an (4): erst nach dem R2-Konto der
  secretlose Pfad (`id-token: write` → Worker prüft GitHub-OIDC → Binding in R2).
- **Free-GPU (Maßnahme 6).** Modal `modal.com/signup` (GitHub/Google-OAuth bereit,
  kein Formularfeld — ein Klick) und Beam `platform.beam.cloud` („Work email"
  vorausgefüllt `code@omegaflow.space`). Konto-Anlage = Operator-Hand; danach die benannte
  Probe (Modal **oder** Beam, Capability-Gate ≥4/4 + Tempo) — kein Roster ohne Messung.
  **Modal-Trust (gemessen 2026-10-10):** SOC 2 Type II (`modal.com/blog/soc2type2`),
  Trust Center `trust.modal.com`, HIPAA + DPA (`modal.com/legal/dpa`), TLS 1.3 in transit
  + at rest, Retention dokumentiert, Inference zero-retention. `omegaflow` ist ein GitHub
  **User** (id `295896184`), kein Org → „Continue with GitHub" registriert unter dieser
  Identität (Scopes am Consent-Screen, nicht gemessen). No-leak-Grenze bleibt: `state/`
  nie an Modal; public-Repo-Code ist der sanktionierte Fall. Nutzung braucht danach
  `modal setup` → Token → `.secrets.local` (Operator-Hand).
  **Zwei gemessene Schranken (2026-10-10, nach dem Operator-Login):** (1) **Das Modal-„Gratis"
  braucht eine Zahlungsmethode** — Workspace `omegaflow` Starter zeigt „You have $1 of
  $30/mo in free credits. Add payment method to unlock the rest"; ohne Karte nur **$1**.
  (2) **Toolchain-Konflikt:** der einzige Modal-/Beam-/Kaggle-Weg ist **Python**
  (`pip install modal` · `python3 -m modal setup`; Beam-CLI ebenso) — das kollidiert mit der
  Haus-Regel „kein Python im oder für das Repo". Maßnahme 6 ist damit nicht nur
  `operator-gebunden` (Karte), sondern ein **Riss gegen die Sprach-Doktrin** — nur ein
  Operator-Lauf eines externen Notebooks (nicht „für das Repo") oder ein non-Python-Pfad
  löst ihn. R2-Kante bleibt der eine Button „Add R2 subscription to my account".

## LOCK

- **SuperDARN Record-Download** — Operator-Wort | 2026-09-29 | „nein super darn musst du nicht messen das lade ich erst herunter wenn ich glasfaser habe." (`state/future/handover/archiv/handover-2026-09-29-future-folge153.md:25`). Kein Maschinen-Akt; Download = Operator-Hand. Das Wort gilt: bis Glasfaser vertagt.

## Abschluss

- **Burn:** `session_burn` open/close — siehe Stehender Pass.
- **Runde:** Mycelium schließt als erste; die Pass-Schreibung (frischer HEAD) folgt nach dem Push.
- **Meta:** der Stehende Pass trägt die CI-Tafel und den Postfach-/Register-Stand.
