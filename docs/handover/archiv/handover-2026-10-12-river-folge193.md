<!--
  title: Handover — River-Folge 193 (2026-10-12)
  session: River-Linie in einem Pass — SPAN-Schritt 2 gebaut: die Schneidung als Faltung/Varianzsumme (σ_eff² = extent² + global_scale²) in `field_spatial`/`field_spatial_grad` (WGSL) + CPU-Spiegel `kernel_omega`/`kernel_omega_grad`; `max(extent, scale)` gestrichen; Doppelnatur von `extent` je Kernel deklariert (Kernel-Sigma 0/1/3/4/5/6/7 vs. Träger-Radius 2, hart `d²≤extent²`); WGSL/CPU-Paritäts-Gate `field_spatial_fold_wgsl_cpu_parity` (324-Zeilen-Grid, Toleranz 1e-5+1e-3·|cpu|); Kontrakt `docs/concepts/archivar-mathematikerin.md:34` nachgezogen. Netz-End-Validator erweitert (`bpa_gic` + Surrogat-Null; bpa_gic deckt den MCON-Zeitraum nicht → pending). THEMIS-ASI-Kamera-Modell gemessen (Skymap statt Formel; FOV-Riss 160°/170°). `cargo check` 0/0. Kein Trigger gefeuert (`--fired`=0, `--stale`=0).
  class: handover
  date: 2026-10-12
  sha256: f63667ad038ce9b286c5123fedfb18ab283dad6f837fac475aef5844eca049d3
  status: live
-->
# Handover — River-Folge 193 (2026-10-12)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Eine Session arbeitet so
viele Punkte ab wie möglich — die Delegation an Sub-Agenten (eigener Kontext)
macht die Anzahl problemlos. Nur eigene Arbeit: bei geteilten Dateien nur die
eigenen Hunks — committet wird nur der eigene Teil, fremde uncommittete Arbeit
wird nie überschrieben; gepusht wird, sobald der eigene Commit steht und
`origin/main` Vorfahr von HEAD ist (Fast-Forward).

## Burn: open 0.0 · close 0.13 · cap 0.50 — Grund: River-Linien-Session 193, deepseek-flash, kein pro/max — gemessen via `session_burn` (Linien-Session + 1 `grind-flash`-Taucher: SPAN-Schneidung/Paritäts-Gate + GIC-Validator-Erweiterung; + 1 `general`-Taucher read-only: THEMIS-ASI-Kalibrierung). Fälligkeit gemessen (2026-10-12 via `register_lookup`): `--fired river` = 0, `--stale river --persist 3` = 0; `--addressed river` = 3 Blöcke (alle in 191/192 gefaltet; die Blöcke bleiben in den Sender-Übergaben, Entfernung = Sender-Duty); `open_points_check` F192 = 0 absent / 0 stale-citations. Die wartenden Trigger (`intermagnet-xyz-cdn 38164890620`/`38164966250`, `field-te-query 38172797345`, `wy-max-t 38166787414`) am 2026-10-12 via `ci_manage view` gemessen: **queued/pending** — nicht gefeuert.

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
„ja" | 2026-10-11 | Operator (Session, River 192) — (1) den SPAN-Feld-Edit bauen; (2) die vier verwaisten `cargo fmt`-Hünke (Mountain-316/Mycelium-312-Rückstand) als Haus-Dienst committen. Beide ausgeführt (fmt: `8b52a9426`; SPAN-Schritt 1: `ReceiverAperture`).

Verbatim: `state/operator-gespraeche/2026-10-11-river.md`, `2026-10-10-river.md` und
`2026-10-09-river.md`.

## Träger (Prosa, eigene)

- `docs/concepts/kanal-ontologie-komplettbau.md` — der komplette Bauplan (P0–P10); P10.2a (der Parser-Arm ist Rivers Teil) als gebaut nachgeführt (`:215`).
- `docs/concepts/archivar-mathematikerin.md` — der Wire/GPU-Force-Vertrag; Träger des kinetischen Rahmens + der **Receiver-Apertur**-Kontrakt-Zeile (`:34`), erweitert um das **Schneidungs-Gesetz** (Faltung/Varianzsumme, Doppelnatur von `extent`); sha `3f395ea1…`.
- `docs/concepts/der-paradigmenwechsel.md` (9 offene Marker) — von sensory-255 gefaltet (Träger-Abgabe JUICE/Flyby-Path-2; Wartend.φ `flyby-path2-recon` Aufnehmer river).
- `docs/paper/flyby-path-2-falsification-metric-addendum.md` (4) — von sensory-255 gefaltet.
- `docs/paper/flyby-path-2-preregistration.md` (1) — von sensory-255 gefaltet.
- `docs/paper/flyby-path-2-addendum-2026-09-29.md` — die Zellen der Flugbahn stehen `pending` (nach dem Flyby); der `pending`-Rest ist die 1-σ-Kovarianz (siehe §Flyby-Kette).
- `docs/auftrag/auftrag-flyby2-kette.md` — Auftrag der Flyby-2-Kette; Marker sichten und schließen oder terminieren.
- `state/river/gic-research-dossier-2026-10-11.md` — das konsolidierte Dossier (GIC/Netz/THEMIS).
- `state/river/themis/` — Kapuskasing `kapu_themis21` + Gillam `gill_themis19`, 2024-05-10: KAPU `stream2`-Keogramme + `stream0`-Full-PNGs; GILL stream0-Anker.
- `tools/measure/src/bin/themis_asi_timed_keogram.rs` — das getimte Keogramm-Instrument (rein lokal, TKDG, UT-Achse); `Frame` trägt `#[derive(Debug)]` (river-192).
- `tools/measure/src/bin/gic_onset_relation.rs` — (river-192) paart die GIC-1-min-Reihe (FMI) mit der MCON-Onset-Liste; **(river-193)** erweitert um `bpa_gic` (11 Kanäle, eigener `BPAG`-Parser) als zweite Zeugenreihe + Surrogat-Null (10 Kreisverschiebungen); `cargo check`/`build` 0/0.
- `state/river/themis/calibration-2026-10-12.md` — **neu (river-193):** die gemessene THEMIS-ASI-Kamera-Kalibrierung (Skymap statt Formel; PGM-Kopf `Optical projection a1=1.5…`; Orientierung row0=N/col0=Ost; FOV-Riss 160°/170°; gill-Kopf 56.3539/−94.6557/0.0; kapu-`alt` pending).
- `src/mathematikerin/receiver.rs` — `ReceiverAperture` (deklariertes Receiver-Feld, Schritt 1, river-192); Kontrakt `docs/concepts/archivar-mathematikerin.md:34`.
- `src/mathematikerin/shaders.rs` + `src/mathematikerin/tests.rs` — **Schritt 2 (river-193):** die Schneidung als Faltung; `field_spatial`/`field_spatial_grad` + Spiegel `kernel_omega`/`kernel_omega_grad`; Paritäts-Gate `field_spatial_fold_wgsl_cpu_parity` (324-Grid, Toleranz `1e-5+1e-3·|cpu|`); `cargo check` 0/0.
- `state/stimmen/2026-10-11_river-apertur-span-rat.md` — Rat+Forschungs+UI-Runde zur SPAN/Apertur-Frage (Verdikt (a), Riss Qwen (b)).
- `state/stimmen/2026-10-11_river-apertur-cut-rat.md` — Runde zur **Schneidungsform** (Forschung + Rat 5/5 + UI 5/5 → (a) Faltung; Riss: `extent`-Doppelnatur).
- `state/river/intermagnet-dispatch-2026-10-11.md`, `state/river/gic_null_baseline_measured.md`, `state/river/gic-sources-*.md`, `state/river/registrierungen-2026-10-11.md`, `state/stimmen/2026-10-11-river-gic-*.md` — GIC-Zeugen-/Quellen-/Stimmenbestand.
- `docs/blatt/blatt-gic-breitenband-familien.md` (`status: unsealed`) — Siegel = Operator-Wort (als `## An future` transportiert).
- `docs/surveys/survey-2026-10-05-stoerungs-experiment-fehlende-faeden.md` — Asservatenkammer (11 offen).
- `docs/surveys/survey-2026-10-08-sonnen-render-archaeologie.md` — §4 am 2026-10-11 nachgezogen.
- **Offene-Prosa-Träger (future-235-Abgabe, River-Carrier):** `docs/auftrag/auftrag-flyby2-kette.md` (3) · `docs/auftrag/auftrag-universelles-vlies.md` (2) · `docs/blatt/blatt-te-externer-steuerparameter.md` (1) · `docs/blatt/fruehwarnsystem-praeregistrierung.md` (1, `status: unsealed`) · `docs/paper/flyby-path-2-addendum-2026-09-29.md` (27) · `docs/surveys/survey-2026-09-26-membran-ladearchitektur.md` (1 — Marker `:122` gemessenes `descoped`, Träger nur zur Bestätigung). `register_lookup --orphan-docs` = 0.

## Netzbetreiber-Referenz (state, nicht re-run)

Gemessen 2026-10-11. Quellen: `state/future/space-weather-service-kante.md` §Adressat · `state/river/gic-sources-broad-2026-10-11.md` · `state/river/registrierungen-2026-10-11.md` · `state/future/funding-register-fuenf-achsen-2026-10-09.md`.

- **ENTSO-E** (EU, Dach 40 TSOs) — `token pending` (A78-Compiler gebaut). **Fingrid** (FI) — Token + Compiler; `fingrid-cdn 38162332055`. **NESO** (UK) — keyless CKAN 200, kein Outage-Datensatz. **BGS** (UK) — nur B_GIC-Index/Modell. **Statnett** (NO) — PDF-only. **Amprion** (DE) — keyless CSV-Outage-API. **TenneT** (DE) — 403 geo-block. **50Hertz** (DE) — PDF-only. **TransnetBW** (DE) — keine eigene maschinenlesbare Reihe; delegiert an die ENTSO-E-Plattform (`10YDE-ENBW-----N`). **Svenska Kraftnät** (SE) — CKAN, kein Outage.

Operator-Wort 2026-10-11: auf die Zulassung warten (ENTSO-E/NERC `pending`); die Netzbetreiber aus `state` referenzieren, nicht re-suchen.

## Offen (aufgeschlüsselt)

### Receiver-Apertur (SPAN) — Schritt 2 (Faltung) gebaut; Paritäts-Gate + Schritt 3 offen
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** der nächste CI-Lauf (`ci-gate`/`clippy` auf dem nächsten Push) für das Paritäts-Gate `field_spatial_fold_wgsl_cpu_parity`; danach Schritt 3 (die deklarierte `ReceiverAperture` an die Query-Naht binden).
- **Lage:** (gemessen 2026-10-12 via `cargo check` 0/0) **Schritt 2 gebaut:** die Schneidung ist die **Faltung** `σ_eff² = extent² + global_scale²` in `field_spatial`/`field_spatial_grad` (WGSL, `shaders.rs`) + CPU-Spiegel `kernel_omega`/`kernel_omega_grad` (`tests.rs`); `max(extent, scale)` gestrichen; `extent`-Doppelnatur deklariert (Kernel-Sigma 0/1/3/4/5/6/7 vs. Träger-Radius 2, hart `d²≤extent²`); Paritäts-Gate `field_spatial_fold_wgsl_cpu_parity` (324 Zeilen, d×extent×scale×absorption, Toleranz `1e-5+1e-3·|cpu|`, Kernel 2 direkt in Rust); Kontrakt `:34` + sha `3f395ea1…` nachgezogen. Verdikt: Rat 5/5 (a) + UI 5/5 + Forschung; Riss getragen: `extent`-Doppelnatur.
- **Blockade:** das Paritäts-Gate ist ein Test-Target — `cargo check` kompiliert `#[cfg(test)]` nicht lokal; es läuft erst im CI.
- **Braucht:** (a) den nächsten `ci-gate`-Lauf lesen (`ci_manage log`), das Paritäts-Gate bestätigen; (b) **Schritt 3:** die deklarierte `ReceiverAperture` (`src/mathematikerin/receiver.rs`) an die Query-Naht (`vp.surface.w = global_scale`) binden — die deklarierte Blende je Receiver, nie in den Record; (c) die Operator-Browser-Messung (Fenster-Edit = Operator-Wort).

### GIC-Onset — AL-Einhüllende über die 154-Station-Familie
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** Abschluss der Harvest-Läufe `intermagnet-xyz-cdn` `38164890620` + `38164966250` → die Assets `<code>_xyz_1m.bin` liegen im CDN. (gemessen 2026-10-12 via `ci_manage view`: beide **queued/pending** — nicht gefeuert)
- **Lage:** (gemessen 2026-10-11) Kette `intermagnet_xyz_compiler` → `al_network_proxy` (`--decl auto`, Block-Median-Baseline, untere Einhüllende, `--min-stations 2`) → `gic_onset_detector` gebaut; die 154-Station-Partition: `state/river/gic-family-auroral.txt` (31), `…-sub-auroral.txt` (25), `…-mid-latitude.txt` (98). Null-Baseline gemessen (`gic_onset_detector --null 200`: real/null = 1.62×/1.47×/1.35×, tol5/15/30, `prec_ge = 0/200`). **Riss:** die same-workflow-Concurrency-Gruppe (`cancel-in-progress:false`) hält nur 1 running + 1 pending.
- **Blockade:** die per-Station-`<code>_xyz_1m.bin`-Assets fehlen noch (Harvest queued/pending).
- **Braucht:** nach Abschluss `ci_manage view 38164966250` lesen; dann je Familie `al_network_proxy --family <fam.txt> --dir <pfad> --decl auto --baseline-block 60 --min-stations 2` und die Enveloppe mit `gic_onset_detector --null 200` gegen McPherron messen. (CDN-Registrierung + McPherron-`terms CC-BY-4.0` = Mountain/Mycelium.)

### GIC-Familien-Partition — zwei Register-Nägel, dann die Matrix-Läufe
- **Status:** blockiert | **Bindung:** eigen (Blockade: Mountain-Nägel)
- **Trigger:** die zwei Register-Nägel sind gesetzt (Mountain), dann Run der Matrizen.
- **Lage:** (gemessen `state/stimmen/2026-10-06_gic-familien-arm-ui-stimmen.md:20-80`) die Stimmen-Runde (GLM-5.3, qwen 3.8 Max, Kimi) einig: **kein** neuer Grammatik-Arm, sondern (i) ein Satz über Querfamilien-Zellen und (ii) die Familienregel als deklariertes `station→Familie`-Artefakt (Quelle, Epoche, Schwellen, Version). Minimaler Patch: `matrix gic full` + `family auroral|subauroral|midlat stations <code…>` + `expect families 3` + `fdr by 0.05 over matrix` + `wymaxt 0.05 over family`.
- **Blockade:** die zwei Nägel (Mountain-Register); der Grammatik-Patch ist aus dem Verdikt noch nicht gebaut.
- **Braucht:** Mountain setzt die zwei Nägel; danach den Patch bauen und `matrix gic_auroral|gic_subauroral|gic_midlat full` fahren.

### Disjunkter optischer Onset-Zeuge (THEMIS-ASI) — Kamera-Modell gemessen
- **Status:** eigen (Messung/Bau) | **Bindung:** eigen
- **Trigger:** der nächste Mess-/Bau-Schritt (Compiler-MIRACLE-Form `:325`; kapu-`alt` im PGM-Kopf).
- **Lage:** (gemessen 2026-10-12, Artefakt `state/river/themis/calibration-2026-10-12.md`) Die Pixel→Sky-Abbildung ist **keine** Formel, sondern das per-Station-**UCalgary-Skymap** (`.sav`: `FULL_AZIMUTH`/`FULL_ELEVATION`/`FULL_MAP_*`), gelesen von asilib/`pyUCalgarySRS`. Der nominale optische Kopf steht in jeder `stream0`-PGM-Datei (`Optical projection a1=1.5,a3=0,b2=0,b4=0`, stationsspezifisches `Optical center x0,y0`, `Pixel aspect ratio 0.5181`, `Geodetic lat/lon/alt`); die Form der a1..b4-Koeffizienten ist **nicht publiziert** (`pending`). Orientierung (am Code gemessen): roh row0=N, Spalte 0=Ost, Spalten nach West; Kompass-Azimut im Uhrzeigersinn; das 16-Spalten-Band (cols 120–135) tastet den **N–S-Meridian**. **FOV-Riss: 160° (THEMIS-GBO-Team) vs 170° (NASA SPASE)** — getragen, nicht gemittelt. gill = 56.3539/−94.6557/0.0 (PGM-Kopf, autoritativ; CSV 56.376 weicht ab); kapu `alt` `pending`. **(river-192, Referenznacht)** kapu 2024-05-10: 86 stream0-Minuten, 1720 Frames, UT 06:50:00.035–08:15:57.032Z; größter Framesprung **+16.3 @ 07:55:27.008Z** (~2 min nach dem McPherron-07:53-Onset). **Koord.form (Mountain-320):** die Stationstabelle ist **Compiler-Eingabe**, kein `record`/`witness`; Riss a (asilib `asi_locations.csv`, 23, lon −180..180) autoritativ, b (Berkeley `thm_asi_stations.pro`, 20, lon 0–360) als zweite Zeugenlinie; fehlendes `alt` → `pending`, nie `0.0`.
- **Blockade:** (i) die a1..b4-Form unpubliziert (das Skymap selbst ist die Projektion); (ii) kapu-Höhe noch nicht aus dem PGM-Kopf gelesen; (iii) die echte meridionale Projektion braucht das Skymap, derzeit nutzt das Instrument ein dokumentiertes Proxy-Band.
- **Braucht:** (a) `themis_asi_timed_keogram_compiler.rs:325` die MIRACLE-Form emittieren (`on earth <lat> <lon> <alt>` + `station <CODE>`, Datum aus dem PGM-Kopf) — Mycelium; (b) kapu-PGM-Kopf (Artefakt `…/stream0/2015/03/17/kapu_themis21/ut06/20150317_0600_kapu_themis21_full.pgm.gz`) lesen → lat/lon/alt; (c) den FOV-Riss tragen oder über den Skymap auflösen (nicht mitteln); (d) den Skymap `.sav` als Projektionsquelle für die echte meridionale Projektion nutzen.

### Netz-End-Validator — GIC↔Onset-Bin; `bpa_gic`-Paarung + Surrogat-Null gebaut
- **Status:** wartend | **Bindung:** eigen (Bau)
- **Trigger:** das MCON-Asset wird CDN-manifestiert (`mcpherron-cdn 38176003927` queued) UND `bpa_gic` deckt den MCON-Zeitraum.
- **Lage:** (gemessen 2026-10-12 via `cargo run -p omegaflow-measure --bin gic_onset_relation`, rc=0) `gic_onset_relation.rs` erweitert: **`bpa_gic`** (11 Kanäle; eigener `omegaflow::archivar::bpa_gic`-Parser — MAGIC `BPAG`, 8-B-Header, `t_unix` bereits Unix-Sekunden, `present`-Bit → `Option`) als zweite Zeugenreihe + **Surrogat-Null** (10 feste Kreisverschiebungen ±1/3/6/12/24 h, in die fmi-Spanne gewickelt; eine Permutation wurde bewusst ausgelassen — gleiche Response-Multimenge). Messung fmi_gic_1min: n=95 019, median Response 0.28 A, +6h-Control 0.23 A, ratio 1.217; Surrogat-Null median 0.23, mean 0.355, σ 0.929 (**vorläufig — eine Messung, keine Behauptung**). **bpa_gic:** 11 424 Records, Spanne **2026-10-03..2026-10-06** (~4 Tage) → **0 Onsets im MCON-Fenster** (MCON endet 2024-05-29) → Reihe bleibt `pending`, keine fabrizierten Werte. `cargo check` 0/0.
- **Blockade:** (i) MCON nicht manifestiert (`phi/sources.φ:21545-21551` trägt `format substorm`/sha `0c99e7d8…`; der Compiler emittiert `mcon_event`/sha `441df5f8…`); (ii) `bpa_gic` deckt den MCON-Zeitraum nicht.
- **Braucht:** (a) MCON-Manifestation + `format mcon_event` (Mycelium/Mountain, gefaltet); (b) die `bpa_gic`-Harvest-Spanne auf den MCON-Zeitraum (1999–2024) ausdehnen (`bpa_gic_compiler.rs`/Workflow, Mycelium) — sonst bleibt die zweite Zeugenreihe leer; (c) die Surrogat-Null-Form vor jeder Behauptung durch den Rat.

### Vlies-Matrix-Alignment — Fluß-Fix gebaut (CDAWeb-ACE-Archiv); Matrix-Lauf unterwegs
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss von `field-te-query 38172797345` (Job `matrix-vlies`). (gemessen 2026-10-12 via `ci_manage view`: **queued** — nicht gefeuert)
- **Lage:** (gemessen 2026-10-11) Operator-Wort an River (Mountain/Mycelium beschäftigt). **Ursache gemessen** (Log `38119733631`): die SWPC/RTSW-Kanäle hingen an den **Live-Kurzreihen** `rtsw_mag_1m.json`/`rtsw_wind_1m.json` (Breite ≈ 1 Tag) — die mit den Archiv-Kanälen nie überlappen → 126/210 Zellen `alignment pending`. **Fix gebaut (river-191, Commit `b5d557b72`):** die zwei Blöcke in `phi/sources.φ:223-239` zeigen auf den stündlichen CDAWeb-ACE-Archiv-HAPI `AC_H2_MFI`/`AC_H2_SWE`, Fenster `2010-01-01..2024-07-01`.
- **Blockade:** keine (Fix gebaut, gepusht); der Messlauf läuft.
- **Braucht:** nach Abschluss `ci_manage log 38172797345` lesen → die `matrix-vlies`-Zeilen: erwartet, dass die 126 `alignment pending` auf ~0 fallen; sonst die verbliebenen Paare + Ursache messen (nicht raten).

### wy-max-t — α/X/Z/Bz benennen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Abschluss des Laufs `wy-max-t 38166787414`. (gemessen 2026-10-12 via `ci_manage view`: **queued** — nicht gefeuert)
- **Lage:** (gemessen 2026-10-11) Die kalibrierte Westfall–Young-max-T-Null folgt daraus; α (und X/Z/Bz-Schwelle) bleibt bis dahin `pending`. Vorläufer: `docs/blatt/fruehwarnsystem-praeregistrierung.md` (unsealed, GIC/Bz-Vorhersagezelle).
- **Blockade:** Lauf queued.
- **Braucht:** nach Abschluss `ci_manage view 38166787414` lesen → Artefakte `wy-max-t-selftest.txt` + `wy-null-*`; dann α/X/Z/Bz benennen; das Siegel setzt der Operator.

### Flyby-Kette — recon bleibt (`ephemeris-juice-recon` + `flyby-path2-recon`)
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** `ephemeris-juice-recon` Wiedervorlage 2026-11-01 (ESA/ESOC publiziert die Post-Flyby-SPK). Wahrheit: `state/zustand/wartend.φ`.
- **Lage:** (gemessen 2026-10-11) die Wiedervorlage 2026-11-01 steht; `data/ssd.jpl.nasa.gov/ephemeris_juice_recon.bin` absent. `flyby-path2-recon`: orbc-SPK keyless `https://spiftp.esac.esa.int/data/SPICE/JUICE/kernels/spk/juice_orbc_000116_230414_310721_v01.bsp` (HTTP 206); die **1-σ-Kovarianz liegt nicht im SPICE-Baum**.
- **Blockade:** ESA/ESOC-recon-SPK + die 1-σ-Kovarianz absent.
- **Braucht:** am 2026-11-01 `archive_search --verdict` auf den ESOC-recon-Pfad; dann `flyby_ephemeris_gate --recon <recon.bin> --sigma-recon <km>`.

## Abschluss

Der Commit steht aus — er wartet auf das Operator-Wort (`/commit`); `/consent` ist der
session-weite Delegations-Consent, nie das Commit-Wort.
