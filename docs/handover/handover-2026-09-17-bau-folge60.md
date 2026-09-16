<!--
  title: Handover — Bau-Folge 60 (Stand 2026-09-17)
  session: Bau-Folge 60
  class: handover
  date: 2026-09-17
  sha256: 7975263e81fc71e95727820c760ba3125c701f94d245e64fbdf4cb5c96c7e6dc
  status: live
-->
# Handover — Bau-Folge 60 (2026-09-17)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird, sobald der eigene Commit steht und `origin/main`
Vorfahr von HEAD ist (Fast-Forward) — ein Push sendet nur Commits, der Arbeitsbaum
darf schmutzig sein.

Der erste offene Abschnitt benennt den härtesten undatierten Punkt (datierte
Wiedervorlagen schweigen vor ihrem Datum). Jeder offene Punkt trägt seinen
nächsten Schritt in derselben Zeile — Werkzeug, Datei, URL oder Anfrage;
„Schritt unbekannt — erste Messung: X" ist ein vollständiger Schritt. Kein
Dokument wächst ohne Messung; die Droh-Sprache ersetzt den Schritt nicht.
Der Planungs-Pass nennt die offenen Punkte als nummerierte Auswahl (der erste ist
der härteste undatierte); die Session arbeitet so viele ab wie möglich.

Das Handover wird **vor allem anderen gegen den Baum gehalten**
(`sgrep`/`git log`/`sread`) — das Register ist die Frage, der Baum die Messung;
eine Session, die nur dem Register glaubt, baut Stehendes neu.

## TE-Gate-Arx — Lauf noch in Arbeit; pending

- Der Lauf `35129638318` @ `1337c6c1` ist `in_progress` (gemessen 2026-09-17):
  Steps 4 (shift-sweep), 5 (n=1000-FPR) und 7 (block-sweep) success, Step 8
  (`n=1000 KSG sweep`) läuft seit 2026-09-16 19:56:43Z, Step 9 (`arx sweep`)
  pending.
  (Schritt: `gh run view 35129638318` — success: Punkt löschen; failure:
  `gh run view 35129638318 --log-failed`, hält keine Blocklänge rise ≤ 2pp,
  `tools/measure/src/bin/multi_force_te_probe.rs:74` auf Arx, Block ausmustern.)

## las CDN — Fixtures korrigiert; CI-Lauf ausstehend

- Die zwei roten Fixtures in `tools/harvest/src/bin/las_compiler.rs` waren
  intern inkonsistent (der Snyder-Inverse ist gegen die PROJ-Referenz Zone 32N
  auf 0.6 mm validiert). Neu gemessen gegen EPSG:26918→4326
  (`utm_inverse_zone18n_dd10045_corner_reference` = 39.2059397 / -76.5177658)
  und EPSG:4326→3857 (`web_mercator_inverse_roundtrips_the_reference_point`
  x = -11_688_546.53); dieselben dd10045-Werte in
  `noaa_ocs_hydrodata_compiler.rs` angeglichen.
  (Schritt: nach dem Push `gh workflow run las-cdn.yml`, dann
  `gh run view <id>` — success: Punkt löschen; failure: `--log-failed`.)

## ODR — 566-B-Quelle im Compiler; CDN-Re-Manifestation ausstehend

- `90320204.ODR` (Volume `GO-JS-RSS-1-ODR-V1.0`, HTTP 200, 7 188 766 B, sha256
  `33b350220e6f891900d999b8a85b1b4c79176e537855c89961a0d113bd1868a2`,
  12 701 × 566 B, 200 Sa/s) ist in `galileo_odr_compiler.rs` aufgenommen; das
  Packen ist stride-fähig (`record_stride`/`stride_split` statt fixem 2666).
  `phi/sources.φ` bleibt (Asset-Name `galileo_odr.bin` unverändert, Inhalt wächst
  von 10 auf 11 Dateien).
  (Schritt: nach dem Push das alte `galileo_odr.bin` aus dem Release
  `pds-ppi.igpp.ucla.edu` (omegaflow/sources) löschen — sonst überspringt der
  Idempotenz-Gate den Lauf —, dann `gh workflow run galileo-odr-cdn.yml`,
  `gh run view <id>` — success: Punkt löschen; failure: `--log-failed`.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
