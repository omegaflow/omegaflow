<!--
  title: BLATT — Solar-24-s-Sekunden-Matrix: der 72-Paare-Flotten-Beweis (XRS→EUV)
  class: sheet
  date: 2026-09-08
  sha256: fef80b7bf7f7629507576494fbebe25800efbe8130b2bdfa3bd89c03b3ddee54
  status: done
  see-also: docs/handover/archiv/handover-2026-09-09-te-atom-4.md docs/specs/spectral-oscillator.md
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

## Der Schlichtungsfall — gemessen

**211A → 193A** — der einzige intra-AIA-Pfeil — ist konditional geprüft. Die
Sonde `corona_conditional_probe` führt das Paar als `193A->211A` (Ladder
cool→hot): `TE c->h|C` ist 193→211, `TE h->c|C` ist 211→193, und
`D|C = TE(c->h|C) − TE(h->c|C)`. Konfund ist der gemeinsame XRS-Treiber
(GOES `b_flux`), Korpus 2014, 1315 Ereignisse, Lags 0/96/192 s, lag-bewusste
ARX-Null, 10 Surrogate je Zelle:

```
target/release/corona_conditional_probe --year /tmp/opencode/solar-corpus/jsoc.stanford.edu/aia2014_fullyear.bin /tmp/opencode/solar-corpus/ncei.noaa.gov/goes15 --max-lag 8 --confound goes
year 0: 1315 events (GOES b_flux > 5e-6 W/m2), window +/-40 min, 24-s cells
stack: 1315 events; confounders goes
path kde-1; null max_lag 8
conditional directional excess D|C = TE(cool->hot|C) - TE(hot->cool|C), lagged null

pair      | lag |   mean D|C | TE c->h|C | TE h->c|C | fwd/events | rev/events
193A->211A |   0s |   -1.40e-2 |    5.57e-2 |    6.97e-2 | 284/1315 | 339/1315
193A->211A |  96s |   -3.79e-2 |    7.01e-2 |    1.08e-1 | 146/1315 | 254/1315
193A->211A | 192s |   -3.36e-2 |    8.21e-2 |    1.16e-1 | 119/1315 | 176/1315
```

Der Pfeil hält: bei 96 s trägt 211→193 die größere konditionale TE
(1.08e-1 gegen 7.01e-2; D|C = −3.79e-2) und liegt in 254 von 1315 Ereignissen
über seiner lag-bewussten Null (Gegenrichtung 146). Die gemeinsame
XRS-Antwort erklärt die Richtung also nicht weg — kein Hüllen-Artefakt. Die
Gegenprobe mit allen fünf AIA-Bändern als Konfunden (Run `36194519096`, Commit
`42471cde`, Korpus `abc49399…`) ist jedoch schätzerabhängig: `binned` bestätigt
das Vorzeichen (`D|C = −3.40e-3` bei 96 s, `rev` 161/1315 gegen `fwd` 117/1315),
`ksg` kippt es (`+6.71e-3`, `fwd` 337/1315 gegen `rev` 45/1315). Der Pfeil ist
damit XRS-konditional bestätigt, all-band-konditional aber nicht schätzerrobust
— nicht mehr ungeklärt, und kein zweiter unabhängiger Zeuge.

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
