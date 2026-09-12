<!--
  title: Handover — Ernte-Folge V (Stand 2026-09-11)
  session: Ernte-Folge V
  class: handover
  date: 2026-09-11
  sha256: bed4af552f45168cb4b25aaa36a9fe4b68a8985538878ed54893ad11e2806564
  status: live
-->
# Handover — Ernte-Folge V (2026-09-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## CDN-Manifestation (Duty)

- igets.bin — dispatcht (Run 34634693904, seit 18:41Z in_progress, >3 h); Wächter:
  igets-Release `igetsftp.gfz.de` trägt igets.bin — gemessen 404.
- pioneer-atdf — Bug gefunden und gefixt: `pioneer_atdf_compiler.rs` schrieb nach
  `data/spdf.gsfc.nasa.gov/`, legte aber nur `data/` an (`create_dir_all("data")`)
  → `write … void` → Exit 0 vor `upload_release`. Grüner Run, kein Asset. Der Fix
  liegt uncommittet; nach Commit/Push redispatchen; Wächter: `pioneer10_skyfreq.bin`
  auf `spdf.gsfc.nasa.gov`.
- aia2014 — dispatcht (Run 34652444462, seit 22:05Z in_progress);
  `aia2014_fullyear.bin` antwortet 302 (Asset liegt); Wächter: Run grün.

## Ernte

- Hi-net — `HINET_PASS` fehlt weiter (Operator). Live-Web-Vertrag gemessen: Auth-Endpoint
  `https://hinetwww11.bosai.go.jp/auth/?LANG=en` lebt (200), Felder `auth_un`/`auth_pw`
  stimmen zum Compiler; aber die vier Post-Login-Endpunkte (`select_check.cgi`,
  `select_confirm.php`, `cont_request.php`, `cont_status.php`) = 404 auf beiden Hosts —
  der Compiler-Vertrag benennt Endpunkte, die nicht existieren; erst ein echter Lauf mit
  `HINET_PASS` misst die wahren URLs. Vor-2004-Bestand = descoped (Operator-Wort).
- noaa-jpss — Zugang gemessen: EDL-Token gültig (uid omegaflow.space, exp 2026-09-30),
  Download = 403 EULA-Acceptance-Failure. Operator-Akt: EULA akzeptieren unter
  `https://urs.earthdata.nasa.gov/approve_app?client_id=e2WVk8Pw6weeLUKZYOxvTQ`. Danach
  `--url` auf ein echtes OMPS-Granule (CMR short_name mit Underscores: `OMPS_NPP_NMTO3_L2`,
  GES_DISC).
- ESO-TAP — vo-tap-Arme gebaut (fremde Session); offen: Feldblöcke + Compiler. Die
  ernte-folge4-Zeile trägt die fremde Aktualisierung; ernte-folge4 bleibt live.

## TAP-Klassifikation

- 7 Pending — Re-Probe (HEAD) heute: alle sieben antworten (200/301, zuvor 500/refused).
  Schwächeres Signal (HEAD auf Basis-URL, kein sync-GET); der Re-Probe-Takt reicht, ein
  sync-GET bestätigt die Funktion. Datei:
  `phi/pipeline/research/agent_output/tap_klassifikation_2026-09-11.φ` (gitignored, lokal).

## Descoped (gemessen, kein offener Punkt)

- gebco — Punkt-Abfrage lebt und ist manifestiert (`gebco_bathymetry.gbco`, opentopodata.org);
  das 4,25-GB-Grid ist ungebaut und hat keinen Konsumenten — Re-Raise-Pfad = opentopodata-Endpoint.
- dataone — Katalog-only (`phi/pipeline/catalog/dataone_catalog.φ`); Data-Terms antworten 401,
  Lizenz unverifiziert — die Quelle verweigert.

## Register

- eve_lines_2011.bin — url-line registriert (format eve, at sun, ttl 86400, ssd-Release).
  Bleibende benannte Schuld: `eve_compiler.rs` trägt `CDN_TAG "lasp.colorado.edu"` (Mismatch
  zum ssd-Asset) — eigene Zeile, nächster Atom.
- goes15 — avg1m (`goes_xrs.bin`) war bereits registriert; das 2-s-Korpus
  (`goes15_xrs_2s_*.tar`, ncei.noaa.gov) bleibt unregistriert benannt — kein Format trägt
  einen unkonsumierten tar-Korpus (Rat: keine erfundene Format-Zeile; Konsumform ist registriert).

## Abschluss

Commit/Push hält auf das Wort. Eigene Dateien dieses Atoms: `pioneer_atdf_compiler.rs`
(1 Hunk), `phi/sources.φ` (1 Hunk, eve-Block) + dieses Handover. Fremde uncommittete Arbeit
im Baum (`src/archivar/*`, `src/mathematikerin/*`, `phi/blocked_sources.φ`, ernte-folge4-Zeile)
bleibt unberührt. ernte-folge4 wird erst archiviert, wenn die vo-tap-Session gelandet ist.
