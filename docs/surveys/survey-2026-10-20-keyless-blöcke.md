<!--
  title: Vier keyless Routen als Register-Vorschlag (2026-10-20)
  class: survey
  date: 2026-10-20
  sha256: 69da0613f0b70e28b31e2b4e67a84cd1e699f25ea2d92024d97b5eb55bbddba0
  status: done
  see-also: phi/sources.φ
-->
# Vier keyless Routen als Register-Vorschlag

**Gemessen 2026-10-20 (UTC)** mit `curl --max-time` gegen die live Endpunkte.
Die Frage: welche der vier keyless Routen tragen einen Register-Block, welche
Felder liefert der Endpunkt wirklich, und was bleibt `absent`? Kein CDN-Upload,
keine Secrets, keine Massendownloads. Die Vorschlagsblöcke stehen in
`phi/sources.φ` mit `ttl 0`; der Parser verwirft sie dadurch (flush verlangt
`ttl > 0`), `status vorschlag` ist heute ein Marker ohne Parserarm.

## Bilanz

| Route | Messung | Felder | Kadenz (Vorschlag) | Lizenz | Status |
|---|---|---|---|---|---|
| CDAWeb HAPI DSCOVR | 200 · 1.278 B info · Datenprobe 5.516 B / 60 Zeilen | `B1F1`, `B1GSE.0..2` | `ttl 3600` | PD-USGov | **Vorschlag**, `ttl 0` |
| MEEO Sentinel-5P S3 | 200 · 1.105 B Listing / 2 Keys | `Key`, `LastModified`, `ETag`, `Size`; NetCDF-Messfelder `absent` | `ttl 86400` (Katalog) | Copernicus (frei, Quellenangabe) | **Vorschlag**, `ttl 0` |
| USGS Geomag FRD | 200 · 74.899 B JSON / 1.440 Minuten | `H`, `D`, `Z` (`values[].values[]`) | `ttl 60` | PD-USGov | **Vorschlag**, `ttl 0` |
| BPA GIC | 200 · 81.976 B · 1.152 vollständige 5-min-Zeilen | 11 Stationsspalten (A) | `ttl 300` | PD-USGov (Nutzungsbedingungen ungeklärt) | **bereits aktiv** (`bpa_gic.bin` in `phi/sources.φ`) |

FIRMS ist bereits als keyless CSV-Block im Register (`MODIS_C6_1_Global_7d.csv` in `phi/sources.φ`) und wurde nur
verifiziert, nicht dupliziert.

## DSCOVR/ACE (CDAWeb HAPI)

`https://cdaweb.gsfc.nasa.gov/hapi/info?id=DSCOVR_H0_MAG` → **200**, 1.278 B,
`startDate 2015-06-08T00:00:00Z`, `stopDate 2026-09-13T23:59:59Z`.
Parameter: `Time`, `B1F1` (nT), `B1SDF1`, `B1GSE` (nT, size 3), `B1SDGSE`,
`B1RTN`, `B1SDRTN`, `FLAG1`.

Datenprobe `time.min=2026-09-13T12:00:00Z&time.max=2026-09-13T12:00:59Z` →
**200**, 5.516 B, **60 Zeilen**, erster Zeitstempel `2026-09-13T12:00:00.500Z`,
`B1F1 = 6,084264 nT`, `B1GSE = [0,6481979, 5,881024, 1,416444] nT`.
Ein rollierendes `{yesterday}`-Fenster liefert heute **status 1201 „no data for
time range"** — die Route ist ein Archiv, kein Live-Feed.

Die weiteren IDs sind keyless erreichbar (info je 200): `DSCOVR_H1_FC`
(stop 2019-06-27), `DSCOVR_AT_DEF` (stop 2026-09-03),
`DSCOVR_COHO1HR_MERGED_MAG` (stop 2026-09-13), `TS1_L2_ACE_DEF`
(stop 2026-10-03), `TS2_L2_ACE_DEF` (stop 2026-10-02). ACE `_DEF` ist ein
49×21-Spektrum (`size [49,21]`, `bins`), also kein skalarer Block ohne
Compiler; deshalb trägt der Vorschlag DSCOVR-H0-MAG, nicht ACE.

## MEEO Sentinel-5P S3

`https://meeo-s5p.s3.amazonaws.com/?list-type=2&max-keys=2` → **200**, 997 B,
`KeyCount 2`, `IsTruncated true`. Der Bucket ist ein frei zugängliches S3-Listing ohne
Login. Probe unter `prefix=OFFL/L2__O3_TCL/` → **200**, 1.105 B, erster Key
`OFFL/L2__O3_TCL/2018/04/30/S5P_OFFL_L2__O3_TCL_20180430T105026_…nc`,
`Size 962.515`, `LastModified 2021-07-26T14:38:31Z`; Jahre 2018–2022,
`2026/` ist leer. Es ist damit ein **Teilarchiv**, nicht der vollständige
S5P-Bestand.

Messfelder im NetCDF (O3, Aerosol, NO2 …) wurden **nicht** gelesen — kein
Massendownload. Der Block trägt deshalb nur den Katalogzähler
`xmlcount Contents`; die physikalischen Felder stehen als `absent` im Survey,
nicht als erfundene `field`-Zeilen.

## USGS Geomag FRD

`https://geomag.usgs.gov/ws/data/?id=FRD&type=variation&format=json&elements=H,D,Z`
→ **200**, 74.899 B, `times` 1.440 Einträge (`2026-10-20T00:00:00Z` …
`2026-10-20T23:59:00Z`), drei Serien `values[0..2].values[1440]`.
Letzter nicht-null-Wert um `2026-10-20T12:49:00Z`: `H = 21784,24 nT`,
`D = 8,635`, `Z = 45053,4 nT`. Ein Fenster
`starttime=11:45&endtime=12:00` → **200**, 1.655 B, 16 Zeilen.

Das JSON ist **spaltenorientiert**: `values[]` sind Kanäle, `values[].values[]`
die Zeitreihe. Der Parser kann diesen Pfad mit `last` nicht erreichen
(`values.0.values` endet auf einem Objekt, nicht auf einem Array). Der
Vorschlagsblock nennt daher `format usgs_geomag_variation`; der Compiler/die
Format-Arm fehlen noch. Die Einheiten stehen nicht im JSON; die IAGA-2002-CSV
desselben Endpunkts nennt `FRDH`, `FRDD`, `FRDZ` (H/Z nT, D Bogenminuten).
Weitere Observatorien laufen über `id=`.

## BPA GIC (bereits aktiv)

`https://transmission.bpa.gov/business/operations/gic/gic.txt` → **200**,
81.976 B, `Last-Modified 2026-10-20T12:46:41Z`, 1.159 Zeilen, 1.152
vollständige 5-Minuten-Zeilen (4 Tage), letzter nicht-leerer Wert
`10/20/2026 05:45` Pacific Time. Header: 11 Stationen (Eugene OR, Spokane WA,
The Dalles OR, Lewiston ID, Kalispell MT, Monroe WA, Wilsonville OR,
Vancouver WA, Ellensburg WA, Bothell WA, Moses Lake WA). Einheit Ampere,
abgeleitet aus den 5-min-MW-Werten; die Datei selbst nennt keine Einheit.

Der Block ist **kein Vorschlag mehr**: `bpa_gic.bin` in `phi/sources.φ` registriert
`bpa_gic.bin`, `format bpa_gic`, `compiler tools/harvest/src/bin/bpa_gic_compiler.rs`,
`ttl 300` und elf `field`-Zeilen. Die Ernte ist rollierend: der Compiler liest
`gic.txt`, schreibt `bpa_gic.bin` und lädt unter dem Release-Tag
`transmission.bpa.gov` hoch. **Frage:** Es gibt keine Zeitstempel-Versionierung;
jeder Lauf überschreibt denselben Asset-Namen, und das 4-Tage-Fenster ist die
einzige Historie. Nutzungsbedingungen sind nicht separat geprüft (Register
trägt `PD-USGov`).

## FIRMS (Ist-Zustand, kein neuer Block)

Der keyless CSV-Block steht bereits als CDN-Asset in `phi/sources.φ:14346`
(`MODIS_C6_1_Global_7d.csv`, `format csv`, `at earth`, `no-cadence`) und trägt
**keine** `field`-Zeilen. Daneben stehen drei API-Blöcke
die drei `api/area/csv/{FIRMS_MAP_KEY}`-Blöcke mit `{FIRMS_MAP_KEY}` und
`bright_ti4`/`frp`.

Probe mit `Range: bytes=0-4095` → **206**, 4.096 B von 5.208.942 B,
`Last-Modified 2026-10-20T12:46:59Z`. Header:
`latitude,longitude,brightness,scan,track,acq_date,acq_time,satellite,confidence,version,bright_t31,frp,daynight`;
erste Zeile `2026-10-13 00:37`. Die ganze Datei wurde nicht geladen.

## Was quer liegt

- DSCOVR/ACE enden 2026-09-13 bzw. 2026-10-03; ein rollierender Block läuft
  heute ins Leere (1201). Die Route ist ein Archiv.
- MEEO listet nur ein Teilarchiv 2018–2022; die NetCDF-Messfelder bleiben
  `pending` (nicht `absent`), bis ein Objekt-Fetch/Compiler gebaut ist.
- USGS liefert die Werte, aber spaltenorientiert; ohne Format-Arm bleibt der
  direkte JSON-Block parser-def.
- BPA ist bereits gebaut, aber nicht versioniert; das 4-Tage-Fenster ist keine
  Historie.
- `status vorschlag` hat keinen Parserarm. Die Inaktivität kommt heute allein
  von `ttl 0`; wer `ttl` setzt, aktiviert den Block.

## Folge

1. Operator entscheidet je Route über `ttl` (Aktivierung) und Lizenz.
2. USGS: `usgs_geomag_variation`-Format/Compiler bauen (H/D/Z aus
   `values[].values[]`), dann `ttl 60`.
3. MEEO: Objekt-Fetch für `OFFL/L2__O3_TCL/` und NetCDF-Felder messen, bevor
   ein `field`-Block entsteht.
4. BPA: Asset-Versionierung (Zeitstempel im Namen oder Release pro Lauf)
   entscheiden, sonst bleibt die Historie bei vier Tagen.

## Nachtrag 2026-10-21 — Vorschläge verlassen das Register

`status vorschlag` war ein Marker ohne Parserarm und gehört nicht in
`phi/sources.φ`. Die drei DSCOVR/USGS/MEEO-Blöcke wurden aus dem Register
entfernt und als `dscovr-hapi`, `usgs-geomag-frd`, `meeo-s5p-o3` nach
`state/wartend.φ` überführt (Aufnehmer `mycelium`; Trigger: Operator-Wort
bzw. eigener Compiler/Objekt-Fetch). Die Messungen oben bleiben der Beleg.
