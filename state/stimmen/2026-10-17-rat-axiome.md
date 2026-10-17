# Rat — die wahren Axiome der Turn-Datei (2026-10-17)

**Anlass.** Die `AGENTS.md` (wird bei **jedem** Turn gelesen) führt fünf Axiome. Zwei sind
messbar **falsch**: `channel_ref` (0× im Bestand) und `pending — der Operator schließt`
(eine Zuständigkeit, keine Mess-Wahrheit). Der Rat soll sagen, **was bei jedem Turn gelten
muss** — in Kommandos, nicht Sätzen.

**Gefahren:** **acht Sitze.** Vier API (GLM-5.3 · DeepSeek V4 Pro `effort:low` ·
Qwen3.8-Max · Mistral Large 4) · **Opus 5.5** (`reasoning_effort:low`) · **Kimi K3**
(`reasoning_effort:low`) · zwei UI (Sakana Fugu Max · Lumo 2.0 Max).

**Ablage:** Rohantworten `state/stimmen/2026-10-17-axiome-*.txt`.

**Der Fragebogen:** `/tmp/opencode/axiom-formular.txt` · Läufe `rat_testlauf-axiome.sh`.

---

## Der gemessene Befund (der Rat urteilt darüber)

- `channel_ref` steht **0× in `phi/sources.φ`** (31 936 Zeilen). Die Kraft steht **in** der
  Messzeile (`field <name> <kernel> <kraft> <einheit>`) — aber **unsauber**: Spalte 5 trägt
  Kraft (`em` 4885, `diffusion` 471), Kernel (`inverse-square` 980), Nicht-Kräfte (`acoustic`
  435, `seismic-surface` 81), Müll (`X-BAND`, `ANTENNA`, `LEVEL`).
- Nur **13 von 7921** Mess-Zeilen tragen die Kraft **vollständig**: `channel
  energy:primary:maxwell:mixed:vacuum:unspecified:none V/m` (sechs Achsen + Einheit).
- Die **Weberin** §4: *das Teilchen ist Abstammung, nicht Kraft.* Drei Zeugen-Arten:
  S²-Richtung · Gestalt · Presence.

---

## Die Bilanz (acht Sitze)

| Sitz | `channel_ref` | `pending` | Ersatz für `channel_ref` |
|---|---|---|---|
| **GLM-5.3** | überholt (toter Name + falscher Kern) | **raus** | **der Zeuge** (richtung·gestalt·presence) |
| **DeepSeek V4 Pro** | überholt | **raus** | `channel <sechs-achsen>` / `witness` |
| **Qwen3.8-Max** | überholt (fällt als globale Zeile) | **raus** | kein globaler Name; sechs-Achsen als **Datenprüfung** |
| **Mistral Large 4** | überholt („Phantom-Referenz") | **raus** | **`kraft.im.feld`** + `abstammung.vorrang` |
| **Opus 5.5** | „Gehalt Axiom, Wortlaut falsch" | **raus** | `channel <sechs-achse> <einheit>` + Weberin |
| **Kimi K3** | überholt | **raus** (fragt selbst) | `Sample ⇒ channel` / Weberin-Zeile |
| **Sakana Fugu** | überholt | **behält** (`pending state`) | **`witness S2 gestalt presence`** |
| **Lumo 2.0** | überholt („99,84% falsch") | **Status** | `force_field …` |

**Konsens 8/8: `channel_ref` ist tot.** Der Name zeigt auf nichts; die Sache lebt.
**Konsens 6/8: `pending — der Operator schließt` ist eine Prozessregel — raus** aus der Turn-Datei.
**Konsens 8/8: `ICRS/TDB` bleibt** (eigenständig, konkretisiert `A = A`).

---

## Die zwei Schulen (der ungeglättete Riss)

**Schule A — der Ersatz ist die Channel-Identität (sechs Achsen).**
GLM (teilweise)·DeepSeek·Qwen·Mistral·Opus·Kimi:
```
channel <erhaltung>:<rang>:<operator>:<charakteristik>:<medium>:<bindung> <einheit> — sonst kein sample
```
Das ist die **gemessene** Form (13× belegt). Sie steht in der Datenzeile — nicht als Axiom,
sondern als **Prüfung** (Qwen: „Datenprüfung, nicht Turn-Axiom").

**Schule B — der Ersatz ist der Zeuge.**
GLM·Sakana Fugu·(Lumo):
```
witness S2 gestalt presence
```
Die Weberin-Form. Sie sagt, **wie** eine Sache zeugt (Richtung·Gestalt·Presence) — nicht,
welche Kraft sie trug.

**Opus' Vermittlung** (der schärfste Satz): *„An die Stelle tritt die explizite `channel`-Zeile
mit sechs Achsen **und** die Weberin-Regel: Gemessen wird der **Träger**, nicht die Sache."*

---

## Die Fünf-Zeilen-Vorschläge, wörtlich

**GLM-5.3:**
```
A = A
messe in ICRS/TDB — kein Körper ist Ursprung
ohne Zeuge kein Sample — richtung · gestalt · presence
0 ist Messung
messen · bauen · abschließen · committen · pushen
```

**DeepSeek V4 Pro:**
```
assert A = A
frame ICRS/TDB
sample require six_axes
witness S² gestalt presence
honor 0
```

**Qwen3.8-Max** (nur die Arbeit — „die Datei trägt keine Sätze"):
```
messen
bauen
abschließen
committen
pushen
```

**Mistral Large 4:**
```
A=A
ICRS/TDB
kraft.im.feld
0.honored
abstammung.vorrang
```

**Opus 5.5:**
```
A = A
rahmen ICRS/TDB · alle körper gleich
channel <erhaltung>:<rang>:<operator>:<charakteristik>:<medium>:<bindung> <einheit> · sonst kein sample
miss den träger · nicht die sache
0 bleibt 0 · nie leer füllen
```

**Kimi K3** (aus dem Reasoning — sie rang bis zur letzten Zeile):
```
A = A
miss jeden Körper in ICRS/TDB
gib jedes Sample mit channel
ehre die 0
schließe kein pending
```

**Sakana Fugu:**
```
A=A
frame ICRS TDB
witness S2 gestalt presence
0 honored
pending state
```

**Lumo 2.0** (verweigert die Liste — „Workflow muss ein Zustand sein"):
```
force_field <name> <source> <magnitude> <direction> <unit>
messen bauen abschließen committen pushen
```

---

## Die gemeinsamen Zeilen (was fast alle tragen)

Über 8 Sitze stimmen **vier** Zeilen überein (in Worten, nicht Wortlaut):
1. **`A = A`** — 8/8, wortlos.
2. **`ICRS/TDB`** — 8/8 (GLM/Qwen: als Kommando „messe in ICRS/TDB").
3. **`0 honored`** — 8/8 (Opus: „0 bleibt 0"; GLM: „0 ist Messung").
4. **die Arbeit** — `messen · bauen · abschließen · committen · pushen` (GLM, Qwen, Lumo; GLM nennt sie „das einzige Prozess-Gebot, das in jedem Turn ohne es falsch handelt").

**Die fünfte Zeile ist der Streit:** der Zeuge/Channel (Schule A/B) **oder** `pending`.

---

## Die neuen Risse (teure Sitze)

- **Opus — `0 honored` ist mehrdeutig:** „ob Null als Wert, als Abwesenheit oder als
  Zuständigkeit" — nie sauber definiert.
- **Opus — mit `pending` geht ein Schutz verloren:** „Die Regel verhindert, dass ein Turn eine
  Front eigenmächtig schließt. Sie muss an einen anderen Ort wandern (Workflow/Commit-Hook)."
- **DeepSeek — `six_axes` ist ungetestet:** „ein aus dem Befund abgeleitetes Konstrukt, nicht
  im Bestand erprobt."
- **Lumo — Kommandos lesen Axiome zu spät:** „Wenn du ‚bauen' lädst und erst dann A = A prüfst,
  hast du schon gebaut — die Regel kam zu spät."
- **Mistral — `abstammung.vorrang`** als eigenes Axiom: ohne es wird `em` mit der Sache verwechselt.

---

## Der Level-Befund (Nebenergebnis dieser Runde, gemessen)

**Kein bisheriger Rat-Lauf setzte je einen Reasoning-Level** — alle fuhren Default. Dieser Tag litt
an **abgeschnittenen Antworten** (GLM, DeepSeek, Kimi `length`; Qwen Timeout). Gemessen:

| Modell | Parameter | Werte | Default | `low`-Ergebnis |
|---|---|---|---|---|
| **DeepSeek V4 Pro** | `reasoning_effort` | low/high/max | **high** | `stop`, volle Antwort |
| **Kimi K3** | `reasoning_effort` | none/low/high/**max** | **max** | `stop` in 15 s, 9 reasoning-tok (Default: 22 754 tok, `length`) |
| **Opus 5.5** | `reasoning_effort` | — | — | `stop` |
| **Qwen3.8-Max** | `reasoning_effort` | — | — | (Tag belegt) |
| **Mistral Large 4** | `capabilities.reasoning` | — | — | `stop` |

**`low` liefert vollständig; `high`/`max` kappt (ins Reasoning); `none` dünnt** (Kimi `none`:
645 out-tok, ohne Verdikt-Kopf — zu wenig für eine Urteilsfrage). **Der Rat fährt künftig `low`.**
