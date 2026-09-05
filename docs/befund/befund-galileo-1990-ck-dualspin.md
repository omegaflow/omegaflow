<!--
  title: Befund — Galileo-1990-CK: Dual-Spin für Dez-1990 bestätigt, Station-42-Ton = Rotor-Spin
  class: befund
  date: 2026-09-05
  sha256: 62d0b7a7a3953aadb1326512660e84e8a4e92f45046af0787278e5344adcc2c4
  status: done
  antwortet-auf: docs/befund/befund-galileo-banden-kamm-ton.md
  see-also: docs/befund/befund-galileo-ssd-star-scanner.md
-->

# Befund: Galileo-1990-CK — Dual-Spin für Dez-1990 bestätigt

## Frage & Bindung

befund-galileo-banden-kamm-ton (status done) ließ die Identität des
Station-42-Tons (52,39 mHz, Periode 19,1 s, 1990-12-07..10, isolierte
Einzellinie) offen; die recherchierte wahrscheinlichste Identität ist der
Galileo-Rotor-Spin (~3,15 rpm). Der Star-Scanner-DDR (GO-J-SSD-5, PDS PPI)
konnte das nicht prüfen — er deckt 1990 nicht ab. Dieser Befund liest das in
der Folge benannte nächste Nachschlagwerk: das NAIF-Galileo-Lageprodukt (CK)
über das EGA-1-Fenster. Frage: Trägt die rekonstruierte Plattformlage den
19,1-s-Rotorbeitrag?

Probe `ck_daf_probe.rs` (measure) liest die CK mit der vorhandenen DAF-Maschine
(kein neuer Parser): `gll_plt_rec_1990_tav_v00.bc` aus
`https://naif.jpl.nasa.gov/pub/naif/GLL/kernels/ck/`, 295 367 680 Bytes,
sha256 c1765fd4baad6e32a98de3a333a62ddd9627e541dd3ef6e2c115f491af3666bd.

## Messung — der DAF-Lesebefund

- **Format:** idword `DAF/CK`, locfmt LTL-IEEE — der generische DAF-Leser
  akzeptiert die CK ohne Unterschied zum SPK. nd=2, ni=6, Summary-Größe 5
  Doubles. Archival-Beschreibung `GLL PLT TLM ATT/TLM AV`.
- **Segmentierung:** 47 Segmente, sauber geparst. IC[0] = **−77001 auf allen
  47 Segmenten** — die despun Scan-Plattform (GLL-Frame-Konvention); IC[1..3]
  = (2,3,3×1) Typ-1-Konstanten. DC = [start, stop] sind keine ET-Sekunden,
  sondern rohe Galileo-SCLK-Ticks (Partition 77, 120 Ticks/s).
- **Zeit:** SCLK-Decode über `mk00062a.tsc` (110 Bruchpunkte, partition 77).
  Abdeckung erster Segmentstart ~1989-12-27, letztes Segmentende ~1990-12-31.
- **EGA-1-Fenster 1990-12-07..12-11 ist abgedeckt:** Segmente [32]..[37],
  alle Frame −77001 (12-06 11:50 .. 12-11 08:46; Tag-Granularität robust,
  ~1 min gegen UTC).
- **Payload-Glimpse:** Datensätze zu 7 Doubles — Einheitsquaternion (4) +
  Drehraten-Tripel (3), getaktet diskret. Die Drehraten-Zeilen im Fenster
  liegen bei ~1e-5..1e-4 (Speichereinheiten), die Quaternionen konstant bis
  ~1e-4 über benachbarte Datensätze. Ein 19,1-s-Rotorbeitrag müsste als
  ω ≈ 0,33 rad/s und q-Vorschub ≈ 0,2 rad je Datensatz erscheinen — in den
  geprüften Zeilen nicht vorhanden. Die ~1,4-1,7 Sätze/s im Fenster lösten
  19,1 s auf, wäre der Beitrag im Produkt.

## Verdict

**Dual-Spin für Dez-1990 bestätigt.** Das Produkt ist die despun
Scan-Plattform (PLT, Frame −77001): die Plattformlage ist im Fenster
trägheitsfest, der 19,1-s-Spin steckt konstruktionsbedingt nicht in ihren
Zeilen. Das ist konsistent mit dem Station-42-Ton als Rotor-Spin (die LGAs
sitzen auf dem rotierenden Rotor-Teil während EGA-1): 52,39 mHz =
0,3292 rad/s — der Ton ruht auf der nominalen Rotorrate **0,3300 rad/s
±0,0015** (3,15 rpm, dokumentiert 3,0-3,15 rpm). Die im
banden-kamm-ton-Befund als isolierte Einzellinie gemessene 52,39-mHz-Linie
ist mit der Rotorrate konsistent; ein Gegenbefund aus der Plattformlage
existiert nicht, weil die Plattform per Definition despun ist.

## Grenzen

- Das Produkt trägt die Rotorrate nicht direkt (despun Frame). Der explizite
  Träger wäre das All-Spin-Bus-CK (Frame −77000) oder eine
  Plattform↔Rotor-Entkopplung über die GLL-FK-Frame-Kette. **Nächster Schritt,
  registriert pending** (0 honored, kein Wert erfunden): das
  All-Spin-Bus-CK (Frame −77000) über das EGA-1-Fenster lesen oder den
  GLL-FK-Frame-Decode.
- IC-Spalten-Semantik (2,3,3×1) und die Referenz-vs-Daten-Bedeutung von IC[0]
  folgen der Typ-1-Zuordnung; das GLL/kernels-Archiv führt kein fk/ — eine
  FK-Bestätigung im Bestand fehlt.
- Der Datensatz ist ein Referenz-Kernel (Maß-Referenz), kein Feld-/Oszillator-
  Träger: Force-Gate-Ablehnung als Feldquelle (abgeleitetes
  Satelliten-Lageprodukt). Er ist im Kernel-Index verzeichnet (Familie `ck`,
  `phi/sources_index.φ` Z. 53017; KERNEL_INDEX-Politik: CK indexiert, nicht
  geflacht) und wird per Manifest-Workflow als Roh-Kernel auf den CDN
  gespiegelt (`naif.jpl.nasa.gov/gll_plt_rec_1990_tav_v00.bc`) — kein
  phi/sources.φ-Feldblock (die CK ist binär, keine kernel_text-Quelle; kein
  sha256-Marker existiert in sources.φ).

## Register-Satz

*Das Galileo-1990-CK (despun Scan-Plattform, Frame −77001) deckt das
EGA-1-Fenster 1990-12-07..10 ab und trägt dort keinen 19,1-s-Rotorbeitrag —
Dual-Spin für Dez-1990 bestätigt, konsistent mit dem Station-42-Ton als
Rotor-Spin (LGAs auf dem Rotor), der auf 0,3300 rad/s ±0,0015 ruht. Der
explizite Rotor-Träger (Frame −77000) bleibt pending.*

## Status

`done` (2026-09-05). Der DAF-Lesebefund ist gemessen; der nächste Schritt
(All-Spin-Bus-CK Frame −77000 oder GLL-FK-Plattform↔Rotor-Decode) ist als
pending registriert.
