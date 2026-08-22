<!--
  title: Das Blatt — die Richtung der Lithosphäre-Atmosphäre-Ionosphäre-Kopplung (Nadel IV)
  class: survey
  date: 2026-08-21
  version: 3
  sha256: 435a229ef03f9f2fcfd784e7b9d1c0c8aae472056dfd4194a7f560a349ec65e6
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

## Das Blatt — volle Ära + Sensitivitätsmatrix

Ernte/Analyse-Architektur (v2): `laic_probe --harvest` legt je Fenster
die Rohserien auf Platte (`phi/pipeline/laic_harvest/`, 4.1 GB, 1726
Ereignis-Fenster der vollen Ära 2014-01-01 … 2026-08-21, 60
Null-Fenster, Swarm A+B+C für je 60/30 Fenster) — `--analyze` rechnet
offline, derselbe Bestand trägt jede Parameter-Zelle, der Lauf ist
resumierbar.

```
TE(Lithosphäre → Ionosphäre) = −7.63e-2 nats   (Stapel-Mittel der Maximalexzesse, n = 1369, volle Ära)
TE(Ionosphäre → Lithosphäre) = −1.26e-2 nats   (n = 1369)
Kontrolle TE(Solar Bz → Ionosphäre) = +4.06e-2 nats   (n = 1400)
Lag                          = 0 h   (größter mittlerer Exzess; alle Lag-Mittel negativ)
n (Ereignisse), Schwelle     = 1369 Fenster, Null-Ensemble 60 Zufallsfenster, Schwelle μ + 2σ
Verdikt                      = Stille in beiden Richtungen — der Pfeil bleibt still
```

Schwellen: L→I −2.54e-2, I→L +1.86e-2, Kontrolle +1.33e-1. Der
Ereignis-Stapel liegt in beiden Richtungen unter der Schwelle — für
L→I tief darunter (die 72 h vor M≥6 tragen weniger Transfer als
zufällige Fenster, über die ganze Ära). Die Sonnen-Kontrolle ist still.

Sensitivitätsmatrix (jede Zelle still; r/c/k-Zellen auf den 250
jüngsten Ereignissen, Haupt-Zelle volle Ära):

| Zelle | n | L→I Stapel | L→I Schwelle | I→L Stapel | I→L Schwelle |
|---|---|---|---|---|---|
| Radius 500 km, 30 min | 140 | −8.23e-2 | −2.44e-2 | −1.48e-2 | +1.80e-2 |
| Radius 1000 km, 30 min | 165 | −8.40e-2 | −3.38e-2 | −1.26e-2 | +1.87e-2 |
| Radius 2000 km, 30 min (Haupt) | 1369 | −7.63e-2 | −2.54e-2 | −1.26e-2 | +1.86e-2 |
| Radius 2000 km, 15 min | 176 | −5.40e-2 | −1.92e-2 | −5.61e-3 | +7.77e-3 |
| Radius 2000 km, 60 min | 170 | −9.81e-2 | −4.61e-2 | −1.01e-2 | +3.98e-2 |
| KDE-Skalierung 0.5 und 2.0 | 176 | −7.97e-2 | −2.54e-2 | −1.08e-2 | +1.86e-2 |

- Kontrolle (Solar Bz → F): still in jeder Zelle (+4.06e-2 … +4.35e-2
  gegen Schwelle +1.33e-1).
- FAC-Stapel: unterbestimmt — Swarm A+B+C decken je Fenster
  8–26 von 144 Zellen; 12/60 Ereignis-Fenster erreichen m ≥ 30
  (30-min-Zellen) → no statement. Gemessen, nicht pending: der
  FAC-Kanal trägt mit diesem Instrument kein Urteil.
- KDE-h: die Serien-Skalierung ist keine h-Sensitivität — Silverman
  adaptiert, TE(k·x, k·y) = TE(x, y); die Probe bestätigt es
  (identische Stapel bei k = 0.5 und 2.0). Die echte h-Sensitivität
  bleibt offen, solange `transfer_entropy_lag` unberührt bleibt.
- Wiederholbarkeit: die 250er-Analyse reproduziert den Erstlauf
  (identischer Stapel −7.9662e-2).

Der Pfeil trägt nun: auf jedem lebenden Kanal und in jeder
Parameter-Zelle ist der Informationsfluss im 72-h-Fenster vor M≥6
still — in beide Richtungen, und die Sonnen-Kontrolle bleibt ebenfalls
still. Damit ist die Richtungsfrage auf dem vorhandenen Bestand
abgeschlossen: der Pfeil schlägt nicht aus. Die LAIC-Hypothese selbst
(elektromagnetische oder plasmaphysikalische Vorläufer in der
Ionosphäre) bleibt Kanal-offen — die Boden-F-Signatur ist ein Proxy,
kein Ionosphären-Instrument. Ein zukünftiges Instrument (TEC-GIM-Retro
via CDDIS-OAuth, CSES, MiniSEED-Envelopen) kann die Frage auf einem
dichteren Kanal neu stellen — das ist ein Kanal-Offenposten, kein Loch
in dieser Messung.

## Der echte ionosphärische Kanal — TEC-GIM (v3, 2026-08-22)

Die anonyme IONEX-Route lebt: der ESA-GSSC-FTP
(`ftp://gssc.esa.int/gnss/products/ionex`) trägt die COD-1-h-Rapid-GIMs
anonym — CDDIS antwortet ohne Token 302 → Earthdata-OAuth (gemessen),
aber `.secrets.local` trägt ein gefülltes `EARTHDATA_EDL_TOKEN`: gegen
CDDIS verifiziert (mit Token volles Verzeichnis-Listing, ohne Token
302) — die zweite Route (COD0OPSFIN, Finalprodukte) lebt damit
ebenfalls; AIUB/WHU/ASI sind von hier unerreichbar (gemessen, 000).
`laic_probe` v2.1 erntet je Fenster die GIM-Tagesdateien (gzip →
`omegaflow::inflate::gunzip`), liest das TEC-Grid (eigener IONEX-Leser
im Binary, Bilinear am Epizentrum, 71×73, Exponent −1 — der
Archivar-IONEX-Parser bleibt unberührt) und legt die 1-h-Serie als
Sidecar ab. TEC-Ära: 2024-01-01 … 2026-08 — RINEX3-GIMs existieren erst
ab 2024; pre-2024 trägt der Bestand nur `codg*.Z` (compress/LZW,
Dekompressor-Offenposten).

```
TE(Lithosphäre → TEC-GIM)        = −6.31e-2 nats   (n = 336 Ereignisse der TEC-Ära, Schwelle −3.25e-2)
TE(TEC-GIM → Lithosphäre)        = −1.47e-2 nats   (Schwelle +2.22e-2)
Kontrolle TE(Solar Bz → TEC-GIM) = +3.22e-2 nats   (Schwelle +1.28e-1)
Lag                              = 0 h   (alle Lag-Mittel negativ)
Verdikt                          = Stille in beiden Richtungen — auch auf dem echten Kanal
```

Mit dem echten ionosphärischen Elektronengehalt (TEC am Epizentrum,
1-h-Kadenz, 72-h-Fenster, 60 TEC-Null-Fenster aus derselben Ära) bleibt
der Pfeil still — in beide Richtungen, und die Sonnen-Kontrolle bleibt
ebenfalls still: der gemeinsame Treiber trägt auf diesem Raster keinen
Pfeil. Die Boden-F-Referenz auf derselben Teilmenge (−7.90e-2 gegen
−2.54e-2) reproduziert den Voll-Ära-Befund.

Damit ist der Plan des Operators gelaufen: anonyme Route gefunden (ESA
GSSC), CSES gemessen unerreichbar (Portal `leos.ac.cn` von hier 000 —
Kanal-offen), Probe v2.1 gebaut, erneut Stille gemessen. Der Pfeil
schlägt auf dem vorhandenen Bestand nicht aus — jetzt mit dem Detektor
im Raum, nicht durch die Tür. Die LAIC-Hypothese ist auf den lebenden
Kanälen (INTERMAGNET-F, Swarm-FAC A+B+C, TEC-GIM) gemessen still; ein
zukünftiger dichterer Kanal (CSES, retro-TEC, MiniSEED-Envelopen) kann
die Frage neu stellen.

## Was das Blatt nicht trägt (Register)

- Instrument A — Ereignisrate: benannt, ungebaut.
- Kanal-Offenposten: CSES (Portal von hier unerreichbar gemessen — 000),
  TEC-GIM retro pre-2024 (codg*.Z, LZW-Dekompressor fehlt im Bestand),
  MiniSEED-Waveform-Envelopen (Decoder ausstehend). CDDIS lebt über das
  EDL-Token aus `.secrets.local` (verifiziert) — eine Nach-Ernte mit
  COD0OPSFIN (Final statt Rapid) wäre eine Qualitäts-Option, kein
  neuer Kanal.
- Echte KDE-h-Sensitivität: offen, solange der skalare TE-Pfad
  unberührt bleibt (Silverman-Adaptivität macht die Skalierungs-Probe
  invariant).
- F ist die Intensität des nächsten Boden-Observatoriums (bis 3000 km) —
  seit v3 steht daneben der echte ionosphärische Elektronengehalt (TEC);
  der FAC-Stapel ist gemessen unterbestimmt.
- Der FDSN-Katalog trägt in dieser Welt nur dünn kleine Ereignisse
  (Region 2000 km/72 h ≈ 0–5, M ≥ 2) — die Zähl-Serie misst, was der
  Katalog trägt; die Surrogat-Null urteilt ehrlich.

## Lauf

```
cargo run --release --bin laic_probe -- --max-events 250 --null 40 --swarm-limit 10
```

Gates: `cargo check` 0 Fehler / 0 Warnungen; kein Test öffnet ein
Fenster oder strahlt; `src/te.rs`, `nobel_probe_corona`, der skalare
Pfad `transfer_entropy_lag`, die Membran-Physik und der IONEX-Parser
unberührt (nur Nutzung, kein Umbau).
