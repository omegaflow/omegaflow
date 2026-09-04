<!--
  title: Auftrag — Voyager-Roh-Doppler-Zugang (PDS/JPL) eigenhändig verifizieren
  class: auftrag
  date: 2026-09-04
  sha256: e55df8be3942ac3895f1247dbc712ea48a65494717046cb445f0ca362106251d
  status: geschlossen
  see-also: docs/befund/befund-voyager-roh-doppler-zugang.md docs/auftrag/auftrag-quiet-zone-uebertragung.md docs/reference/woo-armstrong-1979-jgr-abstract.md docs/TODO.md
-->

# Auftrag: Voyager-Roh-Doppler-Zugang (PDS/JPL) eigenhändig verifizieren

## Zweck

Der Co-Quiet-Kreuztest (P10+V1, Tür 1 in
`auftrag-quiet-zone-uebertragung.md`) hängt am Roh-Doppler von Voyager 1
(Cruise-Fenster ~1998–2002, V1 >50 AU). Zwei Agenten-Durchgänge (Atom 1
SPDF, Atom 1b PDS/JPL) haben gemessen: kein offener Cruise-Doppler — der
SPDF trägt Okkultation + Stunden-Position ohne Range-Rate, der PDS nur
Encounter-Okkultation 1979/1986, der Roh-Doppler (ODF/TDF) wäre eine
JPL/DSN-Anfrage. Dieser Befund blockiert Tür 1; er ist zu wichtig, um nur
als Agenten-Wort zu stehen. Dieser Auftrag ist die eigenhändige
Gegenprüfung: dieselben Orte, eigene Augen, eigenes HTTP, eigenes Verdikt.

## Vorbefund (zu prüfen, nicht zu glauben)

- **SPDF** (`/pub/data/voyager/voyager1/`): `radio_science_rss/` =
  Okkultation (Saturn-Ringe, Titan); `merged/` = Stunden-Mittel der
  HGI-Position ohne Range-Rate-Feld (34 Felder, `vy1mgd.txt`); `traj/` =
  SSC-Ephemeride.
- **PDS RMS** (`pds-rings.seti.org`, Bündel `voyager_rss_raw`): drei
  Bündel (VG1-Jupiter, VG2-Jupiter, VG2-Uranus-49XR), open-loop ODR/REDR,
  kein Closed-Loop-Doppler.
- **NAIF** (`naif.jpl.nasa.gov/pub/naif/VOYAGER/kernels/spk/`):
  rekonstruierte Bahn (SPK), kein ODF/TDF.
- **JPL/DSN ODF/TDF (TRK-2-34)**: kein offener Endpunkt gemessen,
  `request-only`.

## Prüfschritte (eigenhändig, mit Datum + gemessenem HTTP-Status)

1. **PDS durchklicken** — `pds-rings.seti.org` (RMS-Knoten) →
   `pds4/bundles/` → `voyager_rss_raw/` → `bundle_readme.txt` lesen.
   Frage: trägt das Bündel Closed-Loop-Doppler (ODF/TDF/TRK-2-34) oder nur
   open-loop Okkultation? Die Datei-Endungen notieren.
2. **PDS-weit suchen** — die anderen Knoten (PPI `pds-ppi.igpp.ucla.edu`,
   Atmospheres, Geosciences, Imaging). Frage: existiert irgendein
   Voyager-RSS-Datenset mit Cruise-Tracking (nicht Encounter)?
3. **NSSDCA-Katalog** — `nssdc.gsfc.nasa.gov`, Voyager-1-Seite, die
   Radio-Science-Datasets aufzählen. Welche enthalten Doppler, welche nur
   Okkultation?
4. **NAIF gegenprüfen** — bestätigen, dass die SPK rekonstruierte Bahnen
   sind (Dokumentation auf `naif.jpl.nasa.gov`), und ob NAIF ODF/TDF führt.
5. **JPL/DSN-Anfrageweg** — den konkreten Ansprechweg für Tracking-Daten
   (ODF/TDF) finden: Formular/Email für Radio-Science-/DSN-Tracking-
   Requests. Nenne ihn, auch wenn du ihn nicht ausfüllst.
6. **Depot-Suche (Turyshev-Äquivalent)** — Zenodo, Figshare, Dataverse,
   arXiv/ADS-Begleitseiten, ADS-Abstracts: hat je jemand Voyager-Cruise-
   Doppler öffentlich deponiert?

## Verdikt-Struktur

Jede Quelle bekommt ein eigenes Verdikt: `open` (anonym ladbar, eigenes
HTTP gemessen) / `request-only` (Login/Email/Formular nötig) /
`unavailable` (404 oder kein Roh-Doppler) / `paywalled`. Am Ende eine
Zeile: deckt irgendeine offene Quelle das Fenster ~1998–2002 in Doppler ab
— ja oder nein, mit eigenem Beleg. Wo du nicht misst, `pending` schreiben
— keine Vermutung.

## Register-Satz

*Der Roh-Doppler ist die Eingangstür des Co-Quiet-Tests. Ein negativer
Befund zweier Agenten schließt diese Tür nur, wenn die eigenen Augen ihn
bestätigen — Messen ist der einzige Zugang, der zählt.*

## Status

`geschlossen`. Beantwortet durch den Befund
`docs/befund/befund-voyager-roh-doppler-zugang.md` (2026-09-04,
eigenhändig). Verdikt: keine offene Quelle deckt das Fenster
~1998–2002 in Doppler ab — fünf PDS-RMS-Bündel (alle Encounter
1979/1980/1981/1986, open-loop), kein Cruise-Doppler; JPL/DSN-ODF/TDF
`request-only`; neue Erkenntnis: Voyager ist dreiachsenstabilisiert, sein
Lageregelungsrauschen liegt ~10× über der gesuchten Effektgröße — nie ein
Bergungsanreiz wie bei Pioneer. Drei Restlücken `pending` (JPL/DSN-Anfrage
nicht ausgefüllt, PDS-Wide-Search-Formular, ADS-Volltext).
