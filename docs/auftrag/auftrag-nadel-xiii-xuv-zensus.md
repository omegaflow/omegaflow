<!--
  title: Auftrag — Nadel XIII: XUV/C-O-Zeugen einlesen + voller 48er-Zensus
  class: auftrag
  date: 2026-09-05
  status: pending
  sha256: 9943fe212cb0cebe6ff66a0700db1616015f4365a2c74884199418cb66de05a6
  see-also: docs/paper/jwst-disequilibrium-survey.md docs/auftrag/auftrag-extern-stellar-aktivitaet-xuv-co.md
-->

# Auftrag: die Bio-Zeugen (XUV/C-O) aus der externen Rückmeldung einlesen + der volle 48er-Zensus

## Zweck

Nadel ⅩⅢ (Atmosphären-Biosignatur/Disequilibrium) ist der JWST-begrenzte
Zensus. Die externe Recherche (auftrag-extern-stellar-aktivitaet-xuv-co.md)
hat für die 30 Wirte Stellar-Aktivität/XUV und C/O-Rückmeldung geliefert —
**Datenquelle: vom Operator bereitgestellt, liegt der Session beim Start vor**
(Pfad/Format beim Session-Start übergeben; falls nicht, sofort als fehlend
benennen, nicht annehmen).

## Umfang

1. **Die Rückmeldung einlesen:** die vom Operator bereitgestellten XUV/log-R'HK/
   C-O-Werte je Host in `docs/reference/co_rhk_witness_seed.json` und den
   `disequilibrium_register_probe` (`--witness`) wirken.
2. **Der zweite Reinigungsschritt (Photochemie-Re-Erklärung):** die
   SO2/CO2-Disequilibrium-Hits (WASP-39 b, WASP-107 b, …) gegen die jetzt
   vorhandenen XUV/Aktivitäts-Zeugen regressieren — bewegt sich ein Hit, wenn
   die Stellar-Aktivität abgezogen ist? (Bisher: nur WASP-166 b CO2 bei
   [Fe/H]=+0.19, knife-edge.)
3. **Der volle 48er-Zensus:** die 18 JWST-Transmissions-Ziele OHNE publizierte
   Detektion (GJ 1132, TRAPPIST-1, LHS 1140, … = flach/neblig/Obergrenze) als
   explizite Non-Detection-Zeilen (0 honored) in den Zensus aufnehmen — nicht
   fallen lassen. Damit steht der Katalog über alle 48 beobachteten Ziele.

## Kernregel (0 honored)

Wo die Rückmeldung einen Wert NICHT trägt, bleibt er `pending` mit der
benannten Quelle — nie schätzen/extrapolieren. Ein Hit ist erst dann
re-erklärt, wenn der Zeuge gemessen ist; ohne Zeuge bleibt die
Photochemie-Frage offen.

## Lieferung

Zeugen eingelesen + Regression + voller 48er-Zensus, committed; das Blatt der
Nadel ⅩⅢ aktualisiert. XUV-Werte, die die Rückmeldung nicht trägt, als
`pending` im Register benannt.
