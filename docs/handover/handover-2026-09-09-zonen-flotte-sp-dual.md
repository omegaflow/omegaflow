<!--
  title: Thematisches Handover — Tiefenphasen-Flotte, Folge-Atom: die sP-duale Inversion gebaut und gemessen (Präzision 45,6 km, Genauigkeit unverändert — der Rest ist der gemeinsame Katalogtiefen-Anker; sP-Δ-Faltung ungemessen)
  class: handover
  date: 2026-09-09
  sha256: 5488a80c4fa97dbc523451a55960fbe7e56ab4578272d400f9cbe5d836d5c4c1
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-zonen-flotte-pick-hebel.md docs/befund/befund-2026-09-09-zonen-flotte-sp-dual.md
-->
# Thematisches Handover — Tiefenphasen-Flotte (Folge-Atom: die sP-duale Inversion)

Stehendes Register der Tiefenphasen-/Seismik-Linie nach dem sP-Dual-Atom. Die Zonen-Flotte trägt jetzt beide Phasen; die sP-duale Inversion schärft die Präzision gemessen, nicht die Genauigkeit.

## Geschlossen in diesem Atom

- **Die sP-duale Inversion — gebaut und gemessen (2026-09-09).** `DepthPhase`/`phase_lag`/`invert_depth_dual` (ereignisweiter gewichteter pP+sP-Joint, pP-only wo sP fehlt) neben den unangetasteten pP-Funktionen; `s_p_sigma_s`; Kalibrier-Gate 29 Tests grün, `cargo check` 0/0. Fleet-Neulauf (dieselben 16 Events): sP-|corr|-Verteilung n=86 (min 0,44, Median 0,90, max 1,00) — Gate = gemessener Median 0,90, nicht das entlehnte 0,30. Dual-Fit (Gate 0,90): Mittel −5,6 km (12 Events, sd 45,6, se 13,2) gegen pP-only −6,0 (sd 59,2, se 17,9). (befund-2026-09-09-zonen-flotte-sp-dual.md)
- **Präzision geschärft, Genauigkeit nicht — gemessen, nicht erklärt.** sd 59,2 → 45,6 km (~23 %), Mittel unverändert: der Rest ist der gemeinsame Katalogtiefen-Fensteranker, kein Phasendefekt. us6000q5tp trägt 313 km, wo pP-only ausstand (sigma-absent).
- **Die 660-Kante klemmt in beiden Fits** (us10006scr) — die Wand bleibt die Wand.

## Benannt (Pendings, nicht still)

- **sP-Δ-Faltung** — ungemessen (Register-Duty, nicht 0.0).
- **sP-Beine der pP-übersprungenen Stationen** — pending.
- **Die Tiefe der tiefen Zone bleibt an der Kante unaufgelöst** — beide Fits klemmen.
- **Externe Tiefen-Referenz (TauP/KEB95)** — pending, Instrument benannt.
- **Head-Wave-Lücke 410/660** — direktes-P-Triplikations-Gate gegen TauP.
- **Quell-Strahlungsterm (CMT)** — pending ohne CMT-Lösung.
- **W-Phase-CMT als M9-Nachfolger-Atom** — Ersteinsatz emergent.
- **Stromboli** — als Vulkan-Lehrer pending.
- **Hi-net/NIED** — NIED verlangt Registrierung.
- **Eikonal-Löser über das volle Gitter** — Dijkstra über ETOPO1; das Gitter steht, der Löser pending.
- **Die Erde als Sender** — Kreuzbereichs-Kalibrierung (Tonga 2022) pending.

Baum: eine parallele Sitzung arbeitet an eigenen Linien (Bz/LAIC, disjunkte Linien); ihre Dateien bleiben unangetastet. Mein Pick-Hebel-Commit trägt einen fremden Rename (Ganz-Index-Commit, registriert — b24cf46); die Umzüge dieses Atoms lassen eine Prosa-Pfadangabe in `handover-2026-09-09-disjunkte-linien-folge.md` auf den alten Ort zeigen (fix-as-they-touch, fremde Datei).

Gemessen und geschlossen (bleibt im Befund, nicht hier): Feldstandard Seismik — jede Komponente hat ein Feld-Äquivalent; neu ist die Einbettung (ICRS, t_ref, Signal-Kegel).
