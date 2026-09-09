<!--
  title: Oszillator/Zeuge/Serie — offene Folge-Aufgaben
  class: handover
  date: 2026-09-07
  sha256: f5ef7e8ef95718333c53d3f021d550a0d114d144ae38a876bbf3f0f1d295bea0
  status: archived
  see-also: docs/SOURCE_PORT.md docs/concepts/die-weberin.md docs/TODO.md
-->

# Übergabe — Oszillator/Zeuge/Serie: offene Folge-Aufgaben

Diese Übergabe schreibt die geschlossene Session; eine einzige Folge-Session liest
sie und führt die offenen Punkte aus. Gemessener Stand, nichts geraten. Der
Identitäts-Bogen ist committet; offen sind vier Folge-Aufgaben, von denen drei
durch die Folge-Session ausführbar sind und eine (igets) extern bleibt.

## 1. Committeter Stand (HEAD)

HEAD steht auf `b867c23`. Der Oszillator/Zeuge/Serie-Bogen ist committet:

- `a21d4c8` — die Identität: `src/archivar/zeuge.rs` (`FeldIdentitaet`,
  `magic_identity`, `zeugen_gate`, `serien_gate`), `phi/witnesses.φ` (die Zeugen,
  8 s2-direction + 1 gestalt), die Compiler lesen ihre Identität via
  `magic_identity`, `event_window` in `sky_tick` verdrahtet, GBCO-Verbrauch
  (`gestalt_surface_threads` + `load_gestalt_surface_threads`), NRS-Positions-
  Anbindung, `direction_z_join` + `--ci-mode` (152 Richtungen auf der CDN),
  `docs/SOURCE_PORT.md` `Force-Gate`→`Oszillator-Gate`, presence-Council
  (recorded-not-built, Hydrophon-Ton = Serie nie Presence).
- `6f4dd08` — GBCO-Dispatch geschlossen (Asset auf CDN), cddis →
  `parser-def rinex` (Token öffnet Zugang, kein RINEX-Parser im Baum).
- `b867c23` — NRS-Stationstabelle geerntet + Positions-Anbindung gebaut.

`witnesses.φ`-Assets: die 152 gemessenen Distanzen (10 Gaia + 142 z) sind auf der
CDN (`ssd.jpl.nasa.gov/skydirections.bin`, 67 165 B). Das GEBCO-Gestalt-Asset
`gebco_bathymetry.gbco` liegt auf der CDN (`opentopodata.org`).

## 2. Offene Folge-Aufgaben

### A. des-Footprint Re-Probe (Webberin §9 Stufe 5, Gestalt-Zeuge)

Der DESaccess-Dienst (NCSA) ist RETIRED (gemessen 2026-09-07). Der
DES-DR2-Footprint / die Y6A2-Coverage-Maske liegen jetzt bei drei Providers:
LIneA Science Server (`linea.org.br`), NOIRLab Astro Data Lab
(`datalab.noirlab.edu`), CosmoHub (`cosmohub.pic.es`). Register-Heim:
`phi/blocked_sources.φ` (`des.ncsa.illinois.edu`, Disposition `pending`, Note nennt
die Providers).

Schritt: bei den drei Providers die Y6A2-Coverage-Maske / den DR2-Footprint suchen
(anonym oder einfaches Konto), die anonyme Route messen (curl HTTP-Status), und —
wenn gefunden — als Gestalt-Zeuge registrieren (der Footprint ist das
Webberin-§9-Stufe-5-Asset). Wenn keine der Routen anonym die Maske liefert:
ehrlich `pending` mit gemessenem Grund benennen, nie erfinden. Das CDS-Gegenstück
(`CDS/II/371/des_dr2`, MOCServer 200) bleibt nach dem Footprint-Litmus verweigert
(positions-abgeleitet).

### B. NRS-Re-Emitt (Serie mit Position)

Die Positions-Anbindung im `noaa_nodd_bucket_harvester` ist gebaut (Deployment-
SHAPE zuerst, dann Tabellen-Fallback aus `data/pmel.noaa.gov/nrs_stations.φ`,
sonst positionslos, 0 honored) und committet. Das live-Asset
`noaa_nrs_psd.bin` (214 MB, Release `storage.googleapis.com`) trägt die Positionen
nur wo SHAPE existiert; der Tabellen-Fallback für Stationen ohne SHAPE
(NRS02–10/12/13) ist noch nicht eingefaltet.

Schritt: `noaa_nodd_bucket_harvester --emit-bin noaa_nrs_psd.bin --lsk
kernels/naif0012.tls --days <fenster> --prefix nrs/products/sound_level_metrics/
--stations-table data/pmel.noaa.gov/nrs_stations.φ --ci-mode` (Token:
`OMEGAFLOW_TOKEN` in `.secrets.local`, jetzt gültig). Verifikation: die GeoRec-
Zeilen tragen die Tabellen-Positionen für die SHAPE-losen Stationen. 214 MB —
ein voller Harvest-Lauf, kein Teilschritt.

### C. RINEX-Parser (cddis konsumierbar machen)

cddis (`cddis.nasa.gov/archive`, Earthdata-Token öffnet den Zugang, HTTP 200) ist
als `parser-def rinex` registriert — die Messung (RINEX-GNSS, em) ist echt, aber
kein RINEX-Parser existiert im Baum (gemessen). Schritt: einen RINEX-Parser bauen
(Navigation/Observation), cddis dann als Oszillator in `phi/sources.φ`
registrieren. Ein echter Code-Bau — wenn er diese Session nicht vollständig
gelingt, bleibt er als registrierte Gap, nie still.

### D. igets (extern, nicht durch die Session schließbar)

`igetsftp.gfz.de` SFTP: Passwort verweigert (gemessen 2026-09-07); das
Website-Passwort (`IGETS_USER`/`IGETS_PASS`) öffnet das SFTP nicht. Das
SFTP-Passwort kommt nur von GFZ/igets-support — extern. Re-probe, sobald es steht.

## 3. Grenzen zur Parallel-Session

Nicht anfassen/committen (laufende fremde Arbeit): `phi/sources.φ`,
`.github/workflows/qbo-cdn.yml`, `tools/harvest/src/bin/qbo_compiler.rs`,
`src/archivar/tests.rs` (QBO-Hunk), `tools/measure/src/bin/corona_conditional_probe.rs`.
`blocked_sources.φ` trägt bereits Parallel-Session-Einträge (NOIRLab LS-DR10, ISC,
Global-CMT, COSMIC-2) — committet, nicht überschreiben.

## 4. Zusammengefasst

| Aufgabe | Klasse | Entscheider |
|---|---|---|
| A des-Footprint | Re-Probe via LIneA/AstroDataLab/CosmoHub | Folge-Session |
| B NRS-Re-Emitt | Harvest-Lauf (214 MB) | Folge-Session |
| C RINEX-Parser | Code-Bau | Folge-Session |
| D igets | extern (GFZ) | nicht durch Session |

Die drei ausführbaren Aufgaben (A, B, C) sind diese Übergabe — kein Punkt ist
still, jeder hat ein benanntes Heim und einen gemessenen Stand.
