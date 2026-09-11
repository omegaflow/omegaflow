<!--
  title: Handover — Forschung-Folge (Stand 2026-09-11)
  session: Forschung-Folge
  class: handover
  date: 2026-09-11
  sha256: 0eb8ba2a69324b661bd40558e3b9aa8f33979bb977648388703302750e9c90eb
  status: archived
-->
# Handover — Forschung-Folge (2026-09-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Analyse

- gic-p-wert — p-Wert läuft in CI `bz-retro-probe.yml` (Run 34587298538,
  Matrix ABK-2024 / ABK-2025 / SOD-2024); Artefakte `bz-retro-<point>.txt`;
  p-Wert → Papier nach Landung, dann Wing/Viljanen.
- Flut-Satellit — robuste Flut-/Narbenfläche aus S1.

## Nadeln

- Ⅲ TIAW vs Nanoflares (613-Ereignis-Satz, `aia_ladder_probe`).
- Ⅳ LAIC — CSES, TEC retro pre-2024, Instrument A, KDE-h.
- Ⅴ LSST-Live-Scan — kein laufender Prozess (pgrep leer, 2026-09-11); die Probe ist ein One-Shot-CLI, kein Loop. Coverage-Register letzter Stand 2026-09-05 (3 Fink-Kegel void, 1 ANTARES-Pass, 0 Kandidaten), seit 2026-09-09 nichts Neues. Offen: Positivkontrolle (RR-Lyrae/EB-Mehrband-Kegel) + IR-Exzess-Achse 10–60 μm.
- Ⅷ Dunkler Fluss — Haufen-Kanäle benennen.
- Ⅸ/Ⅹ FRB / Kugelblitz — Kanal-Lage.
- Ⅺ Placebo — Paar-EEG, fam-Schwelle, Nullkontrolle, bedingte TE.
- Ⅻ Urknall — Reihen-Paarung Winkelserie×z-Reihe.
- ⅩⅢ Voller 48er-Zensus (18 Non-Detections); Photochemie-Re-Erklärung.

## Galileo-Floor

- All-Spin-Bus-CK Frame −77000 — EGA-1-Fenster gemessen (`ck_daf_probe.rs`,
  Papier); die volle CK-Ernte jenseits der vier Tage bleibt pending.
- Empirische Rausch-Kurve aus TRK-2-25/2-18 (~6,5 GB Download).

## Weberin

- INPOP `.dat` gegen `testpo` (oder SPK-Weg) — zweite Abstammung.
- Raumsonden-Doppler zweite Linie (VLBI-Winkel + Range).
- Neptun-Planetenzentrum-Tabelle (Astrometrie-Kopplung) — Source-Port.

## TE

- n=1000-Gate-Batterie — Ergebnis verifizieren: die Matrix-Punkte 17–20
  (`--gate-n 1000 --block 10`, `te-operating-point-sweep.yml`) sind gebaut; die
  Gate-Messung läuft in der CI-Welle (`te-n1000-shift.yml`,
  `te-operating-point-sweep.yml`), nicht lokal — dispatcht 2026-09-11
  (`te-operating-point-sweep #2`, `te-n1000-shift #2`).

## Tiefenphasen

- sP-Δ-Faltung — ungemessen (Register-Duty, nicht 0.0).
- sP-Beine der pP-übersprungenen Stationen.
- Externe Tiefen-Referenz (TauP/KEB95).
- Head-Wave-Lücke 410/660 — P-Triplikations-Gate gegen TauP.
- Quell-Strahlungsterm (CMT).
- W-Phase-CMT als M9-Nachfolger-Atom.
- Stromboli als Vulkan-Lehrer.
- Eikonal-Lauf gegen das echte Gitter — dispatch + Ergebnis verifizieren: der
  Löser ist gebaut (`tools/measure/src/eikonal.rs` + `tohoku_eikonal_probe` +
  `.github/workflows/eikonal-tohoku.yml`); der Lauf gegen ETOPO1 ist CI-Duty.
  Benannte Folgen: Vollkugel-Löser, Fast Marching.
- Die Erde als Sender — Kreuzbereichs-Kalibrierung (Tonga 2022).
