<!--
  title: Handover — Forschung-Folge VI (Stand 2026-09-12)
  session: Forschung-Folge VI
  class: handover
  date: 2026-09-12
  sha256: 4653cb608b4d26ce5fe9548c604bad98e102670037b3ce84f208a197173dff0f
  status: live
-->
# Handover — Forschung-Folge VI (2026-09-12)

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

Folge V abgearbeitet. Der Baum trägt fremde uncommittete Arbeit einer aktiven
Parallel-Session (src/archivar/relay.rs + main_flow.rs, static/*,
phi/pipeline/catalog/tap_index_{osa,ssa,vsa,wsa}.φ, docs/specs/*, bau-Handover)
— unberührt. Gepusht wird erst, wenn der Baum ruhig ist und das Wort kommt.

## Analyse (Flut-Satellit)

- Robustheits-Gates, Oberflächen-Artefakt-Format und Kalibrier-Gate gebaut.
  Offen: Incidence-Band (inc) — der per-Pixel-Einfallswinkel liegt in der
  SAFE-Kalibrier-XML, die s1_sar_compiler nicht erntet (absent); DEM-Terrain/
  Layover — keine gerasterte Land-DEM registriert (pending).

## Nadeln

- Ⅰ Jeans-Residuum bis Gaia DR4 (Wiedervorlage 2026-12-02).
- Ⅱ JUICE-Flyby 28./29.9. (Wiedervorlage 2026-09-28) · Europa Clipper 3.12.
  (Wiedervorlage 2026-12-03).
- gaia-dr4-iapetus (Wiedervorlage 2026-12-02).
- Ⅳ LAIC — (a) TEC retro pre-2024 (codg*.Z + lzw + ionex) und (c) KDE-h
  (transfer_entropy_lag_h) gebaut; (b) Instrument A: globaler USGS-FDSN-
  Event-Rate-Stack gebaut — INTERMAGNET-F zugangsblockiert (Account-Pflicht),
  anonymer Dst-Index (Kyoto, HTTP 200) nicht verdrahtet.
- Ⅴ LSST — IR-Exzess-Achse gebaut (far-IR im nadel_gate + infrared_excess_compiler
  --far-ir; akari_irc registriert). Offen: die vier far-IR-Schwellen (W3−W4,
  IRAS F25/F12, MSX E/C, akari 18/9) ohne belastbares Zitat — pending, die Achse
  liefert bis dahin Pending/NoSource; Archivar-Leser für ir_{iras,msx,akari}.bin
  (pending); CDN-Workflow/Asset (pending, wartet auf den Leser).
- Ⅷ Dunkler Fluss — cluster-tap-Assets gelandet (mcxc/psz2/abell HTTP 200);
  dark-flow re-dispatcht (Run 34707203526), Resultat pending.
- Ⅺ Placebo — physionet trägt gepaarte EEG-Bedingungen (eegmmidb u. a.), kein
  Placebo-spezifisches Paar (sham-vs-aktiv) gemessen — Substrat-Wahl offen.

## Galileo-Floor

- Rausch-Kurve — galileo_odf.bin Register-Zeile gebaut; Compiler gebaut. Die
  Curve-Verifikation hängt am offenen TRK-2-18 ODF Sub-Format (Format-1 vs
  Format-2, ungemessen) — unverifiziert bis zur Format-Klärung.

## Weberin

- PSA TNF — Register-Zeile gebaut (format reference, at mars). VLBI-Winkel-Probe
  bleibt pending (PRIDE ΔDOR not-published, gemessen 2026-09-12).

## TE

- Phase-Null — sweep gemessen: 22 Punkte, 12 PASS / 10 FAIL (Punkt 21
  phase-binned FAIL, Punkt 22 ksg PASS); Block/Shift-n-Grenze im Sheet.

## Tiefenphasen

- Externe Referenz — iasp91 + headwave_gate_probe gebaut; P-Slowness-Handoff
  verdrahtet (takeoff_angle_deg/source_legs).
- Quell-Strahlungsterm (CMT) — NDK-Parser + M-Tensor + R_P in
  depth_phase_polarity_probe verdrahtet. Offen: Kalibrier-Gate (die sechs
  Stationsazimute fehlen im Register), Probe-Lauf (jan76_dec25.ndk ~24 MB,
  schwerer Fetch → CI), Archivar-Verdrahtung parser-def ndk (Parser lebt in
  measure-lib).
- W-Phase-CMT (M9-Nachfolger) — Bau-Linie: USGS FDSN moment-tensor/mww
  (anonym 200) statt des M9.1-Pickers, Zentroid-Inversion; GCMT trägt kein
  W-Phase-Produkt (USGS/PTWC ist der Träger).
- Stromboli — INGV-Route + stromboli_station_term_probe gebaut; der Lauf maß
  kein kohärentes Kratersignal (0 honored). Offen: Wiring zur Tiefenphasen-
  Stationskorrektur (baut auf die gemessene Null auf).
- Die Erde als Sender (Tonga 2022) — Kreuz-Abgleich-Probe gebaut (ersetzt die
  Roh-Wellenform für Ankunft/Rückazimut); die Roh-Druckwellenform bleibt
  blockiert (CTBTO-vDEC 403), nicht descoped.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene
Abschluss-Check. Der Baum trägt fremde uncommittete Arbeit; gepusht wird erst,
wenn er ruhig ist.
