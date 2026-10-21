<!--
  title: Messzustände v3 — drei Achsen (Entwurf aus dem Red Team, 2026-10-21)
  class: concept
  date: 2026-10-21
  sha256: f67e324afee26c32bcfb42f908c9b54f36d1d2e605162e6f6cd3ce1821d2f96e
  status: draft
  owner: future
  ack: future 2026-10-21T14:40Z
  next: Test am Bestand — Felder mit 0 oder ≥2 Zellen je Achse
  orphan-after: none
  see-also: docs/concepts/messzustaende.md rat/2026-10-21-rat-messzustaende-v2.md
-->
# Messzustände v3 — drei Achsen

**Herkunft.** Red Team `R-2026-10-21-b` (16 Sitze). v2 hatte zu wenige Achsen:
Achse 2 mischte **Lebenszyklus** und **Zugang**, Achse 1 nahm **Zugriff** auf.
v3 trennt das.

## Die drei Achsen

**Achse 1 — Messzustand (epistemisch, was wissen wir?)**

`pending` | `gemessen` mit Qualifiern. Totalordnung:
**`absent` > `riss` > `null-echt` > `obergrenze`**.

| Zelle | Bedingung |
|---|---|
| `pending` | kein Wert; Grund `nicht_gemessen` oder `teil_datensatz` |
| `gemessen` | Wert; ohne Qualifier |
| `obergrenze` | `0 < Wert ≤ L_D`; Obergrenze, nie 0 |
| `null-echt` | Wert ist 0; PDF schließt 0 ein; exakt 0 nur bei Zählern |
| `absent` | Nicht-Existenz belegt (L + Power + Prior + positiver Beleg) |
| `riss` | Widerspruch (vergleichbar + Kette ausgeschlossen + Budget-Revision) |

**Achse 2 — Lebenszyklus (was tun wir?)**

`offen` | `wartend` | `declined`. Totalordnung: **`declined` > `wartend` > `offen`**.
`declined` ist terminal; `wartend` trägt einen Trigger.

**Achse 3 — Zugang (wer darf sehen?)**

`frei` | `LOCK` + Grund `privat` | `rechtlich`. **LOCK ändert Achse 1 und 2
nicht.** Autolock fügt nur den Grund hinzu; `unlock` entfernt nur ihn.

## Die Projektion

Ein gemessener, aber gesperrter Wert bleibt `gemessen` (Achse 1). Leser ohne
Schlüssel sehen eine **Projektion**: `versiegelt` — ausdrücklich **kein**
Achse-1-Wert, nicht `pending`, nicht 0. Die Projektion ist eine Sicht, keine
zweite Identität.

## Die Regeln

1. **Genau eine Zelle je Achse.** `A = A` gilt je Achse und je Record.
2. **`nicht_verfügbar` gibt es nicht mehr.** „Kein Wert existiert" ist
   `pending/nicht_gemessen`; „Wert existiert, Zugang gesperrt" ist
   `gemessen` + `LOCK`.
3. **`obergrenze` schließt `L_D` ein** (`≤`). Der Grenzfall `Wert == L_D` ist
   eine Zelle, keine Lücke.
4. **`riss` dominiert `obergrenze`/`absent`**, wenn beide zutreffen.
5. **Gesperrte Werte werden nicht umetikettiert**, nur redigiert.
6. **`AGENTS.md`** muss `pending` ergänzen; `granit.md` muss die
   Zugangs-Trennung vorrangig stellen (AGENTS ist read-only).

## Der Test

Für jeden Record: Zellen je Achse zählen. Erfolg ist **0 Felder mit 0 oder ≥2
Zellen**. Der Test läuft über `state/wartend.φ`, die zehn Riss-Cluster und die
16 Red-Team-Fälle; danach über den vollen Registerbestand.

## Was v2 falsch hatte

- `declined + LOCK` war eine Doppelzelle auf Achse 2 — jetzt Achse 2 + 3.
- `gemessen + LOCK` vs. `pending/nicht_verfügbar` — jetzt `gemessen` + Achse 3.
- `null-echt` vs. `obergrenze` — jetzt Totalordnung.
- `obergrenze == L_D` — jetzt `≤`.
