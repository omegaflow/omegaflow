import { execSync } from "node:child_process"
import { readFileSync } from "node:fs"
import { join } from "node:path"
import type { TuiPlugin, TuiPluginApi, TuiPluginModule } from "@opencode-ai/plugin/tui"

type Line = { line: string; title: string }

const LINES: Line[] = [
  { line: "future", title: "Future-Linie" },
  { line: "mycelium", title: "Mycelium-Linie" },
  { line: "sensory", title: "Sensory-Linie" },
  { line: "mountain", title: "Mountain-Linie" },
  { line: "river", title: "River-Linie" },
]

function expandShell(text: string, cwd: string): string {
  return text.replace(/!`([^`]*)`/g, (_, cmd: string) => {
    return execSync(cmd, { cwd, shell: "/bin/bash" }).toString().trim()
  })
}

function commandBody(worktree: string, name: string, arg: string): string {
  const file = join(worktree, ".opencode", "command", `${name}.md`)
  const raw = readFileSync(file, "utf8")
  const body = raw.replace(/^---\n[\s\S]*?\n---\n/, "")
  return expandShell(body, worktree).replaceAll("$ARGUMENTS", arg)
}

function activeSessionID(api: TuiPluginApi): string | undefined {
  const current = api.route.current
  if (current.name === "session" && current.params && typeof current.params.sessionID === "string") {
    return current.params.sessionID
  }
  return undefined
}

const tui: TuiPlugin = async (api) => {
  const worktree = api.state.path.worktree
  for (const { line, title } of LINES) {
    api.keymap.registerLayer({
      mode: "base",
      commands: [
        {
          name: `omegaflow.${line}_new`,
          title: `${title} — neue Session (Plan)`,
          category: "omegaflow",
          namespace: "palette",
          slashName: `${line}_new`,
          async run() {
            try {
              const text = commandBody(worktree, line, "")
              const created = await api.client.session.create({ title, agent: "plan" })
              const id = created.data?.id
              if (!id) {
                api.ui.toast({ variant: "error", title: `${line}_new`, message: "session.create lieferte keine id" })
                return
              }
              api.route.navigate("session", { sessionID: id })
              await api.client.session.prompt({
                sessionID: id,
                agent: "plan",
                parts: [{ type: "text", text }],
              })
            } catch (e) {
              api.ui.toast({ variant: "error", title: `${line}_new`, message: String(e) })
            }
          },
        },
        {
          name: `omegaflow.${line}_go`,
          title: `${title} — Consent & Ausführung (line)`,
          category: "omegaflow",
          namespace: "palette",
          slashName: `${line}_go`,
          async run() {
            try {
              const sessionID = activeSessionID(api)
              if (!sessionID) {
                api.ui.toast({ variant: "error", title: `${line}_go`, message: "keine aktive Session" })
                return
              }
              const text = commandBody(worktree, "consent", "")
              await api.client.session.prompt({
                sessionID,
                agent: "line",
                parts: [{ type: "text", text }],
              })
            } catch (e) {
              api.ui.toast({ variant: "error", title: `${line}_go`, message: String(e) })
            }
          },
        },
      ],
    })
  }
}

const plugin: TuiPluginModule & { id: string } = { id: "omegaflow.line-new", tui }

export default plugin
