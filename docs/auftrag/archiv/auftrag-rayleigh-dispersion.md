<!--
  title: Auftrag — Rayleigh-Dispersion: die volle Seismik-Front (ein Beben, viele Stationen, viele Farben)
  class: auftrag
  date: 2026-09-08
  sha256: dc89e21d78726727afeb33181427d29034605e955e54ad2c2d370d7f284422df
  status: archived
  see-also: docs/TODO.md docs/auftrag/auftrag-dispersionsrelation.md docs/specs/spectral-oscillator.md
-->

# Auftrag: die Rayleigh-Dispersion — die volle Seismik-Front

## Gegenstand

force 4 (seismic-surface) trägt heute band-flach
`v = PROPAGATION_SPEED[4] = 3000 m/s`. Eine echte Rayleigh-Oberflächenwelle
ist dispersiv: verschiedene Perioden tasten verschiedene Krustentiefen, die
geschichtet sind — der tiefe Ton mittelt über tiefe, dichtere (schnellere)
Schichten, der hohe Ton sieht nur die Oberfläche. Die Kurve v(f) ist eine
Behauptung über das Medium (die Erdkruste) und wird **gemessen**, nicht
dekoriert: kein v0·(f/f0)^β im Daten-Slot (0 honored).

## Öffnungs-Bedingung (benannt, jetzt erfüllt)

Die Steckstelle v(freq) steht (Atom E, `v_freq_shelf`). Ihre Öffnungs-
Bedingung ist benannt: entweder der Schichtungs-Nachweis im eigenen Feld
(Farben-Laufzeit gegen Surrogate) **oder** eine echte Datenquelle, wenn die
Seismik-Front drankommt. Atom E hat den ersten Weg gemessen — der
interplanetare EM-Pfad trägt flach/quell-seitig. Rayleigh ist ein anderes
Medium (die Kruste), kein Abkürzen über die Sonnen-Kalibrierung. Die
Seismik-Front kommt jetzt: die FDSN-Webservices sind live gemessen
(EarthScope dataselect + USGS event, HTTP 200).

## Die Datenquelle (neue Quellen-Familie, Fragen statt Horten)

- Ereignis-Katalog: USGS-FDSN event (live) — bekannter Ursprung, Zeit und
  Ort. Der bekannte Ursprung bricht den Zirkel „Epizentrum ↔
  Dispersionsparameter" (dieselbe Rolle wie die Sonne im Atom E).
- Stationen: FDSN dataselect/station (EarthScope) — Breitband-Stationen des
  globalen Netzes (IU u. a.) auf verschiedenen Epizentral-Distanzen.
- Waveform: vertikal (BHZ), miniSEED, Fenster vom Ursprung bis zum
  Oberflächenwellen-Zug (mehrere zehn Minuten).

## Mess-Design — voll, nicht minimal

Ein Beben, **viele** Stationen, Spektrum je Frequenz, Ankunftszeit-
Differenz in TDB. Die Kurve fällt aus der Messanordnung, sie wird nicht
geschrieben:

1. **Band-Zerlegung**: FFT/Goertzel je Band (das „eigene FFT"-Atom der
   Spektral-Achse, hier gebaut) — je Station je Band die Amplituden-Hülle
   über der Zeit.
2. **Gruppen-Geschwindigkeit**: je Band je Station ist die Ankunft der
   Oberflächenwelle der Hüllen-Peak; `v_g(f) = Δ / t(f)` mit Δ der
   Epizentral-Distanz (Stations-Weltlinie gegen Ereignis-Weltlinie, beide
   gemessen).
3. **Die drei Hebel** (der Zirkel ist gebrochen): (a) die nicht-dispersive
   Referenz — der bekannte Ursprung (USGS) und die P-Welle (läuft tief,
   praktisch dispersionsfrei) tragen die Zeit; (b) die Differenz — zwei
   Stationen kürzen die gemeinsame Unbekannte; (c) die Überbestimmung —
   viele Stationen × viele Bänder, eine Kurve je Region; schafft es keine
   Kurve für alle, ist der **Riss** die Messung (achromatisch = Geometrie,
   chromatisch = Medium — das Nadel-V-Kriterium als Trennmesser).

## Tabelle, nicht Formel

Die gemessene v_g(f) wird als force-4-Zeilen in `v_freq_shelf.dat`
registriert (Kernel-Format, sha256 über den Datenkörper), Lookup über
`band_overlap`. Keine Interpolation zwischen Messzeilen, kein β.

## Fertig vor neu

Atom E schließt zuerst (der EM-Lauf trägt sein Verdict). Dieser Auftrag ist
die **echte Datenquelle**-Öffnung derselben Steckstelle — er erbt sie,
er erfindet sie nicht neu.

## Delegation

- explore: FDSN-Verfügbarkeit messen (welches Ereignis, welche Stationen
  haben den Event aufgezeichnet — dataselect availability).
- Der Bau selbst (Compiler für Mehrstation-Ernte, Band-Zerlegung, Probe)
  ist die Kybernautin dieser Sitzung.

## Gates und Puls

`cargo check --workspace` 0/0; die Kalibrier-Gates der TE-Maschine bleiben
unberührt; die Band-Zerlegung bekommt ihre eigene broken-null-Kontrolle
(synthetisches dispersives Signal muss gefunden werden, der flache Fall
nicht).

## Abschluss

Die Rayleigh-Regalklasse (force 4) trägt ihre erste gemessene Kurve mit
sha256; die TODO-Zeile nennt das gemessene Verdict, nicht mehr pending.
