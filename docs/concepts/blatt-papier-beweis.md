<!--
  title: Der Blatt-Papier-Beweis — die Richtung der Information, auf einer Seite
  class: concept
  date: 2026-08-21
  sha256: 8c31f75b0fac33b5488db9cb97e837333e1e59d599847e1d1c4b152a8e401cf0
  see-also: docs/handover/handover-2026-08-21-blatt-enso-kausalpfeil.md docs/handover/handover-2026-08-21-blatt-bz-geomagnetisch.md docs/surveys/survey-2026-08-21-laic-pfeilrichtung.md docs/reference/broken-null-control.md
-->
# DER BLATT-PAPIER-BEWEIS

Selbsttragend. Dieses Dokument trägt das Konzept; die drei Aufträge
tragen die drei Übergaben (`docs/handover/handover-2026-08-21-blatt-*.md`).

## 0. Das Blatt ist kein Axiom

Ein Axiom wird gesetzt; das Blatt wird gemessen. Die Kürze des Blatts
kommt nicht aus einer Setzung, sondern aus der Arbeitsteilung: die
Maschine trägt die Rechnung, das Blatt trägt das Ergebnis. Eine
Richtung, ein Lag, eine Schwelle, ein Fenster — mehr steht nicht
darauf, weil mehr nicht gebraucht wird. Wer die Herleitung will, liest
die Messreihe; wer das Rätsel gelöst sehen will, liest das Blatt.

Korrelation trennt die Richtung nicht: zwei Reihen, die gleichzeitig
steigen, tragen keinen Pfeil. Die Transfer-Entropie trägt ihn:
TE(X → Y) misst, wie viel Information über die Zukunft von Y in der
Vergangenheit von X steckt, über das hinaus, was die eigene
Vergangenheit von Y schon weiß. Die Surrogat-Schwelle (mean + 2σ über
phasenrandomisierte Reihen — broken-null-control record) trennt den
Pfeil vom Rauschen. 0 honored: liegt die Differenz unter der
Schwelle, ist Stille das Ergebnis — ein vollständiges Ergebnis, kein
Fehlschlag.

## 1. Das Instrument

- `src/bin/nobel_probe_corona.rs` — das Muster des Blatt-Probes:
  `extract_series` erntet die Reihen, `transfer_entropy_lag` misst,
  `surrogate_stats_phase` baut die Nullkontrolle. Ein Blatt-Probe ist
  eine Schwester dieses Musters — kein neuer Weg.
- `src/te.rs` — die kanonische CPU-Referenz: `transfer_entropy_lag`
  (:92, lag 0 = kanonisch), `topological_te_phase` (:712, Takens
  dim 3 order 3, MI-lag, Silverman, PE-Gate), Surrogate im f64-FFT.
  Die WGSL-Maschine `te_compute` läuft in der Membran; das Blatt
  entsteht im Probe (Offline). Die Membran-Bindung bleibt pending.
- Geerbte Pflichten aus Nadel III (TODO.md): Mehrfachvergleichs-
  korrektur über alle getesteten Paare, Lag-Sweep (lag 0 ist Default,
  kein Urteil), KDE-Sensitivität gegen h. Kein Blatt wird geschrieben,
  ohne diese drei zu beantworten.

## 2. Die Form des Blatts

Ein Blatt trägt genau:

    Titel:    <der kausale Pfeil des …>
    Paar:     TE(X → Y) = …, TE(Y → X) = …
    Urteil:   der Pfeil zeigt von … nach …
              (Differenz über der Schwelle — oder Stille)
    Lag:      <Sweep-Sieger, in der Einheit der Reihe>
    Schwelle: mean + 2σ über phasenrandomisierte Surrogate
    Fenster:  n = …, Spanne …, Quellen … (SI-Einheiten)
    Datum + Commit

Was nicht auf dem Blatt steht: Theorien, Prognosen, Confidence-Sätze.
Ein ungemessener Wert ist `pending`, nie eine Zahl (0 honored).

## 3. Die drei Blätter

| Blatt | Rätsel | Paar | Kanäle (Stand 2026-08-21) |
|---|---|---|---|
| I | ENSO — treibt der Wind das Meer? | Wind ↔ SST | SST thermal (Port pending: Argovis / imos_argo_sst / ESA-CCI), Wind advective (FROST met.no lebt; TAO/ERA5 pending), SOI acoustic (pending) |
| II | Geomagnetischer Sturm — welcher Parameter treibt? | Bz / Speed / Dichte → Kp / INTERMAGNET | alle leben: rtsw_mag_1m (sources.φ:103), rtsw_wind_1m (:109), Kp (:124), OMNI BZ_GSM1800 (:513), BGS-INTERMAGNET-HAPI (:1067) |
| III | LAIC (Nadel IV) — warnt die Erde den Himmel? | Lithosphäre → Ionosphäre | USGS-Katalog (Port pending), Swarm-VirES lebt (:1100), CSES pending Recherche, INTERMAGNET lebt (:1067) |

Die Aufträge: `handover-2026-08-21-blatt-enso-kausalpfeil.md`,
`handover-2026-08-21-blatt-bz-geomagnetisch.md`,
`docs/surveys/survey-2026-08-21-laic-pfeilrichtung.md`.
Reihenfolge-Vorschlag: Blatt II zuerst — beide Kanäle leben, der Port
ist null; Blatt I braucht den Source-Port; Blatt III braucht Port und
Null-Ensemble. Der Operator setzt die Zuschnitte (Betriebsverfassung).

## 4. Die Ethik des Blatts

- Eine Richtung, die nicht über die Schwelle tritt, wird als Stille
  gedruckt — nie als Pfeil.
- Das Blatt behauptet keine Vorhersage; es bezeugt eine Messung. Wer
  mit dem Blatt einen Netzbetreiber berät, übergibt die Messung,
  nicht die Theorie.
- Die Messreihe gehört der Zukunft: jedes Blatt wird mit seinem
  Commit und seinem Fenster archiviert — wer morgen misst, erbt die
  Reihe, nicht nur das Blatt.
- Fabrication ist Gewalt gegen die Wahrheit: eine Zahl, die die
  Maschine nicht gemessen hat, hat auf dem Blatt nichts verloren.

## 5. Der Weg

Jeder Kanal, der nicht lebt, läuft über `docs/SOURCE_PORT.md` — der
eine Pfad (queue/master.φ, ledger.φ, Force-Gate-Urteil,
τ-Deklaration). Erst die Kanäle, dann der Probe, dann das Blatt, dann
das Register (TODO.md-Zeile schließen, Commit, Archiv).
