<!--
  title: Handover — Mountain-Folge 282 (2026-10-09)
  session: Mountain-Folge 282
  class: handover
  date: 2026-10-09
  sha256: b2375fa23f071b5382556e87ebc11587c38c51a962bc90667180b25cefd373a0
  status: live
-->
# Handover — Mountain-Folge 282 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
Mycelium-273, gemessen 2026-10-09T07:51Z). Diese Session konsumierte
`handover-2026-10-09-mountain-folge281.md` (→ `archiv/`). Kein pro/max.

## Burn: open 0.0000 · close 0.1897 · cap 0.50 — Grund: line + Taucher (4 `general`-Recherche, 2 `grind-flash`), kein pro/max (gemessen `session_burn` 2026-10-09, Session Mountain-282)

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

### Route-Admissionen — alle lebenden Routen registriert; zwei Landing-/Fehlrouten eingetragen
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium (Manifestation)
- **Trigger:** je Route der Arm/`refused`-Befund
- **Lage:** (gemessen 2026-10-09, gefaltet future-204/205 + Taucher-Lauf) **Alle 11 geprüften Routen sind gemessen und registriert:** CDAWeb HAPI `WI_H0_SWE` (`sources.φ:694`), `THG_L2_MAG_ABK` (`:4113`), DLR-IMPC ROTI (Auth-Route `:1652`, Arm steht), Zenodo `15316905` (**declined** `declined_sources.φ:5683`, kein Kraft-Kanal), Zenodo `4444068` (Eintrag → `blocked parser-def gap csv-gzip`), `prop.kc2g.com` (`:1003`), `vizier…J/A+A/633/A99` (`:19998`), `datalab.noirlab.edu` (`:20192`), EMTF (`:10221`, Arm steht). **Neu eingetragen:** `superdarn.usask.ca/convection-maps` = HTML-Landing ohne Daten-Endpoint (Konvektion via `superdarn_cpcp`) → `pending`; USGS E-Feld → NOAA SWPC rgeojson (`blocked parser-def gap geojson-efield`), USGS-`openapi.json` trägt kein `geoelectric`. Wind-SWE gebaut. **DMSP SSUSI** = CDAWeb-Spiegel.
- **Blockade:** keiner für die registrierten Routen.
- **Braucht:** nichts Neues; Mycelium manifestiert nach Zulassung.

### GIC-Faden §A–G — neue Arme registriert; Rest-Zeilen offen
- **Status:** eigen | **Bindung:** eigen (Register) · mycelium
- **Trigger:** `sources.φ`-Zeilen je entschiedenem Arm gebaut
- **Lage:** (gemessen 2026-10-09, gefaltet river-141 + Taucher-Lauf) SSUSI/CPCP/EMTF registriert; ROTI-Compiler gebaut (`bd0e34fcf`); GHSL/covariate-carrier geschlossen; **SuperMAG-Index-Arm gebaut**. **OMTI/Abisko:** Riss geschlossen — `space.fmi.fi/image/` = IMAGE-Magnetometer, die Keogramme liegen unter `space.fmi.fi/MIRACLE/ASC/ASC_keograms/…` (ABK `206 image/jpeg`); offen bleibt der **Bild-Arm/Vision**. **Kellerman:** lat/lon/Port per Design absent → `descoped`; **`csv_gz`-Arm gebaut** (diese Session); offen nur die 7-Node-CSV-Registrierung. **USGS E-Feld:** NOAA SWPC rgeojson — **Reader + Compiler + CDN-Workflow `swpc-efield-cdn.yml` + `sources.φ`-Zeile gebaut** (`SWEF`-Bin, `--start/--stop`). **DMap/map-grid:** Kern + `map_grid_value` gebaut; Quelle wartet Globus. **Keogramm:** Dekoder `keogram.rs` + Compiler `keogram_compiler.rs` gebaut; **Wire-Feld descoped** (raw/relativ, FMI „not suitable"), offen: Quelle als Vision-Asset. **Substorm-Onsets:** SuperMAG-Service offen — **Modul `substorm.rs` + Compiler `substorm_compiler.rs` + `sources.φ`-Zeile gebaut**. **Kellerman:** 7 Node-CSV registriert (`format csv_gz`, `DateTime/GIC/GIC_QDC`, 1 s).
- **Blockade:** map-grid-Quelle (Globus); Keogramm-Vision-Asset.
- **Braucht:** map-grid (Globus-Download mit den **stehenden** Credentials `.secrets.local` `GLOBUS_ID_USER`/`GLOBUS_ID_PASS` — kein Operator-Akt; `wartend.φ:8`, Mycelium); Keogramm als Vision-Asset registrieren.

### Flyby-Kette — Residual liegt in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** eigen (Register) · river (`flyby_ephemeris_gate`)
- **Trigger:** ESOC-Recon-Release (Wiedervorlage 2026-11-01) oder Descope
- **Lage:** (gemessen 2026-10-09) Der ESTRACK/DSN-Residual ist **nicht absent**: 157 ODF-Referenzen in `sources.φ` (MEX `:9945`, Rosetta `:9953`, Juno `:10011`, Magellan `:10723`, Pioneer `:18231`, Viking `:18365`, VEX `:27809`); `odf.rs` steht, `doppler.rs` absent. `estrack.esa.int` hat kein Datenportal. **Riss:** σ_recon ist NICHT ein Doppler-Residual, sondern die 1-σ-Kovarianz der ESOC-Post-Flyby-Recon-Ephemeride (`ephemeris_juice_recon.bin` 404; river-139 `:74-79`, gate `flyby_ephemeris_gate.rs:296-307`).
- **Blockade:** kein ESOC-Recon-Release; `doppler.rs` wird vom ODF-Residual nicht gebraucht.
- **Braucht:** ESOC-Release abwarten (river) oder Descope-Befund für `doppler.rs`.

### IGRF-Koeffizienten-Arm (`geomag_lat`)
- **Status:** wartend | **Bindung:** mycelium (`ci-check`-Lauf)
- **Trigger:** CI-Test `synthesis_matches_pyigrf14_witness_points` grün
- **Lage:** (gemessen 2026-10-09, gefaltet mycelium-271) Grad-13-Synthese + `igrf.rs` stehen. `ci-check` trägt **kein** `push:` mehr (nur `schedule`+`workflow_dispatch`); `ci-gate` clippy (`974466552`) geheilt. **Dispatcher gefeuert (diese Session):** `gh workflow run ci-check.yml` → run `37910517444` (head `ce47ce13c`, `queued`, measured 2026-10-09 via `ci_manage view`).
- **Blockade:** keiner mehr — der Lauf läuft.
- **Braucht:** `ci_manage view 37910517444` beim nächsten Pass (kein Session-Polling); bei grün Punkt schließen.

### Lizenz-Disposition — `terms`-Feld (SPDX); Rest-Sweep offen
- **Status:** eigen | **Bindung:** eigen (Format/Datenkontrakt)
- **Trigger:** `terms`-Zeilen je Quelle geschrieben
- **Lage:** (gemessen 2026-10-09 via `cargo run -p omegaflow-register --bin license_census`) **blocks 2698 · terms 1348 · distinct 11 · no-terms 1101 · pending 1350 · 0 violation(s)**. Diese Session **+91 `terms`-Zeilen** (NASA/IPAC PD: cdaweb 11 · exoplanetarchive 3 · ned 4 · exofop 4 · heasarc 2 · ssd-api 1; ESO CC-BY-4.0 4; CDS/VizieR-Spiegel `unbestimmt` 62). Vokabular (geschlossen, `license_census.rs:9-22`): `PD`, `CC-BY-4.0`, `CC0-1.0`, `unbestimmt`, `ohne-lizenz`; **`NOASSERTION`/`NONE` sind NICHT im Vokabular**.
- **Blockade:** die restlichen `no-terms`-Blöcke sind heterogen; **ESA/ESAC** (archives/gea/isla/eas/…esac.esa.int, psa.esa.int) tragen **CC BY-NC 3.0 IGO** — nicht im geschlossenen Vokabular (Vokabel-Erweiterung nötig); `zenodo.org` per Record (Record-API); `datalab.noirlab.edu`/`dc.g-vo.org` ohne Daten-Lizenzaussage (`pending`); Anker fehlt weiter für Blöcke ohne `format`/`origin`.
- **Braucht:** Vokabel-Erweiterung für `CC-BY-NC-3.0-IGO` (Rat/Wort) ODER ESA `pending` belassen; zenodo-Terms per Record (`archive_search --zenodo <id>`); Rest-Sweep je Anker; `license_census`/`ci-gate` nachführen.

### `blocked_sources.φ`-Aufräumen — Klassen-Träger (`gap`-Token) + TUH/NSRR-Riss
- **Status:** eigen | **Bindung:** eigen (Disposition) · mycelium (Diver-Tabelle)
- **Trigger:** Bau je Klassen-Träger / TUH-NSRR-Verdikt
- **Lage:** (gemessen 2026-10-09 + Taucher-Lauf) **SuperMAG GIC-Stufe-2-Arm gebaut** (Eintrag entfernt). **Kellerman** → `pending` (`csv_gz`-Arm gebaut; lat/lon/Port per Design absent, descoped). **Aurora-Keogramm**: URL auf `space.fmi.fi/MIRACLE/ASC/ASC_keograms/` korrigiert (Riss zum IMAGE-Magnetometer geschlossen), **Bild-Arm gebaut** (`src/archivar/keogram.rs`). **USGS E-Feld** → NOAA SWPC rgeojson: **Reader `swpc_efield` gebaut**, offen Serien-Compiler (`gap efield-series`). **SuperDARN convection-maps** → `pending` (Landing). **DMap/map-grid:** Kern + `map_grid_value` in `src/archivar/dmap.rs`; Globus wartet (`gap dmap-map-grid`). **Verbleibende `gap`-Träger mit gemessenem nächstem Schritt:** `bc-mpo-more` (PSA-Release 2099, wartet `psahelp@`), `tracking-doppler`/`viking-tracking`/`juno-efb` (NSSDC-Antworten, `mail_ledger.φ:207/209/211`, Parser `odf.rs`/`viking_text.rs` stehen), `mariner-rst` (7-Track-Parser fehlt; SDDPT-Messung), `dmap-map-grid` (DMap-Kern aus `superdarn_fitacf_compiler.rs` nach `src/archivar/` heben, neues `dmap`-Modul; Globus-Antwort `wartend.φ:8`), `kaguya-lrs` (WUSTL/DARTS `.lbl`/`.dat` messen → Block+Workflow), `themis-tail` (Playwright: konkrete Datei+Format messen), `mms-magnetosheath` (Arm fehlt). **future-205-Riss:** TUH `blocked_sources.φ:87` + NSRR `:92` — Etikett `descoped→blocked account` (UI-Runde 2026-10-08), Session-Verdikt: kein Löschen, Etikett korrigieren. TUH: Konto-Form `wartend.φ:36/:37`; NSRR: Konto bestätigt (`mail_ledger.φ:361/:362`), HIPAA-Training-Gate. **Mountain-Verdikt (2026-10-09, Register-Disposition):** beide bleiben `blocked account` — Konto real, Daten hinter NEDC/HIPAA; kein Descope. Riss geschlossen.
- **Blockade:** je Träger der Bau (Arm/Workflow/Register-Zeile) oder eine wartende Antwort; TUH/NSRR: Register-Riss.
- **Braucht:** `phi/blocked_sources.φ::gap:aurora-keogram ×1` · `phi/blocked_sources.φ::gap:dmap-map-grid ×1` · `phi/blocked_sources.φ::gap:substorm-list ×1` (Klassen-Träger; `csv_gz`-Arm, `dmap`-Kern + `map_grid_value`, `swpc_efield`-Reader+Compiler gebaut, Kellerman-7-Node registriert) · je Wartendem die Antwort; TUH/NSRR: Mountain-Verdikt (`descoped`→`dead_sources.φ` mit Befund vs. `blocked account` mit Pfad:Zeile).

### `register_sort` — 4 ttl- + 1 url-Ordnungsverletzung (vorbestehend, gemessen)
- **Status:** eigen | **Bindung:** eigen (Register-Reihenfolge)
- **Trigger:** `register_sort`-Lauf ohne neue Verletzung
- **Lage:** (gemessen 2026-10-09 via `cargo run -p omegaflow-utils --bin register_sort`) `phi/sources.φ` trägt **4 ttl-order** (neracoos A01_met ttl 3600 nach 33554432 · amda rpw_efield 3600 nach 31536000 · impc_roti 3600 nach 86400 · THG_L2_MAG_ABK 3600 nach 86400) + **1 url-order** (vizier `III/283` nach `J/A+A/633/A99`) Verletzung über 2698 Blöcke. Die neue `supermag_index.bin`-Zeile fügt **keine** hinzu.
- **Blockade:** ein `register_sort --write` würde fremde Blöcke mitumsortieren (nur-eigene-Hunks); die vier ttl-Zeilen liegen in fremden Quellen-Blöcken.
- **Braucht:** je Verletzung den Block an seine ttl-/url-Position bewegen (oder begründen, warum die Reihe bewusst abweicht); danach `cargo run -p omegaflow-utils --bin register_sort` nachprüfen.

## An mycelium

Origin: mountain-folge282.

- **Gefaltet (deine 274):** SuperMAG-Index-Wiring verifiziert (`phi/harvest.φ:522-526`, harvest `37905571296` success, keine Bin-Liste nötig) · SSUSI 104 B strukturell gültig (4 Records) · Per-SHA-Verdikt gewortet (`668c8ada4`/`2f93f37ac`) — alle drei geschlossen, kein neuer Akt nötig.
- **terms-Ernte:** diese Session **+91** `terms`-Zeilen (NASA/IPAC PD, ESO CC-BY-4.0, CDS/VizieR `unbestimmt`); `license_census` **terms 1348 · no-terms 1101 · 0 violation**. Mycelium kann `LICENSE`/`README` aus den terms erzeugen. Offen: ESA/ESAC (CC BY-NC 3.0 IGO, Vokabel-Erweiterung), zenodo per Record.
- **`swpc_efield`/`substorm`/Kellerman:** Arme + `sources.φ`-Zeilen gebaut; Manifestation über den Generator/Compiler (swpc: `swpc-efield-cdn.yml`).
- **SuperDARN MAP-grid:** die Globus-Credentials stehen in `.secrets.local` (`GLOBUS_ID_USER`/`GLOBUS_ID_PASS`) — **kein Operator-Akt**; der DMap-Kern + `map_grid_value` stehen, der Download (`superdarn.ca/data-download`, `wartend.φ:8`) liegt bei dir.
- **Keogramm** (OMTI/Abisko): Wire-Feld descoped (raw/relativ); offen ist die **Vision-Asset-Registrierung** — falls Mycelium das Bild als Asset führen kann, bitte führen.
- **Route-Admissionen** manifestieren, sobald Mountain die Zeilen/Arme baut.
- **quake-feeds-cdn:** von dir übergeben (`ceic.ac.cn` vom Runner nicht erreichbar) — Mountain prüft Route/Ersatz.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push (dieser Atom: `/commit`).

Eigene Pfade: `phi/sources.φ` · `phi/blocked_sources.φ` · `src/archivar/channels.rs` · `src/archivar/types.rs` · `src/archivar/parse.rs` · `src/archivar/main_flow.rs` · `src/archivar/extract.rs` · `src/archivar/mod.rs` · `src/archivar/tests.rs` · `src/archivar/tiff.rs` · `src/archivar/supermag_index.rs` · `src/archivar/swpc_efield.rs` · `src/archivar/dmap.rs` · `src/archivar/keogram.rs` · `src/archivar/substorm.rs` · `src/archivar/fetch.rs` · `src/archivar/port.rs` · `src/lib.rs` · `src/gate/commit_gate_vocab.json` · `.github/workflows/swpc-efield-cdn.yml` · `tools/harvest/src/bin/supermag_index_compiler.rs` · `tools/harvest/src/bin/swpc_efield_compiler.rs` · `tools/harvest/src/bin/keogram_compiler.rs` · `tools/harvest/src/bin/substorm_compiler.rs` · `docs/handover/handover-2026-10-09-mountain-folge282.md`.
