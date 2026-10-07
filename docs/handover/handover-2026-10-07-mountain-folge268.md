<!--
  title: Handover — Mountain-Folge 268 (2026-10-07)
  session: Mountain-Folge 268
  class: handover
  date: 2026-10-07
  sha256: 92b132b83c30da5c733a1493ca9b6a2eb0daca4c8e96a4f3391198f140871e2a
  status: live
-->
# Handover — Mountain-Folge 268 (2026-10-07)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-07-mountain-folge267.md` (→ `archiv/`). Kein
pro/max. Gefaltet: die drei adressierten Blöcke — river-125 (CGM-Provenienz →
`cgm_source`-Arm), future-191 (KCG2 → `blocked_sources.φ pending`, SDO Dashboard →
`declined_sources.φ decline imagery`), mycelium-261 (`terms`→SPDX-Migration, 134×
`CC0`→`CC0-1.0` + 1× `ODC-BY-1.0`→`ODC-By-1.0`; `terms-vocab 0 violation(s)`).
**Taucher-Wellen (flash):** Wave 1 gebaut (`5c3ded9bc`) — KCG2-JSON-Reader,
GIC-Band-Deskriptoren + Coverage-Test, dropped-gate Träger-Ableitung, Sonne-GM
Bit 11 (`de_compiler`); Wave 2 — `span`-Direktive (Rat: `extent` reusen, kein 27.
Wire-Feld), `eionet_cdr`-Block (276 Felder, register canonical), obis.osha als
`pending parser-gap` registriert. Wave 3 — 4 GIC-/CDAWeb-Compiler (`cdaweb_tec`,
`cdaweb_roti`, `goes16_mag`, `poes19_meped`) + `*-cdn.yml`, IGRF-Modul
`src/archivar/igrf.rs` mit `geomag_lat`-Arm (Grad-1-Dipol gegen Beggan 2026
verifiziert), **13 Klassen-(a)-Baseline-Zeilen gelöscht** (Release `tools-latest`
@ `6d33a91da` gemessen fresh). Riss: Mycelium hat zwei Taucher-Dateien unter
eigener Botschaft mitgenommen (`0fc491b7e`, kc2g_stations.rs + dropped_gate.rs).

**Burn** (`session_burn`): Mountain-Linie-Session $0.0558 (`line`, deepseek-flash) + general ×1 (KCG2/SDO-Messung); kein pro/max (Aggregat deepseek-flash $0.8412/24 Sessions).

## Burn: open 0.0 · close 0.0558 line — no pro/max; general ×1 (KCG2/SDO)

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„verschleppen und nicht eigenes ist verboten" | 2026-09-30 | Operator (Session, Mountain 213)
„1 ja bitte" — privater TE-Pfad, Lauf lokal/silent, nie CI | 2026-10-02 | Operator (river-folge82)
„ja bitte" — `descoped` aus `blocked_sources.φ` auflösen | 2026-10-03 | Operator (Session, Mountain 229)
„kannst du dich bitte darum kümmern? 9 blocked parser-def" — als Weberin-zweite-Linie führen | 2026-10-03 | Operator (Session, Mountain 229)
„fixe die aktuellen Medizinische Datenquellen aber setze den rest auf on hold" | 2026-10-04 | Operator (Session, Mountain 230)
„Macht EFD/HPM/SCM Sinn? — Ja." | 2026-10-04 | Operator (Session, Mountain 230)
„also bitte alles umsetzen ich möchte nicht dass du etwas in die nächste runde nimmst was jetzt von agenten bearbeitet werden kann" | 2026-10-04 | Operator (Session, Mountain 232)
„braucht es dafür wirklich pro?" — flash-first; der Katalog-Rest per flash geschlossen | 2026-10-04 | Operator (Session, Mountain 232)
„braucht es pro?" — flash-first bestätigt | 2026-10-04 | Operator (Session, Mountain 233)
„was fehlt hast du in die secrets local geschaut?" — vorhandene Keys nutzen; kein „Operator-Hand" ohne Messung | 2026-10-05 | Operator (Session, Mountain 234)
„braucht es max?" — flash-first erneut bestätigt; ExoMars-Parser per grind-flash gebaut | 2026-10-05 | Operator (Session, Mountain 235)
„braucht es pro und kannst du dir das bitte ansehen?" — flash-first; die zwei Punkte-Listen gegen den Baum messen | 2026-10-05 | Operator (Session, Mountain 235)
„messe nochmal den aktuellen zustand dann commit" | 2026-10-05 | Operator (Session, Mountain 235)
„braucht es pro und max?" — flash-first bestätigt: alle Kanal-/Serien-Arme per grind-flash/general geschlossen, kein pro/max | 2026-10-05 | Operator (Session, Mountain 236)
„bitte gib das dem rat den tauchern für wissenschaft und forschung und den 3 online stimmen" — Contract-Frage (Sentinel vs. Presence-Bit) an Rat + research-max + UI-Stimmen | 2026-10-05 | Operator (Session, Mountain 236)
„brauchen wir überhaupt pro für den rat/council … in dateien steht veraltet wann pro angebracht ist" — Council → flash/low | 2026-10-05 | Operator (Session, Mountain 236)
„kannst du das bitte fixen? Fink … trishuli …" — Fink-Route (Messfehler) + trishuli klären | 2026-10-06 | Operator (Session, Mountain 242)
„bitte lasse darauf nochmal den schwarm los und gib mir die frage für glm und claude" | 2026-10-06 | Operator (Session, Mountain 242)
„die beiden live nachbarn wären doch als triangulierung gut?" — Dhunche 4657 + Bhorle 4661 als Trishuli-Ingestion | 2026-10-06 | Operator (Session, Mountain 242)
„go" — Fink=Quellen-Ursprung (A), FARA=Index (B): FARA streichen, Anker `at sun`, Gate-Fixture | 2026-10-06 | Operator (Session, Mountain 245)
„kannst du das noch machen?" — OSHA/JAXA/Fink-sky1 fertig bauen (Unit-Tabelle, JAXA-Arm, sky1-Messung) | 2026-10-06 | Operator (Session, Mountain 245)
„bitte umsetzen" — OSHA-Geocoder (ZCTA) bauen · JAXA-Granule verifizieren · Vlies-Felder · Exposom-Register | 2026-10-06 | Operator (Session, Mountain 246)
„da sind doch viel mehr stimmen offen" — die Kp-Operatorfrage dem Rat UND den offenen Browser-Chat-Stimmen vorlegen | 2026-10-06 | Operator (Session, Mountain 246)
„du kannst claude nochmal versuchen" — Claude (Sonnet 5.5) als letzte Frontier-Stimme einholen | 2026-10-06 | Operator (Session, Mountain 246)
„bitte umsetzen" — Kp-Entscheidung: kein Peer-Feld/Treiber; Register-Riss binden, Wege benennen | 2026-10-06 | Operator (Session, Mountain 246)
„bitte setze die drei arme um" — die drei serienlosen Vlies-Arme am `.bin` wiren | 2026-10-06 | Operator (Session, Mountain 247)
„bitte setze das um:" — die offenen Punkte der eigenen Übergabe in einem Pass | 2026-10-06 | Operator (Session, Mountain 247)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251/252/253/255/256/257/259/260/262/263/264/265/266)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251/252/253/255/256/257/260/262/263/264/265/266)
„bitte gib die ratsfragen auch den 12 stimmen (mit 5 stimmen und 5 axiomen)" — die zwei Architektur-/Ethik-Fragen (eionet-Kraftmedium, GIC-Familien-Deskriptoren) an 5 API-Stimmen + 5 UI-Chats + 2 Open-Weight, gerahmt mit 5 Stimmen + 5 Axiomen + 5 Achsen | 2026-10-07 | Operator (Session, Mountain 259)
„bitte warte die antworten ab" — die UI-Antworten abwarten, nicht nach dem Send abbrechen | 2026-10-07 | Operator (Session, Mountain 259)
„sage kimi weiter und sende glm und gib die frage nochmal sonnet 5.5max" — Kimi fortsetzen, GLM/z.ai senden, Sonnet 5.5 Max erneut | 2026-10-07 | Operator (Session, Mountain 259)
„glm ist fertig" — die GLM/z.ai-Antwort ist eingetroffen und wird nachgelesen | 2026-10-07 | Operator (Session, Mountain 259)
„nein ich meinte claude sonnet 5.5 max ich habe es abgeschickt" — der Operator hat die Frage selbst an Claude Sonnet 5.5 Max gesendet | 2026-10-07 | Operator (Session, Mountain 259)
„immer noch <url>" — der neutrale umgeformte Prompt wird weiterhin `[bio]`-markiert; die Schadstoff-/Masse-Rahmung wird vollständig neutralisiert, die Fassung passiert den Klassifikator | 2026-10-07 | Operator (Session, Mountain 259)
„qwen und duck sind da und nemotron habe ich in tryingopen nochmal laufen lassen" — die drei dropped-gate/Carrier-Antworten sind da | 2026-10-07 | Operator (Session, Mountain 259)
„warte ich habe die richtige frage nochmal gestellt" — der Operator hat qwen/duck/tryingopen die korrekte eionet/GIC-Frage erneut gestellt | 2026-10-07 | Operator (Session, Mountain 259)
„ja bitte beides" — MiMo V2.6 Pro auf tryingopen ansetzen + Kimis Q2 nachholen | 2026-10-07 | Operator (Session, Mountain 259)
„gib mir nochmal die frage" — die Ratsfrage erneut ausgeben (neutralisierte Fassung) | 2026-10-07 | Operator (Session, Mountain 259)
„ja gib sie kimi k3 nochmal" — die Frage erneut an Kimi K3 auf tryingopen senden | 2026-10-07 | Operator (Session, Mountain 259)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„ich meine die architektur ethik und recherche methodik, ich möchte dass du die offenen punkte so untersuchen lässt" — offene Punkte durch die Architektur-/Ethik-Linse (Rat) und die Recherche-Methodik (`archive_search`) untersuchen | 2026-10-07 | Operator (Session, Mountain 264)
„und dann nimm die ui chats dazu (die tabs sind alle offen)" — den zweiten Kanal (UI-Frontier) je Tab als eigene Stimme mitnehmen | 2026-10-07 | Operator (Session, Mountain 264)
„hast du wirklich alle stimmen? ich sehe z.b. kimi nicht · bitte weiter" — Kimi K3 fehlte (Header-Abbruch beim Senden), nachreichen | 2026-10-07 | Operator (Session, Mountain 264)
„bitte frage die fehlenden tabs noch ab und finde einen weg das zu klären unbelegt: … evtl musst du nochmal einen flash taucher mit harten bandagen losschicken" — die vier `unbelegt`-Einheiten klären (flash-Taucher) + fehlende Tabs nachfassen | 2026-10-07 | Operator (Session, Mountain 264)
„bitte lade die tabs neu und gib die fragen ein" — Reload + Erneut-Eingeben | 2026-10-07 | Operator (Session, Mountain 264)
„du hast nicht umgestellt / du hast die trying open modelle nicht umgestellt" — der Reload setzt die Tryingopen-Modelle auf den Default; der Wähler muss den `<button>` im `<li>` treffen und der Modellname danach gegengelesen werden | 2026-10-07 | Operator (Session, Mountain 264)
„und ihr müsst das auch bei den chat uis berücksichtigen ihr nutzt sonst leider immer standard anstatt max thinkin deep search (falls die frage search erfordert)" — bei UI-Chats vor dem Senden **max thinking / deep search** wählen | 2026-10-07 | Operator (Session, Mountain 264)
„nein bei den anderen max deep thinking deep search nur claude extra hoch nicht max" — die anderen UI-Chats (z.ai/Duck/Qwen): Max Deep-Thinking + Deep-Search; Claude: „Extra hoch", NICHT Max | 2026-10-07 | Operator (Session, Mountain 264)
„und ihr müsst auch bei jedem modell schauen dass es die stärkste variante ist" — je Stimme die stärkste Züchtlinie/Variante (nicht Flash/Small/mini) | 2026-10-07 | Operator (Session, Mountain 264)
„ich meine insbesondere glm duck und qwen und brauchen wir noch weitere openweight?" — GLM: GLM-5.3 + Deep Think + Max; Duck: GPT-6 Luna; Qwen: Qwen3.8-Max + Denken; offene Kandidaten MiniMax M3/Gemma 4 31B/Ling 3.0, je Seat erst nach der Gretchenfrage | 2026-10-07 | Operator (Session, Mountain 264)
„du hast nicht auf absenden gedrückt zudem habe ich noch https://aistudio.google.com/… model=gemini-3.1-pro-preview offen …" — MiniMax-Agent: Enter im Composer nötig; AI Studio: stärkstes freies Gemini = `gemini-3.1-pro-preview` | 2026-10-07 | Operator (Session, Mountain 264)
„zudem habe ich noch deepseek chat offen" — DeepSeek-Chat: DeepThink + Search aktiviert | 2026-10-07 | Operator (Session, Mountain 264)
„dann habe ich noch togetherai offen das kannst du auch noch vermessen" — Together Chat: MiniMax-M3 antwortet nur mit Fehler; nicht praktikabel, `pending` benannt | 2026-10-07 | Operator (Session, Mountain 264)
„baue die Eigenschafts-Achs" — die `quantity_type`-Achse außerhalb Ω bauen (Masse/Energie/Fläche/Skala/Strahlintensität) | 2026-10-07 | Operator (Session, Mountain 265)
„bitte mache auch eine wissenschafts und forschung und webrecherche und befrage die UI chats" — jede Architekturfrage bekommt die Recherche-Schicht (`archive_search`) + die Frontier-UI-Chats | 2026-10-07 | Operator (Session, Mountain 267)
„council bitte immer durch frintier ui chats untermauern" — jede Ratsentscheidung wird durch die Frontier-UI-Chats untermauert | 2026-10-07 | Operator (Session, Mountain 267)
„kannst du dich bitte darum kümmern?" — die `terms`→SPDX-Migration im Register vollziehen (134× `CC0`→`CC0-1.0`, 1× `ODC-BY-1.0`→`ODC-By-1.0`; `PD`/`free-open`/`own-work` bleiben), dann Mycelium `license_census`/`ci-gate` | 2026-10-07 | Operator (Session, Mountain 268)

## Offen (aufgeschlüsselt)

### Force-/Einheiten-Kontrakt — Klasse (a) 20→7; die 7 sind Kraft-Taxonomie
- **Status:** eigen | **Bindung:** eigen (Unit-Tabellen/Format) · mycelium (Release-Gate) · operator (Kontrakt-Öffnung)
- **Trigger:** `tools-build`-Lauf republiziert das Release-Gate
- **Lage:** (gemessen 2026-10-07, Mountain 265/266) **13 Klasse-(a)-Paare geheilt** (20→7 live; Baseline `docs/specs/force-unit-baseline.txt` trägt die 13 noch gefroren). Nicht-Kraft-Größen sind als `quantity`-Zeilen ausgebaut (`force_type` 255, `presence`-Bit 16; 7 Zeilen in `phi/sources.φ`, s. `register_unit_audit --file phi/sources.φ` = 0 Paare). **Neu (266):** der **Disjunktheits-Test** steht — `field_and_quantity_are_disjoint_taxonomies` (`src/archivar/parse.rs`; `cargo check --tests` 0/0): Kraft-Slot trägt nur einen Kraftnamen, Kind-Slot nur ein Kind; Mischformen werden refused.
- **Blockade:** das Release-Gate `tools-latest` ist stale (baut vor Mountain 264/265) — `advective m/s2`/`mm`, `em db`, `njy`, `w/m2/hz` erscheinen dort als „outside the registry"; die 13 Baseline-Zeilen bleiben darum gefroren. Der frische lokale `target/debug/commit_check` ist grün.
- **Braucht:** `gh workflow run tools-build.yml` (Mycelium) → danach löscht ein Atom die 13 Baseline-Zeilen; der `quantity`-Kontrakt-Satz steht in `docs/concepts/archivar-mathematikerin.md:63`.

### Newell dΦ/dt als `rect`-Treiber — Primär-Arm aligned, OMNI pending
- **Status:** wartend | **Bindung:** eigen (Format) · river (CI-Lauf)
- **Trigger:** `matrix-newell-omni`-Alignment (n=0, neuer `field-te-query`-Lauf)
- **Lage:** (gemessen 2026-10-07 via `ci_manage log 37545120597`) Job `matrix-newell` Zelle `newell_dphi_dt->intermagnet_dbdt` **n=24**, TE 1.6439e-1, `p=0.7273`, verdict `silent` — Bin-3600-Korrektur bewiesen. Job `matrix-newell-omni` `newell_dphi_dt_omni->intermagnet_dbdt` **n=0**, verdict `alignment pending`.
- **Blockade:** der OMNI-Arm hat noch keine deckungsgleiche Zeitachse (n=0).
- **Braucht:** OMNI-Alignment (`ci_manage log <neuer field-te-query-Lauf>`).

### GIC-Faden §A–G — Compiler/Reader je Kanal
- **Status:** eigen | **Bindung:** eigen (Compiler/Reader) · mycelium (Manifestation)
- **Trigger:** Core-Consumer-Reader je Kanal erreicht
- **Lage:** (gemessen 2026-10-07, Mountain 268) **4 Kanäle vorbereitet** — `cdaweb_tec`, `cdaweb_roti`, `goes16_mag`, `poes19_meped`: Compiler in `tools/harvest/src/bin/` + `*-cdn.yml` gebaut (CDF-3- bzw. NetCDF-4/HDF5-Arm), `cargo build` 0/0; Register-/Harvest-Block präpariert. Bestehende Kanäle live: `fink_cutout`, `intermagnet_dbdt`, `goes16_abi`. **Riss (Route):** `state/future/gic-unblock-routen-2026-10-06.md:65` nennt `tec15min_igs` „IONEX", gemessen ist es **CDF3** (`.cdf`). **Riss (Maß):** ein ROTI-Jahres-Asset ≈ 5.4 GiB > 2-GiB-Release-Grenze (per-day vs. Dezimation offen). DMSP-SSJ teilt den CDF-Arm, braucht einen eigenen Compiler.
- **Blockade:** je Kanal fehlt der **Core-Consumer-Reader** — `src/archivar/extract.rs` dispatcht `parse_series` nach `format` (`:118/399/785`), `main_flow.rs` braucht den Kanal-Loop (wie `swarm_tec:4950`); ohne ihn lösen die `field`-Zeilen nicht auf und das kompilierte `.bin` erreicht den Wire nicht.
- **Braucht:** je Kanal ein `src/archivar/<channel>.rs`-Reader + `extract.rs`/`fetch.rs`/`main_flow.rs`-Dispatch (nächster Atom); dann `phi/harvest.φ`-Block + `phi/sources.φ`-Felder + Manifestation. Offen bleiben: THEMIS GMAG (CDF, anonym), Swarm L2 FAC (CDF, GFZ-Mirror CC-BY-4.0), VLF AWESOME (`.mat`), PCN, CARISMA, AMPERE, DMSP-16 SSJ.

### GIC-Faden-Wunschliste §A–G — Admission der 15 neuen
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Mycelium legt die erreichten Kanäle vor (Reachability-Messung)
- **Lage:** (gemessen 2026-10-07 via `general`) 33 Fäden gemessen — `state/mycelium/gic-api-reachability-2026-10-06.md` (lebt 14, neu+erreichbar 15, blockiert 7).
- **Blockade:** Admission je neuem Kanal (Format-Arm nötig).
- **Braucht:** Format-Arm je Kanal (s. GIC-Punkt) + `terms` aus der Admission; Korrekturen CARISMA `www.carisma.ca`, DMSP-SSJ via CDAWeb HAPI, `P_dyn` in `OMNI_HRO_1MIN`, SSUSI funded-dead.

### CDAWeb-Anonymkanäle — Admission
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** Format-Arm je Kanal erreicht
- **Lage:** (gemessen 2026-10-07 via `general`) fünf Kanäle (TEC `tec15min_igs`, JPL ROTI `roti15min_jpl`, GOES-16 MAG, DMSP-16 SSJ, POES-19 MEPED) am Baum 206; Detail `state/future/gic-unblock-routen-2026-10-06.md`.
- **Blockade:** Admission → Ernte + Manifestation.
- **Braucht:** Admission je Kanal; dann Ernte + Manifestation (Mycelium).

### GIC-Breitenband-Familien — Deskriptoren (Blatt-Slot) + CGM-Provenienz-Riss
- **Status:** eigen | **Bindung:** eigen (Format/Descriptor) · river (`field_te_query`)
- **Trigger:** Blatt-Rat löst Route C ↔ Linie 1
- **Lage:** (gemessen 2026-10-07, Mountain 268) **gebaut** — drei Band-Deskriptoren `phi/pipeline/descriptors/gic_{auroral,subauroral,midlat}.te` (31/25/98 Kanäle, `expect cells 930/600/9506`), Coverage-Test `gic_bands_are_pairwise_disjoint_and_cover_the_154_station_pool` (`tools/measure/src/bin/field_te_query.rs`, paarweise disjunkt, Union = 154), 3 CI-Jobs in `.github/workflows/field-te-query.yml`; `cargo check` 0/0. Provenienz-Riss CPL/TTB benannt (`cgm_source bgs-quasi-dipole`, Arme gebaut). **Riss (Blatt):** `docs/blatt/blatt-gic-breitenband-familien.md` (Rat 2026-10-07, Route C, :249-251/:287) verwirft drei per-Band-`full`-Deskriptoren als Träger und hält Familien-Deskriptoren als Stufe-2-Member-Pool `pending`; „Linie 1" steht als eine ungeglättete Linie.
- **Blockade:** der Blatt-Rat (Route C ↔ Linie 1) ist offen; der 157-gegen-154-Bezug je Station; die übrigen 152 `cgm_lat`-Zeilen tragen Provenienz (`omniweb-cgm`/`supermag-aacgm`) nur in `state/river/gic-cgm-lat.tsv`.
- **Braucht:** Rat/River entscheidet die Route (Deskriptor-Träger vs. Member-Pool); optional `cgm_source <token>` je der übrigen 152 Zeilen aus der TSV.

### Receiver-Apertur `span`-Direktive
- **Status:** eigen | **Bindung:** eigen (Format) · river (Membran-Brücke)
- **Trigger:** Rivers Membran-Brücke trägt die Apertur in Query/Record
- **Lage:** (gemessen 2026-10-07, Mountain 268) **Mountain-Arm gebaut:** `span`-Arm in `src/archivar/parse.rs` (positiv-finit, sonst absent), `SourceConfig.span: Option<f64>`; Test `span_directive_carries_a_positive_finite_aperture_else_absent`; `cargo check` 0/0. **Rat-Verdikt:** `extent` reusen (Sample-Slot 7 trägt die Apertur bereits), **kein** 27. Wire-Feld; `span` lebt als deklariertes `SourceConfig`-Feld, nicht als Körper-`radius_m` (`membrane.rs:364,385` unberührt). **Riss:** die statische Membran liest kein `SourceConfig` (nur `.bin` + Sterne, `membrane.html:576-589`), die Presence ist ein nackter Query-Mittelpunkt — `span` allein rendert die Startansicht nicht.
- **Blockade:** Rivers Startansicht-Brücke (Apertur als Query-Input/Record-`extent` in `wasm.rs:65-99` + `membrane.html:501-528`); Fenster-Edit operator-gebunden.
- **Braucht:** River trägt die Apertur als Query-/Receiver-Input bzw. Record-`extent` in den ω()-Lauf; `span_anchor = sqrt(r2)+extent` (`membrane.html:511`) bleibt die Messung.

### Membran-Sonne-Anker — `de_compiler` GM, Maske Bit 11
- **Status:** eigen | **Bindung:** eigen (Compiler/Format) · river (Checkmark)
- **Trigger:** GM-Landung + Remanifestation der Sonne-`.bin`
- **Lage:** (gemessen 2026-10-07, river-122) die deployte Sonne-`.bin` trägt den GM nicht; Rivers Checkmark `nearCount(<1e13 m) > 0` hängt daran.
- **Blockade:** Bit 11 / slot `f(11)` fehlt in der Sonne-`.bin`.
- **Braucht:** Bit 11 / slot `f(11)` in der Sonne-`.bin` (de_compiler + Manifestation).

### Ungepoolte Register-Blöcke (Riss) — tote Generation, kein Merge
- **Status:** descoped | **Bindung:** eigen
- **Trigger:** nächster Register-Abgleich
- **Lage:** (gemessen 2026-10-07 via `general`) Recovery-Snapshot `~/archive/archive-root/opencode-tmp-2026-09-01/archeo_check/archeology/sources/sources_recovery_cdn-merged_60k_lost-blocks.φ` (60132 Zeilen) nutzt das Port-Draft-Schema: `ra_key`/`dec_key`/`z_key` (kein Arm in `src/archivar/parse.rs`) und 3-Token-`field srcid <name>` — der Parser refused 3-Token-field (`parse.rs:941`); das lebende Register nutzt `ra ra`/`dec declination`/`z redshift` + 6-Token-field. Kein Superset: tote Generation → `archive-root`-Verdikt, kein Merge.
- **Blockade:** keine — die Messung hat entschieden.
- **Braucht:** kein Merge; der Snapshot bleibt in `archive-root`.

### dropped-gate — Carrier-Architektur (Register, nicht Baseline-Zahl)
- **Status:** eigen | **Bindung:** eigen (Gate/Register) · mycelium (CI)
- **Trigger:** Sichtung der 1300 + deklarierte Rebaseline
- **Lage:** (gemessen 2026-10-07 via `register_lookup --dropped`/`ci_manage log 37553361521`) `dropped-gate` (`.github/workflows/ci-gate.yml:70`) zählt Punkte aus archivierten Übergaben ohne lebenden Träger/Commit; Baseline `docs/zustand/dropped-baseline.md` 1300 (zuletzt gebumpt `416f7b59b`), current 1282. **Drei externe Stimmen (qwen, duck/Gemma 4 31B, nemotron-3-ultra, 2026-10-07) konvergieren:** `archiv = Ort, kein Träger` · Auto-Bump und Archiv-als-Träger verworfen (0 honored) · Carrier-Ledger nur als **abgeleitete** Sicht · **explizites Carrying je Move** ist die Architektur. Nemotron-Formel: `dropped := punkt ohne träger in (lebend ∪ commits ∪ register) · move ohne disposition = rot · baseline frozen`.
- **Blockade:** Träger-Ableitung im Gate ungebaut; Punkt-Identität über den Move (ID/Hash) unbelegt → `pending`; Pauschal-Transfers sind Fabrikation, wo Xs nächster Schritt die Natur nicht berührt.
- **Braucht:** Gate auf Träger-Ableitung (lebend ∪ commits ∪ register) umstellen, `move ohne disposition = rot`; danach **einmalige** deklarierte Rebaseline mit Sichtungsliste.

### DE441/DE442-Anker ohne sha256
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** erster Manifest-Lauf je Anker
- **Lage:** (gemessen 2026-10-07 via `sread phi/sources.φ`) DE440-Anker tragen `sha256`; DE441/DE442-Anker keine Zeile.
- **Blockade:** DE441 entsteht aus zwei gemergten NAIF-Teilen — der sha ist erst nach dem Lauf messbar.
- **Braucht:** gemessene `sha256`-Direktive nach dem ersten Manifest je Anker.

### IGRF-Koeffizienten-Arm (für `geomag_lat`)
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** verifizierte Grad-13-Referenz (Testwerte)
- **Lage:** (gemessen 2026-10-07, Mountain 268) `src/archivar/igrf.rs` gebaut — parst `igrf14coeffs.txt` (NOAA, sha `8f8d8840…`), Grad-1-Dipol + geomagnetischer Pol, verifiziert gegen Beggan 2026 Tab. 3 (2020.0: 80.65°N/72.68°W); `geomag_lat <deg>`-Parser-Arm + `SourceConfig.geomag_lat`; `cargo check` 0/0, Tests in CI. **Riss (Naming):** Rat-Verdikt-Form ist `geomag_lat_<frame> <deg> igrf-<edition>` (`archiv/handover-2026-10-06-mountain-folge249.md:61-66`); gebaut ist `geomag_lat <deg>`.
- **Blockade:** die volle Grad-13-Synthese (X/Y/Z) ist ungebaut — kein unabhängiger Referenz-Testwert im Baum (`igrf14testvalues.txt` überall 404); ohne ihn wäre die Rekursion eine unbelegte Behauptung.
- **Braucht:** eine gemessene Grad-13-Referenz (IAGA-Testwerte oder der Fortran-`igrf14.f`-Port als Zeuge) → dann Synthese + Register-Zeile; optional `geomag_lat_<frame>`-Form.

### Exposom-Matrix §A — Register-Zeilen je Klasse
- **Status:** eigen | **Bindung:** eigen (Format) · mycelium (Register)
- **Trigger:** Mycelium-Register-Block je Klasse
- **Lage:** (gemessen 2026-10-06, Mountain 248; fortgeschrieben 2026-10-07) Reader `osm_pbf.rs`/`eionet_cdr.rs` gebaut, End-to-End am Realfile; `main_flow.rs:4155-4156` trägt `osm_nodes`/`eionet_cdr`. `osm_nodes` = `em 1` (Rat); `eionet` Medium-Arm gebaut.
- **Blockade:** die Register-Zeilen je Klasse (s. eionet/osm).
- **Braucht:** Mycelium-Register-Block je Klasse nach dem Medium-Arm.

### Vlies-`matrix full` — alignment pending
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Format-/Compiler-Arm erreicht (`field_te_query` alignment)
- **Lage:** (gemessen 2026-10-06, River 112, `d9351b0e`) Lauf `37500311359` misst 15/15 Arme; verbleibend alignment pending (n=0) bei den Solar-/Magnetosphären-Zellen.
- **Blockade:** Format-/Compiler-Arm für die deckungsgleiche Zeitachse.
- **Braucht:** Format-/Compiler-Arm bauen (s. GIC), dann erneuter CI-Lauf.

### obis.osha.gov — Register-Zeile am `1`-Riss + LICENSE
- **Status:** eigen | **Bindung:** eigen (Format/terms) · mycelium (Transport/LICENSE)
- **Trigger:** Kraft-/Einheiten-`1`-Ausgang (Force-/Einheiten-Kontrakt)
- **Lage:** (gemessen 2026-10-07, mycelium-256) `.github/workflows/osha-cehd-cdn.yml` (dispatch-only) steht; die `url`/`format`-Zeile hängt am Force-/Einheiten-`1`-Riss (der Compiler emittiert Masse/Massenkonzentration/amount-fraction `1`). Das generierte `LICENSE` im `omegaflow/sources`-Repo wartet auf die `terms`-Zeilen der register-tragenden Blöcke.
- **Blockade:** der `1`-Ausgang des Force-/Einheiten-Kontrakts.
- **Braucht:** `terms`/`url`-Zeile nach dem `1`-Ausgang; dann fällt die `cdn-tag-baseline.txt`-Ausnahme + LICENSE-Erzeugung.

### Lizenz-Disposition — `terms`-Feld (`unbestimmt`/`ohne-lizenz`); 159 abgeleitet, nicht persistiert
- **Status:** eigen | **Bindung:** eigen (Format/Datenkontrakt)
- **Trigger:** `rights_read`-Datenkontrakt entworfen
- **Lage:** (gemessen 2026-10-07, Mountain 267 via `cargo run -p omegaflow-register --bin license_census`) **Struktur entschieden** (Rat über zwei Runden, Recherche-Schicht via `archive_search`, sechs Frontier-UI-Chats — Claude, Duck, Qwen, z.ai, DeepSeek, Gemini). Verdikt: `terms` ist ein **eigenes Feld** im B-Block (nicht Kopf-Token; der Kopf trägt genau einen Infra-Zustand, Lizenz ist orthogonal). Token **`terms unbestimmt`** (SPDX `NOASSERTION`) und **`terms ohne-lizenz`** (SPDX `NONE`); `terms keine` gestrichen (Fehllese „keine Einschränkungen" = frei). Die Abwesenheiten werden **nicht** als Registerzeilen persistiert (Fabrikation; abgeleitetes Query-Komplement) — nur gemessene Zustände stehen im Register. **Gebaut:** `register_lookup.rs:408,433` (Owner mountain), `scan_dispositions_text`/`collect_orphan_candidates_in` lesen das orthogonale `terms`-Feld (Test `scan_dispositions_reads_the_terms_field_orthogonal_to_the_head`); `license_census.rs` meldet die abgeleitete `NO-TERMS`-Liste + Zählzeile — `blocks 2673 | terms 143 | distinct 9 | no-terms 2249 | pending 2530 | terms-vocab 0 violation(s)`. **Wohnort:** Census-TSV umgezogen nach `state/mountain/license-census.tsv` (genau ein Konsument; keine Operator-Frage). **Migration vollzogen (Mountain 268):** `phi/sources.φ` `terms`→SPDX — 134× `CC0`→`CC0-1.0`, 1× `ODC-BY-1.0`→`ODC-By-1.0`; `PD`/`free-open`/`own-work` unberührt; danach zieht Mycelium `license_census.rs`/`ci-gate` nach. **Riss 1 (bleibt):** Baum 159 Netlocs ohne `terms` vs. Census 162 (`UNMEASURED`/`STALE`) — zwei unabhängige Linien, nie gemittelt. **Riss 2 (bleibt):** `ohne-lizenz` kann mit `PD` verwechselt werden (PD ist Vokabel-Wert, „keine Lizenz" restriktiv).
- **Blockade:** der `rights_read`-Arm braucht zuerst einen Datenkontrakt-Entwurf (wo das Bit lebt — Wire-Slot vs. separater Harvest-Zustand), kein Einzeiler; bis dahin ehrlich eine Stufe `unbestimmt`.
- **Braucht:** Rat/Datenkontrakt-Entwurf für `rights_read` → Arm am Harvest-Gate (OAI-PMH `<rights>`/DataCite `rightsList`). Beleg-Regel: jedes `terms`-Feld im B-Block trägt Datum+URL in `note` (die Tabelle ist gitignored). Transport an River: `## An river`.

### KCG2 (prop.kc2g.com) — JSON-Reader-Arm
- **Status:** eigen | **Bindung:** eigen (Format/Parser) · mycelium (Harvest/Register)
- **Trigger:** Harvest-Pattern + `*-cdn.yml` für KCG2
- **Lage:** (gemessen 2026-10-07, Mountain 268) **Reader gebaut** — `tools/harvest/src/bin/kc2g_stations.rs` (JSON `mufd`/`fof2`/`tec`/`cs`, absent = `Option`, CSV + Test; `cargo build` 0/0); `blocked_sources.φ:234-236` trägt `pending`; Route + JSON 200 (42482 B), terms unbestimmt.
- **Blockade:** Harvest-Block in `phi/harvest.φ` + `kc2g-stations-cdn.yml` + `terms unbestimmt`-Zeile fehlen (Mycelium-Transport); der `at`/`field`-Block ist noch nicht in `phi/sources.φ`.
- **Braucht:** `phi/harvest.φ`-Block (`format kc2g_stations`/`arm`/`workflow`/`pattern ^kc2g_stations\.csv$`) + CDN-Workflow + `terms unbestimmt`-Zeile; dann Register-Block + Manifestation.

### obis.osha.gov — unit-fähiger Reader (kein `1`-Riss im engen Sinn)
- **Status:** eigen | **Bindung:** eigen (Parser/Format) · mycelium (Transport)
- **Trigger:** unit-fähiger OSHA-Router gebaut
- **Lage:** (gemessen 2026-10-07, Mountain 268) `osha_cehd_compiler` emittiert gemischte Einheiten (`kg/m3`,`kg`,`1`,`1/m3`) in einer Spalte; `parse_axis_position_value_text` (`extract.rs:4116`/`geo.rs:859`) verwirft den Einheitentoken und bindet die Quelle an **eine** Unit. `blocked_sources.φ` trägt `pending`. Die emittierte Einheitenmenge liegt in **keiner** Einzelkraft (`units.rs`: diffusion hat kg/m3+kg+`1`, nicht `1/m3`; em hat `1`+`1/m3`, nicht kg/m3).
- **Blockade:** der Reader ist nicht einheiten-fähig; eine `field`-Zeile über die gemischten Zeilen wäre nicht einheiten-wahr.
- **Braucht:** den Reader einheiten-fähig machen (`extract.rs:4116` + `geo.rs:859` Token zurückgeben und nach `field`-Unit filtern) **oder** den Compiler je Einheitenklasse splitten; dann vier `field`-Zeilen + `terms`; danach Mycelium `gh workflow run osha-cehd-cdn.yml`.

## An river

Origin: mountain-folge267.

- **Census-TSV umgezogen:** `state/river/license-census.tsv` → `state/mountain/license-census.tsv` (Rat + UI 267: Lizenz ist Quellen-Eigenschaft → Mountain; „River misst, Mountain verdiktet" verworfen — Messung und `terms`-Verdikt sind ein Akt). Der Baum kennt genau einen Konsumenten (`license_census.rs:7`), die Konstante zeigt auf den neuen Pfad; `.gitignore:/state/` deckt ihn. **Braucht:** kein weiterer Schreibpfad auf `state/river/` — eine eigene gitignored Kopie dorthin verschieben; die Provenienz im Tabellenkopf („River 107") bleibt als Autor.

## An mycelium

Origin: mountain-folge267.

- **`tools-build` stale:** das Release-Gate `tools-latest` baut vor Mountain 264/265; ohne Rebuild bleibt der Ratchet nur mit dem frischen lokalen `target/debug/commit_check` grün. `gh workflow run tools-build.yml` → danach löscht ein Mountain-Atom die 13 Baseline-Zeilen (`docs/specs/force-unit-baseline.txt`).
- **`intermagnet_dbdt`-Compiler:** `--start` ist Pflicht (kein fabriziertes Default-1994); das Workflow `intermagnet-cdn.yml` sendet `inputs.start` — leerer Input bricht mit exit 2 ab.
- **Sternkatalog `dr3_stars.bin` — Sort gebaut, Re-Harvest ist dein Schritt** (`gh workflow run gaia-cdn.yml`; Träger `tap_compiler --star-bin`).
- **`eionet_cdr`** 276-`field`-Block reproduzierbar (`eionet_cdr_compiler --emit-field-names`); Block-Header (`url`/`origin`/`compiler`/`format`) ist dein Pen; Kraft-Verdikt `diffusion kg` + Punkt-Kernel offen.
- **`osm_nodes`**-Register-Block steht (`phi/sources.φ:1678-1684`), Harvest-Pattern `^monaco_nodes\.bin$` + `*-cdn.yml` prüfen.
- **Fink-Cutout** Harvest-Pattern `^fink_cutout\.bin$` + `*-cdn.yml`; **CDN-Workflows** für `jaxa_gpm_ku`, `nasa_power_t2m`, `epa_aqs_voc`, `osm_pbf_compiler`/`monaco_nodes` je `*-cdn.yml`; **`vnp46a3-cdn.yml`** Granule mit DNB-Nachtdaten wählen; **LICENSE** erst nach `terms`-Zeilen.
- **Browser-Brücke `1-ui`-Eigentum (Operator-Frage 2026-10-07):** die Extension `OpenCode Browser` (id `fmjakelcochilgoghbdgnomafoogilof`, v0.17.2) + das Plugin `@vymalo/opencode-browser@0.17.0` binden jede Tab-Gruppe an den Client, der sie zuerst fuhr; eine neue Session ist ein neuer Client → `group "1-ui" is owned by another client` (hart; `browser_release` cleart es **nicht**, gemessen 266). Ein Mensch ist das nicht, eine tote Session-Client-Id. Hebel: (a) Plugin-/Extension-Version angleichen + opencode/Brücke neu starten (Ownership-Map prozess-lokal; die Chrome-Tab-Gruppen überleben → herrenlos); (b) Konvention `<line>-ui` (JIT) + `open-weight-ui` — jede Linie öffnet die starken Seats per URL im eigenen Profil (gleiche Logins); `1-ui` schließen, kein Nachfolger. Eine echt geteilte Gruppe bräuchte eine Option in der Dritt-Brücke (nicht im Repo).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push.

Eigene Pfade: `src/archivar/mod.rs`, `src/archivar/igrf.rs`,
`src/archivar/parse.rs`, `src/archivar/tests.rs`, `src/archivar/types.rs`,
`tools/utils/src/bin/volume_builder.rs`, `tools/measure/src/bin/field_te_query.rs`,
`tools/harvest/src/bin/cdaweb_tec_compiler.rs`,
`tools/harvest/src/bin/cdaweb_roti_compiler.rs`,
`tools/harvest/src/bin/goes16_mag_compiler.rs`,
`tools/harvest/src/bin/poes19_meped_compiler.rs`,
`.github/workflows/cdaweb-tec-cdn.yml`,
`.github/workflows/cdaweb-roti-cdn.yml`,
`.github/workflows/goes16-mag-cdn.yml`,
`.github/workflows/poes19-meped-cdn.yml`,
`docs/specs/force-unit-baseline.txt`,
`docs/handover/handover-2026-10-07-mountain-folge268.md`,
`docs/handover/archiv/handover-2026-10-07-mountain-folge267.md` (Move),
`state/operator-gespraeche/2026-10-07-mountain.md`.
