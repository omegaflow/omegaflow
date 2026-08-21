<!--
  title: Global-Akteure — der Kausalmaschinen-Playground
  class: handover
  date: 2026-08-21
  sha256: a35dceebdea15930fd83bbf0df25b2c6a7ed55ab886dc91164c83347b2c70eda
  status: live
  see-also: handover-2026-08-21-enso-kausalpfeil.md (archiviert, consumed), docs/concepts/blatt-papier-resultat.md, TODO.md
-->

# Übergabe — Die Global-Akteure der Matrix (Kausalmaschinen-Playground)

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

## Verifikation

`cargo check` 0/0 in allen vier Feature-Kombis, `cargo test --lib`
(307, still), Hidden-Lauf: Ringzeilen aller Kanäle, erste
Matrix-Zellen, keine GPU-Hänger. Auf der GTX 970: derselbe Lauf,
Hang-Grenze neu gemessen, ENSO_PROBE_MAX danach gesetzt (oder
gelassen). Der Befund und die Registerzeile (TODO.md) im selben
Commit; nach eigenem Commit dieses Handover nach
`/home/johannes/projects/archive/handover/` archivieren.
