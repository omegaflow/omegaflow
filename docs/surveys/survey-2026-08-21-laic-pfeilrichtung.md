<!--
  title: Das Blatt — die Richtung der Lithosphäre-Atmosphäre-Ionosphäre-Kopplung (Nadel IV)
  class: survey
  date: 2026-08-21
  sha256: d08970e75e8c125a08b1f839842cc22ba3b1124733b0f978123c178c498d1edc
  status: live
  see-also: docs/concepts/blatt-papier-resultat.md docs/surveys/survey-2026-08-21-bz-kausalpfeil.md
-->
# Das Blatt — die Richtung der Lithosphäre-Atmosphäre-Ionosphäre-Kopplung (Nadel IV)

Registriert 2026-08-21. Blatt 1 steht — Instrument B (Fensterstapelung),
gebaut als `src/bin/laic_probe.rs` (stummer Print-Binary, std-only, offline).
Der Befund ist die Stille: 0 honored.

## Das Blatt

```
TE(Lithosphäre → Ionosphäre) = −7.97e-2 nats   (Stapel-Mittel der Fenster-Maximalexzesse, n = 176)
TE(Ionosphäre → Lithosphäre) = −1.08e-2 nats   (n = 176)
Kontrolle TE(Solar Bz → Ionosphäre) = +4.35e-2 nats   (n = 171)
Lag                          = 0 h   (größter mittlerer Exzess beider Richtungen; alle Lag-Mittel negativ)
n (Ereignisse), Schwelle     = 176 Fenster, Null-Ensemble = 40 Zufallsfenster, Schwelle μ + 2σ
Verdikt                      = Stille in beiden Richtungen — ein vollwertiger Befund
```

Schwellen des Null-Ensembles (μ + 2σ über 40 Zufallsfenster):
Litho→Iono −1.60e-2, Iono→Litho +4.90e-2, Kontrolle +1.28e-1. Der
Ereignis-Stapel liegt in beiden Richtungen unter der Schwelle — für
Litho→Iono tief darunter (die 72 h vor M≥6 tragen auf diesem Instrument
weniger Transfer als zufällige Fenster). Die Sonnen-Kontrolle ist still:
der gemeinsame Treiber trägt auf diesem Raster keinen Pfeil.

## Protokoll (Instrument B — Fensterstapelung)

- Ereignisse: USGS-FDSN M ≥ 6.0, die 250 jüngsten der Ära 2014-01-01 …
  Laufzeitpunkt (Katalog: 1726).
- Fenster: 72 h vor t0; Zellen 30 min (n = 144).
- Litho-Serie: Zählrate der Katalogereignisse (M ≥ 2.0) je Zelle im
  2000-km-Radius um das Epizentrum (der FDSN-Katalog ist ein
  Punktprozess — die Zähl-Serie ist die benannte Konstruktion, keine
  erfundene Kontinuität; MiniSEED-Envelopen: Decoder ausstehend).
- Iono-Serie: INTERMAGNET-Gesamtfeld F des nächsten BGS-Observatoriums
  (≤ 3000 km), 1-min-Mittel je Zelle.
- TE: skalarer Pfad `transfer_entropy_lag` (die Probe); Schwelle je Lag =
  μ + 2σ über 10 phasenrandomisierte Surrogate je Serie
  (`surrogate_stats_phase`, Saat deterministisch); Lag-Sweep 0…72 h in
  1-h-Schritten, Lags mit m < 30 Zellen unterbestimmt (effektiv 0…57 h —
  der Rest ist benanntes Nicht-Urteil).
- Fenster-Statistik je Richtung = Maximalexzess über den Sweep; Stapel =
  Mittel über die Ereignis-Fenster; Pfeil ⇔ Stapel > μ + 2σ des
  Null-Ensembles.
- Null-Ensemble: Zufallsfenster (t0 gleichverteilt in der Ära, Zentrum =
  zufälliges Katalog-Epizentrum, gleiche Pipeline, kein Ausschluss — das
  Null-Fenster trägt, was der Katalog trägt). Die
  Mehrfachvergleichskorrektur ist strukturell: die Null-Fenster tragen
  dieselbe Maximalstatistik über denselben Lag-Sweep; zusätzlich steht je
  Lag eine Bonferroni-adjustierte Schwelle (z = 3.32 für 116
  Richtung×Lag-Zellen, z = 3.35 für die 43 Kontroll-Lags) im Lauf-Protokoll.
- Kontrolle: TE(Solar Bz → F) auf 1-h-Zellen, Sweep 0…48 h (m ≥ 30 →
  ≤ 42 h) — der gemeinsame Treiber wird gemessen, nicht angenommen.
- Instrument A — Ereignisrate (eine globale Ratenserie × eine
  Ionosphären-Serie über die ganze Ära): benannt, nicht gebaut → Register.

## Befund

- Stille in beiden Richtungen; kein Lag trägt einen Pfeil (alle
  Lag-Mittel negativ, Maximum bei Lag 0).
- Fenster-Bilanz: 176 qualifiziert; 2 ohne Observatorium ≤ 3000 km; 66
  mit leerer Zählrate (Silbermann auf konstanter Serie → fehlt, kein
  Wert); 6 mit BGS-Datenlücken; BGS-Ernte-Ausfälle (Daten noch nicht im
  Archiv) tragen fehlt.
- Swarm-FAC (SW_OPER_FACATMS_2F, Teilmenge 10 Fenster): 7 Fenster tragen
  FAC-Samples im Radius (514–8576), aber nur 8–25 von 144 Zellen —
  die FAC-Serie je Fenster ist unterbestimmt (m < 30): das Blatt trägt
  INTERMAGNET-F, der FAC-Stapel bleibt offen.
- Die Kontrollrichtung Solar→Ionosphäre ist still — der kausale Pfeil
  der Sache selbst wurde nicht gemessen, aber auch kein Sonnen-Pfeil
  fabriziert. Stille ist die Antwort.

## Was das Blatt nicht trägt (Register)

- Instrument A — Ereignisrate: ungebaut.
- Faden B — TEC-Port (swpc tec_global.json) und CSES: erst nach diesem
  Blatt, offen.
- Volle Ära: 1726 Ereignisse — der Lauf trug die 250 jüngsten (~1 Jahr);
  der volle Lauf ist ein Registerposten mit benannten Laufzeitkosten.
- F ist die Intensität des nächsten Boden-Observatoriums (bis 3000 km) —
  die ionosphärische Signatur am Überflugspunkt ist eine andere Messung
  (FAC/TEC).
- Sensitivitäten: Radius (2000 km), Kadenz (30 min), KDE-Bandbreite
  (h×2), M-Schwelle der Zählserie (2.0) — offen.

## Lauf

```
cargo run --release --bin laic_probe -- --max-events 250 --null 40 --swarm-limit 10
```

Gates: `cargo check` 0 Fehler / 0 Warnungen; kein Test öffnet ein
Fenster oder strahlt; `src/te.rs`, `nobel_probe_corona`, der skalare
Pfad `transfer_entropy_lag`, die Membran-Physik und der IONEX-Parser
unberührt (nur Nutzung, kein Umbau).
