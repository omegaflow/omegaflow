<!--
  title: Handover — Mycelium-Folge 161 (2026-09-25)
  session: Mycelium-Folge 161
  class: handover
  date: 2026-09-25
  sha256: 10a0e10e9553944b7f18763e14e15845c07d9054ee2c30a06fb14811200f59cb
  status: live
-->
# Handover — Mycelium-Folge 161 (2026-09-25)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was gemacht
wurde. Geteilter externer Zustand lebt in `docs/zustand/external-state.md`, nie als
Kopie hier. Keine Rangfolge, kein „härtester Punkt". Sortierung: erst Akteur
(**Linie** | **Rat** | **Operator** | **Dritter**), dann chronologisch nach
`Lage`-Datum. Jeder Punkt aufgeschlüsselt: **Trigger** / **Lage** / **Blockade** /
**Braucht**. Status-Tag: `autonom` | `operator-gebunden` | `blockiert` | `wartend` |
`termin` | `LOCK`. Eine Zeile ohne externen Trigger ist ungültig — `nächster
Dispatch`/`nächste Session` ist kein Trigger (der lesende Lauf IST der nächste);
Regel in AGENTS.md, Gate-Fixture `commit_gate_vocab.json::deferral_markers`.

Diese Session konsumierte `handover-2026-09-25-mycelium-folge160.md`.

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### DECaPS-Loader-Arm
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 via grind-max) `phi/sources.φ` trägt
  `decaps_dr2_stars.bin` (format decaps_dr2_stars, ttl 31536000, origin Dataverse
  K88GFI); `src/archivar/geo.rs` `magic_of`/`comp_max` hat keinen Arm, und ein
  GeoRec-Arm wäre falsch: der Record ist 56 B (ra/dec f64 + 10×f32; 16-B-Header
  `0xCF860500`+count+rec_bytes+reserved), ohne Zeitachse/`t` — `parse_bin`
  (60-B-GeoRec) fehlparst.
- **Blockade:** keine.
- **Braucht:** `src/archivar/decaps.rs` (gaia_sso-Muster) + Dispatch in
  `main_flow`/`extract.rs` + Magic-Registrierung; dann gemessen manifestieren.

#### TOAR-Komponenten-Arm
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 via grind-max) `geo.rs` `MAGIC_TOAR`/`COMP_TOAR_O3`
  stehen; `src/archivar/extract.rs::geo_series_component_name` hat keinen
  `"toar_surface_o3"`-Arm → `field toar_surface_o3_ppb` matcht keinen Kanal; der
  Block lädt, manifestiert aber nichts.
- **Blockade:** keine.
- **Braucht:** 3-Zeilen-Arm `"toar_surface_o3"` in `extract.rs`.

#### Manifestations-Hashes (TOAR · Zenodo · DECaPS)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** CI-Manifest-Lauf (kernel-flatten / CDN).
- **Lage:** (gemessen 2026-09-25 via grind-max) Zenodo `data.zip` 23 506 041 274 B —
  kein Stream-sha256 (Record trägt nur md5 `86b37400…`), Block ohne sha256-Zeile;
  TOAR sha256 `8fd55224…` nur 5-Serien-Sample (IDs 1000–2000 ersetzt); DECaPS
  Partial-Hash bewusst nicht eingetragen.
- **Blockade:** Manifest-Lauf.
- **Braucht:** CI-Full-Range-Manifest trägt die Hashes nach (Register-Duty bei der
  Manifestation).

#### gap-Klassen-Träger fortschreiben
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (eigener Schritt).
- **Lage:** (gemessen 2026-09-25 via sgrep) `gap konverter 4→0` geschlossen: 4
  Gaia-Blöcke portiert (`field phot_g_mean_mag gaia_dr3_g_mag inverse-square em mag
  604800 0.0 0.0`); RR Lyrae + cluster_ka als `gap curation` zurück (HTTP 400,
  Spalte fehlt); live: `unit-auto-detect ×168`, `force-undetermined ×16`,
  `curation ×17`; gap-Token-Kanon (Header-Notizen) steht.
- **Blockade:** keine.
- **Braucht:** die `unit-auto-detect`-/`force-undetermined`-Klassen je Klassen-Träger
  `phi/blocked_sources.φ::gap:<token> ×N` weiterführen (Port-Kandidaten einzeln
  messen).

#### Voyager-Okkultation (D1)
- **Status:** autonom | **Bindung:** eigen
- **Trigger:** sofort (Spec gemessen).
- **Lage:** (gemessen 2026-09-25 via research-max) Kollektion PSPA-00217;
  `FORMAT_IDENTIFIER NSSD1395`, `MACHINE_REPRESENTATION Data General Eclipse`,
  `RECORD_FORMAT variable`, `STREAM_RECORD_DELIMITER 2-BYTE HEADER`,
  `MAXIMUM_RECORD_LENGTH_BYTES 4098`; Frame 122 B + 3908×4098 B = 16 015 106 B je
  `.DAT`; 120-B-Metadatum Byte-Map (F1/F2) gemessen; Zeit-Tag-Slot („time of first
  sample…") ohne Offset.
- **Blockade:** Zeit-Tag-Encoding ohne Spec-Text (DSC_0629-Scan als PDF, Text
  ungemessen).
- **Braucht:** Vision-OCR der DSC_0629-Seiten ODER NSSDC-CRUSO-Anfrage; dann
  `src/archivar/voyager_occlt.rs` + Magic + `voyager_occlt_compiler.rs`.

#### mycelium-ORPHANs (10)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `tools-build` Lauf `36161207629` success (dispatcht 2026-09-25).
- **Lage:** (gemessen 2026-09-25 @`93c4bb773` via `register_lookup --orphans`) 10
  ORPHAN_COMMITTED in `phi/blocked_sources.φ` (Zeilen driften:
  31/36/41/45/49/53/57/61/74/78); Release-Artefakt hinkt HEAD.
- **Blockade:** Lauf-Ergebnis.
- **Braucht:** bei success `register_lookup --orphans`; je Eintrag Träger /
  `gap`-Direktive.

#### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check` am HEAD.
- **Lage:** (gemessen 2026-09-25) der HEAD-Lauf `36132607670` ist pending; der
  jüngste abgeschlossene rote `36124590241` @`1bebe6dad` liegt **vor** Mountain
  `c4592e347` — überholt.
- **Blockade:** CI-Lauf.
- **Braucht:** `ci_manage view <id>`; bei Rot `ci_manage log <id>`.

#### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ps1_dr2_coverage.fp01` erreicht.
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) Upload-URL
  `.github/workflows/ps1-cdn.yml:200` direct 404 / proton 404 / Wayback kein
  Snapshot → absent.
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** Re-Messung bei Ernte-Fortschritt; dann Note finalisieren.

#### Fremdmodell-Benchmark
- **Status:** blockiert | **Bindung:** eigen
- **Trigger:** Browser-Target verbunden (`browser_targets` nicht leer).
- **Lage:** (gemessen 2026-09-25 via `browser_targets` = `[]`) Rekord
  `docs/surveys/survey-2026-09-24-fremdmodell-bedienung.md`.
- **Blockade:** kein Browser-Target.
- **Braucht:** Browser verbinden, dann Benchmark fahren.

**Geschlossene Benchmark-Klasse (Operator-Wort 2026-09-25):** Register-/Konverter-Port
— `grind-flash` gegen `grind-max`: der max-Arm registrierte 4 Gaia-Quellen live, die
flash-Messung fand 2 davon als HTTP 400 (Spalte fehlt); Sieger `grind-flash` (billiger,
fand den Defekt). Die 2 sind als `gap curation` zurückgeführt.

#### Fehlende Mess-Serien (abk_dbdt_1h, kegel, GIC/corona)
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-03-daten-holdings-inventur.md) Die Lokalisierung unter allen Holdings ist FEHLGESCHLAGEN — als Datei nirgends vorhanden (nur ein Verdict-Report `knowledge/archive/reports/report-09-signalkegel…`); lokal `pending`, dauerhafte Heimat wäre das CDN.
- **Blockade:** keine.
- **Braucht:** `register_lookup`/`archive_search --verdict` auf die Serien gegen das CDN; bei Abwesenheit Eintrag in `phi/blocked_sources.φ`.
- **Quelle:** docs/surveys/survey-2026-09-03-daten-holdings-inventur.md

#### 55 undocumented stale_pending Netlocs
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-03-orphan-verdicts.md) Von 156 Orphan-Releases sind 135 `stale_pending`, davon 55 in keinem Register; Erreichbarkeit ist der Vorfilter, die Disposition je Netloc offen.
- **Blockade:** keine.
- **Braucht:** je Netloc Force-Gate nach `docs/SOURCE_PORT.md` → `sources.φ`-Block oder `dead_sources.φ`-Eintrag.
- **Quelle:** docs/surveys/survey-2026-09-03-orphan-verdicts.md

#### Step-4 CI-Dedupe neu fassen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-03-orphan-verdicts.md) Step 4 (CI-Dedupe) ist gegen die gemessene Job-Zahl (health-check 4, kernel-flatten 18) neu zu fassen.
- **Blockade:** keine.
- **Braucht:** Step-4-Plan gegen die gemessenen Job-Zahlen neu fassen.
- **Quelle:** docs/surveys/survey-2026-09-03-orphan-verdicts.md

#### NOAA-NRS passive-bioacoustic — Quell-Entscheid
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-07-tmp-opencode-scan.md) NOAA-NODD NRS `sound_level_metrics`/`daily.nc` ist unregistriert und trägt keinen eigenen Compiler; Quell-Entscheid `pending`.
- **Blockade:** keine.
- **Braucht:** Quell-Entscheid → eigener `tools/harvest`-Compiler, dann `sources.φ`-Registrierung.
- **Quelle:** docs/surveys/survey-2026-09-07-tmp-opencode-scan.md

#### dead_sources.φ — 157 klassenlose Einträge
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md) In `phi/dead_sources.φ` tragen 157 Keyword-Einträge keine Klasse (Keyword mit trailing space, z. B. `:501` `decline `, `:2561` `dead `) — Parser-Lücke, kein Wert.
- **Blockade:** keine.
- **Braucht:** die 157 Einträge klassifizieren bzw. die Parser-Lücke schließen.
- **Quelle:** docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md

#### NOIRLab Astro Data Lab TAP — Wiedervorlage
- **Status:** termin:2026-12-02 | **Bindung:** termin
- **Trigger:** 2026-12-02.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md) Reg. abgelehnt, TAP öffentlich; `phi/blocked_sources.φ:32`; Wiedervorlage 2026-12-02.
- **Blockade:** Termin.
- **Braucht:** am 2026-12-02 Reg.- und TAP-Stand re-messen.
- **Quelle:** docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md

#### UNIVAC-1108-Parser (Voyager Saturn-TARs)
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md) Voyager closed-loop überlebt in den Saturn-Encounter-Daten (UNIVAC-1108-Binär, closed-loop Doppler+Range: V1 `PSPA-00049`, V2 `PSPA-00123`, SPDF 200); der Parser ist offen.
- **Blockade:** keine.
- **Braucht:** UNIVAC-1108-Parser für die Saturn-TARs bauen.
- **Quelle:** docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md

#### Force-Kanal Re-Check (remon.jrc, irsn.fr)
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-16-dead-sources-relevanz.md) 3 Force-Kanal-Kandidaten behalten (Re-Check-Pflicht): `remon.jrc.ec.europa.eu` (em) und `www.irsn.fr` (×2, em).
- **Blockade:** keine.
- **Braucht:** die 3 em-Quellen als `sources.φ`-Blöcke bauen oder das Verdikt belegen.
- **Quelle:** docs/surveys/survey-2026-09-16-dead-sources-relevanz.md

#### dead_sources Pending-Rest nachmessen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-16-dead-sources-relevanz.md) 4 Pending-Fälle behalten: `dods.wh.gov` (Akustik-Pfad), `osdr.nasa.gov` (ISS-Dosimetrie), `pskreporter.info` + `reversebeacon.net` (Amateurfunk-Propagation).
- **Blockade:** keine.
- **Braucht:** `dods.wh.gov` Nachfolger des OPeNDAP-Acoustic-Endpoints suchen; direkten Dosimetrie-Feed in `osdr.nasa.gov` messen; `pskreporter.info`/`reversebeacon.net` gegen die sources.φ-Ionosphären-Abdeckung wiegen.
- **Quelle:** docs/surveys/survey-2026-09-16-dead-sources-relevanz.md

#### arvo-registry.sci.am — Proton-Eskalation
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-16-dead-sources-relevanz.md) `arvo-registry.sci.am` (×2) ist `dead unreachable` (SOURCE_PORT §16.4); die Proton-Eskalation ist offen.
- **Blockade:** keine.
- **Braucht:** `archive_search --verdict <url>` (Proton-Stufe) fahren.
- **Quelle:** docs/surveys/survey-2026-09-16-dead-sources-relevanz.md

#### sources-Repo klonen und messen
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-17-verlorene-diskussionen.md) `omegaflow/sources` (I02/`refresh.yml`) ist lokal nicht geklont; dort entscheidet sich, ob der 5-min-Takt lebt und ob die I02-Python-Behauptung stimmt.
- **Blockade:** keine.
- **Braucht:** `omegaflow/sources` klonen und `refresh.yml`/I02 messen.
- **Quelle:** docs/surveys/survey-2026-09-17-verlorene-diskussionen.md

#### Atom D — phase/presence-Konsum
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort (Rat 2026-09-23, Route B).
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-17-verlorene-diskussionen.md) Bau-fähig; vier phase-tragende Klassen am CDN (`cassini_rsr` I/Q `sources.φ:8081`, cassini/maven TNF `:8099/:8819`, fdsn BHZ `:110`); die Slots fahren seit v9 mit, nichts liest sie; das Beat-Paar bleibt pending.
- **Blockade:** keine.
- **Braucht:** Producer schreibt `phase: Some(fract(cycles)·2π)` + `freq=ramp_freq` + `bin_width=0.0` auf der TNF-Route; WGSL-Beat-Term für ein Paar; ein Atom, `grind-max`.
- **Quelle:** docs/surveys/survey-2026-09-17-verlorene-diskussionen.md

#### HRV/ESP32-Puls-Bindung
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-17-verlorene-diskussionen.md) Das RMSSD/tone-Gate steht in `src/archivar/hrv.rs`; die Bindung an den Radiation-Pfad ist pending.
- **Blockade:** keine.
- **Braucht:** das hrv.rs-RMSSD/tone-Gate an den Radiation-Pfad binden.
- **Quelle:** docs/surveys/survey-2026-09-17-verlorene-diskussionen.md

#### field absorption per force_type
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-17-verlorene-diskussionen.md) `src/mathematikerin/shaders.rs:53` blendet nur Kernel 5; jeder andere Kernel ignoriert `absorption`; ein per-force_type-Absorptionsgesetz ist ungebaut.
- **Blockade:** keine.
- **Braucht:** ein per-force_type-Absorptionsgesetz bauen.
- **Quelle:** docs/surveys/survey-2026-09-17-verlorene-diskussionen.md

#### row-parallel TE-Re-Shape / WGSL-FFT
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-17-verlorene-diskussionen.md) Die benannte Alternative ist offen: row-parallel re-shape (ein Thread per t — Ringwachstum) bzw. WGSL-FFT.
- **Blockade:** keine.
- **Braucht:** die benannte Alternative bauen.
- **Quelle:** docs/surveys/survey-2026-09-17-verlorene-diskussionen.md

#### Doku-Behauptung ≠ Baum korrigieren
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort (5-min/I02 erst nach der sources-Repo-Messung entscheidbar).
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-17-verlorene-diskussionen.md) „CI Archivar runs every 5 minutes" (3 Live-Doku-Stellen + `fetch.rs:900`), `biotic`-Präsens-Erzählung (`methodology.md:23`), `remove-bias.md`-Plan referenziert totes `warm_cache`, `kernel-curation`-Versionszitat v6 — Doku ≠ Baum.
- **Blockade:** keine.
- **Braucht:** die 3 Doku-Stellen + `CI_REFRESH_S` auf die gemessene Wahrheit korrigieren (oder bauen); die `biotic`/`remove-bias`/`kernel-plan`-Zeilen korrigieren oder archivieren.
- **Quelle:** docs/surveys/survey-2026-09-17-verlorene-diskussionen.md

#### Deep-Sky-Pfad (richtungsbasierte Lieferung, Upload-Stille, Relay-Trailer)
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-fortschritt.md) Offene Verbesserungen: Deep-Lieferung richtungsbasiert (Stern über den Sichtkegel statt radius-begrenzt), Deep-Upload-Stille (`deep_dirty` feuert bei jedem Sense — die 29-MB-Sterne werden auch unverändert hochgeladen), Relay-Trailer (gen u64 + 9×Ω f64 für `browser_relay`, ~80 B).
- **Blockade:** keine.
- **Braucht:** richtungsbasierte Deep-Lieferung, die `deep_dirty`-Stille und den Relay-Trailer bauen.
- **Quelle:** docs/surveys/survey-fortschritt.md

#### Membran-Messachse (Zell-Achse, Rgba8Unorm, Fovea-Kappe)
- **Status:** offen | **Bindung:** eigen
- **Trigger:** sofort.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-fortschritt.md) Offene Verbesserungen: Zell-Achse (Messpunkt-Vergröberung gegen die 8-Bit-Display-Quantisierung; `survey-auswertung.md` §1-2), Rgba8Unorm-Nachmessung mit `intel_gpu_top` gegen die ~200-ms-Baseline, Fovea nur als Budget-Kappe.
- **Blockade:** keine.
- **Braucht:** Zell-Achse und Rgba8Unorm nachmessen; Fovea als reine Budget-Kappe führen.
- **Quelle:** docs/surveys/survey-fortschritt.md

### Operator

#### api.sensor.community — Operator-Exit-Wort
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zum Exit/Route.
- **Lage:** (gemessen 2026-09-25 via grind-flash)
  `api.sensor.community/v1/data/measurements` direct 403 / proton 403 (ip-blocked).
- **Blockade:** IP-Blockade; ein Exit-Wechsel berührt Terms/§ 95a UrhG.
- **Braucht:** Operator-Wort; danach `archive_search --verdict` erneut.
- **Vorbereitung (autonom, erledigt):** `bin/proton-wg.sh suggest api.sensor.community`.

#### Sicherungs-Archiv-Layout (knowledge/ + backups/, ~50 G)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort zur Ziel-Layout-Entscheidung.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-03-daten-holdings-inventur.md) `knowledge/` (32 G) und `backups/` (23 G) liegen als Sicherungs-Archive in situ; nichts im Repo referenziert sie; eine Umlagerung dieser irreplacebaren Sicherungsdaten bedarf einer definierten Ziel-Layout-Entscheidung.
- **Blockade:** keine Ziel-Layout-Entscheidung.
- **Braucht:** Operator-Wort zur Umlagerung oder zum Verbleib in situ.
- **Quelle:** docs/surveys/survey-2026-09-03-daten-holdings-inventur.md

#### Lizenz-Entscheidung (PolyForm / CC BY-NC-SA)
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort.
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md) Die Sammlung steht unter PolyForm/CC BY-NC-SA — non-commercial; jede Monetarisierung beginnt mit einer Lizenz-Entscheidung.
- **Blockade:** keine Lizenz-Entscheidung.
- **Braucht:** Operator-Wort zur Lizenz-Entscheidung.
- **Quelle:** docs/surveys/survey-2026-09-16-fremde-parser-sammlungen.md

### Dritter

#### DEMETER Order 18387 (WAF, nicht Workflow)
- **Status:** blockiert | **Bindung:** dritter
- **Trigger:** F5-ASM-WAF erholt ODER Order-Ablauf 2026-09-28.
- **Lage:** (gemessen 2026-09-25) `regards.cnes.fr/api/v1/rs-order` 403 (Jetty Access
  Denied), POST 403; Lauf `35851193831` failure (8× `F5 ASM: Request Rejected`).
- **Blockade:** CNES/REGARDS F5-ASM-WAF.
- **Braucht:** Wiedervorlage; bei Erholung `gh workflow run demeter-cdn.yml`.

#### ESA LPF Legacy Archive AIO (D3)
- **Status:** blockiert | **Bindung:** dritter
- **Trigger:** ESA-Helpdesk-Antwort / Backend-Erholung.
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) `/lpfsa-sl/data-action`
  500 (87 B, „Malformed retrieval request"); Param-Route 502; Portal `/lpfsa/` 200;
  `auth_method: cas`.
- **Blockade:** Backend-Session-Fehler (ESA).
- **Braucht:** Anfrage `support.cosmos.esa.int/lpfsa/` (Vorbereitung durch
  Future/Operator).

#### src.pas TAP
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** `/tap/tables` 200.
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) `/tap` 200 (13342 B);
  `/tap/tables` 500 (PostgreSQL localhost refused); `ledger.φ:10`.
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei Erholung.

#### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Backend-Erholung.
- **Lage:** (gemessen 2026-09-25) `api.lasair.lsst.ac.uk/api` direct keine Antwort,
  proton **HTTP 500**; Frontend 200; ZTF-Zwilling `lasair-ztf.lsst.ac.uk/api/objects`
  **401** (Token nicht angehängt).
- **Blockade:** Broker-Backend.
- **Braucht:** Token-Probe mit `LASAIR_LSST_TOKEN` am ZTF; Multi-Exit-Re-Messung.

#### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** PSA-Freigabe.
- **Lage:** (gemessen 2026-09-25) TAP 200; `release_date 2099-01-01`; Produkt 403.
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

#### SuperDARN MAP
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Globus-Task `af68c4f1`-Ende.
- **Lage:** (gemessen 2026-09-25 via research-max) `superdarn.ca/data-download` 200;
  Globus-Task-API **HTTP 400** `ClientError.AuthenticationFailed`; Activity-SPA 200
  (Status hinter Login).
- **Blockade:** kein anonymer Statuskanal (OAuth2-Bearer nötig).
- **Braucht:** Task-Status via Globus-Transfer-Token/Kanal messen; bei Abschluss Note
  schließen.

#### SSDC Limadou (CSES-L2)
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur / Sotgiu-Antwort.
- **Lage:** (gemessen 2026-09-25 via `archive_search --verdict`) Portal 200;
  `query.php` anonym 302 → CAS-Login; Operator-Wort **nein** (2026-09-23).
- **Blockade:** PI-seitige Prozedur.
- **Braucht:** wartend lassen.

#### nse_haug_trisp — TRISP-NSE-Dateien
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Maileingang (MPI-FKF/TRISP sendet die Roh-/reduzierten TRISP-NSE-Dateien, „a few days").
- **Lage:** (gemessen 2026-09-25 via sread docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md) Die Route ist offen (Mail 2026-09-17, `state/mail/mail_ledger.φ`); MPI-FKF/TRISP sendet die rohen/reduzierten TRISP-NSE-Dateien direkt.
- **Blockade:** Eingang der Dateien.
- **Braucht:** bei Maileingang `nse_haug_trisp`-Quelle + Compiler + `sources.φ`.
- **Quelle:** docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md

### Gelesen — keine offene Arbeit (descoped)
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** —
- **Lage:** (gemessen 2026-09-25 via Dokument-Lesung + `register_lookup --orphan-docs`) die offenen Marker dieser Dokumente sind Prosa, kein handlungsfähiger Punkt.
- **Blockade:** keine.
- **Braucht:** — (descoped mit Befund).
- **Quelle:** `docs/concepts/docs-naming.md`, `docs/concepts/mirror-research.md`

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`); `/consent` ist der
session-weite Consent (Delegation), nie das Commit-Wort.
