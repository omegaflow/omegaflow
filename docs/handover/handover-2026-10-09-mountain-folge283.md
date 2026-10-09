<!--
  title: Handover — Mountain-Folge 283 (2026-10-09)
  session: Mountain-Folge 283
  class: handover
  date: 2026-10-09
  sha256: 60cfff2282dca6a8893b4fd45d214d60677ae6b1f9e24b9e8573c0bb1e7a401d
  status: live
-->
# Handover — Mountain-Folge 283 (2026-10-09)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, git trägt es. Der
Stehende Pass wird zitiert, nie kopiert (`state/zustand/standing-pass.md`,
Mycelium, gemessen 2026-10-09). Diese Session konsumierte
`handover-2026-10-09-mountain-folge282.md` (→ `archiv/`). Kein pro/max.

## Burn: open 0.0000 · close 0.1081 · cap 0.50 — Grund: line (flash); Σ `session_burn` total 5.7670→5.8751 (42→45 Sessions, Aggregat-Delta), keine Taucher, keine pro/max-Dispatches

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

### GIC-Faden §A–G — THEMIS-Tail + MMS-Magnetosheath über CDAWeb-HAPI gebaut
- **Status:** eigen | **Bindung:** eigen (Register)
- **Trigger:** verbleibende `gap`-Träger (map-grid-Download / Keogramm-Vision-Asset)
- **Lage:** (gemessen 2026-10-09, diese Session) **THEMIS-Tail + MMS-Magnetosheath als CDAWeb-HAPI-Arme registriert** (`phi/sources.φ`): `MMS1_EDP_FAST_L2_DCE` (3× E-Feld mV/m), `MMS1_FPI_FAST_L2_DES-MOMS` (Dichte cm-3 + 3× GSE-Bulkgeschwindigkeit km/s), `THA_L2_MOM@0` (Dichte + 3× GSE-Bulkgeschwindigkeit); `THA_L2_FGM@0` steht bereits (live, stop 2026-10-07). Beide `blocked_sources.φ`-Einträge (`themis-tail`, `mms-magnetosheath`) entfernt. Messung: `THA_L2_MOM@0` liefert 21363 Zeilen für 2026-10-06..09 (live); MMS lagt ~2 Monate (bestehendes `MMS1_FGM` ebenso — Absenz wird als Absenz geführt). **THEMIS EFI (`THA_L2_EFI@0..5`) endet 2025-01-05** — kein Lebendarm, nicht registriert. `register_sort` canon 2710 Blöcke; `license_census` terms 1624 · 0 violation; `cargo check` 0/0. Keogramm-Dekoder + Compiler stehen; Wire-Feld descoped (raw/relativ).
- **Blockade:** SuperDARN-MAP-grid-Download liegt bei Mycelium (`wartend.φ:8`, Globus-Credentials stehen); Keogramm-Vision-Asset bei Mycelium.
- **Braucht:** Mycelium: MAP-grid-Download + Keogramm als Vision-Asset; sonst nichts Neues.

### Flyby-Kette — Residual liegt in ODF; σ_recon getrennt
- **Status:** termin | **Bindung:** eigen (Register) · river (`flyby_ephemeris_gate`)
- **Trigger:** ESOC-Recon-Release (Wiedervorlage 2026-11-01) oder Descope
- **Lage:** (gemessen 2026-10-09) Der ESTRACK/DSN-Residual ist **nicht absent**: 157 ODF-Referenzen in `sources.φ` (MEX `:9945`, Rosetta `:9953`, Juno `:10011`, Magellan `:10723`, Pioneer `:18231`, Viking `:18365`, VEX `:27809`); `odf.rs` steht, `doppler.rs` absent. `estrack.esa.int` hat kein Datenportal. **Riss:** σ_recon ist NICHT ein Doppler-Residual, sondern die 1-σ-Kovarianz der ESOC-Post-Flyby-Recon-Ephemeride (`ephemeris_juice_recon.bin` 404; river-139 `:74-79`, gate `flyby_ephemeris_gate.rs:296-307`).
- **Blockade:** kein ESOC-Recon-Release; `doppler.rs` wird vom ODF-Residual nicht gebraucht.
- **Braucht:** ESOC-Release abwarten (river) oder Descope-Befund für `doppler.rs`.

### IGRF-Koeffizienten-Arm (`geomag_lat`)
- **Status:** wartend | **Bindung:** eigen (CI-Lauf)
- **Trigger:** CI-Test `synthesis_matches_pyigrf14_witness_points` grün
- **Lage:** (gemessen 2026-10-09, diese Session) Grad-13-Synthese + `igrf.rs` stehen. Run `37910517444` (`ci-check`, head `ce47ce13c`) endete **cancelled** (attempt 1, nicht grün). **Re-Dispatch (diese Session):** `gh workflow run ci-check.yml` → run `37929072865`.
- **Blockade:** keiner.
- **Braucht:** `ci_manage view 37929072865` beim nächsten Pass (kein Session-Polling); bei grün Punkt schließen.

### Lizenz-Disposition — `terms`-Feld (SPDX); Rest-Sweep offen
- **Status:** eigen | **Bindung:** eigen (Format/Datenkontrakt)
- **Trigger:** `terms`-Zeilen je Quelle geschrieben
- **Lage:** (gemessen 2026-10-09, diese Session via `license_census`) **blocks 2710 · terms 1624 · distinct 11 · no-terms 841 · pending 1086 · 0 violation(s)**. Vokabular (geschlossen, `license_census.rs:9-22`): `PD`, `CC-BY-4.0`, `CC0-1.0`, `unbestimmt`, `ohne-lizenz`; `NOASSERTION`/`NONE` sind NICHT im Vokabular.
- **Blockade:** **ESA/ESAC** (archives/gea/isla/eas/…esac.esa.int, psa.esa.int) tragen **CC BY-NC 3.0 IGO** — nicht im geschlossenen Vokabular; `zenodo.org` per Record; `datalab.noirlab.edu`/`dc.g-vo.org` ohne Daten-Lizenzaussage (`pending`); gemischte Hosts (arcgis/jaxa/kasi/…); Anker fehlt weiter für Blöcke ohne `format`/`origin`.
- **Braucht:** Vokabel-Erweiterung um `CC-BY-NC-3.0-IGO` (Operator-Wort) ODER ESA `pending` belassen; zenodo-Terms per Record (`archive_search --zenodo <id>`); Rest-Sweep je Anker; `license_census`/`ci-gate` nachführen.

### `blocked_sources.φ`-Aufräumen — 15 Klassen-Träger (`gap`-Token)
- **Status:** eigen | **Bindung:** eigen (Disposition) · mycelium (Diver-Tabelle)
- **Trigger:** Bau je Klassen-Träger
- **Lage:** (gemessen 2026-10-09, diese Session) **15 verbleibende `gap`-Träger** (themis-tail + mms-magnetosheath gebaut/entfernt): `bc-mpo-more` (PSA-Release 2099, wartet `psahelp@`) · `tracking-doppler`/`viking-tracking`/`juno-efb` (NSSDC-Antworten, `mail_ledger.φ:207/209/211`, Parser `odf.rs`/`viking_text.rs` stehen) · `mariner-rst` (7-Track-Parser fehlt, SDDPT) · `dmap-map-grid` (Globus-Download bei Mycelium `wartend.φ:8`; Kern `dmap.rs` + `map_grid_value` stehen) · `kaguya-lrs` (WUSTL/DARTS `.lbl`/`.dat`) · `inpe-big-stac` (79 Sammlungen, Sammlung→Feld-Compiler) · `hi-21cm` (VO/FITS-Arm) · `cmb-lambda` (LAMBDA/PLA-Produkt-Arm) · `solar-vso` (FITS/VSO-TAP) · `laic-cssdc` (Portal-Parser) · `particle-cern` (CERN-Open-Data-API + GWOSC) · `blinkverse-frb` (direct+Proton pending, kein Wayback). TUH/NSRR-`blocked account`-Riss geschlossen (Mountain-Verdikt 2026-10-09: kein Descope, Konto real, Daten hinter NEDC/HIPAA).
- **Blockade:** je Träger der Bau (Arm/Workflow/`sources.φ`-Zeile) oder eine wartende Antwort.
- **Braucht:** je Träger Arm/Workflow/`sources.φ`-Zeile oder Disposition.

### Kraft-/Register-Physik-Migration (`force` → Quantity | Mechanism | Medium) — river-143
- **Status:** eigen | **Bindung:** Rat + Operator-Wort
- **Trigger:** Operator-Wort zur Taxonomie + Migrationsfreigabe
- **Lage:** (gemessen 2026-10-09, gefaltet river-143) River legt near-unanim (Rat + 8 Open-Weight + UI) vor: `electric` ⊂ `em`; `thermal` ≡ `diffusion` (gleiche PDE); `acoustic`/`seismic-body`/`seismic-surface` = ein elastisches Medium (Vorschlag `elastic` + Modus-Parameter); fehlend chemisch/Teilchenstrahlung/quasistat. Magnetfeld; `strong`/`weak` explizit out-of-scale. Werkzeug `source_physics_lint` (read-only): field 8006 · quantity 27 · 95 Geometrie-Kandidaten. Rat-Entwurf Zielschema `docs/concepts/kanal-ontologie-komplettbau.md` §P10.2a: `field <selector> <quantity> <kernel> <pde_type> <medium> [<interaction>] <role> <unit> <tau>`, `force` als n:m-Tag. Migrationsreihenfolge: `gravity`/`seismic` → `acoustic`/`diffusion` → **`em` zuletzt**. River hat **nichts** in `phi/` geändert; Protokolle `state/stimmen/2026-10-09-river-kraefte-tonmodelle.md` · `…-river-register-physik.md`.
- **Blockade:** Architektur-/Register-Entscheid; kein Messschritt offen.
- **Braucht:** Operator-Wort (Taxonomie + Migrationsfreigabe); die Verdikt-Zeilen schreibt Mountain.

## An mycelium

Origin: mountain-folge283.

- **terms-Ernte:** **terms 1624 · no-terms 841 · pending 1086 · 0 violation** (gemessen 2026-10-09, `license_census`). Der `sources_repo_license`-Bin bleibt **dein** uncommitteter Draft (`tools/register/src/bin/sources_repo_license.rs`); Mountain baut ihn nicht. Offen für dich: ESA/ESAC (CC BY-NC 3.0 IGO, Vokabel-Erweiterung), zenodo per Record, gemischte Hosts.
- **SuperDARN MAP-grid:** Globus-Credentials stehen (`GLOBUS_ID_USER`/`GLOBUS_ID_PASS`), kein Operator-Akt; der Download (`superdarn.ca/data-download`, `wartend.φ:8`) liegt bei dir.
- **Keogramm** (OMTI/Abisko): Wire-Feld descoped (raw/relativ); offen ist die **Vision-Asset-Registrierung** — bitte führen, falls Mycelium das Bild als Asset trägt.
- **Route-Admissionen / Arme:** THEMIS-Tail + MMS-Magnetosheath-Arme stehen jetzt (`phi/sources.φ`); manifestiere, sobald die Zeilen stehen. Übrige `gap`-Träger siehe oben.

## LOCK

- **Privater TE-Pfad (Mountain 217).** Wort „1 ja bitte" (2026-10-02, river-folge82): `complex_te_probe` um Detrend-along-p + CMI/pTE-mit-p-Kovariate erweitern (`docs/blatt/blatt-te-externer-steuerparameter.md`), Lauf lokal/silent, nie CI. Träger `state/mountain/kuprat-complex-te/`. Beide Arme gebaut, `--selftest` grün; offen: der Sweep. Riss: KDE-CMI verliert Power bei großer Kovariat-Varianz.

## Abschluss

Der Commit ist die letzte Handlung; das Commit-Wort des Operators trägt Commit und Push (dieser Atom: `/commit`).

Eigene Pfade: `phi/sources.φ` · `phi/blocked_sources.φ` · `docs/handover/handover-2026-10-09-mountain-folge283.md` · `docs/handover/archiv/handover-2026-10-09-mountain-folge282.md`.
