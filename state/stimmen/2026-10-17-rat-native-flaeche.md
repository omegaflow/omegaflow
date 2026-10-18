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
