<!--
  title: Handover — River-Folge 203 (2026-10-13)
  session: River-Linie (river-203/204) — QN-Alias-Enforcement + δn-Diagnose gebaut (ConstraintRegistry, from_specs-Alias-/Region-Enforcement, charge_imbalance; cargo check 0/0); mycelium-337 omni2-Block gefaltet; Newell-Doppelregistrierung als tokenisierter `## An mountain`-Block gesetzt — von Mountain gefaltet/geheilt (`1dd4e3549`); **river-204:** gemessen, dass die gemessenen CHAMP-Dichte-Kanäle schon registriert sind (Neutral `champ_thermosphere_density_kgm3`, geladen `champ_plpt_electron_density_cm3`, Wind `champ_thermosphere_crosswind_ms`) — der Rat-Dichte-Weg (IRI/NRLMSISE/IGRF) ist Modell und überholt; die Produktions-Naht `ChannelCouplingLayer::from_sources` gebaut (channels + couplings + constraints aus Sources + Register), `cargo check` 0/0; folge202 konsumiert/archiviert; **river-205:** Spezies-Achse gebaut (`species: Option<String>` Descriptor-Achse in `hash`/`PartialEq` + `m^-3` in `unit_token` + `species <token>` in der `channel`-Direktive; DE-2-N2-Kanal entkollidiert vom CHAMP-Gesamtdichte-Kanal, `cargo check` 0/0), CINDI-NWM-Zugangsweg gemessen (UTD Hanson Center, kein offenes NWM-Archiv) + Anfrage-Entwurf an die Kante (`state/mail/2026-10-13-cindi-nwm-anfrage-entwurf.md`, nicht gesendet)
  class: handover
  date: 2026-10-13
  sha256: b53728a444f770ac50de52241a6aa2b776c9e501e6e600fb20634bb4de2fc16e
  status: live
-->
# Handover — River-Folge 203 (2026-10-13)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward).

## Arbeitsweise (Operator-Wort 2026-10-12) — mehrere Agenten parallel

Der Operator: „schreibe in die übergabe dass ihr mehrere agenten losschickt ich hab echt
keine lust mehr auf eure verschlepperei und vergessen." Jede River-Session dispatcht **alle**
parallel arbeitbaren Punkte in **einem** Zug an Agenten (`grind-flash`/`general`/`vision`) —
nie nacheinander, nie „nächster Dispatch", nie „nächste Session". Verschleppung und Vergessen
sind der benannte Fehler; ein Punkt, den die Session selbst arbeiten kann, wird im selben Atom
gearbeitet (Sofort-Prinzip).

## Burn: open 0.00 · close 0.29 · cap 0.50 — Grund: session river-203/204 (deepseek-flash, kein pro/max); 1 `grind-flash` (QN-Enforcement-Bau) + 1 `general` (Newell-OMNI) + die CHAMP-Messwert-Recherche + `ChannelCouplingLayer`-Naht selbst gebaut; die close-Zahl ist das gemessene Haus-Delta ($2.7047 → $2.9935). Fälligkeit: `--fired river` = 0, `--stale river --persist 3` = 0, `open_points_check` folge202 = 0 absent / 0 stale-citations / 0 format-gaps / 0 owner-drift. 1 adressierter Token gefaltet (mycelium-337 omni2). Kein `smail --send`, kein Window-Edit. `git_safety --snapshot` `refs/safety/1791844786`.

## Burn: river-205 — 1 `general` (CINDI-NWM-Zugangsweg) + 2 `archive_search --verdict`; die Spezies-Achse selbst gebaut (`cargo check` 0/0, rustfmt clean); der CINDI-Anfrage-Entwurf selbst geschrieben. `--fired river` = 0, `--stale river --persist 3` = 0; `open_points_check` folge203 = 2 absent / 2 format-gaps — beide gepflegt (operator-gespraeche-Datei angelegt; DE-2-Sektion Trigger/Blockade gesetzt). Kein `smail --send`, kein Window-Edit.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Nicht gebaut — die P10.2a-Token-Semantik ist nicht entschieden; ein geratener Bau wäre Fabrikation (A=A). bitte archive search all rat und roster" | 2026-10-10 | Operator (Session, River 165)
„wenn du den rat befragst befrage bitte auch die wissenschaft mit archive serach alll und die ui und oenweight frontier voices" | 2026-10-09 | Operator (Session, River 145)
„warum nur duck … ich möchte dass du alle frontier chats befragst" | 2026-10-07 | Operator (Session, River 127)
„ja ich meine alle blöcke müssen korrekt sein dafür haben wir doch die wissenschaft" | 2026-10-09 | Operator (Session, River 160) — **P10-Wort**: jeder `field`-Block wird wissenschaftlich geprüft und korrekt etikettiert (Quantity | Mechanism | Medium)
„ja natürlich sonst wartest du doch bis zum st. nimmerleinstag" + „ja bitte ihr müsst das jetzt echt mal in den griff bekommen" | 2026-10-10 | Operator (Session, River 170) — den abgebrochenen `ci-gate` neu anstoßen; den CI-Burst in den Griff bekommen
„mach A" | 2026-10-10 | Operator (Session, River 170) — `ci-gate.yml`-`subset` von dem geteilten t420 auf GitHub-hosted `ubuntu-24.04-arm` mit per-SHA-Concurrency umziehen
„beides" | 2026-10-10 | Operator (Session, River 170) — (1) die zwei fremden ci-gate-Roten heilen; (2) die `observer`→`receiver`-Frage als Rat-Frage aufsetzen; Audit „wo wurde der Observer-Bias wieder eingeschleust"
„C (Skalenspektrum je Paar) ist der nächste begrenzte Schritt — Architekturfrage → Linse der fünf Stimmen, bevor gebaut; danach D (Summary-Graph/PCMCI, Runge 2105.10381/1702.07077). Die Seats: B ohne Skalenannotation wäre die stille Fabrikation. bitte --all rat und max roster" | 2026-10-10 | Operator (Session, River 172)
„Nächster Schritt: D2 durch die fünf Stimmen (Rat), dann Skeleton/MCI bauen. bitte --all max roster und rat" | 2026-10-10 | Operator (Session, River 172)
„bitte --all max roster und rat" | 2026-10-10 | Operator (Session, River 174)
„z.ai immer glm 5.3 max gibt es auch in tryong open genauso wie qwen" | 2026-10-10 | Operator (Session, River 174)
„mit archive search dem rat und max roster (alle und warten bedenke tryingopen1)" | 2026-10-11 | Operator (Session, River 181)
„ok dann ist es pending … dann warten wir bis wir zugelassen sind kannst du nochmal in state schauen wir hatten da eigentlich einige netzbetreiber" | 2026-10-11 | Operator (Session, River 188) — ENTSO-E/NERC bleiben `pending`; auf die Zulassung warten, bis dahin die Netzbetreiber aus `state` referenzieren (nicht re-suchen).
„mountain ist sehr beschäftigt und mycelium auch … kümmer dich bitte selbst drum" | 2026-10-11 | Operator (Session, River 191) — **River übernimmt den Vlies-Matrix-Alignment-Punkt selbst** (SWPC/RTSW-Archivreihe finden + registrieren + `matrix-vlies` neu), auch in Mountain- (`phi/sources.φ` url) und Mycelium-Domäne (Manifestation/Compiler).
„ja" | 2026-10-11 | Operator (Session, River 192) — (1) den SPAN-Feld-Edit bauen; (2) die vier verwaisten `cargo fmt`-Hünke (Mountain-316/Mycelium-312-Rückstand) als Haus-Dienst committen.
„ja bitte selbst mountain ist beschäftigt ich glaube wir müssen projektownerschaften einführen nicht aufgabenownerschaften das reibt so dermassen" | 2026-10-12 | Operator (Session, River 196) — (1) River setzt die Stations-Tabellen-Korrektur **selbst** (fremder Schreibakt auf Operator-Wort); (2) **Architekturfrage: Projekt-Ownerschaft statt Aufgaben-Ownerschaft** (Rat/Operator).
„aber die neun kraftmedien sind doch auch abgeschafft die hast doch sogar du selbst abgeschafft warum seid ihr gerade alle s kopflos" | 2026-10-12 | Operator (Session, River 197) — die 9 Kraft-Medien UND die flachen `force`-Tokens sind abgeschafft; die Kanal-Ontologie (`channel <conserved>:<role>:<op>:<pde_type>:<medium>:…) ist die einzige Form.
„warum legacy bridge hast du deine arbeit nicht zuende gemacht ich habe immer gesagt ich möchte keine abwärtskompatibilität" | 2026-10-12 | Operator (Session, River 197) — keine Abwärtskompatibilität; den P9.2-Bau zu Ende bringen.
„gab es da nicht schon rats gespräche und archive search all und ui seats sag mal was ist denn heute los" | 2026-10-12 | Operator (Session, River 197) — die P9.2/P10-Entscheidung ist durch Rat + `archive_search --all` + UI entschieden; **kein neuer Rat**.
„was heisst morgen glieich aber nicht in dieser session mach eine übergabe" | 2026-10-12 | Operator (Session, River 197) — P9.2 sofort als nächstes Atom, aber **nicht in dieser Session**.
„bitte setze eine taucher staffel an ich möchte dass du jetz fertig wirst Commit und Push stehen" | 2026-10-12 | Operator (Session, River 199) — Taucher-Staffel ansetzen, P9.2 zu Ende bringen; Commit + Push.
„ich habe es mountai geschickt" | 2026-10-12 | Operator (Session, River 199) — die Register-Reconciliation (`channel`-Direktiven, P9.1) liegt bei **Mountain**; River wartet auf die Faltung von `xfer:river199-1`.

„wir wollen doch keine modelle, oder?" | 2026-10-13 | Operator (Session, River) — **Modelle sind aus** (A=A): IRI-2020/NRLMSISE-00/IGRF (und jeder Modell-Nachbau) sind keine Register-Quellen; die gemessenen CHAMP-Kanäle sind der Weg.
„nein lass recherchieren und ja messwerte und du baust" | 2026-10-13 | Operator (Session, River) — die Quellen-Recherche läuft autonom; **Messwerte** sind die Quellen; **River baut**.
„mountain ist ziemlich beschäftigt kannst du das nicht slebst übernehmen" | 2026-10-13 | Operator (Session, River) — River übernimmt die Mountain-Punkte (Regime-Zeilen, Newell, WGSL); die Mountain-Register-Schreibakte + Mycelium-Manifestation trägt River auf Operator-Wort.
„los" | 2026-10-13 | Operator (Session, River) — Startwort für den Messwert-Ketten-Bau.

Verbatim: `state/operator-gespraeche/2026-10-13-river.md` (neu), `2026-10-12-river.md`, `2026-10-11-river.md`,
`2026-10-10-river.md` und `2026-10-09-river.md`.

## Träger (Prosa, eigene)

- `docs/concepts/kanal-ontologie-komplettbau.md` — der komplette Bauplan (P0–P10); P10.2a (der Parser-Arm ist Rivers Teil) als gebaut nachgeführt (`:215`).
- `docs/concepts/archivar-mathematikerin.md` — der Wire/GPU-Force-Vertrag; Träger des kinetischen Rahmens + der **Receiver-Apertur**-Kontrakt-Zeile (`:34`) + des **Schneidungs-Gesetzes**.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`status: unsealed`) — Präregistrierung der Vorhersage-Zelle; die Intervall-Konstruktion benannt (studentisiertes Moving-Block-Bootstrap, delete-a-block-Jackknife-SE). Siegel = Operator-Wort.
- `docs/paper/gic-causal-driver.md` — die Punktschätzer-Unsicherheit auf das studentisierte Moving-Block-Bootstrap umgestellt (`:171`).
- `docs/concepts/der-paradigmenwechsel.md` (9 offene Marker) · `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4) · `docs/paper/flyby-path-2-preregistration.md` (1) — **von sensory-255 gefaltet (JUICE/Flyby-Path-2-Trägerschaft, `An river`-Block 2026-10-11; registriertes Operator-Wort 2026-09-27 „bitte prüfe, was wirklich dir gehört"; `state/zustand/wartend.φ` Key `flyby-path2-recon` Aufnehmer river)**. Tokens (sensory-256, 2026-10-12 gefaltet): `xfer:sensory.ses_ece7532c3ffe16Z417fBRj8b31.1~4844adaa` (paradigmenwechsel) · `.2~5243b5f0` (falsification-metric-addendum) · `.3~1f5f2109` (preregistration) · `.4~59f0ba87` (trägerlose Flyby-Orphans: `auftrag-flyby2-kette.md` + `flyby-path-2-addendum-2026-09-29.md`).
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` — die Zellen der Flugbahn stehen `pending` (nach dem Flyby); der `pending`-Rest ist die 1-σ-Kovarianz (siehe §Flyby-Kette).
- `docs/surveys/survey-2026-10-12-standort-lokalisierung-ortho-dsm.md` — die generalisierte Standort-Lokalisierungs-Landkarte (Orthofoto · DSM/DOM · Street-Level) + Methode `dsm − dtm`; Carrier dieser Survey.
- **Offene-Prosa-Träger (future-235/future-240-Abgabe, River-Carrier):** `docs/auftrag/auftrag-flyby2-kette.md` (3) · `docs/auftrag/auftrag-universelles-vlies.md` (2) · `docs/blatt/blatt-te-externer-steuerparameter.md` (1) · `docs/blatt/fruehwarnsystem-praeregistrierung.md` (1, `status: unsealed`) · `docs/paper/flyby-path-2-addendum-2026-09-29.md` (27) · `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (1 — Marker `:122` gemessenes `descoped`). `register_lookup --orphan-docs` = 0.
- **Mountain327-Abgabe (Marker abgearbeitet 2026-10-12 river-199):** `docs/blatt/blatt-gic-breitenband-familien.md` (4→2) · `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (4→2) · `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md` (2→0). `xfer:mountain327-3` (echoed) · `xfer:mountain327-4` · `xfer:mountain327-5`.

## Netzbetreiber-Referenz (state, nicht re-run)

Gemessen 2026-10-11. Quellen: `state/future/space-weather-service-kante.md` §Adressat · `state/river/gic-sources-broad-2026-10-11.md` · `state/river/registrierungen-2026-10-11.md` · `state/future/funding-register-fuenf-achsen-2026-10-09.md`.

- **ENTSO-E** (EU, Dach 40 TSOs) — `token pending` (A78-Compiler gebaut). **Fingrid** (FI) — Token + Compiler; `fingrid-cdn 38162332055`. **NESO** (UK) — keyless CKAN 200, kein Outage-Datensatz. **BGS** (UK) — nur B_GIC-Index/Modell. **Statnett** (NO) — PDF-only. **Amprion** (DE) — keyless CSV-Outage-API. **TenneT** (DE) — 403 geo-block. **50Hertz** (DE) — PDF-only. **TransnetBW** (DE) — delegiert an die ENTSO-E-Plattform (`10YDE-ENBW-----N`). **Svenska Kraftnät** (SE) — CKAN, kein Outage.

Operator-Wort 2026-10-11: auf die Zulassung warten (ENTSO-E/NERC `pending`); die Netzbetreiber aus `state` referenzieren, nicht re-suchen.

## Offen (aufgeschlüsselt)

### P7.1 Kopplung/Interface — Messwert-Kette gemessen: CHAMP-Dichten schon registriert; Produktions-Naht gebaut
- **Status:** eigen (teilgebaut) | **Bindung:** eigen
- **Trigger:** kein — die Quelle ist gemessen; der nächste Schritt (Ionengeschwindigkeits-Kanal + Feld-Eval-Injektion) ist benannt.
- **Lage:** (gemessen 2026-10-13 river-204) der **CPU-Referenz-Resolver** + die **Grammatik** (Schritt 1–5: channel-Alias/charge, coupling-Block, constraint-Block, Gate-Fixtures, QN-Enforcement/δn) sind gebaut. **Korrektur des Quellen-Wegs:** die vom Rat genannten `IRI-2020/NRLMSISE-00/IGRF` sind **Modelle** (A=A → nicht das Register) — die **gemessenen CHAMP-Kanäle liegen schon im Register**: Neutral-Dichte `champ_thermosphere_density_kgm3` (VirES-HAPI, `phi:42`, `channel` `:44`; `--verdict` HTTP 200) · geladene Dichte `champ_plpt_electron_density_cm3` (GFZ-ISDC, Compiler + `extract.rs`-Arm vorhanden, `phi:11541`) · Wind `champ_thermosphere_crosswind_ms` (`phi:57`/`:59`). Alle von **CHAMP** → co-temporal/co-lokal. **Neu gebaut (river-204):** die Produktions-Naht **`ChannelCouplingLayer::from_sources(&[SourceConfig], content)`** (`src/archivar/parse.rs`) — aggregiert `source.channels` in eine `ChannelRegistry` (inkl. Aliase), baut `ConstraintRegistry` + `CouplingRegistry` aus `parse_constraints`/`parse_couplings`; Test `a_channel_coupling_layer_resolves_the_register_aliases`, `cargo check` 0/0, rustfmt clean.
- **Blockade:** (1) die **Ionengeschwindigkeits**-Kanal fehlt (CHAMP-IDM Ionen-Drift ist nicht registriert); die `coupling`-Zeile braucht ihn als `from` (der Wind-Kanal steht als `to`). (2) `K` ist noch nicht gemessen (Rate bleibt `pending`). (3) die **Feld-Eval-Injektion** (WGSL Slot (i) `shaders.rs:450–451`) + der Produktions-Aufrufer der Naht sind noch nicht gebaut; `parse_sources`-Konsumenten liegen in fremd-dirty Dateien (`src/archivar/fetch.rs` · `src/archivar/main_flow.rs`), darum nur die Naht in `parse.rs`.
- **Braucht:** (1) den **CHAMP-IDM-Ionen-Drift** als Kanal registrieren (Comp. + Format-Arm + `channel … alias … charge 1`) — Quelle messen (ISDC GFZ `CH-ME-2-IDM` / VirES); (2) den Produktions-Aufrufer der Naht + die WGSL-Injektion + CPU/GPU-Parität (`ΣρΔu ≡ 0`); (3) `K` aus dem gemessenen Overlap bestimmen.

### Kopplung Ion-Neutral — Riss getragen: co-lokale Route DE-2 (Rat-Verdikt 2026-10-13)
- **Status:** riss (getragen) | **Bindung:** eigen
- **Trigger:** der Bau der DE-2-Injektion (Naht-Aufrufer über `exchange_resolved` + WGSL-Slot) — dann wird die Kopplung messbar; bis dahin trägt der Riss.
- **Lage:** (gemessen 2026-10-13 via 3 Taucher + Rat) CHAMPs co-lokale Ionengeschwindigkeit (DIDM) ist **nicht offen archiviert** (`isdc-data.gfz.de/champ/ME/Level2/IDM/…` HTTP 404 → `blocked account`, kein Quellen-Verdikt). DMSP-SSIES-Ionendrift ist offen (NCEI HTTP 206), aber ~840 km vs CHAMP ~400 km → **kein per-Punkt-Paar** (Riss). Rat-Konsens: die lokale Kopplung bleibt `riss`/`pending`; die Auflösung ist eine **co-lokale Route**, nicht eine Cross-Mission-Paarung (Cross-Mission nur als deklarierte Klimatologie `conjunction:<alt>/<mlt>`, nie Kopplungszeile). Die **co-lokale Route ist gemessen:** **DE-2 `de2_ua16s_all`** (HAPI `DE2_UA16S_ALL`, CDF, 16 s, 1981-08→1983-02, HTTP 206, HAPI-JSON live) trägt auf **einer** Plattform Ionendrift (`RPA_IVE/N/UionDrift`) + Neutralwind (`WATS_NVE/NVUneutralWind`, `FPI_FVNneutralwind`) + Neutraldichte (`NACS_N2/O/HE/AR/N`); fällt in den vorhandenen CDAWeb-HAPI-Pfad (`hapi_csv`, Vorlage `phi:631`) — kein neuer Parser nötig. C/NOFS CINDI = Ionendrift only (kein offener Neutralwind) → Ergänzung, nicht das Paar. Rat-Blatt: `state/stimmen/2026-10-13-river-p71-ion-drift-riss-rat.md`.
- **Breite Suche (`archive_search --all`, 1101 Zeilen, gelesen 2026-10-13; Lehre: nicht nur oberflächlich/gezielt):** bestätigt DE-2 als einzige offene **co-lokale In-situ**-Route. Weitere gemessene Kandidaten: **ICON** (MIGHTI-Neutralwind + IVM-Ionendrift, 2020-22, CDAWeb) — trägt, aber MIGHTI misst den Wind **remote** (Limb-Sounder), nicht in-situ → **kein** Plattform-Paar; **C/NOFS CINDI** (IVM + NWM-Instrument) — der offene Archivbestand fehlt (bestätigt); **e-POP** (EFI/IRM, Ionendrift); **AE-C/D/E** (klassischer In-situ-Satz); **SuperDARN-Meteor-Wind** (`zenodo 20767153`, 1993-2024) + SuperDARN-Ionendrift = Radar-**Konjunktion**, kein Plattform-Paar. Crossref `10.5194/angeo-41-339-2023`: die einzigen umfassenden co-temporalen/co-spatialen In-situ-Sätze sind **AE-C/D/E und DE-2**.
- **Entscheidung (river-204, was gehalten wird):** **DE-2 primär** (baubar — co-lokal, offen). **C/NOFS CINDI zweit** — konzeptionell genau das Paar (Operator-Wort), aber der offene Bestand (CDAWeb) trägt nur das **IVM** (Ionendrift); das **NWM** (Neutralwind) ist nicht offen archiviert (`cindispace.utdallas.edu` = Bildungsseite, HTTP 200, kein Datenarchiv; CDAWeb `cnofs/cindi/` = nur `ivm_*`) → **Zugangs-/Antrags-Aufgabe** (Dritter → Operator, `blocked account`). **AE-C/D/E Reserve** (historisch co-lokal, noch zu messen). **Als Paar descoped:** ICON (nur IVM als Ionendrift-Supplement; MIGHTI remote) · SuperDARN-Meteorwind (nur als deklarierte Radar-Konjunktion). Der Bau läuft auf DE-2; CINDI hält die Tür offen.
- **Lage (fortgeschrieben):** DE-2 ist **registriert** (`phi/sources.φ:61-72`, HAPI `DE2_UA16S_ALL`, festes Fenster 1981-08-06…1983-02-19, 5 `field` + 2 `channel`-Aliase `de2_ion_drift` (Medium `plasma`, `charge 1`) / `de2_neutral_wind` (Medium `fluid`) — die Medien trennen die zwei sonst hash-gleichen Kanäle).
- **Lage (fortgeschrieben 3, 2026-10-13; Rat-Verdikt):** Dichte-Kanäle + `constraint de2_qn` + `coupling`-Zeile stehen (`phi/sources.φ:71-77`); `cargo check` 0/0. **Rat-Korrekturen:** (i) der `mass:`-Ref auf der DE-2-Neutralseite war ein **Bug** — das Feld trägt *Zahl*-Dichte, nicht Massendichte; korrigiert auf `density_to number:de2_neutral_density:4.65e-26` (N2; `density_from number:de2_ion_density:2.66e-26` O⁺). (ii) Der Einheiten-Riss (b) **ist keiner** — die `cm-3`→`m-3`-Konversion fließt im Harvest-Arm (`extract.rs:6905`, `units.rs:86`); `mass:` bleibt für echte kg/m³-Slots. (iii) **Getragener Riss (a):** `ChannelDescriptor::hash()`/`PartialEq` (`channel.rs:396-409,558-579`) tragen **keine Spezies** → der DE-2-N2-Kanal ist hash-gleich mit dem CHAMP-Gesamtdichte-Kanal (`phi:44`), `register` dedupt still; **die Spezies muss eine Descriptor-Achse werden** (`species: Option<…>` in `PartialEq` + `hash()`), nicht `Conserved`/`Medium`/Alias. Folgen: `schema_hash` verschiebt (`channel.rs:1520,1840`) → alle hash-gekeyten Konsumenten neu; und `unit_token` (`channel.rs:2092`) hat keinen Zahl-Dichte-Token (`m^-3`).
- **Blockade:** CHAMP-IDM-Ionendrift ist nicht offen archiviert (`blocked account isdc-data.gfz.de`, HTTP 404, kein Quellen-Verdikt).
- **Braucht:** (1) **Spezies-Achse gebaut** (river-205: `species: Option<String>` in `ChannelDescriptor` + `hash`/`PartialEq`, `m^-3` in `unit_token`, `species <token>` in der `channel`-Direktive, `~species=` im `spec_token`; DE-2-N2-Kanal auf `species N2`/`m^-3`, DE-2-Ionendichte auf `m^-3` — die kg/m³-Kollision mit dem CHAMP-Gesamtdichte-Kanal ist damit gelöst; `cargo check` 0/0, rustfmt clean, Test `the_species_enters_the_channel_identity_and_round_trips`). **Offener Rest-Vokabel-Riss:** die Gesamtdichte (CHAMP, DE-2-Ionendichte) = `species None` vs. explizite Aggregat-Spezies (Register-Wort, offen). (2) Produktions-Injektion über **`exchange_resolved`** + CPU/GPU-Parität; (3) `K` aus der geordneten Reihe.
- **Rat-Vorbedingung gemessen (2026-10-13, `explore`):** die Spezies-Achse berührt (a) die `Conserved`-Switches (`channel.rs:8,15,149,396,558,581,636,706,733,756,970`; `parse.rs:1314,1463,1573`); (b) die `schema_hash`-Konsumenten — **Wire-/GPU-Kontrakt**: `relay.rs:1121` (Frame-Header u32), `relay.rs:1089` (`kinetic_frame_bytes`), `omega.rs:405` (`PresenceFrame.schema_hash`), `channel.rs:1520,1840` (`live_schema_hash`); (c) `unit_token` (`channel.rs:2092`, 7 Tokens — kein `m^-3`); (d) die `ChannelDescriptor`-Konstruktion. **`tools/` trägt null Referenzen** zu `ChannelDescriptor`/`Conserved`/`schema_hash`/`unit_token`; in `src/` bauen fast alle über `::new` (Rest-Vorbehalt: `ChannelDescriptor { … }`-Literale prüfen). **Folge: der `schema_hash` verschiebt sich → das Wire-/GPU-Schema ändert sich** — ein bewusster Kontrakt-Sprung, kein Nebenprodukt.
- **CINDI-`--all` nachgezogen (897 Zeilen, 2026-10-13):** kein offener NWM-Bestand (bestätigt; `cindispace.utdallas.edu` = Bildungsseite). Der NWM bleibt die Zugangs-/Anfrage-Aufgabe (Operator).
- **CINDI-NWM-Zugangsweg gemessen (river-205, `general`-Taucher + `--verdict`):** NWM (CTWS+RWS) gebaut am **UTD Hanson Center** (PI R. A. Heelis, historische Adresse `heelis@utdallas.edu`); **kein offenes/request-basiertes NWM-Archiv** (CDAWeb/SPASE/Datacite = nur IVM); das historische Portal `cindispace.utdallas.edu` ist tot (Login 404, jede Pfad → Bildungsseite). **Entwurf an die Kante:** `state/mail/2026-10-13-cindi-nwm-anfrage-entwurf.md` (QUELLEN-Block, **nicht gesendet**). **Offener Rest:** die aktuelle UTD-Kontaktadresse (`archive_search --playwright` auf `spacesciences.utdallas.edu`, JS-gerendert) — danach ist der Punkt `operator-gebunden`, der Send die Operator-Hand.
- **Register-Schreibakte:** auf Operator-Wort trägt **River** die Mountain-Register-Zeilen + die Mycelium-Manifestation (Operator-Wort 2026-10-13).

### P10 flache Tokens → Kanal-Deskriptor (`force_id_of`/`FIXED_CHANNELS`/Modul `force` weg) — gated
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Mountains `field`-Zeilen-Migration (`phi/sources.φ`: 7943 flache `field`-Zeilen → Deskriptor-Form, P10.3) — dann sterben der flache Parser-Branch (`parse.rs:1600`) und die `force_id_of`-Compiler, River zieht die flachen Tokens auf den Deskriptor.
- **Lage:** (gemessen 2026-10-12 river-199 am Baum) **Schritt 1 (Rename, layout-neutral) + Schritt 2 (Sentinel 255 → Presence-Bit) gebaut:** `FieldConfig.channel_ref: Option<u8>`, `Sample.channel_ref: Option<f64>`; `CHANNEL_REF_QUANTITY` und `wire_channel_ref` gelöscht; „kein Kanal" trägt allein `PRESENCE_FLAG_QUANTITY` (Bit 16, Wire-Pad über `wire_channel_ref(Option<f64>)`, Apertur über `ApertureSet::for_channel`); 57 Dateien (types/spatial/parse/fetch/extract/channels/main_flow + ~40 `declared_fields`-Compiler); `cargo check` 0/0. **Noch `force` im Baum:** die Brücke `descriptor_for_force`/`descriptor_for_channel_ref`/`channel_ref_of_descriptor` (`channel.rs:930-958`) + `ChannelRegistry::alias`/`resolve` + die zwei Legacy-Tests (`channel.rs:2004/2022`); `force.rs` (`force_id_of`/`force_name_of`/`FIXED_CHANNELS`-Re-Export, 76 `force_id_of(`-Stellen in 45 Dateien; 6 davon in `tools/harvest/src/bin/*.rs`) + `FIXED_CHANNELS` (9 Tags, `channel.rs:873`); `gate/commit_gate.rs` + die `omegaflow::force`-Re-Exports. **Gemessener Riss:** `phi/sources.φ` trägt 9 `channel`-Direktiven (nach Hash-dedup 7 lebende), die Registry registriert in Quell-Reihenfolge, der Parser vergibt den Handle über die feste 9er-Liste — ab dem zweiten Kanal fallen Parser-Index und Laufzeit-Registry-Index auseinander; der register-sourced Handle-Rebase ist river-199 `f2623bb72`.
- **Blockade:** `phi/sources.φ` trägt noch 7943 **flache** `field`-Zeilen, 0 Deskriptor-`field`-Zeilen (Mountains P10.3).
- **Braucht:** nach Mountains `field`-Migration den flachen Parser-Branch `parse.rs:1600` + die `force_id_of`-Aufrufe (Parser + 6 Compiler unter `tools/harvest/src/bin`) streichen und auf `descriptor_from_axes`/`ChannelRegistry::handle_of` ziehen; `FIXED_CHANNELS`/`force_id_of`/`force_name_of`/Modul `force` + `gate/commit_gate.rs`-Re-Exports auflösen (P10.4-Gate: kein `force`-Feld ohne `quantity`). Verifikation `cargo check` 0/0; CI-`ci-gate` ist das Testnetz.
- **Faltung `xfer:mountain329-3` (Sender mountain-folge330):** der Register-Nenner ist geräumt — `phi:27`→`seismic-body`-Kanal, `phi:164`→`electric`-Kanal (`7aa6fdd32`), 9 `channel`-Zeilen / 9 unique; der Handle-Rebase auf die Register-Reihenfolge ist river-199 `f2623bb72`. Der register-sourced Handle ist baubar; er stirbt wie der flache Branch erst mit Mountains `field`-Zeilen-Migration (P10.3).

### Prosa-Träger mountain327-Abgabe — Marker abgearbeitet, Rest offen
- **Status:** eigen (Carrier-Duty) | **Bindung:** eigen
- **Trigger:** kein — die Träger stehen.
- **Lage:** (gemessen 2026-10-12 river-199) die drei `--orphan-docs`-Dokumente sind gefaltet und ihre Marker abgearbeitet: `docs/blatt/blatt-gic-breitenband-familien.md` (4→2; offen: IGRF-14/AACGM-v2-Rust-Bin, Route-C-Stufe-2-Member-Pool) · `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (4→2; offen: CTBTO-vDEC-Antrag, Rest) · `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md` (2→0).
- **Blockade:** keine.
- **Braucht:** die zwei verbleibenden Marker je Blatt/Survey abarbeiten, sobald ihr Gegenstand gemessen ist (aacgm-Bin = Bau; CTBTO = Operator-Antrag).
- **Tokens (Sender mountain-folge330):** `xfer:mountain327-3` · `xfer:mountain327-4` · `xfer:mountain327-5` — gefaltet (Marker 4→2 / 4→2 / 2→0); die zwei verbleibenden Marker tragen weiter.

### 14-min-Lücke Magnetometer→Optik — im 2D + Ganznacht gemessen, Pick-Riss getragen
- **Status:** eigen (getragener Riss) | **Bindung:** eigen
- **Trigger:** der GIC-Onset-Relationsschritt auf der Meridian-Spur (Blatt/Antwortseite).
- **Lage:** (gemessen 2026-10-12 river-197 + river-200, Anker `state/river/themis/frame-onset-2026-10-12.md` · `overlay-night-2026-10-12.md`) **2D (Tool `themis_asi_frame_onset`, Cube `fodg_20240510_kapu_onset.bin`):** kein kohärenter off-meridian optischer Eintritt bei 07:41; der optische Onset ist ein einzelnes azimutal wanderndes Band 07:54:45→07:55:45 (az 155–240, el 45–55). **Ganznacht-Overlay (Lauf `38259065347` success → CDN-TKDG `themis_asi_timed_keogram_20240510_kapu.bin`, 3 826 168 B sha `5a59429f…`, 7358 Frames 02:21:12–08:29:03Z, `mag_optical_overlay`):** Stundenmittel 03–06 ≈ 3310–3343 (ruhige Nacht), 07 = 3370.6, Ränder 02 = 3895 / 08 = 3911; größter Optik-Wert 5623.5 @ 02:21:12Z (Rand-Kandidat, erster Frame); YKC-Spitze 52.37 @ 07:20. Der Hauptsturm liegt 17:00–23:00 UT (Tageslicht). **Riss:** der river-196-Pick „07:41" ist nicht eindeutig — Stunde 07 trägt eine Kette (07:13–07:48), größte YKC-Spitze bei 07:20 (52.4); die „+14 min 27 s"-Lücke ist pick-abhängig.
- **Blockade:** keine (2D-, Tages- und Ganznacht-Messung stehen).
- **Braucht:** den Riss tragen (Phasendifferenz + Pick-Abhängigkeit). **CDN-Duty gefaltet:** mycelium-333 meldet das Compiler-Asset registriert (`phi:21027`), Lauf `38258632187` success; Token `xfer:mycelium.ses_eceb3d78fffeTNxaWJuGfFG211.6~3ed6b902` (2026-10-12 gefaltet).

### Weberin-Zeugen-Fäden — Inventar konsolidiert (kein GIC-Silo)
- **Status:** eigen (Admission offen) | **Bindung:** eigen
- **Trigger:** Operator-Wort zur Aufnahme; dann je Faden Admission (Mountain) + Arm/Manifestation (Mycelium).
- **Lage:** (gemessen 2026-10-12 river-200; konsolidiert in `state/river/weberin-faden-inventar-2026-10-12.md`) die Zeugen-Fäden liegen über **vier** Träger: `docs/surveys/survey-2026-10-02-weberin-zweite-linie.md` · `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` §2 · `state/future/gic-riss-154-wunschliste-2026-10-06.md` §A–G (33 Fäden) · die Erweiterung `state/river/gic-wunschliste-erweiterung-2026-10-12/` (Riometer · Ionosonde/ISR · thermisch/neutral · BARREL/ELF-VLF). **Register-Stand 2026-10-12** (`sgrep -c <term> phi/sources.φ`): admitted GOES 44 · superdarn 22 · champ 14 · image 14 · carisma 8 · dmsp 7 · themis_gmag 4 · madrigal 5 · hfrnet 5 · ampere 5 · poes 5 · awesome 4 · vlf 5; **noch 0**: barrel · emfisis · riometer · ionosonde · eiscat · pfisr · kaira · norstar · cgsm · airglow · saber · pithia · jicamarca · fabry · interferometer. GIC ist **kein** Silo: dieselben Fäden tragen die allgemeine Thread-Matrix/Vlies (`docs/concepts/die-weberin.md:64`); kein GIC-eigenes Register, kein gic-eigenes Werkzeug.
- **Blockade:** keine.
- **Braucht:** je Faden die Quellen-Zeile/`channel`-Direktive (Mountain) + Arm/Manifestation (Mycelium), in der allgemeinen Channel-Etikettierung (Quantity | Mechanism | Medium, P10); GIC fährt sie als `field_te_query`-Störung, nicht als eigenes Netz.
- **Rat-Stand (river-200, Runden 1–3 + Bau):** `Plasma` ist die vierte Medium-Klasse `{Vacuum, Fluid, ElasticSolid, Plasma}`; Ionosphäre/Thermosphäre sind Region-Datums-Werte, nie Klasse; `domain` bleibt Geometrie. (a) Plasma-Regime = Parameterfeld innerhalb `Plasma` (keine Unterklassen); (b) `medium` = pro Pfadsegment (Deskriptor-Kette); (c) sparse Paar-Matrix in `CouplingRegistry`; (d) Grenzfläche = `InterfaceRegistry`; (e) `PdeRegime` + `PlasmaParameters`. Blätter: `state/stimmen/2026-10-12-river-medium-kinds-rat-roster.md` · `…-plasma-regime-granularity-…` · `…-p71-coupling-interface-regime-…`. Getragene Risse: Kopplungseinträge nicht unabhängig (`ρ_i·ν_in = ρ_n·ν_ni`); scharfes Interface vs. dünnes Segment; Regime-Abbildung nicht 1:1.

### Netz-End-Validator — GIC↔Onset-Bin; MCON manifestiert, Register-Drift gelöst
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** `bpa_gic` deckt den MCON-Zeitraum (1999–2024) UND die `format mcon_event`-Registerzeile steht (Mountain).
- **Lage:** (gemessen 2026-10-12 via `ci_manage log 38176003927` + mycelium-318) **MCON manifestiert:** `mcpherron_onsets.bin`, 314 555 Onsets, `format mcon_event`, sha `441df5f8…`. **Register-Drift gelöst** (mycelium-318). `bpa_gic` führt nur ein **4-Tage/5-min-Fenster** (2026-10-03..06), **kein Archiv** → 0 Onsets im MCON-Fenster; die zweite GIC-Zeugenreihe bleibt leer, die Relation läuft über die FMI-Reihe.
- **Blockade:** `bpa_gic` deckt den MCON-Zeitraum nicht (kein Archiv; nur FOIA).
- **Braucht:** (a) `bpa_gic`-Harvest auf 1999–2024 ausdehnen (Mycelium) — oder den MCON-Zeitraum verwerfen und die Relation auf die FMI-Reihe stützen; (b) die Surrogat-Null-Form vor jeder Behauptung durch den Rat.

### wy-max-t — α als Family-Bound gemessen; Blatt-Siegel offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Siegel auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`.
- **Lage:** (gemessen 2026-10-12 via `gh run download 38166787414` + `wy_max_t_probe --combine`) der Westfall–Young-max-T-Null ist gemessen, α = 0.05, pooled B = 9999: (1-α)-Quantil **ABK-2024 = 2.4467 · ABK-2025 = 2.4433 · SOD-2024 = 2.4831** (Selftest 2.5856). family-clearing: `Bz→dB/dt` (3/3), `dB/dt→Speed` (3/3), `dB/dt→Density` (3/3), `dB/dt→Bz` (ABK-2024, SOD-2024), `Speed→dB/dt` (ABK-2025). α steht im Blatt (`:78-85`).
- **Blockade:** das Blatt-Siegel ist Operator-Wort.
- **Braucht:** Operator-Siegel; die gemessenen Werte im GitHub-Artefakt `38166787414` in den Bestand (Mycelium).

### Receiver-Apertur (SPAN) — Schritt 3 offen
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** der nächste `ci-gate`-Lauf (Paritäts-Gate `field_spatial_fold_wgsl_cpu_parity`); danach Schritt 3.
- **Lage:** (gemessen 2026-10-12 via `cargo check` 0/0 + `git log` `5a3e629e7`) **Schritt 2 (Faltung)** gebaut und committet: `σ_eff² = extent² + global_scale²` in `field_spatial`/`field_spatial_grad` (WGSL) + CPU-Spiegel; `max(extent, scale)` gestrichen; Kontrakt `:34` nachgezogen.
- **Blockade:** das Paritäts-Gate ist ein Test-Target — `cargo check` kompiliert `#[cfg(test)]` nicht lokal; es läuft erst im CI.
- **Braucht:** (a) den nächsten `ci-gate`-Lauf lesen (`ci_manage log`), das Paritäts-Gate bestätigen; (b) **Schritt 3:** die deklarierte `ReceiverAperture` (`src/mathematikerin/receiver.rs`) an die Query-Naht (`vp.surface.w = global_scale`) binden; (c) die Operator-Browser-Messung (Fenster-Edit = Operator-Wort).

### GIC-Onset — AL-Einhüllende über die 154-Station-Familie
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** Abschluss der Harvest-Läufe `intermagnet-xyz-cdn` → die Assets `<code>_xyz_1m.bin` liegen im CDN.
- **Lage:** (gemessen 2026-10-12 via `ci_manage jobs`/`log 38164890620`) der Lauf trug 25 Stationen (`start 2000`, `end 2024`): 24 success, **`lov` cancelled** (LOV-Fetch HTTP 400 für 2023+); `38164966250` queued. Die Kette `intermagnet_xyz_compiler` → `al_network_proxy` → `gic_onset_detector` ist gebaut; die 154-Station-Partition liegt in `state/river/gic-family-*.txt`; Null-Baseline gemessen (`--null 200`: 1.62×/1.47×/1.35×).
- **Blockade:** die per-Station-`<code>_xyz_1m.bin`-Assets der noch fehlenden Stationen (24 fertig; `lov` 400).
- **Braucht:** `ci_manage log 38164966250 --all` nach Abschluss; dann je Familie `al_network_proxy --family <fam.txt> --dir <pfad> --decl auto --baseline-block 60 --min-stations 2` und `gic_onset_detector --null 200`.

### GIC-Familien-Partition — zwei Register-Nägel, dann die Matrix-Läufe
- **Status:** blockiert | **Bindung:** eigen (Blockade: Mountain-Nägel)
- **Trigger:** die zwei Register-Nägel sind gesetzt (Mountain), dann Run der Matrizen.
- **Lage:** (gemessen `state/stimmen/2026-10-06_gic-familien-arm-ui-stimmen.md:20-80`) die Stimmen-Runde (GLM-5.3, qwen 3.8 Max, Kimi) einig: **kein** neuer Grammatik-Arm, sondern (i) ein Satz über Querfamilien-Zellen und (ii) die Familienregel als deklariertes `station→Familie`-Artefakt. Minimaler Patch: `matrix gic full` + `family auroral|subauroral|midlat stations <code…>` + `expect families 3` + `fdr by 0.05 over matrix` + `wymaxt 0.05 over family`.
- **Blockade:** die zwei Nägel (Mountain-Register); der Grammatik-Patch ist aus dem Verdikt noch nicht gebaut.
- **Braucht:** Mountain setzt die zwei Nägel; danach den Patch bauen und `matrix gic_auroral|gic_subauroral|gic_midlat full` fahren.

### Newell-OMNI-Alignment — Doppelregistrierung von Mountain geheilt; Zelle neu messen
- **Status:** eigen (Messung offen) | **Bindung:** eigen
- **Trigger:** der nächste `field-te-query.yml`-Lauf (`matrix-newell-omni`).
- **Lage:** (gemessen 2026-10-13) das CDN-Asset `omni2_serie_1h.bin` ist **HTTP 206** (`phi:1836`); die `omni_imf_bz_gsm_nt`-Doppelregistrierung ist **von Mountain geheilt** (`1dd4e3549` — der Live-Block ist auf `..._live` umbenannt, das Archiv trägt den kanonischen Namen, den der Newell-Descriptor bindet; mein `xfer:river.river-203.1`-Block ist gefaltet). Die Zelle `newell_dphi_dt_omni->intermagnet_dbdt` ist seither nicht neu gemessen.
- **Blockade:** die Zelle ist CI-only (`field-te-query.yml` Job `matrix-newell-omni`, `workflow_dispatch`, nie lokal).
- **Braucht:** `gh workflow run field-te-query.yml` → Artifact `field-te-matrix-newell-omni` lesen; bei weiterer Fenster-Disjunktheit den Descriptor auf ein gemeinsames Fenster ziehen.

### Flyby-Kette — recon bleibt (`ephemeris-juice-recon` + `flyby-path2-recon`)
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** `ephemeris-juice-recon` Wiedervorlage 2026-11-01 (ESA/ESOC publiziert die Post-Flyby-SPK). Wahrheit: `state/zustand/wartend.φ`.
- **Lage:** (gemessen 2026-10-11) die Wiedervorlage 2026-11-01 steht; `data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin` absent. `flyby-path2-recon`: orbc-SPK keyless `https://spiftp.esac.esa.int/data/SPICE/JUICE/kernels/spk/juice_orbc_000116_230414_310721_v01.bsp` (HTTP 206); die **1-σ-Kovarianz liegt nicht im SPICE-Baum**.
- **Blockade:** ESA/ESOC-recon-SPK + die 1-σ-Kovarianz absent.
- **Braucht:** am 2026-11-01 `archive_search --verdict` auf den ESOC-recon-Pfad; dann `flyby_ephemeris_gate --recon <recon.bin> --sigma-recon <km>`.

### Blatt-Kalibrierung — Z-Null ABK gelesen (Z = 58 min); Signifikanz-Null offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort (Siegel) auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`; der Signifikanz-Null (Blocklänge) ist bestimmbar.
- **Lage:** (gemessen 2026-10-12) der BCA-Z-Null über die ABK-Reihe: `Z = 58 min`.
- **Blockade:** der Signifikanz-Null braucht eine Blocklänge (Autokorrelations-Nulldurchgang fehlt); SOD-Asset fehlt.
- **Braucht:** (a) Blocklängen-Regel für dieses Korn benennen (Rat) oder den Deskriptiv-Wert tragen; (b) SOD-`dB/dt`-Asset (Mycelium); (c) α/Bz-Schwelle + Siegel = Operator-Wort (nach Future geroutet).

## Abschluss

river-205 (Spezies-Achse + CINDI-Kante): gearbeitet ohne Pro-Solo, ohne Fenster-Edit, ohne Send.
**Spezies-Achse gebaut** (Operator-Wort „Mountain ist beschäftigt, mach du das selbst"): `species: Option<String>`
als Descriptor-Achse in `PartialEq` + `hash()`, `m^-3` in `unit_token`, `species <token>` in der
`channel`-Direktive, `~species=` im `spec_token` (round-trip). Der DE-2-N2-Kanal entkollidiert vom
CHAMP-Gesamtdichte-Kanal (`species N2` + `m^-3`); die DE-2-Ionendichte auf `m^-3`. Test
`the_species_enters_the_channel_identity_and_round_trips`; `cargo check` 0/0, rustfmt clean. Bewusster
Wire-Schema-Sprung (`schema_hash` ändert sich) — kein externer Konsument assertet den Wert (`tools/` = 0 Refs,
kein `.js` liest `schema_hash`). **CINDI-NWM:** Zugangsweg gemessen (UTD Hanson Center; kein offenes
NWM-Archiv), Anfrage-Entwurf an die Kante (`state/mail/2026-10-13-cindi-nwm-anfrage-entwurf.md`, nicht
gesendet); offener Rest = die aktuelle UTD-Kontaktadresse. **Nächster Schritt:** DE-2-Naht-Aufrufer über
`exchange_resolved` + WGSL-Injektion + CPU/GPU-Parität; dann `K`.

river-204 (Messwert-Kette): gearbeitet ohne Pro-Solo, ohne Fenster-Edit, ohne Send.
**Gemessen und korrigiert:** der Rat-Dichte-Weg (`IRI-2020/NRLMSISE-00/IGRF`) sind **Modelle** →
überholt (Operator-Wort); die **gemessenen CHAMP-Kanäle** (Neutral-Dichte `champ_thermosphere_density_kgm3`,
geladene Dichte `champ_plpt_electron_density_cm3`, Wind `champ_thermosphere_crosswind_ms`) sind
schon registriert, co-temporal. **Produktions-Naht gebaut:** `ChannelCouplingLayer::from_sources`
(`src/archivar/parse.rs`, channels+couplings+constraints), Test
`a_channel_coupling_layer_resolves_the_register_aliases`, `cargo check` 0/0, rustfmt clean.
**Newell:** Doppelregistrierung von Mountain geheilt (`1dd4e3549`), mein `## An mountain`-Block
nach Echo entfernt; die Zelle ist CI-neu-zu-messen. **Nächster Schritt:** CHAMP-IDM-Ionen-Drift-Kanal
registrieren → Kopplungs-Zeilen → Naht-Aufrufer + WGSL-Injektion → `K`.

river-203: gearbeitet ohne Pro-Solo, ohne Fenster-Edit, ohne Send. **QN-Alias-Enforcement
+ δn-Diagnose gebaut** (River-Session + 1 `grind-flash`): `ConstraintRegistry` (Duplikat-Verweigerung),
`Coupling.quasi_neutral: Option<String>` (Constraint-Name erhalten), `from_specs(…, constraints)`
verweigert undeklarierte `quasi-neutral`-Referenzen und Region-Mismatch, `charge_imbalance`
(`Σ Z_c·ρ_c`, absent bei unauflösbarem Mitglied); 4 Tests, `cargo check` 0/0,
`rustfmt --edition 2024 --check src/mathematikerin/channel.rs` clean. **mycelium-337-Block
gefaltet** (omni2-Asset HTTP 206, kein CDN-Lauf nötig; `xfer:mycelium.ses_…576548f1` ge-echot).
**Newell-OMNI-Blocker als Register-Drift gemessen** (Doppelregistrierung `omni_imf_bz_gsm_nt`),
als tokenisierter `## An mountain`-Block gesetzt (`xfer:river.river-203.1~0e4958d2`) — der
Transport-Riss der Vorsession (Prosa-nach-Mountain ohne Token) ist damit für diesen Punkt geschlossen.
`--fired river` = 0, `--stale --persist 3` = 0, `open_points_check` folge202 = 0 absent /
0 stale-citations / 0 format-gaps / 0 owner-drift. Kein Pro/max-Dispatch, kein `smail --send`,
kein Window-Edit; `/consent` ist der session-weite Delegations-Consent, nie das Commit-Wort.
