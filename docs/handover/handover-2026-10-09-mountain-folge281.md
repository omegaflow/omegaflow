<!--
  title: Handover — Mountain-Folge 281 (2026-10-09)
  session: Mountain-Folge 281
  class: handover
  date: 2026-10-09
  sha256: a177398667d5bba7ed497b53a749d4a9f14daadef5d950501e2bc776c4413278
  status: live
-->
# Handover — Mountain-Folge 281 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
Mycelium-272, gemessen 2026-10-09T06:18Z). Diese Session konsumierte
`handover-2026-10-09-mountain-folge280.md` (→ `archiv/`). Kein pro/max.

## Burn: open 0.0000 · close 0.0620 · cap 0.50 — Grund: line-only, kein pro/max, keine Sub-Agenten (gemessen `session_burn` 2026-10-09, Session Mountain-281)

## Operator-Wort-Register

Wort | Datum | Quelle
--- | --- | ---
„Starte die Mountain-Linie in einem Pass — kein Planungstheater, keine Tafel, kein Consent-Stopp für Bekanntes" | 2026-10-07 | Operator (Session, Mountain 251–279)
„Architektur-/Ethik-Entscheidungen gehen durch die Linse der fünf Stimmen (Rat), nie in Pro-Solo" | 2026-10-07 | Operator (Session, Mountain 251–279)
„mach das ab jetzt automatisch — committe und pushe selbst, du bist die einzige Linie die das nicht automatisch tut" | 2026-10-07 | Operator (Session, Mountain 264)
„ja bitte" — Index-Riss als Mountain-Verdikt `quantity` setzen + die `sources.φ`-Zeilen bauen | 2026-10-08 | Operator (Session, Mountain 276)
„ich glaube du musst nochmal breiter fragen" — Science-Layer + starke Frontier-Seats für die Route-Admission | 2026-10-08 | Operator (Session, Mountain 276)
„bitte umsetzen Offen (im Report benannt): 2 blocked_sources-Risse (limadou/vco_rs Dubletten; cluster_ka-Zeile ohne gap), SuperDARN dritter Layout-Slot (kein pot.drop.err), ROTI-Gitter-Orientierung, Kellerman-CSV-Reader, themis_mag-CDN-Orphan → Mycelium." | 2026-10-09 | Operator (Session, Mountain 280)
„Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent (auto-bestätigt). Delegiere an die Taucher, höre die Stimmen bei Architektur-/Abschluss-Entscheidungen. Dies ist der session-weite Consent (Delegation), nicht das Commit-Wort — Commit und Push trägt `/commit`." + „aber mach dann auch wirklich die Arbeit" | 2026-10-09 | Operator (Session, Mountain 281)

## Offen (aufgeschlüsselt)

### Route-Admissionen — Liveness gemessen; Wind-SWE gebaut; IMPC = Login-Wall
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium (Manifestation)
- **Trigger:** je Route der Arm/`refused`-Befund
- **Lage:** (gemessen 2026-10-09, gefaltet future-204) **200:** CDAWeb HAPI `WI_H0_SWE` · `THG_L2_MAG_ABK` · DLR-IMPC ROTI (`sources.φ:1629-1635`, live) · Zenodo `15316905` · Zenodo `4444068` · `prop.kc2g.com/api/stations.json` · `superdarn.usask.ca/convection-maps` · `vizier.cds.unistra.fr/…/J/A+A/633/A99/members` · `datalab.noirlab.edu/tap/sync`. **HTML-Index (kein File):** `space.fmi.fi/image/` (200, 6599 B) → echte Keogramm-Datei-URL+Format offen. **Login-Wall:** `data.impc.dlr.de` Root + `/tec/` = SSO-Login-Seite. **400:** `geomag.usgs.gov/ws/data/` (bare). Wind-SWE-Zeile gebaut (Rat F1). **DMSP SSUSI** lebende Route = CDAWeb-Spiegel. **EMTF** DataCite-DOI `10.17611/DP/EMTF/USARRAY/TA` live (206).
- **Blockade:** per-Route-Verdikt (`sources.φ`-Zeile/Arm oder `refused`/`pending account` mit Trigger).
- **Braucht:** je Route `archive_search --verdict`/`--sniff` gegen `phi/sources.φ` prüfen, Arm/`ttl` setzen oder `refused`/`pending account` registrieren; Mycelium manifestiert nach Zulassung.

### GIC-Faden §A–G — neue Arme registriert; Zeilen-Bau offen
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium
- **Trigger:** `sources.φ`-Zeilen je entschiedenem Arm gebaut
- **Lage:** (gemessen 2026-10-09, gefaltet river-139) SSUSI/CPCP/EMTF registriert; INTERMAGNET-HAPI-Form im Compiler; **ROTI-Compiler gebaut** (`bd0e34fcf`). GHSL/covariate-carrier geschlossen (Rat A: `quantity ghsl_built_surface_m2 … point area m2`, `sources.φ:794-800`). LEOS auf `pending` Auth-Route. Offen: OMTI/Abisko (Bild-Arm, File-URL), Substorm-Onsets (`account`), USGS E-Feld (Query-Endpunkt+Arm), Kellerman (Knoten-Koordinaten+Port).
- **Blockade:** Zeilen-/Arm-Bau für die verbleibenden Quellen.
- **Braucht:** Arme/Zeilen bauen oder `refused`/`quantity`/`blocked account` registrieren.

### GIC-Stufe-2 — SuperMAG-Konto gemessen; SME/SMU/SML-Zeilen offen
- **Status:** eigen | **Bindung:** eigen (Register)
- **Trigger:** SME/SMU/SML als `sources.φ`-Zeilen gebaut
- **Lage:** (gemessen 2026-10-09) AE/AL/AU stehen (`sources.φ:4087-4092`); der positionale Geo-Serien-Arm nutzt den Receiver-Reader (`main_flow.rs:4518-4527`). **SuperMAG-Konto/Login steht; Station-Route live gemessen** (`archive_search --supermag 'station=ABK start=202401010000 end=202401010100'` → 10 Zeilen, kein `pending`). `blocked_sources.φ:117` von `blocked account` → `pending` umgetaggt (Mountain-Verdikt dieser Session); offen nur der SME/SMU/SML-Arm + `quantity`-Zeile(n), abgeleiteter Index nie `em`.
- **Blockade:** keiner (Konto steht); Arm-Bau.
- **Braucht:** SME/SMU/SML als `quantity`-Zeilen + Arm bauen.

### Flyby-Kette — Residual liegt in ODF; σ_recon getrennt
- **Status:** eigen | **Bindung:** eigen (Register) · river (`flyby_ephemeris_gate`)
- **Trigger:** ESOC-Recon-Release (Wiedervorlage 2026-11-01) oder Descope
- **Lage:** (gemessen 2026-10-09) Der ESTRACK/DSN-Residual ist **nicht absent**: 157 ODF-Referenzen in `sources.φ` (MEX `:9945`, Rosetta `:9953`, Juno `:10011`, Magellan `:10723`, Pioneer `:18231`, Viking `:18365`, VEX `:27809`); `odf.rs` steht, `doppler.rs` absent. `estrack.esa.int` hat kein Datenportal. **Riss:** σ_recon ist NICHT ein Doppler-Residual, sondern die 1-σ-Kovarianz der ESOC-Post-Flyby-Recon-Ephemeride (`ephemeris_juice_recon.bin` 404; river-139 `:74-79`, gate `flyby_ephemeris_gate.rs:296-307`).
- **Blockade:** kein ESOC-Recon-Release; `doppler.rs` wird vom ODF-Residual nicht gebraucht.
- **Braucht:** ESOC-Release abwarten (river) oder Descope-Befund für `doppler.rs`.

### IGRF-Koeffizienten-Arm (`geomag_lat`)
- **Status:** eigen | **Bindung:** mycelium (`ci-check`-Lauf)
- **Trigger:** CI-Test `synthesis_matches_pyigrf14_witness_points` grün
- **Lage:** (gemessen 2026-10-09, gefaltet mycelium-271) Grad-13-Synthese + `igrf.rs` stehen. `ci-check` trägt **kein** `push:` mehr (nur `schedule`+`workflow_dispatch`) — die Verdrängung ist geheilt. `ci-gate` clippy (`974466552`) geheilt. Offen: ein nicht-cancelled `ci-check`-Lauf.
- **Blockade:** kein nicht-cancelled `ci-check`-Lauf.
- **Braucht:** `gh workflow run ci-check.yml` (Mycelium) oder Nächst-Schedule.

### Lizenz-Disposition — `terms`-Feld (SPDX); Rest-Sweep offen
- **Status:** eigen | **Bindung:** eigen (Format/Datenkontrakt)
- **Trigger:** `terms`-Zeilen je Quelle geschrieben
- **Lage:** (gemessen 2026-10-09 via `license_census`) **blocks 2697 · terms 1257 · distinct 11 · no-terms 1164 · pending 1440 · 0 violation(s)**. **1086 `terms PD <url>`-Zeilen geschrieben** (NASA/JPL/PDS/NOAA). Vokabular (geschlossen, `license_census.rs:9-22`): `unbestimmt`, `ohne-lizenz`; **`NOASSERTION`/`NONE` sind NICHT im Vokabular**.
- **Blockade:** die restlichen 1164 `no-terms`-Blöcke sind ein Sweep; 92 Blöcke ohne `format`/`origin` (kein Anker) + 81 Mehrfach-Host-Blöcke (Riss) offen.
- **Braucht:** Rest-`terms` schreiben (Anchor fehlt für 92; 81 Mehrfach-Host-Blöcke per Hand auflösen); `license_census`/`ci-gate` nachführen.

### Bias-Tor (`docs/auftrag/auftrag-bias-tilgung.md`) — 3 von 7 geheilt
- **Status:** eigen | **Bindung:** eigen (Gate/Fixture)
- **Trigger:** je Restkandidat Rat-Wort (Hart-Block vs. Review)
- **Lage:** (gemessen 2026-10-09) **geheilt (diese Session):** `ck.rs:164` (Modulus-Default 1.0 → Block ohne `SCLK01_MODULI_` verworfen, `filter_map`), `gate/axioms.rs:6` (`unwrap_or_default` → `load → Option`, fehlende `granit.md` = keine System-Message statt leerem Axiom), `double.rs:167/170/173` (NaN-Datenmarker → `Option<f64>`; Konsument `infrared_anomaly_compiler.rs` druckt `absent`; Tests auf `Some`/`None`). **Zwei Fixtures ergänzt** (`commit_gate_vocab.json`): SCLK-`unwrap_or(1.0)`-Muster + `unwrap_or(f64::NAN)`. `cargo check` 0/0; `cargo build -p omegaflow-harvest --bin infrared_anomaly_compiler` grün. **Offen:** `channels.rs:119` (Receiver-Presence absent → Anker substituiert); `fits.rs:796-799` (CRPIx/R_SUN/DATAMEAN → NaN); `hdf5.rs:1965` (`scale_type` 2 default); `weberin.rs:1877-1879` (Winkel-Separationen → NaN). FITS-`BSCALE`/`TSCAL`-Default 1.0 ist FITS-Standardwert (kein Fix).
- **Blockade:** Klassifikation Hart-Block (Fixture) vs. Review je Restkandidat = Rat.
- **Braucht:** Rat-Wort je Restkandidat, dann Fix (Option statt NaN) oder Fixture + Gate-Test.

### HadISST SST — SOURCE_PORT gebaut, CI-Lauf offen
- **Status:** eigen | **Bindung:** mycelium (CDN)
- **Trigger:** `hadisst-cdn.yml`-Lauf grün
- **Lage:** (gemessen 2026-10-09, gefaltet mycelium-272) Compiler/Arm/Register/Workflow stehen; Compiler-Dim-Toleranz geheilt (`latitude`/`longitude` CF-Namen); Neu-Dispatch `hadisst-cdn.yml` (`cc9f4cf7c`) in Flug.
- **Blockade:** Lauf noch nicht grün.
- **Braucht:** `ci_manage view` des `hadisst-cdn`-Laufs; bei Erfolg `sha256`-Zeile nachziehen.

### `blocked_sources.φ`-Aufräumen — Klassen-Träger (`gap`-Token) + future-204
- **Status:** eigen | **Bindung:** eigen (Disposition) · mycelium (Diver-Tabelle)
- **Trigger:** Bau je Klassen-Träger / future-204-Verdikt
- **Lage:** (gemessen 2026-10-09) Klassen-Vollstreckung (`bd0e34fcf`) + Risse geschlossen: `limadou`/`vco_rs`/`cluster_ka` entfernt; `covariate-carrier` gebaut (GHSL `sources.φ:794-800`); LEOS auf `pending`; `openmadrigal-api` gebaut (`src/archivar/openmadrigal.rs`, `sources.φ:27808-27813`). **SuperMAG `:117` umgetaggt `blocked account` → `pending`** (Konto + Station-Route gemessen). **Offen: 10 `gap`-Träger** `bc-mpo-more · tracking-doppler · mariner-rst · viking-tracking · juno-efb · dmap-map-grid · kaguya-lrs · themis-tail · mms-magnetosheath · aurora-keogram`. **future-204:** TUH `:86` + NSRR `:90` Etikett-Verdikt offen — UI-Runde 2026-10-08 ist ein Riss (GLM/Gemini/DeepSeek: Etikett falsch; Claude/Duck/Qwen: nicht ohne Gegenmessung entfernen; Session-Verdikt: kein Löschen, Etikett korrigieren).
- **Blockade:** je Träger der Bau (Arm/Workflow/Register-Zeile); TUH/NSRR: Register-Riss.
- **Braucht:** je Träger Arm/Workflow/`sources.φ`-Zeile oder Disposition; TUH/NSRR: Mountain-Verdikt (`descoped` → `dead_sources.φ` mit Befund vs. `blocked account` mit Pfad:Zeile).

## An future

Origin: mountain-folge281.

- **future-204 (drei owner-future-Einträge):** SuperMAG `phi/blocked_sources.φ:117` **Riss aufgelöst** — Konto/Login steht, Station-Route live gemessen (`archive_search --supermag 'station=ABK …'` → 10 Zeilen); Etikett `blocked account` → `pending` umgetaggt, offen nur der SME/SMU/SML-Arm (Mountain, kein Operator-Akt). **Noch kein Verdikt:** TUH `:86` und NSRR `:90` — das Etikett hängt am Register-Riss der UI-Runde 2026-10-08 (entfernen vs. umtaggen); der Session-Verdikt „kein Löschen, Etikett korrigieren" verlangt entschiedenen Riss. Das Mountain-Verdikt folgt als eigene Register-Zeile, sobald der Riss geschlossen ist.

## An mycelium

Origin: mountain-folge281.

- **`ci-gate` Per-SHA-Verdikt:** der dauerhafte Verdikt ist die totale Funktion `SHA → {grün,rot,pending}`, Default `pending`, als Ergebnis-Register — **Braucht: Operator/Rat-Wort für den Ort** (getracktes Register vs. lokaler Zustands-Speicher), dann baut Mountain Register + Abfrage.
- **terms-Ernte:** 1086 `terms PD <url>`-Zeilen geschrieben; `license_census` terms 1257 · no-terms 1164; Rest-Sweep folgt (Mountain). Mycelium kann `LICENSE`/`README` aus den terms erzeugen.
- **Route-Admissionen** manifestieren, sobald Mountain die Zeilen/Arme baut.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push (dieser Atom: `/commit`).

Eigene Pfade: `src/archivar/ck.rs` · `src/archivar/double.rs` · `src/gate/axioms.rs` · `src/gate/commit_gate_vocab.json` · `tools/harvest/src/bin/infrared_anomaly_compiler.rs` · `phi/blocked_sources.φ` · `docs/handover/archiv/handover-2026-10-09-mountain-folge280.md` · `docs/handover/handover-2026-10-09-mountain-folge281.md`.
