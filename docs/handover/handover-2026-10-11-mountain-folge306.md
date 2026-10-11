<!--
  title: Handover — Mountain-Folge 306 (2026-10-11)
  session: Mountain-Linie in einem Pass — Ephemeriden Schritt 1 gefahren (Pioneer-10/11-ODF-Residuum gemessen), Asservatenkammer 9/9, FMHY-Klasse verortet
  class: handover
  date: 2026-10-11
  sha256: 85809170b11750e955c103dd8105e78f55c01e0900f7746a7673f994645a91f1
  status: live
-->
# Handover — Mountain-Folge 306 (2026-10-11)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). Diese
Session konsumierte `handover-2026-10-11-mountain-folge305.md` (→ `archiv/`).
flash only, kein pro/max. Der Operator-Trigger dieser Session: „was ist mit unseren
ephemeriden und warum fängst du die grossen punkte immer wieder an anstatt sie fertig
zu machen" — **Schritt 1 der Ephemeriden-Kette ist jetzt gefahren, nicht neu angesetzt.**

## Burn: open 0.0 · close 0.128 · cap 0.2 — Grund: line 0.0531 + general 0.0615 + explore 0.0135 (gemessen `session_burn`) + 2 lokale Einzel-Bin-Läufe (pioneer10/11_odf_residuum); deepseek-flash, kein pro/max

## Offen (aufgeschlüsselt)

### Beobachtungsoperator + Fit — Schritt 1 gefahren (P10+P11-Residuum gemessen); Schritt 2 offen
- **Status:** eigen (Bau/Messung) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt — Schritt 2 (Residuum verhören: Kette falsch / Modell trägt Rest / Beobachtung trägt Artefakt)
- **Lage:** (gemessen 2026-10-11, zwei lokale Einzel-Bin-Läufe `OMEGAFLOW_HIDDEN=1 cargo run -p omegaflow-measure --bin pioneer{10,11}_odf_residuum`, Daten aus den registrierten CDN-Assets `pioneer{10,11}_odf.bin` + `ephemeris_{earth,pioneer{10,11}_daily}.bin`)
  - **P11:** 27 907 ODF-Samples (Stationen 11/12/14/42/43/44/51/61/62/63, 1974-04-19..1990-10-01) → **24 808 modelliert**; je Station `obs = A·ṙ₂w + B_Pass`, **A ≈ +f/c (7,65–9,16 Hz/(m/s))**, Downlink-only A ≈ 2× (Uplink-Bein getragen); Residuum-RMS 2,6e2..6,6e4 Hz. `pioneer11_residuum.bin` **1 786 184 B, 24 808 Samples, Roundtrip parst**.
  - **P10:** 59 486 ODF-Samples (14/42/43/61/63, 1973-10-15..1998-07-21) → **59 363 modelliert**; A ≈ +f/c (Station 61: A 7,636 Hz/(m/s), RMS **0,169 Hz** bei n=6); **Riss** Stationen 43/63 (RMS 2,1e6/1,8e6 Hz) — die `ref_hz` läuft dort bis 2,292e9 Hz (X-Band-Mix) gegen 2,198e7 Hz der übrigen. `pioneer10_residuum.bin` **4 274 144 B, 59 363 Samples, Roundtrip parst**.
  - **Verdikt:** Vorzeichen/Maßstab des Beobachtungsoperators stimmen (A ≈ +f/c, Downlink ≈2×); das **erste Modell (baryzentrisches ṙ, `obs=A·ṙ+B_Pass`) trägt die Serie nicht** zu Hz für die meisten Stationen — die volle Beobachtungsgleichung (Uplink-Ramp, Station, Moyer) fehlt noch. Gegen die Horizons-DE440-Tagesephemeride → **`fit-residuum`, kein Blindtest**.
  - **Träger** (neuer Bin, mech. Klon von `pioneer11_odf_residuum.rs`): `tools/measure/src/bin/pioneer10_odf_residuum.rs` — `cargo check -p omegaflow-measure --bin pioneer10_odf_residuum` 0/0. **Namens-Riss:** die Ausgabe serialisiert mit `odf::write_p11r_bin` (Formatsname, nicht Mission) — funktional, aber der Name trägt p11.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Schritt 2 — die drei Lesarten; erster Hinweis ist der S/X-Band-`ref_hz`-Mix der Stationen 43/63.

### particle-cern — Teil B (Branches) gebaut; TStreamerElement-Liste + Baskets offen
- **Status:** eigen (Parser) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Bau-Schritt
- **Lage:** (gemessen 2026-10-11 via `explore`) `src/archivar/root.rs` trägt `inflate_zlib:391`, `decompress_object:573`, `parse_tree:687`, `visit_directory:734`, `walk_trees:774`, `parse_streamer_info_header:797` (endet nach `fLowerBound:835` **vor der Elementliste**). Baskets `fBasketBytes`/`fBasketEntry` ungebaut.
- **Blockade:** TStreamerElement-Elementliste nutzt ROOTs Klassen-Ref-Map (Vorbild `parse_tree:704–710`); verworfen statt unverifiziert ausgeliefert.
- **Braucht:** `parse_streamer_elements(obj, start, n_members) -> Result<Vec<StreamerElement>, _>` an Anker `root.rs:835` (n_members Objekte, je Klassen-Tag/Ref-Map, `kNewClassTag`→Name, sonst Ref-u32; TObject/TNamed-Präfix wie `:829–833`; `fType`+`fSize`); Header um `elements_offset` erweitern. **Baskets = eigener Schritt** (nicht mitbauen).

### FMHY-Quellen-Verdikt — research-data-Klasse verortet (nicht in den Surveys); 12 NEW gemessen
- **Status:** eigen (Quellen-Verdikt) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Verdikt-Schritt je Teilklasse
- **Lage:** (gemessen 2026-10-11) **Riss:** die „74/72-NEW"-Klasse lebt **nicht** in `survey-2026-10-08-fmhy-research-landscape.md`/`survey-2026-10-07-fmhy-forschungsschicht.md` (beide voll gelesen), sondern allein in `state/future/source-kandidaten-fmhy-2026-10-10.md:7` („Academic Papers", 422 Z.). 12 NEW gemessen (`archive_search --verdict`): `chinarxiv.org` 200 direkt · `alphaxiv.org` 200 (Arm `--alphaxiv` besteht) · `scholar.google.com` 200 direkt (ToS) · `citrus-search.com` 200 (Sniff) · `researchgate.net` 403/403/wayback-200 (bot) · `scilit.com` 403/403/wayback-200 (bot) · `zotero.org` 200 · `mendeley.com` 200 · `chunkr.ai` 200 (Sniff) · `sophon.at` 200 · `bulletpapers.ai` 429/429/wayback-200 (rate-limit) · `link.springer.com` 200 (Paywall).
- **Blockade:** die Klasse ist eine Kandidatenliste der Future-Linie, kein Mountain-Register; je Zeile fehlt der Arm/Compiler oder das Verdikt.
- **Braucht:** je Quelle `archive_search --verdict`/`--sniff` (Rest der Liste) + Verdikt-Zeile in `phi/{sources,declined_sources,blocked_sources}.φ` nach `docs/SOURCE_PORT.md`; Auth-Route statt `declined` (Operator-Wort 2026-10-08).

### Asservatenkammer — 9/9 Doks mit erstem Schritt gemessen
- **Status:** eigen (Register/Träger) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt je offenem Marker
- **Lage:** (gemessen 2026-10-11) gedeckt: `survey-2026-09-14-kapitulationen-pendings-inventur.md` 31/32 · `survey-2026-10-08-open-sources-delta.md` 11/13 · `survey-2026-09-03-orphan-verdicts.md` 12/15 · `survey-2026-10-09-redistribution-alternativen.md` 8/10 (Rest `globalfloods.eu`/GloFAS, `opensky-network.org` — kein Register-Träger) · `survey-2026-10-09-domaenen.md` 6/8 (Rest 44-Domänen-Liste `:86`, medizinisches Resultat `:93`) · `survey-2026-09-16-fremde-parser-sammlungen.md` 5/6 (Rest Airbyte-Totalzähler `:159`) · `survey-2026-10-08-fmhy-research-landscape.md` 4/5 (Rest MiMo Studio `:53`) · `survey-2026-10-07-fmhy-forschungsschicht.md` 2/2 · `survey-2026-09-17-omegaflow-legacy-konzepte.md` 2/2.
- **Blockade:** 5 Rest-Marker ohne Träger.
- **Braucht:** je Rest-Marker einen Träger (Register-Zeile/Code) **oder** ein Descope-Befund; `docs/surveys/survey-2026-10-08-research-api-mcp.md` vier Marker setzen (Mycelium-Bitte). Header-sha per `omega_sh sha` nachgerechnet.

### Mycelium-φ-Blöcke — Klima-Blöcke stehen; ecad-Riss + Serien-Manifest offen
- **Status:** eigen (Register/Verdikt) | **Bindung:** eigen (CI-Lauf: mycelium)
- **Trigger:** ecad-Riss entscheiden; je Block der erste CI-Lauf (sha256)
- **Lage:** (gemessen 2026-10-11) die Blöcke **stehen vollständig** in `phi/sources.φ` — SURFRAD `:11292–11310` (terms PD, format surfrad, origin, compiler, **sha256**, at earth, ttl 604800, field) · ECAD `:11172` · DWD-CDC `:21348–21398` · WorldClim `:31404` · AODN `:3` · KNMI `:1712`. Die in folge305 als „Verdikt-Zeilen fehlen" geführte Lage ist damit stale. Offen: der **`ecad`-Riss** (`ecad.eu` Portal vs. Quellhost `knmi-ecad-assets-prd.s3.amazonaws.com`); das **Serien-Manifest** (Compiler erzeugen je-Akt-Instanzen, die Blöcke sind Workflow-Default-Repräsentanten).
- **Blockade:** ecad-Quellhost nicht entschieden; ohne Serien-Manifest trägt jeder Block nur einen Default-Tag.
- **Braucht:** `archive_search --verdict` auf beide ECAD-Hosts + Verdikt-Zeile; Serien-Manifest-Format (Mountain) / CI-Lauf (Mycelium).

### PEP + Tudat — Schritt 0+1 gefahren; Schritt 3 (mehrere Bahnen) offen
- **Status:** eigen (Register + Bau) | **Bindung:** eigen
- **Trigger:** Schritt 3 — weitere Zeugen (Merkur/Venus Radar+VEX-Ranging) durch die Kette
- **Lage:** (gemessen 2026-10-11) Schritt 1 ist mit dem Beobachtungsoperator-Punkt gefahren (Pioneer-10 durch die Kette, `fit-residuum` gegen die Horizons-DE440-Ephemeride). Träger-Doks `docs/concepts/eigene-ephemeride.md`, `docs/surveys/survey-2026-10-10-ephemeris-quellen.md`. Vier Häuser (DE440/INPOP19a/EPM2021/PETREL19) als `ephemeris_*` registriert.
- **Blockade:** keine (Mountain-Seite).
- **Braucht:** Schritt 3 — der Operator auf weiteren Zeugen gegen weitere Häuser; Kette CI-nah verankern.

### FMHY-Routing — Arm-Hälfte gegenstandslos; Riss 4 bot-gated
- **Status:** eigen (Architektur) | **Bindung:** eigen
- **Trigger:** Riss 4/5-Abschluss
- **Lage:** (gemessen 2026-10-10, unverändert) Grenze ist die Manifestations-Achse: Query → `archive_search`-Arm; Messwert → `phi/sources.φ` + Compiler; Operator-Werkzeug → `tools/`; Blick/Portal → Lead. Riss 4 (Overpass — HTTP 406 direkt+Proton → bot-gated `pending`). KNMI gebunden via EDR; Mindat wartend.
- **Blockade:** Riss 4 bot-gated.
- **Braucht:** Riss 4-Abschluss (bot-gated) oder Descope.

### blocked_sources — 1 offene Klasse (particle-cern)
- **Status:** eigen (Bau/Register) | **Bindung:** eigen
- **Trigger:** nächster begrenzter Schritt
- **Lage:** (gemessen 2026-10-11) `phi/blocked_sources.φ` trägt **1** `blocked parser-def`: `::gap:particle-cern ×1`. PDS-PPI-Arm und GWOSC-HDF5-Arm stehen.
- **Blockade:** TBranchElement-v9-Decode (siehe particle-cern).
- **Braucht:** (1) particle-cern-Decode; (2) PDS-PPI Manifest-CI-Lauf `pds-ppi-cdn.yml` + Register-Zeile (Mycelium); (3) GWOSC/LOSC Registrierung + sha256 (siehe GWOSC-Strain).

### GWOSC-Strain — Block steht ohne sha256; CI-Lauf queued
- **Status:** eigen (Bau) | **Bindung:** eigen
- **Trigger:** `gwosc-cdn`-Lauf `38096865460` Abschluss
- **Lage:** (gemessen 2026-10-11) Operator-Wort **A** (eigene `quantity`-Klasse, `relative`); Block `phi/sources.φ:22007` steht (`url`/`terms CC-BY-4.0`/`format losc`/`origin`/`compiler`/`on earth 0 0 0`/`ttl 31536000`/`quantity losc_strain …`); **sha256 fehlt** (kein Lauf). `ci_manage view 38096865460`: **queued**, Ergebnis nicht gelesen.
- **Blockade:** kein CI-Ergebnis.
- **Braucht:** `ci_manage view 38096865460` → sha256-Zeile in den Block `:22013` nachtragen.

### ci-gate-Rot — `pc.rs` von der Fremd-Linie committet; nächsten Lauf messen
- **Status:** eigen (Messung) | **Bindung:** eigen
- **Trigger:** der nächste abgeschlossene `ci-gate`-Lauf am HEAD
- **Lage:** (gemessen 2026-10-11) die Mountain-Heilung `mci.rs`/`parcorr.rs`/`receiver.rs` ist committet; `pc.rs` wurde von der parallelen River-Linie committet (`10e0d1323 river 177: fill the screened set in pc_stable_skeleton_screened`). Die `ci-gate`-Läufe am HEAD sind queued/in_progress — kein Ergebnis gelesen.
- **Blockade:** kein abgeschlossener Lauf.
- **Braucht:** `ci_manage log <ci-gate-id>` auf dem nächsten abgeschlossenen Lauf → grün/rot messen; bei rot die verbliebenen Clippy-Lints unter `-D warnings` tilgen.

### Flyby-Kette — Residual in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** termin:2026-11-01
- **Trigger:** ESOC-Recon-Release (oder Descope)
- **Lage:** (gemessen 2026-10-09, unverändert) 157 ODF-Referenzen; `doppler.rs` absent; Wahrheit `state/zustand/wartend.φ:34`.
- **Blockade:** kein ESOC-Recon-Release.
- **Braucht:** ESOC-Release oder Descope-Befund für `doppler.rs`.

### iEEG — registriertes Wort 2026-10-06 maßgeblich
- **Status:** eigen (Register) | **Bindung:** eigen
- **Trigger:** ein neues Operator-Wort, das den Riss über 2026-10-06 hebt
- **Lage:** (gemessen 2026-10-11, unverändert) iEEG = privates Experiment (`state/zustand/wartend.φ:40`), kein CDN.
- **Blockade:** keine.
- **Braucht:** kein Schritt — nur ein neues Operator-Wort öffnet es.

### GIC-Paper — Trigger: Mycelium-Artefakt
- **Status:** wartend | **Bindung:** mycelium (Träger folge295 `#te-ground-truth`)
- **Trigger:** `te-bias-n`-Lauf `38038722712` Abschluss → Mycelium meldet den Ground-Truth-Abschnitt
- **Lage:** (gemessen 2026-10-11, unverändert) `te_ground_truth` in `.github/workflows/te-bias-n.yml:48`.
- **Blockade:** kein CI-Ergebnis.
- **Braucht:** nach Mycelium-Meldung — Paper §3.5/Abstract/§7 nachziehen.

### Exposom-Matrix → TE-Paar-Feed — Repräsentativpunkt ist Annahme-Akt
- **Status:** wartend | **Bindung:** eigen (Mountain-Feder)
- **Trigger:** extern gedeckter Repräsentativpunkt je Zeile gesetzt (Operator-Hand; Operator-Wort 2026-10-10)
- **Lage:** (gemessen 2026-10-10) Kein offener Datensatz trägt eine im Datensatz gemessene Koordinate; x-Kern-Serien (OpenAQ/Open-Meteo/NASA POWER) stehen wie gemessen.
- **Blockade:** der Repräsentativpunkt ist eine wissenschaftliche Annahme, keine Messung.
- **Braucht:** je Zeile den extern gedeckten Repräsentativpunkt — dann `phi/sources.φ`-Zeile (Mountain-Feder), dann Te-Paar-CI-Feed (Mycelium).

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82):
  `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern; Lauf lokal/silent,
  nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen:
  der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„mach das ab jetzt automatisch — committe und pushe selbst" | 2026-10-07 | Operator (Session, Mountain 264)
„bitte die grossen punkte parallel mit agenten abarbeiten und ABSCHLIESSEN, NICHT VERSCHLEPPEN" | 2026-10-11 | Operator (Session, Mountain 305)
„was ist mit unseren ephemeriden und warum fängst du die grossen punkte immer wieder an anstatt sie fertig zu machen" | 2026-10-11 | Operator (Session, Mountain 306)
„bypass-mirror 23 (UrhG-§95a- was bedeutet das die möchte ich bitte raus haben keine fragwürdigen links" | 2026-10-10 | Operator (Session, Mountain 303)
„können wir nun eine untersuchung machen was davon als arme in archive search sollte, was in tools und was in phi dateien?" | 2026-10-10 | Operator (Session, Mountain 303)

## Abschluss

Der Commit ist die letzte Handlung; das Operator-Wort („mach das ab jetzt automatisch", 2026-10-07)
trägt Commit und Push. **Dieses Atom (Mountain 306):** Ephemeriden-Schritt 1 gefahren (P10+P11-Residuum),
Asservatenkammer 9/9, FMHY-Klasse verortet, drei begrenzte Dispatches (general/grind-flash/explore).
- **Ephemeriden Schritt 1 gefahren:** `pioneer{10,11}_odf_residuum` lokal (Einzel-Bin, `OMEGAFLOW_HIDDEN=1`);
  A ≈ +f/c je Station, Downlink ≈2×; erstes Modell trägt die Serie nicht → volle Beobachtungsgleichung fehlt;
  `fit-residuum`, kein Blindtest. Neuer Bin `tools/measure/src/bin/pioneer10_odf_residuum.rs` (cargo check 0/0).
- **Asservatenkammer 9/9 Doks** mit erstem Schritt gemessen; 5 Rest-Marker ohne Träger benannt.
- **FMHY:** research-data-Klasse liegt in `state/future/source-kandidaten-fmhy-2026-10-10.md:7`, nicht in den Surveys (Riss); 12 NEW gemessen.
- **Mycelium-φ-Blöcke:** als stale belegt — die Klima-Blöcke stehen vollständig (SURFRAD sha256 verifiziert).
- **particle-cern:** nächster begrenzter Schritt exakt verortet (`root.rs:835`, `parse_streamer_elements`).
Geteilter Baum: `tools/utils/src/bin/archive_search.rs` und die drei `tools/harvest/src/bin/*_coverage.rs`
gehören fremden Linien — nicht angefasst/committet.
Eigene Pfade: `tools/measure/src/bin/pioneer10_odf_residuum.rs` · diese Übergabe · der Archiv-Move folge305.
