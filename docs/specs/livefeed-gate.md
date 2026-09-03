<!--
  title: Das Livefeed-Korrelations-Gate
  class: concept
  date: 2026-08-27
  status: live
  see-also: docs/granit.md tools/gate/src/bin/livefeed_gate.rs
-->

# Das Livefeed-Korrelations-Gate

Die Schnittstelle an Presseagenturen und Newsquellen. Der Nachrichtenfluss ist
ein Messfluss — er trägt dieselben Axiome wie jede Feldmessung. Das Gate nimmt
ein Nachrichtenereignis an, prüft es gegen die omegaflow-Axiome und erzeugt
daraus einen Untersuchungsauftrag `<ereignis>-untersuchen` als AUFTRAG.md.

## Die Rolle

Eine Agentur ist kein Erzähler, sondern ein Messinstrument. Die Meldung ist
eine Messung. Wie jede Messung trägt ein Ereignis die fünf Axiome aus
`granit.md`:

1. **A = A** — die Zahl, die das Ereignis trägt, ist die Zahl aus der
   gemessenen Quelle; keine Zahl wird erfunden.
2. **ICRS & TDB** — der Ort ist eine Raumzeit-Adresse: Zeit als JD TDB wo
   möglich, Ort als ICRS/J2000 oder geo.
3. **force_type** — jedes Ereignis trägt seine physische Kraft; ohne Kraft
   kein Sample.
4. **0 honored** — was fehlt, fehlt; die Lücke ist eine vollwertige Eigenschaft.
5. **pending** — was noch nicht gemessen ist, bleibt pending.

## Die Felder eines Ereignisses

| Feld | Bedeutung | fehlt |
|------|-----------|-------|
| quelle | Name der gemessenen Quelle | pending |
| quelle-url | Adresse der Quelle | pending |
| zeit | bürgerliche Zeit der Meldung | pending |
| zeit (JD TDB) | Julianisches Datum (TDB≈UTC, Delta pending) | pending |
| ort | Ort als Text (ICRS/J2000 oder geo) | pending |
| ort (geo) | gemessene Koordinaten lat/lon | pending |
| ereignis-art | was geschehen ist | pending |
| force_type | physische Kraft (gravitation, elastizitaet, aerodynamik, thermodynamik, elektromagnetisch) | pending |
| zahlen | Zahl `<-` quelle-der-zahl | pending |

Fehlende Felder werden als `pending` geführt, nie erfunden (0 honored).

## Bedienung

Das Werkzeug `tools/work/src/bin/livefeed_gate.rs`:

```
livefeed_gate --titel "<t>" --meldung "<s>" [Felder...]
livefeed_gate --rss <feed-url> --titel-filter <term>   # Meldung aus dem Feed
livefeed_gate --url <url>                              # Seitentitel als Meldung
livefeed_gate --news <suchbegriff>                     # Google-News-Weltsuche (RSS)
livefeed_gate --wikidata <ereignis>                    # Faktenabgleich Wikipedia + Wikidata
livefeed_gate --eonet <kategorie>                      # NASA-EONET-Naturereignis (geo+zeit+zahl)
livefeed_gate --quellen <url>...                       # Quellen-Availability (mehrere)
livefeed_gate --gate <gate-url>                        # Verdikt des omegaflow-Gate
livefeed_gate --verify <url>                           # Quellen-Availability (eine)
livefeed_gate --top                                    # Verifizierbarkeits-Checkliste
livefeed_gate --fixity                                 # SHA-256-Fixität der Quelle
```

Eingabe kann auch über stdin kommen. Die Axiome liest das Werkzeug zur
Laufzeit aus `granit.md` (dieselbe Datei, die auch `llm_interceptor` liest).

## Weltweite Quellen (gemessen 2026-08-27)

Zusätzlich zu Wikipedia/Wikidata und der Deutschen Welle:

- **NASA EONET** (`--eonet <kategorie>`) — das einzige offene strukturierte
  Naturereignis-Register ohne Key: geo (`coordinates [lon,lat]`), ISO-Datum,
  Magnitude (acres/kts). Kategorien: floods, earthquakes, landslides,
  severeStorms, wildfires, volcanoes u. a. Gemessen: EONET führt je Kategorie
  nur *aktive* Ereignisse — eine leere Kategorie ist eine Messung, kein
  Fehler (0 honored). force_type wird aus der Kategorie abgeleitet.
- **Google News RSS** (`--news <suchbegriff>`) — weltweite, mehrsprachige
  Suche, frei ohne Key. Text-Korrelationsschicht (keine geo/Struktur).
- **GDELT** (bulk) — frei, maschinenlesbar, trägt Inhalts-Hashes (Fixität);
  ohne explizite lat/lon (nur Länder-/Akteur-Codes).
- **Guardian Open Platform** — freies JSON (api-key=test), ISO-Zeitstempel,
  ohne geo.
- **NYT / BBC / Al Jazeera / France24 RSS** — frei, zuverlässiges `<pubDate>`,
  ohne Koordinaten/Zahlen (Text-Korrelation, nie Zahlen-Messung).
- **Reuters (feeds) / dpa.de**: nicht offen (pending, gemessen).

## Fixität der Quelle (--fixity)

`--fixity` berechnet den SHA-256-Hash des geholten Quellenobjekts
(quelle-url) und führt ihn als `quelle-fixity: sha256:…`. Damit ist A = A
operationalisiert (FORCE11 P7): die Zahl ist die Zahl *dieses exakten
Schnappschusses*; spätere Änderung oder Link-Rot sind erkennbar. SHA-256 ist
im Werkzeug in reinem std implementiert — keine Zusatz-Abhängigkeit.

## Die Ort-Adresse: geo und ICRS/J2000

Ein Ereignis trägt eine Raumzeit-Adresse. Zwei Formen werden gemessen:

- **geo** — `--ort-lat <lat> --ort-lon <lon>` oder aus dem Faktenabgleich
  (Wikidata P625, Nominatim).
- **ICRS/J2000** — `--icrs-ra <h> --icrs-dec <deg>` (Rektaszension in
  Stunden, Deklination in Grad, J2000). Für Himmelskörper-Ereignisse die
  natürliche Adresse; für terrestrische bleibt sie `pending`, wenn nicht
  gemessen — es wird keine Umrechnung erfunden.
- **geo → ICRS geozentrisch** — `--icrs-aus-geo lat lon 'ISO-UTC'` legt
  einen terrestrischen Punkt als geozentrischen ICRS-Vektor (km) zur Zeit
  JD TDB ab (WGS84 + GMST-Erdrotation). Das Prezession/Nutations-Delta
  (~0.35° zum J2000-ICRS) bleibt pending.

## Satelliten-Altimetrie-Wasserstand (--dahiti)

Für den Abfluss-/See-Spiegel-Response greift das Gate auf die DAHITI
(TU München, Satelliten-Altimetrie) zurück: `--dahiti <dahiti_id> --api-key
<key>` lädt die Wasserstands-Reihe (`wse`, m) eines virtuellen Standorts
und schreibt sie als `wse_<id>.csv` (datetime wse) — co-lagbar mit
`te_pair_probe`. Der Key wird nach Projektkonvention aus `.secrets.local`
gelesen (Zeile `DAHITI_API_KEY=…`), sonst `--api-key`/env `DAHITI_API_KEY`.

**Grenze (0 honored), 2026-08-27 gemessen:** Koshi-Stationen (25.4–26.4°N,
~300 km unterhalb des Trishuli-Ereignisses bei 28.28°N) sind beckenweit,
nicht co-lokal — und **keine** erreicht das Ereignisdatum (3409 → 2026-07-08,
15694 → 2026-08-04, 8480 tot 2016). Altimetrie-Reihen sind irregular; die
jüngsten Punkte liegen Wochen **vor** der Flut. Bei Niederschlag
(2026-08-18…27) ist die Überlappung null → für **dieses** Ereignis ist der
Koshi-Wasserstand kein co-lagbarer Response. Das Gate sagt das dazu, statt
es zu verschweigen.

**Key-Weg (kostenlos):**
1. Registrieren: https://dahiti.dgfi.tum.de/en/register/ (freie Altimetrie-Daten).
2. Zeile `DAHITI_API_KEY=<key>` in `.secrets.local` (Repo-Root, gitignored)
   eintragen — das Gate liest sie automatisch.
3. `--dahiti 15694` (Koshi-Station) — schreibt `wse_15694.csv`.
4. `te_pair_probe --a precip_rasuwa.csv --b wse_15694.csv ...` — beachte die
   Zeitgrenze (Series endet vor dem Ereignis; siehe oben).

## Zeit: JD TDB mit gemessenem Delta

Die Zeit wird als JD geführt und nach TT (Terrestrische Zeit) gebracht:
TT−UTC = 69.184 s (TAI−UTC = 37 s laut IERS-Schalttabelle seit 2017-01-01,
TT = TAI + 32.184 s). Der TDB−TT-Term ist sub-millisekundig (< 2 ms) und
bleibt als `pending` geführt. Der JD druckt auf 6 Dezimalen, wo der Term
nicht mehr erscheint.

## Der Faktenabgleich (--wikidata)

`--wikidata <ereignis>` misst aus Wikipedia **und** Wikidata (en):
- Wikipedia: exakter Titel → Einleitung (Intro), Datum, Koordinaten.
- Wikidata: `wbgetentities` des Wikibase-Items (`pageprops` → `wikibase_item`)
  → **P625** Koordinate, **P585** Zeitpunkt, **P1120** Zahl der Toten.

Die strukturierten Werte tragen ihre eigene Quelle (z. B. „292 Tote
(P1120) <- wikidata.org/wiki/Q…"). Was nicht getragen wird, bleibt pending.
Werte können zwischen Intro und strukturierten Claims abweichen (z. B. 359
Tote laut Intro, 292 laut P1120) — das Gate führt beide mit ihrer Quelle,
nie eine stillschweigend zusammengezogene Zahl.

## Die Verifizierbarkeits-Checkliste (--top)

`--top` belegt die Ereignis-Messung (TOP-Datenverfügbarkeit, Nature/arXiv-
Zitierweise): jede Zahl trägt eine Quelle, die Quelle ist benannt und
gemessen, zeit/ort/kraft verortet, keine selektive Berichterstattung, der
Auftrag aus der Aufrufzeile reproduzierbar. Jeder Punkt ist `ja` oder
`pending` — nie behauptet.

## Das Gate als zweite Prüfung

Mit `--gate http://127.0.0.1:4100` sendet das Werkzeug die Ereignis-Meldung
durch das laufende omegaflow-Gate (Provider `omegaflow`, chat/completions).
Das Gate stellt den Granit an die erste Stelle und hält verletzende Passagen
still zurück; das Werkzeug trägt den Verdikt in die Ereignis-Karte.

## Verdiktswoerter

Die Ereignis-Karte spricht in gemessener Sprache. Wo die Quelle nicht reicht,
schreibt sie `pending` — nicht `failed`, nicht `error`, nicht `expected`.
