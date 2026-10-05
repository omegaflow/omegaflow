<!--
  title: Handover — Mountain-Folge 237 (2026-10-05)
  session: Mountain-Folge 237
  class: handover
  date: 2026-10-05
  sha256: 8917514aee42903c7de0faa8c267c1552f2282f1a1b9f083d9ae87a17d345675
  status: live
-->
# Handover — Mountain-Folge 237 (2026-10-05)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`, gelesen
2026-10-05). Diese Session konsumierte `handover-2026-10-05-mountain-folge236.md`
(→ `archiv/`) und faltete die adressierten Blöcke `future-folge181`,
`mycelium-folge233`/`-234` und `river-folge94`/`-95` in **einem** Pass
(flash-first: Line + 5 grind-flash, kein pro/max). `phi/sources.φ` ist per
`register_sort --write` kanonisch (2623 Blöcke, 0 ttl-/url-Verletzungen; löst
`ci-gate 37327990225` und die mycelium-234-Meldung). `bin/.tools_ensure`-Werkzeugstand
frisch; `git_safety --snapshot` am Anfang, `--close` vor dem Commit.

## Burn: open 0.0 · close 0.4200 · cap 0.5 Grund: Runde flash-first — Line + 5 grind-flash (größte: netcdf 0.089, register 0.082, presence 0.059, arms 0.049), kein pro/max-Default

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„verschleppen und nicht eigenes ist verboten" | 2026-09-30 | Operator (Session, Mountain 213)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-02 | Operator (Session, Mountain 225)
„1 ja bitte" — privater TE-Pfad, Lauf lokal/silent, nie CI | 2026-10-02 | Operator (river-folge82)
„ja bitte" — `descoped` aus `blocked_sources.φ` auflösen | 2026-10-03 | Operator (Session, Mountain 229)
„kannst du dich bitte darum kümmern? 9 blocked parser-def" — als Weberin-zweite-Linie führen | 2026-10-03 | Operator (Session, Mountain 229)
„fixe die aktuellen Medizinische Datenquellen aber setze den rest auf on hold" | 2026-10-04 | Operator (Session, Mountain 230)
„Macht EFD/HPM/SCM Sinn? — Ja." | 2026-10-04 | Operator (Session, Mountain 230)
„also bitte alles umsetzen ich möchte nicht dass du etwas in die nächste runde nimmst was jetzt von agenten bearbeitet werden kann" | 2026-10-04 | Operator (Session, Mountain 232)
„braucht es dafür wirklich pro?" — flash-first; der Katalog-Rest per flash geschlossen | 2026-10-04 | Operator (Session, Mountain 232)
„braucht es pro?" — flash-first bestätigt | 2026-10-04 | Operator (Session, Mountain 233)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-05 | Operator (Session, Mountain 235)
„was fehlt hast du in die secrets local geschaut?" — vorhandene Keys nutzen; kein „Operator-Hand" ohne Messung | 2026-10-05 | Operator (Session, Mountain 234)
„braucht es max?" — flash-first erneut bestätigt; ExoMars-Parser per grind-flash gebaut | 2026-10-05 | Operator (Session, Mountain 235)
„braucht es pro und kannst du dir das bitte ansehen?" — flash-first; die zwei Punkte-Listen gegen den Baum messen | 2026-10-05 | Operator (Session, Mountain 235)
„messe nochmal den aktuellen zustand dann commit" — Atom 2: neue adressierte Blöcke falten, FMI-GIC-fein-grain bauen, em-Apertur messen, committen | 2026-10-05 | Operator (Session, Mountain 235)
„braucht es pro und max?" — flash-first bestätigt: alle Kanal-/Serien-Arme per grind-flash/general geschlossen, kein pro/max | 2026-10-05 | Operator (Session, Mountain 236)
„bitte gib das dem rat den tauchern für wissenschaft und forschung und den 3 online stimmen" — Contract-Frage (Sentinel vs. Presence-Bit) an Rat + research-max + UI-Stimmen | 2026-10-05 | Operator (Session, Mountain 236)
„brauchen wir überhaupt pro für den rat/council … in dateien steht veraltet wann pro angebracht ist" — Council → flash/low; pro/max nur noch Eskalation nach gemessener flash-Fehllage | 2026-10-05 | Operator (Session, Mountain 236)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, kein Consent-Stopp für Bekanntes" | 2026-10-05 | Operator (Session, Mountain 237)

## Offen (aufgeschlüsselt)

### GODAS + GISTEMP Vulkan-AOD — Format-Reader + Register-Zeile
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05 via grind-flash) Substrat erweitert:
  `units.rs:540 cf_time_unix_seconds`, `netcdf.rs:522 values_numeric` + `:576 fill_value`,
  `opendap.rs:1004 DapFile::fill_value`; zwei Harvest-Arme gebaut + `--selftest` grün:
  `tools/harvest/src/bin/gistemp_aod_compiler.rs` (Sato-Lacis CDF-1, 1956 Monate
  cos-lat-gewichtet, sha `1ba4e901…`) und `godas_pottmp_compiler.rs` (DAP2-ASCII, 12
  Monate, 300–303 K, sha `c2448a10…`); `cargo check` 0/0.
- **Blockade:** `format gistemp_aod550_axis_value_text`/`godas_pottmp_axis_value_text`
  haben keinen Archivar-Reader (`axis_value` in `src/` = 0×)
- **Braucht:** `extract.rs`- + `geo.rs`-Arm für die Zeitreihen-Formate, dann die zwei
  `sources.φ`-Zeilen (url/format/origin/compiler/at/ttl/field/sha256).

### Exposom-Quellenmatrix — Domänen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05) `exposom` in `phi/*.φ` = 0; die zwei gebauten Arme
  sind registriert (`eea_noise` `sources.φ:651`, `wqp_result` `sources.φ:677`, Felder
  `:7155/:7163`). Die 16-Klassen-Matrix liegt nur privat
  (`state/future/exposom-matrix-2026-10-04.md`), nicht committed.
- **Blockade:** keine committed Host/Domain-Liste
- **Braucht:** die 16-Klassen-Matrix als committed Liste (future-Linie); dann
  Register-Zeilen je Klasse mit deckendem Arm.

### em-Apertur — Rest (TNS-τ; ≥25-Felder-Riss)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05 via grind-flash) Rats-Verdikt umgesetzt: drei
  NED-`z`-Felder auf `inverse-linear` (Kernel 6) — `sources.φ:7090`, `:9718`, `:9729`.
  `:1183` TNS trägt `τ 3600` (Rats-Verdikt: τ aus Prozesswissen oder statische CSV
  messen); `sources.φ` trägt **≥25** em-Felder mit `z`-Wert (nicht sieben).
- **Blockade:** TNS-τ ungemessen (API vs. statische CSV)
- **Braucht:** keyed POST der statischen TNS-CSV messen ODER `τ` aus Prozesswissen
  benennen; den ≥25-Felder-Riss an river/Rat (`spatial.rs:718` Wirkung).

### trishuli (DHM Nepal) — Parser-Arm
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05) keyless HTML, Pegel m 10-min; Ad-hoc-Parser
  `tools/gate/src/bin/livefeed_gate.rs:1852 parse_dhm_stage`; kein Archivar-`format`-Arm;
  als `blocked parser-def gap html-parser-arm` registriert (`blocked_sources.φ`).
- **Blockade:** `format`-Arm fehlt
- **Braucht:** `format trishuli_stage` + HTML-Arm in `extract.rs`, dann Register-Zeile.

### MTG-LI (EUMETSAT) — Arm
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05) Landing `data.eumetsat.int` 206/2745 B HTML;
  `EUMETSAT_KEY`/`EUMETSAT_SECRET` vorhanden (Schlüsselnamen gelesen, nie Werte);
  kein `eumetsat`-Arm/Workflow; `pending` in `blocked_sources.φ`.
- **Blockade:** Data-Store-API-Route + Arm ungemessen
- **Braucht:** Data-Store-API-Route messen (`archive_search --verdict`), Arm bauen.

### GOES-18 ABI — Arm + goes16-Block-Riss
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05) Bucket-Root `noaa-goes18.s3` 200 (486709 B
  S3-Listing), `ABI-L1b-RadC/` 404 (S3-Prefix-Semantik); `goes_abi`-Arm nur
  goes16/goes19; **Riss:** der goes16-Block (`sources.φ:859`) trägt
  `origin …noaa-goes19.s3… then goes_abi_compiler`; kein goes18-Compiler; `pending`
  in `blocked_sources.φ`.
- **Blockade:** S3-Objekt-Key-Route + Arm
- **Braucht:** Objekt-Key-Route messen, goes18-Compiler (Muster goes16) bauen,
  goes16-Block-Riss beheben.

### IOC Sea Level API v1 — Konto
- **Status:** eigen | **Bindung:** eigen (Vorbereitung)
- **Trigger:** keiner
- **Lage:** (gemessen 2026-10-05) `api.ioc-sealevelmonitoring.org/v1` HTTP 403 direkt
  + Proton (239 B); kein IOC-Key in `.secrets.local` (nur Schlüsselnamen gelesen);
  keyless `service.php` steht `sources.φ:1138`; `blocked account` in `blocked_sources.φ`.
- **Blockade:** Konto/Token
- **Braucht:** Vorbereitung bis zur Kante ist getan; der **Akt** (Konto/Key) ist
  operator-gebunden — lebt in Futures Operator-Queue.

## Träger (Prosa, eigene)

- `docs/surveys/survey-2026-10-03-medizinische-datenquellen.md` (`class: survey`, Header-sha `feb28078…`) — Pool disponiert (2 admit / 3 hold / ~97 declined).
- `docs/specs/sources-v2-spec.md` (`class: concept`, Header-sha `11c6db3c…`) — `range`-Zeile trägt den Rat-Befund (kein Wire-Slot).
- `docs/surveys/survey-raetsel-bestand.md` (`class: survey`, Header-sha `524d61dc…`).
- `docs/blatt/blatt-pioneer-floor-falsifikation.md` (`class: sheet`, Header-sha `5bb1696b…`).
- `docs/concepts/kybernetische-astrophysik.md` (`class: concept`).
- `docs/concepts/tools-map.md` (`class: concept`).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` (`class: survey`).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` (`class: survey`, Header-sha `ac672e3e…`).

## An mycelium

Origin: mountain folge237 (faltet mycelium-folge233/234).

- **FMI-IMAGE-Magnetometer NUR** — Arm + Register gebaut (`sources.φ`, `format
  fmi_image_mag`, `image_mag_compiler.rs`). Braucht: `image-cdn`-Workflow (Muster
  `fmi-gic-cdn.yml`) + `gh workflow run`; Asset `fmi_image_mag_nur.bin` auf Release
  `space.fmi.fi`.
- **IERS EOP C04 LOD** — Arm + Register gebaut (`iers_eop_c04_compiler.rs`, `format
  iers_eop_c04_lod`, `COMP_IERS_LOD`). Braucht: CDN-Manifestation `iers_eop_c04_lod.bin`.
- **gbco axis-value Serie** — `gbco_series_compiler.rs` gebaut. Braucht: Manifestation
  `gbco_elevation_gebco_{2023,2026}.txt` (`data.ceda.ac.uk-gebco`), dann `witnesses.φ`
  URLs tauschen.
- **EMM `emm_exi_l2a`** — Register-Block jetzt in `sources.φ` (`format emm_exi_l2a`,
  `at mars`, `ttl`; **kein `field`** — Rat: L2a = Bildquelle ohne Skalarfeld).
  Braucht: `origin`/`compiler`/`sha256`, `emm-sdc-cdn.yml`-Angleiche, Frame-Bundle-Arm.
- **Neue CDN-Assets ohne vollständigen Register-Block:** `clpds_annex.jsonl`
  (`clpds.bao.ac.cn`, sha `c7ddec83…`) fehlt; supermag-1999-Serie
  (`supermag_*_1999-01-01_31d.bin`) fehlt; openneuro ds004100-iEEG-Assets fehlen
  (99 `openneuro_pd_eeg`-Blöcke tragen kein `sha256`). `url`/`sha256`/`compiler` = deine Zeile.
- **superdarn-Tag** auf `pending` (`wartend.φ:8`); RST-Ernte über Globus offen.

## An river

Origin: mountain folge237 (faltet river-folge94/95).

- **Enclosure-Hülle — Mountain-Seite behoben.** `law_bounds_over_span` neu
  (`spatial.rs:276-321`): exakte Spann-Obergrenze (`v² = GM(2/r−1/a)` monoton in r;
  Periapsis im `64·ttl`-Fenster wird erfasst), `build_asteroid_samples` ruft sie statt
  `law_bounds` (`spatial.rs:449-452`). Test `tests.rs:3474` grün: alle 9 DASTCOM-Records
  `v_span <= v_hull/Φ`; Icarus/Phaethon als die zwei gebrochenen Fälle belegt.
  Bitte prüfen/übernehmen (Query-Horizont `64·ttl`, `spatial.rs:863`). Der Stern-Pfad
  (`spatial.rs:665`) bleibt bei `law_bounds` — nur `build_asteroid_samples` spannt.
- **absorption/advection — Archivar-Seite gebaut.** `PRESENCE_FLAG_{PHASE,ABSORPTION,
  ADVECTION}=1/2/4` (`types.rs:306-308`), `slot_or_pad`/`presence_flags`
  (`types.rs:328-346`), Emission gesetzt (`spatial.rs:823/842/973/992`,
  `archivar/membrane.rs:112/127`), `absorption_for_force` gibt absent als `SLOT_ABSENT`
  (`channels.rs:1229`); `SLOT_ABSENT` reitet **nie** den Draht. Vertrag in
  `docs/concepts/archivar-mathematikerin.md:53`. **Deine Seite:** WGSL liest Bit 0/1/2
  (`shaders.rs:344/387/187/213/128`), `ADVECTIVE_BASE_SPEED = 1.0` fällt
  (`archivar/membrane.rs:424/491/767/789`). Danach Mountains Register-Migration der
  9059 Zeilen (Standard absent).
- **main_flow:** Loader-Token `"fmi_image_mag"` in `src/archivar/main_flow.rs:4118`
  ergänzt (mechanisch) — dein Pfad, bitte prüfen/übernehmen.
- **declustered Mainshock:** `erbq_mainshock` (`witnesses.φ:180-184`, 153/198) ist
  unverdrahtet an die ETA-Form; Compiler reproduziert es objektid-identisch.
- **em-Apertur-Riss:** Apertur erreicht ≥25 em-Felder mit `z` (`sources.φ:8814-8820`),
  nicht die sieben; beide Linien benannt, nicht geglättet.
- **goes_euvs Tag-Riss:** Probe `ssd.jpl.nasa.gov/goes_euvs.bin` vs Register
  `ncei.noaa.gov/goes_euvs.bin` (`sources.φ:24775`).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern
  (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI.
  Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün;
  offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz. Der
  private Wort-Laut nur im privaten `state/operator-gespraeche/`.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und
Push. `/consent` ist der session-weite Consent, nie das Commit-Wort.
