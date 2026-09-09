<!--
  title: Thematisches Handover — Tiefenphasen-Flotte (Tiefherd erweitert)
  class: handover
  date: 2026-09-09
  sha256: e373c21a2829eeeb729273b264b3995a42d96f91c98f653e6c5d71abd9320a9e
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-tiefenphasen-flotte-thematisch.md docs/handover/archiv/handover-2026-09-09-tiefenphasen-flotte.md docs/handover/archiv/handover-2026-09-09-tiefenphasen-feldpilot.md docs/handover/archiv/handover-2026-09-09-seismische-ortung-tsunami.md docs/concepts/die-akteure-im-boden-und-wasser.md docs/befund/befund-2026-09-09-ak135-tiefherd-erweiterung.md
-->
# Thematisches Handover — Tiefenphasen-Flotte (Tiefherd erweitert)

Stehendes Register der offenen Tiefenphasen-/Seismik-Linie. Der Feldpilot
(ein Ereignis, us10003re5) hat die Kette gemessen; die Flotte hat die
Statistik getragen; die Tiefherd-Erweiterung hat das Tor über 250 km geöffnet.

- **Tiefherd-Erweiterung über 250 km — geschlossen (2026-09-09).**
  `MAX_DEPTH_KM` 250 → 700, `DEPTH_KM` + [300 … 700] mit exakten Knoten auf
  410/660, Inversion bis 700 km. Interne Physik-Gates grün; die externe
  Tiefen-Referenz (TauP/KEB95) bleibt `pending` (Instrument benannt).
  (befund-2026-09-09-ak135-tiefherd-erweiterung.md). Offene Folge: die
  Head-Wave-Lücke 410/660 (nur direktes P bei Triplikations-Distanzen, die
  pP/sP-Inversion unberührt) — Instrument: direktes-P-Triplikations-Gate
  gegen TauP; die Rand-Klemmung best-at-MAX (ein >700-km-Ereignis läse 700).
- **Die Zonen-Flotte — bereit, ungemessen.** Tonga, 410–660 km: die
  Tiefherd-Erweiterung ist die benannte Voraussetzung, sie steht. Das Atom:
  Region/Katalog-Filter (mindepth ≥ 250 km), Flotten-Lauf über
  `depth_phase_fleet_probe`. Noch nicht gelaufen.
- **Die Flotte — geschlossen (2026-09-09).** 16 Ereignisse × Stationen,
  Mittelwert +1,7 km (se 4,7 km, unverzerrt); σ 19 km (Ereignisse) und 36 km
  (Stationen) dominiert das ±10-km-Gate. (befund-2026-09-09-tiefenphasen-flotte.md).
  Offene Folge: die Streuung senken — besseres Picken / der mehrdeutige pP-Zweig
  bei Δ≈30°.
- **Quell-Term/CMT** — die gemessene pP-Mischung (4×+, 2×−) trägt der
  Quell-Strahlungsterm; `pending` ohne CMT-Lösung. (befund-2026-09-09-tiefenphasen-polaritaet)
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
- **Vorgefundener Bruch (Fremdregister, nicht diese Linie)** —
  `tools/measure/src/bin/ephemeris_structure_probe.rs:88` kompiliert nicht
  (positional nach named im `println!`-Format), committed unter `baab781`;
  `cargo check -p omegaflow-measure` ist dadurch rot, die Lib ist sauber.
- **Verbraucher gekappt** — `quake_location_probe` `DEPTHS` bleibt bei 250
  (Demo-Sonde); ein Tiefherd-Lauf über diesen Probe müsste das Raster
  erweitern.

Gemessen und geschlossen (bleibt im Befund, nicht hier): Feldstandard Seismik —
jede Komponente hat ein Feld-Äquivalent; neu ist die Einbettung (ICRS, t_ref,
Signal-Kegel).
