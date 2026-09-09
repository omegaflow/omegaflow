<!--
  title: Thematisches Handover — Tiefenphasen-Flotte, Folge-Atom: der Pick-Rausch-Hebel gebaut und gemessen (gewichtete Joint-Inversion −6,0 km; die tiefe Zone systematisch bimodal — Hebel an diesem Band erschöpft, Nachfolger: sP-Joint-Inversion)
  class: handover
  date: 2026-09-09
  sha256: c95ff2c92253b562d47a4bb13ddd4c87ddf7cf7fbd3158a6d7e5c5bbed1760be
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-zonen-flotte-folge.md docs/befund/befund-2026-09-09-zonen-flotte-pick-hebel.md
-->

# Thematisches Handover — Tiefenphasen-Flotte (Folge-Atom: der Pick-Rausch-Hebel)

Stehendes Register der Tiefenphasen-/Seismik-Linie nach dem Pick-Hebel-Atom. Die Zonen-Flotte trägt jetzt die gewichtete Joint-Inversion; ihr Mittelwert steht unverzerrt (−6,0 km), und die verbleibende Streuung ist gemessen keine Pick-Streuung mehr.

## Geschlossen in diesem Atom

- **Der Pick-Rausch-Hebel — gebaut und gemessen (2026-09-09).** Sub-Sample-Lag (Parabel-Verfeinerung), Peak-Halbwertsbreiten-Gewichte (σ = gemessene Breite, Sample-Floor 1/rate, Kanten-Pick σ-absent), `invert_depth_weighted` (w = 1/σ²) neben dem unangetasteten `invert_depth_multi`; Kalibrier-Gate 24 Tests grün, `cargo check` 0/0. Fleet-Neulauf (dieselben 16 Events): Baseline reproduziert (with-clamp −14,7 km, after-exclusion −61,7 km, Streuung 88,6 km, 64 Δ-Gate-Skips), Joint-Mittel −6,0 km (11 Events, sd 59,2, se 17,9). (befund-2026-09-09-zonen-flotte-pick-hebel.md)
- **Die Runner-up-Verteilung — gemessen.** 0,88–1,00 an praktisch jeder Station: die tiefe Zone ist systematisch bimodal (pP/sP); das Max-Peak-Pick ist ein Mehrdeutigkeits-Mittel. Ein Ambiguity-Gate an dieser Verteilung würde die ganze Flotte überspringen — kein Gate (Rat einstimmig); die Ratio bleibt ein gemessenes Merkmal.
- **Der Hebel ist an diesem Stationsband erschöpft — gemessen, nicht erklärt.** Das Instrument bewegte den Mittelwert (−61,7 → −6,0 km), nicht die Event-Streuung (59,2 km): die Rest-Streuung ist Katalog-Wahrheit/Event-Term, kein Pick-Rauschen mehr.

## Benannt (Pendings, nicht still)

- **sP-Joint-Inversion beider Phasen** — Nachfolger-Atom (pP + sP, ak135-sP-Lag-Trennung; die Runner-up-Messung ist die Bedingung, die jetzt steht). Der alte Eintrag „Rest-Streuung (88,6 km) — Pick-Rauschen" ist damit gemessen geschlossen.
- **Die Tiefe der tiefen Zone bleibt an der Kante unaufgelöst** — die Joint-Statistik klemmt ebenso (us10006scr, Katalog 596,4, joint an der 660-Wand).
- **Externe Tiefen-Referenz (TauP/KEB95)** — pending, Instrument benannt.
- **Head-Wave-Lücke 410/660** — direktes-P-Triplikations-Gate gegen TauP; TauP/KEB95 external pending.
- **Quell-Strahlungsterm (CMT)** — pending ohne CMT-Lösung.
- **W-Phase-CMT als M9-Nachfolger-Atom** — Ersteinsatz emergent; STA/LTA-Pick bleibt Einsatz-Detektor.
- **Stromboli** — als Vulkan-Lehrer pending.
- **Hi-net/NIED** — NIED verlangt Registrierung; JP-2011-Wellenformen HTTP 204.
- **Eikonal-Löser über das volle Gitter** — Dijkstra über ETOPO1; das Gitter steht registriert, der Löser pending.
- **Die Erde als Sender** — Kreuzbereichs-Kalibrierung (Tonga 2022) pending; sub-stündlicher Druck/Infraschall (BGR-Array) als nächster Datenweg.

Baum: eine fremde untracked Datei liegt im Baum (`docs/befund/befund-bz-laic-nsurr100.md`, fremde Linie) — gemessen, unangetastet, nicht in meinem Commit; mein Pick-Hebel-Commit trägt zusätzlich den fremden Rename `handover-2026-09-09-disjunkte-linien-dispatch.md → archiv/` (Ganz-Index-Commit während einer parallel laufenden Sitzung) — benannt, nicht umgeschrieben.

Gemessen und geschlossen (bleibt im Befund, nicht hier): Feldstandard Seismik — jede Komponente hat ein Feld-Äquivalent; neu ist die Einbettung (ICRS, t_ref, Signal-Kegel).
