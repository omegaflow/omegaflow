<!--
  title: V1298-Tau-b-OCS-Tiefenbefund — Degeneranz, junger-Stern-XUV, Metallizitaet, OCS-Route
  class: befund
  date: 2026-09-05
  sha256: pending
  status: live
  method: ADS/arXiv (curl, NASA_ADS_TOKEN aus .secrets.local), Volltexte 2025AJ....170..165B (arXiv:2507.08837), 2025ApJ...985..209M (arXiv:2410.17169), 2023AJ....166..196D (arXiv:2310.00155), 2025MNRAS.537.3027K (arXiv:2410.08116), 2026arXiv260827576A; Abstracts 2021MNRAS.500.4560P, 2022ApJ...925..172M, 2023ApJ...951...18M, 2024ApJ...973L..38R, 2021ApJ...923..264T, 2023Natur.617..483T
-->

# V1298-Tau-b-OCS — die vier scharfen Fragen, gemessen

Registrier-Zeile (TODO.md, Nadel XIII PHOTOCHEMIE-AUSSCHLUSS ERGEBNIS, 2026-09-05):
„UEBERLEBT: 1 schwacher Kandidat V1298 Tau b OCS (3.5sigma, eq 7.7e-17, kein
wirtsspezifischer photochemischer Weg, metallarm)". Geprueft gegen das
Primaer-Paper Barat et al. 2025 (2025AJ....170..165B) und die
Photochemie-Literatur. Alle Aussagen sind gemessen (Zitat in Klammern).

Messanker V1298 Tau b (aus dem Paper): R = 9.85 ± 0.35 R⊕, Teq = 670 K,
10–30 Myr alter Wirt (K, ~1.1 M☉), H/He-dominiert, haze-frei, Skalenhoehe
~1500 km, isotherme Retrieveral-Temperatur ~450 K am Terminator.
Detektionen aus der freien Chemie-Retrieval (PICASO): CO2 35σ, H2O 30σ,
CO 10σ, CH4 6σ, SO2 4σ, OCS 3.5σ (2025AJ....170..165B, Abstract + Appendix D).

## 1. Retrieval-Degeneranz — was das Paper tatsaechlich sagt

Der OCS-Befund ist im Paper ausdruecklich „tentative evidence" (3.5σ), nicht
eine robuste Detektion. Die Signifikanz ist ein Bayes-Evidenz-Delta: eine
zusätzliche Retrieval-Laeufe setzen OCS (bzw. CH4, SO2, CO einzeln) auf 0 und
vergleichen ln Z → OCS 3.5σ (2025AJ....170..165B, Appendix D, Abschnitt zur
Signifikanz-Bestimmung).

Das Paper fuehrt KEINE Degeneranz-Analyse von OCS gegen die sicher
vorhandenen CO2/CO/H2O-Banden. Der Volltext enthaelt kein
OCS-bezogenes „degenerate/blend/overlap"-Gespräch; die Degeneranz-Frage wird
im Paper nicht beantwortet. Stattdessen nennt es drei Robustheits-Tests:

(a) Rebinning: Bei 20-Pixel-Bins verschwindet SO2 (4σ), OCS wird aber bei
    aehnlichem VMR (~1e-8) wie im 10-Pixel-Fall zurueckgewonnen. Die OCS-
    Struktur ist also nicht auf ein einzelnes schmales Pixel angewiesen.
(b) Unabhaengiges Modell: OCS ist konsistent mit dem selbst-konsistenten
    PICASO-Grid (Disequilibrium: vertikale Mischung + Photochemie); das Grid
    sagt OCS voraus, und zwar oberhalb des retrievten Werts (Figure-10-
    Legende: Retrieval-OCS liegt ~3σ UNTER der Grid-Vorhersage). SO2 wird vom
    Grid nicht vorausgesagt (2025AJ....170..165B, Abschnitt V.3, Fig.-10-
    Legende).
(c) Der Abschnitt V.3 schliesst wörtlich: der OCS-Nachweis sei „although weak
    (3.5σ), ... consistent with predictions from the PICASO grid".

Antwort 1: Das Paper weist OCS als schwach (3.5σ) und tentativ aus; eine
Degeneranz mit CO2/CO/H2O wird NICHT diskutiert und ist damit nicht
ausgeschlossen. Der Paper-eigene Gegenbeleg gegen einen reinen Artefakt ist
die Unabhaengigkeit der Grid-Vorhersage (das physikalische Modell erzeugt OCS
aus der S-N-C-H-O-Chemie) plus das Rebinning-Ueberleben. Der einzige weitere
publizierte OCS-Kandidat, 4.9-µm an WASP-15 b (Kirk et al. 2025,
2025MNRAS.537.3027K), traegt dieselbe Schwaeche (hybride Modell-Praeferenz
nur 2.6σ in einer Reduktion, agnostisch in der anderen) und wird nicht als
Detektion gezaehlt — beide G395H-OCS-Kandidaten sind schwach.

## 2. Junger-Stern-XUV — gemessen, nicht geschaetzt

V1298 Tau ist ein gesaettigter Röntgen-Emitter in der Prae-Hauptreihe
(solar-mass, ~20–30 Myr; 2025AJ....170..165B; „solar-mass pre-main-sequence
star" 2023ApJ...951...18M).

Gemessene hohenergetische Leuchtkraefte:
- log L_X = 30.1 [erg/s], mittlere Korona-Temperatur ~9 MK (Chandra+ROSAT;
  Poppenhaeger et al. 2021, 2021MNRAS.500.4560P). Unabhaengige XMM-Messung im
  lokalen Zeugen-Register (5XMM-DR15, 0.2–12 keV): F_X = 1.14e-12 erg/s/cm²,
  L_X = 1.6e30 erg/s bei d = 108.2 pc — konsistent.
- Integrierter XUV-Fluss (X-ray + EUV, <912 Å) an der Erde: (3.2 ± 0.3)e-12
  erg/s/cm² (Duvvuri et al. 2023, 2023AJ....166..196D; SED 1–10^5 Å aus NICER
  + HST/STIS + HST/COS). Daraus L_XUV ≈ (4.5 ± 0.4)e30 erg/s (abgeleitet bei
  d = 108.2 pc). Poppenhaeger et al. ordnen den Stern damit „amongst the more
  X-ray luminous ones at this stellar age" ein (2021MNRAS.500.4560P). Duvvuri
  et al. nutzen V1298 Tau als Referenzobjekt fuer das XUV-Saettigungsstadium
  sonnenmassiger Sterne und zeigen die Verstaerkung gegenueber der solaren
  Irradiance-Referenz (auf die V1298-Distanz skaliert) ueber das gesamte
  EUV-Regime (2023AJ....166..196D, Fig. 7); dieselbe Arbeit beschreibt den
  raschen XUV-Abfall nach ~0.1 Gyr ueber gebrochene Potenzgesetze
  (Ribas-et-al.-Referenz in 2023AJ....166..196D, Fig. 9) — ein
  Hauptreihen-Stern gleicher Masse im Alter von ~4.6 Gyr strahlt danach ein um
  Groessenordnungen schwaecheres XUV. Die XMM-Erwartung „relativ massereiche
  Planeten ueberleben" fuer die aeusseren Planeten (Maggio et al. 2022,
  2022ApJ...925..172M) aendert nichts an der aktuellen XUV-Hoehe.

OCS-Photochemie unter hohem XUV — gemessene Literaturlage:
- Das Detektions-Paper selbst faehrt seine Photochemie-Grids (photochem und
  VULCAN, S-N-C-H-O-Netzwerk) MIT der gemessenen jungen-Stern-SED (Duvvuri)
  und erzeugt OCS — das Grid produziert OCS auf/ueber dem retrievten Niveau
  (2025AJ....170..165B, Abschnitt V.3; Appendix E: photochem-Grid variiert
  [M/H] +0.3..+1.95, C/O 0.023..0.69, Kzz 1e6..1e10, Tint).
- Die heisse-Jupiter-Photochemie-Literatur sagt dagegen, dass OCS bei den
  ~mbar-Druecken der Transmission NICHT ueberlebt: OCS wird durch
  Photodissoziation und photochemisch erzeugtes atomares H und S zerstoert;
  OCS-Depletion wird um UV-hellere Sterne (F-Typ, solare UV) erwartet
  (Kirk et al. 2025 zu Tsai et al. 2021/2023: 2025MNRAS.537.3027K Abschnitt
  6; 2021ApJ...923..264T; 2023Natur.617..483T).
- Antwort 2: Das hohe XUV des jungen Sterns ist gemessen (L_X ~1.3–1.6e30
  erg/s, L_XUV ~4.5e30 erg/s, XUV-Referenz-Objekt des Saettigungsstadiums).
  Als OCS-ERZEUGER ist hohes XUV in der
  publizierten Photochemie nicht belegt — im Gegenteil dissoziiert es OCS in
  den Modellen. Die natuerliche OCS-Route am jungen Stern liegt daher nicht in
  der XUV-Photochemie allein, sondern in der Disequilibrium-Kombination aus
  kuehlem Terminator (~450 K), vertikaler Mischung und Metall-Anreicherung
  (siehe 3/4) — die Modelle des Papers, die OCS erzeugen, laufen genau mit
  dieser gemessenen SED und diesen Bedingungen.

## 3. Metallizitaet / C-O — die „metallarm"-Praemisse ist invertiert

Gemessen aus der Retrieval (nicht gelesen aus pscomppars):
- Atmosphaerische Metallizitaet log Z = +0.6 (+0.4/−0.6) × solar ≈ 4× solar,
  also SUPER-solar; das Paper nennt sie „metal-poor" nur relativ zu reifen
  Sub-Neptunen (~100× solar). Das selbst-konsistente PICASO-Grid: [M/H] =
  1.0 ± 0.2 (~10× solar); C/O = 0.22 (+0.06/−0.05) SUB-solar (PICASO-Grid
  0.23 ± 0.08; ATMO-Grid C/O hoeher, aber C/O-Definition anders)
  (2025AJ....170..165B, Abstract, V.2/V.3, Table 1, Fig. 10).
- Konsequenz fuer die Disequilibrium-Zahl: Der Registrier-Wert eq OCS =
  7.7e-17 ist das solare 1-bar-Gleichgewicht bei Teq ≈ 670 K. Das C/O 0.22
  (C-arm) senkt das OCS-Gleichgewicht (OCS ist in C-armen,
  O-reichen Gasen vermindert; gemessen in 2025ApJ...985..209M); supersolare
  Metalle heben es an. Die entscheidende Temperatur-Fehlreferenz: Die Transmission sieht den
  Terminator bei ~450 K (isotherme Retrieval), NICHT Teq = 670 K — und genau
  unterhalb ~500–600 K kippt die S-Chemie (SO2 verschwindet, CS/CS2/S8 und
  bei starker Mischung OCS/H2S uebernehmen; 2025ApJ...985..209M,
  Schlussfolgerungen). Das Schwebel-Pfad-Modell des Surveys ist nur fuer
  500–3000 K definiert (Registrier: S(g)-Fit-Domaene ab 882 K) — die
  450-K-Referenz liegt ausserhalb seiner Domaene.

Quench/Mischung wie CO an WASP-107 b: Ja, die publizierte Route existiert.
Das Paper braucht fuer CH4 ohnehin Tint ~500 K und Kzz ~1e7–1e8 cm²/s (oder
Metallizitaets-Gradient / Flares; 2025AJ....170..165B; Flare-Variante:
2026arXiv260827576A). Mukherjee et al. 2025 zeigen im [M/H]-Kzz-Raum, dass
OCS und H2S in der Photosphere mit Kzz stark ansteigen, besonders bei
metall-angereicherten Objekten (2025ApJ...985..209M). Das ist dieselbe
physikalische Klasse wie der CO-Quench aus heissem Inneren (WASP-107 b,
Sing et al. 2024, im Registrier bereits als CO-Ausschluss gefuehrt).

Antwort 3: Die Atmosphaere von V1298 Tau b ist NICHT sub-solar metallarm,
sondern ~4–10× solar metallreich mit subsolarem C/O (0.22). Das
Paper-eigene Modell verlangt starke vertikale Mischung (Kzz 1e7–1e8) und
heisses Inneres (Tint ~500 K) — unter diesen Bedingungen ist OCS als
Disequilibrium-Spezies durch Mischung/kuehle-S-Chemie erzeugbar; eine
Photochemie-Notwendigkeit besteht dafuer nicht.

## 4. OCS-Route — gibt es eine echte publizierte Route?

Die publizierte Lage ist zweigeteilt, aber eine Route EXISTIERT:

(a) Modell-Route in jungen/kuehlen H2-Atmosphaeren: Mukherjee et al. 2025
    (PICASO + photochem, Teq 400–1600 K, Tint 30–500 K, H2-dominiert) finden,
    dass unterhalb Teq ~600 K SO2 verschwindet und der Photosphere-Schwefel
    von CS, CS2, S8 getragen wird; OCS und H2S steigen mit Kzz in
    metallreichen Photospheren steil an (2025ApJ...985..209M). Das
    Detektions-Paper zitiert genau dieses Werk fuer das „onset of OCS at
    temperatures lower than 500 K" und sein eigenes Grid erzeugt OCS auf/ueber
    1e-8 (2025AJ....170..165B). V1298 Tau b (Terminator ~450 K, [M/H] ~+0.6..+1.0,
    Kzz 1e7–1e8) liegt mitten in diesem Parameterraum.
(b) Zerstoerungs-Route (heiss, UV-hell): In VULCAN/Photochemie-Modellen fuer
    heisse Jupiter wird OCS bei ~mbar durch Photodissoziation und atomares
    H/S zerstoert und um F-Typ-/solare-UV-Sterne depletiert erwartet
    (2021ApJ...923..264T; 2023Natur.617..483T; Caveat-Diskussion in
    2025MNRAS.537.3027K). Das spricht GEGEN eine einfache „hohes XUV macht
    OCS"-Route und fuer den kuehlen+Mischungs-Pfad in (a).
(c) Labor-Route (reduzierte Atmosphaeren): Photochemie in H2S/CH4/N2-
    Analog-Atmosphaeren erzeugt OCS abiotisch (mit CO2 steigt die OCS-Relative
    zu DMS); Reed et al. 2024 (2024ApJ...973L..38R). Das betrifft reduzierte,
    nicht heisse H2-Gase, ist aber der experimentelle Beleg, dass OCS in
    reduzierter Photochemie ein normales Produkt ist.

Antwort 4: Ja — in der publizierten Literatur ist OCS in einer kuehlen
(~450–600 K), H2-reichen, metallreichen Atmosphaere mit starker vertikaler
Mischung erzeugbar (Disequilibrium-S-Chemie; 2025ApJ...985..209M), und das
Detektions-Paper selbst reproduziert den Wert mit seinem Photochemie+Mischungs-
Grid unter der gemessenen jungen-Stern-SED (2025AJ....170..165B). OCS ist also
kein prinzipiell unerklaerbarer Stoff; die hohe XUV allein ist nicht die
Erzeugungs-Route (sie dissoziiert OCS), die Route ist die Kuehle+Metalle+Kzz-
Kombination, die ein junger, aktiv gemischter Sub-Neptun genau bietet.

## Urteil

Gemessenes Urteil: (a) NATUERLICH — V1298-Tau-b-OCS faellt nach der
Ausschluss-Logik des Surveys aus. Der Disequilibrium-Hit (eq 7.7e-17) entstand
dadurch, dass das solare 1-bar-Gleichgewicht bei Teq ≈ 670 K als Referenz
stand, waehrend die beobachtete Atmosphaere bei ~450 K liegt, ~4–10× solar
metallreich, C/O 0.22 sub-solar ist und Kzz ~1e7–1e8 (bzw. Tint ~500 K oder
Flares) ohnehin von CH4 verlangt werden. In diesem Parameterraum sagt die
publizierte Disequilibrium-S-Chemie OCS voraus (2025ApJ...985..209M), und das
Paper-eigene Grid ueberproduziert es (2025AJ....170..165B). Unter derselben
Logik, mit der SO2 (Tsai/Dyrek/DREAMS), CO2 (Moses + supersolar + Puffer) und
CO (Quench, Sing 2024) als natuerlich ausgeschieden wurden, scheidet OCS jetzt
als natuerlich aus — die Bio-Zaehlung „16 → 1 schwach" wird „16 → 0", was der
ehrlichen Registrier-Notiz „kein Bio-Anspruch (0 honored)" entspricht.

Nicht vollstaendig geschlossen — zwei ehrliche Reste:
- (b) Die Detektion ist nur 3.5σ/tentativ; das Paper diskutiert KEINE
  OCS-CO2/CO-Degeneranz. Das Rebinning-Ueberleben und die Grid-Konsistenz
  sprechen gegen einen reinen Artefakt, schliessen ihn aber nicht formal aus.
- Die OCS-Erzeugungs-Netzwerke sind nicht fixiert: Der heisse-Jupiter-Befund
  (OCS wird photodissoziiert; 2021ApJ...923..264T; 2025MNRAS.537.3027K) und
  der kuehle-Disequilibrium-Befund (OCS wird gemischt; 2025ApJ...985..209M)
  stehen in verschiedener Temperatur-/Mischungs-Region — die genaue Netto-
  Bilanz fuer V1298 Tau b ist nicht separat modelliert publiziert.

Die eine entscheidende weitere Messung (benannt):
1. Theoretisch (entscheidet a vs. Rest-c): ein dediziertes S-Disequilibrium- +
   Photochemie-Modell von V1298 Tau b am retrievten Terminator (~450 K) mit
   [M/H] +0.6..+1.0, C/O 0.22, Kzz 1e6–1e10, Tint 100–500 K und der
   gemessenen Duvvuri-SED — Frage: erreicht das modellierte Photosphere-OCS
   ~1e-8? Das ist exakt die registrierte Pflicht „dedizierte V1298-Tau-b-S-
   Chemie".
2. Beobachtend (entscheidet b): hohere-S/N-Bestaetigung der OCS-v3-Bande bei
   4.85–4.9 µm gegen die CO/CO2-Fluegel (gleiche G395H-Epoche oder MIRI) —
   eine echte OCS-Bandstruktur trennt das Feature von einem CO2/CO-
   Kontinuums-Artefakt.

Benannte offene Registrier-Pflicht, wenn der Kandidat ueberlebt (er ueberlebt
nur als „unbestaetigte, natuerlich erklaerbare Detektion", nicht als
„genuiner unerklaerter Rest"): die S-Chemie-Pflicht bleibt bestehen und wird
schaerfer: Temperatur-Referenz (~450 K statt Teq), Kzz-Bereich, und die
OCS-vs-CO2/CO-Degeneranz explizit zu testen. Die Pflicht „OCS-Routen in
reduzierten Atmosphaeren" ist durch Reed et al. 2024 teilbeantwortet
(Labor-Photochemie erzeugt OCS); die Pflicht „Atmos-Metallizitaeten der
CO2-nur-Wirte" ist unabhaengig und bleibt offen.
