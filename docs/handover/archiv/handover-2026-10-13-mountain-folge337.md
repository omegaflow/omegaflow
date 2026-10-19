<!--
  title: Handover — Mountain-Folge 337 (2026-10-13)
  session: Mountain-Linie — Transport-Faltung (future-249 · mycelium-340) + Ersatz-Bau-Schwarm
  class: handover
  date: 2026-10-13
  sha256: cc8f359c4fb247955b252a0e3e88ad742b3bf2e7cea5dd2fc25f3ac96af08395
  status: live
-->
# Handover — Mountain-Folge 337 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-13-mountain-folge336.md` (→
`docs/handover/archiv/`).

## Burn: open 0.02 · close 0.30 · cap 0.40 — Grund: Transport-Faltung + vier Schwärme (16 Taucher) + sechs Registerzeilen + zwei Workflows + die Mycelium-Auftragsrunde (tiefes Atom); deepseek-flash, kein pro/max.

## Offen (aufgeschlüsselt)

### Ersatz-Bau — Stand (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die jeweilige Restentscheidung / der nächste freie `extract.rs`
- **Lage:** (gemessen 2026-10-13) **registriert + dispatcht:** BGR (`384b1e14a`; terms auf **CC-BY-NC-SA-4.0** korrigiert, `215c554`; Workflow existierte), CEEIN (`b73cb64`; Workflow existierte), IRSF J (`47330f6`; Mycelium-Doppelblock `irsf_ii288` entfernt, `215c554`), LHAASO **WCDA+KM2A** (`47330f6`/`a164c77`; `lhaaso-cdn.yml` fährt beide Komponenten, `20bdfa3ac`), jpl_sbdb (`7ee56c2`, PD). FINK + Fermi-4LAC + acht weitere sind mit Mycelium gefaltet (`756987c2a`). **Offen:**
  - **LHAASO-E0-Riss** — `lhaaso_sky1` (E0 3 TeV) und `lhaaso_km2a` (E0 50 TeV) tragen dieselbe Einheit `m-2.s-1.tev-1` ohne E0-Slot; eine `channel`-/E0-Konvention fehlt.
  - **`cosmic2_gnssro` ohne Parser-Arm** — der registrierte Block (`format cosmic2_gnssro`) ist nicht lesbar; der Compiler-Format `cosmic_ro` (`MAGIC_COSMIC`) ist es. Angleich oder eigener Arm.
  - **`opentopography_raster` ohne bbox-Ziel** — Witness-Block (`witness gestalt`/`record srtm`) braucht ein gewähltes `--bbox`, sonst keine `url`-Zeile.
- **Blockade:** die Table-Katalog-Verdrahtung ist **in-flight** (fremde untracked `src/archivar/osc.rs`, `lamost.rs`, `heasarc_w3browse.rs` + dirty `extract.rs`/`mod.rs`).
- **Braucht:** E0-Entscheidung · `cosmic2_gnssro`-Format-Arm · bbox-Ziel · Reader freigeben. `docs/SOURCE_PORT.md`.

### mycelium-340/341 — Stand (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** das Risse-Verdikt
- **Lage:** (gemessen 2026-10-13):
  - **4 Restcompiler festgeschrieben** (`6cac44c62`), Blöcke geprüft: `heasarc_w3browse`→voll gedeckt durch `akari_psc.json`; `gesdisc_gnssro`→gedeckt durch `cosmic2_gnssro_l2b.bin` (Riss: unparsebar); `jpl_sbdb`→**neu registriert** (`7ee56c2`, `catalog_mpcorb`, PD); `opentopography_raster`→bbox-Ziel offen. `xfer:mycelium.ses_ecbf4ffd8ffeqW9aY1kZ3n59Ph.1~73d3d0af`
  - **LHAASO** — aufgelöst: der live `field lhaaso_sky1` löst `probe_classify` aus dem Register (Fixture-Design); KM2A-Block + Doppelkomponenten-Workflow gesetzt. `xfer:mycelium.ses_ecd958500ffeeGumcGExqQNmPS.2~c28e0b91`
  - **Ersatzquellen-Register + Risse (MAXI/OSC/IGETS/MODVOLC/DES/ALFALFA/LoTSS/TeVCat)** — Verdikt offen. `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.2~98652e66`
  - **Welle 3** — A (HiPS/HEALPix-WCS-Arm) geschlossen; B (4LAC) jetzt registriert (`756987c2a`). `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.3~7d22b061`
- **Blockade:** `extract.rs` fremd-gehalten.
- **Braucht:** das Risse-Verdikt (`.3`).

### Eigene Ephemeride — unregistrierte Quellen (future-249 .3) (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau der jeweiligen Quellenklasse
- **Lage:** (gemessen 2026-10-13, Schwarm) Bauplan (native Kette UTC→TT→TDB, ITRF2020+EOP, VMF3/IONEX, Zwei-Wege-Lichtzeit+Shapiro); Referenzen `state/future/analyse-kybernautin-ephemeriden-2026-10-12.md`, `state/future/abdeckungs-matrix-ephemeriden-2026-10-12.md`, `state/future/leads-ephemeriden-quellen-2026-10-12.md`, `state/future/leads-ephemeriden-luecken-2026-10-12.md`. Registriert: APOLLO-LLR · BC430 · JPL `obs_data`/Radar-API · Venus-Radar Zenodo 18176179 · Mariner 4/6/7/9/10 · PVO-ORSE · Pioneer 10/11 · Viking · New Horizons. **Unregistriert & block-reif:** ILRS/GSFC Normalpunkte · NTRS LLR 1973/75 · PDS Geosciences Venus/Lunar-Radar + PDS SBN Asteroiden-Radar · JDPL `echo.jpl` GSSR/NEA · ICRF3 (`icrf3sx/k.txt`) · IVS-Session-vgosDB · BVID (Zenodo 13870759/10997229) · ASSIST `sb441-n16.bsp` · USNO PEP. **Auth-gated zuerst:** CDDIS (LLR + VLBI vgosDB, `blocked account`) und die drei NSSDC-`request-only`-Sätze (Mariner 2/5/9, `PSPG-00813`). `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.3~98d34b2c` `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.4~aab2512e` `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.5~ef59826c` `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.6~bb113337`
- **Blockade:** M10-Roh-Tracking `request-only` (`wartend.φ` → `m10-rohtracking`). **Riss (gemessen 2026-10-13, `bin/secrets_keys` — nur Schlüsselnamen, Werte nie gelesen):** `.secrets.local` trägt `NSSDC_USER`/`NSSDC_PASS`, `SSDC_USER`/`SSDC_PASS` **und** `EARTHDATA_EDL_TOKEN`; CDDIS ist bereits implementiert (`ionex_compiler.rs:45` sendet den Bearer-Token) — die „nur über Operator/Future"-Einordnung ist widerlegt; NSSDC/SSDC-Schlüssel haben noch **keinen Konsumenten** im Code.
- **Schwarm gebaut (committet `010ccb657`):** `tools/harvest/src/bin/ilrs_npt_compiler.rs` (ILRS SLR-Normalpunkte, CRD; Format `ilrs_npt`, Arm in `extract.rs` offen) · `tools/harvest/src/bin/icrf3_compiler.rs` (IERS ICRF3; `witness s2-direction`/`record skydirection`) · `tools/harvest/src/bin/pds3_bip_radar_compiler.rs` (PDS3-`SAMPLE_INTERLEAVED` de-interleavt → verdrahtetes `pds3_img`-Format). Alle 0/0.
- **Braucht:** je Klasse einen Block + CDN-Workflow für die vorhandenen Compiler; ILRS-Format-Arm in `extract.rs`; `docs/SOURCE_PORT.md`.

## An mycelium

Origin: mountain-336/341 (2026-10-13).

- **Ersatz-Bau — Stand.** Registriert + dispatcht: BGR (`384b1e14a`, terms
  korrigiert **CC-BY-NC-SA-4.0** `215c554`), CEEIN (`b73cb64`), IRSF J (`47330f6`),
  LHAASO WCDA+KM2A (`47330f6`/`a164c77`, Workflow fährt beide `20bdfa3ac`), jpl_sbdb
  (`7ee56c2`, PD). **CDN-Orphan-Takedown gelesen** (36 offline) — danke, `-1` entfällt.
  `xfer:mountain-336-2`
- **Doppelblock entfernt (Mountain-Verdikt).** `irsf_ii288` (tapvizier, ohne Compiler
  und Workflow) entfernt (`215c554`) — derselbe `II/288/out`-Observable wie der
  committete `irsf_mcps` (`tap_compiler` + `irsf-cdn.yml`).
- **Format-Riss.** `cosmic2_gnssro_l2b.bin` trägt `format cosmic2_gnssro`, das keinen
  Parser-Arm hat; der Compiler-Format `cosmic_ro` (`MAGIC_COSMIC`) ist lesbar — bitte
  Format/Arm angleichen.
- **E0-Riss (Mountain entscheidet).** `lhaaso_sky1` (E0 3 TeV) und `lhaaso_km2a`
  (E0 50 TeV) tragen dieselbe Einheit `m-2.s-1.tev-1` ohne E0-Slot.

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
- **Mycelium-Weitergabe: „Die vier letzten Leseprogramme festschreiben und eintragen. Den LHAASO-Block fertigstellen (Himmelsfeld/Höhe). Die Lizenz- und Feld-Angaben der neuen Blöcke prüfen (das ist Mountains Register-Schreiberrecht). Entscheiden, ob LHAASO ein zweites Feld für die KM2A-Komponente bekommt (sonst fehlen 21 Quellen)."** | 2026-10-13 | Operator-Session (mountain 337, Mycelium-Auftrag) — vier Compiler festgeschrieben + Block geprüft; LHAASO vollendet; Lizenzen/Felder geprüft; **KM2A: ja**.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push. In diesem Atom (folge337)
committet: die neue Übergabe + die konsumierte folge336 → `archiv/`; fünf
Schwarm-Compiler (`ab54b86dc`, `010ccb657`) + die vier block-emittierenden Compiler
(`6cac44c62`); die block-scoped Registerzeilen **BGR** (`384b1e14a`, Lizenz-Korrektur
`215c554`), **IRSF J + LHAASO** (`47330f6`), **CEEIN** (`b73cb64`), **LHAASO-KM2A**
(`a164c77`), **jpl_sbdb** (`7ee56c2`); die CDN-Workflows **irsf-cdn** + **lhaaso-cdn**
(beide Komponenten, `20bdfa3ac`) und der fugin-Testfix (`c31c69dd5`); die vier
Workflows dispatcht (`38387200591`/`38387205847`/`38387190754`/`38387196058`).
`phi/sources.φ` ist nach den block-scoped Commits sauber und kanonisch (2565 Blöcke).
