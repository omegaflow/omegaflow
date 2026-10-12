<!--
  title: Auftrag — Eigene Ephemeriden aus Primärdaten + Kulturerbe-Rettung (eine Maschine)
  class: auftrag
  date: 2026-10-12
  sha256: 77394444a92c677b28c8e49ab62294167fa4274359e32e58c4e416818ffd01ab
  status: live
  see-also: docs/surveys/survey-2026-10-10-ephemeris-quellen.md docs/concepts/archivar-mathematikerin.md docs/SOURCE_PORT.md
-->
# Auftrag — Eigene Ephemeriden aus Primärdaten + Kulturerbe-Rettung (2026-10-12)

**Zweck.** Zwei Ziele, eine Maschine. (A) **Wir bauen unsere eigenen Ephemeriden** —
aus Primärbeobachtungen (LLR, Raumsonde-Tracking, Radar, VLBI, ITRF, EOP, Troposphäre),
nicht durch Re-Ingest von DE440/INPOP/EPM. (B) **Wir retten Kulturerbe** — den
historischen Tiefraum-Korpus (Mariner 2/4/5/9/10, Pioneer 10/11, Viking, Voyager,
Galileo, Cassini) und insbesondere die 1974er Mariner-10-Bilder + Pose, die auf Bändern
verrotten und als Datei **nirgends** existieren. Der Kern: **jeder gerettete historische
Datensatz ist eine Primärbeobachtung für unsere Ephemeride** — die Rettung ist der
Brennstoff des eigenen Baus.

**Arbeitsteilung.** Register-Zeilen (`url`/`origin`/`compiler`/Tags) = Mycelium;
`terms`/`format`/`field`/`ttl` + Verdikte = Mountain; Ausgleichung/Chebyshev +
Kamera-/Kontrollnetz-Geometrie = Mathematikerin (`src/mathematikerin`); Fetch/Parse/Cache
= Archivar (`src/archivar`). Jeder Dispatch ist **ein begrenzter Schritt** mit
`cargo check` 0/0 als Gate. Kein Verdikt ohne Messung.

---

## Was schon steht (gemessen, nicht behauptet)

- **Maschine:** `ephemeris_compiler` (SPK→Chebyshev→`ephemeris_binary`),
  `BodyEphemeris`/`parse_ephemeris_binary`/`body_barycenter_position`, `house_gate`
  (DE/INPOP/EPM Erde = **22,1 / 15,9 / 30,7 km** am JUICE-Perigäum), `granule_census`,
  `structure_probe`; `bsp_reader` (SPK/DAF), `lsk`, `pck`.
- **ODF-Familie** (Roh-Tracking-Parser): `pioneer{10,11}_odf`, `voyager{1,2}_odf`,
  `galileo_odf`, `cassini_odf`, `vex_odf`, `pathfinder_odf`, `odf_serie`,
  `mariner_rst`, `mariner4_odg`, `mariner_m10_edr`.
- **Register:** DE440/441/442, INPOP, EPM, NOE-4, **12+ ODF**; `eop_iers_*`, IONEX-TEC,
  `gm_de440.tpc`, `naif0012.tls`.
- **Vorarbeit:** `state/stimmen/2026-10-10_mountain_ephemeris-sources-round.md` +
  `docs/surveys/survey-2026-10-10-ephemeris-quellen.md` (**Schließungsliste**);
  `state/mountain/eph_research/` (8); `state/research-map.md` (7476 Dateien).
- **Mariner-10:** `camera_angles.txt` (sha256 `0a65df59c8685e22…`, 194 640 B, verifiziert)
  mit Pose je Bild für die drei Encounters; DSC-0119-Format gemessen
  (`state/mountain/mariner4_dsc0119_format.md`); Sonke-&-Robinson-2026-Volltext + Table 1
  (**CLR 705 Bilder, σ0 = 0,570 px**, Skalen 0,4/0,63/2,6 km); die 8 Artefakte
  (Table 1–3, A1-MRT, ASU, USGS, LPSC) mit URL+sha.

---

## Pfeiler A · Eigene Ephemeride (Primärdaten → Ausgleichung → Asset)

**Beobachtungsklassen und Routen** (Schließungsliste der Ephemeriden-Runde):

- **A1 · LLR-Normalpunkte** — ILRS/CDDIS (laufend) und historisch **POLAC/Z&N/Revised-Lunar**
  (1969–1998). *Die größte Lücke; Parser steht.* Route: `cddis.nasa.gov` (Login
  `blocked account`), ILRS-Datenzentren, POLAC-Archiv.
- **A2 · Raumsonde-Tracking** — ODF/ODG der Missionen oben. *Der Rettungskorpus = der
  Brennstoff:* jede Dopplermessung ist eine Beobachtungsgleichung für Bahn + Planet.
- **A3 · Planeten-Radar** — JPL `ssd.jpl.nasa.gov/obs_data.html` (1962–1997; nicht echo.jpl).
- **A4 · VLBI/DDOR + ICRF3** — Rahmenanbindung (DDOR roh meist nicht öffentlich).
- **A5 · ITRF2020** Stationskoordinaten · **IERS EOP** (im Register) · **VMF1/VMF3**
  Troposphäre (IMGW Wien) · **GRAIL** Mondschwerefeld (`GRGM1200A`) · **naif0012.tls** (im Register).

**Maschine.** Archivar lädt die Primärdaten als Beobachtungsreihen (Zeit, Typ, Wert,
Gewicht, Station) → **Mathematikerin** bildet die **Beobachtungsgleichungen** und löst die
**gewichtete Ausgleichung** (Bahnparameter + ggf. Stationskoordinaten + EOP gemeinsam) →
Chebyshev-Fit → `ephemeris_binary` → CDN, **sha-versiegelt**.

**Gates.** `house_gate` gegen DE/INPOP/EPM (Fenster 22–31 km) **und** eigene
**LLR-O−C-Residuen als Blindtest** — denn DE/INPOP/EPM fitten LLR selbst; ohne eigene O−C
ist kein blinder Test möglich (Riss der Runde).

---

## Pfeiler B · Kulturerbe-Rettung (Mariner 10 + der Tracking-Korpus)

**Alle Zutaten offen und lizenziert** (gemessen): M10-CLR-Bilder (ASU `ser.im-ldi.com`, PD) ·
`camera_angles.txt` (Pose je Bild, offen) · ISIS-Tars `ENC1/ENC2/ENC3/ALL_RES.tar` ·
MESSENGER-Global-DEM v2 (PDS, PD) · MDIS-Basemaps (PDS, PD) · USGS-Kombi-Mosaik (US-PD) ·
**ISIS** (USGS, public domain) · Methode (Sonke & Robinson 2026, CC BY 4.0) ·
Kalibrier-Ziel (Table 1: σ0 = 0,570 px).

- **B1 · Gleis 1 — Registrierung.** `camera_angles.txt` + ISIS-Tars + USGS-Mosaike als
  `url`-Zeilen (Mycelium) → `terms`/`format` (Mountain) → **Pose je Frame in den Archivar**.
- **B2 · Gleis 2 — ISIS-Replikation.** Die Methode aus dem Volltext: SPICE-Pose (Robinson 1999)
  + MESSENGER-DEM als Shape-Model an die M10-Labels → ISIS **`autoseed`** →
  **`pointreg`** (subpixel) → **Bundle Adjustment**. **Keine SPK-Bahn nötig** — die
  verfeinerte Pose kommt aus `camera_angles`. Ausgabe: **das Kontrollnetz, das als Datei
  nicht existiert** → sha-versiegelt im eigenen Baum. **Kalibrier-Gate: σ0 ≈ 0,570 px.**
- **B3 · Gleis 3 — Kontakt** (nur falls Gleis 2 den Riss nicht schließt): Sonke/Robinson
  anschreiben — **Operator-Hand**, nicht autonom.

---

## Phasen (begrenzt, mit Gates)

- **Phase 0 — Forschungslage + Linse.** `archive_search --all` je Beobachtungsklasse (einmal
  als Text), dann **Rat (fünf Stimmen)** + **voller UI-Roster** (eigene Frontier + geteilte
  Open-Weight + neue Seats, Suchmodus aktiv). Der Auftrag wird erst Gesetz, wenn beide Linien
  konvergieren oder der Riss benannt ist.
- **Phase 1 — LLR + ITRF2020** (größter Hebel). Parser + Ausgleichung; Gate: LLR-O−C.
- **Phase 2 — Tracking-Korpus** (Mariner/Pioneer/Viking/Voyager) → eigene Planetenephemeride;
  Gate: `house_gate` + Horizons-Vergleich.
- **Phase 3 — Radar + VLBI/ICRF3 + VMF + GRAIL**; Gate: Residuen.
- **Phase 4 — Mariner-10-Kontrollnetz (ISIS)**; Gate: σ0.
- **Phase 5 — Siegel + Blatt.** Jedes Ergebnis sha-versiegelt, versioniert, reproduzierbar;
  ein Blatt „aus offenen Teilen nachgerechnet".

---

## Risse und Blocker (ehrlich)

- **CDDIS/Earthdata-Login** = `blocked account` (Operator/Future) — LLR-Rohdaten hängen daran.
- **Beobachtungsoperator** (nativ Rust) fehlt → ohne ihn bleiben Rohdaten un-reduzierbar.
- **ISIS-Quelle unverifiziert:** „Zenodo/Unlicense" ist im Baum **nicht** belegbar; das echte
  ISIS3 ist **USGS/public domain**. Vor Phase 4 messen.
- **M10-FOV je Kamera** ungemessen (Näherung 0,42°/0,98° verworfen).
- **M1-Proxy** (`2·asin(R/range)` vs FOV) ist **nicht** die Paper-Methode — nicht Kernpfad.
- **Geo/blocked:** RAND R-2090 (1976er Kontrollnetz, 403 → Wayback), Smithsonian 403.
- **Paper-only:** die *letzte Meile* M10→MDIS-Registrierung (Sonke & Robinson) ist Methode,
  kein Datensatz — genau deshalb die Replikation.

---

## Rat (fünf Stimmen, 2026-10-12) — gehalten

**Konsens.** EIN Archivar, ZWEI Register — vier Datenklassen: `obs` (Beobachtungsreihe,
**neue** Klasse mit Format/Heimat — schließt das Sprachloch), `heritage` (Artefakt),
`observable` (Beobachtungsgleichung/Residuum), `asset` (`ephemeris_binary`).
- **`house_gate` demoten** von „Gate" zu **Konsistenz-Hülle** (22–31 km ist die Streuung der
  drei Modelle, die wir *nicht* re-ingestieren). Ehrliches Gate = **Hold-out**: Testdaten
  (Zeitsegment + Beobachtungsklasse) **vor** dem Fit festgeschrieben.
- **Operator minimal:** eine Observable, ein Körperpaar; unmodellierte Terme (Gezeiten,
  Relativität, Ionosphäre, Uhr, thermischer Rückstoß) geben **`pending`**, nie 0.0. Der
  Beobachtungsoperator ist ein Compiler mit `canObserve`-Eigenschaft.
- **Zeit/Rahmen:** Epoche + Chebyshev-Argument in **einer deklarierten** Skala (TDB/TT);
  Station ITRF→GCRS mit **EOP + IAU 2006/2000A**; SI am Parse-Rand (kein stilles km/m);
  **Chebyshev-Fitfehler als eigene Budget-Zeile** (sonst liest das Gate den Polynomgrad).
- **`ω()`-Loop:** die eigene Ephemeride manifestiert als **Query-Eigenschaft** (Position +
  Unsicherheit als Property), getragen von der exponentiellen Relaxation.
- **Reihenfolge nach Realisierbarkeit** (`Bindung: eigen` zuerst): ITRF2020/EOP + **ein
  registrierter ODF** → ODF-Korpus (Pfeiler A2 = B-Brennstoff) → Mariner-10 als **eigenes
  Erbe-Register** (σ0-Gate, sha) → LLR, wenn das Konto offen ist → Radar/VLBI/GRAIL.
- **Operator-Akte = Operator-Hand** (CDDIS/Earthdata-Konto, CDDIS-Terms, Sonke/Robinson):
  **eine** Future-Queue, einfache Sprache; die bewertende Anfrage trägt **LOCK**.

**Riss 1 (River/Sensory gegen Mountain/Mycelium/Future).** Der Mariner-10-**Bildpfad** ist
**Erbe, nicht Brennstoff** — solange die Pose aus `camera_angles` (einem DE/SPICE-Fit) kommt
und die Methode ein Kontrollnetz an einen MESSENGER-DEM bindet, liefert er **keine unabhängige
Beobachtung**. „Erbe, bis er eine rahmen-unabhängige astrometrische Gleichung emittiert
(Sternenkatalog + Kameramodell)" — ein eigener, größerer Bau.

**Riss 2.** „Kein DE-Re-Ingest" — der a-priori-Zustand und das Kraftmodell (Asteroidenmassen)
reisen mit. **Der Rahmen ist aus ICRF3/VLBI wirklich unabhängig; die Massen nicht.**

**UI-Roster (2026-10-12, `mountain-ui` + `open-weight-ui`).**
`state/stimmen/2026-10-12_ui_eigene-ephemeriden-verdikt.md` — **Qwen ✓ · Claude ✓ · DeepSeek
Chat ✓ · DeepSeek V4 Pro (1.7T) ✓ · Qwen3.8 2.4T ✓** (alle konvergent); Z.ai (Deep Think
**Max**) und Duck.ai `pending` (gemessen).

**Die UI-Verschärfungen (in diesen Auftrag aufgenommen):**
- **`raw_level` 0/1/2 (Qwen3.8 2.4T).** Level 0 = Brennstoff (Tracking-Roh-Counts, VLBI-
  Korrelator, LLR-Normalpunkte, Radar-Echos); Level 1 = vorbelastet (ODF, SPICE SPK/CK —
  bereits Fits gegen jemandes DE/Frame/Uhr); Level 2 = Erbe (Bildpose aus DE/SPICE-Fit).
  **Level ≥ 1 ist Kreuzcheck/Erbe, nie Brennstoff** — sonst reparametrisiert omegaflow nur
  JPLs Ephemeriden.
- **Blinder Fleck = die Reduktionskette, nicht der Solver** (DeepSeek V4 Pro + Qwen3.8 2.4T):
  Zeitskalen (TDB/TT/UTC/SCLK), Frames (ITRF↔ICRF-Tie), Antennendelays, Tropo/Iono,
  Stationskoordinaten, Nongrav. Frame-Fehler werden vom Fit als „Physik" absorbiert; ein
  schlanker Dynamik-Anteil erzeugt systematische Residuen, die der Hold-out nicht sieht.
- **Reihenfolge (UI-Konsens, schärfer als der Rat):** **ITRF2020/EOP einfrieren (nicht
  mitfitten) → LLR als erster Operator (billigster vollständiger End-to-End-Beweis) → erster
  Level-0-Sondenpass (Counts, nicht ODF) → Erbe.** „ODF zuerst" ist die Falle. **Hold-out
  nach Bögen und Stationen splitten**, nicht zufällig nach Samples. Jeder Operator braucht
  ein explizites Noise-/Bias-Budget, sonst wird `pending` zur Müllkippe.

## Erste begrenzte Schritte

1. **Phase 0:** Forschungslage je Klasse messen; Rat + voller UI-Roster; diesen Auftrag halten.
2. **ISIS-Quelle messen** (`astrogeology.usgs.gov/software/ISIS`, `DOI-USGS/ISIS3`) — Quelle,
   Lizenz, Installationsweg.
3. **Phase 1 starten:** LLR/CDDIS-Route + ITRF2020 als erstes Bau-Atom (~1 Parser + 1 Fit).
4. **B1 registrieren:** `camera_angles` + ISIS-Tars + USGS-Mosaike als `url`-Zeilen (Mycelium).
