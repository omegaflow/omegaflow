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

const TOKEN_TEACH: { tokens: string[]; msg: string }[] = [
  { tokens: ["ls"], msg: "`ls` ist strukturell verboten → nutze `glob <pattern>` (Discovery) oder `archive_search --index`." },
  { tokens: ["grep"], msg: "bash-`grep` ist verboten → nutze `sgrep [-i] <muster> [dir]` oder `archive_search <kw> --root <dir>`." },
  { tokens: ["rg"], msg: "`rg` ist verboten → nutze `sgrep`." },
  { tokens: ["cat"], msg: "bash-`cat` ist verboten → nutze `sread <datei> [--offset N --limit M]` oder das `read`-Tool." },
  { tokens: ["head", "tail"], msg: "`head`/`tail` kastrieren das Ergebnis (`tail -f` ist zusätzlich Polling) → nutze `sread <datei> --offset N --limit M` oder das `read`-Tool (offset/limit); einen gekappten Tool-Output vollständig aus der Spill-Datei (`full: <pfad>`) lesen, nie anschneiden." },
  { tokens: ["copilot", "copilot_ask", "copilot_review"], msg: "`copilot` ist ein Cloud-Dienst und erhält keine omegaflow-Daten (Operator-Wort 2026-09-28) → lokal und deterministisch: `bin/session_check --in|--out`." },
  { tokens: ["python", "python3"], msg: "Python ist verboten → nutze Rust (`cargo`) oder die kanonischen Tools." },
  { tokens: ["cd"], msg: "`cd` ist verboten → nutze den `workdir`-Parameter des bash-Tools." },
  { tokens: ["rustc"], msg: "`rustc` ist verboten → baue über `cargo`." },
  { tokens: ["watch", "sleep", "inotifywait", "entr"], msg: "Polling ist verboten → keine `watch`/`sleep`-Schleifen in einer Session." },
]

const PHRASE_TEACH: { phrases: string[]; msg: string }[] = [
  { phrases: ["sed -i"], msg: "`sed -i` ist verboten → nutze das `edit`-Tool." },
  { phrases: ["tail -f", "journalctl -f"], msg: "Polling ist verboten → kein `-f`-Folgen." },
  { phrases: ["gh run list"], msg: "`gh run list` ist verboten → nutze `ci_manage list`." },
  { phrases: ["gh run view"], msg: "`gh run view` ist verboten → nutze `ci_manage view <id>` / `ci_manage log <id>`." },
  { phrases: ["gh run delete"], msg: "`gh run delete` ist verboten → ein cancelled Lauf behält seinen Log." },
  { phrases: ["gh run watch"], msg: "`gh run watch` ist verboten → kein Polling; `ci_manage view <id>` einmalig." },
  {
    phrases: ["git reset", "git checkout", "git clean", "git rebase", "git stash", "git restore", "git switch"],
    msg: "destruktives git ist verboten → kein `reset`/`checkout --`/`clean`/`rebase`/`stash`/`restore`/`switch`/`worktree`.",
  },
  { phrases: ["git push --force", "git push -f"], msg: "force-push ist verboten." },
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

export default async () => {
  return {
    "tool.execute.before": async (input: { tool: string }, output: { args: { command?: unknown } }) => {
      if (input.tool !== "bash") return
      const cmd = output.args?.command
      if (typeof cmd !== "string") return
      for (const seg of segments(cmd)) {
        const msg = teaching(seg)
        if (msg) throw new Error(msg)
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
