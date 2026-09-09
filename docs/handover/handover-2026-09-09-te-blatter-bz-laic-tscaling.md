<!--
  title: Handover — TE-Linie: Bz/LAIC n_surr=100 und T-Skalierung geschlossen; der Riß (n=1000) ist die nächste Adresse
  class: handover
  date: 2026-09-09
  sha256: dc8d78c885c37355606c2367fbdcda9faecc979d32cc62ccd289d3c9da489110
  status: live
  see-also: docs/befund/befund-bz-laic-nsurr100.md docs/befund/befund-tscaling.md docs/befund/befund-betriebspunkt-sweep.md
-->
# Handover — TE-Linie: Bz/LAIC n_surr=100 und T-Skalierung geschlossen

Nachfolger des thematischen Registers `handover-2026-09-09-te-galileo-nsurr-folge.md`
(archiviert). Diese Sitzung hat die zwei Folge-Sheets gezogen und als Blätter
geschlossen; der Rat hielt beide Blätter einmal vor dem Commit (drei Konfunden
korrigiert: „schärfen", „AE↔Dst", „byte-genau").

## Geschlossen (diese Sitzung)

- **Bz/LAIC n_surr=100 → Blatt** (`docs/befund/befund-bz-laic-nsurr100.md`,
  `done`): die Bz-Pfeile halten unter 100 (AE 4,67×, Dst 3,12×, SYM-H 3,08×,
  |B| 3,22×, n 4,27×), kein Pfeil kehrt um; die ratio-Wanderung ist die
  Schwellen-Neuschätzung über 100 Surrogaten, nicht die Kante (n_surr steckt nur
  in der Schwelle, der TE-Wert ist derselbe). Die direkte **AE→Dst**-Kante trägt
  1,64× (lag2)/1,58× (lag1), die Umkehr Dst→AE bleibt still (0,71×/0,70×) — der
  „AE↔Dst"-Wortlaut des Sheet-Verdikts ist damit präzisiert. LAIC-Stille hält
  (mittlerer Exzess überall negativ; einziger Rohwert über dem Floor: Bz→F 0,204
  > 0,072, unter n=10 war die Zahl 0,27).
- **T-Skalierung → Blatt** (`docs/befund/befund-tscaling.md`, `done`): T=150/300/600
  am Betriebspunkt (Block/KSG, n_surr 100, α=0,05) tragen Power med 0,20/0,30/0,40
  und FPR 4,53/4,18/4,88 % — Power steigt mit T, FPR bleibt in enger Fläche unter
  α=0,05 ohne monotone Tendenz. T=600 stimmt in den zwei gemeldeten Zahlen mit
  Run 34387500520 überein (FPR 4,88 %, med 0,40; kein Byte-Beleg der Vorlage).
  Jede FPR ist die Kalibrierung ihrer eigenen Null — die neue Null (Block) sitzt
  bei 4,18–4,88 %, die alte (Residual/Binned/n10) bei 6,7–7,7 %; nicht als eine
  Achse gelesen.

## Offen (an die nächste Sitzung)

- **n=1000-Riß** — der Riß selbst (Block-Länge n^(1/3)=10 bei n=1000 zu kurz /
  KSG-Dimension bei n=1000) ist die nächste Adresse, kein Shift. Die Null bleibt
  Block (gemessen, `befund-betriebspunkt-sweep.md`); n=1000-Shift bleibt eine
  Register-Duty (`pending`).
- **Nadel Ⅲ** — Richtung TIAW vs Nanoflares; 613-Ereignis-Satz
  (`data/jsoc.stanford.edu/aia2015_fullyear.bin`, CDN-Monats-Assets);
  `aia_ladder_probe`/`aia_three_year_probe`; Kaskaden-Stärke × Sonnenstruktur.
- **Baupunkte** — `cycle_phase_shift_surrogate`-Nutzung, bedingte Multi-Force-TE
  (Phasenraum) — pending-Instrumente.
- **Desktop-Fork (GTX 970)** — 30-Jahres-Lauf bleibt Operator-seitig.

## Unberührt (nicht diese Sitzung)

Der Parallellauf (Tiefenphasen-Linie) hält uncommittete Dateien im Baum:
`tools/measure/src/bin/depth_phase_fleet_probe.rs`, `tools/measure/src/depthphase.rs`,
`docs/handover/handover-2026-09-09-mechanische-reste.md`. Diese Sitzung hat sie
nicht angefasst; der Commit dieser Sitzung trägt nur die zwei Blätter.
