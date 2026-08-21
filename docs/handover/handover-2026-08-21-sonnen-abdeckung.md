<!--
  title: Die Sonne lückenlos — SSI-Ernte, GOES-XRS- und OMNI2-Serien
  class: handover
  date: 2026-08-21
  sha256: 2ea6b819b06d9444846430d211bf22eec2e85e116d3fc5a82151b36f2ba7bcf8
  status: live
  see-also: docs/surveys/survey-2026-08-21-sonnen-abdeckung.md docs/handover/handover-2026-08-21-ncei-ssi-hdf5.md docs/handover/handover-2026-08-21-sonnen-pfad-solar-te.md docs/handover/handover-2026-08-21-offene-atome.md
-->
# Handover: Die Sonne lückenlos — SSI-Ernte, GOES-XRS- und OMNI2-Serien

Registriert 2026-08-21. Die nächste Session liest genau dieses eine Dokument
und beginnt. Selbsttragend — interpretierbar mit null Vorkontext. Der Auftrag
ist nicht die Ausführung; ausgeführt wird erst auf das Wort des Operators.
Die vermessene Grundlage ist `docs/surveys/survey-2026-08-21-sonnen-abdeckung.md`
(die Recherche, am 2026-08-21 vollständig live verifiziert).

## Einstieg

```bash
cd /home/johannes/projects/omegaflow
git status                       # fremde Arbeit benennen, nichts übernehmen
cargo check                      # muss 0/0 sein (beide Features)
sed -n '1,70p' docs/surveys/survey-2026-08-21-sonnen-abdeckung.md
```

Referenzen (stehend): `docs/surveys/survey-2026-08-21-sonnen-abdeckung.md`
(alle URLs, Fenster, Dateinamen — am 2026-08-21 gemessen),
`docs/handover/handover-2026-08-21-ncei-ssi-hdf5.md` (Faden A, Detailplan),
`docs/handover/handover-2026-08-21-sonnen-pfad-solar-te.md` (Faden B),
`docs/handover/handover-2026-08-21-offene-atome.md` (Sample-Budget, Atom 1).

## Auftrag — die Sonne als Serie

Der Survey zeigt: Gravity ist lückenlos (1599→3000); das bolometrische Licht
(SSI) wartet auf die Ernte (`spectra.bin` ist 404 auf dem CDN); GOES-XRS,
F10.7, Mg II, SSN und OMNI2/Sonnenwind existieren als vollständige Serien bei
NCEI/CDAWeb/SPDF/LISIRD — sie fehlen nur im Feld. Vor 1610 kein direktes Licht
(NNL-Modell, 0 honored).

Die Reihenfolge ist Teil des Auftrags (bedingt sich durch das Sample-Budget):

### Atom 1 — Faden A: HDF5-Reader + NCEI-SSI-Ernte (die leuchtende Sonne)

Der größte einzelne Lücken-Schließer: 416 Jahre Licht. Detailplan steht in
`docs/handover/handover-2026-08-21-ncei-ssi-hdf5.md`.

1. HDF5-Reader (`src/hdf5.rs`): Der Baum trägt bereits einen uncommitteten
   Entwurf (siehe Einstieg, `git status`). Prüfen, ob er die NCEI-SSI-Dateien
   parst (Proben im Katalog: `phi/pipeline/katalog/ncei_ssi/` — monthly
   187405, preliminary 2026-04/06, `filters.h5`). Magie `89 48 44 46 0d 0a 1a 0a`.
2. `spectral_compiler --input-nc <file.nc>` (oder kleiner `ncei_ssi_compiler`):
   wavelength + ssi extrahieren, `bins_from_lambda_rows`
   (`src/spectral.rs:16`) → `spectra.bin`, `--ci-mode` → CDN
   (ersetzt den 404).
3. Verifikation: `parse_spectral_bin`-Roundtrip, ~4300 Bänder,
   Integralsumme ≈ 1361 W/m² bei 1 AU.
4. Register: `ledger.φ`-pending-Zeile + `TODO.md` schließen, AGENTS.md- und
   `src/spectral.rs`-Kopfzeile mitschärfen.

### Atom 2 — Faden B: Solar-Kanäle in den probe_ring (Nadel Ⅲ)

Detailplan in `docs/handover/handover-2026-08-21-sonnen-pfad-solar-te.md`
(Faden B, §3): GOES X-Ray (beide Bänder), GOES EUV (304/284), F10.7 müssen als
unabhängige Zeitreihen in den `probe_ring` der GPU fließen
(`src/mathematikerin.rs:1410`), damit `te_compute` die kausale DAG rechnet —
TE(F10.7 → X-Ray) vs. TE(X-Ray → F10.7). Der skalare TE-Pfad
(`transfer_entropy_lag`) bleibt unberührt.

### Atom 3 — GOES-XRS-Serie als Compiler (1995–2020, die 1974–heute-Lücke)

Quelle (am 2026-08-21 verifiziert): NCEI
`…/goes-space-environment-monitor/access/science/xrs/` —
`xrsf-l2-avg1m_science` (1-min-Mittel, netCDF-4/HDF5, 1 Datei/Tag):
goes08 1995–2003, goes10 1998–2009, goes12 2003–2007, goes13 2013–2017,
goes14 2009–2020, goes15 2010–2020. Plus `full/` 3-s-Daten 1974–2020
(netCDF classic `CDF\x01` + CSV, `gNN_xrs_3s_YYYYMMDD_YYYYMMDD.nc`).

- Eigenes Atom `goes_xrs_compiler` analog `bia_efield_compiler`: Tages-NetCDFs
  → Serie, `at sun`, `wm2_1au`-Konvention, Band-Trennung via `where energy`
  (0.05–0.4 nm / 0.1–0.8 nm, ledger.φ:578).
- **Sample-Budget-Vorbehalt:** 1-min über ~25 a ≈ 25 M Werte übersteigt
  `MAX_SAMPLES = 1 << 22` (`src/archivar.rs:9038`); der Rebuild kappt
  epoch-absteigend. Vor dem Compiler die Auflösung entscheiden
  (1-min-Mittel auf den CDN, im Feld geglättet/daily) — siehe
  `handover-2026-08-21-offene-atome.md` Atom 1 (Messung zuerst, dann Kataloge).

### Atom 4 — OMNI2-Full-Serie (1963–heute)

Der Sonnenwind-Kanal als Serie — Grundlage für die Nadel-Ⅲ-Archive
(Bz/OMNI über 90 d). Quelle (verifiziert): CDAWeb-HAPI
`OMNI2_H0_MRG1HR`, start **1963-01-01** → 2026-08-06; die 7 Parameter
(V1800, N1800, T1800, BX/BY/BZ, Pressure) und ihre Force-Zuordnung stehen
bereits im Live-Block (`phi/sources.φ:518`). Alternativ
`spdf.gsfc.nasa.gov/pub/data/omni/low_res_omni/omni2_YYYY.dat` (64 Dateien,
1963–2026, ASCII).

- HAPI-Window-Ernte in Jahres-Schritten → Serie; E1800 bleibt entfernt
  (derived, ledger.φ:327).
- Kein Doppel mit dem Live-Block: Live deckt die Gegenwart, die Serie die
  Historie (Überlappung deduplizieren).

### Atom 5 — F10.7-Historie, Mg II, SSN (Compilers)

- F10.7: NCEI `pent_noontime-flux_1947.txt … _2026.txt` (80 Jahresdateien)
  oder LISIRD `noaa_radio_flux` (1947–, daily, keyless). Live-Kanal
  `f107_cm_flux.json` bleibt.
- Mg II: LISIRD `bremen_composite_mgii` (1978–).
- SSN: SILSO `SN_d_tot_V2.0.txt` (1818–, daily) / SWPC
  `observed-solar-cycle-indices.json` (1749–, monthly, bereits im Block).

## Constraints

- **std-only** in Rust (std + curl; kein `extern crate hdf5`, kein FFI, kein
  Shell-out wie `ncdump`).
- **Name = Implementation** — keine Docstrings, keine Kommentare.
- `cargo check` **0 Fehler / 0 Warnungen** (beide Features); Warnungen sind
  Codrot, nie mit `#[allow]` oder Unterstrich stummlegen.
- **0-Kanon:** Ausfall = Sample-Skip (fehlt), nie 0.0-Fabrikation; 0.0 nur bei
  physikalisch echtem Nullwert. `is_finite() && > 0` als Plausibilitäts-Gate,
  kein `unwrap_or(0.0)` für physikalische Werte.
- Die Sonnenkanäle sind Oszillatoren unter Oszillatoren — kein
  First-Class-Objekt, kein Sonderpfad in der Membran.

## Gates & Abschluss

- Jede abgeschlossene Einheit ist ein Commit; Register-Update (TODO.md /
  ledger.φ) im selben Commit. Ein Commit je Atom.
- Manuelle Verifikation nach AGENTS.md: die Rust → JS → WGSL-Kontrakt
  Zeile für Zeile nachlesen, wenn Samples oder Feldbedeutungen berührt sind;
  WGSL-Shader gegengelesen; Kantenfälle (leere Arrays, ttl-Ablauf,
  Absorption 0/1).
- Atom 1: `spectra.bin` roundtrip-geprüft (~4300 Bänder, ≈1361 W/m²).
- Atom 3/4: Serien liefern echte Samples (Zeile im stderr-Log), keine
  erfundenen Felder; der 404-Befund von `spectra.bin` ist dokumentiert.
- Nach eigenem Commit die drei Handover dieses Tages nach
  `/home/johannes/projects/archive/handover/` archivieren (Regel in
  AGENTS.md), wenn ihre Atome vollständig geschlossen sind.

## Nicht anfassen

Die Membran/`fs`-Rendering-Physik (nur em trägt Farbe, nicht-em neutral),
`src/te.rs` (der kanonische CPU-Referenzpfad), der `wm2_1au`-
Luminositäts-Pfad (erledigt, `ledger.φ:582`), die OMEGAFLOW_HIDDEN-
Radiator-Stille, die vier offenen Atome aus
`handover-2026-08-21-offene-atome.md` (eigenes Handover, eigene Session),
die uncommitteten HDF5-Reader-Entwürfe anderer Sessions im Baum
(`git status` benennen, nichts übernehmen).
