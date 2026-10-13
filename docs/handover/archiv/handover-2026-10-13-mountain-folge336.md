<!--
  title: Handover — Mountain-Folge 336 (2026-10-13)
  session: Mountain-Linie — Lizenz-Riegel, ttl-Kadenz, CDN-Zulassung
  class: handover
  date: 2026-10-13
  sha256: 6ca2e9e00239a1a9e0758a3d6b22ca1613d1129642102c15219a07242993cefc
  status: live
-->
# Handover — Mountain-Folge 336 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-12-mountain-folge335.md` (→
`docs/handover/archiv/`).

## Burn: open 0.02 · close 0.50 · cap 0.50 — Grund: tiefes Lizenz-/ttl-/Kadenz-Atom (Riegel + 87-Auflösung + Kanonisierung + JMA), deepseek-flash, kein pro/max.

## Offen (aufgeschlüsselt)

### Ersatz-Bau für die entfernten unlizenzierten Quellen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau jeder Ersatz-Blockzeile
- **Lage:** (gemessen 2026-10-13) die unlizenzierten Quellen sind entfernt; je Quelle ist die offene Alternative **gemessen**, aber **nicht registriert**. Zu bauen (`url`/`origin`/`compiler` je Zeile):
  `jades`→JADES Zenodo `18306484` CC-BY-4.0 · `igets`→Zenodo `14545565` CC-BY-4.0 · `bgr`→BGR-IMS via umwelt.info `e8018059` CC-BY-4.0 · `jvo`→HEASARC `akaripsc` PD + VizieR `II/288` · `ssdc`(BZCAT5)→Fermi 4LAC FSSC PD · `magic`→Zenodo `13898269` CC-BY-4.0 · `egg.astro.cornell`(ALFALFA)→HI4PI Zenodo `5500447` CC-BY-4.0 · `alerce`→Zenodo `11123894` CC-BY-4.0 · `fink`→Zenodo `4036589` CC-BY-4.0 · `argovis`→Argo GDAC `data-argo.ifremer.fr` argo-free · Pantheon+-raw→Zenodo `16365279` CC-BY-4.0 · CEERS/`corral`→Zenodo `15472354` CC-BY-4.0 · ATLAS/`tap.roe`→Zenodo `15108911` CC-BY-4.0.
- **Blockade:** die Ersatz-Observablen sind teils **nicht identisch** (HiPS/Cube vs. Katalog).
- **Braucht:** je Quelle Compiler + Blockzeile (Mountain) → `terms`/`format`; die CDN-Manifestation folgt (Mycelium).

### CDN-Orphan-Takedown — Bestände der entfernten Quellen (linie:mycelium)
- **Status:** wartend | **Bindung:** linie:mycelium
- **Trigger:** Myceliums CDN-Lauf
- **Lage:** (gemessen 2026-10-13) der Riegel (`src/archivar/cdn.rs`) stoppt **neue** Uploads unlizenzierter Tags, **löscht aber keine Bestände**. Die Releases der entfernten Quellen stehen weiter auf `omegaflow/sources` (gemessen `gh release list` = 377): `minorplanetcenter.net` · `services1…9/ap1/services.arcgis.com` · `raw.githubusercontent.com` · `api.wolfx.jp` · `api.alerce.online` · `api.lsst.fink-portal.org` · `argovis-api.colorado.edu` · `babamul.caltech.edu` · `blinkverse.zero2x.org` · `blpd0.ssl.berkeley.edu` · `ceein.infp.ro` · `clpds.bao.ac.cn` · `data.kasi.re.kr` · `hinetwww11.bosai.go.jp` · `igetsftp.gfz.de` · `jades.herts.ac.uk` · `lasp.colorado.edu`(+`-eve`) · `maxi.riken.jp` · `media.githubusercontent.com` · `modis.higp.hawaii.edu` · `opendata.magic.pic.es` · `padc-tap-rcsed.obspm.fr` · `voparis-tap-astro-m.obspm.fr` · `vo.lmd.jussieu.fr` · `resonanceone.app` · `sheffieldcitycouncil.cloud.esriuk.com` · `www.hou.usra.edu`.
- **Blockade:** das CDN ist Myceliums Recht.
- **Braucht:** Mycelium nimmt die Orphan-Releases offline (`gh release delete <tag> --repo omegaflow/sources --yes`).

### Offene Ersatz-Alternativen für die noch nicht gedeckten Observablen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der jeweilige Bau
- **Lage:** (gemessen 2026-10-13) für die entfernten Quellen, deren Observable **nicht** schon durch einen registroten PD/CC-Beleg gedeckt ist (USGS/NOAA/OpenAQ/Water-Quality-Portal/Open-Meteo decken die ArcGIS-Layer), sind die Alternativen gebaut: siehe „Ersatz-Bau".
- **Blockade:** keine.
- **Braucht:** den Ersatz-Bau (oben).

## An mycelium

Origin: mountain-folge336 (2026-10-13).

- **CDN-Orphan-Takedown (der Riegel löscht keine Bestände).** Der Riegel
  (`upload_release`/`registered_netlocs`, `src/archivar/cdn.rs`) verweigert jetzt
  jeden Upload eines Tags, dessen Block keine anerkannte Lizenz trägt — **neue**
  unlizenzierte Uploads sind ausgeschlossen. Die **Bestände** der entfernten
  Quellen stehen aber weiter auf `omegaflow/sources`: bitte die Orphan-Releases
  offline nehmen (`gh release delete <tag> --repo omegaflow/sources --yes`). Volle
  Liste im Punkt „CDN-Orphan-Takedown". `xfer:mountain-336-1`
- **Ersatz-Bau der offenen Alternativen.** Die entfernten unlizenzierten Quellen
  tragen je eine gemessene offene Alternative (Liste im Punkt „Ersatz-Bau"); die
  Quellen mit **nicht bereits registriertem** Ersatz sind dort namentlich genannt.
  Mountain baut die Blöcke; Mycelium manifestiert sie aufs CDN (nach dem Riegel).
  `xfer:mountain-336-2`

## Operator-Wort-Register

- **„Projekt-Ownership statt statischer Verantwortlichkeiten — jeder schiebt den schwarzen Peter … ihr seid alle gleich ausgebildet"** | 2026-10-13 | Operator-Session (mountain 336) — kein Ball-Zuschieben; jede Linie führt die Projekt-Arbeit durch.
- **„warum haben wir immer noch no-license-stated und fragwürdige lizenzen auf cdn?"** | 2026-10-13 | Operator-Session (mountain 336) — Riegel + Auflösung der 87; `no-license-stated` = 0, unzugelassene Terme = 0.
- **„bitte prüfe was zu viel entfernt wurde"** | 2026-10-13 | Operator-Session (mountain 336) — 8 zu viel entfernte Blöcke (Host-Muster) wiederhergestellt.
- **„und haben wir jetzt für alle offene alternativen?"** | 2026-10-13 | Operator-Session (mountain 336) — teils; der Ersatz-Bau ist offen.
- **„liegen jetzt nur noch einwandfreie Assets auf CDN?"** | 2026-10-13 | Operator-Session (mountain 336) — nein; die Orphan-Bestände stehen, Takedown an Mycelium.
- **„das muss eindeutig recherchiert und an mycelium gegeben werden"** | 2026-10-13 | Operator-Session (mountain 336) — der `## An mycelium`-Block trägt Orphans + Alternativen.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push. In diesem Atom (folge336)
committet (pfad-begrenzt/block-scoped): der **Lizenz-Riegel** (`src/archivar/cdn.rs`),
der **Gate-Test** (`src/gate/commit_gate.rs`), die **Kadenz-Korrekturen**
(`phi/sources.φ`: epm/impc ttl, 1149 statische → `no-cadence`), die **Lizenz-Auflösung**
(87 `no-license-stated` → 0, unzugangene Terme → 0, 8 wiederhergestellt), die
**Producer-Crons** (4 Workflows), das **OpenNeuro-Compiler-Block-Emission**, der
`register_ttl_apply`-Werkzeugbau, die **Kanonisierung** (`register_sort`,
(ttl,url)), der **JMA-Doppelblock** entfernt. **Warnung:** der Arbeitsbaum ist von
parallelen Linien gehalten — vor jedem Commit das eigene Pfad-Set messen.
