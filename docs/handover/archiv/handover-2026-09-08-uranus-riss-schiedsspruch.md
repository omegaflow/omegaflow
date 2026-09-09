<!--
  title: Handover — Uranus-Riss-Schiedsspruch: die topozentrische Residuen-Probe (DE/INPOP/EPM gegen Camargo-Astrometrie)
  class: handover
  date: 2026-09-08
  sha256: 3ae046679149c2f859377d2cbf62c5e46407d2d3da5c636cfbfd8deb961c9dba
  status: archived
  see-also: docs/handover/handover-2026-09-08-weberin-zweitlinien-geschlossen.md docs/concepts/die-weberin.md docs/auftrag/auftrag-extern-weberin-zweitlinien.md docs/TODO.md
-->

# Handover — Uranus-Riss-Schiedsspruch (das Atom der nächsten Sitzung)

Übergabe für die nächste Sitzung. Das Atom: die **Blatt-gegen-Wurzel-Messung**
— die drei Ephemeriden (DE/JPL, INPOP/IMCCE, EPM/IAA-RAS) gegen die rohe
Uranus-Astrometrie auswerten und den Eisriesen-Riss einem Schiedsspruch
zuführen, statt nur drei Modell-Fäden gegeneinander zu halten.

## 0. Die abgebende Sitzung ist ein vollständig abgeschlossenes Atom

Die Weberin-Sonnensystem-Arbeit (zweite Linie je Körper-Klasse) ist gebaut,
gemessen, registriert, manifestiert und committet — Abschluss:
`docs/handover/handover-2026-09-08-weberin-zweitlinien-geschlossen.md`.
Alle Assets auf dem CDN (mpcorb, dcom5, inpop, gaia, epm, des_y6, ossos).
Die drei gemessenen Befunde stehen im Register: Eisriesen-Riss echt
(DE/INPOP/EPM divergieren, kein Paar konvergiert), TNO-2-Körper-Skala ~5e9 m,
Sonden-Zweitlinie `not-published`. Diese Übergabe eröffnet das **nächste**
Atom; nichts aus der abgebenden Sitzung ist offen.

## 1. Das Atom — Blatt gegen Wurzel

Drei Ephemeriden sind Modell-Fäden am selben Beobachtungs-Baum; ihr
Blatt-gegen-Blatt-Vergleich (Uranus ~1,6e6 m, Neptun ~1–8e6 m) misst die
Modell-Spreizung, nicht die Wahrheitsfrage. Die echte Messung: **jede
Ephemeride gegen die rohen Astrometrie-Beobachtungen** auswerten —
Residuum (Vorhersage − Beobachtung) je Epoche, je Ephemeride.

## 2. Die konditionierte Messung — drei vorab genagelte Verdikte

- **(i) Die Beobachtungen schlichten:** eine Ephemeride trägt die Residuen
  messbar besser → der Riss bekommt einen Schiedsspruch.
- **(ii) Die Beobachtungen schlichten nicht:** alle drei liegen innerhalb der
  Beobachtungs-Unsicherheit → der Riss ist ein Extrapolations-Phänomen
  (Modell-Raum), kein beobachtbarer Widerspruch. Auch das ist ein Ergebnis.
- **(iii) Die Beobachtungen widersprechen allen dreien:** eine Systematik oder
  etwas, das kein Modell trägt — der interessanteste Fall. Die Probe muss
  alle drei Ausgänge kennen, nicht nur zwei.

## 3. Die eigentliche Zahl ist die Streitort-Zeitreihe

Alle drei Fits sind gut; ein nacktes Gesamtmittel wird „eng" ausfallen. Die
Probe muss **zuerst die Divergenz-Zonen der drei Ephemeriden kartieren**
(wo laufen DE/INPOP/EPM auseinander), dann die Beobachtungsdichte genau dort
prüfen — und ehrlich melden, wenn am Streitort keine Beobachtungen liegen
(weißes Feld, kein Schiedsspruch). Die Residuum-Zeitreihe, nicht ihr
Mittelwert, ist der Schiedsspruch.

## 4. Register-Disziplin

Jede Beobachtungszeile trägt mindestens: RA/Dec + Epoche (TDB) +
Referenzrahmen + Quellen-Publikation. Der Verdict-Satz lautet „Ephemeride X
trägt die Beobachtungen näher (ungewichtetes RMS)" — **nie** „X ist
korrekter": die Original-Fits haben gewichtet, die Probe (zuerst) nicht. Eine
Zeile, die den Gewichtsunterschied benennt, hält die Probe davon ab, mehr zu
behaupten als sie misst.

## 5. Die Voyager-2-Anker-Frage

Astrometrie ist eine **Richtungs-Waage** (RA/Dec quer zur Sichtlinie). Die
1000–8000 km sitzen aber überwiegend **entlang** der Bahn — dort schlichtet
nur der Distanz-Anker: die Voyager-2-Flybys 1986 (Uranus) / 1989 (Neptun).
Die Probe muss klären: stecken die Voyager-2-Anker in allen drei Fits, und
tauchen sie als eigener Zeugentyp in der Messung auf? Richtungs-Waage plus
Distanz-Anker entscheidet, ob der Riss eine Richtungsfrage ist (Astrometrie
schlichtet) oder eine Bahn-Phasenfrage (nur der Distanz-Anker schlichtet).

## 6. Das geerntete Asset (liegt schon)

`data/vizier.cfa.harvard.edu/camargo_uranu_j.tsv` — Camargo+ 2015
(A&A 582, A8), `uranu_j`: **3.516 Uranus-Positionen** (indirekt aus den
Satelliten bestimmt), 1992–2011, Spalten `JD (F16.8, UTC) | RAJ2000 (h:m:s) |
e_RAJ2000 (mas) | DEJ2000 (d:m:s) | e_DEJ2000 (mas) | Sat`. Rahmen:
J2000/eq_FK5 (ICRS-konsistent), UCAC4-reduziert, Fehler ~40–112 mas.
Topozentrisch am Pico dos Dias (LNA, MPC 874). Abruf: VizieR-CfA-Mirror
`asu-tsv?-source=J/A+A/582/A8/uranu_j&-out.max=100000` (der kanonische
CDS-Host ist Anubis-Bot-gesperrt — der Mirror nicht).

Weitere gemessene Quellen (für die Folge): APDB/GeoAzur
(`www.geoazur.fr/astrogeo/observations/base/`, Uranus/Neptun-Astrometrie
1753–1995, aber FK4/Boss-GC-Rahmen); Zhang+ 2024 (`J/ApJS/273/25`,
10.339 Satellitenpositionen, Gaia-DR3-Rahmen — Satelliten, nicht
Planetenzentrum); Yunnan Neptun+Triton 2020–2024 (Icarus 437:16625,
ScienceDirect 400, nicht in VizieR).

## 7. Der technische Pfad

Im Repo vorhanden: die drei Ephemeriden als Bins (DE `ephemeris_*.bin`,
INPOP `data/ftp.imcce.fr/ephemeris_inpop_*.bin`, EPM
`data/ftp.iaaras.ru/ephemeris_epm_*.bin`); Leap-Second/TDB-Umrechnung
(`lsk.rs`, `naif0012.tls`); `body_barycenter_position`,
`body_fixed_to_icrs` (`motion.rs`) für die topozentrische Auswertung.
**Fehlt (kleine std-only-Ergänzung):** die retardierte Lichtzeit-Iteration
und die Apparent/Astrometrisch-Konvention. Die topozentrische Parallaxe
(~0,45″ bei Uranus) ist größer als der Riss (~0,5″) und die Datenfehler
(~0,05″) — sie MUSS korrekt entfernt werden; ein geozentrischer Ansatz
verschmierte den Tag/Nacht-Parallaxen-Term in das Residuum. Erste Wahl:
topozentrisch am MPC-874-Geodät mit Lichtzeit.

## 8. Arbeitsregeln für die bauende Sitzung

- **Bauen, nicht registrieren:** „pending for now" oder „registrieren statt
  arbeiten" sind KEINE Optionen. Die Daten liegen, der Pfad ist benannt —
  das Atom wird gebaut und gemessen.
- **Kontext sparen per Sub-Agenten erlaubt und erwünscht:** wenn die Sitzung
  Agenten beauftragen will, um den eigenen Kontext zu schonen, soll sie das
  tun — die abgebende Sitzung hat es genauso gehandhabt. Die Messung selbst
  (Verdikte, Streitort, Register-Zeilen) ist ihr Urteil, nicht das des Agenten.
- **Geteiltes Repo:** parallele Sessions committen gleichzeitig und stagen in
  den gemeinsamen Index. Immer nur eigene Dateien stagen (`git add -p` bei
  geteilten Dateien), parallele Hunks nie committen, `.git/index.lock`-Fehler
  kurz abwarten und erneut versuchen.
- **Commit-Gate:** `cargo check` null Fehler/null Warnungen; das
  `commit_check`-Gate blockt `unwrap_or(0.0)`, `unwrap_or_else`,
  `#[derive(Default)]`, Code-Kommentare. Vor dem Commit prüfen.
- A = A: absent bleibt absent, ungebaut bleibt ungebaut — aber das Atom ist
  ein Bau, keine Register-Zeile.
