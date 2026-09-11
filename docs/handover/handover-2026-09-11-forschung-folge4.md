<!--
  title: Handover — Forschung-Folge IV (Stand 2026-09-11)
  session: Forschung-Folge IV
  class: handover
  date: 2026-09-11
  sha256: aae278fae3f71a1aa894643a0e1a81871041bac5031132a89ac8e6c891f7ea13
  status: live
-->
# Handover — Forschung-Folge IV (2026-09-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Sitzungs-Stand (kommende Session zuerst)

Der Baum war beim Abschluss NICHT ruhig: fremde uncommittete Arbeit liegt im
Baum (docs/reference→surveys-Umzug der Seed/Census-JSONs, hinet/win32,
pioneer_atdf, arxiv, ernte-folge3-Archiv). Mein Commit trägt nur die 16
eigenen Dateien (Code-Fixes, TE-Phase-Null, 9 Workflows, sources.φ).
Push + alle Workflow-Dispatches warten auf den ruhigen Baum. Die gebauten,
aber noch nicht dispatchten Workflows: aia-cdn (2014-fullyear), frb-chime-cdn,
gll-ck-full-spin, dark-flow-probe, disequilibrium-census, depth-phase,
depth-phase-fleet, inpop-testpo, te-operating-point-sweep (21/22).

## Analyse

- gic p-Wert — Join-Bug gefixt (omni2 TDB→unix via J2000_UNIX_OFFSET, wie
  nobel_probe_bz); bz-retro lief „common window absent". Re-Dispatch + p-Wert
  verifizieren → Papier → Wing/Viljanen. paper-check-Failure auf HEAD
  (82c2692) ungeklärt.
- Flut-Satellit — robuste Flut-/Narbenfläche aus S1.

## Nadeln

- Ⅳ LAIC — CSES, TEC retro pre-2024, Instrument A, KDE-h.
- Ⅴ LSST-Live-Scan — Positivkontrolle (RR-Lyrae/EB-Kegel) + IR-Exzess-Achse
  10–60 μm; Workflow fehlt.
- Ⅷ Dunkler Fluss — Kanäle benannt + eingebaut (MCXC/PSZ2/Abell in
  dark_flow_probe); Register-Zeilen der drei TAP-Kataloge + Manifestation
  offen → Kanäle bleiben pending bis die Assets da sind.
- Ⅸ/Ⅹ Kugelblitz — Kugelblitz-Kanal-Lage ungebaut.
- Ⅺ Placebo — ungebaut (kein EEG-Probe; Paar-EEG, fam-Schwelle, Nullkontrolle,
  bedingte TE).
- Ⅻ Urknall — gebaut-unbemessen; Workflow + Reihen-Paarung Winkelserie×z-Reihe.
- ⅩⅢ 48er-Zensus — disequilibrium-census.yml gebaut; die Census-Seed-Pfade
  (docs/reference/*.json) werden von einer fremden Session nach docs/surveys/
  verlegt → Pfade im Workflow nachziehen, sobald der Umzug landet.

## Galileo-Floor

- CK-Volll-Ernte — `--harvest` gebaut (570 rtr-Produkte, gll-ck-full-spin.yml);
  Manifestation/Register offen: format `ck` vs `spk`-Wiederverwendung,
  gll-ck-cdn.yml auf den vollen Satz, rtr/-Index + mk00062a.tsc.
- Rausch-Kurve TRK-2-25/2-18 (~6,5 GB) — ungebaut.

## Weberin

- INPOP testpo — inpop-testpo.yml gebaut; Dispatch offen.
- VLBI-Winkel — PRIDE ΔDOR not-published (EVN-Rohdaten login-gated); PSA
  MaRS/VeRa closed-loop TNF (Doppler+Range) live (HTTP 200, Beleg
  M15TNF0L1A_TNF_032000724_00.DAT) — PSA-Quelle unregistriert; VLBI-Winkel-Probe
  bleibt pending.

## TE

- Phase-Null — Beschluss gebaut (TeNull::Phase + sweep 21/22 + Gate-Test
  gate_fpr_autocorrelation_phase_null_binned_n_1000); Sweep-Dispatch misst
  21/22. Block/Shift-n-Grenze bleibt gemessen im Sheet.

## Tiefenphasen

- sP-Δ-Faltung / sP-Beine — depth-phase.yml + depth-phase-fleet.yml gebaut;
  Dispatch offen.
- Externe Tiefen-Referenz (TauP/KEB95) — ungebaut.
- Head-Wave-Lücke 410/660 (P-Triplikations-Gate gegen TauP) — ungebaut.
- Quell-Strahlungsterm (CMT) — ungebaut.
- W-Phase-CMT als M9-Nachfolger — ungebaut.
- Stromboli als Vulkan-Lehrer — ungebaut.
- Eikonal — TypeZ-Fix committet (Short/Int/Float/Double, scale/offset default
  1/0); Re-Dispatch misst das Feld.
- Die Erde als Sender (Tonga 2022) — ungebaut.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators und der gemessene
Abschluss-Check. Push + Dispatches warten auf den ruhigen Baum (siehe
Sitzungs-Stand).
