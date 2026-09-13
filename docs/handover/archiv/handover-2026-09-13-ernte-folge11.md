<!--
  title: Handover — Ernte-Folge 11 (Stand 2026-09-13)
  session: Ernte-Folge 11
  class: handover
  date: 2026-09-13
  sha256: b9ae8136d68e7ab985dc4f127710b24e204dd0824ad8fe7cafcb303fb0463bae
  status: live
-->
# Handover — Ernte-Folge 11 (2026-09-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Nur eigene Arbeit: bei
geteilten Dateien nur die eigenen Hunks; gepusht wird erst, wenn der Baum ruhig ist.

## CDN-Manifestation (Duty)

- igets.bin — Wächter: Redispatch 34718762450 stand am 2026-09-13 (15-min-Watch)
  noch `queued` — der Wächter misst das Asset im Release igetsftp.gfz.de.
- slab2_depth.bin — der `slab2-cdn`-Dispatch steht beim Push an (das Wort) — der
  Wächter misst das Asset im Release www.sciencebase.gov (Run über
  `gh run list --workflow slab2-cdn.yml`).

## Ernte

- Slab2 — registriert (phi/sources.φ, `format slab2_depth`, ttl 31536000) und
  gebaut: `slab2_compiler` (xyz→Records, NaN/≤0-Skip, Roundtrip, `--ci-mode`),
  Zeuge `SLB2`, `slab2-cdn.yml` (idempotent, 240 min). Gemessen: ScienceBase-
  Item 5aa1b00ee4b0b1c392e86467 → HTTP 200, tar.gz 140 213 438 B, 27 Regionen,
  `_dep_`-xyz. Offen: die Loader-Verdrahtung (Gestalt-Klasse GL30/GBCO) — pending.
- 3D-Geschwindigkeitsmodelle — gemessen (2026-09-13): EarthScope EMC
  `data.earthscope.org/catalog/emc/model_pages/EMC-UUP07.json` → 403 AccessDenied
  anonym (auch LLNL-G3D-JPS, TX2019, S40RTS; IRIS-Legacy ds.iris.edu/media/
  product/emc → 403). Kein öffentlich-anonymer Fetch → nicht registriert.
  Offen: Alternativ-Routen (SubMachine, IRIS-Events-Legacy) ungesucht — pending,
  der nächste Schritt ist benannt.
- Hi-net — HINET_PASS fehlt lokal (gemessen 2026-09-13) → die Re-Messung des
  serverseitigen Blocks (36-s-Cut/Prep-Abbruch, gemessen 2026-09-12) steht aus;
  Portal-Routen erreichbar (HTTP 200). Die Registrierung (sources.φ,
  hinet-cdn.yml, hinet-Compiler) steht als Socket.

## Weberin — offene Fäden (Ernte)

- Innenplaneten/Monde-Zweitlinie — INPOP/EPM-Kernel für Merkur…Mars + die
  Monde ernten (zweite unabhängige Abstammung; die Eisriesen tragen sie schon).
- WWLLN — offener Thunder-Hour-Host (Nachfolger des toten GHRC-ERDDAP), Format
  + Auflösung; Realtime-Roh ist `not-published` (Mitgliedschaft).
- NRS-Hydrophon — Position/Identität: NRS02–10/12/13-Koordinaten aus der
  Netz-Tabelle, Spektren-Anker (nur NRS01/11 tragen SHAPE).
- INPOP25c-Asteroidenmassen — gravity-Katalogroute (`die-weberin` §1).
- MPC-Orbits — unabhängige zweite Körper-Linie (`mpcobs_compiler` steht, Route
  live).
- Broker-/GW-Positionen — Lasair/ANTARES/Fink loci + bayestar-Sky-Maps:
  positions-pending (die Richtung trägt der Compiler, die Position fehlt).

## Offene Pendings

- Archivar-Cache — gemessen (2026-09-13): ein stale lokales earth-bin
  (2026-09-09 15:52Z, 19 MB) überlebte die CDN-Regeneration (2026-09-11) bis zum
  ttl — der eclipse-Drift-Befund war Cache-Stale, kein Daten-Defekt (die
  Re-Verifikation steht im Paper). Der updatedAt-Abgleich des Cache-Layers gegen
  das CDN steht aus — pending.
- Eclipse 2024-Kanon-Punkt — gemessen: Δ 793,3 km auf allen fünf Linien gleich
  (Kanon-vs-Algorithmus-Punktdefinition, keine Ephemeriden-Drift) — pending
  (im Paper getragen).

## Abschluss

- Baum beim Sessionsstart: HEAD == origin/main == f6cea32. Fremde uncommittete
  Arbeit: docs/concepts/die-akteure-im-boden-und-wasser.md — unberührt.
  Erledigt aus Folge 10: harps-Wächter (success), H₀-Crossmatch (74/74
  source_ids, Blatt + Paper), de441 mars (Anker-Δ 0,0 km), de441 e/m/s (eine
  Stimme, Kalibrier-Gate hält), Slab2-Bau. Gepusht wird erst mit dem Wort.
