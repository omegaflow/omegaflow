<!--
  title: Handover — Bau-Folge 57 (Stand 2026-09-16)
  session: Bau-Folge 57
  class: handover
  date: 2026-09-16
  sha256: c33df1988f5c0f6cb19a93ec899a4e4c0649e2d3489e83cbfd257edba1f79678
  status: live
-->
# Handover — Bau-Folge 57 (2026-09-16)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## TE-Gate-Arx — der verifizierende Lauf trägt die Messung; pending

- Der Lauf `35129638318` @ `1337c6c1` ist `in_progress` (gemessen 2026-09-16):
  `shift_sweep_n1000`, das n=1000-FPR-Gate und `block_sweep_n1000` sind **success**
  — der Block hält (rise ≤ 2pp), die FPR-Gate trägt; der KSG-Sweep läuft noch.
  Der zweite Lauf `35139361939` ist `pending` und **concurrency-blockiert** hinter
  `35129638318`; sein headSha ist `43af521f` (nicht `e28abbdf` — Register in
  `docs/zustand/external-state.md` korrigiert). (Schritt: `gh run view 35129638318`
  — grün: Gate zu; rot: der Assert nennt die Zelle; hält keine Blocklänge rise ≤ 2pp,
  `multi_force_te_probe.rs:74` auf Arx, Block ausmustern.)

## las — Konsument gebaut; Manifestation und CRS-Äste offen

- Der φ-Konsument ist gebaut (Rat-Verdikt BUILD): `src/archivar/geo.rs` (`MAGIC_LAS`,
  `COMP_LAS_*`, `magic_of`/`comp_max`), `src/archivar/extract.rs` (Serien-Arm +
  Komponentennamen), `src/archivar/las/las_series.rs` (LAS1-Bin), neuer
  `tools/harvest/src/bin/las_compiler.rs`, neues `.github/workflows/las-cdn.yml`;
  die Quelle `usgs-lidar-public.s3.amazonaws.com` (USGS 3DEP EPT, Root-Tile
  `AK_BrooksCamp_2012`, CRS EPSG:3857) ist in `phi/sources.φ` registriert.
  (Schritt: nach Push `gh workflow run las-cdn.yml` — die CDN-Manifestation; erst
  danach greift die `url`-Zeile.)
- Offen: NOAA NOS Coastal Lidar (NAD83/UTM 4N + MLLW) — inverse Transverse-Mercator
  (GRS80) und die MLLW→Ellipsoid-Kette sind ungebaut; der Compiler lehnt diese CRS
  ehrlich ab (Zähler statt Fabrikation). (Schritt: die beiden Projektionsketten in
  `tools/harvest/src/bin/las_compiler.rs` bauen, dann die zweite `format las`-Zeile.)
- Offen: Vertikaldatum des USGS-Tiles (keine VLR-Angabe → z als gelieferte Höhe, die
  Geoidundulation zum WGS84-Ellipsoid bleibt ungemessen) und UTM-Äste
  (EPSG 326xx/327xx) + beliebiges WKT. (Schritt: `sgrep "crs" tools/harvest/src/bin/las_compiler.rs`.)
- Offen: räumliche Promotierung — x/y/z laufen als Serienkomponenten (`las_x_icrs_m` …),
  nicht als `StateVector`-Anker. (Schritt: den Serienpfad in `src/archivar/main_flow.rs` prüfen.)

## ODR — Voyager-Serie sharded, Lauf in_progress

- Lauf `35143340703` lädt die 484 `.ODR`-Dateien in ~1-GiB-Shards; der Serien-Job ist
  `in_progress` (gemessen 2026-09-16), noch keine Shard hochgeladen. (Schritt:
  `gh run view 35143340703`, dann Shard-Zeilen in `phi/sources.φ`.)
- Offen: Galileo 12-bit — der Dekoder-Pfad erzeugt Records (Tests grün), aber die
  gemessene PPI-Quelle (`pds-ppi.igpp.ucla.edu/annex/`) ist 8-bit; die SIS-Treue des
  12-bit-Pfads bleibt ohne realen 12-bit-Record `pending`. (Schritt:
  `archive_search --verdict <PPI-ODR-URL>`.)
- Offen: Nebenbefund `90320204.LBL` (GO-JS 1999) trägt `RECORD_BYTES = 566` — ein
  anderes ODR-Layout, das `galileo_odr.rs` (2666) nicht parst. (Schritt: das
  566-B-Layout messen, bevor es gebaut wird.)
- Geschlossen (gemessen 2026-09-16, grind-flash): `year_full` deckt 92..99 — die
  gemessene Archiv-Spanne ist 1992–1999, kein Record fällt durch; Voyager
  Decimation>1 ist im Code verifiziert (`voyager_odr.rs:300` + Test).

## CI-Format-Gate — fremde Linien

- Fremd unformatiert bleiben `src/archivar/vtscat.rs`,
  `tools/measure/src/bin/{aia_ladder_probe,trishuli_gauge_probe}.rs`,
  `tools/utils/src/bin/archive_search.rs` und die ernte-eigenen harvest-Compiler;
  `src/gate/commit_gate.rs` ist konform (gemessen 2026-09-16). (Schritt: die
  jeweilige Linie formatiert ihre eigene Datei; Post-Zeile in `post.md`.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
