# data/redact — Recall-Schicht: Quellen, Lizenzen, Begrenzung

Eine Zeile je Quelle: Menge — Weg — Lizenz.

vornamen.txt — Wikidata, P31 Q12308941 (maennlich) + Q11879590 (weiblich), rdfs:label@de — https://query.wikidata.org/sparql — CC0 1.0
plz.txt — GeoNames Postal Code Data, allCountries.zip — https://download.geonames.org/export/zip/ — CC BY 4.0
patterns.md — validator.js v13.15.0, Regex-Referenz — https://github.com/validatorjs/validator.js — MIT

nachnamen.txt — Union, sortiert + case-insensitiv eindeutig — 1293653 Namen. Quellen:
- Wikidata Familienname P31 Q101352, rdfs:label@de via Label-Service, seichte Offsets 0/1000/2000/4000 — https://query.wikidata.org/sparql — CC0 1.0 — 4000
- de.wikipedia Kategorie:Familienname, MediaWiki-API categorymembers + cmcontinue, 16 Seiten — https://de.wikipedia.org/w/api.php — CC BY-SA 4.0 — 6510 (aus 7928 Titeln bereinigt)
- US Census 2010 Surnames (Public Domain), ueber GitHub-Spiegel fivethirtyeight/data — https://www2.census.gov/topics/genealogy/2010surnames/names.zip — Public Domain — 151670
- HuggingFace FabianToSpace/lastnames, Name-Feld — https://huggingface.co/datasets/FabianToSpace/lastnames — MIT — 79514
- HuggingFace eltorio/french_last_names_insee_2024, lastname (INSEE-Sterberegister) — https://huggingface.co/datasets/eltorio/french_last_names_insee_2024 — MIT — 607829
- HuggingFace rustemgareev/russian-surnames, surname_cyrl + surname_latn — https://huggingface.co/datasets/rustemgareev/russian-surnames — CC BY-SA 4.0 — 636958
- HuggingFace Umkho-AI/South_African_Popular_Surnames, surname-Spalte — https://huggingface.co/datasets/Umkho-AI/South_African_Popular_Surnames — CC BY-SA 4.0 — 651
- GitHub ndsvw/JSON-Namen, nachnamen.json — https://github.com/ndsvw/JSON-Namen — MIT — 2000
- Bestand vor der Union (Wikidata-de-Labels, de.wikipedia-notable) — 9251

Begrenzung sichtbar, nicht still:
- Wikidata: die volle Familiennamen-Menge ist 699968 (P31 Q101352, gezaehlt). Die de-Label-Join-Zaehlung und tiefe Offsets (>=5000) liefen in den WDQS-Timeout (HTTP 504); aufgenommen sind nur die seichten Label-Service-Seiten = 4000.
- NavidVafaei/surnames (10980) NICHT aufgenommen: keine Lizenz deklariert.
- mikeymo-Datasets NICHT aufgenommen: Vor- und Nachnamen gemischt, nicht trennbar.
- SA-CSV: nur einteilige Namen uebernommen (651).
- Normalisierung: All-Caps -> Titelform, sonst erste Buchstabe gross; Dedupe case-insensitiv; ohne Komma/Anfuehrungszeichen, max 60 Zeichen; mehrteilige Namen erhalten.
