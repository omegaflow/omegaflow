<!--
  title: Auftrag — M10-Haltung eigen ableiten (ISIS-cnet/Sternfeld), O−C gegen DE/INPOP/EPM, Position offen
  class: auftrag
  date: 2026-10-18
  sha256: 8ab049df3c8e75c46effe3f95471a51e8422a0229d526ce483a7de91b6f46af7
  status: live
  see-also: docs/auftrag/auftrag-eigene-ephemeriden-und-kulturerbe.md docs/plans/ref-mariner-faden-index.md docs/handover/archiv/handover-2026-10-12-mountain-folge333.md state/stimmen/2026-10-18-alternativen-offene-lizenzen.md
-->
# Auftrag — M10-Haltung eigen ableiten (ISIS-cnet/Sternfeld) · O−C gegen DE/INPOP/EPM · Position offen (2026-10-18)

**Zweck.** Die ASU-`camera_angles.txt` ist **Erbe (Level 2)**: ihre Sondenposition `SX SY SZ` ist
DE/SPICE-abgeleitet, ihre Kamerawinkel `C1C2C3` sind Robinsons Navigations-Fit (1999). Für eine
**Ephemeriden-Validierung** ist das zirkulär. Dieser Auftrag löst genau einen Teil heraus: die
**Haltung (Attitude) unabhängig aus den Bildern** ableiten und **O−C gegen DE/INPOP/EPM** messen.
Die **Position bleibt ausdrücklich offen** — sie braucht eigenes M10-Radio-Tracking.

**Nicht-Ziel.** Kein Ersatz der ASU-Datei durch eine Behauptung. Kein „DE-Re-Ingest". Kein
Vortäuschen von Unabhängigkeit: was geliehen ist, steht im Manifest.

---

## Warum (der Riss, benannt)

- `SX SY SZ` (Position) kommt aus der **DE/SPICE-Ephemeride** → ein Bild→Merkur-Vergleich gegen DE
  ist **teil-zirkulär**, solange diese Position benutzt wird.
- `C1C2C3` (Haltung) kommt aus **Robinson et al. 1999** — ein externer Fit.
- NAIF führt für M10 **nur** `M10_archive_1.bsp` (≈50 KB, 3/21–4/4/1974); **keine** CK/IK/SCLK.
  Die ASU-Datei ist die **einzige** Pose-Quelle (`aareadme`).
- **Folge:** Der Bildpfad ist „Erbe, nicht Brennstoff", solange die Pose nicht eigen ist
  (`handover-2026-10-12-mountain-folge333.md`, Riss 1 des Programms).

## Ausgangslage (gemessen)

| Baustein | Stand |
|---|---|
| `camera_angles.txt` | 811 Bilder × 3 Zeilen, 194 640 B, sha256 `0a65df59…`; Zeitraum 1974-03-29…1975-03-17 (M1/M2/M3) |
| Format | `[JD + FDS]` · `[SX SY SZ]` · `[C1 C2 C3]`; Doku `ser.im-ldi.com/M10/TXT/angles.html` |
| Referenz | Robinson et al. 1999, JGR 104(E12) 30847, DOI `10.1029/1999JE001081` |
| Methode/Ziel | Sonke & Robinson 2026, PSJ 7:35, DOI `10.3847/PSJ/ae32f5`, **CC BY 4.0**; Kalibrierziel **σ0 ≈ 0,570 px** |
| ISIS | USGS `DOI-USGS/ISIS3`, `LICENSE.md` = **public domain / CC0**; Tars `ENC1/ENC2/ENC3/ALL_RES.tar` |
| Shape-Model | MESSENGER Global DEM v2 (PDS, PD) + MDIS-Basemaps (PDS, PD) |
| Kamera-Modell | Brennweiten A 1493,6 / B 1500,1 mm; 832×700; NA 0,5° / WA 11°×14° |
| Werkzeuge | `src/archivar/camera_angles.rs` (Parser) · `tools/measure/src/bin/{m10_attitude_check,m10_camera_angles_frame,m10_camera_angles_geometry,m10_expected_diameter}.rs` |
| Lokaler Korpus | `recherche/m10-camera-angles/camera_angles.txt` + `MANIFEST.md` (privater Arbeitsbaum) |

## Die drei Gleise (getrennt, weil unterschiedlich ehrlich)

### G1 · Haltung durch **Verfeinerung** (nicht unabhängig)
ISIS `autoseed` → `pointreg` (subpixel) → **Bundle Adjustment** gegen den MESSENGER-DEM.
A-priori-Pose = Robinson/ASU. **Ausgabe:** eigenes Kontrollnetz + verfeinerte Winkel je Frame.
**Ehrlich:** das ist eine **Verfeinerung** von Robinson, **kein** Unabhängigkeitsbeweis.

### G2 · Haltung **absolut** aus Sternfeld/Rand (der Unabhängigkeits-Pfad)
Pose je Bild aus **Sternen im Frame** (Sternenkatalog) bzw. **Planetenrand/Limb** gegen das
Kameramodell — ohne Robinsons A-priori. **Ausgabe:** absolute `C1C2C3` je Frame, wo Sterne/Rand
messbar sind. **Das** ist der Pfad, der „Erbe → Brennstoff" drehen kann.

### G3 · O−C gegen DE/INPOP/EPM — **zwei getrennte Tabellen**
- **O−C(A)** mit der verfeinerten G1-Pose: interne Konsistenz (Aussage: „verfeinert, nicht unabhängig").
- **O−C(B)** mit der absoluten G2-Pose: der **eigentliche** Unabhängigkeitstest.
Beide je Encounter (M1/M2/M3) und mit **Hold-out** (Bilder/Zeitsegmente **vor** dem Fit festgeschrieben
— Rat/UI-Regel). Position in beiden als **geliehen** deklariert.

### P · Position — **offen, eigener Auftrag**
Eigene M10-Bahn aus **Zweiweg-Doppler/Range**. Messung: **M10-Radio-Tracking = 0 im Register**
(M4/M6/M7 liegen vor). Bis dahin bleibt `SX SY SZ` geliehen; kein G3-Ergebnis darf als
„positionsunabhängig" gelesen werden.

## Arbeitsteilung

- **Mycelium:** `url`/`origin`-Zeilen für ISIS-Tars, MESSENGER-DEM, MDIS-Basemaps, Sternenkatalog.
- **Archivar:** Fetch/Parse/Cache; `camera_angles`-Frames, DEM-Kacheln, Bild→Frame-Zuordnung.
- **Mathematikerin:** Beobachtungsgleichungen, Bundle Adjustment, Kovarianz, O−C-Statistik.
- **Mountain:** `terms`/`format`/`field`/`ttl` + das **Verdikt** (was ist unabhängig, was geliehen).

## Gates (keine harten Schwellen — gemessen und benannt)

- `cargo check` **0/0** an jedem Schritt.
- **σ0 gemessen und berichtet** gegen das Ziel 0,570 px; eine Abweichung ist ein **benannter Riss**,
  kein stiller Durchfall.
- **Hold-out** vor dem Fit festschreiben; Residuen getrennt nach Encounter und nach G1/G2.
- **Independence-Manifest** je Ergebnis: „unabhängig bzgl. Haltung (G2); Position geliehen (DE/SPICE);
  Shape-Model = MESSENGER-DEM; Kamera-Modell = ISIS".

## Fertig, wenn

- Ein Mess-Binary emittiert **per Bild** `frame_id · jd · C1 C2 C3 · residual_px · quelle(G1|G2)` als JSON,
  und `cargo check` ist 0/0.
- **σ0** für G1 und (soweit messbar) G2 ist berichtet — mit dem Paper-Ziel 0,570 px daneben.
- Die **O−C-Tabellen A und B** je Encounter liegen vor, mit Hold-out-Split und Independence-Manifest.
- Die **Position** ist im Ergebnis ausdrücklich als **offen** markiert, mit der Blockade
  „M10-Radio-Tracking = 0" — nicht als gelöst.

## Risse und Blocker (ehrlich)

- **Position** ohne eigenes M10-Tracking nicht unabhängig (größter Blocker).
- **G2-Abdeckung** unklar: nicht jedes Vidicon-Bild trägt Sterne; der Limb ist nur bei
  Randabbildung brauchbar. Wie viele Frames G2 trägt, ist **vor** dem Urteil zu messen.
- **σ0** ist ein Paper-Ziel, kein Beweis; Abweichung benennen.
- **ASU-Lizenz** bleibt Riss (`state/stimmen/2026-10-18-alternativen-offene-lizenzen.md`): der
  Quelltext hat keine ausgewiesene Lizenz; die Zutaten (DE/SPICE, ISIS/CC0, Robinson-Paper) sind offen.
- **Letzte Meile** M10→MDIS-Registrierung ist Paper-Methode, kein Datensatz — genau deshalb G1.

## Erste begrenzte Schritte

1. **ISIS-Quelle + Installationsweg messen** (`DOI-USGS/ISIS3`, Version, Tars-Verfügbarkeit) —
   `cargo check` 0/0, Quelle/Lizenz im Register.
2. **DEM-/Basemap-Abdeckung** der drei Encounter prüfen (welche Kacheln, welche Bilder).
3. **G1 auf einem Encounter** (M1, 410 Records) end-to-end: autoseed → pointreg → BA → σ0.
4. **G2-Machbarkeit** messen: wie viele Frames tragen Sterne/Limb? Erst danach G2 bauen.
5. **O−C(A)** emittieren; **O−C(B)** erst, wenn G2 trägt.

## Nicht in diesem Auftrag

- Eigene M10-Bahn / Radio-Tracking (separater Auftrag, blockiert).
- Ersatz/Entfernung der ASU-Datei (bleibt Zwischenträger, bis G2+G3 belegt sind).
- Vollständige Neu-Ephemeride (Programm-Auftrag `auftrag-eigene-ephemeriden-und-kulturerbe.md`).
