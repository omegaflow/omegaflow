<!--
  title: Survey — DE-2 Ion↔Neutral-Kopplung (erste Messung)
  class: survey
  date: 2026-10-17
  sha256: 9a16d8da387966d95bcb2fd8851af5b97e3b5a9b42f02d0579a0d76afe1a701f
  status: done
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

## Roh-Parser WATS-2s + IDM-250ms (gebaut, 2026-10-18)

`src/archivar/de2_raw.rs` parst die beiden **NDADS-ASCII-Rohprodukte**, aus denen die
dichtere Paarung entsteht — ohne Shell, ohne `awk`:

- **WATS 2 s** (`n_T_v_2s_ascii/<yyyyddd>_de2_wats_2s_v01.asc`): Record = **zwei Zeilen**,
  **fixed-width**. Die Felder kleben (`311 32`, `6549765535`, `-0.26959E+07`), darum wird
  **spaltenweise** gelesen, nicht per Whitespace. Gelesen werden alle 25 Wörter:
  Datum/UT, Mode/Slot/Outin/Masse, Dichte, Tn/Tn_corr, V_s/c, Counts, Baffle-Zeiten,
  V_geo/V_geo_cor, Orbit, Höhe, Lat/Lon, LST/LMT, L, Inv.-Lat, SZA. Füllwerte
  (`-9.0`, `9999.0`) bleiben als Wert erhalten; `corrected_wind_ms()` liefert `None`
  statt einer erfundenen Zahl.
- **IDM 250 ms** (`vi_250ms_ascii/<yyyy>/<mm>/<yyyyddd>T<hhmm>_de2_idm_250ms_v01.asc`):
  **mehrzeilig** — Kopfzeile mit 9 fixed-width-Feldern (Datum, UT, Lat, Lon, Inv.-Lat,
  MLT, Höhe, s/c-Geschwindigkeit, **n Paare**), danach **6 Paare pro Zeile**
  (`I9` Zeit + `F11.2` Geschwindigkeit). Die beiden Nachkommastellen der Geschwindigkeit
  sind **keine Dezimalen**, sondern Flags: erste Ziffer = Qualität (0/1 gut),
  zweite = Sensorachse (gerade = s/c **z**, ~horizontal; ungerade = s/c **y**, ~vertikal).
  Der Parser trennt beides und liefert `velocity_ms`, `quality`, `axis`.
- `yyddd_unix_ms` bildet `yyddd` + Millisekunden des Tages auf Unix-Millisekunden ab;
  damit sind WATS- und IDM-Zeitachsen direkt paarbar.

**Format-Beleg (gemessen 2026-10-18, CDAWeb):**
`…/de2/neutral_gas_wats/format_description_ASCII.txt`,
`…/de2/neutral_gas_wats/NSSDC_processing_program.for` (Fortran-`FORMAT (1X,I5,I9,2I2,I3,…)`),
`…/de2/plasma_idm/format_description_ASCII.txt`.

**Test:** Fixtures `src/archivar/de2_fixtures/wats_2s_1981230.asc` (8 Records) und
`idm_250ms_1981227T0344.asc` (3 Records, 32 + 1 + 32 Paare) aus den CDAWeb-Dateien.
`cargo test --lib de2_raw` → **4 passed**; `cargo check` grün.

**Grenze:** gebaut ist der **Parser**, nicht die Paarung. Die 2-s/250-ms-Zeitreihe ist
noch nicht über den vollen Rohbestand gefahren; die unten gezeigten Zahlen bleiben die
**16-s-HAPI-Paarung**.

## Kausalitätstest (gemessen, 2026-10-18)

Gefahren mit dem vorhandenen `tools/measure/src/bin/de2_conditional_te.rs`
(`omegaflow::te`, bedingte TE, **ARX-Null**, 100 Surrogate, seed `0x5eedde2`):
`de2_conditional_te --csv <de2_ua16s_all.csv> --ae <wdc_ae.bin> --bin-seconds 300 --max-lag 3 --surr 100`.

**Eingang.** Volles `DE2_UA16S_ALL`-Produkt (HAPI, 62 MB,
sha256 `c128daa832b1fb14a749d019fff8bbc66f136702bd2783559191912355ac0067`) und
WDC-Kyoto `min_ae`-Bin (89 MB). Der `de2_compiler`-Pfad ist **live geprüft**: ein
Monatsfenster `1981-08-06..1981-08-31` liefert 13 676 Datenzeilen; die registrierte
Gesamt-CSV trägt denselben sha256 wie im Workflow gepinnt.

**Gitter.** DE-2 und AE auf **300-s-Bins** gemittelt, gemeinsame Bins:
**n = 23 202**, 3 735 Segmente, Median-Lücke 300 s, größte Lücke 783 000 s (9,06 d),
8 TE-Bins, Lags 300/600/900 s. Schwelle = Null-Mittel + 2σ.

| lag | Richtung | roh TE | roh Schwelle | roh | TE \| AE | Schwelle \| AE | bedingt |
|---|---|---|---|---|---|---|---|
| 300 s | NVE→IVE | 6,439e-3 | 1,248e-3 | **ja** | **6,724e-3** | 2,318e-3 | **ja** |
| 300 s | IVE→NVE | 4,642e-3 | 1,239e-3 | **ja** | 4,196e-3 | 2,577e-3 | **ja** |
| 600 s | NVE→IVE | 1,237e-3 | 1,042e-3 | **ja** | 2,446e-3 | 2,019e-3 | **ja** |
| 600 s | IVE→NVE | 0,917e-3 | 1,188e-3 | nein | 2,126e-3 | 2,510e-3 | nein |
| 900 s | NVE→IVE | 1,001e-3 | 1,179e-3 | nein | 2,239e-3 | 2,198e-3 | **ja** (knapp) |
| 900 s | IVE→NVE | 0,831e-3 | 1,161e-3 | nein | 2,122e-3 | 2,434e-3 | nein |

**Befund.** Auf dem 300-s-Gitter **dreht die Richtung nicht mehr**: NVE→IVE liegt bei
jedem Lag über der Schwelle, IVE→NVE nur bei 300 s. Das ist der **Gegensatz** zur
Record-TE oben (lag 4/16: IVE→NVE stärker) — die Drehung war eine Eigenschaft der
**ungebinnten, unregelmäßig entlang der Bahn gepaarten** Reihe, nicht der 300-s-Reihe.
**AE als gemeinsamer Treiber erklärt die Abhängigkeit nicht:** die Bedingung auf AE
senkt die TE nicht (300 s: 6,72e-3 bedingt vs. 6,44e-3 roh); bedingt auf AE bleibt
NVE→IVE bei allen drei Lags signifikant. Das ist **ein** getesteter Treiber (AE-Skalar), nicht der
vollständige Treiber (Joule-Heizung, E-Feld, Poynting); der bedingte Schätzer auf
Bins kann die TE zudem leicht aufblähen. Der **Kausalkanal bleibt `pending`** —
gemessen ist: die Abhängigkeit überlebt AE, sie ist nicht auf AE zurückführbar.

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
- Die **dichtere** Paarung (WATS 2 s + IDM 250 ms) ist **parsebar**, aber noch **nicht gepaart**:
  `src/archivar/de2_raw.rs` liest beide Rohformate samt s/c-Achsen-Flags; die Zeitreihe über den
  vollen Rohbestand und ihre TE stehen aus.
- Der IDM liefert **schnellere** Ionendrift (250 ms) als das RPA (16 s) — die hier gezeigten
  Messungen nutzen das RPA; das ist benannt, nicht ersetzt.
- Der Kausalitätstest prüft **einen** Treiber (AE, 300-s-Bins). Ein anderer gemeinsamer Treiber
  (Joule/E-Feld/Poynting) oder die Filterwirkung des Erwerbskanals bleibt offen; eine
  Impulsbilanz ist nicht gemessen.

## Nächster Schritt

0. **HAPI-Route gebaut** (`tools/harvest/src/bin/de2_compiler.rs`: `DE2_UA16S_ALL` mit
   `WATS_NVEneutralWind`/`WATS_NVUneutralWind` + `RPA_NIionDensity`/`IVE`/`IVN`) — die
   16-s-Paarung ist ohne Roh-Parser baubar. Der Roh-Parser bleibt nur für die dichtere
   2-s/250-ms-Paarung nötig.
1. **Gebaut:** Rust-Parser für WATS-2s (fixed-width) + IDM (mehrzeilig, s/c), Test grün.
   **Offen:** den vollen Rohbestand harvesten und die 2-s/250-ms-Paarung tatsächlich rechnen.
2. **Gemessen:** Kausalitätstest auf dem 16-s-Produkt, AE-bedingt (300-s-Bins). Die
   Abhängigkeit überlebt AE; die frühere Lag-Drehung verschwindet im 300-s-Gitter.
   **Offen:** auf Joule/E-Feld/Poynting konditionieren und die Lag-Drehstelle an
   `1/ν_in` aus Dichte/Komposition binden (vorhergesagt, nicht post hoc).
3. Erst danach eine Aussage zur ion-neutral friction — Korrelation + TE allein tragen sie nicht.
