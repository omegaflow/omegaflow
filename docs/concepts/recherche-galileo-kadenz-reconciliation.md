# Recherche — Galileo-resid Kadenz: 1-s gegen 60-s (Reconciliation)

Datum: 2026-09-05. Subagent grind-pro. Quellen: `data/galileo_resid.bin` (gemessen),
`src/archivar/atdf.rs` (Reduktionspfad), `tools/harvest/src/bin/galileo_atdf_compiler.rs`,
`tools/measure/src/bin/galileo_band_probe.rs`, `befund-galileo-banden-kamm-ton.md`,
`befund-galileo-te-spec.md`, `TRK_2_25.TXT` (PDS GO-J-RSS-1-TDF-V1.0/DOCUMENT, in-volume,
HTTP 200, 33 920 B, PDS3-Text-Label).

## Frage

Zwei Befunde desselben `galileo_resid.bin` scheinen zu widersprechen: der
Banden-Befund reproduzierte den 20-s-Kamm auf einem "60-s-Raster"; der Spec-TE-Befund
maß die realisierte Kadenz als 1 s (frac 0.998666). Welche Kadenz ist echt, und was
war das "60-s-Raster" der Band-Analyse — eigene 60-s-Segmentierung der Probe oder
native Sub-Struktur?

## Reduktionspfad (gemessen, atdf.rs + compiler)

`galileo_atdf_compiler.rs` liest alle `.TDF`-Dateien der fünf RSS-TDF-Volumes und ruft
je Datei `reduce_resid` (atdf.rs:603). `reduce_resid` schreibt **einen GASR-Record je
nativem Tracking-Data-Logical-Record** (dtype 1/2) — kein Dezimieren, kein
Interpolieren, kein 60-s-Resampling. Feld [6] des GASR = `sampler_time/100` s ist der
Per-Record-Sampler des TDF-Records (TRK-2-25 Feld 30, "seconds*100"); das resid (Feld
[1]) ist Feld 60 "Doppler Residual (Hz*1000)" / 1000, wörtlich kopiert. Die Kadenz des
Bins **ist** die native Record-Kadenz der archivierten TDF-Dateien.

## Messung (eigener Scan, ganzes Bin, 14 077 825 Records)

dt zwischen aufeinanderfolgenden Records derselben (Station, Mode), dt ≤ 240 s:

- Mode 1: n_pairs 6 015 170, bei 1 s 6 009 794 (frac 0.99911), bei 60 s 1 506.
- Mode 2: n_pairs 2 152 782, bei 1 s 2 141 047 (frac 0.99455), bei 60 s 9 898.
- Mode 3: n_pairs 446 485, bei 1 s 445 820 (frac 0.99851), bei 60 s 215.
- Alle: 60-s-Paare 11 619; 1-s-Paare 8 596 661. (Record-Zählung des te-spec-Befunds
  "60 s nur 18 674 von 14 M" ist mit dieser Paar-Zählung konsistent — ein 60-s-Lauf von
  L Records ergibt L−1 Paare.)

**Dez-1990-Fenster (Unix-Tage 7638..7670), Mode 2, Stations 14/43/63:**
60-s-dt-Paare st14 1 235, st43 3 253, st63 3 625; dazu vereinzelte 5-s-Paare (112/34/77).
Die drei Stationen tragen zusammen ≈ 8 113 der globalen 9 898 Mode-2-60-s-Paare — das
Dez-1990-Zweiweg-Fenster der 70-m-Stationen ist **das** 60-s-Fenster des Bestands.
Tage beobachtet: st14 7638–7649, st43/63 7638–7645.

## Reconciliation

- **Beide Befunde messen dasselbe Bin, verschiedene Epochen.** Die 60-s-Kadenz ist in
  den Dez-1990-Zweiweg-Pässen der 70-m-Stationen (14/43/63) **nativ** — gemessen am
  realisierten dt der Records, nicht an einem Feld. Die 1-s-Kadenz (99.4–99.9 % je Mode)
  ist die native Kadenz des übrigen Bestands (die te-spec-Läufe lagen bei TDB-Tag
  9457–9903 ≈ 1996–97). Zwei native Kadenzen koexistieren im Archiv, epochal getrennt.
- **Das "60-s-Raster" der Band-Analyse ist keine Artefakt der Proben-Segmentierung.**
  `galileo_band_probe.rs` nutzt GAP_S=60 nur zur Segment-/Pass-Bildung für die
  lineare Detrendung; das Lomb-Scargle rechnet auf den **nativen** Zeitstempeln, kein
  Downsampling auf 60-s-Stützstellen (detrend_blocks:178, ls_power:56). Eine 1-s-Serie
  kann diesen Alias **nicht** erzeugen: Nyquist 0.5 Hz, 50 mHz ist dort ein ordinäres
  auflösbares Band. Die Degeneranz-Explosion bei exakt 50/100/150/200 mHz entsteht nur,
  wenn die Stützstellen wirklich 60 s auseinanderliegen (50 mHz × 60 s = 3.0 Zyklen je
  Abtastschritt — ganze Zahl). Der Kamm **beweist** die native 60-s-Stützstelle; die
  dt-Histogramme messen sie direkt.
- **1 s ist nativ, keine Interpolation.** TRK-2-25 (in-volume) definiert das ATDF als
  die von NOCC/NAV aus den IDRs erzeugte Archivierung **aller** empfangenen
  Radio-Metrik-Samples: je empfangener Doppler-Zählung ein time-geordneter
  Tracking-Data-Logical-Record mit eigenem Zeit-Tag, Feld 30 Sampler Time (seconds*100)
  und Feld 60 Doppler Residual. Die Spec schreibt keine feste Sample-Rate vor — die
  Kadenz liegt je Pass-Konfiguration im Record (Sampler Time) und ist im Galileo-TDF-
  Archiv realisiert als ~1 s (Mehrheit, alle Moden) bzw. 60 s (Dez-1990-Zweiweg
  70-m-Stationen). Der Reduktionspfad interpoliert/dezimiert nicht; 1-s wie 60-s sind
  native Record-Kadenzen.

## Register-Satz

*Die realisierte Galileo-resid-Kadenz ist nativ je Record (TRK-2-25 Feld 30/60; Reduktion
ohne Interpolation/Dezimierung): ~1 s über den Bestand (Mode 1/2/3 ≥ 0.9945), nativ 60 s
im Dez-1990-Zweiweg-Fenster der 70-m-Stationen 14/43/63 (≈ 8 100 der 9 900 Mode-2-60-s-
Paare). Das "60-s-Raster" des Kamm-Befunds ist diese native Sub-Struktur — keine Proben-
Segmentierungs-Artefakt; eine 1-s-Serie kann den 50-mHz-Kamm nicht erzeugen (Nyquist
0.5 Hz, Degeneranz nur bei 60-s-Stützstellen). Die beiden Befunde messen verschiedene
Epochen desselben Bins; kein Widerspruch. Offen bleibt (wie im Kamm-Befund), welcher
Reduktions-/Tracking-Schritt die 60-s-Zählintervalle der Dez-1990-Zweiweg-Pässe setzte.*
