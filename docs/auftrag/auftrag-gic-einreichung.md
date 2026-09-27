<!--
  title: Auftrag — GIC-Paper einreichen (Ziel: Space Weather)
  class: auftrag
  date: 2026-09-27
  sha256: e9a97e4b1b0458ba1830ba86beb8ab17ea4b6ebcbb1ea9f79e1e77ad178866a5
  status: live
  see-also: docs/paper/gic-causal-driver.md
-->
# Auftrag: die Einreichung des GIC-Papers

## Zweck

Der Bau-Auftrag aus `future-folge112` (7 Lücken + Prior-Art-Framing) ist
abgeschlossen; `docs/paper/gic-causal-driver.md` ist reif. Dieser Auftrag trägt
die **Einreichkante**: Venue, Cover Letter, Autoren-/Framing-Klärung, der eine
verbleibende Vorbereitungsschritt. Der **Akt** (Absenden über GEMS / ESSOAr) ist
die Operator-Hand — das Gegenüber ist ein Dritter (Verlag).

Das Papier ist **höchste Priorität** (Operator-Wort, 2026-09-27).

## Venue (gemessen 2026-09-27 via `archive_search`)

| Venue | Scope-Treffer | OA/APC | Einreichung | Preprint |
|---|---|---|---|---|
| **Space Weather (AGU/Wiley)** | „understanding, forecasting and the mitigation of space weather and its impacts" — das Ground-dB/dt ist der Impact-Kanal | OA, **$3,240** | GEMS | ESSOAr |
| Annales Geophysicae (EGU/Copernicus) | solar-terrestrial + space weather + geomagnetism; offenes interaktives Review | OA, **€1,350** | Copernicus-Editor | EGUsphere (frei) |
| JSWSC (EDP) | „new methods … data analysis techniques" (Technical Article) | OA, **€1,250** | Editorial Manager | arXiv |

**#1 = Space Weather.** Die Scope-Zeile nennt genau die gemessene Kette; die
direkten Nachbarn liegen dort (Pulkkinen et al. 2017, `10.1002/2016SW001501`;
Ground-dB/dt-Statistik 2024, `10.1029/2023sw003767`; GIC-Forecast-Methodenpapier
2026-09-26). ≤25 publication units passen; kein Grund-/Seitenpreis; 54 %
Annahme, 42 Tage erste Entscheidung. **Fallback:** AnGeo (offenes Review, Venue
der TE→Dst-Linie Johnson, Wing & Camporeale 2018), dann JSWSC (Technical
Article). GRL fällt (Letters, 12 PU, „research articles not accepted").

## Prior-Art (gemessen 2026-09-27)

Kein Papier gefunden, das TE von L1-Solarwind-Treibern auf **Ground-dB/dt-Maxima
an INTERMAGNET-Stationen** unter einer phasen-randomisierten Null **mit
family-wise Schranke** misst. Nächste Nachbarn: Manshour et al. 2021
(`10.3390/e23040390`, Ziel AE/SYM-H); Boutsi et al. 2025 (`10.3390/e27020172`,
GIC-Indices, kein TE); Johnson, Wing & Camporeale 2018
(`10.5194/angeo-36-945-2018`, Ziel Dst). **Novelty = Kanal (TE → Ground-dB/dt) +
Schranke (family-wise round-maximum)**, nicht die phasen-randomisierte Null allein.
**CJSS 2022 gelesen — Volltext, gemessen 2026-09-27:** Yu, Tong, Fang & Hu
(`10.11728/cjss2022.03.210406045`) ranken Solarwind-Treiber zum Sym-H-Index per TE
(93 Stürme, 2010–2018); E und Bz dominieren bei 60 min (E 0.200, Bz 0.196 nats); der
Null ist ein Quell-Shuffle (100 Resamples, 95 %), **keine** family-wise Korrektur. Ziel
ist der Sturm-Index, nicht Ground-dB/dt → unsere Novelty-Zeile wird **nicht
vorweggenommen**, in der Richtung **gestützt**; als Zitat im Papier aufgenommen
(Related Work + Referenz). Volltext/Referenzen:
`docs/paper/yu-tong-fang-hu-2022-transfer-entropy-solar-wind-drivers.md`; PDF
`data/cjss.ac.cn/210406045.pdf`.

## Cover Letter (Entwurf, 8 Sätze)

1. We submit "The directional driver of geomagnetically induced currents" as a Research Article in *Space Weather*.
2. Which L1 quantity drives the ground excitation in the directional information-flow sense is open sub-daily; prior transfer-entropy work ends at magnetospheric indices, and this manuscript measures the ground end of the chain.
3. We measure KDE transfer entropy (Silverman bandwidths, phase-randomized surrogates, fixed seeds) from L1 Bz, speed and density to minute, hourly and daily maxima of dB/dt at INTERMAGNET Abisko and Sodankylä, with a family-wise bound — the round-maximum surrogate TE — as the multiple-comparison control.
4. The result is reported exactly as measured: no family-clearing hourly driver is established; the yearly-round Bz→dB/dt arrow (2024/2025 ABK, 2024 SOD) does not survive the hardened quarterly null (24/24 directed rows family bound, n_surr = 100, lag sweep 0–6 h) and PCMCI removes the conditioned edge in 13 of 16 shards; the two witnesses are carried as a riss, never averaged.
5. What survives the riss: the density control stays silent throughout, the forward direction dominates the reverse in every yearly round, and the 32-year daily grain is empty because daily means wash the storm signal out — Bz remains the leading sub-daily candidate via the yearly arrow and the asymmetry.
6. The contribution is the instrument and its honesty: a family-wise bound that gives the GIC-driver literature a conservative standard for reading single-lag driver claims, applied at the ground end of the chain where it has not been applied before.
7. All values are machine-measured; estimator, probes and data routes are published in the omegaflow repository, and the estimator is validated against the Schreiber (2000) coupled-Hénon benchmark in the manuscript.
8. The manuscript has not been submitted elsewhere; we will post it to ESSOAr at submission.

## Offene Vorbereitung (autonom bis zur Kante)

- **Titel (entschieden 2026-09-27):** englisch, „The directional driver of
  geomagnetically induced currents" (58 Zeichen, Gate title ≤ 75). Der Titel nennt
  den Impact-Kanal (GIC); gemessen ist dB/dt, die GIC-Anregung — §6 benennt es. Kein
  Untertitel (Gate-Länge).
- **Autorenblock (angelegt 2026-09-27):** nicht-anonyme Form privat in
  `state/paper/gic-autoren-2026-09-27.md` (Johannes Tyroller, ORCID
  `0009-0007-5565-6348`; getrackt bleibt `*Omegaflow Working Group*`).
- **ESSOAr-Preprint** zum Einreichzeitpunkt posten.

## Kante (Operator-Hand)

- **Artefakt:** dieses Doc + `docs/paper/gic-causal-driver.md` (Gate grün,
  title 58 / abstract 199 / nums 712 / sha `45a20a09…`).
- **Ausführbefehl:** GEMS-Einreichung (AGU-Portal, Zugang über
  `agupubs.onlinelibrary.wiley.com/journal/15427390`) → Space Weather;
  ESSOAr-Preprint parallel.
- **Wort erwartet:** „gic einreichen".
