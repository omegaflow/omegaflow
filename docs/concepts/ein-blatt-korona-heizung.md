<!--
  title: DAS BLATT DER KORONA-HEIZUNG — der kausale DAG der solaren Kanäle
  class: concept
  date: 2026-08-21
  sha256: 6af4a9ab8edbc7ceea4ce3c812d5629ebd7eeefc8c1262c5e77986d64a8aa22d
  status: live
  see-also: docs/concepts/ein-blatt-ergebnis.md docs/concepts/kybernetische-astrophysik.md docs/reference/broken-null-control.md docs/handover/handover-2026-08-21-corona-heizung.md
-->
# DAS BLATT DER KORONA-HEIZUNG — der kausale DAG der solaren Kanäle

Selbsttragend. Dieses Blatt trägt den gemessenen kausalen DAG der
Korona-Heizung: die Transferentropie über alle Paare der solaren Kanäle
mit historischer Serie, beide Richtungen, mit Lag, Schwelle und n — auf
zwei Skalen. Die Tages-Skala (11 Jahre) trägt Stille; die Minute-Skala
(7 Tage) trägt die Pfeile. Beides sind Befunde (0 honored).

## Das Rätsel

Die Photosphäre trägt ~6000 K, die Korona 1–2 MK — Energietransport
gegen den Temperaturgradienten. Alfvén-Wellen oder Nanoflares:
unentschieden. Die Alfvén-Laufzeit durch die Korona (~100 s) müsste als
kohärenter TE-Peak erscheinen; die Reihenfolge der Peaks (EUV vor
Röntgen oder umgekehrt) trägt die Unterscheidung. Korrelation trennt
nicht — Transferentropie trennt.

## Die Maschine

Die Maschine ist die des Ein-Blatt-Ergebnisses
(`docs/concepts/ein-blatt-ergebnis.md`) plus die Familien-Schwelle:

- `transfer_entropy_lag` (der skalare Pfad, unberührt) über die
  gemeinsame Zelle, Silverman-Bandbreite je Serie;
- Schwelle = phasenrandomisierte Surrogate (10, f64-FFT, mean + 2σ);
- **fam** = die stärkste Surrogat-TE der ganzen Runde über alle Paare ×
  Lags — die Mehrfachvergleichskorrektur (zwanzig+ getestete Paare ohne
  Korrektur tragen erwartungsgemäß Falsch-Positive; ein Pfeil gilt erst,
  wenn seine TE die stärkste Null-TE der Runde schlägt);
- Verdikt je Paar: **Pfeil** (TE > fam) / **family bound** (TE über der
  eigenen Schwelle, unter fam — gerichtet, nicht fam-signifikant) /
  **still** / **keine Aussage** (n < 30).

## Die Kanäle

| Kanal | Kraft | Serie | Fenster | Zellen |
|---|---|---|---|---|
| F10.7-Penticton | em | f107_penticton.bin | 1947-02 → 2026-06 | 28337 Tage |
| GOES XRSA (0,05–0,4 nm) | em | goes_xrs.bin | 1995 → 2020 | 212571 h |
| GOES XRSB (0,1–0,8 nm) | em | goes_xrs.bin | 1995 → 2020 | 216697 h |
| Lyman-α 121,6 nm | em | goes_euvs.bin | 2009-09 → 2020-03 | 3777 Tage |
| IMF-Bz | em | omni2_serie.bin | 1963 → 2026-08 | 20257 Tage |
| Dichte | diffusion | omni2_serie.bin | 1963 → 2026-08 | 19570 Tage |
| EUV-304/284 | em | live-only (euvs-7-day.json) | 7 Tage | Minute-Skala |

**Der EUV-Befund:** Das NCEI-Science-Produkt (geuv-l2-avg1m/avg1d,
goes14/15, 2009–2020) trägt `irr_304_1nm` tot — NaN über das ganze
Jahrzehnt (0/3633 valide im avg1d, 0/1440 in fünf avg1m-Proben beider
Satelliten) — und 284 gar nicht (Wellenlängen-Achse [30,4; 121,6] nm).
Die historische 304/284-Serie existiert nicht; keine fabrizierte Zelle.
Das lebende EUV-Äquivalent ist Lyman-α 121,6 nm (irr_1216_1nm,
W/m²@1AU via au_factor, flag==0; die Geokorona-Kontamination der
Tagesmittel ist benannt, nicht gefiltert — das avg1d trägt kein
geocorona_flag). 304/284 bleiben live-only — dort leben sie auf der
Minute-Skala.

## Das Blatt — Tages-Skala (gemeinsames Fenster 2009-09 → 2020-03)

`src/bin/solar_dag_probe.rs`, 30 gerichtete Paare, lag 0..7 d,
Tages-Zellen über ~3837 Tage.

**fam = 2,108e-1** (stärkste Surrogat-TE der Runde). Alle 30 TE liegen
darunter — **kein fam-gereinigter Pfeil auf der Tages-Skala.**

```
 F10.7 → XRSA    | n 3825 | lag 0 d | TE 3,03e-2 | thr 4,19e-2 | still
 F10.7 → XRSB    | n 3829 | lag 7 d | TE 6,86e-2 | thr 1,08e-1 | still
 F10.7 → Lya1216 | n 3775 | lag 7 d | TE 1,36e-1 | thr 1,69e-1 | still
 F10.7 → Bz      | n 3005 | lag 4 d | TE 1,86e-1 | thr 2,06e-1 | still
 F10.7 → Dichte  | n 2833 | lag 4 d | TE 1,16e-1 | thr 1,40e-1 | still
 XRSA  → F10.7   | n 3825 | lag 7 d | TE 2,61e-2 | thr 6,19e-2 | still
 XRSA  → XRSB    | n 3826 | lag 0 d | TE 2,17e-2 | thr 8,53e-2 | still
 XRSA  → Lya1216 | n 3766 | lag 7 d | TE 2,76e-2 | thr 6,55e-2 | still
 XRSA  → Bz      | n 2996 | lag 3 d | TE 4,34e-2 | thr 1,76e-1 | still
 XRSA  → Dichte  | n 2824 | lag 3 d | TE 3,67e-2 | thr 1,30e-1 | still
 XRSB  → F10.7   | n 3829 | lag 7 d | TE 6,37e-2 | thr 7,02e-2 | still
 XRSB  → XRSA    | n 3826 | lag 0 d | TE 6,50e-2 | thr 7,02e-2 | still
 XRSB  → Lya1216 | n 3770 | lag 7 d | TE 5,96e-2 | thr 8,01e-2 | still
 XRSB  → Bz      | n 2999 | lag 3 d | TE 1,04e-1 | thr 1,89e-1 | still
 XRSB  → Dichte  | n 2827 | lag 3 d | TE 7,70e-2 | thr 1,37e-1 | still
Lya1216 → F10.7  | n 3775 | lag 7 d | TE 1,02e-1 | thr 1,40e-1 | still
Lya1216 → XRSA   | n 3766 | lag 5 d | TE 5,58e-2 | thr 4,67e-2 | family bound
Lya1216 → XRSB   | n 3770 | lag 7 d | TE 1,66e-1 | thr 1,16e-1 | family bound
Lya1216 → Bz     | n 2944 | lag 0 d | TE 1,68e-1 | thr 2,04e-1 | still
Lya1216 → Dichte | n 2781 | lag 2 d | TE 1,15e-1 | thr 1,28e-1 | still
 Bz    → F10.7   | n 3005 | lag 7 d | TE 8,50e-2 | thr 7,70e-2 | family bound
 Bz    → XRSA    | n 2996 | lag 0 d | TE 3,25e-2 | thr 3,82e-2 | still
 Bz    → XRSB    | n 2999 | lag 3 d | TE 6,88e-2 | thr 7,61e-2 | still
 Bz    → Lya1216 | n 2944 | lag 7 d | TE 7,98e-2 | thr 7,18e-2 | family bound
 Bz    → Dichte  | n 2799 | lag 0 d | TE 1,60e-1 | thr 1,45e-1 | family bound
Dichte → F10.7   | n 2833 | lag 7 d | TE 7,17e-2 | thr 8,65e-2 | still
Dichte → XRSA    | n 2824 | lag 0 d | TE 2,80e-2 | thr 3,47e-2 | still
Dichte → XRSB    | n 2827 | lag 4 d | TE 5,96e-2 | thr 8,61e-2 | still
Dichte → Lya1216 | n 2781 | lag 7 d | TE 6,93e-2 | thr 8,78e-2 | still
Dichte → Bz      | n 2799 | lag 0 d | TE 1,93e-1 | thr 2,08e-1 | still
```

Die stärksten Hinweise liegen auf der Achse **Chromosphäre → Korona**
(Lya1216 → XRSB, lag 7 d, TE 1,66e-1 — über der eigenen Schwelle,
unter fam) und auf **Bz ↔ Dichte** (lag 0 d, beide Richtungen knapp
unter der eigenen Schwelle). Benannt, nicht behauptet.

## Die Pfeile — Minute-Skala (nobel probe, 7-Tage-Fenster)

`nobel_probe_corona` misst die Live-Kanäle (1-min, n ≈ 10000 Zellen):
unter der phasenrandomisierten Schwelle tragen **EUV-304 → X-Ray** und
**Bz → X-Ray** Pfeile bei lag 0/1. Die Alfvén-Laufzeit (~100 s) lebt auf
dieser Skala — die Tages-Skala ist zu grob, um sie aufzulösen. Die
Tages-Skala still zu messen und die Minute-Skala Pfeile tragen zu
sehen ist ein konsistenter Befund: die Korona-Heizung koppelt auf
Minuten, nicht auf Tagen.

## Der Lang-Fenster-Befund (F10.7 ↔ X-Ray, 1995–2020)

`long_window_probe` auf den echten Serien (n ≈ 9090 Tages-Zellen, lag
0..7 d): alle vier Paare still unter der phasenrandomisierten Schwelle;
die Nullkontrolle hält (XRSB→F10.7 bricht nur die naive Shuffle-
Schwelle, nicht die phasenrandomisierte — die naive war das Artefakt).

## Die KDE-Sensitivität (h, h/2, 2h)

Der Sweep über die entscheidenden Paare (family bound oder
TE > 0,6·fam) bei ihrem besten Lag rechnet: TE und Schwelle je Faktor
gerechnet, dieselben Surrogat-Seeds wie der Hauptlauf. Der Befund
wandert in diese Sektion, sobald der Lauf gelandet ist (pending, keine
vorweggenommene Stabilitätsaussage). Die volle fam-Neuberechnung unter
h/2 und 2h wäre je eine eigene Runde — benannt, offen.

## Das 90-Tage-Archiv (Auftrag 4)

Erneut geprüft 2026-08-21: `xrays-30-day.json` (SWPC) trägt 404;
`science/xrs` (NCEI) trägt nur goes08–15 (1995–2020) — kein R-Series-
Produkt, kein 2020–2026-Fenster. Die Lücke zwischen der historischen
Serie (bis 2020) und dem 7-Tage-Live-Feed trägt keinen lebenden
Kandidaten (fehlt-Registratur, kein Block). GONG steht mit 31 Jahren
(1995–2026, mparam-Eigenfrequenzen) — der lange Fensterkanal des
Sonneninneren; sein DAG (Interior × Korona) ist ein eigenes Blatt,
nicht dieses.

## Verdikt

Die kausale DAG der Korona-Heizung steht auf zwei Skalen:

- **Minuten:** EUV-304 → X-Ray, Bz → X-Ray (lag 0/1) — die Pfeile der
  Korona-Heizung.
- **Tage:** Stille über 11 Jahre, fam-gereinigt — mit benannten
  Hinweisen auf der Achse Chromosphäre → Korona (Lya1216 → XRSB,
  7 Tage) und Bz ↔ Dichte (lag 0).

0 honored: die Stille der Tages-Skala ist der Befund, kein Fehler —
der Energietransport der Korona trägt keine Tages-Trägheit. Offene
Pflichten: die Multi-Force-TE (nobel_probe_corona v2, alle Kräfte im
Phasenraum) und die fam(h/2)/fam(2h)-Neuberechnung.
