<!--
  title: Handover — Mountain-Folge 337 (2026-10-13)
  session: Mountain-Linie — Transport-Faltung (future-249 · mycelium-340) + Ersatz-Bau-Schwarm
  class: handover
  date: 2026-10-13
  sha256: 7dd984c1b2c2c423854e8604896961679772c0ee6f47387ad8cf7ef309dc5998
  status: live
-->
# Handover — Mountain-Folge 337 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-13-mountain-folge336.md` (→
`docs/handover/archiv/`).

## Burn: open 0.02 · close 0.22 · cap 0.30 — Grund: Transport-Faltung (12 Einträge) + drei Schwärme (15 Taucher) + vier Block-Faltungen + zwei Workflows (tiefes Atom); deepseek-flash, kein pro/max.

## Offen (aufgeschlüsselt)

### Ersatz-Bau — Restschritt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der jeweilige Bau / der nächste freie `extract.rs`
- **Lage:** (gemessen 2026-10-13) **registriert + Workflow vorhanden + dispatcht:** BGR (`384b1e14a`; `bgr-infrasound-cdn.yml` existierte schon), CEEIN (`b73cb64`; `ceein-infrasound-cdn.yml` existierte), IRSF J (VizieR `II/288/out`, `irsf-cdn.yml` neu), LHAASO (`sky1`, `terms`/`at sun`/`field`, `lhaaso-cdn.yml` neu) — Läufe `38387200591` · `38387205847` · `38387190754` · `38387196058` (Ergebnis nicht gelesen; kein Polling). **Offen:**
  - **SSDC bzcat5 → Fermi 4LAC-DR3** — Compiler `fermi_4lac_dr3_compiler.rs` (`ab54b86dc`); Reader fehlt (neues Modul `fermi_4lac` + fünf Arme): `src/archivar/mod.rs` + `extract.rs::verify_records` (nach `:224`) + `extract.rs::extract_raw` (nach `:4198`) + `main_flow.rs:5958-5966`.
  - **FINK (Zenodo 4036589)** — Compiler `zip_parquet_compiler.rs` (`ab54b86dc`), Ausgabe = Table-Format; Schema gemessen (`candidate.ra/dec/magpsf/sigmapsf/magap/sigmagap`, kein `finkclass`); `record` parse-inert → `cmap`/`field`-Block. Reader analog.
  - **LHAASO-N0-Riss** — der Compiler schreibt rohes `N0` (WCDA 3 TeV/1e-13 **und** KM2A 50 TeV/1e-16 in einer Spalte); der Block deklariert `m-2.s-1.tev-1`. Compiler um `Comp`/`E0`/Skalierung erweitern (oder WCDA-only).
- **Blockade:** die Table-Katalog-Verdrahtung ist **in-flight** (fremde untracked `src/archivar/osc.rs`, `lamost.rs`, `heasarc_w3browse.rs` + dirty `extract.rs`/`mod.rs`).
- **Braucht:** 4LAC/FINK-Reader, sobald `extract.rs`/`mod.rs` frei sind; LHAASO-Compiler-Fix. `docs/SOURCE_PORT.md`.

### mycelium-340 — Stand (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Setzen der jeweiligen Zeilen
- **Lage:** (gemessen 2026-10-13):
  - **4 Restcompiler** — der Baum korrigiert drei der vier Namen: `heasarc_w3browse` (format `heasarc_w3browse`, `at earth`, 604800) · `jpl_sbdb` (format **`catalog_mpcorb`**, `at sun`, 86400) · `gesdisc_gnssro` (format **`cosmic_ro`**, `at earth`, 604800; überlappt `cosmic2_gnssro`) · `opentopography_raster` (kein `format` — Witness `witness gestalt`/`record srtm`). `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af`
  - **LHAASO** — Block + Workflow gesetzt; der `port.rs`/Test-Widerspruch ist **aufgelöst**: der live `field lhaaso_sky1 … m-2.s-1.tev-1` löst `probe_classify` aus dem Register (Fixture-Design); der Keyword-Arm bleibt für neue Namen. `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.2~c28e0b91`
  - **Ersatzquellen-Register + Risse (MAXI/OSC/IGETS/MODVOLC/DES/ALFALFA/LoTSS/TeVCat)** — Verdikt offen. `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.2~98652e66`
  - **Welle 3** — (A) HiPS/HEALPix-WCS-Arm existiert (`src/archivar/fits.rs:361-565`); Schranke geschlossen. (B) 4FGL registriert, 4LAC fehlt → eigene Quelle. `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.3~7d22b061`
- **Blockade:** `extract.rs` fremd-gehalten.
- **Braucht:** die Blockzeilen `heasarc_w3browse` + `jpl_sbdb` setzen (Register-Arbeit); Risse-Verdikt.

### Eigene Ephemeride — unregistrierte Quellen (future-249 .3) (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau der jeweiligen Quellenklasse
- **Lage:** (gemessen 2026-10-13, Schwarm) Bauplan (native Kette UTC→TT→TDB, ITRF2020+EOP, VMF3/IONEX, Zwei-Wege-Lichtzeit+Shapiro); Referenzen `state/future/analyse-kybernautin-ephemeriden-2026-10-12.md`, `state/future/abdeckungs-matrix-ephemeriden-2026-10-12.md`, `state/future/leads-ephemeriden-quellen-2026-10-12.md`, `state/future/leads-ephemeriden-luecken-2026-10-12.md`. Registriert: APOLLO-LLR · BC430 · JPL `obs_data`/Radar-API · Venus-Radar Zenodo 18176179 · Mariner 4/6/7/9/10 · PVO-ORSE · Pioneer 10/11 · Viking · New Horizons. **Unregistriert & block-reif:** ILRS/GSFC Normalpunkte · NTRS LLR 1973/75 · PDS Geosciences Venus/Lunar-Radar + PDS SBN Asteroiden-Radar · JDPL `echo.jpl` GSSR/NEA · ICRF3 (`icrf3sx/k.txt`) · IVS-Session-vgosDB · BVID (Zenodo 13870759/10997229) · ASSIST `sb441-n16.bsp` · USNO PEP. **Auth-gated zuerst:** CDDIS (LLR + VLBI vgosDB, `blocked account`) und die drei NSSDC-`request-only`-Sätze (Mariner 2/5/9, `PSPG-00813`). `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.3~98d34b2c` `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.4~aab2512e` `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.5~ef59826c` `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.6~bb113337`
- **Blockade:** CDDIS braucht den Earthdata-Login (Operator/Future); M10-Roh-Tracking `request-only` (`wartend.φ` → `m10-rohtracking`). **Riss/Neu (gemessen 2026-10-13 via `bin/secrets_keys`, nur Schlüsselnamen):** `.secrets.local` trägt `NSSDC_USER`/`NSSDC_PASS` **und** `SSDC_USER`/`SSDC_PASS` — die „request-only"-Annahme ist damit nicht mehr gedeckt; mit den hinterlegten Schlüsseln ist der Zugang messbar (Werte nie gelesen).
- **Schwarm gebaut (committet `010ccb657`):** `tools/harvest/src/bin/ilrs_npt_compiler.rs` (ILRS SLR-Normalpunkte, CRD; Format `ilrs_npt`, Arm in `extract.rs` offen) · `tools/harvest/src/bin/icrf3_compiler.rs` (IERS ICRF3; `witness s2-direction`/`record skydirection`) · `tools/harvest/src/bin/pds3_bip_radar_compiler.rs` (PDS3-`SAMPLE_INTERLEAVED` de-interleavt → verdrahtetes `pds3_img`-Format). Alle 0/0.
- **Braucht:** je Klasse einen Block + CDN-Workflow für die vorhandenen Compiler; ILRS-Format-Arm in `extract.rs`; `docs/SOURCE_PORT.md`.

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
  manifestiert sie (nach dem Riegel). BGR ist registriert (`384b1e14a`); Rest:
  CEEIN (Compiler da, Lizenz offen), IRSF J (VizieR II/288), SSDC bzcat5 → Fermi
  4LAC-DR3, FINK (`4036589`). `xfer:mountain-336-2`

## Operator-Wort-Register

- **„Projekt-Ownership statt statischer Verantwortlichkeiten — jeder schiebt den schwarzen Peter … ihr seid alle gleich ausgebildet"** | 2026-10-13 | Operator-Session (mountain 336) — kein Ball-Zuschieben; jede Linie führt die Projekt-Arbeit durch.
- **„warum haben wir immer noch no-license-stated und fragwürdige lizenzen auf cdn?"** | 2026-10-13 | Operator-Session (mountain 336) — Riegel + Auflösung der 87; `no-license-stated` = 0.
- **„bitte prüfe was zu viel entfernt wurde"** | 2026-10-13 | Operator-Session (mountain 336) — 8 zu viel entfernte Blöcke wiederhergestellt.
- **„und haben wir jetzt für alle offene alternativen?"** | 2026-10-13 | Operator-Session (mountain 336) — teils.
- **„liegen jetzt nur noch einwandfreie Assets auf CDN?"** | 2026-10-13 | Operator-Session (mountain 336) — nein; Orphan-Bestände, Takedown an Mycelium.
- **„das muss eindeutig recherchiert und an mycelium gegeben werden"** | 2026-10-13 | Operator-Session (mountain 336).
- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater … kein Consent-Stopp für Bekanntes."** | 2026-10-13 | Operator-Session (mountain 337) — Linien-Start = session-weiter Consent (Delegation), getrennt vom Commit-Wort.
- **„du hast nur gemessen und registriert … bitte delegiere an einen taucher schwarm"** | 2026-10-13 | Operator-Session (mountain 337) — der Ersatz-Bau wird per Schwarm gearbeitet, nicht nur registriert.
- **„was davon kannst du umsetzen und an taucher deligieren?"** | 2026-10-13 | Operator-Session (mountain 337) — das autonom Umsetzbare selbst bauen, den Rest delegieren.
- **„mach das [IRSF/CEEIN-Blöcke + Workflows, LHAASO-Widerspruch] und hast du die secrets local geprüft? Der Zugang zu CDDIS und den NSSDC-request-only-Sätzen: Operator-/Future-Hand."** | 2026-10-13 | Operator-Session (mountain 337) — die Register-Blöcke + Workflows umsetzen, den LHAASO-Widerspruch auflösen, `.secrets.local` prüfen (nur Namen).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push. In diesem Atom (folge337)
committet: die neue Übergabe + die konsumierte folge336 → `archiv/`; fünf
Schwarm-Compiler (`ab54b86dc`, `010ccb657`); die block-scoped Registerzeilen
**BGR** (`384b1e14a`), **IRSF J + LHAASO** (`47330f6`), **CEEIN** (`b73cb64`);
die CDN-Workflows **irsf-cdn** + **lhaaso-cdn** und der fugin-Testfix (`c31c69dd5`);
die vier Workflows dispatcht (`38387200591`/`38387205847`/`38387190754`/`38387196058`).
`phi/sources.φ` ist nach den block-scoped Commits sauber; die 4LAC/FINK-Verdrahtung
wartet auf freies `extract.rs`/`mod.rs`.
