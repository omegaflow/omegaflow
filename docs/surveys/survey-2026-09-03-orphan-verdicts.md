<!--
  title: Survey — Orphan-Releases-Verdikt (Step 3, saubere Datenbank)
  class: survey
  date: 2026-09-03
  sha256: 1b8d21809c6f228cec42a379a30ca82a1196922759297e13c69fd0de9f1be82b
  status: live
  see-also: docs/auftrag/auftrag-saubere-datenbank.md,
            docs/specs/cdn_reconciliation.json,
            docs/specs/cdn_orphan_verdicts.json,
            docs/specs/cdn-ziel-schema.md
-->

# Survey — Orphan-Releases-Verdikt (Step 3)

Der Registry↔CDN-Abgleich (Step 1, `docs/specs/cdn_reconciliation.json`)
zählte 156 Orphan-Releases: 6 `dataset_host`, 15 `repo_tag`, 135
`stale_pending`. Diese Survey legt für jeden Orphan einen dokumentierten
Status vor — die Entscheidungsbasis, die Step 5 (CDN-kanonisch, destruktiv)
und die Force-Gate-Disposition der Registry brauchen. Maschinenform:
`docs/specs/cdn_orphan_verdicts.json` (diesem Blatt übergeordnet).

## Methode

Je Orphan-Netloc:

1. **Klasse** aus der reconcile-Engine (`orphan_releases_by_class`).
2. **Dokumentation** — Netloc-Cross-Ref gegen die URL-Hosts der `url`-Zeilen
   in `phi/sources.φ` und `phi/dead_sources.φ` (`awk`-Extraktion). Ein Netloc
   ist `dead_documented`, wenn er dort als Host einen `dead`/`decline`-Eintrag
   trägt → die Release ist ein dokumentiert-toter Rest. `undocumented` = in
   keinem der zwei Register.
3. **Erreichbarkeit** (nur die undocumented `stale_pending`, live gemessen
   2026-09-03, HEAD auf die Host-Wurzel): der finale HTTP-Code oder der
   Verbindungsfehler.

Die Erreichbarkeit ist **Beweis, kein Urteil**: ein erreichbarer Host ist
noch keine Gate-Entscheidung. Sie ist der grobe Lebend-/Totfilter für die
Disposition.

## Korrektur (2026-09-03)

Eine frühere Fassung dieses Blatts und des Ledgers nannte 70 dead_documented /
65 undocumented `stale_pending`. Diese Zahl kam aus einer fehlerhaften
Netloc-Extraktion und war falsch. Verlässlich per `awk`-Host-Abgleich gegen
`phi/sources.φ` + `phi/dead_sources.φ` sind **80 dead_documented / 55
undocumented**. Zehn Netlocs (u. a. `ncei.noaa.gov`, `ngdc.noaa.gov`,
`sidc.be`, `orfeus-eu.org`, `bodc.ac.uk`, `metoffice.gov.uk`,
`marinespecies.org`, `sciencebase.gov`, `amsmeteors.org`, `globalfloods.eu`)
waren zuvor zu Unrecht als undocumented geführt — sie sind in `dead_sources.φ`
dokumentiert. Der Fehler ist im Ledger behoben. Eine frühere Behauptung über
die Lage der Kandidaten im Pipeline-Bestand (master.φ-Anteile, ledger.φ-Posten)
stammte aus derselben unzuverlässigen Extraktion und wird hier nicht
fortgeschrieben, bis sie verlässlich nachgemessen ist.

## Befund (2026-09-03, gemessen)

- **135 `stale_pending`**: **80 in `dead_sources.φ` dokumentiert** tot/decline
  → Release ist dokumentierter Rest, kein Registry-Urteil offen. **55 in
  keinem Register**, per Wurzel-Probe befragt: die Mehrheit erreicht
  (200/3xx); unerreichbar bzw. deutlich abgestorben gemessen:
  `chime-frb.ca` (conn-refused), `dasch.rc.fas.harvard.edu` (tls-err),
  `ionosonde.iap-kborn.de` (dns-fail), `physics.mcgill.ca` (dns-fail),
  `api.waqi.info` (timeout), `geomag.usgs.gov` (leerer 301),
  `g6goyz4w56.execute-api.us-west-2.amazonaws.com` (AWS 403, Pfad unbekannt),
  `api.coral.tsr.lol` (404, Natur ungeklärt). Die übrigen sind erreichbar und
  von bekannten Daten-Diensten.
- **15 `repo_tag`** (github.com-…, raw.githubusercontent.com): per
  CDN_ZIEL_SCHEMA §1 keine Registry-Heimat (ein Repo ist nie eine
  Release-Identität). 1 in dead_sources.φ, 14 undocumented.
- **6 `dataset_host`** (ssd/spdf.jpl/gsfc, physionet.org,
  sentinel1euwest…, service.iris.edu, archive-api.open-meteo.com):
  Compiler-/Mess-Datensatz-Netlocs, **nie löschen**. 3 in dead_sources.φ.

## Registry-Urteile (Step 3, was die nächste Session tut)

- **80 dead_documented `stale_pending`** → kein Registry-Urteil offen; Release
  ist Rest, Verbleib entscheidet Step 5 (mit Nachweis, nie die letzte Kopie).
- **55 undocumented `stale_pending`** → die eine offene Disposition, je Netloc
  nach SOURCE_PORT.md (Force-Gate → `sources.φ`-Block oder
  `dead_sources.φ`-Eintrag). Erreichbarkeit ist der Vorfilter. Nicht
  entschieden → bleibt `pending` (0 honored), nie am CDN geraten.
- **14 undocumented `repo_tag`** → §1 kein Registry-Heim; das Verdikt ist
  CDN-seitig (Step 5, nach Sicherung), kein sources.φ-Urteil.
- **3 undocumented `dataset_host`** → Compiler-Lease, behalten.

## Messgrenze

Erreichbarkeit = einzelner HEAD auf die Host-Wurzel; ein 4xx/5xx/Timeout kann
transient sein. Momentaufnahme vom 2026-09-03, kein lebenslanger Befund.
`dead_documented` = Host-Übereinstimmung mit dead_sources.φ; per-URL-Lesart
bleibt bei unklaren Einzelfällen offen.

## Registrierung

Die Disposition der 55 undocumented `stale_pending` ist eine offene Pflicht in
`docs/TODO.md` (Auftrags-Programm, Pflege & Struktur →
`auftrag-saubere-datenbank.md`). Step 4 (CI-Dedupe) ist gegen die gemessene
Job-Zahl (health-check 4, kernel-flatten 18) neu zu fassen.
