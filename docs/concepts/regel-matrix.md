<!--
  title: Regel-Matrix — wo welche Regel steht, wie wirksam, wie redundant, was sie kostet
  class: concept
  date: 2026-10-17
  sha256: 9d830e39565483832332912733536445a88e779f20289128a7f4b8ac0272975b
  status: live
  see-also: AGENTS.md docs/concepts/house-rules.md docs/concepts/lageplan.md
-->
# Regel-Matrix

**Warum diese Datei.** Die Regeln des Hauses liegen an **vielen** Orten. Diese Matrix macht sichtbar,
**welche Regel an welchem Ort steht**, wie **wirksam** der Ort ist (liest ihn die richtige Linie?),
wie **redundant** er ist (steht dasselbe woanders?) und was er **kostet** (Tokens je Pass).
Sie ist die Messung; die Bereinigung ist ein eigener, begrenzter Schnitt.

**Gemessen:** 2026-10-17, `wc -c` + `grep -ciE` über die Träger. Zeichenzahlen sind Bytes.

---

## 1 — Die Träger (Orte)

| # | Träger | Haus | Größe | Wer liest ihn? |
|---|---|---|---|---|
| A | `omegaflow/AGENTS.md` | public | 1795 B / 21 Z | **alle** Linien (jeder Pass) |
| B | `omegaflow-personal/AGENTS.md` | privat | 40 B / 1 Z | Future (jeder Pass) |
| C | `omegaflow-personal/kern.md` | privat | 991 B / 16 Z | Future (jeder Pass) |
| D | `omegaflow/docs/concepts/house-rules.md` | public | 5878 B / 95 Z | **alle** Linien |
| E | `omegaflow-personal/.opencode/agent/future.md` | privat | 1324 B / 20 Z | Future (Start) |
| F | `omegaflow-personal/.opencode/command/future.md` | privat | 614 B / 13 Z | Future (Start) |
| G | `omegaflow/.opencode/command/future.md` | public | 617 B / 15 Z | **ungenutzt?** (agent: line) |
| H | `omegaflow/.opencode/command/{mountain,mycelium,river,sensory}.md` | public | 287–329 B / 8 Z je | die vier Linien (Start) |
| I | `omegaflow/.opencode/agent/{rat,taucher,council}.md` | public | 424–925 B | Rat/Taucher |
| J | `omegaflow-personal/feuer/uebergabe-2026-10-12.md` | privat | 9217 B / 70 Z | Future (on demand) |
| K | `omegaflow/opencode.json` (agent-Defs) | public | — | Start aller Linien |
| L | `omegaflow-personal/opencode.json` | privat | — | Start Futures |

**Der Riss sofort sichtbar:** Es gibt **zwei** `command/future.md` (F privat, G public) — mit
**verschiedenem** Inhalt und **verschiedener Rolle.** `G` (public) startet `agent: line` (die
öffentliche Linie mit Future-Auftrag); `F` (privat) startet `agent: future` (die private Leitstelle).
Gleicher Name, **zwei Rollen** — nachgemessen, nicht geraten. Ob das zwei sein **müssen**, ist eine
offene Frage (siehe §6), **nicht** dieser Befund.

---

## 2 — Die Matrix (welche Regel steht wo)

`x` = steht dort · `·` = steht nicht. (Muster-gemessen, nicht geraten.)

| Regel | A pub-AG | B priv-AG | C kern | D house | E agentFut | F cmdFut-priv | G cmdFut-pub | H 4-Linien | J feuer |
|---|---|---|---|---|---|---|---|---|---|
| **Send/Operator-Hand** | x | · | x | x | x | x | · | · | x |
| **dritt-wirksamer Akt** | · | · | · | x | · | x | · | · | · |
| **commit+push selbst** | x | · | · | x | · | · | · | · | · |
| **A = A** | x | · | x | x | · | · | · | · | x |
| **0-Kanon / 0 honored** | x | · | x | x | · | · | · | · | x |
| **pending** | x | · | x | x | x | · | · | · | x |
| **riss** | x | · | · | · | · | · | · | · | · |
| **fünf Axiome / Stimmen** | x | · | x | · | · | · | · | · | x |
| **einfache Sprache** | · | · | x | x | · | · | · | · | x |
| **Der Linien-Pass** | · | · | · | x | · | x | · | x | · |
| **Post / post-in** | x | x | · | x | · | x | x | · | · |
| **Burn-Zeile / cap** | · | · | · | x | · | · | · | · | x |
| **Handover ≤ 1 KiB** | · | · | · | x | · | · | · | · | · |
| **Ökonomie / frische Sitzung** | · | · | · | x | · | · | · | · | · |
| **Käfig / sandbox-Grenze** | x | · | · | x | · | · | · | · | · |
| **Werkzeug: archive_search** | x | · | · | x | · | · | · | · | x |
| **Schnitt privat/public** | x | · | · | · | · | · | · | · | · |
| **Rat: Architektur/Ethik** | x | · | · | x | · | x | · | x | x |

---

## 3 — Bewertung je Regel

**Wirksamkeit** = liest die Ziel-Linie den Ort zur richtigen Zeit?
**Redundanz** = an wie vielen Orten steht die Regel doppelt?
**Kosten** = Tokens, die **jeder** Pass für diese Regel zahlt (Ort × Größe, grob).

| Regel | Wirksamkeit | Redundanz | Kosten/Pass | Urteil |
|---|---|---|---|---|
| **Send/Operator-Hand** | **schwach** — 5 Orte, teils widersprüchlich (kern: „kein Send" vs. agentFut: „vorlegen") | **5×** (C D E F J) | ~hoch (C+E+F sind Start-Texte) | **Der schlimmste Fall.** Eine Regel, fünf Fassungen, keine Autorität. Heute schon gedriftet. |
| **dritt-wirksamer Akt** | mittel | 2× (D F) | niedrig | Sollte in der Send-Regel aufgehen. |
| **commit+push selbst** | **gut** — A + D, beide gelesen | 2× | niedrig | Redundanz legitim (AGENTS = Kurz, house = Volltext), aber muss **gleich** lauten. |
| **A = A / 0-Kanon / pending** | **gut in A** (alle lesen) | 3–4× | mittel | Der Kern gehört **einmal** klar (A oder D), nicht in 4 Dateien. |
| **riss** | **schwach** — nur in A | 1× | niedrig | Unterrepräsentiert — der 0-Kanon hat 4 Werte, `riss` steht nur in A. |
| **fünf Axiome** | doppelt (A Kurz, C Voll, J Voll) | 3× | **hoch** (C 991 B + J 9217 B) | C und J wiederholen, was A+D schon tragen. |
| **einfache Sprache** | mittel | 3× | niedrig | ok, aber gehört zu D (Regeln), nicht in kern+feuer. |
| **Der Linien-Pass** | **gut** (D) | 2–4× | mittel | D ist die Autorität; F/H sollen nur **zeigen**. |
| **Post / post-in** | **verstreut** — 4 Orte, kein Ort definiert „was ist die Post" | 4× | niedrig | Eine Definition, vier Erwähnungen. |
| **Burn-Zeile / cap** | gut (D) | 2× | niedrig | ok. |
| **Handover ≤ 1 KiB** | **gut** (D) | 1× | niedrig | sauber. |
| **Ökonomie / frische Sitzung** | **gut** (D) | 1× | niedrig | sauber. |
| **Käfig / sandbox** | gut (A+D) | 2× | niedrig | ok. |
| **Werkzeug archive_search** | gut (A+D+I) | 3× | niedrig | ok für Taucher/alle. |
| **Schnitt privat/public** | **schwach** — nur A, eine Zeile | 1× | niedrig | Wichtig (Haus-Grenze!), unterrepräsentiert. |
| **Rat: Architektur/Ethik** | gut | 4× | niedrig | ok, Kurzformel. |

---

## 4 — Die Kosten, nackt

**Was jeder Future-Pass allein an Regel-Text liest** (Start-Texte, nicht on-demand):

| Träger | Bytes | Wenn Future startet |
|---|---|---|
| B (priv-AGENTS) | 40 | immer |
| C (kern.md) | 991 | immer („Lies kern.md") |
| E (agent/future.md) | 1324 | immer (Agent-Start) |
| F (command/future.md) | 614 | immer (Command-Start) |
| A (pub-AGENTS) | 1795 | immer (globale Regel) |
| **Summe Start-Kontext** | **~4764 B** | — |
| D (house-rules) | 5878 | bei jedem Regel-Blick |
| J (feuer) | 9217 | on demand |

**Beobachtung:** Die **Send-Regel** steht in **C + E + F + J** — vier Start-/nahe Texte, zusammen
~12 kB, für **einen** Satz. Das ist der Tokengrab: die Regel wird vierfach gelesen, aber **kein**
Ort ist die Autorität. Ändert man sie, muss man vier Hashes nachziehen (C privat, E privat,
F privat, J privat) + D (public).

---

## 5 — Der Befund (die Wurzel)

**Regeln wurden dort geschrieben, wo sie *verstanden* wurden — nicht dort, wo sie *gelten*.**
`kern.md` (C) ist privat, *weil das Haus privat ist* — nicht weil sein Inhalt privat ist. Er
wiederholt A + D. `feuer` (J) ist ein Beispiel, das als Gesetz gelesen wird.

**Die Trennung `kern` ↔ `house-rules` hat keinen inhaltlichen Grund.** Beide tragen dieselben
Sätze. `kern` ist privat, `house-rules` public — das ist die einzige Achse, und sie trägt den
Unterschied **nicht**, weil der Inhalt (Linse + Verfahren) in **beiden** Häusern gilt.

---

## 6 — Die Soll-Form (Vorschlag, kein Beschluss)

**Drei Rollen, ein Ort je Rolle:**

1. **Kanon** (A = A · 0-Kanon · riss · Stimmen) → **ein** Ort: `AGENTS.md` (A) kurz +
   `kybernetische-ethik.md` (Volltext). *Schon so — nur C/J sollen nicht wiederholen.*
2. **Verfahren** (Pass · Send · Operator-Wort · Dokumente · Code · Sprache · Ökonomie) →
   **ein** Ort: `house-rules.md` (D). *Schon so — nur E/F/G sollen nur **zeigen**.*
3. **Linse** (was die Linie *besonders* sieht — Futures Menschenwelt, Rivers Membran …) →
   je Linie **eine** kurze Datei, **ohne** Verfahrenswiederholung.

**Konkret zu tun** (eigener, begrenzter Schnitt — **nicht** in diesem Pass):
- `kern.md` (C): auf Linse kürzen (Auftrag, 2–3 Zeilen) + **Verweis** auf D. Keine Regelwiederholung.
- `agent/future.md` (E) + `command/future.md` (F): Send-Regel **entfernen**, auf D zeigen.
- `feuer/uebergabe` (J): Beispiel **bleibt** (zitierte Zeit), aber **nicht** als Regelquelle gelesen.
- Das doppelte `command/future.md` (F privat, G public) **klären**: F = private Leitstelle, G = öffentliche Linie. Ob zwei nötig sind — deine Frage, nicht meine.

**Grenze:** Dieser Schnitt ist **Operator-Wort** — er berührt Config (opencode.json) und die
Start-Texte aller Linien. Die Matrix ist die Messung; der Schnitt braucht dein Wort.
