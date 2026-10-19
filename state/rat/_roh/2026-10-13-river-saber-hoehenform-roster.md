# Stimmen-Runde — SABER-Höhenform (Ratsfrage, River-204/206) — 2026-10-13

Frage: Wie tritt eine höhenaufgelöste geophysikalische Größe (Temperatur, 400 Höhenstufen je
Scan, Höhenraster variiert je Scan) in ein System ein, dessen Datensatz EIN skalarer Wert je
Zeitstempel ist (Wire = 26×f64; Achsen nur als geschlossene diskrete Klassen)? (A) feste
Höhenstufe als Skalar · (B) ganzes Profil als Achsenobjekt · (C) Höhe als Koordinate je Record.

Runde: Forschungsschicht (`general`-Taucher, Standard-URLs) + Lokalrat (`council`, flash) +
UI-Frontier (river-ui: Z.ai GLM-5.3 · Claude Sonnet 5.5 Extra hoch · Qwen3.7-Plus · Duck.ai
GPT-6 Luna) + geteilter Open-Weight-Seat (tryingopen DeepSeek V4 Pro; Lock river).

## Forschungsschicht (gemessen am Web, `general`)
- **CF:** die Vertikale ist eine **Koordinatenvariable** mit Dimensionsachse
  (`positive="up|down"`), nicht eine Record-Koordinate — `cfconventions.org` §4.3.
- **CF-Profile H.3.2/H.3.4:** SABER-Fall = *incomplete multidimensional* `alt(profile,z)`;
  Ragged-Array `z(obs)` (per-Record) nur **mit** Kopplungsvariable ans Profil.
- **ISTP/CDF:** Zeit = Record-Dimension; alle anderen Dimensionen sind **Variablen-Dimensionen**
  mit Begleit-`support_data` über `DEPEND_i` (FLUX/ENERGY = 1-D der Größe 8, nicht 8 Records).
- **HAPI:** Zeit = einzige unabhängige Achse; Array-Parameter tragen `size:[400]` + Begleit-
  parameter (`tpgpaltitude`, km) bzw. `bins`. SABER nutzt den Begleit-Parameter, kein `bins`.
- **SABER konkret:** `ktemp` ist `size:[400]`, `tpgpaltitude[400]` ist die zeitabhängige
  Höhenkoordinate (gemessen: `[0]` variiert je Scan 150,38–150,59 km) — Raster **nicht** global fest.
- **Baum-Präzedenz:** `GeoRec.alt: f64` ist per-Record-Koordinate (`src/archivar/geo.rs:293`);
  IGRA macht genau das Vorgeschlagene (`extract.rs:4506-4574`, `igra.rs`).

## Rat (`council`, flash)
Verdikt **(C)**; (B) ist grammatisch nicht ausdrückbar (nur geschlossene Achsenklassen, keine
kontinuierliche Höhe); (A) verkauft 399 Stufen als stillen Null (Sensory/Mountain-Abweichung
benannt). Höhe als Record-Koordinate, kein `field` je Höhenstufe. Offen: Einbettung der 400
Höhen in die ICRS-Position (lokale Vertikale) — eigener Kontrakt-Punkt.

## UI-Frontier (gemessen am lebenden Chrome; Rohmaterial)

### Claude (Sonnet 5.5 Extra hoch) — Form (C), aber sauber umgesetzt
„Das Level (0…399) ist die geschlossene diskrete Achse, und die Höhe läuft als zweite, gleich
indizierte Größe mit, nicht als Achse. Das ist das CF/xarray-Muster der Hilfskoordinate
`altitude(time, level)`." **Riss:** Ein Record **kann nicht „seine eigene Höhe tragen"** — bei
einem Skalar je Record gibt es dafür keinen Platz. Die Höhe muss eine eigene Reihe sein, die
denselben Schlüssel (Zeit, Level) teilt; das Level muss Teil des Schlüssels sein, sonst
überschreiben sich die 400 Records mit identischem Zeitstempel. Der Level-Index ist **keine
physische Identität** — jede Aggregation entlang des Level-Index ist stillschweigend falsch,
solange nicht vorher über die Höhenreihe auf ein gemeinsames Raster umgerechnet wird.

### Duck.ai (GPT-6 Luna) — Form (C)
„(C) — Höhe als Koordinate je Record, mit Level-Index als Dimension: so bleibt der Skalar je
Record erhalten und die scanabhängige Höhe explizit mitgeführt." **Riss:** „Das ist ein
**abgeflachtes Profil, kein natives Profilobjekt**; gleiche Zeitstempel und rekonstruierbare
Level-Gruppierung müssen erhalten bleiben." Quelle: cfconventions.org.

### Qwen (Qwen3.7-Plus) — Form (A) (Abweichung)
„Nur die Extraktion eines festen Levels erfüllt die harte Wire-Vorgabe von exakt einem
Zahlenwert pro Zeitstempel." **Riss:** dimensionaler Riss (erzwungener Informationsverlust).
Anmerkung der Session: Qwen liest den Kontrakt als starr — die 400 Records je Scan sind im
Wire zulässig (der Wire trägt nur die Identität `(time, level)` nicht explizit; vgl. Claude).

### Z.ai (GLM-5.3-Flash) — antwortet nicht innerhalb der Frist (`pending`)
Beim Lesen nach ~5 min noch „Thinking…". Kein Abbruch, aber die Antwort lag zur Abnahmezeit
nicht vor — als `pending` benannt, nicht als stille Null.

### DeepSeek V4 Pro (tryingopen, geteilter Seat) — antwortet nicht innerhalb der Frist (`pending`)
Beim Lesen nach ~5 min noch in der Reasoning-Kette (kreiste um den Kontrakt), noch keine
Endantwort. Als `pending` benannt.
