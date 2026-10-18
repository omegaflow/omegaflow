# Formular — F3: Das Register kennt `state/stimmen` nicht

**Datum:** 2026-10-18 · **Linie:** future (Leitstelle) · **Register:** `R-2026-10-18-k`
**Status:** offen — der Operator fährt den Rat (`/rat`).

## Die Frage

`register_lookup` durchsucht `docs/{handover,surveys,plans,auftrag,blatt,concepts,paper}`, aber
**nicht** `state/stimmen/`, wo alle Rat-Verdikte liegen (`grep -c stimmen` = 0). Folge: eine Linie
fragt „was ist für mich offen?" und sieht fremde Verdikte nur, wenn die Leitstelle sie **von Hand**
als Brief verteilt. Wie soll das Register die Verdikte lesen — und braucht die Wahl vorher eine RAT-ID?

## Der gemessene Befund (2026-10-18)

1. **`tools/register/**` braucht keine RAT-ID.** Hook-Test (`tools/gate/commit_msg.sh`):
   `tools/register/src/bin/register_lookup.rs` ohne RAT-ID → **exit 0**; `docs/concepts/x.md` ohne
   RAT-ID → **exit 1**. Das Architektur-Regex ist
   `^(tools/gate/|\.opencode/|AGENTS\.md$|opencode\.json$|docs/concepts/)` — `tools/register/**` fehlt.
2. **Ein `## An <linie>`-Block ist 521 B bis 542 B** (`2026-10-17-rat-offen-stand.md` → mycelium;
   `2026-10-18-rat-gate-rat-id.md` → future).
3. **Kappe `HANDOVER_CAP = 1024`** (`commit_check.rs:138`): `1024 / 542 = 1,89` → **ein** Block je
   Handover; der zweite sprengt die Kappe. **7 von 9** lebenden Handovern liegen bereits über 1024 B
   (z. B. `handover-2026-10-16-mycelium-folge361.md` = 1140 B) — eingefroren, nicht konform.

## Die Stimmen

| Quelle | Runde 1 | Runde 2 |
|---|---|---|
| Schwarm (21 Sitze) | **A 12** · B 8 · C 1 | unverändert |
| DeepSeek `#low` | B | **A** |
| Fugu Max High | B | **A** |
| Lumo 2.0 Max | C | *noch offen* |

## Die Optionen

- **A** — `state/stimmen/` als zusätzliche Dokumentwurzel im Leser aufnehmen.
- **B** — derive: Verdikte wandern als `## An <linie>`-Blöcke ins lebende Handover.
- **C** — ein eigenes Register-Dokument `state/stimmen/_register.md` (nur Index) als Quelle.
- **D** — status quo: die Leitstelle verteilt weiter von Hand.
- Zusatz: Muss die Wahl selbst eine RAT-ID im Index haben, **bevor** sie gebaut wird?

## Was der Rat entscheiden soll

1. **A, B, C oder D** — unter der gemessenen Kappe (B trägt nur **einen** Block je Handover).
2. **RAT-ID vorher?** `tools/register/**` ist heute **nicht** gewallt (exit 0) — ist die Änderung
   trotzdem ein Architektur-Akt, der eine ID im Index braucht?
3. Wenn A: Welche Dokumenttypen unter `state/stimmen/` werden gelesen — nur Verdikte, oder auch
   Formulare/Rohantworten (Rauschen)?

## Riss, der bleibt

- **`exit 0` heißt „nicht nötig", nicht „unterstützt"**: A schafft eine Wurzel, die geschrieben,
  aber möglicherweise nie gelesen wird — dann bricht A=A (die Datei existiert, ist für das Werkzeug
  aber nicht dasselbe Objekt).
- Die 7/9-Handover über der Kappe sind ein **zweiter, unabhängiger** Riss (Kappe vs. Pfad-Zwang).
- n = 1 Messung je Quelle; Lumo Runde 2 offen.

## An future

Origin: leitstelle-2026-10-18
- `R-2026-10-18-k` ist die Folge von `R-2026-10-18-e` (RAT-ID-Prüfung) für den **Leser**.
  future baut nach dem Verdikt; der Bau zitiert diese RAT-ID.

## An mountain · river · mycelium · sensory

Origin: leitstelle-2026-10-18
- Bis zum Verdikt gilt: die Leitstelle verteilt Verdikte weiter als Brief (Dispatch) — kein stilles
  Warten auf eine Lese-Sicht, die es noch nicht gibt.
