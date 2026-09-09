<!--
  title: Thematisches Handover — mechanische Reste
  class: handover
  date: 2026-09-09
  sha256: d9b953f80fddeb8b7d6ac0940883350f8e404a25dce31247fff25c5d8e4778fd
  status: archived
  see-also: docs/handover/archiv/handover-2026-09-09-housekeeping-sauberer-schnitt.md docs/auftrag/archiv/
-->
# Thematisches Handover — mechanische Reste

Stehendes Register der mechanischen und forschenden Reste — der konsolidierte
Follow-up der aufgelösten Aufträge, der Nadeln, der Weberin-Linien und der
CDN-/Source-Port-Reste. Jeder Eintrag trägt einen gemessenen offenen Punkt;
Geschlossenes trägt Git.

## vo-tap / uvor

- **Crate pushen** — `ivoa/uvor` existiert (HTTP 200 gemessen); der Seed
  `tools/vo-tap` (BSD-3-Clause, Copyright Johannes Tyroller) steht. Der Push
  hängt am Operator-Wort/Markus-Übergabe, nicht an Code.

## CDN-Manifestation (Assets fehlen bis zum Dispatch)

- **neptune-c-spk-cdn.yml** — erledigt: Dispatch 34359296647, `ephemeris_neptune_c.bin`
  200 / 17.18 MB (breit 1802–2030). (handover-2026-09-09-uranus-push-cdn)
- **AllWISE-Ernte** — läuft (~13 Tage); `allwise_coverage.fp01` noch nicht
  verifiziert; CI-Partial-Check + Regrid-Optimierung (korrektheitskritisch).
- **2MASS-Footprint** — Compiler + Workflow stehen, Dispatch = Operator-Wort.
- **PS1-fraktional** — läuft als Tiefen-Ernte (Autoresume).
- **Asset-Manifestations-Pflicht** — `aia2014_fullyear.bin`, `planck_dust_av`,
  `eve2011_lines.bin`, `omni2_raw/`, `goes15*/`, `galileo_tdf_cache_*.TDF`,
  SPICE-`.bc`-Kernels — Kernel-/CDN-Schicksal je Quelle.
- **gebco 56-Byte-Stub** — `gebco_bathymetry.gbco` trägt 2 Records statt
  Bathymetrie; prüfen: echter 2-Punkt-Test oder fehlgeschlagener Upload.
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
