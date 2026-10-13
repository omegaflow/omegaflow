<!--
  title: Handover — Mountain-Folge 337 (2026-10-13)
  session: Mountain-Linie — Transport-Faltung (future-249 · mycelium-340) + Ersatz-Bau-Schwarm
  class: handover
  date: 2026-10-13
  sha256: 33cda668df0685e0045d7570206653cc2c71b8adf3899e1ab07bb143a04dd29a
  status: live
-->
# Handover — Mountain-Folge 337 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-13-mountain-folge336.md` (→
`docs/handover/archiv/`).

## Burn: open 0.02 · close 0.11 — Grund: Transport-Faltung (12 Einträge) + Ersatz-Bau-Schwarm (8 Taucher); deepseek-flash, kein pro/max.

## Offen (aufgeschlüsselt)

### Ersatz-Bau — Schwarm-Befunde, Blockzeilen bereit (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Falten der jeweiligen Blockzeile
- **Lage:** (gemessen 2026-10-13, Schwarm) die Ersatzquellen sind großenteils registriert; für die vier Reste liegt die Messung:
  - **BGR/CEEIN Infraschall** — die Compiler + Formate **existieren schon** (`tools/harvest/src/bin/bgr_infrasound_compiler.rs` und `tools/harvest/src/bin/ceein_infrasound_compiler.rs`; verdrahtet `src/archivar/geo.rs:309/315`, `extract.rs:950-954/977-978`, `port.rs:4088/4146`). Lizenz **gemessen CC-BY-4.0** (die Auflösungstabelle nennt NC-SA — Riss, die Datei spricht). Route `https://download.bgr.de/bgr/geophysik/Infrasound_hf_product/netCDF/BGR_infrasound_hf_product.zip` (HTTP 206, 1,53 GB, 994 `.nc`). Nur **Block + CDN-Manifestation** fehlen. `field … acoustic deg|m/s|pa|hz 300 0.0 0.0`, `no-cadence`.
  - **IRSF J (VizieR II/288)** — `tap_compiler.rs` genügt (kein neues File); die Tabelle heißt **`II/288/out`** (nicht `/irsf`), 18 015 012 Zeilen, Spalten `RAJ2000/DEJ2000/Jmag`; `terms vizier-scientific-free`.
  - **SSDC bzcat5 → Fermi 4LAC-DR3** — Route `https://fermi.gsfc.nasa.gov/ssc/data/access/lat/4LACDR3/table_4LAC.fits` (HTTP 206, PD, 2863 AGN); Compiler `tools/harvest/src/bin/fermi_4lac_dr3_compiler.rs` (committet `ab54b86dc`, baut 0/0), aber Format `fermi_4lac_dr3` (magic `F4LC`) ist im Reader **nicht verdrahtet**.
  - **FINK (Zenodo 4036589)** — `year=2019.zip` (5,6 GB, CC-BY-4.0, ZIP+Snappy-Parquet); Compiler `tools/harvest/src/bin/zip_parquet_compiler.rs` (committet `ab54b86dc`, baut 0/0); Feld/cmap ausstehen.
- **Blockade:** BGR/CEEIN + IRSF J sind falt-reif; 4LAC/FINK brauchen die Reader-Verdrahtung in `src/archivar/extract.rs` (fremd-gehalten, dirty).
- **Braucht:** BGR + IRSF J als Blockzeilen falten (block-scoped: `block_commit --path phi/sources.φ --key <url>`) + CDN-Workflow; 4LAC/FINK: Format-Arm in `extract.rs`/`geo.rs` + Block + Workflow. `docs/SOURCE_PORT.md`.

### mycelium-340 — Schwarm-Befunde (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Setzen der jeweiligen Zeilen
- **Lage:** (gemessen 2026-10-13, Schwarm):
  - **4 Restcompiler** — der Baum korrigiert drei der vier Namen: `heasarc_w3browse` (format `heasarc_w3browse`, `at earth`, ttl 604800) · `jpl_sbdb` (format **`catalog_mpcorb`**, `at sun`, ttl 86400) · `gesdisc_gnssro` (format **`cosmic_ro`**, `at earth`, ttl 604800 — überlappt das bestehende `cosmic2_gnssro`, `phi:10215`) · `opentopography_raster` (kein `format` — Witness `witness gestalt`/`record srtm`, kollidiert mit dem SRTM15+-Block). `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af`
  - **LHAASO** — `terms vizier-scientific-free https://cds.unistra.fr/vizier-org/licences_vizier.html` · `at sun` · `field lhaaso_sky1 lhaaso_sky1 inverse-square em m-2.s-1.tev-1 31536000 0.0 0.0`. **Riss:** `N0` ist die Power-law-Normierung (differentieller Fluss), einheiten- **und** E0-komponentenabhängig (WCDA 3 TeV/1e-13, KM2A 50 TeV/1e-16) — der Compiler wirft beide in eine Spalte; `port.rs:2554` widerspricht dem Test `:4171` (`em`/`1` vs `em`/`m-2.s-1.tev-1`); ein `lhaaso-cdn.yml` fehlt (kein Workflow ruft `lhaaso_compiler`; Vorlage `hawc-cdn.yml`). `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.2~c28e0b91`
  - **Ersatzquellen-Register + Risse (MAXI/OSC/IGETS/MODVOLC/DES/ALFALFA/LoTSS/TeVCat)** — Verdikt offen. `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.2~98652e66`
  - **Welle 3** — (A) der HiPS/HEALPix-WCS-Arm **existiert** (`src/archivar/fits.rs:361-565`, `src/mathematikerin/healpix.rs`); die Schranke ist mit der Messung geschlossen; offen bleibt allein ein HiPS-**Kachel-Föderator** (nicht gefordert). (B) Fermi-4FGL ist registriert (`phi:14583`, PD), 4LAC fehlt → eigene Quelle (kein Duplikat). `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.3~7d22b061`
- **Blockade:** N0-Komponenten-Entscheidung (Compiler um `Comp`/E0 erweitern oder WCDA-only); `extract.rs` fremd-gehalten.
- **Braucht:** die drei Blockzeilen (4 Compiler) setzen; LHAASO: `port.rs`-Widerspruch auflösen, `lhaaso-cdn.yml` bauen; Risse-Verdikt.

### Eigene Ephemeride — unregistrierte Quellen (future-249 .3) (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau der jeweiligen Quellenklasse
- **Lage:** (gemessen 2026-10-13, Schwarm) Bauplan (native Kette UTC→TT→TDB, ITRF2020+EOP, VMF3/IONEX, Zwei-Wege-Lichtzeit+Shapiro); Referenzen `state/future/analyse-kybernautin-ephemeriden-2026-10-12.md`, `state/future/abdeckungs-matrix-ephemeriden-2026-10-12.md`, `state/future/leads-ephemeriden-quellen-2026-10-12.md`, `state/future/leads-ephemeriden-luecken-2026-10-12.md`. Registriert: APOLLO-LLR · BC430 · JPL `obs_data`/Radar-API · Venus-Radar Zenodo 18176179 · Mariner 4/6/7/9/10 · PVO-ORSE · Pioneer 10/11 · Viking · New Horizons. **Unregistriert & block-reif:** ILRS/GSFC Normalpunkte · NTRS LLR 1973/75 · PDS Geosciences Venus/Lunar-Radar + PDS SBN Asteroiden-Radar · JDPL `echo.jpl` GSSR/NEA · ICRF3 (`icrf3sx/k.txt`) · IVS-Session-vgosDB · BVID (Zenodo 13870759/10997229) · ASSIST `sb441-n16.bsp` · USNO PEP. **Auth-gated zuerst:** CDDIS (LLR + VLBI vgosDB, `blocked account`) und die drei NSSDC-`request-only`-Sätze (Mariner 2/5/9, `PSPG-00813`). `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.3~98d34b2c` `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.4~aab2512e` `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.5~ef59826c` `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.6~bb113337`
- **Blockade:** CDDIS/NSSDC brauchen Zugang (Operator/Future); M10-Roh-Tracking `request-only` (`wartend.φ` → `m10-rohtracking`).
- **Braucht:** je Klasse einen Block + Compiler (Text-`/pds`/`odf`/API-Reader); `docs/SOURCE_PORT.md`.

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
  manifestiert sie (nach dem Riegel). Rest: BGR/CEEIN (Compiler da, CC-BY-4.0),
  IRSF J (VizieR II/288), SSDC bzcat5 → Fermi 4LAC-DR3, FINK (`4036589`).
  `xfer:mountain-336-2`

## Operator-Wort-Register

- **„Projekt-Ownership statt statischer Verantwortlichkeiten — jeder schiebt den schwarzen Peter … ihr seid alle gleich ausgebildet"** | 2026-10-13 | Operator-Session (mountain 336) — kein Ball-Zuschieben; jede Linie führt die Projekt-Arbeit durch.
- **„warum haben wir immer noch no-license-stated und fragwürdige lizenzen auf cdn?"** | 2026-10-13 | Operator-Session (mountain 336) — Riegel + Auflösung der 87; `no-license-stated` = 0.
- **„bitte prüfe was zu viel entfernt wurde"** | 2026-10-13 | Operator-Session (mountain 336) — 8 zu viel entfernte Blöcke wiederhergestellt.
- **„und haben wir jetzt für alle offene alternativen?"** | 2026-10-13 | Operator-Session (mountain 336) — teils.
- **„liegen jetzt nur noch einwandfreie Assets auf CDN?"** | 2026-10-13 | Operator-Session (mountain 336) — nein; Orphan-Bestände, Takedown an Mycelium.
- **„das muss eindeutig recherchiert und an mycelium gegeben werden"** | 2026-10-13 | Operator-Session (mountain 336).
- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater … kein Consent-Stopp für Bekanntes."** | 2026-10-13 | Operator-Session (mountain 337) — Linien-Start = session-weiter Consent (Delegation), getrennt vom Commit-Wort.
- **„du hast nur gemessen und registriert … bitte delegiere an einen taucher schwarm"** | 2026-10-13 | Operator-Session (mountain 337) — der Ersatz-Bau wird per Schwarm gearbeitet, nicht nur registriert.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push. In diesem Atom (folge337)
committet (pfad-begrenzt): die neue Übergabe + die konsumierte folge336 →
`archiv/`; die zwei Schwarm-Compiler-Entwürfe (`fermi_4lac_dr3_compiler.rs`,
`zip_parquet_compiler.rs`, `ab54b86dc`). `phi/sources.φ` bleibt unangetastet — der
Arbeitsbaum trägt dort einen fremden uncommitteten Hunk; die Block-Faltung wartet
auf `block_commit`, die 4LAC/FINK-Verdrahtung auf `extract.rs`/`geo.rs`.
