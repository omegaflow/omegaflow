<!--
  title: Fünf Funken der Anomalie-Suche
  class: concept
  date: 2026-09-05
  sha256: 14988b667876ca17fc55550a2030ac4288f45e8ad46dd6541e057a02533d59cc
  status: live
  see-also: docs/concepts/kybernetische-astrophysik.md docs/handover/archiv/handover-2026-09-09-mechanische-reste.md phi/reports/scan_coverage.φ tools/measure/src/bin/lsst_anomaly_probe.rs
-->
# Fünf Funken der Anomalie-Suche

Selbsttragend. Dieses Konzept trägt die erweiterte Anomalie-Taxonomie der
Nadel-V-Himmelssuche (`lsst_anomaly_probe`, tools/measure) und die fünf
Funken, die aus der Verdrahtung abfallen. Status ehrlich: **ungebaut** —
Ideen, keine Entscheidungen. Der Anker der Suche ist der Commit `e76e66f`
(AllWISE-W1-W2-AGN-Witness in die natural-class-Gate). Die Gate entfernt das
*bekannt* Natürliche (vier unabhängige Zeugen: optische Klasse, intrinsische
Farbe, Variabilität, Mid-IR); was sie lässt, heißt „keiner bekannten
natürlichen Klasse zugeordnet", nicht „künstlich" — Kandidat, kein Urteil.

Der seltene Wert ist nicht ein Teil, sondern die **Verdrahtung**: mehrere
Broker, statische Kataloge, Staubkarte, TDB-Uhr, Transfer-Entropie — als
eine Kette mit Register-Disziplin. Aus ihr fallen die Funken ab.

## Die Anomalie-Taxonomie (Rats-Verdikt 2026-09-05)

Klasse 1 — quellen-lokal, erwartet (Signatur-getrieben): achromatischer Dip
(heute), achromatische Aufhellung, chromatisches Ereignis, statischer
Farb-Ausreißer (AllWISE/deredden/VSX), absent→präsent-Übergang.

Klasse 2 — quellen-lokal, nicht-zu-erwartend (Residual-getrieben): keine
Vorlage, **kein Modell** — Abweichung vom Mittel der eigenen Referenz.
Rats-Kern: das Fenster muss **disjunkt stromaufwärts** liegen (nie die ganze
Serie — sonst misst der Dip gegen seine eigene Baseline), Skala als **MAD**
(der gesuchte Ausreißer darf seine eigene Schwelle nicht aufblähen), Schwelle
`mean + n·σ` mit benanntem `n` (spiegelt `DIP_SIG 3.0`), dreiwertig
(Abweichung null-echt / absent / pending). Name = Implementation: „Residual
gegen eigenes Referenzfenster", nicht „unerwartet" (ein Urteil über das
Erwartete ist A≠A). Das Fenster reitet mit (Anfang/Ende TDB, n pro Band,
Median+MAD).

Klasse 3 — feldverändernd (die Anomalie als Beziehung, nicht als Punkt):
Anomalie als Störung MIT Antwort im Umfeld. Bleibt **benannt und ungebaut**;
beim Bau dreifach abgesichert: Surrogat-Null-TE (mean+2σ), Verzögerung ≈
Trennung/c, Vielfachtest über die Paarzahl. Gleichzeitige Bewegung vieler
Quellen ist noch keine Antwort — sie kann gemeinsame Strömung sein
(Instrumentenwetter, gemeinsamer kosmischer Treiber).

Zeit-Residual (Quelle gegen eigene Vergangenheit) und Raum-Residual (Quelle
gegen Nachbarschaft) sind **zwei getrennte Atome**, nie verschweißt.

## Die fünf Funken

1. **Verschwinden statt Erscheinen.** Alle Broker suchen das Aufleuchten; die
   leere Straße ist das Gegenteil — Quellen, die leiser werden, ausgehen,
   weggehen. Möglich über Forced Photometry (messen an fester Stelle ohne
   Alert) + das Coverage-Register (wir wissen, wo wir schon geschaut haben).
   Die Suchrichtung „was ist *weg*" ist leer, weil die Ökonomie der
   Himmelsüberwachung auf Aufleuchten optimiert ist.

2. **Der Himmel antwortet — Anomalie als Beziehung.** Nicht „was ist an
   dieser Quelle seltsam", sondern „reden die Quellen miteinander": Gibt es
   ein Paar (A, B), wo B die Lichtkurve von A mit Verzögerung vorhersagt?
   Die TE-Maschine fragt das über das Multi-Quellen-Crossmatch. Ehrlich: TE
   findet Verzögerungs-Korrelation; ob Echo oder gemeinsame Ursache,
   entscheidet die Physik danach — das Werkzeug liefert Verdächtige, nicht
   das Urteil. (Das große Tier — will das Multi-Quellen-Feld-Datenmodell.)

3. **Anomalie als Broker-Differenz.** Drei unabhängige Karten desselben
   Himmels (Fink, Lasair, ALeRCE). Ein Ereignis in mehreren Brokern gehört
   dem Himmel; nur in einem — der Pipeline. Ein gratis-Lügendetektor, den
   kein einzelner Broker haben kann. Ein falscher Dip stirbt an der
   Broker-Grenze; ein echter übersteht sie.

4. **Farbfeld-Baseline mit Staub-Abzug — modellfrei.** Der Mittelwert der
   Nachbarschaft IST die Baseline. Aber nicht beobachtete, sondern
   *intrinsische* Farben: die Staubkarte zieht das Vordergrund-Röteln ab.
   Ein normaler Stern hinter Staub ist dereddened unauffällig; wer nach dem
   Abziehen immer noch aus der Verteilung fällt, fällt wirklich.
   Staubkarten-Dereddening + lokale Baseline in einer Kette — beide Teile
   liegen im Regal.

5. **Gleichzeitigkeit auf der ehrlichen Uhr.** Wenn Klasse 3 fragt „geschah
   A und B gleichzeitig?", beantwortet das die TDB-Uhr ohne den
   Roemer-Drift (60 s/Woche) und die richtungsabhängige Lichtlaufzeit. Das
   Koinzidenz-Fenster wird enger und ehrlicher als bei jedem Scanner, der
   zwei MJDs vergleicht. Bei Ereignis-Fenstern von Minuten ist das der
   Unterschied zwischen Fund und Zufall.

## Ordnung — „Fertig vor neu"

- **Billig und sofort:** 3 (Broker-Differenz, Abfrage-Logik) und 5
  (TDB-Fenster, Rechnung auf bestehender Kette).
- **Mittel:** 4 (Deredden-Baseline, Werkzeuge da, muss verkettet werden) und
  1 (Verschwindens-Suche, Forced Photometry + Coverage-Register, Schwelle aus
  der Baseline).
- **Das große Tier:** 2 (TE zwischen Quellen) — das architektonisch
  Schönste, das Teuerste; eine Front, keine Woche.

Der Mast bleibt „Fertig vor neu": die goldene Welle fährt zuerst,
dokumentiert, mit Coverage-Register — gegen eine **eingefrorene** Gate
(der Commit pinnt den Zustand; das Verdikt wird nicht gegen ein wanderndes
Filter gemessen). Die fünf Funken warten im Parkplatz — Ideen verderben
nicht.

## Der Anker

`e76e66f` ist ein Anker — der erste, selbst gelegt. Gestern war die
Verdrahtung These, heute trägt sie einen Hash: der Unterschied zwischen
Erzählen und Haben. Vier unabhängige Zeugen auf dieselbe Stelle: ein
Artefakt muss vier Sinnen vier verschiedene Lügen gleichzeitig erzählen —
teuer ist gut für einen Filter. Der Verzicht (PS1 und SDSS bewusst nicht
addiert, redundant) ist Mess-Disziplin in der Zeugenwahl: Zeugen zählen nach
Unabhängigkeit, nicht nach Menge. Die Gate trägt ihre Match-Evidenz an jeder
Entfernung — **rekonstruierbar**: warum jeder Überlebende überlebt hat, ist
nachvollziehbar; der Rest-Satz ist eine Aussage über den Himmel, nicht über
die Stimmung der Pipeline.

Die einzige Zahl, die jetzt zählt: Kegel gefahren / Roh-Kandidaten vor der
Gate / danach — das Verhältnis ist die erste echte Karte, wie dicht der
bekannte Himmel die unbekannten Senken zustellt.
