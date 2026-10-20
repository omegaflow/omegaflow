<!--
  title: Survey — die Sonne→Erde-Kette im Haus (Bestand, gemessen 2026-10-20)
  class: survey
  date: 2026-10-20
  sha256: 4beaf7c3ef81659be767edbab1c1734612954429fb465b41754e74eba76357f6
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
| **Netz** | `entsoe_unavailability` (MW, wöchentlich), `fingrid_datasets` (nur Katalog) | **1 (+0)** | 1 Woche | CC-BY / per-service |

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
| **Radioausbrüche** | **verdrahtet seit 2026-10-20** (`swpc_radio_bursts`, Typ II/III; 34 Sweeps im rollierenden ~25-Tage-Fenster; PD-USGov). **Zweiter Arm seit 2026-10-20:** `callisto`/MRO, 9 Bänder 50–846 MHz, 1-min, Hintergrund per gleitendem Median; 50.328 Minuten aus 2020-01, 2022-01, 2024-01 und 2026-09-24..10-10. Lizenz: freier Zugang mit Nennung, Host-Kontakt und PI-Co-Autorenschaft (`other-attribution`) |
| **Ionosonden** | **Quelle gefunden (2026-10-20):** Juliusruh (`0318`), 1957–2025, **552.100 foF2-Stundenwerte**, maschinenlesbar. Lizenz **SWS-WDC**: Namensnennung, **keine kommerzielle Nutzung** ohne Vereinbarung, **Weitergabe an Kollegen erlaubt, wenn die Bedingungen mitgegeben werden** — das Haus ist NC (PolyForm), also nutzbar und weitergabefähig. **CDN-Asset 200, 9.385.708 B, sha256 geprüft.** GIRO/DIDBase liefert von hier 504/404 |
| **Live-Sonnenwind (RTSW/DSCOVR)** | kein `rtsw`-, kein `dscovr`-Block; nur OMNI-Archiv und `wind_waves` |
| **SuperMAG-Felder** | Compiler da, nur `substorm` registriert; **nicht redistribuierbar** (auth) |
| **Finnisches Netz** | nur der Datensatz-Katalog, keine Zeitreihe |
| **Ausfallereignisse** | gar kein Datensatz (kein Zeit/Ort/Ursache-Träger) |

**Radio, zweiter Arm — gemessen, nicht behauptet (e-CALLISTO/MRO, 2026-10-20):**
Gewählt wurde MRO (Metsähovi, 60,2° N, 24,4° O; `on earth 60.2 24.4 0.09`),
weil die Tagesverzeichnisse an jedem 15. Januar 2018–2026 durchgehend
110–112 Dateien zeigen (HUMAIN: 46–49; Australia-ASSA erst ab 2020). Das Asset
`callisto_mro.bin` (4.126.904 B, sha256 `309ebd47…c4068`) trägt **50.328
Minuten** aus vier Fenstern: 2020-01, 2022-01, 2024-01 und 2026-09-24..10-10.
Pro Fenster wurden die 15-min-FITS (3600×200, TIME/FREQUENCY-Tabelle) auf
**9 Frequenzbänder 50–846 MHz** und **1-Minuten-Kadenz** reduziert; der
Hintergrund ist ein gleitender Median (±30 min), die Einheit ist `count`
(Rohdigits der Quelle, unkalibriert; BUNIT `digits`). Der vorhandene FITS-Leser reichte nicht: `TFORM` wie
`3600D8.3` (Breite/Präzision) wurde als Code `3` gelesen; `fits.rs` liest jetzt
den ersten Buchstaben als Code.

**Validierung gegen die 34 SWPC-Sweeps** (Überlappung 2026-09-24..10-10,
Detektor z > 5 in ≥ 2 Bändern, ≥ 2 aktive Minuten): 18 Ereignisse liegen in
MRO-Abdeckung, **16 sind unbeobachtet** (MRO deckt im Oktober ~04:30–15:45 UT,
die SWPC-Ereignisse reichen bis 23:01 UT). Von den 18 beobachteten: **7 Treffer,
11 Fehlschläge, 44 ungematchte Detektionen**. Die 1-min-Bandreduktion ist damit
**kein verlässlicher Ereignis-Trigger** — Typ-III-Sweeps dauern Sekunden und
verschwinden in der Minutenmitte; die 44 Alarme zeigen die RFI-Anfälligkeit.

**Kante Radio → OMNI** (2020-01, 2022-01, 2024-01; 93 brauchbare Tage,
OMNI2 1-h): stärkste Korrelation **|r| = 0.3899** (Band 420–560 MHz, Lag 3 d,
Tagesminimum Bz, n = 90). Der Block-Shift-Surrogat-Null (500 Replikate, zirkulär
innerhalb jedes Jahres) liegt bei **95 % = 0.4535**, **p = 0.1497** →
**trägt nicht**. Die Datenlage (nur drei Januar-Monate, tageszeitlich
beschränkt, keine durchgehende Mehrjahresreihe) trägt keine stärkere Aussage;
das ist die gemessene Grenze, kein Nullbefund über die Physik.

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

Mehrere Register-Kadenzen sind **gröber als die native Auflösung**:
`aia` 1 Tag (nativ ~12 s), `goes_euvs` 1 Jahr, `cdaweb_tec` 1 Tag, `magic_dl3`
1 Jahr, `soho_lasco_cme` 30 Tage. Für die Vorlauf-Leiter heißt das: die Knoten
sind da, aber die Zeitauflösung mehrerer Sonden trägt keine Minutenkante.

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
| M18 | Funk → Sonnenwind (MRO, 9 Bänder) | Survey, `callisto_edge_probe` | 93 Tage / n = 90 | \|r\| = 0.3899 (420–560 MHz, Lag 3 d); Block-Shift-Null 95 % = 0.4535, **p = 0.1497** → **trägt nicht** |
| M19 | MRO-Bänder vs. SWPC-Ereignisse | Survey, `callisto_swpc_validation` | 18 beobachtete Ereignisse | **7 Treffer, 11 Fehlschläge, 44 Fehlalarme** → kein verlässlicher Ereignis-Trigger |

**Datenfehler, benannt:** Die mehrjährigen Abisko-Reihen (`abk_dbdt_1h.bin`, `abk_dbdt_1m.bin`) tragen Zukunftsstempel (2024–2056 bzw. 2053–2055) und überlappen die GIC-Reihe deshalb nicht. Sie wurden **nicht** verwendet; der Fehler ist zu reparieren, bevor sie als Kante zählen.

**Der zentrale Riss:** M2–M4 (Jahresrunden) und M5 (Quartalsrunden) widersprechen
sich. Die Jahresrunde findet den Pfeil, die gehärtete Quartalsrunde nicht. Beide
sind gemessen, keine ist geglättet. Die Auflösung ist nicht „welche recht hat",
sondern die Fenster-/Macht-Abhängigkeit selbst.

**Was gemessen fehlt:**
- Sonne → L1 (Radio/Flare → Sonnenwind) als Zuwachs.
- L1 → Magnetfeld, Magnetfeld → Ionosphäre, Ionosphäre → Aurora.
- Aurora → Boden als Kante (F3: Schalter, nicht Vorläufer).
- Boden → Ausfall: Datensatz da (ECOD 2014–2023, 525.045 County-Ereignisse), Kante nicht gemessen.



Die Kette hat **alle Knoten**; Radio (zwei Arme) und Ausfall sind seit 2026-10-20
verdrahtet. Was fehlt, sind die **Kanten** — und bei drei Sonden die Auflösung.
Die nächste Arbeit ist daher nicht „mehr Quellen", sondern:

1. die vorhandenen Knoten auf ihre native Kadenz bringen (AIA, EUVS, TEC),
2. die Kanten als Zuwachs mit Null messen (F2-Regel),
3. die dauerhafte Mehrjahres-Funkreihe sichern (die zwei Arme sind verdrahtet;
   die Kante trägt mit dieser Datenlage nicht),
4. die Lizenzgrenzen (INTERMAGNET NC, SuperMAG auth, e-CALLISTO-Kontakt) in der Ablage respektieren.
