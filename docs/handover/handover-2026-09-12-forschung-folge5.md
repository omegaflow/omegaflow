<!--
  title: Handover — Forschung-Folge V (Stand 2026-09-12)
  session: Forschung-Folge V
  class: handover
  date: 2026-09-12
  sha256: 0746d6eafd7024a8a118ee429e5a4d7730560124617e113005bf58c0729519c8
  status: live
-->
# Handover — Forschung-Folge V (2026-09-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Dies ist die Forschung-Linie: hier steht nur, was diese Linie autonom trägt —
Tasks, die nicht autonom hier erfolgen können, sind in das Handover ihrer Linie
überführt. Wiedervorlage gilt nur für Termine; „Warten auf Rückmeldung" trägt
kein Datum.

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
- Ⅳ LAIC — drei Bau-Linien gemessen (CSES descoped): (a) TEC retro pre-2024 —
  `codg*.Z` vom ESA-GSSC-FTP (anonym, 226) laden, `omegaflow::lzw::uncompress_z`
  + `omegaflow::ionex`-Parser (beide gebaut) in `laic_probe` verdrahten;
  (b) Instrument A — globaler Event-Rate-Stack (USGS-FDSN + INTERMAGNET-F) in
  `laic_probe`; (c) KDE-h — `transfer_entropy_lag_h` (gebaut, te.rs:142) statt
  `kde_scale` verdrahten.
- Ⅴ LSST — IR-Exzess-Achse 10–60 μm: keine Quelle fehlt (IRAS FSC 12/25/60,
  MSX 12/14.7/21.3, AllWISE W3/W4 liegen anonym in IRSA-TAP); offen ist der
  Achsen-Compiler — `akari_irc` (flux09/18) registrieren, `infrared_excess_compiler`
  über W3−W4 hinaus erweitern, Workflow + CDN-Asset, Achse in `nadel_gate.rs`
  neben den W1-W2-Keil hängen.
- Ⅷ Dunkler Fluss — cluster-tap-cdn gebaut + dispatcht (mcxc/psz2/abell →
  CDN tapvizier.cds.unistra.fr); die Kanäle bleiben pending bis die Assets
  landen und dark-flow re-dispatcht ist.
- Ⅸ FRB — Bau-Linie: `frb_chime_cat1.json` (sources.φ Z.8135) trägt nur eine
  nackte url-Zeile; Feld-Semantik füllen (`at`, `cmap`, `field dm`/`freq`/
  `bin_width`) — Probe `frb_blatt_probe` steht.
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
- Quell-Strahlungsterm (CMT) — Bau-Linie: GCMT-NDK-Parser (`blocked_sources.φ`
  „parser-def ndk", `jan76_dec25.ndk` anonym 200) + CMT-Quellterm-Probe
  (strike/dip/rake → M-Tensor → P-Abgangs-Vorzeichen), verdrahtet in
  `depth_phase_polarity_probe.rs` (löst die pP-Vorzeichen-Mischung).
- W-Phase-CMT (M9-Nachfolger) — Bau-Linie: USGS FDSN `moment-tensor`/`mww`
  (anonym 200) statt des M9.1-Pickers, Zentroid-Inversion; GCMT trägt kein
  W-Phase-Produkt (USGS/PTWC ist der Träger).
- Stromboli als Vulkan-Lehrer — Bau-Linie: INGV FDSN (`webservices.ingv.it`,
  anonym 200, HHZ/HNZ) + INGV-Route im `fdsn_waveform_compiler` +
  `stromboli_station_term_probe` (Wiederholung aus fester Kratersektion).
- Die Erde als Sender (Tonga 2022) — Bau-Linie: `bgr_infrasound_compiler
  --year 2022` (BGR-Detektion, Compiler gebaut) + Kreuz-Abgleich-Probe
  (Ankunftszeit/Rückazimut gegen Lamb-Laufzeit + Wasser-Startzeit); die Roh-
  Druckwellenform bleibt blockiert (CTBTO-vDEC 403), nicht descoped.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene
Abschluss-Check. Eigene Arbeit committet + gepusht (99cf88a, HEAD == origin/main);
fremde uncommittete Arbeit im Baum bleibt unberührt.
