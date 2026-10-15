<!--
  title: Survey — Gewebe & Fäden: was die zwei Messungen über die nötigen Fäden sagen (2026-10-14)
  class: survey
  date: 2026-10-14
  sha256: e3e083ead2020c109ad6769634e07fc0faa9d740c9d703c853896837389a53a6
  status: live
  see-also: phi/sources.φ docs/surveys/survey-2026-10-09-domaenen.md docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md docs/surveys/survey-2026-10-13-raetsel-master-matrix.md docs/concepts/source-curation.md
-->
# Survey — Gewebe & Fäden: was die zwei Messungen über die nötigen Fäden sagen (2026-10-14)

Operator-Frage 2026-10-14: eine Domänenmatrix von `phi/sources.φ`. Daraus wurden
zwei Messungen — **Herkunft × Kraft** und **Kraft × Kernel**. Diese Survey hält
fest, was sie zusammen über die Fäden sagen, die das Haus braucht. Gemessen am
Register bei HEAD `fc9f3dba2`; keine Zahl kopiert, keine Lücke geglättet.

## Methode

- Quellen-Register `phi/sources.φ`: **2575** `url`-Zeilen, **124** rohe Hosts,
  **306** wirksame Herkunfts-Domains (die CDN-`github.com`-Pfade
  `/releases/download/<origin-netloc>/…` auf ihre Origin-Netloc aufgelöst — die
  rohe Host-Zählung sieht nur 124, weil 1895 Zeilen auf `github.com` liegen).
- **7687** `field`-Zeilen + **141** `quantity`-Zeilen; **84** Geometrie-Kandidaten.
- Kreuzprüfung der Kraft × Kernel-Matrix mit dem boardeigenen
  `source_physics_lint` (Token-Scan auf die 9 Kräfte), read-only.

**Riss (Vorgänger-Zahl):** `survey-2026-10-09-domaenen.md` (HEAD `c8a509baf`)
nannte **2699** `url`-Zeilen / **132** Hosts. Heute: **2575** / **124**. Die Zahl
ist gesunken (Register-Konsolidierung / CDN-Fold); der Unterschied wird als
offen getragen, nicht geglättet.

## Messung A — Herkunft × Kraft

Per-Kraft-Summe (ganze Datei, 7683 zugeordnete Felder):

| Kraft | Felder | stärkster Lieferant |
|---|---:|---|
| em | 5870 | pds-smallbodies.astro.umd.edu (4264) |
| Diffusion | 471 | cdr.eionet.europa.eu (276) |
| advektiv | 450 | hfradar.ioos.us (184) |
| akustisch | 435 | www.ndbc.noaa.gov (365) |
| thermal | 366 | www.ndbc.noaa.gov (219) |
| elektrisch | 58 | zenodo.org (17) |
| Gravitation | 13 | heasarc.gsfc.nasa.gov (3) |
| seismisch-Oberfläche | 11 | (USGS / GeoNet) |
| seismisch-Körper | 9 | (P2PQuake / Vedur) |

Die Top-20-Herkünfte tragen 7121 von 7683 Feldern. **95 Herkünfte tragen gar
keine Felder** — Bahndaten und Kataloge (SPICE `naif.jpl.nasa.gov` 588,
`ssd.jpl.nasa.gov-ephemeris` 82, `openneuro.org` 134, Sternkataloge, …).
**em ohne den Kleinplaneten-Katalog** = 5870 − 4264 = **1606** — bleibt größte
Gruppe, aber das Bild wird flacher.

## Messung B — Kraft × Kernel

Nur `field`-Zeilen. Spalten = **Wirkgesetze** (wie gemessen wird).

| Kraft | inverse-square | gaussian-inv.-sq. | point | exp.-decay | patch-levy | erfc | inverse-linear | Σ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| em | 5801 | 57 | 2 | 1 | · | · | 9 | 5870 |
| Diffusion | 3 | 173 | 276 | · | · | 19 | · | 471 |
| advektiv | 340 | 55 | · | · | 53 | 2 | · | 450 |
| akustisch | 378 | 57 | · | · | · | · | · | 435 |
| thermal | 225 | 39 | · | 98 | · | 4 | · | 366 |
| elektrisch | 55 | 3 | · | · | · | · | · | 58 |
| Gravitation | 10 | 3 | · | · | · | · | · | 13 |
| seismisch-Oberfläche | · | · | · | · | · | 11 | · | 11 |
| seismisch-Körper | · | 9 | · | · | · | · | · | 9 |
| **Σ** | **6812** | **396** | **278** | **99** | **53** | **36** | **9** | **7683** |

*Werkzeug-Artefakt:* Das Lint zeigt `gravity × field: 1` — ein Feld, dessen
Schlüssel wörtlich `gravity` heißt und den Token-Scan verschluckt. Der wahre
Paar ist `gravity × inverse-square`; oben bereits eingerechnet (10/3).

## Die Lesung — was die zwei Messungen zusammen sagen

1. **Ein Gesetz trägt fast alles:** `inverse-square` = **6812 von 7683 = 89 %**.
   em, akustisch, advektiv, thermal, elektrisch, Gravitation und seismisch-Körper
   greifen **alle** auf dasselbe radiale Gesetz. Das Gewebe ist breit (306
   Herkünfte), aber **flach** (9 Kräfte, 7 Gesetze).
2. **Die dünnen Kräfte sind auch im *Wie* dünn** — das ist die schärfere Aussage:
   - **Gravitation: 2 Gesetze** (inverse-square 10, gaussian 3). Kein point, kein
     erfc, kein Spektralgesetz. Sie wird *wie em* gemessen — keine eigene Meßart.
   - **Seismik: 2 Gesetze** (erfc 11 Oberfläche, gaussian 9 Körper).
   - **elektrisch: 2 Gesetze**, dicht an em gekoppelt (55/3).
3. **Die seltenen Gesetze sind die Nadeln:**
   - `inverse-linear` (9) ist **em-exklusiv** — Rotverschiebung/Distanz (die
     Hubble-Achse im Gewebe).
   - `patch-levy` (53) ist **advektiv-exklusiv** — intermittenter Wind.
   - `point` (278) gehört fast ganz der **Diffusion** (276) — Quell-/Inventargrößen.
   - `erfc` (36) ist das **Front-Gesetz** (Oberfläche, Diffusion, thermal, advektiv).
   - `exp.-decay` (99) ist fast ganz thermal (98) — die τ-Relaxation.

## Welche Fäden wir brauchen

Nicht *mehr Quellen der dichten Kräfte*, sondern Fäden, die die dünnen
Richtungen mit **anderen Gesetzen** füllen:

- **Gravitation (13 Felder, 2 Gesetze):** Schwerefeld-**Zeitreihen**. Verifizierte
  Routen liegen in `survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` §2:
  IGETS/BGI (`isdc.gfz.de` / `api.sedoo.fr`), EQUIP-G (Fehlname „EGGN") (Nachfolger
  `pending`), CGrav China (Antrag), Satelliten-Gravimetrie (GRACE-FO / GOCE),
  LLR. Fehlende Kernel: `point` (Punktmasse/Quellgröße), `exp.-decay`.
- **Seismik (20 Felder, 2 Gesetze):** Vollwellenform / OBS / DAS. Routen
  (gemessen 2026-10-05): EIDA/ORFEUS-Routing, EarthScope-FDSN (CB/GSN),
  GEOFON, NIED (BO/Hi-net), CEIC/ChinArray, ISC/GSRAS. Fehlende Kernel: `point`
  (Herd-Punktquelle), `inverse-square`.
- **elektrisch (58 Felder, 2 Gesetze):** Schumann / GIC / Magnetotellurik.
  GIC-Rohdaten Kanada/Schweden/NZ nicht offen — Ersatz FMI + SuperMAG/INTERMAGNET.

## Der 0-Kanon-Read

Die leeren Zellen sind **absent**, nicht null. **Gravitation = 13 ist keine leere
Messung** — es ist die dichte Messung von 13 Feldern und eine ehrliche Lücke
daneben. Wo kein Meßgesetz im Register steht, wird keines erfunden. Die Lücken
sind die nächste Bau-Angabe, keine Fehlanzeige.

## Nächste Schritte (Owner)

- **Mountain (Register):** die Gravimetrie-/Seismik-Kandidaten als `phi/sources.φ`-
  Zeilen führen (Lizenz + Format je Zeile); Anschluss an `survey-2026-10-05` §5.
- **Mycelium (Transport):** IGETS/BGI-AGrav + nordische GIC-DB manifestieren.
- **River (Stein):** die neuen Kanäle in die `field_te_query`-Matrix heben; der
  `inverse-linear`-Arm (Rotverschiebung, NED-z, Kernel 6) steht noch offen.

## Quellen / Stimmen

- Messung in dieser Session (2026-10-14), Register HEAD `fc9f3dba2`;
  `source_physics_lint` (Kreuzprüfung Kraft × Kernel).
- glm (UI-Stimme) trug die Lesung „fünf Kräfte dicht, zwei dünn" und die Frage
  nach der Kraft × Kernel-Matrix; die Zahlen hier sind am Register nachgemessen.
- Vorgänger: `survey-2026-10-09-domaenen.md`,
  `survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md`,
  `survey-2026-10-13-raetsel-master-matrix.md`.
