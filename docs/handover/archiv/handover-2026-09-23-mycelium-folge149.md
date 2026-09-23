<!--
  title: Handover — Mycelium-Folge 149 (DataONE disponiert, NERACOOS accept/AFAD blocked, witnesses/footprints nachgemessen; Beat-Paar + Bayestar19 neu) (Stand 2026-09-23)
  session: Mycelium-Folge 149
  class: handover
  date: 2026-09-23
  sha256: 6f96bb8190be4a74c42321b476ed44c29518e9bf11399ad9b7f9f7050f2e64dc
  status: live
-->
# Handover — Mycelium-Folge 149 (2026-09-23)

Dieses Register trägt nur Offenes — Erledigtes wird gelöscht, nicht als „done"
markiert, nicht erklärt; git trägt, was gemacht wurde. Es gibt keine Rangfolge;
die offenen Punkte werden **parallel** von Agenten abgearbeitet. Jeder Punkt
**aufgeschlüsselt**: **Trigger** / **Lage** / **Blockade** / **Braucht**;
Status-Tag (`wartend` | `operator-gebunden` | `blockiert` | `termin`).

Diese Session hat `handover-2026-09-23-mycelium-folge148.md` konsumiert.

## Stehender Pass (gemessen 2026-09-23)

- **HEAD** `7ded53c` == `origin/main` (Base der Session; fremde Linien committen im geteilten Baum).
- **`git_safety --snapshot`:** Arbeitsbaum == HEAD, nichts zu sichern.
- **Postfach** — `state/mail/mail_ledger.φ`: kein neuer handlungsrelevanter Eingang (`mail_digest` absent).
- **CI am HEAD** — Watchdog-Snapshot 22:33: `ci-check 35906172370` in_progress; `te-gate 35893882101` in_progress; 5× `ci-check` failed (attempt 1, ältere SHAs).
- **`open_points_check` folge148:** 1 absent — `handover-2026-09-23-river-folge16.md` (river konsumierte ihn, live ist folge17); stale Verweis, kein offener Punkt.

## Diese Session (git trägt es)

- **DataONE disponiert** (Operator-Wort): `index.φ:107` `pending` → `index` (Tür-Katalog/Adapter-Route), Notiz auf das Verdikt; `korpora_heim.φ:28` `pending`-Zeile entfernt. Befund: Katalog-Ebene ohne öffentliche Terms (`/terms` 301→`old.dataone.org` tot, kein Wayback-Capture); per-Record-Lizenz regiert; keine Redistribution.
- **13. Korpus Endpunkt-Probe** (`grind-pro`): `data.neracoos.org` → **accept**, `phi/sources.φ` (ERDDAP-json-Block, thermal `neracoos_air_temperature_c` + advective `neracoos_wind_speed_m_s`/`_gust`); `tadas.afad.gov.tr` → **blocked parser-def html**, `phi/blocked_sources.φ` (Angular-SPA + F5, kein JSON).
- **witnesses/footprints nachgemessen** (`grind-flash`): 4 Zeugen (`witnesses.φ:7/13/37/91`) + 2 Footprints (`footprints.φ:12/19`) re-gemessen, `absent` bestätigt, Notes aktualisiert.
- **Rosetta ODF geprüft** (`grind-flash`): Lauf `35881738785` success; 6 Shards all-present, sha256 identisch mit `sources.φ` — **keine Änderung** (nichts zu registrieren).
- **Beat-Paar recherchiert** (`grind-pro`): kein Roh-Datensatz liefert zwei kohärente Töne (Dual-Comb speichert das aufgelöste Spektrum). Neuer Offen-Punkt unten.
- **Bayestar19-Route gemessen** (`grind-pro`): Harvard Dataverse DOI `10.7910/DVN/2EJ9TX` (CC0); korrekter Registerort ist ein **Binding** + `phi/canon.φ`-Deklaration, nicht eine field-Zeile. Neuer Offen-Punkt unten.

## Offen (aufgeschlüsselt)

### pre-cdn CI-Grün
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ci-check` auf dem folge149-Commit.
- **Lage:** offener Lauf `35906172370` in_progress (gemessen 2026-09-23, Watchdog-Snapshot).
- **Blockade:** keine.
- **Braucht:** nach Push `ci_manage view <id>`; Ergebnis ins nächste Handover.

### pre-cdn Stage-Regeneration
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** Merge-Atom.
- **Lage:** Queue gitignored → CI sieht sie nicht; 887 Blöcke, 141 keyless HAPI.
- **Blockade:** keine.
- **Braucht:** `--port` über `queue/sources_potential_pre-cdn_9k_richest.φ` + `…_params.φ`, gebunden im Merge-Atom.

### PS1 final-combine
- **Status:** wartend | **Bindung:** eigen
- **Trigger:** nächster `ps1-cdn`-Lauf bis `all_present`.
- **Lage:** `ps1_dr2_coverage.fp01` HTTP 404 (sniff 2026-09-23; `footprints.φ:19` bestätigt absent); Lauf `35855330990` success, final-combine nicht erreicht; Band-Parts 637–671, `band_max 2643`.
- **Blockade:** Ernte-Fortschritt.
- **Braucht:** bei `all_present` `footprints.φ` PS1-Note setzen.

### GitHub-Release-Asset-Cap
- **Status:** blockiert | **Bindung:** eigen (Rat)
- **Trigger:** Rat-/Architektur-Wort.
- **Lage:** 1000 Assets/Release erreicht; `upload_asset` blockiert (`src/archivar/cdn.rs:40`); ~74 Sites betroffen (`external-state.md`).
- **Blockade:** Architektur.
- **Braucht:** `council`-Verdikt, danach `grind-pro`.

### DEMETER Order 18387
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort für die Neuordnung.
- **Lage:** CNES-Zugang vorhanden (`CDPP_USER`/`CDPP_PASS`); Order 18387 44,5 % Fehler, 16 % fest, Ablauf 2026-09-28 (gemessen 2026-09-23). Browser-Login bei REGARDS autorisiert die API **nicht** (Bearer-Token; `/rs-order/user/orders` → 403).
- **Blockade:** Schreibakt bei CNES (Dritter).
- **Braucht:** Operator-Wort für Neuordnung in 100er-Batches (`DMT_N1_1144`).

### SuperDARN MAP
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator-Wort für Transfer-Start.
- **Lage:** Globus-Zugang vorhanden; Task `af68c4f1`, lokal 2932 `.map` (1993–2002), ~1562 fehlen; `blocked_sources.φ:16` (MAP 6.561 Dateien/21,93 GB).
- **Blockade:** schwerer Transfer.
- **Braucht:** Globus-Transfer starten/resumen (`data/superdarn/map`), dann `sources.φ`-Registrierung.

### src.pas TAP
- **Status:** wartend | **Bindung:** termin
- **Trigger:** `/tap/tables` 200.
- **Lage:** `ledger.φ:10` ausstehend; `/tap` 200, `/tap/tables` 500 (gemessen 2026-09-23).
- **Blockade:** Pithia-Backend.
- **Braucht:** Re-Messung bei Erholung.

### Lasair-LSST
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** Backend-Erholung.
- **Lage:** `blocked_sources.φ:3`; api/Frontend 000, Proton 500 (gemessen 2026-09-23).
- **Blockade:** Broker-Backend.
- **Braucht:** Re-Messung (Wiedervorlage).

### BepiColombo
- **Status:** wartend | **Bindung:** dritter
- **Trigger:** PSA-Antwort / Freigabe.
- **Lage:** `blocked_sources.φ:21`; `release_date 2099-01-01`, `data?PRODUCT` 403.
- **Blockade:** ESA-Freigabe.
- **Braucht:** Antwort `psahelp`.

### EMODNET HFRADAR NADR
- **Status:** termin | **Bindung:** termin 2026-10-19
- **Trigger:** Datum 2026-10-19.
- **Lage:** Re-Messung fällig 2026-10-19.
- **Blockade:** Termin.
- **Braucht:** Re-Messung.

### dropped-Zähler-Wurzel (→ mountain)
- **Status:** blockiert | **Bindung:** linie:mountain
- **Trigger:** Rat-/Architektur-Wort.
- **Lage:** `register_lookup --dropped --count` zählt Drops **vor** der Git-Auflösung; ~95–96 % `commit-resolved` (gemessen 2026-09-23, `grind-flash`).
- **Blockade:** Register-Tool-Semantik.
- **Braucht:** `register_lookup.rs` um die `commit-resolved`-Menge bereinigen.

### Sicherheits-Befund
- **Status:** operator-gebunden | **Bindung:** operator
- **Trigger:** Operator/Council-Urteil.
- **Lage:** eingeschleuster Instruktionsblock gemeldet, Herkunft ungemessen.
- **Blockade:** Herkunft ungemessen.
- **Braucht:** Operator/Council-Urteil.

### Beat-Paar Datenquelle (neu, aus Post river folge12)
- **Status:** blockiert | **Bindung:** linie:mountain
- **Trigger:** eine gemessene Zwei-Ton-Rohquelle oder ein descoped-Befund.
- **Lage:** `beat_pair` (`shaders.rs:306`) verlangt zwei Oszillatoren im selben Kraft-Kanal, `df·dt<0.5`; Dual-Comb/Maser-Kandidaten liefern nur Paper bzw. `df·dt≥0.5` (gemessen 2026-09-23, `grind-pro`); kein Registry-Treffer.
- **Blockade:** Klasse existiert als Roh-Datenquelle nicht.
- **Braucht:** Zwei-Ton-Rohquelle finden oder `descoped mit Befund`.

### Bayestar19 Binding (neu, aus Post river)
- **Status:** operator-gebunden | **Bindung:** owner (Rat) → operator
- **Trigger:** Operator-/Council-Wort für neues Binding + `phi/canon.φ`-Deklaration.
- **Lage:** nicht in `phi/sources.φ`; Route gemessen: Harvard Dataverse `10.7910/DVN/2EJ9TX` (CC0), `bayestar2019.fits.gz` `datafile/3424708`; Felder `NSIDE/HEALPIX_INDEX/BEST_FIT` (120 Bins); Netloc-Mismatch in `bayestar_compiler.rs` (Upload `dataverse.harvard.edu` vs. Workflow-Release `ssd.jpl.nasa.gov`) vor Registrierung klären.
- **Blockade:** neues Binding/canon = Architektur-Akt.
- **Braucht:** Rat-/Operator-Wort; dann Binding-Zeile + `canon.φ`-Deklaration.

## Benchmark

- Dispatches dieser Session: 1× `grind-flash` (Rosetta, witnesses/footprints), 2× `grind-pro` (13. Korpus, Beat-Paar, Bayestar19). Alle trugen (5/5 korrekt). Keine pro/max-Paarung (Routine- und Force-Gate-Klassen bereits gemessen geschlossen).
- **DataONE/Browser-Regel (gemessen):** ein Browser-Login öffnet keine gelöschte Seite (`/terms` 301→tot) und autorisiert keine Token-API (REGARDS 403). Beide „operator-gebunden"-Punkte brauchten den Operator-Wort-Weg, nicht die Session.

## Geteilter Baum — eigener Pfad-Satz

- **Eigene Dateien dieser Session:** `phi/pipeline/index.φ`, `phi/pipeline/catalog/korpora_heim.φ`, `phi/sources.φ` (NERACOOS-Block), `phi/blocked_sources.φ` (AFAD), `phi/witnesses.φ`, `phi/footprints.φ`, `docs/handover/handover-2026-09-23-mycelium-folge149.md`.
- **Move mit dem Commit:** `handover-2026-09-23-mycelium-folge148.md` → `archiv/`.
- **Offener Befund:** `post.md` trägt keine `An future:`-Zeile für die operator-gebundenen Punkte (DataONE/DEMETER/SuperDARN/13. Korpus) — der Operator-Queue-Eintrag steht nicht sichtbar; beim nächsten Pass setzen.
- **Notiz:** DataONE-Session im opencode-Browser zeigt ein Konto „Johannes Tyroller", an das sich der Operator nicht erinnert (Herkunft ungemessen) — nicht als Route genutzt.

## Abschluss

Vor Commit/Push: das Commit-Wort des Operators (`/commit`) — der gemessene
Abschluss-Check läuft dann mit Commit und Push. `/consent` ist der session-weite
Consent (Delegation), nie das Commit-Wort.
