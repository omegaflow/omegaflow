<!--
  title: Befund — Seismische Ortung mit Tiefe (ak135, 3D-Gitter): das Epizentrum steht bei ~14,6 km, die Tiefe ist flach-bestimmt — 0–50 km ununterscheidbar, >100 km verworfen
  class: befund
  date: 2026-09-09
  sha256: 2fbc18809e6fdf12947def72153fbae15188701307a4d08dcf32d372679b8365
  status: done
  see-also: docs/handover/handover-2026-09-09-tiefenphasen-flotte.md docs/befund/befund-2026-09-09-seismische-ortung-ak135.md
-->

# Befund: die seismische Ortung mit Tiefe

Prüffall: USGS `us6000tkt2` (mww 7.8, reviewed). Feld-Herkunft: die
flach-bestimmt-Grenze ist die Feldlage — scharfe Tiefe löst das Feld über
Tiefenphasen (pP/sP) oder Nah-Stationen, nie über ein globales Netz allein.

## Frage & Bindung

TODO-Registerzeile „Beben-Tiefe — pending": T_P(Δ, h) über die ak135-Tiefen-
kurven ersetzt den Oberflächen-Fokus; die Inversion läuft über ein
3D-Gitter (lat/lon/Tiefe).

## Der verallgemeinerte Tracer

`src/archivar/ak135.rs` trägt jetzt die Quelltiefe: `ray(m, p, source_radius)`
mit zwei Schenkeln über die Reziprozität — aufwärts (Quelle→Oberfläche direkt,
kleines Δ) und abwärts (Quelle→Umkehrpunkt→Oberfläche, großes Δ). Verifiziert
gegen die TauP-Tiefentabelle (`ak135_P_shallow.txt`) für Tiefen 15/35/50/100/
150/200/250 km bei Δ 30/60/90° auf < 0,5 s. Die Tiefe verschiebt die Ankunft:
T(60°,100 km) < T(60°,0 km) — gemessen.

## Der Lauf

18 Picks (II.TLY absent). 3D-Gitter über Tiefen 0/10/15/20/35/50/100/150/
200/250 km.

- **Geortet: lat = −8,2240, lon = 121,3800, Tiefe = 20 km, rms = 1,887 s.**
- Katalog: −8,3514, 121,3478, **Tiefe 10 km**.
- **Epizentrum-Offset ≈ 14,6 km** (≈ 14,2 km Nord, 3,5 km Ost).

## Die Tiefe — das Residuum führt sie vor

Das Residuum gegen die Tiefe ist **flach zwischen 0 und 50 km** und steigt
dann scharf:

| Tiefe [km] | rms [s] |
|---|---|
| 0 | 1,947 |
| 10 | 1,910 |
| 15 | 1,935 |
| 20 | 1,887 |
| 35 | 1,919 |
| 50 | 1,908 |
| 100 | 2,441 |
| 150 | 3,454 |
| 200 | 4,865 |
| 250 | 6,496 |

Das Netz ist global (nur II.KAPI unter 5°), also ist die Flachtiefe schwach
bestimmt: die Maschine kann 0–50 km nicht trennen, **verwirft aber > 100 km**
klar. Der Katalog (10 km) liegt im Band — die Positivkontrolle ist „konsistent,
nicht scharf". Der Fund ist die Schärfe der Obergrenze, nicht die Punkt-Tiefe.

## Benannt

- Die Flachtiefe braucht Nah-Stationen oder Tiefenphasen (pP/sP) — beides fehlt
  diesem Netz; pending, keine Erfindung.
- II.KIV +5,88 s (87,6°) bleibt der Ausreißer des ak135-Laufs; die Tiefe ändert
  ihn nicht (Struktur oder Pick — offen).

## Verdikt

Positivkontrolle bestanden: das Epizentrum steht bei ~14,6 km; die Tiefe ist
ehrlich als flach-bestimmt gemessen (0–50 km, > 100 km verworfen), nicht als
Punkt erfunden. Der rms sank 1,947 → 1,887 s durch die Tiefe.

## Folge (Register)

- „Beben-Tiefe" schließt als „gebaut, flach-bestimmt"; Tiefenphasen (pP/sP) als
  eigener pending-Kandidat für scharfe Tiefe.
- GEBCO, Tōhoku/Sumatra, Stromboli bleiben pending.
