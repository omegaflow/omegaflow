<!--
  title: Handover — saubere Datenbank Step 3: Ledger-Korrektur + 18 CDN-Zwillings-Löschungen; Disposition der 55 Netlocs offen
  class: handover
  date: 2026-09-03
  sha256: e0e7d3b5a27f44880a7ddbe0b419b7d93a835a9902cd73b477b9831122a4f2aa
  status: live
  see-also: docs/auftrag/auftrag-saubere-datenbank.md,
            docs/handover/handover-2026-09-03-saubere-datenbank.md,
            docs/specs/cdn_orphan_verdicts.json,
            docs/surveys/survey-2026-09-03-orphan-verdicts.md
-->

# Handover — Step 3 saubere Datenbank, Stand & Übergabe

Diese Session legte die verlässliche Entscheidungsbasis vor und löschte
sichere CDN-Müll-Zwillinge. **Die Verdrahtung der 55 Netlocs ist NICHT
geleistet** — sie geht an eine frische Session (der Operator verweist sie
bewusst hierher). Der Auftrag `auftrag-saubere-datenbank.md` bleibt
`pending`.

## Committet (diese Session)

- `bbe95e4` — Korrektur des Orphan-Ledgers. Die frühere 70/65-Zählung
  (`423d2c9`, `e159293`) stammte aus einer fehlerhaften Netloc-Extraktion.
  Verlässlich per `awk`-Host-Abgleich gegen `phi/sources.φ` +
  `phi/dead_sources.φ`: von 135 `stale_pending` sind **80 in dead_sources.φ
  dokumentiert**, **55 in keinem Register**. Zehn Netlocs (u. a.
  `ncei.noaa.gov`, `ngdc.noaa.gov`, `sidc.be`) waren zu Unrecht als
  undocumented geführt — korrigiert. Die Survey
  (`docs/surveys/survey-2026-09-03-orphan-verdicts.md`) ist bereinigt.

## CDN-Löschungen (extern, 2026-09-03, GitHub Release-Assets, `gh api`)

18 Müll-Zwillinge gelöscht, je mit byte-identischem kanonisch benanntem
Doppel auf **derselben Release** (nie die letzte Kopie):
- Erste Welle (12): `json.json`/`.json.json`/Zahlen-Reste/Hex-Duplikate auf
  services7/services1-arcgis, raw.githubusercontent, ngdc, ncei, macrostrat,
  g6goyz, swpc, mars2020.
- Zweite Welle (6): erddap.aoml + data.pmel (je `max_time_-7days.json` +
  Hex-Variante gelöscht, der **kanonische** lange erddap-Name behalten),
  mars `json.json`, gcn `44806.json.json`.

## Fehler dieser Session, die die nächste Session nicht wiederholen soll

- **Unzuverlässige Python-Extraktionen** erzeugten mehrfach widersprüchliche
  Zahlen (94/0/834, 65/70). Verlässlich waren nur `grep`/`awk`/`rg`. Messen
  mit schlichten Shell-Werkzeugen, nicht mit improvisierten Parsern.
- Eine frühe Ledger-Fassung war falsch (10 Netlocs falsch klassifiziert) —
  in `bbe95e4` korrigiert.
- Der `--draft`-Auto-Block der probe-Maschinerie liefert für viele Endpunkte
  Müll (`uncertain field data.N.0`). Nicht für saubere Blöcke verwenden.
- Die `master_urls.txt`-Inventarliste ist mit den eigenen CDN-Download-Links
  von omegaflow kontaminiert (`github.com/omegaflow/sources/releases/
  download/<netloc>/...`) — nicht als Quell-Kandidaten lesen.

## Offen (frische Session)

- **Disposition der 55 undocumented `stale_pending`** in `sources.φ` /
  `dead_sources.φ` (je Netloc Force-Gate nach SOURCE_PORT.md). Der
  Operator übergibt dies einer anderen Session. Erreichbarkeits-Vorfilter
  und Klasse liegen im Ledger `docs/specs/cdn_orphan_verdicts.json`.
- Erreichbarkeits-Messung der 55 (HEAD Wurzel, 2026-09-03): die Mehrheit
  erreichbar; toter gemessen: chime-frb.ca (conn-refused),
  dasch.rc.fas.harvard.edu (tls-err), ionosonde.iap-kborn.de + physics.
  mcgill.ca (dns-fail), api.waqi.info (timeout), geomag.usgs.gov (leerer
  301), g6goyz4w56... (AWS 403), api.coral.tsr.lol (404, Natur ungeklärt).
- Step 4 (CI-Dedupe) gegen gemessene 4/18-Jobs neu fassen. Step 5
  (destruktiv) nur mit Nachbau-Quelle je Asset.
- Ausgangs-Handover `handover-2026-09-03-saubere-datenbank.md` und dieses
  Blatt sind noch `live`, nicht konsumiert/archiviert.

## Session-Hygiene

Die Fremd-Dateien `.github/workflows/flyby-odf-cdn.yml`,
`.github/workflows/laic-verdict-cdn.yml`,
`tools/measure/src/bin/odf_census_probe.rs` und
`docs/handover/handover-2026-09-03-drift.md` sind nicht von dieser Session;
uncommittet lassen.

## Verifikation

Keine Rust-Änderung → kein `cargo check` nötig. Ledger-JSON validiert
(156 Verdicts), Zahlen per `awk`-Host-Abgleich belegt. CDN-Löschungen
per `gh api` ausgeführt und verifiziert.
