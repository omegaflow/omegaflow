<!--
  title: GitHub-Pipeline — der Kreislauf: Ernte, Disposition, Manifestation auf GitHub
  class: concept
  date: 2026-09-10
  sha256: ad5773f07f36fd21cbe5dbda356ce7e32975b3626f26e9921a11624ed002f659
  status: live
  see-also: docs/SOURCE_PORT.md docs/handover/handover-2026-09-10-autonom.md docs/handover/handover-2026-09-10-nicht-autonom.md
-->
# GitHub-Pipeline — der Kreislauf

Der Plan, die omegaflow-Pipeline (Ernte → Linse → Probe → Review → Manifestation)
auf GitHub laufen zu lassen — der Operator als Reviewer am Tor statt als lokaler
Hand-Prober. Stand 2026-09-10, gemessen aus drei Taucher-Recherchen
(GitHub Actions, GitHub Copilot, GitHub-Angebote jenseits davon).

## Der Befund (gemessen)

- Das Repo ist **public** → Standard-Runner, CodeQL, Secret-Scanning,
  Push-Protection, Pages sind **unbegrenzt gratis**.
- Die **46 Provider-Keys** liegen in GitHub Secrets (`gh secret list`).
- Die **Korpora** (~120 MB Katalog-Inventare) können als **Release-Assets**
  in `omegaflow/sources` liegen: 1000 Assets/Release, je <2 GiB, **kein
  Bandbreiten-Limit** — exakt der bestehende CDN-Pfad. **Kein privates Repo,
  kein LFS** (LFS hat Quota- und Bandbreiten-Falle).
- **GitHub Models ist eingestellt** (30.07.2026) — LLM-Inferenz im Workflow
  ist tot. Ersatz: Azure AI Foundry (secretless via OIDC) oder opencodes
  eigener Provider.
- Die Test-Suite läuft **3,5 h** (768 Tests) — der größte Kostenpunkt.

## Die Entscheidungen

| Frage | Entscheidung | Grund |
|---|---|---|
| Korpora-Heim | **Release-Assets** (`omegaflow/sources`) | kein Limit, ist der CDN-Pfad; LFS/privates Repo verworfen |
| 3,5-h-Tests | `cargo-nextest` + `--partition`-Sharding + `sccache` | gratis auf public, ~30 min |
| Copilot | cloud agent → Draft-PR, Review bleibt menschlich | AGENTS.md ist schon sein Konfig-Kanal |
| LLM im Workflow | Azure AI Foundry (OIDC) nur wenn nötig | GitHub Models tot |
| GPU | self-hosted GTX 970 (compute-only) oder lokal | Codespaces hat kein GPU |

## Der Kreislauf (drei Stufen)

### Stufe 1 — Discovery (Cron + Dispatch)

Ein geplanter Workflow (wöchentlich) lädt die Korpora vom Release-Asset, fährt
die Linse (`source_url_candidates`), dann den Probe-Sweep (Reachability/Parse).
Survivors + Void als Artifact oder Branch. Das ist die mechanische Front —
heute lokal von Hand.

### Stufe 2 — Disposition (Review-PR)

Die Survivors gehen als PR mit vorgeschlagener Disposition ein.
`decline_lens.φ` + `bucket_litmus` (gebaut 2026-09-09, kalibriert FP 3 / FN 12)
klassifizieren automatisch. Der Operator reviewt, entscheidet nur die
Urteils-Fälle (Force-Gate, „echter Oszillator?", Duplikat), der Merge schreibt
in `sources.φ` / `dead_sources.φ`. Der Copilot-cloud-agent kann neuartige
Kandidaten vorschlagen.

### Stufe 3 — Manifestation (läuft schon)

Die Compiler fetchen → kompilieren → CDN (`--ci-mode`). Besteht:
`kernel-flatten.yml` + die `*-cdn.yml`-Workflows.

## Das Tor (Review bleibt, Fabrikation stirbt)

Die 14k-URL-Fabrikation war eine **ungegate** Cron. Der Kreislauf ist gated:
**Environments mit `required reviewers`** + **PR-Review**. Nichts wird
manifestiert oder ins Register geschrieben, ohne dass der Operator den PR
freigibt. GitHub macht die mechanischen 90 %, der Operator die 10 % Urteil.
Das ist technisch exakt die Consent-Ethik („erst fragen, dann strahlen").

## Tests (3,5 h → <1 h)

`cargo-nextest` (Prozess pro Test) + `--partition count:i/N` über eine
Matrix + Build-einmal-`--archive-file` + `sccache`/`rust-cache`. Acht Shards
auf Standard-Runnern: ~30 min Wanduhr, $0 auf public.

## Copilot (Code-Atome)

Der **cloud agent** liest ein Issue → plant → schreibt auf `copilot/`-Branch →
führt `cargo check`/Tests in eigener Actions-Umgebung aus → Draft-PR. Grenze:
kein Approve/Merge, Workflows laufen erst nach „Approve and run". Eignung: hoch
für abgegrenzte Atome („schreibe GRIB-2-Section-Reader std-only gegen Referenz
X"). Harte Reader (Parquet/GRIB-2/OPeNDAP) nur atomar zerlegt + Referenzmaterial.
**Required-Check-Gate:** `cargo check` 0/0 als Pflicht-Status — kein Agent-PR
rutscht durch. `AGENTS.md` ist der Konfig-Kanal (0 honored, keine Fallbacks,
kein `unwrap_or`).

## Sicherheit

Auf public gratis: **Secret Scanning** + **Push Protection** (46 Keys nicht
leaken) + **CodeQL** (Rust) + **Dependabot**. Push-Protection ist bei den
Provider-Keys Pflicht.

## Arbeitsplatz

- **Codespaces** (CPU): reproduzierbarer Rust-Arbeitsplatz, kein GPU —
  WebGPU nicht verifizierbar dort. Für Curation/`cargo`/curl.
- **GPU**: lokal oder self-hosted GTX 970 (compute-only, `compatible_surface:
  None`). Sicherheitsregel: self-hosted Runner nie ungeprüfte Fork-PRs.

## Die Schritte (Setup-Reihenfolge)

1. **Korpora → Release-Assets** bewegen; der Discovery-Workflow lädt sie von dort.
2. **Discovery-Workflow** (cron + `workflow_dispatch`): Linse + Probe, Artifacts.
3. **Disposition-PR**: `decline_lens`/`bucket_litmus`-Output als PR; Environment
   `harvest` mit `required reviewers`.
4. **Test-Sharding**: `cargo-nextest` + Matrix in `ci-check.yml` (3,5 h → <1 h).
5. **Required Checks**: `cargo check` 0/0 + Scan als Pflicht-Status.
6. **Copilot cloud agent** aktivieren; erste Atome als Issues.
7. **Security**: Secret Scanning + Push Protection + CodeQL aktivieren.
8. **Codespaces-`devcontainer.json`** (Rust + WebGPU-CPU-Fallback).

## Grenzen (0 honored)

- Der Plan ist ein Befund aus drei Recherchen, keine Messung des gebauten
  Zustands. Jeder Schritt trägt seine eigene Messung, wenn er gebaut wird.
- Copilot-Automations (Zeitplan/Issue-Trigger) brauchen ein privates/internes
  Repo — das Repo ist public, also bleibt der Trigger manuell (Issue-Zuweisung).
- Larger/GPU-Runner sind Team/Enterprise und per-minute — im Plan nur als
  Option benannt, nicht vorausgesetzt.
