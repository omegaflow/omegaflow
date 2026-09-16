<!--
  title: Handover — Bau-Folge 59 (Stand 2026-09-16)
  session: Bau-Folge 59
  class: handover
  date: 2026-09-16
  sha256: 5cc3ced283d5fb8213ceea7aff0a0eff1935092efe827fdbc78ce6527545fc61
  status: live
-->
# Handover — Bau-Folge 59 (2026-09-16)

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

- Der Lauf `35129638318` @ `1337c6c1` ist `in_progress` (gemessen 2026-09-16):
  Step 4 (shift-sweep), 5 (n=1000-FPR-Gate) und 7 (block-sweep) sind success,
  Step 8 (`n=1000 KSG sweep`) läuft seit 19:56:43Z, Step 9 (`arx sweep`) pending;
  der zweite Lauf `35139361939` @ `43af521f` ist concurrency-blockiert.
  (Schritt: `gh run view 35129638318` — success: Gate zu, Punkt löschen; failure:
  `gh run view 35129638318 --log-failed`, hält keine Blocklänge rise ≤ 2pp,
  `tools/measure/src/bin/multi_force_te_probe.rs:74` auf Arx, Block ausmustern.)

## ODR — Packing der 566-B-Quelle offen

- Offen: `90320204.ODR` (12701 × 566 B, 200 Sa/s, 8-bit) wird von
  `galileo_odr_compiler.rs` (nur die 10 bekannten 2666-B-Quellen) noch nicht
  gepackt und ist nicht in `phi/sources.φ` registriert.
  (Schritt: Quellen-Kuration nach `docs/SOURCE_PORT.md` — sha256, `phi/sources.φ`-
  Eintrag, CDN-Manifestation.)

## las — MLLW→Ellipsoid-Kette offen

- Offen: **MLLW→Ellipsoid-Kette** — VDatum-Gitter registriert (`phi/sources.φ`),
  aber kein GTX-Reader und keine GEOID18/EGM2008-Undulation; das Oahu-Granule
  (EPSG:26904, Hawaii) hat kein benanntes HI-Gitter in der ZIP-Liste.
  (Schritt: `vdatum_regional_20250917.zip` öffnen, HI-Gitter bestätigen oder
  dessen Absenz messen, dann GTX-Reader + Geoid-Undulation bauen.)
- Offen: **CDN-Manifestation** der LAS-Quelle (Oahu) — `las-cdn.yml` Lauf
  `35153867367` dispatched (checkoutt HEAD `548d8fc3`).
  (Schritt: `gh run view 35153867367` — success: Punkt löschen; failure:
  `gh run view 35153867367 --log-failed`.)

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation, zweiter Prompt), nie das Commit-Wort.
