<!--
  title: Handover — Mycelium-Folge 175 (2026-09-27)
  session: Mycelium-Folge 175
  class: handover
  date: 2026-09-27
  sha256: 9789f2408c346c1c61c36c6203800a10a487035144e4867cc0dda3e372e4bf7c
  status: live
-->
# Handover — Mycelium-Folge 175 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `wartend` | `operator-gebunden` |
`blockiert` | `termin`.

Diese Session konsumierte `handover-2026-09-26-mycelium-folge174.md`.

## Operator-Wort-Register

- Wort | 2026-09-26 | „all" — session-weiter Consent (`mycelium_go`), Delegation an alle Taucher.
- Wort | 2026-09-27 | „DEMETER ist Sensory" → DEMETER bleibt aus der Mycelium-Übergabe (Sensory führt ihn).
- Wort | 2026-09-27 | „du machst GOSAT" → GOSAT bleibt Mycelium-Punkt.
- Wort | 2026-09-27 | „monthly + 8-day als per-Granule-Serie bauen; daily descopen" → modis-cdn per-Granule + Manifest; daily descoped.
- Wort | 2026-09-27 | „/commit" — Commit-Wort: Eigenarbeit committet + gepusht (`bd8961c2f`).
- Wort | 2026-09-27 | „lies die secrets local" → `GOSAT_GW_MAIL/PASS` in `.secrets.local` gefunden, als Repo-Secrets gesetzt, `gosat-cdn` dispatcht.
- Wort | 2026-09-27 | „volume" — gibt die φ-Direktive `volume` (3D-Gitter-Ingest) frei; Ingest-Arm gebaut (`88fb49209`).

## Offen (aufgeschlüsselt)

### Linie (eigen)

#### modis-cdn — per-Granule-Serie (monthly + 8-day), daily descoped
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `modis-cdn`-Lauf grün.
- **Lage:** (gemessen 2026-09-27 via `ci_manage`) Lauf `36277648527` failure — der Fehler war der Main-Red-Blocker (`cannot find type PresenceSample`, `main_flow.rs:5299`, durch `cff336062` geheilt), nicht der modis-Compiler; HEAD `cargo check` 0/0. Re-Dispatches laufen auf grünem Main: `36278484232` (in_progress) + `36278795720` (queued).
- **Blockade:** keine.
- **Braucht:** `ci_manage view <id>` bei Abschluss; danach die Jahres-Manifeste ins Register nachziehen + das jahrlose Serien-Manifest bauen.

#### IRIS/EarthScope EMC netCDF-4 — volume-Arm gebaut, Manifestation offen
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** EMC-Asset-Manifestation (Fetch-Route geklärt) ODER Operator-Wort zur Browser-Brücke.
- **Lage:** (gemessen 2026-09-27) Wort `volume` gegeben und umgesetzt (`88fb49209`): maskenbewusster Container (`volume.rs`, Magic `…0x02`, Probe über maskierter Ecke = absent), `Extract::Volume` + Grammatik-Arm `volume`/`depth` (`types.rs`/`parse.rs`), `build_netcdf4_volume` (`channels.rs`, f32 nativ, namensbasierte Re-Order `[depth,lat,lon]`, `_FillValue`→Maske, 4D-refuse), Arm `format volume_netcdf` (`main_flow.rs`), GPU-Head 10→11 (`mask_off`), Tests + CPU/GPU-Parität; `cargo check` 0/0, `ci-check` läuft auf `88fb49209`.
- **Blockade:** EarthScope-ToS (Attribution + Non-human-Visitor) — der Maschinen-Fetch ist nicht gedeckt (Browser-Brücke/Operator-Profil).
- **Braucht:** Fetch-Route (Operator-Wort oder `--playwright`-Brücke) → netCDF auf CDN, dann `phi/sources.φ` Eintrag `format volume_netcdf` (Attribution benannt) → CI-Manifestation.

#### EPN-core — `epncore-spatial`-Gap registriert
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** `epncore-spatial`-Parser-Arm (mountain) ODER src.pas-Backend erholt.
- **Lage:** (gemessen 2026-09-27) Der MASER-Ersatz `voparis-tap-maser` ist bereits `declined` (`declined_sources.φ` — Registry-Katalog, kein Roh-Messwert am Punkt); der Gap ist als `parser-def epncore` + `gap epncore-spatial` in `phi/blocked_sources.φ` registriert (owner mountain).
- **Blockade:** Parser-Arm fehlt (mountain); src.pas tot (PostgreSQL `:5432` refused, `/tap/tables` 500).
- **Braucht:** mountain baut den `epncore-spatial`-Arm; danach Re-Check.

#### GOSAT-GW GWT3F_L1B — Lauf dispatcht
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `gosat-cdn`-Lauf `36280012910` grün.
- **Lage:** (gemessen 2026-09-27 via `gh secret list`/`gh run list`) Arme + `gosat-cdn.yml` gebaut (`fae4a5081`); Repo-Secrets `GOSAT_GW_MAIL/PASS` gesetzt; Lauf `36280012910` (GWT3F_L1B, 2024-01-01..2026-12-31) queued.
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36280012910` bei Abschluss; danach `sha256` in `phi/sources.φ` + `blocked_sources.φ` → released.

#### EMODNET HFRADAR NADR — Termin-Re-Messung
- **Status:** termin | **Bindung:** termin:2026-10-19
- **Trigger:** 2026-10-19.
- **Lage:** (gemessen 2026-09-24 via `external-state.md:43`) Asset registriert; Re-Messung offen.
- **Braucht:** `archive_search --sniff` bei Fälligkeit.

### Dritter

#### BepiColombo bc_mpo_more — Termin
- **Status:** termin | **Bindung:** termin:2027-04-01
- **Trigger:** 2027-04-01 (Science-Phase-Beginn).
- **Lage:** (gemessen 2026-09-26) MORE-Cruise nicht öffentlich, Freigabe April 2027; Ticket `YYM-342-97327`.
- **Braucht:** Wiedervorlage 04/2027, dann MORE-`data_raw`/`calibration_raw`.

#### SSDC / Limadou — wartend auf CSES-02-Prozedur
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** neue Zugangsprozedur `limadou.ssdc.asi.it`.
- **Lage:** (gemessen 2026-09-26 via `sfetch`) `query.php` → SSDC-CAS-Login; neue Prozedur noch nicht live. Sotgiu (`mail_ledger.φ`): „wait a few weeks".
- **Blockade:** Portal-Umbau (dritter).
- **Braucht:** neue Prozedur abwarten; beim Trigger `query.php` re-messen.

#### termin-Punkte — re-verdict
- **Status:** termin | **Bindung:** termin:2026-09-28
- **Trigger:** 2026-09-28 (DEMETER) / 2026-10-02 (übrige) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-26 via `--verdict`) keine Erholung bei `regards.cnes.fr` (403) / `pithia.cbk.waw.pl` (500/TAP down) / `api.lasair.lsst.ac.uk` (404); Ersatzrouten gefunden.
- **Braucht:** `archive_search --verdict <url>`; bei Erholung den `*-cdn.yml`-Lauf dispatchen.

## Träger (Prosadokumente)

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | `mariner-occlt` Asset liegt (1 375 496 B, sha256 4aa487cb…) | nächster Schritt: verbleibende Survey-Marker prüfen.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | SAMPLE_CONTACT (MPI-FKF/LAB_A) sagte LAB_A-`I(q,t)`-Daten zu, danach kein Eingang | wartend auf Mail-Eingang (kein Nachfassen).
- `docs/surveys/survey-2026-09-26-secrets-inventar.md` | Namens-Disposition trägt die Future-Übergabe | nächster Schritt: Future.
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | ausstehend nur Wiedervorlage 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending weiter tot | nächster Schritt: `--verdict` je Host beim Trigger.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | Step 4 (CI-Dedupe) konkretisiert in `docs/auftrag/archiv/auftrag-saubere-datenbank.md`; offen: Step 5 (CDN-kanonisch, destruktiv → Operator-Wort) | nächster Schritt: Klassen-Zensus messen.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Migrationsplan-Vorlage steht; stoppt am Operator-Wort | nächster Schritt: Operator-Wort zum Layout `knowledge/`+`backups/`.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen nur §7 Roh-Korpora/Scratch-Disposition | nächster Schritt: Operator-Wort.

## Abschluss

Commit-Wort (`/commit`) gegeben 2026-09-27.
