<!--
  title: Handover — Ernte-Folge III (Stand 2026-09-11)
  session: Ernte-Folge III
  class: handover
  date: 2026-09-11
  sha256: 98b8820cd95e4ba8c347129f8ac1d074db53be164187bc174183cac40b823c66
  status: live
-->
# Handover — Ernte-Folge III (2026-09-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## CDN-Manifestation (Duty)

- jup365 — kernel-flatten dispatcht (Run 34619465795); Erfolgskriterium:
  bodies-Job grün + jup365-Release am CDN. Der DafFile-pread-Fix ist auf main
  (f67b14d).
- noaa-ccor — dispatcht (Run 34619470037); Erfolgskriterium:
  noaa-ccor-Release am CDN (Block sources.φ Z. 7900).
- igets.bin — die Verengung ist gebaut und committet (7233165: Cache-Persistenz
  über actions/cache `save-always`, size-verifizierter Download, atomarer
  Rename; `--stream` abgelegt; concurrency hält einen Lauf). Offen: Push
  (ruhiger Baum + Wort) → `gh workflow run igets-cdn.yml` → Release. Der
  Cache-Neuaufbau braucht mehrere Dispatches (Resume über den Cache).
- noaa-dcdb / noaa-keo-papa / copernicus-icoads — redispatcht nach
  Rate-Limit-Reset (Runs 34619474106 / 34619477827 / 34619482085); Wächter auf
  grün.

## Ernte (Harvest)

- Hi-net/NIED WIN32 — Compiler + Reader gebaut und committet (827c1ad:
  `src/archivar/win32.rs`, `tools/harvest/src/bin/hinet_win32_compiler.rs`;
  magic/comp-Konstanten in geo.rs). Offen: (a) `HINET_PASS` fehlt in
  `.secrets.local` (gemessen) — Operator-Schritt; (b) der Live-Web-Vertrag
  (auth-Feldnamen, select-CGI, cont_status.php-Form, `.cnt`-Pfad) ist
  unverifiziert — gebaut nach Vertrag, nie gegen den Dienst gefahren; (c)
  sources.φ-Block + Render-Pfad nach dem ersten echten Lauf; (d) die Einheit
  ist raw ADC counts — die Kalibrierung zu m/s fehlt, das Feld darf erst mit
  SI-Einheit registriert werden; (e) Vor-2004-Bestand = Antrag (`form past`).
  Kein sources.φ-Eintrag wurde fabriziert — der Block wurde zurückgenommen, bis
  ein Lauf die Werte trägt.
- Pioneer-10-ATDF — die Prämisse „238 Dateien / 77 Tage" ist gemessen falsch:
  das Verzeichnis trägt 23 ATDF-Dateien (SHA1SUM, ~373 MB). Der Compiler
  enumeriert jetzt zur Laufzeit (19ed6d0) + `pioneer-atdf-cdn.yml` gebaut.
  Offen: (a) Dedup-Entscheid — `PIO10.F18` und `pioneer10.fl2` tragen
  identische SHA1 (7a821e67…), der Voll-Merge zählt doppelt; (b) Push →
  `gh workflow run pioneer-atdf-cdn.yml`.
- ESO-TAP + Discovery-Download-Workflow (Korpora) — warten auf die vo-tap-Arme.
- noaa-jpss — OMPS-SDR-Reader gebaut; Datenzugang offen (anonymes S3-GET liefert
  CLASS-HTML, kein HDF5) → Render-Verdrahtung + sources.φ-Block nach Zugang.

## TAP-Klassifikation

- 7 Pending — Re-Probe gemessen 2026-09-11: alle 7 weiter down. Fremde
  Zustände — warten, nicht bauen; der Re-Probe-Takt reicht.
- Distance-Keys (9/13 ohne plx/dist/z) — Quellen-Kuratierung.

## Register

- Step-5-Folge — destruktiver Schnitt (verifiziert).
- docs-reference-verteilung — Bewegung + Referenz-Rewiring, Seeds → Survey-Heimat.

## Speisekammer / Nachlese

- Speisekammer-Fragen — aia2014, planck, eve, omni2, goes15, gebco.
- dataone — terms-Seite 401; Lizenzstatement unlesbar, Redistribution offen.

## Abschluss

Vor Commit/Push: das Consent-Wort des Operators (`/consent`) und der gemessene
Abschluss-Check (`/abschluss`). Der Push ist gehalten — der Baum ist nicht
ruhig (fremde Commits ahead); die Dispatches igets/pioneer-atdf warten auf ihn.
