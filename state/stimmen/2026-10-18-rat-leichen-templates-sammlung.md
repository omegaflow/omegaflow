# Rat — R-2026-10-18-o: Leichen, Zähler, Templates (2026-10-18)

**Frage:** Sollen die drei Schnitte (eingefrorene Baseline, ein Zähler, Marker fällt beim
Schließen) plus ein Template-/Dateinamen-Zwang als ein Gate-Gesetz gebaut werden — und in
welcher Reihenfolge?
**Formular:** `2026-10-18-rat-leichen-templates-formular.md` · **Index:** `_rat-register.md`
**Runde:** API-Sitze über das Gateway (9) — kein UI-Roster. Der Operator kann den UI-Roster
ergänzen, das Verdikt steht schon.
**Korrektur (2026-10-19):** auch diese Runde fuhr paid (DeepSeek, Kimi K3, Claude Sonnet 5.5,
Qwen3.8-Max) **ohne Operator-Wort** — Verstoß gegen `_rat-anweisung.md` §0a. Der freie Schwarm
(Nemotron, pollinations, Gemma, GLM-5.2) war bereits einstimmig **B**; die paid-Sitze sind
überzogen und tragen nichts zum Verdikt bei.

## Die Bilanz — neun Sitze, ein Verdikt

| Sitz | Verdikt | benannter Riss |
|---|---|---|
| DeepSeek V4 Pro | **B** | Baseline vs. „keine harten Schwellen"; ci-gate rot |
| Kimi K3 | **B** | Baseline ungeprüft — wie viele der 34 sind echt vs. tote Marker |
| Claude Sonnet 5.5 | **B (zwei Stufen)** | 34 ungeprüft; ci-gate rot + Bypass; Baseline erst nach Prüfung |
| Nemotron Ultra (kilo) | **B** | Gate-Mechanismus selbst kompromittiert (Bypass) |
| Qwen3.8-Max | **B** | Template-Migrationslast ungemessen |
| Gemma4 31b | **B** | Admin-Bypass entwertet neue Gates |
| pollinations/openai-fast | **B** | Migrationsdauer/Releases |
| GLM-5.2 (Mistral) | **B** | Gate rot + umgangen; Cut 3 braucht Träger-Reparatur |
| Nemotron (openrouter) | *keine Antwort* | — |

**Verdikt: B — einstimmig (8/8 antwortende Sitze).**

## Die Reihenfolge (Claude, einhellig gestützt)

(2) ein Zähler → (3) Marker fällt beim Schließen → einmalige Prüfung der 34 Orphans →
(1) Baseline einfrieren → Template-Migration → Gate scharf.

## Die Risse (vom Rat benannt)

1. **ci-gate rot seit `0208914fd`, vom Admin-Bypass verdeckt** — der Durchsetzungskanal ist
   kompromittiert; kein neues Gate darf vor der Reparatur scharf werden (gfc.rs
   `excessive_precision` grün, Bypass für dieses Gate weg).
2. **Die 34 sind ungeprüft** — nicht gemessen, wie viele echte offene Punkte vs. tote Marker.
   Eine Baseline friert den Bestand ein; der Bestand muss vorher auditiert werden.
3. **Template-Migrationslast ungemessen** — wie viele der 34/Drafts heute durchfallen würden.
4. **Cut 3 braucht einen Träger-Reparatur-Mechanismus** (Marker entfernen) — in keinem der drei
   Schnitte spezifiziert.
5. **Die Leichen gehören den Linien** (River: Entwürfe/Sweep/`Braucht`; Mountain: ci-gate,
   `matrix-vlies`; Sensory: `.17`/`.12`) — nicht stillschweigend mitzuschließen.

## Wer baut (nach Verdikt)

- **future:** die drei Schnitte bauen (ein Zähler, Baseline, Marker-Fall) — erst, nachdem
  ci-gate grün ist.
- **future:** Template-/Dateinamen-Migration zuerst messen, dann das Gate scharf.
- **river:** Mond-Faden, cancelled-Lauf erklären, zwei falsche `Braucht`-Zeilen korrigieren.
- **mountain:** ci-gate grün (`gfc.rs`), `matrix-vlies` `n=0` benennen.
- **sensory:** Ledger `.17`/`.12` schließen.

## An future

Origin: rat-2026-10-18-o
- R-2026-10-18-o Verdikt **B**: drei Schnitte sofort, Templates als Migration, Gate danach
  scharf. Reihenfolge wie oben. ci-gate zuerst grün.

## An river

Origin: rat-2026-10-18-o
- Mond-Faden bauen (SSCWeb-Lat/Lon, `ephemeris_moon.bin`, `--cond moon`), den cancelled-Lauf
  erklären, die zwei falschen `Braucht`-Zeilen korrigieren.

## An mountain

Origin: rat-2026-10-18-o
- ci-gate grün stellen (`gfc.rs excessive_precision`, Admin-Bypass für dieses Gate weg) —
  bevor irgendein neues Gate scharf wird.

## An sensory

Origin: rat-2026-10-18-o
- Ledger `.17`/`.12` schließen (Quittung liegt im Postfach).
