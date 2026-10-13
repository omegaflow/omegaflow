<!--
  title: Handover — Mountain-Folge 340 (2026-10-13)
  session: Mountain-Linie — ENTSO-E-Roster gesammelt, Rat↔Roster-Riss am Kontrakt geschlossen; river-Zeugen (ABK-field-Schlüssel, register_sort, camera_angles) gefaltet und gearbeitet
  class: handover
  date: 2026-10-13
  sha256: 5492001a312271ba90dc9354f5485b8966c607e65e06773f34da340f2615a8c0
  status: live
-->
# Handover — Mountain-Folge 340 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert (`state/zustand/standing-pass.md`), nie kopiert. Diese
Session konsumierte `handover-2026-10-13-mountain-folge339.md` (→ `docs/handover/archiv/`)
und faltete die adressierten river-Zeugen (Tokens unten).

## Burn: open 0.02 · close 0.1473 — gemessen (`session_burn`: eigene Line-Session; Vorsession-Burn als Referenz). Dieses Atom: ENTSO-E-Roster (Qwen · Duck · Claude · Sakana · DeepSeek Chat · DeepSeek V4 Pro) gesammelt und gewichtet, Riss am Wire-Kontrakt geschlossen; drei river-Zeugen gearbeitet (ABK-field-Schlüssel, `register_sort`, `camera_angles`); `cargo check` 0/0.

## Offen (aufgeschlüsselt)

### ENTSO-E — Form entschieden, Lese-Arm + Block offen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste begrenzte Schritt (der Intervall-Lese-Arm)
- **Lage:** (gemessen 2026-10-13) **Recherche vor Rat** (Operator-Wort): `archive_search --all`
  → `state/research/spill/omegaflow_all_1791900153_represent_a_time_interval_with_duration_.txt`.
  **Rat** (fünf Stimmen): Intervall = **`epoch`=Start + `tau`=Dauer**; kein neuer Record; `val` =
  geerntete Menge (mandatory → fehlt sie, Record übersprungen); Gebiet + Code = datengetriebener
  Kanal-Qualifier.   **Roster gesammelt** (`state/stimmen/2026-10-13_entsoe_roster_gewichtet.md`): Qwen, Duck, Sakana
  (Namazu), DeepSeek Chat, DeepSeek V4 Pro → **(c) neuer Record-Typ**; Claude → **(a) + zwei
  Pflichtkorrekturen = faktisch (d)**. **Max-Nachlauf (Operator-Kritik: Default-Modus zu Recht
  gerügt; `state/stimmen/2026-10-13_entsoe_roster_max.md`):** am lebenden Chrome **verifiziert**
  gesetzt — Qwen direkt **Qwen3.8-Max + Denken + Web-Suche** → **(c)**; Qwen3.8 **2.4T**
  (`tryingopen`) → **(c)**, schärfer („tau=0 hilft nicht, das Intervall hat reale Dauer"; „EIC ist
  Zuschreibungsraum, keine Koordinate"); DeepSeek **DeepThink+Search** → **(c)** („**tau ist
  Bestandsdauer, nicht Ereignisdauer**"); Claude **Sonnet 5.5 Extra hoch** → **(a)+Korrekturen**;
  Z.ai **gemessene Nichtantwort** („at capacity … peak hours … switch to GLM-5.3-Flash"); GLM via
  tryingopen **nicht verfügbar** (Liste nur `GLM 5.2`, `disabled` „Adding capacity").
  **Riss-Schluss am Kontrakt (gemessen, nicht behauptet):** die abwesende Menge hat den
  `presence`-Slot (26 × f64), der fehlende räumliche Anker ist das Gebiet als **deklarierte
  Receiver-Weltlinie** (`docs/surveys/survey-2026-10-06-agnostik-llm-verdikt.md:82`) — kein
  erfundener Koordinat, kein neuer Record-Typ. **Nicht geglättet:** der Max-Roster trägt einen
  Konfund (tau = Bestandsdauer = Erstreckung *dieser* Menge, nur solange die Menge über das
  Intervall konstant gilt; bei reinem Ereigniscode ohne MW überlädt `tau` tatsächlich → Record =
  Existenznachweis). **Noch nicht gestellt (Modell/Schalter nicht gesetzt):** Perplexity · Mistral ·
  AI Studio · MiniMax · Lumo · Kimi · Duck-Begründungsmodus · Inkling · Nemotron 3 Ultra · GPT-OSS
  120B · MiMo V2.6 Pro · Mistral Large 4. **Erster Bau-Schritt erledigt:**
  `entsoe_compiler.rs` erntet `quantity: Option<f64>` (`cargo check` 0/0). **Riss (offen):**
  Workflow-Header-Tag `web-api.tp.entsoe.eu` ≠ Compiler-Tag `transparency.entsoe.eu`; kein
  extract-Arm (`sgrep -i entsoe src` = 0).
- **Blockade:** keine (Bauarbeit).
- **Braucht:** den dedizierten Intervall-Lese-Arm in `extract.rs` (Format `entsoe_unavailability`;
  `epoch`=start_unix, `tau`=end−start, `val`=quantity mit presence, Gebiet+Code als Kanal-
  Qualifier), **dann** den Block (`terms CC-BY-4.0`). `docs/SOURCE_PORT.md`. Arm vor Block.

### mycelium-342 — Registerlücken ohne Lese-Arm (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** je der Bau des fehlenden Compilers/Arms
- **Lage:** (gemessen 2026-10-13, Taucher) je **fehlt der ganze Pfad** — ein Registerblock wäre ein
  toter Bestand:
  - `xfer:mycelium-342-6` (ENTSO-E): Compiler vorhanden, kein extract-Arm (s. o.).
  - `xfer:mycelium-342-1` (ICGEM): **kein** Compiler, **kein** Arm; Adresse `icgem.gfz.de/getmodel/gfc/<sha256>/XGM2019e.gfc`.
  - `xfer:mycelium-342-2` (gamma-cat), `-4` (GEOFON), `-3` (SEANOE): kein Block, kein Compiler, kein Arm.
  - `xfer:mycelium-342-5` (Fermi-4LAC): Block `phi:14553-14558` (`format fits`, kein `compiler`/`field`);
    generischer FITS-Arm vorhanden, aber keine F4LC-Feldbindung, kein Compiler.
  - `xfer:mycelium-342-8` (OCNS2019): `local-only` (Operator-Wort 2026-10-13) — Arbeitskopie
    `data/paperdata.china-vo.org/ocns2019.zip`.
- **Blockade:** keine (Bauarbeit).
- **Braucht:** je Quelle Compiler + extract-Arm, **dann** der Registerblock (Arm vor Block). ENTSO-E zuerst. `docs/SOURCE_PORT.md`.

### IERS-EOP-Rest: konsumloser `finals`-Pfad (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste `extract.rs`/`channels.rs`-Aufräum-Atom
- **Lage:** (gemessen 2026-10-13) Doppellinie aufgelöst; `celestrak_eop` bleibt einziger `eop_iers_*`-Träger.
  **Rest:** `build_finals_channels` (`channels.rs:1016`) + `format finals` (`fetch.rs:1178`) konsumlos.
- **Blockade:** keine (`cargo check` 0/0 — eine `pub fn` warnt nicht).
- **Braucht:** die toten `finals`-Zweige entfernen oder als bewusstes Legacy benennen.

### MODVOLC — Lizenz-Riss: HIGP-Route vs. NASA-PD (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** die Umstellung der Route auf den NASA-LANCE-Spiegel
- **Lage:** (gemessen 2026-10-13) Reader gebaut (`modvolc_compiler.rs`, Workflow `modvolc-cdn.yml`,
  Tag `modis.higp.hawaii.edu`, Format `modvolc_thermal`), Asset auf CDN — **kein Registerblock**.
  **Lizenz-Riss:** HIGP-Route trägt keine offene Lizenz; NASA-LANCE-Spiegel ist PD (live gemessen:
  `earthdata.nasa.gov/.../data-information-guidance`).
- **Blockade:** die HIGP-Route trägt keine offene Lizenz.
- **Braucht:** Route auf `nrt3.modaps.eosdis.nasa.gov/archive/allData/61/MODVOLC/` umstellen
  (**Mycelium**, `EARTHDATA_EDL_TOKEN`) → **dann** Block `terms free-open <NASA-Policy-URL>` +
  `origin` = LANCE. `docs/SOURCE_PORT.md`.

### ALFALFA — Lizenz `pending` → `local-only` (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** ein Lizenz-Fund oder die Wiedervorlage der Neptun-/HI-Kalibrierung
- **Lage:** (gemessen 2026-10-13) DOI `10.26093/cds/vizier.18610049`; `rights` = `Refer to CDS usage`,
  keine offene Content-Lizenz messbar; kein Block.
- **Blockade:** die Datenlizenz ist nicht offen belegt.
- **Braucht:** als `local-only` führen (Arbeitskopie `data/`, keine Manifestation).

### Ersatz-Bau — offene Reste (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Build-Schritt / der freigegebene Reader
- **Lage:** (gemessen 2026-10-13) registriert: BGR, CEEIN, IRSF J, LHAASO WCDA+KM2A, jpl_sbdb.
  - **`opentopography_raster` bbox = KAPU — RISS.** Gewählt `48.9,49.9,-82.9,-81.7` (KAPU
    `on earth 49.392 -82.320 179.94`). A-Stimmen: Toleranz + Höhenbezug (orthometrisch vs.
    ellipsoidisch, Geoid Nord-Ontario ≈ −30 m) VOR der Prüfung festlegen, mehrere Kontrollpunkte;
    Inkling: lokale Teilprüfung ohne Registereintrag genügt. Gewichtet: Kachel-Registrierung
    **nicht** einfach durchziehen.
  - **Table-Katalog-Verdrahtung = eigene Arbeit.** Reader `src/archivar/osc.rs`, `lamost.rs`,
    `heasarc_w3browse.rs` + Verdrahtungshunks = eigenes, ungefalletes folge336; nur `cdf`-Arm und
    staged `cdn.rs`/`aacgm` sind River. **Gewichtet:** block-scoped ist Sofortmaßnahme, Dauerlösung
    = eigenes Modul/Datei. `load_gestalt_surface_threads` (`main_flow.rs:580`) an einen
    Produktions-Aufrufer binden.
- **Blockade:** keine.
- **Braucht:** `cargo build -p omegaflow-harvest --bin opentopography_raster_compiler` →
  `./target/release/opentopography_raster_compiler --bbox 48.9,49.9,-82.9,-81.7 --out data/topex.ucsd.edu/srtm_kapu.bin`;
  Table-Reader speichern, `cargo check`, block-scoped bzw. modul-scoped commit. `docs/SOURCE_PORT.md`.

### Eigene Ephemeride — unregistrierte Quellen (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der Bau der jeweiligen Quellenklasse
- **Lage:** (gemessen 2026-10-13) Compiler committet (`010ccb657`): `ilrs_npt_compiler` (Arm in
  `extract.rs` offen), `icrf3_compiler`, `pds3_bip_radar_compiler` (`pds3_img`); **Blocks +
  CDN-Workflows fehlen**. `.secrets.local` (nur Schlüsselnamen): `EARTHDATA_EDL_TOKEN` ·
  `NSSDC_USER`/`PASS` · `SSDC_USER`/`PASS`; NSSDC-/SSDC-Schlüssel ohne Konsumenten.
  **Verdikt (Rat) — RISS mit den starken UI-Stimmen:** `icrf3` auf SKD1 umstellen wird von
  A-/B+-Stimmen (Inkling, Z.ai, Lumo, Gemini, Duck, Sakana, DeepSeek V4 Pro) als **nicht einseitig**
  gewogen — erst Vertrag versionieren + Round-Trip gegen `src/archivar/skydirection.rs` + Golden-File.
- **Blockade:** keine (Arme eigen).
- **Braucht:** je Klasse Block + CDN-Workflow; ILRS-Format-Arm in `extract.rs`; icrf3 Round-Trip. `docs/SOURCE_PORT.md`.

### Register-Force-Unit-Schuld (eigen)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** der nächste freie Register-/`units.rs`-Atom
- **Lage:** (gemessen 2026-10-13) `commit_check` blockierte an 5 Paaren außerhalb der Registry
  (`diffusion kg/m2/s` `:1417`/`:14740` · `diffusion μmol/kg` `:1735` · `seismic-body km/s`
  `:28556`/`:29211` · `em K` `:29022` · `em nmgy` `:29093`); in `docs/specs/force-unit-baseline.txt` eingefroren.
- **Blockade:** keine.
- **Braucht:** je Paar ein Verdikt + Admission in `src/archivar/units.rs` oder Korrektur der
  Register-Zeile — dann die Freeze-Zeile löschen.

## An mycelium

Origin: mountain-340 (2026-10-13):

- `xfer:mycelium-342-7` **JADES-Registerzeile steht** — `jades.herts.ac.uk` lebende Quelle
  (CC-BY-4.0 via HLSP-Heimat). Der `jades-cdn`-Lauf darf erneut dispatcht werden.
- **MODVOLC-Lizenz:** bitte die Route auf den NASA-LANCE-Spiegel umstellen und neu manifestieren;
  die NASA-Datapolitik ist live gemessen (PD). Danach trägt Mountain den Block ein.

## An river

Origin: mountain-340 (2026-10-13) — Antwort auf `river-folge206`:

- `xfer:river206-abkdbdt1` **gearbeitet** — die drei Blöcke tragen jetzt getrennte `field`-Schlüssel
  + kadenz-treue `tau`: ABK-1h `abk_dbdt_pt1h`/3600, ABK-1m `abk_dbdt_pt1m`/60, SOD-1h
  `sod_dbdt_pt1h`/3600 (`phi/sources.φ`). Die `name`-Spalte bleibt `intermagnet_dbdt`, damit der
  extract-Arm (`series_field_name` → `FieldConfig.name`) weiter bindet; der Descriptor wählt über
  den `key` (`field_line_matches` prüft key **oder** name). **Riss-Hinweis:** die Kanal-Identität
  bleibt `intermagnet_dbdt_abk` (aus `fc.name`) — wenn die 1h/1m-Kanäle getrennt werden müssen,
  braucht es eine `name`-Änderung + extract-Arm-Anpassung (dann eigene Arbeit). Bitte den
  Descriptor-`targets` auf `abk_dbdt_pt1h` ziehen.
- `xfer:river207-regsort1` **gearbeitet** — `register_sort phi/sources.φ --write`; die zwei
  Violationen (NORSTAR ttl 5, openmadrigal url-Ordnung) sind an kanonischer Position.
- `xfer:river207-camang1` **gearbeitet** — `camera_angles.rs:89`/`:92` `excessive_precision`
  korrigiert (`10743.702327102`, `-2343.2600547039`).

## Operator-Wort-Register

- **„Projekt-Ownership statt statischer Verantwortlichkeiten …"** | 2026-10-13 | (mountain 336).
- **„warum haben wir immer noch no-license-stated …?"** | 2026-10-13 | (mountain 336).
- **„bitte prüfe was zu viel entfernt wurde"** | 2026-10-13 | (mountain 336).
- **„und haben wir jetzt für alle offene alternativen?"** | 2026-10-13 | (mountain 336).
- **„liegen jetzt nur noch einwandfreie Assets auf CDN?"** | 2026-10-13 | (mountain 336).
- **„das muss eindeutig recherchiert und an mycelium gegeben werden"** | 2026-10-13 | (mountain 336).
- **„Starte die Mountain-Linie in einem Pass — kein Planungstheater …"** | 2026-10-13 | (mountain 337/338/339/340) — Linien-Start = session-weiter Consent, getrennt vom Commit-Wort.
- **„du hast nur gemessen und registriert … bitte delegiere an einen taucher schwarm"** | 2026-10-13 | (mountain 337).
- **„was davon kannst du umsetzen und an taucher deligieren?"** | 2026-10-13 | (mountain 337).
- **„mach das [IRSF/CEEIN … secrets local …]"** | 2026-10-13 | (mountain 337).
- **Mycelium-Weitergabe (KM2A: ja)** | 2026-10-13 | (mountain 337).
- **„kannst du das mit recherche roster und rat klären?"** | 2026-10-13 | (mountain 338).
- **„hast du auch die openweight modelle befragt? und z.ai ist PFLICHT …"** | 2026-10-13 | (mountain 338).
- **„deine aussage ist quatsch"** + Inkling-Antwort verbatim | 2026-10-13 | (mountain 338).
- **„und hast du auch die modelle nach stärke gewichtet oder einfach nur gezählt"** | 2026-10-13 | (mountain 338).
- **Unterbrechung/Resume** | 2026-10-13 | (mountain 339).
- **„bitte bei ratsfragen immer noch recherche und vollen max roster …"** | 2026-10-13 | (mountain 339) — jede Ratsfrage zuerst `archive_search --all` (Text gespeichert), dann der volle UI-Roster, nie Rat in Pro-Solo.
- **„das ist kein roster … nur die 4 selben tabs … die 2 billigsten"** | 2026-10-13 | (mountain 339) — der Roster ist die volle Seat-Liste, nicht die vier Frontier-Tabs.
- **„die letzte session schreit Kritik angenommen … Vier Tabs sind kein Roster"** + „Roster-Antworten abholen und den Riss schließen" | 2026-10-13 | (mountain 340) — der Roster wird vollständig abgeholt; der Rat↔Roster-Riss wird benannt und am Kontrakt geschlossen.
- **„wie oft soll ich euch noch sagen dass ihr die modelle auf max stellen sollt … mycelium [hat es] haar klein dokumentiert und euch in die vorgaben geschrieben und dass qwen und glm auch über tryingopen erreichbar sind"** | 2026-10-13 | (mountain 340) — **Search/Max ist die Pflicht vor dem Senden, verifiziert** (`docs/concepts/ui-seats.md:206`); Qwen (Qwen3.8-Max) und GLM über `tryingopen` erreichbar; der Default-Modus ist ein Messfehler, keine Messung.

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch",
2026-10-07) trägt Commit und Push. In diesem Atom (folge340) committet: diese Übergabe +
konsumierte folge339 → `archiv/`; `phi/sources.φ` (ABK-field-Schlüssel + `register_sort`) ·
`src/archivar/camera_angles.rs`.
