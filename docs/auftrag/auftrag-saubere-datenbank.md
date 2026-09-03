<!--
  title: Auftrag — saubere Datenbank (sources.φ / CI-Jobs / CDN-Assets vereinheitlichen)
  class: auftrag
  date: 2026-09-03
  status: pending
  version: 1
  sha256: 654386bc8af89e964dbcde001a4ead1c48a28fdcb04faec2b57b04714fa2e473
  see-also: docs/surveys/survey-2026-09-03-daten-holdings-inventur.md,
            docs/auftrag/auftrag-verify-references-regelrunde.md, AGENTS.md
-->

# Auftrag: saubere Datenbank

Eine einzige, konsistente Basis über drei Ebenen: **sources.φ (die eine
Registry), die CI-Manifestation, die CDN-Assets**. Ziel ist eine Datenbank,
in der jede Quelle genau eine Release hat, jede Release genau eine Quelle,
jeder Asset-Name kanonisch ist und keine Workflow-Klasse dupliziert.
Die Verlegung folgt der heute (2026-09-03) etablierten CDN-Manifest-Duty
(AGENTS „CDN-Manifestation — eine Session-Duty"; Granit-Grundsatz 7).

## Ausgangsmessung (2026-09-03)

Gemessenes Chaos als Startbasis (nicht zu wiederholen, aber zu verifizieren):

- **CDN `omegaflow/sources`:** 200 Release-Tags (= Netlocs), 2904 Assets, ~196
  Netlocs. Anomalien: 4 leere (0-Asset-)Releases; 3 doppelte Netloc-Tags
  (`opendap.nccs.nasa.gov`, `ned.ipac.caltech.edu`, `isc.ac.uk`); 70 Assets
  mit Doppel-Endung (`*.json.json`), ~20 literal `json.json`; Pfad-/Paginiere-
  reste als Namen (`1.json`, `100.json`, `1234.json`); Prozent-/MIME-kodierte
  Namen (`SELECT_20TOP_…`, `application_2Fsolr_2Bjson.json`,
  `-180_-60_180_60.json`); GitHub-Repo-Quellen als Tag
  (`github.com`, `raw.githubusercontent.com`, `github.com-…GeoNuclearData`).
- **CI:** ~17 Workflows unter `.github/workflows/`, 4–24 Jobs je Datei;
  Ausreißer `health-check.yml` (20) und `kernel-flatten.yml` (24). Die
  Kategorie-Namenskonvention (`X-cdn` / `X-cdn-watch`) existiert, ist aber
  nicht durchgängig/duplikationsfrei.
- **Registry:** `phi/sources.φ` ist die vorgesehene eine Wahrheit; der
  Registry↔CDN-Abgleich (welche Release hat keine Quelle, welche Quelle keine
  Release, welcher Asset-Name weicht vom kanonischen `source_name_from_url`
  ab) ist **nicht** durchgeführt.

## Auftrag (was die neue Session tut)

Der Auftrag wird in Schritten ausgeführt; jeder Schritt steht allein und wird
committet. Die Reihenfolge ist verbindlich.

1. **Registry↔CDN-Abgleich (nicht-destruktiv, Maschinenbasis).** Eine
   Abgleich-Engine/Tabelle erzeugen: je Release (Netloc) ↔ Quelle in
   `phi/sources.φ`; markiere Orphan-Releases (keine Quelle) und Quellen ohne
   Release; je Asset den kanonischen Namen (`source_name_from_url`) gegen den
   tatsächlichen abgleichen. Ausgabe: maschinenlesbare Tabelle.
2. **Ziel-Schema festlegen (eine kurze Spezifikation).** Die verbindlichen
   Konventionen: Name = flacher Pfad ohne Roh-URL-Reste (`json.json`,
   `1.json`, `_2F`, `.php.json` sind verboten); jede Quelle genau eine Release;
   Netloc = Domain (kein GitHub-Repo als Tag); CI je Quelle genau eine
   `X-cdn`-Klasse, keine Duplikate.
3. **Registry zuerst aufräumen.** `phi/sources.φ` zur einzigen, abgeglichenen
   Wahrheit machen; Duplikate/Fehlbenennungen dort beheben (Registry ändern,
   nicht Assets raten).
4. **CI deduplizieren/schlanken.** `health-check.yml` (20) und
   `kernel-flatten.yml` (24) zerlegen oder verschlanken; Job-Kategorien
   vereinheitlichen; jede manifestierende Workflow-Klasse speist genau die
   Releases aus der Registry.
5. **CDN-Assets kanonisch machen — NUR mit Nachbau-/Sicherungsquelle.** Ein
   umzubenennendes/gelöschtes Asset wird erst angefasst, wenn die Quelle es
   nachbaut (Compiler/`--ci-mode`) oder der Inhalt anderweitig gesichert ist.
   Müll-Zwillinge (`x.json.json`) und Orphans nur dann entfernen. Kein
   Blindwurf über die 2904 Assets.

## Regeln (verbindlich)

- **Nie die letzte Kopie zerstören.** Vor jedem CDN-Entfernen: Nachbau-Quelle
  oder Sicherung nachweisen. `0 honored` — ein nicht nachbaubares Asset bleibt
  `pending`, wird nicht gelöscht.
- **Manifestiert, nicht nur lokal** (Granit 7): jede neue/veränderte Quelle
  wird in `phi/sources.φ` registriert, damit CI sie ins CDN bringt.
- **Registry zuerst:** Was im CDN falsch heißt, wird an der Registry korrigiert
  und von CI manifestiert — nie direkt am CDN geraten.
- `cargo check --all-targets`: 0 Fehler, 0 Warnungen nach jeder Code-Änderung.

## Abnahme

- Abgleich-Tabelle (Schritt 1) liegt committet vor.
- Ziel-Schema (Schritt 2) ist eine versionierte Spezifikation.
- `sources.φ` ↔ CDN sind abgeglichen: keine Orphan-Releases, keine Quelle ohne
  Release, keine Roh-URL-Reste in Asset-Namen (außer mit dokumentierter
  Nachbau-Quelle als `pending`).
- CI: keine duplizierten Job-Klassen; jede manifestierende Klasse liest aus
  der Registry.
