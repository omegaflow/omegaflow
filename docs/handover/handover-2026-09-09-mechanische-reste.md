<!--
  title: Thematisches Handover — mechanische Reste
  class: handover
  date: 2026-09-09
  sha256: 51486e207c53bd9d3cdff55237849818a6eeedab98d75814750be3def7af1b99
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-mechanische-reste.md
-->
# Thematisches Handover — mechanische Reste

Stehendes Register der mechanischen und forschenden Reste — der konsolidierte
Follow-up der aufgelösten Aufträge, der Nadeln, der Weberin-Linien und der
CDN-/Source-Port-Reste. Jeder Eintrag trägt einen gemessenen offenen Punkt;
Geschlossenes trägt Git.

## Geschlossen (trägt Git)

Die CDN-Manifestations-Reste wurden 2026-09-09 gemessen und geschlossen (Rat,
fünf Stimmen einstimmig):

- **gebco 56-Byte-Stub** — `gebco_bathymetry.gbco` (56 B = 2 Records) frisch
  nachkompiliert, byte-identisch (cmp 0) zum CDN-Asset: echter 2-Punkt-Zeuge
  aus dem Workflow, kein Uploadfehler.
- **2MASS** — `2mass_binary.fp01` CDN 200 / 27 MB (Run 34316774357);
  Operator-Wort war erteilt (handover-allwise-2mass-ernte §5).
- **planck_dust_av** — registriert (sources.φ:5540) + CDN 200.
- **eve2011_lines.bin** — CDN 200; Maßkanal, nicht Oszillator
  (`corona_ladder_probe` liest per Pfad, magic `EVL1`) — kein url-line fällig.
- **aia2014_fullyear.bin** — descoped: Monatsbins (aia2014_10.bin CDN 200) sind
  das Dauerheim; der fullyear-Merge für 2014 wurde nie gebaut.
- **omni2_raw/** — descoped: kompilierte Serie (sources.φ 1893/1905/1917 + CDN
  200); Roh-CSVs aus CDAWeb-HAPI reproduzierbar.
- **goes15*/** — descoped: drei Jahres-Tars (CDN 200); loose .nc sind entpackte
  Arbeitskopien.
- **galileo_tdf_cache_*.TDF** — descoped: PDS-Origin lebt (200 gemessen) +
  `galileo_resid.bin` (CDN 200); rohe TDF re-ernterbar, keine einzige Kopie.
- **ck90342a_plt.bc** — descoped: konsumentenlos (kein Treffer im Baum, nicht in
  der 9er-Kernelliste des Workflows).

Die Register-Pflichten wurden 2026-09-09 gemessen und geschlossen (Rat gehört;
je Linie die gemessene Stelle):

- **gate-bereinigung-abschluss** — `04408e4` löste die benannten
  Fabrikationsstellen (fetch/te/thermochem/home_scan/register_verify); heute
  nachgemessen: keine der Stellen im Baum, `claim_verify.rs` existiert nicht
  mehr (Bin-Satz trägt `claim_reader`).
- **drei Techno-Kanäle** — geschlossen: die drei Befunde (done, je eigene
  sha256, je `antwortet-auf`) liegen committet im Baum — letzte Berührung
  `8d2d40b` (Register-Sweep): docs/befund/befund-techno-narrowband-scan.md,
  docs/befund/befund-techno-atmosphaeren-gase.md,
  docs/befund/befund-negativ-fuzzy-techno.md.
- **extern-neutrino-cr-routen** — geschlossen: Routen-Verdikte stehen in
  docs/surveys/survey-2026-09-07-weberin-thread-matrix.md (L92–94: gebaute
  Compiler amon/icecat/ANTARES/KM3NeT/Auger; L172: TA, Super-K, JUNO, LHAASO,
  HAWC not-published gemessen — externer Zustand, kein offener Posten).
  `icecat_compiler.rs` + `icecat-cdn.yml` im Baum (letzte Berührungen
  `a21d4c8` / `825e25b`).
- **glm-uebernahme** — geschlossen: Paper auf main —
  docs/paper/gic-causal-driver.md (letzte Berührung `3b6a50c` — GIC-Re-Messung)
  und docs/paper/lead-geometry-direction.md (`1fe5870`); die Blatt-Anker sind
  grep-gemessen (1378↔1260 im gic-Blatt, PhysioNet-201/202 im
  lead-geometry-Blatt L175/202); die solar-cycle-Punkte sind via `8e1639a` +
  `ec6cca6` aufgelöst (im Auftrag registriert). Die Verifikations-Datei
  `docs/audit/glm-verifikation-2026-08-28.md` existiert nicht und war nie
  committet — der Verlust ist gemessen und registriert, keine Fabrikation.
- **cog-quelle** — geschlossen: Messkette verifiziert (COG-Quelle +
  std-only-15-bit-Reader + UTM; Bhote-Koshi wird als Wasser erkannt, 150
  Pixel); Seebaseline an den OSM-Punkten 0 (gemessen 08-12, NDWI max 0.071);
  kein offener Posten im Blatt.
- **iapetus-scan** — geschlossen: docs/befund/befund-2026-09-09-iapetus-literatur-scan.md
  — Ausgang **offen**: die Lücke ist bestätigt (arXiv `dark matter AND
  Iapetus` = 0 Treffer; Iorio/Pitjeva-EPM/DM-Klumpen-PBH messen je andere
  Proben). Front B (Horizons + N-Body) ist gated frei und bleibt eine eigene,
  benannte Messung.
- **phantom-island-williston-bc** — geschlossen:
  docs/befund/befund-2026-09-09-phantom-island-williston-bc.md — gemessen: ein
  echtes, treibendes Torf-/Holz-Floß (70×140 m, windgetrieben, Rekord-
  Stauwasserstand als Freisetzungs-Bedingung), 08-15 am Ospika-Arm-Nordeingang
  verortet; kein kartografisches Phantom. force_type / ICRS / exakte
  Koordinaten bleiben `pending` (kein Kraft-Medium der 9er-Liste trägt eine
  treibende irdische Masse).
- **verify-references — CASE 5 (Regel-Entscheidung, Rat, einstimmig)** — die
  dokumentierten Ausnahme-Klassen stehen in dieser Zeile und sind im Code
  geformt (tools/register/src/bin/path_reference_scan.rs): Archiv-Ordner
  (`/archiv/`) und docs/reference/ sind historische Anker, keine
  Rückverfolgung; URL-Schwänze nach `://` tragen keinen lokalen Pfad;
  `.rs`-Dateien tragen Beispiele, keine Referenzpflicht; AGENTS.md trägt die
  eine physische Adresse als Einzel-Segment-Allowlist. Kalibrierung (gemessen
  2026-09-09): 17 ABS-Stellen im Baum liquidiert oder klassifiziert → 0 ABS;
  die Regel ist in CI verdrahtet (ci-check.yml testet jetzt
  `omegaflow-register`).
- **verify-references — CASE 6 (Regel-Entscheidung, Rat, einstimmig)** —
  dokumentierte Grenze: die Prüfpflicht gilt see-also- und Link-Referenzen
  (`file_refs`, erzwungen), nicht jedem Prosa-Token; backtick-lose
  Pfad-Tokens in lebender Prosa lösen sich bei Berührung (fix-as-you-touch).
  Kalibrierung (gemessen): 2614 pfad-förmige Tokens in docs/**.md, die
  Prosa-Klasse konzentriert in Struktur-Dokumenten; kein Detektor gebaut
  (False-Positive-Maschine in deutscher Prosa — Rat).
- **Referenz-Hygiene des Baums** — der Kalibrierungs-Lauf fand 30 tote
  see-also-Referenzen in lebenden Dokumenten: Archivierungs-Umzüge
  (Handover → docs/handover/archiv/) hatten ihre Referenten nicht mitgeführt.
  Alle 30 sind repariert (Umzug nach `/archiv/`, Neu-Verdrahtung auf den
  gemessenen Datensatz oder Entfernung nie-existenter Ziele); der Scanner
  meldet 0 MISS / 0 ABS über 1101 Dateien.
- **Kompilat-Stufe** — geschlossen: docs/SOURCE_PORT.md §4 trägt `kompiliert`
  zwischen `verifiziert` und `disponiert` (Rat bestätigt; `void`/`geparkt`
  sind als Endzweige zu lesen, nicht als Kette). Ledger-Vokabular erweitert:
  `ausstehend | verifiziert | kompiliert | void | geparkt | disponiert`.

Die disjunkten Linien wurden 2026-09-09 gemessen und geschlossen (Sitzung
`handover-2026-09-09-disjunkte-linien-dispatch.md`; der schwere TE-Lauf ging
nach CI, Run 34400114106):

- **S3-ListBucketResult-Parser** — `d41cc87`: der NOAA-NODD-`ListBucketResult`-
  Parser gebaut (namespaced Tag-Scan, Contents/CommonPrefixes, Entity-Decode,
  paginierter Walk mit Continuation-Tokens), 4 stille Tests grün, live 1400
  Objekte. Die Bucket-Dispositionen je Dataset bleiben Operator-/Register-Frage.
- **maschinen-audits R4 + Kalibrationslauf** — `eafe72d`: R4-Single-Sheet-Kommata-
  Locale gebaut, Kalibrationslauf/Regression 14/14; R2 (§2-Zählung vs. Tabellen-n)
  bleibt `pending`. Alt-Fabrikationen (derive(Default)/unwrap_or_default/
  must-Diagnostik) auf dem Weg durch das Gate liquidiert.
- **10 bestand-Korpus-Herkunft** — `e197d71`: Befund; der registrierte Pfad zeigte
  auf die alte archive-root-Adresse, die Korpora liegen byte-identisch zweifach
  unter /home/johannes/backup/archive/; kein Datenverlust.
- **matrixmachine** — Suite 12/12 grün gegen HEAD (Urkunden-Zeile in
  archive-root/vanilla-dateidocs aktualisiert); die Gesamtsuite der Core-Crate
  (769 deklariert) gegen HEAD bleibt offen.

## vo-tap / uvor

- **Crate pushen** — `ivoa/uvor` existiert (HTTP 200 gemessen); der Seed
  `tools/vo-tap` (BSD-3-Clause, Copyright Johannes Tyroller) steht. Der Push
  hängt am Operator-Wort/Markus-Übergabe, nicht an Code.

## CDN-Manifestation

- **AllWISE-Ernte** — läuft (~13 Tage); `allwise_coverage.fp01` noch nicht
  verifiziert; CI-Partial-Check + Regrid-Optimierung (korrektheitskritisch).
- **PS1-fraktional** — läuft als Tiefen-Ernte (Autoresume); Partial-Landung
  reißt am 180-min-Timeout vor der ersten Band — eigenes Atom (Chunking/Timeout).
- **SPICE-`.bc`-Kernels** — `gll-ck-cdn.yml` steht (9 GLL-Kernels, sha256-Tor),
  naif-Release leer (404 gemessen); Dispatch = Operator-Wort.
- **Gaia XP** — Ernte-Schnitt (ganze Erde / helle Klasse / Jagd-Regionen);
  Ring↔Nest-Brücke source_id↔FP01-ipix ungemessen; `xp_pilot_p6144.bin` noch
  kein CDN-Asset.
- **ned-Crawl-Landung verifizieren** — 1/40 Slices (`ned_part_00000.json`),
  `ned.json` erst bei 40/40 (~40 h, stündlicher Cron); IPAC-Antwort ausstehend.

## Nadeln (offene Linien; Blätter tragen die Urteile)

- **Ⅰ** Jeans-Residuum bis Gaia DR4 (2.12.2026); Deduktion 42 (VLBI+Doppler-
  Sonde) pending.
- **Ⅱ** Flyby — Prüftermine JUICE 28./29.9.2026 + Europa Clipper 3.12.2026;
  AGU-2013-Abstract (Anderson) menschlich zu prüfen.
- **Ⅳ** LAIC — CSES, TEC retro pre-2024, Instrument A ungebaut, KDE-h.
- **Ⅴ** LSST-Live-Scan läuft (achromatischer Dip + IR-Exzess).
- **Ⅷ** Dunkler Fluss — Haufen-Kanäle benennen.
- **Ⅸ/Ⅹ** FRB / Kugelblitz — Kanal-Lage pending.
- **Ⅺ** Placebo — Paar-EEG, fam-Schwelle, Nullkontrolle, bedingte TE.
- **Ⅻ** Urknall — Reihen-Paarung Winkelserie×z-Reihe.
- **ⅩⅢ** Voller 48er-Zensus (18 Non-Detections); Photochemie-Re-Erklärung.

## Weberin (zweite Linien; die Bau-Linie lebt in die-weberin.md)

- Planeten/Monde zweite Abstammung (INPOP native `.dat` gegen `testpo`
  verifizieren, oder `_spice.tar.gz`-SPK-Weg) — `pending`.
- Raumsonden-Doppler ist keine unabhängige Positions-Linie; echte zweite Linie
  = VLBI-Winkel + Range oder zweite Ephemeriden-Abstammung — `pending`.
- Breite TNO-Kette: Survey-Untermengen (OSSOS/DES/Gaia) stehen; keine
  MPC-unabhängige Linie der vollen 8.082-Menge (not-published).
- Neptun-Planetenzentrum-Tabelle (Astrometrie-Kopplung) — Source-Port.

## Aufgelöste Aufträge (offene Pflichten; die Dateien → docs/auftrag/archiv/)

- **adoption** — Repo public + Drei-Mail-Block (Toth/Turyshev/Markwardt) als
  ein Zug mit Register-Zeile je Mail.
- **bande-split** — Split-Ergebnis + offene Registerzeilen (f*, 1-s-Zählung,
  Amplitude); Restbestand 238 Dateien/77 Tage; Transfer-Frage.
- **gic-p-wert** — GIC p-Wert nachlegen, dann Wing/Viljanen.
- **papier-kleinpass** — nach dem Merge, Zahlen je Blatt.
- **quiet-zone-uebertragung** — Rezept (nicht Pioneer-Ergebnis); New Horizons
  request-only als nächster Harvest-Weg.
- **gaia-dr4-iapetus** — Gaia DR4 (2.12.2026) als 4D-Feld.
- **flyby2-addendum** — Metrik vor dem 28.09.
- **flyby-doppler-rohdaten** — Roh-Doppler historischer Flybys (AGU-Beleg).
- **maschinen-audits** — R4-single-sheet-Locale gebaut + Kalibrationsscore-Lauf
  gefahren (`eafe72d`, Regression 14/14); R2 (§2-Zählung = Tabellen-n) bleibt
  `pending` (gemessener Grund im lauf-log; `number_audit.rs` trägt ihn in der
  R2-Ausgabe und im Test `z_section_counts_are_not_a_double_count_and_r2_stays_pending`).
  Provenienz-Notiz-Muster gebaut (`docs/specs/provenienz-notiz.md`, drei benannte
  Zeilen; `das-eine-instrument.md` §5 trägt die erste Instanz).
- **sicherung-risiko-heime** — einzige-Kopie-Risiko-Heime sichern (der
  Backup-Akt selbst bleibt Operator-Sache).
- **saubere-datenbank** — Step-5-Klasse (14 repo_tag + 3 dataset_host);
  Registry-first mit Rebuild-Quelle vor jeder CDN-Änderung. Gemessen 2026-09-09
  (Rat gehört): alle 14 `repo_tag`-Releases tragen `mirror_*`-Assets mit
  sha256-Digest; 12 Quell-Repos leben (Rebuild-Quelle = das Repo, Inhalt per
  Digest gepinnt), Bowserinator repo-tot (Release-Tag schon 404), GeoNuclearData
  0 Assets. Der destruktive Schnitt bleibt ein benannter Folgeschritt: je Asset
  Byte-Vergleich CDN-Digest ↔ Repo-Raw, einzeln, nie ein Blindwurf. Die 3
  `dataset_host`-Leases: physionet.org + spdf.gsfc.nasa.gov tragen
  sources.φ-url-lines (bidsleep/wind_orbit/wind_waves); sentinel1euwest.blob.
  core.windows.net ist Compiler-Lease (s1_sar_compiler + s1-sar-cdn.yml) ohne
  url-line.
- **extern-weberin-faden-luecken + -folge + -zweitlinien** — 9 Faden-Kategorien
  Routen messen; zweite Linien je Klasse (Teil-A-Subfragen + Teil-B-Mess-Punkte
  je ein Befund; EPM/VLBI-ΔDOR als zweite Linie je Klasse).
- **extern-stellar-aktivitaet-xuv-co** — seed committet (`f7c02de`: L_X + C/O
  für 7/30 Wirte; X-ray <72″ und M-Zwerg-C/O absent gemessen); Rest 23 Wirte
  `pending`.
- **bio-kanal-zeugen** — O2/O3, Rotkante, saisonal + Bio-Zeugen.
- **nadel-xiii-xuv-zensus / nadel-v-lsst-scan** — Zensus + LSST-Scan.
- **lisa-pathfinder-psd + -antrag** — Δg-Zeitreihe anfragen; Antwort = Messung.
- **ned-objdir-zugang** — Desktop-Befund ist eingefaltet
  (docs/handover/archiv/handover-2026-09-09-weberin-folge-cdn-ned-twomrs.md:
  TAP-60-s-Limit, async-Stau, kein Bulk-z, 2MRS-Alternative gebaut);
  IPAC-Anfrage versandt — Antwort ausstehend (Verdikt je Kanal).
- **docs-reference-verteilung** — Prämisse gemessen überholt (80 lebende
  Dateien, kein Pioneer-Zweig; docs/plans leer = erledigt). Verteilung
  vermessen: NAIF/SPICE + DSN/TDA-PR-Paare bleiben als Referenz-Ort unter dem
  README-Index; verarbeitete Messreihen → docs/concepts; publizierte Literatur
  → docs/paper; datierte Konten/Berichte handover-nah; Seeds → Survey-Heimat
  (Benennung `pending`); verbrauchte Alt-Belege → archive-root. Die Bewegung
  selbst bleibt ein eigenes Atom.
- **matrixmachine-register** — Urkunden-Zustandszeile aktualisiert: MatrixMachine-
  Suite 12/12 grün gegen HEAD (archive-root/vanilla-dateidocs); die Gesamtsuite
  der Core-Crate (769 deklariert) gegen HEAD bleibt offen — die Parallellinie
  trägt die Suite-Messung (`cargo test -p omegaflow --lib` lief 2026-09-09
  gegen den Arbeitsbaum); die saubere Messung gegen HEAD steht aus, bis der
  Baum fremdfrei ist.
- **Flut-2026-Linie (Rest)** — cog-quelle geschlossen (siehe oben);
  abfluss-trishuli: Seismik-/Luft-/Oberflächen-Befunde in-file (M5.2/M4.2,
  depth 0, Landslide — gemessen), der Abfluss-Pfeil bleibt `pending`; der
  DHM-Pegel 4913 (Bhotekoshi/Rasuwagadi) ist 2026-09-09 gezogen — der
  keyless-Livestore trägt nur 08-25…08-26 (162 Punkte, max 2,152 m), der Peak
  wurde nicht aufgezeichnet (Sensor-Ausfall erneut gemessen), das Pre-08-25-
  Archiv ist aus dem keyless-Store gerollt (Entsperrung = das archivierte
  externe CSV des 08-27-Zugs). seen-kollabgebiet: GL085494 −73 % gemessen
  (08-24); das CEMS/S1-Fenster ist NICHT abgelaufen (re-gemessen 2026-09-09):
  S1-Post-Szenen 08-28/08-31/09-05 verfügbar, EMSR927 trägt nur Grading-Produkte
  (AOI01–05), auf keinem AOI ein Delineation-Produkt — die Flutflächen-
  Delineation wurde nicht erzeugt. satellitenbilder-post: eine frei ladbare
  Nach-Aufnahme existiert (Landsat 9 26.08., wasserfrei am Kollabpunkt
  gemessen); CEMS-products.zip ohne Login ladbar, S1-SAR-VV-Asset mit keyless
  SAS-Token ladbar; die robuste Flut-/Narbenfläche braucht die (nicht erzeugte)
  Delineation oder eine eigene S1-Ableitung (Abruf offen).

## Source-Port & Katalog-Reste

- Kompilat-Stufe: geschlossen, siehe oben (§4 trägt `kompiliert`).
- Queue: 10 Untested-Korpora; 38 VizieR-Bulks; 77 Archeology-Gaps. Die 10
  `bestand`-Korpus-Dateien: Herkunft geklärt und geschlossen (`e197d71`) — der
  registrierte Pfad zeigte auf die alte archive-root-Adresse, die Korpora liegen
  byte-identisch zweifach unter /home/johannes/backup/archive/.
- Katalog-Lücken (RAVE DR6, APOGEE/GALAH, HyperLEDA, TGSS ADR, VLASS, AMS-02,
  GLADE+); FITS/Parquet/netCDF-4/OPeNDAP/GRIB-2-Struktur-Reader (FITS +
  netCDF-4 + CDF-1/2 gebaut; Parquet/GRIB-2/OPeNDAP offen).
- S3-Harvester-Namespace (NOAA-NODD-Buckets): `ListBucketResult`-Parser-Gap
  geschlossen (`d41cc87`); Inventar `noaa_nodd_inventory.φ` steht (101 Datasets);
  die Bucket-Dispositionen je Dataset bleiben Operator-/Register-Frage.
