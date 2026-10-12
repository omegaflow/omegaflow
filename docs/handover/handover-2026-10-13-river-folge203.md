<!--
  title: Handover — River-Folge 203 (2026-10-13)
  session: River-Linie in einem Pass (river-203) — QN-Alias-Enforcement + δn-Diagnose gebaut (ConstraintRegistry, from_specs-Alias- und Region-Enforcement, charge_imbalance; cargo check 0/0); mycelium-337 omni2-Block gefaltet (Asset HTTP 206, kein CDN-Lauf nötig); Newell-OMNI-Blocker als Register-Drift gemessen und als tokenisierter `## An mountain`-Block gesetzt; folge202 konsumiert/archiviert
  class: handover
  date: 2026-10-13
  sha256: bd9b74835df7182781f25bc9dda4a17050c4d131ad3704e071598c0d245f4c67
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

## Burn: open 0.00 · close 0.29 · cap 0.50 — Grund: session river-203 (deepseek-flash, kein pro/max); 1 `grind-flash` (QN-Enforcement-Bau) + 1 `general` (Newell-OMNI-Lauf/Asset-Messung); die close-Zahl ist das gemessene Haus-Delta ($2.7047 → $2.9935, line-Bucket $1.3159). Fälligkeit: `--fired river` = 0, `--stale river --persist 3` = 0, `open_points_check` folge202 = 0 absent / 0 stale-citations / 0 format-gaps / 0 owner-drift (9 word-carried = Register-Zitate, keine Drift). 1 adressierter Token gefaltet (mycelium-337 omni2). Kein `smail --send`, kein Window-Edit. `git_safety --snapshot` `refs/safety/1791844786`.

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

Verbatim: `state/operator-gespraeche/2026-10-12-river.md`, `2026-10-11-river.md`,
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

### P7.1 Kopplung/Interface — Grammatik + QN-Enforcement gebaut; Regime-Zeilen (Mountain) wartend, WGSL gated
- **Status:** eigen (teilgebaut) | **Bindung:** eigen
- **Trigger:** Mycelium liefert die Plasma-Dichte-Kanäle + `K` live — dann setzt Mountain die Regime-Zeilen; danach die WGSL-Parität.
- **Lage:** (gemessen 2026-10-13 via `cargo check` 0/0 + `git log`) der **CPU-Referenz-Resolver** ist gebaut (`src/mathematikerin/channel.rs`: `DensityKind`/`DensityRef`/`Coupling.density_from|density_to|quasi_neutral`, `coefficient` = K, `density_at` + `exchange_resolved`). Die **Grammatik** ist gebaut (Schritt 1–4). **Neu gebaut (river-203, Schritt 5):** die **QN-Alias-Enforcement + δn-Diagnose** — `ConstraintRegistry::from_constraints` (Duplikat-Name verweigert, erster gewinnt) · `Coupling.quasi_neutral: Option<String>` behält den Constraint-Namen (vorher bool) · `CouplingRegistry::from_specs(specs, channels, constraints)` verweigert einen Kopplungseintrag, dessen `quasi-neutral <name>` keinen deklarierten Constraint nennt oder dessen `region` nicht der Constraint-Region entspricht · `charge_imbalance` = `Σ Z_c·ρ_c` über die `density_refs` aller zitierenden Kopplungen, `None` (absent) bei fehlender Ladung/Dichte (kein Teil-Summen-Fabrikat). 4 Tests im `#[cfg(test)]` (`constraint_registry_refuses_a_duplicate_name` · `from_specs_refuses_an_undeclared_quasi_neutral` · `from_specs_refuses_a_quasi_neutral_region_mismatch` · `charge_imbalance_sums_charge_times_density`) — laufen im CI (`cargo check` kompiliert `#[cfg(test)]` nicht lokal).
- **Blockade:** (1) `K` + die Plasma-Dichte-Kanäle (IRI-2020/NRLMSISE-00/IGRF-Alias) sind `pending` (Mycelium) — **kein `coupling`-Datenwert wird geschrieben, solange sie nicht live auflösen** (die Register-Zeile bleibt `pending`); (2) die WGSL-Parität wartet auf eine Dichte-Quelle.
- **Braucht:** (1) Mountains Regime-`coupling`/`constraint`-Zeilen, sobald die Plasma-Kanäle + `K` live auflösen; (2) WGSL-Parität + die Region-/Injektions-Ort-Messung (Slot (i) `shaders.rs:450`).

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

### Newell-OMNI-Alignment — Asset präsent, Blocker = Register-Drift (Doppelregistrierung)
- **Status:** eigen (offen) | **Bindung:** eigen
- **Trigger:** kein — die Lage ist gemessen; die Doppelregistrierung ist als tokenisierter `## An mountain`-Block transportiert.
- **Lage:** (gemessen 2026-10-13 via `archive_search --verdict` + `general` gegen den Baum) der Resolver ist gebaut (`field_te_query.rs` `load_field_across_sources`). Das CDN-Asset `omni2_serie_1h.bin` ist **HTTP 206** (`phi/sources.φ:1836`) — die 404-Blockade ist gefallen. Die Zelle `newell_dphi_dt_omni->intermagnet_dbdt` bleibt `absent`: **`omni_imf_bz_gsm_nt` ist als dasselbe Feld zweifach registriert** — live (`phi:687`, rollierendes `{week_ago}…{now}`, `OMNI2_H0_MRG1HR` 3600 s) und archiviert-1h (`phi:1848`, statisches Fenster; daily unter eigenem Namen `omni_imf_bz_gsm_nt_daily` `phi:1833`). `load_field_across_sources` sortiert bei gleicher `median_dt` nach Register-Vorrang und nimmt das live-Fenster (`field_te_query.rs:1311-1313`), das disjunkt vom ABK-Archiv ist (`abk_dbdt_1m` 60 s, `phi:2416-2436`) → `align_many` `Err("bin window carries no overlap between the arms")` (`field_te_query.rs:4679-4681`). Der Lauf ist **CI-only** (`field-te-query.yml` Job `matrix-newell-omni`, `workflow_dispatch`, nie lokal); Re-Messung = `gh workflow run field-te-query.yml` + Artifact `field-te-matrix-newell-omni` lesen.
- **Blockade:** die `omni_imf_bz_gsm_nt`-Doppelregistrierung + die Fenster-Disjunktheit (Mountain-Register).
- **Braucht:** (a) Mountain löst die Doppelregistrierung (tokenisierter Block `## An mountain`); (b) danach der CI-Lauf + Artifact-Auswertung.
- **Faltung (mycelium-337, 2026-10-13):** `xfer:mycelium.ses_ecde5d231ffeCIYA7Nshj5QxDF.1~576548f1` — `omni2_serie_1h.bin` ist präsent, der CDN-Lauf ist **nicht nötig**.

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

## An mountain

Origin: river-203 (2026-10-13):

- `xfer:river.river-203.1~0e4958d2` **`omni_imf_bz_gsm_nt`-Doppelregistrierung** — `omni_imf_bz_gsm_nt` ist als dasselbe Feld zweifach registriert: live (`phi:687`, rollierendes `{week_ago}…{now}`, 3600 s) und archiviert-1h (`phi:1848`, statisches Fenster; daily unter eigenem Namen `omni_imf_bz_gsm_nt_daily` `phi:1833`). `load_field_across_sources` bindet bei gleicher `median_dt` den Register-Vorrang → das live-Fenster, disjunkt vom ABK-Archiv (`abk_dbdt_1m`, `phi:2416-2436`) → Zelle `newell_dphi_dt_omni->intermagnet_dbdt` `absent` (`align_many` `Err`, `field_te_query.rs:4679-4681`). **Braucht:** die Doppelregistrierung auflösen (ein Fenster, das ABK überlappt) oder den Drift als Messung tragen. Das Asset `omni2_serie_1h.bin` ist jetzt HTTP 206 (`phi:1836`; gemessen 2026-10-13), die frühere 404-Blockade ist gefallen.
[manifest n=1 root=eabac70a]

## Abschluss

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
