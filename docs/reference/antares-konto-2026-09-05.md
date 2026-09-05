# ANTARES-Kontoanfrage — Vorlage 2026-09-05

Messgrundlage (per HTTP gemessen, 2026-09-05):
- `curl https://antares.noirlab.edu/support` → HTTP 200 (Vue-SPA-Shell; die Seite
  selbst wird clientgerendert).
- `curl https://antares.noirlab.edu/config.json` → HTTP 200, Inhalt:
  `{"ANTARES_API_URL":"https://api.antares.noirlab.edu/v1", …,
  "ANTARES_SUPPORT_EMAIL":"antares@noirlab.edu", …}`.
- Issue-Tracker der ausgelieferten Konfiguration:
  `https://gitlab.com/nsf-noirlab/csdc/antares/antares/-/issues`.
- Im Frontend-Bundle (`/js/app.*.js`) kein Vorkommen von „forced" — ANTARES
  bietet keine gemessene FP-Oberfläche. Die E-Mail fragt die FP-Lage als Frage an.

## Ablauf für den Operator

1. Betreff und Text unten verwenden (englisch — Instrument an die NOIRLab-Mannschaft).
2. Senden an: **antares@noirlab.edu** (die in `config.json` ausgelieferte Supportadresse).
3. Keine Konten anlegen — nur diese eine E-Mail.
4. Antwortet das Team mit Key+Secret: beide in `~/.local/state/omegaflow/.secrets.local`
   ablegen (Schlüsselnamen z. B. `ANTARES_KAFKA_KEY`, `ANTARES_KAFKA_SECRET`),
   dann ist der Eintrag für `lsst_anomaly_probe --antares` nutzbar.

## Betreff

`ANTARES alert-stream access request — key/secret for a broker consumer (achromatic-periodic technosignature scan)`

## E-Mail-Text (wörtlich versandfertig)

```
To: antares@noirlab.edu
Subject: ANTARES alert-stream access request — key/secret for a broker consumer
        (achromatic-periodic technosignature scan)

Dear ANTARES team,

I would like to request consumer credentials (Kafka key + secret) for the
ANTARES alert stream.

Intended use: an autonomous, all-sky search for a rare signal class — an
achromatic, strictly periodic modulation across optical bands (g/r/i), the
"Nadel-V" scan. The consumer is a small, dependency-free program that connects
to the broker's REST/stream interface directly; it does not run Python. It
processes alert photometry per locus/object into multi-band time series and
applies the periodicity/achromaticity gates locally.

Access requested:
  1. Kafka consumer key + secret for the alert stream (which survey topics are
     currently available — ZTF and/or LSST?).
  2. Information on whether ANTARES serves per-object forced photometry
     (fixed-coordinate measurements, including non-detection epochs). If it is
     available, how is it accessed? If it is not, I will pair the alert stream
     with the anonymous forced-photometry endpoint of the Fink/LSST broker, so
     a direct answer is enough.

Background of the user: independent researcher; the work is published openly
under the Omegaflow project. The consumer abides by the broker's rate limits.

Thank you,
[Name]
[Affiliation / independent researcher]
```

## Was die E-Mail genau verlangt (Kurzliste)

- Kafka consumer key + secret (Alertstrom).
- Auskunft verfügbarer Survey-Topics (ZTF, LSST).
- Auskunft, ob ANTARES FP ausliefert und wie (sonst Kopplung an Fink `/api/v1/fp`).
