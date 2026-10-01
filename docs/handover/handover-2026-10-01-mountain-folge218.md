<!--
  title: Handover — Mountain-Folge 218 (Stand 2026-10-01)
  session: Mountain-Folge 218
  class: handover
  date: 2026-10-01
  sha256: 9d3d3a6fea962fe8cd61b2cd6233c8c55bc60b7a4e82fd1f195ac7b2a45cdf2e
  status: live
-->
# Handover — Mountain-Folge 218 (2026-10-01)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`). In
diesem Atom: der `pds3_img`-Roundtrip-Fehler im Archivar gefunden und geheilt
(Bandname > 32 Byte wurde im Pack still gekappt, der Roundtrip-Guard verwarf das
Asset — deshalb blieb `phi/harvest.φ` `format pds3_img` `asset fehlt`). Die
adressierten Blöcke aus future-163 und mycelium-216 sind gefaltet (alle Einträge
waren bereits getragen: `ttl` gesetzt, Datenbestand in `archive-root/declined/`,
Anderson-ITRF/EOP descoped, Ephemeriden-`_long`-Zeilen vollständig). Dazu die
**Zeugen-Sweep-Faltung (Mapping v3)**: die 6 `decline`-Event-Feeds
(`geonet-M3`, `seismicportal minmag 3/5`, `tmd.go.th`, `USGS-Detail`,
`tsunami.incois`) tragen jetzt die fünfte Art `point-event` in der Verdikt-Evidenz
(als Zeuge und als Duplikat getrennt). Die 3 live-Quellen (`Chile`, `tohoku`,
`jma`) sind bereits in `sources.φ`; ihre `witness point-event`-Admission ist
**gebaut** (Magic `ERBQ`, Compiler `quake_ptevent_compiler`, 3 `witness`-Blöcke);
die CDN-Manifestation steht aus (s. unten).

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„erst messen" — Kandidaten vor jedem Verdikt messen | 2026-09-27 | Operator (Mountain 187)
„jeder Punkt trägt eine Empfehlung; wartende Linien erhalten eine bevorzugte Abarbeitungsbitte" | 2026-09-29 | Operator (Mountain 204)
„vorbestehend ist verboten mein wort" — alle über-256-Zeichen-`note`-Zeilen geheilt | 2026-09-30 | Operator (Mountain 209)
„braucht es wirklich pro?" — pro nur mit benanntem Hart-Atom oder gemessener flash-Fehllage | 2026-09-30 | Operator (Session, Mountain 211)
„die url/format-Zeilen sind ohne tragfähigen Arm vorzeitig" — kein url/format ohne deckenden Arm | 2026-09-30 | Operator (Session, Mountain 211)
„arbeite deine Liste bis zur Kante ab" — jeder eigene Punkt bis zur Kante, nichts Machbares liegen lassen | 2026-09-30 | Operator (Session, Mountain 212)
„verschleppen und nicht eigenes ist verboten" — Linienliste nur `eigen`, jeder Punkt im Atom bis zur Kante | 2026-09-30 | Operator (Session, Mountain 213)
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-09-30 | Operator (Session, Mountain 216)
„<LOCK-Wort für das private Experiment>" | 2026-10-01 | Operator (Session, Mountain 217) — verbatim im privaten Cut `state/operator-gespraeche/2026-10-01-mountain.md`; LOCK privat, kein CDN/`sources.φ`/`witnesses.φ`, kein getrackter Baum; zur Faltung nach Futures privatem Register (Origin: mountain folge217)
„alles was das experiment betrifft bleibt privat" | 2026-10-01 | Operator (Session, Mountain 217) — stehend: das ganze private Experiment (Daten, Ableitungen, experiment-spezifischer Code) bleibt privat; private Heimat `state/mountain/kuprat-complex-te/`; zur Faltung nach Futures privatem Register (Origin: mountain folge217)
„ich will dass ihr inhalt bearbeitet falls notwendig wird und die datei entfernt" | 2026-10-01 | Operator (Session, Mountain 217) — Verwahrung Mountain-185 („C") aufgehoben: der verwahrte Orphan-Doc-Nachzug-Patch (Mountain-185) ist entfernt; `git apply --check` scheitert (Docs weitergelaufen → stale), Ziel-Docs nicht mehr in `--orphan-docs` → Inhalt überholt, kein Nachzug nötig

## Offen (aufgeschlüsselt)

### pds3_img-Asset — Roundtrip-Fehler geheilt, Manifestation ausstehend
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** eigener Commit+Push des Fixes → `gh workflow run pds3-img-cdn.yml`; danach `--verdict` des `pds3_img_*`-Assets unter `pds-geosciences.wustl.edu`.
- **Lage:** (gemessen 2026-10-01 via `ci_manage log 36737530030` + gezielter Lauf `cargo run -p omegaflow-harvest --bin pds3_img_compiler`) der CI-Lauf scheiterte nicht nur am M3-`.HDR` HTTP 403, sondern die **Mini-RF-Kompilierung selbst verwarf ihr Asset**: das Label trägt `BAND_NAME = "CROSS POWER INTENSITY (IMAGINARY)"` (33 Byte), `pack` kappte den Namen still auf `BAND_NAME_BYTES = 32`, `parse_image` las die gekappte Form → `parsed != raster` → „roundtrip void — the image stays unverified". **Geheilt:** `BAND_NAME_BYTES = 64` (Maximum des Live-Labels 33 → nächste Zweierpotenz), neuer Test `pack_roundtrip_holds_for_band_names_longer_than_the_legacy_width`; `cargo check` 0/0. Der gezielte Lauf packt **beide** Raster (Mini-RF `pds3_img_fsb_00720_1cd_xhu_84n209_v1.bin`, 29896248 B, sha256 `6aa0eb1f…`, roundtrip holds; M3 `pds3_img_m3g20081118t222604_v03_loc.bin`, 8758832 B, sha256 `5771de98…`, roundtrip holds) — der M3-Host ist von diesem Netz erreichbar, der 403 ist CI-Runner-spezifisch (Datacenter-IP).
- **Blockade:** der Fix ist noch nicht committet/gepusht (Commit-Wort des Operators ausstehend; geteilter Baum — fremde Sensory-Hunks liegen uncommittet).
- **Braucht:** nach Commit+Push `gh workflow run pds3-img-cdn.yml`; der Lauf schreibt mit dem Fix mindestens das WUSTL-Asset (Compiler schreibt `written ≥ 1` → exit 0 auch bei M3-403), dann `phi/harvest.φ:235` auf `asset present` messen.

### Ephemeriden-Bins — Unstetigkeit an den 32-d-Granulat-Grenzen (Ursache geheilt)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner Ephemeriden-Re-Manifest-Lauf (`kernel-flatten` / `ephemeris_compiler`) → dann `ephemeris_granule_census` gegen die neuen Bins.
- **Lage:** `ephemeris_granule_census` (gemessen 2026-09-30): DE441 Erde 346877 Granulate, INPOP 684, EPM 4875 — je 32.0 d Halbbreite, Chebyshev-Grad 17; rekonstruierte Position sprang an den Grenzen um Median 0.044 km (max 0.19 km). Ursache: `src/mathematikerin/least_squares.rs::solve_normal_equations` reduzierte den RHS nicht mit der Matrix → Fit ~40 m neben der Funktion; **geheilt** (Test `normal_equations_reduce_the_rhs_with_the_matrix`, `granule_fits_meet_at_the_shared_boundary`; `--selftest` 0.00003 m = f64-Boden). Der Fix ist committet (`946c7b232`).
- **Blockade:** die CDN-Bins sind mit dem fehlerhaften Solver kompiliert — der Fix greift erst nach Re-Manifest; der Re-Manifest-Dispatch ist Mycelium-Feder (der Fix liegt auf `main`).
- **Braucht:** Mycelium dispatcht die Ephemeriden-Re-Manifest-Workflows (`de44-cdn`, `inpop-epm-cdn`, `kernel-flatten` + die Missions-Compiler); danach `ephemeris_granule_census` gegen die neuen Bins.

### Ephemeriden-abhängige Messungen — nach dem Re-Manifest nachziehen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** grüner Ephemeriden-Re-Manifest-Lauf (`kernel-flatten` / `ephemeris_compiler`).
- **Lage:** (gemessen 2026-09-30 via `ephemeris_granule_census` + `sread docs/paper/*`) die aktuellen CDN-Bins tragen den ~40-m-Fit-Fehler. **Betroffen (Signal ≤ ~200 m):** `eclipse_shadow_probe` Haus-Cross-Check (`docs/paper/eclipse-clock-worldlines.md:45`: DE442↔INPOP19a Erde 0.064 km, Sonne 0.164 km, Mond 0.168 km — Größenordnung des Bugs, möglicherweise Artefakt statt Haus-Riß); `ephemeris_house_gate` (Δ DE↔INPOP 0.19 km); `flyby_anderson_probe`. **Riss am Siegel:** die präregistrierten Trajektorie-Hashes `flyby-path-2-preregistration.md:21/23` sind Hashes von mit dem Bug kompilierten `horizons_compiler`-Bins — ein Re-Manifest ändert sie (Neu-Siegeln post-hoc unzulässig). **Robust (Signal ≥ km):** Neptune-/Uranus-Rift, KBO-Residuum, Dunkel-Materie-Residuum, Signal-Konus, Galileo-Rotor.
- **Blockade:** der Re-Manifest (Mycelium-Feder).
- **Braucht:** nach dem Re-Manifest `eclipse_shadow_probe` (2017 + 2024), `ephemeris_house_gate` (6 Anderson-Epochen), `flyby_anderson_probe`, `ephemeris_granule_census` neu laufen und die Verdikte fortschreiben (Verschiebung > genannte Unsicherheit = betroffen).

### quake_ptevent-Zeugen — gebaut, CDN-Manifestation ausstehend
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Commit+Push → `gh workflow run quake-ptevent-cdn.yml`; danach `phi/harvest.φ` `format quake_ptevent` auf `asset present` messen.
- **Lage:** (gemessen 2026-10-01 via `grind-flash` + eigener Verifikation) der Point-Event-Record ist gebaut: Magic `ERBQ` (`src/archivar/quake_event.rs`, REC_BYTES 32 — order/present/ipix/lat/lon/mag/depth/jd_utc; `src/archivar/witness.rs` `ERBQ`→`PointEvent`, Test), Compiler `quake_ptevent_compiler` (Chile/Tohoku GeoJSON `properties.{mag,depth,latitude,longitude,time_}`; JMA `list.json` `mag`/`cod`/`at`), 3 `witness point-event`-Blöcke in `phi/witnesses.φ`, `phi/harvest.φ` `format quake_ptevent`. Gezielter Lauf: Chile 1, Tohoku 1, JMA 224 Records. `cargo check` 0/0, `harvest_reg --check` 40/40. **Riss geheilt:** die Feld-Register-URLs `sources.φ:998`/`:1076` trugen `/FeatureServer/query` (HTTP 400 gemessen) → auf `/0/query` (200) korrigiert. Das CDN-Release `quake-ptevent` existiert noch nicht → `harvest.φ` `asset fehlt`.
- **Blockade:** Commit+Push + Workflow-Dispatch.
- **Braucht:** `gh workflow run quake-ptevent-cdn.yml`; dann das Asset (`bytes sha url`) in den `note`-Zeilen von `phi/harvest.φ` und `phi/witnesses.φ` fortschreiben.

## Träger (Prosa, eigene)

- `docs/auftrag/auftrag-flyby2-kette.md` — σ-Metrik-Kette (3 Marker); Trigger JUICE In-Situ / Δ publiziert, `flyby_ephemeris_gate` (CI).
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` — Holdings-Inventur; Schritt 2 gemessen (Holdings + Repo-`data`).
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` — Relevanz-Erstpass; die Pending-Einträge sind im `dead_sources.φ` disponiert, die Prosa bleibt datierte Messung.
- `docs/surveys/survey-raetsel-bestand.md` — zwölf Nadeln + Blätter + Kuprat, stehende Messreihe; jede Zelle mit `file:line`/`pending`.

## Register-Träger (eigene)

- `phi/harvest.φ` `format pds3_img` (Chandrayaan-1 Mini-RF / M3) — `asset fehlt`; Roundtrip-Fix steht, Manifestation über `pds3-img-cdn.yml` nach Push.
- `phi/harvest.φ` `format pds4_binary` → auf die gemessene Wahrheit gefaltet: `format pds4_fixed_width_acs` / Tag `archives.esac.esa.int` / `asset present` — die ACS-`data_raw`-Route trägt Table_Character (`pds4_fixed_width_acs_raw_*` present), kein Table_Binary im Baum. Eigener Format-Key (wie `pds4_fixed_width_akatsuki`), `harvest_reg --check` 39/39 in order. Die Binärarm-Fähigkeit bleibt im Code.

## An mycelium

Origin: mountain folge218 (pds3_img-Roundtrip-Fix + pds4_binary-Riss).

- **pds3-img-cdn re-dispatch:** der Compiler-Roundtrip ist geheilt (`BAND_NAME_BYTES = 64`); nach Mountain-Commit+Push `gh workflow run pds3-img-cdn.yml`. Der Lauf schreibt mit dem Fix mindestens das `pds-geosciences.wustl.edu`-Asset (`written ≥ 1` → exit 0 auch bei M3-403); Mountain misst danach `phi/harvest.φ:235` gegen `asset present`.
- **Ephemeriden-Re-Manifest:** der Solver-Fix (`946c7b232`) ist auf `main`; dispatch die Re-Manifest-Workflows (`de44-cdn`, `inpop-epm-cdn`, `kernel-flatten` + Missions-Compiler), dann zieht Mountain die Bins-abhängigen Messungen nach. (Stand gemessen: letzter `de44-cdn`-Erfolg 2026-09-27, `36315903238` — vor dem Fix.)
- **pds4_binary gefaltet (nicht mehr offen):** der Eintrag stand auf `format pds4_binary`/`asset fehlt`, die ESA-ACS-`data_raw`-Route trägt aber Table_Character; Mountain hat ihn auf `format pds4_fixed_width_acs`/`asset present` (Tag `archives.esac.esa.int`, eigener Key wie `pds4_fixed_width_akatsuki`) gefaltet. Die Binärarm-Fähigkeit bleibt im Code; ein echtes Table_Binary-Ziel ist am ESA-PDS nicht gemessen (ACS hat nur `data_raw`, kein `data_derived`/`data_calibrated`). Falls du eines findest: `args --label` + Idempotenz `pds4-binary-cdn.yml:25` auf `^pds4_binary_` eingrenzen.
- **Kaguya-Idempotence-Audit — gebaut, nicht nur gemessen (2026-10-01):** `force`-Input fehlte genau in `pds4-binary-cdn.yml` (Idempotenz auf `^pds4_(binary|fixed_width)_` → übersprang dauerhaft). **Geschlossen:** `force`-Input + Escape ergänzt (spiegelt `pds3-binary-cdn.yml`). Die übrigen: `pds3-img-cdn.yml` hat Idempotenz ohne `force`, verlangt aber **beide** Familien (WUSTL+M3) → überspringt nie (kein Blocker); `de44-cdn.yml`/`kernel-flatten.yml` tragen **keine** Idempotenz → laufen immer; `maven-tnf-cdn.yml`/`physionet-cdn.yml`/`harvest*.yml` tragen `force` bereits.

- **Zeugen-Sweep-Faltung (Mapping v3) — erledigt:** die 6 `decline`-Event-Feeds (`geonet-M3` `declined_sources.φ:530`, `seismicportal minmag 3/5` `:5013/:5017`, `tmd.go.th` `:1569`, `USGS-Detail` `:1573`, `tsunami.incois` `:4177`) tragen jetzt `point-event` in der note (≤256, `register_sort` ohne neue Verletzung). Die 3 live-Quellen (`Chile`/`tohoku`/`jma`) brauchen die `witness point-event`-Admission in `phi/witnesses.φ` — `record`-Magic für Erdbeben noch unbenannt (offener Punkt). Dein `## An mountain` ist damit gefaltet; du kannst den Block entfernen.
- **`register_sort` declined_sources.φ:** 2 vorbestehende url-order-Verletzungen (`data-api.globalforestwatch.org`, `api.purpleair.com/v1/sensors`) — nicht aus meinen Edits; `--write` nicht ausgeführt (großer Reorder).
- **Owner-Drift in mycelium-folge217:** `### pds3_img-Manifestation — Bindung: linie:mountain` verletzt die No-referral-Regel (eine lebende Übergabe trägt nur `eigen`). Der Punkt steht als `eigen` in meiner Übergabe (Register-Träger); bitte deinen `linie:mountain`-Punkt entfernen.

## An river

Origin: mountain folge218 (Solver-Fix + Re-Manifest; Eclipse-Paper ist deine Feder).

- **Finsternis-Haus-Cross-Check ist vom Solver-Bug belastet:** `docs/paper/eclipse-clock-worldlines.md:45` meldet DE442/DE441↔INPOP19a Erde 0.064 km / Sonne 0.164 km / Mond 0.168 km — dieselbe Größenordnung wie der ~40-m-Fit-Fehler (geheilt, aber die CDN-Bins sind noch alt). Bis zum Re-Manifest sind diese Zahlen `pending`, kein Haus-Verdikt.

- **`tap_compiler`-Quoting-Bug:** der Fix liegt bereits als fremder uncommitteter Hunk in `tools/harvest/src/bin/tap_compiler.rs` (`alias_col`/`xq`) — Mountain hat ihn nicht angetastet (fremdes Werk). Nach dem Land: `gaia-cdn` re-dispatch + σ-Zensus.

## An future

Origin: mountain folge218 (Wort-Faltung, Operator 2026-10-01).

- **Das private Experiment — LOCK privat (Operator-Wort 2026-10-01):** „alles was das experiment betrifft bleibt privat". Kein CDN, keine `sources.φ`, keine `witnesses.φ`-Adresse, **kein getrackter Baum**; Daten, Ableitungen (TE-Läufe) und experiment-spezifischer Code bleiben in `state/mountain/kuprat-complex-te/`. Die Zeugen-Klasse steht allein im Code (`LABR`/Substance).
- **History-Akt angefordert (Operator-Wort 2026-10-01):** der private Reader/Compiler und die Experiment-Referenzen in Handover/Rätzel-Survey sollen per History-Akt zurückgezogen werden. Eigentümer: `docs/auftrag/auftrag-pii-history-rewrite.md` (Future/Operator). In der Mountain-Session **strukturell blockiert** (`git push --force` denied, `git filter-repo` fehlt/Python verboten, geteilter Baum); Prozedur privat in `state/mountain/kuprat-complex-te/`.

## Burn: open 0.0075 · close 0.2798 · eigene Session 0.0665 · cap 0.50 · Grund: pds3_img-Roundtrip-Diagnose (CI-Log + gezielter Compiler-Lauf), Bandname-Längenmessung, Code-Fix + Test, Handover-Faltung. Kein pro/max-Dispatch — flash.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der session-weite Consent (Delegation), nie das Commit-Wort.
