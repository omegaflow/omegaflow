<!--
  title: Auftrag (extern) — Neutrino- (IceCube) und Höchstenergie-CR (Telescope Array): offene Daten-Routen
  class: auftrag
  date: 2026-09-06
  status: pending
  sha256: 18d298b9f2c036f137845424ac4fa9ce74e9c0e4e71151be2885094470833d9d
  see-also: docs/concepts/die-weberin.md phi/pipeline/research/agent_output/verify_skymap_2026-09-06.φ
-->

# Rechercheauftrag (extern): IceCube-Neutrino- und Telescope-Array-CR-Daten — offene Routen

Dieser Auftrag ist für eine externe Person gedacht. Er ist selbsttragend —
kein Vorwissen über das omegaflow-System nötig. Ziel ist reine Daten-Routen-
Recherche: welche Adresse liefert wirklich maschinenlesbare, öffentliche
Daten, welche ist tot oder gesperrt. Keine Analyse, keine Beobachtung — nur
das Verifizieren und Zusammenstellen publizierter Messwerte.

## Warum (der eine Satz)

Das System braucht als Zeugen die Ankunftsrichtungen der höchstenergetischen
Neutrinos (IceCube) und der kosmischen Strahlung (Telescope Array) als
positions-tragende Ereignislisten (RA/Dec + Energie). Zwei Routen sind
bereits gemessen-gesperrt — aber im eigenen Register liegen Adressen, die
noch zu prüfen sind.

## Schon gemessen (nicht nochmal suchen)

- `www.telescopearray.org` → 301 → `jws001.dyndns-home.com` (Seite lebt),
  aber kein public-data-Eintrag; `/index.php/*/public-data` 404,
  `/data` 404, `data.telescopearray.org` DNS-tot.
- `icecube.wisc.edu/data-releases/` → 403 origin-seitig (nginx, kein
  IP-Block, keine Auth-Form); der geprüfte HESE-Link `hese_12yr_events.csv`
  → 404.
- Der AMON/GCN-Notices-Kanal ist bereits verdrahtet — nicht Gegenstand.

## Im Register gefundene Adressen (bitte wirklich anfragen, nicht annehmen)

Vorab gemessen (2026-09-06, curl): IceCat-1-Dataverse → 303 (lebt, Redirect
auf die Datei); EHE-DOI `10.7910/DVN/JHK49D` → 200 (lebt); die
VizieR-TA-Abfrage → 403 (vermutlich URL-Kodierung — die `+`-Leerzeichen
der Register-URL sind zu prüfen).

1. **Telescope Array SD-Ereignisse über VizieR** (Katalog `J/ApJ/867/L27`):
   `https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync?REQUEST=doQuery&LANG=ADQL&FORMAT=csv&QUERY=SELECT+RAJ2000,DEJ2000,Energy,logEnergy,Zenith,Azimuth+FROM+"J/ApJ/867/L27/table1"`
   → Prüfen: liefert das die TA-Ereignisliste? Wie viele Zeilen? Voller
   Datensatz oder Ausschnitt?
2. **IceCube IceCat-1** (Astrophysikalischer Neutrino-Katalog), Harvard
   Dataverse, Datafile 7502710:
   `https://dataverse.harvard.edu/api/access/datafile/7502710`
   → Prüfen: ist das IceCat-1? Format? Enthält es RA/Dec/Energie/Signalness?
3. **IceCube EHE-Neutrinos + UHECR-Proton-Fraktion**, Harvard Dataverse
   DOI `10.7910/DVN/JHK49D` → Prüfen: was trägt dieser Datensatz genau?
4. **Telescope Array „Amaterasu"** (Einzelereignis):
   `/newslist/160-amaterasurelease` — nur der Vollständigkeit halber; ein
   Einzelereignis ist kein Katalog.

## Gesuchte Daten — pro Route

- die exakte, funktionierende URL (mit getestetem HTTP-Status),
- das Format (CSV/FITS/JSON/…),
- die Spalten (RA/Dec in welchem System, Energie in welcher Einheit, Zeit),
- Zeilenzahl / Abdeckung,
- ob ein Konto oder Token nötig ist.

## Regel (0 honored)

Nur real abrufbare, publizierte Daten. Eine geprüfte Adresse, die 404/403
liefert, wird als „tot/gesperrt" gemeldet — nie ein Ersatz-Datensatz, nie
eine Schätzung. Was nicht öffentlich liegt, bleibt „nicht publiziert".

## Lieferung

Ein kurzer Bericht: pro Route eine Zeile
`URL | Format | Spalten | Zeilen | Status (offen/tot/gesperrt/Konto nötig)`.
Dazu für die eine beste Route je Quelle die konkrete Empfehlung, welche wir
als Kompilier-Route (HEALPix-Karte) bauen können.
