import { appendFileSync, mkdirSync, writeFileSync } from "node:fs"
import { join } from "node:path"

// act_recorder — Aufzeichnung als Nebenwirkung (Rat + Operator-Wort 2026-09-28).
// Jeder bedeutende Vorgang wird beim Geschehen geschrieben: Operator-Worte ueber
// `chat.message`, Tool-Akte ueber `tool.execute.before`. Kein LLM formuliert sie,
// kein Cloud-Dienst liest sie — nur lokale Datei-I/O. Nie werfen: ein Fehler des
// Recorders darf keinen Tool-Call und keine Nachricht blocken.
//
// Journal: state/zustand/ereignisse.φ (append-only, privat/getrackt im state-Repo).
// Eine Zeile je Vorgang: ISO-UTC | sessionID | quelle | klasse | gegenstand.
// Gegenstand statt Inhalt: host/pfad/agent — Passwoerter und Feldwerte nie.
//
// Aktive Session: state/reports/active_session.φ (eine Zeile, gitignored). Die
// sessionID aus der Plugin-API (input.sessionID, gemessen) wird bei jedem Akt
// geschrieben — der pre-commit-Hook liest sie als OMEGAFLOW_SESSION, wenn die Env
// leer ist (Konvention notes_notify.rs). Kein Default, kein Fake-Token.

const PROBE = "/tmp/opencode/act-probe.log"
const TEXT_CAP = 600

function stamp(): string {
  return new Date().toISOString().replace(/\.\d{3}Z$/, "Z")
}

// Significant tools whose Gegenstand we name. A redacted, fixed field per tool:
// bash -> the leading command (never the full command line: arguments can carry tokens),
// edit/write/read -> the path, browser_* -> the tab group, task -> the subagent.
function gegenstand(tool: string, args: unknown): string {
  if (!args || typeof args !== "object") return ""
  const a = args as Record<string, unknown>
  if (tool === "bash") {
    const cmd = typeof a.command === "string" ? a.command.trim() : ""
    const first = cmd.split(/\s+/)[0] ?? ""
    return first.replace(/^.*\//, "")
  }
  if (tool === "edit" || tool === "write" || tool === "read") {
    return typeof a.filePath === "string" ? a.filePath : ""
  }
  if (tool.startsWith("browser_")) {
    return typeof a.group === "string" ? a.group : ""
  }
  if (tool === "task") {
    return typeof a.subagent_type === "string" ? a.subagent_type : ""
  }
  if (tool === "smail" || tool === "ci_manage") {
    return tool
  }
  return ""
}

function text(parts: unknown): string {
  if (!Array.isArray(parts)) return ""
  return parts
    .filter((p): p is { type: string; text: string } => !!p && typeof p === "object" && (p as { type?: unknown }).type === "text" && typeof (p as { text?: unknown }).text === "string")
    .map((p) => p.text)
    .join(" ")
    .replace(/\s+/g, " ")
    .slice(0, TEXT_CAP)
}

export default async ({ worktree }: { worktree: string }) => {
  const dir = join(worktree, "state", "zustand")
  const journal = join(dir, "ereignisse.φ")
  const reportsDir = join(worktree, "state", "reports")
  const activeSession = join(reportsDir, "active_session.φ")

  const append = (line: string) => {
    try {
      mkdirSync(dir, { recursive: true })
      appendFileSync(journal, line)
    } catch {
      // silent: the record never blocks the act
    }
  }

  const recordSession = (sid: string) => {
    if (!sid) return
    try {
      mkdirSync(reportsDir, { recursive: true })
      writeFileSync(activeSession, `${sid}\n`)
    } catch {
      // silent: the record never blocks the act
    }
  }

  return {
    "chat.message": async (
      input: { sessionID: string; agent?: string },
      output: { parts?: unknown },
    ) => {
      recordSession(input.sessionID)
      const w = text(output.parts)
      if (!w) return
      append(`${stamp()} | ${input.sessionID} | ${input.agent ?? "?"} | wort | ${w}\n`)
    },
    "tool.execute.before": async (input: { tool: string; sessionID: string }, output: { args: unknown }) => {
      recordSession(input.sessionID)
      try {
        appendFileSync(PROBE, `${input.tool}\n`)
      } catch {
        // probe is best-effort
      }
      append(`${stamp()} | ${input.sessionID} | tool:${input.tool} | ${gegenstand(input.tool, output.args)}\n`)
    },
  }
}
