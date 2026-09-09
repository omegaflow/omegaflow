<!--
  title: Handover — Nobel-DAG: der multivariate Atom B für Bz und LAIC (geschlossen)
  class: handover
  date: 2026-09-08
  sha256: 2439bf13215eeee114bd041ceea100f200f15c57e87d4870bca06b43d4b62604
  status: archived
  see-also: docs/concepts/te-literatur-matrix.md docs/TODO.md docs/handover/handover-2026-09-07-nobel-dag-bz-laic.md
-->

# Handover — Nobel-DAG: der multivariate Atom B für Bz und LAIC (geschlossen)

Der Atom B ist gebaut und geschlossen: die volle multivariate Konditionierung
(PCMCI-Klasse) für die zwei Blätter mit mehrköpfigem Konfund — Bz (Sonnenwind →
Geomagnetik) und LAIC (Seismizität → Ionosphäre). Nachfolger-Übergabe von
handover-2026-09-07-nobel-dag-bz-laic.md.

## 1. Was gebaut ist

- te.rs: `transfer_entropy_conditional_binned_n` (Binning, N Konditionen,
  additiv — die KDE-Schätzer bleiben kanonisch unberührt), `conditional_te_stats_lagged_n`
  (N-dim lag-bewusste Null), `pcmci_links` (Vorwärts-Elternsuche, Null-Ordnung
  vom TE-Lag getrennt), `benjamini_hochberg` (FDR). 15 Gates grün.
- omni2.rs: Komponenten AE/AL/AU/SYMH/DST (comps 8–12, additiv zu 1–7).
- omni2_static_compiler: SPDF-Static-Ernte (omni2_YYYY.dat stündlich +
  omni_minYYYYMM.asc 1-min), Format-Positionen an Beispieldaten verifiziert.
- nobel_probe_bz: das Runge-2018-Gegenstück.
- nobel_probe_laic: die Common-Cause-Kontrolle der LAIC-Stille.
- COND_BIN_TE_WGSL + CondBinTeGpu: der konditionale GPU-Pfad, Parität gegen CPU.

## 2. Die gemessenen Befunde

- Bz (2015–2026, stündlich, 6 Kanäle V n Bz |B| AE Dst): Bz ist der gemeinsame
  Treiber von AE und Dst (4,3×/2,4× über der Schwelle), die direkte AE↔Dst-Kante
  ist marginal (1,1×) — Runge-2018 ist reproduziert. Die Rückkanten (AE/Dst →
  Solarwind) sind die Leckage der zeitgleichen Konditionierung, benannt.
- LAIC (1346 Ereignisfenster aus laic.bin): die Stille hält unter der
  Common-Cause-Kontrolle. Der mittlere Exzess ist überall negativ (der
  Surrogat-Floor-Bias); Bz→F ist die einzige erhöhte Kante (0,27).

## 3. Benannte Eigenschaften der Maschine

- Die naive Binning-TE trägt einen Endlich-Stichproben-Bias — die Null absorbiert ihn.
- Die zeitgleiche Konditionierung leckt an zeitgleich gekoppelten Solarwind-Kanälen
  (Rückkanten AE/Dst→Solarwind) — eine Eigenschaft der zeitgleichen (nicht
  lag-geschobenen) Konditionierung, benannt, nicht verschleiert.

## 4. Descoped (mit Messung, 2026-09-08)

- CDAWeb-HAPI-Erweiterung des omni2_compiler: CDAWeb-HAPI down gemessen (Timeout,
  0 Bytes); SPDF-Static liefert denselben OMNI-Bestand (geerntet 1995–2026).
- Kyoto-Realtime-Schwanz: der SPDF-Quicklook trägt AE bis Tag 228 (2026-08-16),
  DST bis Tag 243 (2026-08-31); Kyoto-Realtime (dst2609, aktuell) trägt den
  ~8-Tage-Schwanz — kein Atom-Quantum.

## 5. Register

Der Atom-Eintrag in TODO.md ist geschlossen. Das Index-Asset ist in sources.φ
registriert (omni2_indices.bin, format omni2_serie, at earth); die CDN-Manifestation
läuft über den CI-Manifestator (workflow omni2-indices-cdn.yml). Der 1-min-Kanal
`magnetosphere_symh_nt` ist konsumiert: nobel_probe_bz führt SYM-H als zweiten
Sturm-Kanal neben Dst — Bz→SYM-H 2,63× neben Bz→Dst 2,43× (SYM-H und Dst sind als
Ringstrom-Kanal austauschbar, gemessen).
