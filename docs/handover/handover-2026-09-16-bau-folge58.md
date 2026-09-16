<!--
  title: Handover — Bau-Folge 58 (Stand 2026-09-16)
  session: Bau-Folge 58
  class: handover
  date: 2026-09-16
  sha256: 66d0b367dc51b56c8e0ac3321fff9f3297b141037b84994227f1011a4ec3dae9
  status: live
-->
# Handover — Bau-Folge 58 (2026-09-16)

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

## TE-Gate-Arx — Verifikationslauf noch offen; pending

- Der Lauf `35129638318` @ `1337c6c1` ist `in_progress` (gemessen 2026-09-16):
  `shift_sweep_n1000`, das n=1000-FPR-Gate und `block_sweep_n1000` sind **success**
  — der Block hält (rise ≤ 2pp), die FPR-Gate trägt; der KSG-Sweep läuft noch.
  Der zweite Lauf `35139361939` ist `pending` und **concurrency-blockiert** hinter
  `35129638318` (headSha `43af521f`). (Schritt: `gh run view 35129638318` — grün:
  Gate zu; rot: der Assert nennt die Zelle; hält keine Blocklänge rise ≤ 2pp,
  `multi_force_te_probe.rs:74` auf Arx, Block ausmustern.)

## las — CRS-Äste gebaut; Manifestation, MLLW, Vertikaldatum, zweite Quelle offen

- Der Compiler `tools/harvest/src/bin/las_compiler.rs` invertiert seit diesem Atom
  geographic (EPSG:4326), Web Mercator (3857) und **UTM 326xx/327xx** (GRS80
  inverse Transvers-Mercator, Zentralmeridian aus der Zone); der WKT-Root-Tiefen-Scan
  liest nur die CRS-eigene EPSG-ID (verschachtelte Basis-/METHOD-IDs lösen nichts
  auf). Dabei eine **Fabrikation gefixt**: der alte `wkt.find("ID[\"EPSG\",")` las
  die erste ID irgendwo und hätte ein `BASEGEOGCRS[…ID[4326]]` ohne CRS-Eigen-ID als
  geographisch aufgelöst (UTM-Meter als Grad) → Gate-Fixture + Gate-Test im selben
  Atom. (Schritt: keiner — im Commit.)
- Offen: **MLLW→Ellipsoid-Kette** — kein gemessenes Geoid-/Tidendatum-Modell im Baum
  (mllw/vdatum/geoid einzeln gemessen); der Compiler lehnt ehrlich ab (Zähler statt
  Fabrikation). (Schritt: Tidendatum/Geoid-Modell beschaffen oder descopen.)
- Offen: **Vertikaldatum** des USGS-Tiles (keine VLR-Angabe → z als gelieferte Höhe)
  und **beliebiges WKT ohne EPSG-ID** (bleibt unresolved). (Schritt:
  `sgrep "crs" tools/harvest/src/bin/las_compiler.rs`.)
- Offen: **zweite `format las`-Zeile** (NOAA NOS Coastal Lidar) — keine registrierte
  Quelle. (Schritt: `sgrep "format las" phi/sources.φ`.)
- Offen: **CDN-Manifestation** der LAS-Quelle — `gh workflow run las-cdn.yml` nach
  dem Push dieses Atoms (der Lauf checkoutt HEAD; die CRS-Äste sind erst danach im
  Lauf). (Schritt: nach Push `gh workflow run las-cdn.yml`.)
- Der räumliche Promotierungs-Punkt ist **descoped by measurement** (2026-09-16): der
  Kontrakt kennt keinen v-freien Positions-Anker, LAS trägt kein v → `StateVector`
  (v=0) wäre Fabrikation; die Serienkomponenten `las_x/y/z_icrs_m` sind die korrekte
  Messung. Der Befund steht als `note` an der las-Quelle (`phi/sources.φ`).

## ODR — Voyager-Serie sharded; Galileo 12-bit und 566-B-Parser offen

- Geschlossen (gemessen 2026-09-16): Lauf `35143340703` = **success**; 14 Shards
  `voyager_odr_s0..s13.bin` auf der CDN; die Serien-Registrierung steht in
  `phi/sources.φ` (14 Blöcke), die Einzeldatei `voyager_odr.bin` (Teilmenge —
  `C0XR13AA` ist im INDEX enthalten, gemessen) wurde ersetzt (A = A, kein
  Doppelzählen); der Serien-Leser-Arm ist verdrahtet (`extract.rs:53`,
  `main_flow.rs:2123`). — im Commit.
- Offen: **Galileo 12-bit** — der Dekoder-Pfad ist SIS-treu (Figure 4, gemessen),
  aber die gemessene PPI-Quelle (`pds-ppi.igpp.ucla.edu/annex/`) trägt nur 8-bit
  (LBL-NOTE + RESOLUTION-Bit); kein realer 12-bit-Record gefunden → `pending`.
  (Schritt: realen 12-bit-ODR in anderen DSN-/NSSDCA-Archiven suchen
  (`archive_search --all "Galileo ODR 12-bit"`) oder descopen.)
- Offen: **566-B-ODR-Layout** (`90320204.LBL`, GO-JS 1999) ist gemessen — 566 B =
  166-B-Header + 100 × 4 B (8-bit AD1–AD4), 200 Sa/s, 12701 Records
  (`galileo_odr.rs` parst 2666 B). (Schritt: Parser-Ast für das 566-B-Layout bauen
  oder als descoped messen.)

## CI-Format-Gate — fremde Linien

- `docs/handover/post.md` ist aktualisiert (Lauf `35148755293` @ `83fa92ec`: 14 rote
  Dateien, nach Owner gruppiert; die bau-eigenen `src/archivar/las/mod.rs`,
  `src/mathematikerin/te.rs`, `tools/measure/src/bin/pcmci_class_benchmark.rs` sind
  in diesem Atom formatiert). Fremd bleiben: ernte `src/archivar/{demeter,tests,
  vtscat}.rs`, `src/mathematikerin/s2.rs`, `tools/harvest/…/kcdc_compiler.rs`;
  forschung `src/archivar/{fetch,parse}.rs`, `tools/measure/…/{band_amplitude_probe,
  corona_event_probe,s2_weberin_probe}.rs`; `tools/utils/src/bin/archive_search.rs`.
  (Schritt: die jeweilige Linie formatiert ihre eigene Datei.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
