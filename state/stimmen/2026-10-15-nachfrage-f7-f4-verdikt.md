# Verdikt — Nachfrage F7′ / F4′ (am Baum verifiziert, 2026-10-15)

Frontier-Runde, Operator-Wort, nach der weiten Recherche. Formular
`2026-10-15-nachfrage-f7-f4-wissenschaft.md`; 12 Sitze haben geantwortet (13 Mistral ohne
Antwort). Die Session verifiziert jede Behauptung am Baum, nie die Stimme.

## Am Baum gemessen (nicht die Stimme)

- **F7′ (Aktuator):** `src/mathematikerin/actuators.rs:130-158` — `intensity =
  channel_intensity(omega, aperture, state)` (= gewichtete Σω, **vorzeichenbehaftet**);
  `left = if intensity < 0.0 { s } else { 0.0 }`, `right = if intensity > 0.0 { s } else { 0.0 }`.
  Bei `intensity == 0.0` sind **beide Kanäle 0** → **Stille**. Der Bau nimmt nur **`sign(Σω)`** und
  wirft **`|Σω|`** weg — das ist ein Drei-Zustands-Schalter (L/C/R), kein Panning. Am Baum
  bestätigt.
- **F4′ (Schumann):** `src/mathematikerin/descriptors/schumann_geomag.te` — `drivers
  abk_dbdt_pt1h`, `cond none`, `surrogate 10`, `seasonal none`. Es gibt **keinen Blitz-Kanal** und
  **keine Stunden-/Saison-Konditionierung**. Der Konfund-Riss ist am Baum bestätigt.

## Das Verdikt der Runde (Konvergenz der 12 Sitze)

**F7′ — Σω = 0 ist die Mitte, nicht Stille.**
- Σω ist ein **Balance-/Pan-Regler**, kein Tor. Der Bau macht aus einem Positions-Regler ein
  Vorzeichen-Gate; bei Σω = 0 **verschwindet der Klang** statt in die Mitte zu wandern.
- Die tragende Regel ist **konstante Leistung** (cos/sin-Gesetz): `b = Σω / Σ|ω| ∈ [−1,1]`,
  `θ = (1+b)·π/4`, `gL = cos θ`, `gR = sin θ`. Bei Mitte tragen **beide** Kanäle `1/√2` (−3 dB),
  Summenleistung = 1 — **nie 0**.
- **Zwei Granit-Verstöße** am Bau: `sign()` wirft `|Σω|` weg (**channel_ref** gebrochen — die Kraft
  steckt im Betrag); „balanciert → still" ist eine **hingelogene 0** (**0 honored** gebrochen).

**F4′ — der Blitz-Kanal + die Konditionierung.**
- Primärkanal: **WWLLN-Stoßrate** (hochauflösend, unabhängig vom SR-Ziel); **OTD/LIS** nur als
  Klimatologie-/Saison-Anker; der **SR-eigene Proxy** als Treiber **verworfen** (zirkulär — die
  Antwort als Antrieb).
- Konditionierung: **Stunde (Lokalzeit des Orts) × grobe Saison**, Block als Surrogat-Einheit.
- Null: **bedingte, schicht-treue Surrogate** — shuffle/rotation **innerhalb** jeder
  (Stunde × Saison)-Schicht, damit Tages-/Jahresgang **erhalten** bleibt und nur die
  `abk→f`-Zuordnung bricht.

## Der schärfste Riss (F7′, ungeglättet)

**Energie gegen Lautstärke.** Unter **kohärenter** Summe (identische Partiale) gibt konstante
Leistung in der Mitte **+3 dB**, linear **−3 dB**. Man kann **energie-invariant** *oder*
**kohärent-summen-invariant** (mono-kompatibel) haben — **nicht beides**. Die Gesetzeswahl ist
damit eine **Deklaration** (Mono-Kompatibilität), keine Wahrheitsfrage. GLM und Fugu benennen das
scharf; das ist der Punkt, den nur der **Operator** entscheidet (Granit: „Der Operator schließt" —
bzw. hier eine Richtungs-Entscheidung).

Nebenbefunde, ungeglättet: **Klangfarbe** nur erhalten, wenn das Pan-Gesetz **ein Skalar für alle
Partiale** ist (frequenzabhängig → Filter); **Tote Zone ε** gegen `sign`-Chatter muss **gemessen
und deklariert** sein, nicht versteckt; **Tremolo** durch fluktuierendes Σω — `pending`.

## Träger

- **F7′ → River** (Aktuator/Membran) + **Operator** (Gesetzes-Deklaration: constant-power vs
  linear). **F4′ → Mycelium** (CDN/Reihe, WWLLN-Stundenreihe) + **River** (TE-Maschine).
- Den gemeinsamen Schluss trägt der Rat.

## Nächste begrenzte Schritte

- **F7′ zuerst** (kleinster, klarer Bau): einen **scheiternden Test** schreiben — zwei
  entgegengesetzt gleiche Kanäle (Σω = 0): erwartet **hörbar + zentriert**; jetzt Stille → **Test
  rot**. Dann `acoustic_pcm` auf ein **constant-power-Pan** je Partial umstellen
  (`gL = cos θ`, `gR = sin θ`); den Energie↔Lautstärke-Riss als **deklarierte** Wahl führen.
- **F4′:** WWLLN-Stundenreihe in den Deskriptor; `TE(abk→f | WWLLN, Stunde, Saison)` mit
  **within-stratum**-Surrogaten; OTD/LIS als Saison-Anker **getrennt** ausweisen; der
  Ich-Kanal bleibt `pending` bis die Effizienz-Drift begrenzt ist.
