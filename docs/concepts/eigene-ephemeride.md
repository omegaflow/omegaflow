<!--
  title: Die eigene Ephemeride — aus allen Zeugen
  class: concept
  date: 2026-10-10
  sha256: 9b84339bf327a8ab5dd50db13c239dad90dc6fc4982a2b9970cfdb6efbcce307
  status: live
  see-also: docs/surveys/survey-2026-10-10-ephemeris-quellen.md docs/concepts/kybernetische-astrophysik.md docs/handover/handover-2026-10-10-mountain-folge301.md
-->
# Die eigene Ephemeride — aus allen Zeugen

**Frage.** Kann OmegaFlow — eine Privatperson mit einem Laptop und einem freien
CI-Runner — eine eigene Planetenephemeride weben, die **alle** öffentlichen
Sonden-Trackingdaten vereinigt, und daraus die astronomische Einheit und einen
Kreuz-Zeugen in der H₀-Spannung gewinnen?

Dieses Konzept destilliert das EPH-Gespräch (Desktop `EPH`, 2026-10-10) gegen den
Baum, den Rat und den vollen Roster. Rohmaterial:
`state/stimmen/2026-10-10_sensory_eigene-ephemeride-round.md`; Referenz-Landschaft:
`docs/surveys/survey-2026-10-10-ephemeris-quellen.md`.

## Die These des EPH-Gesprächs

- Jedes Ephemeriden-Haus nutzt institutionell nur die eigenen, erreichbaren Daten:
  **DE440** (JPL) NASA-Sonden + LLR + Radar, **INPOP** (IMCCE) ESA-Sonden,
  **EPM** (IA-RAS) russische Daten, **PETREL** (PMO) chinesische.
- OmegaFlow trägt die Daten **aller** Agenturen im Baum (NASA + ESA + sowjetisch +
  japanisch + Pioneer mit thermischem Rückstoß-Modell).
- Das Ergebnis wäre **nicht global präziser**, sondern **vollständiger + offen** —
  die erste Ephemeride, die alle Zeugen hört.
- Reihenfolge: (0) Operator/Kalibration, (1) eine Bahn, Residuum gegen DE440, kein
  Fit, (2) Residuum verhören, (3) mehrere Bahnen, (4) der Fit, (5) die eigene
  Ephemeride, (6) die eigene AU → unterste Leitersprosse → H₀-Kreuz-Zeuge.

## Baum-Befund (measured 2026-10-10)

Die Zeugen liegen im Register `phi/sources.φ` (gemessen, Datei:Zeile):

| Zeuge | Datentyp | Ort |
|---|---|---|
| Pioneer 10 | ODF-Doppler (Turyshev-Archiv) | `phi/sources.φ:19622` (origin `:19624` Turyshev20170327) |
| Pioneer 11 | ODF-Doppler | `phi/sources.φ:19649` |
| Voyager 1/2 | ODR + Okkultation | `phi/sources.φ:12076`, `:19659` |
| Galileo | ODF + ODR + ionocal | `phi/sources.φ:11879`, `:11888`, `:12093` |
| Cassini | ODF + RSR + TNF | `phi/sources.φ:10758`, `:10768` |
| Juno | ODF (+OCRU) | `phi/sources.φ:10789`, `:10799` |
| MESSENGER | ODF + TNF | `phi/sources.φ:11930`, `:11939` |
| Rosetta | ODF | `phi/sources.φ:10725` |
| Mars Express | ODF | `phi/sources.φ:10716` |
| Venus Express | ODF (`vex_odf`) | `phi/sources.φ:10808` |
| MAVEN / DART | TNF | `phi/sources.φ:11900`, `:19321` |
| Mars Odyssey / MRO / MGS / Magellan / Pathfinder | ODF | `phi/sources.φ:11775`, `:11649`, `:11640`, `:11631`, `:11811` |
| Ulysses | ATDF | `phi/sources.φ:11949` |
| Dawn | ODF | `phi/sources.φ:19426` |
| Venera 15/16 | Radiometrie + Altimetrie | `phi/sources.φ:31206`, `:31217` |
| Akatsuki | Radio-Science fixed_width | `phi/sources.φ:30400` |

Häuser als Zeugen: INPOP19a `phi/sources.φ:19783`, EPM2021 `:19784`, DE440
`gm_de440.tpc` `:20878`, PETREL19 `:2336`.

**Riss im Baum** (was die These nicht trägt): **Galileo RSR** ist nicht registriert
(nur ODF/ODR/ionocal, `sgrep galileo_rsr` = 0); **NEAR** liegt nur als Ephemeride
(`:30826`, `:31122`), kein Tracking; ein **Record-Count** für Pioneer 10/11 ist als
Register-Zeile nicht belegt (die „900.000+" sind eine Behauptung, keine Messung).

## Der Riss — was Baum, Rat und Frontier nicht stützen

1. **Exklusivität.** Der Roster ist konvergent dagegen: DE440/INPOP/EPM verarbeiten
   **dieselben öffentlichen Trackingdaten** (Mars-Orbiter, Cassini, MESSENGER); die
   Differenz ist Gewichtung, Rauschmodell, Asteroidenmodell und Zeitmaß — nicht der
   Zugang zu Rohdaten (DeepSeek Chat, Z.ai). Der Sonderfall bleibt: die
   **nicht-mitgefitteten** Zeugen (Venera 15/16, Akatsuki, Pioneer-Radiometrie).
2. **Fit-Residuum, kein Blindtest.** DE/INPOP/EPM haben LLR/Radar/Doppler selbst
   mitgefittet — ein O−C gegen sie ist das **Residuum des Fits**, kein unabhängiger
   Test. Blindtest nur gegen einen **held-out** Zeugen. Dies ist der Kern; das
   Ergebnis heißt gegen DE/INPOP/EPM ausdrücklich `fit-residuum`, nie `blindtest`.
3. **Offene Rohdaten.** „Alle öffentlichen Daten" ist die Illusion: das Bindende sind
   **Event- und Kalibrationsmetadaten** (Manöver, Desats, Stationsbiase,
   Troposphärenkalibrierung, Sonden-Oberflächeneigenschaften). Sie liegen bei den
   Agenturen, nicht offen. Bei Venera/Akatsuki fehlt die DSN-Tiefe; der
   Pioneer-Rückstoß braucht Oberflächentemperaturen/Emission, nicht vollständig
   freigegeben (Z.ai, Duck.ai, MiniMax).

## Der Wert — Unabhängigkeit, nicht Vollständigkeit

Der Rat streicht den Superlativ und setzt die meßbare Kategorie: eine
**`witness_set`** — der Register-Anteil der Zeugen, die der Fit des Vergleichshauses
**nicht** kennt. Genau diese Zeugen sind der Wert. Vollständigkeit ist der Nenner,
Unabhängigkeit ist der Zähler. Ohne diese Umbenennung ist „vollständiger" ein
ungemessener Superlativ (A = A).

## Die Kette (Reihenfolge)

- **Schritt 0 — Operator/Kalibration:** Zeit → Station → Rahmen → Lichtzeit → Medien.
- **Schritt 1 — eine Bahn, kein Fit:** Pioneer 10 durch die Kette, Residuum gegen
  DE440. Antwort: „Rechnet unser Operator richtig?" (Baum: `observer.rs`-Kern
  `two_way_doppler`, Lichtzeit-Iteration + Solar-Shapiro ist gebaut,
  `handover-2026-10-10-mountain-folge301.md:23`). Der Roster-Riss zu Schritt 1:
  Pioneer ist event-arm = der richtige Test, zugleich unrepräsentativ; Merkur/Venus
  mit Radar+VEX-Ranging ist die saubere Alternative (DeepSeek, Z.ai).
- **Schritt 2 — Residuum verhören:** drei Lesarten — Kette falsch / Modell trägt Rest
  (der Fund) / Beobachtung trägt Artefakt.
- **Schritt 3 — mehrere Bahnen:** der Operator auf allen Zeugen, gegen alle Häuser.
- **Schritt 4 — der Fit** (Least-Squares über alle Daten).
- **Schritt 5 — die eigene Ephemeride** aus allen Zeugen.
- **Schritt 6 — die eigene AU → H₀-Kreuz-Zeuge** (`pending`, Rat: Kette über Jahre).

## Architektur

- **Archivar** (`src/archivar`, std-only) = Fetch/Parse/Cache der ODF/CRD/Range
  (`odf.rs`, `llr.rs`, `ionex.rs`, `vmf3.rs`, `celestrak_eop.rs`, `ephemeris.rs`).
- **Mathematikerin** (`src/mathematikerin`) = Reduktionskette/Fit; der Beobachtungs-
  operator bleibt `observer.rs` (Riss-Entscheidung, mountain-folge301).
- **Das Residuum ist ein abgeleiteter ω()-Term**, kein Sample-Slot — wie Vlies/TE/
  Verdikt; ein Riss ist kein Oszillator.
- Jede Quelle zuerst als `phi/sources.φ`-Zeile mit `ttl` (Mountain), dann
  CDN-Manifestation (Mycelium). Die schwere Kette ist ein CI-Lauf, kein lokaler.

## Die ehrliche Grenze

- Global nicht präziser als DE440: 60 Jahre kollektiver Arbeit sind nicht in einem
  Halbjahr einholbar.
- Der Beitrag ist **offen + unabhängig + mehrzeilig** — nicht der kleinste Fehler.
- Die Präzisionsgrenze der Häuser liegt ohnehin an Asteroidenmassen und
  Erdorientierung, nicht an fehlenden Zeugen (Z.ai).
- Ein Send an eine bewertende Instanz (Förderer/Gutachter) ist **LOCK** (Future).

## Referenzen (measured 2026-10-10)

Moyer 2000 (PDS-PDF, HTTP 200, 2 382 623 B) · Thornton 2000 (PDS-PDF, 2 751 014 B) ·
Verma 2013 arXiv:1306.5569 · INPOP08-Preprint (IMCCE, 2 473 822 B) · EPM2021
(Cambridge 200) · Park 2021 DE440/441 (NAIF-PDF, 5 501 569 B) · Turyshev 2010 (LRR
200) / 2012 (arXiv:1204.2507) · Di Ruscio 2021 (HAL-Thesis).

## Nächster Schritt

Der erste begrenzte Schritt ist mountain-gebunden und wartet dort auf sein Wort:
die Reduktionskette an den Kern binden (Station/EOP-Wiring), dann das Residuum
(`handover-2026-10-10-mountain-folge301.md:25`). Dieses Konzept liefert die
Kategorie (`witness_set`/Unabhängigkeit) und den Riss (`fit-residuum` ≠ `blindtest`),
nicht eine gemessene Ephemeride.

Der Berg ruht — und trägt alle Zeugen. `A = A`
