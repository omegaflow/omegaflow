<!--
  title: Thematisches Handover — mechanische Reste
  class: handover
  date: 2026-09-09
  sha256: 0e096d3a91eadeca4193a5fdba690fb24d319e4a5142bbda08c2f7fbe60fcdb5
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
- **maschinen-audits** — Nummern-Audit, Provenienz-Notiz, Kalibrationsscore.
- **quiet-zone-uebertragung** — Rezept (nicht Pioneer-Ergebnis); New Horizons
  request-only als nächster Harvest-Weg.
- **gaia-dr4-iapetus** — Gaia DR4 (2.12.2026) als 4D-Feld.
- **iapetus-scan** — Literatur-Scan jetzt.
- **flyby2-addendum** — Metrik vor dem 28.09.
- **flyby-doppler-rohdaten** — Roh-Doppler historischer Flybys (AGU-Beleg).
- **glm-uebernahme** — GLM-Verifikation registrieren + Paper auf main.
- **abfluss-trishuli / cog-quelle / seen-kollabgebiet / satellitenbilder-post** —
  Flut-2026-Linie (Trishuli-Reihe, COG-Bandquelle, Seen-Baseline, Post-Bild).
- **docs-reference-verteilung** — docs/reference + docs/plans verteilen.
- **sicherung-risiko-heime** — einzige-Kopie-Risiko-Heime sichern.
- **matrixmachine-register** — Urkunden-Zustandszeile + Statuszeile.
- **verify-references-regelrunde** — CASE 5 (Archiv-Absolutpfade) + CASE 6
  (Fließtext-Drift).
- **saubere-datenbank** — Step-5-Klasse (14 repo_tag + 3 dataset_host).
- **gate-bereinigung-abschluss** — Gate-Bereinigung.
- **extern-weberin-faden-luecken + -folge + -zweitlinien** — 9 Faden-Kategorien
  Routen messen; zweite Linien je Klasse.
- **extern-stellar-aktivitaet-xuv-co** — XUV/C-O-Werte der 30 Wirte.
- **extern-neutrino-cr-routen** — IceCube-/CR-Routen verifizieren.
- **bio-kanal-zeugen** — O2/O3, Rotkante, saisonal + Bio-Zeugen.
- **techno-narrowband-scan / techno-atmosphaeren-gase / negativ-fuzzy-techno** —
  Technosignatur-Kanäle.
- **nadel-xiii-xuv-zensus / nadel-v-lsst-scan** — Zensus + LSST-Scan.
- **lisa-pathfinder-psd + -antrag** — Δg-Zeitreihe anfragen; Antwort = Messung.
- **ned-objdir-zugang** — NED objdir.
- **phantom-island-williston-bc** — Phantom-Island-Prüfung.

## Source-Port & Katalog-Reste

- Kompilat-Stufe in die Zustandsmaschine (entdeckt → kompiliert → disponiert).
- Queue: 10 Untested-Korpora; 38 VizieR-Bulks; 77 Archeology-Gaps.
- Katalog-Lücken (RAVE DR6, APOGEE/GALAH, HyperLEDA, TGSS ADR, VLASS, AMS-02,
  GLADE+); FITS/Parquet/netCDF-4/OPeNDAP/GRIB-2-Struktur-Reader.
- S3-Harvester-Namespace (NOAA-NODD-Buckets).
