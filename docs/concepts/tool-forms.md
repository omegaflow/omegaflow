<!--
  title: Tool-Forms — verbotene Leading-Form → kanonischer Ersatz
  class: concept
  date: 2026-09-27
  sha256: 735862445c7d8720d3c34c15b2748482e0b7058fc5f8d33e309c482c4b6201da
  status: live
  see-also: docs/concepts/tools-map.md AGENTS.md
-->
# Tool-Forms — verbotene Leading-Form → kanonischer Ersatz

Diese Karte liegt am Punkt der Handlung: die **erste Handlung** jeder `line`-Session
ist ihr Lesen. Sie ist die Kurzform; der volle Werkzeug-Katalog ist
`docs/concepts/tools-map.md`, die Wahrheit der Muster ist `opencode.json`
(last-matching Rule gewinnt — die breite `"*": "allow"` steht zuerst, die Verbote
danach; ausdruckbar mit `omega_sh perms [<agent>]`). Die Verbote werden strukturell
durchgesetzt; `.opencode/plugin/form-guard.ts` bricht eine verbotene Leading-Form
zusätzlich mit ihrer kanonischen Form als Meldung ab (der Deny lehrt). Diese Karte
ersetzt die Erinnerung durch die kopierbare Form.

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
`git restore` · `git switch` · das Arbeitsbaum-Kommando · `git push --force`/`-f` sind in
jeder Rolle verweigert. Erlaubt: `status`/`log`/`diff`/`show`/`reflog`/`rev-parse`/
`merge-base`, `add`/`commit`/`mv` im benannten Scope, `push` (Fast-Forward).

## Regel

`./target/release/*` dient dem frisch gebauten eigenen Bin und den benannten Pregates;
die kanonischen Werkzeuge laufen über die `bin/`-Wrapper (Freshness via
`bin/.tools_ensure`). Kein `webfetch`/`websearch` — das Netz läuft über
`archive_search`. `smail` sendet nie durch die Maschine (nur `--dry-run`).
