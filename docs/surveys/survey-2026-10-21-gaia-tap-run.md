<!--
  title: Axiom-Gate-Survey — Gaia-DR4-TAP-Runner (6 Chunks, ein POST je Chunk)
  class: survey
  date: 2026-10-21
  sha256: aa74a073ebb33edc398feeecf6dcdfa8389c9b5761685799e36c4c1f8d086673
  status: live
  see-also: docs/surveys/survey-2026-10-21-gaia-dr4-vorbereitung.md docs/surveys/survey-2026-10-20-frame-wache.md tools/measure/src/bin/gaia_dr4_probe.rs
-->
# Axiom-Gate-Survey — Gaia-DR4-TAP-Runner

**Zweck.** `--tap-query` gab bisher nur die sechs ADQL-Chunks aus. Der neue
Lauf `--tap-run` liest die Anker-CSV und führt die Chunks selbst gegen den
Gaia-TAP-Sync-Endpunkt aus. Jede Antwort wird als VOTable abgelegt; damit ist
der 2.12.-Pfad ein Kommando statt sechs manueller Abfragen.

## Kommando

```sh
cargo build --release -p omegaflow-measure --bin gaia_dr4_probe

./target/release/gaia_dr4_probe \
  --tap-run ../omegaflow-work/sources/cdsarc.cds.unistra.fr/gaia_crf3_icrf3_positions.csv \
  --out-prefix state/gaia_tap/dr4_epoch
```

Gelesen werden die Spalten 1–3 der CSV (`Source,RA_ICRS,DE_ICRS`), also
`id, ra, dec`. Die 512 Anker ergeben sechs Chunks à 100, 100, 100, 100, 100,
12 IDs. Ausgabe: `dr4_epoch_01.vot` bis `dr4_epoch_06.vot`. Ohne
`--out-prefix` gilt `state/gaia_tap/dr4_epoch` (gitignoriert).

Jeder Chunk geht als `POST` mit
`REQUEST=doQuery&LANG=ADQL&FORMAT=votable&QUERY=<ADQL>` an
`https://gea.esac.esa.int/tap-server/tap/sync` (curl, 300 s Zeitlimit,
User-Agent `omegaflow-gaia-dr4-probe/1.0`). Die Abfrage lautet unverändert:

```sql
SELECT source_id, transit_id, ra0, dec0, obs_time_tcb, scan_pos_angle,
       centroid_pos_al, parallax_factor_al, used_by_agis_al
FROM gaiadr4.epoch_astrometry
WHERE source_id IN (<100 IDs>)
```

## Fehlerbehandlung

Ein Chunk gilt nur als geschrieben, wenn curl erfolgreich endet, der
HTTP-Status `2xx` ist, der Rumpf nicht leer ist und ein VOTable mit
`QUERY_STATUS` ungleich `ERROR` zurückkommt. Folgende Fälle brechen den
jeweiligen Chunk mit Meldung ab:

- curl-Exit ungleich 0 (Netz, DNS, TLS),
- HTTP-Status außerhalb `2xx` (der Rumpf wird gekürzt mitgegeben),
- leerer Rumpf (auch bei HTTP 200),
- `QUERY_STATUS="ERROR"` im VOTable (die Servermeldung wird ausgegeben),
- Rumpf ohne `<VOTABLE`.

Erst nach erfolgreicher Prüfung wird die Datei geschrieben. Schlägt ein
Chunk fehl, laufen die übrigen weiter; am Ende steht `written N/6` und der
Prozess endet mit Exit 1, wenn `N < 6`. Ein leeres VOTable wird nie als
Erfolg abgelegt.

## Testergebnis (DR3, 2026-10-21)

Getestet wurde der POST-/Formatpfad gegen die vorhandene Tabelle
`gaiadr3.gaia_source` mit drei echten Anker-IDs — nicht gegen `gaiadr4`,
weil `gaiadr4.epoch_astrometry` erst am 2.12. existiert.

```sh
./target/release/gaia_dr4_probe \
  --tap-adql "SELECT source_id, ra, dec, pmra, pmdec, parallax FROM gaiadr3.gaia_source WHERE source_id IN (15495005053467776,25350786766710528,52907090779778048)" \
  --out-prefix /tmp/opencode/gaia/dr3_probe
```

Ergebnis:

```text
rows 3 -> /tmp/opencode/gaia/dr3_probe.vot
-rw-rw-r-- 1 johannes johannes 4487 ... /tmp/opencode/gaia/dr3_probe.vot
```

Die Datei beginnt mit `<?xml version="1.0" encoding="UTF-8"?>` und
`<VOTABLE version="1.4" ...>`; `votable_to_json` liest drei Zeilen. Damit
sind POST, Formularfelder, VOTable-Antwort und Zeilenparsing belegt.

Negativprobe: eine Query mit unbekannter Spalte liefert HTTP 400 mit
`QUERY_STATUS="ERROR"`; der Runner meldet die Servermeldung, schreibt keine
Datei und endet mit Exit 1.

`--tap-query` liefert weiterhin genau sechs `SELECT`-Zeilen für die 512
Anker.

## Was am 2.12. läuft

1. `gaia_dr4_probe --tap-run <512-Anker.csv> --out-prefix state/gaia_tap/dr4_epoch`
   → sechs VOTables.
2. `gaia_dr4_probe --xml dr4_epoch_01.vot --anchors <512-Anker.csv>`
   → Fit je Quelle, Anker-Matching über 2″.
3. Die übrigen fünf Dateien ebenso; danach Frame-Rotation vor/nach der
   Korrektur (Frame-Wache).

Ein Lauf pro Chunk, kein Parallelismus, keine Retries. Bei Timeout oder
HTTP-Fehler bleibt der Chunk offen und wird einzeln wiederholt.

## Grenzen

- **Kein DR4-Test vor dem 2.12.** Der TAP-Test belegt nur POST und Format;
  Spaltennamen und Tabellenpfad `gaiadr4.epoch_astrometry` sind aus der
  Vorabdatei übernommen und am Freigabetag zu prüfen.
- **`--tap-adql` ist nur eine Einzelabfrage** für den Smoke-Test, kein
  Chunk-Lauf; es schreibt `<prefix>.vot`.
- **Kein Retry, kein Resume.** Ein abgebrochener Lauf überschreibt beim
  nächsten Start die vorhandenen Dateien.
- **Keine Inhaltsprüfung der Zeilen** beim Abholen: der Runner zählt
  parsebare Zeilen, prüft aber nicht, ob alle angefragten `source_id`
  enthalten sind. Das leistet der Fit-Lauf.
- **Serverlimits** (Zeilen-, Zeit- und Größenlimits des Gaia-TAP) sind nicht
  umgangen; die Chunk-Größe 100 ist die bestehende Vorgabe aus
  `--tap-query`.
- Die Anker-CSV muss die ID in Spalte 1 und `ra, dec` in Grad in den Spalten
  2–3 tragen; eine Kopfzeile wird übersprungen.

## Ablage

- Probe: `tools/measure/src/bin/gaia_dr4_probe.rs`
- Vorbereitung: `docs/surveys/survey-2026-10-21-gaia-dr4-vorbereitung.md`
- Frame-Wache: `docs/surveys/survey-2026-10-20-frame-wache.md`
