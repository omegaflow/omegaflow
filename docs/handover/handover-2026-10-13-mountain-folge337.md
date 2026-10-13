<!--
  title: Handover — Mountain-Folge 337 (2026-10-13)
  session: Mountain-Linie — Transport-Faltung (future-249 · mycelium-340) + Ersatz-Bau-Nachmessung
  class: handover
  date: 2026-10-13
  sha256: f0323f125efe0a69adcfdc792bce30ce907876f3550a5fb81a39d25c06bd31ce
  status: live
-->
# Handover — Mountain-Folge 337 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-13-mountain-folge336.md` (→
`docs/handover/archiv/`).

## Burn: open 0.02 · close 0.05 — Grund: Transport-Faltung (2 Blöcke, 12 Einträge) + Ersatz-Bau-Nachmessung; deepseek-flash, kein pro/max.

## Offen (aufgeschlüsselt)

### Ersatz-Bau — Rest ohne registrierten Block (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau der jeweiligen Rest-Blockzeile
- **Lage:** (gemessen 2026-10-13 via `sgrep`/`register_sort`) die Ersatzquellen der entfernten unlizenzierten Blöcke sind **größtenteils registriert**: JADES (`zenodo.org/records/7460116`) · MAGIC (`13898269`) · ALFALFA→HI4PI (`5500447`) · ALeRCE (`11123894`) · Pantheon+ (`16365279`) · CEERS (`15472354`) · ATLAS/`tap.roe` (`15108911`) · IGETS (`dataservices.gfz-potsdam.de/igets_l1.bin`) · Argo (`data-argo.ifremer.fr`) · `jvo`→HEASARC `akaripsc` · FUGIN→CfA-CO (`zenodo.org/records/5501196`). **Rest ohne Block** (`sgrep` = 0): BGR/CEEIN Infraschall (`data.europa.eu` `e8018059`, CC-BY-NC-SA-4.0) · IRSF J (`jvo`, VizieR II/288, `vizier-scientific-free`) · SSDC bzcat5 (Fermi 4LAC-DR3, NASA FSSC PD) · FINK/babamul (Zenodo `4036589`, CC-BY-4.0). Auflösungstabelle: `state/future/licence-alternativen-all-2026-10-12.md`.
- **Blockade:** je Rest fehlt Compiler/Format-Arm; die Observablen sind teils nicht identisch.
- **Braucht:** je Rest Route messen (`archive_search --verdict`/`--tap`/`--dataeuropa`) → Compiler + Blockzeile (`url`/`terms`/`origin`/`compiler`/`at`/`field`); `docs/SOURCE_PORT.md`.

### mycelium-340 — Register/CI-Aufgaben (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der jeweilige Bau
- **Lage:** (gemessen 2026-10-13, gefaltet aus `## An mountain` mycelium-folge340):
  - Format-Namen der 4 Restcompiler `heasarc_w3browse`/`jpl_sbdb`/`gesdisc_gnssro`/`opentopography_raster` — kein Register-Block trägt sie (`sgrep` = 0); `opentopography_raster` kollidiert mit SRTM15+ (`phi:29896`). `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af`
  - LHAASO/VizieR-Block (`phi:28194-28198`): `url`/`format sky1`/`origin`/`compiler` stehen, **`terms`/`at`/`field` fehlen**; kein CI-Workflow ruft `lhaaso_compiler`. `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.2~c28e0b91`
  - Ersatzquellen-Register + Risse (MAXI/OSC/IGETS/MODVOLC/DES/ALFALFA/LoTSS/TeVCat) — Verdikt offen. `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.2~98652e66`
  - Welle 3: HiPS-HEALPix-WCS-Arm in `src/archivar/fits.rs` nötig; 4LAC als eigene Quelle oder gestrichen? `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.3~7d22b061`
- **Blockade:** die zwei letzten sind Bau-/Verdikt-Entscheidungen; die zwei ersten Register-/CI-Arbeit.
- **Braucht:** Format-Namen + Blöcke für die 4 Compiler · LHAASO: N0-Einheit aus VizieR J/ApJS/271/25 messen, dann `terms vizier-scientific-free https://cds.unistra.fr/vizier-org/licences_vizier.html` + `at`/`field` + CI-Workflow · Risse-Verdikt.

### Eigene Ephemeride + Mariner-Rohdaten (future-249 .3–.6) (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau der jeweiligen Quellenklasse
- **Lage:** (gemessen 2026-10-13, gefaltet aus `## An mountain` future-folge249) Bauplan der eigenen Ephemeride aus Primärbeobachtungen (native Kette UTC→TT→TDB, ITRF2020+EOP, VMF3/IONEX, Zwei-Wege-Lichtzeit+Shapiro); Referenzen `state/future/analyse-kybernautin-ephemeriden-2026-10-12.md`, `state/future/abdeckungs-matrix-ephemeriden-2026-10-12.md`, `state/future/leads-ephemeriden-quellen-2026-10-12.md`, `state/future/leads-ephemeriden-luecken-2026-10-12.md`. Registriert: APOLLO-LLR (`llr.bin`, `phi:28991`) · BC430 (`3558850`) · `mariner_rst` (M6/7 Doppler, `phi:31566`) · `mariner4_odg` (`phi:31558`) · `mariner_m10_edr` (Bilder, `phi:31550`) · `mariner_occlt` (`phi:20413`) · `mariner_m10_camera_angles.txt` (`phi:31602`). **Korrektur (future-249 .4):** Mountain's „kein Mariner-Roh-Track" gilt **nur für Mariner 10**. Offen bleiben: die drei fehlenden Quellenklassen (LLR/Radar/VLBI/Astrometrie · Mariner-2/5/9 · Kreuz-Zeugen); M10-Roh-Tracking (762 7-Spur-Tapes `MVM_1001–1007`, NSSDC `PSCM-00009`) = `request-only`/offline (Anfrage 2026-09-16 ohne Antwort); SEDR-Kamera-Zeuge `PSPA-00273`/`-00055`/`-00318` offline. `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.3~98d34b2c` `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.4~aab2512e` `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.5~ef59826c` `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.6~bb113337`
- **Blockade:** DSN-Delta-DOR autorisiert-server-gebunden; M10-Tapes + SEDR offline.
- **Braucht:** Leads Quelle für Quelle in Blöcke/Compiler (`docs/SOURCE_PORT.md`); CDDIS ist kein Key-Gap (`EARTHDATA_EDL_TOKEN` → HTTP 200); M10-Anfrage bei NSSDC/JPL = Operator-Hand, wartend (`state/zustand/wartend.φ` → `m10-rohtracking`).

## An mycelium

Origin: mountain-336 (2026-10-13).

- **CDN-Orphan-Takedown (der Riegel löscht keine Bestände).** Der Riegel
  (`upload_release`/`registered_netlocs`, `src/archivar/cdn.rs`) verweigert jeden
  Upload eines Tags ohne anerkannte Lizenz — **neue** unlizenzierte Uploads sind
  ausgeschlossen. Die **Bestände** der entfernten Quellen stehen weiter auf
  `omegaflow/sources`: bitte die Orphan-Releases offline nehmen (`gh release delete
  <tag> --repo omegaflow/sources --yes`); volle Liste im abgelösten folge336-Punkt.
  `xfer:mountain-336-1`
- **Ersatz-Bau der offenen Alternativen.** Die entfernten unlizenzierten Quellen
  tragen je eine gemessene offene Alternative; Mountain baut die Blöcke, Mycelium
  manifestiert sie (nach dem Riegel). Rest ohne Block: BGR/CEEIN (`e8018059`),
  IRSF J (VizieR II/288), SSDC bzcat5 (Fermi 4LAC-DR3), FINK (`4036589`).
  `xfer:mountain-336-2`

## Operator-Wort-Register

- **„Projekt-Ownership statt statischer Verantwortlichkeiten — jeder schiebt den schwarzen Peter … ihr seid alle gleich ausgebildet"** | 2026-10-13 | Operator-Session (mountain 336) — kein Ball-Zuschieben; jede Linie führt die Projekt-Arbeit durch.
- **„warum haben wir immer noch no-license-stated und fragwürdige lizenzen auf cdn?"** | 2026-10-13 | Operator-Session (mountain 336) — Riegel + Auflösung der 87; `no-license-stated` = 0.
- **„bitte prüfe was zu viel entfernt wurde"** | 2026-10-13 | Operator-Session (mountain 336) — 8 zu viel entfernte Blöcke wiederhergestellt.
- **„und haben wir jetzt für alle offene alternativen?"** | 2026-10-13 | Operator-Session (mountain 336) — teils; der Ersatz-Bau ist offen.
- **„liegen jetzt nur noch einwandfreie Assets auf CDN?"** | 2026-10-13 | Operator-Session (mountain 336) — nein; Orphan-Bestände, Takedown an Mycelium.
- **„das muss eindeutig recherchiert und an mycelium gegeben werden"** | 2026-10-13 | Operator-Session (mountain 336) — `## An mycelium` trägt Orphans + Alternativen.
- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes."** | 2026-10-13 | Operator-Session (mountain 337) — Linien-Start = session-weiter Consent (Delegation), getrennt vom Commit-Wort; kein neues Sachwort.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push. In diesem Atom (folge337)
committet (pfad-begrenzt): die neue Übergabe + die konsumierte folge336 →
`archiv/`. `phi/sources.φ` bleibt unangetastet — der Arbeitsbaum trägt dort einen
fremden uncommitteten Hunk (eine Leerzeile, `git diff`); die Ersatz-Blöcke warten
auf Compiler/Route-Verifikation.
