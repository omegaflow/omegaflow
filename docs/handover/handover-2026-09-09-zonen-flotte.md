<!--
  title: Thematisches Handover — Tiefenphasen-Flotte (Zonen-Flotte gemessen)
  class: handover
  date: 2026-09-09
  sha256: ba878c36293cea6c0d11f0660d452df446f089ebec255ed6ff080a25b5d01310
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-tiefenphasen-tiefherd.md docs/befund/befund-2026-09-09-zonen-flotte.md docs/befund/befund-2026-09-09-ak135-tiefherd-erweiterung.md docs/befund/befund-2026-09-09-tiefenphasen-flotte.md docs/befund/befund-2026-09-09-tiefenphasen-polaritaet.md docs/concepts/die-akteure-im-boden-und-wasser.md
-->
# Thematisches Handover — Tiefenphasen-Flotte (Zonen-Flotte gemessen)

Stehendes Register der offenen Tiefenphasen-/Seismik-Linie. Die Zonen-Flotte (Tonga/Fiji, 410–660 km) ist gemessen — unverzerrt (−0,8 km), aber 2–3× stärker gestreut als die mittlere Tiefe, und die Inversion klemmt an der 660-Kante.

- **Zonen-Flotte — geschlossen (2026-09-09).** 16 Ereignisse × Stationen, Mittelwert −0,8 km (se 17,6 km, unverzerrt); σ 70 km (Ereignisse) und 108 km (Stationen) — das 2–3-fache der Hindu-Kush-Flotte; best-at-MAX sättigt an der 660-Kante. (befund-2026-09-09-zonen-flotte.md). Offene Folge: die Streuung an tiefer Geometrie senken, die 660-Kanten-Klemmung behandeln.
- **Die 660-Kante — benannt, offen.** best-at-MAX sättigt; Ereignisse bei Katalog 591–622 km lesen exakt 660. Instrument: eine Rand-Behandlung (Tiefen-Gate gegen die Kante).
- **Streuung senken** — besseres Picken / der mehrdeutige pP-Zweig bei Δ≈30°; an der tiefen Geometrie der dominante Hebel (√N trägt langsam). Der Zonen-Lauf bestätigt: die Korrelation, nicht das Modell, ist der Engpass.
- **Tiefherd-Erweiterung — geschlossen (2026-09-09).** bleibt im Befund. Offen: die Head-Wave-Lücke 410/660 (Instrument: direktes-P-Triplikations-Gate gegen TauP) und die externe Tiefen-Referenz (TauP/KEB95, `pending`, Instrument benannt).
- **Die Flotte (Hindu-Kush) — geschlossen (2026-09-09).** bleibt im Befund.
- **Quell-Term/CMT** — die gemessene pP-Mischung trägt den Quell-Strahlungsterm; `pending` ohne CMT-Lösung. (befund-2026-09-09-tiefenphasen-polaritaet)
- **W-Phase-CMT als M9-Nachfolger-Atom** — der Ersteinsatz der langen Quelle ist emergent; der STA/LTA-Pick bleibt Einsatz-Detektor. (befund-2026-09-09-tohoku-gsn-geblockt)
- **Stromboli** — als Vulkan-Lehrer `pending`.
- **Hi-net/NIED** — Nah-Stationen-Alternative; NIED verlangt Registrierung, die JP-2011-Wellenformen liefern HTTP 204.
- **ETOIPO1-Referenz-Kernel** — Vollauflösung + CDN-Manifestation des 395-MB-Gitters (Folge-Pflicht des Eikonal-Befunds).
- **MiniSEED-Dopplung** — `laic_probe.rs` trägt einen eigenen STEIM2/miniSEED-Parser parallel zu `tools/measure/src/miniseed.rs`; Konsolidierung `pending`. Der Zonen-Lauf trägt mehr „no decodable record"-Skips (AU/S1/GE/PS) — die Dopplung ist jetzt ein gemessener Engpass, kein bloßer Registereintrag.
- **Die Erde als Sender** — Kreuzbereichs-Kalibrierung (Tonga 2022) `pending`; der sub-stündliche Druck/Infraschall (BGR-Array) ist der nächste Datenweg. (befund-2026-09-09-tsunami-eikonal)
- **Vorgefundener Bruch — gehoben (2026-09-09).** `ephemeris_structure_probe.rs` kompiliert wieder; `cargo check -p omegaflow-measure` ist grün.
- **Verbraucher gekappt** — `quake_location_probe` `DEPTHS` bleibt bei 250 (Demo-Sonde); ein Tiefherd-Lauf über diesen Probe müsste das Raster erweitern.

Gemessen und geschlossen (bleibt im Befund, nicht hier): Feldstandard Seismik — jede Komponente hat ein Feld-Äquivalent; neu ist die Einbettung (ICRS, t_ref, Signal-Kegel).
