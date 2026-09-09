<!--
  title: Handover — de441-mars-Rekompiilat: Dispatch steht (Run 34387873054 in flight), kein Duplikat gezündet; Re-Verifikation ausstehend
  class: handover
  date: 2026-09-09
  sha256: 81267311e7e2659f7e07719926fb9690681549c04695922c11e93272983f5d93
  status: archived
  see-also: docs/handover/archiv/handover-2026-09-09-mars-rekompilat.md docs/handover/handover-2026-09-09-de441-pin-redispatch.md docs/befund/befund-2026-09-09-de721-planeten-selektion.md
-->
# Handover — de441-mars-Rekompiilat: Dispatch

Übergabe der Dispatch-Linie. Der Dispatch steht — nicht durch eine zweite Zündung
dieser Sitzung, sondern als bereits laufender Run. Gemessen, kein Duplikat.
Die Re-Verifikation trägt die nächste Sitzung.

## 1. Dispatch-Befund (gemessen, gh run list / gh run view)

- Laufender Run: `34387873054`, workflow_dispatch, ref main, head `5275552`,
  gestartet 2026-09-09T18:15:14Z, status `in_progress`.
- `git merge-base --is-ancestor 59bf7c4 5275552` → ja: der Run trägt die
  CI-Lücken-Schließung (sb441-GM + solar-omega-g).
- Kein Duplikat gezündet: der Dispatch existiert bereits; ein zweiter paralleler
  Run schriebe in dieselben kanonischen CDN-Assets (der eine CI-Manifestator ist
  der einzige Schreiber) und fabrizierte einen Checkmark.
- Gemessene Vorgeschichte: die 4 früheren kernel-flatten-Dispatches heute
  (01:21, 10:02, 11:11, 11:58Z) lösten nicht grün auf (`completed failure` in
  gh, workflow_dispatch). Der laufende Run ist der fünfte Versuch; nichts ist
  als grün erklärt, bis der Run grün auflöst.

## 2. Run-Head-Abstand (genannt, nicht geglättet)

origin/main ist seit dem Dispatch weitergezogen (zuletzt `ec2ca10`); der Run
reitet head `5275552`, der 59bf7c4 trägt. Ob ein späterer Commit den
bodies-Job berührt, ist diese Sitzung unverified — die Re-Verifikations-Sitzung
prüft es, bevor sie sich auf den Run verlässt.

## 3. Ausstehend (Re-Verifikation, pending)

Nachdem der Run grün auflöst:

1. Local auffrischen: `data/ssd.jpl.nasa.gov/` + Membran-Cache löschen.
2. `OMEGAFLOW_HIDDEN=1 cargo run -p omegaflow-measure --bin orientation_probe`
   — de441-mars-Anker-Δ soll in die ~0-km-Klasse fallen (vorher 6 045,3 km).
3. `cargo test -p omegaflow-measure --bin orientation_probe` (Lehrbuch-Gate,
   silent, kein Fenster).
4. `cargo run -p omegaflow-measure --bin ephemeris_structure_probe` — bestätigt
   die 183-MB-Gestalt für earth/mars.
5. de441-cdn-watch (≥183 MB) grün.

## 4. Producer-Frage (19-MB-earth) — pending

`ephemeris_earth.bin` (19 MB, 1400 Jahre, 6 d) — Produzent `unverified`.
Register-Pflicht, kein Datenwert. Der grüne Flatten überschreibt sie mit der
183-MB-Form; die Frage bleibt benannt.

## 5. Register-Reste (übernommen, unverändert offen)

witness presence bleibt reserviert (Consent-Wurzel Art (c) recorded, nicht
gebaut; ein maschinell gemessener Bio-Ton ist eine akustische Serie, nie
presence). feature-gate `gpu` = eigenes Atom, `pending`. Membran-Reste:
M02 ESP32-Firmware no_std; M03 Audio-Gain ohne tanh; M04 Navigation
(Nebra-Kalibrierung); M07 ⌘K-Palette; M05/M06 Station-Sensoren als SI-4-Token;
Kamera ~19k Pixel-Quellen als WS-Traffic-Hotspot; OPeNDAP-Integration;
advective per-Quelle.

## 6. Zustand des Arbeitsbaums

Working tree sauber bei Sitzungsende. Fremde parallele Sitzung aktiv (origin/main
bewegte sich während dieser Sitzung: `610e3d5` → `1adc2c2` → `b8a2e67` →
`ec2ca10`). Kein Stray dieser Sitzung; nur die zwei Handover-Dateien committet.
