<!--
  title: Survey — Phantom Island im Williston Reservoir (BC), NYT 2026-08-31
  class: survey
  date: 2026-10-21
  sha256: 9d93fd72e79f065d54469d53d695ee9398c633bca411799ee672329acce23614
  status: live
  owner: river
  ack: river 2026-10-21T10:12Z
-->
# Survey — Phantom Island im Williston Reservoir (BC)

## Gegenstand

Eine bewachsene Insel erschien im Williston Reservoir (W.A.C.-Bennett-Damm,
Northern Interior, British Columbia), verschwand und wurde am Ospika Arm
wieder gefunden. Die NYT meldete den Fall am **2026-08-31**. Alte Order:
`projects/archiv/auftrag-2026-09-05-phantom-island-williston-bc.md` (Tag
`archiv-2026-10-20`). Die vier pending Felder werden hier belegt oder als
`pending` mit Grund benannt; kein Feld bleibt leer.

## Die vier pending Felder

| Feld | Wert oder pending | Beleg (URL/HTTP/Zeit) |
|---|---|---|
| quelle-url | `https://www.nytimes.com/2026/08/31/world/canada/missing-island-canada-williston-reservoir.html` | NYT-oEmbed `https://www.nytimes.com/svc/oembed/json/?url=…` HTTP **200**, abgerufen 2026-10-21T10:08Z: `url`, `title`, `publication_date = August 31, 2026`, Autor Adeel Hassan. Artikel direkt HTTP **403** (DataDome, Bot-Schutz), kein inhaltlicher Abruf. |
| ort | Williston Reservoir / Williston Lake, **Ospika Arm, Nord-Eingang an der Ospika-River-Mündung**, ca. 130 km nordwestlich des W.A.C.-Bennett-Dam, nördlich von Mackenzie, British Columbia. Näherung: **56.25° N, 124.07° W** (OSM-Punkt der Ospika-River-Mündung, *nicht* Inselzentrum). | CBC `https://www.cbc.ca/news/canada/british-columbia/island-found-williston-lake-bc-hydro-9.7327399` HTTP **200**, 2026-10-21T10:10Z: Satellit 2026-08-15, „north entrance of the Ospika Arm, where the Ospika River flows into the reservoir“, 30 km NW der letzten Position. CBC `…/floating-island-williston-lake-bc-9.7324586` HTTP **200**: „west of Finlay Bay, about 100 kilometres west of the W.A.C. Bennett Dam“. OSM/Nominatim „Ospika River“ HTTP **200**, 2026-10-21T10:11Z: 56.2479894, −124.0736273 (Flussmündung). |
| zeit | **Meldung 2026-08-31**; Ereignis: Erstbeobachtung **2026-07-21**, Verschwinden bis **2026-08-05**, Wiederfund-Satellit **2026-08-15**, Bestätigung **2026-08-31**. | NYT-oEmbed HTTP **200** (`publication_date: August 31, 2026`); Google-News-RSS HTTP **200**, 2026-10-21T10:07Z, `pubDate = Mon, 31 Aug 2026 07:00:00 GMT` (nur Finder). CBC `…/floating-island-williston-lake-bc-9.7324586` HTTP **200**: Satellit 21.07., weg ab 05.08.; CBC `…/island-found-williston-lake-bc-hydro-9.7327399` HTTP **200**: Satellit 15.08., Fund bestätigt 31.08.; Maple Ridge News `https://mapleridgenews.com/2026/09/02/mystery-of-missing-island-near-mackenzie-solved-bc-hydro-says/` HTTP **200**: BC Hydro informiert 13.08., Foto ~21.07., „solved on Monday (Aug. 31)“. |
| ereignis-art | **Treibende, bewachsene Torf-/Totholzinsel** (floating forested raft), ca. **70 × 140 m**, ca. **30 km Drift nach Nordwesten**, am Ufer des Ospika Arm gestrandet. Kein kartografisches Phantom. | CBC `…/island-found-williston-lake-bc-hydro-9.7327399` HTTP **200**: „forested land mass … landed ashore after drifting some 30 kilometres northwest“, „intact“, 70 × 140 m. CBC `…/floating-island-williston-lake-bc-9.7324586` HTTP **200**: Entstehung aus Treibholz/Totholz, durch Rekordwasserstand freigesetzt, windgetrieben; „not showing up in satellite data“ ab 05.08. Maple Ridge News HTTP **200**: „floating island … come to rest against the shoreline on the Ospika Arm“. |

## Quellenweg und Grenzen

- **Weiterleitung ist keine Quelle.** Der Google-News-RSS-Link
  (`news.google.com/rss/articles/…`) wurde nur als Finder benutzt und über
  `batchexecute` zur kanonischen NYT-URL aufgelöst. In der Tabelle steht die
  NYT-URL, nicht die Weiterleitung.
- **NYT direkt HTTP 403** (DataDome). Die Metadaten (URL, Titel, Datum, Autor)
  sind über den NYT-oEmbed-Endpunkt HTTP 200 belegt; der Artikeltext selbst
  wurde nicht gelesen. Keine Textbehauptung stützt sich auf ungeprüften
  NYT-Inhalt.
- **Exakte Inselkoordinaten bleiben `pending`.** Die accessible Belege nennen
  den Ospika Arm und 30 km NW, aber keinen Koordinatensatz des Inselzentrums.
  Die 56.25/124.07 sind die OSM-Mündung des Ospika River, ausdrücklich eine
  Näherung für den Ort, nicht die Insel.
- **Alter Befund nicht im Git.** `docs/befund/befund-2026-09-09-phantom-island-williston-bc.md`
  ist in keinem der 326 Commits/Refs vorhanden (Pfad nie im Baum); erhalten ist
  nur die Zusammenfassung in
  `handover/archiv/handover-2026-09-09-mechanische-reste.md` (Tag
  `archiv-2026-10-20`). Deren Aussage („echtes, treibendes Torf-/Holz-Floß,
  70×140 m, 08-15 Ospika-Arm-Nordeingang“) deckt sich mit CBC/Maple Ridge.
- Nicht gemessen und nicht Teil dieses Artefakts: `force_type`, ICRS/J2000,
  TDB-Umrechnung. Sie bleiben außerhalb der vier hier verlangten Felder offen.
