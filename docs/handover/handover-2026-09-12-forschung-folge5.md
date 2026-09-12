<!--
  title: Handover — Forschung-Folge V (Stand 2026-09-12)
  session: Forschung-Folge V
  class: handover
  date: 2026-09-12
  sha256: 81bc1641a62a67512ea19f240c63d0767396f6f55acf6654e2610f6650d1426b
  status: live
-->
# Handover — Forschung-Folge V (2026-09-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Sitzungs-Stand

Mein Commit 99cf88a trägt nur die 16 eigenen Dateien und ist gepusht
(HEAD == origin/main). Der Baum trägt daneben fremde uncommittete Arbeit einer
aktiven Parallel-Session (tap_index_*.φ-Kataloge, bau9→archiv +
ernte-folge5-Löschung, eve_compiler/regtap_census/eso-harps-rvcat) — unberührt.
Der ci-check-Failure auf HEAD (Run 34650668589) bleibt ungeklärt; ein neuerer
ci-check-Lauf misst die Behoben-Frage.

## Analyse

- Flut-Satellit — Robustheits-Gates gebaut (Vor-Rauschfloor, Absolut-Rückstreu,
  Speckle-Nachbarschaft). Offen: Incidence-Band (inc), DEM-Terrain/Layover,
  Oberflächen-Artefakt-Format, Kalibrier-Gate.

## Nadeln

- Ⅰ Jeans-Residuum bis Gaia DR4 (Wiedervorlage 2026-12-02).
- Ⅱ JUICE-Flyby 28./29.9. (Wiedervorlage 2026-09-28) · Europa Clipper 3.12.
  (Wiedervorlage 2026-12-03).
- gaia-dr4-iapetus (Wiedervorlage 2026-12-02).
- Ⅳ LAIC — CSES, TEC retro pre-2024, Instrument A, KDE-h — ungebaut.
- Ⅴ LSST — Workflow gebaut + dispatcht; IR-Exzess-Achse 10–60 μm ungebaut
  (nur der AllWISE W1-W2-Keil 3.4/4.6 μm existiert).
- Ⅷ Dunkler Fluss — cluster-tap-cdn gebaut + dispatcht (mcxc/psz2/abell →
  CDN tapvizier.cds.unistra.fr); die Kanäle bleiben pending bis die Assets
  landen und dark-flow re-dispatcht ist.
- Ⅸ/Ⅹ Kugelblitz — ungebaut.
- Ⅺ Placebo — placebo_pair_eeg_probe gebaut; EEG-Datensubstrat pending
  (physionet trägt nur ECG/mitdb, kein Paar-EEG).
- Ⅻ Urknall — bigbang-echo.yml gebaut + dispatcht (Winkelserie×z-Paarung
  eingebaut); Resultat pending.

## Galileo-Floor

- CK-Volll-Ernte — gll_ck_manifestor gebaut (570 Produkte + mk00062a.tsc,
  DAF-BIG-IEEE-Gate statt sha256); gll-ck-cdn.yml um gll-1990-rtr-full erweitert;
  dispatcht (34684534853), Verifikation pending.
- Rausch-Kurve — galileo-trk-noise.yml + galileo_odf_compiler +
  galileo_trk_noise_curve gebaut + dispatcht; offen: TRK-2-18 ODF Sub-Format
  (Format-1 vs Format-2) ungemessen; galileo_odf.bin Register-Zeile
  (Feld-Semantik) pending.

## Weberin

- PSA TNF — Register-Zeile gebaut (format reference, at mars). VLBI-Winkel-Probe
  bleibt pending (PRIDE ΔDOR not-published).

## TE

- Phase-Null — te-operating-point-sweep (22 Punkte) dispatcht, läuft; die
  21/22-Verifikation (sweep-sheet) bleibt pending. Block/Shift-n-Grenze gemessen
  im Sheet.

## Tiefenphasen

- Externe Referenz — iasp91.rs gebaut (KEB95-Mantel, P-Triplikation 410:
  Δ 14.08–21.43°, 660: Δ 17.67–28.09°) + headwave_gate_probe. Wiring zu den
  depth-phase-Probes pending (kein gemessener P-Slowness-Handoff).
- Quell-Strahlungsterm (CMT) — ungebaut.
- W-Phase-CMT als M9-Nachfolger — ungebaut.
- Stromboli als Vulkan-Lehrer — ungebaut.
- Die Erde als Sender (Tonga 2022) — ungebaut.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene
Abschluss-Check. Eigene Arbeit committet + gepusht (99cf88a, HEAD == origin/main);
fremde uncommittete Arbeit im Baum bleibt unberührt.
