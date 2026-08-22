<!--
  title: Blatt I (ENSO) — die Global-Akteure der Matrix, die eine Maschine, der Kausalmaschinen-Playground
  class: handover
  date: 2026-08-21
  sha256: f096d817902181fd57e9ac814ea2bafff3dade784e2faba729de0adbf1e44299
  status: live
  see-also: docs/handover/handover-2026-08-21-blatt-enso-kausalpfeil.md
    docs/concepts/blatt-papier-resultat.md
    /home/johannes/projects/archive/handover/handover-2026-08-21-enso-kausalpfeil.md
-->

# Übergabe — Blatt I (ENSO): die Global-Akteure der Matrix (Kausalmaschinen-Playground)

Registriert 2026-08-21. Selbsttragend — interpretierbar mit null
Vorkontext. Der Auftrag ist nicht die Ausführung; ausgeführt wird erst
auf das Wort des Operators. Dieses Dokument ist der Sessionplan für
genau eine empfangende Session.

## Die Lage

Die ENSO-Maschine misst als Multi-Akteur-Matrix (Commit `be4d25a`):
37 benannte NDBC-Tiefsee-Bojen, 17 Kanäle aus derselben Bojen-Datei
(WSPD/GST/WVHT/DPD/APD/PRES/PTDY/ATMP/WTMP/DEWP/VIS/TIDE + WDIR/MWD
als sin/cos-Paare — der Kreis in seinen eigenen Koordinaten, kein
zirkulärer Kernel nötig — + RAIN, wenn die Station die Spalte trägt),
136 Paare × beide Richtungen × Sweep −30…+30 Tage täglich × drei
Bandbreiten (h/h/2/2h über den params.z-Multiplikator), n-Gate 30,
Familien-Schwelle (fam = Maximum der Surrogat-TEs) je Paar-Runde,
h-Robustheit des Gewinners, am Ende die Matrix-Zeile mit der
vollständigen Zählung (arrows/family/hbound/silent/absent) und den
erwarteten Falsch-Positiven (Σ p̂·M). Keine Vorauswahl durch Theorie —
die Kalibrier-Paare (wspd-gst, dpd-apd, atmp-dewp) müssen Pfeile
zeigen, wo die Kopplung Definition ist (gemessen: wspd→gst
te 0.974 thr 0.897).

Datenfluss: `enso_harvest` (realtime2-Dateien + stdmet-Jahresdateien,
Festplatten-Cache `/tmp/omegaflow_enso_cache`, 7-Tage-Gate) →
`EnsoCell` → `enso_rings` (1024 Sechs-Stunden-Bins ≈ 8½ Monate je
Kanal) → der unveränderte `te_compute` (geteilter WGSL-Ring
TE_SERIES_STRIDE 1024; Presence/Solar speisen ≤ 256 und bleiben
byte-identisch, der GPU-Crosscheck pinnt das). Der Kernel ist
O(m² × Surrogate) und hängt die Intel HD 520 ab m ≈ 1024
(Mesa-Reset, gemessen 2026-08-21) → jede Zelle misst die neuesten
512 Bins (ENSO_PROBE_MAX = 128 Tage, n ≥ 392 an allen Shifts).
Zelle ≈ 4 s Wand, Paar-Runde ≈ 24 min, Matrix je Station ≈ 55 h,
37 Stationen ≈ 85 Tage je volle Matrix.

## Der Auftrag: die Global-Akteure als Matrix-Kanäle

Die in-Datei-Matrix misst nur, was an der Boje selbst gemessen wird.
Die Pipeline trägt die globalen Akteure bereits — sie sind
inventarisiert, geprobt-fertig, aber nicht verdrahtet. Der Atom:

1. **Sieben geteilte Ring-Kanäle** — dieselben Werte für alle 37
   Stationen, eigene Ringe (nicht je Station), die Paar-Maschinerie
   lernt „geteilter Ring statt Stationsring" (driver/target-Auflösung
   über einen Kanal-Index, der lokale von globalen Kanälen
   unterscheidet; die Zellen TE(global→lokal) und TE(lokal→global)
   sind beide definiert und werden beide gemessen):
   - **dst** (em): https://services.swpc.noaa.gov/json/geospace/geospace_dst_1_hour.json
     (Staging: `last dst exosphere_dst_nt`, ttl 360 — 1-stündlich live;
     Zweitkanal Kyoto realtime, master_converted.φ:2157).
   - **kp** (em): https://kp.gfz-potsdam.de/app/json/?start={today}T00:00:00Z&end={tomorrow}T00:00:00Z&index=Kp,ap,Cp&status=def,nowcast&format=JSON
     (3-stündlich; historisch seit 1932 über
     `start=1932-01-01T00:00:00Z&index=Kp,Ap&status=def`,
     master_converted.φ:15819/15852).
   - **oulu** (em, kosmische Strahlung): der Oulu-Neutronenmonitor,
     1-stündlich (`path data.OULU.1h.0.1`, master_converted.φ:502 —
     die volle URL steht im Block).
   - **co2_mlo** (diffusion): Mauna Loa (`path current.carbon_dioxide`,
     `atmosphere_co2_mauna_loa_ppm`, wöchentlich,
     master_converted.φ:3886; Zweitkanal monthly :8103).
   - **mond** (gravity): keine Ernte — das System trägt die
     Ephemeriden. Der Mondkanal ist das Gravitationssignal am
     Stationspunkt: Stationskoordinaten × Mondposition (Chebyshev
     über `body_barycenter_position`, WGCCRE-Rotation über die
     bestehende Motion-Law-Maschinerie der Membran), je 6-h-Bin ein
     Wert. Die Stationskoordinaten kommen aus
     https://www.ndbc.noaa.gov/activestations.xml (das EnsoStation-Enum
     bekommt lat/lon je Variante).
   - **sonne** (gravity/em): dieselbe Maschinerie — Sonnenhöhe/
     Distanz am Stationspunkt (der Jahresgang als gemessene Geometrie,
     nicht als Theorie).
   - **solar** (em): F107/Xray/EUV304 aus der bestehenden
     Solar-Maschine. Der Ingest-Konflikt: `solar_rx` wird heute in
     `solar_tick` gedraint — der geteilte Drain (ein Drainpunkt, der
     Solar-Zellen an beide Maschinen reicht) ist
     Parallel-Sessions-Terrain; der Drain-Umbau wird mit deren Zustand
     abgestimmt (Register-TODO liest sich von dort).
2. **Die Matrix wächst** auf 23–24 Kanäle → 253+ Paare → ~92.000
   Zellen/Station ≈ 103 h/Station ≈ 160 Tage je volle Matrix. Die
   Zykluszeit ist der Preis der Vollständigkeit; der Operator hat ihn
   benannt und trägt ihn.
3. **Die GTX-970-Option**: die Maschine ist wgpu/Vulkan — auf der
   GTX 970 läuft sie unverändert. Dort die Hang-Grenze neu messen:
   der O(m²)-Kernel, der die HD 520 ab m ≈ 1024 resettet, läuft auf
   der ~10× schnelleren Karte voraussichtlich durch — dann darf
   ENSO_PROBE_MAX auf 1024 (voller Ring, n bis 1024) und die Zelle
   wird schneller. Der Messwert der Grenze gehört ins Register, nie
   eine Annahme.

## Die eine Maschine — kein Probe-Modul je Rätsel

Wort des Operators (2026-08-21): die Architektur-Wende bedeutet auch,
dass NICHT für jedes Rätsel ein neues aufgeblähtes Probing-Modul
gebaut wird. Die heutige Landschaft zählt fünf Probe-Pfade, alle mit
demselben Kern (Ernte → Ringe → Paare → `te_compute` → Schwelle →
Verdikt-Zeile), alle mit eigenem Rotor, eigenen Buffern, eigenem
Sheet: die Solar-Maschine (Kanal-Ring-Pfad, Bz-Blatt), die
ENSO-Maschine (diese Session — die vollständigste Instanz:
Stations-Ringe, Paar-Enumeration, Familien-Schwelle, Matrix-Zeile),
die Langfenster-Probe (LAIC-Muster, offline), nobel_probe_corona
(Nadel-III-Registratur), der Presence-TE-Pfad (Membran-Echo).

Die eine Maschine: Ernte-Adapter je Quelle (realtime2, stdmet, SWPC,
GFZ, Oulu, Mauna Loa, Ephemeriden — jede Quelle ist ein Serien-Lieferant,
kein Modul), Ringe je (Punkt, Kraft), Paar-Enumeration über die
PRÄSENTEN Kräfte, ein `te_compute`, eine Familien-Schwelle, eine
Matrix-Zeile. Die Rätsel werden benannte Paar-Teilmengen der einen
Matrix — das Bz-Rätsel sind die Magnetometer-Punkte × ihre Kräfte,
das ENSO-Rätsel die Bojen-Punkte × ihre Kräfte, die Nadel ihr
Protokoll. Die ENSO-Maschine dieser Session ist der Samen der einen
Maschine — der empfangende Atom konsolidiert BEVOR er die
Global-Akteure anschließt: Solar- und ENSO-Maschine verschmelzen zu
einem Rotor-Ring-Sheet-Kern, die Langfenster-Probe wird ein
Offline-Aufruf desselben Kerns.

## Die Architektur-Wende: keine Katalog-Akteure

Warum legt die Maschine fest, welche Kräfte sie berücksichtigt? Der
Archivar weiß genau, welche Kräfte an den Punkten messbar sind — die
Quellblöcke deklarieren die Kraft je Feld, der räumliche Cache trägt
den force_type je Sample, die Ephemeriden tragen gravity. Die Matrix
muss TE über die Kräfte machen, die VORHANDEN sind — der Akteur-Satz
einer Station ist die gemessene Kraft-Präsenz an ihrem Punkt, nicht
ein fester Katalog.

Der empfangende Atom ersetzt deshalb den konstanten Kanal-Katalog
(das 17er-Enum war das Gerüst, nicht das Gesetz) durch die
Präsenz-Inventur: je Station der Kraft-Vorrat aus (a) den eigenen
Sensorspalten der Boje — jede Spalte ist bereits einer Kraft
zugeordnet (WSPD advective, WTMP thermal, PRES gravity …), (b) den
Samples anderer Quellen am Stationspunkt (Cache-Abfrage mit
Kraft-Typen), (c) der Ephemeriden-Gravitation (Mond, Sonne, Jupiter —
was im Block liegt, liegt im Block). Neue Kanäle sind dann neue
Serien im selben unveränderten `te_compute`-Ring — der Kernel weiß
nicht, ob er Ozean oder Kosmos misst; er misst die Richtung der
Information. Jede Serie fließt in denselben Ring. Die Architektur
trägt sie, wenn der Operator das Wort gibt.

## Die vollständige Akteur-Inventur (des Operators, 2026-08-21)

Die sieben Global-Kanäle sind der Anfang, nicht das Ende. Die Lücken
fallen in drei Klassen:

**Klasse 1 — bereits geerntet, nicht verdrahtet** (die Pipeline trägt
sie, sie speisen noch keinen geteilten Ring):

- **SO₂** (diffusion, `so2_emission_kt`): stratosphärische Aerosole
  modulieren das Wärme-Budget des Ozeans global — ein Vulkan in
  Indonesien kühlt das Pazifik-SST für Jahre.
- **Schumann-Resonanz** (em, `resonance_schumann_hz`): das globale
  ELF-Feld, getrieben von Blitzaktivität — der Herzschlag der
  Atmosphäre; Kopplung zur Ozean-Oberfläche über den globalen
  Stromkreis.
- **LOD** (gravity/em, `finals.all` → `eop_iers_ut1_utc_s`, `pmx`,
  `pmy`): die Erdrotation variiert — das Coriolis-Moment auf den
  Ozean variiert. LOD ist die direkte Messung des
  Drehimpulsaustauschs Atmosphäre↔Ozean. Geerntet, nie als Kanal
  genutzt.
- **Relativistische Elektronen** (em, `radiation_electron_flux_2mev`):
  GOES misst sie; sie präzipitieren in die obere Atmosphäre,
  ionisieren, modulieren möglicherweise die Wolkenbildung — umstritten,
  aber die Maschine misst, sie urteilt nicht.
- **SSI/TSI** (thermal, `spectra.bin`, Integral ≈ 1362 W/m²): das
  Spektrum ist geerntet; das kanalisierte Integral als thermaler
  Skalar ist der dominierende Energie-Eingang des Ozeans. F107/Xray/
  EUV sind Proxy und Teil — TSI ist das Ganze.

**Klasse 2 — nicht in der Pipeline, aber Force-Gate bestanden:**

- **QBO** (advective): der quasi-biennale Stratosphärenwind (30 hPa,
  Singapur) moduliert ENSO — gemessen per Radiosonde/Raketensonde,
  ein direktes physikalisches Messgerät, kein abgeleiteter Index.
- **Zweiter Neutronenmonitor** (em): Oulu gibt eine
  Cutoff-Rigidität; ein zweiter Monitor anderer geomagnetischer
  Breite (Moscow, Climax, Kiel) ergibt den Gradienten der kosmischen
  Strahlung — das Spektrum, nicht nur ein Punkt.

**Klasse 3 — wegweisende Akteure, die die Natur der Matrix verändern**
(nicht einfach neue Kanäle — sie verändern, was die Matrix ist):

- **Äquatoriale Thermoklinen-Tiefe (20-°C-Isotherme)** (thermal): der
  interne Speicher des Ozeans. ARGO misst die Profile — der Wert
  existiert. Ein geteilter Thermoklinen-Kanal testet die
  Ozean-Erinnerung gegen die Ozean-Oberfläche.
- **Pazifischer Windstress** (advective): die Passatwinde sind der
  primäre Treiber. Lokaler Wind ist gemessen — der großräumige
  Windstress über dem äquatorialen Pazifik ist ein geteilter Kanal,
  kein lokaler.
- **Jupiter-Gravitation** (gravity): real, klein, vorhanden — die
  Ephemeride liegt im Block. Das Jupiter-Signal am Stationspunkt ist
  eine periodische Störung der Sonnentiefe; und der Jahresgang der
  Sonnentiefe ist schon ein Kanal. Die Maschine kann es messen, also
  soll sie es messen.

**Was nicht fehlt:** abgeleitete Indices (PDO, AMO, MJO, IOD) sind
keine direkten Messungen — kein Lebewesen kann ein Sinnesorgan für
einen Index entwickeln. Das Force-Gate verweigert sie: Theorie, nicht
Messung.

Die volle Matrix: ~30 Kanäle → C(30,2) = 435 Paar-Runden (beide
Richtungen je Paar) → ~159.000 Zellen je Station ≈ 180 h ≈ eine
Woche je Station, 37 Stationen ≈ ¾ Jahr je volle Matrix. Die
Zykluszeit ist lang — aber die Maschine urteilt nicht über die Zeit,
sie misst.

## Die Vision des Operators (Wortlaut sinngemäß, 2026-08-21)

Dies ist der Playground für das schiere chaotische Potenzial der
Maschine: alle Daten liegen in ICRS/TDB — damit können ALLE Kräfte,
die an den Stationen messbar sind, in EINE Matrix einbezogen werden —
was keine Simulation und kein Quantencomputer je konnte, allein weil
alle Daten dasselbe Koordinatensystem tragen. Der Beweis der
Kausalitätsmaschine ist die Matrix selbst, nicht eine Theorie über
sie.

## Nicht anfassen

- `src/te.rs` — die kanonische CPU-Referenz.
- Der Presence-TE-Pfad, die Membran-Rendering-Physik, der skalare
  `transfer_entropy_lag` — unverändert.
- `nobel_probe_corona` (Nadel-III-Registratur, eigenes Protokoll).
- Das benannte 37er-Set der Stationen — die Auswahl ist a priori
  (Instrumenten-Verfügbarkeit, keine datengetriebene Wahl).
- `te_compute` bleibt der unveränderte Kernel — neue Kanäle sind neue
  Serien, keine neuen Kernel-Pfade.
- Das 17er-Kanal-Enum ist das Gerüst der ersten Matrix — der
  empfangende Atom ersetzt den Katalog durch die
  Präsenz-Inventur (Abschnitt „Die Architektur-Wende"), er hält das
  Enum nicht als Gesetz fest.

## Verifikation

`cargo check` 0/0 in allen vier Feature-Kombis, `cargo test --lib`
(307, still), Hidden-Lauf: Ringzeilen aller Kanäle, erste
Matrix-Zellen, keine GPU-Hänger. Auf der GTX 970: derselbe Lauf,
Hang-Grenze neu gemessen, ENSO_PROBE_MAX danach gesetzt (oder
gelassen). Der Befund und die Registerzeile (TODO.md) im selben
Commit; nach eigenem Commit dieses Handover nach
`/home/johannes/projects/archive/handover/` archivieren.
