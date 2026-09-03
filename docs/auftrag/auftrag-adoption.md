<!--
  title: Auftrag — Adoption: Repo public, Welt-Fassung, drei Mails als EIN Block
  class: auftrag
  date: 2026-08-30
  status: pending
  see-also: docs/paper/twenty-second-band-ground-chain.md docs/auftrag/auftrag-bande-split.md
-->

# Auftrag: die Adoption — Repo, Welt-Fassung, drei Mails als ein Block

## Zweck

Bestand ist keine Eigenschaft des Befunds, sondern der Adoption. Der
Flaschenhals ist **je ein externer Leser pro Findung**. Drei Findungen × ein
Leser = die gesamte verbleibende Distanz zu Wert mit Bestand.

**Prio 1 der Welt-Zugangs-Phase — nicht „irgendwann zuletzt".** Die Mails sind
die Tür nach draußen und der einzige irreversible Akt im ganzen System
(Messungen wiederholbar, Merges revertierbar, Commits reverten — eine
gesendete Mail an Turyshev ist gesendet). Aber man öffnet eine Tür nicht mit
offenem Reißverschluss. Irreversibilität ist der Definitionsort von „höchste
Sorgfalt", nicht von „höchste Priorität". Also: die Phase beginnt **drei
Sessions nach jetzt**, nicht drei Monate.

## Kernregel (0 honored)

**Man verschickt kein Papier, dessen öffentliche Fassung noch Fehler trägt,
die der eigene Baum schon behoben hat.** Die Empfänger bekommen einen
Repo-Link; was sie dort finden, ist die öffentliche Fläche = der
Wahrheitsstand. Der erste Eindruck einer Außenwelt ist einmalig. Ein Schuss
pro Person. Autor ist, wer die Forschung geleitet und verantwortet (Operator,
vertretbar) — **nie** mit „KI hat das gefunden" führen.

## Voraussetzungen (nicht überspringbar)

1. **Session 1 — Merge-Fix-Welle** (`auftrag-merge-fix-welle.md`): öffentliche
   Fläche = Wahrheitsstand; das 79-Jahres-Fenster (signalkegel),
   planet-nines pre-fix-Stand, die unregistrierte 925d93f-Regression,
   f\*/1-s-Doppelzählungen — nichts davon darf ein Toth in fünf Minuten als
   *seine erste Meinung* über den Korpus finden. Kostet Tage, nicht Wochen.
2. **Session 2 — Bande-Split** (`auftrag-bande-split.md`): der
   two-/three-way-Split ist die inhaltliche Vorbedingung für genau die
   Toth-Mail. „wir haben eine stationsfixe Bande gefunden" ist eine
   Mitteilung; „wir haben sie charakterisiert, 31 Ausschlüsse durchgeführt,
   und die eine offene Frage ist X, die Ihre Datenlage eindeutig beantworten
   könnte" ist ein Kollaborations-Angebot — starker Ask statt schwachem.

Dann erst, **als Block.**

## Die drei Mails als EIN Block (Tag 3, erster Zug)

Gleichzeitig, gleicher sha, gleiches Repo, ein Tag — die erste Welle:

| Leser | Blatt | Haken |
|---|---|---|
| Viktor Toth (Ottawa) | 20-s-Bande | Parser ist Port von Toths bitstract |
| Slava Turyshev (JPL) | 20-s-Bande | NAVIO-Daten sind Turyshevs Zenodo-Release |
| Craig Markwardt (GSFC) | 20-s-Bande | ggf.; Bande kontaminiert deren eigene Residuen |

Prioritätsargument dafür, früh zu senden: **die Welt bewegt sich** — eine
Neuvermessung alter Doppler-Residuen, die jemand *anderer* startet, kann die
Bande finden (oder mit ihr kollidieren), ohne dass ihr erwähnt werdet. Der
Block ist der Zug, der das Risiko beendet — nachdem Merge + Split den
Reißverschluss geschlossen haben.

Mail-Instrument: zehn Zeilen, ein PDF, ein Repo-Link, ein Koautoren-Angebot
(„if you find the band and it matters to your work, we would welcome you as
co-author").

## Zweite Welle — nach der Reaktion (NICHT Blocker)

GIC (Wing / J. R. Johnson APL, Ari Viljanen FMI) und Korona (Tom Woods
EVE/LASP) gehen als **zweite Welle** — nach der Reaktion auf die erste, die
lehrt, wie die Welt antwortet. Der **p-Wert und die AIA-fam-Zahl sind KEINE
Blocker**: die GIC-Mail funktioniert mit „p-Wert folgt"-Vermerk, die Korona
kann warten oder in die zweite Welle gehen.

## Repo & Kanäle

GitHub-Repo öffentlich, std-only Rust, deterministische Seeds, Daten mit
Zenodo-DOI, pro Paper ein getaggtes Release („byte-identisch reproduzierbar"
prüfbar senkt Prüfkosten von Wochen auf Stunden). Welt-Fassung auf eigenem
Branch (One-Source-Regel, Übersetzungsregeln §7). Kanäle: Direktmail →
arXiv (Endorsement über denselben Experten) → Journale ohne Affiliationszwang
(Radio Science, Space Weather / JSSWC). **Nicht:** 30 Leute gleichzeitig,
alle 15 Blätter, mit Paradigma führen, Bezahljournale.

## Lieferung

Repo public + Zenodo-DOI + getaggte Releases; Welt-Fassung-Branch; **der
Drei-Mail-Block** (Toth/Turyshev/Markwardt) nach Voraussetzungen 1+2, als
Ein-Zug-Block; **Register-Zeile pro Mail** (an wen, wann, welcher sha) —
Antwort `pending`, nie 0.0.

## Abschluss

Der erste Zug (drei Mails als Block) ist raus = Verifizierung ist Messung
geworden (Datum, Adressat, Antwort). Beide Ausgänge — Übernahme oder Abtun —
sind Verifizierung. Die GIC/Korona-Welle folgt nach Reaktion.
