<!--
  title: Handover — Tiefenphasen-Feldpilot: echtes pP/sP-Picken an einem tiefen Ereignis (us10003re5, M7.5 Hindu Kush — Median 250 km gegen Katalog 231 km); die Kette läuft, das ±10-km-Gate wird nicht getragen, die Flachquellen-Polaritätsannahme kippt an tiefer Geometrie
  class: handover
  date: 2026-09-09
  sha256: 00111684ff5c9abc871d42fb627dd484d4b2f3fb560cfc5318dbbb434ed7e6b9
  status: archived
  see-also: docs/TODO.md docs/concepts/die-akteure-im-boden-und-wasser.md docs/befund/befund-2026-09-09-tiefenphasen-feld.md docs/befund/befund-2026-09-09-tiefenphasen-diagonale.md docs/befund/befund-2026-09-09-ak135-tiefenphasen.md
-->

# Handover — Tiefenphasen-Feldpilot

Übergabe für die nächste Sitzung. Gebaut und gemessen wurde das echte
pP/sP-Picken an einem tiefen Ereignis — der Feldpilot, der „Scharfe Tiefe"
schließt. Committet als `3e2ac6f` (Probe + Befund + Blatt) und `7085a48`
(TODO-Registerzeile).

## 1. Was gebaut wurde

- `tools/measure/src/bin/depth_phase_field_probe.rs` (5 Tests) — die Kette an
  echter Wellenform: USGS-FDSN-Katalog → Stations-Band (IRIS fdsnws/station,
  30–90°) → BHZ-Fetch (EarthScope dataselect) → P-Pick (Bandpass 0,5–2 Hz +
  Erstbruch, STA/LTA-Fallback) → SNR-Gate ≥ 3 am Onset *bevor* ins pP-Fenster →
  Kreuzkorrelation des P-Wavelets im Lag-Fenster `[0,7·lag, 1,3·lag]` aus der
  ak135-Diagonale → 1D-Inversion (`DEPTHS_FINE`). sP absent = skip, nie 0.0.
- Die fünf Nägel des Operators sitzen als Code, nicht als Versprechen:
  (1) Auswahlregel registriert vor dem ersten Fetch (Tiefe ≥ 35 km, M ≥ 6,
  Land-Epizentrum Hindu-Kush-Box, GBCO-Zeuge je Ereignis); (2) Fehlerbudget vor
  dem Lauf (3,88 s/5 km → ±2,6 km); (3) Polaritäts-Zeuge (Korrelation trägt
  beide Vorzeichen); (4) Injektionstest in die echte Coda (skaliertes,
  verschobenes P-Wavelet, beide Polaritäten); (5) Kanten-Flagge + Schließ-
  Formulierung („kein pP" trägt zwei Lesarten).
- Befund `docs/befund/befund-2026-09-09-tiefenphasen-feld.md` (sha256 gesetzt).
- Blatt `docs/concepts/die-akteure-im-boden-und-wasser.md`: Punkt 1 → „Die
  Flotte" als Nachfolger; „Was steht" trägt jetzt den Feldpilot.

## 2. Die Messung

Ereignis `us10003re5` — M7.5, 36,5244 N / 70,3676 E (Hindu Kush), Katalog-Tiefe
231,0 km, Ursprung 2015-10-26T09:09:42, GBCO-Land 3273 m. 6 BHZ-Stationen
bestanden das SNR-Gate; je Station pP-Pick → 1D-Inversion → 200–250 km.
**Median 250 km gegen Katalog 231 km, Offset +19,0 km — außerhalb des
±10-km-Gates** (zwei Lesarten: unsere Streuung oder der Katalog daneben).

Zwei benannte Befunde, die der Pilot trägt:
- **Die Flachquellen-Polaritätsannahme kippt.** Lehrbuch (Flachquelle): pP
  invertiert. Gemessen an dieser tiefen Geometrie: pP positiv (+0,63…+0,91) an
  vier, negativ (−0,68, −0,97) an zwei Stationen; sP durchgehend negativ. Der
  Zeuge trennt die Phasen, die Vorzeichen-Zuordnung der Flachquelle überträgt
  sich nicht — der Reflexionskoeffizient am freien Rand hängt am Einfallswinkel.
- **Auflösungs-Grenze benannt.** `DEPTHS_FINE` rastet im Bereich 200–250 km in
  50-km-Schritten; Katalog 231 km liegt zwischen zwei Rasterwerten.

## 3. Offene Nachfolger (benannt, nicht gedeutet)

1. **Die Flotte** — Ereignisse × Stationen, σ und √N; der Pilot trägt ein
   Ereignis, nicht die Statistik.
2. **Feinere Tiefen-Inversionsklasse** — das 50-km-Raster ist die genannte
   Auflösungs-Grenze.
3. **pP-Polarität an tiefer Geometrie neu ableiten** — die Flachquellen-Regel
   gilt hier nicht.

## 4. Zustand des Arbeitsbaums (nachgemessen 2026-09-09)

- **Der Baum war während der Sitzung parallel aktiv.** Eine parallele Sitzung
  brach einen Rebase ab und setzte `main` auf `origin/main`; das pP/sP-Fundament
  (`p_p_travel`/`s_p_travel` in `ak135.rs`, `depth_phase_probe.rs`, der Blatt,
  vier Befunde) fiel aus dem Baum. Auf Operator-Wort cherry-pickte ich `a7be9c7`
  zurück (Commit `4c28bcc`). Das Fundament steht wieder.
- **Kein Rückstand von mir.** Mein Stash (`stash@{0}`, „parallel-TODO-pre-
  cherry-pick") ist gedroppt; die TODO-Registerzeile ist committet (`7085a48`).
- Das uncommittete Werk der parallelen Sitzung (gaia-xp, vo-tap, meteo,
  neptune_apparent_chain_probe, …) gehört ihr — nicht angefasst.
- `cargo check -p omegaflow-measure --bin depth_phase_field_probe --bin
  depth_phase_probe` null Warnungen; 10 Tests grün (5 Feldprobe + 5 Diagonale).
  Die parallele Sitzung trägt ihrerseits einen kompilierfehlenden
  `neptune_apparent_chain_probe.rs` — ihr Stand, nicht meiner.

## 5. Register

Geschlossen: „Scharfe Tiefe" → Feldpilot bestanden (1 Ereignis). Offen
(unverändert, nicht von dieser Sitzung): Stationsterm, Tonga, W-Phase-M9,
Stromboli, CDN ETOPO1, MiniSEED-Dopplung. Neu benannt: die drei Nachfolger
aus Abschnitt 3.
