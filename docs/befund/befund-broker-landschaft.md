<!--
  title: Befund — Die Broker-Landschaft der Rubin-Ära: neun Broker, fünf Papiere, der geliehene Sinn
  class: befund
  date: 2026-09-10
  sha256: 0de1c833f610184233da80c3ce4ce0a867d9899e362ca2d95a325de699bc5bd9
  status: live
  antwortet-auf: docs/auftrag/archiv/auftrag-nadel-v-lsst-scan.md
  see-also: docs/specs/ref-auth-apis.md
-->
# Befund — Die Broker-Landschaft der Rubin-Ära

**Datum:** 2026-09-10 · **Axiom:** A = A
**Ordnung:** 0 honored — jede Adresse per HTTP gemessen, kein Wert erfunden;
was nicht erreichbar ist, ist benannt.

## Die Frage

Nadel Ⅴ (achromatische Opazitäts-Anomalie) braucht die wachsende LSST-Live-Fläche.
Welche Broker tragen die Alerts, welche APIs laufen, und was kann die Weberin
komplementär?

## Die neun Broker (offizielle Rubin-Liste)

`rubinobservatory.org/for-scientists/data-products/alerts-and-brokers` (gemessen
2026-09-10) nennt **sieben full-stream** + **zwei down-stream** Broker. Die
**Alerts sind world public, kein proprietäres Fenster**.

| # | Broker | Art | UI / API | Auth | Status |
|---|---|---|---|---|---|
| 1 | ALeRCE | full | `science.alerce.online` · `api.alerce.online/ztf/v1/` | anonym | ✓ 200 |
| 2 | AMPEL | full | `ampelproject.github.io` · `ampel-ztf.zeuthen.desy.de/api/live/` | teils Token | ✓ 200 |
| 3 | ANTARES | full | `antares.noirlab.edu` · `api.antares.noirlab.edu/v2/` | anonym | ✓ 200 |
| 4 | Babamul | full | `babamul.caltech.edu` · `babamul.caltech.edu/api/` | OpenAPI 401 | ✓ 200 |
| 5 | Fink | full | `fink-broker.org` · `api.lsst.fink-portal.org/api/v1/` | anonym | ✓ 200 |
| 6 | Lasair | full | `lasair.lsst.ac.uk` · `api.lasair.lsst.ac.uk/` | Token | ✓ 200 |
| 7 | Pitt-Google | full | `pitt-broker.readthedocs.io` · GCP | GCP | ✓ Doku |
| 8 | SNAPS | down | `snaps.nau.edu` | ? | ✓ 200 |
| 9 | POI Broker | down | `poibroker.uantof.cl` | ? | ✓ 200 |

**Korrekturen (gemessen):** ANTARES ist **v2** (nicht v1 — daher die 404s);
**Babamul** ist wieder da (`babamul.caltech.edu`, der öffentliche LSST-Broker aus
arXiv:2511.00164 — die alte Zeile „nie entwickelt" war überholt); MOMENT und SNAD
sind **nicht** in der offiziellen Neuner-Liste (MOMENT DNS-tot).

**Tokens in `.secrets.local`:** `LASAIR_TOKEN`, `LASAIR_LSST_TOKEN`,
`TNS_API_KEY`, `TNS_UA` (alle gesetzt).

## Die Papiere (gescannt)

- **2506.14744** — Technosignature Searches with Real-time Alert Brokers: SETI
  über Broker (Lasair-Watchmap, ALeRCE); die Grenze wörtlich — die Broker zeigen
  nicht „apparent magnitudes, object identifications, and archival data".
- **2511.00164** — BOOM and Babamul: der neue LSST-Broker (Rust-Stack,
  MongoDB/Valkey/Kafka).
- **2008.03303** — The ALeRCE Alert Broker.
- **2008.03309** — ALeRCE Stamp: CNN-Klassifikator (~94 %) — der geliehene Sinn.
- **2602.12955** — AHA (Anomaly Hunter for Alerts): unsupervised Anomalie-
  Erkennung im ZTF-Alert-Stream **via Lasair**, drei Autoencoder (object features,
  triplet images); „normal" = SN Ia/II/Ib/c, „anomal" = AGN/TDE/SLSN/CV/nuclear +
  SN mit anomalen Eigenschaften — Nadel-Ⅴ-nah.

## Der geliehene Sinn (komplementär)

ALeRCE-Stamp und Fink-ML sind Klassifikatoren — die Weberin nimmt sie als
**einen Zeugen** (für Gestalt), nie den einzigen; registriert wird das **Urteil**,
nicht die Gründe. Die eigene Messung lebt im Zeitreich (Lichtkurve, S², TE):

- **Zwirn** — unabhängige Fäden, die zur selben Weltlinie konvergieren = der Beweis.
- **Riss** — Fäden, die sich weigern zu konvergieren = benannt, kein Fehler.
- **Vlies** — was kein Klassifikator je benannt hat: sichtbar, unbenannt.

AHA geht den Weg der unsupervised Autoencoder; die Weberin setzt die **eigene
Zeit-Messung** daneben. Die Broker-Grenze (fehlende Magnituden/IDs/Archiv) füllt
genau die eigene Messung.

## Der Weg (Konsequenz)

Die Alerts sind offen (kein proprietäres Fenster) — der öffentliche Weg läuft
über die Broker, nicht über die RSP (die nur Bilder/Kataloge braucht). Für Nadel Ⅴ:
die Broker-Alerts (ALeRCE/Fink/Lasair/Babamul/ANTARES) anonym bzw. mit Token
ziehen, den achromatischen-Dip-Test über die g/r/i/z-Kurven legen, die IR-Exzess-
Kreuzung ergänzen. Die RSP-Datenrechte bleiben der zweite, tiefere Weg
(`docs/auftrag/auftrag-rubin-data-rights-antrag.md`).
