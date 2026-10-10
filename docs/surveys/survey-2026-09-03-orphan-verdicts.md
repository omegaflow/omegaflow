<!--
  title: Survey — Orphan-Releases-Verdikt (Step 3, saubere Datenbank)
  class: survey
  date: 2026-09-03
  sha256: 195546c79fdeb3ef903a0d7e1d04fa1eea6d6da0ac11852927517434c94a6eff
  status: live
  see-also: docs/auftrag/archiv/auftrag-saubere-datenbank.md,
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
  Release-Identität). Gegenprobe 2026-10-11 (`sgrep` gegen `phi/*.φ`): **12 in den
  Registern, 3 ungedeckt** — `raw.githubusercontent.com` in `dead_sources.φ`; 11 über den
  `decline`-Block `declined_sources.φ:2291–2353` (`catalog-register`/`infrastructure`);
  ungedeckt: `github.com-Bowserinator-Periodic-Table-JSON`,
  `github.com-GEMScienceTools-gem-global-active-faults`, `github.com-jbrooksuk-JSON-Airports`.
- **6 `dataset_host`** (ssd/spdf.jpl/gsfc, physionet.org,
  sentinel1euwest…, service.iris.edu, archive-api.open-meteo.com):
  Compiler-/Mess-Datensatz-Netlocs, **nie löschen**. 3 in dead_sources.φ.

## Registry-Urteile (Step 3)

- **80 dead_documented `stale_pending`** → kein Registry-Urteil offen; Release
  ist Rest, Verbleib entscheidet Step 5 (mit Nachweis, nie die letzte Kopie).
- **55 undocumented `stale_pending`** → **disponiert (2026-09-26)**: je Netloc
  gegen die Register gemessen (`sources.φ`/`dead_sources.φ`/`declined_sources.φ`/
  `blocked_sources.φ`), Ergebnis im Maschinenform `cdn_orphan_verdicts.json`
  (`disposition`: 42 declined, 11 released, 2 descoped; 0 pending).
- **14 undocumented `repo_tag`** → gegenproben 2026-10-11: 11 dokumentiert
  (`declined_sources.φ:2291–2353`), 3 ungedeckt (`Bowserinator-Periodic-Table-JSON`,
  `GEMScienceTools-gem-global-active-faults`, `jbrooksuk-JSON-Airports`); §1 kein
  Registry-Heim, das Verdikt ist CDN-seitig (Step 5, nach Sicherung), kein sources.φ-Urteil.
- **3 undocumented `dataset_host`** → Compiler-Lease, behalten.

## Messgrenze

Erreichbarkeit = einzelner HEAD auf die Host-Wurzel; ein 4xx/5xx/Timeout kann
transient sein. Momentaufnahme vom 2026-09-03, kein lebenslanger Befund.
`dead_documented` = Host-Übereinstimmung mit dead_sources.φ; per-URL-Lesart
bleibt bei unklaren Einzelfällen offen.

## Registrierung

Die Disposition der 55 undocumented `stale_pending` ist **erledigt**
(2026-09-26, siehe oben). Step 4 (CI-Dedupe) ist **gefasst** (2026-09-26) als
Abschnitt „Schritt 4 — CI-Dedupe (konkretisiert 2026-09-26)" in
`docs/auftrag/archiv/auftrag-saubere-datenbank.md`: die zwei Hauptklassen sind
gegenüber der Baseline (health-check 4 → 3, kernel-flatten 18 → 5 Jobs)
verschlankt; offen ist der Klassen-Zensus über die 315 Workflows. Step 5
(CDN-kanonisch) bleibt offen.

## Step 5 — Konsolidierungs-Plan (gemessen 2026-09-28)

Der Workflow-Klassen-Zensus (325 `.github/workflows/*.yml`: manifest 246, probe
58, build 12, register 6, manifest-watch 3) nennt 13 Netlocs, deren Release aus
≥2 Workflow-Klassen geschrieben wird. Am 2026-09-28 erneut am Baum gemessen:
`glob` über `.github/workflows/*-cdn.yml` und `*-watch.yml`, `sgrep 'gh release'
.github/workflows` (wörtlich) sowie je Netloc `sgrep -l "release create <netloc>"`
und `"release upload <netloc>"`; die Klasse folgt aus dem Dateinamen (`*-cdn.yml`
= manifest, `*-watch.yml` = manifest-watch, `harvest*.yml` = register,
`kernel-flatten.yml` = build, übrige = probe).

Kernbefund (bestätigt): kein `*-cdn.yml` liest seine Release-Menge aus
`phi/sources.φ` — jedes führt einen eigenen Tag-Satz; die einzige gemessene
Ausnahme ist `planetary-odf-cdn.yml:37`
(`expected=$(grep -oE "releases/download/${{ matrix.netloc }}/..." phi/sources.φ)`).
`sgrep 'phi/sources.φ' .github/workflows` findet außerdem nur Leser dieses
Registers (`register_sort` in `ci-check.yml`, `harvest-dispatch.yml`,
`membrane-hull-probe.yml`, `port-count.yml`), keine weitere
Release-Mengen-Bindung. Ziel für jedes `*-cdn.yml`: seine erwartete Release-Menge
an `phi/sources.φ` binden statt selbst zu führen. **Wo die Tag-Menge Register-Eigentum ist** — dynamisch abgeleitete Jahr-/Slab-Mengen wie `modis_lst_cmg-<product>-<year>` oder `ps1-dr2-*` —, trägt das Register die **Familien-Identität**; die Jahr-/Slab-Menge bleibt **gemessene Laufzeit-Ableitung** (CMR + `gh api`) — die Laufzeit-Ableitung ist die Sache selbst (Council 2026-09-28). Die ≥2 Klassen entstehen aus
`probe`/`register`/`build`-Workflows, die in denselben Release schreiben
(`galileo-trk-noise.yml`, `harvest.yml`/`harvest-long.yml`/`harvest-dispatch.yml`
über `phi/harvest.φ`, Compiler-Konstante `tapvizier.cds.unistra.fr` in
`tools/harvest/src/bin/*.rs`).

| Netloc | Klassen | kanonisch | betroffene Workflows/Releases | Schritt |
|---|---|---|---|---|
| vizier.cds.unistra.fr | manifest + probe | manifest | create/upload `denis-cdn.yml`, `extinction-curves-cdn.yml`, `mktypes-cdn.yml`, `pastel-cdn.yml`, `wds-cdn.yml`; probe `bigbang-echo.yml`; Release `vizier.cds.unistra.fr` | Release an `sources.φ` binden; probe-Writer auf das manifest-Release umstellen |
| pds-ppi.igpp.ucla.edu | manifest + probe + register | manifest | create/upload `galileo-odr-cdn.yml`, `galileo-receiver-cdn.yml`, `maven-tnf-cdn.yml`, `messenger-tnf-cdn.yml`, `voyager-odr-cdn.yml`; probe `galileo-trk-noise.yml`, `galileo-nsurr-20.yml`, `tnf-format-probe.yml`; register `phi/harvest.φ:138,255,264` | Release an `sources.φ` binden; register-Tags prüfen |
| zenodo.org | manifest (+ Klasse lt. Zensus) | manifest | create/upload `cuprate-cdn.yml`, `kyoto-pressure-cdn.yml`, `quaoar-occlt-cdn.yml`, `superdarn-cdn.yml`, `superdarn-fitacf-cdn.yml`, `tnbfits-cdn.yml`; Release `zenodo.org` | Release an `sources.φ` binden |
| spdf.gsfc.nasa.gov | manifest + probe | manifest | create/upload `flyby-odf-cdn.yml`, `mariner-occlt-cdn.yml`, `voyager-occlt-cdn.yml`; probe `pioneer-band-amplitude.yml`, `pioneer-cell-census.yml`, `pioneer-link-correction.yml`; Release `spdf.gsfc.nasa.gov` | Release an `sources.φ` binden; probe-Writer auf das manifest-Release umstellen |
| pds-rings.seti.org | manifest (+ Klasse lt. Zensus) | manifest | create/upload `galileo-ionocal-cdn.yml`, `gll-rss-odr-cdn.yml`, `gll-rss-tnf-cdn.yml`; weitere `gll-rss-atdf-cdn.yml`, `gll-rss-rsr-cdn.yml`; Release `pds-rings.seti.org` | Release an `sources.φ` binden |
| naif.jpl.nasa.gov | manifest + build + probe | manifest | create/upload `camargo-uranus-cdn.yml`, `gll-ck-cdn.yml`, `naif-ura117-cdn.yml` u. v. a.; build `kernel-flatten.yml`; probe `neptune-center-rift.yml`; Release `naif.jpl.nasa.gov` | Release an `sources.φ` binden; build-Writer auf das manifest-Release umstellen |
| tapvizier.cds.unistra.fr | manifest (+ Compiler-Konstante) | manifest | create/upload `cluster-tap-cdn.yml`, `twomrs-cdn.yml`; `upload_release("tapvizier.cds.unistra.fr", …)` in `tools/harvest/src/bin/*.rs` | Default-Konstante an `sources.φ` binden; Release `tapvizier.cds.unistra.fr` prüfen |
| ssd.jpl.nasa.gov | manifest + build + probe | manifest | create/upload `de44-cdn.yml`, `korpora-cdn.yml`, `laic-verdict-cdn.yml`, `mariner10-ephemeris-cdn.yml`, `neptune-c-spk-cdn.yml`, `neptune-de440s-cdn.yml`, `ps1-cdn.yml`, `signal-cone-audit-cdn.yml`, `sky-crossmatch-cdn.yml`, `uranus-c-spk-cdn.yml`; build `kernel-flatten.yml`; probe `bigbang-echo.yml`, `dark-flow-probe.yml`, `ephemeris-horizons-check.yml`, `membrane-hull-probe.yml`, `solar-probes.yml`, `star-dmax-probe.yml` | Releases `ssd.jpl.nasa.gov` und `ssd.jpl.nasa.gov-de` an `sources.φ` binden; probe-Writer auf das manifest-Release umstellen |
| minorplanetcenter.net | manifest + build | manifest | create/upload `mpcobs-cdn.yml`, `mpcobs-shard-cdn.yml`; build `kernel-flatten.yml`; Release `minorplanetcenter.net` | Release an `sources.φ` binden |
| irsa.ipac.caltech.edu | manifest + probe | manifest | create/upload `allwise-cdn.yml`, `dust-cdn.yml`; probe `bigbang-echo.yml`, `dark-flow-probe.yml`; Release `irsa.ipac.caltech.edu` | Release an `sources.φ` binden; probe-Writer auf das manifest-Release umstellen |
| ftp.imcce.fr | manifest + probe | manifest | create/upload `inpop-epm-cdn.yml`, `noe4-cdn.yml`; probe `inpop-testpo.yml`, `neptune-center-rift.yml`; Release `ftp.imcce.fr` | Release an `sources.φ` binden; probe-Writer auf das manifest-Release umstellen |
| data.pmel.noaa.gov | manifest + register | manifest | create/upload `d20-cdn.yml`, `tao-wnd-cdn.yml`; register `phi/harvest.φ:245`; Release `data.pmel.noaa.gov` | Release an `sources.φ` binden; register-Tag prüfen |
| modis_lst_cmg | manifest + probe | manifest | manifest `modis-cdn.yml`; probe `modis-asset-bridge.yml`, `modis-year-split.yml`; Tag-Funktion `modis_lst_cmg_tag_of` (`src/archivar/cdn.rs`) | Familien-Identität als Register-Anker; die Jahr-Menge bleibt gemessene Laufzeit-Ableitung (CMR + `gh api`) — die Laufzeit-Ableitung ist die Sache selbst; Familien-Tag `data.lpdaac.earthdatacloud.nasa.gov-modis_lst_cmg` prüfen |

**Destruktiv-Warnung.** Die Release-Vereinheitlichung ist destruktiv: pro Netloc
wird erst nach Prüfung aller genannten Tags/Releases etwas entfernt — nie die
letzte Kopie. Kanonisch ist für alle 13 Netlocs `manifest`; die abweichenden
Writer (`probe`/`register`/`build`) werden auf das manifest-Release umgestellt,
bevor ein Tag verschwindet. Zu prüfen sind je Netloc der Release-Tag selbst
(`<netloc>`, bei `ssd.jpl.nasa.gov` zusätzlich `<netloc>-de`, bei `modis_lst_cmg`
zusätzlich die `-<product>-<year>`-Tags) und die in der Tabelle genannten
Writer-Releases. Ohne gemessene Tag-Menge aus `phi/sources.φ` (Ziel) kein Löschen.
