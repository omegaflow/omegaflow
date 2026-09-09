<!--
  title: Thematisches Handover — Galileo N_SURR 20-vs-10: das Blatt steht; T-Skalierung und Bz-Hälfte in Flug
  class: handover
  date: 2026-09-09
  sha256: d836543256cb2f8ecb1fe1bb5983ae4bf19f9e30690f635c528b094afc9d8783
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-te-galileo-nsurr.md docs/befund/befund-galileo-nsurr-20.md
-->
# Thematisches Handover — Galileo N_SURR 20-vs-10: das Blatt steht; T-Skalierung und Bz-Hälfte in Flug

Nachfolger des thematischen Registers `handover-2026-09-09-te-galileo-nsurr.md`
(archiviert). Diese Sitzung hat die schweren Läufe nach CI gelegt und das
Galileo-Blatt geschlossen; die zwei Folge-Blätter stehen als Sheets bereit.

## Geschlossen (diese Sitzung)

- **Galileo N_SURR 20-vs-10 → Blatt** (`docs/befund/befund-galileo-nsurr-20.md`,
  `done`): **10 bleibt das gemessene Blatt.** Die Phasen-/Block-Schwellen
  (mean+2σ) bewegen sich zwischen 10 und 20 Surrogaten innerhalb der eigenen
  Kleinstichproben-Streuung; sechs Rand-Zellen ändern die Zugehörigkeit (zwei
  Umkehr-Verluste, ein Vorwärts-Gewinn, drei Null-Familien-Wechsel), keine
  Schlagzeile kippt — der Richtungsbefund (Stärke→Noise über der Block-Null im
  1996-Fenster) hält und wird unter 20 schärfer; spec/external/stair/mode3s1
  bleiben null. Der Auftrag ist archiviert, kein stiller Shift.
- **Externer-Probe-Defekt** — `galileo_floor_external_te.rs` schrieb seinen
  Bericht nur in eine Datei, nicht nach stdout; das erste CI-Sheet trug den
  externen 10-vs-20-Vergleich nicht. Fix: der Probe druckt zusätzlich nach
  stdout (Commit `f546019`), Sheet neu gezogen, externer Vergleich vollständig.
- **Bz-CDN-Fallback** — `nobel_probe_bz.rs` `load_indices` liest jetzt das
  CDN-Asset, wenn `omni2_indices.bin` lokal fehlt (vorher nur Cache → „Bz
  unmeasured"). Das Asset war bereits manifestiert (`sources.φ:1917`, CDN 200);
  es fehlte nur der Fallback.
- **T-Skalierungs-Instrument** — `--tscale`-Flag in `pcmci_class_benchmark.rs`
  (das T=150/300/600-Trio am Betriebspunkt) + `te-tscaling.yml`.
- **Schwere Läufe** — `galileo-nsurr-20` (Runs 34400114106 + 34402236206, done),
  `te-bz-laic-nsurr100` (Run 34401571351, done), `te-tscaling` (Run 34401566532,
  in Flug).

## Gemessen, Blatt folgt (Sheets liegen bereit)

- **Bz-Hälfte** (Run 34401571351, Sheet `bz-laic-nsurr100-sheet.txt`): die
  Index-Kanäle messen jetzt — unter n_surr=100 trägt der Baum Bz→AE 4,67×
  (lag1), Bz→Dst 3,12× (lag2), Bz→SYM-H 3,08×, Bz→|B| 3,22×, Bz→n 4,27×. Die
  alten 4,3×/2,4× (unter n_surr=10) sind damit unter 100 nachgemessen. Das
  Blatt (`docs/befund/…`) schreibt die nächste Sitzung.
- **T-Skalierung** (Run 34401566532, in Flug): T=150/300/600 am Betriebspunkt;
  das T=600-Blatt aus Run 34387500520 (FPR 4,88 %, Power med 0,40) wartet auf
  seine T=150/300-Gegenstücke.

## Offen (an die nächste Sitzung)

- T-Skalierungs-Sheet → Blatt (Run 34401566532).
- Bz/LAIC-Sheet → Blatt (Run 34401571351).
- n=1000: der Riß selbst (Block-Länge n^(1/3)=10 bei n=1000) ist die nächste
  Adresse, kein Shift.
- Nadel Ⅲ: Richtung TIAW vs Nanoflares; 613-Ereignis-Satz; `aia_ladder_probe`/
  `aia_three_year_probe`.
- Baupunkte: `cycle_phase_shift_surrogate`-Nutzung, bedingte Multi-Force-TE
  (Phasenraum) — pending-Instrumente.
- Desktop-Fork (GTX 970): 30-Jahres-Lauf bleibt Operator-seitig.

## Nebenlinie (nicht diese Sitzung)

Die depthphase/ETOPO1-Arbeit (660-Wand, miniSEED-Konsolidierung,
`quake_location` DEPTHS) wurde parallel committet (`a942ee2`, `255a153` — die
Zonen-Flotte-Linie); sie gehört nicht zu dieser TE-Linie und ist dort
abgeschlossen.
