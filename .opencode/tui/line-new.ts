import { execSync } from "node:child_process"
import { readFileSync } from "node:fs"
import { join } from "node:path"
import type { TuiPlugin, TuiPluginModule } from "@opencode-ai/plugin/tui"

type Line = { line: string; title: string }

const LINES: Line[] = [
  { line: "entscheid", title: "Entscheid-Linie" },
  { line: "ernte", title: "Ernte-Linie" },
  { line: "forschung", title: "Forschung-Linie" },
  { line: "bau", title: "Bau-Linie" },
]

function expandShell(text: string, cwd: string): string {
  return text.replace(/!`([^`]*)`/g, (_, cmd: string) => {
    return execSync(cmd, { cwd, shell: "/bin/bash" }).toString().trim()
  })
}

function linePrompt(worktree: string, line: string, arg: string): string {
  const file = join(worktree, ".opencode", "command", `${line}.md`)
  const raw = readFileSync(file, "utf8")
  const body = raw.replace(/^---\n[\s\S]*?\n---\n/, "")
  return expandShell(body, worktree).replaceAll("$ARGUMENTS", arg)
}

const tui: TuiPlugin = async (api) => {
  for (const { line, title } of LINES) {
    api.keymap.registerLayer({
      mode: "base",
      commands: [
        {
          name: `omegaflow.${line}_new`,
          title: `${title} — neue Session`,
          category: "omegaflow",
          namespace: "palette",
          slashName: `${line}_new`,
          async run() {
            try {
              const worktree = api.state.path.worktree
              const text = linePrompt(worktree, line, "")
              const created = await api.client.session.create({ title, agent: "line" })
              const id = created.data?.id
              if (!id) {
                api.ui.toast({ variant: "error", title: `${line}_new`, message: "session.create lieferte keine id" })
                return
              }
              api.route.navigate("session", { sessionID: id })
              await api.client.session.prompt({
                sessionID: id,
                agent: "line",
                parts: [{ type: "text", text }],
              })
            } catch (e) {
              api.ui.toast({ variant: "error", title: `${line}_new`, message: String(e) })
            }
          },
        },
      ],
    })
  }
}

const plugin: TuiPluginModule & { id: string } = { id: "omegaflow.line-new", tui }

export default plugin
