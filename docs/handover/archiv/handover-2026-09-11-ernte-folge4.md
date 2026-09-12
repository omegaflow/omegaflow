<!--
  title: Handover — Ernte-Folge IV (Stand 2026-09-11)
  session: Ernte-Folge IV
  class: handover
  date: 2026-09-11
  sha256: 8fc48058c702b8d7229462382a1b2e962cb529969185aafde6c48d6f55812549
  status: archived
-->
# Handover — Ernte-Folge IV (2026-09-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab, wie sie kann — Sub-Agenten tragen eigenen Kontext, die Anzahl
ist kein Aufwand. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## CDN-Manifestation (Duty)

- igets.bin — dispatcht (Run 34634693904, in_progress); Wächter: igets-Release
  am CDN. Die Verengung + `igets-cdn.yml` leben auf main; die Alt-SHAs
  7233165 / 19ed6d0 sind nach History-Neuschreibung unerreichbar — der Inhalt
  ist da, die SHAs nicht.
- pioneer-atdf — dispatcht (Run 34634697539, in_progress); Wächter:
  `pioneer10_skyfreq.bin` am CDN.
- jup365 — kernel-flatten Run 34619465795 grün (bodies-Job), aber kein jup365.bsp im Release naif.jpl.nasa.gov am CDN gemessen → kernel-flatten redispatchten.

## Ernte (Harvest)

- Hi-net/NIED WIN32 — Kalibrierung ADC→m/s gebaut (WinSensitivity +
  velocity_m_s in src/archivar/win32.rs; natives NIED-Kanaltabellen-Parsing im
  Compiler; uncommitted). Offen: (a) HINET_PASS fehlt in .secrets.local
  (Operator); (b) Live-Web-Vertrag unverifiziert; (c) sources.φ-Block nach dem
  ersten echten Lauf; (d) Vor-2004-Bestand = Antrag.
- Pioneer-10-ATDF — Dedup gebaut (pioneer_atdf_compiler.rs: identische SHA1 →
  ein Asset + Alias-Zeile; uncommitted). Offen: Commit/Push-Wort + Wächter
  Run 34634697539.
- noaa-jpss — Zugang gemessen: NODD `noaa-jpss` (us-east-1) liefert für
  OMPS-Pfade CLASS-HTML (222142 B, Titel „NOAA's Comprehensive Large Array-data
  Stewardship System"), kein HDF5; der Reader ist end-to-end verdrahtet (--url
  → curl → HDF5 → OMP1-bin). Offen: CLASS- oder EDL-Zugang beschaffen, dann
  --url auf ein echtes Granule.
- ESO-TAP + Discovery-Download (Korpora) — vo-tap-Arme gebaut; offen: Feldblöcke + Compiler.

## Step-5-Schnitt — Nachlese (aus dem Schnitt)

- ourairports-Mirror — 351-Byte-Stub (Repo trägt 12,7 MB, Digest ≠) → das
  korrekte File neu ernten.
- gem harmonisiert-Mirror — fehlbenannt (Bytes = nicht-harmonisiertes
  gem_active_faults, 12,3 MB) → das korrekte harmonisierte File ernten.

## TAP-Klassifikation

- 7 Pending — fremde Zustände, Re-Probe-Takt reicht.
- Distance-Keys — 13 cmap-TAP-Drafts vermessen: 1 Abstandsschlüssel ergänzt (ssdc Roma-BZCAT → `z z`); 8 tragen gemessen keinen Abstand (LoTSS, MP3C, Planck-ERCSC, INTEGRAL, HyperLEDA, OGLE, XSA, 4XMM — keine plx/dist/z-Spalte, kein Join) → distanz-lose Referenzen, kein Schlüssel existiert. Datei: phi/pipeline/research/agent_output/tap_klassifikation_2026-09-11.φ.

## Register

- docs-reference-verteilung — ausgeführt (23 Dateien nach der vermessenen Karte bewegt: Seeds → surveys umbenannt, Messreihen → concepts, Literatur → paper, Konten → handover/archiv). Offen: 9 Alt-Pfad-Referenzen in fremder uncommitteter Arbeit (.github/workflows/disequilibrium-census.yml, tools/measure/src/bin/disequilibrium_register_probe.rs, techno_gas_register_probe.rs) zeigen noch auf docs/reference/ → beim Landen jener Session neu verdrahten.

## Speisekammer / Nachlese

- aia2014 — aia2014_fullyear.bin am CDN 404 (lokal vorhanden) → manifestieren
  (aia_compiler --merge --ci-mode) + url-line, oder mit der Monatsbins-Messung
  descopen.
- eve — EVE-URL-Zeile registrieren (solarer EUV-Oszillator) oder
  descope-Grund; eve_compiler.rs existiert.
- omni2 — omni2_indices.bin (AE/AL/AU/DST/SYM-H) url-line registrieren (die 7
  HAPI-Felder sind schon in sources.φ).
- goes15 — goes_xrs avg1m url-line registrieren.
- gebco — Grid-vs-Punkt entscheiden (GEBCO_2026 = 4,25 GB; Punkt-Abfrage
  läuft).
- planck — halten (CMB/Staub/Katalog registriert; Frequenzkarten = benanntes
  ungebautes Feld).
- dataone — Terms 401; Software Apache-2.0 offen, Daten-Lizenz unverifiziert →
  Terms anfragen oder einen Wayback-Snapshot pinnen.

## Abschluss

Commit/Push hält auf das Wort. Die eigenen Dateien
(pioneer_atdf_compiler.rs, win32.rs, hinet_win32_compiler.rs + dieses Handover)
sind isoliert und alleinverfasst; fremde uncommittete Arbeit (.github/*.yml,
phi/sources.φ, tools/measure/*) bleibt unberührt. Baum-Messung: HEAD ==
origin/main (0/0), nichts gestaged.
