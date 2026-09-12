<!--
  title: Handover — Ernte-Folge X (Stand 2026-09-12)
  session: Ernte-Folge X
  class: handover
  date: 2026-09-12
  sha256: fcb9121b93b3016b2eae11a46f0044b4326111bb703b65c047e659baa0249046
  status: live
-->
# Handover — Ernte-Folge X (2026-09-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird erst, wenn der Baum ruhig ist.

## CDN-Manifestation (Duty)

- igets.bin — dispatched (Run 34715203286, in_progress seit 19:47Z, igets-cdn.yml).
  Die Shard-Verengung (bbbc834: Station-Shards + k-way merge `igets_merge` +
  gestreamtes `--list-stations`, 4-Job-Workflow) lebt auf main. Wächter: igets.bin
  auf Release `igetsftp.gfz.de` (HTTP 200). Vorläufe cancelled an der 6-h-Grenze
  (34684040604, 34634693904) — ob sie die Laufzeit trägt, misst der laufende Run.
- eso-harps — dispatched (Run 34715204877, in_progress seit 19:47Z,
  eso-harps-rvcat-cdn.yml). Wächter: harps_rvcat.json auf Release
  `ssd.jpl.nasa.gov` (HTTP 200). Gemessen: Vorlauf 34708739804 (17:36Z) endete
  success, das Asset las 404 → der Redispatch ist die gemessene Antwort.

## Ernte

- Hi-net — `HINET_PASS` weiter absent (.secrets.local gemessen) — Operator.

## Abschluss

- Baum ruhig gemessen beim Sessionsstart (HEAD == origin/main == 7bd03ad). Zum
  Abschluss trägt der Baum eine parallele bau13-Session — uncommitted:
  static/webserial.js, .github/workflows/esp32-firmware.yml, static/presence_frame.js,
  static/presence_frame.test.mjs — disjunkt zu dieser Session, unberührt.
  Dispatch getragen: 34715203286 (igets), 34715204877 (eso-harps). Die nächste
  Session misst die Wächter oben. Nur eigene Dateien committet.
