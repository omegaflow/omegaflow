# TODO

Nur offene Pflichten.

## Nadeln — Register

Eine Quelle, ein Blick: jede Nadel mit ihrem Status an dieser Stelle.
Details stehen in den Nadel-Abschnitten unten und in den benannten
Blättern/Surveys. Status: OFFEN / WARTET AUF DICH / GELAUFEN /
GESCHLOSSEN. Ein geschlossenes Blatt ist GESCHLOSSEN, wenn sein Verdikt
steht (Stille ist ein Verdikt, 0 honored). Was offen ist, bleibt offen
benannt — ein halbes Blatt ist ein Register-Eintrag, kein Ergebnis.

| Nadel | Rätsel | Status | Offener Punkt |
|---|---|---|---|
| Ⅰ | Dunkle Materie — Jeans-Residuum R(V) = ρ_dyn − ρ_vis je 50-pc-Voxel | WARTET AUF DICH | Gaia DR4 (2.12.2026) macht das Residuum zum 4D-Feld; Front Ⅱ = Gravitationssensor (dark_matter_probe, Netz 0/1008, GESCHLOSSEN) + Front C (NAVIO-Ruck-Sweep, 234/234 Form + Deduktion-40 sub-kHz + Deduktion-41 Drift, GESCHLOSSEN 2026-09-03, Blatt v6). Drift-Befund (82ffcc2, dddd138): Anomalie nicht aufgelöst — sunward ~10⁻⁴ der Anomalie, 150–340× unter Tagesmedian-Floor. Formtest (linear/∝t²/exp τ=126,5 a = T½(Pu-238)/ln 2) unaufgelöst, Grenze doppelt: (1) Floor 160–340 Hz (Anomalie ~1 Hz, 200–300× darunter); (2) Hypothesen-Degeneration über die P10-Spanne (thermaler Abfall nur 19,5 %, t²-trennt-von-linear um Zehntel-Hz bei ~1-Hz-Amplitude) — kein tieferer Floor trennte die Modelle bei ~1-Hz-Amplitude. Nächster Hebel: breitere Basis (kombinierte P10+P11-Ära / volle-Serien-Regression / Wochen-Monats-Bins), nicht Floor-Masken. Deduktion 42 (geometrischer Vektortest, Operator-Interferometer-Einwand) = pending, nicht auf Horizons-Residuen baubar (Rat-Verdikt): Vektor braucht zweiten Zeugen (Winkel/VLBI — Pioneer trägt kein Winkel-Record), 3D-Zweit-Differenz trägt Granulat-kohärenten Floor ~26-45× über a_P, Fix-Quellen-Ast als run_grid-Stille 0/1008 schon gemessen; gehört der nächsten Sonde mit VLBI+Doppler. Frage vollständig kartiert (4 Formen, alle gemessen): Paar-Korrelation (e5a2387) r = -0,002 vs Null 0,0802 — nichts gemeinsam über Boden. Geparkt am sub-Hz-Schritt: Rotationstest (sonnensymmetrische Kraft → nach Voll-Normalisierung kein Differenzdrift; raumfeste Kraft → Differenzdrift) als erste Formulierung von „wo sitzt die Kraft", sobald der Bahn-Schlauch dünn wird. Jeans bleibt bis DR4. Sub-Hz-Aggregation gemessen (nicht ungemacht): volle-Serien-Regression = Deduktion 41 bereits ausgeschöpft (Residuum-RMS 154-336 Hz, Anomalie ~200-300x darunter); Binning scheitert an duenner Basis (P10 nur 1126 Tagesmediane, braeuchte ~8000 Tage/sub-Hz-Punkt; Tracking-Luecken = Engpass). 60-s-Rohdaten zugreifbar aber heben Boden nicht (4d921bd): per-Sample-Regression P11 -2,7x Anomalie nur -0,2σ (per-Sample-RMS 6-7 kHz > Tagesmedian 154 Hz, √N-Gewinn verpufft); kein Kadenz-Gewinn. Weg: mehr Tracking-Abdeckung / besseres Instrument (breitere Basis / naechste Sonde). Quiet-Zone-Drift (Deduktion 44, `pioneer_navio_zone_drift`, 2026-09-04, Blatt v7): Zonen-Basis (P10 >50 AU 1036 Tage, P11 15–30 AU 606 Tage) — P10-Median |daily-med| 0,21 Hz erreicht sub-Hz, ABER der Drift ist nicht aufgelöst: Maskierung (Deduktion-10 + Schwanz, 39/1036 bzw. 15/606 Tage verworfen) senkt RMS 257→57,7 Hz (P10) / 306→105 Hz (P11); Drift P10 −1,95× Anomalie (sunward) bei 0,45σ, P11 +6,98× (outward) bei 0,54σ — 6×/5× unter der Block-Bootstrap-Nullschwelle, Vorzeichen widersprechen sich; Formtest degeneriert (exp ≈ linear über 11 a vs τ=126,5 a, ∝t²-Vorteil 0,18 % = Rauschen). Verdikt: keine Präferenz (Grenze) — der sub-Hz-Gewinn liegt im Median, nicht im Drift (die Drift-Regression kämpft gegen die RMS-Streuung, nicht gegen den Median). Front C, Abschluss (Deduktion 40–45, 2026-09-04): die Pioneer-Anomalie-Frage ist auf diesen Daten auf vier Böden beantwortet — kHz (Ereignisse, Volkszählung 234/234), sub-kHz (Drift, Grenze bei widersprüchlichen Vorzeichen), Quiet-Zone (Median 0,21 Hz), Ereignis-Ebene (sd ~1 Hz, Transit-Form strukturell abwesend). Die Anomalie ist nicht widerlegt — sie ist auf jedem erreichbaren Boden unsichtbar, und jeder Boden ist benannt, mit Weg darunter markiert. Die Front endet nicht in Niederlage, sondern in einer vierstöckigen, fraktal verifizierten Stille: der am tiefsten dokumentierte Nicht-Fund der Raumsondengeschichte. |
| Ⅱ | Flyby-Anomalie — Perigäums-Residuum gegen die Sonnenwind-Phase | WARTET AUF DICH | Prüftermine JUICE (28./29.9.2026) + Europa Clipper (3.12.2026); Weg 1 (kalt) + Weg 2 (preregistriert). Roh-Doppler-Beschaffung: `auftrag-flyby-doppler-rohdaten.md` (Juno-Erdflyby 2013 `open`/request-only, Gegencheck jnogrv_0001 + AAS 14-435 verifiziert). Offen (menschlicher Prüfer): AGU-Fall-Meeting-2013-Abstract verifizieren — Anderson et al. „Juno Earth Flyby as a Sensitive Detector of Anomalous Orbital-Energy Changes" (Control-ID 1799584, Pfad unbestätigt): ~7 mm/s erwartet, SE ~0,01 mm/s, kein Signal — belegen gegen die AGU-Quelle, bevor es als Zitat trägt (0 honored). |
| Ⅲ | Koronaheizung — kausale DAG der solaren Kanäle | GELAUFEN | Zahl gemessen 2026; Richtung TIAW vs Nanoflares als Ableitung offen; Zellen pending bis Mehrfachvergleich + Lag-Sweep + KDE-h |
| Ⅳ | LAIC — Lithosphäre → Ionosphäre? | GELAUFEN | Blatt `laic-arrow-direction.md` steht (Stille beide Richtungen); offen: CSES, TEC retro pre-2024, Instrument A ungebaut, KDE-h |
| Ⅴ | Achromatische Opazitäts-Anomalie (GALAXIE-Scan, Techno-Strukturen) | GESCHLOSSEN (Epoche) → LSST-LIVE WIEDER GEÖFFNET | ZTF/WISE-Sweep-Epoche geschlossen (2026-08-28, Kandidat ausgeschlossen, Limit 0 honored). ALS GALAXIE-SCAN auf dem LSST-Live-Stream (läuft seit 29.06.2026, ~1,4 Mio Quellen/Woche, 10 a) wieder zu öffnen: achromatischer Dip + IR-Exzess über der wachsenden Fläche; die Nadel Ⅴ ist der strukturelle Techno-Scan auf Galaxie-Maßstab — getrennt von der Atmosphären-Biosignatur (Nadel ⅩⅢ), nicht konfliert. |
| Ⅵ | KBO-Gravitations-Residuum (Planet Neun) | GESCHLOSSEN | Blatt `planet-nine-kbo-residue.md`: kein fam-Pfeil (Stille); Wege 1/2 geschlossen |
| Ⅶ | Signal-Kegel-Audit — Lichtkegel-Verletzung | GESCHLOSSEN | Blatt `signal-cone-audit-sheet.md`: kein fam-Paar trägt einen Pfeil; nächste Runde bei neuen Kanälen |
| Ⅷ | Dunkler Fluss — Pfeil am Blockrand | WARTET AUF DICH | CMB-Ernte (Atom 1) + Struktur-Ernte CF4 (Atom 2) gelaufen; Haufen-Kanäle zu benennen, pending; kosmologie-cmb |
| Ⅸ | FRB — Streu-Spur | WARTET AUF DICH | Kanal-Lage pending |
| Ⅹ | Kugelblitz | WARTET AUF DICH | Kanal-Lage pending |
| Ⅺ | Placebo — der Pfeil des Glaubens | WARTET AUF DICH | Paar-EEG (electric), fam-Schwelle, Nullkontrolle; bedingte TE pending; Front des Erlebnis-Blocks |
| Ⅻ | Urknall — kausale DAG der Schöpfung | GELAUFEN | Blatt `big-bang-echo-sheet-12.md`: Stille CMB↔Dichte, z-Reihe benannt, t=0 verweigert (0 honored); Reihen-Paarung Winkelserie×z-Reihe offene Form-Entscheidung |
| ⅩⅢ | Atmosphären-Biosignatur / Disequilibrium (JWST-begrenzt, NICHT Galaxie-Scan) | GELAUFEN (30 mit Detektion) — VOLLER 48er-ZENSUS OFFEN | Eigene Nadel, getrennt von Ⅴ (zwei Skalen, keine Konflation). Disequilibrium-Survey der 48 JWST-Transmissions-Ziele: 30 mit publizierter Detektion disequilibrium-geprüft (16 Hits, P=0.9424, Paper gate-konform); die 18 OHNE publizierte Detektion (GJ 1132, TRAPPIST-1, LHS 1140 … = flach/neblig/Obergrenze) gehören in den Katalog-Zensus als Non-Detection (0 honored), nicht fallen gelassen. Offen: XUV/C-O-Zeugen der Sterne (externer Rechercheauftrag), die Photochemie-Re-Erklärung der SO2/CO2-Hits |

Pflicht vor jedem Blatt (blätter-übergreifend, siehe Nadel Ⅲ-Abschnitt):
Mehrfachvergleichskorrektur über alle getesteten Paare, Lag-Sweep
(Lag 0 ist kein Sweep), KDE-Bandbreiten-Sensitivität (h, Faktor 2),
Kontrollrichtung des gemeinsamen Treibers, bedingte Multi-Force-TE
(pending-Instrument), Abzugsliste vorab gebunden (welcher Treiber,
welches Modell, welche Version — vor dem Rest; der Rest ist Boden, kein
Verdikt; Boden → Verdikt nur über den Doppel-Test fam + treiberfrei;
nicht messbar = pending, nie 0). Kein Blatt ohne diese fünf — eine
halbe Messung ist kein Befund.

## Nadel-V-LSST-Erweiterung — TDB+Zeuge (pending: Lasair-Token, Positivkegel; Verdrahtung 2026-09-05)

- **Rømer-Lichtzeit in den Fold-Pfad verdrahtet** (lsst_anomaly_probe):
  MJD/TAI → TDB (Uhr-Skala) → + n̂·(Station−Sonne)/c je Reihe. Real gemessen
  auf dem DDF-Kegel (148.84, 2.55, 600″): 9625/9625 Reihen mit der
  Cerro-Pachón-Station korrigiert (Ephemeride trägt die Erd-Orientierung);
  Budget gemessen: Rømer-Amplitude 499,0 s, Erddrift 60,1 s/Woche (= 3,0
  Zyklen einer 20-s-Periode, 0,83 % einer 2-h-Periode), Diurnal 21,28 ms,
  Shapiro-Koeffizient 2GM/c³ = 9,85 µs (nur limb-nahe ~59 µs; Nachthimmels-
  Elongationen sub-µs). Konsolidiert: die Zwei-Proben-FAP-Konsolidierung ist
  unten geschlossen (der Kriterien-Posten ist kein offenes Tor mehr).
- **FAP-Gate-Fix GESCHLOSSEN (gemessen, 2026-09-05):** die Wurzel der
  Skalenabhängigkeit in `lomb_scargle_fap` ist benannt und behoben: die
  Periodogramm-Leistung trug x² im Nenner (`den = Σ x² sin²`), dadurch skaliert
  die Teststatistik ~ n/σ² (Amplituden-skalen-gebunden) — leise fraktionelle
  Photometrie (σ ~ 0.02) las FAP 0.00e0 auf JEDER Zeile (auch auf echten
  6–10-σ-achromatischen Dips), das aperiodische Tor war zu. Fix: Standard-
  Normalisierung (varianz-normiert, beide Quadraturterme, τ über
  tan(2ωτ)=Σsin2ωt/Σcos2ωt) — amplitudeninvariant. Gemessen an drei
  Dekaden weißem Rauschen (sd 0.02/3/30): FAP 9.52e-1 auf allen drei Skalen
  (Spread 1.6e-8 = f32-Rundung). Negativkontrolle auf dem realen DDF-Kegel
  (ra 148.8746 dec 2.5208) jetzt VOLL durchs Tor: 8σ-Injektion misst
  i −6.1σ/z −5.9σ ratio 0.93 achromatisch, FAP 3.74e-2 ≥ 0.01 → Kandidat
  gefunden; 6σ → −4.2/−4.2 FAP 1.61e-2, 10σ → −7.8/−7.4 FAP 7.48e-2.
  Periodisch-Natur-Positivkontrolle: echte 2-h-Sinusoid mit 8σ-achromatischem
  Dip bleibt vom FAP-Tor ausgeschlossen (FAP 3.79e-4 < 0.01, 0 Kandidaten) —
  das Tor trennt jetzt. Auf dem unverseuchten Kegel: 10/26 Objekte lesen
  periodisch (FAP 2.45e-9…7.08e-3) und bleiben ausgeschlossen, 9 aperiodische
  achromatische Dip-Kandidaten erscheinen als pre-exclusion und werden alle
  als katalogisierte natürliche Dimmer (Fink-Klasse 11/13/22) aussortiert —
  0 unklassifiziert post-exclusion, aber jetzt durch offene Tore gemessen,
  keine geschlossene Tor-Stille mehr. Ein Pfad, alle drei Kopien (beide
  lsst-Proben + ztf_anomaly_probe, byte-identischer Defekt). Folgewirkung:
  lsst_color_coupling liest `is_periodic = FAP < Tor` — unter dem alten
  Skalen-Bug war JEDE Reihe "periodisch" (FAP 0), die gemessenen
  Farbe-Helligkeits-Kopplungs-Zahlen (12 Kandidaten, fam 4.49e-1) stammen aus
  dieser Alles-periodisch-Übermenge; die periodische Heimschicht selektiert
  jetzt echt (FAP < 0.01), ein Neu-Lauf auf dem Kegel ist pending.
- **Lasair-LSST-Quelle verdrahtet (bereit für den Token):** `--lasair-ra/
  --lasair-dec/--lasair-radius [--lasair-max]` in lsst_anomaly_probe. Endpunkte
  und Spalten gemessen (lasair-lsst.readthedocs.io REST-API + lsst-uk/
  lasair-examples-Notebooks): Host api.lasair.lsst.ac.uk/api, `/api/cone/`
  (ra/dec/radius/requestType) → [{object, separation}], `/api/object/`
  (objectId) → diaSourcesList mit band/midpointMjdTai/psfFlux (MJD/TAI, gleiche
  TDB-Faltebene wie Fink). Token-Schlüssel LASAIR_LSST_TOKEN in .secrets.local
  (Env-Override), mast_token-Idiom. Parser am dokumentierten Schema getestet
  (2 Unit-Tests). Token NOCH NICHT gelegt (gemessen: .secrets.local trägt nur
  LASAIR_TOKEN, nicht LASAIR_LSST_TOKEN) → anonymer Lauf misst beide Endpunkte
  401 (lebend, token-gated) und bleibt benannt pending; der Code läuft, sobald
  der Token liegt. Pending: das authentifizierte Live-Schema gegen den ersten
  echten Sample-Abruf verifizieren.
- **Positivkontrolle:** 0 VSX-bestätigte Chromatik-Periodika im DDF-Kegel
  (Feld ist AGN-dominiert) — gemessene Abwesenheit, kein Durchfall; ein
  variablenreicher Kegel (echte RR-Lyrae/EB mit LSST-Mehrband-Zeitreihen)
  ist als nächster Lauf pending.
- **Externer Zeuge VSX verdrahtet** (lsst_color_coupling_probe): VizieR
  `B/vsx`-Kegelkreuzmatch, erreichbar (HTTP 200), Parser am realen
  Schema getestet (OID/Name/Type/Period, 3 Datensätze gemessen). Die
  broker-eigene Klassifikator-Spalte (`f:main_label_classifier`) bleibt
  Report-Label, nie Zeuge. Im Kegel 0 Treffer ≤ 3″ (AGN-Feld) — der Zeuge
  wartet auf den variablenreichen Kegel.
- **Zyklus-konsistenter Surrogat** (`cycle_phase_shift_surrogate`, te.rs,
  amplitudenerhaltende Phasen-Rotation je Zyklus, Tests grün):
  lsst_color_coupling nutzt für die ungefaltete Zeitreihe bereits die
  amplitudenerhaltende FFT-Phasen-Null (kein Shuffle) — die
  Zyklus-Rotation ist die Null der gefalteten/Per-Zyklus-Reihe; eine
  echte Faltung existiert im Layer nicht, die Nutzung bleibt bis dahin
  benannt pending.
- **B-Unlock geparkt:** Fink-Account/Kafka oder Rubin-Strom als benannter
  Kostenpunkt (Konto, Stream-Rechte, Betrieb); kein Datum.

## Auftrags-Programm — offene Ordnungen (docs/auftrag/)

Die versionierten Forschungs-/Recherche-Ordnungen (`class: auftrag`) sind
die Gate-Ausgabe für neue Linien. Das Hauptregister navigiert hierher; jede
Zeile = Datei + Kurzpflicht. Alle `status: pending` (Stand 2026-09-03).

### Welt-Zugang — Adoption & Papiere

- `auftrag-adoption.md` — Repo public, Welt-Fassung, der Drei-Mail-
  Block (Toth/Turyshev/Markwardt) als Ein-Zug; zweite Welle GIC/Korona;
  Mail-Ledger-Zeile je Mail.
- `auftrag-bande-split.md` — 20-s-Bande: two-/three-way-Split + die drei
  offenen Registerzeilen (f*, 1-s-Zählung, Amplitude) vor Mail 1.
  Klasse-2-Galileo-Cross-Mission gemessen (2026-09-05,
  `befund-galileo-banden-negativ.md`): NEGATIV — Pioneer-Linien
  45,75/51,55/47,35 mHz missions-spezifisch (kein Modus/keine Ära ±0,5 mHz
  auf Galileo, Stationen 14/43/63). Nachmessung des exakten 20-s-Kamms
  (2026-09-05, `befund-galileo-banden-kamm-ton.md`): 50/100/150/200 mHz =
  **Degeneranz des 60-s-Abtastrasters** (f·60 s ganzzahlig → singuläre
  Normalengleichung; normiert ≤ 8 % Varianz, Weißrausch-Kontrolle auf demselben
  Grid reproduziert), keine Linie; Station-42-Ton 52,39 mHz = isolierte
  Einzellinie (98,9–100 % Varianz, keine Harmonischen), Identität offen;
  GWE-ODR-Adresse `GO-X-RSS-1-ODR-V1.0` verifiziert erreichbar (HTTP 200).
  GWE-ODR-Cross-Check gemessen (2026-09-05, `befund-galileo-gwe-odr-banden-check`,
  gezielte Ein-Pass-Stichprobe 3/241): die 45,75/51,55/47,35-mHz-Linien erscheinen
  in Trägerfrequenz- und Amplitudenreihe nicht (injektions-kalibriert, nicht blind).
  Offen (pending): Transfer-Frage (ob resid-Linien überhaupt auf ODR prägen) +
  Restbestand 238 Dateien/77 Tage.
- `auftrag-gic-p-wert.md` — GIC: p-Wert nachlegen, dann Mail an
  Wing/Viljanen.
- `auftrag-korona-aia-fam.md` — Korona: AIA-fam-Zahl, dann Woods.
- `auftrag-papier-kleinpass.md` — Papier-Kleinpass nach dem Merge (Zahlen je
  Blatt).
- `auftrag-maschinen-audits.md` — Nummern-Audit, Provenienz-Notiz,
  Kalibrationsscore vor jedem Rewrite.

### Forschungs-Nadeln & Missionen

- `auftrag-subhz-drift-quiet-zone.md` — GESCHLOSSEN (2026-09-04): Sub-Hz-Drift
  auf der Quiet-Zone-Basis (P10 >50 AU, 1991–2002). Fünf Bausteine gebaut
  (Verdikt-Bindung, Maskierung, Surrogat-Null, Regression, Registrierung).
  Befund: Median 0,21 Hz sub-Hz, aber Drift nicht aufgelöst (keine Präferenz,
  Grenze) — der Drift kämpft gegen die RMS-Streuung, nicht den Median.
- Deduktion 45 (2026-09-04, GELAUFEN — `pioneer_navio_zone_events`) — Ereignis-Scan
  auf den stillen Zonen-Tagen. Gemessen: die stillen Tage (|med| ≤ 5 Hz, P10 69 %/
  P11 47 % der maskierten Tage) erreichen sd 1,0 Hz (P10) / 1,6 Hz (P11) — der
  ~1-Hz-Boden ist echt. Aber: keine Transit-Form (mehr-tägig weg, zurück) auf
  beiden Sonden — P10 21 isolierte Ein-Tages-Erhöhungen (>3σ=3,75 Hz, alle
  einzeln, max-Lauf 1 = Null-p95, super-Gauß-Schwanz der stillen Verteilung),
  P11 0. Verdikt: saubere Stille auf dem ~1-Hz-Boden — kein 1–5-Hz-Ereignis
  aufgelöst; Q2 (Vorzeichen je Ära) nicht auflösbar (Deduktion-44-σ), Q3 (Koinzidenz)
  leer (P11 hat 0 Ereignisse). Der erste ~1-Hz-empfindliche Ereignis-Scan dieser
  Daten endet in Stille (0 honored).
- `auftrag-quiet-zone-uebertragung.md` — pending (2026-09-04): Quiet-Zone als
  Rezept, nicht Pioneer-Ergebnis — Rauschen verorten, Zone isolieren, Boden
  messen. Türen: Voyager 1/2 (160/130 AU), New Horizons (60 AU), Mariner/
  Galileo/Cassini (retroaktiv, S-Band). Tür 1 (Co-Quiet-Kreuztest P10+V1)
  GESCHLOSSEN (2026-09-04): kein offener Cruise-Doppler (SPDF/PDS/JPL
  verifiziert negativ, Befund `docs/befund/befund-voyager-roh-doppler-zugang.md`),
  kausale Grenze = dreiachsen-stabilisiertes Selbst-Rauschen (~10⁻⁶ cm/s²,
  ~10× über a_P).   Lehre als Vorfilter eingebaut: Rauschen muss
  Distanz-Geometrie haben (spinstabilisiert/medium-getrieben), sonst fällt
  die Tür ohne Harvest. Vorfilter ausgeführt (2026-09-04): New Horizons
  `besteht` (Spin-Cruise, kein Reaktionsrad, >50 AU bewohnt) = nächster
  Harvest-Kandidat, aber `request-only` (JPL/DSN-ODF-Anfrage); Galileo-GWE
  `open` = Reserve; Cassini/Mariner `fallen`. NH-Daten-Anfrage registriert
  (2026-09-04, an PDS Radio Science Subnode/Geosciences Node, Iess-DOI
  als Referenz, Pilot-Epochen-Frage; Versand menschlicher Akt + Antwort
  `pending`). Literatur-Scan:
  Verdikt (a) bestätigt — kein Zonen-Floor im Turyshev/Toth-Korpus 2002–2012;
  Korrektur: „laute Zone als Instrument" ist belegt (Woo & Armstrong 1979,
  mit Pioneer), kein eigener Fund; Anderson 2002 gelesen (Arcs = Zeit-Intervalle,
  bestätigt), vier Volltexte pending (Armstrong 1998, Bertotti 2003, Tortora
  2004 — born-digital/Lizenz; Woo 1979 — Scan-Kandidat, kein Vision-LLM nötig).
  Galileo-GWE-Ernte geprüft (2026-09-05, `befund-galileo-gwe-bestand.md`):
  `gll.rss` existiert nicht — real = PDS3 `GO-…-RSS-…-V1.0` (TRK-2-25/2-18,
  GWE open-loop ODR); Galileo bleibt Reserve mit eigener relativ-ruhiger
  ≤5-AU-Achse, kein Quiet-Zone-Nachbau.
  Lehre (quer, verbindlich): Register-Behauptungen über externe Archive sind
  Adressen, keine Orte — `lookup vor harvest`; jede `open`-Aussage ist
  `pending` bis zum Registry-Lookup (NH-Anfrage zitiert verifizierte IDs).
  GWE-ODR-Banden-Test (Nebenfund): `GO-X-RSS-1-ODR-V1.0` lief auf DSS 14/43/63
  — denselben Stationen wie die 20-s-Bande, open-loop 1994/95 (überlappend
  mit Pioneer 1987–93) → potenziell stärkste externe Banden-Validierung
  (Klasse 2, `auftrag-bande-split.md`).
  Galileo-Bau (2026-09-05): `galileo_atdf_compiler` (TRK-2-25 → GASR-Residuum-
  Serie, Stationen 14/43/63, Modes 1/2/3 getrennt; `reduce_skyfreq` in die
  Lib, eine Quelle). TRK-2-18-Layout ≠ TRK-2-34-Layout — `parse_odf` gilt nur
  für 2-34; Galileo-ODF braucht Layout-Verifikation vor jedem Wiederaufbau
  (Compiler entfernt, zurückgestellt). Nächster Schritt: Horizons-Ephemeride
  Galileo 1990–97 + Measure-Probe (Rauschen vs Distanz/SEP/Mode/Station);
  vorab gebunden: n_je_Mode_je_Distanzband zuerst, dann Kurve; Lock-Übergänge
  als eigene Ausreißerklasse mit mitgeführtem n_lock je Segment (nicht nur
  Schwelle); Mode 2/3 = Plasma-Test, Mode 1 = Oszillator-Fund, Station =
  Banden-Brücke.
  Job-Prüfung (2026-09-05): GO-SUN-INDEX.TAB ist stale (126 Zeilen vs 83
  Dateien auf der Platte), GO-JG ebenso (28 vs 20) — Enumeration läuft jetzt
  über das Verzeichnis-Listing (Bodengrund), nicht den INDEX; GO-SS ⊆ GO-SUN
  (globaler Dedup). Realer Bestand ~138 TDF-Dateien, nicht 192. Der
  Befund-`192`-Wert war der stale INDEX, keine Platten-Wahrheit.
  Rausch-Kurve revidiert (2026-09-05, `befund-galileo-rausch-kurve.md` v2,
  Achsen-Revision): der gemessene Winkel war α (Winkel am Sonnenort), nicht ε
  (solare Elongation, Winkel an der Erde) — die Achse war falsch benannt; α/ε
  sind für die äußere Sonde komplementär. Der kohärente Kanal ist laut an
  Opposition, leise an Konjunktion — das Gegenteil von Plasma-Szintillation;
  die Plasma-Deutung ist `unverifiziert`, der Mode-2-28×/Mode-3-12×-Fall ein
  Distanz-/Ära-Confound (Stoßparameter b ≈ 1 AU·sin ε, nicht r·sin(SEP)).
  Mode 1 flach auf beiden Achsen — der Stärke-Split
  (`befund-galileo-mode1-fingerabdruck.md`) misst einen empfangsstärke-
  abhängigen Schwachsignal-PLL-Term, kein reines Oszillator-Rauschen.
  Feiner gemessen (2026-09-05, `befund-galileo-mode1-snr-kurve.md`, done): das
  Stärke-Feld ist zweistufig + Epochen/Monats-geschichtet; das starke Plateau
  ist innerhalb jeder Epoche flach (keine ∝-Kurve messbar); das „10–20× lauter"
  gilt nur für die lauten Boden-Populationen (1996/97 + 1995-12 Station 43/63),
  nicht für den ruhigen 1995-11-Boden. Der AGC-Boden ist Epochen-/Stations-
  gebunden, keine reine SNR-Lesart.
  Distanz-Achse n-leer (nur 5–6 AU). ε-Redraw ausgeführt (2026-09-05,
  `befund-galileo-rausch-kurve-epsilon.md`, done): der kohärente Fall ist auf ε
  invers zur Plasma-Erwartung (leise Konjunktion / laut Opposition, mittlere
  ε-Bänder 30–150° n-leer) — die Plasma-Deutung dort `getötet`, der
  Distanz-/Ära-Confound bleibt. Mode-2-Station-Tag-Split gemessen
  (`befund-galileo-mode2-station-split`, done): 1,5-Hz-Wert fragil
  (All-Lock-Tag-Zählung + Stations-Pooling, Konjunktion-Station-Tag 0,28–0,58 Hz);
  Pass-Segmentierung gemessen (`befund-galileo-pass-segmentierung`, done):
  Mode-1-8,2-Hz = Pooling-Artefakt bestätigt, Mode-2-Konjunktion = echte ruhige
  Pässe (mit kurzen lauten an 43/63). Richtungs-TE gemessen
  (`befund-galileo-te-staerke-floor`, done): auf Tages-Achse kein gerichteter
  Stärke→Rauschen-Pfeil — TE überlebt die Monats-/Stations-Konditionierung nicht
  (62/64 unter cThr), der −2560-Boden ist Epochen-/Stations-kollokiert, kein
  SNR-getriebener PLL-Term. Spec-TE gemessen (`befund-galileo-te-spec`, done):
  Spec-Träger ref_hz/mode/Kadenz → resid-Noise **entkoppelt/era-koinzident**
  (kein gerichteter Pfad über die Epoche; Kadenz-Achse im Feld degeneriert,
  1-s realisiert statt 60-s). Letzte pendings gemessen (2026-09-05): Mode-2-
  Stärke-Split (`befund-galileo-mode2-staerke-split`: Boden lauter als starkes
  Q4 im ruhigen Fenster), 1,5→0,65-Tagesmengen-Reconciliation
  (`befund-galileo-tagesmengen-reconciliation`: E1 hält, 2 All-Lock-Tage),
  In-Pass-Stärke-Rampe (`befund-galileo-inpass-staerke-rampe`: an 43/63 genuine
  Boden↔Rauschen-Kovarianz bei Pass-Identität — verfeinert das T1b-Tages-Null für
  die statische Assoziation; Richtung bleibt offen —, an 14 Epochen-Kollokation),
  Same-Day-Spec-Niveau (`befund-galileo-sameday-spec-assoziation`: null).
  Alle letzten pendings gemessen (2026-09-05): In-Pass-Richtung
  (`befund-galileo-inpass-richtung`: simultane Kovarianz, kein anhaltender
  Floor→Rauschen-Pfeil), detrendete Pass-Metrik (`befund-galileo-pass-detrend-
  metrik`: Boden ist Streuung, kein Drift), Same-Day-Floor-vs-Strong-Tag-Paarung
  (`befund-galileo-sameday-floor-strong-paarung`: echt bei fixem Tag, +2,62 Hz
  19/3), α–Zeit–Sonnenzyklus (`befund-galileo-alpha-zeit-sonnenzyklus`:
  Fall-Magnitude Ära-konfundiert, kollabiert auf ~2×), Mode-3
  (`befund-galileo-mode3-und-s1-replikation`: Daten-Dünn-Grenze, 17 isolierte
  Tage), S1-Isolat (repliziert nicht). Late-Conjunction-Split gemessen
  (`befund-galileo-late-conjunction-split`: Station 63 trägt den 1997er-Boden,
  ruhiger Kern = starker Zustand, Fenster zustands-gemischt). Offen (pending):
  Geometrie-innerhalb-Pass (Elevation, G1 — gemessen daten-begrenzt wenn nötig),
  late-conjunction Station-Split (G4), Mode-2/3-Ära-Halte-Kontrast jenseits 1996
  (G4, daten-dünn).
  Nebenfund:
  Stationen 12/15/24/34/42/45/61 (34m) → GWE-Banden-Test-Erwartungsliste.
  `galileo_daily`-Ephemeride manifestiert.
  Register-Pflicht (recipe-level): `pioneer_navio_noise_geo.rs` trug dieselbe
  „SEP = Winkel am Sonnenort"-Fehlbezeichnung (korrigiert: α benannt, ε
  ergänzt). Jedes Blatt der Pioneer-Quiet-Zone-Rezept ist auf der ε-Achse neu
  zu prüfen, nicht nur Galileo. Front-C-ε-Recheck ausgeführt (2026-09-05,
  `befund-front-c-noise-vs-epsilon`, done): Distanz-Befund hält (P10 7412 →
  651–805 Hz, P11 5820 → 993–1181 Hz); die „Winkel nicht der Treiber"-Aussage ist
  auf ε präzisiert — P11 trägt eine Konjunktions-Spitze (ε 0–10°, 8013 Hz, n=43)
  über jedem Distanz-Median, P10 ist auf ε flach mit anti-Plasma-Form, global kein
  ε-Treiber. Distanz-basierte Ergebnisse (Quiet-Zone-`--zone`, sub-Hz-Boden —
  Deduktion 44 geschlossen, leeres Netz) unberührt (Distanz ≠ Winkel). Offen
  (pending): die ε×Distanz×Ära-2D-Entzerrung der P11-Konjunktions-Spitze.
- `auftrag-quiet-zone-vorfilter.md` — GESCHLOSSEN (2026-09-04): Vorfilter Tür 2
  (New Horizons ~60 AU) + Tür 4 (Mariner/Galileo/Cassini) ausgeführt.
  Verdikte gemessen: New Horizons `besteht` (Cruise spinstabilisiert, keine
  Reaktionsräder, ~100 d/a ohne Lage-Manöver; >50 AU bewohnt) — aber kein
  offener Harvest (REX = Okkultation/TNF, SPDF 404, Nav-Doppler
  `request-only`); Galileo `bestehen` als Reserve (Dual-Spin, S-Band
  medium-getrieben, aber ≤5 AU — keine Quiet-Zone; Bestand gemessen
  2026-09-05, `befund-galileo-gwe-bestand.md`: kein PDS4 `gll.rss`, real =
  PDS3 `GO-…-RSS-…-V1.0` — TRK-2-25 TDF 6,3 GB + TRK-2-18 ODF 0,16 GB,
  TRK-2-34 absent, GWE open-loop ODR 4,5 GB); Cassini `fallen` (3-Achsen);
  Mariner `fallen` (kein
  Distanz-Muster). Entscheidung: NH-Doppler-Anfrage (request-only) =
  nächster Harvest-Weg; Galileo-GWE = offene Reserve (eigene
  relativ-ruhige ≤5-AU-Achse, kein Quiet-Zone-Nachbau). pending-Reste:
  NH-Selbst-Rauschen-Größe, NH-JPL/DSN-Anfrage (nicht ausgeführt),
  NH-REX-Tiefenprüfung, Galileo-empirische-Rausch-Kurve (eigene Ernte-Session).
- `auftrag-voyager-roh-doppler-zugang.md` — GESCHLOSSEN (2026-09-04): eigenhändige
  Gegenprüfung des PDS/JPL-Zugangs für den V1-Roh-Doppler (Co-Quiet-Tür 1).
  Befund (`docs/befund/befund-voyager-roh-doppler-zugang.md`, `status: done`):
  keine offene Quelle deckt das Fenster ~1998–2002 in Doppler ab — PDS-RMS
  trägt fünf Bündel, alle Encounter (1979/1980/1981/1986) open-loop; NAIF =
  rekonstruierte Bahn; JPL/DSN ODF/TDF `request-only`. Neue Erkenntnis:
  Voyager ist dreiachsenstabilisiert, sein Lageregelungsrauschen (~10⁻⁶ cm/s²)
  liegt ~10× über der gesuchten Effektgröße — nie ein Bergungsanreiz wie bei
  Pioneer. Nachtrag (2026-09-04): IPNPR (endet Uranus-Ära 1986) + VLBI-
  Kampagnen (1988 Medicina; PRIDE ab 2013 andere Ziele) geprüft — keine
  Fenster-Quelle; Turyshev/Nieto/Anderson 2005 (arXiv:physics/0502123)
  als Fremdbeleg: das JPL-Team hat Voyager-Navigationsdaten intern
  geprüft und für die Anomalie-Frage explizit verworfen — Zugang
  existiert(e) JPL-intern, nie öffentlich archiviert. Drei Restlücken
  `pending`: JPL/DSN-Anfrage (benannt, nicht ausgefüllt),
  PDS-Wide-Search-Formular, ADS-Volltext.
- `auftrag-gaia-dr4-iapetus.md` — Gaia DR4 (2.12.2026): Jeans-Residuum als
  4D-Feld (Nadel Ⅰ).
- `auftrag-iapetus-scan.md` — Iapetus/Halo: Literatur-Scan jetzt.
- `auftrag-flyby2-addendum.md` — flyby-2-Metrik-Addendum vor dem 28.09.
  (Nadel Ⅱ).
- `auftrag-flyby-doppler-rohdaten.md` — Roh-Doppler der historischen Flybys
  (Nadel Ⅱ, Weg 1): Juno-Erdflyby 2013 `open`/request-only; Gegencheck
  jnogrv_0001 + AAS 14-435 verifiziert; AGU-2013-Abstract (Anderson et al.)
  als offener, menschlich zu prüfender Literaturpunkt.
- `auftrag-glm-uebernahme.md` — GLM-Verifikation registrieren + Paper auf
  main führen.
- `auftrag-der-grat.md` — GESCHLOSSEN (2026-09-05): Blatt
  `docs/blatt/blatt-der-grat.md` steht — N = 10, Verdikt aus der Tabelle
  allein: 3 Pfeile (Korona, GIC, Trishuli-Pegel), 6 Stille, 1
  Gegenrichtung (Gyirong); Thuậns These in den Zahlen — die Pfeile sind
  lokal, die Mehrheit der Fronten Stille.
- `auftrag-grat-trishuli-konditionierung.md` — GESCHLOSSEN (2026-09-05):
  Befund `docs/befund/befund-grat-trishuli-konditionierung.md` —
  Regen→Pegel fällt zur Stille unter Konditionierung; 0.265 ist
  Pegel→Regen.
- `auftrag-grat-rest-verifikation.md` — GESCHLOSSEN (2026-09-05): Zelle (b)
  §3.3/§3.7 gespiegelt verifiziert (Gyirong→Rasuwa), Wurzelfix committet;
  Zelle (a) KDE-h robust gelaufen (`trishuli_kde_sensitivity_probe.rs`);
  schwache Regen→Pegel-2h-Kreuzung als Nebenbefund benannt.

### Katastrophen-Recherche (eigene Linie)

- `auftrag-abfluss-trishuli.md` — Abfluss-/Wasserstands-Reihe Trishuli
  (Flut 2026-08-26).
- `auftrag-cog-quelle.md` — COG-Bandquelle für Sentinel-2 NDWI (Seen
  Langjie Cuo / Tuomito).
- `auftrag-seen-kollabgebiet.md` — echte Gewässer & See-Baseline im
  Lirung-Kollabgebiet.
- `auftrag-satellitenbilder-post.md` — offenes Satellitenbild nach dem
  26.08.2026 (Flut-/Narben-Footprint).

### Pflege & Struktur

- PhysioNet-CDN-Manifestation (2026-09-04): die einzige registrierte
  aber unmanifestierte Quelle war `physionet.org/bidsleep_mehrnacht.bin`
  (sources.φ-Referenz auf den gemergten Asset; auf dem CDN lag nur der
  Vorbestands-Chunk `bidsleep_mehrnacht_0.bin`, 2026-08-27, mit unbekannter
  Chunk-Grenze). `physionet-cdn.yml` trug keinen bidsleep-Job. Reparatur:
  bidsleep-Chunk-Matrix (253 Nächte, offsets 0..252 Schritt 36, frische
  Namen `bidsleep_mehrnacht_r0..r7.bin`, da der Merge nicht dedupliziert
  und der Vorbestands-Chunk nicht verifizierbar partitioniert ist) +
  bidsleep-merge. BUG-Fix (Run 2, `2f5ef51`): die `merge_chunks`-Parser
  beider Compiler nehmen alles nach `--merge` als Chunk-Pfade
  (`args[p+1..]`); `--out`/`--ci-mode` standen nach der Chunk-Liste, wurden
  als Chunk-Dateien gelesen → `exit(1)`, von `continue-on-error` als
  Job-"success" verschleiert, kein Upload (daher fehlte auch `ltmm_movement.
  bin` historisch). Fix: `--out`/`--ci-mode` vor `--merge`.
  **bidsleep_mehrnacht.bin MANIFESTIERT (verified 2026-09-05: HTTP 200,
  10.263.728 B).** Die bidsleep-Ernte-Chunks (`_0`, `r0..r7`) wurden mit
  Nachweis (merged-Asset verifiziert) geloescht (2026-09-05, Operator-Wort).
  ltmm offen: Chunks 0/1 (Records 0–71, ~14 GB echter
  ~200-MB/Record-Daten) liefen in die 360-min-Cap (Run 1 cancelled); ltmm
  auf 12-Record-Sub-Chunks `m0..m11` umgestellt (Run 33938814788, m6–m11
  gelandet, m0–m5 in Ernte), merge → `ltmm_movement.bin` pending. Maß ~25 GB
  Rohdownloads über CI; ltmm ist keine sources.φ-Referenz (Extra-Artefakt).
  ltmm-Reste (`ltmm_movement_2/3.bin`, dann `m0..m11`) sind Ernte-Chunks für
  den noch offenen Merge; loeschbar erst nach Manifestation von
  `ltmm_movement.bin` (evidenzbasiert, nicht von Hand vorher).
- `auftrag-docs-reference-verteilung.md` — docs/reference + docs/plans
  verteilen (main-Reinigung).
- `auftrag-sicherung-risiko-heime.md` — Sicherung der
  einzigen-Kopie-Risiko-Heime.
- `auftrag-matrixmachine-register.md` — MatrixMachine ins main-Register
  führen: Urkunden-Zustandszeile nachziehen (Code ist committet, `52eca21`),
  Maschinen-Heimat + Statuszeile; erster Konsument 0 als `pending`.
- `auftrag-verify-references-regelrunde.md` — verify-references Regel-Runde:
  CASE 5 (Archiv-Absolutpfade: strikte Sperre oder dokumentierte Ausnahme)
  + CASE 6 (Fließtext-Drift in lebenden Dokumenten jagen oder als Grenze
  festschreiben) — je Kalibrationslauf, je Commit, getrennt entschieden.
- `auftrag-saubere-datenbank.md` — eine Datenbank über sources.φ / CI / CDN.
  Steps 1-2 committet (cd_reconcile, CDN_ZIEL_SCHEMA). Step 3 (Registry
  zuerst): Verdikt-Ledger `docs/specs/cdn_orphan_verdicts.json` steht — 156
  Orphans klassifiziert; verlässlicher Host-Abgleich gegen sources.φ +
  dead_sources.φ: 80 stale_pending dokumentiert tot, **55 in keinem Register**
  (die eine offene Disposition). **Offen:** SOURCE_PORT-Disposition der 55
  (je Netloc Force-Gate → `sources.φ`-Block oder `dead_sources.φ`-Eintrag);
  14 undocumented `repo_tag` (§1 keine Registry-Heimat) + 3 undocumented
  `dataset_host` (Compiler-Lease) → Step-5-Verdikt. Step 4 (CI-Dedupe)
  ausgeführt (2026-09-05): kernel-flatten von 24 auf 5 Jobs zerlegt (index,
  bodies, jwst-spectra, eve, aia) — 36 Katalog-, 10 Solar-, 3 Radio-,
  cmb/goes/euvs als pro-Quelle-Workflows; eve/aia bleiben (Netloc-Umzug =
  Step 5). Katalog-JOIN-Befund (Schema gemessen): Green-SNRCAT
  (`J/A+A/612/A1/snrcat`) trägt nur Name+Position+Distanz+Typ, kein Flussfeld
  → Force-Gate-decline (position-only), kein sources.φ-Eintrag; `B/psr/psr`
  (tevcat-JOIN) vs `J/ApJS/208/17/psrcat` (Registry, trägt Dist+S1400) =
  alternative Ref derselben ATNF-Pulsar-Familie — Ref-Abgleich als
  Folge-Posten, keine Abdeckungslücke.
  Step 5 destruktiv nur mit Nachbau-Quelle je Asset.

### Register-Lücken des Papier-Korpus (2026-09-03)

Drei Reste aus der Blätter-Übergabe (`~/Schreibtisch/paper`) sind in keinem
Auftrag erfasst; die tauben Anker des Auftrags-Programms sind offene
Register-Pflichten, keine stillen Schwebestände:

- **Leitfrage A — RNG-Umfang corpusweit** (die teuerste offene Unbekannte):
  gilt die Surrogat-/RNG-Korrektur nur dem Takens-Pfad oder der geteilten
  Surrogat-Maschinerie? Antwort aus te.rs / Commit-Historie; entscheidet
  den solar-cycle-Re-Run gegen die Markierung. pending.
- **Versions-/date-Disziplin mit sha-Link je Edit**: fünf Edits bisher ohne
  verlinkten ersetzten sha — der ersetzte sha gehört ins Register, bevor
  weitere Rewrites laufen. pending.
- **text-as-data-pioneer**: p_emp = 0,08 am Raster bei minimalem p = 0,04
  (24 Shuffles) — 200 Shuffles kosten nichts. pending.
- **te_directionality_sweep** (Werkzeug): committet — Benchmark-(c × n)-
  Diagnose der TE-Richtungs-Wiederherstellung, kompiliert, Session-Fenster-
  Fähigkeit verifiziert (5 Zellen laufen, der Voll-Lauf nicht). Gehaertet
  (2026-09-05): der RNG-Seed-Bug ist behoben (`c as u64` kollabierte alle
  c<1 auf Seed 0, jede Zelle teilte einen Seed — jetzt ein per-Zellen-Zaehler),
  und die c=0-Kontrolle ist ergaenzt (ankert das n-Skalieren der Reverse-Fehl-
  pfeile; ein Reverse-Pfeil bei c=0 ist ein Bias, kein Signal). Voll-Sweep
  (jetzt 27 Zellen, 9 c × 3 n) pending/teuer (~2 Sessions): misst den Kreuzungs-
  punkt Reverse×fam als Funktion von (c, n) — n-unabhängig ⇒ Asymmetrie-
  Hypothese bestätigt (Härtungs-Basis), n-wandernd ⇒ Bias-Hypothese (dann
  ETE-Erwägung als eigener Auftrag). Zweck: Validierung der entschiedenen
  TE-RNG-Härtung, billig falls je angefragt. Die (c × n)-Ebene ist sonst
  nirgends im Benchmark (der Schreiber-Test läuft bei fixem c, fixem n).
  pending.
- **Referenzierte, hier fehlende Dokumente** — Verbleib geklärt (2026-09-03):
  die Herkunft der 20-s-These (`handover-2026-08-30-zwanzig-sekunden-herkunft`)
  ist als Provenienz-Abschnitt (§5, Stopp-Punkt = Deduktion 11, 8767 =
  Turyshev-&-Toth-Zählung, Provenienz-Datenbank nicht mehr vorhanden) in
  `docs/concepts/das-eine-instrument.md` eingearbeitet — konsumiert. Die
  übrigen per see-also zitierten legacy-Dokumente (`docs/audit/*`,
  `survey-2026-08-30-provenienz-karte`) sind nicht einstellbar: sie tragen
  Alt-Ordnungs-/Hash-/Alt-Paper-Namen und tote Anker, die hier nicht bestehen;
  ihr Messgehalt ist als Kleinpass-/Auftrags-Pflicht registriert, kein
  Verbindungs-Kopie. `auftrag-merge-fix-welle` ist abgeschlossen (eigenes
  Ergebnis-Audit: die drei Punkte stehen bereits auf main) — konsumiert.
- **Verlorene Deduktion-/Method-Dokumentation im Code** (Provenienz-Audit
  2026-09-03): der deutsche Deduktion-Kommentarblock der historischen
  `link_deduction_probe` (heute `pioneer_link_correction_probe`, Zeilen 1–153,
  ~33 Deduktionen + 0/0b) ist im heutigen Code verloren — die Schritte
  überleben nur als englische Laufzeit-Labels „Deduction N" (1:1 in der
  Zählung). Ebenso verloren: deutscher Kopf von `pioneer_text_korrelation`
  sowie die englischen Method-Köpfe von corona ×4, solar-cycle ×6 und
  `signal-cone-audit-probe`. Die Paper tragen nur eine Teilmenge der
  nummerierten Karte (Deduktionen 1–6 und 32–40 stehen im Code, nicht
  einzeln im Paper; `ground-sources-20s-band` nur zusammengefasst). Offene
  Pflicht: die vollständige Deduktion-Karte aus der Backup-Fassung in
  `probe-front-dark-matter.md` als Register-Heimat nachziehen (Sprache:
  Deutsch gehört ins Register, nicht in den Code). pending.
- **GIC-Stationsname ABK vs SOD (Bezeichnungs-Diskrepanz, nur registriert):
  der geomagnetische Messkanal läuft über die INTERMAGNET-Abisko-Station (ABK,
  via BGS GIN HAPI); die erzeugenden Proben (`bz_blatt_probe` / `bz_retro_probe`)
  messen nur Abisko. Ein separates Stations-Label „SOD“ (Sodankylä) erscheint im
  Paper-/Auftragstext (z. B. `docs/paper/gic-causal-driver.md` §4.5,
  `docs/auftrag/auftrag-gic-p-wert.md`). Abgleich offen — nicht aufgelöst,
  keine Messzahl geändert. pending. (Messbefund 2026-09-04: der
  `bz_retro_probe --station SOD`-Lauf holt eine von ABK getrennte Serie —
  md5 der Caches verschieden; die „nur Abisko"-Notiz gilt nur für
  `bz_blatt_probe` (ABK-hardcodiert), nicht für `bz_retro_probe`.)

- **Stations-dB/dt als CDN-Source — Rat-Entscheidung (2026-09-04):** das
  Gremium entschied einstimmig: Boden-dB/dt gehört als **flache Messreihe (b)**
  ins System, **kein Feld-Kanal** (kein position/force/τ, kein ω()-Konsument;
  dB/dt ist die abgeleitete Echo-Reihe des xyz-B-Messkanals, der bei ABK
  bereits Feld-Kanal ist — A = A: ein Sensor, ein Oszillator). Daraus:
  eigener `intermagnet_dbdt`-Bin (eigene Magic, eigener Loader in
  `src/archivar`), Station = Datum im Record (nicht im Dateinamen, Auflösung
  der ABK/SOD-Diskrepanz durch Benennung ist abgelehnt), kein Wiedereinbau
  in die omni2-Sonnenwind-Comp-Taxonomie. Bau-Auftrag offen: Compiler
  `tools/harvest/src/bin/intermagnet_dbdt_compiler.rs` (BGS-GIN-HAPI-Jahresschleife
  deterministisch, Bucket-Maximum wie `bz_retro_probe`), sources.φ-Eintrag
  (`format intermagnet_dbdt` + `on earth` + ttl, ohne field-Tokens),
  Workflow-Job mit Idempotenz-Guard, Probe um CDN-Lesepfad erweitern,
  Manifestation nur über CI (`--ci-mode`). Bestehende Schuld: auch
  `omni2_serie_1h.bin` ist unmanifestiert. pending.

## Nadel Ⅲ — Coronal Heating (TE-Messprotokoll)

Messreihe archiviert: `archive/messreihe-nadel3-corona.md`. Befund lebt
in `docs/paper/corona-heating-ladder.md`. OFFENE PFLICHTEN vor einer
physikalischen Aussage — kein Blatt ohne diese:

- **AIA-fam-Tool + Zahl (2026-09-05)**: `aia_ladder_probe` rechnet den
  Full-Round-Family-Bound wie `corona_ladder_probe` (6 Paare × 13 Lags × 10
  Surrogate). Das manifestierte aia2014_lines.bin war nur ein 1-Tag-Stub; das
  volle 2014-03-01–05-30-Korpus (3 Monate × 7 Bänder, 4.522.425 Records,
  1-Tage-JSOC-Chunks) ist geerntet (lokal, /tmp), GOES-13+15-Trigger besorgt.
  Gemessen (fam-getestet): GOES-15 194 Ereignisse fam = 1.89e-1, 335→94-Spitze
  +1.42e-1 < fam → stumm; GOES-13 208 Ereignisse fam = 1.80e-1, +1.40e-1 < fam
  → stumm. Kein Instrument belegt die Heiß-Rung-Richtung auf fam-Niveau; der
  „EVE ab/AIA auf"-Zwiespalt löst sich auf (bekannt-schlecht geschlossen).
  Paper v3 + Blatt-Survey aktualisiert. Korpus manifestiert aufs CDN
  (jsoc.stanford.edu/aia2014_lines.bin, 90 MB, ersetzt den 1-Tag-Stub;
  Workflow auf --chunk-days 1 gestellt). geschlossen.
- Mehrfachvergleichskorrektur über die Matrizen und Kanalpaare (2 Pfeile
  bei 20 getesteten Paaren ohne Korrektur — der erwartete
  Falsch-positiv-Bereich ist nicht verlassen);
- Lag-Wahl: lag 0 ist Default, kein Sweep — Robustheit ungeprüft;
- KDE-Bandbreite: Silverman-Heuristik, Sensitivität der Urteile gegen h
  ungeprüft;
- Fenster-Kongruenz: OMNI↔GOES-Schnittmenge bleibt leer (stopDate 06.08.);
- nobel_probe_corona v2 (Multi-Force-TE): die bedingte Multi-Force-TE
  (alle Kräfte im Phasenraum, DAG über alle Paare und Verzögerungen) ist
  pending;
- Desktop-Fork (GTX 970): der Lauf mit 30-Jahres-Daten braucht die GPU
  (1664 CUDA-Cores) — O(n²) × Surrogate-Kosten gegenrechnen
  (~80–90 min gemessen);
- 90-Tage-Archive für den Lauf (Bz/GOES/GONG): GONG steht (31 Jahre);
  Bz/GOES hängen am GOES-30d-Archiv-Block und am OMNI-Ingest-Verzug;
- g-Moden-DETEKTION: verifiziert UMSTRITTEN (Fossat 2017 vs Schunker 2018/
  Appourchaux 2019) — register-nur, kein erntbares Quantum; der echte
  Oszillator ist die BiSON-p-Moden (`bison_compiler`), die g-Moden-Suche
  selbst war nie gelaufen (pending, nicht 0 honored);
- GOES-R-Retro (2017-2025): CI-Job `goes_r_xrs` läuft; das Asset
  `goes_r_xrs.bin` fehlt bis dahin auf dem CDN;
- Wind/WAVES: erster kernel_flatten-Lauf offen (bis dahin trägt das CDN
  das Asset nicht), danach 2022+ (der Baum endet 2021);
- CDAWeb-Live-Block (SOLO_L2_RPW): Publikations-Lag ~5 Monate lässt das
  {hour_ago}-Fenster heute leer (0 honored), sobald die NASA erweitert,
  fließt der Kanal.

## Der Sonnenzyklus — die Dynamo-Ernte

Messreihe archiviert: `archive/messreihe-sonnenzyklus.md`. Befund lebt in
`docs/paper/solar-cycle-dynamo.md`. OFFENE PFLICHTEN:
- die unabhängige Gegenprobe auf die Rohtabelle (Bison-Shift-Rohwerte);
- der generische ~1,8x-Saettigungsfaktor (Svalgaard 1978) bleibt bewusst
  unangewendet — benannt.

## Die Sphären des Unsichtbaren

- Atom 2 (Ringe: eigener rings-Buffer + WGSL ring_transmission,
  Literatur-τ mit Provenienz) — offen, eigene Session.
- Atom 3 (Warp: Linsen-Kompiler — Gaia-BH-Kandidaten + ATNF-Pulsare mit
  gemessener Masse; WD-Modell-Massen ausstehend; f64-Fold-Muster aus
  Atom 1) — offen, eigene Session.
- Atom-1-Grenzen (registriert): der 3D-Orbit des Planetenpunkts bleibt
  ausstehend — Ω (Azimut im Sky-Frame) ist ungemessen, der Schatten ist
  Ω-frei, ein Punktorbit wäre geraten; der Transit-Schatten ist seit
  Atom 8 tot — die Rückkehr läuft über die Feld-Absorption (pending,
  unten); pscomppars trägt mehrere Parametersätze je Planet und keinen
  default_flag — erster Satz je Planetenname zählt; fehlt ein Element →
  kein Schatten (0 honored).
- LuckyStar: decline (Vorhersagen sind Modell, keine Messung; die
  Ergebnisse-Server liefern nur abgeleitete Fits) — der rohe
  em-Lichtkurven-Kanal der Fresnel-Sphäre bleibt ausstehend.
- Okklusions-Reste → Feld-Absorption (pending): kontinuierliche Opazität
  (Partial-Transmission), atmosphärische Dämmerung, kleine Skala
  (Terrain/Bauten — der Mechanismus ist skalenfrei, die Daten fehlen),
  Oszillator-Eigenradius als Rekord-Slot, Transits als Feld-Dämpfung.
  Die geometrische Okklusion (Ephemeriden-Barrieren) starb in Atom 8;
  der absorption-Slot lebt im Protokoll — das Atom ist die Manifestation.
- Atom 1 deckt den Weg für Ringe/Warp — noch kein Konzept-Dokument.

## Materie-Physik — Kuprat, Phononen, Suprastrom

- RIXS-Spin-Ernte (em): erledigt — Spin aus Zenodo 7286412 (siehe
  Kuprat-Blatt). Ladungs-/Plasmon-Kanal (electric, Bi-2223, Zenodo
  15179114, 107 Spektren): geerntet 2026-09-03 zu
  `rixs_charge.bin` (charge v3) auf dem ssd.jpl.nasa.gov-Netloc via
  `cuprate-cdn`-Workflow (Zenodo 15179114 → `--plasmon`, `--ci-mode`;
  Spin aus 7286412 → `--rixs`, `--ci-mode`) —
  158727 Oszillatoren nach Loss-only-Reduktion (Rat-Urteil): die
  181350 Roh-Zeilen trugen 22516 Anti-Stokes-Gain-Zeilen (negative
  Energie, thermische Population/Bose-Faktor — echte Messung, aber
  Absorption, kein sendender Ladungs-Oszillator) + die elastische Linie
  (f = 0, kein Energieübertrag); beide benannt ausgeschlossen, Roh-Heimat
  bleibt Zenodo 15179114. `charge_oscillators` hält e > 0 wie
  spin_oscillators/harvest_eels (negative Frequenz ist keine). Die
  val-Physik (relativ, a.u.) ist die ehrliche Messung — kein
  fabrizierter Querschnitt. Rat-Urteil (2026-09-03): **kein Feld-Kanal.**
  Das Streu-Photon ist ein sendendes em-Signal (ehrlicher Lab-Anker;
  Beamline unverifiziert → `pending`), aber `val` ist relative
  Streu-Intensität (a.u.) auf der Energie-**Verlust**-Achse — ein
  Material-S(q,ω), keine freie Feldgröße; Magnon/Plasmon sind im
  Kristall gebundene Anregungen, keine Ausbreitung ins Feld. Die
  Verweigerung ist 0 honored, kein Gap — und sie unterscheidet sich
  von SRD62 (das an Force/τ/Position scheiterte): RIXS scheitert am
  Feldgrößen-Gate. Force-Korrektur (A = A): der sendende Träger ist
  für Spin UND Charge **em** (das „electric"-Label benennt die
  Anregung, Plasmon = Ladungsschwingung, nicht den Träger; force 8
  trägt ohnehin keinen a.u.-Marker). Die Proben bleiben
  `measure`-Bürger; Kalibrierung (absoluter Querschnitt) + Beamline-
  Koordinate + Linienbreite-τ sind `pending`.
- Kuprat-Blatt (rixs_cuprate_probe): geerntet ist der Spin-Kanal (19
  Spektren, 456 Oszillatoren aus Zenodo 7286412 — Bi₂Sr₂CaCu₂O₈₊δ,
  azimuthal_analysis/sw_spin.txt, Dotierungsklassen UD/OD1/OD2 je
  (q_h,q_l); das RIXS-Streu-Photon ist ein sendendes em-Signal, daher
  ehrlicher Lab-Anker); die SRD62-Suprastrom-Ernte ist eine
  Material-Property-Messung (ρ_s, s.u.), kein Feld-Kanal; Gitter bleibt
  ungeerntet, und die Dotierungs-Achse trägt 3 Klassen < MIN_N 30 — das
  Blatt trägt „keine Aussage" (Stille ist der Befund). Die restliche
  Kanal-Ernte (Phononen = acoustic) und NSE-I(q,t) bleiben die
  Voraussetzung für eine nicht-degenerierte Matrix.
- Suprastrom-Material-Property (ρ_s ∝ λ⁻², aus Penetration Depth): zwei
  benannte Zugänge — (a) ISIS `10.5286/isis.e.rb2410595` (Hussey et al.,
  µSR-Eindringtiefe von Bi-2201, MUSR) ist embargoed bis **2027-08-10**,
  dann roh (NeXus/RAW) frei; (b) NIST SRD 62 `10.18434/t4kp8j` (High-Tc
  Superconducting Materials Database, public domain, HTTP 200) trägt
  die Eigenschaft **Penetration Depth** (Literaturwerte, reduziert —
  gemessen, nicht roh) als Web-Abfrage. Gebaut (2026-09-03):
  `srd62_compiler --out <dir> [--ci-mode]` (Probe HTTP 200, 25
  Citations) + `srd62-cdn`-Workflow manifestiert `srd62_suprastrom.bin`
  auf dem ssd.jpl.nasa.gov-Compiler-Netloc. Draht v7: Serienmodell `id`
  (Quelle) + `label` (Feldrichtung/Dotierung/Bedingung, je Zeile eigene
  Serie), Reader + λ⁻²-Konversion im Modul `suprastrom` (parse/encode +
  Tests), Proben `suprastrom_cuprate_probe` + `suprastrom_form_probe`
  (ρ_s ∝ λ⁻² je Serie). Parser-Korrektur v6→v7: Spalten am Kopf erkannt
  (Penetration = Wert, Temperature = Achse, übrige Spalten =
  Serien-Schlüssel) statt „erste andere Zahl" — die naive Achse hatte
  Feldrichtung und Dotierung konflatiert (A00316 schien eine
  50-Punkte-„Serie" mit λ-Sprüngen 0,2↔1,2 µm; die echte Tabelle misst
  //ab 0,14–0,37 µm und //c 1,04–2,10 µm getrennt je Dotierung).
  Tabellen ohne Temperatur-Achse (Film-Dicke/Sample/Magnetfeld-Scans)
  sind keine ρ_s(T)-Quelle, entfallen ehrlich (0 honored). Ernte v7:
  232 Punkte, 65 Serien. Form-Befund (2026-09-03,
  `suprastrom_form_probe`): die Zwei-Flüssigkeiten-Form
  ρ_s ∝ 1−(T/Tc)⁴ wird von mehreren unabhängigen Serien getragen
  (RMS/ρ₀ ≈ 3%: U00037 Tc 89,1 K über 22 Punkte bis 88 K —
  Übergangsregion beprobt; A00261, A00316, A00395 ähnlich); Auswahl
  nur Serien, deren Daten die Übergangskante erreichen (Tc nicht
  extrapoliert). Ein-Material-Zahl < MIN_N → noch keine
  material-übergreifende Aussage; Einzel-Material-Form als Befund
  registriert. PSI hat keinen offenen Kuprat-ρ_s(T)-Datensatz (nur
  Kagome/Nickelat). Rat-Urteil (2026-09-03): **kein Feld-Kanal.** ρ_s
  (m⁻²) ist eine abgeleitete Material-Eigenschaft — Temperatur-Achse,
  Material-/Dotierungs-Identität, kein ICRS-Ort, keine Ausbreitung; sie
  scheitert korrekt an allen drei Archivar-Gates (Force/τ/Position) und
  wird nicht in `phi/sources.φ` registriert (die Verweigerung ist 0
  honored, kein Gap). Die Proben sind `measure`-Bürger, korrekt benannt
  als Proben, nicht als Feldquellen. Der electric-**Feld**kanal bleibt
  `pending`; er wird erst Feld-Bürger über eine Messung, deren Sache
  selbst ein sendendes electric/em-Signal ist (Streu-Photon /
  THz-Feld mit freq/bin_width, ICRS-Lab-Anker, force em/electric, tau)
  — dieselbe Form, die `rixs_spin.bin` schon trägt. Der Feldkanal wird
  heute schon von echten sendenden Quellen gespeist (solo_rpw_e_rms_vm,
  swarm_*).
- NSE-I(q,t): KEIN offener/embargo-datierter Datensatz (erschöpfend
  belegt, vier Runden: ILL/ISIS/NIST/ORNL/PSI/J-PARC/TRIUMF +
  Zenodo/Figshare/Dataverse/OSF/NOMAD/Materials-Cloud tragen nur
  Soft-Matter-NSE oder Facility-NSE für andere Materialien; Kuprat-
  Treffer sind durchweg INS-Spinkorrelationen S(q,ω) oder NMR-„spin
  echo" — nie I(q,t). Kuprat-NSE (Hayden 2010, arXiv:1008.4298)
  predatiert die Open-Data-Ära → nur Figur). Die Phasen-Quelle bleibt
  pending — kein Substitut, keine Digitalisierung; der Harvester steht,
  sobald Zugang/Embargo kommt.
- ANGEFRAGT (2026-08-23): der Operator hat B. Keimer (MPI-FKF,
  corresponding author der Haug-2010-NJP-Arbeit) transparent um die
  reduzierten I(q,t)-Echo-Kurven gebeten — WARTET AUF ANTWORT; keine
  weitere Suche nötig, der Zugang ist benannt und angefragt.
- Phononen-Messung: einzige offene Quelle ist Zenodo 21859473
  (EELS-Phononen Bi-2212, 25 Profile, 15050 Oszillatoren, acoustic,
  erledigt 2026-08-23); µSR und NSE bleiben pending (nur Figures/Login,
  0 honored — keine Digitalisierung).
- Regeln: kein Namens-Trick (Frequenz lebt als Token, nie im String),
  kein Skalar-Schallpegel aus Spektren errechnet, jedes Atom ein
  vollständiges Session-Artefakt.

## Stern-/Asteroiden-Physik — abgeleitete Geometrie + Ernte-Folgen

Die Daten sind geerntet (Sternkinematik pmra/pmdec/rv + Farbe
Teff/BPmag/RPmag/Gmag via gaiadr3-Crossmatch; Asteroiden-Größe via
NEOWISE/AKARI in `phi/pipeline/catalog/asteroid_diameters_*.φ`). Offen ist
die Nutzung — reine Geometrie, die sonst nirgends liegt, weil alles einen
ICRS-4D-Rahmen teilt:

- Hill-Sphäre je Asteroid: r = a·(1−e)·(m/3M☉)^⅓ — Formel repariert;
  `hill_radius_m` ist heute nur Gate (is_none im Hash,
  src/archivar/spatial.rs:175), der Wert fließt nirgends — Manifestation
  (Hill-Radius als räumliche Reichweite) bleibt offen.
- Hydrostatische Abplattung aus Rotation: Rotationsperiode (LCDB) +
  Radius (NEOWISE) + Dichte (Masse) → Oblatheit im Gleichgewicht (drei
  Kataloge übereinander, niemand macht das systematisch).
- Co-moving Gruppen / Sternströme: Position + 3D-Geschwindigkeit →
  Mitgliedschaft als Geometrie des Geschwindigkeitsfelds.
- Sternbegegnungen: welche Sterne nähern sich der Sonne (Gl-710-
  Problem), für JEDEN Stern live.
- Paarweise 3D-Sternabstände (N², auf Anfrage).
- Oberflächengravitation + Fluchtgeschwindigkeit der Asteroiden mit GM:
  g = GM/r², v_esc = √(2GM/r).
- Neue Quellen (grind-pro, heikler Join/Parsing): LCDB-Rotationsachsen
  (Pol, nicht nur Periode), DAMIT-Formmodelle (3D-Formen → j2/r_eq).
- Empfohlene Reihenfolge: Hill/Abplattung → LCDB/DAMIT.
- H-Schätzung vs. NEOWISE für die Körper, wo DASTCOM einen abgeleiteten
  (nicht gemessenen) Radius trägt — registriert, nicht entschieden.
- Sternbin-rv: das CDN trägt `dr3_stars.bin` = 75.001.828 B = exakt
  1.704.587 × 44 (Stichprobe 200/200 mit rv ≠ 0). `bright_stars.json`
  (45 Records, V<1.94) trägt kein rv — gemessen: die 45 hellsten sind
  oberhalb der Gaia-Bright-Limit (Altair fehlt in
  gaiadr3.gaia_source_lite) — das Fehlen ist die Messung, nicht die
  Lücke (0 honored).
- TESS-Ernte (CI-Schritt steht): der CI-Schritt (kernel_flatten catalogs)
  trägt `--limit 16` (2⁴) — der volle 782-Sterne-Satz (disc_facility =
  TESS) überschreitet Fenster (≈3 min/Stern gemessen) und
  Sample-Budget; die Endzahl entscheidet das Sample-Budget-Atom.
- CDN-Rekompilat ephemeris v3: die ephemeris_{body}.bin-Assets sind noch
  v2 — der nächste kernel_flatten-Lauf schreibt v3 (0x02 + u16-Präsenz-
  Maske). Bis dahin liest der v2-Arm (CI-Reihenfolge eingehalten: Code
  zuerst, Rekompilat folgt). Bis dahin tragen alt-Slot und GM-Slot das
  benannte Wire-Pad.
- kernel_flatten-Neulauf: ephemeris_compiler n_sections 2→3
  (rotationslose Körper wurden verworfen, Rotation abgeschnitten) —
  CDN-Neukompilat verifizieren (rotationslose Körper laden, Rotations-
  Matrizen präsent).

## Spektrale Achse — offene Pflichten

- Dispersionsrelation: die Laufzeit-Geschwindigkeit bleibt band-flach
  (v = PROPAGATION_SPEED[force]); eine echte Dispersionsrelation
  (Rayleigh-Oberflächenwelle) ist pending — die Steckstelle v(freq)
  steht, kein erfundenes v0·(f/f0)^β (0 honored).
- SED → BP−RP: der chromatische Dip als SED-Messung — die Farbe einer
  Spektralquelle aus ihrer SED (SED → BP−RP → color_index → bestehender
  LUT) ist PENDING — die Gaia-DR3-Passbänder (BP/RP-Antwortkurven) sind
  die Ernte-Konstante; color_index bleibt 0 (weiß), kein erinnertes
  Passband (0 honored).
- Farbe-Render-Bein (PENDING): die `color_lut_rgb`-Farb-LUT ist in der
  Membran gebacken (Bindings 9+12), aber der Browser-Render-Pfad
  (Browser-Relay) bindet `color_lut_rgba` noch nicht als Textur — die
  Browser-Station zeigt die Spektralfarbe nicht.
- Passband-CDN-Umzug (PENDING): die Gaia-EDR3-BP/RP-Passbänder (Riello+
  2021, 781 Stützstellen) sind lokal in `sed_to_bp_rp` eingebettet; sie
  gehören auf die CDN (Asset), damit der Compiler die Ernte aktualisieren
  kann statt sie neu zu kompilieren.

## Archivar & Werkzeuge — offene Pflichten

- **EVE-/AIA-Linien als series-Format registriert (2026-09-05)**: `eve_lines`/
  `aia_lines` parsen in `series_parse_bin` (`src/archivar/extract.rs`) über eigene
  Modul-Parser (`EVL1`/`AIA1`, 20-Byte-Records t/v/idx) + Roundtrip-Tests; gegen das
  reale CDN-Asset verifiziert (aia2014_lines.bin, 50362 Records, Länge 8+count×20
  exakt). Die sources.φ-Assets (`at sun`, 558b92e) lesen damit am Loader nicht mehr
  void. Bewusst offen: series_component_name-Namen + field-Tokens — keine fabrizierten
  Kanal-Namen ohne series-Konsument. pending (nur wenn ein series-Konsument die Reihe
  über den Archivar zieht).
- feature-gate `gpu` — eigenes Atom, pending: `pub mod mathematikerin` als
  #[cfg(feature="gpu")] + Co-Gate der main_flow-Verdrahtung (crate::
  mathematikerin::-Stellen PresenceFrame/EMOscillator/KineticRadiator)
  + Feature-Propagation zum Default-Bin — kein Ein-Zeilen-cfg, ein Faden
  durch die ω-Loop.
- Ephemeriden-Kaltstart: Frame-Anker laden jetzt als erste Phase über
  `curl --parallel --parallel-max 8`; die Membran zeigt das Sternfeld
  sofort, die Planeten folgen. Offen: per-Anker-Extraktion (sun/earth
  sofort extrahieren statt nach der ganzen Anker-Phase) für wörtliches
  „Sekunden"-Laden; der Kalt-Download (~360 MB) bleibt einmalig bis zum
  Warm-Cache.
- C_LIGHT konsolidieren (PENDING): `omega::C` + `odp::C` →
  `crate::archivar::types::C_LIGHT`; `solar.rs`-Literal → `C_LIGHT`.
- „41 Parser" klären (PENDING): gemessen sind 49
  `pub mod`-Format-Module in `archivar/mod.rs:10-58`; die „41" ist eine
  Teilmenge, deren Definition das Register klären muss.
- Ephemeriden de442 size 0 (PENDING): `de442.bsp`/`de442t.bsp` tragen
  `size 0` im Index (absent, nicht null-echt); das Frische-Gate matcht  size-0 nie → würde ewig re-fetchen. Berührt src/archivar.
- Ephemeriden de441 Range-Request (PENDING): `download_missing` auf
  SPK-Range lesen (spart 3,31 GB je CI-Lauf).
- DB-Pflege (2026-08-27, wiederkehrend): die opencode-DB
  `~/.local/share/opencode/opencode.db` wächst durch Event-Sourcing auf
  mehrere GB. Gemessen: 2,98 GB, 48.265 events, freelist 233.409 Seiten
  ≈ 955 MB frei nach Session-Löschung. VACUUM gibt die freien Seiten
  frei, braucht aber exklusiven Zugriff und scheitert, solange opencode
  läuft. Regel: nach dem Löschen verwaister Sessions und wenn opencode
  beendet ist, einmal `sqlite3 ~/.local/share/opencode/opencode.db
  "VACUUM;"` ausführen. Bei normaler Beendigung schrumpft zusätzlich die
  WAL.
- Werkzeug-Reibung durch die TE-Maschine (pending): die Messung ist
  gescopt. Reihe = die Wiederholungs-Skalare je Tool-Aufruf
  (`call_similarity_t = sim(fingerprint(args_t), fingerprint(args_{t−1}))`),
  Begleiter `token_cost` (Args-Länge). Zelle = der Tool-Aufruf (Event),
  nie Turn, nie Wanduhr. Richtung zweischichtig: primär das PE-Gate auf
  der call_similarity-Reihe (|pe−mean|>2·sd, live-data-Baseline — kein
  Null nötig), dazu die TE-Schicht `TE(token_cost → call_similarity)` mit
  Lag = MI-Lag. Null = Block-Bootstrap
  (`surrogate_stats_block`/`topological_te_block`, schon in te.rs),
  Blocklänge = mittlere Run-Länge; die phasenrandomisierte Null ist für
  die kategorische Treiber-Reihe ausgeschlossen (keine Phase).
  Index-Skalarisieren des Tool-Typs ist abgelehnt (Fabrikation). Die
  Heuristik-Schwellen fallen mit friction.rs. Ort: on-the-fly im
  Interceptor als neues Modul (`src/tool_te.rs`) + Ernte des Tool-Stroms
  in ein neues Ledger (`phi/llm_tool_ledger.φ` — das bestehende
  `llm_gate_ledger.φ` trägt nur Verdicts, keinen Strom). n<30 fließt
  stumm.
- Webhook-Empfänger: der Sender `tools/work/src/bin/smail.rs` ist gebaut
  (REST über curl, `CLOUDFLARE_ACCOUNT_ID` + `CLOUDFLARE_API_TOKEN`,
  Tests grün); offen bleibt der Webhook-Empfänger und der
  Cloudflare-Worker davor (Lesepfad = Cloudflare Routing → Worker →
  Webhook).

## Ausgabe-Flächen & Sensoren

- SurfaceRadiator-Implementierungen offen: Bluetooth (Smartwatch) und
  HID (Force-Feedback); Vibration hängt am ESP32-Prototyp. (Serial-TX
  lebt: OMEGAFLOW_SERIAL_OUT, 115200, eine Zeile je Tick.)
- Kamera/Mikro/IMU nativ: die Daten existieren, der Sensor-Pfad fehlt
  (Batterie + Zustimmungs-Gate leben).
- Gamepad-Oszillatoren: die gilrs-Steuerung lebt hinter
  `--features gamepad` (Navigation: fold/jump/Rotation); das Gamepad als
  Sensor-Oszillator ist offen — die serielle Ingress-Vokabel deckt
  ESP32, HID-Gamepad steht aus.

## Browser-Relay

- refused-else ohne body-Deklaration (Relay-Rest): SurfaceFlow für
  spd/hdg lebt (index.html 236-249, frame_motion in
  src/archivar/membrane.rs:166) — der offene Rest ist nur noch
  refused-else ohne body-Deklaration.
- Der eingefrorene index.html/fieldShader-Snapshot trägt die tote
  Rotation noch (GRID_TO_ANGLE = 2^62, index.html 42/1245) — bleibt
  registriert, falls der Relay wieder auflebt.
- M01 WebSerial-flow-Protokoll: zwei Spezifikationen konsolidieren —
  4d-membrane.md (`flow <force_name> <force_id> <|Ω|> 1 <tick_ms> <t>
  <x> <y> <z>`) vs. docs/omegaflow_sense_hardware.yaml (`flow <channel>
  <mode> <value> <unit> <duration_ms> <t> <x> <y> <z>`). SeismicOscillator
  schreibt heute die rohe f32-Σω-Intensität (4 B/Frame) an den Port
  (src/mathematikerin/actuators.rs, SeismicOscillator).

## Membran & Wahrnehmung

- Device-Lost-Befund + Farb-LUT (2026-08-20): Mesa 25.2.8 (ANV/Vulkan)
  verlor das Device beim Kompilieren des Feld-Fragment-Shaders
  (create_render_pipeline → Parent device is lost). Bisektion im
  Test-Modus: Okklusion/Stern-Tiles/Tile-Cull/omega-Akkumulation/hsl_to_rgb
  leben — `temperature_to_rgb` im dynamischen Loop tötet (die
  31-Stützstellen-Interpolation Pecaut-Mamajek + Helland-Polynome
  überfordern den gen9-Compiler). Fix: die Wahrheit wanderte in den
  Archivar — `omegaflow::spectral::color_lut_rgba` (256 Bins,
  Rgba32Float, Nearest, NonFiltering) als LUT-Textur (Binding 9+12);
  WGSL sampelt `color_lut_rgb` (weiß bei ci==0); die drei WGSL-Funktionen
  starben — eine Quelle, kein Duplikat. Benannt: Mesa 25.0.7 schluckte
  das Konstrukt, 25.2.8 ist strenger; ob der OOM-Befund (GPU-Thread-
  Panik beim Pipeline-Bau) identisch ist, trägt die nächste Prüf-Rolle;
  ein Upstream-Bericht an Mesa/wgpu ist ein eigenes Atom.
- M02 ESP32-Mantis-Shrimp-Firmware: docs/omegaflow_sense_hardware.yaml
  existiert (35 Sensoren/Aktuatoren). Offen: no_std-Rust-Firmware;
  Browser-Seite (actuate) + M01.
- M03 Audio-Gain ohne tanh: index.html windowMedianExtent() →
  tanh(Ω·median) — Median mit ∞-Extents ungelöst; Normalisierung auf
  die reine Messung steht aus.
- M04 Navigation (Nebra-Kalibrierung): Wheel-Divisor 128 im Hauptpfad
  (Touch-Pfad 512); Initial-Scale: gridStep = 2**31 → 2³⁷; die native
  Parität (−/= ×4, keine Wheel-Kalibrierung) ist offen.
- M05 Station-Sensoren als SI-4-Token: recordSample(name, value, force,
  unit) + convert_to_si im Archivar (Mikrofon→Pa, Kamera→lx,
  Accelerometer→m/s², Magnetometer→µT). „biotic" kollidiert mit der
  Force-Registry — klären.
- M06 Wetterstation-Debug-Konsole: Konsole als 4-Token-Spiegel
  `name [force, unit]: SI-Wert`.
- M07 Command Palette ⌘K: SIMBAD-TAP-Objektsuche (Presence-Jump),
  lokaler Source-Index, Force-Filter.
- Wetterstation: der 4-Token-HUD („wind_speed [advective, m/s]") fehlt
  nativ — kommt mit der Messreihe.
- Advective per-Quelle: Wind in tm.w (Kanal verdrahtet, Messquelle
  fehlt).
- OPeNDAP-Integration.
- Camera: ~19k Pixel-Quellen (4×4-Raster) → WS-Traffic-Hotspot.
- `sensor_config`/`probe_classify`-τ/TTL-Konstanten (60/300/0.01/3600)
  ohne Herleitung — Draft-Konvention: die Werte sind die
  Sensor-Registry-Kadenzen (serial 60 s, battery 300 s) und die
  Quellen-TTL-Familie (86400) — KEINE Messungen der Quelle; die τ-Gate
  beim Einbau entscheidet.
- `getRto` min/max 100/5000 ms — nicht 2ⁿ/Φ-hergeleitet
  (constants.js).
- Probe: `coordinates.2` als alt vs. Tiefe bei Seismik — Vorzeichen
  offen.
- Browser-Sternfeld bleibt pending (eigene Belichtungsrampe; der
  Browser-Relay trägt die dunkle Diode floor = [0;9]).

## Operator-Messungen (ausstehend)

- Radial-Profil eines isolierten breiten Gauß-Punkts (e^(−r²/2)) am
  Fenster — Messung + e/E/P-Gefühl gehören dem Operator.
- Sternenhimmel relativ zur Live-Em-Referenz (ft_ref) statt absolut —
  die Diode ist exakt relativ zur Live-Referenz; die Operator-Messung
  bleibt ausstehend: Wie atmet der Glow beim Übergang Sonnen-Nähe →
  tiefer Raum, wann erscheinen die achsennahen Sterne im Blick-Sweep.
- Galaxien-Zoom-Verifikation: der alte deep-Zähler starb mit Atom 8 —
  offen ist das Operator-Gefühl für die Glows im tiefen Raum (Proxima
  bei 4,2 ly ≈ 2^45,5); keine tiefaufgelöste Vorab-Integration.
- Fireball-Operator (sum vs. mean im `fold`) — Live-Verifikation offen.
- Audio-Phasen-Invariante dokumentieren: sr = 44100, ganzzahlige
  Frequenzen, 1-s-Noten → glatter Nulldurchgang am Tick-Ende; bei
  sr-/Frequenzwechsel bricht sie.
- Sternenhintergrund (integrierter Glow der 1/d²-Schwänze, Milchstraße):
  der Glow ist die lebende Summe der Diode — die Messung gehört dem
  Operator (kein vorab integriertes Feld).

## Wahrheitsfindung — offene Urteile

Der Mechanismus gegen den Verlust: **kein Top-N — das Verzeichnis ist
vollständig.** Jede Funktion des Systems, jedes Konzept, jede fehlende
Funktion trägt ein Urteil. Der Inventar-Prozess ist wiederholbar:
`grep -nE "^\s*(pub\s+)?(async\s+)?fn"` über src/main.rs + src/lib.rs +
tools/work/src/bin/* + tools/live/src/bin/* + die WGSL-Entry-Points
(`@vertex/@fragment/@compute fn`) + `docs/concepts/*` + die Registry
(phi/sources.φ, phi/dead_sources.φ). Urteile: **WAHR** (die Messung ist
die Messung der Sache selbst — der Gradient schweigt), **UNWAHR**
(Fabrication, Ersatzwert, Default — der Gradient spricht), **AUSSTEHEND**
(die Daten existieren, die Forschung oder der Bau fehlt), **ERSETZT**
(von einem stärkeren Gesetz abgelöst — ehrenhaft), **VERSIONIERT**
(gesichert, wartet). Erledigte Urteile trägt Git — hier stehen nur offene
und navigierende Zeilen.

### Die Concepts (offene und navigierende Zeilen)

- **Unter der Nachweisgrenze** (Arbeitsname „leise", Vorschlag des
  externen Sparrings, 2026-08-23): der 0-Kanon kennt drei Gründe für
  „kein Wert" (null-echt / absent / pending); ein vierter fehlt —
  gemessen, aber das Signal trägt die Null-Schwelle nicht. Das
  TE-Verdikt „still" verschmilzt dort „ist null" und „unter Nachweis".
  Offen: die Kategorie benennen und das Verdikt prüfen, ob es die beiden
  auseinanderhält.

| Konzept | Stand | Urteil |
|---|---|---|
| WGSL_SHADER | Konzept | VERSIONIERT — die atmende Membran (σ-lerp, Hysterese, Interest-Map); die Zell-Achse ist der Enkel, der Vorfahr atmet stufenlos |
| 4D-MEMBRANE | ARCHIVED | WAHR — Trommelfell-Doktrin (keine Kamera, Manifestation real ohne Zuschauer); M01 referenziert sie |
| MINKOWSKI_FIELD-PERMEABILITY | ARCHIVED | WAHR — die EXPOSURE-PARABEL (Parabel des Sondierens, Wasser-Form, tanh-Rückkehr) = Ethik §9; VERSIONIERT unten |
| LOST_CONCEPTS | ARCHIVED | WAHR — das Verlust-Register des ersten Zeitalters (Minkowski, Topologie/TE, Permeabilität, Aperturen, Nostr, Überbau, ANISE, Tiles, WebGL2, Observer) — „await their return" |
| FUTURE_CONCEPTS | PLANNED | WAHR — Eis/Wasser/Dampf, Kohärenz-Integration, Retro-Manifestation, Mycelium-Web |
| REMOVE_BIAS | Plan | ERSETZT — ausgeführt (Surface-Frames, body_name, Station-materialize lebt im Code) |
| WETTERSTATION | Konzept | AUSSTEHEND — der 4-Token-HUD fehlt nativ; kommt mit der Messreihe |
| PARSER_MAGIC | DEPLOYED | WAHR — offen: cmap-Füllung, Auto-Frame, extent pro Force |
| PARSER_EVALUATION_MATRIX | SUPERSEDED | ERSETZT — SOURCES_V2_SPEC ist die kontrollierende Spec |
| SOURCES_V2_SPEC | LIVE | WAHR — die Spec, das τ-Gate, die Force-Gate-Prinzipien |
| SI_UNITS | SUPERSEDED | ERSETZT — SI-Konversion total (Option<f64> am Anker, unconverted = unmanifested + registriert); mag/Mw/dex/Crab/counts pending Kuration |
| IAU-2000_EOP | PARTIALLY DEPLOYED | WAHR — 72-B-Orientierungsmatrizen (Binary v2 trägt sie); die Erdrotation ist K06 (Archivar-Abschnitt) |
| SEARCH_COMMAND-PALETTE | PLANNED | AUSSTEHEND — ⌘K nie gebaut (M07) |
| KERNEL-CURATION-CI-AUTOMATION-PLAN | Plan | ERSETZT — K01 geschlossen (kernel_flatten.yml lebt) |

### Die Abweichungen (offen)

- Gravity-Hardcodes im Extract-Pfad (Z04/F35 — Ratsbefund): drei Stellen
  hartkodiert auf gravity statt aus den Daten — beim Vollzug
  verifizieren.
- Commit-Hygiene 925d93f (paper-reviewen, 2026-08-30): die Message nennt
  nur Patient-201/202-Fakt + sha256-Recompute, der Diff dreht aber
  planet-nine fam zurück (6.9082→7.2822, post→pre) samt n-Werte,
  5:2/5:3, 64er-Lücke, §2.1 n. Stand heute: der post-fix-Wert existiert
  nur in 66deafa..cd24606.
- Vokabular — Maschinenlage-Stempel (2026-08-30, Commit 2ee3c1f): das
  Feld `fam-machine` trägt die Lage der familien-Maschine für den
  gedruckten fam-Wert eines Blattes — `pre-fix` (Surrogat-RNG vor der
  Korrektur, die Bande liegt HÖHER) oder `post-fix` (nach der Korrektur,
  die Bande liegt tiefer). Das Feld `fam-round-machine` trägt die Lage
  für ein Blatt, dessen Verdict fam-governed ist, dessen Wert aber
  ungedruckt (Stille) bleibt: `pre-fix (verdict fam-governed, value
  unprinted)`. Konservative Fußnote: eine Stille/Verdikt gegen die
  höhere pre-fix-Bande gilt post-fix a fortiori; eine post-fix gemessene
  Stille gilt per Konstruktion. Gestempelt (docs/paper):
  `fam-machine: pre-fix` → corona-heating-ladder, gic-causal-driver,
  planet-nine-kbo-residue, big-bang-echo-sheet-12,
  probe-front-dark-matter, dark-flow-sheet-8;
  `fam-machine: post-fix` → solar-cycle-dynamo;
  `fam-round-machine: pre-fix (verdict fam-governed, value unprinted)` →
  flyby-path-1-cold-cases, signal-cone-audit-sheet. gic-causal-driver
  zusätzlich `re-run scheduled` (post-fix-Nachmessung gequeued).
  sha256 (Body ohne Header) aller 9 Blätter nachgezogen — die Fußnoten
  sind Teil des Bodys.
- Thermal/Diffusion (force 5/6) tragen zwei Gesetze: Kegel diffusiv
  √(2·D·age) (`membrane.rs:312–313`), Fold linear v = D
  (`membrane.rs:347–348`, `shaders.rs:11–12`) → der Fold fällt für 5/6
  praktisch aus. Physik-Frage: gewollt oder pending?

## Source-Port — der eine Pfad

Alle Source-Arbeit läuft über `docs/SOURCE_PORT.md`. Arbeitsfläche:
`phi/pipeline/` (queue/, park/, stage/, ledger.φ, prompt.φ). Bestand:
`phi/pipeline/catalog/`. Register: `phi/sources.φ` + `phi/dead_sources.φ`.
Der Sweep liest `phi/pipeline/stage/*_converted.φ`. Stale-Specs gebannert:
parser-evaluation-matrix.md + EXTRACT_TYPES.md (SUPERSEDED by
sources-v2-spec.md).

- Kompilat-Pfad in die Zustandsmaschine holen: der Weg tap_index →
  kernel_flatten.yml → tap_compiler → CDN → sources.φ läuft außerhalb der
  Zustandsmaschine (SOURCE_PORT §4) — kein ledger-Eintrag, kein
  Pfadkarten-Eintrag. Deshalb zerfleddert ein großer Katalog in Queue/
  Metadaten/Weights/Stage, ohne je aufgelöst zu werden. Vereinheitlichung:
  eine Kompilat-Stufe (`entdeckt → kompiliert → disponiert`) in ledger.φ +
  Pfadkarte; `disponiert` räumt die Discovery-Reste. Berührt SOURCE_PORT.md
  + ledger.φ + ggf. main.rs (--fish-Flag).

Offen (Detail in phi/pipeline/ledger.φ):

- Solar-Akteure-Folgen: der CDAWeb-Live-Block steht (SOLO_L2_RPW-TDS-
  SURV-STAT, SN_RMS_E V/m); der Publikations-Lag (~5 Monate, stopDate
  2026-03-25) lässt das {hour_ago}-Fenster heute leer (0 honored). Die
  2022-2026-Lücke ist GESCHLOSSEN (cdf_reader-Atom: LIRA-Ernte
  2022-11-25→2025-12-31). Wind/WAVES wav_h1: Ernte-Prototyp steht
  (wind_waves_compiler, Bin magic WAV1, 2021-01, 18848 Records) — volle
  Ernte 1994–2021 + Frame at wind = Folge-Atom. GONG L 31..200 + mparam
  (Eigenfrequenz/Linienbreite → freq/bin_width); GOLF-Zeitreihen (Medoc
  curl 000); der kernel_flatten sun job trägt gong/rpw-CI.
- Die Linse: Folgewelle — NASA-CMR-Keywords + GBIF-Tags downloaden,
  Library feinwägen; --port ersetzt --gold. Die Linse zieht die Buckets
  jetzt selbst (NOAA-NODD: sea-ice/cors/gnss=em, crowdsourced bathymetry
  =acoustic, Stationsklima ghcn/gsod/isd=thermal, GDP-Drifter +52).
- S3-Harvester: xml_harvester löst den ListBucketResult-Namespace nicht
  (0 records) — die NOAA-NODD-S3-Buckets (sea-ice/GDP-Drifter/cors/
  bathymetry) sind geparkt (ledger.φ parser-gap); braucht
  Namespace-Handling oder einen s3_harvester.
- Probe-Stufe: nächste Welle — neue Kandidaten aus den Katalogen in
  batches/ nachrücken.
- Queue: 10 Untested-Korpora (14k/13k/15k/7k/2k/183l/astro/earth/
  exotic/candidate-staging) — Port durch die Prozedur; astro-Korpus:
  28 Blöcke → manueller Port.
- Bestand: 38 offene VizieR-Bulks, IRSA/GAVO/ARI/ExoArchive-Inventare,
  GCNS/MWSC (Kompilat, liegen in GAVO dc.g-vo.org), 77 Archeology-Gaps,
  ESA-Kandidaten (Aeolus key-needed, SMOS parser-def), FRB-Union,
  Arena/Foundation/Research-Schatz im Archiv.
- Harvest: TAP-Indizes ESO (59) + CADC (21) + MAST (15) + Chandra (11)
  und ERDDAP BCO-DMO + NOAA/PMEL (je 1.000) geerntet — SI-Extraktion pro
  Tabelle/Dataset folgt. Dataverse geerntet (Harvard 88.741 + Borealis
  24.063; Linse 455/176 positive — Ozeanchemie, Thermal, CO₂, AOD =
  Probe-Kandidaten); UNC 401 auth-needed. OAI arXiv: 1.300 Records, dann
  Abbruch am skip-Timeout (oai_harvester ohne Timeout-Flag — fixe Länge
  nötig oder --set-Partitionierung). Parser-gap: MGDS (marine-geo.org
  liefert data_set-XML als Attribute, xml_harvester liest Kind-Elemente).
  Recherche: EPN-TAP-Endpoint (VESPA). Probe-Batch gebaut:
  queue/grind_dataverse.φ (136 Blöcke, Harvard 90 + Borealis 46,
  Gewicht ≥ 16, Dataverse-API je DOI live verifiziert) — nächste
  Probe-Welle.
- Grind-Einbau offen: 32 ArcGIS-Drafts (thermal/seismic/diffusion/em/
  advective/gravity); ARI GCNS (331.312 Sterne ≤100pc) + MWSC (3.006
  Haufen) als Kompilat-Kandidaten; 8 VirES-Drafts (CHAMP/GRACE/GOCE/
  CryoSat MAG/DNS/WND/TEC/KBR); archeology-gaps 77 Kandidaten (AERONET,
  IERS-EOP, Fireball/Sentry, Xamin-TAP, GONG2, GIRO-Ionosonde,
  e-CALLISTO …) als nächster Grind; FRB-Union-Merge mit
  TNS-Namens-Normalisierung (FRB121102 ↔ FRB20121102A) + frbcat.org-CSV
  als Quelle.
- Nachlauf: VirES-Vollprobe (64 Drafts, Datei ABSENT) + DONKI-Familie
  (CME-Draft, Datei ABSENT).
- Park: Pegelonline, USGS-Geomag, GWOSC/GraceDB (Skymap), DSN, CENC,
  JMA-Quake (cod-String), SDSS-SkyServer.
- Kraft-Abdeckung: acoustic/electric/thermal/advective/diffusion-
  Kuration offen — electric: GIC-Netze + Live-E-Feldstärke (kein Feed);
  GLM ist em (Ratsurteil); WWLLN radio-em vs. Entladung-electric bleibt
  Force-Gate-Frage.
- Die drei Ports der Nadeln — offene Reste: IONEX-GIM — der
  `format ionex`-Parser lebt, der Kanal ist AUSSTEHEND (CDDIS verlangt
  Earthdata-OAuth, GFZ/BKG/IGN-Routen 404/000); kein Block im Register,
  bis eine Route anonym lebt oder der Earthdata-Account existiert.
  WARTEND: SuperMAG (Positionen-Join + station-Filter server-blockiert —
  db-get-Fault, phi/-Zugang logon-only), Gaia DR4 (2.12.2026 —
  Recompiler der 44-Byte-Records).
- Teleskop-Inventar (ledger.φ geparkt): GCN-API v0.1 tot (SVOM-Block
  nicht baubar); NRAO = Angular-SPA (kein REST); CHIME = CANFAR-DOIs
  statt API; svom.ac.cn = HTML, Zertifikat abgelaufen; ESA-AMA-TAP-Basis
  ungefunden; Keck/KOA unkuratiert; eROSITA DR2 = HTML-Landing; MAGIC/
  HAWC = HTML+FITS-Portale, TLS-Kette unvollständig; LHAASO = News-Seite
  2021 → Decline. IRSA spherex.obscore = VOTableJSON-Atom; Euclid
  mer_catalogue = SpaltenAusMetadata-Atom; ESO tap_obs = echte CSV, aber
  probe_csv klassifiziert Header-CSV nicht; Pan-STARRS dr1 mean =
  Endpoint lebt, Probe-Env kennt {ra}/{dec}/{radius} nicht (Nachweis im
  Register-Lauf offen). Befunde:
  phi/pipeline/research/agent_output/verify_astro{,_b}_2026-08-19.φ.
- Sensor-Kategorien-Welle (2026-08-19): 10 Agenten (Satelliten,
  Flugzeuge, Drohnen, Raumstationen, Radiosonden, Bojen, Wetterstationen,
  Labore, Unterwasser, Sonstiges) + Jina/Wayback-Nachprüfung (Taxonomie
  tot/declined/blocked/live/angekündigt; Agenten-Rezept in SOURCE_PORT
  §13). 18 live-Kandidaten geparkt (ledger.φ — Port ausstehend: AMeDAS,
  ECCC GeoMet, BfS-ODL, GTMBA, EMODnet, EMSO, IOOS-Glider, SmartBay,
  USGS-Grundwasser, NRCS-AWDB, IGRA, Wyoming, Iowa-RAOB, SondeHub,
  AWC-PIREP, COSMIC-2, IMO, GeoNet, meteo.lt); 14 blocked
  (blocked_sources.φ — key-needed; 3 ip-blocked lokal nachprüfen:
  Meteomatics, CelesTrak, MeteoSwiss-Pollen); 13 angekündigt (MTG-I2
  27.08.2026, MetOp-SG B1, Sentinel-3C, C-130J, NASA-777, Axiom, Orbital
  Reef, Starlab, SOFF, ITER, SPARC, DUNE, EMSO-SMART-Cable). Befunde:
  phi/pipeline/research/agent_output/{satellites,aircraft,drones,
  space_stations,radiosondes,buoys,weather_stations,laboratories,
  underwater,misc}_2026-08-19.φ + classify_2026-08-19.φ.
- Parser & Spec: VOTableJSON (ausstehend, ledger.φ) — IRSA-TAP liefert
  VOTable-serialisiertes JSON (s_ra/s_dec nur als FIELD-Metadaten);
  SpaltenAusMetadata (ausstehend, ledger.φ) — Euclid/EAS-TAP antwortet
  {metadata:[{name:…}], data:[[…]]}; Hapi-FieldConfig — die
  deklarierten kernel/force/tau der HAPI-Blöcke erreichen den
  Oszillator nicht (synthetisch {0,0,0}).
- Host-Kuration offen: CENC (Keyed Object No1..NoN), JMA-Quake
  (Position im cod-String), Pegelonline (Fanout-Block steht aus — P09),
  GWOSC/GraceDB (Position nur via Skymap), DSN (statische
  Dish-Positionen), USGS-Geomag (Komponenten-Timeseries).
- Enrichment offen: Name-basierter Ersatz-Join.
- Vorräte (Pfade unter archive-root/archeology/):
  sources/sources_gold_pre-cdn_27k (2572 Blöcke) +
  sources_recovery_pre-cdn_25k (1924) — Migration nach Protokoll
  (docs/SOURCE_PORT.md); sources_new_untested_14k (873) +
  sources_astro_untested (30) + sources_exotic_untested (16) +
  sources_earth_untested (3) — UNTESTED_index.txt nicht archiviert,
  per-Domain-Index rekonstruieren; sources_recovery_cdn-merged_60k
  lost-blocks (5701 urls, 0 field-Tokens) — Extract-Parameter aus
  history/recovery zuordnen; arena/ (batch_01–21, ungeprüft);
  foundation/ (APIs/collection/gaps).
- Port-Migration ohne τ (pending): die pre-cdn-Grammatik trägt kein
  τ-Token — port_field_synth verweigert Felder ohne kuratiertes τ;
  felderlose Konvertate werden nicht übernommen (flush_port_block).
  Die Alt-Blöcke (phi/pipeline/research/batches/ 283 +
  probe_batches/ 242) bleiben unkonvertiert-pending, bis τ je Feld
  kuratiert ist (Register: phi/pipeline/queue/).
- Zwei Bestands-Blöcke in phi/sources.φ deklarieren `on earth 52.5 13.4`
  ohne alt — seit S2 refused; alt deklarieren oder die Blöcke bleiben
  dunkel.
- Fanout-Stationen ohne Höhe (stations_lat/lon ohne stations_alt-
  Direktive): alt-Slot 0.0 = fehlende Messung bis die v3-Maske das Bit
  trägt; eine `stations_alt`-Direktive steht aus.
- mpcobs: das Bin hat keinen Konsumenten im Archivar (Integration
  pending) — der 0.0-Slot bleibt Wire-Pad bis die Konsum-Kette existiert;
  die Autorität liegt dann beim Konsumenten: `mag > 0.0`-Gate (blank →
  kein Messwert), die Vega-Kollision (mag=0 ist ein physikalischer Wert)
  ist benannt (D1-Verdict).
- v8-Präsenz-Maske: der color_index-Slot bleibt bis v8 das
  0.0=absent-Wire-Pad (Weiß); BP−RP=0 (A0V) kollidiert — die v8-Maske
  (Rats-Urteil-1-Muster) trägt den Farb-Slot als Bit (D2-Verdict).
- INTERMAGNET-Fanout (154 Observatorien live): der Fanout trägt über
  `stations GetCapabilities` alle 154 Observatorien — `fanout 154`
  (2026-08-21), der Auroral-Ring (|lat| ≥ 58°, 38 Observatorien) ist
  damit eingeschlossen (das Berlin-Zentrum ordnet nur, schneidet nicht
  mehr ab); ABK erscheint doppelt (fester Block = Probe-Anker). Kosten
  benannt: 154 Requests je Refresh (fanout_delay 15 s → ~13 min, TTL
  86400). Status-Matrix gemessen (GIN-V1-Katalog, 3074 Datensätze):
  je Station `definitive`/`quasi-def`/`reported`/`adjusted`/`best-avail`
  × PT1M/PT1S × native/xyzf/hdzf/diff. `best-avail` ist der Status-Stapel
  — definitiv →2021-12-31, quasi-def 2012→~1 Monat zurück, reported/
  adjusted der letzte Monat; `quasi-def` bis ~1 Monat zurück (P366D max
  je Request). Die Retro-Blatt-Zeile läuft über `best-avail` mit
  benannten Status-Grenzen ODER `quasi-def` (2012→, monatsverzögert) —
  Jahres-Schleife im Retro-Atom.
- Struktur-Reader: netCDF-3 (CDF-1 + CDF-2) lebt; CDF-5 bleibt pending
  (eigener Atom); offen: FITS-Binärtabellen, Parquet/Arrow, netCDF-4/
  HDF5, OPeNDAP, CDF, GRIB-2, GeoParquet, OGC-SensorThings.
- Katalog-Lücken (genuin, verifiziert gegen alle drei Register):
  Photometrie/Spektroskopie — RAVE DR6, APOGEE/GALAH; Extragalaktisch —
  HyperLEDA/PGC; Radio-Kontinuum (Achse leer) — TGSS ADR, SUMSS, RACS,
  LoTSS, VLASS; High-Energy — AMS-02; Sonnensystem — PDS
  (Instrumentendaten), MPC-Live (mpcorb_extended.json.gz); TAP-Indexe —
  ESASky, NOIRLab Data Lab, NED; Terrestrisch — EarthScope-FDSN, EPOS,
  SeaDataNet, Smithsonian GVP, Natural Earth. Exakte Tabellen-IDs +
  Spalten + Mechanismus:
  archive-root/handover/handover-2026-08-20-fischplan-kataloge.md
  + handover-2026-08-20-chunk-kataloge.md (archiviert).
  GLADE+ ist pending: Spalten live verifiziert, drei gemessene Blocker —
  Schrittboden-Kappung des --mag-bands-Banders, 2-GB-Release-Limit,
  MAX_SAMPLES 4.19 M (chunk-plan, archiviert).
- Der Katalog-Chunks-Ernte-Auftrag (2026-08-23): NED-Chunks — der
  objid-Walk läuft auf die volle Tabelle (Status LÄUFT — der ned.json-
  Upload steht am Ende des Walks, ~100+ h über Resume-Läufe; der Block
  wartet leer in sources.φ). Der ned_chunks-Job (kernel_flatten.yml)
  erntet den vollen Walk (objid 1…1,12 Mrd., ~100+ h über Resume-Läufe,
  Cache ned-chunks-v2) und trägt je Teil den Zellen-Merge (sort -m, vier
  Plätze je Zelle, objid-Dedup, Ties deterministisch über objid); das
  Budget ist durch Konstruktion gebunden (1024²×4 = 2²² = MAX_SAMPLES),
  das Gate verifiziert und bleibt laut; der ned.json-Block lebt in
  sources.φ und wartet leer auf den Upload am Ende des Walks
  (spectra.bin-Muster). Benannt: die 1024²-Zellen sind equirectangular —
  die Polzellen sind kleiner (die benannte Anisotropie); die 4 Plätze je
  Zelle füllt das rank-Turnier, nie ein Band-Schnitt. NED trägt nur
  Position + z — das Skelett; Ⅷ/Ⅻ brauchen CosmicFlows/Planck-CMB als
  Kanäle.
- Katalog-CDN-Ernte NVSS/FIRST/Chandra: NVSS und FIRST tragen die
  Distanz als spektroskopische Rotverschiebung (`z z`, SDSS DR16
  `V/154/sdss16` Spalte `zsp` per CONTAINS-Crossmatch), Chandra bleibt
  `z z`. Der Umgehungspfad liegt: `tap_compiler --band <spalte> <lo>
  <hi> <schritt>` (feste Bänder über den lebenden Sync-Endpoint, kein
  COUNT-Split) erntet bandweise mit `--limit`. NVSS/FIRST laufen als
  Hintergrund-Ernte (Route 1, NVSS `--xmatch-radius 10`, FIRST 1,5″) —
  die zwei Assets sind noch 404, bis der Lauf fertig ist und hochlädt.
  Ein leerer Trefferstand ist 0 honored, kein Erfolg.
- 2MASS: der helle Schnitt (CI: jmag 11.0) entscheidet der Operator; der
  CI-Job twomass_bulk (kernel_flatten.yml) erntet einmal (Asset-Existenz
  = Komplett-Check) und lädt twomass_psc.bin (Magie "2MPS", 64-B-Records
  [ra, dec, J/H/K + msigcom-Fehler], 0.0 = absent) — der Zählstand über
  MAX_SAMPLES verweigert den Upload. Der Feld-Lader (format
  catalog_twomass) fehlt bewusst: distanzlose Zeilen fallen (Titan-Skip)
  — das J−K-Farb-Atom trägt Crossmatch + Lader. 2MASS J−K
  (twomass_psc.bin) bleibt ein separater, offener Farbindex
  (Gaia-Crossmatch + Feld-Lader).
- VizieR-async-Befund: --async + gaiadr3-JOIN hängt PENDING — UWS-Jobs
  sind IP-gebunden: stirbt der Runner, verwaist der Job. RA-Slices
  sind der Weg für Crossmatch-Kompilate.
- ω-Loop-Fetch-Sturm (Befund 2026-08-21): der Live-Source-Zyklus fischt
  ~200 Quellen kontinuierlich mit 4 Retries × 23 s und ttl/Φ-Backoff —
  ein unbegrenzter Churn, der die Heimleitung bei jedem Membran-Lauf
  sättigt. Budget-Messungen brauchen einen begrenzten/drosselbaren Lauf
  statt des vollen Membran-Churns — der Sturm selbst ist ein eigener
  Reparatur-Gegenstand (Retry-Exponent, Pausen pro Quelle).
- Sample-Budget des Feldes (2026-09-03, verengt): die statische/
  temporale Vermengung ist geschlossen — der Ring trennt seither die
  Domänen nach SampleSource (temporal_ring in membrane.rs: nur der
  temporale Fluss wird epoch-absteigend auf den Rest der Kappe getrimmt;
  der statische Katalog — Sterne/Asteroiden/Anker — ist dem temporalen
  Überlauf strukturell entzogen, eine statische Über-Kappe meldet eine
  Register-Pflicht, nie ein stilles Abschneiden). Offen bleibt die
  statische Zulassungs-Pforte für den vollen api-Stand: die Summe der
  Katalog-Blöcke (Sterne 1.19 M + Asteroiden 1.56 M + NVSS 1.8 M +
  FIRST 1.1 M + Chandra 0.4 M + vier Chunks 1.4 M + …) liegt über
  MAX_SAMPLES (1<<22) — heute unerreicht (Katalog-Kompilation
  unvollständig), die Maschinenzeile trägt den Aufschluss, sobald ein Lauf
  den vollen api-Stand erreicht.
- Chandra-Drift benannt: der Block trägt erg/cm2, CSC-Fluxb ist
  physikalisch erg/cm²/s — gehört zum Unit-Arm, Block-Label prüfen.
- Katalog-Lücken Welle II: Diffusion/Chemorezeption unbesetzt — TCCON
  (verifiziert, tccondata.org, Registrierung); pending Verifikation:
  AGAGE, NDACC, WDCGG, GLODAP, EBAS. electric: WWLLN
  (registriert/restringiert) — Force-Gate klären, sonst refused. em
  terrestrisch: NSRDB/BSRN (Bodensolar fehlt) — NSRDB pending. gravity:
  BGI/GGP-Bodengravimetrie (IGETS nur indexiert) — pending Verifikation.
- Katalog-Lücken Welle III (genuin): electric — AMPERE, GloCAEM,
  USArray-MT; diffusion — EMEP/CCC, WDCRG, European Waterbase; em —
  NEUBrew (UV), THEMIS/ASI (Polarlicht, CDF), COSMOS2025/COSMOS-Web,
  INTEGRAL, ATLAS-RefCat2, Subaru HSC-SSP, TIC; kosmisch/Neutrino —
  CREDO, KM3NeT; Geodäsie — ILRS, IVS-EOP, DORIS-Live, GRACE-FO-
  Mascons (L2/L3); Atmosphäre/Ozean — E-GVAP, Wyoming-Soundings,
  BGC-Argo-live, IOOS-HFRNet, NOAA-NRS (Ozean-Lärm), MIROVA.
  Zugriffsarten unverified.
- Crossmatch indexiert → live heben: GALEX-GUVcat (UV), SkyMapper DR4,
  UKIDSS/VISTA/VIKING (NIR), DES DR2/Legacy Surveys DR10.
- Zeitkritisch: Gaia DR4 (2. Dez 2026) — dr4_stars.bin + DR4-Schema im
  tap_compiler (5,5 a, halbierte Parallaxenfehler, Gaia-Exoplaneten);
  Rubin LSST DR1 (Ende Juni 2028), Alerts live (Broker declined);
  GCVS-Stand prüfen (HEASARC-Update Juni 2026 vs. gcvs_cat.json);
  Euclid DR1 (Okt 2026); SDSS-V; eROSITA-DR2 (Juli 2026 erschienen —
  prüfen ob via HEASARC-tap_index erreichbar); SPHEREx (IRSA VOAPI +
  AWS S3 + FITS, Quick-Release live, Voll-Katalog 2026 — verifiziert);
  DESI DR1 (NOIRLab Astro Data Lab TAP, ~18 Mio Spektren —
  verifiziert); Roman (2027), 4MOST/WEAVE (2026) — unverified.
- ESA/Geomagnetik: Swarm TCT-E-Feld (keyless), VirES-Aeolus, SMOS,
  MERIS/SAR/Landsat Kandidaten.

## Curation & Quellen

- Pending Unit-Arme: F (Fahrenheit, CHPL-Lufttemperatur), μg/L
  (Chlorophyll, CREST-Boje), mg/L (Sauerstoff, CREST-Boje) — die Felder
  existieren in den Quellen, manifestieren erst mit dem convert_to_si-Arm.
- HorizonsVec-Fetch: `{jd_now}`/`{jd_start}`/`{jd_end}` in render_url
  (TDB, 6 Stellen) lebt. Ein Live-`vectors`-Block in sources.φ bleibt
  Kurationsfrage: dead_sources.φ:3090 deklariert Horizons als
  Compiler-Eingang, keine Live-Quelle.
- mpcobs / mpcorb_extended.json.gz: offener Live-Block (Sonnensystem).
- reverify-Quellen-Drift kuriert (2026-09-03, a3a2595+a): (a) NDBC-hist
  auf Direktpfad `ndbc.noaa.gov/data/historical/stdmet/<st>h{prev_year}.txt.gz`
  migriert (34 Blöcke, curl-verifiziert; 46005/46012 fest auf letztes Jahr
  h2024/h2023 weil das Archiv nachhinkt, 15006-Block entfernt — kein
  historisches stdmet-Archiv, lebt nur als realtime2). (b) kp.gfz auf
  `kp.gfz.de` + nur `index=Kp` migriert (der 500 kam vom Multi-Index
  `Kp,ap,Cp`, nicht vom Server; der Block liest ohnehin nur `last Kp`).
  (c) nmdb `qtipart.php` echt 404 → Block auf `r.jina.ai/…/nest/draw_graph.php`
  mit `dtype=uncorrected` (Rohzählrate) umgezogen, Format text/rows wie
  Schwester-Block 969. (d) argovis-api.colorado.edu gemessen LIVE (200) —
  die "unerreichbar"-Behauptung war ein Datacenter-/Query-Fehlalarm, kein
  Drift; Block unverändert. Register-Pflicht Rest: sobald NDBC das
  h2025(46005)/h2024+(46012)-Jahresfile nachliefert, die festen Jahre wieder
  auf `{prev_year}` heben.

## Validation

- `--verify` CLI existiert (URL-Erreichbarkeit); lädt noch keine Quellen.
- Test-Limit der Curation über 200 Blöcke hinaus erhöhen; 6 Rest-FAILs
  sind Daten-Artefakte (docs/SOURCE_PORT.md §5).
- VirES-Vollprobe: Ergebnis-Datei ABSENT (Schreibverlust) — Nachlauf in
  Blöcken offen.
- DONKI-Familie: Ergebnis-Datei ABSENT — Nachlauf in einem Block offen.
- MSL/MEDA-field-Pfade end-to-end verifizieren (test_live_sources_extract
  deckt nur die ersten 200 Blöcke).
- Firefox-Laufzeit-Verifikation offen (BiDi-Weg: user.js mit
  dom.webgpu.enabled + devtools-Prefs, WS auf /session).
- AGOS-Quarantäne: Katalog endet 2022-02-05 — Kompilat-Kandidat über den
  CDN-Weg.
- EA-Fanout: Runtime-Fanout-Lauf offen (Test überspringt Fanout
  designbedingt).
- Der lebende Wacht-Kanal für die Leap-Sekunden ist
  `datacenter.iers.org/data/latestVersion/bulletinC.txt` (200; die
  registrierte Route 16_LEAP_SECONDS.txt ist 404) — zu registrieren.

## CI Pipeline

- I02-Rest: das Python refresh.yml im sources-Repo bleibt auf Python —
  Abschaltung nach Verifikation der Rust-Katalog-Kompilate im
  kernel_flatten-catalogs-Job (ein Produzent pro Asset). In diesem Repo
  trägt health-check.yml die Rolle (cargo run -- --verify phi, 3-h-Cron,
  Anomalie-Issues).
- Token-Rotation: der git-Remote-Token (keine releases/actions-Rechte)
  gehört rotiert und auf credential-helper/SSH umgestellt.
- Stray-/Basename-Assets im Release ssd.jpl.nasa.gov löschen.
- CI: Compiler-Builds zahlen den wgpu-Compile mit (harte Dependency).
- CI-Chunk-Kompilation der großen Kataloge: der chunk_catalogs-Job
  (kernel_flatten.yml) verdrahtet RAVE/pastel/wds/mktypes/denis als
  Bash-RA-Slices (CI-Replikat von phi/pipeline/chunk_master.py, ohne
  Python). Offen: ein voller grüner Lauf (Verifikation des
  Slice-Schritts) + die MAX_SAMPLES-Budget-Messung. GLADE+ bleibt draußen
  (drei gemessene Blocker, s. Katalog-Lücken).
- CDN-Asset-Naming: `{name}.json` — Konvention ist der Resolver (Regel).

## Verteilung

Die Binaries liegen in GitHub Releases (omegaflow/omegaflow) — Tag =
Identität, `SHA256SUMS.txt` je Release, Rollback = älterer Tag. Pages
(omegaflow.space) trägt nur die Landing; die Binaries verlinkt auf
`releases/latest/download/<asset>`. Atom 1 (Release-Kanal: release.yml +
entschlacktes pages.yml), Atom 2 (Φ-Paket aus allen CDN-Netlocs statt
0-Byte-Lüge) und Atom 3 (Plattform-Wahrheit: userAgentData statt
UA-Selbstbericht, Termux-Bootstrap ersetzt, Unsigned-Status benannt)
sind gebaut — die Verifikation trägt der nächste Release-Lauf.

## Ausstehende Build- und Verteil-Pflichten

- Temporal Topology (TDA, Takens, Transfer Entropy, Surrogates) —
  ausstehend, lost-concepts.md.
- Kraft-Separation (7 omegas statt „one law, five media") — ausstehend,
  LOST_CONCEPTS §13.
- Verzögerungsspektrum / Lichtkegel-Differenz / Stillekarte /
  Synthetischer Flug — ausstehend, der-paradigmenwechsel.md,
  LOST_CONCEPTS §14–17.
- Field Permeability (tanh(vC/g)-Variante ohne TE) — ausstehend,
  minkowski-field-permeability.md.
- Minkowski 4D Weighting (spacelike→0; kosmisches Skalenproblem: Sonne
  wäre spacelike — scale-Anpassung nötig) — ausstehend,
  minkowski-field-permeability.md.
- Auto-Zoom (median-extent/p90): die atmende Membran ist der stärkere
  Vorfahr; die Fenster-Reduktion als Budget-EMA-HUD-Messung — der
  Operator entscheidet.
- Council-Forschungs-Iterationen: Archivar als „langsamer Prior" für den
  Exposure-Kaltstart (aktuell: fixe Rampe); Exposure-EMA auf dem
  Silizium (gegenstandslos solange die Rampe fix ist) — AUSSTEHEND.
- Future: Aggregation of Presence, Retro-Manifestation, Total Coherence
  Integration, Nostr-Stationsweb — AUSSTEHEND, future-concepts.md.
- Binary-Signing (Apple Developer + MS-Zertifikate) — AUSSTEHEND, braucht
  Konten.
- musl-static Linux-Build (kein glibc-Zwang) — AUSSTEHEND.
- Installer (.deb/.rpm/AppImage/.dmg/.msi) — AUSSTEHEND.
- crates.io (`cargo install omegaflow`) — AUSSTEHEND; mit PolyForm-
  Noncommercial als source-available markiert, nicht Open Source.

- JWST-Biosignatur-Kette: jwst_spectra.bin ist SEIT 2026-09-04 auf dem CDN
  manifestiert (HTTP 200, 165824 B, 6 GJ-806-Spektren, 3×1915 +
  375/378/379 Bins — erster wahrhafter Stand; zuvor war das Register-
  Asset 404, nie manifestiert). RESTLICH AUSSTEHEND: (a) jwst_equilibrium.bin
  (Gleichgewichts-Chemie, thermochem) — Compiler jwst_equilibrium_compiler
  lebt; (b) der Scanner jwst_biosignature_scanner (Auftrag 4) wartet auf
  beide; (c) die VOLLE Spektren-Ernte konvergiert nicht — der jwst-Job
  (kernel-flatten.yml) erntete in 5,7 h CI 0 Spektren (0 new, 1502 named
  skips, Budget-Abbruch, kein Finalize; nur Index ~292/3760). Seit
  2026-09-03 überspringt jwst_spectra_compiler proprietäre (EXCLUSIVE_
  ACCESS/PROPRIETARY) MAST-Beobachtungen; CI-Pfad korrigiert (LSK von NAIF
  nach kernels/naif0012.tls statt des nicht existenten src/kernels/…).
  Strategie-Umbau offen: kuratierte AST-Liste der Transmissions-Spektren
  statt aller 3760 Transit-Hosts.

- JWST-Spektren-Strategie-Umbau (2026-09-04): der Compiler erntet jetzt
  per `--curated` die NExScI-`spectra`-Tabelle (JWST-Transmission,
  NIRSpec/NIRISS/MIRI, Komma-Join an ps) = 48 Ziele statt aller 3760
  Transit-Hosts (WASP-15 erntet nachweislich 9834 Bins). Bei Budget-Abbruch
  finalisiert/uploadet er nun den Teilstand statt ihn zu verwerfen (jede
  CI-Ernte bringt den wachsenden Bin aufs CDN). Verifiziert: NExScI-Oracle-
  TAP verweigert `JOIN…AS…ON` (ORA-00933) — Komma-Join nötig. Restrisiko:
  Budget wird nur zwischen Beobachtungen geprüft; ein einzelner grosser
  Download kann ein Budget-Fenster überziehen. Offen: Voll-Ernte der 48 aufs
  CDN, dann jwst_equilibrium + Biosignatur-Scan (Auftrag 4).

- Register (Operator-Entscheid 2026-09-04, Versionierung statt Ersatz):
  (1) Cl-Ernte kuratiert (48 Objekte), dispatched als `curated48_spectra.bin` —
  aktive Biosignatur-Quelle. (2) GJ-806-Asset (`jwst_spectra.bin`) bleibt
  unangetastet als Fehlmatch-Beleg des automatischen Matchers (NExScI-spectra-
  Tabelle: 0 Zeilen für GJ 806 → kein Transmissions-Spektrum); negatives
  Kalibrationsbeispiel für künftige Matches; superseded als Quelle, nie
  ge-clobbered. (3) Lehre: ein Fehlmatch ist ein Befund über den Matcher, nicht
  über die Daten — Assets, die Registerzeilen tragen, werden versioniert, nie
  überschrieben. Downstream (Scanner) zeigt auf das kuratierte Asset.

- WURZEL des CI-0-Spektren-Bugs (2026-09-04, Diagnose-Workflow mast-diag):
  der Compiler lud die FITS hartkodiert nach `/tmp/opencode/jwst_….fits` —
  das Verzeichnis existiert lokal (angelegt), aber NICHT im CI-Runner →
  `curl -o` schlug fehl → jede Ernte "no readable spectrum product", 0
  Sidecars in allen CI-Läufen. mast-diag isolierte: Token ok (set), CAOM 200,
  Download 200 (319 MB WASP-15 in 6 s) — Netzwerk+Token gesund. Fix:
  Downloads gehen in `workdir/tmp` (per create_dir_all angelegt). Diagnose-
  Workflow nach Bestätigung entfernt.

- Gleichgewichts-Ernte (2026-09-05): jwst_equilibrium.bin aus den kuratierten
  Spektren (curated48_spectra.bin, 214 spectra) kompiliert — 90 Records,
  21020 B, Roundtrip ok (113 Multi-Planeten-Hosts übersprungen, 11 out of
  domain) — auf dem CDN manifestiert (HTTP 200, ssd.jpl.nasa.gov/
  jwst_equilibrium.bin). sources.φ: jwst_spectra-URL zeigt jetzt auf das
  aktive kuratierte Asset curated48_spectra.bin (GJ-806 als Fehlmatch-Beleg
  archiviert). OFFEN: der Biosignatur-Scanner (Auftrag 4) liest beide Assets;
  KDE-h/Multi-Force-Pflichten vor einem Befund.

- Biosignatur-Diagnose-Pass (2026-09-05, Rat-Entscheid, kein TE-Verdikt): der
  Scanner trägt in dieser Form KEINE Aussage — vier strukturelle Defekte, die
  eine Stille fabrizieren. (a) fam-Bug: `fam = max(best_te, rev_te)` ist das
  Maximum der beobachteten TE, forward == fam → `forward > fam` unerreichbar,
  "fam-tragend" toter Code, Katalog-fam immer 0, die alte "no causal filter
  breaks the field — silence"-Zeile war ein Code-Artefakt, kein Befund.
  Gemessen: 214 Spektren / 90 Equilibria, 90 Atmosphären ausgewertet, mean TE
  8.1e-2, fam 0.0000. (b) Paar nicht ko-indiziert: x = Flux über Wellenlänge,
  y = Spezies-Brüche über Spezies-Index — keine gemeinsame Achse; lag ≥ 1
  verschiebt entlang der Wellenlänge = Artefakt, kein Lauf. (c) y ist das
  thermochemische Gleichgewicht (reine Funktion von Teq) = der leblose Nullfall
  per Konstruktion — "Leben bricht das Feld" ist mit dem Gleichgewichts-Asset
  strukturell nicht prüfbar (Disequilibrium nicht erzeugbar). (d) n = 16
  Spezies < 30 (Mess-Gate); KDE-h nicht offengelegt (Silverman intern), h-
  Sensitivität bei n < 30 stumm. Scanner umgestellt auf Diagnose (keine
  fabrizierte Kausal-Aussage); Verdict: keine Aussage / pending, nicht
  "silence = no life". Register-Pflichten: bedingte Multi-Force-TE (Phasenraum)
  bleibt pending (kein Instrument, nicht stillschweigend auslassen); Skip-
  Taxonomie (113 multi-planet + 11 out-of-domain) = absent im Equilibrium-
  Asset, korrekt None-geskippt, hier benannt (0 honored). Nächster echter
  Schritt wäre ein ko-indiziertes Disequilibrium-Design (Messwert der
  Atmosphäre), nicht ein Codepatch an diesem Scanner.

- JWST-Detektions-Register (ausstehend — die Ernte, nicht das Instrument): das
  ko-indizierte Disequilibrium-Signal braucht die publizierten Detektions-
  Fakten der 48 Ziele (Spezies, Abundanz, S/N, bibcode) — die liegen in den
  Retrieval-Papieren, nicht in einer TAP-Tabelle. jwst_spectra_compiler wählt
  aus der NExScI-`spectra`-Tabelle heute nur hostname/ra/dec; bibcode +
  wl_min/wl_max werden fallen gelassen. Erster Schritt: Compiler um bibcode +
  Wellenlängen-Abdeckung erweitern, dann je Papier die Detektions-Tabelle
  ernten (Saat: Madhusudhan 2019, ARA&A 57:617). Verdict bleibt pending bis
  das Register steht.
- jwst_detection_registry_compiler (Bauwerk): erzeugt jwst_detection_registry.json
  aus bibcode-Liste + kuratierter Saat; --ci-mode manifestiert das Register
  aufs CDN. Der Biosignatur-Befund ist kein TE-Verdikt; der gebrochene
  fam-Bug bleibt Diagnose.
- disequilibrium_register_probe (ausstehend — wartet auf das Register): liest
  {host, Spezies, Abundanz, S/N, bibcode}, rechnet je Planet die
  Gleichgewichts-Menge bei Teq (thermochem::equilibrium_composition), Urteil
  je Planet: detektiert-ungleichgewichtig / gleichgewichts-präsent / absent.
  Null: Permutations-Familie über den Katalog (Trefferzahl gegen mean+2σ der
  Shuffle-Verteilung) — die ehrliche Fassung der gebrochenen fam-Schranke.


- JWST-Detektions-Saat (2026-09-05, kuratiert): 73 publizierte Spezies-Detektionen,
  30 der 48 kuratierten Ziele tragen >=1 bestaetigte Detektion — Saat unter
  docs/reference/jwst_detection_seed.json (jede Zeile auf das Primaer-Paper
  attribuiert; pre-JWST aus Madhusudhan 2019 Table 1, 7 Zeilen/6 Hosts). Absent
  benannt (GJ 1132, TRAPPIST-1 b/c/d/e, LHS 1140 b u.a. — featureless/strict
  upper limits, keine fabrizierte Detektion). Needs-review: GJ 486 b + GJ 1214 b
  (Wasser-Atmosphaere ODER Stern-Kontamination, mehrdeutig), K2-18 DMS/DMDS
  (degeneriert), L 98-59 d (hints), TOI-270 b (inklusiv). WASP-39 2022arXiv und
  WASP-17 2023ApJ...956L..29G nicht ADS-aufloesbar (bzw. Mid-IR-Quarz, keine
  Gasspezies). jwst_detection_registry_compiler --seed lädt die Saat
  (seed_state loaded). OFFEN: disequilibrium_register_probe (wartet nun auf
  Daten — ist geliefert); die registry braucht einen Manifestations-Heimatort.


- Disequilibrium-Register-Urteil (2026-09-05, disequilibrium_register_probe gebaut):
  10 disequilibrium-Hit-Hosts (alle CO2-auf-heissem-Planet unter Floor 1e-6),
  8 equilibrium-praesent, 12 pending (11 Multi-Transit-Hosts nicht attribuierbar +
  K2-18 unter Modell-Domain 500 K). Katalog-Permutations-Null (10000 Zuege, fixer
  Seed, voller Kreis): beobachtet 10 | Null mean 9.61 | sigma 1.58 | mean+2sigma
  12.77 | P(T>=10)=0.529 — die Hit-Zahl liegt unter der Zufalls-Schwelle, KEIN
  Disequilibrium-Signal im Katalog ueber den Zufall hinaus. Ehrliche Abdeckung:
  das thermochem-Modell ist S-frei (H/C/N/O) — die kanonischen Schwefel-Signale
  (WASP-39b/WASP-107b SO2, HAT-P-26 SO2, TOI-5205 H2S, V1298 Tau OCS) sind
  out-of-model, nicht als Hit/Nicht-Hit klassifiziert. Floor (--floor, default
  1e-6) ist ein benannter Judgment-Wert (Seed traegt keinen Instrument-Floor);
  Sensitivitaet: 1e-7 -> 3 Hit-Hosts, 1e-6/1e-5/1e-4 -> 10.
- Disequilibrium-Register-Urteil II (2026-09-05, sulfur-aware + Planet-Attribution):
  thermochem traegt jetzt einen separaten S-Pfad (equilibrium_composition_sulfur,
  24 Slots: 16 archivierte H/C/N/O-Slots unveraendert + S,S2,SH,H2S,SO,SO2,CS,OCS;
  Daten NIST-JANAF Chase 1998 Shomate-Fits, S(g) nur ab 882 K Fit-Domäne,
  S/H 1.62e-5 Anders-Grevesse 1989; 16-Slot-Vertrag jwst_equilibrium.bin bleibt
  unangetastet — erweiterter Pfad, nicht mutierter Bestand). Saat traegt jetzt
  pl_name je Zeile: alle 11 Multi-Transit-Detektionen dem im Primaer-Paper
  benannten Planeten zugeordnet (GJ 9827 d, HIP 67522 b, L 98-59 b, LP 791-18 c,
  LTT 3780 c = TOI-732 c, TOI-1130 b, TOI-199 b, TOI-270 d, TOI-421 b,
  V1298 Tau b, GJ 3090 b). Gemessen: 14 disequilibrium-hit, 10
  equilibrium-present, 1 ohne-Modell-Daten-only (GJ 3090 b, He), 5 pending —
  alle Domäne (K2-18 b 283 K, LP 791-18 c 354 K, LTT 3780 c 363 K, TOI-199 b
  352 K, TOI-270 d 388 K unter der 500-K-Modellgrenze: benannter Modell-Domänen-
  limit, kein Signal; CH4/CO2 auf K2-18 damit nicht als Gleichgewichts-Urteil
  wertbar). Schwefel-Kanal: SO2 auf WASP-39 b (Teq 1166 K, Gleichgewichts-Anteil
  1.6e-16), WASP-107 b (4.9e-22), HAT-P-26 b, TOI-1130 b, WASP-15 b, V1298 Tau b,
  L 98-59 b und OCS auf V1298 Tau b sind disequilibrium-hit (Detektion um >10
  Groessenordnungen ueber dem solaren 1-bar-Gleichgewicht); H2S auf TOI-5205 b
  ist equilibrium-present (Gleichgewichts-Anteil 3.24e-5 = der S-Reservoir bei
  736 K). Katalog-Permutations-Null: beobachtet 14 | Null mean 15.75 | sigma 1.68
  | mean+2sigma 19.10 | P(T>=14)=0.911 — kein Disequilibrium-Signal ueber den
  Zufall hinaus; Sensitivitaet floor: 1e-7 -> 9, 1e-6/1e-5 -> 14, 1e-4 -> 15.
  L 98-59 b-Praemisse benannt: vulkanische SO2-Atmosphaere einer Sub-Erde, kein
  solares H2-dominiertes Nullgas. OFFEN: jwst_detection_registry_compiler wirft
  pl_name in der Registry fallen (Saat traegt ihn, Compiler-Kopie nicht);
  DMS/DMDS-K2-18 nicht in der Saat (degeneriert). cargo check 0 Warnungen
  (omegaflow-measure + omegaflow-harvest), keine CDN-Manifestation.


- Kondensation unter 500 K (2026-09-05): 500-K-Floor war eine kodierte Schranke,
  kein Fit-Limit — separater Pfad equilibrium_composition_condensed (273,16-500 K,
  H2O(l)-JANAF-Kondensat gesourct, p_sat-Kreuzdatum 3,169 kPa; NH3/CO2/CH4/H2S-
  Kondensate pending, fuer die 5 Ziele irrelevant). Die 5 domain-pending aufgeloest:
  K2-18 b CO2 = Disequilibrium-Hit (Gleichgewichts-CO2 ~1e-30 vs detektiert ~1%,
  photochemisch; CH4 gleichgewichts-konsistent), TOI-270 d Hit; LP 791-18 c /
  LTT 3780 c / TOI-199 b equilibrium-praesent (CH4 im Gleichgewicht). Katalog:
  16 Hits / 13 praesent / 1 ohne-Modell / 0 pending; Permutations-Null mean 18,30
  sigma 1,79 Schwelle 21,89 P(T>=16)=0,9424 — kein Ueberschuss. 23 thermochem-
  Tests gruen (H2O(l)-Anker, p_sat, Grenzabgleich 500 K), cargo check 0 Warnungen.


- Manifestation Biosignatur-Linie (2026-09-05): jwst_detection_registry.json
  auf dem CDN (HTTP 200, 32026 B, 48 Ziele / 73 Detektionen mit pl_name).
  Detektions-Saat versioniert unter docs/reference/jwst_detection_seed.json
  (73 Zeilen). pl_name-Duty geschlossen (Compiler traegt pl_name durch, dedup
  host+pl_name). Das Mess-Urteil (16 Hits, P=0.9424) ist registriert; ein
  Blaetter-Ein-Blatt steht noch aus (ein-blatt-papier.md-Form).


- Drei neue Auftraege (2026-09-05, docs/auftrag/): (1) auftrag-techno-narrowband-
  scan — Radio/Laser-Narrowband-Kanal der Technosignatur; (2) auftrag-techno-
  atmosphaeren-gase — Atmosphären-Technosignatur CFC/NO2/SF6; (3) auftrag-
  negativ-fuzzy-techno — der negative Fuzzy-Index formal auf Bio/Techno-Ausschluss.
  Alle status pending.


- Vier Kanal-Sessions (2026-09-05, parallel, sub-agents): (1) Kanal 2 Techno-Gase:
  equilibrium_composition_halogen (7 Elemente, 11 F/Cl-Spezies NIST-JANAF gesourct)
  + techno_gas_register_probe — 0 Industrie-Detektionen im Seed, systematische
  CFC/SF6-Absenz mit Empfindlichkeits-Achse (23 beobachtbare Paare), Floors
  astronomisch tief (SF6 1e-84..1e-102). (2) Kanal 1 Narrowband: Breakthrough-
  Listen-Produkte verortet (HTTP 200, echte Linien-Samples 339k+ Zeilen); der
  Linien-Weg ist gebaut — bl_narrowband_compiler
  (tools/harvest/src/bin/bl_narrowband_compiler.rs) schreibt den BLN1-Traeger
  (src/archivar/bl_narrowband.rs, Format bl_narrowband, jede Zeile traegt
  freq/bin_width/val + RA/DEC + MJD-Epoche, Lesepfad meidet die hart-0-extract.rs-
  Stellen), Quellen-Block in phi/sources.φ registriert, Lauf auf echtem CSV-Slice
  + Roundtrip gruen. Pending, benannt: der volle 133-MB-Lauf (Download-Budget),
  die CDN-Manifestation (--ci-mode), die Distanz der Ziel-Sterne (HIP-
  Parallaxen-Abgleich pending; die Messung ist am Messort GBT geankert, 0 honored
  statt erfundener Parallaxe). (3) Kanal 3 Negativ-Fuzzy: negativ_fuzzy_probe (zweistufig:
  OLS-Rest + Negativ-Test, 'not carried' nie 'independent', 3 Grenzen sichtbar);
  te.rs ols_residual + Skalen-Fix; Real-Laeufe + Positiv-Kontrolle + Bio-Katalog
  P=0.9424 reproduziert. (4) Bio-Zeugen: st_met [Fe/H] gelesen (30/30 Hosts),
  reservoir-scaling; EIN Hit bewegt sich (WASP-166 b CO2 knife-edge bei
  [Fe/H]=+0.19), 15 bleiben; C/O + Aktivitaet/XUV pending (keine NExScI-Spalte);
  Hit-vs-Metallizitaet r=+0.051 P=0.79. 33 thermochem-Tests gruen, cargo check
  0 Warnungen.


- Narrowband-Manifestation korrigiert (2026-09-05): die volle BL-2017-Datei ist
  Content-Length 10,77 GB (All_hits_turbo_seti.csv), NICHT 133 MB (der Sub-Agent
  hatte die Groesse falsch gemessen; AAA_candidates 440 MB, 2019er Events ~0,9 MB
  mit anderem Schema: Source=HIP, kein RA/DEC -> braucht HIP-Parallaxen-Abgleich).
  Der volle Lauf ist eine Budget-Ernte (Range-Fetch, ~1,6 MB/s -> ~2 h fuer 10,77 GB),
  kein Einzel-Download. Compiler-Upload-Tag auf blpd0.ssl.berkeley.edu korrigiert
  (upload_release, war faelschlich ssd.jpl.nasa.gov). Manifestation pending: volle
  Ernte + CDN + HIP-Parallaxen (kein Fabricat).


- Narrowband-2017-Ernte + CDN GESCHLOSSEN (2026-09-05): volle 10,77-GB-Datei
  gestreamt kompiliert (28,86 M Hits, 0 malformed), Bin 1,385 GB, Release
  blpd0.ssl.berkeley.edu angelegt + Asset manifestiert (HTTP 200, Register-URL
  erfuellt). Compiler liest jetzt gestreamt (read_to_string konnte 10,77 GB
  nicht halten). OFFEN bleibt: HIP-Parallaxen-Abgleich (I/311/hip2-Map gezogen,
  hip2019_icrs.csv) + 2019er-Wiring (Schema ohne RA/DEC/MJD, DriftBW=0,0005 MHz
  ist Such-Konstante, keine Linienbreite — als bin_width waere es Fabricat).


- Nadel-XIII-XUV-Zensus + Nadel-V-LSST-Scan (2026-09-05, externe Rueckmeldung +
  Broker-Eskalation): (1) co_rhk_witness_seed jetzt 77 Zeugen-Zeilen (log R'HK
  10 Wirte, L_X 3, P_rot 13, C/O); XUV-Regression r=-0.609 P=0.0724 — Hits sitzen
  auf ruhigeren Sternen (Gegenteil der Photochemie-Erwartung), alle 16 Hits
  ueberleben, kein Hit bewegt; voller 48er-Zensus (30 detection + 18 non_detection
  als 0 honored, jwst_host_census.json). (2) LSST-Scan: Fink/LSST-Broker anonym
  erreichbar (api.lsst.fink-portal.org 200, echte g/r/i/z-Lichtkurven), --fink in
  lsst_anomaly_probe; Live-Lauf: achromatischer Dip aber periodisch (FAP 0) ->
  still, 0 Kandidaten; Significance-Gate-Bug korrigiert (negative Dip-Signifikanz
  wurde nie >= +DIP_SIG, achromatischer Schnitt war inaktiv). Lasair-LSST token-
  gated (Selbstregistrierung moeglich, pending). VPN (Proton) noetig fuer lasair,
  Fink ohne VPN erreichbar. Photochemie-Re-Erklaerung der SO2/CO2-Hits bleibt
  pending (kein XUV-Kanal im thermochem, 3 Wirte tragen L_X).


- Galaxie-Scan (LSST/Fink) + XUV-Kataloge (2026-09-05, beide parallel): (A) Kegel-
  Scan ueber echte Menge — Fink conesearch (anonym, Radius bis 5°), 62 Objekte
  in 300″, 12 mit Mehrband-Daten erreichen das achromatische Tor, Verdikt 0
  Kandidaten (quantitatives Limit); lsst_anomaly_probe + conesearch. Skalengrenze
  ehrlich: voller Wochen-Galaxie-Scan (~1,4 Mio) braucht Fink-Account/Kafka oder
  Rubin-Science-Platform (kein anonymes Vollflaechen-Endpoint, /api/v1/tags 500).
  (B) XUV via HEASARC/XMM-Newton/ROSAT/eRASS1 + Polanski-KeckSpec: L_X gefunden
  fuer GJ 3090, HIP 67522, L 98-59, V1298 Tau, TOI-270, GJ 9827, WASP-43; C/O
  fuer 7 Wirte; Seed 77 -> 88 Zeilen. Pending ehrlich: X-ray absent <72″ fuer die
  meisten hellen + M-Zwerg-Wirte, log R'HK numerisch blank in Brewer, C/O der
  M-Zwerge absent. Photochemie-Re-Erklaerung: jetzt 10 Wirte mit L_X/F_X
  evaluierbar.

- Disequilibrium-Register-Urteil III — externe Korrekturen verifiziert + L_X-Kanal
  (2026-09-05, disequilibrium_register_probe): (1) Ordnungen-Gegenpruefung:
  Wiederholungslauf misst die solaren 1-bar-Teq-Gleichgewichtsanteile unveraendert
  (WASP-39 b Teq 1166 K SO2 1.625e-16; WASP-107 b Teq 737 K SO2 4.899e-22; K2-18 b
  CO2 8.011e-31). Die Zahl "16-22 Ordnungen" steht nirgends als Text (gemessen per
  grep — das Register sagt "Detektion um >10 Groessenordnungen"); gemessen gegen
  eine publizierte Photochemie-Detektion von 1-10 ppm (Tsai et al. 2023 Nature) ist
  die Diskrepanz ~10-11 Ordnungen (WASP-39 b) und ~15-16 Ordnungen (WASP-107 b).
  Die Literatur-Korrektur "3-4 Ordnungen" misst gegen ein anderes Referenz-
  Gleichgewicht: Tsai nennt ~1 ppb als thermochemische Obergrenze entlang des
  heissen Atmosphären-P-T-Profils, die Sonde loest das solare Gleichgewicht bei
  einheitlichem Teq und 1 bar — beide klein, verschiedene Bezuege; das
  disequilibrium-Urteil (hit) traegt, nicht ein Ordnungen-Exponent. (2) WASP-166 b
  verifiziert: CO2-hit (Gleichgewichts-CO2 4.146e-7 < floor 1e-6), bewegt sich bei
  [Fe/H]+0.19 (1.006e-6) und C/O-Zeuge (1.338e-6) zu equilibrium-present — knife-
  edge bestaetigt; die Saat fuehrt WASP-166 nur als H2O+CO2 (Mayo et al. 2025,
  2025AJ....170...50M) — kein SO2-Hit, keine Korrektur noetig. (3) HST/JWST:
  kein Text in docs/paper, TODO oder auftrag zitiert die 20-Planeten-Stellar-
  Aktivitaets-Kontaminations-Studie (HST STIS/WFC3) als JWST — keine Fehlattribution
  gefunden; die Zensus-Note GJ 1214/GJ 486 "Wasser-Atmosphaere ODER Stern-
  Kontamination" bleibt instrumenten-neutral. (4) NEU im Probe-Code: der zweite
  Reinigungsschritt regressiert die Hit-Indikator-Spalte jetzt auch gegen log10 L_X
  (erg/s, Mittel ueber die L_X-Werte des Registers); F_X/L_X-L_bol sind mit L_X
  nicht kommensurabel und tragen keinen gemeinsamen Kanal. Gemessen, n = 9 Wirte:
  Pearson r = +0.066, Permutations-P = 0.873 — die SO2/CO2-Hits sitzen NICHT
  bevorzugt auf den hohen-L_X-Wirten (Hit-mean log10 L_X 28.16 | present 27.95);
  die Hits spannen die volle L_X-Breite (K2-18 26.42, TOI-270 26.61, L 98-59 27.11,
  WASP-107 27.94 bis HIP 67522 30.64, V1298 Tau 30.20), WASP-121 (29.11) ist
  equilibrium-present — keine XUV-Re-Erklaerung im Katalogmassstab; WASP-39 b (das
  kanonische SO2-Photochemie-Ziel) traegt keinen numerischen L_X-Zeugen (pending).
  Ausgabe /tmp/opencode/disequilibrium_register_verdict_v2.txt. cargo check
  -p omegaflow-measure 0 Warnungen.


- Nadel-V-LSST-Erweiterung — TDB+Zeuge (pending, Verdrahtung 2026-09-05):
  (1) Roemer-Term ergaenzt: je Reihe TDB + n·(Station-Sonne)/c, Cerro Pachon
  WGS84, real gemessen 9625/9625 Reihen; Term-Budget: Roemer 499,0 s Amplitude,
  60,1 s/Woche Drift, TAI-UTC 37 s fold-invariant, Diurnal 21 ms, Shapiro 9,85 us.
  (2) VSX-Zeuge extern verdrahtet (B/vsx 200, 3″-Kreuzmatch, broker-Spalte nie
  Zeuge), Kegel AGN-dominiert: 0 VSX-Treffer (gemessene Abwesenheit).
  (3) NEGATIVKONTROLLE FING EINEN ECHTEN FEHLER: lomb_scargle_fap ist NICHT
  skaleninvariant — FAP 0.00e0 auf jeder Zeile auch bei 6-10σ-achromatischen
  Injektionen; die frueheren LSST-'0 Kandidaten' sind eine Tor-Grenze, keine
  Messung. GESCHLOSSEN (2026-09-05): skaleninvariante Standard-Normalisierung
  verdrahtet (Ursache gemessen: x² im Nenner ⇒ Statistik ~ n/σ²); Negativkontrolle
  findet die Injektion jetzt voll durchs Tor (FAP 1.61e-2/3.74e-2/7.48e-2 bei
  6/8/10σ), Periodisch-Natur bleibt ausgeschlossen (FAP 3.79e-4). (4) cycle_phase_shift_surrogate als getestete Primitive in te.rs gebaut
  (Nutzung pending, keine erfundene Faltung). (5) Positivkontrolle: leer benannt
  (AGN-Feld ohne bestaetigte Chromatik-Periodika, wartet auf variablenreichen
  Kegel). (6) B-Unlock geparkt: Fink-Account/Kafka (fink-broker.org/joining/)
  oder Rubin-Science-Platform, benannte Kosten, kein Datum.


- Nadel-V IRSA-ZTF-Flaeche (2026-09-05, gemessen): das vorhandene
  ztf_lightcurves.bin (CDN, 1,48 MB, 2253 Objekte, 2018-2025) ist KEIN Sky-Sweep,
  sondern eine 7-Kegel-Stichprobe (Cluster 503/421/371/369/260/200/129, RA 243-284,
  dec -24..+25, ohne i-Band) — Re-Scan = dieselbe Flaeche. FRISCHE Flaeche ist
  anonym erreichbar: IRSA nph_light_curves (POS=CIRCLE) liefert echte g/r+i-Kurven
  (frischer Kegel ra 210 dec +30: 78 Objekte, g 3787 + r 6005 + i 2282; langsam
  27-180 s/Kegel aber live, kein Auth fuer die oeffentliche DR). Der
  ztf_lightcurves_compiler ist ein generischer Kegel-Harvester — ein frischer
  Catalog-Lauf ueber die volle Palomar-ZTF-Footprint (ohne --limit 32) erschliesst
  neue Objekte samt i-Band: die echte historische Mehrband-Flaeche fuer Nadel V.


- Nadel-V FRISCHER ZTF-Harvest aufs CDN (2026-09-05): dedizierter Workflow
  ztf-fresh-cdn.yml (1 Job, workflow_dispatch, NICHT kernel-flatten) erntete die
  frische Region ra 210 dec +30 (Radius 0.02) via IRSA anonym — 78 Kurven, 12074
  Samples, g+r+i — und uploadete als NEUES Asset ztf_lightcurves_fresh.bin
  (irsa.ipac.caltech.edu, CDN HTTP 200, 148328 B). Das stale 7-Kegel-Asset
  ztf_lightcurves.bin bleibt unangetastet (A=A: neue Messung = eigener Name).
  sources.phi-Block + cds_watchdog-Paar registriert. Naechster Schritt: den
  frischen Harvest durch die Nadel-V-Achromatizitaets-Logik (lsst_anomaly_probe
  --ztf / ztf_anomaly_probe) laufen lassen.

- Kanonische Ein-Blatt-Auswahl (2026-08-21, offen — Konsolidierung ist
  ein Wort des Operators): die Ein-Blatt-Dokumentation liegt in mehreren
  parallelen Bäumen — Konzepte `ein-blatt-axiom.md`,
  `ein-blatt-papier.md`, `blatt-papier-beweis.md`, `blatt-papier-resultat.md`,
  `der-kausalpfeil.md` und Handover-Varianten je Rätsel (`*enso-kausalpfeil*`,
  `*bz-*`, `*laic-*` — drei bis vier Dateien je Rätsel, teils mit
  `sha256: pending`). LAIC-Baum ist geklärt (das Blatt steht definitiv).
  Offen bleibt, welche ENSO/Bz-Konzept- und Handover-Dateien kanonisch
  sind und welche archiviert werden — deren see-also-Zeilen tragen noch
  tote Handover-Referenzen (docs/paper/-Ordner-Klasse + Concepts-Zweite-
  Achse: 2026-08-22/24 geregelt, die ENSO/Bz-Auswahl bleibt).

- Richtungs-Transient ohne Distanz — das Feld-Atom (Council 2026-09-05,
  verdikt: GROSSES offenes Architektur-Atom, KEIN Quickfix): ZTF-Transienten-
  Quellen (Lasair, ALeRCE, ANTARES) liefern ra/dec + em-Magnitude, aber KEINE
  Distanz. Die CelestialMap-Distanz-Gate (extract.rs ~2121, no_distance_skipped
  assertiert 0) verwirft jedes richtung-only-Element. Drei Dispositionen
  (Council-einstimmig): (A) Referenzradius/Einheitsvektor-in-Metern = erfundene
  Distanz = **Fabrication, ausgeschlossen** (0 honored, A=A). (B) ist der EINZIGE
  ehrliche Pfad: eine Himmelsrichtung als eigenes Atom — Winkel-Kernel ohne
  radialen Abfall, Richtungs-Unsicherheit statt Radius, Präsenzrahmen ohne Tiefe.
  Das ist eine **fundamentale Wire-Vertrags-Erweiterung über drei Ebenen**
  (Rust-Wire 26×f64 + JS-Repack + WGSL), KEIN 26×f64-Overload, KEIN Overload
  der 3D-xyz-Slots. **Gehört vor den vollen Rat**, wenn richtung-only der
  Endzustand einer Messung ist — nicht ihr Wartezustand. Heute lohnt der Bruch
  nicht für drei Quellen, deren Distanz mit Klassifikation/Redshift nachrückt.
  Litmus-Befund: die Richtung IST ein echter Sinn (das Auge misst die Richtung
  eines Blitzes ohne Distanz) — legitimes zukuenftiges Atom, kein totes Ende.
  Drei Quellen = EINE pending-Klasse. Registriert in blocked_sources.φ:
  Lasair (parser-def, 2026-09-05 aus sources.φ verschoben), ANTARES (parser-def
  json), ALeRCE (Code: build_alerce_channels, "dark until a distance channel
  exists").
