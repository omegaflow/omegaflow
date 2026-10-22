<!--
  title: Profile — Format und Ablage
  class: spec
  date: 2026-10-21
  sha256: 2fb84ddc7c2cf199c5a27c478105c29eaa6ff0b014ac245f6d73abacac2718cf
  status: live
  owner: future
  ack: future 2026-10-21T13:20Z
  next: none
  orphan-after: none
  see-also: vorlage-je-dateityp
-->
# Profile

Ein Profil bündelt wiederkehrende Suchparameter als JSON. Die Dateien liegen
unter `specs/profiles/`; der Name ohne `.json` ist der Profilname.

## Format

```json
{
  "name": "astro-solar",
  "group": "astro",
  "arms": ["ads", "simbad", "vizier", "ned", "mast", "heasarc"],
  "since": "2015",
  "type": "article",
  "language": "en",
  "max": 30,
  "stop": 10
}
```

| Feld | Bedeutung |
| --- | --- |
| `name` | Anzeigename des Profils |
| `group` | `astro`, `literature`, `web`, `data` oder `all`; nutzt die bestehende Gruppenlogik, besonders für `--all` |
| `arms` | Liste der Arme, die ohne expliziten Arm oder `--all` laufen |
| `since` | `YYYY` oder `YYYY-MM-DD` |
| `until` | `YYYY` oder `YYYY-MM-DD` |
| `type` | Dokumenttyp, z. B. `article`, `preprint`, `dataset` |
| `language` | ISO-639-1-Code, z. B. `en` |
| `max` | Ergebniszeilen pro Arm |
| `stop` | Stop-Schwelle der `--all`-Pipeline |

Alle Felder sind optional. Das Profil setzt Defaults; explizite CLI-Flags
überschreiben sie. Ein unbekanntes Profil oder kaputtes JSON endet mit Exit 2.

## Aufrufe

```sh
archive_search --profile astro-solar "solar wind"
archive_search --profile specs/profiles/astro-solar.json --all --stop 10 "solar wind"
archive_search --list-profiles
```

`--list-profiles` zeigt die Profile unter `specs/profiles/` mit Gruppe,
Arm-Anzahl, `max` und `stop`.
