<!--
  title: Befund — Uranus-Zentrum-Versionenstruktur: die Versionen-Differenz als per-Punkt-Vektor (|Δ| 39.6/44.6/47.9 mas Mittel, säkulares Wachstum bis 74.8 mas in 2011) + die vier de441-Trägerjahre (1999/2001/2004/2009, z = +8.5 gegen die Shuffle-Null)
  class: befund
  date: 2026-09-09
  sha256: 003a98fba456fcd9e7555f7ef522416d5e67b1b309785754174ee4cf06358fb8
  status: done
  see-also: docs/handover/archiv/handover-2026-09-09-uranus-zentrum-kopplung.md docs/befund/befund-2026-09-09-uranus-zentrum-kopplung.md docs/handover/handover-thematisch-mechanische-reste.md
-->

# Befund: Uranus-Zentrum-Versionenstruktur

## Frage & Bindung

Die Übergabe (handover-2026-09-09-uranus-zentrum-kopplung.md) stellte Bau-Linie
(b): die Versionen-Differenz als registrierbare Größe an jedem Punkt der Bahn —
der Schlüssel für den Übergang (ii)→(i). Nach (a) liegt das reduzierte RMS auf
Riß-Skala (69–73 mas gegen den 27–48-mas-Riß), aber die drei reduzierten RMS
sind fast identisch — die Linien bleiben ununterscheidbar. Die Schlichtung läuft
nicht übers Gesamtmittel, sondern über die per-Punkt-Struktur unter dem
Rauschen. Probe: `tools/measure/src/bin/uranus_center_versionenstruktur_probe.rs`.

## Das Instrument

Die Versionen-Differenz wird als **Vektor** gemessen, nicht als Skalar: je
Epoche und je Linienpaar die Differenz der topozentrischen Zentrums-Richtungen,
projiziert auf die Tangentialbasis der Beobachtung (ΔRA·cosδ, ΔDec in mas).
Ein Skalar-Winkel kann eine Vektor-Injektion nicht exakt zurückgeben; der
Vektor kann es. Der Probe trägt denselben Zentrums-Aufbau wie (a): DE441 liest
`ephemeris_uranus_c.bin` direkt, INPOP/EPM ihr Baryzentrum + ura111 799−7.

## Die Messung

Beobachtungsrauschen ⟨σ⟩ = 82.5 mas (3516 Reihen). Per-Linien-Fit reproduziert
(a) byte-identisch: c_par 0.97/0.96/0.95, c_aber −0.11/−0.08/−0.07, RMS
73.0/69.5/69.3 mas.

Per-Punkt-Versionen-Differenz (Linie a − Linie b, Tangentialebene):

| Paar             | ΔRA·cosδ (mas) | ΔDec (mas) | \|Δ\| Mittel | \|Δ\| max | JD des max |
|------------------|----------------|------------|--------------|-----------|------------|
| de441 − inpop19a | +24.3          | −22.9      | 39.6         | 47.3      | 2455809.76 |
| de441 − epm2021  | +37.3          | +22.8      | 44.6         | 74.8      | 2455822.61 |
| inpop19a − epm   | +13.0          | +45.7      | 47.9         | 52.6      | 2453241.65 |

Die Differenz ist **kein konstanter Offset** — sie wächst säkular mit der
Epoche (Jahr-Bin-Mittel von 1992 bis 2011): de441−inpop 31.3 → 47.2 mas,
de441−epm 9.9 → 74.7 mas (7,5-fach), inpop−epm 40.3 → 52.2 (Gipfel 2004) →
47.1 mas. Der Riß wächst mit dem Extrapolations-Abstand — die Eisriesen-Kluft
(TODO) als per-Punkt-Größe, das Maximum steht 2011 (JD 2455822, nahe dem Ende
der Reihe).

## Die Träger-Struktur unter dem Rauschen

Pro Jahr-Bin die Linie mit dem kleinsten reduzierten Residuen-Mittel (n ≥ 8):

- de441 trägt am besten in **1999, 2001, 2004, 2009** — vier Bins, gegen eine
  Shuffle-Null von 0.2 ± 0.4 (z = +8.5): zeitlich lokalisierte echte Träger,
  kein Zufall — obwohl de441 global die schlechteste Linie ist (RMS 73.0).
- inpop19a: 5 Bins gegen 4.9 ± 1.6 (z = +0.1) — exakt Zufall.
- epm2021: 9 Bins gegen 12.9 ± 1.6 (z = −2.5) — der globale Beste, aber sein
  Vorteil ist diffus und gleichmäßig, kein zeitlich konzentriertes Tragen.

## Der Bogen zum Schiedsspruch

Das Gesamtmittel schlichtet weiterhin nicht (verdict (ii) bleibt — die drei
RMS liegen unter ⟨σ⟩). Aber die per-Punkt-Struktur ist jetzt gemessen und
benannt: die Versionen-Differenz ist ein zeitlich wachsender Vektor, und de441
trägt in vier lokalisierte Jahre, die epm nicht trägt und inpop nicht. Der
Übergang (ii)→(i) verschiebt seine Frage — nicht mehr „sind die Linien
unterscheidbar", sondern „was ist der physikalische Ursprung der vier
de441-Trägerjahre".

## Kalibrier-Gate

Injiziert (+40.0, −25.0) mas in DE441 über JD 2455800–2455850 (937 Epochen):
zurückgewonnen ΔRA·cosδ **+40.00 mas**, ΔDec **−25.00 mas** — exakt. Das Gate
hält.

## Register-Zeilen

- Der physikalische Ursprung der vier de441-Trägerjahre (1999/2001/2004/2009) —
  `pending`.
- (c) Neptun als zweiter Planet desselben Baus — `pending`.
- Die Wobble-Periode (1,4-d-Mond-Signatur) als physikalische Form — `pending`.
