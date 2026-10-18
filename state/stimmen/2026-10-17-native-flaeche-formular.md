# Rat-Frage R-2026-10-17-k — Was bauen wir weiter selbst, was ist schon da?

**Die Frage in einem Satz:** OpenCode V2 bietet nativ eine Fläche, die wir zu großen Teilen nachgebaut haben — welche Bastellösungen ersetzen wir durch native Features, welche bleiben Haus-Semantik, und nach welchem Kriterium entscheiden wir das?

## Was OpenCode V2 nativ anbietet (gemessen am laufenden Server 2.0.32, 2026-10-17)

**CLI (17 Subkommandos):** `upgrade · uninstall · acp · api · debug · auth · mcp · plugin · models · stats · mini · run · session · service · reload · pair · serve`

**API-Gruppen:** `agent · command · config · credential · debug · event · experimental · form · fs · info · integration · location · mcp · model · pair · permission · plugin · project · provider · pty · reference · rpc · shell · skill · vcs · websearch · worktree · session`

**Session-Endpunkte (je Session):** `agent · background · command · compact · context · diff · environment · fork · form · generate · inbox · interrupt · message · model · move · permission · prompt · revert · shell · synthetic · view` — dazu experimentell `import · export · log · instructions/entries · skill · terminal · terminal/read · wait · stats`.

**Kosten/Tokens nativ:** `experimental/session/stats` liefert `cost` (gesamt **und je Modell + Variante**), `tokens{input,output,reasoning,cache.read,cache.write}`, `tools{calls,succeeded,failed}`, `activeDays`, `streak`. `Session.Info` trägt `cost` und `tokens` pro Session.

## Was wir stattdessen selbst gebaut haben (Stand heute)

| Bastellösung | Was sie tut | Natives Gegenstück? |
|---|---|---|
| `bin/session_burn` | Kosten je Session/Agent/Modell | **ja** — `experimental/session/stats` (reicher: Cache, Reasoning, Tools, Variante) |
| `plugin/form-guard.ts` | lehrende Deny-Wand (V2: lud nie) | **teilweise** — `permissions`; offen: „Policies"? |
| `tui/line-new.ts` | Palette `_new`/`_go` | **ja** — `commands`/`agents` |
| `agent/line.md` + `command/*.md` | Rollen/Startprompts | **ja** — `agents` / `commands`; evtl. `skill` + `instructions` |
| `bin/sandbox` / `agent_sandbox` | Pfad-Käfig | **ja** — `permissions` (read/edit/shell deny) |
| `bin/task_dispatch` + `post/in`/`post/out` | Postfach zwischen Linien | **offen** — `session/{id}/inbox` + `form` (Frage/Antwort an eine Session)? |
| Handover-Dateien | Übergabe zwischen Sessions | **offen** — `experimental/session/export` + `import`? |
| `bin/register_lookup`, `xfer`, `phi/sources.φ` | Register, Token, Quellen (A=A) | **nein** — Haus-Semantik |
| `bin/smail`, `private_read`, CDN | dritt-wirksame Akte | **nein** — Haus-Semantik |

## Die Fragen

1. **Ersetzen, wo ein natives Gegenstück existiert** — oder nur dort, wo das native Feature *nachweislich* mehr kann (z. B. `stats` gegen `session_burn`, Palette gegen `commands`)?
2. **Was ist die Grenze?** Ist die Haus-Semantik (A=A, `0 honored`, Register, Transport-Token, `terms`-Wahrheit) prinzipiell nicht nativ ersetzbar — oder ist das eine Behauptung, die man prüfen müsste?
3. **Das Postfach:** ersetzt `session/{id}/inbox` + `form` unseren `post/in`/`post/out`-Mechanismus — oder ist das ein Missbrauch eines Session-Features als Briefkasten?
4. **Das Handover:** ist `session export/import` ein Ersatz für die Übergabe-Datei — oder zerstört es genau das, was die Übergabe leistet (knapp, offen, adressiert)?
5. **Was fehlt uns noch, das nativ da ist und wir nicht nutzen?** (Kandidaten: `skill`, `reference`, `policies`, `worktree`, `compact`/`context`, `permission/saved`, `vcs`, `pty`.)

## Die Optionen

- **A** — Ersetzen, wo nativ: `stats` statt `session_burn`, `commands`/`agents` statt Palette, `permissions` statt Plugins/Käfig; die Haus-Semantik (Register, Token, Mail, CDN) bleibt.
- **B** — Nur Kosten/Stats nativ (der kleinste Schritt), alles andere bleibt.
- **C** — Native Fläche als **Substrat** (Sessions, Commands, Agents, Skills, Policies, Export), nur die Domänenschicht (Register, Quellen, Post-Semantik) bleibt eigen.

Bitte: **Verdikt + der stärkste Riss** — insbesondere: wo verwechselt „nativ vorhanden" mit „für uns geeignet"? Keine Umsetzung — die ist Folge, nicht Frage.
