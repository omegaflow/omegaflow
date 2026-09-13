<!--
  title: Handover — Ernte-Folge 14 (Stand 2026-09-13)
  session: Ernte-Folge 14
  class: handover
  date: 2026-09-13
  sha256: b1dd5f172aac632962f68b61b8b4b1b5f7d74748bf3a8c81c5ee7ed4811cafde
  status: live
-->
# Handover — Ernte-Folge 14 (2026-09-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## CDN-Manifestation (Wächter)

- hinet.bin — Wächter: Run 34746164903 (2026-09-13T07:49Z) rot. Gemessene Ursache:
  `available_id` (tools/harvest/src/bin/hinet_win32_compiler.rs) split das Status-HTML
  auf `"<tr>"` und kollabiert jede `<tr class=…>`-Zeile in einen Brocken — der cont-Poll
  las nie „Available" (8×240s erschöpft). Der Fix (`split("<tr")` + Regressionstest)
  steht in fremdem, ungepushtem Commit 8967503 („implement AVE/REST re-referencing
  before TE"), dieselbe Datei:310. Re-Dispatch Run 34751691539 fliegt auf origin/main
  91528131 → trägt den Fix nicht. Duty: der Fix muss auf origin/main stehen (der fremde
  Push), erst dann trägt ein neuer Dispatch ihn. Kein erneuter Dispatch, kein Cherry-Pick.

## Weberin — offene Fäden (Ernte)

- Phobos/Deimos-Zweitlinie — NOE-4-2020.bsp (ftp.imcce.fr/pub/ephem/satel/NOE/MARS/2020/,
  244 MB, NAIF 401/402) live; keine lokale .bsp-Kopie (die Registrierung trägt nur
  NOE-5-Galilei-Kernels, kein Mars). Kein NOE-Compiler/Format → pending (Bau; Fetch zuerst).
- WWLLN — Nachfolger-Host `ghrc.earthdata.nasa.gov` (WWLLN-DAAC, HS3 WWLLN Storms V1
  C1979872496) + `wwlln.net/climate/th_yr/` netCDF-Jahresgitter (Lead in
  phi/pipeline/queue/grind_weberin_gap_routes.φ). Absent lokal. Compiler+CDN-Duty
  pending (wie iss_lis).
- MPC — Vollkatalog absent lokal (`mpcorb_distant.bin` = TNO-Teilmenge auf dem CDN);
  Distant-Filter lösen + MPCAT-OBS/`get-obs`-Linie. Host `cgi.minorplanetcenter.net`
  (mpeph2.cgi) neu → pending.
- Broker/GW-Positionen — antares.noirlab.edu abgelehnt (credential-gated, nur
  Counts/Position); api.fink-portal.org tot (DNS ok, Connect 000); bayestar: keine neuen
  Positions-Hosts. Der Compiler trägt nur die Richtung → pending (Positions-Ernte).

## INPOP25c — Manifestation (Register-Duty)

- phi/pipeline/catalog/asteroid_gm_inpop25c.φ trägt jetzt 196/196 (Voll-Ernte; die 4
  Altwerte A=A-verifiziert). .gitignore-Ausnahme gesetzt (diese Session).
  OFFEN: die sources.φ-Url-Zeile für die TAP-Quelle J/A+A/705/A189 (VizieR) fehlt —
  sources.φ war beim Abschluss fremd-gestaged, die Zeile ist eine exakte Duty der
  nächsten Session (nach dem fremden Push). Stehendes Muster benannt: kernel-flatten.yml
  nennt asteroid_diameters_{neowise,akari}.φ, die auch gitignored sind — dieselbe Klasse,
  nicht diese Session.

## Offene Pendings

- Eclipse-2024 — Ursache der 793,3 km gemessen: Suchbox-Grenze (die unseeded
  deepest_pierce trug half=90° für lon UND lat; lon braucht ±180°). Probe gefixt
  (half_lat 90°, half_lon 180°) + Paper aktualisiert. OFFEN: Re-Verifikation des
  2024er Stufe-2a-Punkts gegen den Kanon — steht, bis der Baum kompiliert (die Kern-Lib
  ist aktuell von fremder volume.rs-Arbeit zerbrochen, 11 Fehler).

## Abschluss

- HEAD 8967503 (fremd, ungepusht) über origin/main 91528131. Der fremde Commit hat
  eigene Arbeit dieser Session mitgenommen (igets-Zeilen-Löschung, NRS-Tiefen, hinet-Fix)
  — sie reisen mit dem fremden Push.
- Kern-Lib kompiliert nicht (11 Fehler in fremdem src/archivar/volume.rs: E0425 align16,
  E0433 sha256, E0277 Axis:Copy, unused lat0/lat1/lon0/lon1/d0/d1); cargo check blockiert,
  bis die volume-Bauer-Linie pusht.
- Fremde uncommittete Arbeit beim Abschluss (neu gemessen, nicht kopiert): staged
  (forschung-folge11→archiv Rename, forschung-folge12 Add, phi/sources.φ,
  placebo_pair_eeg_probe.rs, tonga_lamb_crosscheck_probe.rs, eeglab.rs), unstaged
  (docs/SOURCE_PORT.md, src/archivar/mod.rs, archive_search{.rs,index.rs,net.rs}),
  untracked (bin/proton-exit.sh, src/archivar/volume.rs, archive_search/playwright.rs +
  playwright_fetch.cjs, volume_builder.rs) — unberührt. Gepusht wird erst, wenn der Baum
  ruhig ist und das Wort kommt.
