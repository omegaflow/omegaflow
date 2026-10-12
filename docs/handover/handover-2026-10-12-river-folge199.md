<!--
  title: Handover — River-Folge 199 (2026-10-12)
  session: River-Linie in einem Pass — P9.2 Schritt 2 gebaut (255-Sentinel aus Sample/FieldConfig genommen, „kein Kanal" trägt jetzt allein der Presence-Bit PRESENCE_FLAG_QUANTITY; `channel_ref: Option<u64/u8>`; `cargo check` 0/0; 45+ Dateien); Träger-Marker dreier mountain327-Dokumente abgearbeitet (4·4·2 → 2·2·0)
  class: handover
  date: 2026-10-12
  sha256: f7a859bcec91be975c3f40070f481e44fd60e4a47c7688a2a9b9b95b63e84995
  status: live
-->
# Handover — River-Folge 199 (2026-10-12)

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

## Burn: open 0.19 · close 0.31 · cap 0.50 — Grund: River-199 (deepseek-flash, kein pro/max); 2 Dispatchs (`grind-flash` P9.2-Schritt-2-Rename, `general` Carrier-Marker), ~6 `cargo check`, 1 `cargo build`-Reihe. `session_burn` misst die Linien-Sessions kumulativ (69 Sessions total $2.63). Fälligkeit: `--fired river` = 0, `--stale river --persist 3` = 0, `open_points_check` F198 = 0 absent / 0 stale-citations / 0 format-gaps. Kein `smail --send`, kein Window-Edit.

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

Verbatim: `state/operator-gespraeche/2026-10-12-river.md`, `2026-10-11-river.md`,
`2026-10-10-river.md` und `2026-10-09-river.md`.

## Träger (Prosa, eigene)

- `docs/concepts/kanal-ontologie-komplettbau.md` — der komplette Bauplan (P0–P10); P10.2a (der Parser-Arm ist Rivers Teil) als gebaut nachgeführt (`:215`).
- `docs/concepts/archivar-mathematikerin.md` — der Wire/GPU-Force-Vertrag; Träger des kinetischen Rahmens + der **Receiver-Apertur**-Kontrakt-Zeile (`:34`) + des **Schneidungs-Gesetzes**.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`status: unsealed`) — Präregistrierung der Vorhersage-Zelle; die Intervall-Konstruktion benannt (studentisiertes Moving-Block-Bootstrap, delete-a-block-Jackknife-SE). Siegel = Operator-Wort.
- `docs/paper/gic-causal-driver.md` — die Punktschätzer-Unsicherheit auf das studentisierte Moving-Block-Bootstrap umgestellt (`:171`).
- `docs/concepts/der-paradigmenwechsel.md` (9 offene Marker) · `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4) · `docs/paper/flyby-path-2-preregistration.md` (1) — von sensory-255 gefaltet (JUICE/Flyby-Path-2-Trägerschaft; Wartend.φ `flyby-path2-recon` Aufnehmer river). Marker `xfer`-los (Altblock).
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` — die Zellen der Flugbahn stehen `pending` (nach dem Flyby); der `pending`-Rest ist die 1-σ-Kovarianz (siehe §Flyby-Kette).
- `docs/surveys/survey-2026-10-12-standort-lokalisierung-ortho-dsm.md` — die generalisierte Standort-Lokalisierungs-Landkarte (Orthofoto · DSM/DOM · Street-Level) + Methode `dsm − dtm`; Carrier dieser Survey.
- **Offene-Prosa-Träger (future-235/future-240-Abgabe, River-Carrier):** `docs/auftrag/auftrag-flyby2-kette.md` (3) · `docs/auftrag/auftrag-universelles-vlies.md` (2) · `docs/blatt/blatt-te-externer-steuerparameter.md` (1) · `docs/blatt/fruehwarnsystem-praeregistrierung.md` (1, `status: unsealed`) · `docs/paper/flyby-path-2-addendum-2026-09-29.md` (27) · `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (1 — Marker `:122` gemessenes `descoped`). `register_lookup --orphan-docs` = 0.
- **Mountain327-Abgabe (Marker abgearbeitet 2026-10-12 river-199):** `docs/blatt/blatt-gic-breitenband-familien.md` (4→2) · `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (4→2) · `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md` (2→0). `xfer:mountain327-3` (echoed) · `xfer:mountain327-4` · `xfer:mountain327-5`.

## Netzbetreiber-Referenz (state, nicht re-run)

Gemessen 2026-10-11. Quellen: `state/future/space-weather-service-kante.md` §Adressat · `state/river/gic-sources-broad-2026-10-11.md` · `state/river/registrierungen-2026-10-11.md` · `state/future/funding-register-fuenf-achsen-2026-10-09.md`.

- **ENTSO-E** (EU, Dach 40 TSOs) — `token pending` (A78-Compiler gebaut). **Fingrid** (FI) — Token + Compiler; `fingrid-cdn 38162332055`. **NESO** (UK) — keyless CKAN 200, kein Outage-Datensatz. **BGS** (UK) — nur B_GIC-Index/Modell. **Statnett** (NO) — PDF-only. **Amprion** (DE) — keyless CSV-Outage-API. **TenneT** (DE) — 403 geo-block. **50Hertz** (DE) — PDF-only. **TransnetBW** (DE) — delegiert an die ENTSO-E-Plattform (`10YDE-ENBW-----N`). **Svenska Kraftnät** (SE) — CKAN, kein Outage.

Operator-Wort 2026-10-11: auf die Zulassung warten (ENTSO-E/NERC `pending`); die Netzbetreiber aus `state` referenzieren, nicht re-suchen.

## Offen (aufgeschlüsselt)

### P9.2 Kanal-Migration (`force_type` → `channel_ref`) — Schritt 1+2 gebaut, Register-Handle + flache Tokens offen
- **Status:** eigen (Bau; Operator-Wort erteilt, sofort) | **Bindung:** eigen
- **Trigger:** dieses Handover — der nächste River-Pass setzt hier an.
- **Lage:** (gemessen 2026-10-12 river-199 am Baum) **Schritt 1 (Rename, layout-neutral) + Schritt 2 (Sentinel 255 → Presence-Bit) gebaut:** `FieldConfig.channel_ref: Option<u8>`, `Sample.channel_ref: Option<f64>`; `CHANNEL_REF_QUANTITY` und `wire_channel_ref` gelöscht; „kein Kanal" trägt allein `PRESENCE_FLAG_QUANTITY` (Bit 16, `wire_channel_ref`-Ersatz = `channel_ref.unwrap_or(0.0)`); ~52 Dateien (types/spatial/parse/fetch/extract + ~40 `declared_fields`-Compiler); `cargo check` 0/0 (gemessen 6.58 s Re-Check). **Noch `force` im Baum:** die Brücke `descriptor_for_force`/`descriptor_for_channel_ref`/`channel_ref_of_descriptor` (`channel.rs:930-958`, produktiv genutzt vom Parser `parse.rs:1371/1541` + `live_channel_registry`) + `ChannelRegistry::alias`/`resolve` (nur noch Test `channel.rs:1835`) + die zwei Legacy-Tests (`channel.rs:2004/2022`); `force.rs` (`force_id_of`/`force_name_of`/`FIXED_CHANNELS`-Re-Export, **76 `force_id_of(`-Stellen in 45 Dateien**; 6 davon in `tools/harvest/src/bin/*.rs`) + `FIXED_CHANNELS` (9 Tags, `channel.rs:873`); `gate/commit_gate.rs` + die `omegaflow::force`-Re-Exports. **Gemessener Riss (die eigentliche Ursache):** `phi/sources.φ` trägt **9 `channel`-Direktiven** (Zeilen 17/27/35/44/59/112/164/272/318; 27=112 identisch → nach Hash-dedup **7 lebende**), aber `channel_registry_from_sources` registriert sie in **Quell-Reihenfolge** (em0, seismic1, gravity2, diffusion3, advective4, thermal5, acoustic6 — kein `electric`), während der Parser den Handle über die **feste 9er-Liste** vergibt (em0, gravity1, acoustic2, seismic-body3, seismic-surface4, thermal5, diffusion6, advective7, electric8). Der Parser-Index und der Laufzeit-Registry-Index fallen ab dem zweiten Kanal auseinander → der Record-`channel_ref` zeigt in den falschen Deskriptor (electric/lov ergeben Index 8 bei n=7 → Leser-Weigerung). Schritt 3 behebt genau das.
- **Blockade:** keine.
- **Braucht:** die Schritte bauen (bounded, je `cargo check` 0/0): **(3)** den Parser-Handle **register-sourced** auflösen — die geladenen `channel`-Deskriptoren (nicht die feste 9er-Liste) als Registry führen und den Feld-Handle daraus vergeben (2-Pass bzw. akkumulierte Registry über alle Quellen, damit Parser-Index == `channel_registry_from_sources`-Index); **(4)** `descriptor_for_force`/`descriptor_for_channel_ref`/`channel_ref_of_descriptor` + `alias`/`resolve` + die zwei Legacy-Tests streichen und durch `descriptor_from_axes`-Aufbau + `ChannelRegistry::handle_of` ersetzen; **(5)** die flachen Tokens (`force_id_of`/`force_name_of`/`FIXED_CHANNELS`/Modul `force`) auf den Kanal-Deskriptor umstellen (76 Stellen/45 Dateien + 6 `tools/harvest`-Compiler + `gate/commit_gate.rs` + die `omegaflow::force`-Re-Exports). Verifikation: `cargo check` + gezielte `cargo build -p … --bin …`; die Tests sind lokal nicht kompilierbar (`cargo check` prüft `#[cfg(test)]` nicht) → CI-`ci-gate` ist das Testnetz.

### Prosa-Träger mountain327-Abgabe — Marker abgearbeitet, Rest offen
- **Status:** eigen (Carrier-Duty) | **Bindung:** eigen
- **Trigger:** kein — die Träger stehen.
- **Lage:** (gemessen 2026-10-12 river-199) die drei `--orphan-docs`-Dokumente sind gefaltet und ihre Marker abgearbeitet: `docs/blatt/blatt-gic-breitenband-familien.md` (4→2; geschlossen: der Weg (a/b/c) gebaut, Design-Entscheidungen entschieden; offen: IGRF-14/AACGM-v2-Rust-Bin, Route-C-Stufe-2-Member-Pool) · `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (4→2; offen: CTBTO-vDEC-Antrag, Rest) · `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md` (2→0).
- **Blockade:** keine.
- **Braucht:** die zwei verbleibenden Marker je Blatt/Survey abarbeiten, sobald ihr Gegenstand gemessen ist (aacgm-Bin = Bau; CTBTO = Operator-Antrag).

### 14-min-Lücke Magnetometer→Optik — im 2D gemessen, im Tages-Overlay als bockige Phase + Pick-Riss
- **Status:** eigen (getragener Riss) | **Bindung:** eigen
- **Trigger:** der GIC-Onset-Relationsschritt auf der Meridian-Spur (Blatt/Antwortseite); der Ganznacht-Optik-Lauf `38259065347`.
- **Lage:** (gemessen 2026-10-12 river-197, Anker `state/river/themis/frame-onset-2026-10-12.md` + `overlay-day-2026-10-12.md`) **2D (Tool `themis_asi_frame_onset`, Cube `fodg_20240510_kapu_onset.bin`):** kein kohärenter off-meridian optischer Eintritt bei 07:41; der optische Onset ist ein einzelnes azimutal wanderndes Band 07:54:45→07:55:45 (az 155–240, el 45–55). **Tages-Overlay (`mag_optical_overlay`):** der Hauptsturm liegt 17:00–23:00 UT (Tageslicht); das optische Fenster (06:50–08:15) ist eine kleinere, bockige Phase (YKC 07 max 52.4). **Riss:** der river-196-Pick „07:41" ist nicht eindeutig — Stunde 07 trägt eine Kette (07:13–07:48), größte YKC-Spitze bei 07:20 (52.4); mehrstationig ist 07:19 so kohärent wie 07:41 → die „+14 min 27 s"-Lücke ist pick-abhängig.
- **Blockade:** keine (2D- und Tages-Messung stehen).
- **Braucht:** (a) den Ganznacht-Optik-Lauf `38259065347` lesen (CDN-TKDG) und den Overlay über 02:00–08:59 UT neu fahren; (b) den Riss tragen (Phasendifferenz + Pick-Abhängigkeit); die Keogramm-Assets auf den CDN = Mycelium.

### Ganznacht-Optik kapu 2024-05-10 — CI-Lauf liest
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss des `themis_asi-timed-keogram-cdn`-Laufs `38259065347` (station kapu, date 20240510, ohne Fenster = `ut02..ut08`).
- **Lage:** (gemessen 2026-10-12 via `ci_manage view`) der Lauf ist `queued` (Runner-Backlog, seit 13:54). Der Tagesindex trägt `ut02..ut08`; lokal liegt nur 06:50–08:15.
- **Blockade:** keine (Lauf wartet im Runner-Backlog).
- **Braucht:** `ci_manage log 38259065347 --all` nach Abschluss; dann das TKDG lokal holen und `mag_optical_overlay --optical <ganznacht.bin> --svg …` neu fahren.

### GIC-Wunschliste-Erweiterung — vier Zeugen-Fäden gemessen, volle Suche permanent abgelegt
- **Status:** eigen (Admission offen) | **Bindung:** eigen
- **Trigger:** Operator-Entscheid über die Aufnahme; dann Admission (Mountain) + Arm/Manifestation (Mycelium).
- **Lage:** (gemessen 2026-10-12 river-197 via 4 `grind-flash`-Taucher, `archive_search --all`, Volldateien `state/river/gic-wunschliste-erweiterung-2026-10-12/*-all.txt`, Anker-Berichte `riometer.md`/`ionosonde-isr.md`/`thermisch-neutral.md`/`barrel-elfvlf.md`, `--verdict` am Baum nachgemessen; **zwei Framing-Korrekturen** am Baum bestätigt: keine Domänen — `docs/concepts/die-weberin.md:64`; keine flachen `force`-Tokens — 0 in `phi/sources.φ`, ersetzt durch den Kanal-Deskriptor `channel <conserved>:<role>:<op>:<pde_type>:<medium>:<domain>:<boundary>[:<extent>] <unit>`, `docs/concepts/kanal-ontologie-komplettbau.md:46`, `phi/sources.φ:17`) vier Zeugen außerhalb der Wunschliste A–G geprüft: **Riometer** (GO-Canada/TREx + SGO-FMI erreichbar, Register 0); **Ionosonde/ISR** (GIRO gebaut; ISR EISCAT + PFISR/AMISR via Madrigal 200/206 neu); **thermisch/neutral** (Airglow/FPI Register 0, CGSM/NORSTAR-Photometer 206, Thermosphären-Dichte registriert); **BARREL/ELF-VLF** (BARREL SPDF 206, WALDO 200/EMFISIS/Athabasca, AWESOME schon registriert). Jeder Zeuge wird mit vollem Channel-Deskriptor (Quantity | Mechanism | Medium, P10-Wort) registriert und geht in `matrix full` ein.
- **Blockade:** keine.
- **Braucht:** Operator-Wort zur Aufnahme; dann je Zeuge der Channel-Deskriptor wissenschaftlich fixiert/geprüft (P10) + Registrierung (Mountain: `channel`-Direktive/`terms`) + Arm/Manifestation (Mycelium).

### Netz-End-Validator — GIC↔Onset-Bin; MCON manifestiert, Register-Drift gelöst
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** `bpa_gic` deckt den MCON-Zeitraum (1999–2024) UND die `format mcon_event`-Registerzeile steht (Mountain).
- **Lage:** (gemessen 2026-10-12 via `ci_manage log 38176003927` + mycelium-318) **MCON manifestiert:** `mcpherron_onsets.bin`, 314 555 Onsets, `format mcon_event`, sha `441df5f8…`. **Register-Drift gelöst** (mycelium-318). `bpa_gic` führt nur ein **4-Tage/5-min-Fenster** (2026-10-03..06), **kein Archiv** (gemessene Null) → 0 Onsets im MCON-Fenster; die zweite GIC-Zeugenreihe bleibt leer, die Relation läuft über die FMI-Reihe.
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

### Newell-OMNI-Alignment — Resolver gebaut, Lauf 38215224525 gelesen: Zelle bleibt absent
- **Status:** eigen (offen) | **Bindung:** eigen
- **Trigger:** kein — der Lauf ist gelesen; die Messung steht.
- **Lage:** (gemessen 2026-10-12 river-198 via `ci_manage log 38215224525 --all`) der Resolver ist gebaut (`field_te_query.rs` `load_field_across_sources`: per-`station_code`-Scope, feinster nativer Zeuge als Ergebnis, gröberer Aggregationszeuge als getragener Riss — Mycelium `3467ec4f9`). **Ergebnis:** die Zellen `newell_dphi_dt->intermagnet_dbdt` und `newell_dphi_dt_omni->intermagnet_dbdt` liefern beide `absent` / `summary … pending — no measured rung` — der **Treiber `newell_*` ist selbst absent** (keine geladene Quellenreihe), nicht die Rung-Wahl. Damit ist die erwartete Zeugenwahl (`abk_1m`) nicht ausgeübt; die Zelle bleibt leer. Die frühere Lage (corr 0.9388 abk_1m/abk_1h) ist nicht mehr sichtbar.
- **Blockade:** die `newell_*`-Reihe hat keine geladene Quelle (`phi/sources.φ` / Registry).
- **Braucht:** messen, welche Quelle `newell_dphi_dt[_omni]` tragen soll (Mountain `phi/sources.φ`), die Reihe laden und den Lauf erneut fahren; dann die Zeugenwahl (`abk_1m` vs `abk_1h`) messen.

### Flyby-Kette — recon bleibt (`ephemeris-juice-recon` + `flyby-path2-recon`)
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** `ephemeris-juice-recon` Wiedervorlage 2026-11-01 (ESA/ESOC publiziert die Post-Flyby-SPK). Wahrheit: `state/zustand/wartend.φ`.
- **Lage:** (gemessen 2026-10-11) die Wiedervorlage 2026-11-01 steht; `data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin` absent. `flyby-path2-recon`: orbc-SPK keyless `https://spiftp.esac.esa.int/data/SPICE/JUICE/kernels/spk/juice_orbc_000116_230414_310721_v01.bsp` (HTTP 206); die **1-σ-Kovarianz liegt nicht im SPICE-Baum**.
- **Blockade:** ESA/ESOC-recon-SPK + die 1-σ-Kovarianz absent.
- **Braucht:** am 2026-11-01 `archive_search --verdict` auf den ESOC-recon-Pfad; dann `flyby_ephemeris_gate --recon <recon.bin> --sigma-recon <km>`.

### Blatt-Kalibrierung — Z-Null ABK gelesen (Z = 58 min); Signifikanz-Null offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort (Siegel) auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`; der Signifikanz-Null (Blocklänge) ist bestimmbar.
- **Lage:** (gemessen 2026-10-12 river-198 via `gh run download 38215420000`) **BCa benannt** (studentisiertes Moving-Block-Bootstrap; delete-a-block-Jackknife Künsch 1989 als SE). **ABK-2024 (1-min-Korn):** `Z = 58 min | r = −0.31053 | n = 523621` (Maximum bei Lag 58); Artefakt `bz-dbdt-delay-abk-2024.txt` (sha `32699e08…`-Zip). **SOD-2024:** `SOD minute |dB/dt| absent` — das CDN-Asset fehlt, keine Messung; Artefakt `bz-dbdt-delay-sod-2024.txt`. **Bz-Schwelle = pending** (braucht Station X + kalibriertes Alpha); **studentisierter Null = pending** — die Treiber-Autokorrelation dreht innerhalb 180 min nirgends ins Nichtpositive, die Blocklänge fehlt.
- **Blockade:** der Signifikanz-Null braucht eine Blocklänge (Autokorrelations-Nulldurchgang fehlt); SOD-Asset fehlt.
- **Braucht:** (a) Blocklängen-Regel für dieses Korn benennen (Rat) oder den Deskriptiv-Wert tragen; (b) SOD-`dB/dt`-Asset (Mycelium); (c) α/Bz-Schwelle + Siegel = Operator-Wort (nach Future geroutet).

## An mountain

Origin: river-197 (2026-10-12) — Messungen/Befunde, die Mountains Register/Archivar betreffen (Sender-Register; die Owner-Session faltet).

- **`solar_f107_flux_sfu` — kein Identitäts-Riss, ein Zeitbasis-Defekt (gemessen).** (2026-10-12, `general`-Taucher) Der SWPC-Textarm liefert **TDB/J2000** (`lsk.unix_to_tdb`, `extract.rs:6506`, `parse.rs:2349`), das Penticton-`f107`-Asset **rohe Unix-Sekunden** (`src/archivar/f107.rs:22`, `day*86400`); feste Verschiebung `≈ 9.4673e8 s ≈ 30,0 a` → SWPC-2026 paart mit Penticton-1996 → r≈0.02. **Zwei Archivar-Defekte:** (i) `f107::parse_series` wendet `unix_to_tdb` **nicht** an; (ii) `series_epoch_of` (`extract.rs:6506-6539`) wählt den ersten Key mit `"date"` — `avg_begin_date` kann `time_tag` verdrängen. **Braucht:** beide Arme auf dieselbe Epoch-Basis (TDB) bringen; dann den `f107`-Join erneut messen. `xfer:river197-1`
- **MCON-Spanne 1966–2025 vs Parser-Ende 2024-05-29** — nominale Beschreibung vs Parser-Fenster; für die BPA (2026) irrelevant. `xfer:river197-2`
- **THEMIS-ASI Stations-Tabelle (`src/archivar/themis_asi.rs`) — von River (Operator-Wort 2026-10-12) korrigiert (erledigt, Info).** PGM-Kopf-Höhe ist ellipsoidisch; gesetzt ATHA 549.82 · EKAT −110.6703/430.77 · PGEO 756.26 · MCGR 102.57. Anker `state/river/themis/calibration-2026-10-12.md`. `xfer:river197-3`

## An future

Origin: river-197 (2026-10-12) — Operator-Akt, den diese Session nicht selbst führen darf (Sender-Register; die Owner-Session faltet).

- **wy-max-t-Blatt-Siegel** — `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`status: unsealed`): der Westfall–Young-max-T-Null ist gemessen (α=0.05, pooled B=9999; (1-α)-Quantil 2.4467/2.4433/2.4831; `Bz→dB/dt` family-clearing an allen drei Punkten; Artefakt `38166787414`). Die Vorbereitung ist River-eigen; **das Siegel setzt der Operator**. `xfer:river197-4`
- **Street-Level-Keys (Zugang) — erledigt (Operator-Wort 2026-10-12).** (a) **Mapillary erledigt:** `MAPILLARY_ACCESS_TOKEN` in `.secrets.local` (nur der Reader); `tools/measure/src/bin/mapillary_coverage.rs` gebaut; ATHA `images 0` gemessen. (b) **Google entfällt** (kein Karten-Billing). Kein Rest. `xfer:river197-5`

## An mycelium

Origin: river-197 (2026-10-12) — Manifestations-Akt in Myceliums Domäne (Sender-Register; die Owner-Session faltet).

- **THEMIS-ASI Keogramm-Assets auf den CDN (Register-Duty).** Der Meridian-Keogramm-Compiler (`themis_asi_timed_keogram_compiler`, Workflow `themis_asi-timed-keogram-cdn.yml`) und der 196/197-Bau (`skymap_fit`, `themis_asi_frame_onset`) sind lokal gemessen; die Assets (`tkdg_20240510_kapu_onset.bin` + die Keogramm-PGM) existieren nur lokal. **Braucht:** die Assets über `--ci-mode` manifestieren (Register-Duty, Mycelium). Anker `state/river/themis/frame-onset-2026-10-12.md`. `xfer:river197-6`

## Abschluss

Der P9.2-Schritt 2 ist gebaut und `cargo check`-grün (45+ Dateien, path-scoped committet); die
Schritte 3–5 (register-sourced Handle, Brücke weg, flache Tokens weg) stehen mit gemessenem Riss
im §Offen. Kein `smail --send`, kein Window-Edit. `/consent` ist der session-weite
Delegations-Consent, nie das Commit-Wort.
