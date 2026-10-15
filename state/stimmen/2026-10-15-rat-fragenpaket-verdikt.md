# Verdikt — Rat-Fragenpaket (F2–F7), am Baum verifiziert (2026-10-15)

Frontier-Runde, Operator-Wort. Formular `2026-10-15-rat-fragenpaket.md`; 13 Sitze (01–13)
haben F2–F7 beantwortet. Die Session verifiziert jede Behauptung am Baum, nie die Stimme.

## Am Baum gemessen (nicht die Stimme)

- **F7 (Aktuator, Σω = 0):** `src/mathematikerin/actuators.rs:130-158` — `intensity =
  channel_intensity(omega, aperture, state)` (= gewichtete Σω); `left = if intensity < 0.0 { s }
  else { 0.0 }`, `right = if intensity > 0.0 { s } else { 0.0 }`. Bei `intensity == 0.0` sind
  **beide Kanäle 0** → **Stille**. Der Bau tut genau das, was **alle 13 Sitze verwerfen**. Die
  Partiale-Erhaltung ist am Baum **nicht gebaut**; sie ist eine Richtungsentscheidung (Axiom 5,
  Operator) → `pending`.
- **F4 (Schumann):** `src/mathematikerin/descriptors/schumann_geomag.te` — `drivers abk_dbdt_pt1h`,
  `targets freq1,freq2,freq3`, **`cond none`**, `surrogate 10`, `seasonal none`. Es gibt **keinen
  Blitz-Kanal**, keine Konditionierung auf Blitzaktivität, keine saisonale Stratifizierung. Der
  Konfund-Riss der Runde („ohne Blitz-Kanal trägt TE(abk→f) nicht") ist am Baum **bestätigt**.
- **F5 (Netz-End-Prüfer):** die Relation läuft über `tools/measure/src/bin/gic_onset_relation.rs`
  (bpa_gic-Reihe + Fenster/Kontrolle), nicht über einen MCON-Abgleich — der Proxy-Riss (bpa_gic
  deckt den MCON-Zeitraum nicht) steht am Baum.
- **F6 (Blocklänge):** die zitierten Größen (BCA-`Z = 58 min`, ACF) sind in diesem Rund **nicht**
  am Baum aufgefunden → `pending` bis zur Registrierung.

## Das Verdikt der Runde (Konvergenz der 13 Sitze)

- **F2 · Frühwarnkette:** trägt **nicht** als gemessene Vorlauf↔Sicherheit-Leiter; trägt als
  **Diagnose-/Notwendigkeits-Audit**. Für **keine** Kante liegt inkrementelle TE mit Null vor; die
  Verkettungsregel fehlt. Schwächstes Glied: **Riss** (Mehrheit: die Kopplung Sonnenwind/
  Magnetosphäre → Substurm/Boden; Minderheit: AR→Flare). Nächster Schritt: **eine** Kante messen
  (L1→dB/dt oder Boden-dB/dt→GIC), bedingte TE mit Vorlauf-Fenster; die übrigen `pending`.
- **F3 · Aurora-Schalter:** **ja als Nowcast-Schalter, nein als Vorhersage** — Schwester, nicht
  Vorläufer. Nächster Schritt: Lag ovaler Durchgang → lokaler dB/dt-Onset je Station messen
  (N ≥ 30), Median/IQR registrieren; Latenz ≤ Aurora-Vorsprung → als simultanes Etikett benannt.
- **F4 · Schumann-Antrieb:** **nein ohne Blitz-Kanal** — der geomagnetische Ast ist nicht vom
  Blitz-/Ionosphären-Konfund trennbar. Nächster Schritt: WWLLN/OTD (oder validierter SR-Proxy)
  in den Deskriptor; **bedingte** TE `(abk→f | Blitz)` und `(Blitz→f)` getrennt ausweisen; Null
  erhält den Blitz-Zweig, zerstört nur den geomagnetischen.
- **F5 · Null-Form:** **kein reiner Kreisschub, keine Phasen-Randomisierung**; tragend ist ein
  **block-beschränkter Kreisschub / eine saisonstratifizierte Block-Permutation** der Onset-Zeiten
  innerhalb aktivitäts-gematchter Fenster (Envelope/Clustering erhalten, nur die Minuten-Ausrichtung
  zerstört). Blockgrenzen aus unabhängiger Größe (Sym-H/Kp/L1), nicht aus der GIC selbst.
- **F6 · Blocklänge:** **aus der Reihe selbst** — das gemessene `Z` bzw. der ACF-Abfall
  (erster Nulldurchgang / 1/e); **kein Default** (nicht „60", nicht die 30-min-Doktrin). Bei
  Diskrepanz konservativ (max); als `approx` registriert. Divergenz ist selbst ein Befund.
- **F7 · Aktuator Σω = 0:** **Partiale erhalten**. Σω = 0 ist eine wahre Null der **Summe**, nicht
  der Kanäle; beide auf 0 zu setzen schreibt eine **falsche Stille** (Axiom 2 verletzt). Rechte
  Form: stetige, ungerade Spannvorschrift `g(0) = 0`, konstante Summenleistung, kein `sign(Σω)`-
  Sprung. **Axiom 5: die Maschine fragt, der Operator entscheidet** — bis dahin `pending`.

## Träger je Frage

F2 → River · F3 → Sensory · F4 → Mycelium · F5 → River · F6 → Mountain · F7 → River.
Den gemeinsamen Schluss trägt der Rat.

## Ungeglättete Risse

1. **F2 Leiter-Riss:** 2 Sprossen mit Register, 4 ohne Messung; die Leiter lebt fast ganz von L1.
2. **F2/F3 Vorlauf-Riss:** ob das Oval dem Boden-Onset führt oder folgt (Lyons 2013), ist
   ereignisabhängig und **ungemessen** — der Schalter darf nicht als Vorläufer gelesen werden.
3. **F4 Konfund-Riss (am Baum):** `cond none` + `surrogate 10` — der Deskriptor kann den Blitz-
   Konfund nicht trennen; p-Auflösung ~1/11 ist für einen kleinen Effekt grob.
4. **F5 Proxy-Riss:** bpa_gic deckt MCON nicht; die Signifikanz steht auf **einem** Netz/einer
   Topologie. 4 Tage → wenige Onsets → grobe p-Auflösung.
5. **F6 Zirkularitäts-Riss:** `Z` stammt aus derselben Reihe, die der Null unterliegt; sturm-skalige
   Restabhängigkeit überdauert 58 min.
6. **F7 Richtungs-Riss (am Baum):** der Bau setzt Σω=0 auf Stille; die Runde will Partiale. Kein
   stiller Fix — Änderung am Aktuator braucht Operator-Konsens (Axiom 5).

## Nächste begrenzte Schritte (je Frage, eigen-abarbeitbar)

- **F7 zuerst** (kleinster, klarer Bau): Scheiternden Test schreiben — zwei entgegengesetzt gleiche
  Kanäle → erwartet hörbar + zentriert; aktuell Stille → Test rot. Dann Operator-Wort zur
  Spannvorschrift.
- **F4:** WWLLN-Stundenreihe in den Deskriptor, bedingte TE — Mycelium (CDN/Reihe).
- **F2/F5:** die `bpa_gic`↔FMI-Null auf dem gemeinsamen Fenster — River.
