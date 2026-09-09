<!--
  title: Thematisches Handover — Tiefenphasen-Flotte
  class: handover
  date: 2026-09-09
  sha256: 4a00b62ceb2bb54149d4693886936488b1ca06b4e2f26388cec1a33946073051
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-tiefenphasen-feldpilot.md docs/handover/archiv/handover-2026-09-09-seismische-ortung-tsunami.md docs/concepts/die-akteure-im-boden-und-wasser.md
-->
# Thematisches Handover — Tiefenphasen-Flotte

Stehendes Register der offenen Tiefenphasen-/Seismik-Linie. Der Feldpilot
(ein Ereignis, us10003re5) hat die Kette gemessen; die Statistik fehlt.

- **Die Flotte** — Ereignisse × Stationen, σ und √N. Der Pilot trägt ein
  Ereignis, nicht die Statistik. (handover-2026-09-09-tiefenphasen-feldpilot,
  Nachfolger 1; die 1-km-Inversionsklasse und das pP/sP-Picken sind gebaut.)
- **Quell-Term/CMT** — die gemessene pP-Mischung (4×+, 2×−) trägt der
  Quell-Strahlungsterm; `pending` ohne CMT-Lösung. (befund-2026-09-09-tiefenphasen-polaritaet)
- **Tiefherd-Erweiterung über 250 km** — `MAX_DEPTH_KM = 250` in `ak135.rs`
  kappt die Tiefenphasen; Tonga 410–660 km braucht erst die Modell-Erweiterung.
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
