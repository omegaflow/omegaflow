<!--
  title: Broker-Differenz an ZTF20acigmel — Fink und ALeRCE sehen das Objekt, Lasair verweigert (2026-10-21)
  class: survey
  date: 2026-10-21
  sha256: 70fe7c8240c5b62a729ca4b9e43569ed751f5aea6e9fc596d88246da8bc8cd7d
  status: live
  owner: mountain
  ack: mountain 2026-10-21T10:09:26Z
-->
# Broker-Differenz an ZTF20acigmel — Fink und ALeRCE sehen das Objekt, Lasair verweigert (2026-10-21)

**Frage.** Sehen die drei unabhängigen Broker Fink, Lasair und ALeRCE dasselbe
Objekt — und antworten sie überhaupt? Funke 3 der Anomalie-Suche fragt nicht
nach einem Kegel, sondern nach einem Objekt: `ZTF20acigmel`.

**Grundlage.** Live-Lauf des Probe-Binaries
`broker_difference_probe ZTF20acigmel` am 2026-10-21T10:09:26Z. Routen aus den
bestehenden `archive_search`-Armen: Fink `/api/v1/objects`, Lasair
`/api/object`, ALeRCE `/ztf/v1/objects`; keine neuen Keys. Ein Treffer zählt
nur, wenn eine Zeile die angefragte Objekt-ID exakt trägt. Nicht-200 ist keine
Abwesenheit.

## Kanäle (gemessen, nicht angenommen)

- **Fink**: HTTP **200**, **2** Alert-Zeilen mit `i:objectId = ZTF20acigmel`,
  letzte Detektion **MJD 59136.2137** (aus `i:jd` 2459136.7137 umgerechnet).
- **Lasair**: HTTP **401** — `Authentication credentials were not provided.`
  `LASAIR_TOKEN` fehlt in Umgebung und `.secrets.local`. Das ist ein Befund
  über den Zugang, keine Aussage „Objekt nicht vorhanden".
- **ALeRCE**: HTTP **200**, **1** Objekt in `items`, `oid = ZTF20acigmel`,
  `lastmjd = 59136.213703699876`.
- Abfragezeit für alle drei: **2026-10-21T10:09:26Z**. Kein Netz-Abuse:
  je Broker genau ein Request.

## Tabelle

| Broker | Status | Treffer | Objekt-ID |
|---|---:|---:|---|
| fink | 200 | 2 | ZTF20acigmel |
| lasair | 401 | 0 | — |
| alerce | 200 | 1 | ZTF20acigmel |

## Zeitstempel

| Broker | Abfrage (UTC) | letzte Detektion |
|---|---|---|
| fink | 2026-10-21T10:09:26Z | MJD 59136.2137 |
| lasair | 2026-10-21T10:09:26Z | — (HTTP 401, kein Read) |
| alerce | 2026-10-21T10:09:26Z | MJD 59136.2137 |

## Bilanz

| | Zahl |
|---|---:|
| Broker angefragt | **3** |
| Broker mit HTTP-Antwort | **3** |
| Broker mit sauberem Treffer | **2** (fink, alerce) |
| Broker verweigert (401/403) | **1** (lasair) |
| Broker ohne Antwort | **0** |
| erfundene Treffer | **0** |

Zwei unabhängige Broker tragen das Objekt; die Broker-Grenze überlebt es. Die
Lasair-Lücke bleibt offen: Mit `LASAIR_TOKEN` wäre der dritte Read möglich,
ohne Token ist 401 der ehrliche Messwert. Fink zählt Alert-Zeilen (2), nicht
unique Objekte (1); die Objekt-ID-Spalte macht das sichtbar.

## Was quer liegt

- **Kein Lasair-Token** in dieser Werkstatt — der 401 ist reproduzierbar und
  blockiert die dritte Membership-Aussage, nicht die zwei vorhandenen.
- **ALeRCE `total` ist `null`**; die Trefferzahl kommt aus der Länge von
  `items`, nicht aus dem Feld `total`.
- **Fink** liefert Alert-Zeilen desselben Objekts; „2 Treffer" heißt zwei
  Zeilen, nicht zwei Objekte.
- Build: `cargo build -p omegaflow-measure --bin broker_difference_probe`
  grün; Tests: **23 passed** (`cargo test -p omegaflow-measure --bin
  broker_difference_probe`).
