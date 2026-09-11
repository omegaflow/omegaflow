# Fink-Kontoanfrage — Vorlage 2026-09-05

Messgrundlage (per HTTP gemessen, 2026-09-05):
- `curl https://fink-broker.org/joining/` → HTTP 200.
- `curl https://api.lsst.fink-portal.org/swagger.json` → HTTP 200 (OpenAPI,
  Titel „Fink/LSST object API", Version 3.7.0). Pfade u. a.:
  `/api/v1/conesearch`, `/api/v1/sources`, `/api/v1/fp`, `/api/v1/objects`,
  `/api/v1/schema`, `/api/v1/tags`.
- Dokumentation: `https://doc.lsst.fink-broker.org/` (HTTP 200).
- Anonyme REST-Endpunkte (`sources`, `fp`) → HTTP 200, kein Token (siehe
  `bericht-forcierte-photometrie-2026-09-05.md`).

## Wichtige gemessene Klarstellung

Die Seite `fink-broker.org/joining/` ist **nicht** der Weg zu Kafka-Stream und
Tokens. Sie führt zur **Fink-Kollaborations-Mitgliedschaft** (Mailingliste +
General-Meeting-Informationen) und ist nur für „academic staff and students
working with a Fink member" gedacht. Die Stream-/Kafka-Zugangsdaten laufen über
ein separates Formular (unten, Schritt 2). Die anonymen REST-Produkte brauchen
ohnehin kein Konto.

## Schritt 1 (optional, nur wenn Kollaborations-Mitgliedschaft gewollt)

`fink-broker.org/joining/` → Link „this form" → Google-Formular
`https://forms.gle/CmvH8vsyyv4AUTpy8` („Joining Fink"; Formular-HTML gemessen).
Gemessene Felder (Pflicht mit *):
1. E-Mail-Adresse * („Please use your institutional email")
2. Your name + surname *
3. Affiliation/Institute *
4. Country *
5. Career stage * (Radio: Undergraduate student / Masters student / PhD
   candidate / Postdoc / Tenure-track / Professor / A.Prof / Other)
6. „If you are an undergraduate/masters/PhD student please provide the name of
   the Fink team member you are working with" (nur für Studierende)
7. Science expertise / interests (a couple of keywords)
8. „Do you agree with Fink's Code of Conduct?" * (Radio: „Yes I do and will
   strive to follow it")

Gewährt: Fink-Kollaborationsmitgliedschaft, Mailingliste, Einladungen zu den
General Meetings. **Keine Kafka-Zugangsdaten.**

## Schritt 2 (der eigentliche Weg zu Stream-/Kafka-Zugang)

Quelle: README `astrolabsoftware/fink-client` (gemessen) — „In order to connect
and poll alerts from Fink, you need to get your credentials":
1. Formular „Subscribe to one or more Fink streams":
   **`https://forms.gle/2td4jysT4e9pkf889`** („Fink services subscription";
   Formular-HTML gemessen). Gemessene Felder (Pflicht mit *):
   - Name *
   - Email *
   - Institution *
   - Service subscription * (Mehrfachauswahl: **Data Transfer service** /
     **Xmatch service** / **Livestream service** / MLflow service (experimental))
     → für den Nadel-V-Zweck **Livestream service** wählen.
   - Motivation to use Fink (ein bis zwei Sätze)
   Text des Formulars (gemessen): „The Fink services are open to any scientists,
   but it is password protected … After filling this form, we will come back to
   you with more information about the credentials."
2. Fink sendet die Zugangsdaten (Benutzername/Passwort) per E-Mail.
3. Registrierung auf dem Rechner: `finkctl auth register` (fink-client v12,
   Python 3.9+, `pip install fink-client --upgrade`); Konfiguration landet in
   `~/.finkclient/lsst_credentials.yml` (Survey lsst). Anzeige: `finkctl auth show -survey lsst`.
4. Topics: Liste `https://lsst.fink-portal.org/schemas` oder
   `finkctl topic list -survey lsst`; abonnieren:
   `finkctl topic subscribe -survey lsst -name <topic>`.
5. Konsum: `finkctl stream -survey lsst --display --limit 1`.

Kontakt bei Schwierigkeiten (gemessen aus der Livestream-Doku):
`contact@fink-broker.org` bzw. Issue-Tracker `github.com/astrolabsoftware/fink-client`.

## Gemessene Grenze für die Maschine (wichtig)

fink-client ist Python-only (Python 3.9+). Die omegaflow-Maschine ist Rust-only
und kann den Kafka-Stream über fink-client **nicht** direkt konsumieren. Der
Zweck der Operator-Anmeldung ist damit vor allem: (a) Zugang für künftige
Mensch-geführte Auswertungen, (b) eventuelle spätere native Kafka-Anbindung
(Rust). Für die Nadel-V-Messung selbst ist die anonyme REST-Fläche
`/api/v1/sources` und `/api/v1/fp` der Maschinenweg — die braucht kein Konto.
