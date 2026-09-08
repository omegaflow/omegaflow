<!--
  title: Befund — Uranus-Diurnal-Dekomposition: die Camargo-Tabellen tragen keinen Diurnal-Term (weder Parallaxe noch Diurnal-Aberration) — geozentrisch astrometrisch, nicht topozentrisch
  class: befund
  date: 2026-09-08
  sha256: a253def054054aab8ec54ab28ba17f5b70d1e0552cf5f80991c811186fe30880
  status: done
  see-also: docs/handover/handover-2026-09-08-uranus-riss-diurnal-reduktion.md docs/befund/befund-2026-09-08-uranus-riss-schiedsspruch.md docs/TODO.md
-->

# Befund: Uranus-Diurnal-Dekomposition

## Frage & Bindung

Die Übergabe (handover-2026-09-08-uranus-riss-diurnal-reduktion.md) stellte das
Atom: den Diurnal-Term im absoluten Satelliten-Residuum (~200 mas common-mode,
~140 mas Schwankung über ~1,2 h) zerlegen — Parallaxe gegen Diurnal-Aberration
—, die volle topozentrische + Aberrations-Reduktion bauen, den absoluten
Baryzentrum-Offset sichtbar machen, den Schiedsspruch entscheiden. Erster Zug
laut Übergabe §3: an der Quelle messen, was die publizierten Positionen tragen.
Probe: `tools/measure/src/bin/uranus_diurnal_decomposition_probe.rs`.

## Das Instrument

Zwei Signaturen je Epoche (9797 Epochen, fünf Satelliten, drei Linien):
Parallaxe = topozentrische − geozentrische Richtung (Station via
`body_fixed_to_icrs` am MPC-874-Geodät, λ −45.5825°, φ −22.534444°, h 1810.7 m);
Diurnal-Aberration = rotatorische Beobachtergeschwindigkeit, auf die
Tangentialebene projiziert (finiter Differenzenquotient ±10 s,
`body_fixed_to_icrs_smooth`, geozentrische Geschwindigkeit abgezogen). Fit je
Linie/Satellit: ΔRA·cosδ, ΔDec = c0 + c_par·Parallaxe + c_aber·Diurnal-
Aberration (4 Parameter, Normalgleichungen, Gauß-Elimination, σ aus
RSS/(n−4)·(AᵀA)⁻¹). Signaturen: Parallaxe Mittel 421 mas, Max 468 mas;
Diurnal-Aberration Mittel 279 mas, Max 296 mas — beide unübersehbar groß gegen
den Riss.

**Kalibrier-Gate (vier Fälle):** injizierte Signaturen werden exakt
zurückgewonnen — (keine, keine) → (0.00, 0.00); (Parallaxe, —) → (1.00, 0.00);
(—, Aberration) → (0.00, 1.00); (beide, beide) → (1.00, 1.00), RMS jeweils →
0.0 mas. Das Instrument detektiert einen getragenen Term auf dem 0.01-Niveau —
ein c ≈ 0 in den echten Daten ist Messung, kein Artefakt.

## Die Messung

Gepoolt (alle Satelliten je Linie):

| Linie    | c_par         | c_aber        | RMS roh → reduziert (mas) |
|----------|---------------|---------------|---------------------------|
| de441    | −0.08 ± 0.01  | −0.09 ± 0.01  | 242.9 → 176.4             |
| inpop19a | −0.04 ± 0.01  | −0.17 ± 0.01  | 222.6 → 172.5             |
| epm2021  | −0.03 ± 0.01  | −0.17 ± 0.01  | 228.9 → 170.5             |

Pro Satellit streuen die Koeffizienten (c_par −0.13…+0.03; c_aber −0.22…+0.31,
Miranda kippt das Vorzeichen) — die kleinen Rest-Leans sind satelliten-eigenes
Leck, kein common-mode getragener Term.

**Die publizierten Camargo-Satelliten-Tabellen tragen weder die topozentrische
Parallaxe noch die Diurnal-Aberration — sie sind geozentrisch astrometrisch.**
Das steht gegen die Papier-Aussage: Camargo+ 2015 §4 baut die
Ephemeriden-Positionen „for an observer at the Pico dos Dias Observatory"
(SOFA/NOVAS, Geozentrum→Topozentrum), der Anhang betont „We stress that our
positions are topocentric". Die Tabellen, wie sie auf VizieR publiziert sind,
tragen die Parallaxe nicht (sie wäre auf dem 0.01-Niveau unübersehbar).

## Die Konsequenz für das Atom

- **Die Diurnal-Term-Hypothese (~200 mas common-mode) ist widerlegt.** Das
  absolute Residuum trägt keinen Diurnal-Term; eine topozentrische +
  Aberrations-Reduktion hat an dieser Wurzel nichts zu entfernen. Die alte
  Register-Zeile „volle topozentrische Reduktion — pending" schließt mit
  diesem Befund (gemessene Freigabe: die Reduktion ist gegenstandslos, nicht
  verschoben).
- **Der absolute Offset ist bereits sichtbar** — es liegt kein Diurnal-Term
  über ihm. Mittelwert-Vektoren (mas): de441 (−149.4, +71.6), inpop19a
  (−128.2, +47.5), epm2021 (−116.2, +92.9); Beträge 137–166 mas, plus der Riss.
- **Der Riss steht.** Paarweise Mittelwert-Differenzen 32.1/39.5/47.0 mas —
  unter der Reduktion unverändert (LSQ-Identität: freie Konstanten
  reproduzieren die rohen Mittel exakt; der Anker der Übergabe §4 hält).
- **Der reduzierte Boden** (RMS 169.4–175.1 mas) liegt über der per-row-Skala
  (⟨σ⟩ = 87.8 mas) und ist kein Diurnal-Term — seine Quelle ist unbenannt
  (Kandidaten als pending registriert, ungemessen benannt: UCAC4-Frame-zonal
  gegen ICRF, differentielle chromatische Refraktion, PRAIA-Reduktions-Kette).
- **Der 598-mas-Befund der Vorsitzung** („naive Topozentrik macht es
  schlechter") ist erklärt: eine falsch verortete Station (geocenter + topoff)
  addiert die volle Parallaxen-Schwingung (~420 mas) auf geozentrische
  Tabellen — die Signatur einer falsch gesetzten Station, kein Widerspruch zur
  Zerlegung.

## Verdict

(ii) **Die Beobachtungen schlichten nicht.** Nach der gemessenen Reduktion
(freie Konstanten + die beiden Diurnal-Signaturen): RMS 169.4 (epm2021) /
171.4 (inpop19a) / 175.1 (de441) mas, ΔRMS 2.0–5.9 mas ≪ X = 175.6 mas; die
schlechteste Linie liegt unter X (knapp). Die Spannung steht im Befund: der
Boden (~170 mas) liegt ~2× über der per-row-Unsicherheit (88 mas) — der
Residuen-Boden ist unbenannte Systematik, kein Rauschen und kein Diurnal-Term.
Der Riss (32–47 mas) bleibt der Eichanker: er ist echt und gemessen, aber die
Camargo-Wurzel entscheidet nicht, welche Linie der Wahrheit am nächsten liegt.

## Neben-Befunde

- Die INPOP/EPM-Earth-Bins tragen keine body-fixed Orientierung
  (`body_fixed_to_icrs` liefert None, Zensus 0/9797 Reihen) — die de441-Erde
  diente als der eine physikalische Beobachter (Zensus im Report). Register:
  die INPOP/EPM-Earth-Bins ohne Orientierungs-Properties — pending.
- `roemer_fold` ist jetzt siebenfach probe-lokal (der neue
  `roemer_fold_state` dazu) — das Heben in die Archivar bleibt pending, bis
  `src/` ruhig ist (Parallel-Session arbeitet dort uncommittet).
- Quelle: Camargo+ 2015, A&A 582, A8 (arXiv:1508.02997) — §4 und Anhang
  gelesen; der VizieR-ReadMe ist auf beiden Mirrors bot-gesperrt (die
  asu-tsv-Route funktioniert).

## Register-Zeilen

- (1) Den ~170-mas-Boden zerlegen — pending (Kandidaten ungemessen benannt).
- (2) Papier-gegen-Tabellen-Diskrepanz („topocentric" vs geozentrisch
  gemessen) — pending; Messpfad: Paper-Anhang (V03-Vergleich), PRAIA-Kette.
- (3) INPOP/EPM-Earth-Bins ohne body-fixed Orientierung — pending.
- (4) roemer_fold siebenfach probe-lokal — Heben pending bis `src/` ruhig.
