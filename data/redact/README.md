# data/redact — Recall-Schicht: Quellen, Lizenzen, Begrenzung

Eine Zeile je Datei: Datei — Quelle-URL — Lizenz.

vornamen.txt — Wikidata, P31 Q12308941 (maennlicher Vorname) + Q11879590 (weiblicher Vorname), rdfs:label@de — https://query.wikidata.org/sparql — CC0 1.0
nachnamen.txt — Wikidata, P31 Q101352 (Familienname), rdfs:label@de; gemessener Schnitt: de.wikipedia-Artikel (schema:about) + de-benannte Teilantwort — https://query.wikidata.org/sparql — CC0 1.0
plz.txt — GeoNames Postal Code Data, allCountries.zip — https://download.geonames.org/export/zip/ — CC BY 4.0
patterns.md — validator.js v13.15.0, Regex-Referenz isIBAN/isBIC/isCreditCard/isVAT/isPostalCode/isMobilePhone/isPassportNumber/isTaxID — https://github.com/validatorjs/validator.js — MIT

Begrenzung sichtbar, nicht still: die vollen Wikidata-P31-Mengen sind 60907 maennliche Vornamen, 39091 weibliche Vornamen, 699967 Familiennamen.
Aufgenommen ist die de-benannte Teilmenge: 14615 eindeutige Vornamen-Labels, 9251 eindeutige Nachnamen-Labels.
Die volle de-Label-Abfrage der Familiennamen lief am 2026-10-15 in ein Upstream-Timeout der WDQS; der gemessene Schnitt ist die de.wikipedia-notable Menge 7103 vereinigt mit der ersten de-Label-Teilantwort 3725.
