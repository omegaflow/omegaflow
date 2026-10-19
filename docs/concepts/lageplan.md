<!--
  title: Lageplan — wo was liegt (eine Seite, geladen statt gesucht)
  class: concept
  date: 2026-10-19
  sha256: 010231a1fce027a567370240257e44fdd76bf4f124af03982fdc89ec1826371b
  status: live
  see-also: AGENTS.md docs/concepts/house-rules.md docs/concepts/tools-map.md
-->
# Lageplan

Diese Seite ersetzt die Suche. Wer sie liest, muss nicht `fd`, `rg` und `git ls-files`
bemühen, um herauszufinden, **wo etwas liegt**. Ein Eintrag je Ort — mit dem Weg dorthin.
Stand nach der Sortierung (2026-10-19): `state/` ist aufgelöst, Post und Handover liegen
in der Werkstatt, `/tmp` hat ein festes Gerüst.

## Die Wurzeln

| Wurzel | was dort lebt | aufgelöst von |
|---|---|---|
| `~/projects/omegaflow` | das öffentliche Repo: Code, `docs/`, `phi/`, `bin/`, `tools/`, CI | `bin/_roots.sh` |
| `~/projects/omegaflow-work` | die Werkstatt: `post/`, `handover/`, `zustand/`, `sources/`, `experiments/`, `recherche/` | `$OMEGAFLOW_WORK` |
| `~/projects/omegaflow-private` | das private Haus (`kern.md`, `mail/`, `zustand/`, `daten/`, `nicht-redistributierbar/`) | `$OMEGAFLOW_PRIVATE` |
| `/tmp/opencode` | die **eine** flüchtige Wurzel: `ci/`, `out/`, `downloads/`, `scratch/` | `$OMEGAFLOW_TMP` |
| `~/archive` | datierte Altbestände (im Backup), z. B. die grosse Sortierung | `backup-home` |

`bin/_roots.sh` ist der Kanon der Pfade — kein Werkzeug erfindet einen Pfad.

## Das öffentliche Haus

| Ort | was |
|---|---|
| `phi/sources.φ` | **das Register** — Quelle der Wahrheit über Quellen, Blöcke, `at <anker>`, `terms`, `ttl` |
| `docs/concepts/` | **Wissen und Regeln**: `house-rules.md` (die eine Denk-Quelle) · `granit.md` (Axiome) · `council.yaml` (Stimmen) · `tools-map.md` · diese Seite · `archiv/` = Altbestand |
| `docs/surveys/` · `docs/paper/` · `docs/blatt/` · `docs/specs/` · `docs/auftrag/` · `docs/reference/` | Funde · Papiere · Ein-Blatt-Verdikte · Spezifikationen · Aufträge · Referenzkarten |
| `tools/*/src/bin/` | die Rust-Binaries (Compiler, Proben, Register, Gate) |
| `bin/` | Wrapper auf die Binaries + die Messwerkzeuge (`lage`, `layout_check`, `sources_index`, `where`, `sread`) |
| `.github/workflows/` | die CI; das CDN ist `github.com/omegaflow/sources` |

## Die Werkstatt

| Ort | was |
|---|---|
| `post/` | **ein** Ordner; der Name trägt die Richtung: `<datum>-<von>-an-<an>-<slug>.md` |
| `handover/` | die **lebenden** Übergaben (je Linie die neueste, ≤ 1 KiB); `archiv/` = konsumiert |
| `zustand/` | Laufzustand: `rat/` (Formulare, Sammlungen, `_rat-register.md`), `ledger/`, `gateway/`, `mountain/`, `operator-gespraeche/` |
| `sources/` | der CDN-Cache: ein Ordner je Netloc + `INDEX.tsv` (erzeugt, 126 944 Dateien, 92 GiB) |
| `experiments/` | nur lokal, wiederholbar; nie im CDN |
| `recherche/` | `funde/` (Belege) · `index/` · `tmp/` (Spill der Suche) |

## Die flüchtige Wurzel

`$OMEGAFLOW_TMP` hat **genau vier** Ordner, jeder mit Frist:

| Ordner | Inhalt | TTL |
|---|---|---|
| `ci/` | `ci_watchdog.log`, `.seen` | keins (wird überschrieben) |
| `out/<datum>/` | Werkzeug-Ausgaben | 7 Tage |
| `downloads/<datum>/` | Rohladungen (`sfetch`/`curl`-Ziel) | 14 Tage |
| `scratch/<datum>-<zweck>/` | ein Lauf, ein Ordner | 2 Tage |

Ein fünfter Ordner ist ein Fehler, kein Sonderfall. Was bleiben soll, wandert **einbahnig** nach
`sources/`, `experiments/` oder `docs/`. `ci_status.md` bleibt als einzige Datei an der Wurzel.
`bin/layout_check` misst das (und `--fail` macht daraus ein Gate).

## Die Wege

- **Suchen:** `archive_search <kw> [--root <dir>]` · `bin/where <begriff>` vor jeder NET-Suche · `rg` für den schnellen Blick.
- **Lesen:** `sread <datei> --offset N --limit M` (Fenster; nie ganze Dateien).
- **Register:** `register_lookup` · **CI:** `ci_manage status|list|view|logs` (nie `gh run`).
- **Ablage messen:** `bin/layout_check` · **Cache-Karte:** `bin/sources_index`.
- **Übergabe:** `omega_sh sha <datei>` (Body ohne Kopf) · `git_safety --close` · Gates: `commit_check`, `open_points_check`.

## Die zwei Häuser

- **Öffentlich** (`.`, Repo `omegaflow`): sieht Repo + Werkstatt. Das private Haus ist per
  `permissions` gesperrt (`read`/`edit`/`shell` deny auf den privaten Pfad).
- **Privat** (`.`, Repo `omegaflow-private`): sieht **beides** — privat und öffentlich. Einweg.
- `opencode.json` ist **lokal und nicht versioniert** (`.gitignore`): die Schreibwände leben auf
  der Maschine, nicht im Klon. Ein frischer Klon hat sie nicht.

## Die fünf Stimmen

Mountain · River · Mycelium · Sensory · Future (die Leitstelle) — jede Linie arbeitet **einen Pass**
(→ `house-rules.md`), der Rat entscheidet Architektur und Ethik.
