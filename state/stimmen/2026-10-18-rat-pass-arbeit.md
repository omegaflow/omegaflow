# Rat — R-2026-10-18-c: Wie wird ein Pass zur Arbeit statt zur Faltung?

**Frage:** `state/stimmen/2026-10-18-pass-arbeit-formular.md` (inkl. Nachtrag des Operators:
*messen → sofort delegieren → prüfen → schließen*)
**Gefahren:** 2026-10-18 — freier Schwarm (2 Läufe, $0) · GLM-5.3 · Mistral Large 4.
**UI-Sitze gefahren (Lumo · Fugu), 2026-10-18.**

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

**4 von 4 Sitzen: A im Kern** (GLM · Mistral · Fugu · Lumo) — mit C als Rollenform und D als Takt.
Kein Widerspruch im Verdikt.

### Die UI-Sitze

**Fugu Max High** (`chat.sakana.ai`, UI):
> Ein Pass endet mit **mindestens einem geschlossenen Punkt samt Beleg**; Lesen und Falten sind
> Vorbedingung, nicht Ergebnis. Nach dem Messen wird **in derselben Sitzung** delegiert, nicht erst
> nach dem Handover. Parallelität gehört zum Kontrakt, gedrosselt nur wo Punkte sich berühren oder
> ein Urteil verlangen. Messgröße ist **nicht `closed`**, sondern „geschlossene Punkte mit Beleg pro
> Pass" und „Orphan-Docs pro Linie = 0".
> **VERDIKT: A** — owner bleibt die Linie, delegiert wird nur Mechanik, Pass ohne geschlossenen Punkt ist kein Pass.
> **RISS:** „Niemand" als owner — die 61 Orphan-Docs müssen **vor der ersten Delegation** namentlich zugewiesen sein, sonst verteilt A nur Unsichtbarkeit statt Verantwortung.

**Lumo 2.0 Max** (`lumo.proton.me`, UI):
> Was im eigenen Handover-Fahrwasser liegt, gehört der Linie; alles darunter fällt an eine
> **Leitstellen-Triage mit Frist**. Ein Pass ohne geschlossenen Punkt ist **Lektüre, keine Arbeit**.
> Delegation ist **Kontrakt, nicht Abwurf**; Sobriety vor Speed. **Beleg vor Metrik** — der Zähler
> rendert nur, was der Ledger trägt.
> **VERDIKT: A** + B als Ausführungsmechanismus.
> **RISS:** 418 offene Punkte in 61 Docs, die niemandem zugewiesen sind — der Engpass ist **fehlende Ownership**, und der Zähler **maskiert** diese Struktur.

## Der Riss (nicht geglättet)

1. **Der Besitz-Dissens ist nur halb gelöst:** GLM sagt „jede Linie ihre eigenen, Leitstelle nur Eskalation";
   der Schwarm war gespalten (Leitstelle hausweit vs. linienlokal). **Wer die 61 heute besitzt, sagt keiner.**
2. **R-i bleibt Vorbedingung:** ohne Bullet-Zählung ist „geschlossen" nicht messbar — Mistral sagt ausdrücklich,
   das Ledger müsse die Wahrheit tragen, nicht der Zähler.
3. **Die Drosselung ist eine Zahl ohne Messung:** 3–5 (Mistral) bzw. „klein" (GLM) — niemand hat gemessen,
   wie viele parallele Subagenten tragen, bevor Kontext/Qualität kippen.

## Stand

**entschieden** (2026-10-18) — **4/4 Sitze**, kein Widerspruch.

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
