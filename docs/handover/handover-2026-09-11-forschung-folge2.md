<!--
  title: Handover — Forschung-Folge II (Stand 2026-09-11)
  session: Forschung-Folge II
  class: handover
  date: 2026-09-11
  sha256: 50f8a4beca00da97197580780946698f00e840a1671c995127dcda5cfb0c7f8f
  status: live
-->
# Handover — Forschung-Folge II (2026-09-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Analyse

- gic-p-Wert — der bz-retro-Lauf (34587298538) landete „success", trägt aber
  `common hourly window: absent` (Bz 0/Speed 0/Density 0): `load_omni2` las nur
  die lokale Cache, nie das CDN-Asset. Fix steht in `bz_retro_probe.rs`
  (CDN-Fetch bei read-/parse-void, uncommitted). Folge: Fix committen + pushen
  → bz-retro-probe neu dispatcht → p-Wert landet → Papier → Wing/Viljanen.
- Flut-Satellit — robuste Flut-/Narbenfläche aus S1.

## Nadeln

- Ⅲ TIAW vs Nanoflares — 613er-Satz fehlt der `aia_ladder_probe` (Matrix nur
  2013/2015); Register-Lücke (keine url-Line für die aia-Volljahr-Bins).
  Folge: 613er-Satz ernten, Matrix-Asset, registrieren, dispatcht.
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

- Volle CK-Ernte jenseits der vier Tage — `gll-ck-cdn.yml` manifestiert nur
  1990-CK + 8 Rotor-CKs; `ck_daf_probe` auf den vollen Satz erweitern.
- Rausch-Kurve TRK-2-25/2-18 (~6,5 GB) — ungebaut.

## Weberin

- INPOP `.dat` vs `testpo` — Probe gebaut (`inpop_testpo_probe`, 2b8e9ed), kein
  Workflow; Workflow (testpo + INPOP SPK, `--ci-mode`) + dispatcht.
- Raumsonden-Doppler Zweitlinie (VLBI-Winkel + Range) — VLBI-Winkel-Probe
  ungebaut; PRIDE/EVN-Datensatz lokalisieren.
- Neptun-Planetenzentrum — Register steht (23 url-Lines `www.geoazur.fr` APDB in
  `phi/sources.φ`, uncommitted). Residual: `neptune_apparent_chain_probe` liest
  APDB nur lokal (kein CDN-Fallback).

## TE

- n=1000-Gate-Batterie — Punkte 17–20 wurden nie gemessen: sweep #2
  (34588877733) lief auf SHA fe1f06e, vor 5688e3f (der die Punkte 17–20 und den
  Benchmark-Bin selbst änderte); das Sheet trägt nur 01–16. Re-dispatcht
  (34603020727) auf main. Folge: Punkte 17–20 nach Landung verifizieren.

## Tiefenphasen

- sP-Δ-Faltung — gebaut-unbemessen (`depth_phase_probe`).
- sP-Beine der pP-übersprungenen Stationen — `depth_phase_fleet_probe`.
- Externe Tiefen-Referenz (TauP/KEB95) — ungebaut.
- Head-Wave-Lücke 410/660 — P-Triplikations-Gate gegen TauP, ungebaut.
- Quell-Strahlungsterm (CMT) — ungebaut.
- W-Phase-CMT als M9-Nachfolger — ungebaut.
- Stromboli als Vulkan-Lehrer — ungebaut.
- Eikonal gegen ETOPO1 — dispatcht (34603024168); Artefakt verifizieren;
  Nachfolger: Vollkugel-Löser, Fast Marching.
- Die Erde als Sender (Tonga 2022) — ungebaut.
