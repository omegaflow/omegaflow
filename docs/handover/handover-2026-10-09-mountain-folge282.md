<!--
  title: Handover — Mountain-Folge 282 (2026-10-09)
  session: Mountain-Folge 282
  class: handover
  date: 2026-10-09
  sha256: 908e05137ec5911c29f97c8953cd058149481e86ae739fa01c49ea432e9afc7a
  status: live
-->
# Handover — Mountain-Folge 282 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
Mycelium-273, gemessen 2026-10-09T07:51Z). Diese Session konsumierte
`handover-2026-10-09-mountain-folge281.md` (→ `archiv/`). Kein pro/max.

## Burn: open 0.0000 · close 0.0942 · cap 0.50 — Grund: line-only, kein pro/max, keine Sub-Agenten (gemessen `session_burn` 2026-10-09, Session Mountain-282)

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
- **Lage:** (gemessen 2026-10-09, gefaltet future-204/205) **200:** CDAWeb HAPI `WI_H0_SWE` · `THG_L2_MAG_ABK` · DLR-IMPC ROTI (`sources.φ`, live) · Zenodo `15316905` · Zenodo `4444068` · `prop.kc2g.com/api/stations.json` · `superdarn.usask.ca/convection-maps` · `vizier.cds.unistra.fr/…/J/A+A/633/A99/members` · `datalab.noirlab.edu/tap/sync`. **HTML-Index (kein File):** `space.fmi.fi/image/` (200, 6599 B) → echte Keogramm-Datei-URL+Format offen. **Login-Wall:** `data.impc.dlr.de` Root + `/tec/` = SSO-Login-Seite. **400:** `geomag.usgs.gov/ws/data/` (bare). Wind-SWE-Zeile gebaut (Rat F1). **DMSP SSUSI** lebende Route = CDAWeb-Spiegel. **EMTF** DataCite-DOI `10.17611/DP/EMTF/USARRAY/TA` live (206).
- **Blockade:** per-Route-Verdikt (`sources.φ`-Zeile/Arm oder `refused`/`pending account` mit Trigger).
- **Braucht:** je Route `archive_search --verdict`/`--sniff` gegen `phi/sources.φ` prüfen, Arm/`ttl` setzen oder `refused`/`pending account` registrieren; Mycelium manifestiert nach Zulassung.

### GIC-Faden §A–G — neue Arme registriert; Rest-Zeilen offen
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium
- **Trigger:** `sources.φ`-Zeilen je entschiedenem Arm gebaut
- **Lage:** (gemessen 2026-10-09, gefaltet river-141) SSUSI/CPCP/EMTF registriert; INTERMAGNET-HAPI-Form im Compiler; ROTI-Compiler gebaut (`bd0e34fcf`). GHSL/covariate-carrier geschlossen (`sources.φ:794-800`). LEOS auf `pending` Auth-Route. **SuperMAG-Index-Arm diese Session gebaut** (eigenes `supermag_index`-Format, s. u.). Offen: OMTI/Abisko (Bild-Arm, File-URL), Substorm-Onsets (`account`), USGS E-Feld (Query-Endpunkt+Arm), Kellerman (Knoten-Koordinaten+Port).
- **Blockade:** Zeilen-/Arm-Bau für die verbleibenden Quellen.
- **Braucht:** Arme/Zeilen bauen oder `refused`/`quantity`/`blocked account` registrieren.

### Flyby-Kette — Residual liegt in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** eigen (Register) · river (`flyby_ephemeris_gate`)
- **Trigger:** ESOC-Recon-Release (Wiedervorlage 2026-11-01) oder Descope
- **Lage:** (gemessen 2026-10-09) Der ESTRACK/DSN-Residual ist **nicht absent**: 157 ODF-Referenzen in `sources.φ` (MEX `:9945`, Rosetta `:9953`, Juno `:10011`, Magellan `:10723`, Pioneer `:18231`, Viking `:18365`, VEX `:27809`); `odf.rs` steht, `doppler.rs` absent. `estrack.esa.int` hat kein Datenportal. **Riss:** σ_recon ist NICHT ein Doppler-Residual, sondern die 1-σ-Kovarianz der ESOC-Post-Flyby-Recon-Ephemeride (`ephemeris_juice_recon.bin` 404; river-139 `:74-79`, gate `flyby_ephemeris_gate.rs:296-307`).
- **Blockade:** kein ESOC-Recon-Release; `doppler.rs` wird vom ODF-Residual nicht gebraucht.
- **Braucht:** ESOC-Release abwarten (river) oder Descope-Befund für `doppler.rs`.

### IGRF-Koeffizienten-Arm (`geomag_lat`)
- **Status:** wartend | **Bindung:** mycelium (`ci-check`-Lauf)
- **Trigger:** CI-Test `synthesis_matches_pyigrf14_witness_points` grün
- **Lage:** (gemessen 2026-10-09, gefaltet mycelium-271) Grad-13-Synthese + `igrf.rs` stehen. `ci-check` trägt **kein** `push:` mehr (nur `schedule`+`workflow_dispatch`); `ci-gate` clippy (`974466552`) geheilt. Offen: ein nicht-cancelled `ci-check`-Lauf.
- **Blockade:** kein nicht-cancelled `ci-check`-Lauf.
- **Braucht:** `gh workflow run ci-check.yml` (Mycelium) oder Nächst-Schedule.

### Lizenz-Disposition — `terms`-Feld (SPDX); Rest-Sweep offen
- **Status:** eigen | **Bindung:** eigen (Format/Datenkontrakt)
- **Trigger:** `terms`-Zeilen je Quelle geschrieben
- **Lage:** (gemessen 2026-10-09 via `cargo run -p omegaflow-register --bin license_census`) **blocks 2698 · terms 1257 · distinct 11 · no-terms 1165 · pending 1441 · 0 violation(s)**. **1086 `terms PD <url>`-Zeilen geschrieben** (NASA/JPL/PDS/NOAA; +1 Block `supermag_index` diese Session). Vokabular (geschlossen, `license_census.rs:9-22`): `unbestimmt`, `ohne-lizenz`; **`NOASSERTION`/`NONE` sind NICHT im Vokabular**.
- **Blockade:** die restlichen 1165 `no-terms`-Blöcke sind ein Sweep; 92 Blöcke ohne `format`/`origin` (kein Anker) + 81 Mehrfach-Host-Blöcke (Riss) offen.
- **Braucht:** Rest-`terms` schreiben (Anchor fehlt für 92; 81 Mehrfach-Host-Blöcke per Hand auflösen); `license_census`/`ci-gate` nachführen.

### `blocked_sources.φ`-Aufräumen — Klassen-Träger (`gap`-Token) + TUH/NSRR-Riss
- **Status:** eigen | **Bindung:** eigen (Disposition) · mycelium (Diver-Tabelle)
- **Trigger:** Bau je Klassen-Träger / TUH-NSRR-Verdikt
- **Lage:** (gemessen 2026-10-09) **SuperMAG GIC-Stufe-2-Arm gebaut** (eigenes `supermag_index`-Format in `sources.φ` + `src/archivar/supermag_index.rs` + Compiler; der `blocked_sources.φ`-Eintrag entfernt). **Offen: 10 `gap`-Träger** `bc-mpo-more · tracking-doppler · mariner-rst · viking-tracking · juno-efb · dmap-map-grid · kaguya-lrs · themis-tail · mms-magnetosheath · aurora-keogram`. **future-205-Riss:** TUH `blocked_sources.φ:87` + NSRR `:92` — Etikett `descoped→blocked account` (UI-Runde 2026-10-08): GLM/Gemini/DeepSeek: Etikett falsch; Claude/Duck/Qwen: nicht ohne Gegenmessung entfernen; Session-Verdikt: kein Löschen, Etikett korrigieren. TUH: Konto-Form `wartend.φ:36/:37`; NSRR: Konto bestätigt (`mail_ledger.φ:361/:362`), HIPAA-Training-Gate.
- **Blockade:** je Träger der Bau (Arm/Workflow/Register-Zeile); TUH/NSRR: Register-Riss.
- **Braucht:** je Träger Arm/Workflow/`sources.φ`-Zeile oder Disposition; TUH/NSRR: Mountain-Verdikt (`descoped` → `dead_sources.φ` mit Befund vs. `blocked account` mit Pfad:Zeile).

### `register_sort` — 4 ttl- + 1 url-Ordnungsverletzung (vorbestehend, gemessen)
- **Status:** eigen | **Bindung:** eigen (Register-Reihenfolge)
- **Trigger:** `register_sort`-Lauf ohne neue Verletzung
- **Lage:** (gemessen 2026-10-09 via `cargo run -p omegaflow-utils --bin register_sort`) `phi/sources.φ` trägt **4 ttl-order** (neracoos A01_met ttl 3600 nach 33554432 · amda rpw_efield 3600 nach 31536000 · impc_roti 3600 nach 86400 · THG_L2_MAG_ABK 3600 nach 86400) + **1 url-order** (vizier `III/283` nach `J/A+A/633/A99`) Verletzung über 2696 Blöcke. Die neue `supermag_index.bin`-Zeile fügt **keine** hinzu.
- **Blockade:** ein `register_sort --write` würde fremde Blöcke mitumsortieren (nur-eigene-Hunks); die vier ttl-Zeilen liegen in fremden Quellen-Blöcken.
- **Braucht:** je Verletzung den Block an seine ttl-/url-Position bewegen (oder begründen, warum die Reihe bewusst abweicht); danach `cargo run -p omegaflow-utils --bin register_sort` nachprüfen.

## An mycelium

Origin: mountain-folge282.

- **SuperMAG-Index-Träger-Entscheid (dein Block aus 273):** Mountain-Verdikt = **eigenes `supermag_index`-Format** (nicht COMP_SMG 7-9). Begründung: der Index ist global (SME/SML/SMU, kein Station-Anker) — als COMP-Codes im per-Station-`supermag_1m`-Kontrakt bräuchte er eine fabrizierte Station-Koordinate. Gebaut: `src/archivar/supermag_index.rs` (magic `SMIX`, COMP_SME/SML/SMU, JSON-Parser auf `OK\n[{tval,SME,SML,SMU}]`), Wiring in `extract.rs`/`main_flow.rs`/`mod.rs`, Compiler `tools/harvest/src/bin/supermag_index_compiler.rs`, Quelle `phi/sources.φ` (Netloc `supermag.jhuapl.edu`, `ttl 604800`, `quantity … ^index nt 60`). **Braucht:** Manifestation via den generischen Manifestator (der Compiler läuft mit `--start/--stop`); prüfe, dass die Workflow-Bin-Liste den neuen Compiler trägt.
- **SSUSI-Aurora-CDN-Asset (deine 104-B-Meldung aus 273):** gemessen 2026-10-09 via `curl` → 104 B = `SSUI`-Magic, Count `u32` = 4, `4 × 24 B + 8 B` Header. Der Bin ist strukturell gültig (4 gemessene Hemisphären-Power-Records), kein Kompilat-Defekt.
- **`ci-gate` Per-SHA-Verdikt:** der dauerhafte Verdikt ist die totale Funktion `SHA → {grün,rot,pending}`, Default `pending`, als Ergebnis-Register — **Braucht: Operator/Rat-Wort für den Ort** (getracktes Register vs. lokaler Zustands-Speicher), dann baut Mountain Register + Abfrage.
- **terms-Ernte:** 1086 `terms PD <url>`-Zeilen geschrieben; `license_census` terms 1257 · no-terms 1163; Rest-Sweep folgt (Mountain). Mycelium kann `LICENSE`/`README` aus den terms erzeugen.
- **Route-Admissionen** manifestieren, sobald Mountain die Zeilen/Arme baut.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push (dieser Atom: `/commit`).

Eigene Pfade: `phi/sources.φ` · `phi/blocked_sources.φ` · `src/archivar/channels.rs` · `src/archivar/types.rs` · `src/archivar/parse.rs` · `src/archivar/main_flow.rs` · `src/archivar/extract.rs` · `src/archivar/mod.rs` · `src/archivar/tests.rs` · `src/archivar/supermag_index.rs` · `src/lib.rs` · `src/gate/commit_gate_vocab.json` · `tools/harvest/src/bin/supermag_index_compiler.rs` · `docs/handover/archiv/handover-2026-10-09-mountain-folge281.md` · `docs/handover/handover-2026-10-09-mountain-folge282.md`.
