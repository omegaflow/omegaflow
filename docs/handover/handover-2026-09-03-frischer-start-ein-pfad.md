<!--
  title: Frischer Start — omegaflow neu, ein Pfad, kein Branch-Legacy
  class: handover
  date: 2026-09-03
  status: live
-->

# Übergabe — omegaflow frischer Start (ein Pfad)

Diese Session hat den vollständigen Neustart von omegaflow vollzogen: aus dem
alten Branch-/Strang-/Leitstellen-Modell wurde ein frisches, einliniges,
public-Repo ohne jede Verzweigungs-Vergangenheit. Der aktuelle Stand ist
committed und gepusht bis `951c024`.

## Was erreicht wurde (gemessen, committed)

- **Repos**: `omegaflow/omegaflow` (public, frisch) + `omegaflow/omegaflow-legacy`
  (privat, archiviert, unsichtbar) + `omegaflow/sources` (public, CDN-Quelle, unberührt).
- **Frische Historie**: 1 saubere main-Linie, keine Branch-Commits. Das Alte ist
  doppelt gesichert: `/home/johannes/projects/omegaflow-legacy/` + Bundle
  `/home/johannes/projects/archive/omegaflow-legacy-backup-2026-09-02/`.
- **Struktur**: `src/` = 3 Funktionsmodule (archivar/mathematikerin/gate);
  `tools/` = 7 Funktions-Crate (harvest/measure/register/service/science/gate/utils).
- **CI**: 15 Workflows kategorien-einheitlich benannt (`X-cdn`, `X-cdn-watch`,
  2-Wort-Meta); `ci-check` ist grün (Build + Tests).
- **Runs/Issues**: beide Repos 0/0 (alle gelöscht für frischen Start).
- **Secrets**: 77+ im neuen Repo aus `.secrets.local` (alle Workflow-genutzten da).
- **Gate**: `single-path`-Regel blockt Verzweigungs-Vokabular-Rückkehr in Code/Doku.
- **Werkzeug**: `tools/register/.../path_reference_scan.rs` — findet fehlende
  Datei-Referenzen (MISS) + absolute Pfade (ABS) in committed Dateien.
- **Strang-Mechanik entfernt**: `.githooks/pre-commit`+`pre-push` (die main-Commits
  blockten), `make_worktree`, `spawn-thread`, `strand-worktree.sh`,
  `.omegaflow-strand`, alle Branch-Wörter aus Code/Doku/Gate.

## Laufende Arbeit (UNCOMMITTED — muss die nächste Session committen)

Die absolute-Pfad-Ersetzung auf `archive-root` ist teilweise ausgeführt, noch
nicht committed. `git status` zeigt modifiziert:
- `AGENTS.md` (archive-root-Definition, Z.166; Z.333/350/356 umgestellt)
- `docs/README.md`, `docs/SOURCE_PORT.md`, `docs/TODO.md`,
  `docs/concepts/kybernetische-astrophysik.md`, `docs/concepts/the-seven-spheres.md`
- `docs/granit.md`, `docs/paper/corona-heating-ladder.md`,
  `docs/paper/probe-front-dark-matter.md`, `docs/paper/text-as-data-pioneer.md`
- `docs/specs/eraen.md`, `docs/specs/docs-naming-versioning.md`
- `docs/surveys/survey-auswertung.md`

**Regel neu (in AGENTS.md gesetzt)**: die externe Archiv-Wurzel heißt `archive-root`;
die physische Adresse (`/home/johannes/projects/archive/`) steht NUR in AGENTS.md
Z.166. Jede andere Datei schreibt `archive-root/...`, nie den absoluten Pfad.

## Noch offen

1. **Committe die laufende archive-root-Ersetzung** (Liste oben) + `cargo check`.
2. **Sonderfälle absolute Pfade noch nicht ersetzt**:
   - `cloudflare/cloudflared-config.yml:2` — `/home/johannes/.cloudflared/...json`
     (Tunnel-Zertifikat, systemd-Config; braucht den absoluten Credential-Pfad —
     prüfen ob committed werden soll oder nur lokal gilt).
   - `docs/surveys/survey-auswertung.md:9` — `/home/johannes/Schreibtisch/survey/`
     (externe Experten-Surveys; → `archive-root`-Referenz oder entfernen).
3. **`path_reference_scan` als CI-Wächter**: Als `#[test]` einbauen, der bei
   neuen toten Links / absoluten Pfaden fehlschlägt. Aktuell meldet er
   **233 MISS** (verwaiste see-also-Links auf archivierte handover-2026-08-*,
   survey-2026-08-*, entfernte concepts) + **~17 ABS**. Diese verwaisten Links
   sind die echte Folge der Archivierung — müssen bereinigt werden (siehe unten).
4. **84+ verwaiste see-also-Links bereinigen**: Die Paper/Specs verweisen auf
   Dateien die ins Archiv wanderten (handover-2026-08-*, survey-2026-08-*,
   concepts/broken-null-control, concepts/livefeed-gate etc.). Zwei Optionen:
   (a) Link auf `archive-root/...`-Pfad korrigieren, oder (b) das verwaiste
   Konzept aus dem Archiv zurückholen. `TODO.md`-Abkürzungen sind legitim
   (Register = docs/TODO.md), aber technisch fehlen sie am Root — klären.
5. **6 funktionslose Alt-Secrets** (ARBIMON_KEY, ICIMOD_RDS_*, PROXY,
   WILDLIFE_INSIGHTS_KEY): nur im alten privaten Repo, keine lokale Quelle,
   kein Workflow nutzt sie. Nicht übertragbar per CLI (gh liest Secret-Werte
   nicht). Bei Bedarf frische Keys anlegen — aber die 3 Dienste werden in
   omegaflow nirgends als Quelle kuratiert.

## Werkzeuge

- Referenz-Scan: `cargo run -p omegaflow-register --bin path_reference_scan -- .`
- `path_reference_scan` skippt: eigene Datei, `docs/reference/`, ledgers.
  Scannt nur **committed** Dateien (via `git ls-files`).

## Legacy / Altes

- Alter Ordner: `/home/johannes/projects/omegaflow-legacy/` (14 GB, mit eigenen
  `.secrets.local` + `docs/`). Nur als Sicherung, wird nicht gepusht.
- Externes Archiv: `/home/johannes/projects/archive/` (Bundle + dateidocs).
- `.secrets.local` Quellen: `/home/johannes/projects/{omegaflow,omegaflow-legacy}/`
  + `backups/omegaflow/` + `backups/state/omegaflow/`.
