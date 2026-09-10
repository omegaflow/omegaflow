<!--
  title: Handover — disjunkte Linien: schwerer TE-Lauf nach CI, drei Register-Schlüsse
  class: handover
  date: 2026-09-09
  sha256: 23d66ba64d5aca32644c6283372e08646ae69329b0fc88393360dd0af20de479
  status: live
  see-also: docs/handover/archiv/handover-2026-09-09-mechanische-reste.md
-->
# Handover — disjunkte Linien: schwerer TE-Lauf nach CI, drei Register-Schlüsse

Diese Sitzung arbeitete auf Operator-Wort („schwere TE-Läufe in CI", „auf
disjunkte Linien ausweichen"). Gemessener Befund zu Beginn: ein paralleler Prozess
arbeitet im selben Baum an den TE-/Tiefenphasen-Linien (Galileo-Instrument,
mars-Dispatch, miniSEED/DEPTHS/ETOPO1, depth_phase-*, te-tscaling,
pcmci_class_benchmark). Diese Sitzung hielt die von ihm geführten Dateien unberührt
und nahm die disjunkten Register-Linien.

## Geschlossen (trägt Git)

- **S3-ListBucketResult-Parser** — `d41cc87`: namespaced Tag-Scan,
  Contents/CommonPrefixes, Entity-Decode, paginierter Walk mit Continuation-Tokens;
  schließt die NOAA-NODD-Namespace-Lücke (ledger.φ:681). 4 stille Tests grün, live
  1400 Objekte über echte Tokens. Die NOAA-NODD-Bucket-Dispositionen je Dataset
  bleiben eine Operator-/Register-Entscheidung (offen).
- **maschinen-audits R4 + Kalibrationslauf** — `eafe72d`: R4-Single-Sheet-Kommata-
  Locale gebaut (K-Klassen-Form), 5 R4-Funde, Kalibrationslauf/Regression 14/14;
  R2 (§2-Zählung vs. Tabellen-n) bleibt `pending` (gemessener Grund im lauf-log).
  Auf dem Weg durch das Gate wurden Alt-Fabrikationen liquidiert
  (derive(Default)/unwrap_or_default/must-Diagnostik).
- **10 bestand-Korpus-Herkunft** — `e197d71`: Befund; der registrierte Pfad zeigt
  auf die alte archive-root-Adresse, die Korpora liegen byte-identisch zweifach
  unter /home/johannes/backup/archive/. Kein Datenverlust; Linie geschlossen.

## Dispatch (CI)

- **Galileo N_SURR 20-vs-10-Erhebung** — workflow_dispatch run 34400114106 auf
  main (das Instrument trägt `45c2fec`; Rat: workflow_dispatch ist operator-frei).
  Die 10 Punkte (5 Proben × n-surr 10/20) laufen in CI; das Blatt
  (befund-galileo-nsurr-20.md) ist ein Sitzungs-Akt, kein CI-Skript.

## Gemessen, nicht im Baum (archive-root)

- **matrixmachine** — Suite 12/12 grün gegen HEAD 02a46d3; Urkunden-Zeile in
  /home/johannes/backup/archive-root/vanilla-dateidocs/handover/handover-2026-08-31-matrixmachine.md
  aktualisiert. Die Gesamtsuite der Core-Crate (769 Tests deklariert) ist gegen
  HEAD noch nicht vollständig nachgemessen (der Parallellauf hielt den
  cargo-test-Lock) — offen.

## Flut-2026 — Abrufe gemessen

- **abfluss-trishuli**: DHM-Pegel 4913 (Bhotekoshi/Rasuwagadi) gezogen — der
  keyless-Livestore trägt nur 08-25…08-26 (162 Punkte, max 2,152 m); das
  Pre-08-25-Archiv ist aus dem keyless-Store gerollt, der Peak wurde nicht
  aufgezeichnet (Sensor-Ausfall erneut gemessen). Der Abfluss-Pfeil bleibt
  `pending`; Entsperrung = das archivierte externe CSV des 08-27-Zugs.
- **seen-kollabgebiet**: das CEMS/S1-Fenster ist NICHT abgelaufen (re-gemessen):
  S1-Post-Szenen 08-28/08-31/09-05 verfügbar; EMSR927 trägt nur Grading-Produkte
  (AOI01–05), auf keinem AOI ein Delineation-Produkt — die Flutflächen-
  Delineation wurde nicht erzeugt (gemessen, kein Abruf-Fehler).
- **satellitenbilder-post**: CEMS-products.zip ohne Login ladbar (302→presigned);
  S1-SAR VV-Asset mit keyless SAS-Token ladbar. Tote Endpunkte gemessen: dhm
  getApiFilter 404, emergency.copernicus.eu/mapping 404 (Portal migriert →
  rapidmapping.emergency.copernicus.eu).

## Zurückgestellt / gehalten

- **docs-reference-verteilung (Bewegung)** — zurückgestellt (Kollisionsrisiko mit
  dem Parallellauf an den Handover-Dateien; Rat: reine Bewegung zuletzt).
- **saubere-datenbank Step-5** — zurückgestellt (trägt CDN-/sources.φ-Kontakt,
  die Datei hält der Parallellauf).
- **Bz/omni2_indices-Manifestation** — `pending` (Rat Q4: manifestieren, nicht
  Fallback; sources.φ hält der Parallellauf).

## An die nächste Sitzung

- Der Parallellauf trägt die TE-/Tiefenphasen-Linien weiter; seine uncommitteten
  Dateien bleiben unberührt.
- Das Galileo-Blatt nach Grün des Runs 34400114106.
