<!--
  title: Handover — v2 · Sandbox · Filter-Riss (2026-10-16)
  class: handover
  date: 2026-10-16
  status: live
-->
# Stand — v2, Sandbox, Filter-Riss

Aus der privaten Future-Sitzung, für die **öffentliche** Linie (v2, gekäfigt).
Kein privater Inhalt hier — nur das Gerüst.

## Läuft
- **opencode v2.0.31** installiert — CLI **und** Desktop (`/opt/OpenCode`).
- Die **v2-GUI läuft.** Der frühere Hänger war die **Einzel-Instanz-Kollision**: v2 nutzt denselben
  Zustandsordner wie ein laufendes v1 → es gibt den Start ab und beendet sich stumm (Exit 0).
  **v1 schließen → v2 kommt hoch.**
- **Kein v1-Backup** vorhanden (`/opt/OpenCode.v1-backup` fehlt, der `cp` scheiterte). Rückweg = Neuinstallation.

## Die Sandbox (schon gebaut)
`bin/agent_sandbox` = `bwrap`-Käfig. Sichtbar: `omegaflow`, `omegaflow-work`, die Programmbasis (ro),
ein `GH_TOKEN`. **Nicht** drin: das Home, `~/.ssh`, `~/.gpg`; `/run` nur leer; `/tmp/opencode` raus;
eigenes PID-getrenntes `/proc`. — Sie ist **schon `shell` in der öffentlichen `opencode.json`**,
also im öffentlichen Repo automatisch aktiv. Ein Hebel statt Neubau.

Nur die **Desktop-GUI** läuft nicht durch sie — dafür liegt `~/opencode-gui-sandbox.sh`
(bwrap um die App: Home maskiert, nur die zwei Repos sichtbar).

## Der Filter-Riss (offen)
`llm-gateway` läuft mit `OMEGAFLOW_FILTER=0`. Bei `=1` ruft `redact()` **pro Nachricht**
`bin/private_read` als **Subprozess** (Temp-Datei + awk/sed über `redact.map`) → ein 80-KB-Body
geht von **18,6 s auf 62 s**. Der **In-Prozess-Lookup** existiert schon als Fallback.
**Fix-Idee:** den Subprozess-Zweig in `redact()` streichen.

## Abweichungen von Stock-opencode
- global `~/.config/opencode/opencode.jsonc`: `snapshot:false` · 10 Provider aus · `mcp.playwright` ·
  Plugin `@vymalo/opencode-browser` · `tool_output` 300/8 KB · `compaction` · `lsp` · `watcher.ignore`.
  **Nicht Default.**
- Projekte: `shell`-Override · Provider `gw` (`127.0.0.1:1621`) · eigene Agenten · Permission-Matrix ·
  Plugins `form-guard`/`act_recorder` (v1 — in v2 **nicht** portiert).

Was nicht messbar ist, ist `pending` — nie raten.
