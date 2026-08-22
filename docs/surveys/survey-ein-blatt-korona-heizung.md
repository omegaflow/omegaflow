<!--
  title: DAS BLATT DER KORONA-HEIZUNG — der kausale DAG der solaren Kanäle
  class: survey
  date: 2026-08-21
  sha256: 8cab868ca8304f1b0e7243d6a71a9fde7ad2df177d6471a802723a6bf9ce7439
  status: live
  see-also: docs/concepts/ein-blatt-ergebnis.md docs/concepts/kybernetische-astrophysik.md docs/reference/broken-null-control.md docs/handover/handover-2026-08-21-corona-heizung.md
-->
# DAS BLATT DER KORONA-HEIZUNG — der kausale DAG der solaren Kanäle

Selbsttragend. Dieses Blatt trägt den gemessenen kausalen DAG der
Korona-Heizung: die Transferentropie über alle Paare der solaren Kanäle
mit historischer Serie, beide Richtungen, mit Lag, Schwelle und n — auf
zwei Skalen. Die Tages-Skala (11 Jahre) trägt Stille; die Minute-Skala
(7 Tage) trägt gerichtete TE ohne fam. Beides sind Befunde (0 honored).

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

## Die Minute-Skala (nobel probe, 7-Tage-Fenster) — benannte Grenzen

`nobel_probe_corona` misst die Live-Kanäle (1-min, n ≈ 10000 Zellen):
EUV-304 → X-Ray und Bz → X-Ray tragen surrogat-signifikante TE bei
lag 0/1 (phasenrandomisierte Schwelle, mean + 2σ je Paar). Zwei Grenzen
sind benannt und dürfen nicht als Pfeil gelesen werden: (a) die
Minuten-Runde trägt **keine fam-Schwelle** — die Mehrfachvergleichs-
korrektur der Tages-Runde (fam = 2,108e-1) deckt eine andere Skala und
gilt hier nicht; die Minuten-fam ist ein Kanal-Offenposten. (b) lag 0/1
bei 1-min-Zellen löst die Alfvén-Laufzeit (~100 s) nicht auf — die
Zellen sind gröber als der gesuchte Effekt. Gemessen ist eine gerichtete
TE, die ihre eigene Surrogat-Schwelle schlägt; NICHT gemessen ist,
welcher Mechanismus (Alfvén-Wellen vs. Nanoflares) sie trägt, und ob sie
die Familien-Schwelle übersteht.

## Der Lang-Fenster-Befund (F10.7 ↔ X-Ray, 1995–2020)

`long_window_probe` auf den echten Serien (n ≈ 9090 Tages-Zellen, lag
0..7 d): alle vier Paare still unter der phasenrandomisierten Schwelle;
die Nullkontrolle hält (XRSB→F10.7 bricht nur die naive Shuffle-
Schwelle, nicht die phasenrandomisierte — die naive war das Artefakt).

## Die KDE-Sensitivität (h, h/2, 2h) — der volle Test

`solar_dag_probe --h-full` hat drei komplette Blätter gerechnet — je
Bandbreite die volle 30-Paar-Matrix, fam je Bandbreite neu aus der
stärksten Surrogat-TE der Runde. Bei Faktor 1,0 ist der Pfad
byte-identisch zum kanonischen `transfer_entropy_lag` (Crosscheck
bestanden: fam = 2,108e-1 identisch).

| Bandbreite | fam | Pfeile |
|---|---|---|
| h × 0,5 | 6,738e-1 | keine |
| h × 1,0 | 2,108e-1 | keine |
| h × 2,0 | 7,962e-2 | **Lya1216 → XRSB** (lag 7 d, TE 1,18e-1) |

**Befund:** Der DAG ist über die Bandbreiten stabil in der Stille — bis
auf EINEN Rand-Kandidaten: **Lya1216 → XRSB** trägt bei h/2 und h nur
„family bound" und wird erst bei h × 2,0 zum fam-gereinigten Pfeil. Der
Kandidat ist bandbreiten-empfindlich — er sitzt an der Kante der
Schwelle, kein robuster Pfeil. Fünf Paare kippen über die Bandbreiten
(F10.7→Dichte, XRSB→XRSA, Lya1216→XRSB, Bz→Dichte, Dichte→Bz), alle
übrigen bleiben still.

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

Die Messung trägt auf zwei Skalen — mit klarer Trennung zwischen
gemessen und geschlossen:

- **Minuten:** EUV-304 → X-Ray und Bz → X-Ray tragen surrogat-
  signifikante TE bei lag 0/1 — eine gemessene Informations-Richtung,
  keine Mechanismus-Entscheidung (die Minuten-fam fehlt, und die Zellen
  lösen ~100 s nicht auf).
- **Tage:** Stille über 11 Jahre, fam-gereinigt (fam = 2,108e-1) — mit
  dem bandbreiten-empfindlichen Rand-Kandidaten Lya1216 → XRSB (7 d,
  nur bei h × 2,0 fam-signifikant) und Bz ↔ Dichte (lag 0) als
  benannten Hinweisen.

0 honored: die Stille der Tages-Skala ist der Befund, kein Fehler. Was
das Rätsel löst (Alfvén-Wellen vs. Nanoflares) ist damit NICHT
entschieden — dafür fehlen die Minuten-fam und die sub-minütige
Auflösung. Die Maschine hinter diesem Blatt ist goldstandard-validiert:
`src/bin/te_ground_truth.rs` rekonstruiert die Schreiber-2000-Referenz
(unidirektional gekoppelte Hénon-Maps) — TE(X→Y) = 2,46e-1,
Asymmetrie 6,75× zur Gegenrichtung, nur die Familien-Schwelle stellt
den Gegenrichtungs-Rest still (fam = 4,55e-2), die c=0-Kontrolle bleibt
still. Offene Pflichten bleiben: die Minuten-fam und die Multi-Force-TE
(nobel_probe_corona v2).
