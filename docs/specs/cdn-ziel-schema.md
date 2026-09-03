<!--
  title: CDN_ZIEL_SCHEMA — die eine Datenbank über sources.φ / CI / CDN
  class: concept
  date: 2026-09-03
  version: 1
  sha256: 0d4671b9c27d4db8651b48508fbf7fd7acb5c5218c0b95fb6be767ff939e29be
  status: live
  see-also: docs/auftrag/auftrag-saubere-datenbank.md,
            docs/specs/cdn_reconciliation.json,
            docs/concepts/4d-membrane.md, AGENTS.md
-->

# CDN_ZIEL_SCHEMA

Die verbindlichen Konventionen für die saubere Datenbank — eine einzige,
konsistente Basis über drei Ebenen: `sources.φ` (die eine Registry), die
CI-Manifestation, die CDN-Assets. Jede Quelle hat genau eine Release, jede
Release genau eine Quelle, jeder Asset-Name ist kanonisch, keine
Workflow-Klasse dupliziert. Die Konventionen sind hier einmal niedergelegt;
Abweichung ist Drift, nicht Ausnahme.

## §1 Registry ↔ Release — eine Quelle, eine Release

- Der **Netloc ist die Release-Identität.** Jede Release im CDN
  (`omegaflow/sources`) trägt einen Tag = Netloc einer Quelle aus
  `sources.φ`. Es gibt genau eine Release je Netloc — keine doppelten
  Netloc-Tags, keine leeren (0-Asset-)Releases.
- **Eine Quelle ⇆ eine Release.** Jede in `sources.φ` registrierte Live-Quelle
  (ein `url`-Block) besitzt genau eine Release unter ihrem Netloc. Eine Quelle
  ohne Release ist `pending` (Granit 7 — Manifest-Duty), keine fabrizierte
  Release.
- **Netloc = Domain, kein GitHub-Repo als Tag.** Ein Netloc ist der Host einer
  Domain (`api.open-meteo.com`, `ndbc.noaa.gov`). Ein GitHub-Repo
  (`github.com/…`, `raw.githubusercontent.com/…`) ist keine Release-Identität;
  das ist Quelle → Registry → CI, nie Tag.
- **Compiler-/Mess-Datensätze** (Ephemeriden, Probe-Serien, GOES/OMNI, …) sind
  keine Live-`url`-Quellen; sie wohnen auf festen Compiler-Netlocs
  (`ssd.jpl.nasa.gov`, `spdf.gsfc.nasa.gov`, `physionet.org`, …) und sind dort
  als eigene, dokumentierte Release-Klasse ausgewiesen — nicht als Orphans zu
  löschen.

## §2 Asset-Name — kanonisch aus der URL

Der kanonische Asset-Name ist **ausschließlich** `source_name_from_url(url)`
(mit dem `cdn_manifest_map`-Kollisions-Override), berechnet aus der
Registry-Quelle — nie geraten, nie aus dem Asset-Dateinamen gelesen. Der
Name ist der flache Pfad der URL (Pfad + Query), kanonisch verflacht:

- Pfad- und Query-Trennzeichen (`/ ? & =`) werden zu `-`.
- Andere Nicht-ASCII-alphanumerische Zeichen werden zu `_`.
- **Verboten sind Roh-URL-Reste:** `*.json.json` (Doppel-Endung),
  paginierte Reste (`1.json`, `100.json`, `200.json`), Prozent-/MIME-Kodierung
  (`SELECT_20TOP_…`, `application_2Fsolr_2Bjson.json`,
  `-180_-60_180_60.json`), `.php.json`-Reste, Roh-`http`-Status.
- **Die Query gehört zur Identität:** zwei Queries sind zwei Messungen, zwei
  Namen. Eine Query wird nie stillschweigend verworfen.
- Ein Asset, dessen Name vom Kanonischen abweicht, wird an der **Registry**
  korrigiert und von CI manifestiert — nie direkt am CDN geraten
  (registry-first).

## §3 CI — eine Workflow-Klasse je Quelle

- Jede manifestierende Klasse folgt der Benennung **`X-cdn`** (Release-Erzeuger)
  und **`X-cdn-watch`** (Beobachter). Keine Klasse dupliziert eine andere;
  keine Quelle wird von zwei Klassen manifestiert.
- Jede manifestierende Klasse speist **genau die Releases aus der Registry**
  (liest den Netloc aus `sources.φ` / der Compiler-Netloc-Liste), nicht einen
  eigenen, abweichenden Tag-Satz.
- `kernel-flatten.yml` und `health-check.yml` sind die manifestierenden
  Hauptklassen; ihre Job-Kategorien sind vereinheitlicht und duplikationsfrei.

## §4 Die drei Zustände — null-echt, absent, pending

Auf der CDN-Ebene gelten dieselben drei Zustände wie in der Kybernetischen
Ethik:

- **present** — das Asset existiert kanonisch benannt und von einer Quelle
  nachbaubar.
- **Müll-Zwilling** (`x.json.json`) und reine Orphans — nur entfernt, wenn eine
  Nachbau-Quelle existiert oder der Inhalt gesichert ist.
- **pending** — das Asset ist nicht nachbaubar und nicht gesichert: es bleibt
  stehen, wird nie gelöscht (0 honored — die Abwesenheit einer Nachbau-Quelle
  ist ein voller Zustand, kein Löschgrund).

## Abnahme-Kriterien (dieses Dokuments Erfüllung)

1. `sources.φ` ↔ CDN abgeglichen: keine Orphan-Releases, keine Quelle ohne
   Release, keine Roh-URL-Reste in Asset-Namen (außer dokumentiert `pending`).
2. CI: keine duplizierten Job-Klassen; jede manifestierende Klasse liest aus
   der Registry.
3. Die gemessene Abgleich-Tabelle (`docs/specs/cdn_reconciliation.json`) und
   dieses Schema stehen versioniert committet.
