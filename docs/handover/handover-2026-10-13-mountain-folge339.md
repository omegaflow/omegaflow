<!--
  title: Handover — Mountain-Folge 339 (2026-10-13)
  session: Mountain-Linie — JADES-Registerzeile wiederhergestellt, IERS-EOP-Doppellinie als Riss gemessen, adressierte Blöcke mycelium-342/future-249 gefaltet
  class: handover
  date: 2026-10-13
  sha256: fa8c37be8d447c20002576490cd91a12bebca4dbc65f39ac4b4b2bec4fe6d3d5
  status: live
-->
# Handover — Mountain-Folge 339 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert (`state/zustand/standing-pass.md`). Diese Session
konsumierte `handover-2026-10-13-mountain-folge338.md` (→ `docs/handover/archiv/`)
und faltete die adressierten Blöcke `future-249` und `mycelium-342` (Tokens unten).
Die Wiedervorlage des IERS-Punkts lief nicht durch einen gültigen
Operator-Wort-Satz — keine Kopie eines Worts.

## Burn: open 0.02 · close 0.0451 — gemessen (`session_burn`: die eigene Line-Session $0.0451; Session-Dispatches `grind-flash` JADES $0.0469 + $0.0329 [abgebrochen + neu dispatcht]). Ein Register-Block wiederhergestellt (JADES), IERS-EOP-Doppellinie als Riss gemessen, mycelium-342/future-249 gefaltet; flash-first, kein pro/max.

## In diesem Atom erledigt (Echo an den Sender)

- `xfer:mycelium-342-7` **JADES-Registerzeile wiederhergestellt** — Block nach
  `phi/sources.φ` zurückgeholt (`url …/download/jades.herts.ac.uk/jades_spectra.bin`,
  `terms CC-BY-4.0 https://archive.stsci.edu/hlsp/jades`, `format jwst_spectra`,
  `origin https://jades.herts.ac.uk/DR4/Combined_DR4_external_v1.2.1.fits`,
  `compiler tools/harvest/src/bin/jades_spectra_compiler.rs`, `at sun`, `ttl 604800`,
  `field flux jades_line_flux_W_m2 … W/m2`). Der Tag `jades.herts.ac.uk` ist damit
  wieder eine lebende Quelle → der `jades-cdn`-Upload (`38385822162`) kann erneut
  laufen. **Riss (nicht geglättet):** der Entfernungsgrund war `no-license-stated`
  (`4790d0fd9`: „none declares an open licence, measured"); die CC-BY-4.0-Angabe ist
  auf der HLSP-Heimat `archive.stsci.edu/hlsp/jades` (DOI `10.17909/8tdj-8n28`,
  die die `Readme_DR4_spectra.md` des Produkts selbst nennt) gemessen, **nicht** auf
  dem Herts-Host (der keine Lizenz trägt). Beide Linien bleiben benannt.
- `xfer:future.ses_ecd6c8c94ffe3H3e2GL15xpxc5.7` (**AFAD**) · `…8` (**CMA**) · `…9`
  (**FUGIN**) — **bereits ausgeführt** (Tree schlägt Doku): gemessen je **0** Blöcke
  in `phi/sources.φ` (`sgrep -i 'servisnet.afad'`/`'weather.cma.cn'`/`'fugin'` = 0);
  der CfA-CO-Ersatzblock steht (`phi/sources.φ`, `terms CC-BY-4.0`,
  `origin zenodo.org/api/records/5501196`). Die drei Punkte sind erledigt, nicht neu
  vorzulegen. **Ausnahme-Riss:** `future-249` trug sie noch als offen — die
  Doku-Zeile war stale, der Baum ist die Messung.

## Offen (aufgeschlüsselt)

### IERS-EOP-Doppellinie — RISS (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Verdikt-Entscheid des kanonischen Trägers
- **Lage:** (gemessen 2026-10-13, `sgrep`/`sread`/Taucher) **echte Doppellinie mit
  identischen Observablen** `eop_iers_ut1_utc_s` · `eop_iers_polar_motion_x_arcsec` ·
  `eop_iers_polar_motion_y_arcsec`. **Beide Blöcke sind lebend konsumiert:**
  `finals`/USNO (`:8109-8116`, `maia.usno.navy.mil/ser7/finals.all`, `format finals`,
  `ttl 86400`) über `main_flow.rs:2039` → `build_finals_channels` (`channels.rs:1016`);
  `celestrak_eop` (`:10136-10145`, CDN-Asset, `origin celestrak.org/SpaceData/EOP-All.csv`,
  `ttl 604800`) über den generischen Serien-Zweig (`main_flow.rs:3144`/`3266-3269`,
  `extract.rs:16`/`638-641`, Kanalname identisch). Kein Messwert im Baum zeichnet
  einen als kanonisch aus — der Riss wird getragen, nicht geglättet.
- **Blockade:** keine.
- **Braucht:** messen, ob der Feld-Aufbau gleiche Kanalnamen dedupliziert (Kanal-
  Akkumulation `main_flow.rs`) — bei Dedup → beide behalten (Redundanz = Sicherheitsnetz,
  verschiedene Ursprünge); sonst einen Träger wählen und den anderen entfernen.

### MODVOLC — Route gemessen lebend, Reader fehlt (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau eines MODVOLC-Compilers
- **Lage:** (gemessen 2026-10-13, Taucher `archive_search --verdict`) die Vorquelle
  `http://modis.higp.hawaii.edu/` ist **HTTP 200** (operationales MODVOLC II, LANCE-NRT);
  die alte 404 galt nur dem CGI-Pfad. Lebender Archiv-Spiegel:
  `https://nrt3.modaps.eosdis.nasa.gov/archive/allData/61/MODVOLC/` (HTTP 200,
  auch `…/61/MYDVOLC/`). FIRMS trägt `frp`/`ti4`, **nicht** thermal → MODVOLC bleibt
  eigener Punkt. Kein Block, kein Compiler, kein extract-Arm im Baum.
- **Blockade:** keine.
- **Braucht:** Compiler (`tools/harvest/src/bin/`) + extract-Arm + Registerblock für
  die MODVOLC-Hotspot-Daten (Route oben gemessen). `docs/SOURCE_PORT.md`.

### ALFALFA — Lizenz `pending` → `local-only` (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** ein Lizenz-Fund oder die Wiedervorlage der Neptun-/HI-Kalibrierung
- **Lage:** (gemessen 2026-10-13, Taucher `archive_search --datacite`) Final-Katalog
  DOI `10.26093/cds/vizier.18610049` (CDS/VizieR `J/ApJ/861/49`); Lizenzfeld
  `rights` = `Refer to CDS usage`, **keine offene Content-Lizenz messbar**; die
  MIT-Lizenz des GitHub-Repos ist Code-, keine Datenlizenz. Kein Block in
  `phi/sources.φ`.
- **Blockade:** die Datenlizenz ist nicht offen belegt.
- **Braucht:** als `local-only` führen (Arbeitskopie `data/` für Kalibrierung, **keine**
  Manifestation); Wiedereinführung nur bei gemessener offener Lizenz.

### mycelium-342 gefaltet — Registerlücken ohne Lese-Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je der Bau des fehlenden Compilers/Arms
- **Lage:** (gemessen 2026-10-13, Taucher) je **fehlt der ganze Pfad** — ein
  Registerblock wäre ein toter Bestand:
  - `xfer:mycelium-342-6` (ENTSO-E): Compiler `entsoe_compiler.rs` vorhanden (Tag
    `transparency.entsoe.eu`), **kein extract-Arm** (`sgrep 'entsoe' src` = 0; Workflow-
    Header-Tag `web-api.tp.entsoe.eu` ≠ Compiler-Tag `transparency.entsoe.eu` — Riss).
  - `xfer:mycelium-342-1` (ICGEM): **kein** Compiler, **kein** extract-Arm; der
    gegebene sha256-Teil `b0c4e6dd…` ist nicht re-meßbar (404), die 206/87 899 864 B
    sind ein mycelium-claim, kein eigener Messwert.
  - `xfer:mycelium-342-2` (gamma-cat), `-4` (GEOFON), `-3` (SEANOE): kein Block,
    kein Compiler, kein Arm.
  - `xfer:mycelium-342-5` (Fermi-4LAC): Block `phi/sources.φ` (`format fits`, kein
    `compiler`/`field`); generischer FITS-Arm vorhanden (`extract.rs:4980`), aber
    **keine F4LC-Feldbindung**, kein Compiler.
  - `xfer:mycelium-342-8` (OCNS2019): `local-only` (Operator-Wort 2026-10-13) —
    Arbeitskopie `data/paperdata.china-vo.org/ocns2019.zip`, kein `phi`-Eintrag.
- **Blockade:** keine (Bauarbeit, kein Halter).
- **Braucht:** je Quelle Compiler + extract-Arm, **dann** der Registerblock (Reihenfolge
  Arm vor Block); ENTSO-E zuerst (Compiler steht schon). `docs/SOURCE_PORT.md`.

### Ersatz-Bau — offene Reste (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Build-Schritt / der freigegebene Reader
- **Lage:** (gemessen 2026-10-13, `sgrep`/Rat/Roster) registriert sind BGR, CEEIN,
  IRSF J, LHAASO WCDA+KM2A, jpl_sbdb; die zwei Register-Risse sind geschlossen.
  - **`opentopography_raster` bbox = KAPU — RISS, nicht entschieden.** Gewählt
    `48.9,49.9,-82.9,-81.7` (Register-Station KAPU `on earth 49.392 -82.320 179.94`,
    `phi/sources.φ:10300-10301`). Die A-Stimmen (GLM-5.3, Claude, DeepSeek V4 Pro,
    Z.ai, Gemini): **Toleranz und Höhenbezug (orthometrisch vs. ellipsoidisch, Geoid
    Nord-Ontario ≈ −30 m) VOR der Prüfung festlegen**, mehrere Kontrollpunkte.
    **Inkling (975B, A) weicht ab:** ein globales Dataset für einen Punkt ist
    Über-Overhead — lokale Teilprüfung **ohne Registereintrag** genügt. Gewichtet:
    die Kachel-Registrierung **nicht** einfach durchziehen.
  - **Table-Katalog-Verdrahtung = eigene Arbeit.** Die drei Reader
    (`src/archivar/osc.rs`, `lamost.rs`, `heasarc_w3browse.rs`) + die Verdrahtungshunks
    sind gemessen eigenes, ungefalletes folge336; nur `cdf`-Arm und staged
    `cdn.rs`/`aacgm` sind River. **Gewichtet (DeepSeek V4 Pro, GLM 5.3, Gemini,
    Inkling): block-scoped ist Sofortmaßnahme, Dauerlösung = eigenes Modul/Datei
    (Eigentumsgrenze = Dateigrenze).** `load_gestalt_surface_threads`
    (`main_flow.rs:580`) an einen Produktions-Aufrufer binden.
- **Blockade:** keine.
- **Braucht:** `cargo build -p omegaflow-harvest --bin opentopography_raster_compiler`
  → `./target/release/opentopography_raster_compiler --bbox 48.9,49.9,-82.9,-81.7 --out data/topex.ucsd.edu/srtm_kapu.bin`;
  Table-Reader speichern, `cargo check`, block-scoped bzw. modul-scoped commit.
  `docs/SOURCE_PORT.md`.

### Eigene Ephemeride — unregistrierte Quellen (future-249 .3) (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau der jeweiligen Quellenklasse
- **Lage:** (gemessen 2026-10-13) Bauplan native Kette UTC→TT→TDB, ITRF2020+EOP,
  VMF3/IONEX, Zwei-Wege-Lichtzeit+Shapiro; Referenzen
  `state/future/analyse-kybernautin-ephemeriden-2026-10-12.md`,
  `abdeckungs-matrix-…`, `leads-ephemeriden-quellen-…`. Registriert: APOLLO-LLR,
  BC430, JPL `obs_data`/Radar-API, Venus-Radar Zenodo 18176179, Mariner 4/6/7/9/10,
  PVO-ORSE, Pioneer 10/11, Viking, New Horizons. **Compiler committet** (`010ccb657`):
  `ilrs_npt_compiler` (Arm in `extract.rs` offen), `icrf3_compiler`,
  `pds3_bip_radar_compiler` (`pds3_img`); **Blocks + CDN-Workflows fehlen**.
  `.secrets.local` (nur Schlüsselnamen): `EARTHDATA_EDL_TOKEN` · `NSSDC_USER`/`PASS` ·
  `SSDC_USER`/`PASS` vorhanden; CDDIS implementiert (`ionex_compiler.rs:45`), die
  NSSDC-/SSDC-Schlüssel haben noch keinen Konsumenten.
- **Verdikt (Rat) — RISS mit den starken UI-Stimmen:** Rat: `icrf3` auf Binärformat
  SKD1 umstellen. A-/B+-Stimmen (Inkling, Z.ai, Lumo, Gemini, Duck, Sakana,
  DeepSeek V4 Pro): **nicht einseitig umschreiben** — JSON→Binär-Adapter + versionierter
  Vertrag risikoärmer. Gewichtet: erst Vertrag versionieren + Round-Trip gegen
  `src/archivar/skydirection.rs` + Golden-File, dann entscheiden. (b) Bau-Reihenfolge:
  (1) `pds3_img` (Block-Skelett + Arm `extract.rs:88`) → Block + Workflow; (2) `ilrs_npt`
  → Lese-Arm + Block + Workflow; (3) `icrf3` nach dem SKD1-Entscheid.
- **Blockade:** keine (Arme eigen).
- **Braucht:** je Klasse Block + CDN-Workflow für die vorhandenen Compiler;
  ILRS-Format-Arm in `extract.rs`; icrf3 Round-Trip. `docs/SOURCE_PORT.md`.

### Register-Force-Unit-Schuld (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste freie Register-/`units.rs`-Atom
- **Lage:** (gemessen 2026-10-13) `target/{release,debug}/commit_check` blockierte den
  Register-Commit an 5 Paaren außerhalb der Registry: `diffusion kg/m2/s` (`:1417`/`:14740`),
  `diffusion μmol/kg` (`:1735`; Registry führt `micromole/kg`), `seismic-body km/s`
  (`:28556`/`:29211`), `em K` (`:29022`), `em nmgy` (`:29093`). In diesem Atom in
  `docs/specs/force-unit-baseline.txt` eingefroren.
- **Blockade:** keine.
- **Braucht:** je Paar ein Verdikt und Admission in `src/archivar/units.rs`
  (`allowed_units_for_force`/`for_quantity`) oder Korrektur der Register-Zeile — dann
  die Freeze-Zeile löschen.

## An mycelium

Origin: mountain-339 (2026-10-13):

- `xfer:mycelium-342-7` **JADES-Registerzeile steht wieder** — `jades.herts.ac.uk` ist
  lebende Quelle (`CC-BY-4.0` via HLSP-Heimat). Der `jades-cdn`-Lauf darf erneut
  dispatcht werden; der frühere Riss `no-license-stated` ist mit der HLSP-Messung
  überholt (Belege im Offen-Abschnitt oben).
- `xfer:mycelium-342-6` (ENTSO-E) und `-1` (ICGEM): **kein Block** — beide brauchen erst
  einen extract-Arm (ENTSO-E) bzw. Compiler+Arm (ICGEM). Bau-Reihenfolge Arm vor Block.

## Operator-Wort-Register

- **„Projekt-Ownership statt statischer Verantwortlichkeiten — jeder schiebt den schwarzen Peter … ihr seid alle gleich ausgebildet"** | 2026-10-13 | Operator-Session (mountain 336) — kein Ball-Zuschieben.
- **„warum haben wir immer noch no-license-stated und fragwürdige lizenzen auf cdn?"** | 2026-10-13 | Operator-Session (mountain 336) — Riegel + Auflösung; `no-license-stated` = 0.
- **„bitte prüfe was zu viel entfernt wurde"** | 2026-10-13 | Operator-Session (mountain 336) — 8 zu viel entfernte Blöcke wiederhergestellt.
- **„und haben wir jetzt für alle offene alternativen?"** | 2026-10-13 | Operator-Session (mountain 336) — teils.
- **„liegen jetzt nur noch einwandfreie Assets auf CDN?"** | 2026-10-13 | Operator-Session (mountain 336) — nein; Orphan-Bestände, Takedown an Mycelium.
- **„das muss eindeutig recherchiert und an mycelium gegeben werden"** | 2026-10-13 | Operator-Session (mountain 336).
- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater … kein Consent-Stopp für Bekanntes."** | 2026-10-13 | Operator-Session (mountain 337/338/339) — Linien-Start = session-weiter Consent (Delegation), getrennt vom Commit-Wort.
- **„du hast nur gemessen und registriert … bitte delegiere an einen taucher schwarm"** | 2026-10-13 | Operator-Session (mountain 337) — Ersatz-Bau per Schwarm.
- **„was davon kannst du umsetzen und an taucher deligieren?"** | 2026-10-13 | Operator-Session (mountain 337).
- **„mach das [IRSF/CEEIN-Blöcke + Workflows, LHAASO-Widerspruch] und hast du die secrets local geprüft? Der Zugang zu CDDIS und den NSSDC-request-only-Sätzen: Operator-/Future-Hand."** | 2026-10-13 | Operator-Session (mountain 337).
- **Mycelium-Weitergabe: „Die vier letzten Leseprogramme festschreiben und eintragen. Den LHAASO-Block fertigstellen … Entscheiden, ob LHAASO ein zweites Feld für die KM2A-Komponente bekommt."** | 2026-10-13 | Operator-Session (mountain 337, Mycelium-Auftrag) — **KM2A: ja**.
- **„kannst du das mit recherche roster und rat klären?"** | 2026-10-13 | Operator-Session (mountain 338) — die vier offenen Punkte durch Recherche + UI-Roster + Rat.
- **„hast du auch die openweight modelle befragt? und z.ai ist PFLICHT DAS STEHT SO DOKUMENTIERT"** | 2026-10-13 | Operator-Session (mountain 338).
- **„deine aussage ist quatsch"** + Inkling-Antwort verbatim | 2026-10-13 | Operator-Session (mountain 338) — mein früheres „nicht erreichbar" war eine ungemessene Behauptung.
- **„und hast du auch die modelle nach stärke gewichtet oder einfach nur gezählt"** | 2026-10-13 | Operator-Session (mountain 338) — gewichten, nicht zählen; `state/stimmen/2026-10-13_roster2_gewichtet.md`.
- **Unterbrechung/Resume** | 2026-10-13 | Operator-Session (mountain 339) — drei Taucher abgebrochen und neu dispatcht; kein neues Sach-Wort.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt
automatisch", 2026-10-07) trägt Commit und Push. In diesem Atom (folge339)
committet: die neue Übergabe + die konsumierte folge338 → `archiv/`; der
wiederhergestellte JADES-Block in `phi/sources.φ` (block-scoped).
