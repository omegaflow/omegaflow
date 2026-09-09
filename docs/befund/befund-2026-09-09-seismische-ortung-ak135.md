<!--
  title: Befund — Seismische Ortung mit ak135: das Epizentrum aus eigenen Ankünften — Offset ≈ 18,7 km, rms 1,947 s; der Zwirn-Test trägt
  class: befund
  date: 2026-09-09
  sha256: 99b3a5b03233cd1e6822fd27e5c57b2cb37b0c0739ded9bb0016e819c42e778a
  status: done
  see-also: docs/TODO.md docs/befund/befund-2026-09-09-seismische-ortung.md docs/auftrag/auftrag-rayleigh-dispersion.md
-->

# Befund: die seismische Ortung mit ak135

## Frage & Bindung

TODO-Registerzeile „ak135-Laufzeitkurve — pending": T_P(Δ) ersetzt die Konstante
in derselben Gittersuche (die Konstant-Sehne trug ein globales Netz nicht).
Probe: `tools/measure/src/bin/quake_location_probe.rs` — STA/LTA-Pick, Weltlinien
über `body_fixed_to_icrs` zur gemeinsamen t_ref, Differenz-Inversion, jetzt mit
`p_travel(Δ)` statt `chord/v_p`.

## Die kuratierte Klasse ak135

- Kernel `src/archivar/kernels/ak135.dat` (136 Zeilen, sha256
  `7518894268980b2591d539ff442d630be3d0e5acfbec9003433e455ef65feba4`),
  Provenienz Kennett/Engdahl/Buland 1995 (GJI 122, 108-124), TauP StdModels.
- Strahlen-Tracer `src/archivar/ak135.rs`: kugelsymmetrische τ(p)-Integration,
  lineare Interpolation in Radius — das ak135-Dokument benennt sie selbst als
  Basis der Laufzeitrechnung. Verifikation: T(30/60/90) gegen die TauP-Referenz
  (370,27 / 608,34 / 781,40 s) auf < 0,15 s; Konstante-Geschwindigkeits-Kontrolle
  gegen die Sehne auf < 0,01 s. Die Kurve ist gemessen, nicht dekoriert.

## Der Lauf

18 von 19 Stationen tragen einen P-Pick (II.TLY absent, HTTP 204).

- **Geortet: lat = −8,1840, lon = 121,3680, rms = 1,947 s.**
- Katalog: −8,3514, 121,3478. **Offset ≈ 18,7 km** (≈ 18,6 km Nord, 2,2 km Ost).
- Vorher (Konstante): rms 164 s, Offset 1449 km.

## Der Zwirn-Test

Der rms fiel 164 s → **1,947 s**. Die 18 gemessenen Ankünfte tragen die
ak135-Form; die kuratierte Kurve trägt die Messung — die kuratierte Klasse zieht
ins Haus ein und wird von den eigenen Daten begrüßt. Die Residuen liegen in
±2,8 s, bis auf II.KIV (+5,69 s bei 87,6°) — ein Ausreißer, benannt, nicht
gedeutet. Die distanzabhängige Systematik der Konstant-Nullhypothese ist weg.

## Benannt

- Oberflächen-Fokus (Quelltiefe 0): die Beben-Tiefe bleibt Folge-Atom.
- Die Nah-Stufe (< 15°) läuft nicht — nur II.KAPI liegt unter 15°.
- II.KIV +5,69 s: Pick oder laterale Struktur (Subduktionszone) — offen.

## Verdikt

Positivkontrolle bestanden: die Maschine ortet das Katalog-Beben aus eigenen
Ankünften auf ~19 km, rms auf Pick-Niveau. Kein fabrizierter Ort — der Katalog
war während der Rechnung verdeckt und diente nur dem Verdikt.

## Folge (Register)

- ak135-Zeile schließt als „gebaut"; die Ortung schließt als „Positivkontrolle
  bestanden".
- Tiefe, GEBCO, Tōhoku/Sumatra, Stromboli bleiben pending (eigene Atome).
