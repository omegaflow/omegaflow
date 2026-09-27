<!--
  title: Tool-Forms — verbotene Leading-Form → kanonischer Ersatz
  class: concept
  date: 2026-09-27
  sha256: b8c812edd255092fa02c0c8de7ccde115c97c1e6cea948c82103b6cf0eb552a6
  status: live
  see-also: docs/concepts/tools-map.md AGENTS.md
-->
# Tool-Forms — verbotene Leading-Form → kanonischer Ersatz

Diese Karte liegt am Punkt der Handlung: die **erste Handlung** jeder `line`-Session
ist ihr Lesen. Sie ist die Kurzform; der volle Werkzeug-Katalog ist
`docs/concepts/tools-map.md`, die Wahrheit der Muster ist `opencode.json`
(last-matching Rule gewinnt — die breite `"*": "allow"` steht zuerst, die Verbote
danach). Die Verbote werden strukturell durchgesetzt; diese Karte ersetzt die
Erinnerung durch die kopierbare Form.

## Inhalt · Suche · Lesen

| verboten (Leading-Form) | kanonischer Ersatz |
|---|---|
| `ls *`, `*/ls *` | `glob <pattern>` (Discovery) |
| `grep *`, `*/grep *` | `sgrep [-i] [-l] [-c] [-g <glob>] <pattern> [dir]` oder `archive_search <kw> --root <dir>` |
| `rg *` | `sgrep` |
| `cat *`, `*/cat *` | `sread <datei> [--offset N --limit M]` oder das `read`-Tool |
| `cd *` | bash-`workdir`-Parameter |
| `python *` / `python3 *` / `*/python*` | Rust (kein Python im oder für das Repo) |

## Bauen · CI

| verboten | kanonisch (exakt, kein Zusatzflag) |
|---|---|
| `cargo *` (nackt) | `cargo check` · `cargo fmt -- <eigene Pfade>` · `cargo build -p <crate> --bin <name>` · `cargo run -p <crate> --bin <name>` |
| `rustc *` | nie direkt — über `cargo` |
| `gh run list *` / `gh run view *` | `ci_manage list` / `ci_manage view <id>` / `ci_manage log <id>` |
| `sed -i*` | das `edit`-Tool |
| `watch` / `while` / `until` / `sleep` / `tail -f` | nie (kein Polling in einer Session) |

Ein Zusatzflag bricht das Allow-Muster: `cargo run -p …` ist erlaubt, `cargo run -q -p …`
nicht.

## Git (destruktiv = nie)

`git reset` · `git checkout -- …` · `git clean` · `git rebase` · `git stash` ·
`git restore` · `git switch` · `git worktree` · `git push --force`/`-f` sind in
jeder Rolle verweigert. Erlaubt: `status`/`log`/`diff`/`show`/`reflog`/`rev-parse`/
`merge-base`, `add`/`commit`/`mv` im benannten Scope, `push` (Fast-Forward).

## Regel

`./target/release/*` dient dem frisch gebauten eigenen Bin und den benannten Pregates;
die kanonischen Werkzeuge laufen über die `bin/`-Wrapper (Freshness via
`bin/.tools_ensure`). Kein `webfetch`/`websearch` — das Netz läuft über
`archive_search`. `smail` sendet nie durch die Maschine (nur `--dry-run`).
