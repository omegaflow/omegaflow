<!--
  title: Thematisches Handover — Tiefenphasen-Flotte
  class: handover
  date: 2026-09-09
  sha256: 52c20570cbf7764b42a145fc234dece5b105299d0b9e728104ddbf5f8b9731f3
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-tiefenphasen-feldpilot.md docs/handover/archiv/handover-2026-09-09-seismische-ortung-tsunami.md docs/concepts/die-akteure-im-boden-und-wasser.md
-->
# Thematisches Handover — Tiefenphasen-Flotte

Stehendes Register der offenen Tiefenphasen-/Seismik-Linie. Der Feldpilot
(ein Ereignis, us10003re5) hat die Kette gemessen; die Statistik fehlt.

- **Die Flotte — geschlossen (2026-09-09).** 16 Ereignisse × Stationen,
  Mittelwert +1,7 km (se 4,7 km, unverzerrt); σ 19 km über Ereignisse und
  36 km über Stationen dominiert das ±10-km-Gate. (befund-2026-09-09-tiefenphasen-flotte.md).
  Die 1-km-Inversionsklasse (befund-2026-09-09-tiefenphasen-inversionsklasse.md)
  und die pP/sP-Polarität (befund-2026-09-09-tiefenphasen-polaritaet.md) sind
  ebenfalls geschlossen. Offene Folge: (a) die Streuung senken — besseres
  Picken / der mehrdeutige pP-Zweig bei Δ≈30°; (b) die Zonen-Flotte — braucht
  zuerst die Tiefherd-Erweiterung über 250 km.
- **Quell-Term/CMT** — die gemessene pP-Mischung (4×+, 2×−) trägt der
  Quell-Strahlungsterm; `pending` ohne CMT-Lösung. (befund-2026-09-09-tiefenphasen-polaritaet)
- **Tiefherd-Erweiterung über 250 km** — `MAX_DEPTH_KM = 250` in `ak135.rs`
  kappt die Tiefenphasen; Tonga 410–660 km braucht erst die Modell-Erweiterung
  (Voraussetzung der Zonen-Flotte).
- **W-Phase-CMT als M9-Nachfolger-Atom** — der Ersteinsatz der langen Quelle
  ist emergent; der STA/LTA-Pick bleibt Einsatz-Detektor, ortet das Epizentrum
  nicht (Kanamori & Rivera 2008). (befund-2026-09-09-tohoku-gsn-geblockt)
- **Stromboli** — als Vulkan-Lehrer `pending`.
- **Hi-net/NIED** — Nah-Stationen-Alternative; NIED verlangt Registrierung,
  die JP-2011-Wellenformen liefern HTTP 204 (nicht offen archiviert).
- **ETOIPO1-Referenz-Kernel** — Vollauflösung + CDN-Manifestation des
  395-MB-Gitters (Folge-Pflicht des Eikonal-Befunds).
- **MiniSEED-Dopplung** — `laic_probe.rs` trägt einen eigenen STEIM2/miniSEED-
  Parser parallel zu `tools/measure/src/miniseed.rs`; Konsolidierung `pending`.
- **Die Erde als Sender** — Kreuzbereichs-Kalibrierung (Tonga 2022, Wasser
  gegen Luft, zwei Uhren am selben Ohr) `pending`; der sub-stündliche
  Druck/Infraschall (BGR-Array) ist der nächste Datenweg. (befund-2026-09-09-tsunami-eikonal)

Gemessen und geschlossen (bleibt im Befund, nicht hier): Feldstandard Seismik —
jede Komponente hat ein Feld-Äquivalent; neu ist die Einbettung (ICRS, t_ref,
Signal-Kegel).
