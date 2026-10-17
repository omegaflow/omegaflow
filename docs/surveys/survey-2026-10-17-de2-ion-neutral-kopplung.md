<!--
  title: Survey — DE-2 Ion↔Neutral-Kopplung (erste Messung)
  class: survey
  date: 2026-10-17
  sha256: dc94b48807f45be3e040bb67df6b0771e4fb59ff2b7b8a46eef2f4a92f2395b5
  status: live
  see-also: docs/surveys/survey-2026-10-17-haertefaelle-offene-routen.md
-->
# Survey — DE-2 Ion↔Neutral-Kopplung (erste Messung, 2026-10-17)

**Frage.** Trägt der offene DE-2-Ion↔Neutral-Satz eine Kopplung zwischen Ionendrift und Neutralwind?

**Datensatz.** DE-2 **UA_16s** (`combined_plasma_neutrals_ua`), 29 Tage 1981 (DOY 218–245),
CDAWeb-Direktlink `https://cdaweb.gsfc.nasa.gov/pub/data/de/de2/combined_plasma_neutrals_ua/ua_16s_ascii/<YYYYDDD>_de2_ua_16s_v01.asc`.
Ein Record = 16 s, trägt zeitsynchron **WATS NVE** (ostwärtiger Neutralwind) und **RPA IVE** (ostwärtige
Ionendrift), beide auf **einem** Fahrzeug. Werkzeug: **awk** (`LC_ALL=C` — die deutsche Locale parst
`1054.2` sonst als `1054`).

## Messung (Korrelation, 4244 gemeinsame Records)

| Paar | n | r |
|---|---|---|
| eastward WATS NVE ↔ RPA IVE | 4244 | **+0,256** |
| — Tag (06–18 h LST) | 899 | **+0,358** |
| — Nacht | 3345 | +0,185 |
| upward WATS NVU ↔ RPA IVU | 136 | +0,286 |

Mittel: NVE **−39,6 m/s** (westwärts), IVE **+330,9 m/s** (ostwärts) — die Ionen sind ~8× schneller;
die **Fluktuationen** beider ostwärts-Komponenten laufen **positiv zusammen**, tagsüber stärker.

## Transfer-Entropie (Haus-Maschine)

`te_pair_probe --a nve.txt --b ive.txt --lags 1,4,16 --surrogat 100` (Rust, `omegaflow::te`,
`transfer_entropy_lag` + `surrogate_threshold_lag`), n=4244, 100 Surrogate:

| lag | TE(NVE→IVE) | Schwelle | TE(IVE→NVE) | Schwelle | Verdikt |
|---|---|---|---|---|---|
| 1 | 7,43e-2 | 2,32e-2 | 6,30e-2 | 2,70e-2 | beide |
| 4 | 8,73e-2 | 3,79e-2 | **1,14e-1** | 6,31e-2 | beide, IVE→NVE stärker |
| 16 | 8,05e-2 | 3,54e-2 | **1,10e-1** | 7,79e-2 | beide, IVE→NVE stärker |

**Beide Richtungen** über der Surrogat-Schwelle (mean + 2σ, phasen-randomisiert) bei **jedem** Lag.
Die **Richtung dreht** mit dem Lag: bei lag 1 ist NVE→IVE etwas stärker, bei lags 4 und 16 **IVE→NVE**.
Das ist **kein aufgelöster Kausalpfeil** — zwei signifikante Richtungen mit lag-abhängigem Vorzeichen
sind ebenso mit einem **gemeinsamen Treiber** (Konvektion/Joule) verträglich wie mit wechselseitigem
Impulsaustausch. A=A: die TE misst **Abhängigkeit**, nicht Reibung.

## Vorlauf (Prior Art — nicht neu erfinden)

Die DE-2-IDM+WATS-Ion↔Neutral-Kopplung ist eine **publizierte Linie**:
- **Coley, Heelis & Spencer** — „Comparison of low-latitude ion and neutral zonal drifts using DE 2
  data", JGR `10.1029/93JA02205` — **genau IDM + WATS auf DE-2**, statistischer Ion↔Neutral-Vergleich.
- Folge auf C/NOFS/ROCSAT-1: Pacheco, Heelis & Su `10.1029/2011JA016786` · Stoneback et al.
  `10.1029/2011JA016712` · Coley & Heelis `10.1029/2011JA017301`.
- Modell/Kontext: St.-Maurice & Schunk `10.1029/ja086ia13p11299` · Ferdousi et al. `10.1029/2018JA026193`.

**Konsequenz (A=A):** der Befund ist **nicht neu** — die Daten und die Analyse sind seit 1994 publiziert.
Der Wert ist die **offene Route** auf dieselben Daten (CDAWeb/NSSDCA „Ready for Electronic Access") für
das Haus, **nicht** die Deutung.

## Grenze (A=A)

- Die UA-Daten sind **entlang der Bahn** gepaart (unregelmäßig) — **keine** kontrollierte
  common-volume-Zeitreihe; die Korrelation ist ein **erstes Signal**, kein Kopplungsverdikt.
- Die **dichtere** Paarung (WATS 2 s + IDM 250 ms) ist noch nicht gebaut: WATS 2 s ist **fixed-width**
  (Felder kleben: `321`, `146189`), das IDM ist **mehrzeilig** und im **s/c-Rahmen** (Richtungs-Flags
  statt geographic east) → braucht einen eigenen **Rust-Parser**, keine Shell-Zeile.
- Der IDM liefert **schnellere** Ionendrift (250 ms) als das RPA (16 s) — die hier gezeigte Reihe nutzt
  das RPA; das ist benannt, nicht ersetzt.

## Nächster Schritt

1. Rust-Parser für WATS-2s (fixed-width) + IDM (mehrzeilig, s/c) → dichtere Paarung.
2. **Kausalität prüfen:** die lag-abhängige Richtungsdrehung gegen einen **gemeinsamen Treiber**
   (Konvektion/Joule) testen — sonst bleibt „bidirektional" mehrdeutig.
3. Erst danach eine Aussage zur ion-neutral friction — Korrelation + TE allein tragen sie nicht.
