<!--
  title: Survey — DE-2 Ion↔Neutral-Kopplung (erste Messung)
  class: survey
  date: 2026-10-17
  sha256: f0cf6f0ef1d1686a8e93099bb8d87a388d48050c2cec20660c5a0cb8184886ad
  status: live
  see-also: docs/surveys/survey-2026-10-17-haertefaelle-offene-routen.md
-->
# Survey — DE-2 Ion↔Neutral-Kopplung (erste Messung, 2026-10-17)

**Anlass (Operator-Wort):** der C/NOFS-NWM-Ersatz (DE-2) soll **ausgewertet** werden — findet die
Kopplung zwischen Ionendrift und Neutralwind sich in den Daten?

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
| 1 | 7,43e-2 | 2,32e-2 | 6,30e-2 | 2,70e-2 | **beide** |

Beide Richtungen **über** der Surrogat-Schwelle → **bidirektionale** Kopplung, NVE→IVE etwas stärker.
Lags 4 und 16 rechnen noch (`~/projects/omegaflow-work/de2/te_out.txt`). Kein stilles `pending`.

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
2. TE-Verdikt eintragen (dieses Survey).
3. Erst danach eine Aussage zur ion-neutral friction — die Korrelation allein trägt sie nicht.
