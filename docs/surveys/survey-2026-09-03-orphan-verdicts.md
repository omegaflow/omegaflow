<!--
  title: Survey — Orphan-Releases-Verdikt (Step 3, saubere Datenbank)
  class: survey
  date: 2026-09-03
  sha256: 071fb3ceaab592cae28d77ed1ad8bda6606eeae0c6d717b75a0cc5ca75b60c8f
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
und der Force-Gate-Grind der Registry brauchen. Maschinenform:
`docs/specs/cdn_orphan_verdicts.json` (diesem Blatt übergeordnet).

## Methode

Je Orphan-Netloc drei gemessene Ebenen:

1. **Klasse** aus der reconcile-Engine (`orphan_releases_by_class`).
2. **Dokumentation** — Netloc-Cross-Ref gegen die URL-Hosts in
   `phi/dead_sources.φ`. `dead_documented` = der Netloc trägt einen
   dokumentierten `dead`/`decline`-Eintrag (z. B. `ssd.jpl.nasa.gov`
   superseded-by-ephemeris, `api.wheretheiss.at` decline) → die Release ist
   ein dokumentiert-toter Rest. `undocumented` = in weder sources.φ noch
   dead_sources.φ.
3. **Erreichbarkeit** (nur die 65 undocumented `stale_pending`, live gemessen
   2026-09-03, HEAD auf die Host-Wurzel, `curl -sIL --max-time 15`): der
   finale HTTP-Code oder der Verbindungsfehler.

Die Erreichbarkeit ist **Beweis, kein Urteil**: ein erreichbarer Host ist
noch keine Gate-Entscheidung (ob die Quelle gehört, ist eine Force-Gate-Frage
des SOURCE_PORT-Grinds). Sie ist nur der grobe Lebend-/Totfilter.

## Befund (2026-09-03, gemessen)

- **135 `stale_pending`**: 70 in `dead_sources.φ` dokumentiert tot → Release
  ist dokumentierter Rest, kein Registry-Urteil offen. 65 undocumented, alle
  per Wurzel-Probe befragt: die Mehrheit erreicht (200/3xx auf Wurzel);
  unerreichbar bzw. deutlich abgestorben gemessen: `chime-frb.ca`
  (conn-refused), `dasch.rc.fas.harvard.edu` (tls-err, Zertifikat abgelaufen),
  `ionosonde.iap-kborn.de` (dns-fail), `physics.mcgill.ca` (dns-fail),
  `api.waqi.info` (timeout), `geomag.usgs.gov` (leerer 301),
  `g6goyz4w56.execute-api.us-west-2.amazonaws.com` (AWS 403, Rest-Pfad
  unbekannt), `api.coral.tsr.lol` (404, Natur ungeklärt). Die übrigen ~57
  sind erreichbar und von bekannten Daten-Diensten (bom.gov.au, api.met.no,
  ncei/ngdc/cpc/modis/omniweb.gsfc, ds.iris.edu, isc.ac.uk, sidc.be,
  simbad.u-strasbg.fr, cmr.earthdata.nasa.gov, globalcmt.org, …).
- **15 `repo_tag`** (github.com-…, raw.githubusercontent.com): per
  CDN_ZIEL_SCHEMA §1 keine Registry-Heimat (ein Repo ist nie eine
  Release-Identität). 14 undocumented, 1 (raw.githubusercontent.com) in
  dead_sources.φ.
- **6 `dataset_host`** (ssd/spdf.jpl/gsfc, physionet.org,
  sentinel1euwest…, service.iris.edu, archive-api.open-meteo.com):
  Compiler-/Mess-Datensatz-Netlocs, **nie löschen**; 3 in dead_sources.φ
  dokumentiert.

## Registry-Urteile (Step 3, was die nächste Session tut)

- **70 dead_documented `stale_pending`** → kein Registry-Urteil offen; Release
  ist Rest, Verbleib entscheidet Step 5 (mit Nachweis, nie die letzte Kopie).
- **65 undocumented `stale_pending`** → `pending`, je einzeln im Force-Gate-
  Grind (SOURCE_PORT.md): gehört der Netloc als lebende Quelle zu sources.φ
  (Granit 7 — dann voller Block: tau/force/map/fields) oder ist er tot
  (→ dead_sources.φ)? Die Erreichbarkeitstabelle ist der Vorfilter. Nicht
  entschieden → bleibt `pending` (0 honored), nie am CDN geraten.
- **15 `repo_tag`** → §1 kein Registry-Heim; das Verdikt ist CDN-seitig
  (Step 5, nach Sicherung), kein sources.φ-Urteil.
- **6 `dataset_host`** → Compiler-Lease, behalten.

## Messgrenze

Die Erreichbarkeit wurde als einzelner HEAD auf die Host-Wurzel gemessen;
ein 4xx/5xx/Timeout kann transient sein (api.waqi.info). Diese Messung ist
eine Momentaufnahme vom 2026-09-03, kein lebenslanger Befund. Die
`dead_documented`-Klassifikation folgt der Netloc-Host-Übereinstimmung mit
dead_sources.φ — per-URL-Lesart bleibt bei unklaren Einzelfällen offen.

## Registrierung

Der Registry-Grind der 65 undocumented Netlocs ist eine offene Pflicht in
`docs/TODO.md` (Auftrags-Programm, Pflege & Struktur →
`auftrag-saubere-datenbank.md`). Step 4 (CI-Dedupe) ist gegen die gemessene
Job-Zahl (health-check 4, kernel-flatten 18) neu zu fassen.
