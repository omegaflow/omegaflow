# Rat — native-flaeche-rundfile.md

*Eine Datei je Runde. Die Teile stehen unverändert unter ihrer Herkunft; nichts zusammengefasst.*

---

## 2026-10-17-native-flaeche-formular.md

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


---

## 2026-10-17-rat-native-flaeche.md

# Rat — R-2026-10-17-k: Native Fläche statt Bastellösungen?

**Frage:** `state/stimmen/2026-10-17-native-flaeche-formular.md`
**Gefahren:** 2026-10-17 — GLM-5.3 · Mistral Large 4 · Fugu Max High. **Lumo 2.0 Max: nicht gefahren** (die Seite blieb in „Verbindung wird hergestellt…"; Enter fügt dann nur einen Zeilenumbruch ein). **Benannt, nicht still.**
**Kosten:** GLM (`#low`) + Mistral (`#none`) + Fugu (frei).

## Die Antworten

**GLM-5.3** (`deepinfra/zai-org/GLM-5.3#low`, `ses_eb43b0967ffeyLfQYb53AFu42J`):
> Kriterium: ersetzen, wo das native Feature (a) gemessen existiert, (b) nachweislich **mindestens gleichwertig** ist, (c) keine Domänensemantik trägt. `session_burn` → `session/stats`; Palette → `commands`/`agents`; `form-guard` → `permissions` (die *Lehre* bleibt Haus); `sandbox` → `permissions`. `inbox`+`form` = Substrat für den Transport, die Post-Semantik bleibt. `export/import` **kein** Ersatz für die Übergabe — es liefert Kontextmasse und zerstört Knappheit.
> **VERDIKT: C** — Substrat nativ, Domäne Haus.
> **STÄRKSTER RISS:** Die OFFEN-Fälle (`inbox`, `export`) und die Behauptung „prinzipiell nicht ersetzbar" sind **ungeprüft** — `null-echt` fehlt; entschieden ist nur behauptet.

**Mistral Large 4** (`mistral/mistral-large-4#none`, `ses_eb43b0960ffefUupumT7kEoyUn`):
> Ersetzen, wo nativ **nachweislich mehr** kann — nicht, weil es existiert. Die Grenze ist **keine technische Unmöglichkeit**, sondern eine **bewusste Nicht-Übersetzung**: Register/Token/`terms`-Wahrheit sind unser Differenzierungsmerkmal, kein Infrastrukturproblem. `inbox` liefert Ereignisse, keine garantierte Konsumreihenfolge unseres Workflows; `export/import` tauscht Knappheit gegen Vollständigkeit. Es bleiben **zwei** eigene Schichten: Domänensemantik **und** Protokollschicht (Adressierung, Reihenfolge, Übergabe-Knappheit).
> **VERDIKT: C** mit Präzisierung — Domäne **und** Protokoll bleiben eigen.
> **STÄRKSTER RISS:** Die Unterscheidung **„technisch unmöglich" vs. „bewusst nicht übersetzt"** fehlt in der gesamten Selbstbeschreibung — Frage 2 war falsch formuliert.

**Fugu Max High** (`chat.sakana.ai`, UI, 2026-10-17):
> Die bloße Existenz eines Gegenstücks genügt nicht. `session_burn` fällt ersatzlos an `session/stats` (strikt mehr: cost, tokens, Cache, Reasoning, Tools, Variante). `form-guard` schrumpft auf die Lücke (`policies` fehlt). `export/import` ist das **Gegenteil** der Übergabe: Import ist vollständig und treu, die Übergabe lebt von **bewusstem Verlust**. `policies` und `skill` sind die dringendsten ungenutzten Features.
> **VERDIKT: C** — native Fläche als Substrat; Domänenschicht bleibt.
> **STÄRKSTER RISS:** Fällt die Behauptung „Haus-Semantik ist prinzipiell nicht ersetzbar" bei `policies`/`skill`, kippt die **ganze Trennlinie**.

**Lumo 2.0 Max** (`lumo.proton.me/guest`, UI, 2026-10-17 — vom Operator abgeschickt):
> Wo ein natives Feature existiert und mindestens gleichwertig ist, sofortiger Ersatz: `session/stats` ersetzt `session_burn` vollständig (reicher an Metriken); `permissions` übernehmen die `sandbox`-Logik; CLI/Agents übernehmen Palette und Agent-Prompts. `form-guard` nur **partiell** gedeckt — die „Policies"-Schicht bleibt offen. `inbox`+`form` **kann** den Post-Mechanismus ersetzen, aber nur wenn die Semantik „kurz, offen, adressiert" erhalten bleibt. `export/import` zerstört die Übergabe-Leistung wahrscheinlich — bleibt hausintern. Register/A=A-Token/Transfer-Token: „prinzipiell nicht nativ ersetzbar, da sie domainspezifische Wahrheit tragen". Grenzziehung: **Kostenerfassung komplett nativ, Kommunikationsfluss hybrid**.
> **VERDIKT: C** — native Fläche als Substrat, nur die Domänenschicht bleibt eigenständig.
> **STÄRKSTER RISS:** `inbox`/`form` gegen den Post-Mechanismus — Semantikverlust möglich.

## Bilanz

**4 von 4 Sitzen: Option C** (GLM · Mistral · Fugu · Lumo). Kein Widerspruch im Verdikt.

## Der Riss (nicht geglättet)

Alle drei sagen C, aber **zwei benennen dieselbe Schwäche schärfer als das Verdikt**:
- **Die Trennlinie ist ungeprüft.** „Haus-Semantik ist nicht nativ ersetzbar" hat niemand gemessen — sie ist eine *Setzung*. Mistral und Fugu sagen: es ist keine technische Unmöglichkeit, sondern eine **bewusste Nicht-Übersetzung**. Der Unterschied ist nicht Kosmetik: eine Setzung kann man überstimmen, eine Messung nicht.
- **Die Protokollschicht fehlt im Verdikt.** Mistral: es bleiben **zwei** Schichten (Domäne + Protokoll). C sagt nur „Domäne" — die Adressierung/Reihenfolge/Knappheit ist nicht mitgemeint.
- **`export/import` ist kein Ersatz, sondern das Gegenteil.** Einhellig. Die Übergabe lebt vom bewussten Verlust; ein Export ist vollständig und treu.

## Stand

**entschieden** (2026-10-17) — 4/4 Sitze, kein Widerspruch.

## An future

Origin: rat-2026-10-17-k
- **Sofort ersetzen (nachweislich überlegen):** `bin/session_burn` → `opencode api get /api/experimental/session/stats` (cost, tokens inkl. Cache, Reasoning, Tools, Variante); Palette/Rollen → `commands`/`agents`; `sandbox`/`form-guard` → `permissions`.
- **Nicht ersetzen:** Übergabe-Dateien (Knappheit ist die Leistung), Register/Token/`terms`-Wahrheit (Domäne), `smail`/CDN (dritt-wirksame Akte).
- **Substrat prüfen, nicht übernehmen:** `session/inbox` + `form` (Transport ok, Adressierung bleibt Haus), `export/import` (nur Archiv/Fallback).
- **Die Trennlinie ist eine Setzung, keine Messung** — vor dem nächsten Umbau `policies` und `skill` real prüfen; fällt die Setzung dort, kippt C.

