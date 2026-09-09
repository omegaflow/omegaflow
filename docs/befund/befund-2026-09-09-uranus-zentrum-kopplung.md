<!--
  title: Befund — Uranus-Zentrum-Kopplung: die Astrometrie gegen das Planetenzentrum (uranu_j, c_par 0.95–0.97) + die getragene Wobble (21.653 m Mittel)
  class: befund
  date: 2026-09-09
  sha256: db0cf3d4a6f553a28a6dd0588bddd5649871fdd75cd65583ae4126baea157d2d
  status: done
  see-also: docs/handover/archiv/handover-2026-09-09-uranus-riss-kontur.md docs/befund/befund-2026-09-08-uranus-riss-schiedsspruch.md docs/befund/befund-2026-09-08-uranus-diurnal-dekomposition.md docs/handover/handover-2026-09-09-mechanische-reste.md
-->

# Befund: Uranus-Zentrum-Kopplung

## Frage & Bindung

Die Übergabe (handover-2026-09-09-uranus-riss-kontur.md) stellte Bau-Linie
(a): die Uranus-Astrometrie bekommt ihren richtigen Zielpunkt — die
topozentrische Kopplung gegen das Planetenzentrum statt gegen das
System-Baryzentrum; die Wobble wird Information statt Störung (sie trägt die
Mond-Massen). Probe:
`tools/measure/src/bin/uranus_diurnal_decomposition_probe.rs` (neu: `--center`).

## Der Mechanismus

Die Planeten-Weltlinie wird zentrums-verankert komponiert: DE441 trägt
`ephemeris_uranus_c.bin` (Zentrum 799) direkt; INPOP/EPM tragen ihr
Baryzentrum + die gemeinsame 799−7-Verschiebung (ura111). Die Wobble
(Zentrum − Baryzentrum) ist damit ein getragener, gemessener Vektor, kein
unbenannter Boden. Die Mond-Weltlinie bleibt unverändert (Baryzentrum +
Satellit−7) — die registrierten Mond-Zahlen sind byte-identisch reproduziert
(c_par 0.95/0.95/0.94, c0 −26.9/−19.6 / −13.3/−42.9 / −5.7/+3.8 mas,
Riß 32.1/39.5/47.0 mas roh).

## Die Messung

Die Planeten-Tabelle (uranu_j, 3516 Positionen) gegen das Zentrum:

| Linie    | c_par        | c_aber       | RMS roh → reduziert (mas) | c0 (ΔRA·cosδ, ΔDec, mas) |
|----------|--------------|--------------|---------------------------|--------------------------|
| de441    | 0.97 ± 0.01  | −0.11 ± 0.02 | 249.5 → 73.0              | (−14.2, −24.5)           |
| inpop19a | 0.96 ± 0.01  | −0.08 ± 0.02 | 226.8 → 69.5              | (+2.0, −46.1)            |
| epm2021  | 0.95 ± 0.01  | −0.07 ± 0.02 | 233.8 → 69.3              | (+10.5, +1.1)            |

Die Wobble (Zentrum − Baryzentrum, DE441): Mittel 21.653 m, max 42.627 m
über 3516 Epochen.

Der Riß am Zentrum (paarweise c0-Differenzen): 27.0 / 35.5 / 47.9 mas —
Skalen-konsistent mit dem Mond-Riß nach der Reduktion (27.0 / 31.5 / 47.3).

## Der Bogen zum Schiedsspruch

Die rohe uranu_j-RMS (249.5/226.8/233.8) trägt dieselben Zahlen wie das
Schiedsspruch-Blatt (249.5/226.8/233.9) — der ~250-mas-Term war die
topozentrische Parallaxe (c_par ≈ 0.95–0.97), kein „DE432-Träger"; der
Konfund „geozentrisch" löst sich auf: die Planeten-Tabelle ist topozentrisch
astrometrisch wie die Monde. Die Planeten-Tabelle bleibt abgeleitet
(„Positions of Uranus … are not observed ones") — ihr c0 trägt die
DE432-Wurzel; in den paarweisen Differenzen kürzt sie sich.

## Verdict

(a) steht gebaut und gemessen: die Uranus-Astrometrie koppelt gegen das
Zentrum (ihr richtiger Zielpunkt), die Wobble ist ein getragener, gemessener
Vektor (21.653 m Mittel — die Mond-Massen), und der Riß am Zentrum
(27.0/35.5/47.9 mas) ist Skalen-konsistent mit dem Mond-Riß. Das
Kalibrier-Gate hält (injizierte Parallaxe + Aberration exakt zurückgewonnen:
c_par 1.00, c_aber 1.00, RMS → 0.0 mas).

## Register-Zeilen

- (b) Versionen-Differenz als registrierbare Größe je Bahn-Punkt — `pending`.
- (c) Neptun als zweiter Planet desselben Baus (`--neptune-c-spk`) — `pending`.
- Die Wobble-Periode (1,4-d-Mond-Signatur) als physikalische Form der
  Wobble separat zu prüfen — `pending`.
