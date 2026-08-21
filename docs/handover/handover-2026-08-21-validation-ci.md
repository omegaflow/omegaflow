<!--
  title: Handover: Validation & CI — die offenen Gate- und Pipeline-Posten
  class: handover
  date: 2026-08-21
  sha256: 42d93c9a237f31aa6a5c36dd5f767e0a677980996eb655550c7b16b401135fd0
  status: live
  see-also: TODO.md docs/handover/handover-2026-08-21-offene-atome.md docs/handover/handover-2026-08-21-source-port-pipeline.md
-->

# Handover: Validation & CI

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext. Der Auftrag
ist nicht die Ausführung; ausgeführt wird erst auf das Wort des Operators.

Register-Quellen: TODO.md, Abschnitte „Validation" und „CI Pipeline".
Ein Fenster trägt EINE Einheit.

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # sauber oder fremde Arbeit nennen
cargo check                      # 0/0
grep -n -- "--verify\|--reverify" src/archivar.rs | head
ls .github/workflows             # die fünf Workflows (siehe Fundort-Befund)
```

Referenzen (stehend): `src/archivar.rs` (--verify bei 15891, --reverify
bei 15984, reverify_mode bei 13573), `docs/SOURCE_PORT.md` §5,
`phi/pipeline/stage/recheck_live.φ`, `phi/pipeline/refusal_ledger.φ`,
TODO.md.

## Die Einheiten — Validation

### A. --verify lädt noch keine Quellen

Die CLI existiert (URL-Erreichbarkeit, src/archivar.rs:15891); das Laden
der Quellen (der eigentliche Verifikations-Lauf gegen das Register) fehlt.

### B. Test-Limit der Curation

Über 200 Blöcke hinaus erhöhen; 6 Rest-FAILs sind Daten-Artefakte
(docs/SOURCE_PORT.md §5).

### C. VirES-Vollprobe + DONKI-Familie

Ergebnis-Dateien ABSENT (Schreibverlust) — Nachlauf in Blöcken offen.

### D. MSL/MEDA-field-Pfade end-to-end

test_live_sources_extract deckt nur die ersten 200 Blöcke — die
MSL/MEDA-Pfade end-to-end verifizieren.

### E. Firefox-Laufzeit-Verifikation

BiDi-Weg: user.js mit dom.webgpu.enabled + devtools-Prefs, WS auf
/session — offen.

### F. Backlog-Test-Reparaturen

Template-Keyed-Dedupe, Letzter-Block-Flush, Limit zählt nur
Fetch-Blöcke, LSK-Volltabelle — unverifiziert, ob mit dem
Parallel-Session-Commit gezogen.

### G. AGOS-Quarantäne

Katalog endet 2022-02-05 — Kompilat-Kandidat über den CDN-Weg.

### H. EA-Fanout

Runtime-Fanout-Lauf offen (der Test überspringt Fanout designbedingt).

## Die Einheiten — CI

### I. I02-Rest

Das Python refresh.yml im sources-Repo bleibt auf Python — Abschaltung
nach Verifikation der Rust-Katalog-Kompilate im
kernel_flatten-catalogs-Job (ein Produzent pro Asset). In diesem Repo
trägt healthcheck.yml die Rolle (cargo run -- --verify phi, 3-h-Cron,
Anomalie-Issues) — Fundort siehe Befund unten.

### J. Token-Rotation — ERLEDIGT (2026-08-21)

Der git-Remote-Token wurde erneuert: origin trägt nur noch die saubere
URL, git läuft über den gh-credential-helper (`gh auth setup-git`), gh
trägt den neuen Token im keyring (Scopes read:org/repo/workflow), das
Actions-Secret `OMEGAFLOW_TOKEN` ist aktualisiert.

### K. Stray-/Basename-Assets

Im Release ssd.jpl.nasa.gov löschen.

### L. Compiler-Builds zahlen den wgpu-Compile mit

Harte Dependency — der wgpu-Baum im CI-Compiler-Pfad. (Das
feature-gate-`gpu`-Atom der Archivar-Übergabe ist die strukturelle
Antwort — diese Einheit ist die CI-Seite.)

### M. CDN-Asset-Naming

`{name}.json` — Konvention ist der Resolver (Regel, kein Auftrag; bei
jedem neuen Asset anwenden).

## Fundort-Befund (2026-08-21, bezeugt)

Die fünf Workflows dieses Repos liegen in `.github/workflows/` und sind
in git getrackt (git ls-files bezeugt sie): `ci.yml`, `healthcheck.yml`,
`kernel_flatten.yml`, `pages.yml`, `probe_sweep.yml`. Die Register-Zeilen
(TODO CI-Pipeline, AGENTS) tragen die Wahrheit. Ein früherer glob-Lauf
fand sie nicht — der glob-Werkzeug-Blick durchdringt das versteckte
`.github/`-Verzeichnis nicht; das war ein Instrument-Befund, kein
Baum-Befund. Bezeugt 2026-08-21: healthcheck.yml trägt verify/reverify/
pages-verify/probe-smoke/probe-full/sources-package (3-h-Cron),
kernel_flatten.yml trägt index/bodies/catalogs/chunk_catalogs/sun
(monatlich + dispatch). Benannt: die lokale Arbeitskopie von
`kernel_flatten.yml` trägt uncommittete Änderungen (git status: M) —
vor jedem CI-Schnitt `git diff .github/workflows/` begutachten und die
Änderung benennen.

## Root-Ursachen der Healthcheck-Fehler (Diagnose 2026-08-21)

Drei Quellen der wiederkehrenden health-Issues — zwei behoben, eine ist
Kurationsdrift:

### N. pages 404 — drei Ursachen, alle behoben (2026-08-21)

- GitHub Pages war für das Repo nicht aktiviert (`GET /pages` → 404).
  Aktiviert: `gh api -X POST repos/omegaflow/omegaflow/pages
  -f build_type=workflow` — html_url ist jetzt
  https://omegaflow.github.io/omegaflow/.
- Ein Run von 2026-08-06 (31122094424) hing 357 h in `waiting` und hielt
  die `concurrency`-Group `pages` (`cancel-in-progress: false`) fest —
  jeder folgende Pages-Run reihte sich `pending` dahinter und lief nie
  an (0 Jobs), bis er gecancelt wurde. Ghost-Run gecancelt.
- Der frische Dispatch legte die dritte Ursache frei: der macOS-Release-
  Build bricht mit E0432 ab — `use winit::platform::wayland/x11`
  (Linux-only, nicht gegated) in src/mathematikerin.rs; die Matrix
  cancelt, kein Deploy. Lokal behoben: `#[cfg(target_os = "linux")]` auf
  die zwei Imports + ihre `with_any_thread`-Aufrufe; cargo check 0/0
  (Linux). Braucht commit + push + Re-Dispatch, um macos/windows/ubuntu
  zu bezeugen.
- Offen: den Deploy-Abschluss bezeugen (pages-Job nach build/probe/
  termux) und die Custom-Domain `omegaflow.space` verifizieren (DNS +
  CNAME-Datei; pages-verify prüft diese Domain, nicht
  omegaflow.github.io).

### O. verify „31 tote Quellen" — Kurationsdrift, kein Bug

`--verify phi` meldet 31 tote Quellen + 81 Anomalien → exit 1 (so ist es
gebaut: der Healthcheck benennt Drift). Die Kuration gehört in die
Source-Port-/Katalog-Lücken-Übergaben + `phi/pipeline/ledger.φ`.
Separater kleiner Befund: beim CDN-Spiegeln `gh returned void: release
not found` (49/214 gespiegelt) — Code-Pfad im ci-mode-Spiegel prüfen
(fehlendes `--repo omegaflow/sources`? oder Token-Umfang). Eigenes
Fenster.

### P. Dedupe — ERLEDIGT (2026-08-21, uncommitted)

Neu: `.github/workflows/scripts/gh_issue_once.sh` (legt eine Issue nur
an, wenn keine offene mit gleichem Titel existiert). 42 Aufruf-Stellen
umgestellt (ci 2, healthcheck 3, kernel_flatten 37). 11 Duplikat-
health-Issues geschlossen. Wirkt erst nach commit + push.

## Verifizierter Kontext (2026-08-21)

- Register-Datenqualität ist abgeschlossen (Gate-Konsistenz, vier
  void-Klassen, refusal_ledger.φ, recheck_live.φ) — Geschichte, keine
  Arbeit.
- Der grüne chunk_catalogs-Lauf trägt `handover-2026-08-21-offene-
  atome.md` (Atom 4) — NICHT anfassen.
- CI-Schreibwege (--ci-mode, CDN-Manifestation) gehören zur
  Source-Port-Übergabe; hier nur die Pipeline-Posten.

## Gates

- cargo check 0/0 (vier Kombis), cargo test komplett — Tests laufen
  kopf-los, kein Fenster, kein Netz-Push in Test-Pfaden.
- Jede CI-Änderung endet mit einem bezeugten Lauf (oder benanntem
  Verzicht) und Register-Zeilen im selben Commit.
- Diese Datei nach dem Abschluss archivieren (Regel in AGENTS.md).

## Nicht anfassen

Source-Port-Pipeline, Katalog-Lücken, offene-atome, Archivar-
Arbeitsliste, 4D-Wahrheit.
