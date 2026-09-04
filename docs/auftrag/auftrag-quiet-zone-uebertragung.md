<!--
  title: Auftrag — Quiet-Zone als Rezept: Übertragbarkeit auf die anderen Sonden
  class: auftrag
  date: 2026-09-04
  status: pending
  sha256: 95e3b7d683b179cc4132a5336c07a40c37dbea17ba5e676583fb7e8504f2ab14
  see-also: docs/auftrag/auftrag-subhz-drift-quiet-zone.md docs/paper/probe-front-dark-matter.md docs/reference/woo-armstrong-1979-jgr-abstract.md docs/befund/befund-voyager-roh-doppler-zugang.md docs/auftrag/auftrag-quiet-zone-vorfilter.md docs/TODO.md
-->

# Auftrag: Quiet-Zone als Rezept — Übertragbarkeit auf die anderen Sonden

## Zweck

Die Quiet-Zone-Isolation (P10 >50 AU, 1991–2002) ist in
`auftrag-subhz-drift-quiet-zone.md` GESCHLOSSEN — ihr Verdikt steht (sub-Hz-
Median 0,21 Hz, Drift nicht aufgelöst). Was die drei Tage gebaut haben, ist
keine Pioneer-Technik, sondern ein Muster. Dieser Auftrag trägt das Muster
vom Testobjekt weg: die Übertragbarkeit auf jede Sonde, deren Rauschen einen
Ort hat. Die Anwendungsfläche ist die, die alle anderen aufgegeben haben.

## Das Muster (drei Teile — bei Pioneer billig, weil ein Frame, eine Zeit)

1. **Das Rauschen hat eine Geometrie** — Sonnennähe → Sonnenwind-Plasma.
2. **Die Sonde hat die Zonen durchflogen** — jahrelang weit draußen.
3. **Also ist der Datensatz teilbar** — laut-nah, leise-weit — und die leise
   Teilmenge hat einen Boden (0,21 Hz), den die Aggregation über die
   Gesamtmission nie erreicht.

## Landkarte — die Türen (0 honored)

| Tür | Objekt | Distanz | Vorfilter | Offene Frage |
|---|---|---|---|---|
| 1 | Voyager 1/2 | 160/130 AU | **GESCHLOSSEN** (2026-09-04): dreiachsen-Selbst-Rauschen, kein offener Cruise-Doppler | — (Befund `docs/befund/befund-voyager-roh-doppler-zugang.md`) |
| 2 | New Horizons | 60 AU | **besteht** (2026-09-04): Spin-Cruise, kein Reaktionsrad, >50 AU bewohnt | Harvest nicht offen — `request-only`-Doppler-Anfrage (JPL/DSN-ODF), REX = nur Okkultation/TNF |
| 4 | Mariner/Galileo/Cassini | — | Galileo **bestehen** (Dual-Spin, S-Band medium-getrieben, ≤5 AU — Reserve; Bestand gemessen 2026-09-05: PDS3 `GO-…-RSS-…-V1.0`, TRK-2-25/2-18, kein TRK-2-34, GWE open-loop ODR — `befund-galileo-gwe-bestand.md`); Cassini **fallen** (3-Achsen); Mariner **fallen** (kein Distanz-Muster) | Galileo-GWE als offene Reserve (eigene relativ-ruhige ≤5-AU-Achse, kein Quiet-Zone-Nachbau); Cassini bleibt Kontrast (Ka sauber) |

Tür 3 (die laute Zone als Plasma-Instrument) ist **nicht als Novum** in der
Landkarte — der Literatur-Scan hat sie widerlegt (siehe unten).

## Die zwei ehrlichen Einschränkungen

1. Die Methode braucht das **Distanz-Muster der Störung**, nicht irgendeine
   Störung. Bei einem anderen Hauptrauschen (stations- statt plasma-getrieben)
   ist der erste Lauf die Diagnose „wo sitzt das Rauschen?" — nicht die
   Annahme.
2. Stille Daten garantieren kein Signal. Der Wert ist die Ausweitung des
   Vermessungs-Terrains auf Sonden, die als unbrauchbar galten — nicht die
   Fundchance.

## Co-Quiet-Kreuztest (P10+V1) — warum Tür 1 mehr als ein tieferer Boden ist (2026-09-04)

Die Übertragung auf Voyager 1 hat einen zweiten, schärferen Wert als nur einen
tieferen Boden: sie ist die erste **Co-Quiet-Zwei-Sonden-Messung** — ein
Interferenz-Design, kein Replikations-Design. „V1 als tieferer Boden" wäre
dieselbe Messung an anderer Sonde (Ergebnis: ein Boden). Co-Quiet stellt zwei
Sonden in dasselbe Fenster (Ergebnis: eine *Entscheidung* zwischen gemeinsamer
Kraft und individuellen Effekten). P10/P11 hatten nie ein gemeinsames Fenster —
P10s ruhige Zone (1991–2002) und P11s (1984–1990) schneiden sich nicht; die
widersprüchlichen Drift-Vorzeichen (Deduktion 44: P10 −1,95× sunward, P11
+6,98× outward) sind damit epochal entkoppelt und erlauben beide Lesarten
(gemeinsame Kraft mit Schwankung *oder* individuelle Sonden-Effekte). P10 und
V1 dagegen teilen ein ruhiges Fenster (~1998–2002, P10 bis 2002-03, V1 >50 AU
ab ~1997). Ein gemeinsames Fenster zerstört die Ambiguität.

**Zwei-Wege-Bindung (vorab, gleichrangig — die Deduktion-44-Disziplin):**

- **Ausgang A** — gemeinsames Vorzeichen im selben Fenster: der erste echte
  Zwei-Sonden-Hinweis in der Geschichte dieser Daten; Grund, die Tür wieder
  weit aufzumachen.
- **Ausgang B** — widersprüchliche Vorzeichen im selben Fenster: die
  sauberste Beerdigung — sonnen-individuell (thermal-artig oder Artefakt),
  *niemals* eine gemeinsame Kraft.

Beide Ausgänge sind Befunde erster Güte; der Auftrag bindet beide vorab, er
hofft keinen still. Ein Zwei-Wege-Test, kein Ein-Wege. (Turyshev verglich nie
zwei Sonden im selben ruhigen Fenster — P11 sendete 1990 aus.)

**Atome in Reihenfolge (vor jeder Regression, 0 honored):**

1. **V1-NAVIO-Doppler-Verfügbarkeit am SPDF verifizieren** — verifiziert
   2026-09-04: **nicht vorhanden** (Befund unten, „Atom 1 —
   SPDF-Verifikation"). Vor jedem Harvest.
2. **Das Fenster selbst vermessen, nicht nur benutzen.** „~1998–2002 geteilt"
   ist eine Hypothese, kein Fakt: Das Fenster existiert nur, wenn *beide*
   Sonden dort gleichzeitig hochauflösende, ruhige Tage tragen (P10s
   Missionsende-Datendichte nach 1997? V1s Datenaufkommen jenseits 70 AU?).
   Gemeinsame Tage zählen, stille Tage je Sonde im Schnitt identifizieren,
   **n festhalten** — ein Fenster mit 40 gemeinsamen stillen Tagen ist eine
   andere Messung als eines mit 400. Die n-Zahl gehört in das Protokoll vor
   dem Lauf (verdikt-bindend, wie Deduktion 44).

## Literatur-Scan — Befund (2026-09-04, agent)

**Verdikt (a) bestätigt — mit Restöffnung.** Im gelesenen Primärkorpus
(Turyshev/Toth-Weg 2002–2012, Volltext: LRR 2010, arXiv:1107.2886,
arXiv:1204.2507) wird Drift/Residuum über die Gesamtmission als eine Masse
gefittet; die einzige globale Residuen-Kennzahl (1107.2886, σ_P10 4,40 mHz)
mittelt laut und leise gemeinsam; das Sonnenplasma erscheint als Störung, die
die Richtungs-Entscheidung begrenzt („rendered the effort fruitless"). Eine
systematische Laut/Leise-Teilung nach heliozentrischer Distanz mit separat
vermessem Floor der leisen Teilmenge (>50 AU) steht in **keinem** gelesenen
Text; der nächste Vorfahr (P10-Arcs I/II/III, 2002) teilt nur zur
aP-Konstanz-Prüfung.

**Korrektur zu Tür 3 (gemessen, wichtig):** Die „laute Zone als Instrument"-
Seite ist **kein eigener Fund** — ein etablierter Strang seit den 1970ern:
Woo & Armstrong 1979 (JGR 84, 7288, R^−3,45, **mit Pioneer** als Observable),
Berman et al. 1976 (Doppler-Noise ↔ integrierte Elektronendichte),
Muhleman & Anderson 1981 (Viking). Dort wird das Rausch-Niveau systematisch
als Funktion des Abstands vermessen — als Sonnenphysik-Instrument, nie als
Zonen-Isolation einer leisen Teilmenge zur Anomalie-Messung. Die Kombination
mit der Zonen-Teilung bleibt unbesetzt; das Instrument selbst ist belegt.

**Ehrliche Formel:** „uns bekannt: nein; Scan: durchgeführt; Anderson 2002
gelesen (bestätigt), vier Volltexte pending."

**Volltext-Analyse (2026-09-04, agent, zweiter Durchgang):** Anderson et al.
2002 (gr-qc/0104064) ist jetzt als Text gelesen (ar5iv) — die drei P10-Arcs
sind **Zeit**-Intervalle (motiviert durch die Spin-/Gasleck-Systematik), keine
Distanz-Bänder; je Arc wird nur aP gelöst, der Residuen-Floor bleibt global
(post-fit weighted RMS ≈ 0,1 mm/s P10). Der gelesene Haupttext bestätigt das
Verdikt (a): kein Zonen-Floor.

**Restöffnung (pending — unerreichbar, Lizenz-Schranke, kein Scan-Problem):**
Armstrong 1998 (Radio Sci. 33, 1727), Bertotti/Iess/Tortora 2003 (Nature 425,
374), Tortora et al. 2004 (JGCD 27, 251) — born-digital hinter Verlags-Paywall
(Abstracts zeigen Plasma-Kompensation, keine Zonen-Teilung); Woo & Armstrong
1979 (JGR 84, 7288) — einziger echter Scan-Kandidat, keine offene Version
erreichbar. **Kein Vision-LLM nötig:** die born-digitalen Texte scheitern an
der Lizenz, nicht an fehlender Textschicht — ein Vision-LLM öffnet keine
Paywall; für Woo & Armstrong (1979) ist heute nichts Offenes zu lesen. ADS
(ui.adsabs.harvard.edu, HTTP 405) und Semantic Scholar (429) bleiben ein
eigener Durchgang, sobald erreichbar.

## Atom 1 — SPDF-Verifikation (Befund 2026-09-04, gemessen)

**V1-NAVIO-Doppler ist am SPDF nicht vorhanden.** Der Voyager-Baum
(`/pub/data/voyager/voyager1/`) trägt drei Radio-/Bahn-Orte, keiner ein
Doppler-Tracking:

- `radio_science_rss/` — Okkultation nur (Saturn-Ringe, Titan): kein
  Doppler-Tracking.
- `merged/` — Stunden-Mittel der HGI-Position (heliographische Distanz/
  Breite/Länge, Feld 4–6) + Sonnenwind + Magnetfeld + Protonenflüsse
  (34 Felder laut `vy1mgd.txt`) — **kein Range-Rate-/Doppler-Feld.**
- `traj/` — SSC-Ephemeride (rekonstruierte Bahn, Modell, kein Roh-Doppler).

Kontrast gemessen: die Pioneer-NAVIO-Quelle
(`pioneer/{name}/radio/{name}_doppler_tracking_SC_*.asc.gz`, 647 MB asc,
ODP-Format DTYPE/SC/TRANS/RCVR1/TIMTAG/OBSVBL/FREQCY/CMPTIM) ist ein
Turyshev-Depot 2017-2018 (`Turyshev20170327_Pioneer-10/`) — für Voyager
existiert kein vergleichbares Depot. Das Landkarten-Hindernis „SPDF-
Datenzugang, grobe Zeitauflösung" ist damit gemessen und schärfer: nicht
nur grob — das Doppler-Feld fehlt ganz.

**Was trotzdem trägt:** `merged/` deckt 1977–2025 in Stunden-Mitteln
(ein ASC je Jahr, `voyager1_daily.asc` als Tagesmittel) — die **Zone** ist
daraus definierbar (Distanz >50 AU), die **Doppler-Messung** nicht. Der
Co-Quiet-Doppler-Test ist am SPDF nicht baubar; der Roh-Doppler-Weg
(PDS/JPL) ist Atom 1b.

**Atom 1 verifiziert (negativ).** Atom 2 (Fenster-n messen) hängt am
Zugang, nicht am SPDF.

## Atom 1b — PDS/JPL-Verifikation (Befund 2026-09-04, agent gemessen)

**Auch am PDS gibt es keinen Cruise-Doppler.** Der einzige rohe
Voyager-RSS-Bestand im PDS (RMS-Knoten, `pds-rings.seti.org`) umfasst
messbar drei Bündel — VG1-Jupiter, VG2-Jupiter, VG2-Uranus-49XR — und
trägt laut `bundle_readme.txt` **open-loop-Okkultationsaufnahmen** der
Encounter (ODR/REDR, 1979/1986), **kein kohärentes Closed-Loop-Doppler-
Tracking** (kein ODF/TDF/TRK-2-34), kein Saturn, keine Cruise.

- PDS RMS `voyager_rss_raw` — HTTP 200, `open`, aber nur Encounter-
  Okkultation (Jupiter/Uranus); das Archiv-Tar (`archives-bundles/`) ist
  404, Einzeldateien ladbar.
- NAIF SPK (`vgr1.*.bsp`) — rekonstruierte Bahn, kein ODF/TDF.
- DSN/JPL ODF/TDF (TRK-2-34) — kein offener Endpunkt gemessen;
  `request-only`, `pending`.
- Zenodo — keine Voyager-Doppler-Datensätze.

**Fenster-Frage 1998–2002 beantwortet: nein.** Keine gemessene offene
Quelle deckt das ruhige Fenster in Doppler ab. Offen liegen nur die
rekonstruierte Bahn (SPICE/Horizons, Doppler-gefitet, Residuen nicht
offen) und die Encounter-Okkultation 1979/1986. Der Cruise-Doppler liegt
nicht im PDS, nicht am SPDF/NSSDCA, nicht bei NAIF, nicht in Depots.

**Folge:** Der Co-Quiet-Doppler-Test (Tür 1) ist am Roh-Doppler-Zugang
blockiert — er bräuchte eine JPL/DSN-Anfrage (ODF/TDF des Fensters,
`request-only`, unverifiziert ob intern vorhanden). Die Zone selbst ist
aus `merged/` (HGI-Distanz, Stunden-Mittel) definierbar — eine Zonen-
Geometrie ohne Doppler-Signal ist aber keine Co-Quiet-Messung.
Eigenhändig verifiziert (Befund
`docs/befund/befund-voyager-roh-doppler-zugang.md`): keine offene Quelle
deckt das Fenster; die kausale Erklärung ist die Dreiachsenstabilisierung
Voyagers — ihr Lageregelungsrauschen (~10⁻⁶ cm/s²) liegt ~10× über der
gesuchten Effektgröße, nie ein Bergungsanreiz wie bei Pioneer.

## Der Vorfilter — die Lehre aus Tür 1 (2026-09-04, gemessen)

Tür 1 ist geschlossen — nicht am Datenzugang allein, sondern an einer
Vorbedingung des Rezepts, die Voyager gemessen hat: das Rauschen muss eine
**Distanz-Geometrie** haben (medium-getrieben), damit die Zonen-Teilung es
trennt. Voyagers dominantes Restrauschen ist **Selbst-Rauschen** der
dreiachsen-stabilisierten Lageregelung (Düsen-ΔV ~10⁻⁶ cm/s², kein
Abstandsgang, ~10× über a_P) — keine Zone trennt es, kein Harvest heilt es.
Befund: `docs/befund/befund-voyager-roh-doppler-zugang.md`.

**Vorfilter je Tür (vor jedem Harvest, 0 honored):**

1. **Stabilisierungs-Schema:** spinstabilisiert (Pioneer-artig, passive
   Lage) oder dreiachsenstabilisiert (aktive Düsen →
   Selbst-Rauschen-Kandidat)?
2. **Dominante Rauschquelle:** medium-getrieben (Plasma, Distanz-Geometrie,
   Zonen-Teilung greift) oder selbst-getrieben (Raumschiff-Dynamik,
   Zonen-Teilung blind)?
3. **Selbst-Rauschen über der Anomalie-Größenordnung** (a_P ≈ 10⁻⁷ cm/s²)
   → die Tür fällt ohne Harvest (Voyager-Urteil, nicht wiederholen).

Tür 2 (New Horizons) und Tür 4 (Mariner/Galileo/Cassini) tragen den
Vorfilter offen — Recherche-Auftrag:
`docs/auftrag/auftrag-quiet-zone-vorfilter.md`.

## NH-Daten-Anfrage (registriert 2026-09-04)

Daten-Anfrage an ein Archiv (Bestandsabfrage, kein Kontaktversuch zu
Befunden): New-Horizons-Nav-Doppler (ODF/TRK-2-34) für die
Tracking-Rausch-Charakterisierung >50 AU (stille Cruise-/Hibernations-
Pässe 2016–2024).

- **An:** PDS Radio Science Subnode (Deep-Space-Tracking-Archive) bzw.
  PDS Geosciences Node — Adressen befund-dokumentiert
  (`docs/befund/befund-voyager-roh-doppler-zugang.md`, Prüfschritt 5:
  `radiosci@att.net` / `geosci@wunder.wustl.edu`); **Erreichbarkeit beim
  Versand zu prüfen** (Adressen von 2026-09-04, `pending`).
- **Anliegen:** (1) sind NH-Nav-Doppler via PDS verfügbar oder nur per
  Anfrage; (2) falls Anfrage — Prozedere und Format; (3) gibt es eine
  Pilot-Epoche, die sofort verfügbar wäre.
- **Referenz (Autorität des Feldes):** Iess et al., Radio Science 2004,
  DOI 10.1029/2004RS003101 (Plasma-Kontamination in S/X-Band-Tracking).
- **Mail-Text:** formuliert (siehe Session; zehn Zeilen + Iess-DOI +
  Pilot-Epochen-Frage, Signatur „independent researcher").
- **Status:** `pending` — Versand ist ein menschlicher Akt (Datum +
  Adresse beim Versand eintragen), Antwort `pending`, nie 0.0.

## Lehre aus Schritt 3 (Galileo-Bestandsaufnahme, 2026-09-05)

Der Vorbefund trug eine falsche Adresse (`gll.rss` existiert nicht; real =
PDS3 `GO-…-RSS-…-V1.0` am PPI-Knoten — Befund
`docs/befund/befund-galileo-gwe-bestand.md`). Lehre, bindend für alle noch
offenen Türen: **Register-Behauptungen über externe Archive sind Adressen,
keine Orte — `lookup vor harvest`.** Jede künftige `open`-Aussage über einen
Datenbestand ist `pending` bis zum Registry-Lookup. Für die NH-Anfrage heißt
das: verifizierte Bestandsbezeichnungen zitieren, nicht die erfundene
Bundle-ID (eine Anfrage mit falscher ID kommt bei JPL nicht an). Für Tür 1
(V1) ist die Frage nicht mehr „gibt es die Daten?", sondern „verifiziere die
Adresse, bevor du an die Tür gehst".

**Nebenfund mit eigenem Wert — der GWE-ODR-Banden-Test:** `GO-X-RSS-1-ODR-V1.0`
(open-loop, 1994/95) lief auf **DSS 14/43/63 — denselben drei Stationen wie
die 20-s-Bande.** Open-loop ohne Sonden-Doppler-Verwirrung, Ära überlappend
mit Pioneer (1987–93). Wenn die 20-s-Bande ein DSN-Ketten-Erbe ist, müsste
sie im GWE-ODR-Material auftauchen — potenziell die stärkste externe
Validierung der Banden-Hypothese (Klasse-2-Item des `auftrag-bande-split.md`,
jetzt mit einer konkreten, offenen Quelle).

## Register-Satz

*Die Quiet-Zone ist kein Pioneer-Ergebnis — sie ist ein Rezept: Rauschen
verorten, Zone isolieren, Boden messen. Jede Sonde, die je weit draußen war,
wird damit neu lesbar. (Die laute Zone als Instrument ist belegt — Woo &
Armstrong 1979; die Kombination mit der Zonen-Teilung ist es nicht.) Die
Methode überlebt die Stille des Objekts, für das sie erfunden wurde.*

## Status

`pending`. Tür 1 (Voyager) ist geschlossen — eigenhändig verifiziert
(Befund `docs/befund/befund-voyager-roh-doppler-zugang.md`): kein offener
Cruise-Doppler, und die kausale Grenze ist das dreiachsen-stabilisierte
Selbst-Rauschen (~10⁻⁶ cm/s², ~10× über a_P) — keine Zone trennt es.
Die vier Lizenz-gesperrten Volltexte bleiben pending (born-digital, kein
Vision-LLM öffnet sie). Vorfilter ausgeführt (2026-09-04,
`auftrag-quiet-zone-vorfilter.md`): New Horizons `besteht` und trägt den
nächsten Harvest — aber `request-only` (JPL/DSN-ODF-Anfrage); Galileo-GWE
steht als offene Reserve; Cassini/Mariner `fallen`. Nächste Hebel:
NH-Doppler-Anfrage formulieren (Galileo-GWE-Ernte geprüft 2026-09-05 —
`docs/befund/befund-galileo-gwe-bestand.md`: `gll.rss` existiert nicht,
real = PDS3 `GO-…-RSS-…-V1.0`, eigene relativ-ruhige ≤5-AU-Achse statt
Quiet-Zone-Nachbau).
