# Rat — id.md

*Eine Datei je Runde. Die Teile stehen unverändert unter ihrer Herkunft; nichts zusammengefasst.*

---

## 2026-10-18-gate-rat-id-formular.md

# Rat-Frage R-2026-10-18-e — Wo prüft der Gate die RAT-ID?

**Kontext:** Rat R-2026-10-18-d hat entschieden: **B** — ein Commit auf einen Architekturpfad
braucht eine **RAT-ID** in der Commit-Nachricht; Bau (Bugfix/rustfmt) bleibt frei. Dazu eine
`edit:deny`-Wand auf `tools/gate/**` (umgesetzt). **B selbst ist noch nicht gebaut.**

**Die Frage:** Wo und wie prüft der Gate die RAT-ID — und was gilt als gültige ID?

## Der Gate-Code (Auszug, `tools/gate/src/bin/commit_check.rs`, 661 Zeilen)

```rust
fn main() {
    let no_canon = std::env::args().skip(1).any(|a| a == "--no-canon");
    let out = Command::new("git")
        .args(["-c", "core.quotePath=false", "diff", "--cached",
               "--name-only", "--diff-filter=ACM"])
        .output().expect("git");
    let files = String::from_utf8_lossy(&out.stdout).to_string();
    let mut gate = Gate::new("commit", "");
    let mut fail = false;
    for path in files.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if !path.ends_with(".rs") { continue; }
        if is_test_file_path(path) { ...test_file_exempt += 1; }
        let content = match std::fs::read_to_string(path) { Ok(c) => c, Err(_) => continue };
        let mut obj = HashMap::new();
        obj.insert("filePath".to_string(), JsonVal::Str(path.to_string()));
        obj.insert("content".to_string(), JsonVal::Str(content));
        let args = json_write(&JsonVal::Obj(obj));
        if let Some(v) = gate.check_tool_call("write", &args) { eprintln!(...); fail = true; }
    }
    let staged: Vec<&str> = files.lines().map(str::trim).collect();
    if staged.contains(&"opencode.json") { /* mcp-no-leak */ }
    if staged.contains(&"AGENTS.md") { /* agents growth */ }
    if staged.contains(&"phi/sources.φ") { /* canon */ }
    /* … weitere Checks … */
    if fail { std::process::exit(1) }
}
```

**Beobachtung am Code:** Der Gate läuft als **pre-commit** und liest die **gestagten Pfade**.
Die Commit-Nachricht steht zu diesem Zeitpunkt in `.git/COMMIT_EDITMSG` (bei `git commit -m`).
Es gibt **keinen** Hook, der die Nachricht heute prüft.

## Die Fragen

1. **Wo gehört die Prüfung hin** — in denselben `commit_check` (liest `.git/COMMIT_EDITMSG`),
   in einen **`commit-msg`-Hook** (die Nachricht ist dort garantiert), oder in die **Datei selbst**
   (eine Kopfzeile `rat: R-…` im gestagten Inhalt)?
2. **Was ist eine gültige RAT-ID** — nur die Syntax (`R-\d{4}-\d{2}-\d{2}-[a-z]`), oder muss sie im
   `_rat-register.md` stehen (und in welchem Stand: `offen` genügt, oder `entschieden`)?
3. **Welche Pfade gelten als Architektur** — `tools/gate/**`, `.opencode/**`, `AGENTS.md`,
   `opencode.json`, `docs/concepts/**` — oder mehr (`bin/**`, `.github/workflows/**`)?
4. **Wie wird Bau von Entscheidung unterschieden** — über den Pfad (nicht möglich: derselbe Pfad),
   über die RAT-ID (Bau ohne, Entscheidung mit — dann ist *jede* Änderung an `tools/gate/` eine
   Entscheidung und `edit:deny` macht B dort gegenstandslos), oder über eine Kennzeichnung im Commit
   (`build:` vs. `decide:`)?

## Die Optionen

- **A — `commit-msg`-Hook.** Ein eigener Hook prüft `$1` (die Nachricht) gegen die Architekturpfade
  aus `git diff --cached --name-only`. Garantierte Nachricht, klarer Fehlertext, kein Umbau des Gates.
- **B — im `commit_check`.** Ein Block mehr in `main()`: `read_to_string(".git/COMMIT_EDITMSG")`,
  Regex, Pfad-Match. Ein Programm, ein Exit-Code — aber die Nachricht ist im pre-commit nicht
  garantiert (bei `git commit` ohne `-m` wird sie erst danach geschrieben).
- **C — in der Datei.** Jede Architekturdatei trägt eine Kopfzeile `rat: R-…`; der Gate prüft den
  gestagten Inhalt. Die Autorisierung reist mit der Datei, nicht mit dem Commit — überlebt Rebase und
  Cherry-Pick, aber die Zeile muss gepflegt werden.

**Der Vorbehalt:** `tools/gate/**` ist jetzt `edit:deny`. Wenn B verlangt, dass *jede* Änderung an
`tools/gate/` eine RAT-ID braucht, dann ist der Bau der Regel selbst ein Architektur-Akt — er braucht
eine RAT-ID. Welche?

**Zweite Frage (getrennt):** Die **13 Alt-Änderungen** (R-d). Lumo sagt revertieren, die anderen
triage. Welche der 13 sind **Bau**, welche **Entscheidung**? (Dateien: `bin/post_lookup`,
`bin/secrets_keys` ×2, `bin/all_wide`, `commit_check.rs`, `tools-map.md`, `eigene-ephemeride.md` ×3,
`kanal-ontologie` ×4, `archivar-mathematikerin`.)

Bitte: **Verdikt + der stärkste Riss.** Keine Umsetzung — die ist Folge, nicht Frage.


---

## 2026-10-18-rat-gate-rat-id.md

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

## Korrektur der KPI-Nachlese (gemessen 2026-10-18, Operator: „Kann man den reasoning effort steuern?")

**Ja — und er ist die entscheidende Stellschraube.** Dieselbe Gabelungsfrage, verschiedene Budgets:

| Sitz | Reasoning-Tokens | Wahl | Kosten |
|---|---|---|---|
| Kimi K2.6 `#none` | **0** | **b — falsch** | $0,00196 |
| DeepSeek V4 Pro `#low` | 360 | **a — richtig** | **$0,00974** |
| GLM-5.3 `#low` | 1.572 | **a — richtig** | $0,01284 |
| Kimi K2.6 (default/thinking) | 2.143 | a — richtig | $0,01444 |

**Die vorige Aussage „Kimi war nötig" war falsch verglichen.** Sie stammte aus der **vollen** Frage
an drei Sitze. Auf der **gezielten** Gabelung sehen **DeepSeek und GLM** das Bootstrapping genauso —
DeepSeek am billigsten.

**Der billigste Weg mit derselben Qualität (gemessen):**
```
Schwarm (frei)  +  gezielte Gabelungsfrage an DeepSeek V4 Pro #low   =  $0,0097
```
gegen $0,0187 für die volle Frage an drei Sitze → **48 % günstiger**.

**Die drei Regeln, die daraus folgen:**
1. **Breite Frage → freier Schwarm** ($0). Er findet die Risse.
2. **Eine Gabelung → ein Reasoning-Modell mit `low`** (DeepSeek/GLM), nicht die teuerste Spitze.
3. **`#none` ist kein Schnäppchen, sondern ein Qualitätsverlust** — Kimi ohne Reasoning wählt falsch.

