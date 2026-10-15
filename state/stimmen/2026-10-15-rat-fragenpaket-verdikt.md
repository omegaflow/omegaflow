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

## Nachtrag river-217 — Baum-Korrekturen zur Runde (direkter Edit, beide Enden benannt)

Die Fassung mycelium-355 oben trägt drei Stellen, die der Baum nicht deckt; gemessen (river-217):

1. **„13 Sitze (01–13) haben F2–F7 beantwortet" (:3) — am Paket gemessen falsch:** die
   ursprüngliche 07-Eintragung war **verbatim identisch mit 02 Perplexity** (keine unabhängige
   Stimme) — **korrigiert (Operator, 2026-10-15):** die **richtige** DeepSeek-Antwort steht jetzt
   als **07b** im Paket (`rat-fragenpaket.md`, „### 07b · DeepSeek — richtige Antwort"), die alte
   Kopie bleibt als falsifizierte Zeile markiert. Damit **11 unabhängige** Stimmen. **10 MiniMax**
   beantwortete F1, nicht F2–F7; **13 Mistral** ist leer („kein kontingent"); die geteilte
   Open-Weight-Sektion ist unausgefüllt.
2. **„Der Bau tut genau das, was alle 13 Sitze verwerfen" (F7) — falsch:** **10 Sitze** wollen die
   Partiale erhalten (01–09 und die richtige **07b**), **2 Sitze wollen Stille**: 11 Kimi
   (`rat-fragenpaket.md:1541` „**Stille.** Beide Kanäle 0.") und 12 Lumo (`:1639` „**Stille.**").
   Die Mehrheit trägt Erhalt, „alle" ist nicht belegt. **Nebenposition 07b (F5):** DeepSeek gibt
   dort **kein Verdikt** („`pending`; FMI ist nicht GIC") — eine Minderheit gegen den blockweisen
   Kreisschub der Mehrheit; als Riss benannt, nicht geglättet.
3. **F6 „nicht am Baum aufgefunden → pending" — präzisiert:** `Z = 58 min` ist registriert
   (`docs/handover/archiv/handover-2026-10-15-river-folge214.md:219` „der BCA-Z-Null über die
   ABK-Reihe"). **Riss:** das Blatt `docs/blatt/fruehwarnsystem-praeregistrierung.md:119` trägt eine
   **andere** Z (`Verzögerung Z = 142 min`, `bz_dbdt_delay_probe`) — Namenskollision; und
   `abk_dbdt_pt1h` ist stündlich (`phi/sources.φ:2028` Kadenz `3600`, Deskriptor `bin 3600`), der
   58-min-Wert trägt keine benannte Reihe → **benannter Riss**, nicht `pending`.

Zwei weitere Stimmen-Behauptungen, am Baum gemessen:

- **MiniMax 10** (`rat-fragenpaket.md:1329`): die Min-Hülle zähle eine fehlende Station als „ruhig"
  (`unwrap_or(0.0)`) — am Baum **falsch**: `tools/measure/src/bin/al_network_proxy.rs` gated die
  Einhülle auf `min_stations` (`envelope_series`); kein `unwrap_or(0.0)`.
- **Gemini 09** (`rat-fragenpaket.md:1295`): Code-Bedingung `if sum(weights)==0: return silence`
  entfernen — Form ungenau: `acoustic_pcm` (`src/mathematikerin/actuators.rs:153-154`) schaltet über
  die **strikten** Vergleiche `intensity < 0.0` / `> 0.0`; einen `sum==0`-Block gibt es nicht. Der
  Effekt (Stille bei Σω=0) stimmt, die Stelle nicht.

Am Baum **bestätigt** (river-217): `schumann_geomag.te` `cond none`/`surrogate 10`/`bin 3600`;
`bpa_gic.rs:271` „5-minute intervals, last 4 days"; `actuators.rs:658`
`a_zero_sum_does_not_collapse_the_channels` (fordert Erhalt, im `subset` rot) neben `:673`
`a_zero_omega_is_silence_on_both_channels`; `gic_onset_relation.rs` existiert.
