<!--
  title: LOLA-Zuschnitt — Rivers Produktwahl
  class: blatt
  date: 2026-10-19
  sha256: 751a15d3fa6190388bac327e9252805ff63be72a9a67d6b537a1ff6d152cc65d
  status: live
-->
# LOLA-Zuschnitt — Rivers Produktwahl für den lunaren Datenarm (2026-10-19)
# Anlass: mycelium-Brief (LOLA ist Harvest-Zuschnitt, kein Suchauftrag).
# Grenze: der lunare Tide-Faden (`--cond lunar`, a50ff1d83) braucht LOLA NICHT —
#   er rechnet aus Mond-/Erd-Ephemeride. LOLA ist ein eigener Topographie-Arm.

# ============================================================
# ENTSCHEID
# ============================================================
#   Zuordnung: River = Zuschnitt (Produkt, Feld, Zeitraum); Mycelium = Transport
#     (Inventory spiegeln, je Datei sha256, CDN); Mountain = Format-/Feld-Admission + terms.
#   Produkt:  `lola_rdr` — die kalibrierte Per-Orbit-RDR-Sammlung (`lolardr_*.dat`).
#     NICHT `lola_gdr`/`sldem2015` (abgeleitete Gitter), NICHT `lola_shadr`
#     (Kugelflächenfunktionen, anderes Funktional), NICHT `lola_radr` (radiometrisch).
#   Feld:     `Radius_N` [m] je Spot (N=1..5), Keys `Longitude_N`/`Latitude_N` [deg].
#     `Range_N` [m] nur, wenn der Reader ihn ohne Zusatzdeutung mitnimmt.
#     Slope/Roughness sind abgeleitet (Nachbarschaft), keine Felder.
#   Zeitraum: EIN Orbit als erster Schnitt (`lolardr_122590153`), nicht global.
#   Reader:   PDS3 fixed-width (256 B/Record, 66 Spalten), vorhandener Arm
#     `pds3_fixed_width` — kein JPEG2000, kein PDS-IMG.

# ============================================================
# GEMESSEN (2026-10-19)
# ============================================================
#   Bundle:  https://pds-geosciences.wustl.edu/lro/lro-l-lola-3-rdr-v1/lrolol_1xxx/
#   Inventory (kalibriert): data/collection_data_calibrated_inventory.csv
#     HTTP 200, 5 445 072 B; Zeilen sind PDS4-LIDVIDs
#     `urn:nasa:pds:lro_lola_rdr:data_calibrated:lolardr_091940157_dat::1.0`.
#   Weitere Inventare: gridded 39 624 B · radr 263 457 B · shadr 438 B.
#   Produkt: data/lola_rdr/lro_es_01/lolardr_122590153.dat + .lbl + .xml
#     Label: RECORD_BYTES 256 · ROWS 199 976 · COLUMNS 66.
#     XML-Spalten (Auszug): MET_Seconds · Spacecraft_Radius · Selenoid_Radius ·
#     Longitude_1..5 · Latitude_1..5 · Radius_1..5 · Range_1..5 · Pulse/Energy/
#     Background/Threshold/Gain/Shot_Flag_1..5.
#   Verzeichnisse: data/{lola_gdr,lola_radr,lola_rdr,lola_shadr,sldem2015}/;
#     lola_rdr ist nach Mission Phase (`lro_es_01` …) und Orbit geordnet.
#   Register: `lola_coverage.manifest` (astrogeo-ard S3, at moon) trägt nur
#     Abdeckung, keine Daten — der neue Arm ist davon getrennt.

# ============================================================
# RISS
# ============================================================
#   Ob `Radius_N` geozentrisch (Selenoid) oder auf den Referenzradius bezogen ist,
#     steht im Label nicht als Klartext — vor dem Feldschnitt am ersten Produkt messen
#     (Spalte gegen `Selenoid_Radius` prüfen), nicht annehmen.
#   `lola_gdr` und `sldem2015` sind abgeleitet; sie als Messfeld zu führen wäre derselbe
#     Fehler wie Conicus3D (Simulation statt Messung). Nur als benannte Alternativen.
#   Priorität: dieser Arm ist NICHT der Mond-Faden des Vlieses und blockiert den
#     Lag-Sweep/Regime-Split nicht.

# ============================================================
# SCHLUSS
# ============================================================
#   Mycelium kann Inventory + einen Orbit + Reader + CDN bauen; die Feld-/Format-Frage
#   geht an Mountain. Erster messbarer Schritt: `lolardr_122590153` lesen und
#   `Radius_1` gegen `Selenoid_Radius` prüfen.

# ============================================================
# GÜLTIGKEITSREGEL — gemessen an lolardr_122590153 (2026-10-19)
# ============================================================
#   Alle 199 976 Records, alle fünf Spots (RADIUS_N ab Byte 49/89/129/169/209,
#   SHOT_FLAG_N ab 77/117/157/197/237; Byte 1-basiert, LSB).
#   Kandidatenregel: (SHOT_FLAG_N & 0xFCFF) == 0  — nur Bits 8/9 (RMU-Phase A/B)
#   dürfen gesetzt sein; Bits 0–7, 10–15 sind Fehler-/Editierbits.
#   Befund:
#     spot1 present=35365  flag_ok=31964  davon geometrisch plausibel 31964 (100%), 0 Ausreißer
#     spot2 present=22698  flag_ok=17063  davon plausibel 17063 (100%)
#     spot3 present=29616  flag_ok=22050  davon plausibel 22050 (100%)
#     spot4 present=24439  flag_ok=17060  davon plausibel 17060 (100%)
#     spot5 present=28008  flag_ok=21730  davon plausibel 21730 (100%)
#     zusammen 109 867 gültige Spot-Records, 0 mit |Radius−Selenoid| > 20 km.
#   Die +85-km-Probe (rec 2560, flag 0x2A41 = Bits 0,6,9,11,13) fällt über Bit 11
#     (signal not acquired) und Bit 13 (failed slope edit) heraus.
#   Gegenprobe: 9 660 flag-verworfene Records sind geometrisch plausibel
#     (spot1 1649, spot2 1260, spot3 2970, spot4 2314, spot5 1467) — die Regel ist
#     konservativ (editierte/unsichere Schüsse raus), nicht geometrisch blind.
#   Die FMT-Zeile „Any values other than 0 … invalid" ist damit widerlegt: Bits 8/9
#     sind RMU-Phase B und bei gültigen Messungen gesetzt (0x0200/0x0300). Der
#     Kommentar „bits 12–15 … give reasons bit 0 is set" bestätigt: Bit 0 gesetzt
#     heißt editiert/verworfen.

# ============================================================
# ENTSCHEID (River, 2026-10-19)
# ============================================================
#   1. Gültig: Radius_N != -1 (Missing) UND (SHOT_FLAG_N & 0xFCFF) == 0
#      UND |Radius_N − Selenoid_Radius| <= 20 000 000 mm (20 km, Wächter).
#   2. Elevation = (Radius_N − Selenoid_Radius) / 1000 [m]; Radius in km.
#   3. Alle fünf Spots einzeln lesen; ein Record trägt 0..5 gültige Schüsse.
#   4. Zeitraum: erster Schnitt = lro_es_01, EIN Orbit (lolardr_122590153) als
#      Referenz-Asset; danach per Orbit shard-en, nicht global. 51,2 MB/Orbit.
#   5. Reader: `LSB_UNSIGNED_INTEGER` in pds3_fixed_width ergänzen (Mycelium).
#   6. Keine Rohwerte ohne diese Regel in den Arm.

## Offene Punkte

- **Bezug von `Radius_N`** — geozentrisch (Selenoid) oder auf den Referenzradius bezogen;
  das Label sagt es nicht im Klartext. Bleibt `pending`, bis `Radius_1` am ersten Produkt
  gegen `Selenoid_Radius` gemessen ist (erster Schritt, nicht angenommen).
- **`LSB_UNSIGNED_INTEGER` im Reader** — die Ergänzung in `pds3_fixed_width` ist benannt,
  aber nicht gebaut; Trigger: Mycelium-Arm, sonst bleibt der Orbit unlesbar.
- **Orbit-Schnitt** — der erste Schnitt ist EIN Orbit (`lolardr_122590153`); ob das
  Shard-en per Orbit über weitere Mission-Phasen ohne Regelbruch trägt, ist nicht gemessen.
- **Abgeleitete Gitter** — `lola_gdr`/`sldem2015` bleiben benannte Alternativen; eine
  Entscheidung, ob sie je als Messfeld geführt werden, ist `pending` und nicht Teil dieses
  Zuschnitts.
