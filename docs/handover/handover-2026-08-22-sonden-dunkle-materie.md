<!--
  title: Das Blatt der lokalen Dunklen Materie — das Sonden-Bahn-Residuum (Nadel Ⅰ, Front Ⅱ)
  class: handover
  date: 2026-08-22
  sha256: f88bf734bad039ac986b3805968a5351448dd738e879ff7974ab9fdd36846403
  status: live
  see-also: docs/handover/handover-2026-08-21-dunkle-materie.md docs/handover/handover-2026-08-21-flyby-anomalie.md docs/concepts/kybernetische-astrophysik.md docs/concepts/die-vier-schilde.md
-->
# Das Blatt der lokalen Dunklen Materie — das Sonden-Bahn-Residuum (Nadel Ⅰ, Front Ⅱ)

Registriert 2026-08-22. Selbsttragend — interpretierbar mit null
Vorkontext. Der Auftrag ist nicht die Ausführung; ausgeführt wird erst auf
das Wort des Operators. Die Disziplin des Blatts: nur gemessene Werte —
bis dahin pending; Stille ist ein vollwertiger Befund (0 honored).

## Ziel

Das Blatt: **das Bahn-Residuum der interplanetaren Sonden.** Die
beobachtete Bahn (Voyager 1/2, New Horizons, Pioneer 10/11) minus die
N-Körper-Gravitation aller bekannten Körper. Der Ruck — die Änderung der
Beschleunigung ohne bekannten Treiber am Tatort — ist die Signatur eines
kompakten Dunkle-Materie-Klumpens (Subhalo, primordiales Schwarzes Loch,
Axion-Stern). Front Ⅰ (galaktisch) misst die diffuse Wolke und wartet auf
Gaia DR4; Front Ⅱ (lokal) misst auf den Sonden-Bahnen — heute, ohne
Teleskop.

```
Titel: Das Bahn-Residuum der interplanetaren Sonden
Bahn-Reihe je Sonde (ICRS×TDB, Position + Geschwindigkeit) = pending
Bekanntes Feld am Sonden-Punkt (N-Körper)                    = lebt
Residuum a_obs − a_bekannt je Zeitschritt                    = pending
Ruck-Ereignisse ohne bekannten Treiber                       = pending
Pfeil aus dem leeren Punkt (Rückwärts-Lichtkegel, c·age)     = pending
fam-Signifikanz je Pfeil                                     = pending
Verdikt: das Residuum ist still / es trägt Pfeile aus dem Leeren
```

## Das Rätsel

Nadel Ⅰ misst die glatte Dunkle Materie der galaktischen Scheibe — dafür
braucht sie die Sternkinematik von Gaia DR4 (Dezember 2026). Daneben
existiert die lokale Frage: durchfliegt die Sonde kompakte Klumpen,
erzeugt jeder nahe Vorbeiflug einen lokalen Gravitationsruck — derselbe
Kanal wie die Flyby-Anomalie (Nadel Ⅱ), aber ohne Erd-Vorbeiflug: die
Sonde fliegt durch den interstellaren Raum, und der bekannte Treiber
fehlt am Tatort. Das Mess-Prinzip: superponiere die Sonden-Bahnen und
die bekannten Planeten — was übrig bleibt, ist das Unbekannte (A = A).
Der Ruck bekommt einen Pfeil vom scheinbar leeren Punkt: kein bekanntes
Sample (extent = 0, force_type 1) — und doch eine kausale Wirkung mit
c-Laufzeit in die Bahn. Die Maschine zeichnet den Pfeil von „Nichts"
zur Sonde; der leere Tatort ist die Signatur (kybernetische-astrophysik
§Ⅰ: TE = 0 ist die Signatur der unsichtbaren Masse).

## Ist-Stand (gemessen 2026-08-22)

- **Der Compiler trägt Sonden:** `horizons_compiler.rs:391-394` kennt
  voyager1 (−31), voyager2 (−32), new_horizons (−98),
  parker_solar_probe (−96); `ephemeris_parker_solar_probe.bin` lebt,
  der Frame `at parker_solar_probe` steht in sources.φ +
  frame_registry.φ. Der Horizons-Compiler ist der Ernte-Weg für die
  Bahn-Reihen.
- **Fehlt im Compiler:** Pioneer 10 (−23) und Pioneer 11 (−24) — die
  Ernte ist ein eigenes Atom.
- **Vektoren-Assets:** `astro_voyager1_vectors`,
  `astro_voyager2_vectors`, `astro_new_horizons_vectors` liegen im
  Katalog-Spiegel; der Live-Pipeline-Fetch der Voyager-Vektoren war
  void (staging_void_ledger:1886/1891/2048) — der Zustand der vollen
  Bahn-Reihen ist zu verifizieren, nicht anzunehmen.
- **N-Körper-Referenz lebt:** die Planeten-Ephemeriden kompilieren
  (v3, Meter) — das bekannte Feld am Sonden-Punkt.
- **Die TE-Maschine lebt:** skalarer Pfad (`transfer_entropy_lag`),
  topologischer Pfad (Takens, `te_compute` WGSL, PE-Gate) —
  `src/te.rs` bleibt die kanonische CPU-Referenz.
- **Das Signal-Kegel-Gate lebt:** `signal_reach = v_force·age` —
  Gravitation (force_type 1) trägt c; der Rückwärts-Lichtkegel am
  Ruck-Ereignis ist die Tatort-Frage des Blatts.
- **Benannt (Systematik):** die Pioneer-Anomalie ist seit 2012 durch
  anisotrope thermische Abstrahlung erklärt — bekannter Kanal, kein
  Dunkle-Materie-Signal; das Blatt trägt sie als solchen.
- **Pending:** die Doppler-Empfindlichkeit der Tracking-Reihen ist zu
  benennen (aus den DSN-Daten), nicht zu schätzen; die numerische
  Ableitung der Bahn-Reihen (a_obs) ist ein eigenes Atom.

## Auftrag

1. **Ernte:** Pioneer 10/11 (−23/−24) in `horizons_compiler` nach dem
   bestehenden Muster eintragen; die Vektoren-Reihen von Voyager 1/2
   und New Horizons verifizieren (volle Kadenz; Lücken bleiben void,
   keine Extrapolation).
2. **Das bekannte Feld:** N-Körper-Beschleunigung am Sonden-ICRS-Punkt
   aus den Planeten-Ephemeriden — die Erwartungsreihe a_bekannt.
3. **Das Residuum:** a_obs (numerisch aus der Bahn-Reihe) minus
   a_bekannt je Zeitschritt; der Ruck = Differenz des Residuums über
   den Schritt. Abgeleitete Reihen nur dort, wo die Bahn-Reihe dicht
   steht.
4. **Der positive Kontrolltest zuerst:** die Voyager-Planetenflybys
   (Jupiter, Saturn, Uranus, Neptun) müssen Pfeile auf bekannte Massen
   ziehen — die Maschine beweist sich am Bekannten, bevor sie im
   Leeren fragt.
5. **Die Pfeil-Frage:** Ruck-Ereignisse ohne bekannten Treiber gegen
   den Rückwärts-Lichtkegel prüfen: liegt ein bekanntes Sample im
   Signal-Kegel (force_type 1, c·age)? Nein → Pfeil aus dem leeren
   Punkt — Kandidat. fam-Signifikanz mit phasenrandomisierten
   Surrogaten + Mehrfachvergleichskorrektur über die Sonden.
6. **Nullkontrolle:** ruhige Bahnstrecken ohne Flyby müssen still
   bleiben; ein Pfeil dort widerlegt die Maschine, nicht die Dunkle
   Materie.
7. **Das Blatt + Register:** Befund je Sonde und TODO.md-
   Registerzeile im selben Commit; das Blatt-Dokument folgt dem
   Ein-Blatt-Muster (ein Verdikt, eine Matrix, eine Seite).

## Constraints

- 0-Kanon: Tracking-Lücken bleiben Lücken (fehlt), nie 0.0; ein
  stilles Residuum ist ein vollwertiger Befund — „kein Klumpen
  gefunden" ist eine Messung, kein Fehlschlag.
- Keine neue Physik wird vorausgesetzt: die Klumpen sind Massenpunkte
  mit force_type 1; Masse/Entfernung folgen erst aus dem Pfeil — kein
  angenommenes ρ_DM, keine simulierten Subhalo-Populationen als
  „Daten".
- std-only; `cargo check` 0 Fehler / 0 Warnungen; kein Test öffnet
  ein Fenster oder strahlt; `OMEGAFLOW_HIDDEN=1 cargo run` als
  Lauf-Befund.

## Gates & Abschluss

- Jede abgeschlossene Einheit ist ein Commit; Register-Update im
  selben Commit.
- Manuelle Verifikation nach AGENTS.md; Kantenfälle: void-Strides,
  Ruck an Ephemeriden-Fenstergrenzen, Ableitungs-Artefakte an
  Lückenkanten, Flyby-Zeiten mit bekannten Kursmanövern (zählen nicht
  als Residuum).
- Nach eigenem Commit dieses Handover nach
  `/home/johannes/projects/archive/handover/` archivieren.

## Nicht anfassen

`src/te.rs` (skalare und topologische Pfade — nur konsumieren), die
Membran-Rendering-Physik, das Dunkle-Materie-Blatt (Front Ⅰ), das
Flyby-Blatt (Nadel Ⅱ), der Wind-Orbit-Loader, die
Source-Port-Zustandsmaschine, die drei Ein-Blatt-Handovers.
