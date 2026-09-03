<!--
  title: Auftrag — NAVIO-Vollmission durch die Kette
  class: auftrag
  date: 2026-08-30
  status: pending
  see-also: docs/paper/twenty-second-band-ground-chain.md docs/paper/probe-front-dark-matter.md docs/paper/laic-arrow-direction.md docs/auftrag/auftrag-gaia-dr4-iapetus.md docs/auftrag/auftrag-iapetus-scan.md
-->

# Auftrag: die NAVIO-Vollmission durch die Kette — Form-Test und Transit-Sweep

## Zweck

Die Anomalie-Debatte ist nicht durch Rauschen geschlossen, sondern durch
**Daten-Dichte** — und genau die existiert: die Vollmission, in einem Zug,
konsistent. Sie wurde aber nie durch die eigene Reduktion gezogen. Dieser
Auftrag ist die reine Fortsetzung dessen, was die Kette kann: die Vollmission
durch `doppler_resid`, mit der Bande als Deduktion 0 vorne weg. Es ist der
zehnte Auftrag und der erste, der **neues Territorium** betritt statt den
Bestand zu versiegeln.

**Reihenfolge:** (flyby-2 → Merge →
Kleinpass), sequenziell, ein Commit je Auftrag. Die Linie ist der
Sauerstoff; die Konsolidierung das Immunsystem. Beide — sonst stirbt je eines.

## Kernregel (0 honored)

**Ergebnis darf Stille sein. Grenzwert ist Erfolgsformat. Und die Frage steht
vor der Maschine, nicht der Fund.** Das Novum der 20-s-Bande kam als
Nebenprodukt einer absurden Frage, nicht durch Jagd danach. Beim Form-Test ist
das messbar Interessante nicht „Anomalie entdeckt", sondern: *die ∝t²-Frage an
die Vollmissions-Daten beantwortet — was auch immer die Antwort ist.* Kein
Interpretieren über die Messung.

## Die Kette in einem Zug

1. **Deduktion 0 — Bande-Mask:** Die 20-s-Bande wird subtrahiert/maskiert,
   bevor irgendein Residuum gelesen wird. Jede DM-Front, die die Bande nicht
   zuerst abzieht, misst sie als DM. (Die Bande sitzt in jedem künftigen
   Doppler-Residuum.)
2. **Deduktion 1 — Form-Test:** Trägt das Residuum die **∝t²-Signatur einer
   konstanten Kraft** oder die **Abklingkurve der RTG-Leistung**? Das ist die
   eine Frage, die das System nie gestellt hat, aber mit eigenen Mitteln
   messbar ist. Die RTG-Abklingkurve ist die echte Null-Hypothese des Tests —
   als Kontaminant explizit mitführen, nicht verschweigen.
3. **Transit-Sweep:** Klumpen hinterlassen einen **Sprung im Residuum**, keine
   Statik — `Ruck` ist auf Transit-Signaturen zugeschnitten. Dies ist das
   einzige Design mit realer Fundchance im ganzen DM-Komplex.
4. **Grenzwert als Erfolgsformat:** Front A (Klumpen-Grenzwert auf
   Doppler-Boden) — machbar, wertvoll als Grenze, kein Fund zu erwarten. Eine
   Grenze ist ein Befund.

## Klassengrenze (nicht vermischen)

- Front C (Transit-Sweep) über der **Vollmission**; Front A als Grenze;
  Front B (Iapetus/Halo) gehört in `auftrag-iapetus-scan.md` — nicht hier.
- Form-Test und Transit-Sweep nacheinander, nicht parallel im selben Lauf:
  der Form-Test setzt den Grenzwert, auf dem der Sweep eine Abweichung als
  Sprung erkennt.

## Lieferung

Vollmissions-Residuum (ein Zug, konsistent) mit Bande-Mask; Form-Test-Ergebnis
(∝t² oder RTG-Abklingkurve); Transit-Sweep über dem Grenzwert. Antwort darf
Stille sein — dann ist sie das Ergebnis.

## Abschluss

Die ∝t²-Frage ist an die Vollmissions-Daten beantwortet. Ausgang: Signal, Grenze
oder Stille — alle drei sind Verifizierung. Bis dahin `pending`, 0 honored.

## Front-C-Verdikt (2026-09-03 — Operator-Verdikt: zurückgerollt, in Fahrt)

Der Rat stellte fest: Front C fährt als **DM-Nachweis** nicht (der Boden liegt
~10² über der Anomalie, zehn Größenordnungen über dem Klumpen-Maßstab; das
leere Netz = 1008 Positionen, 0 Flaggen ist der DM-Befund). Das DM-Limit steht.
Der Operator hat die Auftrags-Schließung jedoch zurückgerollt und Front C in
Fahrt gesetzt: der eigentliche offene Faden ist der **Form-Test (∝t² vs
RTG-Abklingkurve) an die Vollmissions-Daten** — die ∝t²-Frage, die das System
nie mit eigenen Mitteln gestellt hat (siehe `auftrag-dunkle-materie-front-c.md`).
Die NAVIO-ASCII ist live erreichbar (SPDF 200); die Instrument-Lücke (kein
Ruck-Sweep) wird durch Bau geschlossen. Ergebnis darf Signal, Grenze oder
Stille sein — alle drei sind Verifizierung.

## Front-C-Messung (2026-09-03 — Vollmission erstmals durch die Kette)

Die Vollmissions-ASCII wurde erstmals über die eigene Reduktion gezogen:
`pioneer10_doppler.bin` 908309 Records (1973-10-05..2002-03-03),
`pioneer11_doppler.bin` 967272 Records (1973-04-10..1993-07-15), je `_clean`
über `pioneer_navio_clean`. **Form-Test-Ergebnis:** der Barycentric-Rohlauf
trägt die Serie nicht; der Vollmodell-Lauf (`pioneer_doppler_moyer_navio`,
DTYPE-12, 206/85 Epochen) trägt einen **Residuum-Floor 19,0 kHz (p10) / 8,4 kHz
(p11)** — ~2e4×/8e3× über dem ~1-Hz-Signal der Anomalie; die Rest-Drift ist
Modell-Artefakt, nicht die Anomalie (0 honored). Die ∝t²-vs-RTG-Frage ist damit
an die Vollmissions-Daten beantwortet: keine getragene Form über einem
kHz-strukturierten Floor; das **sub-kHz-Residuum (DSN-Station + Orbit)** ist
der von der Reduktion selbst benannte Blocker für den Ruck-Transit-Sweep.

## Sub-kHz-Residuum — Tür geöffnet (2026-09-03, Atom 1+2)

Siehe `auftrag-dunkle-materie-front-c.md`. PNAV-Record (TRANS/RCVR1/linkmode,
PDPL unberührt) + `pioneer_navio_residuum` (Stationsmodell + displaced-count-
Mask) gebaut. Gemessen: Per-Station-Floor fällt von ~5e5 Hz auf 2,8–19 kHz;
serialisierte PNVR/PNDM-Tagesmediane konsumierbar. Atom 3 (Commit 50625aa):
Deduktion-10-Tagesmask (4×p90), p10 5 / p11 15 corrupt-day Cluster verworfen;
Ruck-Scan über gemaskte Tagesmediane läuft, Flaggen als Kandidaten gegen den
lokalen Floor — kein Detektions-Anspruch; sub-kHz-Ziel bleibt am
~1,5-kHz-Streuungsboden (0 honored). Nachfolge-Befund: P11-Flaggen fallen auf
bekannte Encounter (1974 Jupiter, 1979 Saturn) = Granulat-Überschuss; P10s
stärkste Flaggen (1996, 1981, 1982) liegen nach P10s einzigem planetaren
Encounter — nicht durch bekannte Flybys erklärt, offen (Station-/Era-Check
folgt), 0 honored.
