# Security & responsible use

`archive_search` (`tools/utils/src/bin/archive_search.rs` + `archive_search/`) ist ein lokales
Recherche-Werkzeug. Diese Notiz nennt die gemessenen Zugangswege und die Grenzen.

## Zugänge mit Schlüssel (gemessen 2026-10-19)

| Modus | liest | Beleg |
|---|---|---|
| `--ads` | `NASA_ADS_TOKEN` | `archive_search/net.rs` |
| `--brave` | `BRAVE_API_KEY` | `archive_search/net.rs` |
| `--github` | `GH_SEARCH_TOKEN` (read-only) | `archive_search/net.rs` |
| HDF5-Range-Reads (Earthdata) | `EARTHDATA_EDL_TOKEN` | `src/archivar/hdf5.rs` |

**Kein `EARTHDATA_USER`/`EARTHDATA_PASS`** — dieses Paar wird nirgends gelesen (0 Treffer im Code).

**Nicht für die Recherche:** `OMEGAFLOW_TOKEN` gehört dem CI-Pfad (`ci_manage`, `ci_gate_register`)
und wird von `archive_search` nie gelesen.

## Netz-Ausgänge

Direkter Exit → laufender Proton-Tunnel (`proton-wg`) → dessen SOCKS-Proxy. `--playwright` nutzt
denselben SOCKS-Proxy, wenn einer läuft. Rotation zwischen Ausgängen ist Operator-Hand
(`bin/proton-wg.sh`, `bin/proton-exit.sh`), nicht Teil des Werkzeugs.

## Grenzen

- Schlüssel liegen in `.secrets.local` (gitignored) und werden nie committet.
- PII bleibt im Haus: was ein Agent liest, geht durch den Modell-Provider — Personenbezug wird
  deshalb nicht in Agenten-Kontext gelegt, sondern vorher redigiert.
