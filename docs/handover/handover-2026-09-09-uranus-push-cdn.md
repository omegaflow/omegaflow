<!--
  title: Handover — Uranus-Push + CDN-Manifestation: die sieben Commits stehen als Cherry-Picks auf origin/main (8b2947b → cb53b5b); die eine offene Pflicht ist der CDN-Dispatch von neptune-c-spk-cdn.yml (ephemeris_neptune_c.bin 404); der lokale main ist verzweigt (Rebase-Koordination)
  class: handover
  date: 2026-09-09
  sha256: 2ed89ba4f468aea4192e3c4ad1dcc8e286b8e38bbd19cea094ce341dae834db2
  status: live
  see-also: docs/handover/handover-2026-09-09-uranus-zentrum-kopplung-folge.md docs/handover/handover-2026-09-09-neptun-astrometrie-kopplung.md docs/TODO.md
-->

# Handover — Uranus-Push + CDN-Manifestation

Übergabe für die nächste Sitzung. Die empfangende Sitzung hat ihre sieben
Commits gepusht — als Cherry-Picks, weil der lokale `main` die eigenen und die
Parallel-Session-Commits verzahnt trug. Eine Pflicht bleibt: der CDN-Dispatch.

## 0. Die abgebende Sitzung ist abgeschlossen

Die sieben Commits stehen auf `origin/main` (`8b2947b` → `cb53b5b`, Fast-
Forward). Ursprüngliche Hashes → Cherry-Pick-Hashes:

| original | cherry-pick | Atom |
|---|---|---|
| `62096a3` | `1848c86` | (b) Versionen-Differenz |
| `c481b35` | `f7a6243` | Wobble-Periode |
| `dd394f2` | `f2a72d0` | (c) Neptun-Komposition |
| `d97345f` | `10202c7` | Astrometrie-Erreichbarkeit |
| `5363949` | `5b4ff22` | (c) Neptun-Riß |
| `d0480a4` | `d070ad9` | Trägerjahre |
| `6d47cad` | `cb53b5b` | Übergabe |

Worktree + Branch `push-uranus` sind entfernt. Der Cherry-Pick lief ohne
Konflikte (die TODO-Hunks trafen die Uranus-Zeile, die die Parallel-Session
nicht anfaßt).

## 1. Was auf origin/main steht

Die fünf Proben (`uranus_center_versionenstruktur_probe`,
`uranus_wobble_period_probe`, `neptune_center_check_probe`,
`neptune_center_rift_probe`, `uranus_carrier_years_probe`), die vier Befunde,
`horizons_compiler --neptune-c-spk`, `neptune-c-spk-cdn.yml`, die
`phi/sources.φ`-Registrierung (nep097.bsp + nep097xl-899.bsp +
ephemeris_neptune_c.bin) und die Folge-Übergabe.

## 2. Die eine offene Pflicht — CDN-Dispatch

`ephemeris_neptune_c.bin`, `nep097.bsp`, `nep097xl-899.bsp` sind **404** auf
dem CDN (gemessen 2026-09-09). Der Workflow `neptune-c-spk-cdn.yml` liegt jetzt
auf origin/main, ist aber noch nie gelaufen. Auflösung: den Workflow dispatchen
(oder den Cron 3./Monat abwarten), dann CDN-200 von `ephemeris_neptune_c.bin`
verifizieren; bei rotem Lauf den Schritt lesen. Dieselbe Pflicht trägt auch die
Neptun-Scheinbar-Orts-Kette-Übergabe
(`handover-2026-09-09-neptun-scheinbar-orts-kette.md`) — die beiden Übergaben
zeigen auf denselben Dispatch.

## 3. Die Verzweigung — lokaler main vs origin

Der lokale `main` trägt weiterhin die **originalen** sieben Commits
(`62096a3` … `6d47cad`), verzahnt mit den Commits der Parallel-Session
(`bf4d334`, `a685873`, `f975362`, `685ae5f`, `5914798`, `ce5d172`, …), alle
ungepusht. `origin/main` trägt die Cherry-Pick-Kopien. Der nächste Push der
Parallel-Session ist damit non-fast-forward — sie muß ihre Commits auf
`origin/main` rebasen. Der lokale Arbeitsbaum blieb unangetastet (gestagtes
`te.rs`, uncommittete Seismik-/Archivar-Änderungen).

## 4. Arbeitsregeln für die bauende Sitzung

- **Geteiltes Repo:** nur eigene Dateien stagen, Hunks der Parallel-Session
  nie; `.git/index.lock` kurz abwarten. Eigene Commits über privaten Index
  (`GIT_INDEX_FILE`) + anschließend `git reset HEAD -- <eigene Pfade>`.
- **Commit-Gate:** `cargo check` null Fehler/null Warnungen; `commit_check`.
- **Kalibrier-Gate:** injizierte Fälle exakt zurückgewonnen; die physikalische
  Form der Signatur separat geprüft.
- **0-Kanon:** absent bleibt absent; `descoped` mit Befund, kein Parkplatz.
- **A = A:** Konsistenz ist ein Zwirn, keine Bestätigung.
