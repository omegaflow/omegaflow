// form-guard — der Deny lehrt.
//
// Bricht eine verbotene Leading-Form mit der kanonischen Form als Meldung ab,
// statt in den generischen Deny zu laufen: die erste Ablehnung IST die Messung.
// Spiegel der Deny-Liste aus `opencode.json`; die Wahrheit der Muster bleibt dort
// (`omega_sh perms [<agent>]` druckt sie konfigurationsgestützt).
//
// Grenze: der Hook kann eine Bash nicht in den `glob`-Agenten umschreiben — er
// lehrt nur beim Stopp. Wirkt nur, wenn `tool.execute.before` vor der
// Permission-Auflösung läuft; sonst greift weiterhin der strukturelle Deny.
import type { Plugin } from "@opencode-ai/plugin"
import { readFileSync } from "node:fs"

// Fenster-Gate (Rat + Roster 2026-10-14): eine ganze Datei am Stück ist der
// größte Kontext-Multiplikator — dieselben Bytes werden jede Runde erneut
// tokenisiert (gemessen: die größten Nachrichten-Teile sind ganze `read`s bis
// 130 KB; Kontexte bis 600k Token/Turn). Über dieser Zeilenzahl (2^8) wird
// ganz gelesen abgelehnt; der volle Inhalt bleibt über die Spill-Datei
// (`full: <pfad>`) erreichbar — Fenster benennen, nie still deckeln.
const WHOLE_READ_MAX_LINES = 256

function lineCount(path: string): number | null {
  try {
    const s = readFileSync(path, "utf8")
    let n = 1
    for (let i = 0; i < s.length; i++) if (s.charCodeAt(i) === 10) n++
    return n
  } catch {
    return null
  }
}


const TOKEN_TEACH: { tokens: string[]; msg: string }[] = [
  { tokens: ["ls"], msg: "`glob <pattern>` (Discovery) — nicht `ls`. Pfad-Index: `archive_search --index`." },
  { tokens: ["find"], msg: "`glob <pattern>` oder `fd <name>` (Discovery) — nicht `find`; keine Baum-Listen (Kontext-Fresser)." },
  { tokens: ["grep"], msg: "`sgrep [-i] <muster> [dir]` oder `archive_search <kw> --root <dir>` — nicht bash-`grep`." },
  { tokens: ["rg"], msg: "`sgrep [-i] <muster> [dir]` — nicht `rg`." },
  { tokens: ["cat"], msg: "`sread <datei> [--offset N --limit M]` oder das `read`-Tool — nicht bash-`cat`." },
  { tokens: ["head", "tail"], msg: "`sread <datei> --offset N --limit M` oder `read` (offset/limit) — nicht `head`/`tail` (kastrieren das Ergebnis); einen gekappten Tool-Output vollständig aus der Spill-Datei (`full: <pfad>`) lesen." },
  { tokens: ["copilot", "copilot_ask", "copilot_review"], msg: "`bin/session_check --in|--out` (lokal, deterministisch) — nicht `copilot` (Cloud erhält keine omegaflow-Daten; Operator-Wort 2026-09-28)." },
  { tokens: ["python", "python3"], msg: "`cargo` (Rust) oder die kanonischen Tools — nicht Python." },
  { tokens: ["cd"], msg: "der `workdir`-Parameter des bash-Tools — nicht `cd`." },
  { tokens: ["rustc"], msg: "`cargo` — nicht `rustc`." },
  { tokens: ["watch", "sleep", "inotifywait", "entr"], msg: "kein Polling — `watch`/`sleep`-Schleifen sind in einer Session verboten." },
]

const PHRASE_TEACH: { phrases: string[]; msg: string }[] = [
  { phrases: ["sed -i"], msg: "das `edit`-Tool — nicht `sed -i`." },
  { phrases: ["tail -f", "journalctl -f"], msg: "kein `-f`-Folgen (Polling) — einmalig lesen." },
  { phrases: ["gh run list"], msg: "`ci_manage list` — nicht `gh run list`." },
  { phrases: ["gh run view"], msg: "`ci_manage view <id>` / `ci_manage log <id>` — nicht `gh run view`." },
  { phrases: ["gh run delete"], msg: "ein cancelled Lauf behält seinen Log — nicht `gh run delete`." },
  { phrases: ["gh run watch"], msg: "`ci_manage view <id>` einmalig — nicht `gh run watch` (Polling)." },
  {
    phrases: ["git reset", "git checkout", "git clean", "git rebase", "git stash", "git restore", "git switch"],
    msg: "kein destruktives git (`reset`/`checkout --`/`clean`/`rebase`/`stash`/`restore`/`switch`/`worktree`); nur add/commit/status/rm.",
  },
  { phrases: ["git push --force", "git push -f"], msg: "kein force-push." },
]

const CARGO_ALLOWED = ["cargo check", "cargo fmt --", "cargo build -p ", "cargo run -p "]

// Zerteilt an Separatoren, aber quote-bewusst: ein quoted Muster (`sgrep 'a|b'`)
// bleibt unangetastet — nur echte Pipes/Separatoren trennen Segmente.
function segments(command: string): string[] {
  const out: string[] = []
  let cur = ""
  let quote: string | null = null
  for (let i = 0; i < command.length; i++) {
    const ch = command[i]
    if (quote) {
      cur += ch
      if (ch === quote) quote = null
      continue
    }
    if (ch === "'" || ch === '"') {
      quote = ch
      cur += ch
      continue
    }
    if (ch === "\n" || ch === ";") {
      out.push(cur)
      cur = ""
      continue
    }
    if (ch === "|") {
      out.push(cur)
      cur = ""
      if (command[i + 1] === "|") i++
      continue
    }
    if (ch === "&" && command[i + 1] === "&") {
      out.push(cur)
      cur = ""
      i++
      continue
    }
    cur += ch
  }
  out.push(cur)
  return out.map((s) => s.trim()).filter((s) => s.length > 0)
}

function bareToken(seg: string): string {
  const m = seg.match(/^\s*(?:\S*\/)?([^\s/]+)([\s\S]*)$/)
  return m ? `${m[1]}${m[2]}`.replace(/\s+/g, " ").trim() : seg.trim()
}

function teaching(seg: string): string | null {
  const b = bareToken(seg)
  for (const t of TOKEN_TEACH) {
    for (const tok of t.tokens) if (b === tok || b.startsWith(tok + " ")) return t.msg
  }
  for (const p of PHRASE_TEACH) {
    for (const ph of p.phrases) if (b === ph || b.startsWith(ph + " ")) return p.msg
  }
  if (b === "cargo" || b.startsWith("cargo ")) {
    if (!CARGO_ALLOWED.some((a) => b === a.trim() || b.startsWith(a))) {
      return "`cargo` nur exakt: `cargo check` · `cargo fmt -- <pfad>` · `cargo build -p <crate> --bin <name>` · `cargo run -p <crate> --bin <name>` (kein Zusatzflag)."
    }
  }
  return null
}

// Nur-Anzeige-Flags der content-Suche; ändern den Suchraum nicht.
const AS_DISPLAY = ["--lines", "--files", "--max-mb", "--max", "--skip", "--binary", "--count", "--case", "--include"]

// `archive_search <kw>` ohne `--root` und ohne Modus-Flag liest die Platte ab $HOME.
// Erlaubt ist: `--root <dir>` (lokal, begrenzt) ODER irgendein Modus-Flag
// (Netz-Arm `--all`/`--brave`/…, Pfade `--index`, Historie `--git`, `--mft`, `--stats`).
function archive_search_bare(seg: string): string | null {
  const b = bareToken(seg)
  if (!(b === "archive_search" || b.startsWith("archive_search "))) return null
  const toks = b.split(/\s+/).slice(1)
  if (toks.some((t) => t === "--root" || t.startsWith("--root="))) return null
  const mode = toks.some((t) => {
    const name = t.split("=")[0]
    return name.startsWith("--") && !AS_DISPLAY.includes(name)
  })
  if (mode) return null
  return "`archive_search <kw>` ohne `--root` durchsucht ganz $HOME (Datei-Inhalte) → `--root <dir>` (lokal, begrenzt) oder ein Netz-Modus (`--all`/`--brave`/…) bzw. `--index`/`--git`/`--mft`."
}

export default async () => {
  return {
    "tool.execute.before": async (input: { tool: string }, output: { args: Record<string, unknown> }) => {
      // Fenster-Gate: `read` einer ganzen Datei über der Schwelle wird abgelehnt.
      if (input.tool === "read") {
        const fp = output.args?.filePath
        if (typeof fp === "string" && output.args?.offset == null && output.args?.limit == null) {
          const n = lineCount(fp)
          if (n != null && n > WHOLE_READ_MAX_LINES) {
            throw new Error(
              `ganze Datei (${n} Zeilen) → fenstern: \`read\` mit offset/limit (z. B. offset 1, limit ${WHOLE_READ_MAX_LINES}); eine große Datei am Stück wird jede Runde erneut tokenisiert (Kontext-Multiplikator). Der volle Inhalt bleibt über die Spill-Datei (\`full: <pfad>\`) erreichbar.`,
            )
          }
        }
        return
      }
      if (input.tool !== "bash") return
      const cmd = output.args?.command
      if (typeof cmd !== "string") return
      for (const seg of segments(cmd)) {
        const msg = teaching(seg)
        if (msg) throw new Error(msg)
        const bareAs = archive_search_bare(seg)
        if (bareAs) throw new Error(bareAs)
        // `sread` ohne Fenster auf einer großen Datei → fenstern.
        const b = bareToken(seg)
        if ((b === "sread" || b.startsWith("sread ")) && !/--offset\b/.test(b) && !/--limit\b/.test(b)) {
          const file = b.match(/^sread\s+(\S+)/)?.[1]
          if (file) {
            const n = lineCount(file)
            if (n != null && n > WHOLE_READ_MAX_LINES) {
              throw new Error(
                `\`sread\` ohne Fenster auf ${n} Zeilen → \`sread <datei> --offset N --limit M\` (Fenster ≤ ${WHOLE_READ_MAX_LINES}); große Dateien nie am Stück.`,
              )
            }
          }
        }
      }
      // `\|` ist in sgrep KEINE Alternation — sgrep matcht literal. Ein solches
      // Muster findet nur das Zeichen selbst und erzeugt Falsch-Negative.
      if (/(^|\s|\/)sgrep\b[^\n]*\\\|/.test(cmd)) {
        throw new Error(
          "`sgrep` matcht literal — Alternation (`\\|`) existiert nicht → ein Muster je Aufruf (`sgrep -i 'a' <pfad> && sgrep -i 'b' <pfad>`) oder `archive_search <kw> --root <dir>` je Begriff.",
        )
      }
    },
  }
}
