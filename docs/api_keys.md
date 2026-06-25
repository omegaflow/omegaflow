# API Keys

omegaflow funktioniert zu 100% ohne persönliche API-Keys.
Alle verwendeten Endpunkte sind öffentlich oder nutzen offentliche Demo-Keys.

## verwendete Keys

| Variable | Service | Status | Hinweis |
|---|---|---|---|
| `NASA_KEY` | NASA (DONKI, NeoWs) | Optional | Wird in `src/main.rs` automatisch auf `DEMO_KEY` gesetzt, falls nicht in `.env` vorhanden. |
| `WAQI_TOKEN` | World Air Quality Index | Fix in URL | Nutzt `token=demo` (stark rate-limited, reicht für Tests). |

## Konfiguration (.env)

Wenn du eigene Keys nutzen willst (höhere Rate-Limits), lege eine `.env` Datei im Projektroot an:

```env
NASA_KEY=dein_personalisierter_nasa_key
```

Der Server lädt diese Datei beim Start automatisch (siehe `load_env()` in `main.rs`).
