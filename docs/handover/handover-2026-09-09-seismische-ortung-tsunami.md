<!--
  title: Handover — seismische Ortung (ak135, Positivkontrolle bestanden) + Tōhoku-Tsunami-Kette (Pegel gemessen, Vorhersage 4/6 Wege) + M9.1-Picker (Grenze benannt)
  class: handover
  date: 2026-09-09
  sha256: 7181c176e6a6aa7d4203b4acfa171f86610c5ffb3bc3c6f10331a2c0abd948ae
  status: live
  see-also: docs/TODO.md docs/befund/befund-2026-09-09-seismische-ortung.md docs/befund/befund-2026-09-09-seismische-ortung-ak135.md docs/befund/befund-2026-09-09-seismische-ortung-tiefe.md docs/befund/befund-2026-09-09-tohoku-pegsel.md docs/befund/befund-2026-09-09-tohoku-vorhersage.md docs/befund/befund-2026-09-09-tohoku-gsn-geblockt.md
-->

# Handover — seismische Ortung + Tsunami-Kette

Übergabe für die nächste Sitzung. Gebaut wurde die seismische Ortung (ak135,
Positivkontrolle bestanden), der Tsunami-Schenkel der Tōhoku-Kette (Pegel
gemessen, Vorhersage 4/6 Wege) und der M9.1-Picker (gebaut, Grenze benannt).
**Committet als `0c43422`** (nachgemessen 2026-09-09, Folge-Sitzung) — die
Seismik-Sitzung liegt im Baum, nicht mehr offen.

## 1. Was gebaut wurde

### Ortung (seismisch)

- ak135 als kuratierte Klasse: Kernel `src/archivar/kernels/ak135.dat`
  (sha256 751889…feba4, Kennett/Engdahl/Buland 1995, TauP StdModels) +
  Strahlen-Tracer `src/archivar/ak135.rs` (τ(p)-Integration, lineare
  Interpolation in Radius, Quelltiefe über Reziprozität: Aufwärts-/Abwärts-
  Schenkel). Verifiziert gegen TauP-Referenz auf < 0,15 s und gegen die Sehne
  auf < 0,01 s. `p_travel(delta)` / `p_travel_depth(delta, h)`.
- `tools/measure/src/bin/quake_location_probe.rs` — Weltlinien über
  `body_fixed_to_icrs` zur gemeinsamen t_ref (die Rotation kürzt die Sehne,
  das Medium rotiert mit), 3D-Gitter (lat/lon/Tiefe 0–250 km), robuste
  3σ-Verwerfung. CLI: `--origin-unix --start --end`.
- Ergebnis: M7.8 Indonesien geortet −8,2240/121,3800 (20 km), **Offset ≈ 14,6 km**
  gegen Katalog, rms 1,887 s. Positivkontrolle bestanden.

### Tsunami-Kette (Tōhoku 2011)

- `tools/measure/src/bin/tohoku_pegsel_probe.rs` — NOAA CO-OPS `water_level` vs
  `predictions`, Anomalie = gemessen − Gezeit, Einsatz = |Anomalie| > 0,3 m.
  Sechs Pazifik-Pegel gemessen; die Ankunfts-Staffelung trägt ≈ 766 km/h
  (√(g·d), d ≈ 4700 m).
- `tools/measure/src/bin/tohoku_tsunami_vorhersage_probe.rs` — Großkreis-Slerp +
  GEBCO2020-Punktabfrage + Σ Segment/√(g·d), Tiefen-Boden 200 m. 4/6 Wege auf
  1–18 min (Crescent City +1,4, Honolulu +4,9); Adak +57 und Hilo +82
  über-langsam (Beugung um flache Features — der schnellste Weg, Fermat).
- Gemeinsame Module: `tools/measure/src/noaa_coops.rs` (CO-OPS-Helfer + Tests),
  `tools/measure/src/miniseed.rs` (miniSEED-Decoder, aus der Rayleigh-Probe
  gezogen, keine Dopplung).

### M9.1-Picker

- Bandpass 0,5–2 Hz + Erstbruch (5× Rauschboden) mit STA/LTA-Fallback in
  `quake_location_probe.rs` (6 Tests). Re-Lauf Tōhoku: Offset ≈ 600 km,
  rms 34 s — die Streuung (±30–50 s) bleibt. **Die Grenze ist die Automation**
  (emergenter Ersteinsatz der langen M9.1-Quelle), nicht das Modell.

## 2. Offene Steine (Reihenfolge des Rats)

1. **Eikonal** — schnellster Weg über ein Bathymetrie-Gitter (Dijkstra).
   Gemessen: die OpenTopoData-Punktabfrage nimmt nur Längengrade −180…180 und
   begrenzt die Batch-Größe; der saubere Weg ist ein Gitter-Download
   (ETOPO1/GEBCO NetCDF, 1-Bogenminute), kein Punkt-Batch-Werk.
2. **M9.1 / Hi-net** — NIED-Registrierung für die dichte Nah-Station; JP (JMA)
   trägt nur Metadaten (Wellenformen 2011 = HTTP 204, nicht offen archiviert).
3. **Finsternis-Manifestation** — die de440/442-Bins liegen in
   `data/ssd.jpl.nasa.gov/` (geteiltes Dateisystem), aber der Finsternis-Befund
   ist nicht in diesem Baum (parallele Sitzung); die Registrierung in
   `phi/sources.φ` braucht deren Provenienz-Entscheidung.
4. Stromboli (Vulkan-Lehrer), scharfe Tiefe (pP/sP), GEBCO-Kernel — pending.

## 3. Zustand des Arbeitsbaums (nachgemessen 2026-09-09, Folge-Sitzung)

- Die Seismik-Sitzung ist **committet** (`0c43422`, measure: seismic location +
  Tōhoku tsunami). Die Urschrift dieses Abschnitts („Nichts ist committet") war
  der Stand zum Schreibzeitpunkt und ist erledigt.
- Der Baum ist heute parallel aktiv: `docs/TODO.md`,
  `.github/workflows/gaia-xp-cdn.yml`, `src/mathematikerin/te.rs` (TE-Thread,
  Konflikt gelöst), `tools/harvest/src/bin/tap_compiler.rs` und weitere gehören
  parallelen Sitzungen — nicht dieser.
- Event-ID der Positivkontrolle nachgetragen: USGS `us6000tkt2` (mww 7.8); die
  Feld-Herkunft der sechs Befunde trägt jetzt
  `docs/befund/befund-2026-09-09-feldstandard-seismik.md`.

## 4. Register

Geschlossen/beschrieben: „Seismische Ortung" (gebaut, Positivkontrolle
bestanden), „ak135" (gebaut), „Beben-Tiefe" (gebaut, flach-bestimmt),
„Tōhoku-Pegel" (gebaut, gemessen), „Tōhoku-Vorhersage" (gebaut, teilweise
bestanden), „M9.1-Picker" (gebaut, Streuung bleibt). Offen: Eikonal, Hi-net,
Finsternis-Manifestation, Stromboli, scharfe Tiefe.
