<!--
  title: Auftrag — Grat-Folge: Trishuli Regen→Pegel konditionieren (dritter Pfeil)
  class: auftrag
  date: 2026-09-05
  sha256: 4d55173e6385ebccef0e90c452a2640c1fc29044baa78e55584cdea7fa4092fa
  status: pending
  see-also: docs/auftrag/auftrag-der-grat.md docs/blatt/blatt-der-grat.md docs/paper/blatt-pfeil-sturzflut-tibet.md docs/paper/blatt-kreuz-screening-kollab.md
-->
# Auftrag: Grat-Folge — Trishuli Regen→Pegel konditionieren (dritter Pfeil)

## Zweck

Die Grat-Bilanz `docs/blatt/blatt-der-grat.md` (Auftrag
`auftrag-der-grat.md`, GESCHLOSSEN 2026-09-05) zählt den co-lokalen
Trishuli-Regen→Pegel-Pfeil (TE 0.265 > 0.218, Lag 24 h, n = 129,
Vor-Flut-Fenster) als über-Schwellen-Pfeil in die Bilanz — und benennt in
ihren Grenzen die Lücke: das Quell-Blatt
`docs/paper/blatt-pfeil-sturzflut-tibet.md` registriert am co-lokalen
Gauge keine Konfund-Konditionierung. Der Pfeil beantwortet „mehr als
Zufall"; „mehr als gemeiner Treiber" bleibt `pending`. Dieser Auftrag
schließt die Lücke:

**Übersteht die co-lokale Regen→Pegel-Kopplung die Konditionierung auf den
gemeinsamen synoptischen Treiber — denselben, auf den die Kreuz-Screenings
konditionieren (gyirong temperature_2m / die regionalen Wetter-Proxies,
Residuen-Surrogat, mean + 2σ)?**

Das Verdikt entscheidet die dritte Zeile der Grat-Bilanz: festigt sich der
Pfeil als dritter vollständig gemessener Pfeil, oder fällt er in die
Kollab-Stille des geteilten Treibers.

## Kernregel (0 honored)

- **Keine neue Ernte.** Der Lauf nutzt die geernteten Reihen: den
  DHM-Gauge Bhotekoshi/Rasuwagadhi (Überlapp n = 129 im Vor-Flut-Fenster)
  und die regionalen Stationsreihen der Kreuz-Screenings (n = 240,
  stündlich) — der Treiber-Proxi liegt unter ihnen.
- **Eine Zahl ist erst ein Messergebnis, wenn der Lauf steht.** Kein Wert
  wandert aus der Grat-Tabelle als Verdikt, ohne den frischen konditionierten
  Lauf getragen zu haben; was der Lauf nicht als Zahl trägt, bleibt `pending`.
- **Stille ist ein Verdikt.** Fällt der Pfeil unter der Konditionierung aus
  der Signifikanz, ist das die Antwort: die dritte Zeile gehört dem
  geteilten Treiber.
- Die Schwelle ist die bestehende: Residuen-Surrogat / phasenrandomisierte
  Surrogate, mean + 2σ (AGENTS.md — Manifestation breathes with the echo).

## Methode — die vier Blatt-Pflichten binden

Der konditionierte Lauf steht unter den vier Pflichten des Registers
(TODO.md, Nadel-Ⅲ-Abschnitt); ohne sie ist die Messung eine halbe Messung:

1. **Mehrfachvergleichskorrektur über alle getesteten Paare** des Laufs —
   die Zielrichtung Regen→Pegel, die Kontrollrichtung Pegel→Regen und alle
   im Sweep geprüften Konstellationen rechnen gegen dieselbe korrigierte
   Schwelle.
2. **Lag-Sweep** über die Prüf-Lags des Probes (der Quell-Pfeil steht bei
   Lag 24 h; Lag 0 ist kein Sweep).
3. **KDE-Bandbreiten-Sensitivität** (h, Faktor 2) — das Verdikt hält nur,
   wenn es gegen die verdoppelte und die halbierte Bandbreite steht.
4. **Kontrollrichtung des gemeinsamen Treibers** — Konditionierung auf den
   synoptischen Treiber-Proxi (gyirong temperature_2m als Tagesgang-Proxi
   über das Becken; die Zweit-Proxies pressure_msl und
   relative_humidity_2m als unabhängige Bestätigung), Null-Surrogat =
   Residuen-Surrogat (Quelle auf den Treiber regressiert, Residuen
   permutiert, treiber-ausgerichteter Anteil erhalten), mean + 2σ. Getestet
   wird Regen→Pegel; Pegel→Regen läuft als Kontrolle mit.

Der Lauf nutzt das gebaute Instrument der Kreuz-Screenings
(`transfer_entropy_conditional`, `cross_te_screen --cond`); neuer Code ist
nicht Gegenstand dieses Auftrags.

## Die zwei gültigen Verdikte

1. **Der dritte Pfeil festigt sich:** Regen→Pegel übersteht die
   Konditionierung auf den geteilten synoptischen Treiber und schlägt seine
   korrigierte Schwelle — die Grat-Bilanz trägt drei Pfeile komplett
   gemessen.
2. **Kollab-Stille:** der Pfeil fällt unter der Konditionierung aus der
   Signifikanz oder der Lauf trägt keine Zahl — die Grat-Bilanz trägt zwei
   Pfeile komplett gemessen; der dritte war der geteilte Treiber.

## Lieferung

Das Verdikt als Fortschreibung der Grat-Bilanz (Addendum zur Trishuli-Zeile
und zur Grenze „Unkonditionierter Pfeil" in `docs/blatt/blatt-der-grat.md`)
oder als benanntes Blatt, auf das die Bilanz-Zeile zeigt. Die benannte
Stille ist eine Lieferung. Die TODO-Zeile dieses Auftrags wird im selben
Commit geschlossen oder fortgeschrieben. Keine neue Ernte, keine
CDN-Manifestation.

## Abschluss

Der Auftrag ist GESCHLOSSEN, wenn ein konditionierter Lauf steht und das
Verdikt als Bilanz-Fortschreibung oder Blatt kommittiert ist (Stille ist ein
Verdikt). Bis dahin `pending` — dieser Eintrag registriert die Ordnung; die
Ausführung ist nicht Teil der Registrierung.
