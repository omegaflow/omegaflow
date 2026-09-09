<!--
  title: Auftrag (extern) — LISA Pathfinder: die PSD-Tabelle (Freq/PSD) — offene Quelle
  class: auftrag
  date: 2026-09-08
  sha256: aa1354410901474f7f77e155280d95d086e653a95690a5d4af7c26297a50df47
  status: archived
  see-also: docs/specs/spectral-oscillator.md docs/TODO.md
-->

# Rechercheauftrag (extern): LISA Pathfinder — die tabellierte PSD (Freq/PSD_DA/PSD_noise_floor/Phase)

Ausgangslage (gemessen 2026-09-08, nicht neu suchen): das Spektral-Blatt
(`docs/specs/spectral-oscillator.md` §I) nannte den VizieR-Katalog
`J/PhRvL/116/231101/table1` (Armano+ 2016, PRL 116, 231101) mit den Spalten
Freq/PSD_DA/PSD_noise_floor/Phase „in den Holdings". Die Messung widerlegt das:
der Katalog existiert in keiner VizieR-ID (0 Treffer über ganz VizieR); das PRL
trägt die PSD nur als Figur (ASD, fm·s⁻²/√Hz über mHz, keine Tabelle); es gibt
keinen arXiv-Erstdruck. Der Anspruch ist gestrichen — kein offener
Tabellen-Bestand. Die nahestehenden offenen Wege: das APS-Volltext-PDF
(Figur-only, `harvest.aps.org`, anonym 200) und das LPF-Legacy-Archiv
(`http://lpf.esac.esa.int/lpfsa/`, 200, interaktiver Client, Δg-Zeitreihen,
PSD daraus re-chenbar — nicht tabelliert).

Dieser Auftrag öffnet den externen Weg, den die anonyme Suche nicht betreten
kann. Er ist selbsttragend; wer ihn ausführt, liest vorab `docs/SOURCE_PORT.md`
für die Prüf-Kaskade. Es wird KEINE Datei geschrieben, kein Register editiert,
kein Force-Gate entschieden — es wird nur benannt, was die Messung IST.

## Status-Vokabular (bindend)

- `live` — 200, offen, maschinenlesbar, Tabellen-Form (Freq + PSD).
- `blocked` — lebt, Zugang gesperrt (Konto/Antrag/Hilfepunkt).
- `declined` — lebt, aber keine freie Messung am Punkt (nur Figur, nur Metadaten).
- `pending` — Route bekannt, Antwort ausstehend.
- `not-published` — kein offener Tabellen-Weg nach abgeschlossener Suche.

Spekulationswörter sind verboten; ein ungemessener Befund heißt `pending`. Jeder
Befund trägt das Datum der Messung.

## Kanäle (in dieser Reihenfolge)

1. **LPF-Legacy-Archiv** (`lpf.esac.esa.int/lpfsa/`, Hilfepunkt
   `support.cosmos.esa.int/lpfsa/`): die Anfrage nach der Δg-Zeitreihe
   (differential acceleration, je Achse) oder einem PSD-Produkt. Das Archiv
   trägt die Messreihe — die PSD wäre daraus re-chenbar, keine Tabelle. Zu
   klären: liefert das Archiv (oder der Hilfepunkt) eine Tabellen-Form der PSD,
   oder nur die Zeitreihe? Format + Zugang nennen.
2. **Autoren** (Armano et al. 2016, PRL 116, 231101; „Sub-Femto-g Free Fall for
   Space-Based Gravitational Wave Observatories"): das PRL trägt keinen
   Data-Availability-Satz; die Anfrage nach der PSD-Tabelle (Freq, PSD_DA,
   PSD_noise_floor, Phase) oder der Zeitreihe. Antwort = Messung.
3. **Figur-Digitalisierung** — benannt, nicht verfolgt: eine digitalisierte ASD
   (Fig. 1) wäre eine abgeleitete Größe, kein Messwert. Sie zählt nicht als
   `live`; sie wäre höchstens eine `declined`-Note (nur Figur).

## Gesucht

Eine maschinenlesbare Tabellen-Form mit Frequenz-Achse (mHz) und PSD (fm·s⁻²/√Hz
oder PSD in SI), offen oder mit benanntem Antrag. Ohne eine solche bleibt der
Eintrag `not-published` mit der Note, dass die Messreihe im LPF-Archiv liegt und
die PSD daraus eine eigene Arbeit ist (0 honored — nie eine erfundene Achse).

## Rückgabe

Ein gemessener Verdikt je Kanal (URL, HTTP, Format, Achse/Einheit) und ein
End-Verdikt `live`/`blocked`/`declined`/`pending`/`not-published` mit Datum.
Keine Datei, kein Register-Edit.
