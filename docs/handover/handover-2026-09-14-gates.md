<!--
  title: Handover — Gate & CI (Stand 2026-09-14)
  session: Bau-Folge 27
  class: handover
  date: 2026-09-14
  sha256: ddd4ab8fa9068b7ece2f5b4a8812c46049e43d438934d28c69f120956b62a7a2
  status: live
-->
# Handover — Gate & CI (2026-09-14)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt.

## n=1000 TE-Gate — der Rat hat entschieden: fixen, ein Atom

- Der Block- **und** Phase-Null leakt bei n=1000, a=0,9 (FPR 12,14 % > 8 %,
  Anstieg 8,57pp > 2pp). Ursache benannt: `block_len_from_n(n) = n^(1/3)`
  (`src/mathematikerin/te.rs:1077`) → bei n=1000 nur 10 Samples Block-Länge.
  Der Rat (einstimmig): fixen, nicht tragen; descopen ist durch den Benchmark-
  Konsumenten vom Tisch. Die Block-Sweep-Messung ist gebaut: `block_sweep_n1000`
  (te.rs, `#[ignore]`, Block ∈ {10, 16, 24, 32, 48} bei n=1000) läuft als
  Schritt in `.github/workflows/te-gate.yml`. **Schritt:** `gh workflow run
  te-gate.yml`, den kleinsten Block wählen, der FPR ≤ 8 % UND Anstieg ≤ 2pp
  hält, dann `block_len_from_n` anpassen und die Vier-Gate-Batterie (n=150 in
  `cargo test` + n=1000) grün fahren; #13 schließt mit dem Ergebnis.
- Das **Phase-Organ** ist unbenannt (der Rat: erst messen, dann Medizin). Die
  Phase-n=1000-Zelle mit KSG/KDE statt binned messen (Diskriminator); der
  Konsumenten-grep für phase+binned auf n=1000: `tools/measure/src/rest.rs:686`,
  `placebo_pair_eeg_probe.rs`, `pcmci_class_benchmark.rs`. **Schritt:** den
  KSG-Lauf als weiteren `#[ignore]`-Test + te-gate.yml-Schritt ergänzen.
- Der **n=1000-Shift-Riß** (Auftrag Atom 4, pending) teilt den Block-Default —
  im selben Atom mitmessen. **Schritt:** `te-n1000-shift.yml` neu fahren,
  sobald der neue Default steht.

## CI-Issues — was noch offen ist

- `reference_verify` prüft arXiv jetzt über `arxiv.org/abs` (0,12 s statt
  40 s Timeout); `paper-check` grün in 37 s. Die vier `cargo test`-Fehler sind
  gefixt (ca_bundle-Pfad → Temp-Dir, Oulu-URL `formchk=1`→`wget=1`, die zwei
  n=1000-Gates nach te-gate.yml ausgelagert). **Schritt:** der ci-check-Lauf
  `34822020311` verifiziert; fällt er, re-filed der health-Job.
- Der **Reverifikations-Sweep**: `live_sweep` überspringt jetzt `spk`/`reference`
  (nicht-JSON); der Map-Extract padet abwesende Höhe auf 0.0 (geonet-GeoJSON
  hat 2 Koordinaten, die Quelle deklariert `coordinates.2`); die Diagnose prüft
  die Map-Keys an der ersten Zeile. **Schritt:** der nächste health-check-Lauf
  bestätigt; bleiben hfradar/cdaweb/FIRMS-Reste, den Sweep-Befund prüfen
  (Endpunkte liefern bei Direkttest gültige Daten).

## SuperMAG — Vollernte

- 20 von 203 Stationen auf der CDN; der gemergte 31-Tage-Asset (3,2 GB)
  sprengt GitHubs 2-GiB-Limit. `supermag-cdn.yml` shardet jetzt zeitlich
  (2 Assets, `supermag_2025-03a.bin` / `supermag_2025-03b.bin`); der Register-
  Block trägt beide. **Schritt:** den Dispatch `34815628422` prüfen (beide
  Assets 200), sonst neu fahren.

## Membran

- ESP32-Modul — on hold (Operator-Wort, 2026-09-13): das Gerät und sein Flash
  kommen zuletzt. BOM: `docs/specs/mantis-shrimp-bom.md`.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check mit Commit und Push (`/commit`).
