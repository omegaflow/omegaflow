<!--
  title: Auftrag — 20-s-Bande: two-/three-way-Split und Registerzeilen schließen
  class: auftrag
  date: 2026-08-30
  status: pending
  see-also: docs/paper/twenty-second-band-ground-chain.md docs/paper/ground-sources-20s-band.md docs/handover/handover-2026-08-30-zwanzig-sekunden-herkunft.md
-->

# Auftrag: der two-/three-way-Split und die offenen Registerzeilen der 20-s-Bande

## Zweck

Der Kernbefund der 20-s-Bande steht: stationsfixe, kohärente (Q > 2500),
stärkekonstante Linien bei 45,75 / 51,55 / 47,35 mHz (Goldstone/Canberra/
Madrid), upstream im NOCC-eigenen `doppler_resid`, mit 31 gemessenen
Deduktionen (§5.4; `boden-quellen` referenziert zusätzlich 33–35). Der
Kernbefund (Stationsfixität) ist durch die offenen Registerzeilen nicht
berührt — aber bevor die Mail an Toth geht, müssen die offenen Zeilen
geschlossen sein (Schritt 0: „die Zahlen müssen stehen").

Dieser Auftrag schließt die drei offenen Registerzeilen und führt den
wichtigsten ungemachten Test des Korpus aus: den two-/three-way-Split.

## Kernregel (0 honored)

Kein Ausgang des Splits wird vorweggenommen. Beide Ausgänge schärfen den
Befund — das ist das Kriterium, das einen benannten Test von Stochern trennt.
Jede Registerzeile wird gemessen, nicht abgeschätzt. `pending` bleibt
`pending`, nie 0.0.

## Der Split (Klasse-1-Test, vor Mail 1)

Bei three-way-Tracking sendet Station A, empfängt Station B:

- Wandert die Linie mit dem **Empfänger** → die Empfangskette ist endgültig
  vermessen (schließt die Lokalisierung ab).
- Wandert sie mit dem **Sender** → die Sendeseite öffnet sich wieder und
  korrigiert die Lokalisierung.

**Vorbedingung (Registerfrage, keine Annahme):** Tragen die ATDF-Dumps
überhaupt three-way-Segmente? Pioneer 10 wurde spät sparse und single-station
getrackt — möglich, dass lokal keine existieren.

- Falls **ja:** ein Nachmittag mit dem bestehenden Parser.
- Falls **nein:** die Mail selbst ist das Testinstrument — Toth/Markwardt
  haben Zugriff oder Hardware-Wissen; der Split ist genau das, was das
  Koautoren-Angebot kauft.

## Registerzeilen (gehen ins selbe Papier)

1. **f\*** — die drei abweichenden Frequenzwerte (50,73 / 50,71 / 50,714).
   Gegen die soliden Linien (45,75 / 51,55 / 47,35 mHz) abgleichen und die
   Diskrepanz benennen.
2. **1-s-Zählung** — 73 249 / 70 602 / 162 548 ≠ 501 876. Die Zählung gegen
   das Blatt auflösen; Doppel-/Fehlzählung benennen (Nummern-Audit, siehe
   `auftrag-maschinen-audits.md`).
3. **Epochen-Persistenz** — liegen Mehr-Epochen-Detektionen schon im Korpus
   (drei Epochen existieren laut Register)? Persistenz über Jahrzehnte =
   Relevanz für jeden Archiv-Nutzer; ein datierter Beginn = über
   Hardware-Logs identifizierbar.
4. **Amplitude** — die Mail-Vorlage trägt „~160 Hz"; dieser Wert ist eine
   offene Registerzeile und muss vor Mail-Versand gegen das Blatt verifiziert
   werden (kein Wert geht ohne Anker hinaus).

## Klassengrenze (nicht vermischen)

- **Klasse 2 (Verstärkung, nach Mail 1):** Epochen-Persistenz, Cross-Mission-
  Scan (Galileo/Cassini/Ulysses), Kontaminations-Quantisierung. Als Folgeblatt
  registriert, nicht vor Mail 1 ausführen (Single-Blatt-Regel).
- **Klasse 3 (neue Fronten):** benannt bleiben, ungebaut bleiben.

## Lieferung

- Registerzeile pro Punkt: gemessener Wert, Quelle, Status (geschlossen /
  `pending`).
- Split-Ergebnis: Empfänger- oder Sender-Wanderung, oder „keine Segmente
  lokal → Mail als Instrument".

## Abschluss

Erst wenn die Registerzeilen stehen und der Split gelaufen (oder als
Mail-Instrument verbucht) ist, ist die Bande reif für die Toth-Mail
(`auftrag-adoption.md`). Bis dahin bleibt die Bande `pending`, 0 honored —
mit geschärfter Lokalisierung statt ungeprüfter Zahlen.

**Session 2 der Welt-Zugangs-Phase** (nach dem Merge, Session 1): der Split
macht aus „bitte lesen" (schwacher Ask) „wir wissen genau, was wir nicht
wissen, und Ihre Datenlage kann es entscheiden" (starker Ask) — für einen der
meistbeschäftigten Leute der Welt der Unterschied zwischen Löschen und
Antworten.
