<!--
  title: Handover — Mountain-Folge 338 (2026-10-13)
  session: Mountain-Linie — zwei Register-Risse geschlossen + vier Quellen-Fragen durch Recherche/Rat/Roster geklärt
  class: handover
  date: 2026-10-13
  sha256: 66866cee219f8f1ff0aa51c1b986fff0dc4c14b1b15356e36b8f67b933be7229
  status: live
-->
# Handover — Mountain-Folge 338 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-13-mountain-folge337.md` (→
`docs/handover/archiv/`).

## Burn: open 0.02 · close 0.10 — Grund: zwei Register-Risse geschlossen + vier-Quellen-Klärung durch Recherche (4 Taucher), Rat (5 Stimmen) und UI-Roster (4 Seats, GLM pending); deepseek-flash, kein pro/max.

## Offen (aufgeschlüsselt)

### Ersatz-Bau — offene Reste (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Build-Schritt / der freigegebene Reader
- **Lage:** (gemessen 2026-10-13, `sgrep`/Rat/Roster) registriert sind BGR, CEEIN, IRSF J, LHAASO WCDA+KM2A, jpl_sbdb; die zwei Register-Risse sind geschlossen (committet). **Durch Recherche+Rat+Roster geklärt:**
  - **`opentopography_raster` bbox = KAPU** — gewählt `48.9,49.9,-82.9,-81.7` (Register-Station KAPU `on earth 49.392 -82.320 179.94`, `phi/sources.φ:10300-10301`, an der die River-THEMIS-Kalibrierung läuft). Rat: die Kachel am gemessenen Wert 179.94 m verifizieren. **Auflage (Duck/Claude): Toleranz und Höhenbezug (Geoid vs. Ellipsoid) VOR der Prüfung festlegen**; die zwei globalen SRTM15+-Registrierungen (`:28241`, `:29484`) klären; `load_gestalt_surface_threads` (`main_flow.rs:580`) an einen Produktions-Aufrufer binden, sonst tote Struktur.
  - **Table-Katalog-Verdrahtung = eigene Arbeit, block-scoped committen** — die drei Reader (`src/archivar/osc.rs`, `lamost.rs`, `heasarc_w3browse.rs`) + die Verdrahtungshunks sind gemessen eigenes, ungefalletes folge336 (nicht fremd); nur der `cdf`-Arm und staged `cdn.rs`/`aacgm` sind River. Rat: jetzt über privaten Index block-scoped committen (Präzedenz `c14fb3298`), `cargo check` grün, fremde Hunks unberührt. **Riss (Qwen/Nemotron): block-scoped löst keine gleichzeitige Bearbeitung desselben Blocks** — beim Stagen den Diff prüfen (Claude), die drei Leser zuerst speichern.
- **Blockade:** keine (die Verdrahtung ist eigen, kein Halter).
- **Braucht:** `cargo build -p omegaflow-harvest --bin opentopography_raster_compiler` → `./target/release/opentopography_raster_compiler --bbox 48.9,49.9,-82.9,-81.7 --out data/topex.ucsd.edu/srtm_kapu.bin` → Block falten · dann `git add` nur der drei Reader + eigener `extract.rs`/`mod.rs`-Hunks, `cargo check`, block-scoped commit. `docs/SOURCE_PORT.md`.

### mycelium-340/341 — Risse-Verdikt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die zwei offenen Risse (MODVOLC-Route, ALFALFA-Lizenz)
- **Lage:** (gemessen 2026-10-13, Rat+Roster) **Verdikt der Ersatzquellen:** von MAXI/OSC/IGETS/MODVOLC/DES/ALFALFA/LoTSS/TeVCat sind **sechs gestrichen** (gemessen bereits getragen: MAXI `:14669`, OSC `:3787`, IGETS `:10304`, DES `:14417`+`:14509`, LoTSS `:12819`, TeVCat `:10389` — die Ersatzforderung war ein Duplikat). **Zwei echte Risse bleiben** (Roster-Auflage: Duplikat per Inhalts-Nachweis, unbekannte Lizenz = nicht registrieren):
  - **MODVOLC** — kein Block (`sgrep -i modvolc` = 0); Vorquelle `http://modis.higp.hawaii.edu` = 404 (Route messen); FIRMS `:771`/`:14560` tragen `frp`/`ti4`, nicht thermal → eigener Punkt.
  - **ALFALFA** — kein Block; Vorquelle unregistriert, Lizenz UNKNOWN (`state/future/licence-census-2026-10-12.tsv:522`); HI4PI `:29013` trägt den Kubus, nicht den Katalog → Lizenz klären, sonst `local-only` (keine Manifestation).
  - **Weitere Risse (gemessen):** IERS-EOP-Doppellinie (`:8114-8116` USNO `finals.all` ↔ `:10143-10145` `celestrak_eop`, gleiche Observablen `eop_iers_*`), CHIME/FRB-Cat2 canfar 500, KASI-API 500, IRIS LMSAL 403, MPC-Horizon block-vs-compiler.
- **Blockade:** keine.
- **Braucht:** MODVOLC-Route messen (`archive_search --verdict <url>`), ALFALFA-Lizenz messen (DataCite) → `local-only` oder Block; IERS-Doppellinie: eine Quelle als kanonischen Träger wählen. `docs/SOURCE_PORT.md`.

### Eigene Ephemeride — unregistrierte Quellen (future-249 .3) (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau der jeweiligen Quellenklasse
- **Lage:** (gemessen 2026-10-13) Bauplan native Kette UTC→TT→TDB, ITRF2020+EOP, VMF3/IONEX, Zwei-Wege-Lichtzeit+Shapiro; Referenzen `state/future/analyse-kybernautin-ephemeriden-2026-10-12.md`, `abdeckungs-matrix-…`, `leads-ephemeriden-quellen-…`, `leads-ephemeriden-luecken-…`. Registriert: APOLLO-LLR, BC430, JPL `obs_data`/Radar-API, Venus-Radar Zenodo 18176179, Mariner 4/6/7/9/10, PVO-ORSE, Pioneer 10/11, Viking, New Horizons. **Compiler committet** (`010ccb657`): `ilrs_npt_compiler` (Format `ilrs_npt`, Arm in `extract.rs` offen), `icrf3_compiler` (`witness s2-direction`/`record skydirection`), `pds3_bip_radar_compiler` (Format `pds3_img`); **Blocks + CDN-Workflows fehlen**. `.secrets.local` (nur Schlüsselnamen via `bin/secrets_keys`): `EARTHDATA_EDL_TOKEN` · `NSSDC_USER`/`NSSDC_PASS` · `SSDC_USER`/`SSDC_PASS` vorhanden; CDDIS ist implementiert (`ionex_compiler.rs:45`), die NSSDC-/SSDC-Schlüssel haben noch keinen Konsumenten — die „nur über Operator/Future"-Einordnung ist widerlegt.
- **Verdikt (Rat+Roster 2026-10-13):** (a) **icrf3** — den Compiler auf das Binärformat SKD1 (`write_bin`) umstellen, keinen zweiten JSON-Arm bauen; **Auflage (Claude/Duck): den Vertrag explizit machen (Endianness/Header/Version) und einen Round-Trip gegen den echten Leser `src/archivar/skydirection.rs` fahren, bevor Block/Workflow entstehen**; JSON-Konsumenten inventarisieren. (b) **Bau-Reihenfolge nach Vollständigkeit:** (1) `pds3_img` (Block-Skelett `pds3_bip_radar_compiler.rs:140-151` + Arm `extract.rs:88` da) → Block + `pds3-bip-radar-cdn.yml`; (2) `ilrs_npt` → Lese-Arm in `extract.rs` + Block (Skelett um `terms`/`at`/`ttl`/`field` ergänzen) + Workflow; (3) `icrf3` nach dem SKD1-Entscheid.
- **Blockade:** keine (die Verdrahtung/Arme sind eigen).
- **Braucht:** je Klasse einen Block + CDN-Workflow für die vorhandenen Compiler; ILRS-Format-Arm in `extract.rs`; icrf3 auf SKD1 + Round-Trip. `docs/SOURCE_PORT.md`.

### Register-Force-Unit-Schuld (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste freie Register-/units.rs-Atom
- **Lage:** (gemessen 2026-10-13) `target/{release,debug}/commit_check` blockierte den Register-Commit an 5 Paaren außerhalb der Registry, die die `register_sort`-Neuschreibung `75bd350829` in HEAD trägt: `diffusion kg/m2/s` (`phi/sources.φ:1417`/`:14740`), `diffusion μmol/kg` (`:1735`; die Registry führt `micromole/kg`), `seismic-body km/s` (`:28556`/`:29211`), `em K` (`:29022`, hi4pi Helligkeitstemperatur), `em nmgy` (`:29093`, NOIRLab-Flux). In diesem Atom in `docs/specs/force-unit-baseline.txt` **eingefroren**, um den Register-Commit freizugeben.
- **Blockade:** keine.
- **Braucht:** je Paar ein Verdikt und Admission in `src/archivar/units.rs` (`allowed_units_for_force`/`for_quantity`) oder Korrektur der Register-Zeile — dann die Freeze-Zeile in der Baseline löschen (die Datei schrumpft).

## An mycelium

Origin: mountain-338 (2026-10-13):

- `xfer:mountain.ses_ecae8b148ffeT4kqLjZt0U6fKV.1~15a632a3` **Register-Risse geschlossen.**
  `cosmic2_gnssro`-Format → `format cosmic_ro` (`phi/sources.φ:10244`; Arm `extract.rs:1253`,
  Compiler emittiert `cosmic_ro`, `MAGIC_COSMIC`); LHAASO-E0 im Observable-Namen
  (`lhaaso_sky1_e0_3tev` WCDA 3 TeV · `lhaaso_km2a_e0_50tev` KM2A 50 TeV), Feld-Schlüssel
  unverändert. Kein CI-Wiring betroffen (der LHAASO-Compiler emittiert keinen Block).

## Operator-Wort-Register

- **„Projekt-Ownership statt statischer Verantwortlichkeiten — jeder schiebt den schwarzen Peter … ihr seid alle gleich ausgebildet"** | 2026-10-13 | Operator-Session (mountain 336) — kein Ball-Zuschieben.
- **„warum haben wir immer noch no-license-stated und fragwürdige lizenzen auf cdn?"** | 2026-10-13 | Operator-Session (mountain 336) — Riegel + Auflösung; `no-license-stated` = 0.
- **„bitte prüfe was zu viel entfernt wurde"** | 2026-10-13 | Operator-Session (mountain 336) — 8 zu viel entfernte Blöcke wiederhergestellt.
- **„und haben wir jetzt für alle offene alternativen?"** | 2026-10-13 | Operator-Session (mountain 336) — teils.
- **„liegen jetzt nur noch einwandfreie Assets auf CDN?"** | 2026-10-13 | Operator-Session (mountain 336) — nein; Orphan-Bestände, Takedown an Mycelium.
- **„das muss eindeutig recherchiert und an mycelium gegeben werden"** | 2026-10-13 | Operator-Session (mountain 336).
- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater … kein Consent-Stopp für Bekanntes."** | 2026-10-13 | Operator-Session (mountain 337/338) — Linien-Start = session-weiter Consent (Delegation), getrennt vom Commit-Wort.
- **„du hast nur gemessen und registriert … bitte delegiere an einen taucher schwarm"** | 2026-10-13 | Operator-Session (mountain 337) — Ersatz-Bau per Schwarm.
- **„was davon kannst du umsetzen und an taucher deligieren?"** | 2026-10-13 | Operator-Session (mountain 337).
- **„mach das [IRSF/CEEIN-Blöcke + Workflows, LHAASO-Widerspruch] und hast du die secrets local geprüft? Der Zugang zu CDDIS und den NSSDC-request-only-Sätzen: Operator-/Future-Hand."** | 2026-10-13 | Operator-Session (mountain 337) — Register-Blöcke + Workflows umsetzen, LHAASO auflösen, `.secrets.local` prüfen (nur Namen).
- **Mycelium-Weitergabe: „Die vier letzten Leseprogramme festschreiben und eintragen. Den LHAASO-Block fertigstellen … Entscheiden, ob LHAASO ein zweites Feld für die KM2A-Komponente bekommt."** | 2026-10-13 | Operator-Session (mountain 337, Mycelium-Auftrag) — vier Compiler + Block geprüft; LHAASO vollendet; **KM2A: ja**.
- **„kannst du das mit recherche roster und rat klären?"** | 2026-10-13 | Operator-Session (mountain 338) — die vier offenen Punkte durch Recherche + UI-Roster + Rat klären (erledigt; Rohmaterial `state/stimmen/2026-10-13_roster_mountain-vier-entscheidungen.md`).

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push. In diesem Atom (folge338)
committet: die neue Übergabe + die konsumierte folge337 → `archiv/`; die zwei
Register-Riss-Zeilen in `phi/sources.φ` (`cosmic2_gnssro` → `cosmic_ro`; die
LHAASO-Observable-Namen `lhaaso_sky1_e0_3tev`/`lhaaso_km2a_e0_50tev`) — block-scoped;
der Freeze der 5 Force-Unit-Paare in `docs/specs/force-unit-baseline.txt`. Die
Klärung der vier Punkte (Recherche/Rat/Roster) ist in dieses Register geschrieben;
das Rohmaterial liegt unter `state/stimmen/`.
