<!--
  title: Lageplan — wo was liegt (eine Seite, geladen statt gesucht)
  class: concept
  date: 2026-10-16
  sha256: 6362b7902cc509d715cc106564b204f82f299aa1f4dc5f8eda15f914f81f28b6
  status: live
  see-also: AGENTS.md docs/concepts/house-rules.md docs/concepts/tools-map.md
-->
# Lageplan

Diese Seite ersetzt die Suche. Wer sie liest, muss nicht `fd`, `rg` und `git ls-files`
bemühen, um herauszufinden, **wo etwas liegt**. Ein Eintrag je Ort — mit dem Weg dorthin.

## Die Wurzeln

| Wurzel | was dort lebt | aufgelöst von |
|---|---|---|
| `~/projects/omegaflow` | das öffentliche Repo (Code, docs, state, register) | `bin/_roots.sh` |
| `~/projects/omegaflow-work` | die Werkstatt: `post/`, `sources/`, `experiments/`, `recherche/`, `tmp/` | `$OMEGAFLOW_WORK` |
| `~/projects/omegaflow-personal` | das private Haus (`kern.md`, `mail/`, `zustand/`, `future/handover/`) | `$OMEGAFLOW_PRIVATE` |
| `/tmp/opencode` | Werkzeug-Ausgaben, Artefakte | `$OMEGAFLOW_TMP` |

`bin/_roots.sh` ist der Kanon der Pfade — kein Werkzeug erfindet einen Pfad.

## Der Baum

| Ort | was |
|---|---|
| `phi/sources.φ` | **das Register** — die Quelle der Wahrheit über Quellen, Blöcke, `at <anker>`, `terms`, `ttl` |
| `state/` | der Zustand: `ereignisse.φ`, `standing-pass.md`, `external-state.md`, `wartend.φ` |
| `state/rat/` | die Rat-Antworten (je Frage ein Blatt) |
| `state/mail/`, `state/research/`, `state/reports/` | Post-Ablage, Suchprotokolle, Berichte |
| `docs/handover/` | die **lebenden** Übergaben (je Linie die neueste); `archiv/` = konsumiert |
| `docs/concepts/` | **Wissen und Regeln**: `house-rules.md` (die Essenz) · `tools-map.md` (die Werkzeug-Karte) · `lageplan.md` (diese Seite) · Ethik, Architektur, Methode, Glossar · `archiv/` = Altbestand |
| `docs/surveys/` · `docs/paper/` · `docs/blatt/` · `docs/auftrag/` · `docs/specs/` | Funde · Papiere · Ein-Blatt-Verdikte · Untersuchungsaufträge · Spezifikationen |
| `tools/*/src/bin/` | die eigenen Rust-Binaries (Compiler, Proben, Register, `archive_search`) |
| `bin/` | die Wrapper **auf** diese Binaries + die Wände (`sandbox`, `private_read`) |
| `.github/workflows/` | die CI; das CDN ist `github.com/omegaflow/sources` |
| `~/projects/omegaflow-work/post/` | `in/` = Post an die Leitstelle, `out/` = die Dispatches |

## Die Werkzeuge (Weg dorthin)

- **Suchen:** `archive_search <kw> [--root <dir>]` (der Kanon ist `archive_search --help`) · `rg` für den schnellen Blick · `bin/where <begriff>` vor jeder NET-Suche.
- **Lesen:** `sread <datei> --offset N --limit M` (Fenster; nie ganze Dateien).
- **Register:** `register_lookup` · **CI:** `ci_manage status|list|view|logs` (nie `gh run`).
- **Übergabe:** `omega_sh sha <datei>` (Body ohne Kopf) · `git_safety --close` · Gates: `commit_check`, `open_points_check`.
- **Mail:** `smail` (`--dry-run` ist die Vorgabe; der Send ist die Operator-Hand).
- Die volle Karte: `docs/concepts/tools-map.md`. Die Regel-Essenz: `docs/concepts/house-rules.md`.

## Die zwei Häuser

- **Öffentlich** (`.`, Repo `omegaflow`): sieht Repo + Werkstatt. Das private Haus ist per
  `permissions` gesperrt (`read`/`edit`/`shell` deny auf den privaten Pfad).
- **Privat** (`.`, Repo `omegaflow-personal`): sieht **beides** — privat und öffentlich. Einweg.
- Kein Gateway. Die Grenze sind die Werkzeug-Regeln (`opencode.json`), nicht ein Mount-Käfig.

## Die fünf Stimmen

Mountain · River · Mycelium · Sensory · Future (die Leitstelle) — jede Linie arbeitet **einen Pass**
(→ `house-rules.md`), der Rat entscheidet Architektur und Ethik.
