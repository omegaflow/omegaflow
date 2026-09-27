<!--
  title: Handover — Mycelium-Folge 185 (2026-09-27)
  session: Mycelium-Folge 185
  class: handover
  date: 2026-09-27
  sha256: bc988babdf1ab41d2f470339481e71ef468f2d8441d8a856f8f5bc009f354e8a
  status: live
-->
# Handover — Mycelium-Folge 185 (2026-09-27)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht; git trägt, was
gemacht wurde. Keine Rangfolge; jeder Punkt aufgeschlüsselt: **Trigger** /
**Lage** / **Blockade** / **Braucht**. Status-Tag: `eigen` | `wartend` |
`blockiert` | `termin`; Operator-Akte leben in Futures Operator-Queue, Dritt-Waits
in `state/zustand/wartend.φ`, nie als Linien-Punkt.

Diese Session konsumierte `handover-2026-09-27-mycelium-folge184.md`.

Kein Standard-Pass: es gilt der **Stehende Pass** (`state/zustand/standing-pass.md`)
— zitiert, nie in dieses Register kopiert.

## Operator-Wort-Register

- Wort | 2026-09-27 | „kümmer dich drum" — der verwaiste rustfmt-Fix `tools/utils/src/bin/omega_sh.rs` fällt Mycelium zu (Aufenthalt = Eigentum); Commit trägt `/commit`.
- Wort | 2026-09-27 | „Du kannst. Führe den … Plan aus — als `line`-Agent" — session-weiter Consent (Delegation), **nicht** das Commit-Wort.
- Wort | 2026-09-27 | „die Kante bin ich" — jede Linie arbeitet bis zur Kante des Operators; Wert, Wort, Dritt-Akt und Send bleiben seine Hand | Operator (Future-Session).
- Wort | 2026-09-27 | ein gegebenes Wort steht in den Operator-Wort-Registern aller live Übergaben — Verbreitung im selben Atom | Operator (Future-Session).
- Wort | 2026-09-27 | RX100-Kalibrierer descoped — „über exif weg": Luminanz über den Kamera-EXIF-Weg (K=12.5) | Operator (Future-Session).
- Wort | 2026-09-27 | Entscheidungen nie als Liste vorlegen — eine Liste ist keine Entscheidungshilfe; jede Entscheidung braucht eine aussagekräftige Erklärung | Operator (Future-Session).
- Wort | 2026-09-27 | UI-Chat-Stimmen derzeit nicht gebraucht → `LOCK` | Operator (Session, Mountain).
- Wort | 2026-09-27 | D5 (Orphan-Doc-Träger) nicht in die Übergabe falten — die Fakten direkt abarbeiten | Operator (Session, Mountain).
- Wort | 2026-09-27 | RX100 war nur ein Gedanke — Quelle zurückgezogen, Workflow-Zweige entfernen | Operator (Session, Mountain).
- Wort | 2026-09-27 | „Du kannst. Führe den in Phase 1 vorgeschlagenen und jetzt bestätigten Plan aus — als `line`-Agent" — session-weiter Consent der Folge 184 (Delegation), nicht das Commit-Wort | Operator (Mycelium-Session 184).
- Wort | 2026-09-27 | „falte alle" / „ja bitte falten" — die genuin-offenen Punkte der trägerlosen Docs in die Übergaben ihrer Linien falten (Aufenthalt = Eigentum) | Operator (Mycelium-Session 184).
- Wort | 2026-09-27 | „den rest gebe ich future" — die Tafel trägt nur `eigen`; operator-gebundene Punkte → Future-Operator-Queue (matrix-rotor-Design-Wort), Dritt-Waits → `state/zustand/wartend.φ` (voyager-nssdca, bepicolombo-more); die Queue wird nicht kopiert | Operator (Mycelium-Session 185).
- Wort | 2026-09-27 | „Du kannst. Führe den … Plan aus — als `line`-Agent" — session-weiter Consent der Folge 185 (Delegation), nicht das Commit-Wort | Operator (Mycelium-Session 185).

## Offen (aufgeschlüsselt)

### modis-cdn / modis-asset-bridge — 1000-Asset-Cap, Jahr-Split-Umbau
- **Status:** eigen | **Bindung:** eigen
- **Trigger:** sofort (Rat-Verdikt 2026-09-27 liegt vor).
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36320557426` + GitHub-API) `compile` und `series-manifest` scheitern an `HTTP 422 … file_count limited to 1000 assets per release`. Release `data.lpdaac.earthdatacloud.nasa.gov-modis_lst_cmg` trägt 1000/1000 (8day 887, monthly 113); Host-Release `data.lpdaac.earthdatacloud.nasa.gov` 1000/1000 (modis 999). Der Produkt-Split reicht nicht (8day allein wüchse >1000).
- **Blockade:** keine (Architektur entschieden).
- **Braucht:** Rat-Verdikt Schema (b) umsetzen: (1) `src/archivar/cdn.rs` — `pub const MODIS_LST_CMG_FAMILY` + `pub fn modis_lst_cmg_tag_of(name) -> Option<String>` (Produkt + Jahr aus `modis_lst_cmg_<product>_<granuleUR>.bin` bzw. `…_<year>.manifest`, Void → None) + Test, PS1-Präzedenz `ps1_slab_tag`; (2) `.github/workflows/modis-cdn.yml` — `raster`-Present-Check, `compile`-`MODIS_CDN_TAG`, `release`-Notes und `series-manifest` auf `{FAMILY}-{product}-{year}` umstellen; Serien-Manifest weiter in den Familien-Tag (2 Assets, nie 422); (3) Einmal-Migration (Muster `modis-asset-bridge.yml`): Familien-Assets per Tag-Funktion in die Jahrestags heben (download → sha256-Parität → upload), **dann** Delete der verifiziert verschobenen aus dem Familien-Tag (Pflicht — sonst 422t der Serien-Upload); vorher Brücken-Laufstand messen; (4) `tools/register/src/bin/cdn_reconcile.rs` `classify` — Arm `data.lpdaac.earthdatacloud.nasa.gov-modis_lst_cmg-` → `internal` (Prefix aus `cdn.rs`); (5) `phi/sources.φ:11693/11710` bleiben, `origin` (11695/11712) um das Jahr-Tag-Schema ergänzen. Reader-Arm (`format modis_lst_cmg`) bleibt Mountains Territorium.

### gosat-cdn — Ghost-Lauf (2026-Shard)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Re-Dispatch des `gosat-cdn`-Workflows.
- **Lage:** (gemessen 2026-09-27 via `ci_manage view/jobs/log`) Lauf `36322845122` API-`in_progress`, `updated_at` == `run_started_at` (13:34), **nicht** in `ci_manage status`, Job-Log HTTP 404 → Ghost (kein Live-Step ≥2h). `raster`/`release` success; `compile (2026)` hängt.
- **Blockade:** Ghost-Run (runner-seitig).
- **Braucht:** `ci_manage jobs 36322845122` bestätigt den Ghost; dann `gh run cancel 36322845122` und `gh workflow run gosat-cdn.yml` (oder `--ref main`).

### hinet-cdn — CONT-Readiness
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** periodischer Re-Dispatch (`gh workflow run hinet-cdn.yml`) / Hinet-Readiness.
- **Lage:** (gemessen 2026-09-27 via `ci_manage log 36323256126`) Lauf `...963` failure; `hinet: cont status never read Available — the request stays unfetched`, 8× `attempt 0..7 stayed unready` (13:42→14:28). Auth öffnete (200).
- **Blockade:** quellenseitige Readiness.
- **Braucht:** `gh workflow run hinet-cdn.yml` beim periodischen Trigger; bleibt es so, wartend auf Hinet.

### ci-check / dropped-gate
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `ci-check 36329259667` (pending) / `36326102925` (in_progress, Schritt `test`) beendet.
- **Lage:** (gemessen 2026-09-27 via `ci_manage status`) neueste `ci-check`-Läufe am HEAD `8d812fd8` pending/in_progress; der `dropped-gate`-Failure `36324206113` steht (`baseline 989 | current 1023 | delta 34`); lokaler `register_lookup --dropped --count` = Timeout (>180 s, pending).
- **Blockade:** keine.
- **Braucht:** `ci_manage view 36329259667`; bei `dropped-gate`-Rot `ci_manage log <id>` lesen, die 34 je Owner tragen und `docs/zustand/dropped-baseline.md` im annehmenden Commit bumpen (989 → 1023).

### termin-Punkte — re-verdict
- **Status:** termin | **Bindung:** termin:2026-10-02 / 2026-12-02
- **Trigger:** 2026-10-02 (übrige) / 2026-12-02 (NOIRLab/Gaia-DR4).
- **Lage:** (gemessen 2026-09-27) `pithia.cbk.waw.pl` backend-tot; `api.lasair.lsst.ac.uk/api` direct absent / proton 200.
- **Blockade:** keine (Wiedervorlage).
- **Braucht:** `archive_search --verdict <url>`; bei Erholung `*-cdn.yml` dispatchen.

### D5-Orphan-Residuum — Quellen-Registrierung/Route (Myceliums Feder)
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Operator-Wort „falte alle" (2026-09-27) — D5-Nachzug (`state/mountain-185-orphan-doc-nachzug.patch`).
- **Lage:** (gemessen 2026-09-27 via D5-Patch + `sgrep`/`archive_search`) genuin-offene Punkte der trägerlosen Docs, die die Manifestations-/CDN-Feder berühren: ESA-CCI SST (Nachfolger `climate.esa.int`, Host tot `dead_sources.φ:411`, keine `sources.φ`-Zeile); ZTF-Quelle (Kanal lebt, Quelle fehlt `sources.φ`; Lasair-LSST descoped `blocked_sources.φ:28`); Telescope-Array-Vollkatalog (kein `sources.φ`-Eintrag); Occultation-DB-URL (`J.Phys.Conf.Ser.` 1365, 012024 — unregistrierter Kandidat); 3D-Tomografie-Modelle (`positive-maske.md:58`, kein `tomograph` in `phi/`); GW/Neutrino/CR-Skymap (`zeugnis.md:380`, Positionen pending).
- **Blockade:** keine.
- **Braucht:** je Route `archive_search --verdict`/`--sniff` messen, dann `phi/sources.φ`-Zeile (url/origin/compiler) — Myceliums alleiniger Register-Schreibakt; Reader-Arme bleiben Mountain.

### D5-Orphan-Residuum — CDN-Manifestations-Weg + P2P
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** wie oben — Operator-Wort „falte alle" (2026-09-27), D5-Patch (`state/mountain-185-orphan-doc-nachzug.patch`).
- **Lage:** (gemessen 2026-09-27 via D5-Patch) `zeugnis.md:383` §14.4 — CDN-Manifestations-Weg des Röhren-Assets offen; `survey-2026-09-17-omegaflow-legacy-konzepte.md:47` — Mycelium/Nostr-P2P nicht gebaut (Lesen autonom, Schreiben consent-pflichtig).
- **Blockade:** keine.
- **Braucht:** Manifestations-Weg des Röhren-Assets bauen (CDN-Duty); P2P bauen, Schreibpfad nur mit Consent.

### survey-fortschritt.md — Träger-Klärung
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** `register_lookup --orphan-docs` beim nächsten Pass.
- **Lage:** (gemessen 2026-09-27, Sensory-Folge 187 `Träger`-Sektion) `docs/surveys/survey-fortschritt.md` → Mycelium benannt; im aktuellen `--orphan-docs`-Lauf nicht mehr gelistet (entweder markerlos oder inzwischen getragen) — ungeprüft.
- **Blockade:** keine.
- **Braucht:** `register_lookup --orphan-docs` erneut messen; falls trägerlos, als Zeile aufnehmen.

## Träger (Prosadokumente)

- `docs/surveys/survey-2026-09-17-sonden-request-only.md` | `mariner-occlt`-CDN-Dispatch geschlossen; offen: native ODF-Serien-Arm + vier request-only-Routen | nächster Schritt: `archive_search --verdict` beim Trigger.
- `docs/surveys/survey-2026-09-14-warteliste-offene-alternativen.md` | SAMPLE_CONTACT (MPI-FKF/LAB_A) sagte zu, danach kein Eingang (gemessen 2026-09-27 via mail_ledger) | wartend auf Mail-Eingang (kein Nachfassen).
- `docs/surveys/survey-2026-09-14-kapitulationen-pendings-inventur.md` | nur `Wiedervorlage 2026-12-02` bindet | nächster Schritt: 2026-12-02.
- `docs/surveys/survey-2026-09-16-dead-sources-relevanz.md` | 3 Force + 4 pending weiter tot | nächster Schritt: `--verdict` je Host beim Trigger.
- `docs/surveys/survey-2026-09-03-orphan-verdicts.md` | offen: Step 5 (CDN-kanonisch, destruktiv); der Akt liegt in Future's Operator-Queue | nächster Schritt (eigen): Klassen-Zensus messen.
- `docs/surveys/survey-2026-09-03-daten-holdings-inventur.md` | Migrationsplan-Vorlage steht; das Layout-Wort liegt in Future's Operator-Queue | nächster Schritt: Migration nach Wort.
- `docs/surveys/survey-2026-09-07-tmp-opencode-scan.md` | offen nur §7 Roh-Korpora-Disposition; der Akt liegt in Future's Operator-Queue | nächster Schritt: Disposition nach Wort.
- `docs/concepts/tools-map.md` | CI-Werkzeug-Detail (`ci_manage status`/`jobs`) ergänzt; übrige offene Marker unverändert | nächster Schritt: `register_lookup --orphan-docs` beim nächsten Pass; Owner-Klärung offen.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`). `/consent` ist der
session-weite Consent, nie das Commit-Wort.
