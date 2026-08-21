<!--
  title: Der Sonnen-Pfad — SSI-Ernte & Solar-TE-Kanäle (Nadel III) mit Stabilitätsprotokoll für die Aixsponza-Demo
  class: handover
  date: 2026-08-21
  sha256: fe4735167dd3a25d08fdc5a0b32598487aad7a95c298f96dd58866897cd7d220
  status: live
  see-also: docs/handover/handover-2026-08-21-ncei-ssi-hdf5.md docs/handover/handover-2026-08-21-offene-atome.md
-->
# Der Sonnen-Pfad — SSI-Ernte & Solar-TE-Kanäle (Nadel III)

Registriert 2026-08-21. Selbsttragend — interpretierbar mit null Vorkontext.
Der Auftrag ist nicht die Ausführung; ausgeführt wird erst auf das Wort des
Operators. Dieses Dokument ist der Einstieg der Session; der Detailplan für
Faden A liegt in `docs/handover/handover-2026-08-21-ncei-ssi-hdf5.md`.

## Ziel

Parallel-Aufbau des visuellen und epistemischen Sonnen-Pfads. Die Sonne muss
als em-Feld leuchten (SSI) und ihre zeitlichen Kanäle müssen als Oszillatoren
für die TE-Maschine (Nadel III: Koronaheizung) available sein.

Beide Fäden laufen parallel — Faden A ist bereits als Handover registriert
und trägt den Detailplan; Faden B wird hier neu aufgespannt. Das
Stabilitätsprotokoll (Abschnitt 5) hat Vorrang vor beiden.

## 1. Kontext — die fünf Nadeln

| Nadel | Rätsel | Stand |
|---|---|---|
| I | Dunkle Materie | pending — Voxel-Residuum wartet auf Gaia DR4 (Dez 2026) |
| II | Flyby-Anomalie | pending — Live-Flybys JUICE/Europa Clipper (Sep/Dez 2026) |
| III | Koronaheizung | jetzt rechnbar — Solar-APIs live, TE-Maschine läuft |
| IV | LAIC / Erdbeben | pending — Offline-Stapelung gegen Null-Ensemble |
| V | Technosignaturen | pending — IRAS/ZTF-Cross-Match-Compile |

Nadel III ist die einzige, die heute live rechnet: GOES/F10.7 pumpen täglich
frische Daten; die Takens-TE-Maschine (`te_compute`, WGSL) läuft. Sobald die
Kanäle eingespeist sind, sammelt der Ring-Buffer und der kausale Pfeil schlägt
aus. Das erste Video löst Nadel III; die übrigen Nadeln bleiben im Register.

## 2. Faden A — das visuelle Sonnenlicht (SSI-Ernte)

Der vollständige, selbst vermessene Detailplan (HDF5-Format, Dateinamen,
Zugang, Reader-Umfang, Gates) steht in
`docs/handover/handover-2026-08-21-ncei-ssi-hdf5.md` — dieser Faden wird dort
abgearbeitet, nicht hier dupliziert. Kern:

1. HDF5/netCDF-4-Aufbau der NCEI-SSI-Datei vermessen (Magie `89 48 44 46
   0d 0a 1a 0a`; ~172 KB je Standard-Resolution-Monatsdatei).
2. Minimaler std-only Reader für das netCDF-4-Profil (`src/hdf5.rs` neu;
   `src/netcdf.rs` liest nur classic). DEFLATE über den existierenden
   `src/inflate.rs`.
3. `spectral_compiler` erweitern (oder eigener Harvest-Binary): `.nc` lesen,
   `wavelength` + `ssi` extrahieren, über `bins_from_lambda_rows`
   (`src/spectral.rs:16`) nach `spectra.bin` schreiben, `--ci-mode` → CDN
   (ersetzt den 404).
4. Register: die pending-Zeile in `ledger.φ` und `TODO.md` schließen,
   AGENTS.md-Spektralzeile und `src/spectral.rs`-Kopfzeile mitschärfen.

## 3. Faden B — die epistemischen Sonnen-Kanäle (Live-APIs für Nadel III)

### 3.1 Ist-Stand (gemessen 2026-08-21)

In `phi/sources.φ` sind bereits eingetragen:

- GOES X-Ray (0.05–0.4 nm: `sources.φ:92–100`, flux W/m², `force em`,
  `inverse-square`) — die 1–8 Å- und 0.5–4 Å-Kanäle sind über die xrays-Datei
  abzudecken oder als eigene Einträge nachzutragen (prüfen, welche Bänder die
  Zeile tatsächlich trägt).
- GOES EUV (`euvs-7-day.json`, `sources.φ:230`).
- RTSW Magnetometer (`rtsw_mag_1m.json`, `sources.φ:102`) und RTSW Wind
  (`rtsw_wind_1m.json`, `sources.φ:108`) — speed, density, Bz sind dort zu
  prüfen und ggf. zu ergänzen.

**Fehlt laut Register:** F10.7 Radio Flux (Penticton, Einheit sfu =
10⁻²² W m⁻² Hz⁻¹). Dieser Kanal ist neu einzutragen.

### 3.2 Auftrag

1. `phi/sources.φ`: die Solar-Kanäle als unabhängige, blitzschnelle
   Zeitreihen sicherstellen. Oszillatoren am Punkt **`at sun`**.
2. Quellen: GOES X-Ray (1–8 Å und 0.5–4 Å), GOES EUV, F10.7 (Penticton,
   nachtragen), RTSW/ACE Sonnenwind (speed, density, Bz).
3. Konfiguration je Kanal: Frame `at sun`; Force `em` (force_type 0 — die
   Messung IST elektromagnetische Strahlung bzw. das Plasmafeld wird über
   seine em/Teilchen-Signatur gemessen; die Force-Gate-Litmus-Prüfung je
   Kanal führen und das Ergebnis ins Register); `tau` deklariert (τ-Gate:
   ohne tau keine Samples); SI-Einheiten: W/m² (X-Ray, EUV), sfu (F10.7),
   km/s (speed), n/cm³ (density).
4. Fluss in die TE-Maschine: die Kanäle müssen als unabhängige Zeitreihen in
   den `probe_ring` der GPU fließen (`src/mathematikerin.rs:1410`), damit
   `te_compute` die kausale DAG berechnet — TE(F10.7 → X-Ray) vs.
   TE(X-Ray → F10.7). Der bestehende skalare TE-Pfad (`transfer_entropy_lag`,
   die Probe) bleibt unberührt.

## 4. Constraints

- **std-only** in Rust (std + curl; kein `extern crate hdf5`, kein FFI, kein
  Shell-out).
- **Name = Implementation** — keine Docstrings, keine Kommentare.
- `cargo check` muss **0 Fehler / 0 Warnungen** sein (beide Features);
  Warnungen sind Codrot, nie mit `#[allow]` oder Unterstrich stummlegen.
- **0-Kanon (korrigierte Fassung):** Fällt eine API aus, trägt der Kanal den
  Wert nicht → der Sample wird **übersprungen** (fehlt), nie als 0.0
  fabriziert. 0.0 fließt nur, wenn die Quelle selbst einen physikalisch
  echten Nullwert liefert (null-echt). Die mündliche Anweisung „Wert 0.0 an
  diesem Punkt, keine Fabrikation" wäre genau eine Fabrikation — hier gilt
  der Kanon: Ausfall = fehlt, nicht null.
- Einheitenformat SI; Plausibilität positiv testen (`is_finite() && > 0`),
  kein `unwrap_or(0.0)` für physikalische Werte.
- Die Sonnenkanäle sind Oszillatoren unter Oszillatoren — kein
  First-Class-Objekt, kein Sonderpfad in der Membran.

## 5. Stabilitätsprotokoll — Demo am 2026-08-22

Am Folgetag kommt der Aixsponza-Geschäftsführer. Priorität ist ein
bootfähiger, flüssiger, konzeptionell sauberer Zustand — nicht die
Fülle der Features.

1. **Fallback ist der Ist-Zustand:** ~1.7 Mio Gaia-Sterne als em-Felder mit
   echten Farben, 9 Kräfte, HUD mit TE — bereits jetzt eine vollständige
   Demo. Der Fallback gilt, solange die neuen Fäden nicht stabil sind.
2. **Deadline 23:30 (2026-08-21):** Kompiliert der neue Code bis 23:30 nicht
   zu 100 % und läuft `cargo run` fehlerfrei, wird er auf einen Branch
   gelegt; der Arbeitsbaum bleibt im lauffähigen Zustand. Kein git reset,
   keine halbgaren Features auf dem Hauptzweig. A = A — das System muss
   morgen früh booten und flüssig laufen.
3. **Die Sonne ist Bonus, kein Muss.** Sie wird nur gezeigt, wenn sie 100 %
   stabil leuchtet (SSI auf dem CDN, `spectra.bin` geladen, Membran rot-glut
   bei 1 AU, Integral ≈ 1361 W/m²).
4. **Verifikation des stabilen Zustands:** `cargo check` 0/0;
   `OMEGAFLOW_HIDDEN=1 cargo run` — die `φ window:`-Zeile (das maschinen-
   lesbare HUD-Zwillings-Pendant) auf stderr lesen: `te thr tau pe state
   focus keys perm flow gen`. Kein Test öffnet ein Fenster oder strahlt.

## 6. Gates & Abschluss

- Jede abgeschlossene Einheit ist ein Commit; Register-Update (TODO.md /
  ledger.φ) im selben Commit. Ein Commit je Einheit.
- Manuelle Verifikation nach AGENTS.md: den Rust → JS → WGSL-Kontrakt
  Zeile für Zeile nachlesen, wenn Samples oder Feldbedeutungen berührt sind;
  WGSL-Shader gegengelesen; Kantenfälle (leere Arrays, fehlende Spalten,
  ttl-Ablauf, Absorption 0/1).
- Faden A: `spectra.bin` roundtrip-geprüft (`parse_spectral_bin`), ~4300
  Bänder, Integralsumme ≈ 1361 W/m² bei 1 AU.
- Faden B: jeder Kanal liefert echte Samples (Zeile im stderr-Log, kein
  erfundenes Feld); der 404-Befund von `spectra.bin` ist dokumentiert, der
  `pending`-Eintrag geschlossen.
- Nach eigenem Commit dieses Handover und das NCEI-SSI-Handover nach
  `/home/johannes/projects/archive/handover/` archivieren (Regel in
  AGENTS.md).

## Nicht anfassen

Die Membran/`fs`-Rendering-Physik (nur em trägt Farbe, nicht-em neutral),
`src/te.rs` (der kanonische CPU-Referenzpfad), der `wm2_1au`-
Luminositäts-Pfad (erledigt, `ledger.φ:582`), die OMEGAFLOW_HIDDEN-
Radiator-Stille, die vier offenen Atome aus
`handover-2026-08-21-offene-atome.md` (eigenes Handover, eigene Session).
