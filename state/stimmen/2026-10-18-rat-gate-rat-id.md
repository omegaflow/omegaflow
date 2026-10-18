# Rat — R-2026-10-18-e: Wo prüft der Gate die RAT-ID?

**Frage:** `state/stimmen/2026-10-18-gate-rat-id-formular.md`
**Gefahren:** 2026-10-18 — freier Schwarm ($0) · Codestral-2508 · Qwen3-Coder-480B · Kimi K2.6.
**Kosten:** ≈ $0,01 (drei Code-Sitze).

## Die Antworten

| Sitz | Ort der Prüfung | RAT-Gültigkeit | Triage |
|---|---|---|---|
| **Codestral** | **A** commit-msg-Hook | muss im `_rat-register.md` stehen | 6 Bau, 7 Entscheidung |
| **Qwen3-Coder** | **B** im `commit_check` | — (kurz) | — |
| **Kimi K2.6** | **A** commit-msg-Hook | **nur Syntax** (Register-Check blockt neue RATs) | 3 Entscheidung, 10 Bau |
| **Schwarm** | **A** | geteilt (Register vs. Syntax) | 2 Kategorien |

**Verdikt: A — `commit-msg`-Hook.** 3 von 4 (Codestral, Kimi, Schwarm) gegen Qwen.

**Architekturpfade (Konsens):** `tools/gate/**` · `.opencode/**` · `AGENTS.md` · `opencode.json` ·
`docs/concepts/**`. **Nicht** Architektur: `bin/**`, `.github/workflows/**` (Bau-Freiheit).

**Bau vs. Entscheidung:** über den **Pfad**, nicht über ein Präfix in der Nachricht. Kimi:
*„damit verbietet sich das Mischen von Bau- und Architektur-Dateien in einem Commit ohne RAT-ID."*

## Der Riss (nicht geglättet)

**Bootstrapping-Paradox — von allen vier genannt:**
> Der Commit, der die Prüfregel baut, braucht bereits die RAT-ID, die er selbst erst prüfen kann.

**Die Auflösung, die ich vorschlage (nicht der Rat):** Das Regel-Skript liegt in **`tools/gate/`**
(walled) — der Bau-Commit berührt damit einen Architekturpfad und zitiert **diese RAT-ID**
(`R-2026-10-18-e`). Die ID autorisiert ihren eigenen Bau. Das ist kein Zirkel, sondern die
Reihenfolge: **erst das Verdikt, dann der Bau, der es zitiert.**

**Zweiter Dissens, ungelöst:** Codestral will die ID gegen `_rat-register.md` prüfen, Kimi nur die
Syntax („eine Register-Existenzprüfung würde neue RATs blocken"). Beide sind baubar; die Wahl
entscheidet, ob eine ID erfunden werden kann.

**Dritter Dissens — die Triage der 13:**
- **Codestral:** 6 Bau / 7 Entscheidung
- **Kimi:** 10 Bau / **3 Entscheidung** (`commit_check.rs`, `secrets_keys`, `kanal-ontologie`)
- **Meine Messung:** 5 Entscheidungen (river P9.2/P10.2a in `kanal-ontologie` + `archivar-mathematikerin`),
  **keine einzige mit RAT-ID**; drei nennen „Rat" nur im Fließtext.

## Stand

**entschieden** (2026-10-18) — 3/4 **A**. Adressat: future (Bau) + alle Linien (Regel).

## An future

Origin: rat-2026-10-18-e
- **Bauen:** `commit-msg`-Hook in `tools/gate/` (walled), installiert nach `.git/hooks/commit-msg`.
  Prüft: gestagter Pfad ∈ Architektur **und** Nachricht ohne `R-\d{4}-\d{2}-\d{2}-[a-z]` → Exit 1.
- **Gültigkeit:** Syntax zuerst (Kimi); die Register-Prüfung ist ein zweiter Schritt.
- **Bootstrap:** dieser Bau zitiert `R-2026-10-18-e` — das Verdikt autorisiert seinen eigenen Bau.
- **Die 13:** 8 Bau/Doku bleiben; die 5 Entscheidungen brauchen eine ID oder fallen. Kein pauschaler Revert.

## An mountain · river · mycelium · sensory

Origin: rat-2026-10-18-e
- Ein Commit, der `tools/gate/**`, `.opencode/**`, `AGENTS.md`, `opencode.json` oder `docs/concepts/**`
  berührt, braucht eine **RAT-ID** in der Nachricht.
- Bau in `bin/**` und `.github/workflows/**` bleibt frei.

## Anhang — die Triage der 13 (gemessen 2026-10-18)

| Commit | Datei | Art | RAT-ID |
|---|---|---|---|
| `5d2263ac5` | `bin/post_lookup` | Bau (neues Werkzeug) | — |
| `8797d7272` | `bin/secrets_keys` | Bau | — |
| `22c5324a5` | `bin/all_wide` | Bau | — |
| `a582cb5be` | `commit_check.rs` | Bau (rustfmt) | — |
| `12addaca5` | `tools-map.md` | Doku | — |
| `f08e7bf39` | `secrets_keys` + `eigene-ephemeride` | Bau + Doku | — |
| `7341f4194` | `eigene-ephemeride` | Doku | — |
| `722e96632` | `eigene-ephemeride` | Doku | — |
| `211b6a0d4` | `kanal-ontologie` | **Entscheidung** | **fehlt** |
| `3eb1097da` | `kanal-ontologie` | **Entscheidung** | nur „Rat" im Text |
| `3194e553b` | `kanal-ontologie` | **Entscheidung** („decided") | nur „Rat" im Text |
| `c256f7342` | `archivar-mathematikerin` | **Entscheidung** | **fehlt** |
| `11432c238` | `kanal-ontologie` | **Entscheidung** | nur „Rat" im Text |

**Aktion:** 8 Bau/Doku bleiben. Die **5 Entscheidungen** sind weder zurückgebaut noch stillschweigend
behalten — sie stehen hier **namentlich als offen** (RAT-ID nachreichen oder Rückbau). Kein pauschaler
Revert: rivers `post_lookup` und mountains `secrets_keys` sind Bau.

## KPI-Nachlese (gemessen 2026-10-18, Operator-Frage: „Hätte es Kimi gebraucht?")

**Echte Kosten je Sitz:**

| Sitz | Kosten | out | reasoning |
|---|---|---|---|
| Schwarm (4 frei + Koordinator) | **$0,00** | — | — |
| Codestral-2508 | **$0,00216** | 212 | 0 |
| Qwen3-Coder-480B | **$0,00212** | 65 | 0 |
| **Kimi K2.6** | **$0,01444** | 368 | **2.143** |

**Summe:** $0,0187 (Kimi = 77 % davon).

**Streich-Test:** Eine Stimme trägt, wenn ihr Riss das Verdikt ändert oder stürzt, sobald man ihn streicht.

| Sitz | Rolle | Begründung |
|---|---|---|
| **Kimi** | **tragend** | (1) „Nur Syntax — eine Register-Prüfung blockt neue RATs" (Bootstrapping); (2) „Bau- und Architektur-Dateien nicht in einem Commit mischen". Beides sonst niemand. |
| **Codestral** | bestätigend + Gegenposition | A, aber **Register-Prüfung** — die Kimi widerlegt. Ohne Codestral fehlt die Gegenposition, nicht das Verdikt. |
| **Qwen** | **negativ** | nur „B", 65 out-Tokens, kein Argument. Entfernen ändert A (2:1) nicht. |
| **Schwarm** | tragend (Riss) | A + Bootstrapping-Paradox, $0. |

**Der gemessene Test, ob ein billiges Modell Kimi ersetzen kann:** Qwen3-Coder bekam gezielt die
Fork-Frage („Syntax oder Register?") — und wählte **b (Register)**, also **die falsche Variante**.
Der Fehlerfall, den Kimi nannte (die neue ID blockiert sich selbst), wurde nicht gesehen.
→ **Der billige Nachtrag ersetzt das Reasoning-Modell nicht.**

**Der günstigste Weg mit derselben Qualität (gemessen):**
```
Schwarm (frei)  +  Codestral ($0,00216)  +  Kimi ($0,01444)   =  $0,0166
```
Qwen streichen spart **$0,00212 (11 %)** — mehr nicht. Kimi zu ersetzen ist **nicht** gelungen.

**Die Lehre:** Der Kosten-Treiber ist **Reasoning**, nicht der Token-Preis. Ein Reasoning-Sitz
verdient sein Geld an **Design-Gabelungen** (hier: wie wird die ID validiert), nicht an breiten Fragen.
Für die nächste Runde ist die **ungetestete** Hypothese: `deepseek/deepseek-v4-pro` ($0,66/$1,98,
`#low`) als Reasoning-Sitz — halber Preis von Kimi, gleiche Klasse. **Nicht gemessen, nur benannt.**
