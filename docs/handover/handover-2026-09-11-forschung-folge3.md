<!--
  title: Handover — Forschung-Folge III (Stand 2026-09-11)
  session: Forschung-Folge III
  class: handover
  date: 2026-09-11
  sha256: f2f8a496e50f2802cc7543d7950990021948af259550e454e222624b0cf01806
  status: live
-->
# Handover — Forschung-Folge III (2026-09-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Analyse

- gic-p-Wert — der CDN-Fetch-Fix steht auf main (ee76a66); bz-retro-probe neu
  dispatcht (34618724899). Folge: p-Wert nach Landung verifizieren → Papier →
  Wing/Viljanen.
- Flut-Satellit — robuste Flut-/Narbenfläche aus S1.

## Nadeln

- Ⅲ TIAW vs Nanoflares — Register-Lücke geschlossen (38 url-Lines
  `jsoc.stanford.edu`: aia2013/2015-Volljahr + 36 Monats-Bins, committet);
  Matrix trägt 2013/2015. Offen: `aia2014_fullyear.bin` lokal ohne CDN-Asset
  (Manifestations-Duty); aia-ladder-Probe dispatcht.
- Ⅳ LAIC — CSES, TEC retro pre-2024, Instrument A, KDE-h (gemessen: scheduled).
- Ⅴ LSST-Live-Scan — Probe gebaut, kein Workflow; Positivkontrolle (RR-Lyrae/
  EB-Kegel) + IR-Exzess-Achse 10–60 μm.
- Ⅷ Dunkler Fluss — Haufen-Kanäle benennen, einbauen, Workflow.
- Ⅸ/Ⅹ FRB / Kugelblitz — FRB-Probe gebaut, `frb_chime_cat1.json` unregistriert;
  Kugelblitz-Kanal-Lage ungebaut.
- Ⅺ Placebo — ungebaut (kein EEG-Probe; Paar-EEG, fam-Schwelle, Nullkontrolle,
  bedingte TE).
- Ⅻ Urknall — gebaut-unbemessen; Workflow + Reihen-Paarung Winkelserie×z-Reihe.
- ⅩⅢ 48er-Zensus — gebaut-unbemessen; 48-Ziel-Zensus (18 Non-Detections) +
  XUV-Zeugen dispatcht.

## Galileo-Floor

- Volle CK-Ernte jenseits der vier Tage — `ck_daf_probe` auf den vollen Satz
  erweitern.
- Rausch-Kurve TRK-2-25/2-18 (~6,5 GB) — ungebaut.

## Weberin

- INPOP `.dat` vs `testpo` — Probe gebaut (`inpop_testpo_probe`), kein Workflow;
  Workflow (testpo + INPOP SPK, `--ci-mode`) + dispatcht.
- Raumsonden-Doppler Zweitlinie (VLBI-Winkel + Range) — VLBI-Winkel-Probe
  ungebaut; PRIDE/EVN-Datensatz lokalisieren.

## TE

- n=1000-Gate-Batterie — Punkte 17–20 gelandet (Run 34603020727, SHA c0a54c1),
  alle 20 Punkte im Sheet; 17–20 (n1000 block/shift × binned/ksg) tragen GATE
  FAIL: FPR-Anstieg 2.86–6.90pp bei a=0.9/D_Z=4 übersteigt das 2pp-Kriterium.
  Gemessen; der Umgang (Betriebspunkt vs. Kriterium) ist offen.

## Tiefenphasen

- sP-Δ-Faltung — gebaut-unbemessen (`depth_phase_probe`).
- sP-Beine der pP-übersprungenen Stationen — `depth_phase_fleet_probe`.
- Externe Tiefen-Referenz (TauP/KEB95) — ungebaut.
- Head-Wave-Lücke 410/660 — P-Triplikations-Gate gegen TauP, ungebaut.
- Quell-Strahlungsterm (CMT) — ungebaut.
- W-Phase-CMT als M9-Nachfolger — ungebaut.
- Stromboli als Vulkan-Lehrer — ungebaut.
- Eikonal gegen ETOPO1 — dispatcht (34603024168); Report trägt 3 Zeilen, kein
  Feld, `decode note: TypeZ`. Folge: Feld-Ausgabe des Probes verifizieren;
  Nachfolger: Vollkugel-Löser, Fast Marching.
- Die Erde als Sender (Tonga 2022) — ungebaut.
