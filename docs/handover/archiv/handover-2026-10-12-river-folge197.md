<!--
  title: Handover — River-Folge 197 (2026-10-12)
  session: River-Linie in einem Pass — (1) der 2D-Vollbild-Onset-Detektor (`themis_asi_frame_onset`) gebaut + auf kapu 2024-05-10 gefahren: kein off-meridian Eintritt bei 07:41, der optische Onset ist ein azimutal wanderndes Band 07:54:45→07:55:45; (2) der Ganz-Tag-Overlay (`mag_optical_overlay`, INTERMAGNET YKC/FCC/OTT vs kapu-Helligkeit): Hauptsturm 17–23 UT (Tageslicht), das optische Fenster eine kleinere bockige Phase, der 07:41-Pick nicht eindeutig → 14-min-Lücke als pick-abhängiger Phasen-Riss getragen; (3) vier Zeugen-Domänen (Riometer · Ionosonde/ISR · Airglow/FPI · BARREL/ELF-VLF) recherchiert, volle `--all`-Suche permanent abgelegt (GIC-Wunschliste-Erweiterung); (4) Architektur-Framing korrigiert (Weberin: keine Domänen; keine flachen `force`-Tokens — nur die Kanal-Ontologie); der unfertige P9.2-Bau (`force_type`→`channel_ref`, keine Abwärtskompatibilität) als NÄCHSTES ATOM ins Register; mountain327-Adressblöcke gefaltet; Mycelium-Keogramm-CDN-Manifestation registriert
  class: handover
  date: 2026-10-12
  sha256: a63e69305042bffa49964106c38d66b85d8442b9b940a274baac4bca85a92684
  status: live
-->
# Handover — River-Folge 197 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward).

## Burn: open 0.0 · close 0.28 · cap 0.50 — Grund: River-197 (deepseek-flash, kein pro/max); session_burn misst die Linien-Session „River" mit $0.2766 (die DB isoliert die Session-Kost nicht sauber — 70 Sessions total $2.72). Linien-Session + 2 `cargo`-Bauten (`themis_asi_frame_onset`, `mag_optical_overlay`) + 2 lokale Messläufe (1720 Frames; Tages-Overlay) + 1 CI-Dispatch (`themis_asi-timed-keogram-cdn` 38259065347) + 4 `grind-flash`-Recherche-Taucher (`archive_search --all`, volle Suche permanent abgelegt). Fälligkeit: `--fired river` = 1 (Vlies-Matrix-Alignment, Lauf 38215224525 noch `pending`), `--stale river --persist 3` = 0, `open_points_check` F196 = 2 absent (Brace-/Backtick-Artefakt, keine echte stale citation), `--addressed river` = 2 Blöcke (sensory-JUICE bereits in F196 gefaltet; mountain327-Orphans in F197 gefaltet). Gearbeitet: 2D-Frame-Onset-Detektor + Tages-Overlay gebaut + gemessen; vier Zeugen-Domänen recherchiert; P9.2-Atom gesetzt. Kein `smail --send`, kein Window-Edit.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Nicht gebaut — die P10.2a-Token-Semantik ist nicht entschieden; ein geratener Bau wäre Fabrikation (A=A). bitte archive search all rat und roster" | 2026-10-10 | Operator (Session, River 165)
„wenn du den rat befragst befrage bitte auch die wissenschaft mit archive serahc alll und die ui und oenweight frontier voices" | 2026-10-09 | Operator (Session, River 145)
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
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie **in einem Pass** … /commit ist das Commit-Wort — beide getrennt." | 2026-10-11 | Operator (Session, River 188/191/192/193) — session-weiter Delegations-Consent, nicht das Commit-Wort.
„ok dann ist es pending … dann warten wir bis wir zugelassen sind kannst du nochmal in state schauen wir hatten da eigentlich einige netzbetreiber" | 2026-10-11 | Operator (Session, River 188) — ENTSO-E/NERC bleiben `pending`; auf die Zulassung warten, bis dahin die Netzbetreiber aus `state` referenzieren (nicht re-suchen).
„mountain ist sehr beschäftigt und mycelium auch … kümmer dich bitte selbst drum" | 2026-10-11 | Operator (Session, River 191) — **River übernimmt den Vlies-Matrix-Alignment-Punkt selbst** (SWPC/RTSW-Archivreihe finden + registrieren + `matrix-vlies` neu), auch in Mountain- (`phi/sources.φ` url) und Mycelium-Domäne (Manifestation/Compiler).
„ja" | 2026-10-11 | Operator (Session, River 192) — (1) den SPAN-Feld-Edit bauen; (2) die vier verwaisten `cargo fmt`-Hünke (Mountain-316/Mycelium-312-Rückstand) als Haus-Dienst committen.
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie **in einem Pass** … /commit ist das Commit-Wort — beide getrennt." | 2026-10-12 | Operator (Session, River 194) — session-weiter Delegations-Consent, nicht das Commit-Wort.
„Starte die River-Linie **in einem Pass** … Fahr die River-eigenen Teile weiter (BCa, Vlies-Restarm, Z-Null-Zahl im CI) — sie hängen an nichts von mir. Der Resolver steht; sobald `38215224525` grün gelesen ist, ist die Antworseite des Blatts frei. … mach bitte deinen teil" | 2026-10-12 | Operator (Session, River 195) — Work-Order, session-weiter Delegations-Consent, nicht das Commit-Wort.
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie **in einem Pass** … /commit ist das Commit-Wort — beide getrennt." | 2026-10-12 | Operator (Session, River 196) — session-weiter Delegations-Consent, nicht das Commit-Wort.
„ja bitte selbst mountain ist beschäftigt ich glaube wir müssen projektownerschaften einführen nicht aufgabenownerschaften das reibt so dermassen" | 2026-10-12 | Operator (Session, River 196) — (1) River setzt die Stations-Tabellen-Korrektur **selbst** (fremder Schreibakt auf Operator-Wort); (2) **Architekturfrage: Projekt-Ownerschaft statt Aufgaben-Ownerschaft** (Rat/Operator).
„Erste Handlung: `sread docs/concepts/tool-forms.md` … Starte die River-Linie **in einem Pass** … /commit ist das Commit-Wort — beide getrennt. … Soll ich den 2D-Frame-Onset-Detektor bauen und den Bogen um 07:41 suchen — oder die 14-min-Lücke als getragenen Riss stehen lassen und die Register-Duty schließen?" | 2026-10-12 | Operator (Session, River 197) — Work-Order „mache hier weiter"; der Bau ist River-eigen (autonom geführt).

„aber die neun kraftmedien sind doch auch abgeschafft die hast doch sogar du selbst abgeschafft warum seid ihr gerade alle s kopflos" | 2026-10-12 | Operator (Session, River 197) — die 9 Kraft-Medien UND die flachen `force`-Tokens sind abgeschafft; die Kanal-Ontologie (`channel <conserved>:<role>:<op>:<pde_type>:<medium>:…) ist die einzige Form.
„warum legacy bridge hast du deine arbeit nicht zuende gemacht ich habe immer gesagt ich möchte keine abwärtskompatibilität" | 2026-10-12 | Operator (Session, River 197) — keine Abwärtskompatibilität; den P9.2-Bau zu Ende bringen.
„gab es da nicht schon rats gespräche und archive search all und ui seats sag mal was ist denn heute los" | 2026-10-12 | Operator (Session, River 197) — die P9.2/P10-Entscheidung ist durch Rat + `archive_search --all` + UI entschieden; **kein neuer Rat**.
„was heisst morgen glieich aber nicht in dieser session mach eine übergabe" | 2026-10-12 | Operator (Session, River 197) — P9.2 sofort als nächstes Atom, aber **nicht in dieser Session**; Übergabe schreiben.

Verbatim: `state/operator-gespraeche/2026-10-12-river.md`, `2026-10-11-river.md`,
`2026-10-10-river.md` und `2026-10-09-river.md`.

## Träger (Prosa, eigene)

- `docs/concepts/kanal-ontologie-komplettbau.md` — der komplette Bauplan (P0–P10); P10.2a (der Parser-Arm ist Rivers Teil) als gebaut nachgeführt (`:215`).
- `docs/concepts/archivar-mathematikerin.md` — der Wire/GPU-Force-Vertrag; Träger des kinetischen Rahmens + der **Receiver-Apertur**-Kontrakt-Zeile (`:34`) + des **Schneidungs-Gesetzes**.
- `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`status: unsealed`) — Präregistrierung der Vorhersage-Zelle; die Intervall-Konstruktion benannt (studentisiertes Moving-Block-Bootstrap, delete-a-block-Jackknife-SE). Siegel = Operator-Wort.
- `docs/paper/gic-causal-driver.md` — die Punktschätzer-Unsicherheit auf das studentisierte Moving-Block-Bootstrap umgestellt (`:171`).
- `docs/concepts/der-paradigmenwechsel.md` (9 offene Marker) · `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4) · `docs/paper/flyby-path-2-preregistration.md` (1) — von sensory-255 gefaltet (JUICE/Flyby-Path-2-Trägerschaft; Wartend.φ `flyby-path2-recon` Aufnehmer river).
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` — die Zellen der Flugbahn stehen `pending` (nach dem Flyby); der `pending`-Rest ist die 1-σ-Kovarianz (siehe §Flyby-Kette).
- `docs/surveys/survey-2026-10-12-standort-lokalisierung-ortho-dsm.md` — die generalisierte Standort-Lokalisierungs-Landkarte (Orthofoto · DSM/DOM · Street-Level) + Methode `dsm − dtm`; Carrier dieser Survey.
- **Offene-Prosa-Träger (future-235/future-240-Abgabe, River-Carrier):** `docs/auftrag/auftrag-flyby2-kette.md` (3) · `docs/auftrag/auftrag-universelles-vlies.md` (2) · `docs/blatt/blatt-te-externer-steuerparameter.md` (1) · `docs/blatt/fruehwarnsystem-praeregistrierung.md` (1, `status: unsealed`) · `docs/paper/flyby-path-2-addendum-2026-09-29.md` (27) · `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (1 — Marker `:122` gemessenes `descoped`). `register_lookup --orphan-docs` = 0.
- **Mountain327-Abgabe (gefaltet 2026-10-12 river-197, Sender mountain328):** `docs/blatt/blatt-gic-breitenband-familien.md` (4, see-also `docs/paper/gic-causal-driver.md`) `xfer:mountain327-3` · `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (4, see-also river-folge93) `xfer:mountain327-4` · `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md` (2, see-also river-folge139) `xfer:mountain327-5`.

## Netzbetreiber-Referenz (state, nicht re-run)

Gemessen 2026-10-11. Quellen: `state/future/space-weather-service-kante.md` §Adressat · `state/river/gic-sources-broad-2026-10-11.md` · `state/river/registrierungen-2026-10-11.md` · `state/future/funding-register-fuenf-achsen-2026-10-09.md`.

- **ENTSO-E** (EU, Dach 40 TSOs) — `token pending` (A78-Compiler gebaut). **Fingrid** (FI) — Token + Compiler; `fingrid-cdn 38162332055`. **NESO** (UK) — keyless CKAN 200, kein Outage-Datensatz. **BGS** (UK) — nur B_GIC-Index/Modell. **Statnett** (NO) — PDF-only. **Amprion** (DE) — keyless CSV-Outage-API. **TenneT** (DE) — 403 geo-block. **50Hertz** (DE) — PDF-only. **TransnetBW** (DE) — delegiert an die ENTSO-E-Plattform (`10YDE-ENBW-----N`). **Svenska Kraftnät** (SE) — CKAN, kein Outage.

Operator-Wort 2026-10-11: auf die Zulassung warten (ENTSO-E/NERC `pending`); die Netzbetreiber aus `state` referenzieren, nicht re-suchen.

## Offen (aufgeschlüsselt)

### P9.2 Kanal-Migration (`force_type` → `channel_ref`) — NÄCHSTES ATOM (Operator-Wort 2026-10-12: „gleich, aber nicht in dieser Session")
- **Status:** eigen (Bau; Operator-Wort erteilt, sofort) | **Bindung:** eigen
- **Trigger:** dieses Handover — der nächste River-Pass beginnt hier; das Operator-Wort liegt vor.
- **Lage:** (gemessen 2026-10-12 am Baum) die flachen `force`-Tokens sind im Register weg (die Zeilen tragen `channel …`), aber der Wire-Weg trägt sie weiter. **Schon `channel_ref`:** `src/archivar/spatial.rs`, `src/mathematikerin/ozzy.rs`, `actuators.rs`, `CHANNEL_REF_QUANTITY`/`PRESENCE_FLAG_QUANTITY` (`src/archivar/types.rs`). **Noch `force`:** `src/mathematikerin/machines/matrix.rs:173`, `src/mathematikerin/dispersion.rs:4`, `src/mathematikerin/force.rs` (`force_id_of`/`force_name_of`), `FieldConfig.force` (`types.rs:385`), ~30 Compiler (`SLOT_FORCE_TYPE`, `spec.force`), `tools/measure/src/bin/signal_cone_audit_probe.rs`, `static/constants.js:102` (`forceType` @byte 72). **Die Brücke:** `descriptor_for_force`/`channel_ref_of_descriptor` (`channel.rs:930-958`). **P9.2** (Rat-Verdikt 2026-10-10, `state/stimmen/2026-10-10-river-p92-rat.md`; Token-Semantik `…-p10.2a-token-semantik.md`; Identität `2026-10-09-river-p10-identitaet.md`) spezifiziert: Rename `force_type`→`channel_ref` (Wire + JS/WGSL, layout-neutral), 255→Presence-Bit, GPU-Dispatch über das Operator-Enum; Record bleibt 26×f64. **Kein neuer Rat** — entschieden; `tools/gate/src/bin/livefeed_gate.rs` `force_type` ist eine fremde deutsche Klassifikation, nicht der Wire-Slot (nicht anfassen).
- **Blockade:** keine.
- **Braucht:** die Sequenz bauen — (1) Rename layout-neutral (`FieldConfig.force`, `matrix.rs`, `dispersion.rs`, Compiler-`SLOT_FORCE_TYPE`, `constants.js` `forceType`); (2) 255→`PRESENCE_FLAG_QUANTITY`-Bit; (3) `channel_ref` = Registry-Handle → Manifest-Hash (append-only, Tombstones; `schema_hash`-Missverhältnis = Verweigerung, kein Default); (4) `descriptor_for_force`/`channel_ref_of_descriptor` ersatzlos entfernen — keine Abwärtskompatibilität. Verifikation: `cargo check` + gezielte `cargo build -p <crate> --bin <name>` auf die berührten Compiler.

### 14-min-Lücke Magnetometer→Optik — im 2D gemessen, im Tages-Overlay als bockige Phase + Pick-Riss
- **Status:** eigen (getragener Riss) | **Bindung:** eigen
- **Trigger:** der GIC-Onset-Relationsschritt auf der Meridian-Spur (Blatt/Antwortseite); der Ganznacht-Optik-Lauf `38259065347`.
- **Lage:** (gemessen 2026-10-12 river-197, Anker `state/river/themis/frame-onset-2026-10-12.md` + `overlay-day-2026-10-12.md`) **2D (Tool `themis_asi_frame_onset`, Cube `fodg_20240510_kapu_onset.bin`):** kein kohärenter off-meridian optischer Eintritt bei 07:41; der optische Onset ist ein einzelnes azimutal wanderndes Band 07:54:45→07:55:45 (az 155–240, el 45–55); die Meridianzelle 07:55:27 ist eine Zelle davon. **Tages-Overlay (Tool `mag_optical_overlay`, `overlay-20240510.csv`/`.svg`):** der Hauptsturm liegt **17:00–23:00 UT** (YKC 456.94 nT/min @ 18:35 · FCC 272.33 @ 18:38 · OTT 272.53 @ 23:11) — am kapu-Standort **Tageslicht**, kein optischer Zeuge. Das optische Fenster (06:50–08:15) ist eine **kleinere, bockige** Phase (YKC 07 max 52.4); die Meridian-Helligkeit steigt monoton 3329→3811. **Riss:** der river-196-Pick „07:41" ist **nicht eindeutig** — Stunde 07 trägt eine Kette (07:13–07:48), größte YKC-Spitze bei **07:20 (52.4)**; mehrstationig ist 07:19 (YKC 38.1+FCC 16.1) so kohärent wie 07:41 (24.1+12.4) → die „+14 min 27 s"-Lücke ist **pick-abhängig**.
- **Blockade:** keine (2D- und Tages-Messung stehen).
- **Braucht:** (a) den Ganznacht-Optik-Lauf `38259065347` lesen (CDN-TKDG) und den Overlay über 02:00–08:59 UT neu fahren; (b) den Riss tragen (Phasendifferenz + Pick-Abhängigkeit); die Keogramm-Assets auf den CDN = Mycelium.

### Ganznacht-Optik kapu 2024-05-10 — CI-Lauf liest
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss des `themis_asi-timed-keogram-cdn`-Laufs `38259065347` (station kapu, date 20240510, ohne Fenster = `ut02..ut08`).
- **Lage:** (gemessen 2026-10-12 via `gh workflow run`) der Tagesindex trägt `ut02..ut08` (7 h Nacht); lokal liegt nur 06:50–08:15. Der Lauf ist dispatcht; das TKDG landet auf dem CDN (`omegaflow/sources`, tag `data.phys.ucalgary.ca`).
- **Blockade:** keine (Lauf läuft).
- **Braucht:** `ci_manage log 38259065347` nach Abschluss; dann das TKDG lokal holen und `mag_optical_overlay --optical <ganznacht.bin> --svg …` neu fahren.

### Prosa-Träger mountain327-Abgabe (gefaltet)
- **Status:** eigen (Carrier-Duty) | **Bindung:** eigen
- **Trigger:** kein — der Träger steht (Carrier ist der nächste Schritt).
- **Lage:** (gemessen 2026-10-12 via `register_lookup --tokens` + `register_lookup --orphan-docs`) drei `--orphan-docs`-Dokumente als River-Carrier gefaltet (siehe ## Träger): `docs/blatt/blatt-gic-breitenband-familien.md` (4) `xfer:mountain327-3` · `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` (4) `xfer:mountain327-4` · `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md` (2) `xfer:mountain327-5`.
- **Blockade:** keine.
- **Braucht:** die offenen Marker der drei Dokumente abarbeiten (Carrier-Duty); keine neue Messung nötig.

### GIC-Wunschliste-Erweiterung — vier Zeugen-Fäden gemessen, volle Suche permanent abgelegt
- **Status:** eigen (Admission offen) | **Bindung:** eigen
- **Trigger:** Operator-Entscheid über die Aufnahme; dann Admission (Mountain) + Arm/Manifestation (Mycelium).
- **Lage:** (gemessen 2026-10-12 river-197 via 4 `grind-flash`-Taucher, `archive_search --all`, Volldateien `state/river/gic-wunschliste-erweiterung-2026-10-12/*-all.txt`, Anker-Berichte `riometer.md`/`ionosonde-isr.md`/`thermisch-neutral.md`/`barrel-elfvlf.md`, `--verdict` am Baum nachgemessen; **zwei Framing-Korrekturen** am Baum bestätigt: keine Domänen — `docs/concepts/die-weberin.md:64`; keine flachen `force`-Tokens — 0 in `phi/sources.φ`, ersetzt durch den Kanal-Deskriptor `channel <conserved>:<role>:<op>:<pde_type>:<medium>:<domain>:<boundary>[:<extent>] <unit>`, `docs/concepts/kanal-ontologie-komplettbau.md:46`, `phi/sources.φ:17`) vier Zeugen außerhalb der Wunschliste A–G geprüft, als Fäden im **einen Gewebe**: **Riometer** (CNA/D-Region, andere Observable als dB/dt+TEC) — GO-Canada/TREx + SGO-FMI erreichbar (206), Register 0; **Ionosonde/ISR** — GIRO (`giro_fastchar`) schon gebaut, ISR EISCAT + PFISR/AMISR via Madrigal (200/206) neu; **thermisch/neutral** — Airglow/FPI Register 0, CGSM/NORSTAR-Photometer (`cgsm.ca` 206), thermosph. Dichte (CHAMP/GRACE) registriert; **BARREL/ELF-VLF** — BARREL SPDF (`spdf.gsfc.nasa.gov/pub/data/barrel/` 206), WALDO (`waldo.world` 200)/EMFISIS/Athabasca, AWESOME schon registriert. Jeder Zeuge wird mit **vollem Channel-Deskriptor** (Quantity | Mechanism | Medium, P10-Wort) registriert und geht damit in `matrix full` (alles gegen alles) ein; das Frühwarnsystem liest **dieselbe** Matrix, kein neuer Kasten.
- **Blockade:** keine.
- **Braucht:** Operator-Wort zur Aufnahme; dann je Zeuge der Channel-Deskriptor wissenschaftlich fixiert/geprüft (P10) + Registrierung (Mountain: `channel`-Direktive/`terms`) + Arm/Manifestation (Mycelium).

### Vlies-Matrix-Alignment — Fix committet, Lauf 38215224525 liest
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss des `field-te-query`-Laufs `38215224525` (HEAD `3467ec4f9`).
- **Lage:** (gemessen 2026-10-12 via `ci_manage list` + `ci_manage view`) der Fix `f018f4df0` ist committet (per-URL-Memo + Retry gegen den 522-B-Stub); Lauf `38215224525` **noch `pending`**. Vorher: `alignment pending` 126 → **50** (fünf Reihen ohne Quellenzeile: `qbo_30hpa_ms`/`d20_thermocline_depth_m`/`swarm_absolute_vertical_tec_tecu`/`eve_1032_line_irradiance`/`eve_131_line_irradiance`); `solar_wind_density_cm3` echt absent (`Np = -1.0E31` fill → 0 honored).
- **Blockade:** keine (Fix committet).
- **Braucht:** `ci_manage log 38215224525` lesen; erwartet: `bt`/`speed` lösen, `alignment pending` fällt auf die absent-source-Reihen. Der distinct-URL-Throttle (SWE) bleibt tiefer in `src/archivar/fetch.rs` (Backoff auf 522-Stub), benannt.

### Netz-End-Validator — GIC↔Onset-Bin; MCON manifestiert, Register-Drift gelöst
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** `bpa_gic` deckt den MCON-Zeitraum (1999–2024) UND die `format mcon_event`-Registerzeile steht (Mountain).
- **Lage:** (gemessen 2026-10-12 via `ci_manage log 38176003927` + mycelium-318) **MCON manifestiert:** `mcpherron_onsets.bin`, 314 555 Onsets, `format mcon_event`, sha `441df5f8…`. **Register-Drift gelöst** (mycelium-318). `bpa_gic` führt nur ein **4-Tage/5-min-Fenster** (2026-10-03..06), **kein Archiv** (gemessene Null) → 0 Onsets im MCON-Fenster; die zweite GIC-Zeugenreihe bleibt leer, die Relation läuft über die FMI-Reihe.
- **Blockade:** `bpa_gic` deckt den MCON-Zeitraum nicht (kein Archiv; nur FOIA).
- **Braucht:** (a) `bpa_gic`-Harvest auf 1999–2024 ausdehnen (Mycelium) — oder den MCON-Zeitraum verwerfen und die Relation auf die FMI-Reihe stützen; (b) die Surrogat-Null-Form vor jeder Behauptung durch den Rat.

### wy-max-t — α als Family-Bound gemessen; Blatt-Siegel offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Siegel auf `docs/blatt/fruehwarnsystem-praeregistrierung.md`.
- **Lage:** (gemessen 2026-10-12 via `gh run download 38166787414` + `wy_max_t_probe --combine`) der Westfall–Young-max-T-Null ist gemessen, α = 0.05, pooled B = 9999: (1-α)-Quantil **ABK-2024 = 2.4467 · ABK-2025 = 2.4433 · SOD-2024 = 2.4831** (Selftest 2.5856). Family-Maximum cleart an allen drei Punkten. family-clearing: `Bz→dB/dt` (3/3), `dB/dt→Speed` (3/3), `dB/dt→Density` (3/3), `dB/dt→Bz` (ABK-2024, SOD-2024), `Speed→dB/dt` (ABK-2025). MDE je Member gemessen. α steht im Blatt (`:78-85`).
- **Blockade:** das Blatt-Siegel ist Operator-Wort.
- **Braucht:** Operator-Siegel; die gemessenen Werte im GitHub-Artefakt `38166787414` in den Bestand (Mycelium).

### Receiver-Apertur (SPAN) — Schritt 3 offen
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** der nächste `ci-gate`-Lauf (Paritäts-Gate `field_spatial_fold_wgsl_cpu_parity`); danach Schritt 3.
- **Lage:** (gemessen 2026-10-12 via `cargo check` 0/0 + `git log` `5a3e629e7`) **Schritt 2 (Faltung)** gebaut und committet: `σ_eff² = extent² + global_scale²` in `field_spatial`/`field_spatial_grad` (WGSL) + CPU-Spiegel; `max(extent, scale)` gestrichen; Kontrakt `:34` nachgezogen.
- **Blockade:** das Paritäts-Gate ist ein Test-Target — `cargo check` kompiliert `#[cfg(test)]` nicht lokal; es läuft erst im CI.
- **Braucht:** (a) den nächsten `ci-gate`-Lauf lesen (`ci_manage log`), das Paritäts-Gate bestätigen; (b) **Schritt 3:** die deklarierte `ReceiverAperture` (`src/mathematikerin/receiver.rs`) an die Query-Naht (`vp.surface.w = global_scale`) binden — die deklarierte Blende je Receiver, nie in den Record; (c) die Operator-Browser-Messung (Fenster-Edit = Operator-Wort).

### GIC-Onset — AL-Einhüllende über die 154-Station-Familie
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** Abschluss der Harvest-Läufe `intermagnet-xyz-cdn` → die Assets `<code>_xyz_1m.bin` liegen im CDN.
- **Lage:** (gemessen 2026-10-12 via `ci_manage jobs`/`log 38164890620`) der Lauf trug 25 Stationen (`start 2000`, `end 2024`): 24 success, **`lov` cancelled** (LOV-Fetch HTTP 400 für 2023+); `38164966250` queued (same-workflow-Concurrency). Die Kette `intermagnet_xyz_compiler` → `al_network_proxy` → `gic_onset_detector` ist gebaut; die 154-Station-Partition liegt in `state/river/gic-family-*.txt` (31/25/98); Null-Baseline gemessen (`--null 200`: 1.62×/1.47×/1.35×).
- **Blockade:** die per-Station-`<code>_xyz_1m.bin`-Assets der noch fehlenden Stationen (24 fertig; `lov` 400).
- **Braucht:** `ci_manage log 38164966250` nach Abschluss; dann je Familie `al_network_proxy --family <fam.txt> --dir <pfad> --decl auto --baseline-block 60 --min-stations 2` und `gic_onset_detector --null 200`. (CDN-Registrierung + McPherron-`terms CC-BY-4.0` = Mountain/Mycelium.)

### GIC-Familien-Partition — zwei Register-Nägel, dann die Matrix-Läufe
- **Status:** blockiert | **Bindung:** eigen (Blockade: Mountain-Nägel)
- **Trigger:** die zwei Register-Nägel sind gesetzt (Mountain), dann Run der Matrizen.
- **Lage:** (gemessen `state/stimmen/2026-10-06_gic-familien-arm-ui-stimmen.md:20-80`) die Stimmen-Runde (GLM-5.3, qwen 3.8 Max, Kimi) einig: **kein** neuer Grammatik-Arm, sondern (i) ein Satz über Querfamilien-Zellen und (ii) die Familienregel als deklariertes `station→Familie`-Artefakt. Minimaler Patch: `matrix gic full` + `family auroral|subauroral|midlat stations <code…>` + `expect families 3` + `fdr by 0.05 over matrix` + `wymaxt 0.05 over family`.
- **Blockade:** die zwei Nägel (Mountain-Register); der Grammatik-Patch ist aus dem Verdikt noch nicht gebaut.
- **Braucht:** Mountain setzt die zwei Nägel; danach den Patch bauen und `matrix gic_auroral|gic_subauroral|gic_midlat full` fahren.

### Newell-OMNI-Alignment — Resolver gebaut, Lauf 38215224525 liest die Zelle neu
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss des `field-te-query`-Laufs `38215224525`.
- **Lage:** (gemessen 2026-10-12) der Resolver ist gebaut (`field_te_query.rs` `load_field_across_sources`: per-`station_code`-Scope, feinster native Zeuge als Ergebnis, gröberer Aggregationszeuge als getragener Riss statt `Err` — Mycelium `3467ec4f9`); Rat-Verdikt „Regel statt Rename". Vorher: die Zelle `newell_dphi_dt_omni->intermagnet_dbdt` war **floor (p=1)** — `abk_dbdt_1m` (60 s) vs `abk_dbdt_1h` (3600 s), corr 0.9388. **Lauf `38215224525` noch `pending`.**
- **Blockade:** keine (Resolver gebaut).
- **Braucht:** `ci_manage log 38215224525` → die Zelle `newell_geospheric_omni` neu lesen; erwartet: ABK `abk_1m` als Zeuge, `abk_1h` als getragener Riss.

### Flyby-Kette — recon bleibt (`ephemeris-juice-recon` + `flyby-path2-recon`)
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** `ephemeris-juice-recon` Wiedervorlage 2026-11-01 (ESA/ESOC publiziert die Post-Flyby-SPK). Wahrheit: `state/zustand/wartend.φ`.
- **Lage:** (gemessen 2026-10-11) die Wiedervorlage 2026-11-01 steht; `data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin` absent. `flyby-path2-recon`: orbc-SPK keyless `https://spiftp.esac.esa.int/data/SPICE/JUICE/kernels/spk/juice_orbc_000116_230414_310721_v01.bsp` (HTTP 206); die **1-σ-Kovarianz liegt nicht im SPICE-Baum**.
- **Blockade:** ESA/ESOC-recon-SPK + die 1-σ-Kovarianz absent.
- **Braucht:** am 2026-11-01 `archive_search --verdict` auf den ESOC-recon-Pfad; dann `flyby_ephemeris_gate --recon <recon.bin> --sigma-recon <km>`.

### Blatt-Kalibrierung — BCa benannt; Z-Null-Zahl aus CI 38215420000
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** (a) Operator-Siegel; (b) Abschluss des `bz-dbdt-delay-probe`-Laufs `38215420000`.
- **Lage:** (gemessen 2026-10-12) **BCa benannt** — die Intervall-Konstruktion des Punktschätzers steht im Blatt + Papier: **kein „Block-BCa"**, getragen wird ein **studentisiertes Moving-Block-Bootstrap** (delete-a-block-Jackknife Künsch 1989 als SE; Moving-Block-Perzentil als 1.-Ordnungs-Form; Subsampling Politis–Romano als Default) — Rat + Claude + DeepSeek V4 Pro + Nemotron + GPT-OSS konvergent. **Z-Null gebaut** (`bz_dbdt_delay_probe.rs`: 60-min-Shift + delete-one-Block-Jackknife-SE + studentisiertes Lag-Maximum); Workflow `bz-dbdt-delay-probe.yml` (Mycelium `f664c1b96`). **Lauf `38215420000` noch `queued`** (ABK/SOD 2024, `--n-surr 200`).
- **Blockade:** die Z-Null-Zahl braucht den CI-Lauf (Fetch/Compute).
- **Braucht:** `ci_manage log 38215420000` → die kalibrierte Z-Zahl ins Blatt (der Deskriptiv-Wert `Z = 142 min, r = 0.0223` von 2026-10-05 wartet darauf); α/Bz-Schwelle + Siegel = Operator-Wort (nach Future geroutet).

## An mountain

Origin: river-195/196/197 (2026-10-12) — Messungen/Befunde, die Mountains Register/Archivar betreffen (Sender-Register; die Owner-Session faltet).

- **`solar_f107_flux_sfu` — kein Identitäts-Riss, ein Zeitbasis-Defekt (gemessen).** (2026-10-12, `general`-Taucher) Der SWPC-Textarm liefert **TDB/J2000** (`lsk.unix_to_tdb`, `extract.rs:6506`, `parse.rs:2349`), das Penticton-`f107`-Asset **rohe Unix-Sekunden** (`src/archivar/f107.rs:22`, `day*86400`); feste Verschiebung `≈ 9.4673e8 s ≈ 30,0 a` → SWPC-2026 paart mit Penticton-1996 → r≈0.02 (mycelium-318 nennt corr 0.0232). **Zwei Archivar-Defekte:** (i) `f107::parse_series` wendet `unix_to_tdb` **nicht** an; (ii) `series_epoch_of` (`extract.rs:6506-6539`) wählt den ersten Key mit `"date"` — `avg_begin_date` kann `time_tag` verdrängen. **Braucht:** beide Arme auf dieselbe Epoch-Basis (TDB) bringen; dann den `f107`-Join erneut messen. `xfer:river197-1`
- **MCON-Spanne 1966–2025 vs Parser-Ende 2024-05-29** — nominale Beschreibung vs Parser-Fenster; für die BPA (2026) irrelevant. `xfer:river197-2`
- **THEMIS-ASI Stations-Tabelle (`src/archivar/themis_asi.rs`) — von River (Operator-Wort 2026-10-12) korrigiert (erledigt, Info).** PGM-Kopf-Höhe ist ellipsoidisch; gesetzt ATHA 549.82 · EKAT −110.6703/430.77 · PGEO 756.26 · MCGR 102.57. Anker `state/river/themis/calibration-2026-10-12.md`. `xfer:river197-3`

## An future

Origin: river-195/196/197 (2026-10-12) — Operator-Akt, den diese Session nicht selbst führen darf (Sender-Register; die Owner-Session faltet).

- **wy-max-t-Blatt-Siegel** — `docs/blatt/fruehwarnsystem-praeregistrierung.md` (`status: unsealed`): der Westfall–Young-max-T-Null ist gemessen (α=0.05, pooled B=9999; (1-α)-Quantil 2.4467/2.4433/2.4831; `Bz→dB/dt` family-clearing an allen drei Punkten; Artefakt `38166787414`). Die Vorbereitung ist River-eigen; **das Siegel setzt der Operator**. `xfer:river197-4`
- **Street-Level-Keys (Zugang) — erledigt (Operator-Wort 2026-10-12).** (a) **Mapillary erledigt:** `MAPILLARY_ACCESS_TOKEN` in `.secrets.local` (nur der Reader); `tools/measure/src/bin/mapillary_coverage.rs` gebaut; ATHA `images 0` gemessen. (b) **Google entfällt** (kein Karten-Billing). Kein Rest. `xfer:river197-5`

## An mycelium

Origin: river-197 (2026-10-12) — Manifestations-Akt in Myceliums Domäne (Sender-Register; die Owner-Session faltet).

- **THEMIS-ASI Keogramm-Assets auf den CDN (Register-Duty).** Der Meridian-Keogramm-Compiler (`themis_asi_timed_keogram_compiler`, Workflow `themis_asi-timed-keogram-cdn.yml`) und der 196/197-Bau (`skymap_fit`, `themis_asi_frame_onset`) sind lokal gemessen; die Assets (`tkdg_20240510_kapu_onset.bin` + die Keogramm-PGM) existieren nur lokal. **Braucht:** die Assets über `--ci-mode` manifestieren (Register-Duty, Mycelium). Anker `state/river/themis/frame-onset-2026-10-12.md`. `xfer:river197-6`

## Abschluss

Der Commit steht aus — er wartet auf das Operator-Wort (`/commit`); `/consent` ist der
session-weite Delegations-Consent, nie das Commit-Wort.
