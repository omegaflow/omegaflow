<!--
  title: Survey — die Sonne→Erde-Kette im Haus (Bestand, gemessen 2026-10-20)
  class: survey
  date: 2026-10-20
  sha256: af322dbb96ac7c4904c09bdbb1bf28caee6a4ed58e5f77b9db12bac78ef966e7
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
| **Ionosphäre** | `cdaweb_tec` (TECU), `superdarn_fitacf` (Losgeschwindigkeit), `superdarn_rawacf`, `superdarn_cpcp` | **4** | 1 d · 10 min | CC0 / CC-BY / other-attribution |
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
- Boden → Strom als Kette: läuft (Vorlaufzeit + inkrementelle Fassung).

**Nur vorhanden (Knoten, keine gemessene Kante):**
- Sonne → L1: die Reihen liegen, die Kante ist nicht als Zuwachs gemessen.
- L1 → Magnetfeld, Magnetfeld → Ionosphäre, Ionosphäre → Aurora: ebenso.
- Aurora → Boden: F3 hat entschieden, dass Aurora ein **Nowcast-Schalter** ist,
  kein Vorläufer; die Schalterregel ist benannt, nicht als Kante gemessen.

## Die Lücken (gemessen, nicht behauptet)

| Lücke | Befund |
|---|---|
| **Radioausbrüche** | **verdrahtet seit 2026-10-20** (`swpc_radio_bursts`, Typ II/III; 34 Sweeps im rollierenden ~25-Tage-Fenster; PD-USGov). Offen: keine Langzeitreihe, kein 1-s-Spektrogramm |
| **Ionosonden** | kein Feld; nur TEC und SuperDARN |
| **Live-Sonnenwind (RTSW/DSCOVR)** | kein `rtsw`-, kein `dscovr`-Block; nur OMNI-Archiv und `wind_waves` |
| **SuperMAG-Felder** | Compiler da, nur `substorm` registriert; **nicht redistribuierbar** (auth) |
| **Finnisches Netz** | nur der Datensatz-Katalog, keine Zeitreihe |
| **Ausfallereignisse** | gar kein Datensatz (kein Zeit/Ort/Ursache-Träger) |

## Lizenzen, die die Weitergabe begrenzen

- **INTERMAGNET:** CC-BY-NC-4.0 — **nicht kommerziell**; die Rohreihe darf nicht in
  einen kommerziellen Mirror.
- **WDC Kyoto (AE/Dst/Sym):** scientific-noncommercial.
- **SuperMAG:** `auth-required`, „cannot be redistributed" (Rules of the Road) — nur
  lokal messen, nie spiegeln.
- **Fingrid:** per-service-terms; der Katalog ist keine Lizenz für die Zeitreihen.
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
| M17 | Boden → Strom als Kette | läuft | — | Vorlaufzeit + inkrementelle Fassung; Ergebnis offen |

**Der zentrale Riss:** M2–M4 (Jahresrunden) und M5 (Quartalsrunden) widersprechen
sich. Die Jahresrunde findet den Pfeil, die gehärtete Quartalsrunde nicht. Beide
sind gemessen, keine ist geglättet. Die Auflösung ist nicht „welche recht hat",
sondern die Fenster-/Macht-Abhängigkeit selbst.

**Was gemessen fehlt:**
- Sonne → L1 (Radio/Flare → Sonnenwind) als Zuwachs.
- L1 → Magnetfeld, Magnetfeld → Ionosphäre, Ionosphäre → Aurora.
- Aurora → Boden als Kante (F3: Schalter, nicht Vorläufer).
- Boden → Ausfall (kein Ausfalldatensatz).



Die Kette hat **alle Knoten** außer Radio und Ausfall. Was fehlt, sind die
**Kanten** — und bei drei Sonden die Auflösung. Die nächste Arbeit ist daher nicht
„mehr Quellen", sondern:

1. die vorhandenen Knoten auf ihre native Kadenz bringen (AIA, EUVS, TEC),
2. die Kanten als Zuwachs mit Null messen (F2-Regel),
3. Radio als Vorläufer und Ausfall als Endglied verdrahten,
4. die Lizenzgrenzen (INTERMAGNET NC, SuperMAG auth) in der Ablage respektieren.
