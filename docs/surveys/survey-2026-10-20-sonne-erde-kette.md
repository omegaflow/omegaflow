<!--
  title: Survey — die Sonne→Erde-Kette im Haus (Bestand, gemessen 2026-10-20)
  class: survey
  date: 2026-10-20
  sha256: b36857dbd897ba2739b68fe17517a3c87c4ab0567da6644bebcf8b4613345686
  status: live
  see-also: phi/sources.φ docs/paper/gic-causal-driver.md zustand/rat/2026-10-15-rat-fragenpaket.md
-->
# Survey — die Sonne→Erde-Kette im Haus

**Anlass:** Die Linie hat den Bestand wiederholt aus dem Gedächtnis erzählt und dabei
Vorhandenes als fehlend gemeldet. Dieser Survey zählt, was im Register steht —
Segment für Segment, mit Kadenz und Lizenz. Er ist die Antwort auf „was haben wir
eigentlich?" und die Grundlage für F2 (Frühwarnkette).

**Messung:** `phi/sources.φ` am 2026-10-20, Blöcke und Feld-/Größen-Zeilen gezählt.

## Die Kette in sieben Segmenten

| Segment | Was da ist | Felder/Größen | Kadenz (Register) | Lizenz |
|---|---|---|---|---|
| **Sonne** | `f107` (Sonnenflux), `goes_xrs` (Röntgen A/B), `goes_euvs` (Lyman-α), `goes_abi` (3 Bänder), `aia` (7 Kanäle), `soho_lasco_cme` (Geschwindigkeit, Breite, Winkel, Masse, Energie, Beschleunigung), `swpc_radio_bursts` (Typ II/III: Frequenzband, Dauer, Typ) | 1+2+1+3+7+6+5 = **25** | 1 d · 1 h · 1 a · 1 h · 1 d · 30 d · Ereignis | PD-USGov / CC0 |
| **Zwischenpunkt (L1)** | `omni2_serie` (Bx/By/Bz, Dichte, Speed, Druck, Temperatur), `omni2_serie_daily`, `omni_hro` (dieselben acht), `wind_waves` (3 Radio-/Plasma-Bänder) | 7+7+8+3 = **25** | 1 h · 1 d · 1 h · 1 d | CC0 |
| **Magnetfeld** | `wdc_ae` (AE, AL, AU, Dst, Sym-D, Sym-H), `substorm` (SuperMAG: MLT, MLat, GLat, GLon) | **6 + 4** | 1 min (Dst 1 h) · 1 d | WDC non-commercial · SuperMAG auth |
| **Ionosphäre** | `cdaweb_tec` (TECU), `superdarn_fitacf` (Losgeschwindigkeit), `superdarn_rawacf`, `superdarn_cpcp`, `sws_ionosonde` (Juliusruh, foF2) | **5** | 1 d · 10 min · 1 h | CC0 / CC-BY / **SWS-WDC (NC, Weitergabe mit Bedingungen)** |
| **Aurora** | `themis_asi` (Rohbilder), `themis_asi_fodg`, `themis_asi_keogram`, `themis_asi_timed_keogram`, `keogram` (MIRACLE, 2 Blöcke), `ssusi_aurora` (Hemisphärenleistung N/S), `magic_dl3` | **3 Felder + Rohbilder** | Bildkadenz · 1 d · 1 a | free-open / CC-BY |
| **Boden** | `intermagnet_dbdt` (ABK/SOD, 1 min + 1 h), `intermagnet_xyz_*` (Komponenten je Station), `fmi_image_mag` (NUR, 10 s), `carisma_mag` (1 s), `bc_mpo_mag`, `usgs_geomag_efield` | **~474** | 1 min / 1 h / 10 s / 1 s | **CC-BY-NC-4.0** / OGL / PD |
| **Strom (GIC)** | `bpa_gic` (11 Punkte), `fmi_gic` (1 h), `fmi_gic_1min` (1 min), Kellerman-Knoten | **28** | 5 min · 1 h · 1 min | PD-USGov / CC-BY |
| **Netz** | `entsoe_unavailability` (MW, wöchentlich), `grid_outage_event` (ECOD, **167.772** eindeutige County-Ereignisse nach Dedup), `fingrid_datasets` (Katalog; die 8 Zeitreihen sind lizenzbedingt **zurückgezogen**) | 1 + 1 (+0) | 1 Woche · Ereignis | CC-BY / CC-BY / per-service |

## Was gemessen ist — und was nur liegt

**Gemessen (Kanten):**
- L1 → Boden: die Jahres- und Quartalsrunden im Papier (§4.2–4.6), inklusive der
  Divergenz yearly/quarterly.
- Boden → Strom: die skalare Relation im Halloween-Fenster (§4.7, r = 0.9448), und
  die stündliche TE gegen die GIC-Reihe (§4.8, kein fam-clearing Treiber).
- Boden → Strom als Kette: **gemessen** (§4.9) — trägt nicht: kein Vorlauf, Rückkanal nicht genullt, inkrementell 0/14.

**Nur vorhanden (Knoten, keine gemessene Kante):**
- Sonne → L1: die Reihen liegen, die Kante ist nicht als Zuwachs gemessen.
- L1 → Magnetfeld, Magnetfeld → Ionosphäre, Ionosphäre → Aurora: ebenso.
- Aurora → Boden: F3 hat entschieden, dass Aurora ein **Nowcast-Schalter** ist,
  kein Vorläufer; die Schalterregel ist benannt, nicht als Kante gemessen.

## Die Lücken (gemessen, nicht behauptet)

| Lücke | Befund |
|---|---|
| **Radioausbrüche** | `swpc_radio_bursts` (Typ II/III; 34 Sweeps im rollierenden ~25-Tage-Fenster; PD-USGov) ist verdrahtet. **Zweiter Arm (e-CALLISTO/MRO) ist zurückgezogen:** Host-Kontakt vor Nutzung nicht erfüllt, kein öffentliches Mirror. Die Messungen M18/M19 stammen aus dem lokalen Bestand und bleiben gültig; die Lizenzbedingung (Nennung, Host-Kontakt, PI-Co-Autorenschaft) ist offen |
| **Ionosonden** | **Quelle gefunden (2026-10-20):** Juliusruh (`0318`), 1957–2025, **552.100 foF2-Stundenwerte**, maschinenlesbar. Lizenz **SWS-WDC**: Namensnennung, **keine kommerzielle Nutzung** ohne Vereinbarung, **Weitergabe an Kollegen erlaubt, wenn die Bedingungen mitgegeben werden** — das Haus ist NC (PolyForm), also nutzbar und weitergabefähig. **CDN-Asset 200, 9.385.708 B, sha256 geprüft.** GIRO/DIDBase liefert von hier 504/404 |
| **Live-Sonnenwind (RTSW/DSCOVR)** | kein `rtsw`-, kein `dscovr`-Block; nur OMNI-Archiv und `wind_waves` |
| **SuperMAG-Felder** | Compiler da, nur `substorm` registriert; **nicht redistribuierbar** (auth) |
| **Finnisches Netz** | 8 Zeitreihen geharvestet, **zurückgezogen** (`per-service-terms` nicht eindeutig); der Katalog bleibt |
| **Ausfallereignisse** | gar kein Datensatz (kein Zeit/Ort/Ursache-Träger) |

**Radio, zweiter Arm — gemessen, nicht behauptet (e-CALLISTO/MRO, 2026-10-20):**

**Abdeckung, per Verzeichnis-Listing.** Raster 2016–2025, je Jahr
Januar/April/Juli/Oktober, je 2 Tage (5./20.), 80 Tage. „Dateien/Tag“ zählt
alle `<STATION>_<tag>_<hhmmss>_<suffix>.fit.gz`-Zeilen. MRO legt pro
15-min-Stempel vier Suffixe ab (`_59`/`_60` Hochband 50–846 MHz,
`_61`/`_62` Tiefband 16–90 MHz); der Compiler lädt nur den kleinsten Suffix.
Die 110–112 MRO-Zeilen eines Januartags sind also 26–28 Stempel, nicht 110
Beobachtungen.

| Station | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | Tage | Zeilen/Tag Median (min–max) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| MRO | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 80/80 | 180 (50–304) |
| HUMAIN | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 80/80 | 49 (17–63) |
| HURBANOVO | 8/8 | 8/8 | 7/8 | 8/8 | 7/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 78/80 | 52 (31–62) |
| TRIEST | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 80/80 | 104 (58–168) |
| ROSWELL-NM | 7/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 79/80 | 159 (88–288) |
| Australia-ASSA | 0/8 | 0/8 | 0/8 | 4/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 8/8 | 52/80 | 139 (32–249) |

Die UT-Fenster sind nicht gleich: MRO beginnt zwischen 00:45 und 09:09 UT und
endet zwischen 13:00 und 19:45 UT (Jahreszeit); HUMAIN liegt 2018–2021 bei
04:30–15:45 UT und 2022–2025 bei 04:30–16:30 UT; HURBANOVO 03:00–18:15 UT im
Juli und 06:45–14:30 UT im Januar; TRIEST 05:00–17:45 UT. Australia-ASSA hat
erst ab 2019/2020 eine Reihe.

**Wahl.** MRO bleibt der einzige Kandidat mit dem vollen neunbändrigen
50–846-MHz-Bereich (FREQUENCY-Tabelle gemessen: 49,988–846,238 MHz). HUMAIN
ist mit 80/80 und 49 Dateien/Tag leichter, deckt aber nur 45–437 MHz ab
(Bänder 1–7 voll, Band 8 nur 420–437 MHz, Band 9 leer); HURBANOVO deckt
45–200 MHz (5 Bänder), TRIEST 45–80 MHz (2 Bänder), Australia-ASSA nur
52/80 Tage. Die Wahl ist daher MRO; der Preis ist das saisonale UT-Fenster,
das die Kante unten auf ein gemeinsames Fenster schneidet.

**Sniffer.** Lokal (`archive_search --sniff`) auf der gzip-Datei und auf der
entpackten FITS-Datei gemessen:

```
[sniff] path …/MRO_20240105_072200_59.fit.gz
[sniff] bytes 294729
[sniff] format binary-magic
[sniff] columns 0
[sniff] time_column absent
[sniff] unit absent
[sniff] magic gzip

[sniff] path …/MRO_20240105_072200_59.fit
[sniff] bytes 760320
[sniff] format binary-magic
[sniff] columns 0
[sniff] time_column absent
[sniff] unit absent
[sniff] magic fits
```

Auch lokal liefert der Sniffer für FITS nur `magic`, keine Spalten/Zeitrolle/
Einheit; der Text-Pfad des Sniffers greift hier nicht. Die Felder stehen im
FITS-Header und in der BINTABLE: `NAXIS1=3600`, `NAXIS2=200`,
`CTYPE1='Time [UT]'`, `CDELT1=0.25` s, `CTYPE2='Frequency [MHz]'`,
`BUNIT='digits'`, `TFORM1='3600D8.3'`, `TFORM2='200D8.3'`, Tabelle
`TIME`/`FREQUENCY`. Der Compiler liest sie dort, nicht aus dem Sniffer. (HTTP
liefert erwartungsgemäß dasselbe `magic gzip` plus `sha256`; der lokale Pfad
ist nur insofern besser, als er die ganzen Bytes sieht.)

**Stichprobe.** 2018-01-01..2025-12-31, jeder 9. Tag, Anker 2018-01-01,
Offset 0: **325 Tage**. 324 Tage tragen MRO-Dateien (2021-01-03 ist leer),
**16.855 Dateien**, **4.515.281.451 B = 4,515 GB** (13,9 MB/Tag; nur der
kleinste Suffix). 316 Tage decken das gemeinsame Fenster **07:30–13:15 UT**
vollständig; **310 Tage** tragen alle neun Bänder und ≥345 Minuten in diesem
Fenster. Reduktion wie im ersten Lauf: 9 Bänder, 1-min, gleitender Median
(±30 min). Asset `callisto_mro_multiyear.bin`: **241.763 Minuten**,
**19.824.574 B**, sha256 `0151818f…72ab2`; **Asset zurückgezogen** — kein
öffentliches Mirror, bis der PI antwortet; die Messung nutzt den lokalen Bestand. Bei jedem
3. Tag wären es 13,5 GB gewesen; der 9-Tage-Schritt ist die dichteste volle
9-Band-Stichprobe unter 5 GB. Die Rohdaten liegen nicht im Repo.

**Erster Lauf (MRO, vier Fenster, 2026-10-20).** Das Asset `callisto_mro.bin`
(4.126.904 B, sha256 `309ebd47…c4068`; **zurückgezogen**) trägt **50.328 Minuten** aus 2020-01,
2022-01, 2024-01 und 2026-09-24..10-10. Pro Fenster wurden die 15-min-FITS
(3600×200, TIME/FREQUENCY-Tabelle) auf 9 Frequenzbänder 50–846 MHz und
1-Minuten-Kadenz reduziert; der Hintergrund ist ein gleitender Median
(±30 min), die Einheit ist `count` (Rohdigits, BUNIT `digits`). Der FITS-Leser
musste `TFORM` wie `3600D8.3` als ersten Buchstaben lesen, nicht als Code `3`.

**Validierung gegen die 34 SWPC-Sweeps** (Überlappung 2026-09-24..10-10,
Detektor z > 5 in ≥ 2 Bändern, ≥ 2 aktive Minuten): 18 Ereignisse liegen in
MRO-Abdeckung, **16 sind unbeobachtet** (MRO deckt im Oktober ~04:30–15:45 UT,
die SWPC-Ereignisse reichen bis 23:01 UT). Von den 18 beobachteten: **7 Treffer,
11 Fehlschläge, 44 ungematchte Detektionen**. Die 1-min-Bandreduktion ist damit
**kein verlässlicher Ereignis-Trigger** — Typ-III-Sweeps dauern Sekunden und
verschwinden in der Minutenmitte; die 44 Alarme zeigen die RFI-Anfälligkeit.

**Kante Radio → OMNI, erster Lauf** (2020-01, 2022-01, 2024-01; 93 brauchbare
Tage, OMNI2 1-h): stärkste Korrelation **|r| = 0.3899** (Band 420–560 MHz,
Lag 3 d, Tagesminimum Bz, n = 90). Der Block-Shift-Surrogat-Null (500
Replikate, zirkulär innerhalb jedes Jahres) liegt bei **95 % = 0.4535**,
**p = 0.1497** → **trägt nicht**. Die Datenlage (drei Januar-Monate) trug
keine stärkere Aussage; das war die gemessene Grenze des ersten Laufs.

**Kante Radio → OMNI, zweiter Lauf** (2018–2025; 310 Tage mit allen neun
Bändern, 307 mit OMNI-Bz; gemeinsames UT-Fenster 07:30–13:15, OMNI 1-h):
stärkste Korrelation **|r| = 0.1509** (Band 175–235 MHz, Lag 1 d,
Tagesminimum Bz, n = 307). Der Block-Shift-Null (500 Replikate, zirkulär
innerhalb jedes Jahres) liegt bei **95 % = 0.2330**, **p = 0.4311** →
**trägt nicht**. Die Zelle des ersten Laufs (Band 420–560 MHz, Lag 3 d,
Bzmin) liegt im neuen Sample bei **r = 0.0704** (n = 309), ist also nicht
reproduziert; auch die übrigen Spitzenwerte (Band 560–846 MHz, Lag 2 d,
Vmean: r = 0.1376, n = 307) bleiben unter dem Null. Das ist die belastbare
Messung: mit n = 307 über acht Jahre und einem festen 5,75-h-UT-Fenster
trägt die Tageskante nicht. Die Grenze ist das Tageskorn und das kurze
gemeinsame Fenster, nicht mehr die Stichprobengröße; ein Nullbefund über die
Physik ist das nicht.

## Lizenzen, die die Weitergabe begrenzen

- **INTERMAGNET:** CC-BY-NC-4.0 — **nicht kommerziell**; die Rohreihe darf nicht in
  einen kommerziellen Mirror.
- **WDC Kyoto (AE/Dst/Sym):** scientific-noncommercial.
- **SuperMAG:** `auth-required`, „cannot be redistributed" (Rules of the Road) — nur
  lokal messen, nie spiegeln.
- **Fingrid:** per-service-terms; der Katalog ist keine Lizenz für die Zeitreihen.
- **SWS/ASWFC (Ionosonde Juliusruh):** SWS-WDC-Conditions — Namensnennung, **keine kommerzielle Nutzung** ohne schriftliche Vereinbarung; **Weitergabe an Kollegen erlaubt, wenn die Bedingungen mitgegeben werden**. Das Haus ist NC (PolyForm Noncommercial), also Nutzung und Weitergabe zulässig; Provider nennen (BoM/ASWFC, Daten IAP Leibniz-Institut Juliusruh).
- **ENTSO-E:** CC-BY-4.0 — die einzige Netzreihe, die offen weitergegeben werden darf.

## Kadenz-Risse

Am 2026-10-20 am Erzeuger gemessen, nicht aus dem Register übernommen. Die Zahl
im τ-Slot der `field`-Zeile ist bei allen drei Sonden **gröber als der
Compiler-Bucket und deutlich gröber als die native Kadenz**:

| Sonde | Register (τ) | Compiler-Bucket | native Kadenz | Beleg |
|---|---|---|---|---|
| `aia` | 86400 s | 3600 s | **12 s** | `aia_rs_list_compiler.rs` `fetch_channel()` fragt `aia.lev1_euv_12s[…/31d@1h]`; JSOC `rs_list` für 2024-05-10 liefert T_REC `11:59:23`, `11:59:35`, `11:59:47` … (Δ = 12 s) |
| `goes_euvs` | 31536000 s | 86400 s | **60 s** (Produkt `avg1m`) | `euvs_compiler.rs` liest `geuv-l2-avg1d/`; das reale g14-`avg1m`-Jahr 2009 hat 175 680 Zeilen = 122 d × 1440 min |
| `cdaweb_tec` | 86400 s | 900 s | **900 s** | `cdaweb_tec_compiler.rs` (Test: 96 Epochen, 71 × 73); Tages-CDF 2024-05-01: 96 Epochen; das ausgelieferte Tagesbin (2020-10-14) hat 96 Epochen, Δt = 900 s |

Volumen bei nativer Kadenz (nur die im Asset getragenen Skalare, nicht die
Rohbilder/-dateien):

| Sonde | Bytes/Tag | Zeitraum | Lizenz | Entscheidung |
|---|---|---|---|---|
| `aia` | 1 008 000 B (7200 × 7 × 20 B) | 2010-05-20 – 2026-10-01 belegt, ~16,4 a → ~6,0 GB | CC0-1.0 | **nicht geharvestet** (>5 GB; JSOC-`rs_list`-Last) |
| `goes_euvs` | 28 800 B (1440 × 20 B) | g14 2009-09-01 – 2020-03-04; g15 2010-03-25 – 2020-03-04 | PD-USGov | **nicht geharvestet** (Auftrag: nur berichten); Roh-NC 74–186 MB/Jahr |
| `cdaweb_tec` | 15 922 184 B | 1995–2026, vor 2024 lückenhaft | CC0-1.0 | **umgestellt**: 2024-05 |

**Umgestellt: TEC.** Das Tagesbin trug die 900 s bereits, nur der Registerwert
stand auf 86400. Neu gebaut: `cdaweb_tec_2024_05.bin`, Fenster
2024-05-01 00:00 – 2024-05-31 23:45 UTC (31 d, 2 976 Epochen,
15 424 608 Records), **493 587 464 B**, sha256
`bc88df714e60b12e960d2744dc2d73af85ec650b93ca3c222ff684c1842b9e6e`.
Das Fenster überlappt die 2024er GIC-/Bodenreihen (M2, M4, M13) und trägt den
Mai-2024-Sturm. Der Compiler nimmt jetzt mehrere `--file`/`--url` und schreibt
**τ = 900**; ein einzelner Tag ist byte-identisch zum alten Lauf (gleicher
sha256). Vorschlag liegt in `state/register/cadence.proposal.φ`, **nicht** in
`phi/sources.φ`. Ein TEC-Jahr wären 5,81 GB und damit über dem Limit eines
einzelnen Release-Assets (2 GiB) — deshalb ein Monat, nicht das Jahr.

Offen: `aia` bleibt auf dem 1-h-Bucket, `goes_euvs` auf dem 1-d-Bucket; ihre
τ-Zahlen sind noch nicht auf die native Kadenz gesetzt. `magic_dl3` und
`soho_lasco_cme` wurden in diesem Lauf nicht angefasst.

## Was schon gemessen wurde (nicht nur Register)

Nicht der Bestand, sondern die **Arbeit**: was die Linie tatsächlich gerechnet hat,
mit Fenster, Zahl und Ergebnis.

| # | Messung | Dokument | Fenster / n | Ergebnis |
|---|---|---|---|---|
| M1 | Minute: L1 → Bodenfeld | `docs/paper/gic-causal-driver.md` §4.1 | 22 h, n ≈ 1260 | Bz 2.180e-1 > eigene Schwelle 2.083e-1, aber < fam 3.744e-1 → family bound; Speed/Density silent |
| M2 | Stündlich 2024, Abisko | §4.2 | 1 Jahr, n = 8728 | Bz 1.2670e-1 > fam 1.0557e-1 → **Pfeil** (Lag-0/1-Randbin) |
| M3 | Stündlich 2025, Abisko | §4.3 | 1 Jahr, n = 8688 | Bz 1.3309e-1 > fam 1.2136e-1 → **Pfeil** |
| M4 | Stündlich 2024, Sodankylä | §4.4 | 1 Jahr | Bz 1.1695e-1 > fam 1.0571e-1 → **Pfeil**; Rückkanal ebenfalls über fam |
| M5 | Gehärtete Quartalsrunde | §4.6, §6 | 15 Artefakte, n_surr = 100 | **kein** Bz-Lag über fam (fam 0.172–0.210, Bz 0.105–0.166); PCMCI entfernt die Kante in 13/16 |
| M6 | Täglich 1994–2026 | §4.5 | n ≈ 3900 | alle unter fam 1.6995e-1 — Tageskorn leer |
| M7 | Abgeleitete Kanäle (Bs, Clock, P_dyn, M_A) | §4.6 | 15 Artefakte | 15/15 family bound; P_dyn per-Lag in 15/15 über eigener Schwelle |
| M8 | Schätzer-Ground-Truth (Hénon) | §3.5 | n = 10 000, c = 0.2 / 0 | Skalar-Arm: Richtung 2.457e-1 vs fam 2.405e-2, aber Rückkanal 3.64e-2 ebenfalls über fam; KSG-Arm: Rückkanal genullt, c = 0-Kontrolle **nicht** → beide **NOT PASS** |
| M9 | ENSO-Vierkanalblock | §3.6 | 542 Monate | kein Pfeil über fam 2.961e-1; cTE 8.359e-3 < 2.612e-2 |
| M10 | NUR–GIC, Halloween 2003 | §4.7 | n = 72 h | r = 0.9448; \|GIC\| = 0.2118·\|ΔX/10 s\| + 1.755; Richtung verloren, Peaks entkoppelt |
| M11 | GIC-TE, täglich | §4.8 | 2000–2023, n = 2145 | kein fam-clearing Treiber (fam 1.877e-1); Bz 1.9729e-2, Density 1.3386e-2 an der Familienschwelle |
| M12 | GIC-TE, stündlich | §4.8 | 2003, n = 6453 | kein Pfeil (fam 1.4223e-1); **Speed überholt Bz** → Tagesrang „Bz > Speed" ist ein Kornartefakt |
| M13 | α-Kalibrierung (WY max-T) | Blatt `…fruehwarnsystem…` | 3 Station-Jahre | α = 0,05: 2.4831 / 2.4467 / 2.4433; α = 0,01: 3.0964 / 3.0490 / 3.0036 — beide lesbar |
| M14 | Verzögerung Z | Blatt, `bz_dbdt_delay_probe` | ABK 2024, n = 251048 | Z = 142 min, r = 0.0223 — **ohne Minute-Null**, nicht gesiegelt |
| M15 | DE-2 Ion↔Neutral | `survey-2026-10-18-k-te-kadenz.md` | n = 23200 | 3/3 Lags pass; Mond-Konditionierung 0/36, Rückrichtung 3/36 |
| M16 | Signal-cone-Positivkontrolle | `survey-2026-10-19-signal-cone-positivkontrolle.md` | 730 d, 90 Paare | τ = 2 d = 1.728e5 s, d/c = 8.155 min, TE 5.4413e-1 > fam 2.8826e-1 → Pfeil im Kegel (synthetisch) |
| M17 | Boden → Strom als Kette | §4.9, `gic_ground_probe` | 4 319 min; inkrementell n = 72/23 h | unbedingt **Pfeil** (Lag 5 min, TE 1.4043e-1 > fam 1.2928e-1), aber **symmetrisch** (Rückkanal ebenfalls; Verhältnis 1.01–1.12) und **lag-flach**; inkrementell **0/14**, Δ negativ → **trägt nicht**, 0 Vorlauf |
| M18 | Funk → Sonnenwind (MRO, Mehrjahresprobe 2018–2025) | Survey, `callisto_edge_probe` | 325 Tage, n = 307 | \|r\| = 0.1509 (175–235 MHz, Lag 1 d); Block-Shift-Null 95 % = 0.2330, **p = 0.4311** → **trägt nicht**. Der erste n=90-Lauf (0.3899) reproduziert sich nicht (r = 0.0704) — Fenster-/Auswahlkorrelation |
| M19 | MRO-Bänder vs. SWPC-Ereignisse | Survey, `callisto_swpc_validation` | 18 beobachtete Ereignisse | **7 Treffer, 11 Fehlschläge, 44 Fehlalarme** → kein verlässlicher Ereignis-Trigger |
| M20 | Boden/Strom → Netzausfall | `survey-2026-10-20-netzausfall-kante.md` | 3.318 Tage national; BPA-räumlich 0 | bestes \|r\| = 0,0499 (ABK dB/dt), Null 0,0975, **p = 0,755** → **trägt nicht**; BPA-Paarung zeitlich leer (GIC nur 4 Tage rollierend) |

**Datenfehler, benannt:** Die mehrjährigen Abisko-Reihen (`abk_dbdt_1h.bin`, `abk_dbdt_1m.bin`) tragen Zukunftsstempel (2024–2056 bzw. 2053–2055) und überlappen die GIC-Reihe deshalb nicht. Sie wurden **nicht** verwendet; der Fehler ist zu reparieren, bevor sie als Kante zählen.

**Der zentrale Riss:** M2–M4 (Jahresrunden) und M5 (Quartalsrunden) widersprechen
sich. Die Jahresrunde findet den Pfeil, die gehärtete Quartalsrunde nicht. Beide
sind gemessen, keine ist geglättet. Die Auflösung ist nicht „welche recht hat",
sondern die Fenster-/Macht-Abhängigkeit selbst.

**Was gemessen fehlt:**
- Sonne → L1 (Radio/Flare → Sonnenwind) als Zuwachs.
- L1 → Magnetfeld, Magnetfeld → Ionosphäre, Ionosphäre → Aurora.
- Aurora → Boden als Kante (F3: Schalter, nicht Vorläufer).
- Boden → Ausfall: **gemessen** (eigener Survey `survey-2026-10-20-netzausfall-kante.md`) — BPA-Paarung zeitlich leer (GIC nur 4 Tage rollierend); nationaler Proxy **trägt nicht** (\|r\| ≤ 0,05, p ≥ 0,67). ECOD nach Dedup **167.772** eindeutige County-Ereignisse.



Die Kette hat **alle Knoten**; Radio (zwei Arme) und Ausfall sind seit 2026-10-20
verdrahtet. Was fehlt, sind die **Kanten** — und bei drei Sonden die Auflösung.
Die nächste Arbeit ist daher nicht „mehr Quellen", sondern:

1. die vorhandenen Knoten auf ihre native Kadenz bringen (AIA, EUVS, TEC),
2. die Kanten als Zuwachs mit Null messen (F2-Regel),
3. die dauerhafte Mehrjahres-Funkreihe sichern (die zwei Arme sind verdrahtet;
   die Kante trägt mit dieser Datenlage nicht),
4. die Lizenzgrenzen (INTERMAGNET NC, SuperMAG auth, e-CALLISTO-Kontakt) in der Ablage respektieren.
