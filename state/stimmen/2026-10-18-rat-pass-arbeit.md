# Rat — R-2026-10-18-c: Wie wird ein Pass zur Arbeit statt zur Faltung?

**Frage:** `state/stimmen/2026-10-18-pass-arbeit-formular.md` (inkl. Nachtrag des Operators:
*messen → sofort delegieren → prüfen → schließen*)
**Gefahren:** 2026-10-18 — freier Schwarm (2 Läufe, $0) · GLM-5.3 · Mistral Large 4.
**UI-Sitze (Lumo · Fugu): ausstehend** — vom Operator zu senden.

## Die gemessene Lage

| Maß | Wert |
|---|---|
| offene Zeilen in Prosa-Docs | 418 (179 Docs) |
| Orphan-Docs ohne Träger | **61** |
| Orphan-Register-Einträge je Linie | 0 |
| stale Punkte (3 Pässe identisch) | 0 |

**Der Befund:** Die Pässe **arbeiten** (Commits: P10-Migration, GRS80, Routen-Pins) — aber die
offene Menge liegt in Docs, die **niemand besitzt**. Wer keinen Punkt hat, kann ihn nicht delegieren.

## Die API-Sitze

**GLM-5.3** (`deepinfra/zai-org/GLM-5.3#low`, `ses_eb219370fffeKvLfBBjJI30cOh`):
> Jede Linie besitzt ihre eigenen Orphans; Leitstelle nur als Eskalations-/Audit-Instanz.
> Ein Pass ist ein **Arbeits-Akt**: messen, mindestens einen Punkt delegieren, prüfen, schließen, belegen.
> Parallelität ja, aber **gedrosselt** — N ist klein, weil der Pass für jedes geschlossene Ergebnis haftet.
> Beleg: **Zitat + Commit-Hash**; das Ledger ist Nebenprodukt, nicht Kontrakt.
> Delegierbar: Mechanik mit messbarem `done-when`. **Nicht:** Urteil, Priorisierung, das Schließen selbst.
> **VERDIKT: A** — `owner` + `subagent` + `done-when`, Schließ-Urteil beim Pass.
> **RISS:** Besitzlose Orphans + Zähler-Riss (liest 0) — die Arbeit passiert, die Menge gehört niemandem und misst nichts.

**Mistral Large 4** (`mistral/mistral-large-4#none`, `ses_eb21936d5ffejnA6RPovSKlC8U`):
> 61 Orphans sind ein **Besitz-Riss, kein Mengen-Riss**. Ohne geschlossenen Punkt ist der Pass nicht gezählt.
> Parallelität ist Teil des Kontrakts, **gedrosselt auf 3–5 Punkte je Pass**.
> Beleg **dreistufig**: (a) Zitat/Verweis im Doc, (b) Commit-Hash des Agenten, (c) Ledger-Eintrag.
> **`done-when` definiert der Pass, nie der Subagent.**
> **VERDIKT: C+A mit D als Takt** — Linie besitzt, definiert, delegiert sofort N parallel, verifiziert, schließt selbst.
> **RISS:** Solange R-i nicht gebaut ist, steigt `closed` formal, während die Menge in Bullets weiterlebt — **das Ledger muss Quelle der Wahrheit sein, nicht der Doc-Zähler.**

## Bilanz

**Beide: A im Kern, mit C (Rollentrennung) und D (Pipeline als Takt).** Kein Widerspruch.
Der freie Schwarm (2 Läufe: C · A+D) hat die Risse geliefert, die beide Paid-Sitze aufgreifen:
*Resignations-Delegation* und *Durchlauferhitzer*.

## Der Riss (nicht geglättet)

1. **Der Besitz-Dissens ist nur halb gelöst:** GLM sagt „jede Linie ihre eigenen, Leitstelle nur Eskalation";
   der Schwarm war gespalten (Leitstelle hausweit vs. linienlokal). **Wer die 61 heute besitzt, sagt keiner.**
2. **R-i bleibt Vorbedingung:** ohne Bullet-Zählung ist „geschlossen" nicht messbar — Mistral sagt ausdrücklich,
   das Ledger müsse die Wahrheit tragen, nicht der Zähler.
3. **Die Drosselung ist eine Zahl ohne Messung:** 3–5 (Mistral) bzw. „klein" (GLM) — niemand hat gemessen,
   wie viele parallele Subagenten tragen, bevor Kontext/Qualität kippen.

## Stand

**entschieden** (2026-10-18, API-Sitze) — **UI-Sitze ausstehend**; sie können überstimmen.

## An future

Origin: rat-2026-10-18-c
- Register-Zeile `R-2026-10-18-c` eintragen; die 61 Orphan-Docs einem `owner` zuweisen (Linie oder Leitstelle).
- Den Pass-Kontrakt in die fünf Commands schreiben: **messen → sofort N Punkte delegieren → prüfen → schließen**;
  ein Pass ohne geschlossenen, belegten Punkt ist kein Pass.
- `done-when` definiert der **Pass**, nie der Subagent; Beleg = Zitat + Commit-Hash (+ Ledger).
- Vorbedingung: R-i (Bullet-Zählung) — sonst ist `closed` nicht messbar.

## An mountain · river · mycelium · sensory

Origin: rat-2026-10-18-c
- Jede Linie besitzt **ihre eigenen** Orphan-Docs (61 gesamt, noch nicht zugeordnet) und delegiert sie im Pass.
- Delegierbar: Mechanik mit objektivem `done-when` (Zähler, Migration, Messung, Route).
- Nicht delegierbar: Urteil, Priorisierung, das Schließen selbst.
