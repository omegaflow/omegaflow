<!--
  title: BLATT — Die Solar-24-s-Sekunden-Matrix: der 72-Paare-Flotten-Beweislauf (XRS treibt die heißen EUV-Bänder)
  class: sheet
  date: 2026-09-08
  sha256: 4c5dc35b3fe755ef6bdd0bb393832f55f99c16ee1ccddfeb044ee67dea2d6a46
  status: live
  see-also: docs/TODO.md docs/specs/spectral-oscillator.md
-->

# BLATT — Die Solar-24-s-Sekunden-Matrix: der 72-Paare-Flotten-Beweislauf

**Datum:** 2026-09-08 · **Axiom:** A = A

Corpus-Anker (ein Hash über alle 72 Sonden — kein Drift, das Drift-Gate ist
grün): Commit `b3b6a24`, Corpus `4f0bcf2c465d7c82eedb1e1ef978f33f9baf4fc952603a1fb06e20e8b67baa2e`.

## Die Messung

Die volle 72-Paare-Matrix im freien Myzel (Run `34243215526`, vier Wellen à 20
Sonden, ~2 h Wanduhr statt 22 h blind): neun Akteure — sieben AIA-EUV-Bänder
(304/131/171/193/211/335/94 Å) + GOES XRSA/XRSB — über 2013–2015, 24-s-Zellen,
Flare-Stacking im GOES-`b_flux`-Fenster, per-Event-D =
`(TE(A→B) − TE(B→A)) / (|…| + |…|)`, Lags 0–12 Zellen (0–288 s), 10
phasenrandomisierte Surrogate je Zelle, `fam` = stärkste Surrogat-D über alle
72 Paare × 13 Lags × 10 Surrogate.

**fam = 1.3966e-1** · Verteilungs-Blöcke: intra-AIA (42) ARROW 1 / family
bound 28 / still 13 · AIA-XRS beidseitig (28) ARROW 3 / bound 14 / still 11 ·
XRS-intern (2) bound 1 / still 1.

## Der Befund — die vier ARROWs

| Kante | lag | D | > fam | Richtung |
|---|---|---|---|---|
| 211A → 193A | 96 s | 1.658e-1 | ja | intra-AIA |
| XRSA → 131A | 192 s | 1.468e-1 | ja | XRS → AIA |
| XRSB → 131A | 96 s | 1.644e-1 | ja | XRS → AIA |
| XRSB → 193A | 96 s | 1.485e-1 | ja | XRS → AIA |

## Der Schlichtungsfall (pending)

**211A → 193A** — der einzige intra-AIA-Pfeil — trägt nur einen Zeugen (die
Matrix selbst): 211 und 193 antworten beide auf das Röntgen mit etwas
unterschiedlicher Verzögerung, und die paarweise TE liest das als „211 treibt
193". Das ist dieselbe Klasse (intra-AIA, geteilter Treiber), an der die
konditionale Sonde ihre Hüllen-Artefakt-Kippe maß. Stufe 2 der Arbeitsteilung:
die Matrix siebt, die konditionale Sonde schlichtet. Der Pfeil ist `pending`
der konditionalen Prüfung — ehrlich etikettiert, nicht überinterpretiert.

## Das Verdikt

Die **XRS-Kanäle (harte Röntgen-Flare-Treiber) zeigen ARROWs auf die heißen
EUV-Bänder 131A und 193A** (96–192 s, über der Familien-Schwelle). Die
Rückrichtung (AIA → XRS) bleibt fast ganz still — das EUV-Leuchten treibt die
harte Röntgen-Emission nicht an. Die Kausalrichtung geht vom harten
Flare-Signal zu der EUV-Nachleucht-Bande innerhalb des GOES-Fensters, nicht
umgekehrt. 4 ARROW / 43 family bound / 25 still / 0 no-statement über die
volle Matrix — die Stille der 25 Paare ist ein Befund, kein Rauschen
(0 honored).

## Der Korpus

2013/2015 als `fullyear`-Bins, 2014 als 12 Monats-Bins (AIA, `jsoc.stanford.edu`);
GOES-15-2-s-XRS als drei Jahres-Tars (`goes15_xrs_2s_{2013,2014,2015}.tar`,
`ncei.noaa.gov`). Ein Sonden-Corpus-Fetch fiel transient (Probe 26) und wurde
gezielt nachgeholt; die Reduce faltete alle 72 Berichte zu einem Blatt ohne
`missing` (0 honored — kein erfundener Wert für die Lücke).
