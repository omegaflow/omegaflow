# Rat-Verdikt — Netz-Fusion und Stationsplatzierung (2026-10-15)

Grundlage: die Runde des Operators an den 13 Frontier-Sitzen (Rohmaterial
`state/stimmen/2026-10-15-netzfusion-manifest.md`; 12 Antworten, Mistral = Sitzungskontingent).
Die Session verifiziert jede tragende Behauptung am Baum, nie die Stimme; das Verdikt trägt der Rat.

## Am Baum verifiziert (die tragende Behauptung)

`tools/measure/src/bin/al_network_proxy.rs:101-119`: die „Min-Einhüllende" ist `sorted[0]` = das
**Minimum** der anwesenden Stationen (mindestens `min_stations`). `gic_onset_detector` meldet einen
Onset, wenn die Hülle eine Schwelle überschreitet. Damit ist eine Detektion äquivalent zu:
**jede anwesende Station ≥ Schwelle** — eine **Konjunktion (UND)**, kein Konsens. Jede zusätzliche
Station ist ein zusätzliches Veto. Der gemessene monotone Rückrufabfall (9,8 % → 0,35 % bei fester
Schwelle und festem Fenster) ist damit teils **strukturell erzwungen**, teils empirisch.

## Das Verdikt (Einstimmigkeit, 12/12)

1. **Die Min-Einhüllende ist als Fusionsregel geschlagen.** Nicht durch Messfehler, sondern durch
   ihre eigene Struktur (UND) **und** durch die Messung (der Abfall überlebt bei fixer Schwelle und
   fixem Fenster). Sie gibt der verrauschtesten Station das letzte Wort — das verletzt Axiom 3
   (Gewicht aus Messung) und Axiom 4 (die Linse spricht, nicht die Sache).
   Mechanismus (GLM, quantitativ): der Erwartungswert des Minimums von N Rausch-Stationen sinkt
   ~ −σ·√(2·ln N), während die Onset-Amplitude A eine Eigenschaft der Physik ist, nicht des Netzes —
   für wachsendes N wird A > σ√(2·ln(N−1)) zunehmend unerfüllbar.

2. **Das Fenster MUSS fixiert werden — physikalisch, nicht von der Stationszahl.** Das wandernde
   Vergleichsfenster ist ein gemessener Konfund; ohne Fixierung ist der Vergleich zwischen N=3 und
   N=12 bedeutungslos. Fixierung von der Physik (Ausbreitungs-Apertur), gleiches Fenster für alle N.

3. **Die Ausbeute/Falschalarmrate wird fixiert, nicht die Schwelle.** Die adaptive Schwelle koppelt
   θ an N; das macht den ~1800×-Kollaps untrennbar in (θ, N). Für einen Schärfe-Vergleich wird die
   Schwelle je Konfiguration auf **gleiche Ausbeute** kalibriert — auf Trainings-/Kalibrierdaten,
   nicht auf dem Test.

4. **Die Einzelstation ist der Maßstab, den ein Netz erst schlagen muss.** Eine neue Station darf nur
   bleiben, wenn sie auf zurückgehaltenen Daten einen gemessenen **marginalen** Zusatznutzen bringt
   (Axiom 3). `pending` für Stationen ohne berechenbaren Beitrag (Axiom 2) — nie `0.0`.

5. **Die Richtung des Ersatzes:** TE-/Korrelations-geführte **Auswahl kleiner Subsets**, dann ODER/
   Max-Fusion (nicht UND) — konsistent mit unserer TE-null-Methode. Alternativen, von mehreren
   Sitzen benannt: **Ordnungsstatistik k von N** (k=N = altes Min, k=1 = ODER; ein Regler statt
   Doktrinen), **Breitenband-Auswahl** als physikalischer Prior, **Band-Max-Einhüllende mit
   Referenzen** (auroral = Trigger, subauroral/polar = Ausschluss).

## Die offenen Risse (ungeglättet)

- **Der Min-Riss vs. Fenster/Schwelle-Riss:** die feste-Schwelle/konstantes-Fenster-Messung zeigt,
  dass der Abfall bei konstantem θ und Fenster **bleibt** — das Min ist also mitschuldig. Aber der
  Vergleich ist noch ein **Betriebspunkt**-Vergleich (feste Schwelle ≠ gleiche Ausbeute), kein
  reiner Schärfe-Vergleich. Die genaue Aufteilung Min vs. Fenster/θ ist **offen** → nächster Schritt.
- **TE als Fusions-/Platzierungsregel ist Hypothese, nicht Beweis.** Unsere Form (TE + Surrogat-Null
  als Regel) existiert in der Literatur nicht — nur je Einzelpfeil (verifiziert in der weiten Suche).
  Der Beleg für „TE-Auswahl hebt den Rückruf" steht aus.
- **Die Einzelstation als Benchmark kann ein Siegerfluch sein**, wenn sie auf denselben Daten gekürt
  wird, auf denen verglichen wird (Claude). Der Vergleich braucht einen Kalibrierungs-/Test-Schnitt.
- **Platzierung folgt nicht aus der Messung, sondern ist erst zu testen** (Duck, Claude, Fugu):
  die Physik (dB/dt dehnt sich N/W/O aus, ~5 min) gibt einen **Prior** — sie ist eine zu testende
  Hypothese, kein einzubauender Default (A = A). Die Süd-Station ist **Kontrolle**, nicht Veto.
- **Der Faktor ~1800× (adaptiv) ist ohne die adaptive Regel nicht deutbar** — rückruf-Einbruch bei
  steigender Präzision wäre ein Betriebspunktwechsel, kein Schaden. `pending`.

## Die Stimmen (aus den Sitzen konsolidiert)

- **Mountain** — jede Zelle trägt Ereigniszähler, N, Fenster, θ; kein Verdikt ohne Messung; nicht
  vergleichbare Zellen `pending`, nie 0.
- **River** — eine Station darf sich **enthalten** statt zu blockieren; die Maschine darf `pending`
  sagen. Das UND nimmt der Station das Recht zu verweigern, ohne zu veto-en.
- **Mycelium** — Fusion ist ein gerichteter **Fluss**, kein Broadcast-UND; das TE-Netz trägt die
  Richtung, die das Min wegwirft. Fenster/Splits/Seeds als registrierter CI-Lauf.
- **Sensory** — Korrelat ≠ Erleben; eine Station ist ein **Ort**, keine austauschbare Zahl. Der
  Einzelstations-Vorsprung kann sensorspezifisch sein (Rauschboden, Basislinie) — prüfen, bevor
  verallgemeinert.
- **Future** — Onset-Liste, Null und Fenster **jetzt einfrieren und versionieren**; die Skalar-Wahl
  von heute bestimmt, was morgen geerntet wird. Ehrliche Registrierung heißt `pending`, nicht 0.

## Der nächste begrenzte Messschritt (konvergent über die Sitze)

**Daten, Onset-Liste und Surrogat-Null einfrieren; Fenster (a-priori physikalische Apertur) und
Falschalarmrate fixieren; dieselbe Messung erneut fahren** und berichten:
1. Kontrolle: Min-Einhüllende (Status quo) mit fixem Fenster und ausbeute-gleicher Schwelle, R(N)
   für die Leiter (1, 3, 4, 8, 12) — **kehrt die Monotonie zurück, war der Kollaps Fenster+Schwelle,
   nicht das Min; bleibt sie, ist das Min selbst die Ursache.**
2. Dann die Familie k=1…N (Ordnungsstatistik) über dieselbe Leiter, mit Kreisschub-Surrogaten auf
   der fusionierten Detektionsreihe.
3. Erst danach: Min ersetzen durch ODER/Max über TE-gewählte Subsets, bei gleichem eingefrorenem
   Fenster, gleicher FAR, gleichem Vorlauf.
Leitfrage jeder Zelle: **Schlägt irgendein (N, k) die Einzelstation bei gleicher Ausbeute auf den
Testdaten?** × Kalibrierung getrennt vom Test (kein Siegerfluch).

## Axiom-Probe

A = A: das Min ist nicht per Erwartung, sondern per Messung und per Struktur geschlagen.
0 geehrt: `pending` für unvergleichbare Zellen und fehlende TE — kein 0.0. Alle Wesen gleich: das
Gewicht kommt aus der Messung, nicht aus Rang; das Min verletzt das. Die Linse: die Wahl der
Fusionsregel ist ein ethischer Akt — das Min ließ den Gradienten (die verrauschteste Station)
sprechen. Konsens: die neue Regel wird vom Operator bestätigt, bevor sie Doktrin wird.
