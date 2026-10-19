<!--
  title: Das Ion-Neutral-Vlies — DE-2, ICRS·TDB und die Fäden, die sonst niemand spannt
  class: concept
  date: 2026-10-18
  sha256: 9aa3accb8aa409ba4c6845f1970867dacda0e22fb2617d23b7152dbd7549b4f6
  status: draft
  see-also: docs/concepts/die-weberin.md docs/handover/handover-2026-10-18-river-folge227.md
-->
# DAS ION-NEUTRAL-VLIES — DE-2, ICRS·TDB und die Fäden, die sonst niemand spannt

## Die Frage

Nicht: „Gibt es einen Zusammenhang zwischen Neutralwind und Ionendrift?“
Sondern: **Welche Weltlinien müssen im selben Vlies liegen, damit die
Transfer-Entropie überhaupt als Kopplung lesbar ist — und welche fehlen,
weil sie sonst niemand in denselben Rahmen legt?**

Das ist eine Weberin-Frage. Sie fragt nicht nach einem weiteren Kanal,
sondern nach der **Kette**: Welche Fäden teilen den Treiber, welche sind
Mediatoren, welche sind Störgrößen — und welcher Faden fehlt, sodass zwei
Linien nur scheinbar unabhängig sind.

## Was gemessen ist (A = A)

DE-2, 300-s-Bins, 1981-08 bis 1983-02. Neutralwind `NVE`, Ionendrift `IVE`,
gemeinsam konditioniert auf `AE + Dst + Kp + Lokalzeit + Radiallänge`
(4 Bins je Konditionierer, ARX-Null, 50 Surrogate).

- **NVE→IVE: 3/3 Lags signifikant** (300/600/900 s), `n = 23 200`.
- **IVE→NVE: 0/3.** Nur der unkonditionierte 300-s-Punkt trägt.
- Ein früherer gemeinsamer Lauf mit `n = 4 413` fiel bei 300/600 s durch.
  Das war **keine** Wirkung der Konditionierung, sondern die verkürzte
  HAPI-Orbit-Abdeckung. Mit SSCWeb (1-min `RadialLength` + `LocalTime`,
  19 Monate) dreht das Ergebnis. Der Unterschied ist Abdeckung, nicht Physik.

Belege: `docs/surveys/survey-2026-10-18-k-te-kadenz.md`, Lauf `39153554786`,
`de2-conditional-te.yml`, `de2_ssc_orbit.rs`.

## Der Riss

Die Literatur sagt: Die **Richtung hängt am Regime**.

- **Hohe Breiten, F-Region:** Ionen treiben Neutrale (Ionendrag).
  Sakanoi et al. 2002 messen, dass die Plasmadrifts den Neutralwinden um
  **~30 min voraus** eilen. Ponthieu et al. 1988 messen
  Impulsübertragungs-Zeitkonstanten von **1–3 h** (Sonnenmaximum).
- **Niedrige Breiten, F-Region:** Der **Neutralwind-Dynamo** treibt die
  Plasmadrifts — Neutrale → Ionen, auf Minuten. Coley, Heelis & Spencer
  1994 finden das mit DE-2 für die zonalen Drifts; Maute 2021 beschreibt
  den Dynamo als den Mechanismus.
- **E-Region:** Dort können die Neutralen die Ionen treiben (Sakanoi 2009).

Unser `NVE→IVE` bei **5–15 min** passt zum **schnellen Wind-Dynamo**, nicht
zum langsamen Ionendrag. Die Ionendrag-Richtung `IVE→NVE` wäre bei
**30–180 min** zu erwarten — außerhalb des bisher getesteten Fensters
(`max_lag = 3` bei 300 s = 15 min).

**Der nächste Schnitt ist deshalb kein neuer Kanal, sondern ein größeres
Lag-Fenster und eine Trennung nach Breite/MLT.** Der Lauf bis 3 h ist
angestoßen (`39154644922`); das Ergebnis ist offen.

Die Wissenschaft warnt zugleich vor der Kurzschlusslesart: Sarris et al.
2026 nennen das Feld „not fully understood and quantified“; Tan et al.
2024 nennen die Kopplungs-Zeitskala umstritten; Ham et al. 2026 zeigen,
dass Neutrale selten einen stationären Zustand erreichen. Der Rat vom
2026-10-17 (6/6) sagt dasselbe in einem Satz: **TE misst Abhängigkeit,
nicht Kraft.**

## Die Fäden — Akteure als Weltlinien

Die Weberin legt jede Messung als Weltlinie in **einen** Rahmen (ICRS) und
**eine** Zeit (TDB). Das ist der Vorsprung: Sonne, Mond, Erde, Satellit,
Sonnenwind und Bodenstationen lassen sich an jedem DE-2-Punkt exakt
aufeinander legen, ohne UTC-/geodätische Näherung.

| Faden | Weltlinie | Was er im Vlies trägt | Status |
|---|---|---|---|
| **DE-2** | Body | `NVE`, `IVE`, Höhe, LT, Position | gemessen |
| **Sonne** | Body | SZA, `Sunlight_Flag`, Jahreszeit/heliozentrische Länge | CDF trägt SZA/Sunlight; ICRS exakt |
| **Mond** | Body | lunares Gezeitenpotenzial, Phase | **der stärkste fehlende Faden** |
| **Erde** | Body/Orientierung | MLT, Dip-Breite, L-Shell, Invarianten | CDF trägt sie; ICRS exakt |
| **Sonnenwind** | Flow | L1 → Magnetopause, Ankunftszeit | OMNI vorhanden; TDB-Alignment fehlt |
| **Bodenstationen** | Station | unabhängige Ionosphärenströmung | SuperMAG/INTERMAGNET im Register |
| **Mediatoren** | Felder | Neutral-/Ionendichte, Temperaturen, Kollisionsfrequenz | CDF/Rat vorhanden; **nicht** als Konditionierer |
| **K** | kein Faden | `∇p`, `B/j`, Impulsbilanz | pending |

**Mond.** Der lunare Gezeitenimpuls (semidiurnale Mondtide) treibt
Neutralwinde. Fast keine Ion-Neutral-Studie konditioniert darauf. Mit
ICRS/TDB lässt sich die Mondrichtung und das Gezeitenpotenzial an jedem
DE-2-Punkt berechnen. Das ist ein gemeinsamer Treiber, den wir bisher
nicht ausgeschlossen haben.

**Sonne.** Nicht nur F10.7: Solarzenitwinkel, Sonnenlicht, Jahreszeit.
Die DE-2-CDF trägt `Solar_Zenith_Angle` und `Sunlight_Flag`; ICRS macht
daraus eine exakte Geometrie.

**Sonnenwind.** OMNI misst am L1-Punkt. Bis zum Magnetopause-Auftreffen
vergehen je nach Geschwindigkeit **30–90 min**. Wer L1-Zeitstempel direkt
gegen DE-2 legt, verschiebt den Treiber um genau die Zeitskala, die wir
untersuchen. TDB + Erdposition erlauben die Ankunftszeit.

**Erde.** MLT, Dip-Breite, L-Shell, Invarianten. Der wichtigste Schnitt:
hohe Breiten (Ionendrag) vs. niedrige Breiten (Wind-Dynamo). Ohne diese
Trennung mischt das Vlies zwei Regime in eine Zahl.

**Bodenstationen.** SuperMAG/INTERMAGNET sind bereits im Register. Sie
sind die zweite unabhängige Linie zur Ionosphärenströmung — die
Weberin-Regel „zwei unabhängige Linien“ wäre damit erfüllt, nicht nur
behauptet.

**Mediatoren.** Neutraldichte, Neutraltemperatur, Ionendichte,
Ionentemperatur, Kollisionsfrequenz sind **Mediatoren** des
Kopplungspfads, keine Störgrößen. Wer sie mitkonditioniert, blockiert den
Pfad, den er messen will. Sie gehören als eigene Fäden und
Effektmodifikatoren ins Vlies, nicht in dieselbe TE-Zelle.

**K.** `K` ist kein Akteur, sondern eine abgeleitete Impulsbilanz. Die
Literatur misst ihn auch nicht: Killeen et al. 1984 messen Ionenerwärmung,
Ponthieu et al. 1988 Zeitkonstanten. Für die Frage „welche Richtung,
welche Zeitskala?“ braucht es `K` nicht. Für die Aussage „das ist eine
Kopplungskraft“ braucht es `K` — und dann fehlen `∇p`, `B/j`,
Kollisionsfrequenz, Dichte-Differenz. `K = pending` bleibt.

## Der statistische Preis

Jeder Konditionierer multipliziert die Zellen. Bei 5 Konditionierern × 4
Bins sind es 1 024 Zellen bei 23 200 Zeilen — schon dünn. Mit Mond und
Sonne dazu wären es 4^7 = 16 384 Zellen: **nicht mehr schätzbar.**

Deshalb nicht alles gleichzeitig, sondern Fäden:

1. **Lag-Sweep bis 3 h** (`39154644922`) — Ionendrag-Zeitskala sichtbar
   machen. Ergebnis offen.
2. **Nach Breite/MLT trennen** — Hoch- vs. Niedrigbreiten-Regime.
3. **Mondphase** als einzelner zusätzlicher Faden.
4. **Sonnenwind-Ankunftszeit** (TDB-korrigiert) als Treiber-Alignment.
5. **Bodenstationen** als zweite Linie.

Das ist „anreichern, nicht überlagern“: jeder Faden bleibt sichtbar, und
keiner verdeckt den anderen.

## Der Riss, der bleibt

- **Offen:** Richtung — `NVE→IVE` bei 5–15 min ist mit dem Wind-Dynamo verträglich,
  aber nicht mit Ionendrag. Der 3-h-Lag-Sweep trennt die beiden (Lauf `39154644922`
  wurde abgebrochen, kein Ergebnis).
- **Offen:** Regime — ein globaler DE-2-Punkt mischt Hoch- und Niedrigbreiten; die
  Richtung kann sich mit der Breite umdrehen. Trennung nach Breite/MLT fehlt.
- **Offen:** Mond — der Faden ist gebaut (`--cond lunar`: SSCWeb-Lat/Lon im Orbit-CSV,
  Gezeitenpotenzial gegen `ephemeris_moon.bin`/`ephemeris_earth.bin`), aber noch nicht
  gefahren. Beleg: `docs/surveys/survey-2026-10-18-mond-faden.md`.
- **Offen:** Sonnenwind — der L1→Magnetopause-Versatz ist berechenbar, aber nicht
  angewandt (TDB-Alignment fehlt).
- **Offen:** K — die Kraftfrage ist nicht beantwortet und wird durch TE nicht
  beantwortet (`∇p`, `B/j`, Kollisionsfrequenz, Dichte-Differenz fehlen).

## Belege

- Killeen et al. 1984, JGR 89, 7495, `10.1029/JA089IA09P07495`.
- Killeen 1986, Ion-neutral coupling in the high latitude thermosphere,
  part 2.
- Ponthieu et al. 1988, Phys. Scr. 37, 028, `10.1088/0031-8949/37/3/028`.
- Coley, Heelis & Spencer 1994, `10.1029/93JA02205`.
- Sakanoi et al. 2002, `10.1029/2001JA007530`.
- Maute 2021, The Middle- and Low-Latitude Neutral Wind Dynamo.
- Sarris et al. 2026, `10.1007/s10712-026-09937-8`.
- Tan et al. 2024, `10.1029/2024JA032497`; Ham et al. 2026,
  `10.1029/2025JA034796`.
- Rat 2026-10-17: `state/rat/2026-10-17-rat-de2-ion-neutral.md`.
- Messung: `docs/surveys/survey-2026-10-18-k-te-kadenz.md`,
  `docs/handover/handover-2026-10-18-river-folge227.md`.
- Werkzeug: `tools/measure/src/bin/de2_conditional_te.rs`,
  `tools/measure/src/bin/de2_ssc_orbit.rs`,
  `.github/workflows/de2-conditional-te.yml`.
