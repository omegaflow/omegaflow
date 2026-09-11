<!--
  title: Handover — Ernte-Folge II (Stand 2026-09-11)
  session: Ernte-Folge II
  class: handover
  date: 2026-09-11
  sha256: 91670d74ed9c54d06f2617f28fe8b86ce514c6d2f09fb5243bc6e1e0553a232c
  status: live
-->
# Handover — Ernte-Folge II (2026-09-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## CDN-Manifestation (Duty)

- jup365 — der DafFile-pread-Fix liegt uncommittet im Baum
  (`src/archivar/bsp_reader/daf.rs`: `DafSource` `Owned|File`, `read_exact_at`,
  `double_slice` gelöscht, `from_data` bleibt Byte-Paritäts-Referenz;
  `cargo check` 0/0 für core + utils/measure/harvest). Offen: Commit (einer
  Session) → Push (ruhiger Baum + Wort) → `gh workflow run kernel-flatten.yml`
  → bodies-Job grün + jup365-Release am CDN.
- noaa-ccor — sources.φ-Block liegt uncommittet (`format noaa_ccor`, `at sun`,
  Feld `noaa_ccor_intensity_dn`); die CDN-Manifestation wartet auf denselben
  Push.
- igets.bin — letzter Lauf cancelled bei ~6h (Run 34571077129; der Compiler
  läuft den vollen SFTP-Baum und holt jedes `.ggp` sequentiell, kein
  persistenter Cache). Redispatch allein hilft nicht — die Verengung
  (`--station`/`--limit-files` oder Cache-Persistenz im `igets_compiler`) ist
  das nächste Atom.
- noaa-dcdb / noaa-keo-papa / copernicus-icoads — Ursache gemessen: GitHub-API-
  Rate-Limit (transient; `ensure_release` → `gh release create` 403).
  Redispatch nach Reset, wenn der Baum ruhig ist.

## Ernte (Harvest)

- Hi-net/NIED WIN32 — Compiler + WIN32-Reader offen; Zugang entsperrt (Login
  `omegaflow`, Ablauf 2027-03-31). Vor-2004-Bestand = Antrag (`form past`).
- Pioneer-10-ATDF-Rest — 238 Dateien / 77 Tage (SPDF
  `ATDF_Data-Files_CMarkwardt_Readable`).
- ESO-TAP + Discovery-Download-Workflow (Korpora) — warten auf die vo-tap-Arme.
- noaa-jpss — OMPS-SDR-Reader gebaut; Datenzugang offen (anonymes S3-GET liefert
  CLASS-HTML, kein HDF5) → Render-Verdrahtung + sources.φ-Block nach Zugang.

## TAP-Klassifikation

- 7 Pending — Re-Probe gemessen 2026-09-11: alle 7 weiter down (dachs.fai.kz
  sync 500, lmd „Connection refused", roe ×4 „INTERNAL SERVER ERROR", pithia
  „connection pool closed"). Fremde Zustände — warten, nicht bauen; der
  Re-Probe-Takt reicht.
- Distance-Keys (9/13 ohne plx/dist/z) — Quellen-Kuratierung.

## Register

- Step-5-Folge — destruktiver Schnitt (verifiziert).
- docs-reference-verteilung — Bewegung + Referenz-Rewiring, Seeds → Survey-Heimat.

## Speisekammer / Nachlese

- Speisekammer-Fragen — aia2014, planck, eve, omni2, goes15, gebco.
- dataone — terms-Seite 401; Lizenzstatement unlesbar, Redistribution offen.
- cadc.argus — erreichbar direkt (`ws.cadc-ccda…/argus/sync`), FORMAT=json
  abgelehnt (nur CSV/VOTable) → parser-def votable; die Register-Klassifikation
  (blocked_sources.φ) bleibt offen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check (`/abschluss`). Der Push ist gehalten — der Baum ist nicht
ruhig; der Wächter-Dispatch (jup365) und die noaa-ccor-Manifestation warten auf
ihn.
