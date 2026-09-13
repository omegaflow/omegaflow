<!--
  title: Handover — Ernte-Folge 19 (Stand 2026-09-13)
  session: Ernte-Folge 19
  class: handover
  date: 2026-09-13
  sha256: b5a2b773b995e22ea0dcc9fcfea976635e12b8d7c8b3a15f404a0be0ef97f426
  status: live
-->
# Handover — Ernte-Folge 19 (2026-09-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die eigenen
Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit wird nie
überschrieben; gepusht wird erst, wenn der Baum ruhig ist.

## Weberin — offene Fäden (Ernte)

- MPC-Shard-Verdrahtung — der Shard-Modus im `mpcobs_compiler` steht (`--shard
  <prefix>`, packed-number-Bereich, 2³⁰ B je Shard, Name `<prefix>-<lo>-<hi>.bin`,
  Shard-Zahl live berechnet; 7 Tests grün) + `mpcobs-shard-cdn.yml` (UnnObs 455 MB,
  HEAD gemessen 455 768 106 B). OFFEN: der UnnObs-Job dispatcht und misst (a) wahre
  unnobs.bin-Größe (b) inflate-Durchsatz (c) Upload-Zeit; erst mit diesen drei Zahlen
  werden die Shard-`url`-Einträge in sources.φ (je Shard ein Eintrag, Operator-Wort)
  und der NumObs-Job (HEAD gemessen 9 109 431 211 B) registriert — Schwelle aus
  Live-Daten, nicht festgeschrieben.

- Broker/GW-Positionen — pending (gemessene Absenz, re-gemessen 2026-09-13):
  gwosc/graceDB tragen keine anonyme ra/dec-Punktposition. Der Punkt bleibt pending.

## Ernte-Nachlauf (Surveys Weberin-Quellen, Folge)

- Argovis `/bgcargoplus` — live (JSON-Profil, 5 distinkte radiometrische Variablen,
  absent aus argo_bgc.bin: DOWN_IRRADIANCE380/412/490, DOWNWELLING_PAR [em], CDOM
  [diffusion]). OFFEN: der sources.φ-Profilblock (Spiegel der Zeile-1248-Grammatik)
  + die drei `convert_to_si`-Arme (TeV-1/cm2/s → W/m², W/m2/nm → W/m³,
  microMoleQuanta/m2/sec → W/m²) — der Block schreibt erst mit kuratierten Einheiten.

- Gaia-Alerts — kompiliert (`gaia_alerts.bin`, SKD1), Witness-Block steht in
  witnesses.φ (`record skydirection`). OFFEN: der Merge des Assets in
  `skydirections.bin` (`skydirection_compiler --gaia-alerts`-Arm) + SKD1-magic absent
  in `zeuge.rs` `magic_identity` (Zeugen-Gate pending).

- Witness-CDN-Manifestation — `hawc-cdn.yml`, `lhaaso-cdn.yml`, `gaia-alerts-cdn.yml`
  stehen unge-dispatcht; die `.sky1`/`.bin`-Assets manifestieren erst mit dem
  `--ci-mode`-Lauf (zusammen mit dem UnnObs-Job oben).

## Abschluss

- Baum nicht ruhig: fremde Sessions arbeiten weiter (src/archivar/* — hdf5, las, netcdf,
  mod, hsd/mat5/nexrad; tools/measure/mww.rs). Diese Session committet nur eigene
  Hunks; Push wartet auf ruhigen Baum + Wort.
