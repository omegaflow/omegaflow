<!--
  title: Vorlage — meteorologisches Korrelations-Screening
  class: concept
  date: 2026-08-31
  sha256: 13629f57eedef5d75893012e37f36b3c872a045865cb0371727ac6db341e12ef
  status: live
  see-also: docs/concepts/meteo-kanal-kuratierung.md docs/paper/kausalpfeil-sturzflut-tibet.md
-->
# Blatt: Vorlage — meteorologisches Korrelations-Screening

Das ultimative meteorologische Korrelationswerkzeug. Diese Vorlage beschreibt,
wie fuer **jedes** meteorologische Ereignis ein Kausalpfeil-Screening der
Akteure erstellt wird — Aaretal-Hochwasser, Bordeaux-Waldbrand,
Japan-Tsunami, Tibet-Flut. Nur die Ereignis-Konfiguration wechselt; Werkzeug
und Protokoll sind identisch.

## 1. Ereignis definieren

`config/meteo/<ereignis-id>.json`:

```json
{
  "id": "tibet-flut-2026",
  "source": "open-meteo archive-api",
  "cdn": "archive-api.open-meteo.com",
  "window": ["2026-08-18", "2026-08-27"],
  "stations": [
    {"id": "gyirong", "lat": 28.8559, "lon": 85.2950},
    {"id": "kollab", "lat": 28.2710, "lon": 85.5150}
  ]
}
```

- `id`: eindeutige Ereignis-Kennung (Substantiv-Verb).
- `source`/`cdn`: Datendomain; CDN-Tag = Domain (Upload nur in CI).
- `window`: [start, ende] in UTC (archive-api, stuenndlich).
- `stations`: Akteure als {id, lat, lon}.
- Keine `variables`-Liste: Das Werkzeug erntet automatisch den vollstaendigen
  Stuendlich-Katalog der Quelle. Nichts vorwaehlen — eine Vorauswahl testet
  nur die bekannten Hypothesen und macht neues Wissen unmoeglich.

A=A: jede Station/Variable, die nicht messbar ist, wird im JSON als `pending`
vermerkt, nicht geglaettet.

Das Korrelations-Screening sucht unvoreingenommen in allen Kanälen —
auch in solchen, die keine Hypothese vorhersagte. Wissen entsteht aus dem
unvorhergesehenen Pfeil, nicht aus der Bestätigung.

## 2. Serien ernten

```bash
cargo build -p omegaflow-tools --release --bin meteo_harvest
./target/release/meteo_harvest \
  --event config/meteo/<ereignis-id>.json \
  --out phi/pipeline/meteo_harvest/<ereignis-id>
```

Ergebnis: je Station×Variable eine JSON-Datei im cross_te_screen-Format
`{"source","event","station","variable","window","n","points":[{t,v}]}`.

CDN-Upload: nur via CI (`--ci-mode`, Tag = `cdn`). Lokal kein Token.
Workflow: `.github/workflows/meteo-cdn.yml`, ausloesen mit
`workflow_dispatch` → Ereignis-Konfig waehlen.

## 3. Kreuz-Screening

```bash
cargo build -p omegaflow-tools --release --bin cross_te_screen
./target/release/cross_te_screen \
  --dir phi/pipeline/meteo_harvest/<ereignis-id> \
  --lags 1,6,12,24 --surrogate 20 --min-n 100
```

Prueft alle Paare × beide Richtungen × Lags auf Transfer-Entropie mit
Surrogat-Schwelle. Bedeutsame Pfeile werden protokolliert.

## 4. Befund

Fuer den Befund gilt:
- `gemessen` / `gelernt`: Pfeil uebersteht die Surrogat-Schwelle und ist
  nicht auf einen gemeinsamen Treiber zurueckzufuehren.
- `pending`: Pfeil ist da, aber nicht vom gemeinsamen Treiber (z. B. der
  Temperatur-Tagesgang) isolierbar. Dann fehlt die bedingte Transfer-Entropie
  (partielle Korrelation) — Baustein offen.
- `0` honored: kein Pfeil ueber Schwelle.

Befund als Blatt: `docs/paper/blatt-kreuz-screening-<ereignis-id>.md`.

## 5. Referenzfälle

| Ereignis | Konfig | Phänomen |
|---|---|---|
| Tibet-Flut 2026 | `tibet-flut-2026.json` | M5.2-Lawine Trishuli |
| Aaretal 2026 | `aaretal-hochwasser-2026.json` | Hochwasser, Bodenfeuchte |
| Bordeaux 2026 | `bordeaux-waldbrand-2026.json` | Trockenheit, Wind |
| Japan 2026 | `japan-tsunami-2026.json` | Tsunami-Kuestenregion |
