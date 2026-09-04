<!--
  title: Befund — Voyager-Roh-Doppler-Zugang (PDS/JPL), eigenhändig verifiziert
  class: befund
  date: 2026-09-04
  sha256: c8a596e6f021ef4406345cb8d0fabdca2568ed547cdfc1c12ef86e705b904ee8
  status: done
  antwortet-auf: docs/auftrag/auftrag-voyager-roh-doppler-zugang.md
  see-also: docs/auftrag/auftrag-quiet-zone-uebertragung.md docs/reference/woo-armstrong-1979-jgr-abstract.md docs/TODO.md
-->
# Befund: Voyager-Roh-Doppler-Zugang (PDS/JPL) — eigenhändige Gegenprüfung


## Kurzfassung


Der Vorbefund war richtig, aber unvollständig. Es gibt **mehr** offene
Voyager-RSS-Rohdaten-Bündel als die zwei/drei vermuteten — aber **alle**
sind Encounter-Fenster (Jupiter 1979, Titan 1980, Saturn 1981, Uranus
1986), keines berührt die Cruise ~1998–2002. Und es gibt eine
**neue, tragende Erkenntnis**, die der Vorbefund nicht hatte: der Grund,
warum niemand je ein Turyshev-artiges Projekt gestartet hat, um
Voyager-Cruise-Doppler von Band zu bergen (wie bei Pioneer 10/11) — dazu
unten mehr. Verdikt: **Nein**, keine offene Quelle deckt das
Doppler-Fenster 1998–2002 ab. Tür 1 bleibt geschlossen — jetzt mit
eigenem Beleg statt nur Agenten-Wort.


## Prüfschritt 1 — PDS-RMS durchklicken


Gemessen (heute, 2026-09-04, per HTTP-Fetch/Suche):


- `pds-rings.seti.org/pds4/bundles/voyager_rss_raw/` enthält **nicht**
  zwei, sondern mindestens **fünf** Bündel, alle open-loop,
  alle Encounter:
  - `voyager1_rss_jupiter_raw` — VG1 Jupiter, 1979-064 (Tag des Jahres),
    ODR/REDR, S-/X-Band, Dateiendungen `.dat` (binär), `.hdr`, `.tab`,
    `.txt`, `.xml`, `.pdf` (Browse).
  - `voyager2_rss_jupiter_raw` — VG2 Jupiter, 1979-191, gleiches Format.
  - `voyager1_rss_titan_raw` (a.k.a. VG1-SSA-RSS-1-ROCC-V1.0) — VG1
    Titan-Okkultation, Tag 1980/317 (November 1980).
  - `voyager2_rss_saturn_raw` — VG2 Saturn-Okkultation, Tag 1981/238.
  - `voyager2_rss_uranus_49xr_raw` (Parkes) und ein weiteres Uranus-Bündel
    von Canberra (DSS-42/43) — VG2 Uranus, 24.–25. Januar 1986.
  - Direkt gelesen: die SIS (`sis_vgj_rs_1.0.pdf`) für die
    Jupiter-Bündel — bestätigt wörtlich: primärer Datentyp ist die
    „Original Data Record (ODR)", d.h. **open-loop**, nicht
    Closed-Loop-Doppler (ATDF/ODF). Kein Range-Rate-Feld im Sinne von
    Navigation/TRK-2-34.
  - Gemessener HTTP-Fehler: der direkte Index-Link
    `.../voyager1_rss_jupiter_raw/` lieferte „Too many redirects" — das
    Bündel ist über die Such-/Review-Seiten erreichbar, nicht über den
    naiv konstruierten Pfad. Notiert, nicht überbewertet.
- **Verdikt PDS-RMS Voyager-RSS-Raw:** `open` (anonym ladbar, HTTP
  bestätigt über Such-Cache und SIS-PDF-Fetch) — aber **kein**
  Cruise-Doppler, nur Encounter-Okkultation, open-loop.


## Prüfschritt 2 — PDS-weit (andere Knoten)


- **PPI-Knoten** (`pds-ppi.igpp.ucla.edu`): Plasma-Wave-Daten (PWS)
  für die gesamte Mission (1977–2019/2025 laufend gepflegt) — aber das
  ist PWS, kein RSS/Doppler. Für Radio Science fand ich am PPI-Knoten
  keinen Voyager-Cruise-Datensatz, wohl aber ein direktes Analogon bei
  einer anderen Mission: `GO-X-RSS-1-ODR-V1.0` (Galileo,
  Gravitational-Wave-Experiment während der Cruise zu Jupiter,
  1994–1995) liegt **offen** im PPI-Annex. Es existiert also
  Präzedenz dafür, dass Cruise-GWE-Rohdaten anderer Missionen archiviert
  werden — nur für Voyager selbst fand sich kein Gegenstück.
- **Geosciences-Knoten** (`pds-geosciences.wustl.edu`): eigene
  Aussage der Knoten-Startseite: „The PDS Geosciences Node archives
  radio science data from missions to **Mars, Mercury, Venus, and
  Earth's Moon**." Voyager ist dort explizit **nicht** gelistet. Der
  Multi-Mission-DSN-Bundle (`urn-nasa-pds-jpl_dsn_mmm`) enthält nur
  Kalibrationsdaten (EOP/Troposphäre/Wetter), keine
  missionsspezifischen ODF/TNF — auch dort kein Voyager-Eintrag.
  Gefunden wurde allerdings die Formatdokumentation (ATDF-Spacecraft-ID-
  Tabelle), die „Voyager 1" (Code 31) und „Voyager 2" (Code 32) als
  gültige ATDF-Raumfahrzeug-Codes listet — das ATDF-**Format** kennt
  Voyager, es existiert nur kein archiviertes Voyager-ATDF-Bündel bei
  diesem Knoten.
- **Verdikt PDS-weit:** `unavailable` für Voyager-Cruise-Doppler; `open`
  für das strukturanaloge Galileo-GWE-Cruise-Bündel (zeigt: sowas wird
  grundsätzlich archiviert, aber nicht für Voyager).


## Prüfschritt 3 — NSSDCA-Katalog


- `cdaweb.gsfc.nasa.gov/pub/data/voyager/voyager1/00readme.txt`
  bestätigt exakt die Vorbefund-Struktur: Unterverzeichnisse
  `magnetic_fields`, `plasma`, `merged` (SSC-Stundenmittel, keine
  Range-Rate), `particle`, `traj` (SSC-Ephemeride) — kein RSS/Doppler-
  Verzeichnis überhaupt vorgesehen.
- data.gov-Spiegel des NSSDCA-Katalogs listet für Voyager 1 Radio
  Science nur: „VOYAGER 1 TITAN RADIO OCCULTATION RAW DATA V1.0"
  (November 1980, Encounter, open-loop) — deckungsgleich mit dem
  PDS-RMS-Fund oben, keine zusätzliche Cruise-Quelle.
- **Verdikt NSSDCA:** `unavailable` für Cruise-Doppler; `open` für
  Encounter-Okkultationsdaten (bereits unter Schritt 1 gezählt).


## Prüfschritt 4 — NAIF gegenprüfen


- `naif.jpl.nasa.gov/pub/naif/VOYAGER/kernels/spk/aareadme.txt`
  bestätigt: die SPKs (`vgr1_jup230.bsp` etc.) sind **rekonstruierte
  Trajektorien**, erzeugt von R. Jacobson (JPL Solar System Dynamics)
  aus „allen verfügbaren Radio-Tracking- und optischen
  Navigationsdaten" — d.h. NAIF liefert das **Ergebnis** einer
  Bahnbestimmung, nicht die Doppler-Rohmessungen selbst. Explizit
  bestätigt in den `.cmt`-Kommentardateien einzelner Kernel
  (`vgr1_jup204.cmt`, `vgr2_sat261.cmt`).
- **Verdikt NAIF:** `open` für rekonstruierte Bahn (SPK); `unavailable`
  für Roh-Doppler — NAIF führt kein ODF/TDF, wie im Vorbefund vermutet.


## Prüfschritt 5 — JPL/DSN-Anfrageweg


- Der offizielle Formatstandard für Closed-Loop-Tracking ist
  **TRK-2-34** (JPL D-16765/D-76488, „DSN Tracking System Data
  Archival Format"), aktuell Rev. P (2017). Das Dokument selbst ist
  öffentlich als PDF/Text greifbar (mehrere PDS-Spiegelungen,
  z. B. `pdssbn.astro.umd.edu`, `atmos.nmsu.edu`).
  Das ist aber nur die **Formatspezifikation** — kein Datenzugang.
- Der tatsächliche Verteilweg für ODF/TDF/TNF-**Daten** läuft laut den
  gefundenen Archiv-Dokumentationen (Magellan-, MGS-, Odyssey-, MESSENGER-
  Beispielen) über den jeweiligen **PDS Radio Science Subnode**
  (institutionell: Stanford/SETI-Institute, verantwortlich lange Zeit
  Richard A. Simpson, `radiosci@att.net`) bzw. den **PDS Geosciences
  Node** (`geosci@wunder.wustl.edu`) — für Missionen, die dort geführt
  werden. Für Voyager-Cruise-Tracking existiert **kein** solcher
  archivierter Bestand bei einem dieser Knoten (siehe Schritt 2), also
  auch kein automatischer Download-Endpunkt.
- Ein genereller, formularbasierter „Tracking-Data-Request"-Endpunkt
  bei JPL/DSN direkt (außerhalb der PDS-Knoten-Struktur) wurde bei
  dieser Suche **nicht** gefunden. Der plausible, dokumentierte Weg ist
  Kontaktaufnahme mit dem PDS Radio Science Subnode / Geosciences Node
  bzw. dem JPL Navigation- und DSN-Betrieb direkt — nicht anonym, nicht
  über einen offenen HTTP-Endpunkt.
- **Verdikt JPL/DSN ODF/TDF (TRK-2-34):** Format-Dokument `open`;
  Voyager-Cruise-**Daten** selbst `request-only` (kein öffentlicher
  Endpunkt gemessen) — deckungsgleich mit dem Vorbefund, jetzt mit
  konkret benannten Kontaktpunkten statt nur „request-only" pauschal.


## Prüfschritt 6 — Depot-Suche (Turyshev-Äquivalent)


Das ist der ergiebigste Schritt und liefert die entscheidende neue
Erkenntnis.


- Für **Pioneer 10/11** gibt es genau das gesuchte Muster: Doppler-Bänder
  wurden Jahrzehnte später geborgen, aufbereitet und öffentlich gemacht
  (Anderson et al. 1998/2002; Turyshev et al. 2011, arXiv:1107.2886;
  weitere unabhängige Reanalysen, z. B. arXiv:gr-qc/0208046) — genau das
  Vorbild, das der Auftrag als „Turyshev-Äquivalent" sucht.
- Für **Voyager** existiert dieses Pendant nicht, und die Fachliteratur
  nennt explizit den Grund: Voyager 1/2 sind **dreiachsenstabilisiert**
  (nicht spinstabilisiert wie Pioneer). Die dafür nötigen
  Lageregelungs-Gasdüsen erzeugen einen Navigationsfehler in der
  Größenordnung ~10⁻⁶ cm/s² — das ist **eine Größenordnung größer** als
  die gesuchte Pioneer-Anomalie selbst (Nieto, Anderson, Laing, Lau,
  Turyshev, arXiv:hep-ph/0110373, mit Fußnote „The Voyagers are
  three-axis stabilized"; ebenso arXiv:1307.0537 und arXiv:1001.3686,
  letzteres explizit: „unlike Pioneer 10 and 11, Voyager 1 and 2 are not
  spin stabilized [...] despite these difficulties, navigational data
  was obtained [...] with sufficient accuracy for precision orbit
  estimation" — aber eben nicht präzise genug für ein Pioneer-artiges
  Anomalie-Projekt).
- Das erklärt **kausal**, warum nie jemand ein Voyager-Cruise-Doppler-
  Bergungsprojekt aufgesetzt hat: die Physik macht das Signal für genau
  diese Fragestellung unattraktiv, nicht Archiv-Vergesslichkeit.
- Zenodo/Figshare/Dataverse/arXiv/ADS-Suche direkt nach „Voyager cruise
  Doppler" bzw. „Voyager Doppler deposit" fand **keinen** Datensatz-
  Deposit — nur die oben zitierte Sekundärliteratur, die Voyager als
  Kontrastfolie zu Pioneer erwähnt, ohne selbst Rohdaten zu
  veröffentlichen.
- **Verdikt Depot-Suche:** `unavailable` — kein öffentliches Depot,
  mit einer nachvollziehbaren wissenschaftlichen Begründung, warum
  keins entstanden ist.


## Nachtrag — zwei offene Fäden geprüft (auf Nachfrage)


- **IPNPR (Interplanetary Network Progress Report, JPL)** durchsucht:
  die Voyager-Tracking-Serie dort (regelmäßige DSN-Statusartikel) endet
  in der Uranus-Ära (letzter gefundener Artikel Mai 1986). Ein 1984er
  Methodenpapier („The Performance of Differential VLBI Delay During
  Interplanetary Cruise", IPN Progress Report 42-79) nutzt frühe
  Voyager-Cruise-Radiometrie zur Bewertung von Navigationsstrategien —
  aber das ist 1984, nicht 1998–2002, und liefert keinen Datensatz,
  nur Methodenvergleich. Für 1998–2002 keine Fortsetzung gefunden.
  `unavailable` für das Zielfenster.
- **VLBI-Fremdbeobachtungskampagnen** durchsucht: die einzige konkrete
  Voyager-Doppler-VLBI-Kampagne, die auffindbar ist, ist Medicina/Italien,
  Juli–August **1988** (Horton et al. 1990; iris.uniroma1.it) — testete
  digitale Tone-Extraction gegen DSN Madrid, außerhalb des Zielfensters.
  Spätere VLBI-Doppler-Programme wie PRIDE (EVN, ab 2013) zielen auf
  Mars Express u. ä., nicht auf Voyager im Fenster 1998–2002. `unavailable`.
- **Neuer, gewichtiger Fund:** Turyshev, Nieto & Anderson schreiben 2005
  explizit (arXiv:physics/0502123, „Study of the Pioneer anomaly: A
  problem set"): „Attempts to verify the anomaly using other spacecraft
  have proven disappointing, because the Voyager, Galileo, Ulysses, and
  Cassini spacecraft navigation data all have their own individual
  difficulties for use as an independent test of the anomaly." Das JPL-
  Team um Anderson/Turyshev — dieselben Personen, die die
  Pioneer-Bänder bargen — **hat also Voyager-Navigationsdaten geprüft**,
  vermutlich über internen JPL-Zugang (Navigationsteam, nicht PDS), und
  explizit als für diesen Zweck untauglich verworfen. Das ist der
  stärkste bisher gefundene Beleg, dass ein Datenzugang existiert(e) —
  aber JPL-intern, nicht öffentlich, und nie zu einem Public-Archiv-
  Eintrag geführt hat, weil das Ergebnis (aus genau dem in Schritt 6
  genannten Grund: Dreiachsenstabilisierungsrauschen) für die
  Fragestellung nicht ergiebig war. **Verdikt: `request-only`
  (bestätigt durch Fremdbeleg, nicht durch eigene Anfrage), keine
  öffentliche Reproduktion.**


Damit sind, soweit mit den verfügbaren Werkzeugen erreichbar, keine
weiteren *unbesuchten* Quellenkategorien mehr offen — offen bleibt nur
noch das **Ausführen** der bereits benannten Anfragewege (JPL Navigation
/ PDS Radio Science Subnode), nicht das Auffinden neuer Kategorien.


## Verdikt-Tabelle


| Quelle | Verdikt | Fenster 1998–2002 Doppler? | Beleg |
|---|---|---|---|
| PDS-RMS `voyager_rss_raw` (5 Bündel: VG1/VG2 Jupiter, VG1 Titan, VG2 Saturn, VG2 Uranus×2) | `open` | Nein — alles Encounter 1979/1980/1981/1986, open-loop | SIS-PDF direkt gelesen, Review-Seiten gefetcht |
| PPI-Knoten (PWS, Galileo-GWE-Analogon) | `open` (aber falsche Mission/Instrument) | Nein | Volume-READMEs, Annex-Fetch |
| Geosciences-Knoten (ATDF/ODF/TNF, Multi-Mission) | `unavailable` für Voyager | Nein — Knoten deckt nur Mars/Merkur/Venus/Mond ab | Knoten-Startseite, Bundle-Index |
| NSSDCA/SPDF | `unavailable` (kein Doppler-Feld) | Nein | `00readme.txt`, data.gov-Katalog |
| NAIF SPK | `open` (rekonstruierte Bahn) | Nein — keine Rohmessung | `aareadme.txt`, `.cmt`-Dateien |
| JPL/DSN TRK-2-34-Format | `open` (Dokument) | — (kein Datensatz) | mehrere PDS-Spiegel des Formatdokuments |
| JPL/DSN TRK-2-34-**Daten** für Voyager-Cruise | `request-only` | Ungeprüft ohne Anfrage — kein offener Endpunkt gefunden | Kontaktpunkte Simpson/Geosciences Node benannt, nicht ausgefüllt |
| Zenodo/Figshare/Dataverse/arXiv-Depot | `unavailable` | Nein | Literatursuche, kein Treffer |


## Schlusszeile


**Deckt irgendeine offene Quelle das Fenster ~1998–2002 in Doppler ab?
Nein.** Eigenhändig gemessen, nicht nur von zwei Agenten-Durchgängen
übernommen. Der Vorbefund war in der Sache richtig; korrigiert wurde die
Zählung der Bündel (fünf statt zwei/drei, alle Encounter) und ergänzt
wurde die kausale Erklärung (Dreiachsenstabilisierung → Navigationsrauschen
über der gesuchten Effektgröße → nie ein Bergungsanreiz wie bei Pioneer).


## Offen / `pending`


- JPL/DSN-Anfrageweg wurde **benannt**, nicht **ausgefüllt** — ob eine
  konkrete E-Mail an den PDS Radio Science Subnode oder an JPL
  Navigation tatsächlich Cruise-ATDF/ODF für 1998–2002 zutage fördert,
  ist ungeprüft. Jetzt zusätzlich gestützt durch den Turyshev/Anderson-
  2005-Fund: intern existiert(e) offenbar Zugang, extern nicht.
  `pending`.
- Die PDS-Wide-Search-Engine selbst (interaktives Suchformular auf
  `pds.nasa.gov`) wurde nicht per Formulareingabe bedient, nur über
  externe Suche/Kataloge angenähert. `pending`.
- ADS-Volltextsuche (nicht nur arXiv-Cache) nach unveröffentlichten
  Voyager-Cruise-Doppler-Kompilationen wurde nicht erschöpfend
  durchgeführt. `pending`.
- IPNPR und VLBI-Kampagnenrecherche (Nachtrag) haben keine neue Quelle
  für 1998–2002 ergeben; als Kategorien aber jetzt geprüft, nicht mehr
  offen als *Kategorie* — nur die drei o.g. Einzelschritte bleiben
  `pending`.


## Register-Satz


*Der Roh-Doppler ist die Eingangstür des Co-Quiet-Tests. Die eigenen
Augen haben gemessen, nicht nur gelesen — Tür 1 bleibt zu, aber jetzt
mit einem Grund, der über „nichts gefunden" hinausgeht: Voyager wurde
nie zum Pioneer, weil sein eigenes Lageregelungssystem lauter ist als
das Signal, das man hätte suchen wollen.*


## Status


`done`. Eigenhändige Gegenprüfung abgeschlossen; drei Punkte bleiben
`pending` (oben benannt) und sind keine Vermutungen, sondern offen
gelassene nächste Schritte.
