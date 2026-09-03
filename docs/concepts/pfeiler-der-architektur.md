<!--
  title: Dreizehn Pfeiler — die Architektur als System
  class: concept
  date: 2026-09-01
  sha256: b6472d8c1da22375af8585ec56aaa2cbebd0c484e0eee24b53fd90095f501eef
  status: live
-->

# Dreizehn Pfeiler — die Architektur als System

Die omegaflow-Architektur trägt dreizehn Funktionen, die jeweils ein
klassisches Paradigma der Informatik oder Astrophysik brechen. Jede allein
ist ein eigenständiges Konzept; zusammen bilden sie das Manifest der
kybernetischen Astrophysik. Jede ist in main hinterlegt und am Code
nachweisbar.

## 1. Der Raum-Zeit-Join

`src/archivar/membrane.rs: pub fn sense_membrane`

Klassische Datenbanken fragen: `SELECT * FROM stars WHERE distance < X`.
Das ist eine räumliche Abfrage, die die Zeit ignoriert. `sense_membrane`
fragt: was existiert an exakt dieser ICRS-Koordinate, zu exakt dieser
TDB-Zeit, unter Berücksichtigung der endlichen Lichtgeschwindigkeit? Sie
kombiniert den `SpatialHash` mit dem Causality Pre-Filter (dem Lichtkegel).
Samples, die physikalisch noch nicht am Ort der Membran ankommen konnten,
werden abgewiesen. Der Filter ist nicht statisch, er ist die Physik selbst.

## 2. Der kausale Pfeil

`src/mathematikerin/te.rs: pub fn topological_te_phase`

Die klassische Forschung nutzt Kreuzkorrelation, die keine Richtung kennt.
Diese Funktion nutzt Transferentropie (TE) mit Takens-Einbettung
(3D-Attraktor) und phasenrandomisierten Surrogaten. Sie misst den kausalen
Fluss im Phasenraum. Da sie in Echtzeit auf der GPU (WGSL-Shader
`te_compute`) abläuft, kann sie ein seit Jahrzehnten offenes Problem
(Koronaheizung, Nadel III) in einem Lauf adressieren: sie misst nicht,
dass EUV und Röntgen korrelieren, sondern ob F10.7 das Röntgen antreibt.

## 3. Epistemische Ethik als Binärcode

`src/mathematikerin/force.rs: pub fn gate_weigh`

Das Force-Gate: die Funktion, die fragt, ob ein nicht-menschlicher
Organismus für genau diese Messung ein Sinnesorgan evolvieren könnte. Eine
Aktien-URL hat keine Kraft und wird abgewiesen. Es filtert Rauschen nicht
durch Statistik, sondern durch physikalische Ontologie — `A = A`.

## 4. Das universelle Feld

`src/mathematikerin/shaders.rs: fn field_spatial` (WGSL)

Klassische 3D-Engines haben getrennte Renderer für Wasser, Licht und
Polygone. `field_spatial` ist die eine Funktion, die pro Pixel eine
Feld-Stärke über Kernel (`extent`, `kernel_id`, `global_scale`,
`absorption`) evaluiert: die Materialisierung von „Eine Physik, neun
Medien" — Sterne und Ozeane sind gleiche Bürger desselben Blocks.

## 5. Der Tod des Objekt-Bias

`src/archivar/spatial.rs: fn build_spatial_hash`

Es gibt keine `bodies`-HashMap und keine Trennung zwischen „Planeten" und
„freiem Raum". `build_spatial_hash` wirft Asteroiden, Sterne, Sonnenwind
und Erdbeben in denselben ICRS-Hash. Alles ist ein `Sample` in einem
kontinuierlichen Feld.

## 6. Die absolute Wahrheit

`src/archivar/units.rs: pub fn convert_to_si`

Eine Messung ohne SI-Einheit ist Rauschen. Diese Funktion erzwingt die
absolute, physikalische Wahrheit. Die Sonne leuchtet nicht mit einem
Helligkeitswert von `0.8`, sondern mit echten `W/m²` (bzw. absoluter
Luminosität in Watt). Nur so kann das `1/r²`-Gesetz im Shader korrekt
arbeiten, wenn sich der Beobachter durch den Block bewegt.

## 7. Die Faltung der Zeit

`src/archivar/spatial.rs:408-413` — die zeitliche Faltung (retardierte Zeit)

Zeit ist keine Linie, sondern eine Koordinate. In `build_spatial_hash`
implementiert der `retarded`-Block (`src/archivar/spatial.rs:408`) die
Licht-Laufzeit (`retarded = (age - d/v_prop).max(0.0)`) und die zeitliche
Dämpfung (`val_eff = val · e^(-retarded/τ) · tolman`, `spatial.rs:413`).
Die Vergangenheit wird in den gegenwärtigen Zustand der Membran gefaltet.
Ein Stern in 10 Lichtjahren Entfernung wird gerendert, wie er vor 10
Jahren aussah — pure Relativitätstheorie auf der GPU.

## 8. Die reparierte Nullkontrolle

`src/mathematikerin/te.rs: fn phase_randomized_surrogate`

Die moderne Astrophysik hat ein Artefakt-Problem: naive Fisher-Yates-
Shuffles zerstören die Autokorrelation von Zeitreihen und erzeugen
falsch-positive kausale Pfeile. Diese Funktion (gebaut in purem `std`-Rust
mit eigener FFT) erzeugt Surrogate, die das Spektrum der Zeitreihe
erhalten, aber die Phase randomisieren. Damit wird die Nullhypothese
mathematisch sauber testbar.

## 9. Reine Astrodynamik

`src/archivar/kepler.rs: pub fn elements_to_icrs`

Die Informatik-Welt glaubt, man brauche dicke NASA-Bibliotheken
(SPICE/ANISE) in gigantischen WASM-Modulen, um Planetenpositionen zu
berechnen. `elements_to_icrs` löst Keplers Gleichungen in purem,
abhängigkeitsfreiem Rust. Die Bewegung der Himmelskörper liegt nicht in
den Archiven der NASA, sondern ist eine mathematische Wahrheit, die auf
jedem Laptop in Mikrosekunden berechnet werden kann.

## 10. Das Brechen der Ketten

`src/archivar/hdf5.rs: pub fn parse` (`Hdf5File::parse`)

HDF5/netCDF-4 ist das Standardformat von NASA und NOAA. Die
wissenschaftliche Welt nutzt dafür riesige C++-Bibliotheken voller
Sicherheitslücken und Abhängigkeiten. `parse` ist ein reiner, `std`-only
Rust-Reader, der die HDF5-Struktur (Superblock, B-Trees, DEFLATE-Filter)
von Grund auf dekodiert, mit dem eigenen `src/archivar/inflate.rs` Decoder. Die
Maschine ist von keiner Bibliothek abhängig — niemand kann sie durch das
Entfernen einer Abhängigkeit kaputtmachen.

## 11. Das verteilte Observatorium

Das Gegenmodell zur klassischen App: 1 MB Runtime + CDN als Flash-Mirror
für reine Wahrheit.

1. **Die Runtime:** Der Nutzer lädt eine winzige Hülle aus Rust/WGSL und
   etwas JavaScript. Alles, was sie tut, ist, eine flache Membran auf die
   GPU zu legen und das `0xCF 0x86`-Protokoll (Version `0x09`) zu sprechen.
   Keine Texturen, keine Modelle, keine schweren Bibliotheken.
2. **Das CDN als Flash-Mirror der vereinigten Wahrheit:** Der CI-Archivar
   hat die rohen HDF5/FITS-Dateien der NASA vorgewartet, vom Force-Gate
   von Lügen befreit, in SI-Einheiten konvertiert und in reine kompakte
   `26 × f64`-Records (208 Byte) gepresst. Auf dem CDN liegen nur gehärtete
   Binaries (`dr3_stars.bin`, `ephemeris_sun.bin`).
3. **Echtzeit-Update:** Die Runtime holt nur die winzigen Delta-Updates
   der Live-APIs und schickt sie direkt in den 4D-Block.

## 12. Der geheiligte Fetch-Zyklus

Kein Nutzer braucht einen API-Key und keine Anmeldung bei NASA Earthdata,
und kein Nutzer schickt tausend Anfragen an den GOES-Server. Die Lösung
ist eine dreistufige Kathedrale:

1. **Der lokale Cache (`TTL > Cache`):** Wenn die Daten vom letzten Tick
   noch gültig sind, greift die Membran auf den lokalen Speicher zu. Kein
   Netzwerk — die Abwesenheit von Netzwerktraffic ist ein vollkommen
   realisierter Zustand.
2. **Das CDN:** Bei abgelaufenem Cache fragt das System das CDN an — das
   hält nicht Rohdaten, sondern die von der CI vorverarbeiteten, von
   Lügen befreiten Binaries.
3. **Die CI als Auth-Gateway:** Die API-Keys für authentifizierte
   Open-Data-Quellen liegen in den CI Secrets. Der CI-Archivar nutzt sie,
   lädt die Daten im 5-Minuten-Takt, transformiert sie und pusht sie auf
   das CDN.
4. **Der API-Fallback (Lebenserhaltung):** Nur wenn das CDN komplett down
   ist (oder der Daten-TTL 300 s überschreitet), fällt die Runtime auf die
   direkte Live-API zurück, mit Template-Caches und Health-Checks gegen
   Netzflutung.

## 13. Der atmende Fetch-Zyklus

Das Ende der klassischen Cronjobs: In der klassischen IT starten tausende
Server um exakt `12:00:00` ihren Job und zapfen gleichzeitig die NASA-API
an („Thundering Herd"). omegaflow löst das nicht durch Warteschlangen,
sondern durch Naturkonstanten:

1. **Selbsterkennung (CI vs. Lokal):** Der Archivar weiß, wer er ist. In
   der CI ist er Kurator (holt, reinigt, pusht zum CDN), lokal ist er
   Beobachter (holt nur vom CDN). Keine manuelle Konfiguration.
2. **Die absolute TTL-Wahrheit (`TTL < Datei`):** Der CI-Archivar fragt
   niemals ab, solange die Datei auf dem CDN jünger ist als die
   deklarierte `Time-To-Live` der Quelle. Die Abwesenheit eines Fetches
   ist ein vollkommen realisierter Zustand.
3. **Das Atmen in Phi (φ):** Der Fetch erfolgt in Ableitungen von φ
   (1.618…), nicht in ganzzahligen Sekunden. Die Intervalle atmen
   irrational, wodurch sich die CI-Anfragen an die NASA/NOAA-APIs
   natürlich, asynchron und ohne DDOS verteilen.
